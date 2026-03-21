#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

import compare_runs

EARLY_GENERATED_TARGETS = {1: 2144}
LATE_GENERATED_FLOORS = {
    10: 500,
    11: 800,
    12: 1200,
    13: 2200,
    14: 3500,
    15: 5000,
}
MANIFEST_FIELD_SPECS = (
    ("os", (("host", "os"),), "non_empty"),
    ("arch", (("host", "arch"),), "non_empty"),
    ("logical_cpus", (("host", "logical_cpus"),), "positive_int"),
    ("ram_bytes", (("host", "ram_bytes"),), "positive_int"),
    ("cpu_model", (("host", "cpu_model"), ("provenance", "cpu_model")), "non_empty"),
    (
        "physical_core_count",
        (("host", "physical_core_count"), ("provenance", "physical_core_count")),
        "positive_int",
    ),
    (
        "resolved_worker_count",
        (
            ("host", "resolved_worker_count"),
            ("runtime", "resolved_worker_count"),
            ("provenance", "resolved_worker_count"),
        ),
        "positive_int",
    ),
    (
        "build_profile",
        (("build", "profile"), ("provenance", "build_profile")),
        "non_empty",
    ),
    (
        "target_triple",
        (("build", "target_triple"), ("provenance", "target_triple")),
        "non_empty",
    ),
    (
        "target_cpu",
        (("build", "target_cpu"), ("provenance", "target_cpu")),
        "non_empty",
    ),
    (
        "git_commit_sha",
        (("build", "git_commit_sha"), ("provenance", "git_commit_sha")),
        "non_empty",
    ),
    (
        "dirty_tree",
        (("build", "dirty_tree"), ("provenance", "dirty_tree")),
        "bool",
    ),
    (
        "cargo_lock_sha256",
        (("build", "cargo_lock_sha256"), ("provenance", "cargo_lock_sha256")),
        "non_empty",
    ),
    (
        "binary_sha256",
        (("build", "binary_sha256"), ("provenance", "binary_sha256")),
        "non_empty",
    ),
)


def main() -> int:
    args = parse_args()
    claim_run = args.claim_run.resolve()
    guarded_run = args.guarded_run.resolve()

    claim_lane = compare_runs.load_lane("claim", claim_run)
    guarded_lane = compare_runs.load_lane("guarded", guarded_run)
    summary = compare_runs.build_summary([guarded_lane, claim_lane], "guarded")
    claim_lane = next(lane for lane in summary["lanes"] if lane["label"] == "claim")
    claim_audit = claim_lane["claim_lane_audit"]
    manifest = compare_runs.load_json(claim_run / "run.json")
    steps = load_step_reports(claim_run)

    checks = compare_runs.ordered_dict(
        [
            ("accepted_hash_parity", accepted_hash_parity_check(claim_audit)),
            ("search_policy", search_policy_check(claim_audit)),
            ("fallback_honesty", fallback_honesty_check(claim_audit)),
            ("narrative_artifacts", narrative_artifact_check(claim_lane)),
            ("early_breadth", breadth_check(steps, EARLY_GENERATED_TARGETS, exact=True)),
            ("late_generated_floors", breadth_check(steps, LATE_GENERATED_FLOORS, exact=False)),
            (
                "runtime_threshold",
                runtime_threshold_check(steps, args.runtime_threshold_ms),
            ),
            (
                "exact_screen_reason_completeness",
                completeness_check(
                    claim_audit["exact_screen_reasons"],
                    "exact-screen reasons are fully persisted in step artifacts",
                ),
            ),
            (
                "prune_class_completeness",
                completeness_check(
                    claim_audit["prune_classes"],
                    "prune class counts are fully persisted in step artifacts",
                ),
            ),
            ("manifest_completeness", manifest_completeness_check(manifest)),
        ]
    )

    failing_checks = [
        name for name, check in checks.items() if str(check.get("status", "fail")) != "pass"
    ]
    status = "pass" if not failing_checks else "attention"
    certificate = compare_runs.ordered_dict(
        [
            ("certificate_version", 1),
            ("generated_utc", datetime.now(timezone.utc).isoformat()),
            ("status", status),
            ("claim_run", str(claim_run)),
            ("guarded_run", str(guarded_run)),
            ("runtime_threshold_ms", args.runtime_threshold_ms),
            ("failing_checks", failing_checks),
            ("checks", checks),
        ]
    )
    text = render_text_summary(certificate)

    if args.json_out is not None:
        compare_runs.write_text(
            args.json_out, json.dumps(certificate, indent=2, sort_keys=True) + "\n"
        )
    if args.text_out is not None:
        compare_runs.write_text(args.text_out, text)

    sys.stdout.write(text)
    return 0 if status == "pass" else 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Audit a stored desktop_claim_shadow run against the guarded baseline and "
            "emit a claim-lane certification summary."
        )
    )
    parser.add_argument("--claim-run", required=True, type=Path, help="claim-lane run directory")
    parser.add_argument(
        "--guarded-run",
        required=True,
        type=Path,
        help="guarded baseline run directory",
    )
    parser.add_argument(
        "--runtime-threshold-ms",
        type=int,
        help="required upper bound on total stored step wall-clock time in milliseconds",
    )
    parser.add_argument("--json-out", type=Path, help="optional JSON certificate path")
    parser.add_argument("--text-out", type=Path, help="optional text summary path")
    return parser.parse_args()


def load_step_reports(run_dir: Path) -> list[dict[str, Any]]:
    step_dir = run_dir / "reports" / "steps"
    step_paths = sorted(step_dir.glob("step-*-summary.json"))
    return [compare_runs.load_json(path) for path in step_paths]


def accepted_hash_parity_check(claim_audit: dict[str, Any]) -> dict[str, Any]:
    parity = claim_audit["accepted_hash_parity"]
    passed = parity["status"] == "ready"
    detail = "accepted hashes match guarded through step 15"
    if parity["lane_missing_steps"]:
        detail = (
            "claim lane is missing "
            + compare_runs.render_step_list(parity["lane_missing_steps"])
        )
    elif parity["baseline_missing_steps"]:
        detail = (
            "guarded baseline is missing "
            + compare_runs.render_step_list(parity["baseline_missing_steps"])
        )
    elif not parity["matches_baseline"]:
        detail = "claim lane diverges from the guarded accepted hashes"
    return compare_runs.ordered_dict(
        [
            ("status", "pass" if passed else "fail"),
            ("detail", detail),
            ("matches_baseline", bool(parity["matches_baseline"])),
            ("lane_missing_steps", parity["lane_missing_steps"]),
            ("baseline_missing_steps", parity["baseline_missing_steps"]),
        ]
    )


def search_policy_check(claim_audit: dict[str, Any]) -> dict[str, Any]:
    policy = claim_audit["search_policy"]
    return compare_runs.ordered_dict(
        [
            ("status", "pass" if policy["status"] == "honest" else "fail"),
            (
                "detail",
                "claim search policy matches the expected claim-specific metadata"
                if policy["status"] == "honest"
                else "claim search policy metadata does not match the expected claim lane",
            ),
            ("expected", policy["expected"]),
            ("actual", policy["actual"]),
            ("mismatches", policy["mismatches"]),
        ]
    )


def fallback_honesty_check(claim_audit: dict[str, Any]) -> dict[str, Any]:
    fallback = claim_audit["fallback_honesty"]
    passed = fallback["status"] == "clear"
    detail = "no replay or non-claim fallback evidence detected"
    if fallback["run_mode_fallback"]:
        detail = f"run mode {fallback['run_mode']} is not certification-safe"
    elif fallback["reference_replay_steps"]:
        detail = (
            "reference replay provenance appears at "
            + compare_runs.render_step_list(fallback["reference_replay_steps"])
        )
    elif fallback["unexpected_provenance"]:
        detail = "unexpected provenance labels were found in the claim run"
    return compare_runs.ordered_dict(
        [
            ("status", "pass" if passed else "fail"),
            ("detail", detail),
            ("run_mode", fallback["run_mode"]),
            ("resume_steps", fallback["resume_steps"]),
            ("reference_replay_steps", fallback["reference_replay_steps"]),
            ("unexpected_provenance", fallback["unexpected_provenance"]),
        ]
    )


def narrative_artifact_check(claim_lane: dict[str, Any]) -> dict[str, Any]:
    narrative = claim_lane["narrative_artifacts"]
    passed = narrative["status"] == "complete"
    detail = "claim narrative artifacts are complete"
    if narrative["status"] != "complete":
        parts = []
        if narrative["missing_narrative_steps"]:
            parts.append(
                "text="
                + compare_runs.render_step_list(narrative["missing_narrative_steps"])
            )
        if narrative["missing_event_steps"]:
            parts.append(
                "events=" + compare_runs.render_step_list(narrative["missing_event_steps"])
            )
        detail = "claim narrative artifacts are incomplete (" + ", ".join(parts) + ")"
    return compare_runs.ordered_dict(
        [
            ("status", "pass" if passed else "fail"),
            ("detail", detail),
            ("artifact_status", narrative["status"]),
            ("missing_narrative_steps", narrative["missing_narrative_steps"]),
            ("missing_event_steps", narrative["missing_event_steps"]),
        ]
    )


def breadth_check(
    steps: list[dict[str, Any]], targets: dict[int, int], exact: bool
) -> dict[str, Any]:
    step_map = {int(step.get("step_index", 0) or 0): step for step in steps}
    results = []
    failing_steps = []

    for step_index, target in targets.items():
        step = step_map.get(step_index)
        if step is None:
            actual = None
            hit = False
            reason = "missing"
        else:
            actual = generated_count(step)
            hit = actual == target if exact else actual >= target
            reason = "hit" if hit else "miss"
        if not hit:
            failing_steps.append(step_index)
        results.append(
            compare_runs.ordered_dict(
                [
                    ("step_index", step_index),
                    ("target", target),
                    ("actual", actual),
                    ("status", reason),
                ]
            )
        )

    label = "early breadth targets are satisfied" if exact else "late generated floors are satisfied"
    if failing_steps:
        label = (
            ("early breadth failed at " if exact else "late generated floors failed at ")
            + compare_runs.render_step_list(failing_steps)
        )

    return compare_runs.ordered_dict(
        [
            ("status", "pass" if not failing_steps else "fail"),
            ("detail", label),
            ("steps", results),
        ]
    )


def generated_count(step: dict[str, Any]) -> int:
    search_stats = step.get("search_stats") or {}
    demo_funnel = search_stats.get("demo_funnel") or {}
    return int(demo_funnel.get("generated_raw_prefixes", 0) or 0)


def runtime_threshold_check(
    steps: list[dict[str, Any]], runtime_threshold_ms: int | None
) -> dict[str, Any]:
    total_runtime_ms = sum(
        int(
            ((step.get("search_stats") or {}).get("search_timing") or {}).get(
                "step_wall_clock_millis",
                0,
            )
            or 0
        )
        for step in steps
    )
    if runtime_threshold_ms is None:
        return compare_runs.ordered_dict(
            [
                ("status", "fail"),
                ("detail", "no certified runtime threshold was provided"),
                ("total_runtime_ms", total_runtime_ms),
                ("threshold_ms", None),
            ]
        )

    passed = total_runtime_ms <= runtime_threshold_ms
    detail = (
        f"stored runtime {total_runtime_ms} ms is within the certified threshold"
        if passed
        else f"stored runtime {total_runtime_ms} ms exceeds the certified threshold"
    )
    return compare_runs.ordered_dict(
        [
            ("status", "pass" if passed else "fail"),
            ("detail", detail),
            ("total_runtime_ms", total_runtime_ms),
            ("threshold_ms", runtime_threshold_ms),
        ]
    )


def completeness_check(coverage: dict[str, Any], detail: str) -> dict[str, Any]:
    passed = coverage["status"] == "complete"
    if not passed:
        if coverage["missing_steps"]:
            detail = (
                detail
                + "; missing "
                + compare_runs.render_step_list(coverage["missing_steps"])
            )
        elif coverage["derived_steps"]:
            detail = (
                detail
                + "; derived instead of persisted on "
                + compare_runs.render_step_list(coverage["derived_steps"])
            )
        else:
            detail = detail + "; coverage is incomplete"
    return compare_runs.ordered_dict(
        [
            ("status", "pass" if passed else "fail"),
            ("detail", detail),
            ("coverage", coverage),
        ]
    )


def manifest_completeness_check(manifest: dict[str, Any]) -> dict[str, Any]:
    present_fields = {}
    missing_fields = []

    for label, candidate_paths, validator in MANIFEST_FIELD_SPECS:
        resolved = resolve_manifest_field(manifest, candidate_paths, validator)
        if resolved is None:
            missing_fields.append(label)
        else:
            present_fields[label] = resolved

    detail = "manifest includes all required provenance fields"
    if missing_fields:
        detail = "manifest is missing " + ", ".join(missing_fields)

    return compare_runs.ordered_dict(
        [
            ("status", "pass" if not missing_fields else "fail"),
            ("detail", detail),
            ("present_fields", compare_runs.ordered_dict(sorted(present_fields.items()))),
            ("missing_fields", missing_fields),
        ]
    )


def resolve_manifest_field(
    manifest: dict[str, Any], candidate_paths: tuple[tuple[str, ...], ...], validator: str
) -> Any | None:
    for path in candidate_paths:
        value = nested_get(manifest, path)
        if manifest_value_is_valid(value, validator):
            return value
    return None


def nested_get(value: Any, path: tuple[str, ...]) -> Any:
    current = value
    for key in path:
        if not isinstance(current, dict) or key not in current:
            return None
        current = current[key]
    return current


def manifest_value_is_valid(value: Any, validator: str) -> bool:
    if validator == "non_empty":
        return value is not None and str(value).strip() != ""
    if validator == "positive_int":
        try:
            return int(value) > 0
        except (TypeError, ValueError):
            return False
    if validator == "bool":
        return isinstance(value, bool)
    return value is not None


def render_text_summary(certificate: dict[str, Any]) -> str:
    checks = certificate["checks"]
    lines = [
        f"Claim Certification: {certificate['status']}",
        f"claim run: {certificate['claim_run']}",
        f"guarded run: {certificate['guarded_run']}",
    ]
    if certificate["runtime_threshold_ms"] is not None:
        lines.append(f"runtime threshold ms: {certificate['runtime_threshold_ms']}")
    for label, check in checks.items():
        lines.append(f"{label}: {check['status']} - {check['detail']}")
    if certificate["failing_checks"]:
        lines.append("failing checks: " + ", ".join(certificate["failing_checks"]))
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())
