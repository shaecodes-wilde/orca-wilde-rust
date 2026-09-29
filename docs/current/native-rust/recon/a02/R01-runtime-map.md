# A02 R01 runtime source map

Pinned legacy reference: 1db4ef38577ec932f97067b7391625b7be599884. Dedicated worktree only; no reads or edits of the old checkout.

| Concern | Inspected reference / symbol | Pilot invariant |
| --- | --- | --- |
| Session creation / host authority | `source-reference/src/shared/runtime-terminal-contracts.ts`: `RuntimeTerminalCreate`, `RuntimeTerminalSummary` | Execution host owns process identity; observer attachment is separate. |
| Attach, detach, parked terminals | `source-reference/src/shared/runtime-session-contracts.ts`: `RuntimeSyncedLeaf.parked`, `RuntimeTerminalDriverState` | Hiding/detaching a consumer leaves child running. |
| Terminal survival and remote disconnect | `source-reference/docs/reference/ssh-execution-boundary.md`: Survival, Distinguishing unverifiable from exited | No remote adapter yet; never use local fallback to infer remote exit. |
| Positive stop verification | `source-reference/src/main/runtime/unstopped-pty-verification.ts`: `verifyUnstoppedPtys`, `resolveUnstoppedPtyVerdict` | Kill acknowledgement is insufficient; owned child try_wait proves exit. |
| Status authority | `source-reference/docs/reference/agent-status-store.md`: one execution-host store | No second agent-status store in this crate. |
| Socket ownership | `source-reference/src/main/daemon/AGENTS.md`: endpoint publication / private .p names | Pilot has no endpoint, endpoint sweeper or durable daemon. |
| Stream framing | `source-reference/src/shared/terminal-stream-protocol.ts`: `TerminalStreamFrame`, `decodeTerminalStreamJson` | Raw native bytes remain ordered; bounded queue overflow is fatal, not silent loss. |
| Wire skew | `source-reference/docs/reference/remote-wire-compatibility.md` | Consume existing Rust identity/frame/verdict; add no wire opcodes. |
| MSYS process trees | `source-reference/docs/reference/windows-msys-job-breakaway.md` | Fixed PowerShell fixture only; no MSYS or job-object parity claim. |
| Shell launch / EDR | `source-reference/docs/reference/windows-edr-posture.md` | Fixed argv, no user interpolation, profiles, encoded commands, policy bypass or ambient process scan. |
| New contracts | `crates/wilde-contracts/src/lib.rs`: `SessionIdentity`, `TerminalFrame`, `StreamCursor`, `ProcessVerdict` | generation > 0; local fixture host; bounded frames; Live/Unverifiable/Exited. |

The selected snapshot omits actual daemon-server/provider/relay implementations and their test bodies. Their behavior is documented above, not newly measured. References identify the strongest legacy regression candidates: `windows-msys-job.win32.test.ts`, `windows-pty-job.win32.test.ts`, cross-version RPC harness, agent-status-store multi-surface tests. These tests are not present or executed here.

## Failure acceptance matrix

| Case | Required observable result | Evidence prepared |
| --- | --- | --- |
| Real fixed fixture | READY, input acknowledgement, native escape bytes, successful owned exit | PTY integration test, NOT RUN |
| Detach and reattach | Same PID/identity/generation; Live while detached; ordered pending frames on attach | PTY integration test, NOT RUN |
| Resize | Native get_size agrees with requested bounded geometry | PTY integration test, NOT RUN |
| Explicit kill | Owned child observed Exited; reader/watchdog joined; own cwd removed | PTY integration test, NOT RUN |
| Drop | Same bounded cleanup recorded by a receipt; no global process kill | PTY integration test, NOT RUN |
| Slow consumer overflow | Fixed bounded queue; explicit stream fault; owned child killed; no reconstructed terminal state | Deterministic queue test, NOT RUN |
| Missing shell / spawn failure | Error; no surviving child; own temporary directory removed | Setup ordering inspection; NOT RUN |
| Invalid identity / generation / geometry | Refused before cwd or process creation | Pure admission tests, NOT RUN |
| Reader timeout / cleanup uncertainty | Deadline returns; cleanup receipt preserves Unverifiable or pending worker; never fabricate exit | Lifecycle implementation / integration tests, NOT RUN |
| Restart / cursor gap | Consumer rejects stale generation and missing sequence through shared StreamCursor | Existing contracts tests; NOT RUN by A02 |
| SSH/WSL/MSYS/POSIX descendants | Unsupported; no endpoint, remote execution, group/job ownership or parity claim | Explicit boundary only |

Native ConPTY window visibility, EOF after owned kill, reader completion and cwd removal require A1's Windows run. macOS/Linux real fixtures remain NOT RUN. No performance, durable survival, terminal-emulation or production claim.
