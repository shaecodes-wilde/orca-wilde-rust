use super::*;

fn identity() -> SessionIdentity {
    SessionIdentity {
        session_id: "alpha-stream".into(),
        execution_host_id: "synthetic-host".into(),
        workspace_id: "synthetic-workspace".into(),
        generation: 1,
    }
}

fn model() -> TerminalModel {
    TerminalModel::new_session(identity(), 4, 40, 8).unwrap()
}

fn feed(terminal: &mut TerminalModel, bytes: &[u8]) -> Result<(), TerminalError> {
    terminal.accept_frame(&TerminalFrame {
        identity: identity(),
        sequence: terminal.next_sequence(),
        payload: bytes.to_vec(),
    })
}

#[test]
fn sustained_sixteen_mib_keeps_final_unicode_sentinel_and_bounded_history() {
    let mut terminal = model();
    let mut block = [b'x'; 4096];
    block[4094..].copy_from_slice(b"\r\n");
    for _ in 0..4096 {
        feed(&mut terminal, &block).unwrap();
    }
    assert_eq!(terminal.admitted_bytes(), 16 * 1_048_576);
    assert_eq!(terminal.next_sequence(), 4096);
    assert!(!terminal.resource_exhausted());
    for byte in "\r\n\x1b[32mALPHA_FINAL_界🙂e\u{301}\x1b[0m".as_bytes() {
        feed(&mut terminal, &[*byte]).unwrap();
    }
    assert!(terminal.contents().contains("ALPHA_FINAL_界🙂e\u{301}"));
    let row = (0..4)
        .find(|&row| terminal.screen().cell(row, 0).unwrap().contents() == "A")
        .unwrap();
    assert_eq!(
        terminal.screen().cell(row, 0).unwrap().fgcolor(),
        vt100::Color::Idx(2)
    );
    assert!(terminal.screen().cell(row, 12).unwrap().is_wide());
    assert!(terminal.screen().cell(row, 14).unwrap().is_wide());
    terminal.set_scrollback(usize::MAX);
    assert_eq!(terminal.screen().scrollback(), 8);
    assert_eq!(terminal.control_budget.pending_bytes(), 0);
}

#[test]
fn repeated_completed_osc_does_not_accumulate_a_lifetime_budget() {
    let mut terminal = model();
    let mut block = vec![b'x'; 4096];
    block[..4].copy_from_slice(b"\x1b]0;");
    block[4095] = 7;
    for _ in 0..4096 {
        feed(&mut terminal, &block).unwrap();
    }
    feed(&mut terminal, b"FINAL").unwrap();
    assert_eq!(terminal.contents(), "FINAL");
    assert_eq!(terminal.admitted_bytes(), 16 * 1_048_576 + 5);
    assert!(!terminal.resource_exhausted());
}

#[test]
fn exact_control_limit_accepts_terminator_and_next_output() {
    let mut terminal = model();
    let mut osc = vec![b'x'; MAX_CONTROL_SEQUENCE_BYTES];
    osc[..4].copy_from_slice(b"\x1b]0;");
    *osc.last_mut().unwrap() = 7;
    for byte in osc {
        feed(&mut terminal, &[byte]).unwrap();
    }
    feed(&mut terminal, b"AFTER").unwrap();
    assert_eq!(terminal.contents(), "AFTER");
    assert_eq!(terminal.control_budget.pending_bytes(), 0);
}

#[test]
fn rejected_control_frame_keeps_prefix_and_cursor_atomic_and_latches() {
    let mut terminal = model();
    feed(&mut terminal, b"safe\x1b]0;").unwrap();
    let before_sequence = terminal.next_sequence();
    let before_bytes = terminal.admitted_bytes();
    let before_pending = terminal.control_budget.pending_bytes();
    let mut rejected = b"\x07\x1b[2J\x1b[H\x1b]0;".to_vec();
    rejected.extend(std::iter::repeat_n(b'x', MAX_CONTROL_SEQUENCE_BYTES));
    assert_eq!(
        feed(&mut terminal, &rejected),
        Err(TerminalError::ControlSequenceLimit)
    );
    assert_eq!(terminal.contents(), "safe");
    assert_eq!(terminal.next_sequence(), before_sequence);
    assert_eq!(terminal.admitted_bytes(), before_bytes);
    assert_eq!(terminal.control_budget.pending_bytes(), before_pending);
    assert!(terminal.resource_exhausted());
    assert!(!terminal.snapshot_required());
    assert_eq!(
        feed(&mut terminal, b"\x07later"),
        Err(TerminalError::ControlSequenceLimit)
    );
    assert_eq!(
        feed(&mut terminal, b""),
        Err(TerminalError::ControlSequenceLimit)
    );
}

#[test]
fn hostile_control_forms_are_bounded_across_frames() {
    for prefix in [
        &b"\x1b]0;"[..],
        &b"\x1bPq"[..],
        &b"\x1b[1"[..],
        &b"\x1b "[..],
        &b"\x1bX"[..],
        &b"\x1b^"[..],
        &b"\x1b_"[..],
        &b"\x1b[ 1"[..],
        &b"\x1bP 1"[..],
    ] {
        let mut terminal = model();
        feed(&mut terminal, b"safe").unwrap();
        feed(&mut terminal, prefix).unwrap();
        let fill = if prefix == b"\x1b " {
            b' '
        } else if prefix.starts_with(b"\x1b[") {
            b'1'
        } else {
            b'x'
        };
        let remaining = MAX_CONTROL_SEQUENCE_BYTES - prefix.len();
        feed(&mut terminal, &vec![fill; remaining]).unwrap();
        let sequence = terminal.next_sequence();
        assert_eq!(
            feed(&mut terminal, &[fill]),
            Err(TerminalError::ControlSequenceLimit)
        );
        assert_eq!(terminal.next_sequence(), sequence);
        assert_eq!(terminal.contents(), "safe");
        assert_eq!(
            terminal.control_budget.pending_bytes(),
            MAX_CONTROL_SEQUENCE_BYTES
        );
    }
}

#[test]
fn cancelled_and_terminated_controls_allow_long_following_output() {
    for ending in [&b"\x07"[..], &b"\x1b\\"[..], &b"\x18"[..], &b"\x1a"[..]] {
        let mut terminal = model();
        for byte in b"\x1b]0;title" {
            feed(&mut terminal, &[*byte]).unwrap();
        }
        for byte in ending {
            feed(&mut terminal, &[*byte]).unwrap();
        }
        feed(&mut terminal, &vec![b'x'; 65536]).unwrap();
        feed(&mut terminal, b"\r\nAFTER").unwrap();
        assert!(terminal.contents().contains("AFTER"));
        assert!(!terminal.resource_exhausted());
    }
}

#[test]
fn raw_c1_st_ends_dcs_but_not_osc() {
    let mut dcs = model();
    for byte in b"\x1bPqignored\x9cAFTER" {
        feed(&mut dcs, &[*byte]).unwrap();
    }
    assert_eq!(dcs.contents(), "AFTER");
    let mut osc = model();
    for byte in b"\x1b]0;ignored\x9cHIDDEN" {
        feed(&mut osc, &[*byte]).unwrap();
    }
    assert_eq!(osc.contents(), "");
    feed(&mut osc, b"\x07AFTER").unwrap();
    assert_eq!(osc.contents(), "AFTER");
}

#[test]
fn split_controls_match_uninterrupted_mature_parser_cells_and_modes() {
    let transcript = concat!(
        "main界🙂e\u{301}\r\n",
        "\x1b[31mred\x1b[0m\x1b]0;title\x1b\\",
        "\x1bP1;2qignored\x1b\\",
        "\x1bXignored\x1b\\\x1b^ignored\x1b\\\x1b_ignored\x1b\\",
        "\x1b[?1049h\x1b[2J\x1b[HALT\x1b[?1049l",
        "\x1b[ 1ignoredm\x1bP 1ignored\x1b\\",
        "\x1b(]printable\x1b]2;cancel\x18DONE"
    )
    .as_bytes();
    let mut reference = vt100::Parser::new(4, 40, 8);
    reference.process(transcript);
    for chunk_size in [1, 2, 3, 7, 64] {
        let mut terminal = model();
        for chunk in transcript.chunks(chunk_size) {
            feed(&mut terminal, chunk).unwrap();
        }
        assert_eq!(
            terminal.screen().cursor_position(),
            reference.screen().cursor_position()
        );
        assert_eq!(
            terminal.screen().alternate_screen(),
            reference.screen().alternate_screen()
        );
        for row in 0..4 {
            for col in 0..40 {
                assert_eq!(
                    terminal.screen().cell(row, col),
                    reference.screen().cell(row, col)
                );
            }
        }
    }
}

#[test]
fn utf8_gap_and_osc_replay_keep_last_trusted_state_without_reset() {
    for prefix in [&b"safe\xe7"[..], &b"safe\x1b]0;title"[..]] {
        let mut terminal = model();
        feed(&mut terminal, prefix).unwrap();
        let sequence = terminal.next_sequence();
        let admitted = terminal.admitted_bytes();
        let pending = terminal.control_budget.pending_bytes();
        assert_eq!(
            terminal.accept_frame(&TerminalFrame {
                identity: identity(),
                sequence: sequence + 1,
                payload: b"\x07\x1b[2Jbad".to_vec(),
            }),
            Err(TerminalError::Contract(ContractError::SnapshotRequired))
        );
        assert_eq!(terminal.contents(), "safe");
        assert_eq!(terminal.next_sequence(), sequence);
        assert_eq!(terminal.admitted_bytes(), admitted);
        assert_eq!(terminal.control_budget.pending_bytes(), pending);
        assert_eq!(
            feed(&mut terminal, b"\x07late"),
            Err(TerminalError::Contract(ContractError::SnapshotRequired))
        );
    }
}

#[test]
fn repeated_escape_is_one_bounded_incomplete_sequence() {
    let mut terminal = model();
    feed(&mut terminal, b"safe").unwrap();
    for _ in 0..MAX_CONTROL_SEQUENCE_BYTES {
        feed(&mut terminal, b"\x1b").unwrap();
    }
    let sequence = terminal.next_sequence();
    assert_eq!(
        feed(&mut terminal, b"\x1b"),
        Err(TerminalError::ControlSequenceLimit)
    );
    assert_eq!(terminal.next_sequence(), sequence);
    assert_eq!(terminal.contents(), "safe");
}

#[test]
fn escape_cancels_osc_then_opens_a_fresh_bounded_osc() {
    let mut terminal = model();
    feed(&mut terminal, b"\x1b]0;old").unwrap();
    feed(&mut terminal, b"\x1b").unwrap();
    for _ in 0..4 {
        feed(&mut terminal, b"\x1b").unwrap();
    }
    feed(&mut terminal, b"]0;new\x07AFTER").unwrap();
    assert_eq!(terminal.contents(), "AFTER");
    assert_eq!(terminal.control_budget.pending_bytes(), 0);
    feed(&mut terminal, &vec![b'x'; 65536]).unwrap();
    assert!(!terminal.resource_exhausted());
}
