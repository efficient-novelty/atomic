use crate::report::{
    LateStepClaim, LateStepClaimStatus, StepProvenance, StepReport, load_step_reports,
};
use anyhow::{Context, Result};
use pen_search::config::SearchProfile;
use pen_search::engine::{
    ClaimAnchorPolicyDiagnostics, ClaimDebtAxesDiagnostics, ClaimPackageFlags,
    ClaimRootSeedingDiagnostics, ClaimStepOpenDiagnostics, ExactScreenReasonStats,
    StepLiveCheckpoint,
};
use pen_store::manifest::{RunManifestV1, SearchPolicyInfo};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

pub const CLAIM_REQUIRED_COMPLETED_STEP: u32 = 15;
pub const EARLY_GENERATED_TARGETS: &[(u32, usize)] = &[(1, 2144)];
pub const LATE_GENERATED_FLOORS: &[(u32, usize)] = &[
    (10, 500),
    (11, 800),
    (12, 1200),
    (13, 2200),
    (14, 3500),
    (15, 5000),
];

const CLAIM_REGULAR_CATALOG_NOTE: &str = "claim_regular_clause_catalog";
const CLAIM_ROOT_SEEDING_NOTE: &str = "claim_root_seeding_summary";
const CLAIM_EARLY_EXHAUSTIVE_NOTE: &str = "claim_early_exhaustive_catalog";
const CLAIM_ALLOWED_RUN_MODES: &[&str] = &[
    "atomic_search_bootstrap",
    "frontier_checkpoint_resume",
    "step_checkpoint_resume",
    "step_checkpoint_reevaluate",
];

#[derive(Clone, Debug)]
pub struct LoadedRun {
    pub run_dir: PathBuf,
    pub manifest: RunManifestV1,
    pub steps: Vec<StepReport>,
    pub run_mode: String,
    pub live_checkpoints: BTreeMap<u32, Vec<StepLiveCheckpoint>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CompareFieldStatus {
    pub status: String,
    pub detail: String,
    pub diverging_steps: Vec<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Step15ClaimBoundaryStatus {
    pub status: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClaimCompareLane {
    pub label: String,
    pub run_dir: String,
    pub run_id: String,
    pub search_profile: String,
    pub run_mode: String,
    pub completed_step: u32,
    pub trajectory: CompareFieldStatus,
    pub trajectory_fingerprint: String,
    pub accepted_hashes: CompareFieldStatus,
    pub search_space_counts: CompareFieldStatus,
    pub admissibility_diagnostics: CompareFieldStatus,
    pub late_step_competition: CompareFieldStatus,
    pub provenance_sequence: String,
    pub replay_ablation_sequence: String,
    pub demo_phase_latest: String,
    pub demo_funnel_latest: String,
    pub demo_buckets_latest: String,
    pub step15_claim: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClaimCompareSummary {
    pub signoff_status: String,
    pub baseline_label: String,
    pub baseline_run: String,
    pub claim_run: String,
    pub trajectory: CompareFieldStatus,
    pub accepted_hashes: CompareFieldStatus,
    pub search_space_counts: CompareFieldStatus,
    pub admissibility_diagnostics: CompareFieldStatus,
    pub late_step_competition: CompareFieldStatus,
    pub step15_claim_boundary: Step15ClaimBoundaryStatus,
    pub claim_lane_audit: ClaimLaneAudit,
    pub lanes: Vec<ClaimCompareLane>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AcceptedHashParity {
    pub status: String,
    pub matches_baseline: bool,
    pub lane_missing_steps: Vec<u32>,
    pub baseline_missing_steps: Vec<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SearchPolicyAudit {
    pub status: String,
    pub expected: SearchPolicySnapshot,
    pub actual: SearchPolicySnapshot,
    pub mismatches: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SearchPolicySnapshot {
    pub guidance_style: String,
    pub late_expansion_policy: String,
    pub bucket_policy: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CoverageSummary {
    pub status: String,
    pub expected_steps: u32,
    pub present_steps: Vec<u32>,
    pub missing_steps: Vec<u32>,
    pub totals: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NarrativeArtifacts {
    pub status: String,
    pub expected_steps: u32,
    pub present_narrative_steps: usize,
    pub present_event_steps: usize,
    pub missing_narrative_steps: Vec<u32>,
    pub missing_event_steps: Vec<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FallbackHonesty {
    pub status: String,
    pub run_mode: String,
    pub run_mode_fallback: bool,
    pub resume_steps: Vec<u32>,
    pub reference_replay_steps: Vec<u32>,
    pub unexpected_provenance: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClaimLaneAudit {
    pub status: String,
    pub reasons: Vec<String>,
    pub accepted_hash_parity: AcceptedHashParity,
    pub search_policy: SearchPolicyAudit,
    pub exact_screen_reasons: CoverageSummary,
    pub prune_classes: CoverageSummary,
    pub fallback_honesty: FallbackHonesty,
    pub narrative_artifacts: NarrativeArtifacts,
}

#[derive(Clone, Debug, Serialize)]
pub struct BreadthSearchSpace {
    pub well_formed_candidates: usize,
    pub exact_bound_screened: usize,
    pub retained_candidates: usize,
    pub heuristic_dropped: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct BreadthDiagnosis {
    pub source: String,
    pub summary: String,
    pub raw_catalog_telescope_count: Option<usize>,
    pub raw_catalog_clause_widths: Vec<usize>,
    pub claim_root_seeding: ClaimRootSeedingDiagnostics,
    pub claim_step_open: Option<ClaimStepOpenDiagnostics>,
    pub search_space: BreadthSearchSpace,
    pub exact_screen_reasons: ExactScreenReasonStats,
}

#[derive(Clone, Debug, Serialize)]
pub struct BreadthStepResult {
    pub step_index: u32,
    pub target: usize,
    pub actual: Option<usize>,
    pub status: String,
    pub gap_to_target: Option<i64>,
    pub diagnosis: BreadthDiagnosis,
}

#[derive(Clone, Debug, Serialize)]
pub struct BreadthCheck {
    pub status: String,
    pub detail: String,
    pub steps: Vec<BreadthStepResult>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RuntimeThresholdCheck {
    pub status: String,
    pub detail: String,
    pub total_runtime_ms: u64,
    pub threshold_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ManifestCompletenessCheck {
    pub status: String,
    pub detail: String,
    pub missing_fields: Vec<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
struct RawTelemetryEvent {
    #[serde(default)]
    event: String,
    #[serde(default)]
    payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct TrajectoryEntry {
    step_index: u32,
    label: String,
    nu: u16,
    clause_kappa: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct AcceptedHashEntry {
    step_index: u32,
    candidate_hash: String,
    canonical_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct SearchSpaceEntry {
    step_index: u32,
    enumerated: usize,
    well_formed: usize,
    admissibility_rejected: usize,
    evaluated: usize,
    canonical: usize,
    semantically_minimal: usize,
    retained: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct AdmissibilityEntry {
    step_index: u32,
    exact_legality_rejections: usize,
    structural_debt_cap_rejections: usize,
    admitted_deprioritized: usize,
    admitted_focus_aligned: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct LateCompetitionEntry {
    step_index: u32,
    evaluated: usize,
    clears_bar: usize,
    below_bar: usize,
    retained: usize,
    terminal_rank_prunes: usize,
}

trait StepIndexed {
    fn step_index(&self) -> u32;
}

impl StepIndexed for TrajectoryEntry {
    fn step_index(&self) -> u32 {
        self.step_index
    }
}

impl StepIndexed for AcceptedHashEntry {
    fn step_index(&self) -> u32 {
        self.step_index
    }
}

impl StepIndexed for SearchSpaceEntry {
    fn step_index(&self) -> u32 {
        self.step_index
    }
}

impl StepIndexed for AdmissibilityEntry {
    fn step_index(&self) -> u32 {
        self.step_index
    }
}

impl StepIndexed for LateCompetitionEntry {
    fn step_index(&self) -> u32 {
        self.step_index
    }
}

pub fn load_run(run_dir: &Path) -> Result<LoadedRun> {
    let manifest: RunManifestV1 = load_json_file(&run_dir.join("run.json"))
        .with_context(|| format!("load manifest from {}", run_dir.display()))?;
    let steps = load_step_reports(run_dir)
        .with_context(|| format!("load step reports from {}", run_dir.display()))?;
    let telemetry: Vec<RawTelemetryEvent> = load_ndjson_file(&run_dir.join("telemetry.ndjson"))
        .with_context(|| {
            format!(
                "load telemetry from {}",
                run_dir.join("telemetry.ndjson").display()
            )
        })?;
    let run_mode = telemetry
        .iter()
        .find(|event| event.event == "run_started")
        .and_then(|event| event.payload.get("mode"))
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_owned();

    let live_checkpoints = steps
        .iter()
        .map(|step| {
            let checkpoints =
                load_ndjson_file::<StepLiveCheckpoint>(&step_live_path(run_dir, step.step_index))
                    .unwrap_or_default();
            (step.step_index, checkpoints)
        })
        .collect::<BTreeMap<_, _>>();

    Ok(LoadedRun {
        run_dir: run_dir.to_path_buf(),
        manifest,
        steps,
        run_mode,
        live_checkpoints,
    })
}

pub fn build_claim_compare_summary(guarded: &LoadedRun, claim: &LoadedRun) -> ClaimCompareSummary {
    let claim_audit = build_claim_lane_audit(guarded, claim);
    let guarded_lane = compare_lane("guarded", guarded, guarded);
    let claim_lane = compare_lane("claim", claim, guarded);

    let trajectory = top_level_status(
        "all 2 lanes match baseline guarded",
        "mismatches detected in claim",
        claim_lane.trajectory.diverging_steps.clone(),
    );
    let accepted_hashes = top_level_status(
        "all 2 lanes match baseline guarded",
        "mismatches detected in claim",
        claim_lane.accepted_hashes.diverging_steps.clone(),
    );
    let search_space_counts = top_level_status(
        "all 2 lanes match baseline guarded",
        "mismatches detected in claim",
        claim_lane.search_space_counts.diverging_steps.clone(),
    );
    let admissibility_diagnostics = top_level_status(
        "all 2 lanes match baseline guarded",
        "mismatches detected in claim",
        claim_lane.admissibility_diagnostics.diverging_steps.clone(),
    );
    let late_step_competition = top_level_status(
        "all 2 lanes match baseline guarded",
        "mismatches detected in claim",
        claim_lane.late_step_competition.diverging_steps.clone(),
    );
    let step15_claim_boundary = compare_step15_claim_boundary(guarded, claim);
    let signoff_status = if trajectory.status == "matches"
        && accepted_hashes.status == "matches"
        && step15_claim_boundary.status == "consistent"
        && claim_audit.status == "ready"
    {
        "ready"
    } else {
        "attention"
    }
    .to_owned();

    ClaimCompareSummary {
        signoff_status,
        baseline_label: "guarded".to_owned(),
        baseline_run: guarded.run_dir.display().to_string(),
        claim_run: claim.run_dir.display().to_string(),
        trajectory,
        accepted_hashes,
        search_space_counts,
        admissibility_diagnostics,
        late_step_competition,
        step15_claim_boundary,
        claim_lane_audit: claim_audit,
        lanes: vec![guarded_lane, claim_lane],
    }
}

pub fn build_claim_lane_audit(guarded: &LoadedRun, claim: &LoadedRun) -> ClaimLaneAudit {
    let accepted_hash_parity = accepted_hash_parity_check(guarded, claim);
    let search_policy = search_policy_check(&claim.manifest.search_policy);
    let exact_screen_reasons = exact_screen_reason_coverage(&claim.steps);
    let prune_classes = prune_class_coverage(&claim.steps);
    let fallback_honesty = fallback_honesty_check(claim);
    let narrative_artifacts = narrative_artifacts_check(claim);

    let mut reasons = Vec::new();
    if accepted_hash_parity.status != "ready" {
        reasons.push("accepted_hash_parity_through_step_15_open".to_owned());
    }
    if search_policy.status != "honest" {
        reasons.push("policy_mismatch".to_owned());
    }
    if exact_screen_reasons.status != "complete" {
        reasons.push("exact_screen_reason_coverage_incomplete".to_owned());
    }
    if prune_classes.status != "complete" {
        reasons.push("prune_class_coverage_incomplete".to_owned());
    }
    if fallback_honesty.status != "clear" {
        reasons.push("fallback_evidence_detected".to_owned());
    }
    if narrative_artifacts.status != "complete" {
        reasons.push("narrative_artifacts_incomplete".to_owned());
    }

    ClaimLaneAudit {
        status: if reasons.is_empty() {
            "ready".to_owned()
        } else {
            "attention".to_owned()
        },
        reasons,
        accepted_hash_parity,
        search_policy,
        exact_screen_reasons,
        prune_classes,
        fallback_honesty,
        narrative_artifacts,
    }
}

pub fn accepted_hash_parity_check(guarded: &LoadedRun, claim: &LoadedRun) -> AcceptedHashParity {
    let guarded_hashes = accepted_hash_map(&guarded.steps);
    let claim_hashes = accepted_hash_map(&claim.steps);

    let lane_missing_steps = (1..=CLAIM_REQUIRED_COMPLETED_STEP)
        .filter(|step| guarded_hashes.contains_key(step) && !claim_hashes.contains_key(step))
        .collect::<Vec<_>>();
    let baseline_missing_steps = (1..=CLAIM_REQUIRED_COMPLETED_STEP)
        .filter(|step| claim_hashes.contains_key(step) && !guarded_hashes.contains_key(step))
        .collect::<Vec<_>>();
    let matches_baseline = (1..=CLAIM_REQUIRED_COMPLETED_STEP)
        .all(|step| guarded_hashes.get(&step) == claim_hashes.get(&step));

    AcceptedHashParity {
        status: if matches_baseline
            && lane_missing_steps.is_empty()
            && baseline_missing_steps.is_empty()
        {
            "ready".to_owned()
        } else {
            "attention".to_owned()
        },
        matches_baseline,
        lane_missing_steps,
        baseline_missing_steps,
    }
}

pub fn search_policy_check(policy: &SearchPolicyInfo) -> SearchPolicyAudit {
    let expected = SearchPolicySnapshot {
        guidance_style: "claim_debt_guided".to_owned(),
        late_expansion_policy: "claim_generic".to_owned(),
        bucket_policy: "structural_generic".to_owned(),
    };
    let actual = SearchPolicySnapshot {
        guidance_style: policy.guidance_style.clone(),
        late_expansion_policy: policy.late_expansion_policy.clone(),
        bucket_policy: policy.bucket_policy.clone(),
    };
    let mut mismatches = Vec::new();
    if policy.guidance_style != expected.guidance_style {
        mismatches.push(format!(
            "guidance_style expected {} actual {}",
            expected.guidance_style, policy.guidance_style
        ));
    }
    if policy.late_expansion_policy != expected.late_expansion_policy {
        mismatches.push(format!(
            "late_expansion_policy expected {} actual {}",
            expected.late_expansion_policy, policy.late_expansion_policy
        ));
    }
    if policy.bucket_policy != expected.bucket_policy {
        mismatches.push(format!(
            "bucket_policy expected {} actual {}",
            expected.bucket_policy, policy.bucket_policy
        ));
    }

    SearchPolicyAudit {
        status: if mismatches.is_empty() {
            "honest".to_owned()
        } else {
            "mismatch".to_owned()
        },
        expected,
        actual,
        mismatches,
    }
}

pub fn exact_screen_reason_coverage(steps: &[StepReport]) -> CoverageSummary {
    let present_steps = steps.iter().map(|step| step.step_index).collect::<Vec<_>>();
    let missing_steps = expected_missing_steps(steps);
    let totals = steps.iter().fold(BTreeMap::new(), |mut totals, step| {
        let reasons = &step.search_stats.exact_screen_reasons;
        add_total(
            &mut totals,
            "partial_prefix_bar_failure",
            reasons.partial_prefix_bar_failure,
        );
        add_total(
            &mut totals,
            "terminal_prefix_completion_failure",
            reasons.terminal_prefix_completion_failure,
        );
        add_total(
            &mut totals,
            "incumbent_dominance",
            reasons.incumbent_dominance,
        );
        add_total(
            &mut totals,
            "legality_connectivity_exact_rejection",
            reasons.legality_connectivity_exact_rejection,
        );
        totals
    });

    CoverageSummary {
        status: if missing_steps.is_empty() {
            "complete".to_owned()
        } else {
            "incomplete".to_owned()
        },
        expected_steps: CLAIM_REQUIRED_COMPLETED_STEP,
        present_steps,
        missing_steps,
        totals,
    }
}

pub fn prune_class_coverage(steps: &[StepReport]) -> CoverageSummary {
    let present_steps = steps.iter().map(|step| step.step_index).collect::<Vec<_>>();
    let missing_steps = expected_missing_steps(steps);
    let totals = steps.iter().fold(BTreeMap::new(), |mut totals, step| {
        add_total(
            &mut totals,
            "quotient_dedupe",
            step.search_stats.prune_classes.quotient_dedupe,
        );
        add_total(
            &mut totals,
            "sound_minimality",
            step.search_stats.prune_classes.sound_minimality,
        );
        add_total(
            &mut totals,
            "heuristic_shaping",
            step.search_stats.prune_classes.heuristic_shaping,
        );
        totals
    });

    CoverageSummary {
        status: if missing_steps.is_empty() {
            "complete".to_owned()
        } else {
            "incomplete".to_owned()
        },
        expected_steps: CLAIM_REQUIRED_COMPLETED_STEP,
        present_steps,
        missing_steps,
        totals,
    }
}

pub fn fallback_honesty_check(run: &LoadedRun) -> FallbackHonesty {
    let resume_steps = run
        .steps
        .iter()
        .filter(|step| {
            matches!(
                step.provenance,
                StepProvenance::FrontierCheckpointResume
                    | StepProvenance::StepCheckpointResume
                    | StepProvenance::StepCheckpointReevaluate
            )
        })
        .map(|step| step.step_index)
        .collect::<Vec<_>>();
    let reference_replay_steps = run
        .steps
        .iter()
        .filter(|step| step.provenance == StepProvenance::ReferenceReplay)
        .map(|step| step.step_index)
        .collect::<Vec<_>>();
    let unexpected_provenance = run
        .steps
        .iter()
        .filter_map(|step| {
            if matches!(
                step.provenance,
                StepProvenance::AtomicBootstrapSearch
                    | StepProvenance::FrontierCheckpointResume
                    | StepProvenance::StepCheckpointResume
                    | StepProvenance::StepCheckpointReevaluate
            ) {
                None
            } else {
                Some(format!(
                    "step {} {}",
                    step.step_index,
                    step.provenance.as_str()
                ))
            }
        })
        .collect::<Vec<_>>();
    let run_mode_fallback = !CLAIM_ALLOWED_RUN_MODES.contains(&run.run_mode.as_str());

    FallbackHonesty {
        status: if !run_mode_fallback
            && reference_replay_steps.is_empty()
            && unexpected_provenance.is_empty()
        {
            "clear".to_owned()
        } else {
            "fallback_detected".to_owned()
        },
        run_mode: run.run_mode.clone(),
        run_mode_fallback,
        resume_steps,
        reference_replay_steps,
        unexpected_provenance,
    }
}

pub fn narrative_artifacts_check(run: &LoadedRun) -> NarrativeArtifacts {
    let expected_steps = run
        .steps
        .iter()
        .map(|step| step.step_index)
        .collect::<Vec<_>>();
    let present_narrative_steps = expected_steps
        .iter()
        .filter(|step| step_narrative_path(&run.run_dir, **step).exists())
        .copied()
        .collect::<Vec<_>>();
    let present_event_steps = expected_steps
        .iter()
        .filter(|step| step_events_path(&run.run_dir, **step).exists())
        .copied()
        .collect::<Vec<_>>();
    let missing_narrative_steps = expected_steps
        .iter()
        .filter(|step| !present_narrative_steps.contains(step))
        .copied()
        .collect::<Vec<_>>();
    let missing_event_steps = expected_steps
        .iter()
        .filter(|step| !present_event_steps.contains(step))
        .copied()
        .collect::<Vec<_>>();

    NarrativeArtifacts {
        status: if missing_narrative_steps.is_empty() && missing_event_steps.is_empty() {
            "complete".to_owned()
        } else {
            "incomplete".to_owned()
        },
        expected_steps: expected_steps.len() as u32,
        present_narrative_steps: present_narrative_steps.len(),
        present_event_steps: present_event_steps.len(),
        missing_narrative_steps,
        missing_event_steps,
    }
}

pub fn breadth_check(run: &LoadedRun, targets: &[(u32, usize)], exact: bool) -> BreadthCheck {
    let step_map = run
        .steps
        .iter()
        .map(|step| (step.step_index, step))
        .collect::<BTreeMap<_, _>>();
    let mut failing_steps = Vec::new();
    let mut steps = Vec::new();

    for (step_index, target) in targets.iter().copied() {
        let maybe_step = step_map.get(&step_index).copied();
        let actual = maybe_step.map(generated_count);
        let hit = actual
            .map(|value| {
                if exact {
                    value == target
                } else {
                    value >= target
                }
            })
            .unwrap_or(false);
        if !hit {
            failing_steps.push(step_index);
        }
        steps.push(BreadthStepResult {
            step_index,
            target,
            actual,
            status: if hit { "hit" } else { "miss" }.to_owned(),
            gap_to_target: actual.map(|value| target as i64 - value as i64),
            diagnosis: breadth_diagnosis(run, maybe_step, step_index, target, actual),
        });
    }

    let detail = if failing_steps.is_empty() {
        if exact {
            "early breadth targets are satisfied".to_owned()
        } else {
            "late generated floors are satisfied".to_owned()
        }
    } else if exact {
        format!(
            "early breadth failed at {}",
            render_step_list(&failing_steps)
        )
    } else {
        format!(
            "late generated floors failed at {}",
            render_step_list(&failing_steps)
        )
    };

    BreadthCheck {
        status: if failing_steps.is_empty() {
            "pass".to_owned()
        } else {
            "fail".to_owned()
        },
        detail,
        steps,
    }
}

pub fn runtime_threshold_check(
    run: &LoadedRun,
    threshold_ms: Option<u64>,
) -> RuntimeThresholdCheck {
    let total_runtime_ms = run
        .steps
        .iter()
        .map(|step| step.search_stats.search_timing.step_wall_clock_millis)
        .sum::<u64>();
    let passed = threshold_ms
        .map(|threshold| total_runtime_ms <= threshold)
        .unwrap_or(true);
    let detail = match threshold_ms {
        Some(_) if passed => {
            format!("stored runtime {total_runtime_ms} ms is within the certified threshold")
        }
        Some(_) => format!("stored runtime {total_runtime_ms} ms exceeds the certified threshold"),
        None => format!("stored runtime {total_runtime_ms} ms recorded"),
    };

    RuntimeThresholdCheck {
        status: if passed { "pass" } else { "fail" }.to_owned(),
        detail,
        total_runtime_ms,
        threshold_ms,
    }
}

pub fn manifest_completeness_check(manifest: &RunManifestV1) -> ManifestCompletenessCheck {
    let mut missing_fields = Vec::new();
    if manifest.host.os.trim().is_empty() {
        missing_fields.push("os".to_owned());
    }
    if manifest.host.arch.trim().is_empty() {
        missing_fields.push("arch".to_owned());
    }
    if manifest.host.logical_cpus == 0 {
        missing_fields.push("logical_cpus".to_owned());
    }
    if manifest.host.ram_bytes == 0 {
        missing_fields.push("ram_bytes".to_owned());
    }
    if manifest.host.cpu_model.trim().is_empty() {
        missing_fields.push("cpu_model".to_owned());
    }
    if manifest.host.physical_core_count == 0 {
        missing_fields.push("physical_core_count".to_owned());
    }
    if manifest.runtime.resolved_worker_count == 0 {
        missing_fields.push("resolved_worker_count".to_owned());
    }
    if manifest.build.profile.trim().is_empty() {
        missing_fields.push("build_profile".to_owned());
    }
    if manifest.build.target_triple.trim().is_empty() {
        missing_fields.push("target_triple".to_owned());
    }
    if manifest.build.target_cpu.trim().is_empty() {
        missing_fields.push("target_cpu".to_owned());
    }
    if manifest.build.git_commit_sha.trim().is_empty() {
        missing_fields.push("git_commit_sha".to_owned());
    }
    if manifest.build.cargo_lock_sha256.trim().is_empty() {
        missing_fields.push("cargo_lock_sha256".to_owned());
    }
    if manifest.build.binary_sha256.trim().is_empty() {
        missing_fields.push("binary_sha256".to_owned());
    }

    ManifestCompletenessCheck {
        status: if missing_fields.is_empty() {
            "pass".to_owned()
        } else {
            "fail".to_owned()
        },
        detail: if missing_fields.is_empty() {
            "manifest includes all required provenance fields".to_owned()
        } else {
            format!("manifest is missing {}", missing_fields.join(", "))
        },
        missing_fields,
    }
}

pub fn render_claim_compare_text(summary: &ClaimCompareSummary) -> String {
    let mut lines = vec![
        format!("Comparison Signoff: {}", summary.signoff_status),
        "baseline: guarded".to_owned(),
        "lanes: guarded, claim".to_owned(),
        format!("trajectory: {}", summary.trajectory.detail),
        format!("accepted hashes: {}", summary.accepted_hashes.detail),
        format!(
            "search-space counts: {}",
            summary.search_space_counts.detail
        ),
        format!(
            "admissibility diagnostics: {}",
            summary.admissibility_diagnostics.detail
        ),
        format!(
            "late-step competition: {}",
            summary.late_step_competition.detail
        ),
        format!(
            "step15 claim boundary: {}",
            summary.step15_claim_boundary.detail
        ),
        format!(
            "claim lane audit: {} (claim)",
            summary.claim_lane_audit.status
        ),
    ];

    for lane in &summary.lanes {
        lines.push(String::new());
        lines.push(format!("Lane {}", lane.label));
        lines.push(format!("  run_id: {}", lane.run_id));
        lines.push(format!("  search profile: {}", lane.search_profile));
        lines.push(format!("  mode: {}", lane.run_mode));
        lines.push(format!("  completed_step: {}", lane.completed_step));
        lines.push(format!("  trajectory: {}", lane.trajectory.detail));
        lines.push(format!(
            "  accepted hashes: {}",
            lane.accepted_hashes.detail
        ));
        lines.push(format!(
            "  search-space counts: {}",
            lane.search_space_counts.detail
        ));
        lines.push(format!(
            "  admissibility diagnostics: {}",
            lane.admissibility_diagnostics.detail
        ));
        lines.push(format!(
            "  late-step competition: {}",
            lane.late_step_competition.detail
        ));
        lines.push(format!(
            "  provenance sequence: {}",
            lane.provenance_sequence
        ));
        lines.push(format!(
            "  replay ablation: {}",
            lane.replay_ablation_sequence
        ));
        lines.push(format!("  step15 claim: {}", lane.step15_claim));
        lines.push(format!("  demo phase latest: {}", lane.demo_phase_latest));
        lines.push(format!("  demo funnel latest: {}", lane.demo_funnel_latest));
        lines.push(format!(
            "  demo buckets latest: {}",
            lane.demo_buckets_latest
        ));

        if lane.label == "claim" {
            let artifacts = &summary.claim_lane_audit.narrative_artifacts;
            let fallback = &summary.claim_lane_audit.fallback_honesty;
            lines.push(format!(
                "  narrative artifacts: {} (text={}/{}, events={}/{})",
                artifacts.status,
                artifacts.present_narrative_steps,
                artifacts.expected_steps,
                artifacts.present_event_steps,
                artifacts.expected_steps
            ));
            lines.push(format!(
                "  claim audit: {} ({})",
                summary.claim_lane_audit.status,
                if summary.claim_lane_audit.reasons.is_empty() {
                    "none".to_owned()
                } else {
                    summary.claim_lane_audit.reasons.join(", ")
                }
            ));
            lines.push(format!(
                "  search policy: {} guidance_style={} late_expansion_policy={} bucket_policy={}",
                summary.claim_lane_audit.search_policy.status,
                summary.claim_lane_audit.search_policy.actual.guidance_style,
                summary
                    .claim_lane_audit
                    .search_policy
                    .actual
                    .late_expansion_policy,
                summary.claim_lane_audit.search_policy.actual.bucket_policy
            ));
            lines.push(format!(
                "  exact-screen reasons: {} {}",
                summary.claim_lane_audit.exact_screen_reasons.status,
                render_named_totals(&summary.claim_lane_audit.exact_screen_reasons.totals)
            ));
            lines.push(format!(
                "  prune class coverage: {} {}",
                summary.claim_lane_audit.prune_classes.status,
                render_named_totals(&summary.claim_lane_audit.prune_classes.totals)
            ));
            lines.push(format!(
                "  fallback honesty: {} run_mode={} resume_steps={} reference_replay_steps={}",
                fallback.status,
                fallback.run_mode,
                render_optional_steps(&fallback.resume_steps),
                render_optional_steps(&fallback.reference_replay_steps)
            ));
        }
    }

    format!("{}\n", lines.join("\n"))
}

pub fn render_claim_certificate_text(
    claim_run: &Path,
    guarded_run: &Path,
    runtime_threshold_ms: Option<u64>,
    checks: &[(String, String)],
) -> String {
    let failing_checks = checks
        .iter()
        .filter(|(_, detail)| detail.starts_with("fail -"))
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    let mut lines = vec![
        format!(
            "Claim Certification: {}",
            if failing_checks.is_empty() {
                "ready"
            } else {
                "attention"
            }
        ),
        format!("claim run: {}", claim_run.display()),
        format!("guarded run: {}", guarded_run.display()),
        format!(
            "runtime threshold ms: {}",
            runtime_threshold_ms
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned())
        ),
    ];
    lines.extend(
        checks
            .iter()
            .map(|(name, detail)| format!("{name}: {detail}")),
    );
    if !failing_checks.is_empty() {
        lines.push(format!("failing checks: {}", failing_checks.join(", ")));
    }
    format!("{}\n", lines.join("\n"))
}

pub fn current_utc_timestamp() -> String {
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_owned())
}

pub fn write_text(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent directory {}", parent.display()))?;
    }
    fs::write(path, content).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn compare_lane(label: &str, lane: &LoadedRun, guarded: &LoadedRun) -> ClaimCompareLane {
    ClaimCompareLane {
        label: label.to_owned(),
        run_dir: lane.run_dir.display().to_string(),
        run_id: lane.manifest.run_id.clone(),
        search_profile: lane_search_profile(lane).to_owned(),
        run_mode: lane.run_mode.clone(),
        completed_step: lane.manifest.position.completed_step,
        trajectory_fingerprint: trajectory_fingerprint(lane),
        trajectory: compare_trajectory(lane, guarded),
        accepted_hashes: compare_accepted_hashes(lane, guarded),
        search_space_counts: compare_search_space_counts(lane, guarded),
        admissibility_diagnostics: compare_admissibility_diagnostics(lane, guarded),
        late_step_competition: compare_late_step_competition(lane, guarded),
        provenance_sequence: compact_sequence(
            lane.steps
                .iter()
                .map(|step| (step.step_index, step.provenance.as_str().to_owned()))
                .collect(),
        ),
        replay_ablation_sequence: compact_sequence(
            lane.steps
                .iter()
                .map(|step| {
                    (
                        step.step_index,
                        step.replay_ablation.status.as_str().to_owned(),
                    )
                })
                .collect(),
        ),
        demo_phase_latest: demo_phase_latest(lane),
        demo_funnel_latest: demo_funnel_latest(lane),
        demo_buckets_latest: demo_buckets_latest(lane),
        step15_claim: render_step15_claim(lane),
    }
}

fn top_level_status(
    match_detail: &str,
    mismatch_detail: &str,
    diverging_steps: Vec<u32>,
) -> CompareFieldStatus {
    if diverging_steps.is_empty() {
        CompareFieldStatus {
            status: "matches".to_owned(),
            detail: match_detail.to_owned(),
            diverging_steps,
        }
    } else {
        CompareFieldStatus {
            status: "diverges".to_owned(),
            detail: mismatch_detail.to_owned(),
            diverging_steps,
        }
    }
}

fn compare_trajectory(lane: &LoadedRun, guarded: &LoadedRun) -> CompareFieldStatus {
    let diverging_steps = compare_by_step(
        &trajectory_entries(&guarded.steps),
        &trajectory_entries(&lane.steps),
    );
    if diverging_steps.is_empty() {
        CompareFieldStatus {
            status: "matches".to_owned(),
            detail: format!("matches baseline ({})", trajectory_fingerprint(lane)),
            diverging_steps,
        }
    } else {
        CompareFieldStatus {
            status: "diverges".to_owned(),
            detail: format!("diverges at {}", render_step_list(&diverging_steps)),
            diverging_steps,
        }
    }
}

fn compare_accepted_hashes(lane: &LoadedRun, guarded: &LoadedRun) -> CompareFieldStatus {
    simple_compare_status(compare_by_step(
        &accepted_hash_entries(&guarded.steps),
        &accepted_hash_entries(&lane.steps),
    ))
}

fn compare_search_space_counts(lane: &LoadedRun, guarded: &LoadedRun) -> CompareFieldStatus {
    simple_compare_status(compare_by_step(
        &search_space_entries(&guarded.steps),
        &search_space_entries(&lane.steps),
    ))
}

fn compare_admissibility_diagnostics(lane: &LoadedRun, guarded: &LoadedRun) -> CompareFieldStatus {
    simple_compare_status(compare_by_step(
        &admissibility_entries(&guarded.steps),
        &admissibility_entries(&lane.steps),
    ))
}

fn compare_late_step_competition(lane: &LoadedRun, guarded: &LoadedRun) -> CompareFieldStatus {
    simple_compare_status(compare_by_step(
        &late_competition_entries(&guarded.steps),
        &late_competition_entries(&lane.steps),
    ))
}

fn simple_compare_status(diverging_steps: Vec<u32>) -> CompareFieldStatus {
    if diverging_steps.is_empty() {
        CompareFieldStatus {
            status: "matches".to_owned(),
            detail: "matches baseline".to_owned(),
            diverging_steps,
        }
    } else {
        CompareFieldStatus {
            status: "diverges".to_owned(),
            detail: format!("diverges at {}", render_step_list(&diverging_steps)),
            diverging_steps,
        }
    }
}

fn compare_step15_claim_boundary(
    guarded: &LoadedRun,
    claim: &LoadedRun,
) -> Step15ClaimBoundaryStatus {
    let guarded_claim = guarded
        .steps
        .iter()
        .find(|step| step.step_index == CLAIM_REQUIRED_COMPLETED_STEP)
        .map(|step| &step.canon_evidence.late_step_claim);
    let claim_claim = claim
        .steps
        .iter()
        .find(|step| step.step_index == CLAIM_REQUIRED_COMPLETED_STEP)
        .map(|step| &step.canon_evidence.late_step_claim);
    let consistent = guarded_claim
        .zip(claim_claim)
        .map(|(left, right)| normalize_late_step_claim(left) == normalize_late_step_claim(right))
        .unwrap_or(false);
    let detail = claim_claim
        .map(render_late_step_claim_compact)
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| "not recorded".to_owned());

    Step15ClaimBoundaryStatus {
        status: if consistent {
            "consistent".to_owned()
        } else {
            "attention".to_owned()
        },
        detail: if consistent {
            format!("consistent ({detail})")
        } else {
            format!("attention ({detail})")
        },
    }
}

fn breadth_diagnosis(
    run: &LoadedRun,
    maybe_step: Option<&StepReport>,
    step_index: u32,
    target: usize,
    actual: Option<usize>,
) -> BreadthDiagnosis {
    let Some(step) = maybe_step else {
        return BreadthDiagnosis {
            source: "missing_step_summary".to_owned(),
            summary: format!("actual=missing target={target}"),
            raw_catalog_telescope_count: None,
            raw_catalog_clause_widths: Vec::new(),
            claim_root_seeding: ClaimRootSeedingDiagnostics::default(),
            claim_step_open: None,
            search_space: BreadthSearchSpace {
                well_formed_candidates: 0,
                exact_bound_screened: 0,
                retained_candidates: 0,
                heuristic_dropped: 0,
            },
            exact_screen_reasons: ExactScreenReasonStats::default(),
        };
    };

    let checkpoints = run
        .live_checkpoints
        .get(&step_index)
        .cloned()
        .unwrap_or_default();
    let best_checkpoint = preferred_checkpoint(&checkpoints);
    let claim_root_seeding = best_checkpoint
        .and_then(|checkpoint| checkpoint.claim_root_seeding)
        .or(step.search_stats.claim_root_seeding)
        .unwrap_or_default();
    let claim_step_open = best_checkpoint
        .and_then(|checkpoint| checkpoint.claim_step_open)
        .or(step.search_stats.claim_step_open);
    let raw_catalog_telescope_count =
        best_checkpoint.and_then(|checkpoint| checkpoint.raw_catalog_telescope_count);
    let raw_catalog_clause_widths = best_checkpoint
        .map(|checkpoint| checkpoint.raw_catalog_clause_widths.clone())
        .unwrap_or_default();
    let search_space = BreadthSearchSpace {
        well_formed_candidates: step.search_stats.well_formed_candidates,
        exact_bound_screened: step.search_stats.demo_funnel.exact_bound_screened,
        retained_candidates: step.search_stats.retained_candidates,
        heuristic_dropped: step.search_stats.heuristic_drops,
    };
    let exact_screen_reasons = step.search_stats.exact_screen_reasons.clone();
    let actual_value = actual.unwrap_or_default();

    BreadthDiagnosis {
        source: if best_checkpoint.is_some() {
            "summary_plus_live_checkpoints".to_owned()
        } else {
            "summary_only".to_owned()
        },
        summary: format!(
            "actual={} target={} gap={}{}{}{} well_formed={} exact_screened={} retained={} heuristic={} exact=bar={} completion={} incumbent={} legality={}",
            actual_value,
            target,
            target as i64 - actual_value as i64,
            raw_catalog_telescope_count
                .map(|value| format!(" catalog={value}"))
                .unwrap_or_default(),
            if raw_catalog_clause_widths.is_empty() {
                String::new()
            } else {
                format!(" widths={raw_catalog_clause_widths:?}")
            },
            format!(
                "{}{}",
                render_claim_root_seeding(&claim_root_seeding),
                render_claim_step_open(claim_step_open.as_ref())
            ),
            search_space.well_formed_candidates,
            search_space.exact_bound_screened,
            search_space.retained_candidates,
            search_space.heuristic_dropped,
            exact_screen_reasons.partial_prefix_bar_failure,
            exact_screen_reasons.terminal_prefix_completion_failure,
            exact_screen_reasons.incumbent_dominance,
            exact_screen_reasons.legality_connectivity_exact_rejection
        ),
        raw_catalog_telescope_count,
        raw_catalog_clause_widths,
        claim_root_seeding,
        claim_step_open,
        search_space,
        exact_screen_reasons,
    }
}

fn preferred_checkpoint(checkpoints: &[StepLiveCheckpoint]) -> Option<&StepLiveCheckpoint> {
    checkpoints
        .iter()
        .rev()
        .find(|checkpoint| {
            matches!(
                checkpoint.note.as_deref(),
                Some(CLAIM_ROOT_SEEDING_NOTE)
                    | Some(CLAIM_REGULAR_CATALOG_NOTE)
                    | Some(CLAIM_EARLY_EXHAUSTIVE_NOTE)
            )
        })
        .or_else(|| {
            checkpoints.iter().rev().find(|checkpoint| {
                checkpoint.raw_catalog_telescope_count.is_some()
                    || !checkpoint.raw_catalog_clause_widths.is_empty()
                    || checkpoint.claim_step_open.is_some()
                    || checkpoint.claim_root_seeding.is_some()
            })
        })
}

fn render_claim_root_seeding(claim_root_seeding: &ClaimRootSeedingDiagnostics) -> String {
    if claim_root_seeding == &ClaimRootSeedingDiagnostics::default() {
        String::new()
    } else {
        format!(
            " roots=seen{} enqueued{} exact_rejected{}",
            claim_root_seeding.roots_seen,
            claim_root_seeding.roots_enqueued,
            claim_root_seeding.roots_rejected_by_exact_screen
        )
    }
}

fn render_claim_step_open(claim_step_open: Option<&ClaimStepOpenDiagnostics>) -> String {
    let Some(open) = claim_step_open else {
        return " anchor=none axes=path0,trunc0,coupling0,support0,modal0,temporal0,reanchor0,closure0"
            .to_owned();
    };
    format!(
        " surface={} kappa={}..{} anchor={}{}{}{}",
        open.late_family_surface.as_str(),
        open.kappa_min,
        open.kappa_max,
        render_anchor_policy(open.anchor_policy),
        open.historical_anchor_ref
            .map(|value| format!("@{value}"))
            .unwrap_or_default(),
        render_widen_bands(open),
        render_claim_axes(open.claim_debt_axes, open.package_flags)
    )
}

fn render_anchor_policy(policy: ClaimAnchorPolicyDiagnostics) -> &'static str {
    match policy {
        ClaimAnchorPolicyDiagnostics::None => "none",
        ClaimAnchorPolicyDiagnostics::Loop => "loop",
        ClaimAnchorPolicyDiagnostics::Modal => "modal",
    }
}

fn render_widen_bands(open: &ClaimStepOpenDiagnostics) -> String {
    let mut bands = Vec::new();
    if open.claim_widening_band7_active {
        bands.push("7");
    }
    if open.claim_widening_band8_active {
        bands.push("8");
    }
    if open.claim_widening_band9_active {
        bands.push("9");
    }
    if bands.is_empty() {
        String::new()
    } else {
        format!(" widen={}", bands.join(","))
    }
}

fn render_claim_axes(axes: ClaimDebtAxesDiagnostics, flags: ClaimPackageFlags) -> String {
    let packages = {
        let mut packages = Vec::new();
        if flags.operator_bundle {
            packages.push("operator_bundle");
        }
        if flags.hilbert_functional {
            packages.push("hilbert_functional");
        }
        if flags.temporal_shell {
            packages.push("temporal_shell");
        }
        if packages.is_empty() {
            "none".to_owned()
        } else {
            packages.join(",")
        }
    };
    format!(
        " packages={} axes=path{},trunc{},coupling{},support{},modal{},temporal{},reanchor{},closure{}",
        packages,
        axes.path_pressure,
        axes.trunc_pressure,
        axes.coupling_pressure,
        axes.support_pressure,
        axes.modal_pressure,
        axes.temporal_pressure,
        axes.reanchor_pressure,
        axes.closure_pressure
    )
}

fn trajectory_entries(steps: &[StepReport]) -> Vec<TrajectoryEntry> {
    steps
        .iter()
        .map(|step| TrajectoryEntry {
            step_index: step.step_index,
            label: step.label.clone(),
            nu: step.accepted.nu,
            clause_kappa: step.accepted.clause_kappa,
        })
        .collect()
}

fn accepted_hash_entries(steps: &[StepReport]) -> Vec<AcceptedHashEntry> {
    steps
        .iter()
        .map(|step| AcceptedHashEntry {
            step_index: step.step_index,
            candidate_hash: step.accepted.candidate_hash.clone(),
            canonical_hash: step.accepted.canonical_hash.clone(),
        })
        .collect()
}

fn search_space_entries(steps: &[StepReport]) -> Vec<SearchSpaceEntry> {
    steps
        .iter()
        .map(|step| SearchSpaceEntry {
            step_index: step.step_index,
            enumerated: step.search_stats.enumerated_candidates,
            well_formed: step.search_stats.well_formed_candidates,
            admissibility_rejected: step.search_stats.admissibility_rejections,
            evaluated: step.search_stats.evaluated_candidates,
            canonical: step.search_stats.canonical_candidates,
            semantically_minimal: step.search_stats.semantically_minimal_candidates,
            retained: step.search_stats.retained_candidates,
        })
        .collect()
}

fn admissibility_entries(steps: &[StepReport]) -> Vec<AdmissibilityEntry> {
    steps
        .iter()
        .map(|step| AdmissibilityEntry {
            step_index: step.step_index,
            exact_legality_rejections: step
                .search_stats
                .admissibility_diagnostics
                .exact_legality_rejections,
            structural_debt_cap_rejections: step
                .search_stats
                .admissibility_diagnostics
                .structural_debt_cap_rejections,
            admitted_deprioritized: step
                .search_stats
                .admissibility_diagnostics
                .admitted_deprioritized,
            admitted_focus_aligned: step
                .search_stats
                .admissibility_diagnostics
                .admitted_focus_aligned,
        })
        .collect()
}

fn late_competition_entries(steps: &[StepReport]) -> Vec<LateCompetitionEntry> {
    steps
        .iter()
        .filter(|step| step.step_index >= 10)
        .map(|step| LateCompetitionEntry {
            step_index: step.step_index,
            evaluated: step.search_stats.evaluated_candidates,
            clears_bar: step.search_stats.scored_candidate_distribution.clears_bar,
            below_bar: step.search_stats.scored_candidate_distribution.below_bar,
            retained: step.search_stats.retained_candidates,
            terminal_rank_prunes: step.search_stats.incremental_terminal_rank_prunes,
        })
        .collect()
}

fn compare_by_step<T>(baseline: &[T], lane: &[T]) -> Vec<u32>
where
    T: StepIndexed + PartialEq,
{
    let baseline_map = baseline
        .iter()
        .map(|entry| (entry.step_index(), entry))
        .collect::<BTreeMap<_, _>>();
    let lane_map = lane
        .iter()
        .map(|entry| (entry.step_index(), entry))
        .collect::<BTreeMap<_, _>>();
    let all_steps = baseline_map
        .keys()
        .chain(lane_map.keys())
        .copied()
        .collect::<BTreeSet<_>>();

    all_steps
        .into_iter()
        .filter(|step| baseline_map.get(step) != lane_map.get(step))
        .collect()
}

fn accepted_hash_map(steps: &[StepReport]) -> BTreeMap<u32, (String, String)> {
    steps
        .iter()
        .map(|step| {
            (
                step.step_index,
                (
                    step.accepted.candidate_hash.clone(),
                    step.accepted.canonical_hash.clone(),
                ),
            )
        })
        .collect()
}

fn expected_missing_steps(steps: &[StepReport]) -> Vec<u32> {
    let present = steps
        .iter()
        .map(|step| step.step_index)
        .collect::<BTreeSet<_>>();
    (1..=CLAIM_REQUIRED_COMPLETED_STEP)
        .filter(|step| !present.contains(step))
        .collect()
}

fn generated_count(step: &StepReport) -> usize {
    step.search_stats.demo_funnel.generated_raw_prefixes
}

fn render_step15_claim(run: &LoadedRun) -> String {
    run.steps
        .iter()
        .find(|step| step.step_index == CLAIM_REQUIRED_COMPLETED_STEP)
        .map(|step| render_late_step_claim_compact(&step.canon_evidence.late_step_claim))
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| "not recorded".to_owned())
}

fn render_late_step_claim_compact(claim: &LateStepClaim) -> String {
    match claim.status {
        LateStepClaimStatus::NotApplicable => String::new(),
        LateStepClaimStatus::ExecutableCanon => format!(
            "executable_canon step {} {} nu={} matches_accepted={}",
            claim.adopted_step, claim.adopted_label, claim.adopted_nu, claim.matches_accepted
        ),
        LateStepClaimStatus::HistoricalProvenanceOnly => format!(
            "historical_provenance_only step {} {} nu={}",
            claim.adopted_step, claim.adopted_label, claim.adopted_nu
        ),
    }
}

fn normalize_late_step_claim(
    claim: &LateStepClaim,
) -> (LateStepClaimStatus, u32, String, u16, bool) {
    (
        claim.status,
        claim.adopted_step,
        claim.adopted_label.clone(),
        claim.adopted_nu,
        claim.matches_accepted,
    )
}

fn lane_search_profile(run: &LoadedRun) -> &'static str {
    run.steps
        .last()
        .map(|step| step.search_profile.as_str())
        .unwrap_or(SearchProfile::Unknown.as_str())
}

fn trajectory_fingerprint(run: &LoadedRun) -> String {
    hash_json(&trajectory_entries(&run.steps))
}

fn compact_sequence(items: Vec<(u32, String)>) -> String {
    if items.is_empty() {
        return "none".to_owned();
    }

    let mut rendered = Vec::new();
    let mut start = items[0].0;
    let mut end = items[0].0;
    let mut value = items[0].1.clone();

    for (step, next_value) in items.into_iter().skip(1) {
        if step == end + 1 && next_value == value {
            end = step;
            continue;
        }
        rendered.push(render_compact_chunk(start, end, &value));
        start = step;
        end = step;
        value = next_value;
    }
    rendered.push(render_compact_chunk(start, end, &value));
    rendered.join("; ")
}

fn render_compact_chunk(start: u32, end: u32, value: &str) -> String {
    if start == end {
        format!("{start:02} {value}")
    } else {
        format!("{start:02}-{end:02} {value}")
    }
}

fn demo_phase_latest(run: &LoadedRun) -> String {
    let Some(step) = run.steps.last() else {
        return "none".to_owned();
    };
    let phase = &step.search_stats.demo_phase;
    let funnel = &step.search_stats.demo_funnel;
    let closure = &step.search_stats.demo_closure;
    let generated_floor = phase
        .generated_floor
        .map(|value| {
            if funnel.generated_raw_prefixes as u64 >= value {
                format!("hit ({}/{value})", funnel.generated_raw_prefixes)
            } else {
                format!("miss ({}/{value})", funnel.generated_raw_prefixes)
            }
        })
        .unwrap_or_else(|| "not_applicable".to_owned());
    let exact_screened_floor = phase
        .exact_screened_floor
        .map(|value| {
            if funnel.exact_bound_screened as u64 >= value {
                format!("hit ({}/{value})", funnel.exact_bound_screened)
            } else {
                format!("miss ({}/{value})", funnel.exact_bound_screened)
            }
        })
        .unwrap_or_else(|| "not_applicable".to_owned());
    let soft_cap = phase
        .full_eval_soft_cap
        .map(|value| value.to_string())
        .unwrap_or_else(|| "not_applicable".to_owned());
    let breadth_exit = phase
        .breadth_harvest_exit_reason
        .map(|value| render_enum(&value))
        .unwrap_or_else(|| "none".to_owned());
    let proof_close_reason = phase
        .proof_close_entry_reason
        .map(|value| render_enum(&value))
        .unwrap_or_else(|| "none".to_owned());
    let overrun_reason = phase
        .proof_close_overrun_reason
        .map(|value| render_enum(&value))
        .unwrap_or_else(|| "none".to_owned());
    format!(
        "step {} generated_floor={} exact_screened_floor={} soft_cap={} breadth_exit={} proof_close_reason={} overrun_reason={} overrun_full_evals={} proof_close_closure={}% groups_closed={}/{} reserve={}/{}ms reserve_remaining={}ms reserve_overrun={}ms reserve_exhausted={}",
        step.step_index,
        generated_floor,
        exact_screened_floor,
        soft_cap,
        breadth_exit,
        proof_close_reason,
        overrun_reason,
        phase.proof_close_overrun_full_evals,
        closure.closure_percent,
        phase.proof_close_frontier_groups_closed,
        phase.proof_close_frontier_total_groups,
        phase.proof_close_elapsed_millis,
        phase.proof_close_reserved_millis,
        phase.proof_close_remaining_millis,
        phase.proof_close_reserve_overrun_millis,
        phase.proof_close_reserve_exhausted
    )
}

fn demo_funnel_latest(run: &LoadedRun) -> String {
    let Some(step) = run.steps.last() else {
        return "none".to_owned();
    };
    let funnel = &step.search_stats.demo_funnel;
    let closure = &step.search_stats.demo_closure;
    format!(
        "generated={} hard_admissible={} exact_screened={} exact_pruned={} full_eval={} closure={}% winner_overshoot={}",
        funnel.generated_raw_prefixes,
        funnel.hard_admissible,
        funnel.exact_bound_screened,
        funnel.exact_bound_pruned,
        step.search_stats.full_telescopes_evaluated,
        closure.closure_percent,
        funnel.winner_overshoot
    )
}

fn demo_buckets_latest(run: &LoadedRun) -> String {
    let Some(step) = run.steps.last() else {
        return "none".to_owned();
    };
    if step.search_stats.demo_bucket_stats.is_empty() {
        return "none".to_owned();
    }

    step.search_stats
        .demo_bucket_stats
        .iter()
        .map(|bucket| {
            let best = bucket
                .stats
                .best_overshoot
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned());
            format!(
                "{} gen={} adm={} screen={} pruned={} scored={} best={}",
                bucket.bucket_label,
                bucket.stats.generated_terminal_candidates,
                bucket.stats.admissible_terminal_candidates,
                bucket.stats.exact_screened_terminal_candidates,
                bucket.stats.pruned_terminal_candidates,
                bucket.stats.fully_scored_terminal_candidates,
                best
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn render_named_totals(totals: &BTreeMap<String, usize>) -> String {
    if totals.is_empty() {
        "none".to_owned()
    } else {
        totals
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn render_optional_steps(steps: &[u32]) -> String {
    if steps.is_empty() {
        "none".to_owned()
    } else {
        render_step_list(steps)
    }
}

pub fn render_step_list(steps: &[u32]) -> String {
    steps
        .iter()
        .map(|step| format!("step {step}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn add_total(totals: &mut BTreeMap<String, usize>, key: &str, value: usize) {
    *totals.entry(key.to_owned()).or_insert(0) += value;
}

fn hash_json<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_vec(value).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}

fn render_enum<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "unknown".to_owned())
        .trim_matches('"')
        .to_owned()
}

fn load_json_file<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))
}

fn load_ndjson_file<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<T>(line).context("parse ndjson line"))
        .collect()
}

fn step_live_path(run_dir: &Path, step_index: u32) -> PathBuf {
    run_dir
        .join("reports")
        .join("steps")
        .join(format!("step-{step_index:02}-live.ndjson"))
}

fn step_narrative_path(run_dir: &Path, step_index: u32) -> PathBuf {
    run_dir
        .join("reports")
        .join("steps")
        .join(format!("step-{step_index:02}-narrative.txt"))
}

fn step_events_path(run_dir: &Path, step_index: u32) -> PathBuf {
    run_dir
        .join("reports")
        .join("steps")
        .join(format!("step-{step_index:02}-events.ndjson"))
}

#[cfg(test)]
mod tests {
    use super::{
        breadth_check, build_claim_compare_summary, load_run, manifest_completeness_check,
        render_claim_compare_text, render_step_list, runtime_threshold_check,
    };
    use crate::cli::RunArgs;
    use crate::cmd_run::run;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(label: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should advance")
            .as_nanos();
        dir.push(format!("pen-cli-claim-evidence-{label}-{nanos}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    fn create_claim_and_guarded_runs() -> (PathBuf, PathBuf, PathBuf) {
        let root = temp_root("runs");
        run(RunArgs {
            config: "configs/default.toml".into(),
            root: root.clone(),
            run_id: Some("guarded".to_owned()),
            until_step: Some(15),
            debug: false,
            narrative: false,
        })
        .expect("guarded run should succeed");
        run(RunArgs {
            config: "configs/desktop_claim_shadow_smoke.toml".into(),
            root: root.clone(),
            run_id: Some("claim".to_owned()),
            until_step: Some(15),
            debug: false,
            narrative: true,
        })
        .expect("claim run should succeed");
        (root.clone(), root.join("guarded"), root.join("claim"))
    }

    #[test]
    fn claim_compare_summary_renders_real_run_inputs() {
        let (root, guarded_dir, claim_dir) = create_claim_and_guarded_runs();
        let guarded = load_run(&guarded_dir).expect("guarded run should load");
        let claim = load_run(&claim_dir).expect("claim run should load");
        let summary = build_claim_compare_summary(&guarded, &claim);
        let text = render_claim_compare_text(&summary);

        assert!(text.contains("Comparison Signoff:"));
        assert!(text.contains("Lane guarded"));
        assert!(text.contains("Lane claim"));
        assert!(text.contains("claim lane audit:"));
        assert!(text.contains("demo funnel latest:"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn breadth_and_runtime_checks_surface_claim_smoke_readiness() {
        let (root, guarded_dir, claim_dir) = create_claim_and_guarded_runs();
        let claim = load_run(&claim_dir).expect("claim run should load");
        let early = breadth_check(&claim, &[(1, 2144)], true);
        let late = breadth_check(&claim, &[(15, 5000)], false);
        let runtime = runtime_threshold_check(&claim, Some(600000));

        assert_eq!(early.status, "pass");
        assert_eq!(late.status, "pass");
        assert_eq!(runtime.status, "pass");
        assert!(early.steps[0].diagnosis.summary.contains("actual=2144"));
        assert!(early.steps[0].diagnosis.summary.contains("target=2144"));
        assert!(late.steps[0].diagnosis.summary.contains("target=5000"));
        assert_eq!(render_step_list(&[1, 15]), "step 1, step 15");

        let guarded = load_run(&guarded_dir).expect("guarded run should load");
        assert_eq!(
            manifest_completeness_check(&guarded.manifest).status,
            "pass"
        );

        fs::remove_dir_all(root).ok();
    }
}
