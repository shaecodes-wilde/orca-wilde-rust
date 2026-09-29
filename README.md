# Wilde Native Rust

Wilde's native desktop migration lives in this separate repository. The destination is a native first-party shell, business UI, terminal and editor, with Rust owning application/runtime logic. Browser engines remain permitted dependencies.

This is a development pilot, not an installer or production replacement. The existing application, profiles, credentials and sessions stay with the original application. Source provenance: shaecodes-wilde/orca-wilde, wilde/theme, commit 1db4ef38577ec932f97067b7391625b7be599884. Selected public source references and MIT licensing are preserved in source-reference/.

The pilot contains versioned host/session contracts, synthetic client/project/workspace records backed by real SQLite, an allowlisted native PTY fixture, a mature VT parser, bounded UTF8 editing/undo/save-conflict logic, native geometry/focus/launch policy, and a Windows-only diagnostic Tao/Win32 shell with an embedded WRY browser page. The diagnostic terminal shows parser text in a native control; full grid rendering, interactive input and permanent toolkit selection remain gated.

Use the durable first-cycle report under docs/current/native-rust/handoffs/a01/ for actual pass/fail/not-run evidence. No performance improvement or cross-platform parity is claimed.

On the verified Windows setup, Python 3.10+, installed stable Rust 1.94.0 with Rustfmt/Clippy, and the existing native compiler tools are required. Fetch pinned host dependencies with `cargo +stable fetch --locked --target x86_64-pc-windows-msvc`, then run:

```powershell
./tools/verify.ps1 -WindowsChild -WindowsScene
```

This formats/checks, builds tests without running them, executes test binaries inside private owned Windows jobs, and checks both native feature compilation and lint. It sets ORCA_BACKGROUND_LAUNCH=1 and launches no GUI. The shared build lease prevents concurrent worktree builds; output goes to the primary repository's sibling wr-build/a01 directory with 2 jobs and incremental compilation disabled. An uncertain cancellation retains the lease. Never remove a lease by age or kill a process by a saved numeric PID.

Fixture launch requires the reviewed owned-runner attestation. Run the integrated pilot through its guarded test rather than setting that flag for an unowned direct process. Synthetic assignment records share the fixture session's workspace identity; real user workspace provisioning and durable daemon restart/reconnect remain future tasks.

The native scene requires an independently verified isolated display, explicit development application/profile identity and an owned fixture runner. Flags alone do not prove isolation. Never launch it on the user's active desktop. Browser profile data must be separately provisioned beneath a marked wilde-native-rust-dev-profiles root; no default production profile is accepted. General website permissions/navigation remain unsupported in this finite browser experiment.

The manual-only GitHub workflow prepares repeatable headless verification and preserves evidence. It does not run on push, open a native window, release an installer or deploy anything. Linux/Wayland and macOS acceptance, real browser focus/IME/accessibility, production cleanup deadlines, comparable performance and the parity register of 40 capabilities remain open.

A1 owns integration/contracts/apps; A2 runtime; A3 domain/storage; A4 native UI; A5 IDE; A6 browser/platform/resource review. All six roles use Shae's requested GPT-6.1-Sol at High. Ownership, dependencies, gates and review receipts are recorded in coordination/ and docs/current/native-rust/.
