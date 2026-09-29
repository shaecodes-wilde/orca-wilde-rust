# A06 — Browser platform and performance

Preferred model: **Astra**. Verify actual runner/model; this label does not configure it.

Read applicable repository instructions, this package's rules, assigned task and relevant contract. Work only inside the assigned registered worktree and owned paths from `coordination/roles.json`. Preserve production profiles and other agents' changes.

Own browser backend/embedding, profiles/permissions/downloads/automation, platform/Spotify/OBS/voice adapters, resource governor, performance harnesses and installer validation. A1 owns composition/contract authority; A3 owns n8n business data. Your first priority is browser/native UI feasibility plus baseline evidence, not every OS integration at once.

With A4/A5, put a real browser page inside the native scene. Do not equate CDP control with embedding. Verify native child surfaces or offscreen composition, input/focus, DPI/resize, keyboard/IME, overlays, cleanup and browser process cost. Explicitly test Windows and Linux/Wayland; record macOS runner status. WRY generic child-window support has platform caveats—verify the exact backend and integration, not just crate compilation.

Preserve profile identity, permissions, download/audio/automation/remote-view protections and isolation from native RPC. Default warm hidden-page budget in the reviewed app is already eight; the old whole-worktree policy is not the current baseline. Active/protected pages are outside that count. Use real unique process metrics and clear attribution rather than assuming one page equals one process.

No arbitrary live-state handoff between browser engines. Switching backend means explicit reload/re-auth if necessary. No disabling sandbox/cert validation or exposing unauthenticated CDP. A controlled browser service is not zero-cost: measure capture/render/network overhead and texture/input behavior.

Construct comparable release-mode workloads and preserve raw samples/metadata. Unit-test resource policy with protected dirty/unknown-unsaved pages, media, downloads and leases. Use cooldowns to avoid repeated eviction/reload. Keep persistent session authority separate from optional visual surfaces.

Later, implement OS/integration adapters against fakes before authorized live tests. Protect current Spotify/OBS/n8n/account state. Develop a separate dev installer identity; never install over the live app or publish a release without approval. Independently review A1's substantive code and benchmark acceptance decisions.

Return native embedding evidence, platform/capability matrix, baseline results or precise not-run reasons, security constraints, task-owned process cleanup, exact commit and next ready task.

Use `templates/HANDOFF.md` for results. Mark planned, mocked, not-run and verified distinctly. Read broader documents only as the task needs them.
