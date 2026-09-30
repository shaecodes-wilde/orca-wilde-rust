# Orca Wilde Rust — Native Workspace Alpha
## Next-phase execution handoff • 29 September 2026

**Finish line:** Complete a controlled coding session in the native application: create a disposable Git worktree, launch and interact with a real local shell and an explicitly approved coding agent, edit/save/review a change, switch workspaces, reconnect the UI to the still-running session, then clean up only owned resources. Demonstrate comparable resource costs and meet preregistered efficiency criteria. This is a development alpha, not permission to replace the installed app.

### Use this package

Open the **orca-wilde-rust** project, not the working **orca-wilde** application repository. Attach this ZIP to its coding thread where supported, or extract it into a new, short folder such as `F:\wr-alpha`. That path is an example; do not relocate either existing project. Paste `02_START_THREAD.txt` into the coordinating thread and make the files accessible to that agent.

A1 first reads the package without installing anything. After repository preflight, it may adopt these documents under `docs/current/native-rust/alpha/` using its existing local-write authority. If that directory already exists, reconcile differences rather than overwrite it. A1 may append the small `09_AGENTS_APPEND.md` section once to the existing project `AGENTS.md`, retaining all current rules. Do not replace the root file, create an overriding instruction file, or edit global agent configuration.

`01_SYSTEM_PROMPT.md` is the coordinator instruction text. Use a genuine configurable system-instruction field only when the runner supplies one; otherwise supply it as project/task instructions. A filename does not grant higher privileges or change tool permissions. Codex documents repository `AGENTS.md` guidance and layered instruction discovery. [S10]

### Keep the thread small

A1 reads the system prompt, start message, implementation plan, acceptance tests and current repository state. Workers receive the shared worker preamble, their own brief, relevant source/contract files and only their assigned acceptance cases. Nobody needs every historical report in every prompt.

| File | Purpose |
| --- | --- |
| `01_SYSTEM_PROMPT.md` | Coordinator's operating contract |
| `02_START_THREAD.txt` | Initial message that starts implementation |
| `03_IMPLEMENTATION_PLAN.md` | Dependency-led stages mapped to existing R-tasks |
| `04_ACCEPTANCE_TESTS.md` | Observable pass/fail criteria and blocker handling |
| `05_AGENT_BRIEFS.md` | Six roles, ownership, dispatch and review rules |
| `06_PERFORMANCE_PLAN.md` | Comparable workload and resource-measurement protocol |
| `07_RUN_RECORD_TEMPLATE.md` | Human-readable handoff and approvals record |
| `08_RESUME_THREAD.txt` | Restart/compaction continuation prompt |
| `09_AGENTS_APPEND.md` | Small project-only instruction addition |
| `10_SOURCES.md` | Pinned source references and uncertainty notes |
| `templates/alpha-run.json` | Empty machine-readable acceptance/evidence record |

### Scope and permissions

The package continues the existing project. It neither resets repository state nor imports production data. Existing local implementation authority remains; GitHub publication, hosted CI dispatch, new paid services, real accounts and live coding-agent/API calls require the specific approvals applicable to those actions. Purely local fixtures remain the default. A missing runner or approval blocks its dependent acceptance case, not unrelated safe engineering.

**No application code was executed or changed in preparing this handoff. No repository write or publication was performed.** Source-state observations are not new runtime test results. See `10_SOURCES.md`.
