# Native Workspace Alpha — first increment

Started 29 September 2026 (America/Halifax), from `e68c9983d0849dc6798e38403567a17259d066b5` on `rust-v2/integration`. Shae explicitly selected **Begin the next phase**. Local reversible implementation, fixtures, scoped commits and retained worktrees are authorized. Real coding-agent/account calls, spend, GitHub publication, hosted CI and production cutover remain unapproved.

The 12 manifest payloads were verified before adopting all 13 package files under `docs/current/native-rust/alpha/`. Root instructions were preserved and the package pointer appended once. Existing R tasks remain; NWA-01 through NWA-10 are mapped children. The 40 original parity rows are retained with milestone scope; agent/account isolation, attention, orchestration and cleanup have explicit child inventory rows.

## Current environment

`evidence/alpha/preflight.json` records both checkout identities and all five clean retained worker worktrees. The legacy source remains `1db4ef38577ec932f97067b7391625b7be599884`, with 47 observed dirty/untracked entries preserved. These are scoped observations, not a byte-identical claim about concurrent user changes. Native source and synthetic test data stay separate from the installed app and account homes.

Current A1 turn context confirms `gpt-6.1-sol/high`; explicitly requested worker launches used those settings. Worker handles and the distinction between requested routing and independently exposed receipts are in `coordination/roles.json`. A4 finished read-only composition inspection before A2 took its slot; no more than three workers were active.

Windows toolchain is installed stable Rust 1.94.0, Python 3.10.11 and Git 2.52.0. F: is exFAT with approximately 42 GiB available at the current probe. No build lease was present at preflight. One A1-controlled guarded build uses two jobs under `F:\wr-build`; workers do not build.

## Composition and acceptance blockers

No isolated Windows display or Linux/Wayland runner is verified. WSL's listing command reports that WSL is not installed; Docker/VirtualBox tools were not found on PATH, and the available vmcompute service is stopped. Those observations do not prove that every possible remote runner is absent. No infrastructure was installed or launched.

A4/A6 source inspection confirms the diagnostic terminal is a read-only text control, the sidebar is static, terminal dimensions remain 24×80, and focus routing is unwired. No Linux scene exists. Iced/Slint remain candidates, without rendered evidence. C01–C03 stay blocked.

There is also a launch-contract conflict: project instructions require `ORCA_BACKGROUND_LAUNCH=1`, while UI/browser admission rejects that value. Preserve the denial. A future independently verified isolated runner requires versioned A1/A4/A6 reconciliation before launch; clearing the flag on the active desktop is not an acceptable workaround.

F04 requires explicit CLI/account/task/root/spend approval. F08 requires an independent host and authenticated reconnect. P01–P03 require equivalent isolated release workloads, baseline/noise calibration and preregistered budgets. No native interaction, real-agent success, comparative resource savings or full alpha acceptance is claimed.

## Implementation and verification

The first safe increment is continuous bounded parsing and reader-owned output while detached, plus protected browser resource-pressure reporting. See `architecture/continuous-terminal-1.md` for acknowledged boundaries.

The initial integrated checks are **FAIL**, with exact receipts under `evidence/alpha-runs/`. Alpha-01 stopped at formatting before compilation. Alpha-02 passed format/check/test-build, then had 89 successful harness functions (one explicit unverified link case) and two cleanup-deadline failures; all 14 owned jobs emptied without forced survivors. IDE's 30 checks and browser's nine policy checks passed, while the composed continuous pilot was not accepted.

Alpha-03 used timing-only source `2cda7ca3b40d5cb7da47a0b02f0f8439add78fac`. It had 88 successful functions and three native failures, again with all 14 owned jobs empty and no watchdog timeouts or forced survivors. The failed app calls measured approximately 88/120 ms total close and microsecond master destruction, so they do not establish the cause of alpha-02's earlier timeout. All three failures instead reported `0xC000013A`; the raw sustained test had observed zero before cleanup and nonzero afterward. No deadline or natural-zero assertion was weakened. Lint and Windows-feature checks after the failed guarded stage were not run in these attempts.

The independently reviewed Windows correction now requires a signaled owned process handle before reporting exit. It strengthens exit authority; it does not establish the cause of the earlier failures. Vendor provenance preserves the original hashes and records exactly two source exceptions.

Alpha-04 used `89f5d43ed4b4a94e9ee148b6c4651a6a61b69091` and passed all eight verification stages. All 14 owned test executables exited zero. Its 91 harness successes include one explicit NOT RUN native-link case, leaving 90 actually exercised successes. The composed detached parser received 18,874,542 bytes and sequence 4,750, retained its identity, found the final marker, exited naturally with zero, joined both workers and removed its owned fixture directory. The raw sink received 18,874,760 bytes and sequence 73,989. These are real Windows ConPTY fixtures, not a rendered application or independent host/reconnect test.

Alpha-05 repeated the same source and identical executable hashes. It **failed**: the continuous marker missed its deadline; the original pilot returned exit code 1; the six-test native runtime harness exceeded its 45-second watchdog and was terminated. The truncated harness shows three failures and two successful cases but no completed test summary. All 14 owned jobs eventually emptied. `forced_remaining_children=false` does not erase the watchdog termination. Later verifier stages were not run in this attempt. Repeat reliability is therefore unaccepted, regardless of alpha-04's successful run.

One reviewed diagnostic increment records first supervisor kill-request observations, first positive supervisor exit and maximum polling gap. The existing guard now writes test output immediately so a later watchdog termination cannot hide earlier panic receipts. Headless pilots record bounded phase/progress and pre-close observations. Original 7/8/2-second budgets, natural-zero checks and fixture controls are unchanged.

Alpha-06 used `86e2b20f12193ca5b7aa2c2c97097b8052db3158` and **failed** one fixed-lifetime cleanup case. The directory-removal phase took 4.4998156 seconds; total close took 4.5099452 seconds against the two-second budget. Master destruction took 18.8 microseconds and both workers had joined. The supervisor recorded lease fault 3 with no close stop flag before its kill request, followed by signaled exit code 1. This locates this run's failure in directory removal, without identifying the exact slow filesystem call or underlying cause. The source builds and validates its manifest using synchronous filesystem calls before checking the per-path deadline; later operations are also synchronous. No timing limit was relaxed.

Both app pilots and the raw sustained sink passed alpha-06. The composed parser received 18,874,541 bytes/sequence 4,640 with the final marker, unchanged identity, no stream fault, natural zero before/after close and complete cleanup. The raw sink received 18,874,760 bytes/sequence 74,000. There were 90 harness successes and one failure; one success explicitly did not exercise native symlink rejection, leaving 89 exercised successes. All 14 owned jobs ended empty without watchdog timeout or forced survivors. Alpha-07 passed all six current-source compile/lint stages, without executing fixtures or native surfaces.

Alpha-02's timeout phase and alpha-05's deadline/shutdown causes remain unresolved; alpha-06 timing cannot be assigned retrospectively. Model correctness, real PTY output, integrated cleanup and full alpha acceptance remain distinct results. A6 independently audited every selected log/executable hash and outer-job receipt for alpha-04 through alpha-06, plus alpha-07 compiler logs. A1 independently reviewed A6's browser-pressure source and nine observable policy tests; live browser disposal and measured resource behavior remain unaccepted. See `evidence/alpha/alpha-run.json` and `recon/a06/alpha-06-removal-phase-evidence-review.md` for restricted verdicts and receipts.

The existing verifier accepts `-EvidenceRun <unique-name>` to retain this cycle's logs separately under `evidence/alpha-runs/`. Reusing a run name fails before writing its logs. It uses the existing build lease and owned-job runner, without creating another watchdog or launching a GUI.

## Resume and retention

The first local increment is implemented; repeat runtime acceptance remains **FAIL**, and full alpha is **not accepted**. The next focused action is to observe individual owned-directory validation/enumeration/removal durations or review a deadline-safe cleanup boundary, preserving the existing limits and all failed receipts. Do not repeat tests blindly or reinterpret later directory absence as an initial deadline pass.

Independent host/transport, workspace provisioning and bounded native file operations remain separate planned alpha work. Broad toolkit work remains gated on real Windows/Wayland composition; Shae confirmed no existing isolated runner. No runner was installed, no native UI was launched, and no real coding-agent/account call, paid action, publication or cutover occurred. Postflight package hashes and the legacy HEAD/path-status inventory match preflight; this is a scoped observation, not a byte-identical/profile/process noninterference claim.

All worktrees are retained. Their integration ancestry, ignored content and stopped-process cleanup requirements have not all been established. No deletion or external publication is authorized by this handoff.
