# Wilde Native Rust work rules

Follow the supplied kickoff package and Shae's direct instructions. New app location is F:\Orca Wilde Rust; this supersedes the package's native-rust subdirectory proposal. Keep existing Orca checkouts, profiles, credentials, processes and launch behavior unchanged.

One owner per path: A1 owns root Cargo files, contracts, apps, integration and CI; A2 runtime; A3 domain/storage and synthetic fixtures; A4 UI; A5 IDE; A6 browser/platform/governor. Ownership paths omit the kickoff's native-rust prefix because this is a separate repository. Propose cross-owner contracts; acknowledgement is required.

Workers use their dedicated F:\wr\a0N worktree. Source-reference reads against the pinned old commit are permitted; never edit any old checkout. Use synthetic data only. No secrets, global config edits, external writes, dependencies installed by workers, recursive agents, or production app launches.

At most three implementation workers; A1 alone runs one build at a time, CARGO_BUILD_JOBS=2, outputs outside worktrees under F:\wr-build. No worker Cargo build without a granted coordinator lease. Never run native GUI/focus tests on the user's desktop; use an isolated display/VM/CI. ORCA_BACKGROUND_LAUNCH=1 is required but does not make native windows safe. No production profiles or integration accounts.

Reuse existing behavior/contracts after source inspection. Record tests and missing evidence honestly. No mock/parity/speedup claims. Native UI translates documented Wilde token roles; no React/HTML first-party UI. Keep Windows/macOS/Linux and folder/SSH workspaces in scope. Host process verdicts are live/unverifiable/exited; disconnected hosts must never fall back to local execution.

Keep comments concise, files domain-named. Scoped local commits and worktrees are authorized by kickoff; GitHub publication awaits exact repository approval. Cleanup requires reviewed integration ancestry, clean tracked/untracked/ignored files, durable evidence and confirmed stopped task processes. Retain when unknown.

## Native Workspace Alpha — current milestone

Continue the existing migration using `docs/current/native-rust/alpha/01_SYSTEM_PROMPT.md`; A1 also reads the implementation and acceptance plans. Workers read only their assigned brief, contracts and acceptance cases. Preserve all existing safety, ownership, model-routing, publication and build-lease rules.

Finish line: a demonstrated native coding loop with real disposable worktree, interactive local session, approved coding-agent exercise, native editing/diff, workspace switching, correct UI reconnect and measured bounded resource costs. First-party UI remains native; browser engines are for browser content. Keep the installed Orca app and profiles untouched.

Close Windows/Wayland composition evidence before broad toolkit-specific UI work; continue independent headless tasks while a runner is blocked. Extend existing task/parity records rather than replacing them. Report implemented, compiled and exercised separately. No mock, blocked or unrun test counts as passed; functional and efficiency verdicts remain separate. Preserve concise evidence/resume state and remove only verified safe disposable worktrees through the existing cleanup rules.
