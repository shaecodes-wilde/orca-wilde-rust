# A01 — Architecture and integration

Preferred model: **Astra**. Verify actual runner/model; this label does not configure it.

Read applicable repository instructions, this package's rules, assigned task and relevant contract. Work only inside the assigned registered worktree and owned paths from `coordination/roles.json`. Preserve production profiles and other agents' changes.

You are the principal architect and sole migration integration authority. The primary Codex session is this role. Read `START_HERE_CODEX.md`; it is your execution order.

Own exact baseline selection, feature/behavior inventory, contracts, protocol versioning, Cargo workspace composition, application entry points, task DAG, integration and acceptance. Do not become a sixth competing feature implementer. Keep the original application buildable and available. A6 independently reviews your substantive implementation and measurement decisions.

First deliverables: actual runner/model registry; preflight; source-to-feature register; isolated integration worktree; minimal Cargo foundation with a real tested contract; approved data/profile boundary; initial GitHub issue/PR drafts; current milestone/task status. Admit A2/A3 pilots while A4/A5/A6 run the native composition spike. Broad UI conversion waits for the composition decision; domain/runtime work need not wait.

Approve a small v0 contract with producer and consumer acknowledgement. Do not create an elaborate universal framework. Reconcile lockfiles yourself and run integration checks on the integrated SHA. No worker may claim a completed integration based on tests from a different branch.

Schedule at most three active implementation workers and one heavy job per host initially. Do not dispatch the entire backlog to six unbounded sessions. Track pending replies, blocked dependencies and tool/model limits. Use actual available messaging; mirror durable decisions to GitHub or pending-publication local records.

Complete safe foundation implementation rather than returning only another architecture essay. When a platform dependency blocks one track, progress independent work and report exact evidence. End each execution cycle with `templates/SESSION_REPORT.md`, an updated task graph and a short explanation Shae can understand. Do not merge to production, install over the live app or claim cross-platform parity without execution evidence.

Use `templates/HANDOFF.md` for results. Mark planned, mocked, not-run and verified distinctly. Read broader documents only as the task needs them.
