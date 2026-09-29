use anyhow::{Result, bail, ensure};
use serde::Serialize;
use std::{
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use wilde_contracts::{ProcessVerdict, SessionIdentity};
use wilde_domain::{Record, RecordId, Request, Store, TargetKind, WorkspaceTarget};
use wilde_runtime::{FixtureInput, OutputEvent, OwnedFixtureSession, PtySize};

#[derive(Debug, Serialize)]
pub struct PilotEvidence {
    pub application_id: &'static str,
    pub records: usize,
    pub database_reopened: bool,
    pub duplicate_request_replayed: bool,
    pub terminal_ready: bool,
    pub terminal_pong: bool,
    pub terminal_finished: bool,
    pub verdict: ProcessVerdict,
    pub exit_code: Option<u32>,
    pub reader_joined: bool,
    pub lifecycle_joined: bool,
    pub fixture_cwd_removed: bool,
}

pub fn seed_synthetic(path: &Path) -> Result<wilde_domain::Snapshot> {
    let mut store = Store::open_dev(path, "native-pilot-owner")?;
    let id = |number: u32| RecordId::new(format!("00000000-0000-4000-8000-{number:012}"));
    let request = Request {
        request_id: "native-pilot-seed-v1".into(),
        expected_revision: 0,
        records: vec![
            Record::Client {
                id: id(1)?,
                revision: 0,
                name: "Demo Solar".into(),
                owner: "Wilde Demo".into(),
            },
            Record::Project {
                id: id(2)?,
                revision: 0,
                client_id: id(1)?,
                title: "Checklist Demo".into(),
            },
            Record::Assignment {
                id: id(3)?,
                revision: 0,
                client_id: id(1)?,
                project_id: Some(id(2)?),
                target: WorkspaceTarget {
                    kind: TargetKind::Folder,
                    runtime_owner_id: "native-pilot-owner".into(),
                    host_id: "local-fixture".into(),
                    stable_id: "native-synthetic-folder-v1".into(),
                    locator: "synthetic-folder-not-executed".into(),
                    name: "Synthetic fixture workspace".into(),
                },
                reason: "Native migration proof".into(),
            },
        ],
    };
    let saved = store.apply(request.clone())?;
    ensure!(
        store.apply(request)? == saved,
        "retry changed its original outcome"
    );
    drop(store);
    let mut reopened = Store::open_dev(path, "native-pilot-owner")?;
    ensure!(
        reopened.snapshot()? == saved,
        "SQLite reopen changed saved assignment"
    );
    Ok(saved)
}

pub fn fixture_identity() -> Result<SessionIdentity> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(SessionIdentity {
        session_id: format!("native-pilot-{}-{nonce}", std::process::id()),
        execution_host_id: "local-fixture".into(),
        workspace_id: "native-synthetic-folder-v1".into(),
        generation: 1,
    })
}

pub fn run_headless_pilot(path: &Path) -> Result<PilotEvidence> {
    let saved = seed_synthetic(path)?;
    let identity = fixture_identity()?;
    let mut terminal = wilde_ide::TerminalModel::new_session(identity.clone(), 24, 80, 100)
        .map_err(|error| anyhow::anyhow!("terminal admission: {error:?}"))?;
    let mut session = OwnedFixtureSession::spawn(
        identity,
        PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        },
    )?;
    let started = Instant::now();
    let mut raw = Vec::new();
    let mut ping = false;
    let mut finish = false;
    while started.elapsed() < Duration::from_secs(7) {
        match session.read_output(Duration::from_millis(100))? {
            OutputEvent::Frame(frame) => {
                terminal
                    .accept_frame(&frame)
                    .map_err(|error| anyhow::anyhow!("terminal frame: {error:?}"))?;
                raw.extend_from_slice(&frame.payload);
                ensure!(raw.len() <= 65536, "fixture output exceeded pilot bound");
                let text = String::from_utf8_lossy(&raw);
                if text.contains("WILDE_FIXTURE_READY") && !ping {
                    session.detach();
                    ensure!(
                        session.verdict() == ProcessVerdict::Live,
                        "detach stopped fixture"
                    );
                    session.attach();
                    session.resize(PtySize {
                        rows: 24,
                        cols: 80,
                        pixel_width: 0,
                        pixel_height: 0,
                    })?;
                    session.write_input(FixtureInput::Ping)?;
                    ping = true;
                }
                if text.contains("WILDE_FIXTURE_PONG") && !finish {
                    session.write_input(FixtureInput::Finish)?;
                    finish = true;
                }
                if text.contains("WILDE_FIXTURE_DONE") {
                    break;
                }
            }
            OutputEvent::End => break,
            OutputEvent::Detached => bail!("unexpected detached observer"),
            OutputEvent::TimedOut => {}
        }
    }
    let natural_exit_deadline = Instant::now() + Duration::from_secs(1);
    while session.verdict() != ProcessVerdict::Exited && Instant::now() < natural_exit_deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    let natural_exit = session.verdict() == ProcessVerdict::Exited;
    let report = session.close(Duration::from_secs(2));
    let contents = terminal.contents();
    let evidence = PilotEvidence {
        application_id: wilde_contracts::DEV_APPLICATION_ID,
        records: saved.records.len(),
        database_reopened: true,
        duplicate_request_replayed: true,
        terminal_ready: contents.contains("WILDE_FIXTURE_READY"),
        terminal_pong: contents.contains("WILDE_FIXTURE_PONG"),
        terminal_finished: contents.contains("WILDE_FIXTURE_DONE"),
        verdict: report.verdict,
        exit_code: report.exit_code,
        reader_joined: report.reader_joined,
        lifecycle_joined: report.lifecycle_joined,
        fixture_cwd_removed: report.cwd_removed,
    };
    ensure!(
        evidence.records == 3
            && evidence.terminal_ready
            && evidence.terminal_pong
            && evidence.terminal_finished,
        "native headless slice output incomplete: {evidence:?}"
    );
    ensure!(
        natural_exit
            && report.verdict == ProcessVerdict::Exited
            && report.exit_code == Some(0)
            && report.reader_joined
            && report.lifecycle_joined
            && report.cwd_removed,
        "fixture cleanup incomplete: {report:?}"
    );
    Ok(evidence)
}
