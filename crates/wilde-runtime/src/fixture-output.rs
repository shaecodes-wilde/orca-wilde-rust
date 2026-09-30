use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{SyncSender, TrySendError};
use wilde_contracts::{SessionIdentity, TerminalFrame};

pub(crate) const CHUNK_BYTES: usize = 4096;
pub(crate) const QUEUE_CHUNKS: usize = 16;
pub(crate) const OVERFLOW: u8 = 1;
pub(crate) const READ_FAILED: u8 = 2;
pub(crate) const LEASE_EXPIRED: u8 = 3;
pub(crate) const CONSUMER_FAILED: u8 = 4;
pub(crate) const SEQUENCE_EXHAUSTED: u8 = 5;

pub(crate) type OutputSink = Box<dyn FnMut(TerminalFrame) -> anyhow::Result<()> + Send>;

pub(crate) enum OutputConsumer {
    Raw(SyncSender<Vec<u8>>),
    Canonical {
        identity: SessionIdentity,
        next_sequence: u64,
        sink: OutputSink,
    },
}

impl OutputConsumer {
    fn publish(&mut self, bytes: &[u8], fault: &AtomicU8) -> bool {
        match self {
            Self::Raw(sender) => enqueue(sender, bytes.to_vec(), fault),
            Self::Canonical {
                identity,
                next_sequence,
                sink,
            } => {
                let Some(next) = next_sequence.checked_add(1) else {
                    let _ = fault.compare_exchange(
                        0,
                        SEQUENCE_EXHAUSTED,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );
                    return false;
                };
                let frame = TerminalFrame {
                    identity: identity.clone(),
                    sequence: *next_sequence,
                    payload: bytes.to_vec(),
                };
                // The canonical consumer must bound synchronous work; it is never the viewer.
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sink(frame)));
                if !matches!(result, Ok(Ok(()))) {
                    let _ = fault.compare_exchange(
                        0,
                        CONSUMER_FAILED,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );
                    return false;
                }
                *next_sequence = next;
                true
            }
        }
    }
}

pub(crate) fn enqueue(sender: &SyncSender<Vec<u8>>, bytes: Vec<u8>, fault: &AtomicU8) -> bool {
    if fault.load(Ordering::Acquire) != 0 {
        return false;
    }
    match sender.try_send(bytes) {
        Ok(()) => true,
        Err(TrySendError::Full(_)) => {
            let _ = fault.compare_exchange(0, OVERFLOW, Ordering::AcqRel, Ordering::Acquire);
            false
        }
        Err(TrySendError::Disconnected(_)) => false,
    }
}

pub(crate) fn read_output(
    mut reader: Box<dyn Read + Send>,
    mut consumer: OutputConsumer,
    fault: Arc<AtomicU8>,
) {
    let mut buffer = [0; CHUNK_BYTES];
    let mut discard = false;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                if !discard && fault.load(Ordering::Acquire) == 0 {
                    discard = !consumer.publish(&buffer[..count], &fault);
                    if discard {
                        let _ = fault.compare_exchange(
                            0,
                            READ_FAILED,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        );
                    }
                }
                // Drain native bytes after a fatal stream fault; never feed a partial model.
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => {
                let _ = fault.compare_exchange(0, READ_FAILED, Ordering::AcqRel, Ordering::Acquire);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::sync_channel;

    #[test]
    fn slow_observer_gets_explicit_failure_without_byte_reordering() {
        let (sender, receiver) = sync_channel(2);
        let fault = AtomicU8::new(0);
        assert!(enqueue(&sender, b"\x1b[".to_vec(), &fault));
        assert!(enqueue(&sender, b"31m".to_vec(), &fault));
        assert!(!enqueue(&sender, b"overflow".to_vec(), &fault));
        assert_eq!(fault.load(Ordering::Acquire), OVERFLOW);
        assert_eq!(receiver.try_recv().unwrap(), b"\x1b[");
        assert_eq!(receiver.try_recv().unwrap(), b"31m");
        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn dropped_consumer_refuses_new_output_without_blocking() {
        let (sender, receiver) = sync_channel(1);
        drop(receiver);
        assert!(!enqueue(&sender, vec![0], &AtomicU8::new(0)));
    }
}

#[cfg(test)]
mod drain_failure_tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::atomic::AtomicUsize;
    use std::sync::mpsc::sync_channel;

    struct ChunkReader {
        chunks: VecDeque<Vec<u8>>,
        reads: Arc<AtomicUsize>,
    }
    impl Read for ChunkReader {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            self.reads.fetch_add(1, Ordering::Relaxed);
            let Some(bytes) = self.chunks.pop_front() else {
                return Ok(0);
            };
            buffer[..bytes.len()].copy_from_slice(&bytes);
            Ok(bytes.len())
        }
    }

    #[test]
    fn overflow_drains_remaining_native_bytes_without_publishing_them() {
        let (sender, receiver) = sync_channel(1);
        let fault = Arc::new(AtomicU8::new(0));
        let reads = Arc::new(AtomicUsize::new(0));
        let reader = ChunkReader {
            chunks: VecDeque::from([
                b"admitted".to_vec(),
                b"overflow".to_vec(),
                b"discarded".to_vec(),
            ]),
            reads: Arc::clone(&reads),
        };
        read_output(
            Box::new(reader),
            OutputConsumer::Raw(sender),
            Arc::clone(&fault),
        );
        assert_eq!(
            reads.load(Ordering::Relaxed),
            4,
            "native drain must reach EOF after overflow"
        );
        assert_eq!(fault.load(Ordering::Acquire), OVERFLOW);
        assert_eq!(receiver.try_recv().unwrap(), b"admitted");
        assert!(
            receiver.try_recv().is_err(),
            "post-fault bytes must never enter the model stream"
        );
    }

    fn identity() -> SessionIdentity {
        SessionIdentity {
            session_id: "synthetic-canonical-output".into(),
            execution_host_id: "local-fixture".into(),
            workspace_id: "synthetic-folder".into(),
            generation: 1,
        }
    }

    struct SustainedReader {
        remaining: usize,
        sentinel_sent: bool,
    }

    impl Read for SustainedReader {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            if self.remaining > 0 {
                let count = buffer.len().min(self.remaining);
                buffer[..count].fill(b'x');
                self.remaining -= count;
                return Ok(count);
            }
            if !self.sentinel_sent {
                let marker = b"FINAL_SENTINEL";
                buffer[..marker.len()].copy_from_slice(marker);
                self.sentinel_sent = true;
                return Ok(marker.len());
            }
            Ok(0)
        }
    }

    #[test]
    fn canonical_consumer_receives_sustained_ordered_bytes_without_a_viewer_queue() {
        use std::sync::Mutex;
        use wilde_contracts::StreamCursor;
        let total = 17 * 1024 * 1024 + 3;
        let id = identity();
        let mut cursor = StreamCursor::from_snapshot(id.clone(), 0).unwrap();
        let observed = Arc::new(Mutex::new((0usize, false)));
        let sink_observed = Arc::clone(&observed);
        let consumer = OutputConsumer::Canonical {
            identity: id,
            next_sequence: 0,
            sink: Box::new(move |frame| {
                cursor.accept(&frame).unwrap();
                assert!(frame.payload.len() <= CHUNK_BYTES);
                let mut stats = sink_observed.lock().unwrap();
                if frame.payload == b"FINAL_SENTINEL" {
                    assert_eq!(stats.0, total);
                    stats.1 = true;
                } else {
                    assert!(frame.payload.iter().all(|byte| *byte == b'x'));
                    stats.0 += frame.payload.len();
                }
                Ok(())
            }),
        };
        let fault = Arc::new(AtomicU8::new(0));
        read_output(
            Box::new(SustainedReader {
                remaining: total,
                sentinel_sent: false,
            }),
            consumer,
            Arc::clone(&fault),
        );
        assert_eq!(*observed.lock().unwrap(), (total, true));
        assert_eq!(fault.load(Ordering::Acquire), 0);
    }

    #[test]
    fn canonical_rejection_or_panic_latches_fault_and_drains_without_more_callbacks() {
        for panic in [false, true] {
            let calls = Arc::new(AtomicUsize::new(0));
            let sink_calls = Arc::clone(&calls);
            let reads = Arc::new(AtomicUsize::new(0));
            let reader = ChunkReader {
                chunks: VecDeque::from([b"rejected".to_vec(), b"discarded".to_vec()]),
                reads: Arc::clone(&reads),
            };
            let consumer = OutputConsumer::Canonical {
                identity: identity(),
                next_sequence: 0,
                sink: Box::new(move |_| {
                    sink_calls.fetch_add(1, Ordering::Relaxed);
                    assert!(!panic, "synthetic canonical consumer panic");
                    anyhow::bail!("synthetic canonical consumer rejection")
                }),
            };
            let fault = Arc::new(AtomicU8::new(0));
            read_output(Box::new(reader), consumer, Arc::clone(&fault));
            assert_eq!(fault.load(Ordering::Acquire), CONSUMER_FAILED);
            assert_eq!(calls.load(Ordering::Relaxed), 1);
            assert_eq!(reads.load(Ordering::Relaxed), 3);
        }
    }

    #[test]
    fn canonical_sequence_exhaustion_never_publishes_an_unrepresentable_cursor() {
        let fault = AtomicU8::new(0);
        let mut consumer = OutputConsumer::Canonical {
            identity: identity(),
            next_sequence: u64::MAX,
            sink: Box::new(|_| panic!("exhausted sequence must not reach consumer")),
        };
        assert!(!consumer.publish(b"unpublished", &fault));
        assert_eq!(fault.load(Ordering::Acquire), SEQUENCE_EXHAUSTED);
    }
}
