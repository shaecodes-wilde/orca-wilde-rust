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
pub struct CleanupReport {
    pub process_id: Option<u32>,
    pub generation: u64,
    pub cwd: PathBuf,
    pub verdict: ProcessVerdict,
    pub exit_code: Option<u32>,
    pub reader_joined: bool,
    pub lifecycle_joined: bool,
    pub cwd_removed: bool,
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
        return ProcessVerdict::Unverifiable;
    };
    match child.try_wait() {
        Ok(Some(status)) => {
            record_exit(receipt, status);
            ProcessVerdict::Exited
        }
        Ok(None) => ProcessVerdict::Live,
        Err(_) => ProcessVerdict::Unverifiable,
    }
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
            receipt.update(|report| report.verdict = ProcessVerdict::Unverifiable);
            break;
        };
        match owned.try_wait() {
            Ok(Some(status)) => {
                record_exit(&receipt, status);
                break;
            }
            Ok(None) => receipt.update(|report| report.verdict = ProcessVerdict::Live),
            Err(_) => receipt.update(|report| report.verdict = ProcessVerdict::Unverifiable),
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
