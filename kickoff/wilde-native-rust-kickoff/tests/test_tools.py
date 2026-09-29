"""Helper tests use disposable repositories only, never the user's checkout."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
import preflight
import worktrees
import compare_benchmarks as bench


def command(repo, *args):
    return subprocess.run(["git", "-C", str(repo), *args], check=True, text=True,
                          capture_output=True, timeout=30).stdout.strip()


class GitFixture(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="wilde-kickoff-test-")
        self.home = Path(self.temp.name).resolve()
        self.repo = self.home / "repo"
        self.repo.mkdir()
        command(self.repo, "init", "-b", "main")
        command(self.repo, "config", "user.email", "fixture@example.invalid")
        command(self.repo, "config", "user.name", "Disposable Test Fixture")
        command(self.repo, "config", "commit.gpgsign", "false")
        (self.repo / "README.md").write_text("Fixture only\n")
        (self.repo / ".gitignore").write_text("secret.local\n")
        (self.repo / "package.json").write_text(json.dumps({"scripts": {"bench:test": "echo fixture", "start": "do-not-run"}}))
        command(self.repo, "add", ".")
        command(self.repo, "commit", "-m", "fixture base")
        self.sha = command(self.repo, "rev-parse", "HEAD")
        self.workroot = self.home / "wr"

    def tearDown(self):
        self.temp.cleanup()

    def prepare(self):
        worktrees.create(self.repo, self.workroot, self.sha, ["a01", "a02"], True)

    def closeout(self):
        evidence = self.home / "review.txt"
        evidence.write_text("Synthetic test evidence, not app evidence.\n")
        data = {"schema_version": 1, "role": "a02",
                "reviewed_head": command(self.workroot / "a02", "rev-parse", "HEAD"),
                "integration_ref": "refs/heads/rust-v2/integration",
                "integration_sha": command(self.repo, "rev-parse", "refs/heads/rust-v2/integration"),
                "evidence_path": str(evidence), "durable_evidence_reference": "disposable fixture review",
                "evidence_preserved": True, "process_ownership_checked": True,
                "task_processes_stopped": True, "unique_material_preserved": True}
        path = self.home / "closeout.json"
        path.write_text(json.dumps(data))
        return path

    def test_preflight_is_read_only(self):
        before = command(self.repo, "status", "--porcelain")
        result = preflight.collect(self.repo)
        self.assertEqual(result["head"]["stdout"], self.sha)
        self.assertEqual(list(result["benchmark_scripts_to_review_not_execute"]), ["bench:test"])
        self.assertEqual(before, command(self.repo, "status", "--porcelain"))

    def test_preflight_output_refuses_overwrite(self):
        path = self.home / "report.json"
        preflight.write_report({"fixture": True}, path)
        with self.assertRaises(FileExistsError):
            preflight.write_report({"fixture": False}, path)

    def test_create_defaults_to_plan(self):
        result = worktrees.create(self.repo, self.workroot, self.sha, ["a01", "a02"], False)
        self.assertFalse(result["applied"])
        self.assertFalse(self.workroot.exists())
        self.assertEqual(len(worktrees.worktree_records(self.repo)), 1)

    def test_create_reuses_without_reset(self):
        self.prepare()
        (self.workroot / "a02" / "local.txt").write_text("preserve me")
        result = worktrees.create(self.repo, self.workroot, self.sha, ["a02"], True)
        self.assertEqual(result["plan"][0]["action"], "reuse-without-reset")
        self.assertTrue((self.workroot / "a02" / "local.txt").exists())

    def test_nested_root_rejected(self):
        with self.assertRaises(ValueError):
            worktrees.create(self.repo, self.repo / "wr", self.sha, ["a01"], False)

    def test_branch_not_sha_rejected(self):
        with self.assertRaises(ValueError):
            worktrees.create(self.repo, self.workroot, "main", ["a01"], False)

    def test_unknown_duplicate_roles_rejected(self):
        for roles in (["a09"], ["a02", "a02"]):
            with self.assertRaises(ValueError):
                worktrees.create(self.repo, self.workroot, self.sha, roles, False)

    def test_unregistered_existing_directory_rejected(self):
        (self.workroot / "a02").mkdir(parents=True)
        with self.assertRaises(ValueError):
            worktrees.create(self.repo, self.workroot, self.sha, ["a02"], True)

    def test_clean_integrated_remove_and_branch_retained(self):
        self.prepare()
        path = self.closeout()
        result = worktrees.remove(self.repo, self.workroot, "a02", path, False)
        self.assertFalse(result["applied"])
        self.assertTrue((self.workroot / "a02").exists())
        worktrees.remove(self.repo, self.workroot, "a02", path, True)
        self.assertFalse((self.workroot / "a02").exists())
        self.assertEqual(command(self.repo, "rev-parse", "rust-v2/a02-runtime"), self.sha)
        self.assertTrue((self.repo / "README.md").exists())

    def test_untracked_file_blocks_removal(self):
        self.prepare()
        path = self.closeout()
        (self.workroot / "a02" / "local.txt").write_text("unique")
        with self.assertRaises(ValueError):
            worktrees.remove(self.repo, self.workroot, "a02", path, True)
        self.assertTrue((self.workroot / "a02" / "local.txt").exists())

    def test_ignored_file_blocks_removal(self):
        self.prepare()
        path = self.closeout()
        (self.workroot / "a02" / "secret.local").write_text("fixture, not a secret")
        with self.assertRaises(ValueError):
            worktrees.remove(self.repo, self.workroot, "a02", path, True)
        self.assertTrue((self.workroot / "a02" / "secret.local").exists())

    def test_unmerged_commit_blocks_removal(self):
        self.prepare()
        worker = self.workroot / "a02"
        (worker / "new.txt").write_text("new work")
        command(worker, "add", ".")
        command(worker, "commit", "-m", "not integrated")
        with self.assertRaises(ValueError):
            worktrees.remove(self.repo, self.workroot, "a02", self.closeout(), True)
        self.assertTrue(worker.exists())

    def test_changed_head_blocks_removal(self):
        self.prepare()
        path = self.closeout()
        data = json.loads(path.read_text())
        data["reviewed_head"] = "0" * 40
        path.write_text(json.dumps(data))
        with self.assertRaises(ValueError):
            worktrees.remove(self.repo, self.workroot, "a02", path, True)

    def test_missing_process_check_blocks_removal(self):
        self.prepare()
        path = self.closeout()
        data = json.loads(path.read_text())
        data["task_processes_stopped"] = False
        path.write_text(json.dumps(data))
        with self.assertRaises(ValueError):
            worktrees.remove(self.repo, self.workroot, "a02", path, True)

    def test_locked_worker_blocks_removal(self):
        self.prepare()
        path = self.closeout()
        command(self.repo, "worktree", "lock", str(self.workroot / "a02"))
        with self.assertRaises(ValueError):
            worktrees.remove(self.repo, self.workroot, "a02", path, True)

    def test_integration_role_not_removed(self):
        self.prepare()
        with self.assertRaises(ValueError):
            worktrees.remove(self.repo, self.workroot, "a01", self.closeout(), True)


class BenchmarkTests(unittest.TestCase):
    def fixture(self, samples=None):
        return {"schema_version": 1, "run_kind": "measured", "commit": "1" * 40,
                "metadata": {**{key: "test-only" for key in bench.META}, "build_mode": "release", "fixture_sha256": "a" * 64},
                "metrics": {"fixture_ms": {"unit": "ms", "direction": "lower", "statistic": "median", "samples": samples or [100, 100, 100]}}}

    def test_improvement_is_informational_without_gate(self):
        report = bench.compare(self.fixture(), self.fixture([80, 80, 80]))
        self.assertEqual(report["status"], "informational")
        self.assertEqual(report["metrics"][0]["regression_pct"], -20)

    def test_explicit_threshold(self):
        self.assertEqual(bench.compare(self.fixture(), self.fixture([110, 110, 110]), 5)["status"], "threshold-fail")
        self.assertEqual(bench.compare(self.fixture(), self.fixture([102, 102, 102]), 5)["status"], "threshold-pass")

    def test_template_rejected(self):
        data = self.fixture(); data["run_kind"] = "template"
        with self.assertRaises(ValueError):
            bench.compare(data, self.fixture())

    def test_metadata_mismatch_rejected(self):
        data = self.fixture(); data["metadata"]["machine_id"] = "other machine"
        with self.assertRaises(ValueError):
            bench.compare(self.fixture(), data)

    def test_metric_sets_must_match(self):
        data = self.fixture(); data["metrics"]["other"] = copy.deepcopy(data["metrics"]["fixture_ms"])
        with self.assertRaises(ValueError):
            bench.compare(self.fixture(), data)

    def test_nonfinite_boolean_and_short_samples_rejected(self):
        for samples in ([float("nan"), 1, 1], [float("inf"), 1, 1], [True, 1, 1], [1, 2], [-1, 1, 1]):
            with self.assertRaises(ValueError):
                bench.compare(self.fixture(), self.fixture(samples))

    def test_p95_requires_twenty_samples(self):
        data = self.fixture(); data["metrics"]["fixture_ms"]["statistic"] = "p95"
        with self.assertRaises(ValueError):
            bench.validate_run(data)
        data["metrics"]["fixture_ms"]["samples"] = list(range(1, 21))
        bench.validate_run(data)
        self.assertEqual(bench.summarize(data["metrics"]["fixture_ms"]), 19)

    def test_zero_baseline_no_infinite_percentage(self):
        report = bench.compare(self.fixture([0, 0, 0]), self.fixture([1, 1, 1]), 5)
        self.assertIsNone(report["metrics"][0]["regression_pct"])
        self.assertEqual(report["status"], "threshold-fail")

    def test_higher_is_better(self):
        a, b = self.fixture(), self.fixture([120, 120, 120])
        a["metrics"]["fixture_ms"]["direction"] = b["metrics"]["fixture_ms"]["direction"] = "higher"
        self.assertEqual(bench.compare(a, b, 5)["status"], "threshold-pass")

    def test_definition_change_rejected(self):
        data = self.fixture(); data["metrics"]["fixture_ms"]["unit"] = "seconds"
        with self.assertRaises(ValueError):
            bench.compare(self.fixture(), data)

    def test_debug_rejected(self):
        data = self.fixture(); data["metadata"]["build_mode"] = "debug"
        with self.assertRaises(ValueError):
            bench.validate_run(data)


class PackageTests(unittest.TestCase):
    def test_backlog_is_acyclic_and_roles_match(self):
        roles = json.loads((ROOT / "coordination/roles.json").read_text())
        self.assertEqual({item["id"]: item["branch"] for item in roles["roles"]}, worktrees.ROLES)
        tasks = json.loads((ROOT / "coordination/backlog.json").read_text())["tasks"]
        todo = {item["id"]: item for item in tasks}
        self.assertEqual(len(todo), len(tasks))
        for item in tasks:
            self.assertIn(item["owner"], worktrees.ROLES)
            self.assertTrue(all(dep in todo for dep in item["depends_on"]))
        done = set()
        while todo:
            ready = [key for key, item in todo.items() if set(item["depends_on"]) <= done]
            self.assertTrue(ready, "Task graph contains a cycle")
            for key in ready:
                done.add(key); del todo[key]

    def test_templates_are_not_execution_approvals(self):
        closeout = json.loads((ROOT / "templates/closeout.json").read_text())
        self.assertFalse(closeout["task_processes_stopped"])
        data = json.loads((ROOT / "templates/benchmark-run.json").read_text())
        with self.assertRaises(ValueError):
            bench.validate_run(data)

if __name__ == "__main__":
    unittest.main()
