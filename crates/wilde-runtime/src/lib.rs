//! Fixture-only PTY lifecycle, quarantined behind an outer owned-runner watchdog.
//! Native reader/master cleanup has no proven hard deadline; production R08 is not accepted.
#[path = "fixture-command.rs"]
mod fixture_command;
#[path = "fixture-directory.rs"]
mod fixture_directory;
#[path = "fixture-lifecycle.rs"]
mod fixture_lifecycle;
#[path = "fixture-output.rs"]
mod fixture_output;

use anyhow::{Context, Result, bail};
use fixture_command::FixtureDirectory;
use fixture_lifecycle::OwnedChild;
pub use fixture_lifecycle::{
    CleanupReceipt, CleanupReport, CleanupTiming, CwdRemovalError, ExitObservation,
    KillAdapterError, KillObservation, MAX_CLEANUP_WAIT, MAX_FIXTURE_LIFETIME, SupervisionRecord,
};
pub use portable_pty::PtySize;
use portable_pty::{MasterPty, native_pty_system};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, sync_channel};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use wilde_contracts::{ProcessVerdict, SessionIdentity, TerminalFrame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureInput {
    Ping,
    Finish,
    SustainedOutput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamFault {
    Overflow,
    ReadFailed,
    LeaseExpired,
    ConsumerFailed,
    SequenceExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputEvent {
    Frame(TerminalFrame),
    TimedOut,
    Detached,
    End,
}

pub struct OwnedFixtureSession {
    identity: SessionIdentity,
    next_sequence: u64,
    attached: bool,
    closed: bool,
    child: OwnedChild,
    master: Option<Box<dyn MasterPty + Send>>,
    writer: Option<Box<dyn Write + Send>>,
    output: Option<Receiver<Vec<u8>>>,
    fault: Arc<AtomicU8>,
    stop: Arc<AtomicBool>,
    reader: Option<JoinHandle<()>>,
    lifecycle: Option<JoinHandle<()>>,
    cwd: FixtureDirectory,
    receipt: CleanupReceipt,
}

impl OwnedFixtureSession {
    pub fn spawn(identity: SessionIdentity, size: PtySize) -> Result<Self> {
        Self::spawn_inner(identity, size, None)
    }

    /// The reader feeds one canonical host consumer, independent of viewer attachment.
    /// The sink must consume every frame with bounded synchronous work; it cannot restore a parser from a tail.
    pub fn spawn_with_output_sink(
        identity: SessionIdentity,
        size: PtySize,
        sink: impl FnMut(TerminalFrame) -> Result<()> + Send + 'static,
    ) -> Result<Self> {
        Self::spawn_inner(identity, size, Some(Box::new(sink)))
    }

    fn spawn_inner(
        identity: SessionIdentity,
        size: PtySize,
        sink: Option<fixture_output::OutputSink>,
    ) -> Result<Self> {
        fixture_command::require_guard(std::env::var_os("WILDE_RUNTIME_FIXTURE_GUARD").as_deref())?;
        fixture_command::validate(&identity, size)?;
        let mut cwd = FixtureDirectory::create()?;
        let command = fixture_command::command(&cwd.0)?;
        let pair = native_pty_system()
            .openpty(size)
            .context("open fixture PTY")?;
        let reader = pair
            .master
            .try_clone_reader()
            .context("open native output reader")?;
        let writer = pair
            .master
            .take_writer()
            .context("open fixture input writer")?;
        let child = pair
            .slave
            .spawn_command(command)
            .context("spawn fixed fixture")?;
        cwd.1 = false;
        drop(pair.slave);
        let receipt = CleanupReceipt(Arc::new(Mutex::new(CleanupReport {
            process_id: child.process_id(),
            generation: identity.generation,
            cwd: cwd.0.clone(),
            verdict: ProcessVerdict::Live,
            exit_code: None,
            reader_joined: false,
            lifecycle_joined: false,
            cwd_removed: false,
            cwd_remove_error: None,
            timing: CleanupTiming::default(),
            supervision: SupervisionRecord::default(),
        })));
        let (consumer, output) = match sink {
            Some(sink) => (
                fixture_output::OutputConsumer::Canonical {
                    identity: identity.clone(),
                    next_sequence: 0,
                    sink,
                },
                None,
            ),
            None => {
                let (sender, output) = sync_channel(fixture_output::QUEUE_CHUNKS);
                (fixture_output::OutputConsumer::Raw(sender), Some(output))
            }
        };
        let mut session = Self {
            identity,
            next_sequence: 0,
            attached: true,
            closed: false,
            child: Arc::new(Mutex::new(child)),
            master: Some(pair.master),
            writer: Some(writer),
            output,
            fault: Arc::new(AtomicU8::new(0)),
            stop: Arc::new(AtomicBool::new(false)),
            reader: None,
            lifecycle: None,
            cwd,
            receipt,
        };
        let fault = Arc::clone(&session.fault);
        session.reader = Some(
            std::thread::Builder::new()
                .name("wilde-fixture-output".into())
                .spawn(move || fixture_output::read_output(reader, consumer, fault))
                .context("start bounded fixture reader")?,
        );
        let child = Arc::clone(&session.child);
        let fault = Arc::clone(&session.fault);
        let stop = Arc::clone(&session.stop);
        let receipt = session.receipt.clone();
        session.lifecycle = Some(
            std::thread::Builder::new()
                .name("wilde-fixture-lifecycle".into())
                .spawn(move || fixture_lifecycle::supervise(child, fault, stop, receipt))
                .context("start bounded fixture lifecycle")?,
        );
        Ok(session)
    }

    pub fn identity(&self) -> &SessionIdentity {
        &self.identity
    }
    pub fn process_id(&self) -> Option<u32> {
        self.receipt.snapshot().process_id
    }
    pub fn cleanup_receipt(&self) -> CleanupReceipt {
        self.receipt.clone()
    }
    pub fn detach(&mut self) {
        self.attached = false;
    }
    pub fn attach(&mut self) {
        self.attached = true;
    }
    pub fn verdict(&self) -> ProcessVerdict {
        fixture_lifecycle::verdict(&self.child, &self.receipt)
    }

    pub fn stream_fault(&self) -> Option<StreamFault> {
        match self.fault.load(Ordering::Acquire) {
            fixture_output::OVERFLOW => Some(StreamFault::Overflow),
            fixture_output::READ_FAILED => Some(StreamFault::ReadFailed),
            fixture_output::LEASE_EXPIRED => Some(StreamFault::LeaseExpired),
            fixture_output::CONSUMER_FAILED => Some(StreamFault::ConsumerFailed),
            fixture_output::SEQUENCE_EXHAUSTED => Some(StreamFault::SequenceExhausted),
            _ => None,
        }
    }

    pub fn read_output(&mut self, budget: Duration) -> Result<OutputEvent> {
        if let Some(fault) = self.stream_fault() {
            bail!("fixture stream failed: {fault:?}");
        }
        let output = self
            .output
            .as_ref()
            .context("canonical output is owned by the host sink; raw reads are unavailable")?;
        if !self.attached {
            return Ok(OutputEvent::Detached);
        }
        let payload = match output.recv_timeout(budget.min(MAX_FIXTURE_LIFETIME)) {
            Ok(payload) => payload,
            Err(RecvTimeoutError::Timeout) => return Ok(OutputEvent::TimedOut),
            Err(RecvTimeoutError::Disconnected) => {
                if let Some(fault) = self.stream_fault() {
                    bail!("fixture stream failed: {fault:?}");
                }
                return Ok(OutputEvent::End);
            }
        };
        if let Some(fault) = self.stream_fault() {
            bail!("fixture stream failed: {fault:?}");
        }
        let sequence = self.next_sequence;
        self.next_sequence = sequence
            .checked_add(1)
            .context("fixture sequence exhausted")?;
        Ok(OutputEvent::Frame(TerminalFrame {
            identity: self.identity.clone(),
            sequence,
            payload,
        }))
    }

    pub fn write_input(&mut self, input: FixtureInput) -> Result<()> {
        if self.closed || self.stream_fault().is_some() {
            bail!("fixture is closed or failed");
        }
        #[cfg(windows)]
        let bytes: &[u8] = match input {
            FixtureInput::Ping => b"ping\r",
            FixtureInput::Finish => b"finish\r",
            FixtureInput::SustainedOutput => b"sustained-output\r",
        };
        #[cfg(not(windows))]
        let bytes: &[u8] = match input {
            FixtureInput::Ping => b"ping\n",
            FixtureInput::Finish => b"finish\n",
            FixtureInput::SustainedOutput => b"sustained-output\n",
        };
        let writer = self.writer.as_mut().context("fixture input is closed")?;
        writer
            .write_all(bytes)
            .context("write fixed fixture input")?;
        writer.flush().context("flush fixture input")
    }

    pub fn resize(&self, size: PtySize) -> Result<()> {
        fixture_command::validate_size(size)?;
        self.master
            .as_ref()
            .context("fixture PTY is closed")?
            .resize(size)
    }

    pub fn size(&self) -> Result<PtySize> {
        self.master
            .as_ref()
            .context("fixture PTY is closed")?
            .get_size()
    }

    pub fn close(&mut self, budget: Duration) -> CleanupReport {
        let started = Instant::now();
        let mut timing = CleanupTiming::default();
        self.closed = true;
        self.stop.store(true, Ordering::Release);
        let phase = Instant::now();
        self.writer.take();
        timing.writer_drop = phase.elapsed();
        let deadline = Instant::now() + budget.min(MAX_CLEANUP_WAIT);
        loop {
            let phase = Instant::now();
            let verdict = self.verdict();
            timing.verdict_observation = timing.verdict_observation.saturating_add(phase.elapsed());
            // Native ConPTY destruction may block; the outer owned runner is the timeout boundary.
            if verdict == ProcessVerdict::Exited {
                let phase = Instant::now();
                self.master.take();
                timing.master_drop = timing.master_drop.saturating_add(phase.elapsed());
            }
            let phase = Instant::now();
            join_finished(&mut self.reader, &self.receipt, true);
            join_finished(&mut self.lifecycle, &self.receipt, false);
            timing.worker_join = timing.worker_join.saturating_add(phase.elapsed());
            let workers = self.receipt.snapshot();
            if verdict == ProcessVerdict::Exited
                && workers.reader_joined
                && workers.lifecycle_joined
            {
                let phase = Instant::now();
                let removal = self.cwd.remove_before(deadline);
                timing.manifest_removal = timing.manifest_removal.saturating_add(phase.elapsed());
                match removal {
                    Ok(()) => {
                        self.receipt.update(|report| {
                            report.cwd_removed = true;
                            report.cwd_remove_error = None;
                        });
                        break;
                    }
                    Err(error) => {
                        let failure = CwdRemovalError::from_io(&error);
                        let retry = failure.is_transient_lock();
                        self.receipt.update(|report| {
                            report.cwd_removed = false;
                            report.cwd_remove_error = Some(failure);
                        });
                        if !retry {
                            break;
                        }
                    }
                }
            }
            if Instant::now() >= deadline {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        timing.total_close = started.elapsed();
        timing.deadline_exhausted = Instant::now() >= deadline;
        self.receipt.update(|report| report.timing = timing);
        self.receipt.snapshot()
    }
}

fn join_finished(worker: &mut Option<JoinHandle<()>>, receipt: &CleanupReceipt, reader: bool) {
    if worker.as_ref().is_some_and(JoinHandle::is_finished) {
        let joined = worker.take().is_some_and(|handle| handle.join().is_ok());
        receipt.update(|report| {
            if reader {
                report.reader_joined = joined;
            } else {
                report.lifecycle_joined = joined;
            }
        });
    }
}

impl Drop for OwnedFixtureSession {
    fn drop(&mut self) {
        // Failed supervisor creation still leaves an owned child requiring cleanup.
        if self.lifecycle.is_none()
            && self.verdict() != ProcessVerdict::Exited
            && let Ok(mut child) = self.child.lock()
        {
            let _ = child.kill();
        }
        let _ = self.close(MAX_CLEANUP_WAIT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn identity() -> SessionIdentity {
        SessionIdentity {
            session_id: "synthetic".into(),
            execution_host_id: "local-fixture".into(),
            workspace_id: "fixture-folder".into(),
            generation: 1,
        }
    }
    #[test]
    fn invalid_host_generation_and_geometry_are_refused_before_launch() {
        let mut id = identity();
        id.execution_host_id = "ssh-host".into();
        assert!(fixture_command::validate(&id, PtySize::default()).is_err());
        id = identity();
        id.generation = 0;
        assert!(fixture_command::validate(&id, PtySize::default()).is_err());
        let invalid = PtySize {
            rows: 0,
            ..PtySize::default()
        };
        assert!(fixture_command::validate(&identity(), invalid).is_err());
    }
    #[test]
    fn oversized_and_control_character_identifiers_are_refused() {
        let mut id = identity();
        id.session_id = "x".repeat(241);
        assert!(fixture_command::validate(&id, PtySize::default()).is_err());
        id.session_id = "bad\nidentity".into();
        assert!(fixture_command::validate(&id, PtySize::default()).is_err());
    }
}
