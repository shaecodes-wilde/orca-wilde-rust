# A02 cursor-flag vendor experiment

A1 authorized the exact cached portable-pty 0.9.0 package under crates/wilde-runtime/vendor/portable-pty. A6 approved the reversible cursor-only source experiment. No build or fixture launch by A02.

Observed evidence, reported by A1: all four guarded real Windows fixtures emitted only four admitted bytes, escaped \x1b[6n, before missing-ready-marker timeouts. This is the native cursor query; causation remains a hypothesis. A1 must test the changed dependency under its owned runner and record actual outcomes. Existing failures are not converted to passes.

Only src/win/psuedocon.rs::PsuedoCon::new changes: remove PSUEDOCONSOLE_INHERIT_CURSOR term from CreatePseudoConsole flags; retain resize quirk and Win32 input flags. No API, native lifecycle, terminal parser, process policy or runtime configuration change. Copy verification found exactly this one file differs from cached upstream. Original 18-file hashes, registry archive SHA-256, VCS commit and unmodified MIT license accompany the vendor copy.

A1 root composition proposal:

    [patch.crates-io]
    portable-pty = { path = "crates/wilde-runtime/vendor/portable-pty" }

A1 owns root Cargo/lock and should exclude this upstream package from workspace-wide formatting/lint restyles if Cargo automatically treats a path dependency as a member. Upstream normalized Cargo.toml uses only paths inside its own package; no escaping dependency path. Do not use Cargo.toml.orig as the active manifest. Vendor source was not reformatted; the exact one-flag source delta is preserved.

The guarded-only execution marker, discard-only overflow draining, shared receipt uncertainty and genuine observation-error tests remain unchanged from 430453144346e9a7a84433021284f174900c602c. Synchronous native ClosePseudoConsole and lack of read cancellation remain outside the Rust polling deadline. Production R08 bounded cleanup stays NOT ACCEPTED regardless of startup result.

Source checks prepared: exact cached-file hash comparison, source delta review and staged whitespace. Full build, unit/real fixture tests, macOS/Linux checks and A6 integrated runtime review are NOT RUN by A02. A1 preserves an outer ownership/watchdog lease until fixture processes are positively stopped; no ambient PID/name kills. Old app, profiles, sessions and global cache remain untouched.

Expected drain consequence, not measured: if inherited-cursor startup is the cause, removing that request should allow fixture READY output without inventing a cursor responder. Once any queue fault occurs, the existing reader continues draining into its fixed buffer and discards the remainder while the owned child is stopped. This may reduce native shutdown pipe pressure, but does not prove bounded native EOF or ClosePseudoConsole.

A1 reported the failed guarded candidate test jobs ended at actual zero owned processes, without forced surviving children. The first failed fixture receipt had reader_joined=false and cwd_removed=false; its empty scratch cwd is retained. Job-zero is separate outer ownership evidence and does not retroactively make those Rust receipt fields pass. No ambient cleanup or directory deletion is performed here.
