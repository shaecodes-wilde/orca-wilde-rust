# A6 independent evidence review: alpha-20260929-03

Verdict: FAIL integrated guarded verification; diagnostic source did not establish a repair. No A6 build/test/native launch or source change outside role documentation. Preserve alpha-02 cleanup failures and alpha-03 nonzero exit failures independently. Full alpha, GUI/platform composition, daemon/recovery and efficiency acceptance remain open.

Reviewed frozen primary source 2cda7ca3b40d5cb7da47a0b02f0f8439add78fac, alpha run record source_commit matches. Current tracked source clean; only untracked evidence. A2 diagnostic source matches independently reviewed immutable commit 9674f7b6f37a1c126a5dd9ab4c35477e5f373659 exactly by working-byte SHA256: runtime/src/lib.rs 79D35F9C36A034B679E8E82740EC1691EF230897950E19CE15FA63B803E3E6CB; runtime/src/fixture-lifecycle.rs 69C3602A816CD01AD960B13C2ED2D4AFD62C5B54482CA8D9C882D04CE596BB43. Original deadline placement and test assertions remain unchanged.

Guarded run ID 7c35408e6b8d4aa49e71658e3340f51e, completed=true, fourteen exact receipts. Three executable exit101, eleven exit0; zero timeout/forced_remaining_children and all fourteen owned_job_empty=true. Format/check/test-build exit0, guarded-tests exit1; later Windows feature/lint checks NOT_RUN. This remains debug finite test evidence, never release/performance comparison.

## What the fresh diagnostic actually measured

| Failed test | Cleanup phase observations | Returned outcome |
| --- | --- | --- |
| Existing composed pilot | writer20.9 us; verdict185 us; master36.4 us; worker38.1 us; manifest15.0402 ms; total87.6335 ms; deadline_exhausted=false | Exited, code3221225786 (0xC000013A), reader/lifecycle joined, cwd_removed=true, no removal error. Natural-zero acceptance failed. |
| Continuous composed terminal | writer12.3 us; verdict279.9 us; master26.1 us; worker124.8 us; manifest14.9418 ms; total120.1778 ms; deadline_exhausted=false | Exited, same nonzero code, reader/lifecycle joined, cwd_removed=true, no removal error. Natural-zero acceptance failed. |
| Runtime native canonical sink | No successful/failed phase-timing Debug record captured by this test's custom print; admitted18874760 bytes, sequence74022, unchanged identity/PID, final_fault=None | Same nonzero code, reader/lifecycle joined and cwd_removed=true; code0 assertion failed. No successful native receipt verdict. |

These two measured master drops are short. They do not show the previous suspected blocking phase or reproduce alpha-02 cleanup timeout. The lifetime case that failed in alpha-02 passed in this run, but successful phase timings were not printed, so no measured phase explanation follows. Nonzero exit cause remains unknown; do not call it a repaired timeout, explain it from an API's historical semantics, or accept it as normal shutdown. Returned error assertions correctly preserve the failures even though directories now disappear within these observed cleanup calls.

IDE30 deterministic checks and scene/browser policy9 each passed again; SQLite12, contracts7 and runtime unit harness16 successes. Runtime unit output still explicitly reports actual symlink creation NOT RUN with raw error1. A6 author-owned browser changes remain A1's independent-review responsibility; these logs cannot prove actual browser/native rendering. Expected injected canonical-consumer panic remains a successful fault-path test.

Next action: A2/A1 capture original pre-close natural-exit verdict/code and shutdown/supervisor/cancellation ordering for the exact failing fixture, retaining original deadline and zero-exit assertions. Diagnose the current nonzero exit independently and instrument the original cleanup-timeout path if it recurs. No deadline reset, broad retry, quiet assertion removal or production-stability claim. Preserve both complete runs and their identities; do not pool them into a success claim.

## Reviewed evidence hashes

Paths below relative to evidence/alpha-runs/alpha-20260929-03; receipt selects only this run's fourteen logs.

| Path | SHA256 |
| --- | --- |
| verification-steps.json | 9A9F1D7EB58E3245E693DF3E59D0C93B3E46D93F77E600F27E181E797D9433A2 |
| guarded-tests/results.json | 653DFADA52F5BCFD3AEF3B14526DB24EF7E4DE5EC267278DD2481B67D4509577 |
| cargo-test-artifacts.jsonl | B24B6902F4B6BD131B751ACF09DC5B48B0114841CF827BA3D5B1574162CE8A0E |
| guarded-tests/06-pilot-3436b4bb1fa93ebf.log | FCD875A3E64A5326543373630B7C02CD94E9FFF500A94E78C4940680164878EF |
| guarded-tests/07-continuous_terminal-bbadcfcd0aaaace9.log | 6FE8FEF345524008817FB59863FD58FA2A89F3C540B067E02498D36AA647A20B |
| guarded-tests/13-fixture_lifecycle-9c5806f7e2573ed9.log | 27F6936FADD0C10D9B8144DF760C7DBFF1969121290B2AABCAA1D86912E5AC9D |
