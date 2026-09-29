# A03 — Business domain and storage

Preferred model: **Opus 5.5**. Verify actual runner/model; this label does not configure it.

Read applicable repository instructions, this package's rules, assigned task and relevant contract. Work only inside the assigned registered worktree and owned paths from `coordination/roles.json`. Preserve production profiles and other agents' changes.

Own clients, contacts, projects, assignments, tasks, notes, activity, business commands, SQLite, migrations/backups and n8n data ingestion. The CRM already exists; inspect and migrate it rather than inventing an unrelated replacement.

First task: build synthetic fixtures for snapshot, preview/commit, stale revision, preview expiry, duplicate request IDs, import failure and relationship validation. Trace `BusinessDatabase`, its worker, shared commands/domain schema and migration tests. Keep the old semantics authoritative unless A1 records a versioned change.

Implement a small Rust service over an isolated test database, with typed IDs and command validation. Preserve legacy IDs and distinguish existing runtime `ownerId` from business owner categories. A project need not be a Git repository, and saving an assignment must not start processes. Prefer a single write authority and bounded query results. Database blocking work must not run on the UI thread.

Imports start from synthetic data; any later legacy migration reads a consistent backup into a separate native profile. Never open or migrate the live database by default. Test WAL/snapshot consistency, transaction rollback, foreign keys, revisions, backup integrity and idempotency. Do not simplify away confirmation/preview behavior for speed.

A4 owns presentation and A2 runtime sessions. Your API returns records/projections, not native widgets or PTYs. n8n ingestion uses a fake provider first and preserves current read-only integration intent; do not deploy workflows or modify client servers. A6 owns secrets/provider transport policy where shared, while you own business data semantics and retention.

Return differential fixtures, storage/schema decision, exact tests/commit, failure evidence, benchmark workload data generator, limitations and an A4-facing contract handoff. Do not label a basic client table complete CRM parity.

Use `templates/HANDOFF.md` for results. Mark planned, mocked, not-run and verified distinctly. Read broader documents only as the task needs them.
