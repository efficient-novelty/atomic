#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any

PRUNE_KEYS = ("quotient_dedupe", "sound_minimality", "heuristic_shaping")
CLAIM_KEYS = (
    "status",
    "adopted_step",
    "adopted_label",
    "adopted_nu",
    "matches_accepted",
)
SEARCH_SPACE_COUNT_KEYS = (
    "enumerated",
    "well_formed",
    "admissibility_rejected",
    "evaluated",
    "canonical",
    "semantically_minimal",
    "retained",
)
LATE_STEP_COMPETITION_START = 10
ADMISSIBILITY_REASON_LIMIT = 5
WORKSTREAM4_REALISTIC_PROFILE = "realistic_frontier_shadow"
WORKSTREAM4_COLD_MODES = {"atomic_search_bootstrap"}
WORKSTREAM4_RESUME_REQUIRED_MODES = (
    "frontier_checkpoint_resume",
    "step_checkpoint_resume",
    "step_checkpoint_reevaluate",
)
NEUTRAL_GOVERNOR_STATES = {"green", "unknown"}
NEUTRAL_PRESSURE_ACTIONS = {"none", "unknown"}


def main() -> int:
    args = parse_args()
    lanes = build_lane_specs(args)
    if not lanes:
        raise SystemExit("provide at least one run directory or --lane LABEL=PATH")

    lane_summaries = [load_lane(label, path) for label, path in lanes]
    lane_by_label = {lane["label"]: lane for lane in lane_summaries}
    baseline_label = args.baseline or lane_summaries[0]["label"]
    if baseline_label not in lane_by_label:
        raise SystemExit(f"baseline lane not found: {baseline_label}")

    summary = build_summary(lane_summaries, baseline_label)
    text_summary = render_text_summary(summary)

    if args.text_out is not None:
        write_text(args.text_out, text_summary)
    if args.json_out is not None:
        write_text(args.json_out, json.dumps(summary, indent=2, sort_keys=True) + "\n")

    sys.stdout.write(text_summary)
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Compare persisted pen-atomic run directories and emit a canonical "
            "human-readable plus machine-readable evidence summary."
        )
    )
    parser.add_argument(
        "run_dirs",
        nargs="*",
        metavar="RUN_DIR",
        help="run directories to compare; lane labels default to the directory name",
    )
    parser.add_argument(
        "--lane",
        action="append",
        default=[],
        metavar="LABEL=PATH",
        help="explicit lane label and run directory",
    )
    parser.add_argument(
        "--baseline",
        metavar="LABEL",
        help="lane label to use as the authoritative baseline; defaults to the first lane",
    )
    parser.add_argument(
        "--json-out",
        type=Path,
        help="optional path for the machine-readable JSON summary",
    )
    parser.add_argument(
        "--text-out",
        type=Path,
        help="optional path for the human-readable text summary",
    )
    return parser.parse_args()


def build_lane_specs(args: argparse.Namespace) -> list[tuple[str, Path]]:
    lanes: list[tuple[str, Path]] = []
    seen_labels: set[str] = set()

    for spec in args.lane:
        label, path = parse_lane_spec(spec)
        if label in seen_labels:
            raise SystemExit(f"duplicate lane label: {label}")
        seen_labels.add(label)
        lanes.append((label, path))

    for raw_path in args.run_dirs:
        path = Path(raw_path).resolve()
        label = path.name
        if label in seen_labels:
            raise SystemExit(f"duplicate lane label: {label}")
        seen_labels.add(label)
        lanes.append((label, path))

    return lanes


def parse_lane_spec(spec: str) -> tuple[str, Path]:
    if "=" not in spec:
        raise SystemExit(f"invalid --lane value '{spec}'; expected LABEL=PATH")
    label, raw_path = spec.split("=", 1)
    label = label.strip()
    raw_path = raw_path.strip()
    if not label or not raw_path:
        raise SystemExit(f"invalid --lane value '{spec}'; expected LABEL=PATH")
    return label, Path(raw_path).resolve()


def load_lane(label: str, run_dir: Path) -> dict[str, Any]:
    manifest_path = run_dir / "run.json"
    steps_dir = run_dir / "reports" / "steps"
    telemetry_path = run_dir / "telemetry.ndjson"

    if not manifest_path.exists():
        raise SystemExit(f"missing run manifest: {manifest_path}")
    if not steps_dir.exists():
        raise SystemExit(f"missing step summaries: {steps_dir}")

    manifest = load_json(manifest_path)
    step_paths = sorted(steps_dir.glob("step-*-summary.json"))
    if not step_paths:
        raise SystemExit(f"no step summaries found in {steps_dir}")

    steps = [load_json(path) for path in step_paths]
    telemetry_events = load_ndjson(telemetry_path)
    telemetry_steps = {
        int(event["step_index"]): event.get("payload", {})
        for event in telemetry_events
        if event.get("event") == "step_accepted" and event.get("step_index") is not None
    }
    run_mode = "unknown"
    for event in telemetry_events:
        if event.get("event") == "run_started":
            run_mode = str(event.get("payload", {}).get("mode", "unknown"))
            break

    search_profile = load_search_profile(run_dir, steps, telemetry_events)
    trajectory = [trajectory_entry(step) for step in steps]
    accepted_hashes = [accepted_hash_entry(step) for step in steps]
    search_space_counts = [search_space_count_entry(step) for step in steps]
    admissibility_diagnostics = [admissibility_diagnostics_entry(step) for step in steps]
    late_step_competition = [
        late_step_competition_entry(step)
        for step in steps
        if int(step.get("step_index", 0) or 0) >= LATE_STEP_COMPETITION_START
    ]
    demo_phase_evidence = [
        entry
        for step in steps
        if (entry := demo_phase_entry(step, search_profile)) is not None
    ]
    provenance_sequence = []
    replay_sequence = []
    governor_sequence = []
    prune_totals = Counter({key: 0 for key in PRUNE_KEYS})
    frontier_deltas = []
    frontier_retention_totals = Counter()

    for step in steps:
        step_index = int(step.get("step_index", 0))
        telemetry_step = telemetry_steps.get(step_index, {})

        provenance = str(step.get("provenance", "unknown") or "unknown")
        replay_status = str(
            (step.get("replay_ablation") or telemetry_step.get("replay_ablation") or {}).get(
                "status", "not_recorded"
            )
        )
        pressure = step.get("frontier_pressure") or telemetry_step.get("frontier_pressure") or {}
        governor_state = str(pressure.get("governor_state", "unknown") or "unknown")
        pressure_action = str(pressure.get("pressure_action", "unknown") or "unknown")
        rss_bytes = int(pressure.get("rss_bytes", 0) or 0)

        provenance_sequence.append({"step_index": step_index, "value": provenance})
        replay_sequence.append({"step_index": step_index, "value": replay_status})
        governor_sequence.append(
            {
                "step_index": step_index,
                "state": governor_state,
                "action": pressure_action,
                "rss_bytes": rss_bytes,
                "value": f"{governor_state}/{pressure_action}",
            }
        )

        step_prunes = prune_counts(step, telemetry_step)
        for key in PRUNE_KEYS:
            prune_totals[key] += step_prunes.get(key, 0)

        frontier_deltas.append(
            {"step_index": step_index, "delta": frontier_retention_delta(step)}
        )
        frontier_retention_totals.update(frontier_retention_counts(step))

    latest_frontier = load_latest_frontier(run_dir)
    step15 = next(
        (step for step in steps if int(step.get("step_index", 0)) == 15),
        None,
    )

    lane = {
        "label": label,
        "path": str(run_dir),
        "run_id": str(manifest.get("run_id", "")),
        "completed_step": int(manifest.get("position", {}).get("completed_step", len(steps))),
        "search_profile": search_profile,
        "run_mode": run_mode,
        "trajectory": trajectory,
        "trajectory_fingerprint": hash_json(trajectory),
        "accepted_hashes": accepted_hashes,
        "search_space_counts": search_space_counts,
        "admissibility_diagnostics": admissibility_diagnostics,
        "late_step_competition": late_step_competition,
        "demo_phase_evidence": demo_phase_evidence,
        "provenance_sequence": provenance_sequence,
        "provenance_summary": summarize_counter(
            Counter(item["value"] for item in provenance_sequence)
        ),
        "replay_ablation_sequence": replay_sequence,
        "replay_ablation_summary": summarize_counter(
            Counter(item["value"] for item in replay_sequence)
        ),
        "prune_sample_totals": ordered_dict(
            [(key, int(prune_totals.get(key, 0))) for key in PRUNE_KEYS]
        ),
        "frontier_retention_delta_total": sum(item["delta"] for item in frontier_deltas),
        "frontier_retention_delta_by_step": frontier_deltas,
        "frontier_retention_totals": summarize_counter(frontier_retention_totals),
        "governor_summary": {
            "states": summarize_counter(
                Counter(item["state"] for item in governor_sequence)
            ),
            "actions": summarize_counter(
                Counter(item["action"] for item in governor_sequence)
            ),
            "max_rss_bytes": max((item["rss_bytes"] for item in governor_sequence), default=0),
            "sequence": governor_sequence,
        },
        "latest_frontier": latest_frontier,
        "step15_claim": normalize_claim(
            step15.get("canon_evidence", {}).get("late_step_claim") if step15 else None
        ),
    }
    return lane


def accepted_hash_entry(step: dict[str, Any]) -> dict[str, Any]:
    accepted = step.get("accepted", {})
    return ordered_dict(
        [
            ("step_index", int(step.get("step_index", 0) or 0)),
            ("candidate_hash", str(accepted.get("candidate_hash", ""))),
            ("canonical_hash", str(accepted.get("canonical_hash", ""))),
        ]
    )


def search_space_count_entry(step: dict[str, Any]) -> dict[str, Any]:
    stats = step.get("search_stats") or {}
    return ordered_dict(
        [
            ("step_index", int(step.get("step_index", 0) or 0)),
            ("enumerated", int(stats.get("enumerated_candidates", 0) or 0)),
            ("well_formed", int(stats.get("well_formed_candidates", 0) or 0)),
            (
                "admissibility_rejected",
                int(stats.get("admissibility_rejections", 0) or 0),
            ),
            ("evaluated", int(stats.get("evaluated_candidates", 0) or 0)),
            ("canonical", int(stats.get("canonical_candidates", 0) or 0)),
            (
                "semantically_minimal",
                int(stats.get("semantically_minimal_candidates", 0) or 0),
            ),
            ("retained", int(stats.get("retained_candidates", 0) or 0)),
        ]
    )


def late_step_competition_entry(step: dict[str, Any]) -> dict[str, Any]:
    stats = step.get("search_stats") or {}
    distribution = stats.get("scored_candidate_distribution") or {}
    return ordered_dict(
        [
            ("step_index", int(step.get("step_index", 0) or 0)),
            ("evaluated", int(stats.get("evaluated_candidates", 0) or 0)),
            ("clears_bar", int(distribution.get("clears_bar", 0) or 0)),
            ("below_bar", int(distribution.get("below_bar", 0) or 0)),
            ("retained", int(stats.get("retained_candidates", 0) or 0)),
            (
                "terminal_rank_prunes",
                int(stats.get("incremental_terminal_rank_prunes", 0) or 0),
            ),
        ]
    )


def admissibility_diagnostics_entry(step: dict[str, Any]) -> dict[str, Any]:
    stats = step.get("search_stats") or {}
    diagnostics = stats.get("admissibility_diagnostics") or {}
    reasons = diagnostics.get("reason_counts") or {}
    top_reasons = [
        ordered_dict([("reason", str(reason)), ("count", int(count or 0))])
        for reason, count in sorted(
            reasons.items(),
            key=lambda item: (-int(item[1] or 0), str(item[0])),
        )[:ADMISSIBILITY_REASON_LIMIT]
    ]
    return ordered_dict(
        [
            ("step_index", int(step.get("step_index", 0) or 0)),
            (
                "exact_legality_rejections",
                int(diagnostics.get("exact_legality_rejections", 0) or 0),
            ),
            (
                "structural_debt_cap_rejections",
                int(diagnostics.get("structural_debt_cap_rejections", 0) or 0),
            ),
            (
                "admitted_deprioritized",
                int(diagnostics.get("admitted_deprioritized", 0) or 0),
            ),
            (
                "admitted_focus_aligned",
                int(diagnostics.get("admitted_focus_aligned", 0) or 0),
            ),
            ("top_reasons", top_reasons),
        ]
    )


def demo_phase_entry(step: dict[str, Any], search_profile: str) -> dict[str, Any] | None:
    stats = step.get("search_stats") or {}
    funnel = stats.get("demo_funnel") or {}
    closure = stats.get("demo_closure") or {}
    phase = stats.get("demo_phase") or {}
    if not has_demo_phase_evidence(search_profile, funnel, closure, phase):
        return None

    generated_floor = optional_int(phase.get("generated_floor"))
    exact_screened_floor = optional_int(phase.get("exact_screened_floor"))
    full_eval_soft_cap = optional_int(phase.get("full_eval_soft_cap"))
    generated = int(funnel.get("generated_raw_prefixes", 0) or 0)
    exact_screened = int(funnel.get("exact_bound_screened", 0) or 0)
    full_evaluated = int(funnel.get("full_telescopes_evaluated", 0) or 0)

    return ordered_dict(
        [
            ("step_index", int(step.get("step_index", 0) or 0)),
            ("generated_raw_prefixes", generated),
            (
                "canonical_prefix_signatures",
                int(funnel.get("canonical_prefix_signatures", 0) or 0),
            ),
            ("hard_admissible", int(funnel.get("hard_admissible", 0) or 0)),
            ("exact_bound_screened", exact_screened),
            ("exact_bound_pruned", int(funnel.get("exact_bound_pruned", 0) or 0)),
            ("heuristic_dropped", int(funnel.get("heuristic_dropped", 0) or 0)),
            ("full_telescopes_evaluated", full_evaluated),
            ("bar_clearers", int(funnel.get("bar_clearers", 0) or 0)),
            (
                "semantically_minimal_clearers",
                int(funnel.get("semantically_minimal_clearers", 0) or 0),
            ),
            ("winner_overshoot", rational_to_string(funnel.get("winner_overshoot"))),
            ("frontier_total_seen", int(closure.get("frontier_total_seen", 0) or 0)),
            (
                "frontier_certified_nonwinning",
                int(closure.get("frontier_certified_nonwinning", 0) or 0),
            ),
            ("closure_percent", int(closure.get("closure_percent", 0) or 0)),
            ("generated_floor", generated_floor),
            (
                "generated_floor_status",
                floor_status(generated, generated_floor),
            ),
            ("exact_screened_floor", exact_screened_floor),
            (
                "exact_screened_floor_status",
                floor_status(exact_screened, exact_screened_floor),
            ),
            ("full_eval_soft_cap", full_eval_soft_cap),
            (
                "full_eval_soft_cap_status",
                soft_cap_status(full_evaluated, full_eval_soft_cap),
            ),
            (
                "breadth_harvest_exit_reason",
                optional_string(phase.get("breadth_harvest_exit_reason")),
            ),
            (
                "proof_close_entry_reason",
                optional_string(phase.get("proof_close_entry_reason")),
            ),
            (
                "proof_close_overrun_reason",
                optional_string(phase.get("proof_close_overrun_reason")),
            ),
            (
                "proof_close_reserved_millis",
                int(phase.get("proof_close_reserved_millis", 0) or 0),
            ),
            (
                "proof_close_elapsed_millis",
                int(phase.get("proof_close_elapsed_millis", 0) or 0),
            ),
            (
                "proof_close_remaining_millis",
                int(phase.get("proof_close_remaining_millis", 0) or 0),
            ),
            (
                "proof_close_reserve_overrun_millis",
                int(phase.get("proof_close_reserve_overrun_millis", 0) or 0),
            ),
            (
                "proof_close_reserve_exhausted",
                bool(phase.get("proof_close_reserve_exhausted", False)),
            ),
            (
                "proof_close_frontier_total_groups",
                int(phase.get("proof_close_frontier_total_groups", 0) or 0),
            ),
            (
                "proof_close_frontier_groups_closed",
                int(phase.get("proof_close_frontier_groups_closed", 0) or 0),
            ),
            (
                "proof_close_frontier_groups_remaining",
                int(phase.get("proof_close_frontier_groups_remaining", 0) or 0),
            ),
            (
                "proof_close_closure_percent",
                int(phase.get("proof_close_closure_percent", 0) or 0),
            ),
            (
                "materialize_full_evals",
                int(phase.get("materialize_full_evals", 0) or 0),
            ),
            (
                "proof_close_full_evals",
                int(phase.get("proof_close_full_evals", 0) or 0),
            ),
            (
                "proof_close_overrun_full_evals",
                int(phase.get("proof_close_overrun_full_evals", 0) or 0),
            ),
            (
                "materialize_soft_cap_triggered",
                bool(phase.get("materialize_soft_cap_triggered", False)),
            ),
        ]
    )


def has_demo_phase_evidence(
    search_profile: str,
    funnel: dict[str, Any],
    closure: dict[str, Any],
    phase: dict[str, Any],
) -> bool:
    if search_profile == "demo_breadth_shadow":
        return True
    return any(
        (
            optional_int(phase.get("generated_floor")) is not None,
            optional_int(phase.get("exact_screened_floor")) is not None,
            optional_int(phase.get("full_eval_soft_cap")) is not None,
            optional_string(phase.get("breadth_harvest_exit_reason")) is not None,
            optional_string(phase.get("proof_close_entry_reason")) is not None,
            optional_string(phase.get("proof_close_overrun_reason")) is not None,
            int(phase.get("proof_close_reserved_millis", 0) or 0) > 0,
            int(phase.get("proof_close_elapsed_millis", 0) or 0) > 0,
            int(phase.get("proof_close_frontier_total_groups", 0) or 0) > 0,
            int(funnel.get("generated_raw_prefixes", 0) or 0) > 0,
            int(funnel.get("exact_bound_screened", 0) or 0) > 0,
            int(closure.get("frontier_total_seen", 0) or 0) > 0,
        )
    )


def optional_int(value: Any) -> int | None:
    if value is None or value == "":
        return None
    return int(value)


def optional_string(value: Any) -> str | None:
    if value is None:
        return None
    text = str(value).strip()
    return text or None


def floor_status(actual: int, target: int | None) -> str:
    if target is None:
        return "not_applicable"
    return "hit" if actual >= target else "miss"


def soft_cap_status(actual: int, target: int | None) -> str:
    if target is None:
        return "not_applicable"
    return "within_cap" if actual <= target else "overrun"


def load_search_profile(
    run_dir: Path, steps: list[dict[str, Any]], telemetry_events: list[dict[str, Any]]
) -> str:
    for step in reversed(steps):
        value = step.get("search_profile")
        if value:
            return str(value)

    for event in telemetry_events:
        if event.get("event") == "run_started":
            payload = event.get("payload", {})
            value = payload.get("search_profile")
            if value:
                return str(value)
            break

    config_path = run_dir / "config.toml"
    if config_path.exists():
        text = config_path.read_text(encoding="utf-8")
        match = re.search(r'^\s*search_profile\s*=\s*"([^"]+)"\s*$', text, re.MULTILINE)
        if match:
            return match.group(1)

    return "unknown"


def trajectory_entry(step: dict[str, Any]) -> dict[str, Any]:
    accepted = step.get("accepted", {})
    return ordered_dict(
        [
            ("step_index", int(step.get("step_index", 0))),
            ("label", str(step.get("label", ""))),
            ("candidate_hash", str(accepted.get("candidate_hash", ""))),
            ("canonical_hash", str(accepted.get("canonical_hash", ""))),
            ("bit_kappa", int(accepted.get("bit_kappa", 0) or 0)),
            ("clause_kappa", int(accepted.get("clause_kappa", 0) or 0)),
            ("nu", int(accepted.get("nu", 0) or 0)),
            ("rho", rational_to_string(accepted.get("rho"))),
            ("objective_bar", rational_to_string(step.get("objective_bar"))),
            ("overshoot", rational_to_string(accepted.get("overshoot"))),
        ]
    )


def prune_counts(step: dict[str, Any], telemetry_step: dict[str, Any]) -> Counter:
    counts = Counter({key: 0 for key in PRUNE_KEYS})
    prune_reports = step.get("prune_reports") or []
    if prune_reports:
        for report in prune_reports:
            key = str(report.get("prune_class", ""))
            if key in PRUNE_KEYS:
                counts[key] += 1
        return counts

    telemetry_counts = telemetry_step.get("prune_samples", {})
    for key in PRUNE_KEYS:
        counts[key] += int(telemetry_counts.get(key, 0) or 0)
    return counts


def frontier_retention_delta(step: dict[str, Any]) -> int:
    stats = step.get("search_stats") or {}
    kept = int(stats.get("frontier_hot_states", 0) or 0) + int(
        stats.get("frontier_cold_states", 0) or 0
    )
    dropped = int(stats.get("frontier_drops", 0) or 0)
    return kept - dropped


def frontier_retention_counts(step: dict[str, Any]) -> Counter:
    counts = Counter()
    for candidate in step.get("candidate_reports") or []:
        key = normalize_frontier_retention(candidate.get("frontier_retention", "not_recorded"))
        counts[key] += 1
    return counts


def normalize_frontier_retention(value: Any) -> str:
    label = str(value or "not_recorded")
    if label == "cold":
        return "cold_resident"
    return label


def load_latest_frontier(run_dir: Path) -> dict[str, Any]:
    frontier_root = run_dir / "checkpoints" / "frontier"
    if not frontier_root.exists():
        return {"present": False}

    candidates = []
    for path in sorted(frontier_root.rglob("frontier.manifest.json")):
        manifest = load_json(path)
        candidates.append(
            (
                int(manifest.get("step_index", 0) or 0),
                int(manifest.get("band_index", 0) or 0),
                int(manifest.get("frontier_epoch", 0) or 0),
                str(path),
                path,
                manifest,
            )
        )

    if not candidates:
        return {"present": False}

    _, _, _, _, manifest_path, manifest = max(candidates)
    cache_blob = manifest.get("files", {}).get("cache_blob")
    cache = {}
    if cache_blob:
        cache_path = manifest_path.parent / str(cache_blob)
        if cache_path.exists():
            cache = load_json(cache_path)

    return ordered_dict(
        [
            ("present", True),
            ("step_index", int(manifest.get("step_index", 0) or 0)),
            ("band_index", int(manifest.get("band_index", 0) or 0)),
            ("frontier_epoch", int(manifest.get("frontier_epoch", 0) or 0)),
            (
                "prefix_explored",
                int(manifest.get("counts", {}).get("prefix_states_explored", 0) or 0),
            ),
            (
                "prefix_exact_pruned",
                int(manifest.get("counts", {}).get("prefix_states_exact_pruned", 0) or 0),
            ),
            (
                "prefix_heuristic_dropped",
                int(
                    manifest.get("counts", {}).get("prefix_states_heuristic_dropped", 0) or 0
                ),
            ),
            ("hot_states", int(manifest.get("counts", {}).get("hot_states", 0) or 0)),
            ("cold_states", int(manifest.get("counts", {}).get("cold_states", 0) or 0)),
            ("dedupe_keys", int(manifest.get("counts", {}).get("dedupe_keys", 0) or 0)),
            ("worker_count", int(manifest.get("scheduler", {}).get("worker_count", 0) or 0)),
            (
                "spill_generation",
                int(manifest.get("scheduler", {}).get("spill_generation", 0) or 0),
            ),
            ("governor_state", str(cache.get("governor_state", "unknown"))),
            ("pressure_action", str(cache.get("pressure_action", "unknown"))),
            (
                "resident_cold_records",
                int(cache.get("resident_cold_records", 0) or 0),
            ),
            (
                "spilled_cold_records",
                int(cache.get("spilled_cold_records", 0) or 0),
            ),
        ]
    )


def normalize_claim(claim: Any) -> dict[str, Any]:
    if not isinstance(claim, dict) or not claim:
        return ordered_dict(
            [
                ("present", False),
                ("status", "missing"),
                ("adopted_step", 0),
                ("adopted_label", ""),
                ("adopted_nu", 0),
                ("matches_accepted", False),
            ]
        )

    return ordered_dict(
        [
            ("present", True),
            ("status", str(claim.get("status", "missing"))),
            ("adopted_step", int(claim.get("adopted_step", 0) or 0)),
            ("adopted_label", str(claim.get("adopted_label", ""))),
            ("adopted_nu", int(claim.get("adopted_nu", 0) or 0)),
            ("matches_accepted", bool(claim.get("matches_accepted", False))),
        ]
    )


def build_summary(lanes: list[dict[str, Any]], baseline_label: str) -> dict[str, Any]:
    baseline = next(lane for lane in lanes if lane["label"] == baseline_label)
    matching_lanes = []
    mismatched_lanes = []
    accepted_matching_lanes = []
    accepted_mismatched_lanes = []
    count_matching_lanes = []
    count_mismatched_lanes = []
    admissibility_matching_lanes = []
    admissibility_mismatched_lanes = []
    competition_matching_lanes = []
    competition_mismatched_lanes = []
    claim_matching_lanes = []
    claim_mismatches = []

    baseline_claim = comparable_claim(baseline["step15_claim"])

    for lane in lanes:
        matches, deltas = compare_trajectory(baseline["trajectory"], lane["trajectory"])
        lane["trajectory_vs_baseline"] = {
            "baseline_label": baseline_label,
            "matches": matches,
            "deltas": deltas,
        }
        if matches:
            matching_lanes.append(lane["label"])
        else:
            mismatched_lanes.append(lane["label"])

        accepted_matches, accepted_deltas = compare_step_tables(
            baseline["accepted_hashes"], lane["accepted_hashes"]
        )
        lane["accepted_hashes_vs_baseline"] = {
            "baseline_label": baseline_label,
            "matches": accepted_matches,
            "deltas": accepted_deltas,
        }
        if accepted_matches:
            accepted_matching_lanes.append(lane["label"])
        else:
            accepted_mismatched_lanes.append(lane["label"])

        count_matches, count_deltas = compare_step_tables(
            baseline["search_space_counts"], lane["search_space_counts"]
        )
        lane["search_space_counts_vs_baseline"] = {
            "baseline_label": baseline_label,
            "matches": count_matches,
            "deltas": count_deltas,
        }
        if count_matches:
            count_matching_lanes.append(lane["label"])
        else:
            count_mismatched_lanes.append(lane["label"])

        admissibility_matches, admissibility_deltas = compare_step_tables(
            baseline["admissibility_diagnostics"], lane["admissibility_diagnostics"]
        )
        lane["admissibility_diagnostics_vs_baseline"] = {
            "baseline_label": baseline_label,
            "matches": admissibility_matches,
            "deltas": admissibility_deltas,
        }
        if admissibility_matches:
            admissibility_matching_lanes.append(lane["label"])
        else:
            admissibility_mismatched_lanes.append(lane["label"])

        competition_matches, competition_deltas = compare_step_tables(
            baseline["late_step_competition"], lane["late_step_competition"]
        )
        lane["late_step_competition_vs_baseline"] = {
            "baseline_label": baseline_label,
            "matches": competition_matches,
            "deltas": competition_deltas,
        }
        if competition_matches:
            competition_matching_lanes.append(lane["label"])
        else:
            competition_mismatched_lanes.append(lane["label"])

        if comparable_claim(lane["step15_claim"]) == baseline_claim:
            claim_matching_lanes.append(lane["label"])
        else:
            claim_mismatches.append(
                {
                    "label": lane["label"],
                    "claim": lane["step15_claim"],
                }
            )

    trajectory_status = (
        "all_match_baseline" if not mismatched_lanes else "deltas_present"
    )
    claim_status = "consistent" if not claim_mismatches else "inconsistent"
    workstream4_rollout = build_workstream4_rollout(
        lanes,
        baseline_label,
        baseline_claim,
    )
    signoff_status = (
        "ready"
        if trajectory_status == "all_match_baseline"
        and claim_status == "consistent"
        and workstream4_rollout["status"] != "attention"
        else "attention"
    )

    return ordered_dict(
        [
            ("comparison_version", 4),
            ("baseline_lane", baseline_label),
            ("lane_order", [lane["label"] for lane in lanes]),
            (
                "accepted_hash_consistency",
                consistency_summary(
                    baseline_label,
                    baseline["accepted_hashes"],
                    accepted_matching_lanes,
                    accepted_mismatched_lanes,
                ),
            ),
            (
                "search_space_count_consistency",
                consistency_summary(
                    baseline_label,
                    baseline["search_space_counts"],
                    count_matching_lanes,
                    count_mismatched_lanes,
                ),
            ),
            (
                "admissibility_diagnostics_consistency",
                consistency_summary(
                    baseline_label,
                    baseline["admissibility_diagnostics"],
                    admissibility_matching_lanes,
                    admissibility_mismatched_lanes,
                ),
            ),
            (
                "late_step_competition_consistency",
                consistency_summary(
                    baseline_label,
                    baseline["late_step_competition"],
                    competition_matching_lanes,
                    competition_mismatched_lanes,
                ),
            ),
            (
                "trajectory_consistency",
                ordered_dict(
                    [
                        ("status", trajectory_status),
                        ("baseline_label", baseline_label),
                        ("baseline_fingerprint", baseline["trajectory_fingerprint"]),
                        ("matching_lanes", matching_lanes),
                        ("mismatched_lanes", mismatched_lanes),
                    ]
                ),
            ),
            (
                "step15_claim_boundary",
                ordered_dict(
                    [
                        ("status", claim_status),
                        ("baseline", baseline["step15_claim"]),
                        ("matching_lanes", claim_matching_lanes),
                        ("mismatches", claim_mismatches),
                    ]
                ),
            ),
            ("workstream4_rollout", workstream4_rollout),
            (
                "signoff",
                ordered_dict(
                    [
                        ("status", signoff_status),
                        (
                            "summary",
                            signoff_summary(
                                signoff_status,
                                baseline_label,
                                len(lanes),
                                baseline["step15_claim"],
                            ),
                        ),
                    ]
                ),
            ),
            ("lanes", lanes),
        ]
    )


def consistency_summary(
    baseline_label: str,
    baseline_value: Any,
    matching_lanes: list[str],
    mismatched_lanes: list[str],
) -> dict[str, Any]:
    return ordered_dict(
        [
            (
                "status",
                "all_match_baseline" if not mismatched_lanes else "deltas_present",
            ),
            ("baseline_label", baseline_label),
            ("baseline_fingerprint", hash_json(baseline_value)),
            ("matching_lanes", matching_lanes),
            ("mismatched_lanes", mismatched_lanes),
        ]
    )


def build_workstream4_rollout(
    lanes: list[dict[str, Any]],
    baseline_label: str,
    baseline_claim: dict[str, Any],
) -> dict[str, Any]:
    realistic_lanes = [
        lane
        for lane in lanes
        if lane["label"] != baseline_label
        and lane["search_profile"] == WORKSTREAM4_REALISTIC_PROFILE
    ]
    parity_rows = [
        build_workstream4_parity_row(lane, baseline_claim)
        for lane in realistic_lanes
        if lane["run_mode"] in WORKSTREAM4_COLD_MODES
    ]
    resume_rows = [
        build_workstream4_resume_row(lane, baseline_claim)
        for lane in realistic_lanes
        if lane["run_mode"] in WORKSTREAM4_RESUME_REQUIRED_MODES
    ]
    pressure_rows = [
        build_workstream4_pressure_row(lane, baseline_claim)
        for lane in realistic_lanes
        if has_pressure_evidence(lane)
    ]

    parity_set = rollout_set_summary(parity_rows)
    resume_set = rollout_set_summary(resume_rows, WORKSTREAM4_RESUME_REQUIRED_MODES)
    pressure_set = rollout_set_summary(pressure_rows)
    overall_status = workstream4_rollout_status(
        parity_set["status"],
        resume_set["status"],
        pressure_set["status"],
        len(realistic_lanes),
    )

    return ordered_dict(
        [
            ("status", overall_status),
            ("baseline_label", baseline_label),
            ("authoritative_lane", baseline_label),
            ("realistic_profile", WORKSTREAM4_REALISTIC_PROFILE),
            ("parity_set", parity_set),
            ("resume_set", resume_set),
            ("pressure_set", pressure_set),
        ]
    )


def build_workstream4_parity_row(
    lane: dict[str, Any], baseline_claim: dict[str, Any]
) -> dict[str, Any]:
    trajectory_matches = bool(lane["trajectory_vs_baseline"]["matches"])
    accepted_hashes_match = bool(lane["accepted_hashes_vs_baseline"]["matches"])
    claim_matches = comparable_claim(lane["step15_claim"]) == baseline_claim
    search_space_differs = not lane["search_space_counts_vs_baseline"]["matches"]
    late_step_competition_differs = not lane["late_step_competition_vs_baseline"]["matches"]
    prefix_frontier_present = has_prefix_frontier_evidence(lane)
    pressure_exercised = has_pressure_evidence(lane)
    status = (
        "ready"
        if trajectory_matches
        and accepted_hashes_match
        and claim_matches
        and late_step_competition_differs
        and prefix_frontier_present
        else "attention"
    )

    return ordered_dict(
        [
            ("label", lane["label"]),
            ("status", status),
            ("completed_step", lane["completed_step"]),
            ("run_mode", lane["run_mode"]),
            ("trajectory_matches_baseline", trajectory_matches),
            ("accepted_hashes_match_baseline", accepted_hashes_match),
            ("step15_claim_matches_baseline", claim_matches),
            ("search_space_differs_from_baseline", search_space_differs),
            ("late_step_competition_differs_from_baseline", late_step_competition_differs),
            ("prefix_frontier_present", prefix_frontier_present),
            ("pressure_exercised", pressure_exercised),
        ]
    )


def build_workstream4_resume_row(
    lane: dict[str, Any], baseline_claim: dict[str, Any]
) -> dict[str, Any]:
    return build_workstream4_parity_row(lane, baseline_claim)


def build_workstream4_pressure_row(
    lane: dict[str, Any], baseline_claim: dict[str, Any]
) -> dict[str, Any]:
    parity_row = build_workstream4_parity_row(lane, baseline_claim)
    latest_frontier = lane["latest_frontier"]
    return ordered_dict(
        list(parity_row.items())
        + [
            (
                "latest_frontier_pressure_action",
                str(latest_frontier.get("pressure_action", "unknown")),
            ),
            (
                "latest_frontier_governor_state",
                str(latest_frontier.get("governor_state", "unknown")),
            ),
            (
                "spilled_cold_records",
                int(latest_frontier.get("spilled_cold_records", 0) or 0),
            ),
        ]
    )


def rollout_set_summary(
    rows: list[dict[str, Any]], required_modes: tuple[str, ...] = ()
) -> dict[str, Any]:
    if not rows:
        return ordered_dict(
            [
                ("status", "not_present"),
                ("required_modes", list(required_modes)),
                ("present_modes", []),
                ("missing_modes", list(required_modes)),
                ("ready_lanes", []),
                ("attention_lanes", []),
                ("lanes", []),
            ]
        )

    present_modes = sorted({str(row.get("run_mode", "unknown")) for row in rows})
    missing_modes = [
        mode for mode in required_modes if mode not in set(present_modes)
    ]
    ready_lanes = [row["label"] for row in rows if row["status"] == "ready"]
    attention_lanes = [row["label"] for row in rows if row["status"] != "ready"]
    status = "ready" if not attention_lanes and not missing_modes else "attention"
    return ordered_dict(
        [
            ("status", status),
            ("required_modes", list(required_modes)),
            ("present_modes", present_modes),
            ("missing_modes", missing_modes),
            ("ready_lanes", ready_lanes),
            ("attention_lanes", attention_lanes),
            ("lanes", rows),
        ]
    )


def workstream4_rollout_status(
    parity_status: str,
    resume_status: str,
    pressure_status: str,
    realistic_lane_count: int,
) -> str:
    if realistic_lane_count == 0:
        return "not_present"
    if (
        parity_status != "ready"
        or resume_status != "ready"
        or pressure_status != "ready"
    ):
        return "attention"
    return "ready"


def signoff_summary(
    status: str, baseline_label: str, lane_count: int, baseline_claim: dict[str, Any]
) -> str:
    if status == "ready":
        return (
            f"all {lane_count} lanes preserve baseline {baseline_label} and the "
            f"step-15 {baseline_claim['adopted_label']} claim boundary"
        )
    return f"one or more lanes diverge from baseline {baseline_label} or the step-15 claim"


def compare_trajectory(
    baseline: list[dict[str, Any]], current: list[dict[str, Any]]
) -> tuple[bool, list[dict[str, Any]]]:
    return compare_step_tables(baseline, current)


def compare_step_tables(
    baseline: list[dict[str, Any]], current: list[dict[str, Any]]
) -> tuple[bool, list[dict[str, Any]]]:
    baseline_by_step = {entry["step_index"]: entry for entry in baseline}
    current_by_step = {entry["step_index"]: entry for entry in current}
    deltas = []

    for step_index in sorted(set(baseline_by_step) | set(current_by_step)):
        baseline_entry = baseline_by_step.get(step_index)
        current_entry = current_by_step.get(step_index)
        if baseline_entry == current_entry:
            continue
        deltas.append(
            ordered_dict(
                [
                    ("step_index", step_index),
                    ("changed_keys", changed_keys(baseline_entry, current_entry)),
                    ("baseline", baseline_entry),
                    ("current", current_entry),
                ]
            )
        )

    return (not deltas, deltas)


def changed_keys(
    baseline_entry: dict[str, Any] | None, current_entry: dict[str, Any] | None
) -> list[str]:
    if not isinstance(baseline_entry, dict) or not isinstance(current_entry, dict):
        return []

    keys = sorted(set(baseline_entry) | set(current_entry))
    return [
        key
        for key in keys
        if key != "step_index" and baseline_entry.get(key) != current_entry.get(key)
    ]


def comparable_claim(claim: dict[str, Any]) -> dict[str, Any]:
    return {key: claim.get(key) for key in CLAIM_KEYS}


def has_prefix_frontier_evidence(lane: dict[str, Any]) -> bool:
    latest_frontier = lane.get("latest_frontier") or {}
    return int(latest_frontier.get("prefix_explored", 0) or 0) > 0


def has_pressure_evidence(lane: dict[str, Any]) -> bool:
    governor_sequence = lane.get("governor_summary", {}).get("sequence") or []
    for item in governor_sequence:
        if str(item.get("state", "unknown")) not in NEUTRAL_GOVERNOR_STATES:
            return True
        if str(item.get("action", "unknown")) not in NEUTRAL_PRESSURE_ACTIONS:
            return True

    latest_frontier = lane.get("latest_frontier") or {}
    return (
        str(latest_frontier.get("governor_state", "unknown")) not in NEUTRAL_GOVERNOR_STATES
        or str(latest_frontier.get("pressure_action", "unknown")) not in NEUTRAL_PRESSURE_ACTIONS
    )


def render_text_summary(summary: dict[str, Any]) -> str:
    workstream4_rollout = summary["workstream4_rollout"]
    lines = [
        f"Comparison Signoff: {summary['signoff']['status']}",
        f"baseline: {summary['baseline_lane']}",
        f"lanes: {', '.join(summary['lane_order'])}",
        f"trajectory: {render_trajectory_status(summary)}",
        f"accepted hashes: {render_consistency_status(summary['accepted_hash_consistency'])}",
        (
            "search-space counts: "
            f"{render_consistency_status(summary['search_space_count_consistency'])}"
        ),
        (
            "admissibility diagnostics: "
            f"{render_consistency_status(summary['admissibility_diagnostics_consistency'])}"
        ),
        (
            "late-step competition: "
            f"{render_consistency_status(summary['late_step_competition_consistency'])}"
        ),
        f"step15 claim boundary: {render_claim_status(summary['step15_claim_boundary'])}",
        f"workstream4 rollout: {workstream4_rollout['status']}",
        (
            "workstream4 parity set: "
            f"{render_workstream4_set(workstream4_rollout['parity_set'])}"
        ),
        (
            "workstream4 resume set: "
            f"{render_workstream4_set(workstream4_rollout['resume_set'])}"
        ),
        (
            "workstream4 pressure set: "
            f"{render_workstream4_set(workstream4_rollout['pressure_set'])}"
        ),
    ]

    for lane in summary["lanes"]:
        lines.append("")
        lines.extend(render_lane_summary(lane))

    return "\n".join(lines) + "\n"


def render_trajectory_status(summary: dict[str, Any]) -> str:
    return render_consistency_status(summary["trajectory_consistency"])


def render_consistency_status(consistency: dict[str, Any]) -> str:
    if consistency["status"] == "all_match_baseline":
        return (
            f"all {len(consistency['matching_lanes'])} lanes match baseline "
            f"{consistency['baseline_label']}"
        )
    return (
        f"mismatches detected in {', '.join(consistency['mismatched_lanes'])}"
    )


def render_claim_status(claim_boundary: dict[str, Any]) -> str:
    baseline = claim_boundary["baseline"]
    if claim_boundary["status"] == "consistent":
        return (
            f"consistent ({baseline['status']} step {baseline['adopted_step']} "
            f"{baseline['adopted_label']} nu={baseline['adopted_nu']} "
            f"matches_accepted={str(baseline['matches_accepted']).lower()})"
        )
    mismatched = ", ".join(item["label"] for item in claim_boundary["mismatches"])
    return f"inconsistent ({mismatched})"


def render_workstream4_set(rollout_set: dict[str, Any]) -> str:
    status = rollout_set["status"]
    if status == "not_present":
        return "not present"

    missing_modes = rollout_set.get("missing_modes") or []
    if missing_modes:
        return f"attention (missing modes: {', '.join(missing_modes)})"

    if status == "ready":
        return f"ready ({', '.join(rollout_set['ready_lanes'])})"

    return f"attention ({', '.join(rollout_set['attention_lanes'])})"


def render_lane_summary(lane: dict[str, Any]) -> list[str]:
    deltas = lane["trajectory_vs_baseline"]["deltas"]
    accepted_hash_deltas = lane["accepted_hashes_vs_baseline"]["deltas"]
    search_space_deltas = lane["search_space_counts_vs_baseline"]["deltas"]
    admissibility_deltas = lane["admissibility_diagnostics_vs_baseline"]["deltas"]
    late_step_deltas = lane["late_step_competition_vs_baseline"]["deltas"]
    trajectory_line = (
        f"trajectory: matches baseline ({lane['trajectory_fingerprint']})"
        if not deltas
        else (
            f"trajectory: diverges at {render_delta_steps(deltas)} "
            f"({lane['trajectory_fingerprint']})"
        )
    )
    governor_compact = compact_sequence(lane["governor_summary"]["sequence"])
    replay_compact = compact_sequence(lane["replay_ablation_sequence"])
    provenance_compact = compact_sequence(lane["provenance_sequence"])
    last_retention_delta = lane["frontier_retention_delta_by_step"][-1]["delta"]
    last_retention_step = lane["frontier_retention_delta_by_step"][-1]["step_index"]
    latest_demo = lane["demo_phase_evidence"][-1] if lane["demo_phase_evidence"] else None

    lines = [
        f"Lane {lane['label']}",
        f"  run_id: {lane['run_id']}",
        f"  search profile: {lane['search_profile']}",
        f"  mode: {lane['run_mode']}",
        f"  completed_step: {lane['completed_step']}",
        f"  {trajectory_line}",
        f"  accepted hashes: {render_lane_comparison(accepted_hash_deltas)}",
        f"  search-space counts: {render_lane_comparison(search_space_deltas)}",
        f"  admissibility diagnostics: {render_lane_comparison(admissibility_deltas)}",
        (
            "  late-step competition: "
            f"{render_late_step_competition(lane['late_step_competition'], late_step_deltas)}"
        ),
        f"  provenance sequence: {render_compact_sequence(provenance_compact)}",
        f"  replay ablation: {render_compact_sequence(replay_compact)}",
        f"  prune samples: {render_prune_totals(lane['prune_sample_totals'])}",
        (
            "  frontier retention delta: "
            f"total={lane['frontier_retention_delta_total']} "
            f"latest=step {last_retention_step} {render_signed(last_retention_delta)}"
        ),
        (
            "  governor sequence: "
            f"{render_compact_sequence(governor_compact)} "
            f"(max_rss_bytes={lane['governor_summary']['max_rss_bytes']})"
        ),
        f"  latest frontier: {render_latest_frontier(lane['latest_frontier'])}",
        (
            "  step15 claim: "
            f"{lane['step15_claim']['status']} step {lane['step15_claim']['adopted_step']} "
            f"{lane['step15_claim']['adopted_label']} nu={lane['step15_claim']['adopted_nu']} "
            f"matches_accepted={str(lane['step15_claim']['matches_accepted']).lower()}"
        ),
    ]
    if latest_demo is not None:
        lines.append(
            "  demo phase latest: "
            f"step {latest_demo['step_index']} "
            f"generated_floor={render_floor_result(latest_demo['generated_raw_prefixes'], latest_demo['generated_floor'], latest_demo['generated_floor_status'])} "
            f"exact_screened_floor={render_floor_result(latest_demo['exact_bound_screened'], latest_demo['exact_screened_floor'], latest_demo['exact_screened_floor_status'])} "
            f"soft_cap={render_floor_result(latest_demo['full_telescopes_evaluated'], latest_demo['full_eval_soft_cap'], latest_demo['full_eval_soft_cap_status'])} "
            f"breadth_exit={latest_demo['breadth_harvest_exit_reason'] or 'none'} "
            f"proof_close_reason={latest_demo['proof_close_entry_reason'] or 'none'} "
            f"overrun_reason={latest_demo['proof_close_overrun_reason'] or 'none'} "
            f"overrun_full_evals={latest_demo['proof_close_overrun_full_evals']} "
            f"proof_close_closure={latest_demo['proof_close_closure_percent']}% "
            f"groups_closed={latest_demo['proof_close_frontier_groups_closed']}/{latest_demo['proof_close_frontier_total_groups']} "
            f"reserve={latest_demo['proof_close_elapsed_millis']}/{latest_demo['proof_close_reserved_millis']}ms "
            f"reserve_remaining={latest_demo['proof_close_remaining_millis']}ms "
            f"reserve_overrun={latest_demo['proof_close_reserve_overrun_millis']}ms "
            f"reserve_exhausted={str(latest_demo['proof_close_reserve_exhausted']).lower()}"
        )
        lines.append(
            "  demo funnel latest: "
            f"generated={latest_demo['generated_raw_prefixes']} "
            f"hard_admissible={latest_demo['hard_admissible']} "
            f"exact_screened={latest_demo['exact_bound_screened']} "
            f"exact_pruned={latest_demo['exact_bound_pruned']} "
            f"full_eval={latest_demo['full_telescopes_evaluated']} "
            f"closure={latest_demo['closure_percent']}% "
            f"winner_overshoot={latest_demo['winner_overshoot']}"
        )
    return lines


def render_delta_steps(deltas: list[dict[str, Any]]) -> str:
    steps = [f"step {delta['step_index']}" for delta in deltas]
    return ", ".join(steps)


def render_lane_comparison(deltas: list[dict[str, Any]]) -> str:
    if not deltas:
        return "matches baseline"
    return f"diverges at {render_delta_steps(deltas)}"


def render_late_step_competition(
    entries: list[dict[str, Any]], deltas: list[dict[str, Any]]
) -> str:
    if not entries:
        return "not recorded"
    if deltas:
        return f"diverges at {render_delta_steps(deltas)}"

    compact = compact_sequence(
        [
            {
                "step_index": entry["step_index"],
                "value": (
                    f"eval={entry['evaluated']} clears={entry['clears_bar']} "
                    f"below={entry['below_bar']} retained={entry['retained']} "
                    f"terminal_rank_prunes={entry['terminal_rank_prunes']}"
                ),
            }
            for entry in entries
        ]
    )
    return render_compact_sequence(compact)


def render_prune_totals(prune_totals: dict[str, int]) -> str:
    return ", ".join(f"{key}={prune_totals.get(key, 0)}" for key in PRUNE_KEYS)


def render_latest_frontier(frontier: dict[str, Any]) -> str:
    if not frontier.get("present"):
        return "not present"
    return (
        f"step {frontier['step_index']} band {frontier['band_index']} "
        f"epoch={frontier['frontier_epoch']} hot={frontier['hot_states']} "
        f"cold={frontier['cold_states']} dedupe={frontier['dedupe_keys']} "
        f"spill_generation={frontier['spill_generation']} "
        f"cache={frontier['governor_state']}/{frontier['pressure_action']}"
    )


def render_floor_result(actual: int, target: int | None, status: str) -> str:
    if target is None:
        return "not_applicable"
    return f"{status} ({actual}/{target})"


def compact_sequence(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    if not items:
        return []

    compacted = []
    start_step = items[0]["step_index"]
    end_step = items[0]["step_index"]
    value = items[0]["value"]

    for item in items[1:]:
        if item["value"] == value and item["step_index"] == end_step + 1:
            end_step = item["step_index"]
            continue
        compacted.append(
            {"start_step": start_step, "end_step": end_step, "value": value}
        )
        start_step = item["step_index"]
        end_step = item["step_index"]
        value = item["value"]

    compacted.append({"start_step": start_step, "end_step": end_step, "value": value})
    return compacted


def render_compact_sequence(chunks: list[dict[str, Any]]) -> str:
    if not chunks:
        return "none"
    parts = []
    for chunk in chunks:
        if chunk["start_step"] == chunk["end_step"]:
            step_label = f"{chunk['start_step']:02}"
        else:
            step_label = f"{chunk['start_step']:02}-{chunk['end_step']:02}"
        parts.append(f"{step_label} {chunk['value']}")
    return "; ".join(parts)


def render_signed(value: int) -> str:
    return f"+{value}" if value >= 0 else str(value)


def hash_json(value: Any) -> str:
    payload = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def load_ndjson(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    events = []
    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if not line:
                continue
            events.append(json.loads(line))
    return events


def rational_to_string(value: Any) -> str:
    if isinstance(value, str):
        return value
    if isinstance(value, dict) and "num" in value and "den" in value:
        return f"{value['num']}/{value['den']}"
    return "0/1"


def summarize_counter(counter: Counter) -> dict[str, int]:
    return ordered_dict((key, int(counter[key])) for key in sorted(counter))


def ordered_dict(items: Any) -> dict[str, Any]:
    return dict(items)


def write_text(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


if __name__ == "__main__":
    raise SystemExit(main())
