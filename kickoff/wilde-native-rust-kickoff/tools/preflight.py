#!/usr/bin/env python3
"""Read-only local repository/tool inventory. No network, installs or app launches."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys
from datetime import datetime, timezone


def run(argv: list[str], cwd: Path | None = None) -> dict:
    try:
        result = subprocess.run(argv, cwd=cwd, capture_output=True, text=True,
                                timeout=30, check=False, shell=False)
        return {"exit_code": result.returncode, "stdout": result.stdout.strip(),
                "stderr": result.stderr.strip()}
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {"exit_code": None, "stdout": "", "stderr": str(exc)}


def collect(repo: Path) -> dict:
    repo = repo.resolve(strict=True)
    found = run(["git", "-C", str(repo), "rev-parse", "--show-toplevel"])
    if found["exit_code"] != 0:
        raise ValueError("Repository must be a non-bare Git working tree.")
    root = Path(found["stdout"]).resolve(strict=True)
    git = lambda *args: run(["git", "-C", str(root), *args])
    status = git("status", "--porcelain=v1", "--untracked-files=normal")
    scripts, package_error = {}, None
    package = root / "package.json"
    if package.is_file():
        try:
            parsed = json.loads(package.read_text(encoding="utf-8"))
            scripts = {key: value for key, value in parsed.get("scripts", {}).items()
                       if key.startswith("bench:") or key.startswith("test:perf")
                       or key.startswith("test:e2e:terminal-perf")}
        except (OSError, ValueError, TypeError, AttributeError) as exc:
            package_error = str(exc)
    tools = {}
    # Agent CLIs are located, not launched; they may load user configuration.
    for name in ("git", "rustc", "cargo", "node", "pnpm", "codex", "claude", "gh"):
        executable = shutil.which(name)
        item = {"path": executable, "version_check": "not-run"}
        if executable and name in ("git", "rustc", "cargo", "node"):
            item["version_check"] = run([executable, "--version"])
        tools[name] = item
    disk = shutil.disk_usage(root)
    return {
        "schema_version": 1, "collected_at_utc": datetime.now(timezone.utc).isoformat(),
        "repository_path": str(root), "head": git("rev-parse", "HEAD"),
        "branch": git("branch", "--show-current"),
        "origin_default_ref_if_locally_known": git("symbolic-ref", "--quiet", "refs/remotes/origin/HEAD"),
        "remote_names": git("remote"),  # Never capture raw remote URLs or environment values.
        "worktrees": git("worktree", "list", "--porcelain"),
        "dirty_status_available": status["exit_code"] == 0,
        "changed_or_untracked_entries": len(status["stdout"].splitlines()) if status["stdout"] else 0,
        "disk_bytes": {"total": disk.total, "used": disk.used, "free": disk.free}, "tools": tools,
        "benchmark_scripts_to_review_not_execute": scripts, "package_read_error": package_error,
        "instruction_paths_present": [p for p in ("AGENTS.md", "CLAUDE.md", "WILDE.md", "docs/STYLEGUIDE.md") if (root / p).is_file()],
        "existing_native_rust_directory": (root / "native-rust").exists(),
        "not_done": ["No network fetch", "No dependency installation", "No worktree mutation",
                     "No live app/profile inspection", "No model/provider verification", "No runtime benchmark"],
    }


def write_report(data: dict, path: Path | None) -> None:
    text = json.dumps(data, indent=2) + "\n"
    if path is None:
        print(text, end="")
    else:
        with path.open("x", encoding="utf-8") as handle:  # Refuse overwrite.
            handle.write(text)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    try:
        report = collect(args.repo)
        if args.output:
            output, root = args.output.resolve(), Path(report["repository_path"])
            if output == root or root in output.parents:
                raise ValueError("Save preflight outside the inspected checkout.")
        write_report(report, args.output)
        return 0
    except (ValueError, OSError) as exc:
        print(f"preflight: {exc}", file=sys.stderr)
        return 2

if __name__ == "__main__":
    raise SystemExit(main())
