# A5 terminal/editor state pilot source map

Role: A05. Actual selected model: GPT-6.1-Sol, High. Worktree: F:\wr\a05; branch rust-v2/a05-ide. This is a new native Rust repository; the legacy app was read only.

## Behavior sources inspected

Pinned legacy commit: 1db4ef38577ec932f97067b7391625b7be599884.

- src/shared/terminal-stream-protocol.ts: binary version1 framing, sequences, separate output/snapshot/resize messages, optional negotiated opcodes. The Rust pilot is a separate v1 contract and is not a compatible replacement for this wire protocol.
- src/renderer/src/components/editor/editor-save-queue.ts: one save queue per file, generation invalidation, live draft selection, read-only/owner migration guards, host-owned writes. No browser/editor implementation copied.
- src/renderer/src/components/editor/editor-save-target.ts: preview save resolves to the editing source. Preview integration remains open.
- Kickoff 05-contracts-and-boundaries.md: A2 owns execution and filesystem authority; A5 owns emulation/editor state; one canonical model per session; disconnect is not termination.

## Reused component

vt100 = 0.16.2, exact pin. Public API inspected on 2026-09-29: Parser::new/process/screen; Screen::set_size/size/contents/cell/cursor_position/alternate_screen/set_scrollback; Cell::contents/is_wide/is_wide_continuation/fgcolor. Primary documentation: https://docs.rs/vt100/0.16.2/vt100/struct.Parser.html and https://docs.rs/vt100/0.16.2/vt100/struct.Screen.html. A1 owns fetched dependency/license review and lockfile integration.

Bytes are first checked by the shared StreamCursor. Accepted bytes enter a single persistent mature VT parser, which retains split escape and UTF8 state across frames and updates its cell grid. A native scene can inspect that grid or its plain contents. No JavaScript terminal or hand-written ANSI parser is involved.

Fresh sessions start at sequence 0. A gap, replay, or identity/generation mismatch latches SnapshotRequired and leaves the last screen intact. No public resync constructor exists: vt100's formatted visible screen is not proven to capture parser fragments, both buffers, all scrollback, and host ordering. Recovery requires future host-authoritative snapshot semantics. Too-large frames return FrameTooLarge without advancing or latching recovery.

Screen admission: 1..200 rows, 1..400 columns; at most 10000 scrollback rows; at most 2000000 retained grid cells, including both screen buffers and the largest accepted column width. Resize must be ordered by the caller with A2's PTY resize. A6 inspected cached vt100 0.16.2 / vte 0.15.0 source: cell combining storage is fixed and can truncate long sequences; the default std parser retains an unbounded OSC buffer for an unterminated terminal title/control sequence, including across frames. A grid limit alone does not contain this buffer. The repaired pilot admits at most 4 MiB of total bytes over a fresh session lifetime. Shared ingress validation is staged on a cursor clone; the live cursor, parser and admitted counter change only after the remaining lifetime budget check succeeds. A valid frame exceeding the remaining budget latches LifetimeByteLimit, leaves the last screen/cursor/count intact, and rejects every later frame. No bytes are silently discarded and no counter reset is exposed. Individual oversized frames retain FrameTooLarge and consume no budget. This is a short-lived fixture safeguard, not a production terminal lifetime policy, a hard allocator/whole-process byte cap, or transport backpressure. A production terminal requires a reviewed state-aware bounded parser/backend and host-authoritative recovery policy. Current app integration remains limited to A2 owned fixtures.

A2 must serialize saves per document and route each candidate acknowledgement to its owning document. The state model carries no filesystem/host authority.

Editor admission: at most 1 MiB UTF8 document; at most 8 MiB / 128 retained full-text history states. Edits use checked byte ranges, preserving Unicode boundaries; this is not grapheme-aware navigation. SaveCandidate owns draft and expected disk baseline plus a private document-origin token. Acknowledgement from another document fails before mutation even when baseline and revision match. A2 must compare that baseline at the actual write boundary and acknowledge only a confirmed successful save. Changes made while a candidate is being saved stay dirty.

## Evidence boundaries

Deterministic terminal fixtures are synthetic byte streams in terminal_model/tests.rs, not captured agent transcripts. They test parser state and observable cells, not CLI readiness or visible rendering. Native drawing, keyboard/IME/selection, hidden native-window integration, durable session recovery, search, LSP, syntax, diff/review, file explorer and rich previews are OPEN. No GUI launch or visible-window test was attempted.

## State pilot acceptance criteria

Accepted consecutive frames produce the same cells as uninterrupted bytes, including ANSI and UTF8 splits. Alternate screen exit restores the primary screen and cursor. Rejected order/generation/host identity leaves screen and cursor unchanged and requires recovery. Oversized frames are rejected whole and the same sequence may be retried with an admissible payload. A split unterminated OSC can fill the lifetime admission cap exactly, but a following terminator/screen-clear frame must fail before live cursor/parser/count mutation and latch a resource fault; subsequent frames must remain refused. Invalid resizing cannot mutate the screen. Scrollback count and editor document/history admission stay within their configured bounds. Invalid edit ranges/Unicode splits preserve text and history; undo/redo restores user text; stale disk content produces a conflict; saving an earlier draft never marks later edits clean. All of these are model-level criteria, not native UI or external filesystem acceptance.


