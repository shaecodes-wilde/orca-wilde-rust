# Agent dispatch briefs

These are roles, not six simultaneous heavy jobs. A1 coordinates; at most three implementation workers are active. Existing ownership in `AGENTS.md`/`coordination/roles.json` controls unless explicitly reconciled. Paths that do not yet exist are proposed ownership, not instructions to create empty crates. [S1,S2]

## Shared worker preamble — send with every assignment

You are the named role within the existing Native Workspace Alpha migration. Read applicable repository instructions, your assigned source/contract, the relevant task and acceptance cases. Work only in your verified dedicated worktree and owned paths. Do not edit legacy Orca, shared root files, app composition or another role's files. Propose cross-owner changes and obtain producer/consumer acknowledgement. No recursive workers, dependency installation, external writes, production profiles, account calls, native GUI on the active desktop, or unleased builds. A1 runs builds through the existing single-build mechanism; submit a precise build/test request.

Report implementation and tests separately. Use direct available agent messaging for contract questions; otherwise send A1 a concise relay. Do not invent session IDs or say a worker is running without a real handle. Keep comments and handoffs concise. Explain one important design tradeoff in plain English. Preserve unrelated files and unknown artifacts. Finish with source commit(s), owned paths changed, test requests/results, risks, integration instructions and next action. You do not authorize or perform your own final worktree cleanup.

## A1 — architecture, app composition and integration

**Own:** existing root Cargo/toolchain/configuration, `crates/wilde-contracts/`, `apps/`, integration/CI and shared registers. Preserve current ownership before any additions.

**Assignment:** reconcile current state; adopt the handoff locally; map NWA child tasks; verify model/runner reality; dispatch at most three workers; settle minimal contracts before parallel implementation; integrate the actual application flow. Run one guarded build at a time with two Cargo jobs and shared external outputs. Keep safety/permissions and environment blockers explicit.

**First action:** P0 preflight, then allocate A4/A5/A6 composition work with bounded responsibilities. Runtime/domain tasks may rotate in when a slot frees. Freeze a tested integration candidate before acceptance runs; do not give workers a moving source target during evidence capture.

**Review:** independent role reviews A1's contracts/integration. A1 reviews A6-authored platform changes or assigns another uninvolved reviewer; A6 never certifies its own code by itself.

## A2 — session runtime, process ownership, Git/filesystem services

**Own:** `crates/wilde-runtime/` and its existing role documentation/tests. Contract changes go through A1. Metadata semantics belong to A3; editor logic to A5.

**Assignment:** NWA-03/04/05 and shared filesystem support for NWA-06. Build a real independent local session host with authenticated dev IPC, bounded input/output, genuine shell operation, safe argument/environment boundaries, stable identity and explicit detach/stop/failure semantics. Implement actual Git worktree provisioning/status/diff service operations without global trust changes. Prevent duplicate recovery and unowned cleanup.

**Cases:** F01–F03, F07–F08, R01–R03; partner with A5 on stream/snapshot/resize and A3 on provisioning reconciliation. Investigate unexplained fixture exits and cancellation deadlines without deleting guard assertions.

**First deliverable:** proposed host/launch/stream contract plus a deterministic owned-process test request. No blanket replacement of the fixture allowlist, no dependency on the UI destructor for session lifetime.

## A3 — durable domain/storage and workspace lifecycle records

**Own:** `crates/wilde-domain/`, existing storage modules and synthetic fixtures. Do not create a second storage crate merely because an older role list names one.

**Assignment:** NWA-05 with A2. Preserve client/project/host/install identity, revisions and durable idempotency. Connect synthetic assignment records to actual provisioned workspaces, model partial creation/failure and reconcile safely across Git and SQLite. Persist only what is required for the alpha; do not launch a full CRM or import production records.

**Cases:** F01, F06/F10 where storage applies, R01/R03 and relevant synthetic legacy differential cases. Continue independently useful preview-token/import contract tests only within mapped existing tasks; do not claim full legacy parity from the alpha subset.

**First deliverable:** the workspace-operation/result contract and tests for retries, identity collisions, unavailable targets and failed provisioning. A2 executes Git; you commit metadata transitions. Never conflate transaction-safe SQLite with an atomic transaction across filesystem and Git.

## A4 — native toolkit, UI interaction and visual hierarchy

**Own:** `crates/wilde-ui/` and role documentation. A1 owns `apps/`; coordinate integration through interfaces/patch requests.

**Assignment:** NWA-02 first, NWA-07 after the composition gate. Compare existing toolkit candidates using the same Windows/Wayland scene. Prove focus, keyboard routing, editable native text, resize/DPI behavior and available accessibility behavior. Document the candidate decision with evidence before broad UI conversion.

Then provide a coherent sidebar/workspace selector, terminal layout, editor/diff navigation and clear lifecycle state. Use verified Wilde token roles/assets without guessing a new brand system. Prioritize useful hierarchy and low idle work over animation.

**Cases:** C01–C03, F02/F05/F07–F10. Hidden components stop unnecessary repainting, not runtime processing. Do not present fake functional controls or web-based first-party surfaces.

**First deliverable:** bounded scene layout/input contract and an exact isolated-runner test request. A missing display blocks rendered evidence, not source/model work.

## A5 — native terminal and editor

**Own:** `crates/wilde-ide/` and role documentation/tests. Coordinate runtime with A2 and native integration with A4/A1.

**Assignment:** NWA-02/04/06/07. Reuse the existing parser/document models after inspection. Add real cell rendering/input/selection/scrollback and native file editing/save/conflict/diff. Remove the cumulative-output design limitation by agreeing bounded continuous stream/snapshot semantics; do not simply enlarge the constant. Avoid unbounded queues, byte loss and rendering a whole transcript on each frame.

**Cases:** C01–C03, F02–F08; test split Unicode/escape sequences, long output, resize ordering, slow viewer resync and external-file changes. A mock text control is not a terminal, and an editor model without actual disk writes is not editor acceptance.

**First deliverable:** terminal-view/input/snapshot consumer contract and targeted deterministic test cases. Implement terminal reliability first, then the smallest useful editor/review path; full LSP/preview breadth remains later work.

## A6 — browser, platform/resource measurements and independent review

**Own:** `crates/wilde-browser/`, existing platform/resource modules, platform/performance tests and role documentation. A1 owns shared app wiring/CI.

**Assignment:** NWA-02/08/09. Prove browser child-surface composition on Windows and Wayland, dev-profile isolation, permissions/focus and actual lifecycle release. Wire retention policy to real ownership without evicting dirty/unknown or protected pages. Maintain explicit browser capability gaps; no fixture-based general-web or CDP claims.

Define and run the release-build comparison protocol with app/runtime, browser, tools/agents and total-workload buckets. Do not benchmark while other roles compile on the same host. Keep raw records sanitized and compare equivalent functionality.

**Cases:** C01–C03, F09, R03, P01–P03. Review others' critical evidence independently, but obtain A1/another uninvolved reviewer for your own browser/governor/measurement code.

**First deliverable:** composition risk/runner matrix and preregistered baseline protocol. Record unknowns rather than inventing low-RAM targets or compatibility results.

## Dispatch and handoff message shapes

**A1 dispatch:** task ID; goal; exact base; worktree; allowed/forbidden paths; dependencies and contract versions; acceptance cases; verified model/effort; test/build request route; permission boundaries; expected deliverable.

**Contract request:** sender → owner; producer/consumer; proposed payload/API; invariant; compatibility impact; acceptance cases; acknowledgement required before coding dependent assumptions.

**Worker return:** task; actual runner; commit(s); changed paths; implemented behavior; executed versus requested checks; exact evidence; failures/blockers; reviewer needed; integration order; cleanup eligibility (not cleanup authorization).

A1 records receipts in existing role/task records. Do not create a second competing task tracker or commit private agent transcripts/credentials.
