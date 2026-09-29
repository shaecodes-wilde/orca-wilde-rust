# Wilde Native — Codex kickoff

**Goal:** migrate Orca-Wilde to a native Rust desktop application and Rust-owned runtime, without sacrificing the working application, client data, agent sessions, or cross-platform behavior.

**Team:** six engineering roles: 2 Astra + 4 Opus 5.5. The main Codex session is Agent 1; it coordinates five workers, not six additional workers.

## Start

Extract this folder outside your live repository, open the Orca-Wilde repository in Codex, and give Codex the extracted folder path with the prompt in `START_HERE_CODEX.md`. Do not overwrite your repository's existing `AGENTS.md` or `.codex` configuration.

The first assignment includes implementation: inspect the checkout, establish the six ownership tracks, run safe preflight, create the isolated migration foundation, and begin the first small tested change. It is not an instruction to perform the whole rewrite in one uncontrolled run.

## What is inside

| Location | Purpose |
|---|---|
| `START_HERE_CODEX.md` | Main kickoff prompt and immediate deliverables |
| `AGENTS.md` | Compact operating rules for this package |
| `prompts/` | Six role-specific prompts and a continuation prompt |
| `docs/` | Architecture, scope, contracts, milestones, safety, tests, sources |
| `coordination/` | Ownership map, dependency-aware backlog, feature-parity register |
| `templates/` | Decision, handoff, change request, closeout and benchmark templates |
| `tools/` | Standard-library Python preflight, worktree management and benchmark comparison |
| `tests/` | Executable tests for the supplied helper tools |
| `VALIDATION.md` | What was actually tested when the package was built |

## Deliberate boundaries

This is a migration kickoff package, not a completed app or an auto-installing plugin. It contains no credentials, production data, model access configuration, downloaded dependencies, or claimed performance results. The Rust application scaffold is the first repository deliverable for Codex; it is not represented here by empty crates pretending to be a migrated system.

Native means no Electron/React/DOM requirement for the final first-party desktop UI, editor or terminal. Browsers and platform libraries may remain mature non-Rust dependencies behind Rust interfaces. Tauri/React or a WebView-rendered shell must not silently replace the requested native end state.

Source review: 2026-09-29; `wilde/theme` at `1db4ef38577ec932f97067b7391625b7be599884`. Codex must re-read the current checkout and account for newer commits and local changes. The source review is selective, not a complete audit.
