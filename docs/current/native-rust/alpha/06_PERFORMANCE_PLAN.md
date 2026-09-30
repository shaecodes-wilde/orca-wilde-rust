# Performance and resource acceptance protocol

**Purpose:** demonstrate whether the native architecture reduces first-party overhead without losing useful behavior. This is a proposed experiment, not a current benchmark result. Existing source-report snapshots are not comparative measurements. [S5]

## Before measuring

Use separate development builds/profiles of legacy and native apps on the same measurement host, one at a time. Do not benchmark the user's installed/live app. Match release mode, repository fixtures, terminal geometry/transcript, saved workspace count, visible panes and browser content. Record any feature mismatch; exclude incomparable pairs from savings claims. Native build flags must be reproducible.

Record OS/version, CPU/RAM/GPU, filesystem, display scale/resolution, compositor where applicable, browser engine/version, power mode, toolchain/build flags and exact source/artifact IDs. Do not expose personal paths, credentials or private projects in public evidence. Windows and Linux have separate baselines and metric definitions; do not combine unlike memory counters.

Use deterministic local terminal workloads for comparison, not variable live model output or paid agent calls. Run one separate approved actual-agent smoke test for functionality. Stop unrelated development builds on the measurement host; do not terminate unrelated user processes to make a benchmark cleaner. If the host cannot be isolated, record the confound and defer acceptance measurement.

## Workload set

| ID | Paired workload | Required observations |
| --- | --- | --- |
| B01 | Cold and warm start into a quiet native/legacy workspace | Launch-to-usable timing, idle memory/CPU, process categories, warm/cold definition |
| B02 | 100 saved inactive synthetic workspace records, no active tools | Incremental metadata cost, absence of eager child sessions/watchers/browser/LSP activity |
| B03 | Three active local terminals with the same recorded output, geometry and timing | UI responsiveness, parser/render throughput, queues/history memory, final sentinel correctness |
| B04 | B03 plus the same local browser fixtures | Browser and total-workload cost, engine/version differences, focus/input correctness |
| B05 | Declared sustained stream and repeated create/switch/close cycles | Memory trend and plateau, handle/process/task counts, spool/cache disk bounds, cleanup time |
| B06 | Hidden/minimized UI with active sessions; then reconnect | Idle repaint/timer behavior, uninterrupted drain, correct state recovery, no duplicate children |

Proposed reproducibility floor: five paired repetitions for B01–B04; at least 30 lifecycle cycles and a 20-minute declared sustained-output run for B05; repeated orderly/abrupt UI detach for B06. These are test durations/counts, not a promise about agent completion time. A1/A6 may preregister justified alternatives due to hardware constraints; an after-the-fact reduction to hide failures is not acceptable. B02 must use genuinely comparable saved-workspace semantics in both apps; otherwise label it native-only and do not compute a ratio.

## Resource accounting

Separate: first-party shell/runtime; browser engines and children; coding tools/agents and descendants; language servers/watchers; total attributed workload. Record the measuring runner/compiler separately, outside application totals. Count only descendants/objects whose ownership is established. Shared engine processes and shared memory must be labeled; do not naively double-count shared pages or claim a process was destroyed when the platform legitimately retains a shared engine.

Use appropriate OS counters and state their names/units: retained/private allocation and resident/working-set measurements are not interchangeable. Record CPU sampling interval, UI timing method, handle/task counts and cache/spool disk. Mark unsupported metrics unavailable. A proxy event-loop timing metric is not automatically input-to-photon latency.

## Preregister acceptance budgets

After collecting the baseline and calibrating measurement noise, but **before scoring the native candidate**, A1/A6 record numeric budgets for:

- First-party idle/active memory and the minimum improvement beyond measurement variability.
- Interactive p95 timing and allowed regression/noise for each comparable workload.
- Idle CPU/background work, shutdown/cancellation deadlines, retained scrollback/replay/queue/spool caps.
- Allowed settled lifecycle variation and resource plateau criteria.

Include rationale, units, measurement tool and excluded/incomparable scenarios. Do not insert an invented universal RAM promise. If budgets remain unset, P02 is BLOCKED, not PASS. Preserve each protocol revision and rerun affected workloads when it changes.

Report raw samples, median and range; use quantiles only with adequate event samples and explain the sample population. Five startup runs do not justify a robust p95 claim. Separate shell savings from total-workload savings; report regressions and platform/browser differences. Flag a native-only feature subset that makes a pair artificially cheaper.

## Interpretation and evidence

A functional alpha can exist while efficiency is not yet validated. Efficiency passes only when the predefined comparable criteria pass and lifecycle resources remain bounded. If native overhead is not better, profile the actual hotspots and repair them; do not declare Rust itself proof.

Record the candidate hash/build artifact hash, baseline hash, run IDs, timestamp, fixture/transcript hashes, protocol/budget revision, actual results, exclusions and reviewer. Screenshots/process snapshots supplement time-series measurements. Repeated bounded tests establish that specific result, not indefinite leak-free operation or full cross-platform parity.
