CREATE TABLE metadata (
    singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
    runtime_owner_id TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK(revision >= 0)
);
CREATE TABLE records (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL CHECK(kind IN ('client', 'project', 'assignment')),
    client_id TEXT REFERENCES records(id) DEFERRABLE INITIALLY DEFERRED,
    project_id TEXT REFERENCES records(id) DEFERRABLE INITIALLY DEFERRED,
    revision INTEGER NOT NULL CHECK(revision > 0),
    body TEXT NOT NULL,
    target_key TEXT UNIQUE
);
CREATE TABLE requests (
    id TEXT PRIMARY KEY,
    payload TEXT NOT NULL,
    result TEXT NOT NULL
);
PRAGMA application_id = 1464099891;
PRAGMA user_version = 1;
