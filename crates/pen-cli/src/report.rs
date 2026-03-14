use crate::human::{clause_lines, describe_candidate, library_refs, step_label, translation_guide};
use anyhow::Result;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_search::engine::{AtomicSearchStep, search_bootstrap_prefix, supports_live_atomic_search};
use pen_search::expand::evaluate_candidate;
use pen_store::manifest::AcceptedCandidate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StepReport {
    pub step_index: u32,
    pub label: String,
    pub objective_bar: Rational,
    pub accepted: AcceptedCandidate,
    pub telescope: Telescope,
    pub trace: Vec<String>,
    #[serde(default)]
    pub search_stats: StepSearchStats,
    #[serde(default)]
    pub candidate_reports: Vec<CandidateReport>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StepSearchStats {
    pub evaluated_candidates: usize,
    pub dedupe_prunes: usize,
    pub heuristic_drops: usize,
    pub retained_candidates: usize,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateStatus {
    #[default]
    AcceptedMinimalOvershoot,
    ClearsBarHigherOvershoot,
    BelowBar,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateReport {
    pub status: CandidateStatus,
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub nu: u16,
    pub rho: Rational,
    pub distance_to_bar: Rational,
    pub telescope_class: TelescopeClass,
    pub headline: String,
    pub library_refs: Vec<String>,
    pub clauses: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StepGenerationMode {
    AtomicBootstrapSearch,
    ReferenceReplay,
}

impl StepGenerationMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtomicBootstrapSearch => "atomic_search_bootstrap",
            Self::ReferenceReplay => "reference_replay",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedSteps {
    pub mode: StepGenerationMode,
    pub steps: Vec<StepReport>,
}

pub fn generate_steps(until_step: u32, window_depth: u16) -> Result<GeneratedSteps> {
    if supports_live_atomic_search(until_step) {
        return Ok(GeneratedSteps {
            mode: StepGenerationMode::AtomicBootstrapSearch,
            steps: search_atomic_bootstrap_steps(until_step, window_depth)?,
        });
    }

    Ok(GeneratedSteps {
        mode: StepGenerationMode::ReferenceReplay,
        steps: replay_reference_steps(until_step, window_depth)?,
    })
}

pub fn replay_reference_steps(until_step: u32, window_depth: u16) -> Result<Vec<StepReport>> {
    let mut library: Library = Vec::new();
    let mut history: Vec<DiscoveryRecord> = Vec::new();
    let mut steps = Vec::new();

    for step_index in 1..=until_step.min(15) {
        let telescope = Telescope::reference(step_index);
        let objective_bar = compute_bar(window_depth as usize, step_index, &history).bar;
        let evaluated = evaluate_candidate(&library, &history, telescope.clone())?;
        let accepted = AcceptedCandidate {
            candidate_hash: evaluated.candidate_hash.clone(),
            canonical_hash: evaluated.canonical_hash.clone(),
            bit_kappa: evaluated.bit_kappa,
            clause_kappa: evaluated.clause_kappa,
            nu: evaluated.nu,
            rho: evaluated.rho,
            overshoot: evaluated.rho - objective_bar,
            shape_fingerprint: evaluated.shape_fingerprint.clone(),
            support_fingerprint: evaluated.support_fingerprint.clone(),
        };

        steps.push(StepReport {
            step_index,
            label: step_label(step_index).to_owned(),
            objective_bar,
            accepted,
            telescope: telescope.clone(),
            trace: evaluated.trace.clone(),
            search_stats: StepSearchStats {
                evaluated_candidates: 1,
                dedupe_prunes: 0,
                heuristic_drops: 0,
                retained_candidates: 1,
            },
            candidate_reports: vec![candidate_report(
                objective_bar,
                &evaluated,
                true,
            )],
        });

        history.push(DiscoveryRecord::new(
            step_index,
            u32::from(evaluated.nu),
            u32::from(evaluated.clause_kappa),
        ));
        library.push(LibraryEntry::from_telescope(&telescope, &library));
    }

    Ok(steps)
}

pub fn search_atomic_bootstrap_steps(
    until_step: u32,
    window_depth: u16,
) -> Result<Vec<StepReport>> {
    Ok(search_bootstrap_prefix(until_step, window_depth)?
        .into_iter()
        .map(step_to_report)
        .collect())
}

pub fn write_step_reports(run_dir: &Path, steps: &[StepReport]) -> Result<()> {
    let steps_dir = run_dir.join("reports").join("steps");
    fs::create_dir_all(&steps_dir)?;

    for step in steps {
        let path = steps_dir.join(step_summary_file_name(step.step_index));
        let json = serde_json::to_string_pretty(step)?;
        fs::write(path, format!("{json}\n"))?;
    }

    Ok(())
}

pub fn load_step_reports(run_dir: &Path) -> Result<Vec<StepReport>> {
    let steps_dir = run_dir.join("reports").join("steps");
    if !steps_dir.exists() {
        return Ok(Vec::new());
    }

    let mut reports = Vec::new();
    for entry in fs::read_dir(&steps_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        reports.push(serde_json::from_str::<StepReport>(&text)?);
    }
    reports.sort_by_key(|step| step.step_index);
    Ok(reports)
}

pub fn step_summary_file_name(step_index: u32) -> String {
    format!("step-{step_index:02}-summary.json")
}

pub fn render_standard_report(run_id: &str, steps: &[StepReport]) -> String {
    let Some(last) = steps.last() else {
        return format!("run {run_id}: no accepted steps");
    };

    format!(
        "run {run_id}\ncompleted_step: {}\nlatest: step {} ({})\nnu: {}\nkappa: {}\nrho: {}\nbar: {}\nminimal_overshoot: {}",
        last.step_index,
        last.step_index,
        last.label,
        last.accepted.nu,
        last.accepted.clause_kappa,
        last.accepted.rho,
        last.objective_bar,
        last.accepted.overshoot
    )
}

pub fn render_debug_report(run_id: &str, steps: &[StepReport]) -> String {
    let mut lines = vec![format!("run {run_id} debug"), String::new(), "translation guide:".to_owned()];
    lines.extend(
        translation_guide()
            .into_iter()
            .map(|line| format!("  - {line}")),
    );

    for step in steps {
        lines.push(String::new());
        lines.push(format!(
            "step {:02} {:<11} nu={} kappa={} rho={} bar={} accepted={} canonical={}",
            step.step_index,
            step.label,
            step.accepted.nu,
            step.accepted.clause_kappa,
            step.accepted.rho,
            step.objective_bar,
            step.accepted.candidate_hash,
            step.accepted.canonical_hash
        ));
        lines.push(format!(
            "  accepted minimal overshoot: {}",
            step.accepted.overshoot
        ));
        lines.push(format!(
            "  search stats: evaluated={} dedupe_prunes={} heuristic_drops={} retained_valid={}",
            step.search_stats.evaluated_candidates,
            step.search_stats.dedupe_prunes,
            step.search_stats.heuristic_drops,
            step.search_stats.retained_candidates
        ));

        if step.candidate_reports.is_empty() {
            lines.push("  retained valid candidates: none recorded".to_owned());
            continue;
        }

        lines.push("  retained valid candidates:".to_owned());
        for candidate in &step.candidate_reports {
            lines.push(format!(
                "    - {} | rho={} | nu={} | kappa={} | bits={} | {}",
                candidate_status_heading(candidate.status, candidate.distance_to_bar),
                candidate.rho,
                candidate.nu,
                candidate.clause_kappa,
                candidate.bit_kappa,
                candidate.candidate_hash
            ));
            lines.push(format!("      {}", candidate.headline));
            if candidate.library_refs.is_empty() {
                lines.push("      imports: none".to_owned());
            } else {
                lines.push(format!(
                    "      imports: {}",
                    candidate.library_refs.join(", ")
                ));
            }
            lines.push(format!(
                "      canonical: {} | class: {:?}",
                candidate.canonical_hash, candidate.telescope_class
            ));
            for clause in &candidate.clauses {
                lines.push(format!("      {clause}"));
            }
        }
    }
    lines.join("\n")
}

fn step_to_report(step: AtomicSearchStep) -> StepReport {
    let accepted = AcceptedCandidate {
        candidate_hash: step.accepted.candidate_hash.clone(),
        canonical_hash: step.accepted.canonical_hash.clone(),
        bit_kappa: step.accepted.bit_kappa,
        clause_kappa: step.accepted.clause_kappa,
        nu: step.accepted.nu,
        rho: step.accepted.rho,
        overshoot: step.accepted.rho - step.objective_bar,
        shape_fingerprint: step.accepted.shape_fingerprint.clone(),
        support_fingerprint: step.accepted.support_fingerprint.clone(),
    };
    let mut candidate_reports = step
        .retained_candidates
        .iter()
        .map(|candidate| {
            candidate_report(
                step.objective_bar,
                candidate,
                candidate.candidate_hash == accepted.candidate_hash,
            )
        })
        .collect::<Vec<_>>();
    candidate_reports.sort_by_key(candidate_report_rank);

    StepReport {
        step_index: step.step_index,
        label: step_label(step.step_index).to_owned(),
        objective_bar: step.objective_bar,
        accepted,
        telescope: step.telescope,
        trace: step.accepted.trace,
        search_stats: StepSearchStats {
            evaluated_candidates: step.evaluated_candidates,
            dedupe_prunes: step.dedupe_prunes,
            heuristic_drops: step.heuristic_drops,
            retained_candidates: candidate_reports.len(),
        },
        candidate_reports,
    }
}

fn candidate_report(
    objective_bar: Rational,
    candidate: &pen_search::expand::ExpandedCandidate,
    accepted: bool,
) -> CandidateReport {
    let status = if accepted {
        CandidateStatus::AcceptedMinimalOvershoot
    } else if candidate.rho >= objective_bar {
        CandidateStatus::ClearsBarHigherOvershoot
    } else {
        CandidateStatus::BelowBar
    };
    let distance_to_bar = if candidate.rho >= objective_bar {
        candidate.rho - objective_bar
    } else {
        objective_bar - candidate.rho
    };

    CandidateReport {
        status,
        candidate_hash: candidate.candidate_hash.clone(),
        canonical_hash: candidate.canonical_hash.clone(),
        bit_kappa: candidate.bit_kappa,
        clause_kappa: candidate.clause_kappa,
        nu: candidate.nu,
        rho: candidate.rho,
        distance_to_bar,
        telescope_class: candidate.telescope_class,
        headline: describe_candidate(&candidate.telescope, candidate.telescope_class),
        library_refs: library_refs(&candidate.telescope),
        clauses: clause_lines(&candidate.telescope),
    }
}

fn candidate_report_rank(candidate: &CandidateReport) -> (u8, Rational, u16, std::cmp::Reverse<u16>, String) {
    let group = match candidate.status {
        CandidateStatus::AcceptedMinimalOvershoot => 0,
        CandidateStatus::ClearsBarHigherOvershoot => 1,
        CandidateStatus::BelowBar => 2,
    };
    (
        group,
        candidate.distance_to_bar,
        candidate.clause_kappa,
        std::cmp::Reverse(candidate.nu),
        candidate.canonical_hash.clone(),
    )
}

fn candidate_status_heading(status: CandidateStatus, distance_to_bar: Rational) -> String {
    match status {
        CandidateStatus::AcceptedMinimalOvershoot => {
            format!("ACCEPTED clears bar with minimal overshoot {distance_to_bar}")
        }
        CandidateStatus::ClearsBarHigherOvershoot => {
            format!("ALSO CLEARS bar with overshoot {distance_to_bar}")
        }
        CandidateStatus::BelowBar => format!("BELOW BAR by {distance_to_bar}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CandidateStatus, StepGenerationMode, generate_steps, render_debug_report,
        render_standard_report, replay_reference_steps, search_atomic_bootstrap_steps,
    };

    #[test]
    fn replay_reference_steps_matches_expected_sequence_length() {
        let steps = replay_reference_steps(4, 2).expect("reference replay should succeed");
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[3].accepted.nu, 5);
    }

    #[test]
    fn bootstrap_search_matches_the_current_canon_for_fifteen_steps() {
        let steps = search_atomic_bootstrap_steps(15, 2).expect("bootstrap search should succeed");
        assert_eq!(steps.len(), 15);
        assert_eq!(
            steps[14].telescope,
            pen_core::telescope::Telescope::reference(15)
        );
    }

    #[test]
    fn step_generation_prefers_live_search_only_where_supported() {
        assert_eq!(
            generate_steps(3, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(4, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(5, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(6, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(7, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(8, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(9, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(10, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(11, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(12, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(13, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(14, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
        assert_eq!(
            generate_steps(15, 2).expect("generate steps").mode,
            StepGenerationMode::AtomicBootstrapSearch
        );
    }

    #[test]
    fn report_renderers_include_latest_step_details() {
        let steps = replay_reference_steps(2, 2).expect("reference replay should succeed");
        assert!(render_standard_report("run-1", &steps).contains("latest: step 2 (Unit)"));
        assert!(render_standard_report("run-1", &steps).contains("minimal_overshoot:"));
        assert!(render_debug_report("run-1", &steps).contains("translation guide:"));
        assert!(render_debug_report("run-1", &steps).contains("ACCEPTED clears bar with minimal overshoot"));
    }

    #[test]
    fn live_search_reports_candidate_breakdowns() {
        let steps = search_atomic_bootstrap_steps(4, 2).expect("bootstrap search should succeed");
        let debug = render_debug_report("run-1", &steps);

        assert!(steps[3]
            .candidate_reports
            .iter()
            .any(|candidate| candidate.status == CandidateStatus::AcceptedMinimalOvershoot));
        assert!(debug.contains("retained valid candidates:"));
        assert!(debug.contains("class: Former"));
        assert!(debug.contains("c01 [introduction]"));
        assert!(debug.contains("fun x1 ->"));
    }
}
