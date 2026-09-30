# A2 NWA-04 canonical output increment

Base: `7d9c47f8999e4ba645ec182d142e8edca81ecede`, dedicated `F:\wr\a02`. Primary alpha package adopted by A1 at `e68c9983d0849dc6798e38403567a17259d066b5`. A1/A5 acknowledged the producer/consumer contract before edits.

## Implemented

`OwnedFixtureSession::spawn_with_output_sink(identity, size, sink)` adds a single synchronous canonical consumer. The native reader assigns contiguous sequence numbers and invokes `FnMut(TerminalFrame) -> anyhow::Result<()> + Send + 'static` with at most 4096 payload bytes. Viewer detach/attach never suspends this consumer; raw `read_output` is unavailable in canonical mode. Existing raw spawn/16-chunk overflow behavior remains intact.

Sink error/panic latches `ConsumerFailed`; exhausted sequence latches `SequenceExhausted`. Existing owned-child supervision observes the fault and stops the fixture. After fault, the reader drains native bytes to EOF without publishing them. This drain is cleanup, not recovery. No display snapshot can restore parser state from an arbitrary raw tail.

The sink must bound its work and retained memory. Runtime retains one 4096-byte read buffer and one at-most-4096-byte frame at a time in canonical mode; this API cannot enforce bounds on caller-owned sink storage or synchronous execution time. A1 owns canonical `TerminalModel`/latest bounded display integration; A5 owns parser bounds. No runtime-to-IDE dependency was added.

Fixed enum `FixtureInput::SustainedOutput` emits 4608 fixed 4096-byte blocks, each 64 lines of 62 ASCII x bytes plus CRLF, then `WILDE_FIXTURE_STREAM_DONE`. Authored output is 18 MiB; acceptance requires actual native received bytes above 16 MiB. ConPTY can transform output, so authored byte count alone is not evidence. Existing guard attestation, fixed shell argv, environment clearing, isolated cwd, eight-second lease, natural-exit checks, cleanup deadlines and outer-watchdog requirement remain unchanged.

## Verification request

A2 ran source formatting and `git diff --check`; no Cargo build, fixture process, GUI or dependency install was run by A2.

A1 should run through the existing owned-runner watchdog with `CARGO_BUILD_JOBS=2` and external outputs:

- Runtime library tests: sustained synthetic `Read` through the real reader function without a viewer queue; sink error/panic draining; sequence exhaustion; existing queue and lifecycle checks.
- Guarded runtime integration tests: `real_native_canonical_sink_receives_over_16_mib_while_viewer_is_detached` and `canonical_sink_failure_stops_owned_fixture_and_preserves_explicit_fault`, plus all existing fixture tests. Integration checks actual bytes, contiguous identity/sequence, fixed resize/input, same PID/generation on reattach, natural exit zero and cleanup. First throughput run keeps the eight-second lease; failures must remain failures.
- A1 in-process app integration: mature parser receives every frame while display observation is hidden/slow/detached; final sentinel/current bounded display view; actual byte count, retained parser/history/view limits; natural zero exit/cleanup.

All new runnable tests are **NOT_RUN** by A2. Formatting is not compilation. Synthetic `Read` is not native PTY evidence. The guarded PTY fixture is not interactive arbitrary-shell/coding-agent evidence.

## Remaining boundaries

This is an in-process reader/canonical-consumer increment. It does not implement independent host process lifetime, authenticated IPC, UI-process restart survival, parser serialization/recovery, ordered parser/PTY resize transactions, real shell/general-session launch policy, SSH/WSL, process-tree parity, native display composition or performance acceptance. Full F03/R01/R02 and functional alpha remain open.

Integration requires only A2-owned runtime files and this handoff; no shared contract/root/dependency edits. Worktree cleanup remains A1-reviewed and is not authorized by this handoff.
