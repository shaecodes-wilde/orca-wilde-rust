# First-cycle decisions

The source is shaecodes-wilde/orca-wilde, wilde/theme, 1db4ef38577ec932f97067b7391625b7be599884. Current checkout has 47 changed/untracked entries. The pinned committed snapshot is the initial reference; ongoing working changes are not silently imported. Runtime RPC reference moved from src/shared/runtime-rpc.ts to src/shared/runtime-rpc-envelope.ts plus the session/terminal contract modules. All other selective baseline paths were present.

Shae explicitly chose F:\Orca Wilde Rust and a new repository, so the new project begins with its own immutable bootstrap commits rather than sharing the legacy Git history. A1 integration is the isolated new primary checkout; the five workers use short sibling linked worktrees F:\wr\a02..a06. The package's helper creates them from ce470be8c01a54f68125e4de111cce312cf6ffd6. No existing repository worktree or branch is reset/pruned.

Dependency pins use locally cached stable Rust 1.94.0. Local invocations explicitly select +stable, and CI selects pinned +1.94.0; Cargo.lock and rust-version 1.94 are checked in. The locally installed stable toolchain is used without global changes or a forced download. CI must specify 1.94.0; future updates require rerunning evidence. One build at a time, CARGO_BUILD_JOBS=2, external target F:\wr-build\a01. No six Node/dependency installations.

Actual Claude Code 2.1.285 opus probe returned claude-opus-5-5, and two tightly constrained file-only workers began. Shae then changed all workers to gpt-6.1-sol High. Both owned Claude sessions were stopped by their recorded runner sessions; completed exit receipts and no matching owned Claude processes confirmed. No source files had been written. All five roles now route through Codex collaboration at gpt-6.1-sol/high, staggered at three implementers. A1 was independently verified from its own current-session turn metadata as gpt-6.1-sol/high. All six roles use the requested model and effort.

The source app uses a shared production profile; native tests require a distinct systems.wilde.native-rust.dev identity, explicit native dev profile, separate SQLite application marker, and no default endpoint. No legacy credential/profile data is copied. No daemon listener is launched this cycle.

Joint feasibility begins early: A4 native layout/input/launch admission; A5 reusable VT/editor model; A6 WRY child WebView browser. Native GUI input/focus/DPI/IME/accessibility/cleanup must run on isolated Windows and Linux/Wayland before broad UI conversion. The current desktop is not such an environment. Compilation and headless policy tests leave G1 open; no production toolkit chosen yet. Iced and Slint comparison remains part of R03.

GitHub target preference: shaecodes-wilde/orca-wilde-rust, PUBLIC (Shae's explicit answer). Repository creation and publication require a final exact reviewed local change set. Until that action is approved and succeeds, records remain pending-publication.

