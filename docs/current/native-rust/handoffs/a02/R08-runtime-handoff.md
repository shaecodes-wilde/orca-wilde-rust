# Task handoff

Task / role: A02 runtime/process owner; R01 reconnaissance and R08 fixture lifecycle.
Actual model / runner / session: GPT-6.1-Sol High, native collaboration worker /root/a02_runtime. User-requested model overrides kickoff Opus label.
Worktree / branch / exact HEAD: F:\wr\a02 / rust-v2/a02-runtime. Base ce470be8c01a54f68125e4de111cce312cf6ffd6; exact implementation commit supplied in completion message.
Base contract version: Rust protocol major 1; legacy selected snapshot 1db4ef38577ec932f97067b7391625b7be599884.
Status: IMPLEMENTED FIXTURE PILOT; production R08 bounded cleanup NOT ACCEPTED. Superseded acceptance notes: R09-review-response.md. Source formatted; BUILD AND REAL FIXTURE TESTS NOT RUN by A02. Not integrated.

## Result in plain English

A small library owns a fixed synthetic shell and its PTY independently of whether a viewer is attached. Hiding the terminal therefore cannot stop the process. Explicit close or the eight-second fixture lifetime stops only that owned child; process exit, reader completion and temporary-directory removal are separate receipt fields.

## Implementation

Owned crate: crates/wilde-runtime/Cargo.toml; src/lib.rs; src/fixture-command.rs; src/fixture-output.rs; src/fixture-lifecycle.rs; tests/fixture-lifecycle.rs. Recon and acceptance matrix: docs/current/native-rust/recon/a02/R01-runtime-map.md.

A1 acknowledged portable-pty =0.9.0, anyhow =1.0.102 and wilde-contracts path dependency plus identity/frame/verdict API. A1 alone registers workspace members and controls root Cargo/lock. Current fields consumed unchanged; A1's pending contract repair adds validation methods without changing these fields. Local admission requires local-fixture execution host, nonzero generation, bounded identifiers and bounded geometry. No dependency on an unacknowledged terminal renderer. A5 stream-consumer review remains pending. A6 independent platform review requested through coordinator.

API: OwnedFixtureSession::spawn(identity, size), identity/process_id, attach/detach, resize/size, write_input(FixtureInput::Ping/Finish), read_output(budget), verdict, stream_fault, close(budget), cleanup_receipt. OutputEvent returns TerminalFrame, TimedOut, Detached or End; End is stream EOF and never itself proves process exit. Consumers use the shared StreamCursor for sequence/generation admission. There is no emulator/snapshot authority in this crate.

Each native read allocates at most 4096 bytes and queue capacity is 16 chunks (65536 bytes), plus one producer and one consumer chunk. Full queue is a fatal stream fault, followed by owned-child termination, never silent frame loss or a fabricated snapshot. No replay or recovery after stream fault.

Windows fixed argv uses installed SystemRoot/System32/WindowsPowerShell/v1.0/powershell.exe with -NoLogo -NoProfile -NonInteractive -Command and one static harmless fixture script. Unix uses /bin/sh -c and one static harmless script. Only Ping/Finish enum input is exposed. Environment is cleared and synthetic HOME/USERPROFILE/TEMP/TMP point to an exclusively created temporary cwd. WILDE_RUNTIME_TEST_ROOT selects an existing absolute canonical scratch root when supplied; standalone runs otherwise use the OS temporary root. No copied credentials, user commands, profiles, network or durable endpoints.

Exactly two session workers: bounded output reader and lifecycle supervisor. Supervisor polls owned child under the same mutex used for kill/reap; it has an 8s fixture lifetime plus at most 2s verification window. No clone_killer: cached dependency inspection found Unix cloned killers carry only numeric PIDs, while Windows kill return handling is unreliable. Only owned try_wait evidence establishes Exited. PID is informational; caller generation fences the consumer identity and is not a kernel creation timestamp.

## Evidence

PASS, exit 0: rustfmt --edition 2024 on the five owned Rust files. PASS, exit 0: rustfmt --edition 2024 --check and git diff --cached --check. No Cargo command, dependency install, fixture process or GUI launch was run by A02.

A1 guarded-runner command only, after establishing owned subprocess/job watchdog; WILDE_RUNTIME_FIXTURE_GUARD=owned-runner-watchdog-v1 is mandatory attestation, not proof. Standalone fixture execution is quarantined. Previous candidate command after registering crate: ORCA_BACKGROUND_LAUNCH=1, WILDE_RUNTIME_TEST_ROOT=F:\wr-build\runtime-tests (existing directory), CARGO_BUILD_JOBS=2, CARGO_TARGET_DIR=F:\wr-build\<coordinator-selected-target>; cargo test -p wilde-runtime -- --test-threads=1. This starts only installed fixed fixture shells, with synthetic temporary cwd and no GUI. A1 chooses the external build target and executes all builds sequentially.

Prepared real integration tests (NOT RUN): native output/escape bytes, write/read acknowledgement, viewer detach/reattach with stable PID and identity, native resize, successful natural exit, explicit kill and repeatable close, Drop receipt, fixed lifetime/automatic stop. Prepared pure tests (NOT RUN): fatal bounded queue overflow preserves ordering of admitted chunks; disconnected consumer refusal; invalid host/generation/geometry/identifier admission.

Actual platform runner: Windows desktop; ConPTY process tests NOT RUN. Linux and macOS PTY tests NOT RUN. Legacy regression suites NOT RUN; their bodies are absent from selected snapshots. No speedup, parity, physical device, SSH or production claim.

## Review and integration

A6 source review requested; not yet completed. A5 consumer review pending. No PR, external publication or root/shared file change.

Native portable-pty Read has no deadline/cancel API. Reader joins are attempted only after is_finished; close never waits on a blocking join. Fixed fixtures create no descendant processes, and child termination should cause native EOF; that must be measured. If native EOF fails, the receipt reports reader_joined=false. Such a pending reader is a real unresolved resource, not verified cleanup or a guarantee of bounded OS-reader lifetime. The ConPTY master destructor is also native synchronous work and needs the real Windows timing test. Production adoption requires a cancellable native read strategy if these checks fail.

Other limitations: no process trees, Windows job object or POSIX-group ownership, MSYS shells, WSL, SSH, host-key checks, endpoint publication, daemon durability, terminal emulation, screen snapshots, arbitrary programs or observer fanout. Generation/PID are session metadata; OS creation-time identity is not implemented. Unexpected cwd contents are retained; removal is nonrecursive.

## Runtime and cleanup

A02 launched no task-owned fixture, GUI, endpoint or test worker. No task process IDs exist to report. No credential or production-profile reads. No dependencies installed or build outputs created. Worktree remains for review; cleanup is not authorized here.

At runtime CleanupReceipt remains available after Drop and records PID/generation, cwd, observed verdict/exit code, worker joins and cwd removal. Unconfirmed cleanup retains the synthetic cwd. Synthetic temporary files are not created by the fixtures.

## Next

A1: register acknowledged dependencies/member, integrate own contract repair, build and run actual Windows fixture tests, preserve measured output and cleanup receipts, then request A6/A5 review fixes as needed. Linux/macOS fixture runs follow on isolated test hosts. Next runtime task is native read cancellation and process-tree containment design before widening beyond fixtures. No Shae decision required for this scoped local pilot.
