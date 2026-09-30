# Implementation plan — Native Workspace Alpha

This is a scoped continuation of the existing migration, not a replacement architecture. The R-identifiers below refer to the current backlog. The `NWA-*` labels are proposed child work items; A1 must check for collisions and map them into the existing register without changing its status vocabulary. [S3]

## 1. Target and exclusions

**Primary functional-alpha host:** isolated Windows development environment. **Architecture gate:** actual Windows and Linux/Wayland composition/input/lifecycle evidence before broad UI conversion. macOS and broader Linux functional parity remain explicit open requirements, not implied by compilation. Linux/Wayland shell/terminal acceptance should be expanded when its runner exists; Windows results never stand in for it.

The alpha must deliver real local workspace provisioning, interactive shell and approved agent operation, native terminal/editor/review, independent session-host lifetime, a bounded embedded development browser, and measured resource behavior. Display a minimal client/project/workspace association using synthetic records so the existing business foundation is connected, but do not build a full CRM.

Not this milestone: full business import/backup parity, n8n collection, Spotify/OBS/voice integrations, SSH/WSL implementations, full LSP/editor/preview coverage, complete OAuth/media/CDP/Design Mode compatibility, old mobile/remote wire compatibility, installer/update migration or production cutover. Keep their entries and known architectural requirements. A small code change needed to preserve a future boundary is allowed; silently expanding these into parallel product builds is not.

## 2. Stage P0 — reconcile actual state and prepare execution

**NWA-01; A1; R00/R01/R02/R06/R07.**

Inspect both accessible checkouts without altering the legacy checkout. Record repo identities, actual branches/HEADs, dirty/untracked state, applicable instructions, source-reference provenance, existing worktrees/owners, test tools, OS/filesystem, disk headroom and build lease. Distinguish published reference behavior from local-only changes. The earlier report's dirty-file count is historical, not a current inventory. Do not read secret values to perform preflight. [S5]

The published Rust baseline reviewed for this package is `e68c9983d0849dc6798e38403567a17259d066b5`; the legacy reference is `1db4ef38577ec932f97067b7391625b7be599884`. These identify evidence, not instructions to reset. `roles.json` includes legacy-origin metadata and prior publication/session values: reconcile destination/current authorization separately without treating stale fields as present permission. [S1–S5]

Verify available model/effort, actual worker tools and path ownership. Historical `F:` paths apply only if the same checkout is present. A1 reuses the established target directory/lease mechanism after inspection; workers do not install dependencies or launch their own builds. Inspect existing `tools/verify.ps1` and guarded runner before deriving commands; do not create a second conflicting watchdog/build-lock framework.

Complete legacy source/test references for alpha-relevant parity rows, preserving all 40 existing rows. Add explicit child cases for agent launch/account isolation, attention state, orchestration/coordination and cleanup where broad rows conceal requirements. Mark scope as alpha-required, later-required or decision-pending; do not mark nonexistent functionality migrated. [S8]

**P0 exit:** current-state receipt, accepted ownership/contracts, a short dependency map, runner/permission inventory and proposed measurement protocol. No need for a second long strategy document.

## 3. Stage P1 — prove native composition, then commit to toolkit

**NWA-02; A4 with A5/A6; R03/R04/R05/R06/R12.**

Use the existing two native toolkit candidates and previous findings as the starting point. Build a bounded, runnable composition scene with sidebar/navigation, actual terminal cell presentation, editable native text and a real embedded browser fixture. A compile-only Win32 diagnostic text control is not the target. [S3,S7]

Exercise keyboard routing, terminal input, text selection/clipboard, editable text, resize, relevant DPI changes, browser focus handoff and disposal on isolated Windows and genuine Wayland environments. Record engine/toolkit versions and whether XWayland was involved. An X11 result is not a Wayland pass. Keep HTML restricted to browser content; no hidden web-based editor/shell.

A4 owns native UI abstractions, A5 terminal/editor surfaces, A6 browser/platform adapters; A1 owns app composition and shared contracts. Validate the selected candidate against the same scene. Record the viable choice, blockers, event-loop/child-surface strategy, maintenance tradeoffs and independent review. Do not expand a blocked two-candidate experiment into an unbounded framework survey. A newly proposed candidate needs a clear reason the current candidates cannot meet the target.

**P1 exit:** real Windows and Wayland evidence plus an accepted toolkit/composition decision. If one environment is missing, this gate stays blocked; continue P2's independent models/runtime/contracts rather than broad toolkit-specific conversion. The user may explicitly amend scope later, but a one-platform result must retain its actual label.

## 4. Stage P2 — make the foundations production-shaped, not production-connected

These tasks can begin after P0 where their contracts are settled; they do not all wait for P1.

### NWA-03 — independent session host; A2/A1; R06/R08/R13 subset

Define a versioned, authenticated local session transport and capability/error envelopes. Session IDs, workspace IDs, host IDs, generations and stream sequence numbers have distinct meanings. A2/A5 acknowledge stream changes before implementation. Existing 64 KiB decoded-frame and ingress bounds must be retained or deliberately revised with tests. Host labels are routing fences, never authorization. [S4]

Move real session ownership to a separate Rust host process/service that can outlive an individual UI connection. Product session ownership must not depend on the viewer's destructor or UI process. The outer isolated test runner may still own both host and UI for bounded test cleanup.

Specify create/attach/detach/input/resize/stop/query and host failure behavior. Validate dev roots, command argv, cwd, environment and session ownership. Use scoped launch policy for approved executables; do not simply broaden the fixture enum and inherit every environment variable. Harden local endpoints against unauthorized cross-user/cross-profile access and path-length/collision problems. Do not change old application's endpoints or agent homes.

UI restart with a living host must reattach without a duplicate process. Host restart is different: reconcile actual owned state or report ended/unverifiable; do not promise uninterrupted jobs unless architecture and tests prove it. Lost contact must never trigger local fallback. Record and investigate the historical intermittent abnormal fixture exit if it recurs; keep natural-exit assertions and bounded diagnostics. Prove shutdown/cancellation deadlines, not merely timeouts around blocking calls. [S5]

### NWA-04 — continuous bounded terminal stream; A2/A5/A1; R06/R08/R11

Replace the pilot's cumulative 4 MiB parser admission limit with a sustained-output design. Retain bounded screen/history, frame/input queues, replay and optional disk spool with explicit limits. Crossing 4 MiB must not permanently refuse new output. [S6]

Keep a canonical terminal state/snapshot authority at the appropriate host boundary; do not pretend an arbitrary raw-byte tail reconstructs terminal modes. Preserve Unicode and escape sequences across chunk boundaries. A slow or hidden viewer must not stop the host draining necessary output. Coalesce view updates or require sequence-aware resync; never silently drop raw escape bytes while pretending a complete stream. Define ordering across resize/input/output and a bounded overload policy. Test recovery and resource limits together.

### NWA-05 — actual workspace provisioning and identity; A2/A3; R09/R13/R15 subset

A2 owns Git/filesystem operations; A3 owns durable workspace metadata, host/install ownership, revisions and assignment semantics. Use separate modules and acknowledged commands rather than having both edit the same files.

Open an approved local dev repo, create a dedicated branch/worktree, persist its identity and link it to synthetic client/project records. Avoid eager terminals, watchers or language servers for merely saved workspace records. No name/path-only merging across hosts. Handle failed creation with a durable recoverable state: SQLite and Git are not one atomic transaction. Add reconciliation and idempotent retry rather than claiming cross-resource atomicity.

Use argument arrays, exact permitted paths and scoped Git trust where required; no broad global trust or forced cleanup. Prevent removal of dirty, unmerged or process-owned worktrees. Inspect ignored content without exposing credentials. Do not modify the development source repository's own branch/history as a test fixture.

### NWA-06 — file operations and conflicts; A5/A2; R18 subset

Add bounded file reading, explicit encoding/line-ending policy, native editing/undo, safe disk save and external-change conflict detection. A5 owns document/UI logic; A2 owns any shared filesystem operations/watch services. Prevent silent overwrite across observed external changes, duplicate saves and failed writes; test the relevant race window and record limits. Preserve unsaved content on cancel/failure.

Build a modest file explorer and Git diff review; stage/commit only the user's explicit selected changes inside a disposable repository. Full LSP, terminal graphics extensions and document previews remain separate later tasks. Do not substitute Monaco/web editing for native behavior.

## 5. Stage P3 — integrate the everyday native loop

**NWA-07; A1/A4/A5; R10/R11/R15 and R18 subset.** Depends on P1 and the required P2 contracts/runtime.

Compose a coherent native UI: Wilde token roles from verified assets, compact sidebar, workspace selection, terminal split/tab surface, file tree, editor, diff view and clear running/attention/disconnected states. A1 remains the app-layer writer; other roles supply reviewed interfaces. No simulated buttons that claim to perform unimplemented operations.

Complete the acceptance walkthrough in `04_ACCEPTANCE_TESTS.md`. Exercise at least three simultaneously displayed/switchable session contexts, using local deterministic sessions except for the one separately approved real-agent task. Hiding/minimizing the UI must stop unnecessary painting, not kill or stall active work. Close-view and stop-session are distinct actions with clear labels.

**NWA-08; A6/A1/A4; R12/R16/R20 subset.** Wire browser retention decisions to actual surface ownership/disposal. Use isolated local development pages. Protect dirty/unknown pages, transfers, media, remote viewers and automation leases; expose over-budget protected pages rather than breaking them. No blanket scripting/native bridge permissions. Validate true disposal, focus, reopening and bounded cache behavior. Full real-web compatibility stays unclaimed.

Do not construct a generic plugin ecosystem, universal resource-governor framework or second orchestration product as an alpha prerequisite. Implement the specific resource/ownership boundaries the alpha needs.

## 6. Stage P4 — acceptance, efficiency and review

**NWA-09; A6 with an uninvolved reviewer; R15/R16/R20 subset.**

Run equivalent release-build workloads in isolated environments. Pre-register workload definitions, measurement method, budgets and noise tolerance in the run record before evaluating the candidate. Include browser engines and tool/agent descendants, not just the Rust executable. Run one benchmark workload at a time without builds/other agents competing on the measurement host.

**NWA-10; A1 plus independent reviewers; R15/R16.** Re-run the integrated candidate, inspect failures/leaks, repair, and repeat affected acceptance cases. Use the machine record and concise handoff; raw private traces stay excluded from publication. Source changes invalidate affected results; unrelated old tests are not silently pooled into a new run.

Publish separate verdicts: `functional_alpha`, `efficiency_validated`, `full_alpha`. No full alpha pass while a required case is failed, blocked, waived without an explicit scope revision, or not run. Review-ready is not merged; local verification is not hosted CI; alpha acceptance is not production cutover.

## Dependency schedule and immediate assignments

Start A4 + A5 + A6 on the composition gate while A1 performs contract integration/preflight. As slots free or a physical-runner task blocks, rotate A2/A3 into runtime, streaming, workspace and storage tasks. Never exceed three implementation workers. A5 splits time between terminal and editor by dependency order; do not assign two concurrent writers to its paths. Reviewers can be scheduled in a freed role slot. No worker is required to sit idle waiting for unrelated phases.

At each boundary preserve evidence, update existing task/parity records, capture a resume action, and perform only the verified cleanup permitted by root instructions. Retain an uncertain worktree or build lease and explain exactly what evidence is missing.
