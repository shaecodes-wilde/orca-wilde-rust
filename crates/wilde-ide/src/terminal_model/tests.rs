use super::*;
use wilde_contracts::MAX_FRAME_BYTES;

fn identity() -> SessionIdentity {
    SessionIdentity {
        session_id: "synthetic-session".into(),
        execution_host_id: "local-fixture".into(),
        workspace_id: "synthetic-folder".into(),
        generation: 1,
    }
}
fn model() -> TerminalModel {
    TerminalModel::new_session(identity(), 4, 20, 8).unwrap()
}
fn frame(sequence: u64, payload: &[u8]) -> TerminalFrame {
    TerminalFrame {
        identity: identity(),
        sequence,
        payload: payload.to_vec(),
    }
}

#[test]
fn split_ansi_retains_parser_state_across_frames() {
    let mut terminal = model();
    terminal.accept_frame(&frame(0, b"\x1b[3")).unwrap();
    assert_eq!(terminal.contents(), "");
    terminal.accept_frame(&frame(1, b"1mRED\x1b[0m!")).unwrap();
    assert_eq!(terminal.contents(), "RED!");
    assert_eq!(
        terminal.screen().cell(0, 0).unwrap().fgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        terminal.screen().cell(0, 3).unwrap().fgcolor(),
        vt100::Color::Default
    );
}

#[test]
fn every_byte_split_preserves_utf8_wide_and_combining_cells() {
    let mut terminal = model();
    for (sequence, byte) in "界🙂e\u{301}".as_bytes().iter().enumerate() {
        terminal
            .accept_frame(&frame(u64::try_from(sequence).unwrap(), &[*byte]))
            .unwrap();
    }
    assert_eq!(terminal.contents(), "界🙂e\u{301}");
    assert!(terminal.screen().cell(0, 0).unwrap().is_wide());
    assert!(terminal.screen().cell(0, 1).unwrap().is_wide_continuation());
    assert!(terminal.screen().cell(0, 2).unwrap().is_wide());
    assert_eq!(terminal.screen().cursor_position(), (0, 5));
}

#[test]
fn alternate_screen_restores_primary_contents_and_cursor() {
    let mut terminal = model();
    terminal
        .accept_frame(&frame(0, b"main\x1b[?1049h\x1b[2J\x1b[HALT"))
        .unwrap();
    assert!(terminal.screen().alternate_screen());
    assert_eq!(terminal.contents(), "ALT");
    terminal.accept_frame(&frame(1, b"\x1b[?1049l")).unwrap();
    assert!(!terminal.screen().alternate_screen());
    assert_eq!(terminal.contents(), "main");
    assert_eq!(terminal.screen().cursor_position(), (0, 4));
}

#[test]
fn resize_clamps_cursor_and_rejects_invalid_size_without_mutation() {
    let mut terminal = model();
    terminal.accept_frame(&frame(0, b"\x1b[4;20H")).unwrap();
    terminal.resize(2, 5).unwrap();
    assert_eq!(terminal.screen().size(), (2, 5));
    assert_eq!(terminal.screen().cursor_position(), (1, 4));
    assert_eq!(terminal.resize(0, 5), Err(TerminalError::InvalidDimensions));
    assert_eq!(terminal.screen().size(), (2, 5));
}

#[test]
fn stream_gap_latches_recovery_without_consuming_escape_fragment() {
    let mut terminal = model();
    terminal.accept_frame(&frame(0, b"safe\x1b[")).unwrap();
    assert_eq!(
        terminal.accept_frame(&frame(2, b"2Jbad")),
        Err(TerminalError::Contract(ContractError::SnapshotRequired))
    );
    assert_eq!(terminal.contents(), "safe");
    assert_eq!(terminal.next_sequence(), 1);
    assert!(terminal.snapshot_required());
    assert!(terminal.accept_frame(&frame(1, b"2Jlate")).is_err());
    assert_eq!(terminal.contents(), "safe");
}

#[test]
fn identity_generation_and_replay_rejections_preserve_screen() {
    for mutation in 0..3 {
        let mut terminal = model();
        terminal.accept_frame(&frame(0, b"safe")).unwrap();
        let mut rejected = frame(1, b"bad");
        match mutation {
            0 => rejected.identity.generation += 1,
            1 => rejected.identity.execution_host_id = "other-host".into(),
            _ => rejected.sequence = 0,
        }
        assert!(terminal.accept_frame(&rejected).is_err());
        assert!(terminal.snapshot_required());
        assert_eq!(terminal.contents(), "safe");
        assert_eq!(terminal.next_sequence(), 1);
    }
}

#[test]
fn oversized_frame_is_atomic_and_can_be_retried_at_same_sequence() {
    let mut terminal = model();
    assert_eq!(
        terminal.accept_frame(&frame(0, &vec![b'x'; MAX_FRAME_BYTES + 1])),
        Err(TerminalError::Contract(ContractError::FrameTooLarge))
    );
    assert_eq!(terminal.contents(), "");
    assert_eq!(terminal.next_sequence(), 0);
    assert!(!terminal.snapshot_required());
    terminal.accept_frame(&frame(0, b"valid")).unwrap();
    assert_eq!(terminal.contents(), "valid");
}

#[test]
fn scrollback_retention_and_admission_are_bounded() {
    let mut terminal = TerminalModel::new_session(identity(), 2, 20, 2).unwrap();
    terminal
        .accept_frame(&frame(0, b"one\r\ntwo\r\nthree\r\nfour\r\nfive"))
        .unwrap();
    assert_eq!(terminal.contents(), "four\nfive");
    terminal.set_scrollback(usize::MAX);
    assert_eq!(terminal.screen().scrollback(), 2);
    assert_eq!(terminal.contents(), "two\nthree");
    assert!(matches!(
        TerminalModel::new_session(identity(), 200, 400, 10_000),
        Err(TerminalError::RetentionLimit)
    ));
    assert!(matches!(
        TerminalModel::new_session(identity(), 4, 20, 10_001),
        Err(TerminalError::RetentionLimit)
    ));
    let mut invalid = identity();
    invalid.generation = 0;
    assert!(matches!(
        TerminalModel::new_session(invalid, 4, 20, 8),
        Err(TerminalError::Contract(ContractError::InvalidIdentity))
    ));
}

#[test]
fn resize_budget_remembers_wider_retained_history() {
    let mut terminal = TerminalModel::new_session(identity(), 1, 400, 4_900).unwrap();
    terminal.resize(1, 1).unwrap();
    assert_eq!(terminal.resize(200, 1), Err(TerminalError::RetentionLimit));
    assert_eq!(terminal.resize(1, 0), Err(TerminalError::InvalidDimensions));
    assert_eq!(terminal.screen().size(), (1, 1));
}
