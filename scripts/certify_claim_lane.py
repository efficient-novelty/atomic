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
CLAIM_REGULAR_CATALOG_NOTE = "claim_regular_clause_catalog"
CLAIM_ROOT_SEEDING_NOTE = "claim_root_seeding_summary"


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
            (
                "early_breadth",
                breadth_check(claim_run, steps, EARLY_GENERATED_TARGETS, exact=True),
            ),
            (
                "late_generated_floors",
                breadth_check(claim_run, steps, LATE_GENERATED_FLOORS, exact=False),
            ),
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
    run_dir: Path, steps: list[dict[str, Any]], targets: dict[int, int], exact: bool
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
            diagnosis = compare_runs.ordered_dict(
                [
                    ("source", "missing_step_summary"),
                    ("summary", f"actual=missing target={target}"),
                ]
            )
        else:
            actual = generated_count(step)
            hit = actual == target if exact else actual >= target
            reason = "hit" if hit else "miss"
            diagnosis = breadth_diagnosis(run_dir, step, target, actual)
        if not hit:
            failing_steps.append(step_index)
        result = compare_runs.ordered_dict(
            [
                ("step_index", step_index),
                ("target", target),
                ("actual", actual),
                ("status", reason),
                (
                    "gap_to_target",
                    None if actual is None else int(target) - int(actual),
                ),
            ]
        )
        result["diagnosis"] = diagnosis
        results.append(result)

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


def breadth_diagnosis(
    run_dir: Path, step: dict[str, Any], target: int, actual: int
) -> dict[str, Any]:
    step_index = int(step.get("step_index", 0) or 0)
    stats = step.get("search_stats") or {}
    funnel = stats.get("demo_funnel") or {}
    checkpoints = load_step_live_checkpoints(run_dir, step_index)
    catalog_checkpoint = checkpoint_with_note(checkpoints, CLAIM_REGULAR_CATALOG_NOTE)
    root_checkpoint = checkpoint_with_note(checkpoints, CLAIM_ROOT_SEEDING_NOTE)
    best_checkpoint = root_checkpoint or catalog_checkpoint

    exact_screen_reasons = normalize_exact_screen_reasons(
        stats.get("exact_screen_reasons")
    )
    claim_root_seeding = normalize_claim_root_seeding(
        (root_checkpoint or {}).get("claim_root_seeding") or stats.get("claim_root_seeding")
    )
    raw_catalog_clause_widths = normalize_int_list(
        (best_checkpoint or {}).get("raw_catalog_clause_widths")
    )
    raw_catalog_telescope_count = optional_int(
        (best_checkpoint or {}).get("raw_catalog_telescope_count")
    )
    claim_step_open = normalize_claim_step_open(
        (best_checkpoint or {}).get("claim_step_open") or stats.get("claim_step_open")
    )
    search_space = compare_runs.ordered_dict(
        [
            ("enumerated_candidates", int(stats.get("enumerated_candidates", 0) or 0)),
            ("well_formed_candidates", int(stats.get("well_formed_candidates", 0) or 0)),
            (
                "admissibility_rejections",
                int(stats.get("admissibility_rejections", 0) or 0),
            ),
            (
                "exact_bound_screened",
                int(funnel.get("exact_bound_screened", 0) or 0),
            ),
            (
                "full_telescopes_evaluated",
                int(stats.get("full_telescopes_evaluated", 0) or 0),
            ),
            ("heuristic_dropped", int(funnel.get("heuristic_dropped", 0) or 0)),
            ("candidate_heuristic_drops", int(stats.get("heuristic_drops", 0) or 0)),
            ("retained_candidates", int(stats.get("retained_candidates", 0) or 0)),
        ]
    )

    return compare_runs.ordered_dict(
        [
            (
                "source",
                "summary_plus_live_checkpoints"
                if best_checkpoint is not None
                else "summary_only",
            ),
            (
                "summary",
                breadth_diagnosis_summary(
                    target,
                    actual,
                    raw_catalog_telescope_count,
                    raw_catalog_clause_widths,
                    claim_root_seeding,
                    claim_step_open,
                    search_space,
                    exact_screen_reasons,
                ),
            ),
            ("raw_catalog_telescope_count", raw_catalog_telescope_count),
            ("raw_catalog_clause_widths", raw_catalog_clause_widths),
            ("claim_root_seeding", claim_root_seeding),
            ("claim_step_open", claim_step_open),
            ("search_space", search_space),
            ("exact_screen_reasons", exact_screen_reasons),
        ]
    )


def breadth_diagnosis_summary(
    target: int,
    actual: int,
    raw_catalog_telescope_count: int | None,
    raw_catalog_clause_widths: list[int],
    claim_root_seeding: dict[str, Any],
    claim_step_open: dict[str, Any],
    search_space: dict[str, Any],
    exact_screen_reasons: dict[str, int],
) -> str:
    parts = [
        f"actual={actual}",
        f"target={target}",
        f"gap={int(target) - int(actual)}",
    ]
    if raw_catalog_telescope_count is not None:
        parts.append(f"catalog={raw_catalog_telescope_count}")
    if raw_catalog_clause_widths:
        parts.append(f"widths={raw_catalog_clause_widths}")
    if claim_root_seeding:
        parts.append(
            "roots="
            f"seen{claim_root_seeding['roots_seen']} "
            f"enqueued{claim_root_seeding['roots_enqueued']} "
            f"exact_rejected{claim_root_seeding['roots_rejected_by_exact_screen']}"
        )
    claim_step_open_summary = summarize_claim_step_open(claim_step_open)
    if claim_step_open_summary:
        parts.append(claim_step_open_summary)
    parts.append(f"well_formed={search_space['well_formed_candidates']}")
    parts.append(f"exact_screened={search_space['exact_bound_screened']}")
    parts.append(f"retained={search_space['retained_candidates']}")
    parts.append(f"heuristic={search_space['heuristic_dropped']}")
    parts.append(
        "exact="
        f"bar={exact_screen_reasons['partial_prefix_bar_failure']} "
        f"completion={exact_screen_reasons['terminal_prefix_completion_failure']} "
        f"incumbent={exact_screen_reasons['incumbent_dominance']} "
        f"legality={exact_screen_reasons['legality_connectivity_exact_rejection']}"
    )
    return " ".join(parts)


def load_step_live_checkpoints(run_dir: Path, step_index: int) -> list[dict[str, Any]]:
    path = run_dir / "reports" / "steps" / f"step-{step_index:02}-live.ndjson"
    return compare_runs.load_ndjson(path)


def checkpoint_with_note(
    checkpoints: list[dict[str, Any]], note: str
) -> dict[str, Any] | None:
    for checkpoint in reversed(checkpoints):
        if str(checkpoint.get("note", "")) == note:
            return checkpoint
    return None


def normalize_exact_screen_reasons(value: Any) -> dict[str, int]:
    data = value if isinstance(value, dict) else {}
    return compare_runs.ordered_dict(
        [
            (
                "partial_prefix_bar_failure",
                int(data.get("partial_prefix_bar_failure", 0) or 0),
            ),
            (
                "terminal_prefix_completion_failure",
                int(data.get("terminal_prefix_completion_failure", 0) or 0),
            ),
            ("incumbent_dominance", int(data.get("incumbent_dominance", 0) or 0)),
            (
                "legality_connectivity_exact_rejection",
                int(data.get("legality_connectivity_exact_rejection", 0) or 0),
            ),
        ]
    )


def normalize_claim_root_seeding(value: Any) -> dict[str, Any]:
    data = value if isinstance(value, dict) else {}
    return compare_runs.ordered_dict(
        [
            ("roots_seen", int(data.get("roots_seen", 0) or 0)),
            (
                "roots_rejected_by_insert_root",
                int(data.get("roots_rejected_by_insert_root", 0) or 0),
            ),
            (
                "roots_rejected_by_exact_screen",
                int(data.get("roots_rejected_by_exact_screen", 0) or 0),
            ),
            ("roots_enqueued", int(data.get("roots_enqueued", 0) or 0)),
        ]
    )


def normalize_claim_step_open(value: Any) -> dict[str, Any]:
    data = value if isinstance(value, dict) else {}
    claim_debt_axes = data.get("claim_debt_axes") or {}
    package_flags = data.get("package_flags") or {}
    return compare_runs.ordered_dict(
        [
            ("kappa_min", int(data.get("kappa_min", 0) or 0)),
            ("kappa_max", int(data.get("kappa_max", 0) or 0)),
            ("late_family_surface", str(data.get("late_family_surface", ""))),
            ("anchor_policy", str(data.get("anchor_policy", ""))),
            (
                "historical_anchor_ref",
                optional_int(data.get("historical_anchor_ref")),
            ),
            (
                "claim_widening_band7_active",
                bool(data.get("claim_widening_band7_active", False)),
            ),
            (
                "claim_widening_band8_active",
                bool(data.get("claim_widening_band8_active", False)),
            ),
            (
                "claim_widening_band9_active",
                bool(data.get("claim_widening_band9_active", False)),
            ),
            (
                "claim_debt_axes",
                compare_runs.ordered_dict(
                    [
                        ("kappa_min", int(claim_debt_axes.get("kappa_min", 0) or 0)),
                        ("kappa_max", int(claim_debt_axes.get("kappa_max", 0) or 0)),
                        (
                            "path_pressure",
                            int(claim_debt_axes.get("path_pressure", 0) or 0),
                        ),
                        (
                            "trunc_pressure",
                            int(claim_debt_axes.get("trunc_pressure", 0) or 0),
                        ),
                        (
                            "coupling_pressure",
                            int(claim_debt_axes.get("coupling_pressure", 0) or 0),
                        ),
                        (
                            "support_pressure",
                            int(claim_debt_axes.get("support_pressure", 0) or 0),
                        ),
                        (
                            "modal_pressure",
                            int(claim_debt_axes.get("modal_pressure", 0) or 0),
                        ),
                        (
                            "temporal_pressure",
                            int(claim_debt_axes.get("temporal_pressure", 0) or 0),
                        ),
                        (
                            "reanchor_pressure",
                            int(claim_debt_axes.get("reanchor_pressure", 0) or 0),
                        ),
                        (
                            "closure_pressure",
                            int(claim_debt_axes.get("closure_pressure", 0) or 0),
                        ),
                    ]
                ),
            ),
            (
                "package_flags",
                compare_runs.ordered_dict(
                    [
                        ("operator_bundle", bool(package_flags.get("operator_bundle", False))),
                        (
                            "hilbert_functional",
                            bool(package_flags.get("hilbert_functional", False)),
                        ),
                        ("temporal_shell", bool(package_flags.get("temporal_shell", False))),
                    ]
                ),
            ),
        ]
    )


def normalize_int_list(value: Any) -> list[int]:
    if not isinstance(value, list):
        return []
    return [int(item or 0) for item in value]


def optional_int(value: Any) -> int | None:
    if value is None:
        return None
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def summarize_claim_step_open(claim_step_open: dict[str, Any]) -> str:
    if not claim_step_open:
        return ""

    surface = str(claim_step_open.get("late_family_surface", "")).strip()
    kappa_min = int(claim_step_open.get("kappa_min", 0) or 0)
    kappa_max = int(claim_step_open.get("kappa_max", 0) or 0)
    anchor_policy = str(claim_step_open.get("anchor_policy", "")).strip() or "none"
    historical_anchor_ref = optional_int(claim_step_open.get("historical_anchor_ref"))
    widening_bands = active_widening_bands(claim_step_open)
    package_flags = active_package_flags(claim_step_open.get("package_flags") or {})
    claim_debt_axes = claim_step_open.get("claim_debt_axes") or {}

    parts = []
    if surface:
        parts.append(f"surface={surface}")
    if kappa_min > 0 and kappa_max > 0:
        parts.append(f"kappa={kappa_min}..{kappa_max}")
    anchor = anchor_policy
    if historical_anchor_ref is not None:
        anchor = f"{anchor}@{historical_anchor_ref}"
    if anchor:
        parts.append(f"anchor={anchor}")
    if widening_bands:
        parts.append(f"widen={','.join(str(band) for band in widening_bands)}")
    if package_flags:
        parts.append(f"packages={','.join(package_flags)}")
    axes_summary = summarize_claim_debt_axes(claim_debt_axes)
    if axes_summary:
        parts.append(axes_summary)
    return " ".join(parts)


def active_widening_bands(claim_step_open: dict[str, Any]) -> list[int]:
    active = []
    for band in (7, 8, 9):
        if bool(claim_step_open.get(f"claim_widening_band{band}_active", False)):
            active.append(band)
    return active


def active_package_flags(package_flags: dict[str, Any]) -> list[str]:
    active = []
    for key in ("operator_bundle", "hilbert_functional", "temporal_shell"):
        if bool(package_flags.get(key, False)):
            active.append(key)
    return active


def summarize_claim_debt_axes(claim_debt_axes: dict[str, Any]) -> str:
    if not claim_debt_axes:
        return ""
    axes = [
        ("path", int(claim_debt_axes.get("path_pressure", 0) or 0)),
        ("trunc", int(claim_debt_axes.get("trunc_pressure", 0) or 0)),
        ("coupling", int(claim_debt_axes.get("coupling_pressure", 0) or 0)),
        ("support", int(claim_debt_axes.get("support_pressure", 0) or 0)),
        ("modal", int(claim_debt_axes.get("modal_pressure", 0) or 0)),
        ("temporal", int(claim_debt_axes.get("temporal_pressure", 0) or 0)),
        ("reanchor", int(claim_debt_axes.get("reanchor_pressure", 0) or 0)),
        ("closure", int(claim_debt_axes.get("closure_pressure", 0) or 0)),
    ]
    return "axes=" + ",".join(f"{label}{value}" for label, value in axes)


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
        if label in {"early_breadth", "late_generated_floors"} and check["status"] != "pass":
            for step in check.get("steps", []):
                if step.get("status") == "hit":
                    continue
                diagnosis = step.get("diagnosis") or {}
                summary = str(diagnosis.get("summary", "")).strip()
                if summary:
                    lines.append(f"  step {step['step_index']}: {summary}")
    if certificate["failing_checks"]:
        lines.append("failing checks: " + ", ".join(certificate["failing_checks"]))
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())
