# A6 independent review — runtime and verification runner

Reviewed A2 exact HEAD 0b2e20ed445c8c4d7879e823eed9d6f60078d849 on rust-v2/a02-runtime at F:\wr\a02. Authorized cross-worktree reads only: runtime source/tests, source map, handoff; cached portable-pty 0.9.0 native implementation. Reviewed A1 tools/verify.ps1 and current contracts at F:\Orca Wilde Rust. No implementation, build, fixture, GUI or ambient process inspection performed by reviewer.

## Decision

APPROVE A2 source invariants for fixed synthetic/no-descendant fixture admission: explicit local host, fixed argv, cleared environment, bounded identifiers/geometry; owned child under one mutex for polling/kill/reap; PID informational, no ambient/cloned numeric PID killer; detach does not kill; bounded queue fault preserves admitted chunk order and forbids continued frame delivery; independent exit/join/cwd receipt fields and nonrecursive directory removal.

REQUEST CHANGES before claiming bounded cleanup or accepting failure cleanup. Real native happy-path behavior remains a test gate, not a source-proven failure. A1 must run native tests under an outer owned-process watchdog. This is neither production runtime approval nor PTY/SSH/MSYS/process-tree parity.

## Findings

1. High — close/Drop native work is outside the enforceable deadline. src/lib.rs close calls master.take synchronously before checking its deadline; cached Windows PsuedoCon Drop calls ClosePseudoConsole. Queue overflow/read failure exits the native reader; cached backend enables INHERIT_CURSOR and the crate has no cursor responder. Native disposal can block waiting for final output/cursor traffic. The two-second loop deadline and is_finished join checks therefore do not bound all OS teardown. This blocks hard-deadline/fault-cleanup acceptance; it does not establish that the tiny no-descendant happy path will fail. The handoff already discloses native read/master uncertainty, but tests alone cannot turn a loop timer into a general cancellation guarantee.

Safe immediate scope: run the compiled test executable under a watchdog owning its actual process handle, preserve unfinished cleanup as failure, retain receipt and cwd. No taskkill/name scan/saved-PID kill. Followup design may keep a draining native reader after fatal queue overflow, move native disposal to an owned cleanup worker and expose pending completion; API return can be bounded without pretending OS resources are gone. Portable-pty offers no public ConPTY flag control, so inherited-cursor disabling or a real owner-controlled responder requires a separately reviewed backend decision. Do not guess cursor-response strings from a remembered terminal screen.

2. Medium — receipt can lag observation. fixture-lifecycle.rs verdict returns Unverifiable after mutex poison/try_wait error without writing that observation to CleanupReceipt. After supervisor termination, close can return a stale Live/Exited receipt despite its own unsuccessful observation. Update receipt on every verdict observation and cover error/poison behavior with an injected owned child failure. Cached Windows portable-pty additionally converts GetExitCodeProcess failure into Ok(None), which the crate interprets as Live; document that native backend failure remains unverified or add a reviewed native guard before wider status guarantees. Kill result is correctly not accepted as proof of exit.

3. High — verification lease can be bypassed by output-root choice. A1 verify.ps1 lines4-7 locates both mutable target and lease below unchecked WILDE_BUILD_ROOT; two roots create two leases, and relative/C-drive roots bypass the host disk constraint. Anchor the one host lease to canonical F:\wr-build and reject alternate/relative root overrides for this host. CreateNew is sound only when every contender uses that same canonical path.

4. Medium — interrupted compiler ownership needs explicit reconciliation. verify.ps1 releases its lease in finally after a synchronous Cargo invocation; it does not retain a handle/start identity for Cargo and establish that owned compiler work is stopped after interrupted invocation. Hold or retain the lease when cancellation leaves child completion uncertain; an abandoned lease must not be removed by age. Nominal sequential steps, two Cargo jobs, one test thread, no incremental build and background flag are appropriate. This finding is a cancellation safety requirement, not an assertion that an observed compiler escaped.

## Contracts and evidence

Current A1 local pilot contracts preserve the earlier independent approval: bounded pre-decode wire and decoded frame, bounded IDs, authorized-host fence, unknown verdict string to Unverifiable, wrong types refused, generation/sequence recovery without cursor consumption. No real authentication/transport/legacy pairing claimed.

The recorded A1 integrated-test.log inspected during this review shows seven Windows browser policy tests passed (the Unix-only symlink test is the eighth portable test). Corrected A6 handoff count accordingly. Native browser compile pass was relayed by A1; no GUI or input proof. Current SQLite/PTy compilation was in progress according to coordinator, so no runtime test pass is inferred from older artifacts. Reviewed scripts are mutable A1 working changes; this approval/request is tied to captured source, not future revisions.

Official native limitations: [Microsoft pseudoconsole lifecycle and inherited-cursor warning](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session), [ClosePseudoConsole](https://learn.microsoft.com/en-us/windows/console/closepseudoconsole), [GetExitCodeProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess). These support the possibility of blocking native teardown; they do not prove a failure happened on this host.

## Next

A1 acknowledged canonical fixed-root lease repair and outer watchdog. A2 followup should address honest pending cleanup and receipt errors before more general runtime acceptance. A1 retains independent data review authority; A3 was not reviewed in this cycle. Root app/native scene will need a later independent review. Reviewer's worktree retained; no cleanup or external writes.
A1 verify.ps1 reviewed source SHA-256: EF94AAF2053B71544DCCC5267049DA6C90368659B90C3CC690FCBE2C59813BAD
