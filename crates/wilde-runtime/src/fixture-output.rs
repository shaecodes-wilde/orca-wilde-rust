use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{SyncSender, TrySendError};

pub(crate) const CHUNK_BYTES: usize = 4096;
pub(crate) const QUEUE_CHUNKS: usize = 16;
pub(crate) const OVERFLOW: u8 = 1;
pub(crate) const READ_FAILED: u8 = 2;
pub(crate) const LEASE_EXPIRED: u8 = 3;

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
    sender: SyncSender<Vec<u8>>,
    fault: Arc<AtomicU8>,
) {
    let mut buffer = [0; CHUNK_BYTES];
    let mut discard = false;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                if !discard && fault.load(Ordering::Acquire) == 0 {
                    discard = !enqueue(&sender, buffer[..count].to_vec(), &fault);
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
        read_output(Box::new(reader), sender, Arc::clone(&fault));
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
}
