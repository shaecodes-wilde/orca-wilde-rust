# A6 handoff — browser policy and native embedding compile spike

Task / role: R05 / A6 browser/platform.
Actual model / runner: GPT-6.1-Sol High / native Codex subagent, coordinator routing (not independently introspected).
Worktree / branch: F:\wr\a06 / rust-v2/a06-platform. Base ce470be8c01a54f68125e4de111cce312cf6ffd6. Exact final commit supplied in coordinator message; this file is included in that commit.
Base contract: native pilot protocol major 1. Source baseline: 1db4ef38577ec932f97067b7391625b7be599884.
Status: IMPLEMENTED, checks delegated to sole A1 build runner. NOT native embedding acceptance.

## Result in plain English

Added a small browser crate with marked explicit dev profiles, strict navigation policy, owner-scoped page cleanup authorization, finite contained bounds and protected-page eviction decisions. Optional Windows wrapper borrows an existing Win32 parent on the owning main thread and constructs only fixed CSP-locked synthetic HTML. It exposes no website/RPC/focus/window launch; isolated owner may make its child visible without parent activation. First about:blank only is admitted for NavigateToString; browser input/textarea fixture is included. Website navigation is unsupported until actual native permissions are enforced.

## Implementation

Owned paths only: crates/wilde-browser/src, tests/browser_policy.rs, Cargo.toml and docs/current/native-rust/recon/a06 plus this handoff. A1 acknowledged profile/scene boundary. Proposed root membership and exact optional target dependencies wry=0.54.4, raw-window-handle=0.6.2; no root Cargo or lock edits by A6. A4 owns bounds/DPI/native controls, A5 terminal sibling, A1 native event loop/parent lifetime. No native toolkit selected by this spike.

## Evidence

Inspected cached wry-0.54.4, tao-0.34.8 and raw-window-handle-0.6.2 APIs plus official upstream docs; links and actual limitations in recon/browser-feasibility.md. rustfmt --edition 2024 over owned Rust files: exit 0. No dependency install or build by A6.

A1 to run with CARGO_BUILD_JOBS=2 and CARGO_TARGET_DIR under F:\wr-build:
- Set WILDE_BROWSER_TEST_ROOT=F:\wr-build\a06\evidence; cargo test -p wilde-browser (8 Windows headless tests; extra Unix symlink test).
- cargo check -p wilde-browser --features windows-child (compile only).

Tests cover profile markers/traversal/revalidation, strict origin escape rejection, cleanup ownership/duplicate IDs, all protected retention reasons, cooldown/clock uncertainty, invalid bounds and launch refusal. Fixtures use synthetic data and remain in the assigned evidence root. Permission policy does not claim backend enforcement.

Windows GUI: NOT RUN (user desktop forbidden, isolated display unavailable).
Linux/Wayland GUI: NOT RUN (generic WRY child API unsupported; GTK/native composition unproved).
macOS: NOT RUN (no runner).
Browser native compile/headless tests: NOT RUN by A6; A1 sole runner to report exact output.
Benchmarks/process memory/installer/integrations: NOT RUN; no speedup or parity claims.

## Review and integration

A6 independently reviewed A1 substantive contracts, requested four boundary repairs, then reviewed repaired F:\Orca Wilde Rust artifact and APPROVED local pilot semantics only. Unknown verdict string=>Unverifiable, bounded IDs/wire/payload, authenticated-host argument fence and cursor recovery are acceptable. Caller must actually authenticate host; transport and legacy compatibility unproved. A1 must review A6 code and run checks; not integrated at handoff. A2 safety review queued for later coordinator turn.

## Runtime and cleanup

No task browser, native window, event loop, CDP endpoint, profile, session or integration connection created. Only scoped command processes for inspection, formatting and git. No ambient processes killed or enumerated. Browser engine processes may outlive controller close: no profile deletion exists. Build evidence belongs under F:\wr-build, never C. Retain worktree; no removal, recursive cleanup or production changes.

## Next

A1 serial headless/Windows compile checks and independent code review. Then isolated native scene gates with A4/A5. Native permissions, WebAuthn/OAuth, popup/download/media, IME/accessibility and Wayland/macOS support remain substantive blockers before any website feature. No Shae approval requested for this reversible local spike.
