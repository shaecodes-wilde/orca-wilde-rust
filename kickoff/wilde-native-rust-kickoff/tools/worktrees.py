#!/usr/bin/env python3
"""Conservative linked-worktree setup/closeout. Dry-run unless --apply is explicit."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys

ROLES = {"a01": "rust-v2/integration", "a02": "rust-v2/a02-runtime",
         "a03": "rust-v2/a03-domain", "a04": "rust-v2/a04-ui",
         "a05": "rust-v2/a05-ide", "a06": "rust-v2/a06-platform"}
SHA = re.compile(r"^(?:[0-9a-fA-F]{40}|[0-9a-fA-F]{64})$")


def git(repo: Path, *args: str, allow_failure: bool = False) -> subprocess.CompletedProcess:
    result = subprocess.run(["git", "-C", str(repo), *args], text=True,
                            capture_output=True, timeout=120, check=False, shell=False)
    if result.returncode and not allow_failure:
        raise ValueError(f"Git command failed ({args[0]}): {result.stderr.strip()}")
    return result


def repo_root(repo: Path) -> Path:
    return Path(git(repo.resolve(strict=True), "rev-parse", "--show-toplevel").stdout.strip()).resolve(strict=True)


def common_dir(repo: Path) -> Path:
    value = Path(git(repo, "rev-parse", "--git-common-dir").stdout.strip())
    return (value if value.is_absolute() else repo / value).resolve(strict=True)


def worktree_records(repo: Path) -> list[dict]:
    raw = git(repo, "worktree", "list", "--porcelain", "-z").stdout
    records, item = [], {}
    for field in raw.split("\0"):
        if not field:
            if item:
                records.append(item)
                item = {}
            continue
        key, _, value = field.partition(" ")
        item[key] = value
    if item:
        records.append(item)
    return records


def checked_root(repo: Path, root: Path) -> Path:
    root = root.resolve()
    for record in worktree_records(repo):
        registered = Path(record["worktree"]).resolve()
        if root == registered or registered in root.parents:
            raise ValueError("Worktree root must be outside every existing checkout.")
    if root.exists() and not root.is_dir():
        raise ValueError("Worktree root is not a directory.")
    return root


def exact_sha(repo: Path, value: str) -> str:
    if not SHA.fullmatch(value):
        raise ValueError("--base must be a full immutable commit SHA, not a branch or option.")
    return git(repo, "rev-parse", "--verify", f"{value}^{{commit}}").stdout.strip()


def plan_create(repo: Path, root: Path, base: str, roles: list[str]) -> list[dict]:
    repo = repo_root(repo)
    root, base = checked_root(repo, root), exact_sha(repo, base)
    if not roles or len(set(roles)) != len(roles) or any(role not in ROLES for role in roles):
        raise ValueError("Roles must be unique values from a01 through a06.")
    records, plan = worktree_records(repo), []
    for role in roles:
        target, branch = root / role, ROLES[role]
        if target.is_symlink():
            raise ValueError(f"Refusing symlink target: {target}")
        record = next((r for r in records if Path(r["worktree"]).resolve() == target), None)
        if record:
            if "locked" in record or "prunable" in record or not target.exists():
                raise ValueError(f"Existing worktree is locked, missing or prunable: {role}")
            if record.get("branch") != f"refs/heads/{branch}" or common_dir(target) != common_dir(repo):
                raise ValueError(f"Existing worktree identity does not match role: {role}")
            action = "reuse-without-reset"
        else:
            if target.exists():
                raise ValueError(f"Unregistered path already exists: {target}")
            exists = git(repo, "show-ref", "--verify", "--quiet", f"refs/heads/{branch}", allow_failure=True)
            if exists.returncode == 0:
                raise ValueError(f"Branch exists without expected worktree; reconcile manually: {branch}")
            if exists.returncode != 1:
                raise ValueError(f"Cannot determine branch existence: {branch}")
            action = "create"
        plan.append({"role": role, "branch": branch, "path": str(target), "base": base, "action": action})
    return plan


def create(repo: Path, root: Path, base: str, roles: list[str], apply: bool) -> dict:
    repo = repo_root(repo)
    plan, completed = plan_create(repo, root, base, roles), []
    if apply:
        parent = root.resolve()
        disk_parent = parent
        while not disk_parent.exists():
            disk_parent = disk_parent.parent
        if shutil.disk_usage(disk_parent).free < 2 * 1024**3:
            raise ValueError("Less than 2 GiB free; inspect needs before provisioning. Safety floor, not a size estimate.")
        parent.mkdir(parents=True, exist_ok=True)
        for item in plan:
            if item["action"] == "create":
                git(repo, "worktree", "add", "-b", item["branch"], item["path"], item["base"])
            completed.append(item["role"])
    return {"applied": apply, "plan": plan, "completed": completed,
            "note": "No models, dependencies or apps launched. No reset. Preserve created trees after any partial failure."}


def validate_closeout(repo: Path, root: Path, role: str, closeout_path: Path) -> dict:
    repo = repo_root(repo)
    root = checked_root(repo, root)
    if role not in ROLES or role == "a01":
        raise ValueError("Only worker roles a02–a06 may be removed by this helper.")
    target = root / role
    if target.is_symlink() or not target.is_dir():
        raise ValueError("Target must be an existing real directory, not a symlink.")
    cwd = Path.cwd().resolve()
    if target == cwd or target in cwd.parents:
        raise ValueError("Leave the target worktree before removal.")
    record = next((r for r in worktree_records(repo) if Path(r["worktree"]).resolve() == target), None)
    if not record or record.get("branch") != f"refs/heads/{ROLES[role]}":
        raise ValueError("Target is not the registered worktree/branch for this role.")
    if "locked" in record or "prunable" in record or common_dir(target) != common_dir(repo):
        raise ValueError("Target is locked/prunable or belongs to another repository.")
    data = json.loads(closeout_path.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or data.get("schema_version") != 1 or data.get("role") != role:
        raise ValueError("Closeout schema/role mismatch.")
    required = ("evidence_preserved", "process_ownership_checked", "task_processes_stopped", "unique_material_preserved")
    if any(data.get(key) is not True for key in required):
        raise ValueError("Closeout must attest actual evidence, process and unique-material checks.")
    evidence, reference = data.get("evidence_path"), data.get("durable_evidence_reference")
    if not isinstance(evidence, str) or not evidence or not isinstance(reference, str) or not reference.strip():
        raise ValueError("Preserved evidence path and durable reference are required.")
    evidence_path = Path(evidence).resolve(strict=True)
    if not evidence_path.is_file() or target == evidence_path or target in evidence_path.parents:
        raise ValueError("Evidence must exist as a file outside the removable worktree.")
    if target in closeout_path.resolve(strict=True).parents:
        raise ValueError("Closeout record must be preserved outside the worktree.")
    head = git(target, "rev-parse", "HEAD").stdout.strip()
    if data.get("reviewed_head") != head:
        raise ValueError("Target HEAD changed since review, or reviewed_head is absent.")
    integration_ref = data.get("integration_ref")
    if integration_ref != "refs/heads/rust-v2/integration":
        raise ValueError("Only the migration integration reference is accepted.")
    integrated = git(repo, "rev-parse", "--verify", f"{integration_ref}^{{commit}}").stdout.strip()
    if data.get("integration_sha") != integrated:
        raise ValueError("Integration HEAD changed or is not recorded; recheck closeout.")
    if git(repo, "merge-base", "--is-ancestor", head, integrated, allow_failure=True).returncode != 0:
        raise ValueError("Worker HEAD is not fully integrated by ancestry. Retain after squash/rebase and reconcile manually.")
    if git(target, "status", "--porcelain=v1", "--untracked-files=all", "--ignored=matching").stdout.strip():
        raise ValueError("Tracked, untracked or ignored material remains; preserve/reconcile before cleanup.")
    return {"role": role, "path": str(target), "reviewed_head": head,
            "integration_sha": integrated, "evidence_path": str(evidence_path),
            "process_check": "attested by closeout; not inferred by this helper"}


def remove(repo: Path, root: Path, role: str, closeout_path: Path, apply: bool) -> dict:
    info = validate_closeout(repo, root, role, closeout_path)
    if apply:
        info = validate_closeout(repo, root, role, closeout_path)  # Recheck immediately before mutation.
        git(repo_root(repo), "worktree", "remove", info["path"])
    return {"applied": apply, "closeout": info, "branch_deleted": False}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="action", required=True)
    add = sub.add_parser("create", help="Plan or create selected linked worktrees")
    delete = sub.add_parser("remove", help="Check closeout and optionally remove one worker")
    for command in (add, delete):
        command.add_argument("--repo", type=Path, required=True)
        command.add_argument("--root", type=Path, required=True)
        command.add_argument("--apply", action="store_true")
    add.add_argument("--base", required=True)
    add.add_argument("--roles", default="a01", help="Comma-separated role IDs")
    delete.add_argument("--role", choices=list(ROLES), required=True)
    delete.add_argument("--closeout", type=Path, required=True)
    args = parser.parse_args()
    try:
        result = create(args.repo, args.root, args.base, args.roles.split(","), args.apply) if args.action == "create" else remove(args.repo, args.root, args.role, args.closeout, args.apply)
        print(json.dumps(result, indent=2))
        return 0
    except (ValueError, OSError, subprocess.TimeoutExpired) as exc:
        print(f"worktrees: {exc}", file=sys.stderr)
        return 2

if __name__ == "__main__":
    raise SystemExit(main())
