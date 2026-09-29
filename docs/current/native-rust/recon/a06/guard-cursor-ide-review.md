# A6 safety review — owned test guard, cursor experiment and IDE model

Source-only cycle; no process/native API calls, tests, builds, profile changes or GUI launches. Reviewed A1 tools/guarded_tests.py SHA-256 EB855D5DCE75F916B281500B6466EC9A923BE1E0D6FE1461DAF16C9D14133A11 and repaired tools/verify.ps1 SHA-256 DF53504BD807EEBD2E955EDD2FDEB254CD78B96CF1E0B5820261420E21EE2E41. Reviewed A5 current HEAD 611dd32c4424bf285c66f9d0a70a5e8c9e9bd516 in F:\wr\a05, newer than its earlier relayed commit. Exact future revisions require re-review.

## Owned Windows test guard

APPROVE source ownership sequence: unnamed job with kill-on-close and no breakaway allowance; explicit compiled executable resolved within an absolute canonical reviewed Cargo target; suspended CREATE_NO_WINDOW launch; job assignment before ResumeThread; process HANDLE wait/exit retrieval; timeout terminates only that private job; accounting query records whether all owned processes reached zero. ctypes ABI structures/signatures inspected against Win32 definitions; no runtime compilation or installation. PID is diagnostics only. Environment guard attestation is inherited before creation but test instructions run only after job assignment.

REQUEST CHANGES before execution: unassigned child finally ignores TerminateProcess BOOL and WaitForSingleObject result. If assignment fails, cleanup must check termination and observe WAIT_OBJECT_0 by the already owned process HANDLE; retain explicit incomplete-cleanup evidence on failure. Assigned exception cleanup currently closes kill-on-close job without checking/accounting completion; this is a valid owned stop request but cannot be reported as observed empty-job cleanup. Capture exception/error cleanup state and retain uncertainty. No ambient taskkill, process-name scan, or stored PID signal is acceptable.

Nominal result already fails on timeout, nonzero exit, remaining children requiring force, or owned_job_empty false. Avoid translating an exception into success. POSIX waitid WNOWAIT prevents leader/group-ID reuse before killpg; however descendants can change groups and the implementation reports owned_job_empty=None. Non-Windows NOT RUN and no all-descendant containment approval. A general OS launch test can still show a native error dialog, so the guard is process ownership protection, not GUI acceptance evidence.

[Microsoft job nesting/assignment](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-assignprocesstojobobject), [job object behavior](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects), [extended limit layout](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_extended_limit_information).

## Build verifier repair

Updated source anchors its exclusive CreateNew lease in the canonical shared Git directory, derives one external build root from the primary repository parent, rejects alternate/relative override roots, rejects a linked/reparse build root, sets two build jobs and serial tests, and compiles tests separately before guard execution. Script interruption during an owned invocation retains its lease rather than deleting by age. APPROVE these repaired source mechanisms for the current F-drive repository; cancellation/abandoned-lease recovery is NOT RUN and direct child completion alone is not a general proof of all compiler-descendant cleanup. The earlier unchecked alternate-root source is superseded. No guarded execution approval until guard failure cleanup is repaired.

## Narrow cursor-flag experiment

Coordinator reports the four actual ConPTY tests failed readiness in about 17.7 seconds and no matching fixed fixture PowerShell processes remained; reviewer did not execute or inspect those full raw logs in this cycle. Treat failed readiness as failed acceptance, never simulated pass.

APPROVE reversible hypothesis experiment removing only PSEUDOCONSOLE_INHERIT_CURSOR from exact cached portable-pty 0.9.0 Windows CreatePseudoConsole flags. This pilot has no rendered parent cursor to inherit. Preserve resize quirk and Win32-input-mode terms, upstream license/checksum/provenance and a one-line source diff. A2 owns an expressly authorized vendor path; A1 owns root Cargo patch and lock. Never modify shared cache or old app dependency. Disabling an unneeded native request is preferable to guessed terminal replies. Actual bounded raw output and repeatable READY/PONG/exit/cleanup rerun are required before assigning root cause or accepting the patch. It does not repair all native teardown uncertainty by itself.

## IDE state pilot

TerminalModel APPROVED for local pilot cursor/identity/generation admission, snapshot-required latch, split escape-byte parsing API, dimension/scrollback cell admission and high-water width guard. Native rendering, resize ordering with host PTY, recovery snapshot serialization and variable-length parser allocation remain unverified. Cached vt100 source was unavailable at inspected cache paths; do not claim a hard whole-model byte cap from cell count.

EditorDocument REQUEST CHANGES before multi-document composition: SaveCandidate contains text/expected disk/revision but no document-origin token. Two documents with identical baseline and matching revision can accept each other's candidates, causing one buffer to track another file's save. Add a private per-instance identity carried by candidate and reject cross-document acknowledgement atomically; test same-baseline documents with divergent edits. The host still must bind the candidate to document/file identity, compare baseline and write durably as one authorized operation. UTF-8 boundary/edit/history-budget logic and keeping later edits dirty after an earlier save look appropriate at source level. No disk-save success or native-editor parity claimed.

## Handoff

Guard cleanup blockers, flag experiment scope and A5 document-token request sent to A1; A5 request also sent directly to its owner. Worktree retained and clean after this docs commit. Free review slot; re-review only actual repaired artifacts. No external writes or source modifications in this cycle.

## Superseding A5 repair review

Read exact db6a880e0c18d116d80f1466fcff667def858339. APPROVE private per-instance Arc<()> token carried into SaveCandidate and Arc::ptr_eq refusal before baseline mutation. Added same-baseline/matching-revision cross-document regression preserves first document state and accepts the candidate on its original document. This resolves the multi-document acknowledgement blocker above at source level. Tests still delegated to A1; durable file identity/atomic write remain host requirements. No build or test by A6.
