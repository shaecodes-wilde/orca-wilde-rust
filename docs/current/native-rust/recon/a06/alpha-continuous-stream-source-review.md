# A6 independent source review: continuous terminal increment

Verdict: APPROVED for A1 guarded verification at the finite headless in-process boundary below. This is source-only review; no test result, GUI, daemon/restart, SSH, snapshot recovery or efficiency acceptance. A6 executed only read/diff/hash/whitespace checks and owned documentation writes; A1 alone builds/runs test executables.

## A5 immutable source

Reviewed commit 683ef377a7067adafcea10a571fc1f13a73637fa, verified locally at F:\wr\a05 with clean short status. Compared persistent vt100 0.16.2 and vte 0.15.0 cached implementation, including ground UTF8 handling, CSI/DCS entry/param/ignore, raw C1 ST, OSC BEL/CAN/SUB/ESC and repeated Escape transitions.

| File | Reviewed working-byte SHA256 (matches committed worktree) |
| --- | --- |
| crates/wilde-ide/src/terminal_model.rs | F8D53D63E10F4E53C3604D8DA4B96B868C13549F8AAFA8D2C5806A11383E2355 |
| crates/wilde-ide/src/terminal_model/control_budget.rs | 9EC4F8BC9E6823ED141AEEB73F7CBB5CBF37A4D02728DEEDFF1376F86F5319BD |
| crates/wilde-ide/src/terminal_model/stream_tests.rs | DAFB677997A8E3EF25886885B4C222B9045662DD540DB3F52A1EE46BD3AB4418 |

Ordinary output removes the cumulative 4 MiB admission refusal; admitted bytes are saturating u64 telemetry. Constant-space preflight guards one 16 KiB control sequence across frames before live parser/cursor/telemetry change. Frame admission errors preserve trusted state and latch refusal. Mature persistent parser receives accepted bytes unchanged once; dimensions/scrollback/cells stay bounded. Unknown replay/gap/identity still refuses rather than rebuilding from a raw tail.

Initial review found repeated ESC reset the watchdog counter while cached vte remains in the same Escape state. A5 corrected the precise transition and added repeated-ESC rejection plus old-OSC cancellation/new-OSC continuity tests. Re-review confirms correction. All eleven authored cases are meaningful model boundaries; A6 did not execute them. A5 reports direct rustfmt and whitespace checks, which do not prove behavior.

OSC logical admitted size is bounded but Vec allocator capacity may be larger and retained. Excess combining marks can be truncated by fixed cached parser cell content. No hard allocator-byte guarantee or Unicode parity. No display snapshot/parser restore API was introduced; unfinished UTF8/escape state requires preserved canonical parser continuity or a separately agreed authoritative protocol.

## A1 application and verifier working-source boundary

Reviewed primary working changes for run_continuous_terminal_pilot, apps/wilde-desktop/tests/continuous-terminal.rs, and tools/verify.ps1 EvidenceRun. These hashes identify the reviewed bytes before A1 formatting; immutable integrated candidate and executed receipts remain pending. Reconfirm any non-format semantic change.

| File | Reviewed working-byte SHA256 |
| --- | --- |
| apps/wilde-desktop/src/lib.rs | 3D199ACA095BF5AAC91D3A5C4CA08AE59965A7B5C053F882F488550C6A11750E |
| apps/wilde-desktop/tests/continuous-terminal.rs | FFFDCCC4753E61D832A7720FD85FCD7D6479BC6919F12251B71B149680C91450 |
| tools/verify.ps1 | 39F3A57F3005BCF8F7F342529ECC981BA1CC749668A62BAD1361F3CD2B81DA85 |

Pilot owns one canonical model via reader sink while observer is detached, then inspects >=16 MiB and final sentinel before reconnect/Finish. It requires unchanged owned process/session identity, natural zero exit before cleanup, zero after cleanup, joined workers and exact fixture directory removal. This demonstrates only the proposed in-process host boundary; attach flag/Arc model do not establish independent daemon lifetime, UI crash/restart, slow viewer delivery, snapshot resync, native cells or full F03/F08.

A6 found a late stream fault could pass once sentinel was observed because close/exit evidence alone did not inspect consumer/read faults. A1 added a post-close stream_fault None assertion and stream_fault_free evidence; source re-review confirms it. A6 found EvidenceRun created alpha-runs before checking an existing evidenceRoot reparse link. A1 moved the check before creation and checks an existing runParent before creating any child. Unique ordinary run names refuse existing output and preserve prior evidence. Source re-review confirms fixes; same-user concurrent filesystem substitution is not a hostile-boundary guarantee.

No GUI launch path is invoked. Existing ORCA_BACKGROUND_LAUNCH=1 and private-job guard remain. Runtime synchronous sink can still block under a hostile consumer; external watchdog is containment, not a proven production cancellation deadline. A1 independently reviews A2 source. Required next check: freeze immutable integrated source, run serial guarded verifier under build lease with two Cargo jobs and unique EvidenceRun, inspect all exact receipts/output before assigning case results.

A6 retention changes are excluded from this independent approval; A1 separately reviewed/integrated that author-owned change. Composition/runner/performance blockers remain in alpha-composition-and-performance.md. No source review converts BLOCKED/NOT_RUN cases to PASS.
