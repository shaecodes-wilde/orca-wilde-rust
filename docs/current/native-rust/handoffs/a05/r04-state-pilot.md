# Task handoff

Task / role: R04 / A05 native terminal and editor state pilot
Actual model / runner / session: GPT-6.1-Sol High / Codex child agent / a05_native_ide
Worktree / branch / exact HEAD: F:\wr\a05 / rust-v2/a05-ide / see implementation commit supplied to A1
Base contract version: wilde-contracts protocol major 1; unchanged SessionIdentity, TerminalFrame, StreamCursor
Status: implemented local state APIs; integration build/tests NOT RUN pending A1 sole build slot

## Result in plain English

A terminal keeps a persistent VT parser across accepted frames, so byte splits do not turn into broken escape codes or Unicode. Gaps freeze the last trusted screen and demand host recovery. The editor supports bounded UTF8 edit/undo/redo and returns a save candidate only when the observed disk text matches its baseline. Neither state model executes a shell or writes a file.

## Implementation

Owned crate crates/wilde-ide: terminal_model.rs, editor_document.rs, focused tests, lib.rs and Cargo.toml. Reuses mature vt100 exactly 0.16.2. A1 acknowledged API proposal. No root Cargo, lock, shared contracts, app, runtime, UI or old app mutations. See recon/a05/source-map.md for sources and limitations.

Terminal API: TerminalModel::new_session(SessionIdentity, rows:u16, cols:u16, scrollback_rows:usize)->Result<Self,TerminalError>; accept_frame(&TerminalFrame)->Result<(),TerminalError>; screen()->&vt100::Screen; contents()->String; resize(rows,cols); set_scrollback(rows); next_sequence(); snapshot_required(). One owner/model per fresh session. No recovery/reset API.

Editor API: EditorDocument::open(String,EditorLimits); replace(Range<usize>,&str); undo/redo; text/revision/is_dirty; prepare_save(observed_disk:&str)->SaveCandidate; acknowledge_saved(SaveCandidate). SaveCandidate owns text/expected_disk/revision with read accessors. Actual host write must recheck baseline atomically.

## Evidence

Direct rustfmt --edition 2024 on all five owned Rust files: exit 0. Cargo build/test: NOT RUN by A5. A1 should add workspace member and run cargo test -p wilde-ide under its existing 2-job build slot. Tests cover split ANSI, every-byte UTF8 and wide emoji/combining cells, alternate screen restore, cursor/resize, ordering/generation/host identity rejection, oversized frame retry, scrollback retention; Unicode edits, failed edit atomicity, undo/redo branching, history budget, save conflict, edits during save and document admission. Fixtures are synthetic model tests, not native/real PTY evidence.

## Review and integration

Independent reviewer: A1/A6 requested; no review verdict yet. PR: none. Integrated SHA: not yet integrated. Dependency/license and build verification remain A1-owned. No terminal emulator/editor parity or performance claim.

## Runtime and cleanup

No task-owned app/PTY/GUI processes, endpoints, production profiles or temp fixture data. Rustfmt completed. No build/cache created by A5; A1 owns F:\wr-build. Worktree retained for integration/review.

## Next

A1 fetch/lock/build; compose A2's owned real ConPTY fixture with this model and A4's native sibling scene. Native grid drawing and visible input/render evidence require isolated display/CI. Durable host snapshot recovery, editor filesystem adapter and full IDE functions remain open.

