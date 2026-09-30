# A6 independent evidence review: alpha-04 and unchanged-source alpha-05

Verdict: FAIL repeat acceptance. Alpha-20260929-04 is valid finite Windows headless success evidence; alpha-20260929-05 fails on the same source and identical fourteen executable hashes. Do not promote one passing run into reliable increment acceptance, timeout repair, full alpha, rendered composition or performance acceptance. A6 performed read-only evidence/source inspection and hashing; no build, test executable, native window or measurement was run by A6. A6 retention remains independently certifiable by A1, not its author.

Both summaries select source 89f5d43ed4b4a94e9ee148b6c4651a6a61b69091. Their guard IDs are c8367cfed1ed4063afc47b8e4e851dcd (04) and ebad526e0f3f4b1d9c98871a6e2267a8 (05), completed=true. Independently checked every summary-selected log and executable SHA256: 28 matches per run. All fourteen selected executable hashes also match across runs, despite their different invocation order. Alpha-05 summary entries match all fourteen exact guard receipts. Summaries record hashes captured after guard completion before another build; current binary matching is independently observed. Diagnostics added afterward are outside this frozen evidence source.

## Passing run and failed repeat

| Boundary | Alpha-04 | Alpha-05 |
| --- | --- | --- |
| Verifier | Eight steps exit0, including Windows child/scene compilation and scene/root lint | Format/check/test-build exit0; guarded-tests exit1. Later feature/lint steps NOT RUN |
| Owned guard | Fourteen executable exit0; no timeout or forced remaining children; every owned job empty | Eleven executable exit0, two exit101, one watchdog exit124; every owned job empty |
| Composed continuous terminal | 18,874,542 admitted bytes, next sequence4750, final sentinel, unchanged identity/PID, no stream fault, natural exit0 before and after close, reader/lifecycle joined, fixture cwd removed | Sentinel deadline error; one failed test, harness25.30s, executable25.907s. No captured model progress, fault or cleanup receipt at the early return |
| Raw runtime canonical sink | 18,874,760 bytes, next sequence73989, unchanged identity/PID, no final fault, exit0, both workers joined, cwd removed | Visible FAILED entry; detailed panic/receipt missing because fixture harness was terminated before libtest printed captured failures |
| Existing composed pilot | Passed natural-zero and cleanup assertions | Exited code1; reader/lifecycle joined, cwd removed, no removal error. Total close380.6142ms, master drop46.8us, manifest27.5637ms, deadline_exhausted=false. Natural-zero assertion correctly failed |
| Lifecycle harness | Six tests complete and pass | Partial: two visible ok entries, three visible FAILED entries, sixth unfinished. Guard timeout=true, exit124, wall52.094s. No complete harness verdict |

The alpha-05 watchdog used owned-job termination. forced_remaining_children=false describes the subsequent survivor phase and must not be read as absence of forced termination. Empty jobs establish final containment, not successful shutdown within original fixture deadlines. The 25.30s harness duration does not measure the seven-second sentinel interval alone: the pre-error work and Drop cleanup are not separately visible. Do not infer which phase consumed the extra time.

Alpha-04 reports91 successful harness functions, including one explicit NOT RUN native symlink case;90 exercised checks. Alpha-05 completed passing harnesses report83 successes, again including that explicit NOT RUN case. Two additional partial lifecycle ok entries are visible, not a completed lifecycle receipt. Native symlink creation reports Incorrect function/raw OS error1, so actual link/reparse rejection remains unverified. IDE30 deterministic model checks and scene/browser policy9 each pass in both runs. Browser author-owned retention behavior is A1's independent-review responsibility; neither policy logs nor feature compilation prove live browser/native rendering. The unused vendored cursor constant warning remains visible.

## Source authority and missing evidence

The integrated vendor process-state source matches the independently reviewed A2 correction 0fcee9797b217dd17594ac73bfa208f88145d3c6: retained owned handle must be signaled before exit status is accepted. Its src/win/mod.rs SHA256 is 9340A166A12D1907F84B6A69C8FFBB8EFD8022544AE217884E411AF12E0FE1E6. This strengthens source authority independently of these failures. Genuine final259 and API error branches were not independently exercised live. Alpha-02's original cleanup timeout and alpha-03's nonzero exits remain separate unresolved failed evidence; these runs establish no causal explanation or stable repair.

The continuous success exercises one in-process canonical model behind a real synthetic Windows PTY and a detached observer flag. Separate daemon ownership, UI restart/reconnect, interactive shell/editor/cell renderer, Windows/Wayland focus/IME/clipping/DPI/accessibility, macOS/Linux adapters and production hosts remain unverified. No verified isolated display is available. No release benchmark, paired baseline, steady-state memory/CPU or efficiency claim follows from debug test timings or bytes. Existing composition/performance report remains the blocked runner and preregistered protocol authority.

Next action: preserve original assertions, 7s sentinel/8s lifetime/2s close budgets and guard limits; capture early failure observations into the existing file sink. Narrow source-reviewed diagnostics may expose stage elapsed time, nonblocking model progress, bounded supervision decisions/adapter errors and first signaled exit. An adapter return is not proof of OS termination. Review their exact source before a focused A1-guarded diagnostic run; no blind retry or deadline reset.

## Evidence identity

Paths relative to F:/Orca Wilde Rust/evidence/alpha-runs/. Summaries enumerate all exact selected logs and executable hashes.

| Path | SHA256 |
| --- | --- |
| alpha-20260929-04/summary.json | 0E93A75F6534FF8224B8051CAB91530E41E5A8A884840CDCA385B050FE7099FE |
| alpha-20260929-04/verification-steps.json | AB100369BB4471726DF7E3CE3A3594574992BADB47AFA637E592FCCD387C344A |
| alpha-20260929-04/guarded-tests/results.json | B66E8074FDB9FD41405A6660C8AB8417E3B57DA238BDA61CD6C33A39FEB1F8AB |
| alpha-20260929-04/cargo-test-artifacts.jsonl | 4CDC0F25C2DC072B02C5DAEDF7149435C4B756934ADF692539BD19EA241383D6 |
| alpha-20260929-05/summary.json | 8F97721463FBEEE4365E7F17FADE86B5AFA6B162E8C45695C575985D0B0B1CC9 |
| alpha-20260929-05/verification-steps.json | 512CDFDDB65EAA81AF8F5886FEC4A83D6247B53B944DC5237DE3648FF900406B |
| alpha-20260929-05/guarded-tests/results.json | B9DFD13A0C639BD93A2236B643C2961A4785EFEA9AB25043CE2538B4A6A7BAAE |
| alpha-20260929-05/cargo-test-artifacts.jsonl | 8FF2961A2E319882B0057C20B81872EB1D92DA073F8B7571738EA0652E6DD0F1 |
| alpha-20260929-05/guarded-tests/02-continuous_terminal-bbadcfcd0aaaace9.log | 6868C6B81E7F5FF70F44AB15CCEB78D076D4986ED4D51F8AF38300A64CDF480A |
| alpha-20260929-05/guarded-tests/03-pilot-3436b4bb1fa93ebf.log | C7C09FC80081C5B505E87CF1C58ECA2FCF2B35E902D79BEFA3C6BD324CA51B94 |
| alpha-20260929-05/guarded-tests/06-fixture_lifecycle-9c5806f7e2573ed9.log | 412A3761478BCCA5CCD113FAE33C49ECD4558FB0A3C56634ED5CF18CFE9688B0 |
