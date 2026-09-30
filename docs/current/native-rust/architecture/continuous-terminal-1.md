# Continuous terminal increment 1

This is a local fixture contract extension for NWA-04, mapped to R06/R08/R11. Protocol major 1 and the 64 KiB decoded-frame bound are unchanged. It does not implement authenticated IPC, an independent session-host process, native terminal rendering or parser restoration.

A1 acknowledged the contracts below. A2 and A5 acknowledged each other's producer/consumer boundary before implementation. A6 independently reviews implementation and evidence; author acknowledgements are not acceptance.

## Parser admission

A5 retains `accept_frame(&TerminalFrame)`, identity/generation/sequence validation, gap recovery latching, and bounded screen/history geometry. Accepted lifetime bytes become saturating telemetry, rather than a reason to permanently reject normal output. A constant-space preflight guard limits a single control sequence to 16 KiB across frames before calling the pinned mature parser. Oversized controls latch an explicit fault without accepting the rejected frame, changing its cursor or partially updating its screen. No escape bytes are silently dropped. The parser remains authoritative for Unicode, cells and terminal modes.

The guard must account for pinned vte control-state transitions, including UTF-8 and control cancellation. Its purpose is to bound the cached std parser's otherwise unbounded OSC storage. This is a compatibility limit for exceptionally long control sequences, not a full terminal-emulation parity claim.

## Reader and observer ownership

A2 adds a fixture-only synchronous output sink. The native reader assigns ordered frames of at most 4096 bytes and invokes the sink independently of viewer attachment. A sink must bound its work and must not enqueue an unbounded transcript. Callback error/panic and sequence exhaustion latch explicit faults and retain existing owned-process supervision. Canonical mode refuses the raw queue read API; existing raw-queue behavior remains available for the finite original fixture.

A1 composes the reader sink with an in-process, host-owned `TerminalModel`. A detached observer reads the current model only after sustained output completes; a raw byte tail is never treated as a parser snapshot. This proves neither UI-process crash recovery nor survival of a separate host process. Authenticated daemon lifetime, snapshot/replay transport, stdin/resize ordering and slow-viewer resynchronization remain separate work.

## Fixed workload and limits

The allowlisted `SustainedOutput` fixture command emits 4608 fixed 4096-byte blocks followed by `WILDE_FIXTURE_STREAM_DONE`. Line feeds force observable output through ConPTY; authored bytes are not the acceptance measurement. The actual native reader and parser must each receive more than 16 MiB and retain the final sentinel. The existing eight-second fixture lease, natural-zero-exit assertions, cleanup receipts and outer owned-job watchdog remain required. An exceeded deadline is a failed run, not permission to enlarge the lease after scoring.

A5 additionally requests deterministic generated parser cases for sustained output, byte-split Unicode/control sequences, cancellation/termination, overlong sequences, frame atomicity and bounded history. A1 runs the real PTY integration inside the existing guarded verifier. No worker runs Cargo or launches the fixture.

## Browser pressure boundary

A6's additive retention fields report protected resident count and residents remaining above the warm allowance after planned eviction. Eligible warm eviction, cooldown and all page protections retain their existing behavior. These counts expose resource pressure; they neither establish a hard total cap nor authorize eviction of protected pages. Actual browser disposal remains a native-runner gate.
