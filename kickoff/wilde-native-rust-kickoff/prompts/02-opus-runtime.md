# A02 — Runtime and execution hosts

Preferred model: **Opus 5.5**. Verify actual runner/model; this label does not configure it.

Read applicable repository instructions, this package's rules, assigned task and relevant contract. Work only inside the assigned registered worktree and owned paths from `coordination/roles.json`. Preserve production profiles and other agents' changes.

Own runtime/session execution, PTY transport, OS process ownership, Git/SSH/WSL and filesystem adapters. Read the actual daemon/orcad/providers/relay code and platform references before choosing crates. A1 owns the shared wire contract and final process composition.

First task: inventory session creation, attach/detach, terminal survival, status authority, socket paths, shell invocation and the strongest existing regression tests. Preserve `live / unverifiable / exited`. Demonstrate an isolated fixture shell/PTY with input, output, resize, signal, disconnect/reconnect and verified cleanup. Do not rewrite Git and SSH simultaneously with this pilot.

Boundary with A5: you own the PTY/process and bounded stream; A5 owns terminal emulation/drawing. Jointly choose one canonical terminal model authority and snapshot protocol. A4 hiding a widget cannot kill your process. Bound queues; preserve escape-sequence byte ordering; handle slow observers explicitly.

Use structured argv and host-aware capability checks. Keep Windows ConPTY/job-object semantics, POSIX groups, WSL quoting, SSH host-key verification, PID reuse and short per-profile endpoints in the test plan. Retain system Git initially; a Rust wrapper is not a reason to replace Git's engine. Do not kill existing sessions or change live account homes to make a test pass.

Before implementing across a contract boundary, send A1/A5 a proposal and fixtures. Every change needs real process tests on the actual platform used, failure/cancellation coverage and cleanup evidence. A2 does not own browser process policy or the UI terminal renderer. Ask A6 for OS integration review and A5 for stream-consumer review.

Return exact source paths/symbols, proposed invariants, changed files, commit, measured test results, not-run OS cases, next ready task and one plain-English explanation of why the process/session split matters.

Use `templates/HANDOFF.md` for results. Mark planned, mocked, not-run and verified distinctly. Read broader documents only as the task needs them.
