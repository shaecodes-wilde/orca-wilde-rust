# 10 — Evidence and references

Reviewed 2026-09-29. Repository sources are pinned to the reviewed commit where possible. Web documentation can change; verify versions at implementation. Statements in this package about the proposed architecture and gates are recommendations, not measured performance findings.

## R1 — Branch listing

https://api.github.com/repos/shaecodes-wilde/orca-wilde/branches?per_page=100

Feature-branch and main commit identity at review time.

## R2 — WILDE.md

https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/WILDE.md

Shared current profile, business/integration features, hard fork, update posture.

## R3 — AGENTS.md

https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/AGENTS.md

Operational, platform, testing and remote behavior constraints.

## R4 — package.json

https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/package.json#L125-L178

Selected existing benchmark/test scripts.

## R5 — BusinessDatabase

https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/src/main/wilde/business/database.ts#L1-L170

Current command, revision, idempotency, preview and snapshot behavior.

## R6 — BrowserBackend

https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/src/main/browser/browser-backend.ts

Electron-backed desktop/offscreen interface.

## R7 — Browser retention

https://github.com/shaecodes-wilde/orca-wilde/blob/1db4ef38577ec932f97067b7391625b7be599884/src/renderer/src/components/browser-pane/host-guest/browser-guest-worktree-retention.ts

Default 8 hidden warm pages and protection exclusions.

## W1 — OpenAI Codex subagents

https://developers.openai.com/codex/multi-agent/

Verify installed-version configuration and actual model routing; preferred role labels alone do not launch models.

## W2 — Iced upstream

https://github.com/iced-rs/iced

Native Rust UI candidate and renderer architecture; prototype suitability.

## W3 — Slint licensing/get started

https://slint.dev/get-started

Native UI candidate; inspect actual license terms before distribution.

## W4 — Rust ownership

https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

Ownership model; no whole-application performance guarantee.

## W5 — Electron process model

https://www.electronjs.org/docs/latest/tutorial/process-model

Baseline process separation and browser hosting.

## W6 — Electron security guidance

https://www.electronjs.org/docs/latest/tutorial/security

Retain isolation and distrust remote web content during transition.

## W7 — WRY documentation

https://docs.rs/wry/latest/wry/

Platform-specific native webview/child embedding considerations.

## W8 — portable-pty documentation

https://docs.rs/portable-pty/latest/portable_pty/

Potential reusable PTY abstraction; not a terminal renderer.

## W9 — Git worktree documentation

https://git-scm.com/docs/git-worktree

Linked worktree lifecycle and safe removal; current Git-version support must be checked.

The package does not rely on previously quoted model migration claims, model benchmarks or large file counts. It does not assert that a particular agent runner exposes Astra or Opus 5.5. The current environment must establish that. No entire third-party source files or brand/font assets are redistributed.
