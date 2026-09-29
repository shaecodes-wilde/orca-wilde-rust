# A04 — Native UI and business surfaces

Preferred model: **Opus 5.5**. Verify actual runner/model; this label does not configure it.

Read applicable repository instructions, this package's rules, assigned task and relevant contract. Work only inside the assigned registered worktree and owned paths from `coordination/roles.json`. Preserve production profiles and other agents' changes.

Own the native window shell, design tokens, client/project/workspace screens, command palette, settings, dialogs, menus, navigation, keyboard/accessibility behavior and application layout. Preserve Wilde identity using repository assets and tokens. Do not overwrite root style instructions or redistribute private font files.

First task: together with A5 and A6, prove native controls + native terminal + real embedded browser inside one window on the required platforms. Start with two candidate toolkits. Verify actual versions, platform APIs and license implications. A low-memory empty window does not select the toolkit.

Your initial UI must handle selection of a synthetic client/project and a saved workspace without eagerly opening every resource. Build against A1-approved business/runtime contracts. A3 owns domain/storage, A5 terminal/editor widgets and A6 the browser adapter. You own host layout, focus routing, DPI/bounds and surface lifetime coordination. Do not implement parallel clients/databases or a second terminal renderer.

Test focus transfers, menus/overlays above embedded content, resize, scroll, IME, clipboard, accessibility and multi-monitor scale. Use an isolated display/VM/CI for native testing. Never launch focus-stealing windows on Shae's active desktop; the reference app's Electron flag does not automatically apply to a Rust executable.

After the composition gate, create native client→project→workspace navigation and controls. No React, web-shell or hidden Monaco/xterm fallback can be presented as final native completion. Temporary bridges must be explicitly labeled with retirement conditions.

Return toolkit comparison evidence, actual OS/backend results, accessibility/input gaps, component contracts, changed paths/commit, test evidence and the next small screen increment. Have A5 and A6 review the integrated scene before broad UI conversion.

Use `templates/HANDOFF.md` for results. Mark planned, mocked, not-run and verified distinctly. Read broader documents only as the task needs them.
