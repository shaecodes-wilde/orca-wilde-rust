# A4 native UI handoff

Task / role: A4 native UI/shell policy and first-scene proposal.
Actual model / runner / session: GPT-6.1-Sol High as assigned by coordinator; Codex delegated agent a04_native_ui. Kickoff Opus label is overridden, not model configuration.
Worktree / branch / exact implementation HEAD: F:\wr\a04 / rust-v2/a04-ui / daf4c85000d357aa6228f3f61d72fd424cd029e8.
Base contract version: ce470be8c01a54f68125e4de111cce312cf6ffd6; no wilde-contracts dependency or contract changes.
Status: IMPLEMENTED HEADLESS POLICY, FORMATTING VERIFIED; COMPILE/TEST/GUI NOT RUN by A4.

## Result in plain English

A1 now has a small dependency-free layout, focus-intent and development-launch gate to compose a diagnostic native scene. Tiny windows suspend all child surfaces instead of producing overlapping or zero-sized child bounds. No native UI toolkit is selected and no app is launched.

## Implementation

Owned changes: crates/wilde-ui/{Cargo.toml,src/{lib,layout,focus,launch}.rs,tests/scene_policy.rs} and docs/current/native-rust/recon/a04/native-ui-policy-and-candidates.md. This handoff is a later documentation-only commit.

A1 acknowledged the proposed API and suspension policy. SceneLayout::from_physical receives u32 physical client dimensions and f64 DPI scale. Its bounds(Region) yields validated private LogicalBounds with x()/y()/width()/height() getters. FocusRoute::step returns an optional Region without executing focus effects. LaunchRequest/admit/LaunchPermit enforce policy only; A6 must inspect the canonical profile root and exact native marker, and A1 must verify actual isolation before GUI composition. The exact dev identity is systems.wilde.native-rust.dev; root name is wilde-native-rust-dev-profiles.

Consumer impact: A1 must register the crate in root Cargo.toml, add application dependency wiring, and integrate A6 browser adapter plus A5 terminal. A4 deliberately did not edit root Cargo/lock, apps, shared contracts or other owners' paths. No production fallback exists in the crate.

Iced 0.14.0 and Slint 1.18.1 official comparison, licensing, native handle constraints and design role translation are linked in the reconnaissance document. Neither candidate was installed or built. A Tao/Win32 native control diagnostic is proposed as a reversible integration step, not a permanent toolkit or terminal-completion claim.

## Evidence

- rustfmt --edition 2024 on the five owned Rust files: exit 0.
- rustfmt --edition 2024 --check on the same files: exit 0.
- git -c safe.directory=F:/wr/a04 diff --cached --check before implementation commit: exit 0.
- Targeted local metadata inventory found WRY 0.54.4 (Apache-2.0 OR MIT, Rust 1.77 metadata); no Iced/Slint package found in that source-cache inventory.
- Official primary documentation read on 2026-09-29; links preserved in reconnaissance.
- Nine runnable test cases authored for resize, suspension/restore, DPI, signed-coordinate overflow, focus route and launch rejection; NOT RUN by A4.
- Fixtures are synthetic dimensions, paths and strings only. No real profile created or accessed by policy tests.
- Actual OS/backend executed: none. Windows workstation performed file reads/writes/formatting only.
- NOT RUN: crate compile/tests, GUI/input/IME/accessibility/Wayland/macOS, embedded browser scene, physical DPI placement and native terminal rendering. No mock or parity evidence claimed.

Recommended A1 command after member registration: cargo test -p wilde-ui in A1's exclusive build slot with CARGO_BUILD_JOBS=2 and CARGO_TARGET_DIR outside worktrees. This is a recommendation, not an executed check.

## Review and integration

Reviewer: A1 owns integration/review; A5/A6 integrated scene review remains pending. Implementation commit sent to A1; integrated SHA unknown when this handoff was written. No PR/publication.

Risks: profile path syntax alone cannot detect links/junctions or validate the marker; flags do not prove an isolated display; physical rounding must share checked edges; child webview z-order/focus and Wayland composition remain unresolved. No performance measurements or native user-input acceptance exists.

## Runtime and cleanup

No Cargo/rustc/build/install or GUI process launched. No task-owned endpoints or profile data. Formatting commands completed. No cache/target directory created by A4. Dedicated worktree and commits retained; no cleanup performed.

## Next

A1 can cherry-pick implementation daf4c85000d357aa6228f3f61d72fd424cd029e8, register the crate and run its headless tests. Then compose the compile-only Tao+Win32/A5/A6 Windows diagnostic without launching on the user desktop. Render/input/DPI/IME/accessibility and teardown proof requires a truly isolated display/VM/CI. After composition, reuse A3 contracts for synthetic client -> project -> explicitly opened saved workspace; preserve folder and remote host behavior. Permanent toolkit/distribution-license decision remains open until scene evidence and terms review.
