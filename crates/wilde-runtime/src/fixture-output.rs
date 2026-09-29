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
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                if !enqueue(&sender, buffer[..count].to_vec(), &fault) {
                    break;
                }
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
