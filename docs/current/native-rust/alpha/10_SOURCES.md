# Sources, scope and historical-state caveats

Prepared 29 September 2026 from read-only connected GitHub inspection and official Codex instructions. This package is newly authored execution guidance. It does not contain application patches, new test results, credentials, copied production profiles or third-party assets.

## Pinned implementation references

The reviewed published Rust HEAD is `e68c9983d0849dc6798e38403567a17259d066b5` on `rust-v2/integration`. The earlier comparison used legacy `wilde/theme` at `1db4ef38577ec932f97067b7391625b7be599884`. Reconcile current local and remote state before executing; never reset to these merely because the package names them.

**[S1] Root work rules** — native boundary, protection of old app/checkouts, single-path ownership, maximum three implementation workers, A1-only builds with two Cargo jobs, isolated GUI requirements and safe cleanup.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/AGENTS.md

**[S2] Role register** — actual path ownership and historical model routing. It records Shae's later request for GPT-6.1-Sol at High for all six roles, overriding the earlier kickoff's Astra/Opus preferences. This is project history/intent, not independent proof of present model availability or the model currently running a new thread. Previous task handles, worktree state and publication fields are historical.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/coordination/roles.json

**[S3] Existing backlog** — R03/R04/R05 composition/toolkit work; R08/R09 runtime/domain pilots; R10–R12 native surfaces; R15 integration; R16 comparable benchmark; later parity/governor/integration tasks. The new NWA labels are proposed mappings into this plan, not completed tasks.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/coordination/backlog.json

**[S4] Contract 1** — host authority, detach/termination distinction, version/size constraints, stream sequence/generation recovery and missing authenticated daemon/snapshot-authority implementation in the first cycle.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/docs/current/native-rust/architecture/contract-1.md

**[S5] First-cycle report** — what was actually tested versus not run; synthetic SQLite and fixture ConPTY; no launched native scene or comparable performance; isolated display blockers; known abnormal-exit/link-test qualifications; historical local modifications and preserved worktrees. Its tested-source identifier `52e478ce83db480df779cc0255cf898495844452` is reported historical evidence, not a new test result or an instruction to assume that commit is available in every clone.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/docs/current/native-rust/handoffs/a01/SESSION_REPORT.md

**[S6] Terminal model** — the pilot's cumulative 4 MiB parser-input admission limit and existing screen/retention/frame handling. Production retention proposals in this package are requirements for new work, not implemented behavior.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/crates/wilde-ide/src/terminal_model.rs

**[S7] Windows diagnostic scene** — native controls plus child browser; parsed terminal text display and diagnostic update behavior. This source is not evidence that a complete native terminal/editor has been exercised.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/apps/wilde-desktop/src/windows_scene.rs

**[S8] Feature-parity register** — forty capabilities, partial/not-started native status and legacy-source verification still needed. Maintain its capability inventory and qualify acceptance by demonstrated scope.
https://github.com/shaecodes-wilde/orca-wilde-rust/blob/e68c9983d0849dc6798e38403567a17259d066b5/coordination/feature-parity.csv

**[S9] Existing Wilde app** — current business workspace, native-adapter integrations, branding and fork/update behavior described by the legacy repository. Inventory source/tests; do not treat documentation alone as universal runtime verification.
https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/WILDE.md
https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/docs/wilde-super-app-setup.md

**[S10] Official Codex project-instruction guidance** — AGENTS.md is project guidance; Codex discovers global/project instructions with more specific files layered into the run. Keep project instructions local and small; do not replace existing safety rules or create an override that suppresses them.
https://developers.openai.com/codex/guides/agents-md
The official page redirected during retrieval to:
https://learn.chatgpt.com/docs/agent-configuration/agents-md

## Interpretation rules

This handoff preserves six roles but does not require six simultaneous workers or invent a new model recommendation. It preserves the full-native destination without requiring browser engines, external agents or system libraries to be rewritten in Rust. Choosing a permanent toolkit remains evidence-led; no new toolkit/version was selected by this package.

All new tasks, acceptance thresholds, experiments and workload counts are proposed instructions. Every machine-record acceptance case is initialized NOT_RUN. The package deliberately excludes installer/release/cutover authorization, account access and automatic GitHub publication. Historical approval-pending fields do not prove present approval or revoke a later explicit instruction; reconcile the actual current authority.
