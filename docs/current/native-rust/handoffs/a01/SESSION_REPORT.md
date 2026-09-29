# Wilde Native Rust — first implementation cycle

Date: 29 September 2026. Tested source commit: **52e478ce83db480df779cc0255cf898495844452**, branch **rust-v2/integration**, independent project **F:\Orca Wilde Rust**. Later review/register/evidence commits contain documentation and evidence-packaging rules only; their source delta against this tested commit must remain empty. G0 inventory is recorded; **G1 toolkit/native scene gate and later parity/cutover gates remain open**.

The existing app, production profiles, credentials, installation and running sessions were not modified or launched by this work. The source reference is shaecodes-wilde/orca-wilde at wilde/theme, 1db4ef38577ec932f97067b7391625b7be599884. Its 47 dirty/untracked entries were preserved. Repeated HEAD/status observations are not a byte-for-byte claim about concurrent user changes. Only 25 committed source references plus the source MIT license were imported; the old Git history and dirty changes were not imported.

## What works now

The native development pilot creates a synthetic client, project and assigned-folder record in a real, separately marked SQLite database; verifies durable request replay after reopen; runs a real Windows PowerShell fixture through ConPTY; feeds its ordered byte stream into the mature VT100 terminal model; detaches/reconnects the viewer; sends fixed ping/finish messages; observes a natural exit code zero; and confirms reader, supervisor and owned fixture-directory cleanup. The assignment is synthetic metadata, not actual Git/folder provisioning. SQLite reopen is not durable terminal-daemon restart/reconnect.

Seven useful workspace members implement contracts, runtime, domain/storage, UI policy, IDE models, browser policy/adapter and app composition. The editor model supports bounded UTF-8 editing, undo/redo, disk-text conflict detection and document-specific save acknowledgements. It is not a full native editor. The terminal model includes Unicode, alternate screen, bounded scrollback and a 4 MiB admitted lifetime-byte cap; long-lived production terminal limits and a full cell renderer remain open.

A diagnostic Windows native shell with native controls and a real WRY child-browser adapter compiles and passes code checks. Its first-party shell and controls are native; HTML is confined to the browser fixture. No native scene was launched. The terminal diagnostic control shows parsed text, not a complete terminal renderer. Iced and Slint were compared, but no permanent toolkit has been selected. Windows and Linux/Wayland focus, input, DPI, IME, accessibility and browser disposal must be proven on isolated displays before broad UI conversion.

## Tests and measurements

Windows host; Rust/cargo stable 1.94.0; rustfmt/clippy available; dependency versions pinned and Cargo.lock checked in. Local checks select +stable; the manual CI source selects +1.94.0. Run from this project's primary checkout: `./tools/verify.ps1 -WindowsChild -WindowsScene`. It compiles the scene without running a GUI, uses ORCA_BACKGROUND_LAUNCH=1 and private Windows jobs, and writes repeatable evidence.

All eight final verification steps passed: format, workspace check, test executable build, guarded tests, Windows child-browser check, Windows scene check, Windows scene code checks and workspace code checks. Evidence: `evidence/verification-steps.json`, `evidence/integrated-*.log`, `evidence/cargo-test-artifacts.jsonl`. Final guarded run ID: **aa71cc5015f74927895e03d3b4e6469f**. Only the thirteen log paths selected by that receipt are current-run evidence; older logs are retained separately and must not be pooled into current results.

| Area | Harness successes | Practical qualification |
| --- | ---: | --- |
| Browser policy | 7 | Windows policies; actual webview behavior not run |
| Native UI policy | 9 | Bounds/focus/launch policy; rendered tests not run |
| Terminal/editor models | 20 | Mature parser and bounded deterministic models |
| SQLite | 12 | Real isolated native databases; no production records |
| Runtime unit checks | 13 | One function explicitly NOT RUN because link creation failed |
| Contracts | 7 | Local version 1 stream/identity/recovery rules |
| Native PTY lifecycle | 4 | Real owned Windows fixture children |
| Composed pilot | 1 | Real SQLite and real ConPTY boundary |

Total: **73 successful harness functions, 72 exercised checks and one unverified symbolic-link case**. F: is exFAT; real symlink creation failed with Windows error 1, “Incorrect function.” Its full error is visible under --show-output. This is not native link/reparse rejection proof. The successful runtime failure-injection check intentionally poisons a mutex and catches that panic; its captured panic is not a failed test. No doctests, GUI, macOS, Linux/Wayland, production integration, old-path-disabled parity, installer or comparable performance benchmark was run. The supplied kickoff helper suite separately passed 29 Python tests.

All thirteen final private-job receipts show exit zero, no timeout, no forced surviving child, and a confirmed empty owned job after a short bounded observation grace. Tests run serially. The private job closes only our task descendants and never matches ambient process names or kills by stale PID.

Prior failures remain under `evidence/attempts/`: the original ConPTY cursor request stalled the fixture; a reviewed, narrow portable-pty 0.9.0 patch removes only cursor inheritance, retaining provenance/license/file hashes. Empty AppData/Roaming fixture directories required an exact, reviewed empty-directory cleanup manifest. A later terminal lifecycle run reported 0xc000013a after DONE despite an existing natural-exit wait. The strengthened current test checks zero **before and after** cleanup and passes, but the earlier abnormal exit's cause and repeatability remain unknown. It is not claimed fixed or production-stable. A dependency unused-constant warning from the narrow patch remains visible; no lint suppression was added.

## What changed and why

The new repository and development identity keep migration work separate from production. Shared contracts are versioned; a disconnected execution host remains `unverifiable`, never `exited`. For example, losing contact with an SSH host cannot justify cleaning up its process or starting a replacement locally. Actual authenticated transport, mixed-version remote negotiation and SSH/WSL execution remain future work.

The runtime permits only a fixed synthetic fixture, with no arbitrary commands or inherited credentials, a finite lifetime and bounded output. Cleanup is tied to the owned child handle and task job. Its fixture directory is removed only after process exit and both workers finish; every allowed path must be an empty, ordinary directory. Unexpected files or links preserve the entire fixture tree. Native blocking ConPTY close/read cancellation still has no proven hard deadline, so the outer private watchdog is required; general production lifecycle acceptance is open.

Browser admission requires an explicit native development profile marker and rejects link/reparse ancestors. The fixture page has no external navigation, scripts, native RPC, popups or downloads. This does not prove OAuth, passkeys, media permissions, protected content or CDP/Design Mode parity.

## Team, task and contract state

All six roles used **gpt-6.1-sol, High**, as Shae requested. A1's own session metadata and explicit native worker model/effort launches were verified. An earlier tool-free Claude model probe and two file-only starts were stopped after Shae's correction; they produced no implementation files. Raw session receipts remain ignored and unpublished. No billing conclusion is made.

A1 owns contracts, root build/lockfiles, app composition, CI and integration. A2 owns runtime; A3 domain/storage; A4 native UI; A5 IDE; A6 browser/platform and independent acceptance review. At most three workers ran beside A1; the other roles were queued. Only A1 ran builds, two Cargo jobs at a time. Native collaboration task handles and worktrees are recorded in `coordination/roles.json`; unavailable global worker UUIDs are explicitly null, not fabricated.

Working registers are `coordination/backlog.json` (26 tasks) and `feature-parity.csv` (all 40 supplied capabilities retained). The supplied kickoff is preserved unchanged under kickoff/. R00 is verified; R01/R02/R04/R06/R07/R08/R09/R11/R12 contain reviewed or tested subsets; R03/R05/R10 are blocked by the isolated native scene/toolkit gate; R15 is partial/in-progress. No capability is retired, no subsystem is declared fully migrated, and no cutover is authorized. Minimal native contract version is 1. Shared root configuration and source ownership acknowledgements remain required.

A6 independently reviewed immutable code changes and the current receipt/log hashes in `docs/current/native-rust/recon/a06/final-headless-evidence-review.md`. Source-only CI review is recorded separately. The manual Windows GitHub workflow has read-only repository permission, verifies the exact fetched commit, uses pinned official artifact upload and does not run on push. No hosted CI run has happened.

## Resource and cleanup state

A1 build outputs are shared under F:\wr-build\a01, outside all worktrees. Incremental compilation is disabled for exFAT compatibility. The shared Git-common-directory lease prevents overlapping builds; uncertain cancellation retains it and requires exact owner/child recovery, never age-based deletion. No global toolchain or Git configuration was changed. Windows tool invocations use scoped repository trust only.

The five linked worktrees F:\wr\a02 through a06 are retained deliberately for the next gates; all delivered branches are integrated by ancestry. No original worktree was pruned. Historical failed-fixture directories are retained; five inspected roots had only empty AppData/Roaming descendants, but their creator was not instrumented. Future successful fixtures are cleaned by the exact reviewed manifest. Raw logs, credentials, development profiles, generated caches and compiled outputs are excluded from publication. Final resource observations and worktree receipts are in `evidence/final-state.json`; memory and disk observations are snapshots, not comparative performance benchmarks.

## Needs Shae

The settled destination is **shaecodes-wilde/orca-wilde-rust, public**. Local work is complete for this cycle. Final exact repository creation/publication approval is required by the supplied workspace approval rule before any external write. The publication manifest identifies the reviewed commit contents; it includes native source, synthetic tests, selected committed reference source with licenses, the kickoff and sanitized evidence. It excludes raw agent sessions, profiles, credentials and compiled caches. A public Git history can be copied by others; deleting the repository cannot guarantee removal of those copies.

Native display validation needs an isolated Windows VM/display and a Linux/Wayland runner. This desktop is not such a runner. No cloud runner, paid service or signing identity was purchased or assumed.

## Resume instruction

Read this report, root AGENTS.md, the working registers, contract-1.md and relevant role handoffs. First investigate the recorded intermittent terminal exit with bounded native error/elapsed diagnostics if it recurs; never weaken natural-exit checks. Complete the isolated Windows and Linux/Wayland R03/R04/R05 joint scene experiment before selecting a toolkit or implementing R10. Independently complete versioned preview/commit and legacy differential fixtures for R09/R14; design durable daemon restart/reconnect and actual workspace provisioning for R08/R13/R15. Full native Rust remains the destination. No unattended continuation or production deployment is promised.

Publication packaging preserves evidence bytes without Git line-ending conversion so the independently recorded receipt/log hashes remain reproducible. The publication manifest hashes staged Git blob contents, with its own bytes excluded to avoid circular hashing.
