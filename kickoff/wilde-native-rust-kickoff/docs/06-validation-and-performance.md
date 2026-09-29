# 06 — Evidence, not language-based promises

## Separate four questions

Does behavior match? Does the system recover safely? Is resource cost bounded? Does the same workload improve?

A compiling port answers none of those by itself. Preserve current TypeScript tests as specifications and adapters when useful; Rust unit tests do not automatically execute the old test suite. Add differential fixtures and a stable command interface that both implementations can exercise.

## Test ladder

1. Pure unit/property tests: IDs, relationships, state machines, parsers, budgets, malformed inputs.
2. Contract tests: the same synthetic request/response fixtures against old and new services; normalization rules explicit for timestamps/IDs/order.
3. Real integration tests: temp SQLite/WAL transactions, disposable shell/PTY, permissions, cancellation, stream pressure, reconnect, subprocess crashes.
4. Native UI tests: actual window/surface composition, DPI, focus, IME, text selection, shortcuts, drag/drop, keyboard navigation, screen-reader exposure. Do not use DOM selectors as proof of native UI coverage.
5. Real platform/installer tests: supported Windows, macOS and Linux (Wayland and X11 where supported), updates and rollback in isolated machines/profiles.
6. Workload benchmarks and long-run stability.

Fuzz byte-stream parsers and RPC decoders. Reject malformed/oversized frames and resource leaks. Reuse recorded terminal transcripts after scrubbing secrets. Fault injection includes daemon death, UI death, disk-full, lost SSH connection, stale PID, partial import, stuck download, browser crash and cancellation mid-operation.

## Benchmark inventory

Review these existing commands in `package.json` before execution: `bench:idle-cpu`, `bench:startup`, `bench:daemon-coldstart`, `bench:main-thread-jank`, `bench:multi-workspace-typing`, `bench:cold-park-reveal`, `bench:cold-park-resource`, `bench:session-write-hot-path`, `test:e2e:terminal-perf`, `test:e2e:terminal-perf:scale`, and `bench:compare`. They are evidence starting points, not guaranteed-safe arbitrary commands. [R4]

Initial workloads: empty shell; populated client/project list with 10,000 synthetic records; 1/4/10/20 terminals with deterministic idle and output-heavy patterns; repeated workspace switches; 1/5/10 controlled browser pages; mixed terminal + browser + CRM; suspend/resume and repeated open/close. Use local fixture websites for reproducibility and a separate realistic-site pass with network conditions documented.

Measure input-to-present latency with a native/OS-aware method, not just command completion. Measure UI shell/runtime separately from the browser and external agent processes, then the unique total process tree. RAM attribution must define the OS metric: Windows private bytes/working set are different; Linux PSS and RSS are different; macOS footprint is different. Never compare unlike metrics or add shared pages repeatedly. Track GPU usage, disk writes and build/cache disk consumption separately from process RAM.

## Reproducibility

Record exact commit, machine ID, OS/kernel, CPU/RAM/GPU/driver, toolkit/backend versions, release/debug mode, display scaling/resolution, workload fixture hash, browser version/profile, power mode, measurement method, sample count and environment fingerprint. Compare release against release, same workload and hardware. Counterbalance run order and warmups; closed DevTools and no concurrent build/benchmark. A blank native window is not comparable to a loaded Electron workspace.

Use repeated runs. Record median and tail latency; a p95 based on two runs is not meaningful. Longer soak tests may target 24 hours as a TEST DURATION, not a promise this kickoff executes them now. Report what duration actually ran. A stable bounded cache need not return exactly to process-start RSS after each close; look for retained-resource growth over repeated identical cycles.

## Gates

Before seeing candidate results, A1 and A6 set scenario-specific acceptable regression limits and target budgets based on baseline hardware and measurement noise. Record the decision. Do not insert invented memory-saving percentages. Functional regressions block integration independently of a speedup. Performance improvements must be large enough and repeatable enough to distinguish from noise.

`tools/compare_benchmarks.py` compares measured scalar sample sets only. It refuses templates, empty samples, nonfinite values, unmatched metrics and incompatible environment metadata. It does not collect measurements or make a statistical-significance claim. Its default report is informational; an explicit regression threshold enables a mechanical gate, not full acceptance.

## CI design for A1/A6

Create scoped format/check/test/clippy jobs for the new workspace without replacing existing gates. Pin toolchain/dependencies and CI action revisions after verification. Unit tests may run across an OS matrix; native visual/PTY/packaging tests need real supported hosts and permissions. Use fixture accounts and isolated data. Keep expensive benchmarks serialized and out of shared noisy runners when measurements are the acceptance basis.

Do not claim macOS/Windows coverage unless those jobs actually executed. Expected skips and missing runners remain visible. The native release candidate needs validated browser sandboxing, credentials, session ownership, profile migration, rollback and feature register sign-off, not merely green unit tests.
