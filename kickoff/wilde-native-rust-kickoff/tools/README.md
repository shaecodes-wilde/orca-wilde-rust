# Helper tools

These tools support the handoff; they do not launch agents or perform the Rust migration. Python 3.10+ and Git are required. All scripts use the Python standard library. Review before executing on your machine.

## Read-only inventory

```sh
python tools/preflight.py --help
python tools/preflight.py --repo /actual/repo --output /outside/repo/preflight.json
```

Reports Git/tool metadata, free disk and existing benchmark script names/commands. It does not execute those benchmarks, read `.env` files or print raw remote URLs. Output files are create-only. Tool availability is not model access. Keep local absolute paths private when publishing a report.

## Worktrees

```sh
python tools/worktrees.py create --help
python tools/worktrees.py create --repo /actual/repo --root /short/wr --base FULL_COMMIT_SHA --roles a01
```

Default is a dry run. Add `--apply` only after reviewing the plan. After A1 integrates initial contracts, use that newer exact SHA to create ready worker roles. Existing matching trees are reused without resets. This tool does not synchronize branches, fetch remotes, delete branches, install dependencies, or launch models.

```sh
python tools/worktrees.py remove --help
python tools/worktrees.py remove --repo /actual/repo --root /short/wr --role a02 --closeout /outside/repo/a02-closeout.json
```

Default is validation only. `--apply` also removes the exact registered worker tree using Git without force. Review `docs/09-worktrees-and-lifecycle.md` first. Closeout attestations must be based on actual process/material inspection; the helper cannot independently prove every OS process is stopped. It requires preserved evidence outside the removed tree, exact reviewed HEAD and integration SHA, merge ancestry, and clean tracked/untracked/ignored state. Squash/rebase ambiguity is a deliberate refusal, not a reason to bypass safeguards. Retained branches prevent accidental loss and require deliberate reconciliation before reusing the same role branch after cleanup.

## Compare actual measurements

```sh
python tools/compare_benchmarks.py --help
python tools/compare_benchmarks.py baseline.json candidate.json
python tools/compare_benchmarks.py baseline.json candidate.json --max-regression-pct 5 --output comparison.json
```

The `5` above only illustrates the command syntax; it is not the project's approved performance budget. Set scenario-specific thresholds before comparing candidates. The comparator accepts only complete `run_kind: measured` reports with actual samples and compatible metadata. Use the template to structure collection, not as a result. Exit 0 means an informational report or explicit-threshold pass; exit 1 means threshold fail; exit 2 means invalid or incompatible evidence.

## Test the helpers

From the extracted package root:

```sh
python -m unittest discover -s tests -v
```

Tests create temporary Git repositories and synthetic benchmark data. Synthetic test values are not Orca-Wilde measurements. See `VALIDATION.md` for platforms actually tested. No third-party Python package installation is required.
