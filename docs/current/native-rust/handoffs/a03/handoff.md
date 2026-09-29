# A03 task handoff

Task / role: A03 domain/storage, R02 source map + R09 first executable SQLite pilot.
Actual model / runner / session: user-selected GPT-6.1-Sol High per coordinator assignment; Codex subagent. Package Opus label overridden by user selection.
Worktree / branch / exact HEAD: `F:\wr\a03`, `rust-v2/a03-domain`; commit recorded in coordinator receipt after scoped commit.
Base contract version: existing `wilde-contracts` protocol major 1; business types remain private native pilot v1. A1 acknowledged Store API and pinned dependencies via coordination message.
Status: IMPLEMENTED; REAL SQLITE TESTS NOT RUN BY A3. Formatting completed. Not integrated.

## Result in plain English

A small business store now has runnable Rust code and real SQLite acceptance tests. It connects synthetic clients to delivery projects and folder/worktree assignments, keeps business owner and runtime owner separate, and never starts a process. Transaction failures preserve earlier data; persisted request receipts allow safe retries and replay the original result. This is the first domain/storage slice, not complete CRM parity.

## Implementation

Owned files: `crates/wilde-domain/{Cargo.toml,src,tests,examples}`, `tests/fixtures/domain/{native-pilot-v1.json,legacy-expectations-v1.json}`, this handoff and `docs/current/native-rust/recon/a03/source-map.md`.

Dependencies proposed and accepted: `rusqlite = =0.31.0` bundled, workspace serde `=1.0.228` derive and serde_json `=1.0.149`. A1 owns workspace registration and lock resolution; A3 did not touch root Cargo files or run Cargo/build/install.

API for A4: `Store::in_memory(runtime_owner_id)` for synthetic memory state; `Store::open_dev(explicit_path, runtime_owner_id)` for isolated development state; `snapshot`, `apply(Request)`, `export_json`, `import_json(request_id, expected_revision, content)`. These are blocking functions to host on a service thread, not the UI thread. Returned `Snapshot` has native store revision, runtime owner and sorted typed records. It contains no widget, PTY, secret or provider transport. No shared business contract is silently introduced.

A2 boundary: assignments are persisted metadata only. `WorkspaceTarget` contains folder/worktree kind, runtime owner, execution host, stable ID, locator and name. A business project has a client relationship, not an implicit repository/session. Foreign target owners retain their identity and should appear unresolved until the host/runtime resolves them; no local fallback.

Safety: path must be absolute and end `.wilde-dev.sqlite`; reopening requires exact application ID `1464099891`, schema version 1 and matching runtime owner. Unknown nonempty DBs are rejected before schema/WAL changes. In-memory opening has no filesystem fallback. No real client DB, production profile, credentials, integration or GUI was accessed.

## Evidence

`rustfmt --edition 2024` was run only on five A3 Rust source/test/example files and exited 0. This establishes formatting/parser acceptance, not compilation. Both fixture files parsed successfully with PowerShell `ConvertFrom-Json`; native fixture contains four records and legacy scenario map ten cases. Scoped staged `git diff --cached --check` exited 0.

Real SQLite acceptance tests: `crates/wilde-domain/tests/sqlite.rs`, 12 tests, NOT RUN. Coverage: persist/reopen; exact original receipt after later writes/reopen; store and entity stale revisions; two connections; failed relationship rollback + same-ID safe retry; cross-client/duplicate target rejection; confirmation-dependent edits blocked; native export/import equality + foreign runtime identity; malformed/schema/conflicting atomic imports; missing-parent rollback; unknown DB remains unchanged; input/path bounds.

Coordinator execution after workspace/lock registration: set `ORCA_BACKGROUND_LAUNCH=1`, `CARGO_BUILD_JOBS=2`, `CARGO_TARGET_DIR` to approved `F:\wr-build` output, and `WILDE_DOMAIN_TEST_ROOT` to an isolated absolute F scratch root. Then `cargo test --offline -p wilde-domain`. No build lease granted to A3. Every test uses a unique owned scratch directory and removes only its three known SQLite filenames plus the empty owned directory.

Fixture v1: native JSON and legacy scenario map contain synthetic identities only. `cargo run --offline -p wilde-domain --example synthetic_workload -- 100` generates 400 native synthetic records on stdout; 1..2500 clients accepted (4 records each). Generator NOT RUN, no timing or speedup claims.

## Review and integration

Independent reviewer: pending A1/coordinator. Local owned-path commit only; no GitHub/PR/external publication. Source drift and missing legacy source modules recorded in source map. SQLite/WAL is used for pilot persistence; production backups, migration integrity and full legacy import/export remain OPEN. Preview/commit/expiry is OPEN; assignment updates/project moves fail `NeedsPreview` rather than bypass confirmation. Archive/delete, history/activity, knowledge/tasks/evidence/n8n remain OPEN. SQLite busy failures have a bounded timeout; callers may retry the exact request, but no runtime worker integration exists.

## Runtime and cleanup

A3 launched no application, build, test runner, provider or GUI. Formatting command completed; no background A3 process retained. No database files created by A3. Only owned source/docs/fixtures written. No cleanup of old worktrees/profile/runtime attempted.

## Next

A1: review/register crate, resolve pinned cached lock, execute real SQLite tests under granted build slot, record results and integration SHA. A4/A2: consume only after explicit contract promotion/acknowledgement. Next domain increment needs captured legacy backup/migration/worker/command sources and a real preview implementation before confirmation-dependent workflows become available. No Shae decision required for the current reversible synthetic slice.
