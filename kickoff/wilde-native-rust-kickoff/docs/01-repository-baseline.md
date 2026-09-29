# 01 — Verified starting point

Reviewed 2026-09-29 at `1db4ef38577ec932f97067b7391625b7be599884` on `wilde/theme`. The branch listing also contains a different `main`; do not automatically base the migration on it. The user’s local uncommitted changes and installed binary were not available for this review. Reconcile them before execution.

## Selective source evidence

| Source | Observed behavior | Migration consequence |
|---|---|---|
| `WILDE.md` | Current build shares regular Orca identity/profile; auto-update disabled; hard fork documented | Native dev builds need a distinct identity and profile; never test installer overwrite against the live app |
| `WILDE.md` | Existing local business workspace, n8n reader, push-to-talk navigation, Spotify and OBS integrations | These are existing capabilities to inventory, not greenfield inventions |
| `AGENTS.md` | Hidden/CDP Electron testing; no focus stealing; Windows/WSL/SSH/platform contracts | Preserve operational safeguards; native UI needs its own isolated test environment |
| `package.json` lines 125–178 | Terminal scale, startup, idle CPU, jank, cold-park and comparison commands | Reuse/adapt real benchmark scenarios after validating their safety and prerequisites |
| `src/main/wilde/business/database.ts` | SQLite business commands, revisions, preview tokens, idempotent request tracking, backups; snapshots capped at 10,000 records | Port semantics before optimizing API shape; fixture-test conflicts, stale previews and rollback |
| `src/main/browser/browser-backend.ts` | Desktop webviews and offscreen WebContents feed the same browser manager | Current abstraction is Electron-backed, not yet a browser-engine-neutral contract |
| `src/renderer/src/components/browser-pane/host-guest/browser-guest-worktree-retention.ts` | Default eight warm hidden-worktree pages; protected pages and active-worktree pages excluded | Preserve protections; assess total measured resource cost, not only warm-page count |

The browser policy already evicts individual hidden pages. Do not reimplement the older four-worktree policy described in an earlier conversation. All active-worktree pages are treated as visible by this function; actual per-tab visibility is a potential later refinement, not an assumed existing feature. Page count is not an exact process count or RAM reading. [R1–R7]

## First reads by role

A1: root instructions, README/WILDE, package scripts, runtime RPC types and remote wire compatibility.
A2: `src/main/daemon/`, `src/main/orcad/`, providers, `src/relay/`, child-process wrappers and Windows/SSH references.
A3: `src/main/wilde/business/`, `src/shared/wilde/`, `src/main/wilde/n8n/`, database worker/migrations and tests.
A4: `src/renderer/src/app-shell/`, style guide, Wilde assets/tokens, business screens and shortcut behavior.
A5: terminal-pane, pane-manager, editor/diff, file viewer and e2e terminal rendering fixtures.
A6: `src/main/browser/`, OS/native helpers, `src/main/wilde/spotify/`, `src/main/wilde/obs/`, packaging and perf scripts.

Paths in the first-read list are navigation starting points. Workers must resolve exact current symbols and tests; this package did not inspect every file in those directories.

## Carry forward, not blindly translate

Preserve host-owned process status (`live`, `unverifiable`, `exited`); loss of connectivity is not process death. Folder workspaces are valid without Git. Keep short runtime/socket paths, account separation, safe shell argument handling, credential storage, cancellation and idempotency. Do not rewrite native libraries merely to increase Rust file count.

No full-repository file/line count, speedup estimate or model migration benchmark is used as planning evidence here. No runtime benchmark was executed against the user's installed app. Sources and exact reviewed links: `10-sources.md`.
