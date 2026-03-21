#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import math
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

import certify_claim_lane
import compare_runs


def main() -> int:
    args = parse_args()
    guarded_run = args.guarded_run.resolve()
    claim_runs = [claim_run.resolve() for claim_run in args.claim_run]
    if not claim_runs:
        raise SystemExit("provide at least one --claim-run directory")

    entries = [benchmark_entry(guarded_run, claim_run, args.runtime_threshold_ms) for claim_run in claim_runs]
    summary = compare_runs.ordered_dict(
        [
            ("benchmark_version", 1),
            ("generated_utc", datetime.now(timezone.utc).isoformat()),
            ("guarded_run", str(guarded_run)),
            ("runtime_threshold_ms", args.runtime_threshold_ms),
            ("aggregate", aggregate(entries)),
            ("runs", entries),
        ]
    )
    text = render_text_summary(summary)

    if args.json_out is not None:
        compare_runs.write_text(args.json_out, json.dumps(summary, indent=2, sort_keys=True) + "\n")
    if args.text_out is not None:
        compare_runs.write_text(args.text_out, text)

    sys.stdout.write(text)
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Aggregate stored desktop_claim_shadow runs into a repeatable benchmark "
            "summary with runtime, parity, breadth-floor, and manifest evidence."
        )
    )
    parser.add_argument(
        "--guarded-run",
        required=True,
        type=Path,
        help="guarded baseline run directory",
    )
    parser.add_argument(
        "--claim-run",
        action="append",
        default=[],
        type=Path,
        help="stored claim-lane run directory; repeat for multiple benchmark samples",
    )
    parser.add_argument(
        "--runtime-threshold-ms",
        type=int,
        help="optional certified runtime threshold to evaluate for every stored run",
    )
    parser.add_argument("--json-out", type=Path, help="optional JSON summary path")
    parser.add_argument("--text-out", type=Path, help="optional text summary path")
    return parser.parse_args()


def benchmark_entry(
    guarded_run: Path, claim_run: Path, runtime_threshold_ms: int | None
) -> dict[str, Any]:
    claim_lane = compare_runs.load_lane("claim", claim_run)
    guarded_lane = compare_runs.load_lane("guarded", guarded_run)
    summary = compare_runs.build_summary([guarded_lane, claim_lane], "guarded")
    claim_lane = next(lane for lane in summary["lanes"] if lane["label"] == "claim")
    claim_audit = claim_lane["claim_lane_audit"]
    manifest = compare_runs.load_json(claim_run / "run.json")
    steps = certify_claim_lane.load_step_reports(claim_run)

    parity = certify_claim_lane.accepted_hash_parity_check(claim_audit)
    early_breadth = certify_claim_lane.breadth_check(
        steps, certify_claim_lane.EARLY_GENERATED_TARGETS, exact=True
    )
    late_floors = certify_claim_lane.breadth_check(
        steps, certify_claim_lane.LATE_GENERATED_FLOORS, exact=False
    )
    runtime = certify_claim_lane.runtime_threshold_check(steps, runtime_threshold_ms)
    manifest_completeness = certify_claim_lane.manifest_completeness_check(manifest)
    total_runtime_ms = int(runtime["total_runtime_ms"])

    return compare_runs.ordered_dict(
        [
            ("claim_run", str(claim_run)),
            ("completed_step", int((manifest.get("position") or {}).get("completed_step", 0) or 0)),
            ("runtime_ms", total_runtime_ms),
            ("parity", parity),
            ("early_breadth", early_breadth),
            ("late_generated_floors", late_floors),
            ("runtime_threshold", runtime),
            ("manifest_completeness", manifest_completeness),
            (
                "manifest_snapshot",
                compare_runs.ordered_dict(
                    [
                        ("search_profile", claim_lane.get("search_profile")),
                        ("run_mode", manifest.get("run_mode")),
                        ("search_policy", manifest.get("search_policy")),
                        ("position", manifest.get("position")),
                        ("host", manifest.get("host")),
                        ("runtime", manifest.get("runtime")),
                        ("build", manifest.get("build")),
                    ]
                ),
            ),
            (
                "step_floor_hits",
                compare_runs.ordered_dict(
                    [
                        ("early", floor_hit_map(early_breadth)),
                        ("late", floor_hit_map(late_floors)),
                    ]
                ),
            ),
        ]
    )


def aggregate(entries: list[dict[str, Any]]) -> dict[str, Any]:
    runtimes = [int(entry["runtime_ms"]) for entry in entries]
    runtime_summary = compare_runs.ordered_dict(
        [
            ("samples", len(runtimes)),
            ("min_ms", min(runtimes)),
            ("median_ms", percentile(runtimes, 0.5)),
            ("p90_ms", percentile(runtimes, 0.9)),
            ("max_ms", max(runtimes)),
        ]
    )

    early_targets = sorted(certify_claim_lane.EARLY_GENERATED_TARGETS)
    late_targets = sorted(certify_claim_lane.LATE_GENERATED_FLOORS)
    early_floor_counts = {
        str(step_index): sum(
            1 for entry in entries if entry["step_floor_hits"]["early"].get(str(step_index), False)
        )
        for step_index in early_targets
    }
    late_floor_counts = {
        str(step_index): sum(
            1 for entry in entries if entry["step_floor_hits"]["late"].get(str(step_index), False)
        )
        for step_index in late_targets
    }

    return compare_runs.ordered_dict(
        [
            ("claim_run_count", len(entries)),
            ("completed_step_15_count", sum(1 for entry in entries if int(entry["completed_step"]) >= 15)),
            ("parity_success_count", sum(1 for entry in entries if entry["parity"]["status"] == "pass")),
            (
                "full_early_breadth_hit_count",
                sum(1 for entry in entries if entry["early_breadth"]["status"] == "pass"),
            ),
            (
                "full_late_floor_hit_count",
                sum(1 for entry in entries if entry["late_generated_floors"]["status"] == "pass"),
            ),
            (
                "runtime_threshold_pass_count",
                sum(1 for entry in entries if entry["runtime_threshold"]["status"] == "pass"),
            ),
            ("runtime_ms", runtime_summary),
            (
                "step_floor_hit_counts",
                compare_runs.ordered_dict(
                    [
                        ("early", compare_runs.ordered_dict((str(key), early_floor_counts[str(key)]) for key in early_targets)),
                        ("late", compare_runs.ordered_dict((str(key), late_floor_counts[str(key)]) for key in late_targets)),
                    ]
                ),
            ),
        ]
    )


def floor_hit_map(result: dict[str, Any]) -> dict[str, bool]:
    hits: dict[str, bool] = {}
    for step in result.get("steps", []):
        hits[str(int(step["step_index"]))] = step["status"] == "hit"
    return hits


def percentile(values: list[int], fraction: float) -> int:
    ordered = sorted(values)
    rank = max(1, math.ceil(len(ordered) * fraction)) - 1
    return int(ordered[rank])


def render_text_summary(summary: dict[str, Any]) -> str:
    aggregate_summary = summary["aggregate"]
    runtime = aggregate_summary["runtime_ms"]
    lines = [
        "Claim Benchmark Summary",
        f"guarded run: {summary['guarded_run']}",
        f"claim run count: {aggregate_summary['claim_run_count']}",
        (
            "runtime ms: "
            f"min={runtime['min_ms']} median={runtime['median_ms']} "
            f"p90={runtime['p90_ms']} max={runtime['max_ms']}"
        ),
        f"completed step-15 count: {aggregate_summary['completed_step_15_count']}",
        f"parity success count: {aggregate_summary['parity_success_count']}",
        f"full early breadth hit count: {aggregate_summary['full_early_breadth_hit_count']}",
        f"full late floor hit count: {aggregate_summary['full_late_floor_hit_count']}",
    ]
    if summary["runtime_threshold_ms"] is not None:
        lines.append(
            "runtime threshold pass count: "
            f"{aggregate_summary['runtime_threshold_pass_count']}"
        )
    lines.append("per-run details:")
    for entry in summary["runs"]:
        lines.append(
            "  - "
            f"{entry['claim_run']}: completed_step={entry['completed_step']} runtime_ms={entry['runtime_ms']} "
            f"parity={entry['parity']['status']} early={entry['early_breadth']['status']} "
            f"late={entry['late_generated_floors']['status']} manifest={entry['manifest_completeness']['status']}"
        )
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())
