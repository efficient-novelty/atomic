use anyhow::Result;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_search::engine::{search_bootstrap_prefix, supports_live_atomic_search};
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
        .map(|step| StepReport {
            step_index: step.step_index,
            label: step_label(step.step_index).to_owned(),
            objective_bar: step.objective_bar,
            accepted: AcceptedCandidate {
                candidate_hash: step.accepted.candidate_hash.clone(),
                canonical_hash: step.accepted.canonical_hash.clone(),
                bit_kappa: step.accepted.bit_kappa,
                clause_kappa: step.accepted.clause_kappa,
                nu: step.accepted.nu,
                rho: step.accepted.rho,
                overshoot: step.accepted.rho - step.objective_bar,
                shape_fingerprint: step.accepted.shape_fingerprint.clone(),
                support_fingerprint: step.accepted.support_fingerprint.clone(),
            },
            telescope: step.telescope,
            trace: step.accepted.trace,
        })
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

pub fn step_label(step_index: u32) -> &'static str {
    match step_index {
        1 => "Universe",
        2 => "Unit",
        3 => "Witness",
        4 => "Pi",
        5 => "S1",
        6 => "Trunc",
        7 => "S2",
        8 => "S3",
        9 => "Hopf",
        10 => "Cohesion",
        11 => "Connections",
        12 => "Curvature",
        13 => "Metric",
        14 => "Hilbert",
        15 => "DCT",
        _ => "Unknown",
    }
}

pub fn render_standard_report(run_id: &str, steps: &[StepReport]) -> String {
    let Some(last) = steps.last() else {
        return format!("run {run_id}: no accepted steps");
    };

    format!(
        "run {run_id}\ncompleted_step: {}\nlatest: step {} ({})\nnu: {}\nkappa: {}\nrho: {}\nbar: {}",
        last.step_index,
        last.step_index,
        last.label,
        last.accepted.nu,
        last.accepted.clause_kappa,
        last.accepted.rho,
        last.objective_bar
    )
}

pub fn render_debug_report(run_id: &str, steps: &[StepReport]) -> String {
    let mut lines = vec![format!("run {run_id} debug")];
    for step in steps {
        lines.push(format!(
            "step {:02} {:<11} nu={} kappa={} rho={} bar={} candidate={} canonical={}",
            step.step_index,
            step.label,
            step.accepted.nu,
            step.accepted.clause_kappa,
            step.accepted.rho,
            step.objective_bar,
            step.accepted.candidate_hash,
            step.accepted.canonical_hash
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{
        StepGenerationMode, generate_steps, render_debug_report, render_standard_report,
        replay_reference_steps, search_atomic_bootstrap_steps,
    };

    #[test]
    fn replay_reference_steps_matches_expected_sequence_length() {
        let steps = replay_reference_steps(4, 2).expect("reference replay should succeed");
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[3].accepted.nu, 5);
    }

    #[test]
    fn bootstrap_search_matches_the_current_canon_for_eleven_steps() {
        let steps = search_atomic_bootstrap_steps(11, 2).expect("bootstrap search should succeed");
        assert_eq!(steps.len(), 11);
        assert_eq!(
            steps[10].telescope,
            pen_core::telescope::Telescope::reference(11)
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
            StepGenerationMode::ReferenceReplay
        );
    }

    #[test]
    fn report_renderers_include_latest_step_details() {
        let steps = replay_reference_steps(2, 2).expect("reference replay should succeed");
        assert!(render_standard_report("run-1", &steps).contains("latest: step 2 (Unit)"));
        assert!(render_debug_report("run-1", &steps).contains("step 02 Unit"));
    }
}
