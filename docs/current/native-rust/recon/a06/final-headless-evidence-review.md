# A6 final finite headless evidence review

Independent read-only review of immutable integration code 52e478ce83db480df779cc0255cf898495844452. Runtime/application source diff against this candidate was empty at inspection. A1 alone built and executed tests; A6 launched no builds, native fixture, GUI, benchmark or external write.

## Accepted evidence scope

Fresh guarded receipt run_id aa71cc5015f74927895e03d3b4e6469f is completed=true with exactly13 selected executables. Independently checked each actual exit_code=0, typed Boolean owned_job_empty=true, timeout=false and forced_remaining_children=false. Only receipt-selected logs were considered current; older differently numbered logs remain earlier failures.

Selected logs report browser7, UI9, IDE20, SQLite12, runtime13, contracts7, native lifecycle4 and integrated pilot1:73 successful harness functions. ONE runtime link test explicitly records NOT RUN because Windows native creation returned rawcode1 Incorrect function on this filesystem;72 cases exercised their target checks. Do not count native link/reparse rejection as verified. Zero-test binaries also exited safely but provide no feature acceptance. The synthetic poisoned-child ownership panic is intentionally caught by its passing failure-injection test, visible because --show-output is enabled.

Four real native lifecycle tests passed, including detach/resize/reattach with contiguous StreamCursor output, observed natural exitSome0 BEFORE close and unchangedSome0 AFTER close, explicit owned stop, drop receipt, and fixed lifetime. Cleanup assertions require observed Exited, actual reader/lifecycle joined and exact cwd removed. The integrated pilot passed synthetic client/workspace records, duplicate request replay, real SQLite reopen and READY/PONG/DONE native PTY journey with actual thread/cwd completion; run_headless_pilot also rejects absent natural exit or nonzero code. These are local synthetic fixture results, not production process-tree/SSH/WSL/MSYS acceptance.

verification-steps.json independently shows format, workspace check, test compilation, guarded execution and Windows child/scene compile each exit0. Feature clippy was still running at this read, so this document does not claim every verifier gate completed. Scene is compile-only: no GUI, focus, IME, clipping, accessibility or native render acceptance. Windows compile output retains one unused upstream cursor-constant warning after the intentionally narrow vendor experiment; no wrapper runtime behavior was exercised.

APPROVE finite local Windows headless pilot evidence and compile-only diagnostic source boundary. Prior abnormal0xc000013a remains unexplained; this successful run proves the strengthened assertions passed on this run and does not establish cause repair or repeated-run stability. Native read cancellation/synchronous ClosePseudoConsole hard deadlines remain unproved; outer private-job watchdog is the containment boundary for tests. Windows link/reparse test unavailable, non-Windows guard, Wayland/macOS native composition, GUI, benchmarks, production profiles and production cleanup NOT RUN/NOT ACCEPTED. Manual CI workflow source approved separately, no published CI job evidence here.

## Receipt-selected evidence hashes
Receipt SHA-256: 53A1D017C9A0472CE826EAB6AC8CE5C4C74F03354685754089714803624E9F6E
- 00-browser_policy-ffe5dc5c29684c41.log: A0A4B60E2173EAAA81221B661B8E694C6E50FBFB8F184E2214A43CCD6FC02C89
- 01-scene_policy-e24851f6a09220e8.log: 1F75F592969E3BBF76948BACD3B9CB77833E674C4C2AC1B813140E4653DF9137
- 02-wilde_ide-a9ad64da2301dc85.log: 9DBBA7C3324026073C96CE079CE329319E185E5399CAA862D247FAE14FCB6470
- 03-sqlite-3e0b0a6a48cf93ec.log: 801B06F106B2D6022DB6AF35E606BFD87C778FF2B2846F94D02831405AA38092
- 04-wilde_runtime-85e9dada5a6c18cc.log: 1B960747500F49C14C2A0001045A1310DA644BAB5DD6B439526CA4896014E8B9
- 05-wilde_desktop-88bf298cd7fdd6ef.log: 5005259F2BF5BDA55D5BAC9E61B7BCFBE75A9A9FD42D0FF00623682EDFD7B6A4
- 06-pilot-3436b4bb1fa93ebf.log: FBE3D136147338C7E72578CBB2E6E75C28FF743E73AA57CD7539435DF8D864D0
- 07-wilde_desktop-e00c0d9ebf9ba506.log: 5005259F2BF5BDA55D5BAC9E61B7BCFBE75A9A9FD42D0FF00623682EDFD7B6A4
- 08-wilde_domain-07b36f04555022d2.log: 5005259F2BF5BDA55D5BAC9E61B7BCFBE75A9A9FD42D0FF00623682EDFD7B6A4
- 09-wilde_contracts-49997b206d700627.log: 297051449AEF49FEA5D783E38E822965F1EE9284F9F5C22B0E187053C7CDB3FE
- 10-wilde_browser-07d4f2ec4c1c6b42.log: 794DE10484CD1ACB1D14C6084197ED50DD6AF82CA3CC23095E55DB84766DA0E7
- 11-wilde_ui-300c7f4d91fd22a1.log: 5005259F2BF5BDA55D5BAC9E61B7BCFBE75A9A9FD42D0FF00623682EDFD7B6A4
- 12-fixture_lifecycle-9c5806f7e2573ed9.log: C400D4BF9C84FD5E0D02448B77C8FD815548D461F74DB388FBCD5B614DDEDFE0
Verification steps SHA-256 at read: C5F9E2A93A13AE8741B07970855B9F46FB4658C1156F931879319E79455A5FD1
