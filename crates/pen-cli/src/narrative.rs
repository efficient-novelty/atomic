use crate::progress::{
    format_millis, format_seconds, render_bar, render_goal_line, render_limit_line,
};
use crate::report::{
    CandidateStatus, FrontierRetention, PruneReportClass, StepReport, stored_prune_class_stats,
};
use anyhow::Result;
use pen_search::config::{DemoConfig, RuntimeConfig, SearchProfile};
use pen_search::narrative::{NarrativeEvent, NarrativeEventKind, NarrativeProgressSnapshot};
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct NarrativeOutputConfig<'a> {
    pub demo: &'a DemoConfig,
    pub search_profile: SearchProfile,
}

impl<'a> NarrativeOutputConfig<'a> {
    pub fn from_runtime(config: &'a RuntimeConfig) -> Self {
        Self {
            demo: &config.demo,
            search_profile: config.mode.search_profile,
        }
    }

    pub const fn lane_label(self) -> &'static str {
        self.search_profile.narrative_label()
    }
}

pub fn write_step_narrative_artifacts(
    run_dir: &Path,
    steps: &[StepReport],
    config: &RuntimeConfig,
) -> Result<()> {
    if !config.mode.search_profile.supports_narrative_artifacts() || !config.demo.enabled {
        return Ok(());
    }

    let narrative = NarrativeOutputConfig::from_runtime(config);
    let steps_dir = run_dir.join("reports").join("steps");
    fs::create_dir_all(&steps_dir)?;

    for step in steps {
        fs::write(
            steps_dir.join(step_narrative_file_name(step.step_index)),
            format!("{}\n", render_step_narrative(step, narrative)),
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

pub fn render_step_narrative(step: &StepReport, narrative: NarrativeOutputConfig<'_>) -> String {
    let demo = narrative.demo;
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
        format!(
            "step {:02} {} narrative",
            step.step_index,
            narrative.lane_label()
        ),
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
        closure_line(&closure, demo),
        demo_phase_line(step),
        proof_close_line(step),
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
    let exact_screen_reasons = stored_exact_screen_reasons(step);
    let prune_classes = stored_prune_class_stats(step);
    if !exact_screen_reasons.is_empty() {
        lines.push(format!(
            "exact_reasons partial_prefix_bar={} terminal_completion={} incumbent_dominance={} legality_connectivity={}",
            exact_screen_reasons.partial_prefix_bar_failure,
            exact_screen_reasons.terminal_prefix_completion_failure,
            exact_screen_reasons.incumbent_dominance,
            exact_screen_reasons.legality_connectivity_exact_rejection
        ));
    }
    lines.push(format!(
        "prune_totals quotient_dedupe={} sound_minimality={} heuristic_shaping={}",
        prune_classes.quotient_dedupe,
        prune_classes.sound_minimality,
        prune_classes.heuristic_shaping
    ));

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
    if !step.search_stats.demo_bucket_stats.is_empty() {
        lines.push(format!(
            "demo_buckets {}",
            render_demo_bucket_summary(&step.search_stats.demo_bucket_stats)
        ));
    }

    let mut sections = Vec::new();

    let mut event_lines = Vec::new();
    for event in &events {
        let phase = event
            .phase
            .map(|phase| format!(" {}", phase.as_str()))
            .unwrap_or_default();
        event_lines.push(format!(
            "  [{}{}] {}",
            event_kind_label(event.kind),
            phase,
            event.message
        ));
        if let Some(detail) = &event.detail {
            event_lines.push(format!("    {detail}"));
        }
    }
    sections.push(NarrativeSection::new("events", event_lines, "event lines"));

    let retained_lines = if step.candidate_reports.is_empty() {
        vec!["  none".to_owned()]
    } else {
        let mut lines = step
            .candidate_reports
            .iter()
            .take(8)
            .map(|candidate| {
                format!(
                    "  {} {} rho={} distance={} frontier={} headline={}",
                    candidate_status_label(candidate.status),
                    candidate.candidate_hash,
                    candidate.rho,
                    candidate.distance_to_bar,
                    frontier_retention_label(candidate.frontier_retention),
                    candidate.headline
                )
            })
            .collect::<Vec<_>>();
        if step.candidate_reports.len() > 8 {
            lines.push(format!(
                "  ... {} more retained candidates",
                step.candidate_reports.len() - 8
            ));
        }
        lines
    };
    sections.push(NarrativeSection::new(
        "retained candidates",
        retained_lines,
        "retained candidate lines",
    ));

    let prune_lines = {
        let mut lines = vec![format!(
            "  totals quotient_dedupe={} sound_minimality={} heuristic_shaping={}",
            prune_classes.quotient_dedupe,
            prune_classes.sound_minimality,
            prune_classes.heuristic_shaping
        )];
        if step.prune_reports.is_empty() {
            lines.push("  sampled: none".to_owned());
        } else {
            lines.push("  sampled:".to_owned());
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
        lines
    };
    sections.push(NarrativeSection::new("prunes", prune_lines, "prune lines"));

    let trace_lines = if step.trace.is_empty() {
        vec!["  none".to_owned()]
    } else {
        step.trace
            .iter()
            .map(|trace| format!("  {trace}"))
            .collect()
    };
    sections.push(NarrativeSection::new(
        "accepted trace",
        trace_lines,
        "trace lines",
    ));

    apply_step_line_budget(lines, sections, demo).join("\n")
}

pub fn render_run_narrative(steps: &[StepReport], narrative: NarrativeOutputConfig<'_>) -> String {
    if steps.is_empty() {
        return format!("{} narrative\n  none", narrative.lane_label());
    }

    let mut sections = vec![format!("{} narrative", narrative.lane_label())];
    sections.extend(
        steps
            .iter()
            .map(|step| render_step_narrative(step, narrative)),
    );
    sections.join("\n\n")
}

#[derive(Clone, Debug)]
struct NarrativeSection {
    title: &'static str,
    lines: Vec<String>,
    omission_label: &'static str,
    omitted_lines: usize,
}

impl NarrativeSection {
    fn new(title: &'static str, lines: Vec<String>, omission_label: &'static str) -> Self {
        Self {
            title,
            lines,
            omission_label,
            omitted_lines: 0,
        }
    }

    fn rendered_line_count(&self) -> usize {
        2 + self.lines.len()
    }

    fn trim_one_line(&mut self) -> bool {
        if self.lines.len() <= 1 {
            return false;
        }
        self.lines.pop();
        self.omitted_lines += 1;
        true
    }

    fn finalize_omission_line(&mut self) {
        if self.omitted_lines == 0 {
            return;
        }
        let summary = format!("  ... {} more {}", self.omitted_lines, self.omission_label);
        if let Some(last) = self.lines.last_mut() {
            *last = summary;
        } else {
            self.lines.push(summary);
        }
    }
}

fn apply_step_line_budget(
    header_lines: Vec<String>,
    mut sections: Vec<NarrativeSection>,
    demo: &DemoConfig,
) -> Vec<String> {
    let max_lines = usize::from(
        demo.narrative
            .max_lines_per_step
            .max(demo.narrative.min_lines_per_step),
    );
    if max_lines == 0 {
        return header_lines;
    }

    let mut total_lines = header_lines.len()
        + sections
            .iter()
            .map(NarrativeSection::rendered_line_count)
            .sum::<usize>();

    // Trim the least critical trailing detail first so the per-step narrative
    // stays within the configured line budget without dropping the headline.
    let trim_order = [3usize, 2, 1, 0];
    while total_lines > max_lines {
        let mut trimmed = false;
        for section_index in trim_order {
            while total_lines > max_lines && sections[section_index].trim_one_line() {
                total_lines -= 1;
                trimmed = true;
            }
        }
        if !trimmed {
            break;
        }
    }

    for section in &mut sections {
        section.finalize_omission_line();
    }

    let mut lines = header_lines;
    for section in sections {
        lines.push(String::new());
        lines.push(section.title.to_owned());
        lines.extend(section.lines);
    }
    lines
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

fn stored_exact_screen_reasons(step: &StepReport) -> pen_search::engine::ExactScreenReasonStats {
    if !step.search_stats.exact_screen_reasons.is_empty() {
        return step.search_stats.exact_screen_reasons.clone();
    }

    pen_search::engine::ExactScreenReasonStats::from_incremental_counters(
        step.search_stats.incremental_connectivity_prunes,
        step.search_stats.incremental_terminal_clause_filter_prunes,
        step.search_stats.incremental_terminal_rank_prunes,
        step.search_stats.incremental_partial_prefix_bound_prunes,
        step.search_stats.incremental_terminal_prefix_bar_prunes,
    )
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

fn proof_close_line(step: &StepReport) -> String {
    let phase = &step.search_stats.demo_phase;
    format!(
        "proof_close  reserve={} elapsed={} remaining={} reserve_overrun={} exhausted={} groups_closed={}/{} groups_remaining={} closure={}%",
        format_millis(phase.proof_close_reserved_millis),
        format_millis(phase.proof_close_elapsed_millis),
        format_millis(phase.proof_close_remaining_millis),
        format_millis(phase.proof_close_reserve_overrun_millis),
        phase.proof_close_reserve_exhausted,
        phase.proof_close_frontier_groups_closed,
        phase.proof_close_frontier_total_groups,
        phase.proof_close_frontier_groups_remaining,
        phase.proof_close_closure_percent
    )
}

fn render_demo_bucket_summary(buckets: &[pen_search::engine::DemoBucketReport]) -> String {
    let mut buckets = buckets.to_vec();
    buckets.sort_by_key(|bucket| {
        (
            std::cmp::Reverse(bucket.stats.exact_screened_terminal_candidates),
            std::cmp::Reverse(bucket.stats.fully_scored_terminal_candidates),
            bucket.bucket_label.clone(),
        )
    });
    let bucket_count = buckets.len();

    let mut rendered = buckets
        .into_iter()
        .take(3)
        .map(|bucket| {
            format!(
                "{} gen={} adm={} screen={} pruned={} scored={} best={}",
                bucket.bucket_label,
                bucket.stats.generated_terminal_candidates,
                bucket.stats.admissible_terminal_candidates,
                bucket.stats.exact_screened_terminal_candidates,
                bucket.stats.pruned_terminal_candidates,
                bucket.stats.fully_scored_terminal_candidates,
                bucket
                    .stats
                    .best_overshoot
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "none".to_owned())
            )
        })
        .collect::<Vec<_>>();
    if bucket_count > 3 {
        rendered.push(format!("... {} more", bucket_count - 3));
    }
    rendered.join(" | ")
}

fn time_line(step_index: u32, elapsed_millis: u64, demo: &DemoConfig) -> String {
    let (budget, note) = if step_index <= 4 {
        (
            Some(u64::from(demo.early_exhaustive_budget_sec)),
            Some("shared early window across steps 1-4"),
        )
    } else {
        (
            demo.floors
                .step_floor_sec
                .get(&step_index.to_string())
                .copied()
                .map(u64::from),
            None,
        )
    };
    let Some(budget_seconds) = budget else {
        return format!("time         {}", format_millis(elapsed_millis));
    };
    let budget_millis = budget_seconds * 1_000;
    let status = if elapsed_millis <= budget_millis {
        "within limit"
    } else {
        "over limit"
    };
    let note_suffix = note.map(|value| format!("; {value}")).unwrap_or_default();
    if demo.narrative.show_progress_bar {
        return format!(
            "time         [{}] {} / {} ({status}{note_suffix})",
            render_bar(elapsed_millis.min(budget_millis), budget_millis),
            format_millis(elapsed_millis),
            format_seconds(u32::try_from(budget_seconds).expect("budget seconds exceeded u32"))
        );
    }

    format!(
        "time         {} / {} ({status}{note_suffix})",
        format_millis(elapsed_millis),
        format_seconds(u32::try_from(budget_seconds).expect("budget seconds exceeded u32"))
    )
}

fn closure_line(closure: &pen_search::engine::DemoClosureStats, demo: &DemoConfig) -> String {
    if demo.narrative.show_progress_bar {
        return format!(
            "closure      [{}] {}% frontier_total={} certified_nonwinning={}",
            render_bar(u64::from(closure.closure_percent), 100),
            closure.closure_percent,
            closure.frontier_total_seen,
            closure.frontier_certified_nonwinning
        );
    }

    format!(
        "closure      {}% frontier_total={} certified_nonwinning={}",
        closure.closure_percent, closure.frontier_total_seen, closure.frontier_certified_nonwinning
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
    use super::{NarrativeOutputConfig, render_step_narrative, write_step_narrative_artifacts};
    use crate::report::generate_steps_with_config_and_runtime;
    use pen_search::config::RuntimeConfig;
    use pen_search::diversify::FrontierRuntimeLimits;
    use pen_search::narrative::NarrativeEventKind;
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
        let text = render_step_narrative(
            steps.last().expect("step should exist"),
            NarrativeOutputConfig::from_runtime(&config),
        );

        assert!(text.contains("events"));
        assert!(text.contains("closure"));
        assert!(text.contains("closure      ["));
        assert!(text.contains("demo_funnel"));
        assert!(text.contains("phase_eval"));
        assert!(text.contains("proof_close  reserve="));
        assert!(text.contains("time         ["));
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

        write_step_narrative_artifacts(&run_dir, &steps, &config)
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

    #[test]
    fn demo_narrative_respects_the_configured_step_line_budget() {
        let mut config = RuntimeConfig::from_toml_str(include_str!(
            "../../../configs/demo_breadth_shadow_10m.toml"
        ))
        .expect("demo config should parse");
        config.demo.narrative.max_lines_per_step = 40;
        let mut steps =
            generate_steps_with_config_and_runtime(3, &config, FrontierRuntimeLimits::unlimited())
                .expect("demo steps should build")
                .steps;
        let mut step = steps.pop().expect("step should exist");
        let last_event = step
            .narrative_events
            .last()
            .cloned()
            .expect("demo step should have events");
        for offset in 0..24_u16 {
            let mut event = last_event.clone();
            event.ordinal = 200 + offset;
            event.kind = NarrativeEventKind::BudgetPulse;
            event.message = format!("extra live pulse {offset}");
            event.detail = Some(format!("detail {offset}"));
            step.narrative_events.push(event);
        }
        step.trace = (0..40).map(|index| format!("trace line {index}")).collect();

        let text = render_step_narrative(&step, NarrativeOutputConfig::from_runtime(&config));

        assert!(
            text.lines().count() <= usize::from(config.demo.narrative.max_lines_per_step),
            "expected narrative to stay within {} lines, got {}",
            config.demo.narrative.max_lines_per_step,
            text.lines().count()
        );
        assert!(text.contains("more event lines"));
        assert!(text.contains("more trace lines"));
    }

    #[test]
    fn demo_narrative_surfaces_exact_screen_reason_codes() {
        let config = RuntimeConfig::from_toml_str(include_str!(
            "../../../configs/demo_breadth_shadow_10m.toml"
        ))
        .expect("demo config should parse");
        let mut steps =
            generate_steps_with_config_and_runtime(15, &config, FrontierRuntimeLimits::unlimited())
                .expect("demo steps should build")
                .steps;
        let mut step = steps.pop().expect("step should exist");
        step.search_stats.exact_screen_reasons =
            pen_search::engine::ExactScreenReasonStats::default();

        let text = render_step_narrative(&step, NarrativeOutputConfig::from_runtime(&config));

        assert!(text.contains("exact_reasons"));
        assert!(text.contains("partial_prefix_bar="));
        assert!(text.contains("incumbent_dominance="));
        assert!(text.contains("prune_totals"));
        assert!(text.contains("sound_minimality="));
    }

    #[test]
    fn claim_narrative_uses_claim_headline() {
        let config = RuntimeConfig::from_toml_str(include_str!(
            "../../../configs/desktop_claim_shadow_smoke.toml"
        ))
        .expect("claim config should parse");
        let steps =
            generate_steps_with_config_and_runtime(2, &config, FrontierRuntimeLimits::unlimited())
                .expect("claim steps should build")
                .steps;

        let text = render_step_narrative(
            steps.last().expect("step should exist"),
            NarrativeOutputConfig::from_runtime(&config),
        );

        assert!(text.contains("claim narrative"));
        assert!(!text.contains("demo narrative"));
    }
}
