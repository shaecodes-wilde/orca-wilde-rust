# A6 source-only review: bounded supervision and early failure diagnostics

Source-only approval of A2 immutable 0bac5a5f64b680ccb8a9010fd741114ebd3cd40a and A1 immutable b3ef2a0404bd40115cc725d1863390e06242fc35. Reviewed exact diffs and surrounding lock/guard flow. No A6 build or executable run. These diagnostics explain observations when captured; they do not establish a repair, root cause or reliable acceptance. Unchanged-source alpha-05 fails after alpha-04 passed; see alpha-04-05-headless-repeat-evidence-review.md.

A2 adds fixed-size CleanupReport.supervision fields. Existing supervisor-owned child poll still precedes the existing stop/fault kill decision; the child ownership lock is retained through kill, then dropped before the existing stop-deadline check and sleep. Eight-second fixture lifetime retains the same initial Instant, and two-second cleanup/stop budgets are unchanged. No fixture, vendor kill implementation, worker, assertion, allowlist/environment or deadline changes. Receipt closures do bounded updates only. The new per-loop receipt lock is released before child acquisition, with no reverse lock order introduced.

first_kill captures existing decision-time elapsed, current stop/fault snapshots and optional bounded adapter-returned error. Atomic flag reads are separate observations, not an atomic joint snapshot. None can mean pending adapter call or masked/no returned error; it never proves OS termination. first_exit records only the supervisor's first positive owned-handle exit status; a caller can observe exit earlier. Both elapsed fields use supervisor start. Receipt-lock acquisition/scheduling can delay publication. max_poll_gap covers intervals between loop beginnings, including sleep and operations; it cannot identify a delayed phase. There is no per-poll output or terminal payload retention. Supervision persists while per-close timing can be replaced by a subsequent Drop call. Fault mapping0..5 is explicit in A2's handoff.

A1 switches the existing guarded libtest invocation from --show-output to --nocapture so early panic output reaches the same file sink before later watchdog termination. One test thread, file redirection, private owned job/process group, target admission and timeout/termination rules are unchanged. No guarantee follows that every future truncated failure will have a complete receipt.

A1's continuous pilot logs monotonic invocation stages, pre-close natural receipt and returned-close receipt. Only a failed sentinel branch probes the model with try_lock and prints admitted byte count, sequence and sentinel boolean; lock unavailability yields None. It prints existing atomic stream fault and bounded cleanup snapshot before returning the same error/Drop flow. Model content is inspected only for sentinel boolean; terminal payload is not logged. Existing snapshot uses a short mutex; the model acquisition is nonblocking, not a claim that every diagnostic operation is wait-free. Existing pilot gains pre-close and post-close observations. Original seven-second sentinel, eight-second lifetime, two-second close limits, callback and acceptance assertions remain intact. Logging adds finite runtime work, not a deadline extension or performance measurement.

Next A1 guarded diagnostic evidence must retain failures and distinguish kill request/adapter return from positive signaled exit and final owned-job emptiness. No blind retry, timeout reset or natural-zero assertion relaxation is approved.

| Reviewed file | Working bytes SHA256 matching immutable source |
| --- | --- |
| A2 crates/wilde-runtime/src/lib.rs | CC0B45701174F6497402E0423865A754B9FBCD315F7BCCC39BE283F33CB389BE |
| A2 crates/wilde-runtime/src/fixture-lifecycle.rs | 5F1B9331BEC0621FBBDF55855EF100436DAC7ABFCFB1B33972A8D8D78822EB39 |
| A1 apps/wilde-desktop/src/lib.rs | 080FED73D829B97FF5B76C554D5D44491BDCB635A73A8DFA743395B638BE89D9 |
| A1 tools/guarded_tests.py | B81DAC5DED35BE7AF59404CF632A139D5CAF3C675A6A9FE90C272C5D3E169183 |
