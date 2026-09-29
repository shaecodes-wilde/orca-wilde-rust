# A05 — Native IDE surfaces

Preferred model: **Opus 5.5**. Verify actual runner/model; this label does not configure it.

Read applicable repository instructions, this package's rules, assigned task and relevant contract. Work only inside the assigned registered worktree and owned paths from `coordination/roles.json`. Preserve production profiles and other agents' changes.

Own the native terminal emulation model/renderer, scrollback and selection, native code editor, file explorer presentation, diffs, review UI, Markdown/PDF/image/diagram preview integration. A2 owns PTYs and filesystem execution; A4 owns app layout. The first task is not to rebuild Monaco wholesale.

First task: capture existing terminal/editor behavior and evaluate reusable native components. With A4/A6, run the integrated native-window scene. Prove a real byte stream, alternate-screen/cursor behavior, Unicode widths, emoji, resizing, keyboard/IME and selection in a native terminal surface. Use captured, scrubbed transcripts instead of imagined agent screens.

Jointly define canonical terminal-model ownership with A2. Bound scrollback/retained history and implement recovery after stream gaps; do not silently drop arbitrary terminal bytes. UI parking is distinct from PTY termination. Verify hide/show and reattachment under output pressure.

Create a minimal native editor demonstrating open/edit/undo/save/conflict handling, then inventory language services, search, large-file behavior, syntax, diffs and missing capabilities. A native text box is not editor parity. Mature libraries are preferred to new parsers/renderers; audit dependencies/licenses with A1/A6. Browser-based editor/terminal fallbacks remain migration bridges, not full-native acceptance.

Tests must cover real native input and rendering on the available OS, deterministic parser fixtures, file conflicts and memory/resource retention. Keep bitmap/screenshot evidence secondary to behavioral tests. Share A4 widget/surface contracts and A2 required transport/snapshot semantics before cross-owner changes.

Return reused component candidates with actual versions, transcript fixtures, native scene evidence, exact implemented feature subset, missing parity items, tests/commit and one clear explanation of how bytes become a terminal screen.

Use `templates/HANDOFF.md` for results. Mark planned, mocked, not-run and verified distinctly. Read broader documents only as the task needs them.
