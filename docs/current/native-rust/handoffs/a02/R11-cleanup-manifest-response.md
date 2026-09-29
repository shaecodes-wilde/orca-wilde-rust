# A02 cleanup receipt and exact profile-directory manifest

A1 reported the cursor-flag candidate now emits READY/PONG/DONE, naturally exits with code 0, and joins reader/lifecycle workers. Actual fixture/pilot checks still FAILED because cwd_removed=false. Logs referenced by A1: evidence/guarded-tests/12-fixture_lifecycle-9c5806f7e2573ed9.log. A02 did not rerun or read cross-owner file contents; these are coordinator-reported outcomes and remain recorded as failures.

## Confirmed metadata and authorization

A1 authorized names/type/size only for exactly five retained synthetic roots: F:\wr-build\runtime-tests\wilde-pty-fixture-22164-1 and wilde-pty-fixture-25912-{1..4}. Each root contains ONLY AppData -> empty Roaming directories. No files, links or unexpected entries were found. Scan limit was 32 entries/root, depth 8, no following links; each scan finished at two entries/depth two with incomplete=false. Exact bounded map: R11-retained-fixture-metadata.json. Existing historical directories remain untouched and retained; their removal is not authorized.

This disproves the initial empty-directory/async-lock-only hypothesis for the observed roots. It confirms nonempty profile directory structure; the subsystem that creates it was not instrumented here. USERPROFILE points to the isolated fixture cwd, so Windows startup directory initialization is plausible, without a claim of a measured creator.

A1 approved the future-owned-fixture manifest ONLY: empty regular AppData/Roaming, AppData containing only Roaming, and cwd containing only AppData; an already empty cwd is also permitted. Full manifest validation precedes every mutation. Links, any Windows reparse-point attribute, files, unexpected names and unreadable metadata refuse cleanup. All candidate paths are built under the exclusive owned root and checked to remain there. The routine has no recursive delete, general directory walk or sweeper. It removes only these exact paths bottom-up via empty-only remove_dir. Partial prior removal can resume with empty AppData or cwd. Native filesystem operations are not a new hard deadline guarantee.

## Changes

CleanupReport now includes cwd_remove_error: Option<CwdRemovalError>, with precise std::io::ErrorKind and raw OS code. Successful removal or direct NotFound observation clears the error; metadata/permission failures never masquerade as absence. Only after owned child Exited and both receipt worker-join fields are true does close validate/remove the approved manifest. The existing close deadline is passed through and checked before each path mutation. Only Interrupted/WouldBlock or Windows sharing/lock errors 32/33 retry inside that deadline; nonempty, access denial, link or unknown failures preserve the error and material.

Explicit-close fixture test no longer assumes the queue is immediately empty after READY. It accepts and validates up to 32 contiguous native trailing frames in at most 100ms before expecting a quiet timeout. This accommodates actual ConPTY ESC[m output without filtering native bytes or changing the protocol.

Drop's nested condition is now a Rust 1.94 if-let chain with identical owned kill behavior; this addresses A1's reported clippy collapsible_if failure without an allow or semantics change.

## Evidence and limits

Prepared pure filesystem tests (NOT RUN by A02): exact empty startup tree removal; an unexpected root file preserves the entire tree before mutation; expired budget preserves the tree; a linked AppData entry is refused without following its independently owned target; ordinary nonempty cwd preservation. The Windows link test uses the std directory symlink API and records NOT RUN if error 1314 denies creation, with no privilege request, shell helper, junction generator or stub. Such a skip cannot establish native reparse rejection acceptance.

Owned rustfmt check and staged whitespace check are recorded at completion. No Cargo/build, dependency install, new fixture process, GUI, global config or old-app change by A02. Vendor source is unchanged from bcf6142ee1c881c656d638655dab59a3291169ab. Historical five roots are read-only evidence, with no cleanup. A6 independent source review must precede A1's next guarded actual test.

Production R08 hard native cleanup remains NOT ACCEPTED: native read cancellation and synchronous ClosePseudoConsole still require the independently owned outer-runner watchdog. No actual passing fixture claim is made by this followup. Receipt and directory preservation failures remain visible until A1's real tests prove otherwise.
