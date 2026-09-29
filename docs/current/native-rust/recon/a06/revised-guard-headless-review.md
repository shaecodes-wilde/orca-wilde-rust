# A6 followup — revised test guard and headless app

Read-only review; no tests/build/native API calls/GUI. Revised guard reviewed at SHA-256 091FFC2839C230EF039ED049E4AD6E184BC4DF3A78B76CEE4CAA1A4F71EEC34B. Verifier reviewed at C584B84411D03708526BB60A014578129973BD7C634E38651C8C2228996C36A1. Initial unformatted headless app lib hash D29AB7FCC603F1559336AA6210F3ADEB78701D74F82267CD7A83068DBF5BC525; later formatting was inspected without substantive difference. These are mutable A1 artifacts, not fixed commits.

## Windows guard repair

APPROVE guarded synthetic test execution mechanism at source level: failed job-assignment child is terminated using its owned handle with checked result and observed wait completion; assigned exception terminates its private job, waits process completion, queries active count to zero and records explicit false uncertainty on cleanup failure. Normal timeout/remaining-child paths refuse pass. Child remains suspended until successful job assignment, with CREATE_NO_WINDOW and no breakaway. No PID/name kills. Guard run ID and incomplete receipt are written before artifact parsing/native execution; atomic pending-file replacement updates records; completed is set only for deliberate successful/failing return. This supersedes the prior unchecked failure-cleanup request.

## Remaining verifier change

REQUEST CHANGE for fail-closed receipt parsing. Current verifier promotes ownedStepExited=true immediately after Python returns, then parses results JSON; malformed/unreadable JSON throws before false is set, so finally deletes its lease. Set false before parsing and promote true only after matching run ID, completed=true, nonempty records and each owned_job_empty having actual bool true. Fresh IDs/completion markers already repair stale/partial receipt reuse. Repaired guard itself is suitable for coordinator-owned synthetic execution; final verifier approval depends on this default-false parsing line. POSIX null cleanup verdict remains NOT RUN/unverified and must retain uncertainty rather than inherit Windows acceptance.

## Headless pilot composition

Positive source findings: synthetic records through one domain store, duplicate-request replay equality, close/reopen equality, unique local fixture session identity, existing TerminalModel ingress, bounded fixed-marker journey and detached-viewer check; no UI/browser/native window creation. Runtime cleanup result fields remain separate. No real database/PTY integration pass inferred by reviewer.

REQUEST CHANGE before pilot success acceptance: DONE marker is followed immediately by close, which signals stop and may kill the fixed child before its natural exit0. Exited alone also admits forced kill1. Await natural exit under a bounded deadline, require observed exit_code=Some(0), and expose that exit code in evidence; still perform owned cleanup on failures. A successful native output marker is not successful process completion.

Compile concern: inspected A5 TerminalError derives Debug/Clone/PartialEq/Eq without Error/Display, whereas app lib directly uses ? into anyhow::Result. Use explicit error conversion/context or approved Error implementation. No build by A6; compiler result must come from A1.

The main binary has windows_subsystem and explicit pilot/scene command routing; default features do not invoke a scene. WindowsScene source is still a future gate, not approved GUI code. Test deletes only its uniquely created synthetic SQLite files and directory, nonrecursive; on failed journey preserve evidence sufficient to diagnose output/cleanup. Data ownership review remains A1 authority. Host save/SSH/native platform parity outside this composition.

## Communication

Guard repair approval, verifier parse default, pilot natural-exit requirement and error conversion concern sent to A1 before guarded launch. Current review remains source-only. Retain A6 worktree and source history; no external writes.
