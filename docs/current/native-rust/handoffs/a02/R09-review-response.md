# A02 response to A6 R1/R2 review

Status: fixture pilot quarantined; production R08 bounded-cleanup acceptance NOT ACCEPTED. No Cargo build or fixture launch by A02. This followup changes only A02 owned files.

## R1: native cleanup is outside the Rust polling deadline

Confirmed from cached portable-pty 0.9.0 source: src/win/psuedocon.rs::PsuedoCon::Drop calls ClosePseudoConsole synchronously. Creation sets PSUEDOCONSOLE_INHERIT_CURSOR along with resize/input flags. The crate's read interface exposes no cancellable read or readiness polling. OwnedFixtureSession::close drops the native master after observed child exit; this native drop may block before another Rust deadline check. A two-second polling budget is therefore not a hard operating-system cleanup deadline. Merely spawning another cleanup thread would move the unbounded operation and would not solve it.

Mitigation implemented: spawn refuses before creating any cwd, PTY or process unless WILDE_RUNTIME_FIXTURE_GUARD=owned-runner-watchdog-v1 is present. A1 acknowledged this exact marker. It is an explicit runner attestation, not authentication or security authority. Only A1's reviewed outer owned test runner may supply it, after arranging independent process ownership, a timeout, captured output and positive cleanup checks. Default app/library use must not enable it. No arbitrary cursor parser/responder, native dependency fork, endpoint or extra cleanup thread is introduced.

After output queue overflow, output remains failed closed and all subsequent native output is drained into the fixed read buffer and discarded. Discarded bytes never enter TerminalFrame or terminal model state. The supervisor still requests termination of the one owned child. The drain test uses a chunked synthetic reader, actually reaches EOF after overflow and verifies that only the pre-fault bytes were admitted. This is a deterministic stream-failure test, not real ConPTY acceptance.

A1's outer-runner prerequisite: isolated fixture-test subprocess; ownership established before execution; on Windows no-breakaway job with kill-on-close plus retained process handle, a bounded watchdog and captured logs. After timeout/exit, stop only owned native processes and positively wait for their death. Retain build lease, scratch cwd and evidence until completion is verified. A timeout is failure evidence, never proof that cleanup completed. This worker does not implement or certify that runner.

Prepared actual PTY tests remain quarantined and NOT RUN by A02. Native startup may wait for a cursor responder; EOF or master drop may stall. Outer watchdog must stop such a test and preserve its logs. Linux/macOS real tests also remain NOT RUN. Neither a successful fixture nor safe outer cancellation establishes production daemon, process-tree or hard native cleanup parity.

## R2: failed observations and receipt consistency

Both poisoned child-owner lock and an actual error returned by Child::try_wait now update CleanupReceipt.verdict to Unverifiable and clear exit_code before returning Unverifiable. The supervisor uses that same uncertainty update. Two deterministic fake-child tests exercise an actual std::io::Error from the Child boundary and an actually poisoned ownership mutex; these launch no operating-system process. Tests are prepared, NOT RUN by A02.

Cached Windows portable-pty 0.9.0 src/win/mod.rs::WinChild::is_complete converts GetExitCodeProcess failure to Ok(None). That means this dependency can return Live without a successful kernel observation. This followup cannot recover that hidden error via the public interface; it records the limitation rather than claiming complete liveness proof. A1's independent retained-process-handle check is required for cleanup acceptance. No PID scan or ambient process kill is added.

## Verified and remaining

PASS, exit 0: owned Rust rustfmt --edition 2024 --check and final staged whitespace check. Cargo and actual process tests NOT RUN by A02. No dependency install, app, GUI, fixture child, root Cargo/lock/contracts edit or old checkout access by A02. The dedicated worktree stays retained. A6 independent rereview and A1 guarded executable runs remain required. No production R08 acceptance or hard native timeout claim.

## A1 actual Windows failure report and next diagnostics

A1 reported that all four original candidate real PTY tests FAILED waiting for WILDE_FIXTURE_READY (about 17.7 seconds total), while unit tests and SQLite tests passed. Coordinator evidence path: F:\Orca Wilde Rust\evidence\integrated-test.log. This worker has not read that cross-owner log or repeated execution. The failing candidate precedes this followup; failures remain failures.

Fixture await_marker now includes admitted byte length and an escaped prefix of at most 4096 raw native output bytes when a read errors, stream ends early or marker wait times out. This is a bounded transcript diagnostic, never lossily decoded text or terminal-state parsing. A1's next guarded run must collect it before interpreting startup behavior.

Inherited-cursor startup is a hypothesis supported by the flag present in cached source, not a confirmed root cause. Proposed narrow dependency mitigation, pending A1 ownership authorization and A6 review: remove only PSUEDOCONSOLE_INHERIT_CURSOR from PsuedoCon::new's CreatePseudoConsole flags for this app-owned virtual terminal. Exact cached upstream package is portable-pty 0.9.0, Cargo.toml declares MIT, LICENSE.md accompanies the source, repository https://github.com/wezterm/wezterm. Root patch composition belongs to A1; no vendor copy or dependency patch is made in this followup. No guessed cursor response.
