# 04 — Dependency-driven delivery

Milestones are evidence gates, not calendar promises. Multiple tracks can advance concurrently whenever dependencies and ownership permit.

| Gate | Deliverable | Proof to advance |
|---|---|---|
| G0 — Safe baseline | checkout/feature inventory, isolated profiles and branch plan, actual worker routing | live app untouched; exact base and known restrictions recorded |
| G1 — Native feasibility + contracts | small Cargo foundation, native UI/terminal/browser experiment, transport/schema decision | build/test on available OS; real embedded surface on Windows and Linux/Wayland before broad UI selection; macOS validation status explicit |
| G2 — Useful vertical slice | synthetic client/project/workspace, real native PTY terminal, persistence and restart | deterministic end-to-end tests, independent review, no live-profile writes |
| G3 — Runtime/domain parity | process ownership, background sessions, Git/SSH/WSL, business commands/backups | captured fixtures, real OS tests, fault/reconnect coverage, existing API compatibility or reviewed version adapter |
| G4 — IDE/browser feature coverage | native editor/diff/explorer/previews, browser profiles/automation/downloads and Design Mode | feature register complete for accepted scope; no hidden web-shell substitution |
| G5 — Everything App integration | n8n history, Spotify/OBS/voice, resource center | fake-provider tests plus separately authorized live smoke; integration disable/reconnect cleanup |
| G6 — Release candidate | resource/soak tests, signed packaging where required, import/rollback rehearsal | supported-platform evidence, no unresolved critical regression, explicit Shae cutover approval |

## Initial parallel wave

A1: inspection and ownership → minimal contracts/workspace/CI.
A2: existing session/PTY behavior inventory → fixture process lifecycle.
A3: business fixtures/preview/idempotency inventory → safe import parity pilot.
A4: toolkit shell/input spike paired with A5/A6.
A5: terminal parser/rendering/IME spike, then native editor baseline.
A6: current benchmark map + browser/native-window integration spike.

The team must not wait until G4 to discover that the toolkit cannot embed the required browser. G1 blocks broad UI conversion, not independent domain or runtime work.

## G2 acceptance scenario

Use synthetic owner `Wilde Demo`, client `Demo Solar`, project `Checklist Demo` and a throwaway working directory. Save an assignment without opening a process. Open the workspace in a native window, spawn a harmless fixture shell, type, resize, run a deterministic output command, hide/show the terminal, disconnect/reconnect the UI and verify documented session behavior. Save a note, restart into the isolated profile, and verify IDs and assignment persistence. Then prove browser opening, input and disposal separately if not yet composed into the slice.

One happy-path demonstration is not G3 parity. Real agent launch is a later optional smoke test with user-authorized tooling, not the first foundation dependency.

## Keep progress honest

`planned` → `ready` → `in-progress` → `review` → `integrated` → `verified` → `retired-old-path`.

Use `blocked` or `not-tested` where evidence is unavailable. A mocked browser is not browser embedding; a process echo test is not terminal emulation; a data table is not the complete CRM. A good baseline result is not a guaranteed whole-app improvement. A poor early result calls for profiling and a revised design, not automatic abandonment or a claim that Rust cannot meet the goal.

## Scope lock

Desktop Windows/macOS/Linux compatibility remains in scope. Web/mobile companion rewrites are not part of the initial native desktop conversion; preserve wire compatibility or explicitly track a migration adapter. Existing helper languages can be retained behind reviewed adapters until their replacement is justified. Do not retire features, platform support, notices or profile compatibility without a recorded decision.
