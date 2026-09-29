# 03 — Six agents without six conflicting codebases

## Roles and review partners

| ID | Preferred model | Owns | Independent review |
|---|---|---|---|
| A1 | Astra | architecture, protocols, workspace composition, integration | A6 reviews its code and benchmark decisions |
| A2 | Opus 5.5 | runtime/PTY transport, processes, host Git/SSH/files | A5 terminal contract; A6 OS/process safety |
| A3 | Opus 5.5 | domain/storage, business API and n8n records | A1 data semantics; A4 consumer contract |
| A4 | Opus 5.5 | native shell and business UI | A5 input/rendering; A6 embedded surface behavior |
| A5 | Opus 5.5 | native terminal/editor/diff/explorer/previews | A2 stream correctness; A4 interaction contract |
| A6 | Astra | browser, OS integrations, resource policy, performance/release | A1 security/architecture; A2 lifecycle |

A1 is the primary Codex session. Launch at most five additional roles. No model superiority or benchmark claims are prerequisites. Record actual runner/model access at startup. An unavailable preferred worker is a routing blocker, not permission to fabricate a worker or to silently change its identity. See official agent configuration docs for supported installed-version settings; this package intentionally does not overwrite `.codex/config.toml`. [W1]

## Avoiding overloaded tracks

A2's initial priority is local process/PTY lifecycle, not simultaneous rewrites of Git, SSH and WSL. A5 first proves a native terminal surface and minimal save/undo editor, not complete Monaco equivalence. A6 first proves embedding + performance instrumentation, then browser behavior, then integrations/packaging. A3 contributes fake adapters and fixtures while UI contracts settle. A1 must schedule each track as small tasks from the backlog, not one enormous assignment.

## Shared files

A1 integrates `native-rust/Cargo.toml`, `Cargo.lock`, root composition, protocol schemas and top-level CI. Workers can propose dependency changes in their branch, but must not independently resolve conflicting integrated lockfiles. Source owners maintain their module tests. Only the assigned owner writes a file; consumers send patch suggestions/contract requests. Update the ownership map before moving responsibility.

## Branches and worktrees

A1 creates `rust-v2/integration` from the approved exact feature-branch base. Worker branches start from the integrated contract commit, not a guessed default branch. Proposed worker branches are `rust-v2/a02-runtime`, `a03-domain`, `a04-ui`, `a05-ide`, and `a06-platform`. Use short sibling worktree paths, never nested paths inside the main checkout. Worktree changes do not automatically appear in another agent's worktree; share immutable commits, tests and version references.

GitHub is the permanent record. Each task records owner, dependency IDs, branch, path, actual runner/model, interface version, acceptance checks and evidence. When writes are unavailable/unapproved, create local issue/PR drafts and explicitly mark `pending-publication`. Do not claim an issue/PR URL that was not created.

## Message protocol

Send: `task`, `from/to`, `contract version`, `commit`, `changed paths`, `impact`, `requested action`, `blocker`.

Contract change flow: producer proposal → consumer acknowledgement → A1 decision/version → producer implementation → consumer tests → integration. Default acknowledgement is not inferred from silence. Escalate disputed product behavior/security/destructive operations to Shae; technical choices with reversible experiments can continue.

## Resource scheduling

Six roles may exist concurrently, but start with three active implementation workers plus A1 and one heavy build/GUI/performance job per physical machine. Read-only reviews may continue while a heavy runner is busy. Begin with `CARGO_BUILD_JOBS=2`; record the host and change limits only after measurement.

Use one build slot protected by a coordinator-managed lease/lock. An abandoned lock is not removed by age alone; verify owner/process death. Shared source download caches are acceptable. Keep separate target directories for concurrently active branches, outside removable worktrees. Never run competing Cargo builds against one mutable target directory without serialization. Building on a remote PC reduces local pressure, not total system work; measure both.

## Integration and resumability

A1 is the only integration authority, but cannot self-approve its own substantive changes. Small reviewable commits/PRs land after owner tests, independent review and integration tests. Do not merge to the production branch or release without authorization. Every session writes a handoff containing exact commit, passed/failed/not-run checks, next tasks and blockers. Include a brief plain-English explanation of one significant change.
