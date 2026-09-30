# A6 independent evidence review: alpha-06 final diagnostic

Verdict: FAIL guarded increment reliability. The fixed-lifetime cleanup test failed its original directory-removal assertion. Both app pilots and the raw canonical native stream passed this finite run; they do not erase alpha-02/03/05 failures or establish stable acceptance. No A6 build, executable test, native GUI, dependency change or performance measurement. Author-owned browser retention remains A1's independent-review responsibility.

Frozen source 86e2b20f12193ca5b7aa2c2c97097b8052db3158 matches primary HEAD and summary. Integrated runtime/app/runner source bytes match the A6-approved A2 0bac5a5 and A1 b3ef2a0 diagnostics recorded in alpha-supervision-progress-source-review.md. Guard ea2f3d8b81a347bfaa042e59655c5a55 completed with fourteen exact receipts: thirteen exit0, one exit101, no guard watchdog timeout or forced remaining children, every owned job empty. Independently hashed all fourteen summary-selected logs and fourteen current executables:28/28 matches. All summary entries correspond to exact guard receipt paths/exit/timeout/job state. Summary states capture after guard completion before another build; current bytes were independently matched before any changed artifact was observed.

Format/check/test-build exit0; guarded-tests exit1. Later verifier feature/lint steps NOT RUN in alpha-06. Harness90 successes and one failure include one explicit native symlink NOT RUN/raw OS error1;89 exercised successes. Lifecycle harness completes five pass/one fail, unlike alpha-05's truncated harness. Expected injected consumer panic is the passing fault-path check.

## Measured failed phase

The fixed-lifetime case returned Exited/code1 with reader and lifecycle joined, cwd_removed=false, CwdRemovalError TimedOut/no raw OS error, deadline_exhausted=true. Captured per-close phases: writer16.2us, verdict17.5us, master18.8us, worker12.4us, manifest-removal4.4998156s, total-close4.5099452s. This identifies the manifest-removal scope as the slow measured phase in this call. It is not evidence that PTY destruction blocked here, nor an explanation of the earlier alpha-02 timeout.

Supervisor first_kill at8.0101044s observes stop=false and fault3 (fixed lease expired); first supervisor-positive signaled exit at8.0727168s/code1; max loop gap21.87ms. adapter_error=None means no captured returned error and cannot prove OS kill success. The owned-handle signaled status and final private job emptiness are separate positive authorities. This test expects lifetime termination; code1 is not the failing assertion here. Directory removal is.

Read-only inspection of remove_before confirms that removal_manifest performs synchronous metadata/directory enumeration before the per-path deadline check. Per-path validation and synchronous remove_dir also occur after that check. The measured scope can therefore exceed its two-second cooperative budget before a subsequent check reports TimedOut. It contains validation/enumeration/removal and possible retry work, not a timer for a single OS deletion. Which call delayed, OS/filesystem cause, external interference and hardware/resource contention remain unmeasured. No inference from the historical filesystem or one API document supplies that missing evidence. Existing total-close includes polling sleeps and unmeasured lock work; phase durations are not an exact sum partition.

Next independent step: A2 source-inspect and propose bounded operation observations within manifest validation/removal, preserving original owned-root/link refusal/retention rules and original deadline. Review that proposal before mutation or a focused guarded exercise. A deadline check around synchronous calls is not a hard cancellation guarantee; do not reset/extend the budget, remove the assertion, retry blindly or delete retained directories outside reviewed ownership.

## Passing boundaries remain finite

Continuous composed receipt:18,874,541 admitted bytes, next sequence4640, final sentinel=true, unchanged identity/PID=true, fault-free=true, exit0, both workers joined, cwd removed. Pre-close receipt establishes code0 at6.0785053s; close returns at6.1134693s with no first_kill, supervisor exit0, total-close33.5691ms, manifest23.363ms, deadline_exhausted=false. Raw canonical receipt:18,874,760 bytes, sequence74000, unchanged identity/PID, no final fault, exit0, joins/cwd=true. Existing composed pilot pre-close natural_exit=true/code0, no first_kill; total-close53.7671ms, manifest43.5116ms, deadline_exhausted=false, cleanup complete.

IDE30, SQLite12, scene policy9, browser policy9, contracts7 and runtime unit harness16 report success; the native symlink case remains explicitly unexercised. Actual browser rendering, terminal cell rendering/input, independent daemon/UI restart, isolated Windows/Wayland composition, macOS/Linux adapters, remote disconnected execution and approved real-agent loop are open. No verified isolated display, release comparison, paired baseline, resource soak or numeric savings evidence. Debug durations are diagnostic observations, not efficiency acceptance.

Supplemental alpha-07 compilation-only receipt was read after alpha-06 hash audit: format/check/windows-child/windows-scene/windows-scene-clippy/clippy all exit0. Every selected log was read and hashed; existing unused vendored cursor-constant warning remains. Coordinator states same86e2b20 source and current HEAD is unchanged. These six source-check receipts add no fixture or rendering acceptance and do not change FAIL. Run-source attribution follows the coordinator's frozen-source statement; alpha-07 has no independent runtime result.

## Evidence identity

Paths relative to F:/Orca Wilde Rust/evidence/alpha-runs/. Alpha-06 summary enumerates exact fourteen logs and executable hashes.

| Path | SHA256 |
| --- | --- |
| alpha-20260929-06/summary.json | 0DF5A4F819C743D93154EF42BCA770418B81ACF50CFAAC1A10449FDC1C44E217 |
| alpha-20260929-06/verification-steps.json | 8DBD2DAA6BFF24CB0FCC0B19503405120A42BF1CA92FFDF98B21DCE508BC61F3 |
| alpha-20260929-06/guarded-tests/results.json | 2C6AC692CEA5C24A7698D01CEEA244C71B812B86CC43001325479E49050BFA05 |
| alpha-20260929-06/cargo-test-artifacts.jsonl | 56814CD8D7219561BD29895946EE9435983FB05C14041D2E524366BDE2AC1722 |
| alpha-20260929-06/guarded-tests/05-fixture_lifecycle-9c5806f7e2573ed9.log | 61B60B3942D3F401312B5193A4C9C83994914DB00199D3C2A011501E56F3AB9D |
| alpha-20260929-06/guarded-tests/07-continuous_terminal-bbadcfcd0aaaace9.log | 755EFB945348E484167B0E50A24B26E904E508507DFC450366702C749DF30609 |
| alpha-20260929-07/verification-steps.json | 7EA6FB9B95577212C026088D5AC9C99D892004622C6EF97326E81A096E48E053 |
| alpha-20260929-07/integrated-format.log | E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855 |
| alpha-20260929-07/integrated-check.log | 13D55135783B82F57BC4EF6429F368A80297994AB26AFBD83E10C0FF35857320 |
| alpha-20260929-07/integrated-windows-child.log | 8787A6283231CDEF294B812239066A6F50F0BB504A98279A78453BB431FC3589 |
| alpha-20260929-07/integrated-windows-scene.log | 112452F91BBA5DF05D14DE3137B6720DC6E553BEC3E0CCBAD2C9D67E509FF300 |
| alpha-20260929-07/integrated-windows-scene-clippy.log | 05BA135041AA6302BEFE765606211980582407D57B963E38F3F72D1742B68537 |
| alpha-20260929-07/integrated-clippy.log | ADA539CCBBC75F8CBA45271FF1B3D6E0C11EF9F288D48DAB77DED050DDFC3D7D |
