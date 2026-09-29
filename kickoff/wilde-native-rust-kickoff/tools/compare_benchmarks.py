#!/usr/bin/env python3
"""Compare compatible measured benchmark sample sets; never collect or invent data."""
from __future__ import annotations
import argparse
import json
import math
from pathlib import Path
import re
import statistics
import sys

META = ("machine_id", "os", "environment_fingerprint", "build_mode", "workload_id",
        "fixture_sha256", "measurement_version", "memory_metric", "power_mode", "display_profile")


def validate_run(data: dict) -> None:
    if not isinstance(data, dict) or data.get("schema_version") != 1 or data.get("run_kind") != "measured":
        raise ValueError("Only schema v1 measured runs are accepted; templates are not results.")
    if not isinstance(data.get("commit"), str) or not re.fullmatch(r"[0-9a-fA-F]{40}|[0-9a-fA-F]{64}", data["commit"]):
        raise ValueError("A full measured commit SHA is required.")
    meta = data.get("metadata", {})
    if not isinstance(meta, dict) or any(not isinstance(meta.get(key), str) or not meta[key].strip() for key in META):
        raise ValueError("Complete nonempty measurement/environment metadata is required.")
    if meta["build_mode"] != "release":
        raise ValueError("This acceptance comparator requires release-mode runs.")
    if not re.fullmatch(r"[0-9a-fA-F]{64}", meta["fixture_sha256"]):
        raise ValueError("fixture_sha256 must identify the actual workload fixture.")
    metrics = data.get("metrics")
    if not isinstance(metrics, dict) or not metrics:
        raise ValueError("At least one measured metric is required.")
    for name, metric in metrics.items():
        if not isinstance(metric, dict) or metric.get("direction") not in ("lower", "higher"):
            raise ValueError(f"Invalid direction: {name}")
        if not isinstance(metric.get("unit"), str) or not metric["unit"]:
            raise ValueError(f"Missing unit: {name}")
        kind = metric.get("statistic")
        if kind not in ("median", "p95"):
            raise ValueError(f"Unsupported statistic: {name}")
        samples = metric.get("samples")
        minimum = 20 if kind == "p95" else 3
        if not isinstance(samples, list) or len(samples) < minimum:
            raise ValueError(f"{name} requires at least {minimum} real samples.")
        if any(type(value) not in (int, float) or not math.isfinite(value) or value < 0 for value in samples):
            raise ValueError(f"{name} samples must be finite, nonnegative numbers, not booleans.")


def summarize(metric: dict) -> float:
    values = sorted(metric["samples"])
    if metric["statistic"] == "median":
        return float(statistics.median(values))
    return float(values[math.ceil(0.95 * len(values)) - 1])


def compare(baseline: dict, candidate: dict, max_regression_pct: float | None = None) -> dict:
    validate_run(baseline)
    validate_run(candidate)
    if max_regression_pct is not None and (not math.isfinite(max_regression_pct) or max_regression_pct < 0):
        raise ValueError("Regression threshold must be finite and nonnegative.")
    mismatches = [key for key in META if baseline["metadata"][key] != candidate["metadata"][key]]
    if mismatches:
        raise ValueError("Incompatible measurement metadata: " + ", ".join(mismatches))
    if set(baseline["metrics"]) != set(candidate["metrics"]):
        raise ValueError("Both runs must contain the identical metric set.")
    rows = []
    for name in sorted(baseline["metrics"]):
        left, right = baseline["metrics"][name], candidate["metrics"][name]
        if any(left[key] != right[key] for key in ("unit", "direction", "statistic")):
            raise ValueError(f"Metric definition changed: {name}")
        old, new = summarize(left), summarize(right)
        sign = 1 if left["direction"] == "lower" else -1
        absolute_regression = sign * (new - old)
        regression = 100 * absolute_regression / old if old else (0.0 if new == old else None)
        passed = None
        if max_regression_pct is not None:
            passed = absolute_regression <= 0 if regression is None else regression <= max_regression_pct
        rows.append({"metric": name, "unit": left["unit"], "statistic": left["statistic"],
                     "baseline": old, "candidate": new, "baseline_samples": len(left["samples"]),
                     "candidate_samples": len(right["samples"]), "regression_pct": regression,
                     "threshold_pass": passed})
    overall = "informational" if max_regression_pct is None else ("threshold-pass" if all(row["threshold_pass"] for row in rows) else "threshold-fail")
    return {"schema_version": 1, "baseline_commit": baseline["commit"], "candidate_commit": candidate["commit"],
            "status": overall, "max_regression_pct": max_regression_pct, "metrics": rows,
            "limitations": "Mechanical scalar comparison only: not statistical significance, feature parity, causal proof, or whole-app acceptance. Metadata truth requires actual measurement review."}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("baseline", type=Path)
    parser.add_argument("candidate", type=Path)
    parser.add_argument("--max-regression-pct", type=float)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    try:
        result = compare(json.loads(args.baseline.read_text(encoding="utf-8")),
                         json.loads(args.candidate.read_text(encoding="utf-8")), args.max_regression_pct)
        text = json.dumps(result, indent=2, allow_nan=False) + "\n"
        if args.output:
            with args.output.open("x", encoding="utf-8") as handle:
                handle.write(text)
        else:
            print(text, end="")
        return 1 if result["status"] == "threshold-fail" else 0
    except (ValueError, OSError) as exc:
        print(f"benchmark comparison: {exc}", file=sys.stderr)
        return 2

if __name__ == "__main__":
    raise SystemExit(main())
