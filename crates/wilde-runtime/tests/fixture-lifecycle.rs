#![cfg(any(windows, unix))]
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use wilde_contracts::{ProcessVerdict, SessionIdentity, StreamCursor};
use wilde_runtime::{FixtureInput, MAX_CLEANUP_WAIT, OutputEvent, OwnedFixtureSession, PtySize};

static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);

fn fixture() -> OwnedFixtureSession {
    OwnedFixtureSession::spawn(
        SessionIdentity {
            session_id: "synthetic-pty".into(),
            execution_host_id: "local-fixture".into(),
            workspace_id: "synthetic-folder".into(),
            generation: NEXT_GENERATION.fetch_add(1, Ordering::Relaxed),
        },
        PtySize::default(),
    )
    .expect("installed fixture shell and native PTY must be available")
}

fn await_marker(
    session: &mut OwnedFixtureSession,
    marker: &[u8],
    cursor: &mut StreamCursor,
) -> Vec<u8> {
    let deadline = Instant::now() + Duration::from_secs(4);
    let mut bytes = Vec::new();
    while Instant::now() < deadline {
        match session
            .read_output(Duration::from_millis(100))
            .expect("ordered native output")
        {
            OutputEvent::Frame(frame) => {
                cursor
                    .accept(&frame)
                    .expect("identity and contiguous sequence");
                bytes.extend(frame.payload);
                assert!(bytes.len() <= 65536, "fixture output must stay bounded");
                if bytes.windows(marker.len()).any(|window| window == marker) {
                    return bytes;
                }
            }
            OutputEvent::TimedOut => (),
            event => panic!("fixture ended before expected marker: {event:?}"),
        }
    }
    panic!("fixture output deadline exceeded");
}

fn assert_cleanup(session: &mut OwnedFixtureSession) {
    let report = session.close(MAX_CLEANUP_WAIT);
    assert_eq!(
        report.verdict,
        ProcessVerdict::Exited,
        "owned child exit must be observed: {report:?}"
    );
    assert!(
        report.reader_joined && report.lifecycle_joined,
        "workers must finish: {report:?}"
    );
    assert!(
        report.cwd_removed && !report.cwd.exists(),
        "own cwd must be removed: {report:?}"
    );
}

#[test]
fn real_native_fixture_preserves_bytes_and_survives_viewer_detach_resize_reattach() {
    let mut session = fixture();
    let identity = session.identity().clone();
    let pid = session.process_id();
    assert!(pid.is_some());
    let mut cursor = StreamCursor::from_snapshot(identity.clone(), 0).unwrap();
    let ready = await_marker(&mut session, b"WILDE_FIXTURE_READY", &mut cursor);
    assert!(
        ready.contains(&0x1b),
        "native terminal escape bytes must be retained"
    );
    session.detach();
    assert_eq!(
        session.read_output(Duration::ZERO).unwrap(),
        OutputEvent::Detached
    );
    assert_eq!(session.verdict(), ProcessVerdict::Live);
    let size = PtySize {
        rows: 31,
        cols: 101,
        pixel_width: 0,
        pixel_height: 0,
    };
    session.resize(size).unwrap();
    assert_eq!(session.size().unwrap(), size);
    session.write_input(FixtureInput::Ping).unwrap();
    session.attach();
    assert_eq!(session.process_id(), pid);
    assert_eq!(session.identity(), &identity);
    await_marker(&mut session, b"WILDE_FIXTURE_PONG", &mut cursor);
    session.write_input(FixtureInput::Finish).unwrap();
    await_marker(&mut session, b"WILDE_FIXTURE_DONE", &mut cursor);
    let deadline = Instant::now() + Duration::from_secs(1);
    while session.verdict() != ProcessVerdict::Exited && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(session.verdict(), ProcessVerdict::Exited);
    assert_cleanup(&mut session);
    assert_eq!(session.cleanup_receipt().snapshot().exit_code, Some(0));
}

#[test]
fn explicit_close_kills_only_the_owned_fixture_and_is_repeatable() {
    let mut session = fixture();
    let mut cursor = StreamCursor::from_snapshot(session.identity().clone(), 0).unwrap();
    await_marker(&mut session, b"WILDE_FIXTURE_READY", &mut cursor);
    let first = session.cleanup_receipt().snapshot();
    assert_eq!(
        session.read_output(Duration::from_millis(20)).unwrap(),
        OutputEvent::TimedOut
    );
    assert_cleanup(&mut session);
    let repeated = session.close(Duration::ZERO);
    assert_eq!(repeated.process_id, first.process_id);
    assert_eq!(repeated.generation, first.generation);
    assert_eq!(repeated.verdict, ProcessVerdict::Exited);
    assert!(session.write_input(FixtureInput::Ping).is_err());
    assert!(session.resize(PtySize::default()).is_err());
}

#[test]
fn dropping_viewer_owner_has_an_observable_cleanup_receipt() {
    let mut session = fixture();
    let mut cursor = StreamCursor::from_snapshot(session.identity().clone(), 0).unwrap();
    await_marker(&mut session, b"WILDE_FIXTURE_READY", &mut cursor);
    let receipt = session.cleanup_receipt();
    let started = Instant::now();
    drop(session);
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "drop must not wait without a deadline"
    );
    let report = receipt.snapshot();
    assert_eq!(
        report.verdict,
        ProcessVerdict::Exited,
        "exit must be observed: {report:?}"
    );
    assert!(
        report.reader_joined && report.lifecycle_joined && report.cwd_removed,
        "cleanup incomplete: {report:?}"
    );
}
#[test]
fn fixed_lifetime_stops_an_unattended_fixture_without_observer_involvement() {
    let mut session = fixture();
    let mut cursor = StreamCursor::from_snapshot(session.identity().clone(), 0).unwrap();
    await_marker(&mut session, b"WILDE_FIXTURE_READY", &mut cursor);
    session.detach();
    let deadline = Instant::now() + wilde_runtime::MAX_FIXTURE_LIFETIME + MAX_CLEANUP_WAIT;
    while session.verdict() != ProcessVerdict::Exited && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    assert_eq!(session.verdict(), ProcessVerdict::Exited);
    assert_eq!(
        session.stream_fault(),
        Some(wilde_runtime::StreamFault::LeaseExpired)
    );
    session.attach();
    assert!(session.read_output(Duration::ZERO).is_err());
    assert_cleanup(&mut session);
}
