# A6 followup — acceptance repairs, isolated scene and parser allocation

Source-only review. No tests/builds/native calls/GUI/profile writes by A6. Current A1 guarded verifier is sole runner; reviewer never polls or controls its process. Source hashes below identify mutable A1 artifacts.

## Confirmed acceptance repairs

APPROVE actual repaired verifier: ownedStepExited becomes false before receipt read/JSON parsing and is promoted only for exact fresh run ID, completed=true, nonempty records and every cleanup having actual bool true. Read/parse exceptions retain the lease. This resolves the prior remaining verifier blocker. Revised owned Windows guard execution mechanism remains approved for guarded synthetic tests, not GUI acceptance.

APPROVE actual headless pilot fixes: explicit terminal error mapping into anyhow; bounded one-second natural-exit observation before close; natural_exit=true plus report.exit_code=Some(0) required for success; exit code included in evidence. Forced close can no longer count as successful natural completion. The fixture's native close uncertainty remains under outer guard and separate receipt fields. This resolves the earlier headless success/compile concerns at source level; no live test result is inferred.

Coordinator reports an actual guarded run stopped on a zero-test binary exiting0 with transient private-job active count1; cleanup confirmed true, runtime fixtures not reached. No source attribution to console host is established. APPROVE bounded one-second passive active-count polling after owned leader wait observes exit and before forced termination. Do not start next executable until actual job zero. Survivors after grace must still trigger private-job force, FAIL and separate cleanup confirmation; record initial count, grace duration and post-grace count. This is a timing grace, not weakened cleanup acceptance. Actual revised grace source/rerun still needs coordinator evidence.

## Isolated Windows scene source

APPROVE compile-only diagnostic composition boundary: explicit argument shape, dual isolation flag/environment admission, background-launch refusal before event-loop/window creation, explicit inspected marked dev profile, one main-thread event loop/window, hidden creation, existing A6 synthetic-only browser child, native child controls in disjoint A4 regions, per-DPI layout updates and explicit child disposal before parent drop. Browser profile data is preserved. Source has no untrusted website RPC/CDP or cookie migration. Runtime separately requires owned-runner attestation before fixture spawn.

No desktop launch authorized by this review. External VM/isolated display provenance is an operational requirement; environment/flag attestation alone does not prove it. No screenshot/focus/IME/accessibility/resize/clipping/cleanup success measured. Linux/Wayland and macOS unsupported/not run.

Terminal display is a native read-only EDIT fed TerminalModel.contents(), so colors/cursor/wide-cell geometry, selection semantics and emulator grid drawing are not implemented. PTY/model dimensions stay24x80 when visual pane resizes. It is a text diagnostic surface and cannot pass full native terminal rendering or native IDE acceptance. Default Windows controls/static labels also do not establish final Wilde theme/toolkit fit. UI toolkit gate stays open; source layout alone is not two-candidate feasibility evidence.

Native controls are privately owned within the function and dropped before window; no ambient HWND access. Actual main-thread parent lifetime and OS teardown remain isolated GUI gates. Native window visibility is explicit only after launch admission. Browser initial about:blank/page load remains a real engine gate, not a headless policy proof.

## Cached terminal parser allocation

Inspected exact vt1000.16.2 and vte0.15.0 from local Cargo source after fetch. vt100 Cell has fixed22-byte inline contents and compile assertion size32; combining append stops near that capacity, so it is bounded but truncates long combining sequences. Screen-cell cap can bound this retained payload class, with overhead separate.

vte0.15.0 default feature std selects osc_raw:Vec<u8>. action_osc_put pushes without a maximum; osc_end clears but retains capacity. vt100's vte dependency uses defaults. An unterminated OSC sequence spanning many individually valid bounded frames can therefore grow without bound despite terminal dimensions/cell cap. Earlier parser-allocation uncertainty is now a confirmed source finding.

REQUEST CHANGE before arbitrary output/hard terminal budget acceptance. Current allowlisted tiny fixture stays a narrower source trust scope, not general safety. A1 authorized A5's simplest pilot repair: cumulative admitted-byte cap checked before cursor/parser mutation, explicit resource fault on exceed and meaningful split unterminated OSC test; no silent dropping of escape fragments. Production needs deliberate bounded parser/session policy. Setting a direct vte default-features=false alone cannot disable vt100's transitive std feature by unification. A5 repair source/tests not yet reviewed in this cycle.

## Handoff

A1 acceptance fixes approved independently; scene compile source scope and terminal limitations sent to coordinator. One-second job grace concept approved without attributing the transient child. A5 parser budget work explicitly authorized and owner informed. Reviewer retains clean worktree and source evidence. No external writes or old-app changes.
Reviewed verifier SHA-256: A2AAEFCAF50D75497FBD40368F87C814663CC94FD91DB53CA5E8EDAB9457492B
Reviewed pilot lib SHA-256: 664A6CF275D560DFAC27A62AA66B8F72F37E48B913BE17B797DF35EE6E2EF7CC
Reviewed scene SHA-256: E9DE6580A31E1CCE0D47466DFCC6AC7BD22C960715ED4273E45D09056B1476B7
