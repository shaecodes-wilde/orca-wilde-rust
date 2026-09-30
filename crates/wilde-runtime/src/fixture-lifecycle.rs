use crate::fixture_output::LEASE_EXPIRED;
use portable_pty::{Child, ExitStatus};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use wilde_contracts::ProcessVerdict;

pub const MAX_FIXTURE_LIFETIME: Duration = Duration::from_secs(8);
pub const MAX_CLEANUP_WAIT: Duration = Duration::from_secs(2);
pub(crate) type OwnedChild = Arc<Mutex<Box<dyn Child + Send + Sync>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CwdRemovalError {
    pub kind: std::io::ErrorKind,
    pub raw_os_error: Option<i32>,
}

impl CwdRemovalError {
    pub(crate) fn from_io(error: &std::io::Error) -> Self {
        Self {
            kind: error.kind(),
            raw_os_error: error.raw_os_error(),
        }
    }
    pub(crate) fn is_transient_lock(&self) -> bool {
        if matches!(
            self.kind,
            std::io::ErrorKind::Interrupted | std::io::ErrorKind::WouldBlock
        ) {
            return true;
        }
        #[cfg(windows)]
        if matches!(self.raw_os_error, Some(32 | 33)) {
            return true;
        }
        false
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CleanupTiming {
    pub writer_drop: Duration,
    pub verdict_observation: Duration,
    pub master_drop: Duration,
    pub worker_join: Duration,
    pub manifest_removal: Duration,
    pub total_close: Duration,
    pub deadline_exhausted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupReport {
    pub process_id: Option<u32>,
    pub generation: u64,
    pub cwd: PathBuf,
    pub verdict: ProcessVerdict,
    pub exit_code: Option<u32>,
    pub reader_joined: bool,
    pub lifecycle_joined: bool,
    pub cwd_removed: bool,
    pub cwd_remove_error: Option<CwdRemovalError>,
    pub timing: CleanupTiming,
}

#[derive(Clone)]
pub struct CleanupReceipt(pub(crate) Arc<Mutex<CleanupReport>>);

impl CleanupReceipt {
    pub fn snapshot(&self) -> CleanupReport {
        self.0
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .clone()
    }
    pub(crate) fn update(&self, apply: impl FnOnce(&mut CleanupReport)) {
        apply(&mut self.0.lock().unwrap_or_else(|poison| poison.into_inner()));
    }
}

pub(crate) fn verdict(child: &OwnedChild, receipt: &CleanupReceipt) -> ProcessVerdict {
    let Ok(mut child) = child.lock() else {
        record_unverifiable(receipt);
        return ProcessVerdict::Unverifiable;
    };
    match child.try_wait() {
        Ok(Some(status)) => {
            record_exit(receipt, status);
            ProcessVerdict::Exited
        }
        Ok(None) => ProcessVerdict::Live,
        Err(_) => {
            record_unverifiable(receipt);
            ProcessVerdict::Unverifiable
        }
    }
}

fn record_unverifiable(receipt: &CleanupReceipt) {
    receipt.update(|report| {
        report.verdict = ProcessVerdict::Unverifiable;
        report.exit_code = None;
    });
}

fn record_exit(receipt: &CleanupReceipt, status: ExitStatus) {
    receipt.update(|report| {
        report.verdict = ProcessVerdict::Exited;
        report.exit_code = Some(status.exit_code());
    });
}

pub(crate) fn supervise(
    child: OwnedChild,
    fault: Arc<AtomicU8>,
    stop: Arc<AtomicBool>,
    receipt: CleanupReceipt,
) {
    let deadline = Instant::now() + MAX_FIXTURE_LIFETIME;
    let mut stop_deadline = None;
    loop {
        let now = Instant::now();
        if now >= deadline {
            let _ = fault.compare_exchange(0, LEASE_EXPIRED, Ordering::AcqRel, Ordering::Acquire);
        }
        let Ok(mut owned) = child.lock() else {
            record_unverifiable(&receipt);
            break;
        };
        match owned.try_wait() {
            Ok(Some(status)) => {
                record_exit(&receipt, status);
                break;
            }
            Ok(None) => receipt.update(|report| report.verdict = ProcessVerdict::Live),
            Err(_) => record_unverifiable(&receipt),
        }
        if stop_deadline.is_none()
            && (stop.load(Ordering::Acquire) || fault.load(Ordering::Acquire) != 0)
        {
            // Poll and signal under one ownership lock; a reaped PID is never signalled later.
            let _ = owned.kill();
            stop_deadline = Some(Instant::now() + MAX_CLEANUP_WAIT);
        }
        drop(owned);
        if stop_deadline.is_some_and(|limit| Instant::now() >= limit) {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod observation_failure_tests {
    use super::*;
    use portable_pty::ChildKiller;

    #[derive(Debug, Clone)]
    struct UnavailableObservation;

    impl ChildKiller for UnavailableObservation {
        fn kill(&mut self) -> std::io::Result<()> {
            Ok(())
        }
        fn clone_killer(&self) -> Box<dyn ChildKiller + Send + Sync> {
            Box::new(self.clone())
        }
    }
    impl Child for UnavailableObservation {
        fn try_wait(&mut self) -> std::io::Result<Option<ExitStatus>> {
            Err(std::io::Error::other(
                "synthetic owned child observation failed",
            ))
        }
        fn wait(&mut self) -> std::io::Result<ExitStatus> {
            Err(std::io::Error::other(
                "synthetic owned child observation failed",
            ))
        }
        fn process_id(&self) -> Option<u32> {
            None
        }
        #[cfg(windows)]
        fn as_raw_handle(&self) -> Option<std::os::windows::io::RawHandle> {
            None
        }
    }

    fn receipt() -> CleanupReceipt {
        CleanupReceipt(Arc::new(Mutex::new(CleanupReport {
            process_id: None,
            generation: 1,
            cwd: PathBuf::from("synthetic-unused-cwd"),
            verdict: ProcessVerdict::Live,
            exit_code: None,
            reader_joined: false,
            lifecycle_joined: false,
            cwd_removed: false,
            cwd_remove_error: None,
            timing: CleanupTiming::default(),
        })))
    }

    #[test]
    fn genuine_child_observation_error_replaces_stale_live_receipt() {
        let child: OwnedChild = Arc::new(Mutex::new(Box::new(UnavailableObservation)));
        let receipt = receipt();
        assert_eq!(receipt.snapshot().verdict, ProcessVerdict::Live);
        assert_eq!(verdict(&child, &receipt), ProcessVerdict::Unverifiable);
        assert_eq!(receipt.snapshot().verdict, ProcessVerdict::Unverifiable);
        assert_eq!(receipt.snapshot().exit_code, None);
    }

    #[test]
    fn poisoned_ownership_lock_also_replaces_stale_live_receipt() {
        let child: OwnedChild = Arc::new(Mutex::new(Box::new(UnavailableObservation)));
        let receipt = receipt();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _held = child.lock().unwrap();
            panic!("synthetic child owner failure");
        }));
        assert!(result.is_err());
        assert_eq!(verdict(&child, &receipt), ProcessVerdict::Unverifiable);
        assert_eq!(receipt.snapshot().verdict, ProcessVerdict::Unverifiable);
    }
}
