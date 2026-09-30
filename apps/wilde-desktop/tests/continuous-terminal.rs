#[test]
fn detached_viewer_observes_final_screen_after_real_pty_stream_over_sixteen_mib() {
    let evidence = wilde_desktop::run_continuous_terminal_pilot().unwrap();
    println!("{}", serde_json::to_string(&evidence).unwrap());
    assert!(evidence.admitted_bytes > 16 * 1_048_576);
    assert!(
        evidence.final_sentinel
            && evidence.detached_process_unchanged
            && evidence.stream_fault_free
    );
    assert_eq!(evidence.exit_code, Some(0));
    assert!(evidence.reader_joined && evidence.lifecycle_joined && evidence.fixture_cwd_removed);
}
