# A03 source-to-replacement map (R02)

Source base: committed `wilde/theme` `1db4ef38577ec932f97067b7391625b7be599884`. Native base: `ce470be8c01a54f68125e4de111cce312cf6ffd6`. Source reads were confined to the isolated worktree's `source-reference`. The old working checkout has 47 unrelated changes reported by A1; these were not inspected or imported. Committed snapshot is authoritative for this reconnaissance; uncommitted domain drift remains OPEN.

| Source / observed behavior | Native pilot / evidence | Status |
| --- | --- | --- |
| `src/shared/wilde/domain.ts`: UUID record IDs; runtime target owner differs from business client owner | `model.rs`: validated UUID `RecordId`, client business owner label, `WorkspaceTarget.runtime_owner_id` | Narrow native pilot, NOT RUN |
| Domain target identity: owner + host + kind + stable ID; locator is presentation/location | `store.rs`: same identity tuple serialized as unique SQLite key; folder and SSH target fixtures | Implemented, NOT RUN |
| Business project separate from repository | Project references a client only; no runtime/repository/process dependency | Implemented, NOT RUN |
| `database.ts`: one synchronous write authority, BEGIN IMMEDIATE, rollback, bounded snapshot | `Store` mutable owner, immediate write transactions, deferred FK, bounded reads and exports | Implemented, NOT RUN; worker/thread hosting OPEN |
| `mutations.ts`: entity revision increments; stale edits reject; types cannot change | Native entity + store revisions, type guard, atomic rollback | Implemented, NOT RUN |
| `record-integrity.ts`: correct client/project types and project/client consistency | SQLite FK plus transaction-wide relationship/type checks | Core subset implemented, NOT RUN |
| Active assignment target uniqueness | Native pilot assignment target unique; no archive support | Core uniqueness implemented, archives OPEN |
| `database.ts`: same request ID with changed digest rejects | Persisted canonical typed payload; exact original Snapshot replay | Native v1 explicit change: legacy replay returns current snapshot; native returns original outcome. A1 accepted API; NOT RUN |
| `database.test.ts`: reopen, relationships, revisions, retries, backup imports | `tests/sqlite.rs`: real SQLite acceptance harness + native fixture | Runnable, NOT RUN |
| Preview validation rolls back; tokens hold exact payload, revision, five-minute expiry; max 20 tokens | `legacy-expectations-v1.json`: synthetic scenario specifications only; direct assignment edits/project moves return `NeedsPreview` | OPEN, no preview API or parity claim |
| `database.ts:applyPreview`: merge identical IDs, reject changed IDs, all relationship checks before commit | Native v1 synthetic merge only | Implemented, NOT RUN; legacy-format parity OPEN |
| Replace backup, pre-import recovery snapshot, consistent WAL backup, digest, history/activity/checkpoints | Referenced `backup.ts`, migrations, export and worker files are absent from provided snapshot | OPEN; no inferred migration/backup behavior |
| Contacts/status/tags/notes/links, tasks, knowledge, workflows, executions, immutable outcomes, archive/delete/project move expansion | Not present in this narrow native schema | OPEN; not complete CRM |
| n8n ingestion / secrets / provider transport | No provider calls or credentials; native fixtures only | OPEN, no integration writes |

`native-pilot-v1.json` is a native-format synthetic fixture. `legacy-expectations-v1.json` is a source-derived scenario map, not an executed differential harness. Export/import equality checks only native records; it does not prove legacy parity.

Acceptance criteria for R09: synthetic client -> delivery project -> folder and worktree assignments persists across SQLite reopen; invalid relationships roll back all writes; store/entity conflicts reject; identical request replay preserves original outcome even after later writes/reopen; changed request payload rejects; malformed/unsupported/conflicting imports preserve all prior state; unknown SQLite databases stay unchanged. Tests written for all of these, execution reserved for A1.

Implementation choices: one crate avoids a second parallel storage abstraction. JSON bodies preserve the typed native record shape, with indexed relationship columns and unique target key. Foreign runtime targets retain their IDs and remain unresolved for presentation; they trigger no process or filesystem action. An explicit absolute `*.wilde-dev.sqlite` filename and pilot SQLite application ID/user version are required for reopening. Unknown databases are not migrated. No default/profile discovery exists.

Bounds: 10000 records, 1000 save mutations, 8 MiB JSON payload/result, 10000 request IDs and 64 MiB aggregate durable payload/result log. Capacity errors roll back; no pruning that could invalidate retries. Busy timeout is one second; no automatic retry loop. ID strings preserve input case; typed serialization canonicalizes whitespace/field order for duplicate payload comparison. Import is merge only and preserves record revisions; each successful command advances native store revision once, including identical-record merge with a new request ID.
