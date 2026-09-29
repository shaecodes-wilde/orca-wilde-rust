# A02 guarded-run assertion followup

Coordinator-reported actual candidate 22d09ec: guarded runtime pilot PASS, including native READY/PONG/DONE, natural exit 0, reader/lifecycle completion and exact fixture cwd removal. Three lifecycle tests passed. These are A1 actual outcomes; A02 launched no build, fixture or GUI.

Two remaining actual failures are retained as failures: Windows symlink fixture creation on F returned raw OS code 1 (Incorrect function), so the link-admission test failed before it could exercise rejection. The detach/resize/natural-exit integration reported Some(3221225786) instead of Some(0) after cleanup. The cause of the exit-code change is not confirmed here.

## Changes and limits

The symlink test classifies ONLY observed known Windows creation-unavailable codes 1 and 1314 as explicit NOT RUN. It prints the original error, ErrorKind and raw OS code; the NOT RUN case must stay visible in A1's --show-output captured log. This is not evidence that native link or reparse rejection works. Other creation errors still fail. No alternative filesystem, global setting, privileges, native junction helper or stub is introduced. Cleanup of the test's ordinary empty owned directories remains unchanged.

The natural-exit test already contained a one-second verdict wait before close in A02's prior source. It is strengthened to an explicit two-second helper that requires observed Exited AND receipt Some(0) before cleanup. A nonzero observed code immediately fails with the complete receipt. The original Some(0) assertion after cleanup remains. This is a bounded natural completion check, not a delay that converts a failed or killed process into success. If the code changes during close, the test still fails; no hard native termination claim is added.

A6 source review precedes A1's next guarded actual run. Real tests and strict clippy NOT RUN by A02. Owned rustfmt and staged whitespace checks recorded at completion. Vendor, runtime process policy, profile manifest, historical roots and old app are unchanged. Production R08 hard native cleanup remains NOT ACCEPTED; the owned outer runner is still required.
