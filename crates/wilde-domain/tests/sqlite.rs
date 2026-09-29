use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
use wilde_domain::{Error, Export, Record, RecordId, Request, Store, TargetKind, WorkspaceTarget};

const FIXTURE: &str = include_str!("../../../tests/fixtures/domain/native-pilot-v1.json");
fn id(n: u32) -> RecordId {
    RecordId::new(format!("00000000-0000-4000-8000-{n:012}")).unwrap()
}
fn client(n: u32) -> Record {
    Record::Client {
        id: id(n),
        revision: 0,
        name: format!("Synthetic client {n}"),
        owner: "Synthetic business owner".into(),
    }
}
fn project(n: u32, client: u32) -> Record {
    Record::Project {
        id: id(n),
        revision: 0,
        client_id: id(client),
        title: "No repository needed".into(),
    }
}
fn assignment(n: u32, client: u32, project: u32, kind: TargetKind) -> Record {
    Record::Assignment {
        id: id(n),
        revision: 0,
        client_id: id(client),
        project_id: Some(id(project)),
        target: WorkspaceTarget {
            kind,
            runtime_owner_id: "synthetic-runtime".into(),
            host_id: "ssh:synthetic-host".into(),
            stable_id: format!("stable-{n}"),
            locator: "synthetic-locator".into(),
            name: "Synthetic workspace".into(),
        },
        reason: "Synthetic delivery".into(),
    }
}
fn request(name: &str, revision: u64, records: Vec<Record>) -> Request {
    Request {
        request_id: name.into(),
        expected_revision: revision,
        records,
    }
}
fn export(records: Vec<Record>) -> String {
    serde_json::to_string(&Export {
        format: "wilde-native-synthetic".into(),
        schema_version: 1,
        records,
    })
    .unwrap()
}

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch {
    directory: PathBuf,
}
impl Scratch {
    fn new() -> Self {
        let root = std::env::var_os("WILDE_DOMAIN_TEST_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        assert!(root.is_absolute(), "test scratch root must be absolute");
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = root.join(format!(
            "wilde-domain-{}-{timestamp}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&directory).unwrap();
        Self { directory }
    }
    fn path(&self) -> PathBuf {
        self.directory.join("synthetic.wilde-dev.sqlite")
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        for name in [
            "synthetic.wilde-dev.sqlite",
            "synthetic.wilde-dev.sqlite-wal",
            "synthetic.wilde-dev.sqlite-shm",
        ] {
            let file = self.directory.join(name);
            if file.exists() {
                std::fs::remove_file(file).unwrap();
            }
        }
        std::fs::remove_dir(&self.directory).unwrap();
    }
}

#[test]
fn client_project_folder_and_worktree_survive_sqlite_reopen_without_launching() {
    let scratch = Scratch::new();
    let mut store = Store::open_dev(&scratch.path(), "synthetic-runtime").unwrap();
    let saved = store
        .apply(request(
            "journey",
            0,
            vec![
                client(1),
                project(2, 1),
                assignment(3, 1, 2, TargetKind::Folder),
                assignment(4, 1, 2, TargetKind::Worktree),
            ],
        ))
        .unwrap();
    assert_eq!(saved.records.len(), 4);
    assert!(saved.records.iter().all(|r| r.revision() == 1));
    drop(store);
    let mut reopened = Store::open_dev(&scratch.path(), "synthetic-runtime").unwrap();
    assert_eq!(reopened.snapshot().unwrap(), saved);
    drop(reopened);
    assert!(matches!(
        Store::open_dev(&scratch.path(), "different-runtime"),
        Err(Error::WrongDatabase)
    ));
}

#[test]
fn retry_returns_original_outcome_after_later_writes_and_reopen() {
    let scratch = Scratch::new();
    let mut store = Store::open_dev(&scratch.path(), "synthetic-runtime").unwrap();
    let first = request("durable-request", 0, vec![client(1)]);
    let outcome = store.apply(first.clone()).unwrap();
    store.apply(request("later", 1, vec![client(2)])).unwrap();
    drop(store);
    let mut reopened = Store::open_dev(&scratch.path(), "synthetic-runtime").unwrap();
    assert_eq!(reopened.apply(first.clone()).unwrap(), outcome);
    assert_eq!(reopened.snapshot().unwrap().records.len(), 2);
    let changed = request("durable-request", 0, vec![client(3)]);
    assert!(matches!(
        reopened.apply(changed),
        Err(Error::RequestIdConflict)
    ));
    assert_eq!(reopened.snapshot().unwrap().revision, 2);
}

#[test]
fn stale_store_and_entity_revisions_preserve_saved_state() {
    let mut store = Store::in_memory("synthetic-runtime").unwrap();
    let saved = store.apply(request("first", 0, vec![client(1)])).unwrap();
    assert!(matches!(
        store.apply(request("stale-store", 0, vec![client(2)])),
        Err(Error::RevisionConflict)
    ));
    assert!(matches!(
        store.apply(request("stale-entity", 1, vec![client(1)])),
        Err(Error::RevisionConflict)
    ));
    assert_eq!(store.snapshot().unwrap(), saved);
}

#[test]
fn two_connections_cannot_overwrite_a_stale_snapshot() {
    let scratch = Scratch::new();
    let mut first = Store::open_dev(&scratch.path(), "synthetic-runtime").unwrap();
    let mut second = Store::open_dev(&scratch.path(), "synthetic-runtime").unwrap();
    let stale = second.snapshot().unwrap();
    let saved = first.apply(request("first", 0, vec![client(1)])).unwrap();
    assert!(matches!(
        second.apply(request("second", stale.revision, vec![client(2)])),
        Err(Error::RevisionConflict)
    ));
    assert_eq!(second.snapshot().unwrap(), saved);
}

#[test]
fn invalid_relationship_rolls_back_every_record_and_does_not_consume_request_id() {
    let mut store = Store::in_memory("synthetic-runtime").unwrap();
    let empty = store.snapshot().unwrap();
    assert!(
        store
            .apply(request("retryable", 0, vec![client(1), project(2, 99)]))
            .is_err()
    );
    assert_eq!(store.snapshot().unwrap(), empty);
    let saved = store
        .apply(request("retryable", 0, vec![client(1), project(2, 1)]))
        .unwrap();
    assert_eq!(saved.revision, 1);
    assert_eq!(saved.records.len(), 2);
}

#[test]
fn cross_client_and_duplicate_workspace_assignments_roll_back() {
    let mut store = Store::in_memory("synthetic-runtime").unwrap();
    let saved = store
        .apply(request(
            "seed",
            0,
            vec![client(1), client(5), project(2, 1)],
        ))
        .unwrap();
    assert!(
        store
            .apply(request(
                "cross-client",
                1,
                vec![assignment(3, 5, 2, TargetKind::Folder)]
            ))
            .is_err()
    );
    let a = assignment(3, 1, 2, TargetKind::Folder);
    let mut b = a.clone();
    if let Record::Assignment {
        id: record_id,
        target,
        ..
    } = &mut b
    {
        *record_id = id(4);
        target.locator = "new-locator-same-identity".into();
    }
    assert!(
        store
            .apply(request("duplicate-target", 1, vec![a, b]))
            .is_err()
    );
    assert_eq!(store.snapshot().unwrap(), saved);
}

#[test]
fn consequential_edits_require_a_preview_implementation() {
    let mut store = Store::in_memory("synthetic-runtime").unwrap();
    let saved = store
        .apply(request(
            "seed",
            0,
            vec![
                client(1),
                client(5),
                project(2, 1),
                assignment(3, 1, 2, TargetKind::Folder),
            ],
        ))
        .unwrap();
    let mut moved = project(2, 5);
    if let Record::Project { revision, .. } = &mut moved {
        *revision = 1;
    }
    assert!(matches!(
        store.apply(request("move", 1, vec![moved])),
        Err(Error::NeedsPreview)
    ));
    let mut edited = saved
        .records
        .iter()
        .find(|r| r.id() == &id(3))
        .unwrap()
        .clone();
    if let Record::Assignment { reason, .. } = &mut edited {
        *reason = "Changed".into();
    }
    assert!(matches!(
        store.apply(request("reassign", 1, vec![edited])),
        Err(Error::NeedsPreview)
    ));
    assert_eq!(store.snapshot().unwrap(), saved);
}

#[test]
fn native_synthetic_export_import_preserves_records_and_foreign_runtime_identity() {
    let mut source = Store::in_memory("synthetic-runtime-a").unwrap();
    let original = source.import_json("seed", 0, FIXTURE).unwrap();
    let exported = source.export_json().unwrap();
    let mut destination = Store::in_memory("separate-native-runtime").unwrap();
    let mut unordered: Export = serde_json::from_str(&exported).unwrap();
    unordered.records.reverse();
    let unordered_json = serde_json::to_string(&unordered).unwrap();
    let restored = destination
        .import_json("restore", 0, &unordered_json)
        .unwrap();
    assert_eq!(restored.records, original.records);
    assert_eq!(restored.runtime_owner_id, "separate-native-runtime");
    assert_eq!(source.snapshot().unwrap(), original);
    assert!(restored.records.iter().any(|r| matches!(r, Record::Assignment { target, .. } if target.runtime_owner_id == "synthetic-runtime-b")));
    let repeated = destination
        .import_json("merge-again", 1, &exported)
        .unwrap();
    assert_eq!(repeated.records, original.records);
    assert_eq!(
        destination
            .import_json("restore", 0, &unordered_json)
            .unwrap(),
        restored
    );
}

#[test]
fn malformed_unsupported_and_conflicting_imports_leave_destination_unchanged() {
    let mut store = Store::in_memory("synthetic-runtime").unwrap();
    let saved = store.import_json("seed", 0, FIXTURE).unwrap();
    assert!(store.import_json("malformed", 1, "{broken-json").is_err());
    let unsupported = FIXTURE.replace("\"schema_version\": 1", "\"schema_version\": 2");
    assert!(matches!(
        store.import_json("unsupported", 1, &unsupported),
        Err(Error::UnsupportedSchema)
    ));
    let mut incoming = client(50);
    if let Record::Client { revision, .. } = &mut incoming {
        *revision = 1;
    }
    let mut conflict = saved.records[0].clone();
    if let Record::Client { name, .. } = &mut conflict {
        *name = "Conflicting content".into();
    }
    assert!(
        store
            .import_json("conflict", 1, &export(vec![incoming, conflict]))
            .is_err()
    );
    assert_eq!(store.snapshot().unwrap(), saved);
}

#[test]
fn missing_relationship_import_is_atomic_even_when_parent_is_written_first() {
    let mut store = Store::in_memory("synthetic-runtime").unwrap();
    let original = store.snapshot().unwrap();
    let broken = FIXTURE.replace(
        "00000000-0000-4000-8000-000000000002\",\"target",
        "00000000-0000-4000-8000-000000000099\",\"target",
    );
    assert!(store.import_json("broken", 0, &broken).is_err());
    assert_eq!(store.snapshot().unwrap(), original);
    assert_eq!(
        store
            .import_json("broken", 0, FIXTURE)
            .unwrap()
            .records
            .len(),
        4
    );
}

#[test]
fn unrelated_existing_database_is_rejected_without_schema_or_data_mutation() {
    let scratch = Scratch::new();
    let raw = rusqlite::Connection::open(scratch.path()).unwrap();
    raw.execute_batch("CREATE TABLE legacy_marker(value TEXT); INSERT INTO legacy_marker VALUES ('synthetic legacy sentinel')").unwrap();
    drop(raw);
    assert!(matches!(
        Store::open_dev(&scratch.path(), "synthetic-runtime"),
        Err(Error::WrongDatabase)
    ));
    let raw = rusqlite::Connection::open(scratch.path()).unwrap();
    let value: String = raw
        .query_row("SELECT value FROM legacy_marker", [], |r| r.get(0))
        .unwrap();
    assert_eq!(value, "synthetic legacy sentinel");
    let objects: usize = raw
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(objects, 1);
    let app_id: i64 = raw
        .query_row("PRAGMA application_id", [], |r| r.get(0))
        .unwrap();
    assert_eq!(app_id, 0);
}

#[test]
fn unmarked_relative_invalid_or_oversized_inputs_never_write() {
    assert!(matches!(
        Store::open_dev(Path::new("relative.wilde-dev.sqlite"), "synthetic-runtime"),
        Err(Error::WrongDatabase)
    ));
    let unmarked = std::env::temp_dir().join("business.sqlite");
    assert!(matches!(
        Store::open_dev(&unmarked, "synthetic-runtime"),
        Err(Error::WrongDatabase)
    ));
    assert!(RecordId::new("not-a-uuid").is_err());
    assert!(serde_json::from_str::<RecordId>("\"not-a-uuid\"").is_err());
    let mut store = Store::in_memory("synthetic-runtime").unwrap();
    let mut oversized = client(1);
    if let Record::Client { name, .. } = &mut oversized {
        *name = "x".repeat(241);
    }
    assert!(store.apply(request("invalid", 0, vec![oversized])).is_err());
    assert!(
        store
            .import_json(
                "oversized",
                0,
                &"x".repeat(wilde_domain::MAX_JSON_BYTES + 1)
            )
            .is_err()
    );
    assert_eq!(store.snapshot().unwrap().records.len(), 0);
    assert_eq!(store.snapshot().unwrap().revision, 0);
}
