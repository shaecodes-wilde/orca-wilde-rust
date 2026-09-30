# Native Workspace Alpha: continuous terminal admission

Role: A5. Tasks: NWA-02/NWA-04; mapped R04/R06/R11. Synthetic model work only.
Initial worktree: F:\wr\a05, rust-v2/a05-ide, HEAD d03b45090a87f73528fc69eb9229af0af7684b53; tracked/untracked status clean.
Source truth compared: integration e68c9983d0849dc6798e38403567a17259d066b5; no wilde-ide differences from initial A5 HEAD.

## Agreed first increment

A1 and A2 acknowledged unchanged TerminalModel::accept_frame, 64 KiB decoded-frame checks, stream-cursor identity/order fencing, fixed dimensions/history/cell budgets and no parser restoration API. A2 additionally proposed and A5 acknowledged an additive synchronous canonical output sink; its implementation belongs to A2.

Ordinary output no longer has a cumulative 4 MiB admission cutoff. admitted_bytes() is saturating u64 telemetry only. A constant-space preflight watchdog limits one terminal control sequence to 16 KiB, including introducer, ignored bytes and terminating byte, across frames. It covers ESC, CSI, OSC, DCS, SOS, PM and APC, using the inspected pinned vte 0.15 state transitions. Consecutive ESC bytes within Escape remain the same unfinished sequence.

A valid frame exceeding the per-control limit latches TerminalError::ControlSequenceLimit. The rejected frame leaves cursor, parser, admitted telemetry and watchdog state unchanged, including any otherwise valid prefix in the same frame. Every later frame is refused; no bytes are silently discarded or sequence reset exposed. This is a deliberate hostile/unsupported control policy, not transparent support for arbitrarily large clipboard/title/DCS payloads.

The watchdog and cursor are staged before parser mutation. Each accepted payload is then passed unchanged to the same persistent vt100 parser once. Split Unicode, escape sequences, alternate screen and terminal input modes remain parser-owned. Wrong session/host/workspace/generation, replay or sequence gap still latch SnapshotRequired with last trusted state intact. Oversized frames remain retryable at the same sequence and consume no watchdog or admitted budget.

## Inspected boundaries and limits

Cached vt100 0.16.2 / vte 0.15.0 were inspected directly. Standard vte retains an unbounded OSC Vec unless admission is fenced. The watchdog bounds a single OSC's admitted bytes; Vec capacity may exceed its logical length and remains retained. This is not a hard process/allocator byte guarantee. vte UTF8 fragments, parameters and intermediates use fixed storage. Default vt100 callbacks do not retain title/clipboard/DCS payloads. Cells use fixed 32-byte storage with 22 content bytes and can truncate excessive combining marks; no Unicode parity claim.

Unchanged terminal limits: 1..200 rows, 1..400 columns, at most 10,000 scrollback rows and 2,000,000 retained cells; resize remembers largest retained column width. No transcript or additional raw-payload queue is held by the watchdog. Editor remains the inspected bounded document/history/save-candidate model: actual disk writes and native editing acceptance are still open.

The pinned legacy terminal protocol was read as a behavior reference: separate output, snapshot and resize opcodes with sequence framing. This Rust contract is separate and is not wire-compatible with that protocol.

## Repeatable evidence

Direct rustfmt --edition 2024 on changed A5 source files: exit 0. git diff --check and direct rustfmt --check --edition 2024: exit 0. Cargo build/test: NOT RUN by A5; A1 alone owns the build slot. No PTY, GUI, focus, platform, production, performance or external integration acceptance is claimed.

Eleven new observable model cases are authored:
- Generated 16 MiB ordinary output with no whole-transcript allocation, followed by a styled wide/combining Unicode final sentinel split byte by byte; final cells and retained scrollback checked.
- 16 MiB of recurring completed OSC sequences followed by final visible output.
- Exactly 16 KiB terminated OSC split byte by byte.
- Oversized control with screen-clear prefix: rejection is atomic and latches future refusal.
- Hostile incomplete ESC/CSI/OSC/DCS/SOS/PM/APC and ignored CSI/DCS forms across frames.
- BEL, ESC-ST, CAN and SUB control endings, followed by output larger than the control limit.
- Raw C1 ST terminates DCS but does not terminate OSC.
- Split controls/Unicode at several chunk sizes compared against uninterrupted pinned mature parser cells/cursor/alternate screen.
- Gap while UTF8/OSC incomplete freezes trusted state and forbids tail reset.
- 16 KiB repeated one-byte ESC followed by oversized refusal.
- ESC cancellation of OSC, repeated ESC and a fresh OSC, followed by continuous output.

Existing tests retain frame retry, identity/generation/replay, ANSI/Unicode, primary/alternate buffers, resize and history admission behavior. The obsolete lifetime-cap test was removed.

A6 independently reviewed the stable source against cached parser code, identified the repeated-ESC counter restart, and A5 repaired it with two focused regression cases. A6 re-reviewed the corrected stable source and approved this finite in-process increment for A1 guarded tests. A6 ran no tests; A1 test receipts remain pending.

## Remaining acceptance work

This increment supplies a bounded continuous parser consumer, not a canonical session host or recovery implementation. A2 must verify synchronous host consumption independently of fast/slow/hidden/detached viewers, explicit consumer failure, bounded queues and unchanged process lifecycle. A1 must compose and verify stdin/resize/output ordering. Authoritative replay or recovery must retain real parser continuity; a raw tail, formatted screen or display snapshot is insufficient to reconstruct unfinished UTF8/escape state. No resync constructor was added.

Native terminal cell rendering/input/selection and editor filesystem/conflict acceptance require later A4/A1 integration and isolated platform runners. No build lease, dependency install, GUI/PTY/app launch, external write, global configuration change or production profile access occurred. Worktree is retained for reviewed integration.
