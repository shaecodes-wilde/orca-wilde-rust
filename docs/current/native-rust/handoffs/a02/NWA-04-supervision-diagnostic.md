# A2 bounded supervision diagnostic

Repeat alpha-20260929-05 failed on unchanged integrated source `89f5d43`; alpha-04 had passed. Neither the prior exit-code flake nor cleanup timeout is established as fixed. A2 read the exact alpha-04/05 guarded logs and current source; no build or fixture ran here.

Confirmed alpha-05: continuous output sentinel deadline failed (25.30-second whole test), pilot observed exit 1 with total close 380.6 ms and no cleanup budget exhaustion, runtime fixture binary was killed at the guard timeout before libtest could print captured failure details. The guard confirmed every owned job empty. Whole-test time does not identify the delayed stage; host load was not measured and is not asserted.

The pilot waits one second after DONE for positive natural exit, then calls `close` even when exit remains unobserved. Close requests stop; the supervisor's existing native termination code is 1. Thus a delayed natural exit followed by explicit termination is a plausible explanation for the pilot result, but code 1 alone does not prove the kill source. Continuous failure can spend additional time in setup/input/verdict or Drop; its generic error currently omits progress and runtime authority state.

## Acknowledged minimal record

A1/A6 acknowledged additive `CleanupReport.supervision: SupervisionRecord` before coding. It contains:

- `first_kill`: the existing supervisor kill decision recorded before calling the adapter, with elapsed time since supervisor start, the stop flag observed at that decision, the current fault code, and bounded adapter-returned error kind/raw OS code.
- `first_exit`: the first positive owned-process exit status observed by the supervisor, with elapsed time from the same supervisor start and exit code. The caller may have observed exit earlier; this is not a global first-observation claim.
- `max_poll_gap`: the maximum interval between beginnings of existing supervisor loop iterations. It measures the loop interval, including sleep and scheduling/operation delays, and does not identify their cause.

Fault codes: 0 = no latched stream fault, 1 = raw queue overflow, 2 = native read failure, 3 = fixed fixture lease expired, 4 = canonical consumer error/panic, 5 = stream sequence exhaustion. These are runtime codes, not OS exit codes.

An absent adapter error does not prove OS termination success: the vendored kill adapter masks errors, and the record is published before the call, so a call that has not returned also has no returned error. Positive owned-handle exit observation and outer job verification remain the authority. A2 does not change the adapter kill implementation.

The new record uses only fixed-size enums/options/durations; it stores no terminal payload or identifiers beyond the existing receipt and emits no per-poll logs. Existing loop conditions/order, eight-second fixture lease, two-second cleanup deadline, stop-decision timing, workers, launch allowlist/environment and natural-zero assertions remain unchanged. The lifetime origin uses the same initial Instant as before; no reset or extension is introduced. Supervision survives subsequent close calls; existing `timing` remains per-close and can be replaced by Drop.

## Next evidence

A6 reviews the immutable diagnostic diff. A1 owns the runner's `--nocapture` change so timeout logs retain failure context and owns early continuous failure progress/fault/receipt reporting before Drop. Capture pre-close natural verdict/code/deadline exhaustion, supervisor kill decision, positive exit observation and cleanup phase receipts in focused guarded cases. No blind repeat or acceptance relaxation is proposed.

A2 verification: formatting and whitespace checks only. Executable diagnostic evidence remains NOT_RUN. No retained-directory removal, source-reference edit, dependency/global configuration change or production launch occurred.
