#![cfg(any(windows, unix))]
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use wilde_contracts::{ProcessVerdict, SessionIdentity, StreamCursor, TerminalFrame};
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

fn sink_fixture(
    sink: impl FnMut(TerminalFrame) -> anyhow::Result<()> + Send + 'static,
) -> OwnedFixtureSession {
    OwnedFixtureSession::spawn_with_output_sink(
        SessionIdentity {
            session_id: "synthetic-sink-pty".into(),
            execution_host_id: "local-fixture".into(),
            workspace_id: "synthetic-folder".into(),
            generation: NEXT_GENERATION.fetch_add(1, Ordering::Relaxed),
        },
        PtySize::default(),
        sink,
    )
    .expect("guarded installed fixture shell and native PTY must be available")
}

#[derive(Default)]
struct SinkObservation {
    cursor: Option<StreamCursor>,
    bytes: usize,
    tail: Vec<u8>,
}

fn await_sink_marker(
    session: &OwnedFixtureSession,
    observation: &Mutex<SinkObservation>,
    marker: &[u8],
) {
    let deadline = Instant::now() + Duration::from_secs(7);
    loop {
        let observed = observation.lock().unwrap();
        if observed
            .tail
            .windows(marker.len())
            .any(|window| window == marker)
        {
            return;
        }
        let bytes = observed.bytes;
        drop(observed);
        assert!(
            session.stream_fault().is_none(),
            "canonical stream fault after {bytes} bytes: {:?}",
            session.stream_fault()
        );
        assert!(
            Instant::now() < deadline,
            "canonical marker deadline after {bytes} bytes: {:?}",
            session.cleanup_receipt().snapshot()
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn real_native_canonical_sink_receives_over_16_mib_while_viewer_is_detached() {
    let observed = Arc::new(Mutex::new(SinkObservation::default()));
    let sink_observed = Arc::clone(&observed);
    let mut session = sink_fixture(move |frame| {
        let mut observed = sink_observed.lock().unwrap();
        let cursor = observed
            .cursor
            .get_or_insert_with(|| StreamCursor::from_snapshot(frame.identity.clone(), 0).unwrap());
        cursor
            .accept(&frame)
            .expect("canonical reader identity and contiguous sequence");
        assert!(frame.payload.len() <= 4096);
        observed.bytes += frame.payload.len();
        observed.tail.extend_from_slice(&frame.payload);
        let remove = observed.tail.len().saturating_sub(128);
        observed.tail.drain(..remove);
        Ok(())
    });
    let identity = session.identity().clone();
    let pid = session.process_id();
    await_sink_marker(&session, &observed, b"WILDE_FIXTURE_READY");
    assert!(
        session.read_output(Duration::ZERO).is_err(),
        "raw reads cannot misrepresent canonical output as EOF"
    );
    session.detach();
    let size = PtySize {
        rows: 31,
        cols: 101,
        pixel_width: 0,
        pixel_height: 0,
    };
    session.resize(size).unwrap();
    assert_eq!(session.size().unwrap(), size);
    session.write_input(FixtureInput::SustainedOutput).unwrap();
    await_sink_marker(&session, &observed, b"WILDE_FIXTURE_STREAM_DONE");
    assert!(
        observed.lock().unwrap().bytes > 16 * 1024 * 1024,
        "actual native received bytes must exceed 16 MiB"
    );
    assert_eq!(session.verdict(), ProcessVerdict::Live);
    assert!(session.stream_fault().is_none());
    session.attach();
    assert_eq!(session.process_id(), pid);
    assert_eq!(session.identity(), &identity);
    session.write_input(FixtureInput::Ping).unwrap();
    await_sink_marker(&session, &observed, b"WILDE_FIXTURE_PONG");
    session.write_input(FixtureInput::Finish).unwrap();
    await_sink_marker(&session, &observed, b"WILDE_FIXTURE_DONE");
    await_natural_exit(&session);
    assert_cleanup(&mut session);
    let cleanup = session.cleanup_receipt().snapshot();
    let final_fault = session.stream_fault();
    let observation = observed.lock().unwrap();
    println!(
        "canonical native receipt: admitted_bytes={}, next_sequence={}, identity_unchanged={}, pid_unchanged={}, exit_code={:?}, reader_joined={}, lifecycle_joined={}, cwd_removed={}, final_fault={:?}",
        observation.bytes,
        observation.cursor.as_ref().unwrap().next_sequence(),
        session.identity() == &identity,
        session.process_id() == pid,
        cleanup.exit_code,
        cleanup.reader_joined,
        cleanup.lifecycle_joined,
        cleanup.cwd_removed,
        final_fault,
    );
    assert_eq!(cleanup.exit_code, Some(0));
    assert_eq!(
        final_fault, None,
        "stream must remain fault-free after reader cleanup"
    );
}

#[test]
fn canonical_sink_failure_stops_owned_fixture_and_preserves_explicit_fault() {
    for panics in [false, true] {
        let mut session = sink_fixture(move |_| {
            assert!(!panics, "synthetic canonical consumer panic");
            anyhow::bail!("synthetic canonical consumer rejection")
        });
        let deadline = Instant::now() + Duration::from_secs(2);
        while session.stream_fault().is_none() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(
            session.stream_fault(),
            Some(wilde_runtime::StreamFault::ConsumerFailed)
        );
        assert!(session.read_output(Duration::ZERO).is_err());
        assert!(session.write_input(FixtureInput::Ping).is_err());
        assert_cleanup(&mut session);
        assert_eq!(
            session.stream_fault(),
            Some(wilde_runtime::StreamFault::ConsumerFailed)
        );
    }
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
            .unwrap_or_else(|error| {
                panic!(
                    "fixture native read failed: {error}; admitted_bytes={}; escaped_prefix={}",
                    bytes.len(),
                    escaped_transcript(&bytes)
                )
            }) {
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
            event => panic!(
                "fixture ended before expected marker: {event:?}; admitted_bytes={}; escaped_prefix={}",
                bytes.len(),
                escaped_transcript(&bytes)
            ),
        }
    }
    panic!(
        "fixture output deadline exceeded; admitted_bytes={}; escaped_prefix={}",
        bytes.len(),
        escaped_transcript(&bytes)
    );
}

fn escaped_transcript(bytes: &[u8]) -> String {
    bytes[..bytes.len().min(4096)].escape_ascii().to_string()
}

fn await_quiet(session: &mut OwnedFixtureSession, cursor: &mut StreamCursor) {
    let deadline = Instant::now() + Duration::from_millis(100);
    for _ in 0..32 {
        let remaining = deadline.saturating_duration_since(Instant::now());
        match session
            .read_output(remaining)
            .expect("bounded admitted trailing output")
        {
            OutputEvent::Frame(frame) => cursor
                .accept(&frame)
                .expect("ordered trailing native frame"),
            OutputEvent::TimedOut => return,
            event => panic!("fixture ended while awaiting quiet output: {event:?}"),
        }
        if Instant::now() >= deadline {
            break;
        }
    }
    panic!("fixture output did not become quiet within 32 frames / 100 milliseconds");
}
fn await_natural_exit(session: &OwnedFixtureSession) {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let verdict = session.verdict();
        let report = session.cleanup_receipt().snapshot();
        if verdict == ProcessVerdict::Exited {
            assert_eq!(
                report.exit_code,
                Some(0),
                "natural exit must be observed before cleanup: {report:?}"
            );
            return;
        }
        assert!(
            Instant::now() < deadline,
            "natural exit observation deadline: {report:?}"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
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
    await_natural_exit(&session);

    assert_cleanup(&mut session);
    assert_eq!(session.cleanup_receipt().snapshot().exit_code, Some(0));
}

#[test]
fn explicit_close_kills_only_the_owned_fixture_and_is_repeatable() {
    let mut session = fixture();
    let mut cursor = StreamCursor::from_snapshot(session.identity().clone(), 0).unwrap();
    await_marker(&mut session, b"WILDE_FIXTURE_READY", &mut cursor);
    let first = session.cleanup_receipt().snapshot();
    await_quiet(&mut session, &mut cursor);

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
