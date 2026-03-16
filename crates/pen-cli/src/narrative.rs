use crate::progress::{format_millis, format_seconds, render_goal_line, render_limit_line};
use crate::report::{CandidateStatus, FrontierRetention, PruneReportClass, StepReport};
use anyhow::Result;
use pen_search::config::{DemoConfig, RuntimeConfig, SearchProfile};
use pen_search::narrative::{NarrativeEvent, NarrativeEventKind, NarrativeProgressSnapshot};
use std::fs;
use std::path::Path;

pub fn write_demo_step_artifacts(
    run_dir: &Path,
    steps: &[StepReport],
    config: &RuntimeConfig,
) -> Result<()> {
    if config.mode.search_profile != SearchProfile::DemoBreadthShadow || !config.demo.enabled {
        return Ok(());
    }

    let steps_dir = run_dir.join("reports").join("steps");
    fs::create_dir_all(&steps_dir)?;

    for step in steps {
        fs::write(
            steps_dir.join(step_narrative_file_name(step.step_index)),
            format!("{}\n", render_step_narrative(step, &config.demo)),
        )?;

        let events_path = steps_dir.join(step_events_file_name(step.step_index));
        if config.demo.narrative.persist_step_events {
            let body = step_events(step)
                .iter()
                .map(serde_json::to_string)
                .collect::<std::result::Result<Vec<_>, _>>()?
                .join("\n");
            fs::write(events_path, format!("{body}\n"))?;
        } else if events_path.exists() {
            fs::remove_file(events_path)?;
        }
    }

    Ok(())
}

pub fn render_step_narrative(step: &StepReport, demo: &DemoConfig) -> String {
    let events = step_events(step);
    let progress = events
        .last()
        .and_then(|event| event.progress.as_ref())
        .cloned()
        .unwrap_or_else(|| fallback_progress(step));
    let funnel = stored_demo_funnel(step);
    let closure = stored_demo_closure(step);
    let generated_floor = step
        .search_stats
        .demo_phase
        .generated_floor
        .or_else(|| generated_floor(step.step_index, demo));
    let exact_screened_floor = step
        .search_stats
        .demo_phase
        .exact_screened_floor
        .or_else(|| exact_screened_floor(step.step_index, demo));
    let full_eval_soft_cap = step
        .search_stats
        .demo_phase
        .full_eval_soft_cap
        .or_else(|| full_eval_soft_cap(step.step_index, demo));

    let mut lines = vec![
        format!("step {:02} demo narrative", step.step_index),
        format!("label        {}", step.label),
        format!("profile      {}", demo.profile),
        format!(
            "accepted     {} nu={} kappa={} overshoot={}",
            step.accepted.candidate_hash,
            step.accepted.nu,
            step.accepted.clause_kappa,
            step.accepted.overshoot
        ),
        time_line(step.step_index, progress.elapsed_millis, demo),
        render_goal_line(
            "generated",
            u64::try_from(funnel.generated_raw_prefixes).expect("generated count exceeded u64"),
            generated_floor,
            "generated",
        ),
        render_goal_line(
            "exact_screen",
            u64::try_from(funnel.exact_bound_screened).expect("screened count exceeded u64"),
            exact_screened_floor,
            "screened",
        ),
        render_limit_line(
            "full_eval",
            u64::try_from(funnel.full_telescopes_evaluated).expect("full-eval count exceeded u64"),
            full_eval_soft_cap,
            "evaluated",
        ),
        format!(
            "closure      frontier_total={} certified_nonwinning={} closure={}%",
            closure.frontier_total_seen,
            closure.frontier_certified_nonwinning,
            closure.closure_percent
        ),
        demo_phase_line(step),
        format!(
            "timing       discovery={} frontier_plan={} selection={}",
            format_millis(
                step.search_stats
                    .search_timing
                    .candidate_discovery_wall_clock_millis
            ),
            format_millis(
                step.search_stats
                    .search_timing
                    .prefix_frontier_planning_wall_clock_millis
            ),
            format_millis(step.search_stats.search_timing.selection_wall_clock_millis)
        ),
        format!(
            "funnel       enumerated={} well_formed={} malformed={} admissibility_rejected={} scored={} canonical={} semantically_minimal={}",
            step.search_stats.enumerated_candidates,
            step.search_stats.well_formed_candidates,
            step.search_stats.malformed_rejections,
            step.search_stats.admissibility_rejections,
            step.search_stats.full_telescopes_evaluated,
            step.search_stats.canonical_candidates,
            step.search_stats.semantically_minimal_candidates
        ),
        format!(
            "demo_funnel  raw={} canonical_prefixes={} hard_admissible={} exact_pruned={} heuristic_dropped={} bar_clearers={} semantically_minimal_clearers={} winner_overshoot={}",
            funnel.generated_raw_prefixes,
            funnel.canonical_prefix_signatures,
            funnel.hard_admissible,
            funnel.exact_bound_pruned,
            funnel.heuristic_dropped,
            funnel.bar_clearers,
            funnel.semantically_minimal_clearers,
            funnel.winner_overshoot
        ),
    ];

    if step.search_stats.prefix_states_explored > 0 {
        lines.push(format!(
            "prefixes     created={} explored={} merged={} exact_pruned={} heuristic_dropped={} hot={} cold={}",
            step.search_stats.prefixes_created,
            step.search_stats.prefix_states_explored,
            step.search_stats.prefix_states_merged_by_signature,
            step.search_stats.prefix_states_exact_pruned,
            step.search_stats.prefix_states_heuristic_dropped,
            step.search_stats.prefix_frontier_hot_states,
            step.search_stats.prefix_frontier_cold_states
        ));
        lines.push(format!(
            "memo         legality_hits={} active_window_hits={} terminal_completion_hits={} partial_bound_hits={} terminal_rank_prunes={}",
            step.search_stats.incremental_legality_cache_hits,
            step.search_stats.incremental_active_window_clause_filter_hits,
            step.search_stats.incremental_terminal_prefix_completion_hits,
            step.search_stats.incremental_partial_prefix_bound_hits,
            step.search_stats.incremental_terminal_rank_prunes
        ));
    }

    lines.push(String::new());
    lines.push("events".to_owned());
    for event in &events {
        let phase = event
            .phase
            .map(|phase| format!(" {}", phase.as_str()))
            .unwrap_or_default();
        lines.push(format!(
            "  [{}{}] {}",
            event_kind_label(event.kind),
            phase,
            event.message
        ));
        if let Some(detail) = &event.detail {
            lines.push(format!("    {detail}"));
        }
    }

    lines.push(String::new());
    lines.push("retained candidates".to_owned());
    if step.candidate_reports.is_empty() {
        lines.push("  none".to_owned());
    } else {
        for candidate in step.candidate_reports.iter().take(8) {
            lines.push(format!(
                "  {} {} rho={} distance={} frontier={} headline={}",
                candidate_status_label(candidate.status),
                candidate.candidate_hash,
                candidate.rho,
                candidate.distance_to_bar,
                frontier_retention_label(candidate.frontier_retention),
                candidate.headline
            ));
        }
        if step.candidate_reports.len() > 8 {
            lines.push(format!(
                "  ... {} more retained candidates",
                step.candidate_reports.len() - 8
            ));
        }
    }

    lines.push(String::new());
    lines.push("prunes".to_owned());
    if step.prune_reports.is_empty() {
        lines.push("  none".to_owned());
    } else {
        for prune in &step.prune_reports {
            lines.push(format!(
                "  [{}] {} rho={} delta={} {}",
                prune_class_label(prune.prune_class),
                prune.candidate_hash,
                prune.rho,
                prune.bar_delta,
                prune.headline
            ));
            if !prune.note.is_empty() {
                lines.push(format!("    {}", prune.note));
            }
        }
    }

    lines.push(String::new());
    lines.push("accepted trace".to_owned());
    if step.trace.is_empty() {
        lines.push("  none".to_owned());
    } else {
        for trace in &step.trace {
            lines.push(format!("  {trace}"));
        }
    }

    lines.join("\n")
}

fn step_events(step: &StepReport) -> Vec<NarrativeEvent> {
    if !step.narrative_events.is_empty() {
        return step.narrative_events.clone();
    }

    vec![
        NarrativeEvent {
            step_index: step.step_index,
            ordinal: 1,
            kind: NarrativeEventKind::StepOpen,
            phase: None,
            message: format!(
                "opened step {} with bar {}",
                step.step_index, step.objective_bar
            ),
            detail: None,
            progress: Some(fallback_progress(step)),
        },
        NarrativeEvent {
            step_index: step.step_index,
            ordinal: 2,
            kind: NarrativeEventKind::AcceptanceSummary,
            phase: None,
            message: format!(
                "accepted {} with overshoot {}",
                step.accepted.candidate_hash, step.accepted.overshoot
            ),
            detail: Some(format!(
                "nu={} kappa={}",
                step.accepted.nu, step.accepted.clause_kappa
            )),
            progress: Some(fallback_progress(step)),
        },
    ]
}

fn fallback_progress(step: &StepReport) -> NarrativeProgressSnapshot {
    NarrativeProgressSnapshot {
        elapsed_millis: step.search_stats.search_timing.step_wall_clock_millis,
        generated_surface: if step.search_stats.prefix_states_explored > 0 {
            step.search_stats
                .prefixes_created
                .max(step.search_stats.enumerated_candidates) as u64
        } else {
            step.search_stats.enumerated_candidates as u64
        },
        exact_screened_surface: (step.search_stats.prefix_states_exact_pruned
            + step.search_stats.full_telescopes_evaluated) as u64,
        full_telescopes_evaluated: step.search_stats.full_telescopes_evaluated as u64,
    }
}

fn stored_demo_funnel(step: &StepReport) -> pen_search::engine::DemoFunnelStats {
    if step.search_stats.demo_funnel.generated_raw_prefixes > 0
        || step.search_stats.demo_funnel.full_telescopes_evaluated > 0
        || step.search_stats.demo_funnel.bar_clearers > 0
        || step.search_stats.demo_funnel.semantically_minimal_clearers > 0
        || step.search_stats.demo_funnel.exact_bound_screened > 0
        || step.search_stats.demo_phase.generated_floor.is_some()
        || step.search_stats.demo_phase.exact_screened_floor.is_some()
    {
        return step.search_stats.demo_funnel.clone();
    }

    pen_search::engine::DemoFunnelStats {
        generated_raw_prefixes: if step.search_stats.prefix_states_explored > 0 {
            step.search_stats
                .prefixes_created
                .max(step.search_stats.enumerated_candidates)
        } else {
            step.search_stats.enumerated_candidates
        },
        canonical_prefix_signatures: if step.search_stats.prefixes_created > 0 {
            step.search_stats.prefixes_created
        } else {
            step.search_stats.enumerated_candidates
        },
        well_formed_terminals: step.search_stats.well_formed_candidates,
        hard_admissible: step
            .search_stats
            .well_formed_candidates
            .saturating_sub(step.search_stats.admissibility_rejections),
        exact_bound_screened: step.search_stats.prefix_states_exact_pruned
            + step.search_stats.full_telescopes_evaluated,
        exact_bound_pruned: step.search_stats.prefix_states_exact_pruned,
        heuristic_dropped: step.search_stats.prefix_states_heuristic_dropped
            + step.search_stats.heuristic_drops,
        full_telescopes_evaluated: step.search_stats.full_telescopes_evaluated,
        bar_clearers: step.search_stats.scored_candidate_distribution.clears_bar,
        semantically_minimal_clearers: step
            .candidate_reports
            .iter()
            .filter(|candidate| candidate.rho >= step.objective_bar)
            .count(),
        winner_overshoot: step.accepted.overshoot,
    }
}

fn stored_demo_closure(step: &StepReport) -> pen_search::engine::DemoClosureStats {
    if step.search_stats.demo_closure.frontier_total_seen > 0
        || step.search_stats.demo_closure.frontier_certified_nonwinning > 0
    {
        return step.search_stats.demo_closure.clone();
    }

    let funnel = stored_demo_funnel(step);
    let closure_percent = if funnel.exact_bound_screened == 0 {
        0
    } else {
        u16::try_from(
            ((funnel.exact_bound_pruned as u128) * 100) / (funnel.exact_bound_screened as u128),
        )
        .expect("closure percent exceeded u16")
    };
    pen_search::engine::DemoClosureStats {
        frontier_total_seen: funnel.exact_bound_screened,
        frontier_certified_nonwinning: funnel.exact_bound_pruned,
        closure_percent,
    }
}

fn generated_floor(step_index: u32, demo: &DemoConfig) -> Option<u64> {
    if step_index == 1 {
        return Some(2144);
    }
    demo.floors
        .generated_floor
        .get(&step_index.to_string())
        .copied()
}

fn exact_screened_floor(step_index: u32, demo: &DemoConfig) -> Option<u64> {
    demo.floors
        .exact_screened_floor
        .get(&step_index.to_string())
        .copied()
}

fn full_eval_soft_cap(step_index: u32, demo: &DemoConfig) -> Option<u64> {
    demo.caps
        .full_eval_soft_cap
        .get(&step_index.to_string())
        .copied()
}

fn demo_phase_line(step: &StepReport) -> String {
    let phase = &step.search_stats.demo_phase;
    let soft_cap = phase
        .full_eval_soft_cap
        .map(|limit| limit.to_string())
        .unwrap_or_else(|| "none".to_owned());
    let breadth_exit = phase
        .breadth_harvest_exit_reason
        .map(|reason| reason.as_str())
        .unwrap_or("none");
    let proof_close_reason = phase
        .proof_close_entry_reason
        .map(|reason| reason.as_str())
        .unwrap_or("none");
    let proof_close_overrun_reason = phase
        .proof_close_overrun_reason
        .map(|reason| reason.as_str())
        .unwrap_or("none");
    format!(
        "phase_eval   materialize={} proof_close={} overrun={} cap={} cap_triggered={} breadth_exit={} proof_close_reason={} overrun_reason={}",
        phase.materialize_full_evals,
        phase.proof_close_full_evals,
        phase.proof_close_overrun_full_evals,
        soft_cap,
        phase.materialize_soft_cap_triggered,
        breadth_exit,
        proof_close_reason,
        proof_close_overrun_reason
    )
}

fn time_line(step_index: u32, elapsed_millis: u64, demo: &DemoConfig) -> String {
    if step_index <= 4 {
        return format!(
            "time         {} (shared early window {} across steps 1-4)",
            format_millis(elapsed_millis),
            format_seconds(demo.early_exhaustive_budget_sec)
        );
    }

    let budget = demo
        .floors
        .step_floor_sec
        .get(&step_index.to_string())
        .copied()
        .map(u64::from);
    render_limit_line(
        "time",
        elapsed_millis,
        budget.map(|seconds| seconds * 1_000),
        "budget_ms",
    )
}

fn step_narrative_file_name(step_index: u32) -> String {
    format!("step-{step_index:02}-narrative.txt")
}

fn step_events_file_name(step_index: u32) -> String {
    format!("step-{step_index:02}-events.ndjson")
}

fn event_kind_label(kind: NarrativeEventKind) -> &'static str {
    match kind {
        NarrativeEventKind::StepOpen => "step_open",
        NarrativeEventKind::PhaseChange => "phase_change",
        NarrativeEventKind::BudgetPulse => "budget_pulse",
        NarrativeEventKind::SurfaceSummary => "surface_summary",
        NarrativeEventKind::PrefixSummary => "prefix_summary",
        NarrativeEventKind::MemoSummary => "memo_summary",
        NarrativeEventKind::FrontierSummary => "frontier_summary",
        NarrativeEventKind::AcceptanceSummary => "acceptance_summary",
        NarrativeEventKind::StepComplete => "step_complete",
    }
}

fn candidate_status_label(status: CandidateStatus) -> &'static str {
    match status {
        CandidateStatus::AcceptedMinimalOvershoot => "accepted",
        CandidateStatus::ClearsBarHigherOvershoot => "bar_clearer",
        CandidateStatus::BelowBar => "below_bar",
    }
}

fn frontier_retention_label(retention: FrontierRetention) -> &'static str {
    match retention {
        FrontierRetention::NotRecorded => "not_recorded",
        FrontierRetention::Hot => "hot",
        FrontierRetention::ColdResident => "resident_cold",
        FrontierRetention::SpillBacked => "spill_backed",
        FrontierRetention::Dropped => "dropped",
    }
}

fn prune_class_label(prune_class: PruneReportClass) -> &'static str {
    match prune_class {
        PruneReportClass::QuotientDedupe => "quotient_dedupe",
        PruneReportClass::SoundMinimality => "sound_minimality",
        PruneReportClass::HeuristicShaping => "heuristic_shaping",
    }
}

#[cfg(test)]
mod tests {
    use super::{render_step_narrative, write_demo_step_artifacts};
    use crate::report::generate_steps_with_config_and_runtime;
    use pen_search::config::RuntimeConfig;
    use pen_search::diversify::FrontierRuntimeLimits;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-cli-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn demo_narrative_renders_budget_and_event_sections() {
        let config = RuntimeConfig::from_toml_str(include_str!(
            "../../../configs/demo_breadth_shadow_10m.toml"
        ))
        .expect("demo config should parse");
        let steps =
            generate_steps_with_config_and_runtime(3, &config, FrontierRuntimeLimits::unlimited())
                .expect("demo steps should build")
                .steps;
        let text = render_step_narrative(steps.last().expect("step should exist"), &config.demo);

        assert!(text.contains("events"));
        assert!(text.contains("closure"));
        assert!(text.contains("demo_funnel"));
        assert!(text.contains("phase_eval"));
        assert!(text.contains("phase_change"));
        assert!(text.contains("scout"));
        assert!(text.contains("retained candidates"));
        assert!(text.contains("accepted trace"));
    }

    #[test]
    fn demo_narrative_writer_persists_text_and_ndjson() {
        let config = RuntimeConfig::from_toml_str(include_str!(
            "../../../configs/demo_breadth_shadow_10m.toml"
        ))
        .expect("demo config should parse");
        let steps =
            generate_steps_with_config_and_runtime(2, &config, FrontierRuntimeLimits::unlimited())
                .expect("demo steps should build")
                .steps;
        let run_dir = temp_dir("demo-narrative");

        write_demo_step_artifacts(&run_dir, &steps, &config)
            .expect("demo artifacts should persist");

        assert!(
            run_dir
                .join("reports")
                .join("steps")
                .join("step-02-narrative.txt")
                .exists()
        );
        assert!(
            run_dir
                .join("reports")
                .join("steps")
                .join("step-02-events.ndjson")
                .exists()
        );

        fs::remove_dir_all(run_dir).ok();
    }
}
