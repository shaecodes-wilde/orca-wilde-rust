# Portable PTY pilot patch provenance

Source: exact cached registry package portable-pty 0.9.0, copied locally without modifying the global Cargo cache.
Upstream repository: https://github.com/wezterm/wezterm
Cached .cargo_vcs_info.json: commit f8921727a11b9f8b073e8c24821d72fd41283500, path pty.
License: MIT; original LICENSE.md retained verbatim, copyright 2018 Wez Furlong.
Registry archive portable-pty-0.9.0.crate SHA-256: b4a596a2b3d2752d94f51fac2d4a96737b8705dddd311a32b9af47211f08671e.
UPSTREAM-FILE-HASHES.json records SHA-256 hashes of all 18 exact cached package files before the experiments and remains immutable. Every copied file except the two scoped source exceptions below remains byte-for-byte upstream. Added .gitattributes pins this package to LF checkout bytes so platform Git newline settings cannot silently alter the recorded copy; no global configuration is changed.

The Cargo registry-normalized Cargo.toml is retained unchanged; its source/example paths stay inside this directory. Cargo.toml.orig is upstream archival evidence and is not the active manifest. Neither manifest nor upstream Cargo.lock is edited. Root workspace patch/lock composition belongs to A1. Do not use or inherit upstream development workspace paths from Cargo.toml.orig.

The first source exception removes PSUEDOCONSOLE_INHERIT_CURSOR from PsuedoCon::new's CreatePseudoConsole flags. PSEUDOCONSOLE_RESIZE_QUIRK and PSEUDOCONSOLE_WIN32_INPUT_MODE remain. The constant itself and native drop remain unchanged. No guessed cursor responder, escape parser or new cleanup thread.

Reason: A1's guarded Windows fixture run captured exactly four admitted native bytes, ESC [ 6 n (escaped as \x1b[6n), then timed out without WILDE_FIXTURE_READY in all four real integration tests. This proves a cursor-position query was emitted. Inherited-cursor negotiation preventing startup remains a cause hypothesis until this reversible experiment is independently reviewed and tested. A6 approved the narrow source experiment; A1 authorized this scoped cached copy. This file claims no test pass or root-cause confirmation.

The second exception changes only `WinChild::is_complete` and necessary imports in `src/win/mod.rs`. A zero-time `WaitForSingleObject` on the retained owned process handle must return WAIT_OBJECT_0 before querying an exit code. WAIT_TIMEOUT returns no exit observation; WAIT_FAILED, unexpected wait results and GetExitCodeProcess failure return an observation error. A signaled process's actual exit code is returned, including 259. `wait`, Future polling implementation, `do_kill`, native teardown, fixture commands/lease, cleanup deadlines and all acceptance assertions remain unchanged. No dependency or worker was added.

Reason: actual alpha-20260929-03 logs show the raw sustained native test accepted exit zero before close, then observed 0xC000013A after cleanup; app pilot and continuous paths also failed natural-zero acceptance. A premature positive exit observation followed by console close is an inference, not an established flake root cause. Checking the process handle's signaled state strengthens positive exit authority independently of that explanation. The earlier alpha-02 cleanup timeout did not reproduce and remains unexplained. A1 and A6 acknowledged this exact producer correction before coding; immutable source review and focused guarded native tests are required before acceptance. No build/fixture execution by A2.

Official references checked 2026-09-29: [WaitForSingleObject](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject), [GetExitCodeProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess), [process termination](https://learn.microsoft.com/en-us/windows/win32/procthread/terminating-a-process), and [ClosePseudoConsole](https://learn.microsoft.com/en-us/windows/console/closepseudoconsole).

## Scoped SHA-256 exceptions

| Source path | Original upstream SHA-256 | Current patched SHA-256 |
| --- | --- | --- |
| `src/win/psuedocon.rs` | `297d0622d7b0401708f1ce3b92076632dd5a0c188b4a8d09b2f4891190b0e392` | `3b8bcdd3204d94ce072cd004efc6552166fe4d3370bb2266e71e2bf08a4df664` |
| `src/win/mod.rs` | `c983e63d15800f73ac05105a69ec2b53fea482c066ea5559682bd024561f999a` | `9340a166a12d1907f84b6a69c8ffbb8efd8022544ae217884e411af12e0fe1e6` |

Immutable upstream manifest SHA-256: `1e603b3715d40ac7ef8f2233eb2bb315e220af5814e3c4ab86fdbc303e08a07d`.

A1 test request after independent source review: guarded full runtime fixture tests plus app pilot/continuous tests, capturing natural exit zero before and after close, stream fault absence, native bytes/sequences, joins/cwd removal, phase timings and positive outer-job emptiness. Keep both earlier failed runs as failures. No success claim follows from formatting, compilation or a single prior pass.

Fixture execution still requires WILDE_RUNTIME_FIXTURE_GUARD=owned-runner-watchdog-v1 and A1's independently owned outer runner. Removing the startup flag does not make synchronous ClosePseudoConsole, native read cancellation, process-tree containment or production R08 cleanup bounded. Production R08 acceptance remains NOT ACCEPTED. A1 alone builds/runs; A02 ran no Cargo build, fixture child, native UI or app.

Revert: A1 removes the workspace [patch.crates-io] entry and regenerates its own lock selection, preserving this experiment and evidence for review. No global-cache edits or old-app changes are needed.
