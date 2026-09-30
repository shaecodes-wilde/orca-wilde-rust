# Acceptance tests — what the alpha must actually prove

All cases below begin **NOT_RUN**. Source review, compile success, a screenshot and actual interactive behavior are different evidence classes. Record exact steps/assertions, source/build identity, host environment, run ID, sanitized artifacts and independent review. A supported native event trace may prove an interaction; a policy-model test alone cannot.

## Verdicts

`functional_alpha` requires S01–S02, C01–C03, F01–F10, R01–R03 and Q01–Q02 to pass in their specified environments. F04 requires an explicitly approved real coding-agent exercise, not just a mock. `efficiency_validated` requires P01–P03 plus the prerequisite functional results. `full_alpha` requires both verdicts and no unresolved critical data-loss, duplicate-session, unauthorized-access or resource-lifecycle defect.

Alpha functional testing is Windows-first; C01–C03 additionally require actual Linux/Wayland composition evidence. This does not establish full Linux/macOS parity. If an environment or live-agent approval is absent, the dependent case is **BLOCKED** and the full-alpha verdict remains blocked; report the tested subset honestly.

Allowed case results: NOT_RUN, PASS, FAIL, BLOCKED. Waiving or deferring a required case requires an explicit versioned scope decision; it does not change its result to PASS. Task status values in the existing backlog remain unchanged.

## Safety and setup

**S01 — environment/provenance.** Verify current destination checkout, immutable candidate identity, instruction/owner map, isolated display(s), dev app/profile/endpoint roots and owned test process boundaries. Include the legacy comparison build identity and profile isolation. A missing safe runner is a blocker, not permission to use the active desktop.

**S02 — noninterference.** Check the development application did not change the installed Orca app, legacy checkout, update feed, profiles, auth homes or existing sessions. Use scoped observations of authorized paths/processes; do not copy secrets or claim a concurrent user's entire filesystem is byte-identical. Verify all test writes are attributable to approved development roots.

## Native composition — both Windows and Linux/Wayland

**C01 — genuine surfaces.** Native sidebar, interactive terminal cells, native editable text and an actual browser fixture share the application window. Verify input reaches the intended surface. No HTML/React first-party controls or editor. Record compositor and embedded engine; label XWayland honestly.

**C02 — interaction.** Exercise keyboard traversal, terminal keystrokes, mouse selection, copy/paste, browser text input, command routing and at least one real IME composition path where supported. Check resize, minimize/restore and available DPI transitions without corrupted layout or focus capture. Record accessibility tree/focus behavior using available native tooling; include unsupported/untested cases, not blanket accessibility conformance. Missing physical multi-monitor coverage remains explicitly open for later parity.

**C03 — disposal and toolkit decision.** Close/reopen browser and native surfaces repeatedly; verify subscriptions, handles and owned objects are released according to policy. Record limitations and selection of the permanent toolkit/composition approach; an uninvolved reviewer signs off against the actual scene results. Compilation alone fails this criterion.

## Functional coding loop — isolated Windows development environment

**F01 — real workspace.** From a disposable local repository with an initial commit, create a dedicated branch/worktree through the app. Verify actual Git state plus stable local workspace identity and a synthetic client/project association. Restart metadata storage and recover the association. Inject one creation failure and reconcile without an orphan or duplicate record/worktree.

**F02 — real terminal interaction.** Start a real approved local shell, type commands, resize, interrupt a running test command, copy/select/paste text and exercise cursor/color/alternate-screen behavior. Test split UTF-8, combining/wide characters and selection against deterministic expectations. The old Ping/Finish fixture alone is insufficient.

**F03 — sustained stream.** Feed a deterministic transcript exceeding the old 4 MiB lifetime limit (proposed case: at least 16 MiB per session), using bounded frames. Verify a final sentinel still appears, retained history/queues respect configured limits, and byte/chunk boundaries do not corrupt parser state. Repeat with the viewer hidden, slow and detached; recover state after a sequence gap using authoritative snapshot/replay. Test stdin and resize ordering too.

**F04 — actual coding agent.** With explicit permission for the particular installed CLI, account, synthetic task, filesystem scope and spending cap, run one agent in the native terminal. Ask it to make a small change and run a local test inside the disposable worktree; inspect its actual output and changed files. Interact with its genuine prompt/input behavior. Record CLI/version and outcome without capturing credentials. Do not import the production CLI profile or launch a paid call automatically. Without approval/availability, mark BLOCKED and separately report shell/fixture success.

**F05 — native file save/review.** Open the changed file in the native editor, modify it, undo/redo, save, inspect its diff, and explicitly stage/commit only selected test changes if that UI is implemented. Verify bytes/line endings/encoding and resulting Git state. Minimum alpha requires diff review; any deferred commit UI must be labeled unsupported rather than faked, and a real shell commit can complete the disposable walkthrough.

**F06 — conflict/failure.** Externally change an open document before save; verify the app refuses silent overwrite and offers a safe resolution. Inject failed write/read, invalid encoding or configured large-file rejection and preserve unsaved edits. Test concurrent save/revision changes and workspace disappearance.

**F07 — three session contexts.** Run three independent local sessions/workspaces and switch between them; use only one real approved agent when needed, with deterministic local sessions for the rest. Verify identity/input routing, meaningful state, no cross-session output, no hidden-view repaint churn and no stopped output drain. Closing a pane is not stopping a session.

**F08 — UI reconnect.** Leave a deterministic job running in the independent host, disconnect/close only the UI, reopen it and reattach to the same generation/session. Assert the owned process was not restarted, duplicated or unintentionally stopped, and recovered terminal state is correct. Test both orderly viewer close and controlled viewer crash.

**F09 — minimal development browser.** Open a local fixture preview alongside the coding workspace, navigate allowed fixture pages and exercise text input/focus. Verify forbidden origins/popups/native calls remain denied. Close/reopen it and account for resources. No claims about general OAuth, passkeys, media or Design Mode follow from this test.

**F10 — persistent small application state.** Restart using the same isolated dev profile; recover workspace metadata, intentional UI state and available session references without launching every saved workspace. A database reopen is not evidence of a surviving daemon. Moved/unavailable workspaces remain unavailable until explicitly reconciled.

## Failure, authority and cleanup

**R01 — host and transport failure.** Crash or disconnect the controlled session host/transport. Distinguish unverifiable from positively observed exit, then reconcile without silently starting a replacement. Reject wrong profile/host/session/generation and unauthorized local IPC access. Test incompatible protocol version and malformed/oversized frames. Simulated remote loss must never produce local execution; this is not full SSH proof.

**R02 — owned termination.** Explicitly stop a session and verify its owned process tree, reader/writer workers and handles exit within preregistered deadlines. Exercise natural exit, interrupted commands and one cancellation/fault path repeatedly. The test watchdog's survival/cleanup must not mask production-host deadlocks. Retain and diagnose unexplained abnormal exits; never weaken assertions to pass.

**R03 — safe cleanup and browser protection.** Refuse worktree removal when unique/unmerged/dirty/ignored work or running owned processes remain. Complete normal removal only after required checks; never force. Exercise real link/reparse-path cases on a filesystem that supports them; unsupported creation is NOT proof of rejection. Protect dirty/unknown browser state, transfers/media/leases/viewers from automatic eviction; expose over-budget protection and prove eligible surface disposal.

## Performance and quality

**P01 — equivalent paired baseline.** Execute the agreed release workloads against native and legacy development builds with matching repository data, terminal transcript and browser fixtures. Include process categories, raw samples, environment, repetitions, exclusions and comparable feature coverage. Blank-native versus fully loaded legacy is not a comparison.

**P02 — efficiency criteria.** Meet preregistered first-party overhead and responsiveness criteria, accounting for variability, and disclose total-workload impact. The exact numeric budgets must be recorded before candidate acceptance is scored. A lower process count or unmeasured assertion does not pass. Functional progress with no demonstrated savings remains useful but is not efficiency validated.

**P03 — bounded lifecycle.** Run the retained-workspace scenario, a declared sustained-output soak, and repeated create/close cycles. Verify retained allocations, watchers, handles, child processes, cache/spool disk and browser surfaces converge within the recorded bounds; explain shared engine residency. Show disabled integrations cause no hidden recurring work. Do not imply this bounded experiment proves indefinite uptime.

**Q01 — integrated verification.** A1 executes the repository's appropriate guarded format/check/test/lint and platform build checks against the integrated candidate. Retest affected functionality after changes. Report known environmental skips separately. No stale evidence from a different source tree or diagnostic-only build may satisfy this gate.

**Q02 — independent acceptance and handoff.** Uninvolved reviewers examine critical runtime/storage/browser decisions and corresponding evidence. Deliver current task/parity status, exact source/build/run IDs, reproducible isolated launch/retest steps, unresolved scope and cleanup/retention receipts. Review-ready, local integration, GitHub publication and production cutover remain distinct.

## Required demonstration sequence

Record one reproducible walkthrough: create dev worktree → shell command → approved agent's small code change/test → native file edit/save/diff → switch among three sessions → leave a controlled job running → close/reopen UI → reattach without duplication → browser fixture interaction → deliberate stop → safe cleanup. Use separate fault tests for destructive-to-fixture failure injection. Screenshots/video support but do not replace assertions and identity/ownership evidence.

## When something is unavailable

Write: exact blocked case, missing environment/permission/tool, checks already attempted, dependent tasks withheld, safe independent work continued, and one concrete next action. Do not invent a runner, recommend a paid service without approval, or declare a fixture/live-platform test interchangeable. Preserve resumable changes and stop only the affected execution path.
