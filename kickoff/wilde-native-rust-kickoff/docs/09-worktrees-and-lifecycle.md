# 09 — Disk-conscious worktrees and safe closeout

Use linked Git worktrees, not full repository clones. They share Git objects but still duplicate checked-out files; build targets, browser profiles, dependencies and test data consume additional disk. No disk-saving percentage is assumed. Inspect free space before provisioning and avoid installing Node dependencies into every Rust worktree. [W9]

## Helper usage

Python 3.10+ and Git are required. On Windows, `py -3` can replace `python` when appropriate. Run help first. Use real absolute paths selected for this machine; examples are not automatic commands against your home directory.

```sh
python tools/preflight.py --repo /actual/repo --output /outside/repo/preflight.json
python tools/worktrees.py create --repo /actual/repo --root /short/wr --base EXACT_SHA --roles a01
# The preceding command prints a dry-run plan. Add --apply to create the reviewed worktree.
```

Create A1 first, integrate the contract foundation, then create ready worker trees from that NEW exact integrated commit. The six role names map to fixed branch/path suffixes in the tool. Provision only ready roles. Re-running creation validates an existing worktree rather than resetting it. A mismatched path/branch/common repository fails closed. No checkout, reset, remote fetch, dependency installation or agent launch is done by this tool.

Keep mutable build outputs outside worktrees selected for cleanup. Separate targets per active branch unless one runner serializes access; a shared immutable/download cache is different from a shared mutable target directory. Git's common config is shared: workers must not edit it casually.

## Cleanup decision

Automatic closeout is authorized only after the work is integrated, evidence is durable and task-owned processes are stopped. Never delete by age, branch name alone, or an agent's statement that it is done.

Before removing a worker: verify its exact HEAD is an ancestor of the approved integration reference; verify clean tracked/untracked/ignored state; preserve unique configuration, evidence and user data; confirm no task processes, sessions or locks remain; confirm exact registered path and matching repository. Squash/rebase integration may not satisfy ancestry: retain and reconcile evidence manually rather than bypassing the check.

`tools/worktrees.py remove` requires a closeout file based on `templates/closeout.json`. The file records the reviewed HEAD, integration reference, durable evidence URI and explicit process/material checks. Those two checks are human/agent attestations backed by actual process inspection; the helper does not pretend it can infer process ownership on every OS. It never kills processes. Its dry-run is default; `--apply` removes only after all checks pass.

```sh
python tools/worktrees.py remove --repo /actual/repo --root /short/wr --role a02 --closeout /outside/repo/a02-closeout.json
```

Ignored files also block cleanup; this deliberately protects `.env`, local notes and unknown caches even though Git may otherwise consider them disposable. Relocate genuinely reproducible cache outputs only after inspection; preserve unique material. The helper does not remove A1, branches, lockfiles, or unrelated directories and never uses force removal or broad `git clean`.

If cleanup is blocked, leave the tree and record why. The safe fallback is retention, not aggressive deletion. A manual discarded-branch cleanup needs separate authorization and is intentionally outside this helper.
