use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
#[test]
fn synthetic_client_workspace_sqlite_reopen_and_real_native_pty_share_one_slice() {
    let root = std::env::var_os("WILDE_DOMAIN_TEST_ROOT")
        .map(PathBuf::from)
        .expect("explicit test scratch root required");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = root.join(format!("integrated-pilot-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let database = directory.join("journey.wilde-dev.sqlite");
    let result = wilde_desktop::run_headless_pilot(&database);
    for suffix in ["", "-wal", "-shm"] {
        let file = directory.join(format!("journey.wilde-dev.sqlite{suffix}"));
        if file.exists() {
            std::fs::remove_file(file).unwrap();
        }
    }
    std::fs::remove_dir(&directory).unwrap();
    let evidence = result.unwrap();
    assert_eq!(evidence.records, 3);
    assert!(evidence.database_reopened && evidence.duplicate_request_replayed);
    assert!(evidence.terminal_ready && evidence.terminal_pong && evidence.terminal_finished);
    assert!(evidence.reader_joined && evidence.lifecycle_joined && evidence.fixture_cwd_removed);
}
