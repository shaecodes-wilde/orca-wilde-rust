# A2 cleanup phase diagnostic

Observed primary run: `evidence/alpha-runs/alpha-20260929-02/guarded-tests`, source `5c54c4a`. The runtime sustained native sink test passed with 18,874,822 actual bytes, 74,020 contiguous frames, unchanged PID/identity, natural exit zero, both workers joined, cwd removed and final fault absent. Existing fixed-lifetime cleanup and app continuous-terminal cleanup failed with `CwdRemovalError { kind: TimedOut, raw_os_error: None }`. The guard reported no timeout/forced children and an empty owned job.

## Confirmed versus unknown

`fixture-directory.rs::remove_before` creates this TimedOut error when the original close deadline has elapsed before a directory mutation. This is not an observed OS sharing error or evidence of unexpected retained contents. `close` assigns the two-second deadline after writer drop, then observes the child, synchronously drops the master, joins finished workers and attempts the manifest. Vendor `PsuedoCon::Drop` calls `ClosePseudoConsole`. Existing logs lack phase timings, so the exact stage that consumed the budget is unknown; whole-binary times around 35 seconds cannot identify it.

Current machine reports OS build 26200 and kernel32 file version 26100.9278; no `conpty.dll` is beside the tested dependency binaries or their parent debug directory. Microsoft documents immediate `ClosePseudoConsole` return starting build 26100, so older blocking semantics must not be asserted as this run's root cause. Source: https://learn.microsoft.com/en-us/windows/console/closepseudoconsole (read 2026-09-29).

## Diagnostic-only change

A1 acknowledged `CleanupReport.timing: CleanupTiming`, using fixed `Duration` fields for writer drop, verdict observation, master drop, worker join, manifest removal and total close, plus `deadline_exhausted`. Repeated loop phases accumulate saturating durations; each close call records its own timing. App failure Debug receipts include all fields automatically. Total close includes writer drop and polling sleep; the original deadline placement remains after writer drop. Timing is measured wall time, not a hard cancellation guarantee.

No deadline reset/extension, additional retry, cleanup rule change, native teardown thread, vendor modification or retained-directory removal is included. Existing guards, exit assertions and directory-safety checks remain unchanged. No clock-mirroring unit test was added: the focused real failing cases provide the useful observation boundary.

A2 ran formatting and diff checks only; new diagnostic executable evidence is NOT_RUN. A6 reviews this source before A1's focused owned-watchdog run. Recommend capturing the two failing cases and existing sustained sink case with exact source/run binding. A repair proposal depends on those measured phase receipts; production hard cleanup remains unaccepted.
