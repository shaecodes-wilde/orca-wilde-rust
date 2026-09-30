# A6 independent source review: signaled process exit authority

APPROVED A2 immutable commit 0fcee9797b217dd17594ac73bfa208f88145d3c6 for guarded verification. Read exact two-file diff in clean F:\wr\a02. A1/A6 acknowledged the producer correction before coding. A6 ran only reads/diffs/hashes/whitespace checks and wrote this owned review; no build/test/native launch. This is an authority correction independently useful of any proposed flake explanation, not proof of root cause or repair.

## API and retained ownership

Current official [WaitForSingleObject](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject) describes a zero-millisecond check as immediate and WAIT_OBJECT_0 as signaled, WAIT_TIMEOUT as nonsignaled and WAIT_FAILED as failure requiring last error. A process handle requires SYNCHRONIZE access and must remain live throughout. The dependency obtains an OwnedHandle from CreateProcessW and keeps a local clone through the wait and exit query.

[Process termination](https://learn.microsoft.com/en-us/windows/win32/procthread/terminating-a-process) identifies the process object becoming signaled upon termination. [GetExitCodeProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess) distinguishes live status and final codes and warns that using259 as an application exit code can mislead code-only polling. Combining a signaled retained handle with the actual exit query avoids interpreting a legitimate final259 as a still-running process. These documents support the tighter condition; they do not prove the earlier apparent code change was caused by early teardown.

Exact source changes only necessary imports and WinChild::is_complete: zero-time wait on retained owned clone; timeout returns None; signaled permits GetExitCodeProcess and returns actual ExitStatus including259; WAIT_FAILED/query failure immediately captures last_os_error; unexpected wait result returns an observation error. No PID lookup, new blocking wait, permission expansion, cleanup deadline/reset, lease/fixture/assertion/worker change or do_kill correction. Runtime try_wait observation can now receive genuine API errors instead of silently treating query failure as still live.

Existing cloned-handle/poison unwraps, wait()/Future waiter behavior and inverted do_kill return/error handling remain separate source weaknesses. Future polling calls the corrected method but its unrelated implementation is unchanged. Approval is for the current reviewed try_wait-based headless runtime boundary, not unrestricted dependency correctness or production lifecycle completion.

## Verified package provenance

UPSTREAM-FILE-HASHES.json remains immutable, no diff in this commit; byte SHA256 1e603b3715d40ac7ef8f2233eb2bb315e220af5814e3c4ab86fdbc303e08a07d. A6 independently hashed all eighteen upstream manifest entries: sixteen byte-identical, exactly two declared source exceptions. Neither dependency manifests/license/version nor global Cargo cache changed.

| Path | Upstream SHA256 | Patched SHA256 |
| --- | --- | --- |
| src/win/psuedocon.rs (existing cursor exception) | 297d0622d7b0401708f1ce3b92076632dd5a0c188b4a8d09b2f4891190b0e392 | 3b8bcdd3204d94ce072cd004efc6552166fe4d3370bb2266e71e2bf08a4df664 |
| src/win/mod.rs (new exit authority) | c983e63d15800f73ac05105a69ec2b53fea482c066ea5559682bd024561f999a | 9340a166a12d1907f84b6a69c8ffbb8efd8022544ae217884e411af12e0fe1e6 |

Provenance accurately declares both exceptions and preserves upstream records. git diff --check exit0. A1 must compile/retest actual native fixture boundary against frozen integrated source, including zero natural exit before and after cleanup, bounded stream final sentinel, no fault, joined workers, safe directory cleanup and confirmed private-job emptiness. Keep alpha-02/03 failed verdicts unchanged; one future passing run cannot establish repeatability or explain the previous timeout. No GUI/SSH/production/process cancellation hard deadline or efficiency acceptance follows.
