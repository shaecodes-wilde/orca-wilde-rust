# Package validation

Built 2026-09-29 for the Wilde Native Rust kickoff.

## Executed in this environment

- **29 automated helper tests passed**, using Python 3.13.5 and Git 2.47.3 in a Linux container.
- Worktree tests used disposable local repositories, not Orca-Wilde or any user checkout.
- Tested dry-run creation, safe reuse, clean integrated removal with branches retained, and refusal for unmerged commits, changed HEAD, locked trees, ignored files, untracked files, missing process attestations, invalid roles, nested roots and unregistered directories.
- Tested benchmark comparison directions/thresholds, metadata and metric-definition mismatch, template rejection, nonfinite/boolean/negative inputs, p95 sample minimum, zero baseline and release-mode requirement.
- Verified the task dependency graph is acyclic and owner/branch mappings match the helper.
- Parsed every Python and JSON file and exercised each helper's command-line help.
- Archive paths, entry contents and checksums are checked during final packaging.

Run the tests again after extracting:

```sh
python -m unittest discover -s tests -v
```

## Not executed or claimed

No native Rust application was compiled, benchmarked or migrated here. This package contains migration instructions and helper tooling, not application implementation or empty application stubs. Codex's first implementation task is to create and validate the real Cargo foundation inside an isolated repository worktree.

The helper scripts were not executed on Windows or macOS here; their use of cross-platform Python/Git is not a claim of platform validation. Native UI/browser/PTY behavior, installer paths, real model routing, external runner availability and user hardware performance remain kickoff checks.

The repository review was selective and pinned to `wilde/theme` commit `1db4ef38577ec932f97067b7391625b7be599884`. No user repository changes, GitHub issues/PRs, app installations, account changes or profile migrations were performed while creating this ZIP.

Worktree process/material closeout checks include explicit attestations from the actual operator/agent; the helper does not independently prove OS-wide process ownership. Benchmark fixture numbers inside unit tests are synthetic test inputs, not Orca-Wilde performance results.
