use crate::human::{clause_lines, describe_candidate, library_refs, step_label, translation_guide};
use anyhow::Result;
use pen_core::ids::{ClauseId, ObligationSetId, StateId};
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_search::config::SearchProfile;
use pen_search::diversify::{FrontierPressure, FrontierRuntimeLimits};
use pen_search::engine::{
    AtomicSearchStep, CandidateScoreDistribution, DedupePruneEvidence, FrontierRetentionOutcome,
    MinimalityPruneEvidence, search_bootstrap_from_prefix_for_profile_with_runtime,
    search_bootstrap_prefix_for_profile_with_runtime, supports_live_atomic_search,
};
use pen_search::expand::evaluate_candidate;
use pen_search::state::FrontierStateRecV1;
use pen_store::manifest::{AcceptedCandidate, NearMiss};
use pen_type::admissibility::AdmissibilityDiagnostics;
use pen_type::obligations::{RetentionClass, RetentionPolicy};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StepReport {
    pub step_index: u32,
    pub label: String,
    #[serde(default)]
    pub search_profile: SearchProfile,
    pub objective_bar: Rational,
    pub accepted: AcceptedCandidate,
    pub telescope: Telescope,
    pub trace: Vec<String>,
    #[serde(default)]
    pub near_misses: Vec<NearMiss>,
    #[serde(default)]
    pub search_stats: StepSearchStats,
    #[serde(default)]
    pub candidate_reports: Vec<CandidateReport>,
    #[serde(default)]
    pub prune_reports: Vec<PruneReport>,
    #[serde(default)]
    pub frontier_policy: RetentionPolicy,
    #[serde(default)]
    pub frontier_pressure: FrontierPressure,
    #[serde(default)]
    pub prefix_frontier: PrefixFrontierArtifact,
    #[serde(default)]
    pub provenance: StepProvenance,
    #[serde(default)]
    pub canon_evidence: CanonEvidence,
    #[serde(default)]
    pub replay_ablation: ReplayAblation,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StepSearchStats {
    pub enumerated_candidates: usize,
    pub well_formed_candidates: usize,
    pub malformed_rejections: usize,
    #[serde(default)]
    pub malformed_rejection_reasons: BTreeMap<String, usize>,
    pub admissibility_rejections: usize,
    #[serde(default)]
    pub admissibility_diagnostics: AdmissibilityDiagnostics,
    #[serde(default)]
    pub full_telescopes_evaluated: usize,
    pub evaluated_candidates: usize,
    pub canonical_candidates: usize,
    pub semantically_minimal_candidates: usize,
    #[serde(default)]
    pub canonical_dedupe_prunes: usize,
    #[serde(default)]
    pub semantic_minimality_prunes: usize,
    pub dedupe_prunes: usize,
    pub minimality_prunes: usize,
    pub heuristic_drops: usize,
    #[serde(default)]
    pub prefixes_created: usize,
    #[serde(default)]
    pub prefix_states_explored: usize,
    #[serde(default)]
    pub prefix_states_merged_by_signature: usize,
    #[serde(default)]
    pub prefix_states_exact_pruned: usize,
    #[serde(default)]
    pub prefix_states_heuristic_dropped: usize,
    #[serde(default)]
    pub incremental_legality_cache_hits: usize,
    #[serde(default)]
    pub incremental_connectivity_shortcuts: usize,
    #[serde(default)]
    pub incremental_connectivity_fallbacks: usize,
    #[serde(default)]
    pub incremental_connectivity_prunes: usize,
    #[serde(default)]
    pub incremental_clause_family_filter_hits: usize,
    #[serde(default)]
    pub incremental_clause_family_prunes: usize,
    #[serde(default)]
    pub incremental_terminal_admissibility_hits: usize,
    #[serde(default)]
    pub incremental_terminal_admissibility_rejections: usize,
    #[serde(default)]
    pub prefix_frontier_hot_states: usize,
    #[serde(default)]
    pub prefix_frontier_cold_states: usize,
    pub retained_candidates: usize,
    pub frontier_hot_states: usize,
    pub frontier_cold_states: usize,
    pub frontier_resident_cold_states: usize,
    pub frontier_spill_states: usize,
    pub frontier_drops: usize,
    #[serde(default)]
    pub scored_candidate_distribution: CandidateScoreDistribution,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrefixFrontierArtifact {
    #[serde(default)]
    pub hot_states: Vec<FrontierStateSnapshot>,
    #[serde(default)]
    pub cold_states: Vec<FrontierStateSnapshot>,
    #[serde(default)]
    pub dedupe_keys: Vec<String>,
}

impl PrefixFrontierArtifact {
    pub fn is_empty(&self) -> bool {
        self.hot_states.is_empty() && self.cold_states.is_empty() && self.dedupe_keys.is_empty()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FrontierStateSnapshot {
    pub state_id: u64,
    pub parent_state_id: u64,
    pub last_clause_id: u32,
    pub obligation_set_id: u32,
    pub shape_hash64: u64,
    pub support_hash64: u64,
    pub nu_lower_bound: u16,
    pub nu_upper_bound: u16,
    pub bit_kappa_used: u16,
    pub clause_kappa_used: u16,
    pub depth: u16,
    pub step_index: u8,
    pub band_index: u8,
    pub flags: u16,
    pub priority_key: u32,
    pub worker_hint: u16,
}

impl From<FrontierStateRecV1> for FrontierStateSnapshot {
    fn from(record: FrontierStateRecV1) -> Self {
        Self {
            state_id: record.state_id.get(),
            parent_state_id: record.parent_state_id.get(),
            last_clause_id: record.last_clause_id.get(),
            obligation_set_id: record.obligation_set_id.get(),
            shape_hash64: record.shape_hash64,
            support_hash64: record.support_hash64,
            nu_lower_bound: record.nu_lower_bound,
            nu_upper_bound: record.nu_upper_bound,
            bit_kappa_used: record.bit_kappa_used,
            clause_kappa_used: record.clause_kappa_used,
            depth: record.depth,
            step_index: record.step_index,
            band_index: record.band_index,
            flags: record.flags,
            priority_key: record.priority_key,
            worker_hint: record.worker_hint,
        }
    }
}

impl FrontierStateSnapshot {
    pub fn to_record(&self) -> FrontierStateRecV1 {
        FrontierStateRecV1 {
            state_id: StateId::new(self.state_id),
            parent_state_id: StateId::new(self.parent_state_id),
            last_clause_id: ClauseId::new(self.last_clause_id),
            obligation_set_id: ObligationSetId::new(self.obligation_set_id),
            shape_hash64: self.shape_hash64,
            support_hash64: self.support_hash64,
            nu_lower_bound: self.nu_lower_bound,
            nu_upper_bound: self.nu_upper_bound,
            bit_kappa_used: self.bit_kappa_used,
            clause_kappa_used: self.clause_kappa_used,
            depth: self.depth,
            step_index: self.step_index,
            band_index: self.band_index,
            flags: self.flags,
            priority_key: self.priority_key,
            worker_hint: self.worker_hint,
            reserved: 0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CanonEvidence {
    pub charged_clause_kappa: u16,
    pub bar_distance: Rational,
    pub clears_bar: bool,
    #[serde(default)]
    pub late_step_claim: LateStepClaim,
}

impl Default for CanonEvidence {
    fn default() -> Self {
        Self {
            charged_clause_kappa: 0,
            bar_distance: Rational::zero(),
            clears_bar: false,
            late_step_claim: LateStepClaim::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LateStepClaim {
    #[serde(default)]
    pub status: LateStepClaimStatus,
    #[serde(default)]
    pub adopted_step: u32,
    #[serde(default)]
    pub adopted_label: String,
    #[serde(default)]
    pub adopted_nu: u16,
    #[serde(default)]
    pub matches_accepted: bool,
    #[serde(default)]
    pub note: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LateStepClaimStatus {
    #[default]
    NotApplicable,
    ExecutableCanon,
    HistoricalProvenanceOnly,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateStatus {
    #[default]
    AcceptedMinimalOvershoot,
    ClearsBarHigherOvershoot,
    BelowBar,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontierRetention {
    #[default]
    NotRecorded,
    Hot,
    #[serde(alias = "cold")]
    ColdResident,
    SpillBacked,
    Dropped,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateReport {
    pub status: CandidateStatus,
    #[serde(default)]
    pub frontier_retention: FrontierRetention,
    #[serde(default)]
    pub retention_class: RetentionClass,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PruneReport {
    pub prune_class: PruneReportClass,
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub nu: u16,
    pub rho: Rational,
    pub bar_delta: Rational,
    #[serde(default)]
    pub retention_class: RetentionClass,
    pub headline: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PruneReportClass {
    QuotientDedupe,
    SoundMinimality,
    HeuristicShaping,
}

impl PruneReportClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuotientDedupe => "quotient_dedupe",
            Self::SoundMinimality => "sound_minimality",
            Self::HeuristicShaping => "heuristic_shaping",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayAblation {
    #[serde(default)]
    pub status: ReplayAblationStatus,
    #[serde(default)]
    pub reference_candidate_hash: String,
    #[serde(default)]
    pub reference_canonical_hash: String,
    pub rho_delta: Rational,
    pub objective_bar_delta: Rational,
    pub overshoot_delta: Rational,
    #[serde(default)]
    pub nu_delta: i32,
    #[serde(default)]
    pub clause_kappa_delta: i32,
    #[serde(default)]
    pub note: String,
}

impl Default for ReplayAblation {
    fn default() -> Self {
        Self {
            status: ReplayAblationStatus::NotRecorded,
            reference_candidate_hash: String::new(),
            reference_canonical_hash: String::new(),
            rho_delta: Rational::zero(),
            objective_bar_delta: Rational::zero(),
            overshoot_delta: Rational::zero(),
            nu_delta: 0,
            clause_kappa_delta: 0,
            note: String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayAblationStatus {
    #[default]
    NotRecorded,
    MatchesReferenceReplay,
    DivergesFromReferenceReplay,
    NotApplicable,
}

impl ReplayAblationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRecorded => "not_recorded",
            Self::MatchesReferenceReplay => "matches_reference_replay",
            Self::DivergesFromReferenceReplay => "diverges_from_reference_replay",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StepProvenance {
    #[default]
    Unknown,
    AtomicBootstrapSearch,
    FrontierCheckpointResume,
    StepCheckpointResume,
    StepCheckpointReevaluate,
    ReferenceReplay,
}

impl StepProvenance {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::AtomicBootstrapSearch => "atomic_bootstrap_search",
            Self::FrontierCheckpointResume => "frontier_checkpoint_resume",
            Self::StepCheckpointResume => "step_checkpoint_resume",
            Self::StepCheckpointReevaluate => "step_checkpoint_reevaluate",
            Self::ReferenceReplay => "reference_replay",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StepGenerationMode {
    AtomicBootstrapSearch,
    FrontierCheckpointResume,
    StepCheckpointResume,
    StepCheckpointReevaluate,
    ReferenceReplay,
}

impl StepGenerationMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtomicBootstrapSearch => "atomic_search_bootstrap",
            Self::FrontierCheckpointResume => "frontier_checkpoint_resume",
            Self::StepCheckpointResume => "step_checkpoint_resume",
            Self::StepCheckpointReevaluate => "step_checkpoint_reevaluate",
            Self::ReferenceReplay => "reference_replay",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedSteps {
    pub mode: StepGenerationMode,
    pub steps: Vec<StepReport>,
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn generate_steps(until_step: u32, window_depth: u16) -> Result<GeneratedSteps> {
    generate_steps_with_runtime(until_step, window_depth, FrontierRuntimeLimits::unlimited())
}

pub fn generate_steps_with_runtime(
    until_step: u32,
    window_depth: u16,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<GeneratedSteps> {
    generate_steps_for_profile_with_runtime(
        until_step,
        window_depth,
        SearchProfile::StrictCanonGuarded,
        retention_runtime,
    )
}

pub fn generate_steps_for_profile_with_runtime(
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<GeneratedSteps> {
    if supports_live_atomic_search(until_step) {
        return Ok(GeneratedSteps {
            mode: StepGenerationMode::AtomicBootstrapSearch,
            steps: search_atomic_bootstrap_steps_for_profile_with_runtime(
                until_step,
                window_depth,
                search_profile,
                retention_runtime,
            )?,
        });
    }

    Ok(GeneratedSteps {
        mode: StepGenerationMode::ReferenceReplay,
        steps: replay_reference_steps(until_step, window_depth)?,
    })
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn extend_steps_from_reports(
    existing_steps: &[StepReport],
    until_step: u32,
    window_depth: u16,
) -> Result<GeneratedSteps> {
    extend_steps_from_reports_with_runtime(
        existing_steps,
        until_step,
        window_depth,
        FrontierRuntimeLimits::unlimited(),
    )
}

pub fn extend_steps_from_reports_with_runtime(
    existing_steps: &[StepReport],
    until_step: u32,
    window_depth: u16,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<GeneratedSteps> {
    extend_steps_from_reports_for_profile_with_runtime(
        existing_steps,
        until_step,
        window_depth,
        SearchProfile::StrictCanonGuarded,
        retention_runtime,
    )
}

pub fn extend_steps_from_reports_for_profile_with_runtime(
    existing_steps: &[StepReport],
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<GeneratedSteps> {
    let completed_step = existing_steps
        .last()
        .map(|step| step.step_index)
        .unwrap_or(0);
    if completed_step >= until_step {
        let mut steps = existing_steps.to_vec();
        steps.truncate(until_step as usize);
        return Ok(GeneratedSteps {
            mode: StepGenerationMode::StepCheckpointResume,
            steps: finalize_step_reports(steps, window_depth)?,
        });
    }

    ensure_contiguous_steps(existing_steps)?;

    if supports_live_atomic_search(until_step) {
        let mut steps = existing_steps.to_vec();
        let telescopes = existing_steps
            .iter()
            .map(|step| step.telescope.clone())
            .collect::<Vec<_>>();
        let resumed = search_bootstrap_from_prefix_for_profile_with_runtime(
            &telescopes,
            until_step,
            window_depth,
            search_profile,
            retention_runtime,
        )?
        .into_iter()
        .map(|step| step_to_report_with_provenance(step, StepProvenance::StepCheckpointResume));
        steps.extend(resumed);
        return Ok(GeneratedSteps {
            mode: StepGenerationMode::StepCheckpointResume,
            steps: finalize_step_reports(steps, window_depth)?,
        });
    }

    generate_steps_for_profile_with_runtime(
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
    )
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn reevaluate_steps_from_reports(
    existing_steps: &[StepReport],
    until_step: u32,
    window_depth: u16,
) -> Result<GeneratedSteps> {
    reevaluate_steps_from_reports_with_runtime(
        existing_steps,
        until_step,
        window_depth,
        FrontierRuntimeLimits::unlimited(),
    )
}

pub fn reevaluate_steps_from_reports_with_runtime(
    existing_steps: &[StepReport],
    until_step: u32,
    window_depth: u16,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<GeneratedSteps> {
    reevaluate_steps_from_reports_for_profile_with_runtime(
        existing_steps,
        until_step,
        window_depth,
        SearchProfile::StrictCanonGuarded,
        retention_runtime,
    )
}

pub fn reevaluate_steps_from_reports_for_profile_with_runtime(
    existing_steps: &[StepReport],
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<GeneratedSteps> {
    ensure_contiguous_steps(existing_steps)?;

    let prefix_len = existing_steps.len().min(until_step as usize);
    let prefix_telescopes = existing_steps
        .iter()
        .take(prefix_len)
        .map(|step| step.telescope.clone())
        .collect::<Vec<_>>();
    let mut steps = reevaluate_prefix_steps(&prefix_telescopes, window_depth)?;

    if u32::try_from(prefix_len).expect("prefix length exceeded u32") >= until_step {
        return Ok(GeneratedSteps {
            mode: StepGenerationMode::StepCheckpointReevaluate,
            steps: finalize_step_reports(steps, window_depth)?,
        });
    }

    if supports_live_atomic_search(until_step) {
        let resumed = search_bootstrap_from_prefix_for_profile_with_runtime(
            &prefix_telescopes,
            until_step,
            window_depth,
            search_profile,
            retention_runtime,
        )?
        .into_iter()
        .map(|step| step_to_report_with_provenance(step, StepProvenance::StepCheckpointReevaluate));
        steps.extend(resumed);
        return Ok(GeneratedSteps {
            mode: StepGenerationMode::StepCheckpointReevaluate,
            steps: finalize_step_reports(steps, window_depth)?,
        });
    }

    generate_steps_for_profile_with_runtime(
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
    )
}

pub fn replay_reference_steps(until_step: u32, window_depth: u16) -> Result<Vec<StepReport>> {
    let steps = replay_reference_steps_raw(until_step, window_depth)?;
    finalize_step_reports(steps, window_depth)
}

fn replay_reference_steps_raw(until_step: u32, window_depth: u16) -> Result<Vec<StepReport>> {
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
        let canon_evidence = step_canon_evidence(step_index, &accepted, objective_bar);

        steps.push(StepReport {
            step_index,
            label: step_label(step_index).to_owned(),
            search_profile: SearchProfile::Unknown,
            objective_bar,
            accepted,
            telescope: telescope.clone(),
            trace: evaluated.trace.clone(),
            near_misses: Vec::new(),
            search_stats: StepSearchStats {
                enumerated_candidates: 1,
                well_formed_candidates: 1,
                malformed_rejections: 0,
                malformed_rejection_reasons: BTreeMap::new(),
                admissibility_rejections: 0,
                admissibility_diagnostics: AdmissibilityDiagnostics::default(),
                full_telescopes_evaluated: 1,
                evaluated_candidates: 1,
                canonical_candidates: 1,
                semantically_minimal_candidates: 1,
                canonical_dedupe_prunes: 0,
                semantic_minimality_prunes: 0,
                dedupe_prunes: 0,
                minimality_prunes: 0,
                heuristic_drops: 0,
                prefixes_created: 0,
                prefix_states_explored: 0,
                prefix_states_merged_by_signature: 0,
                prefix_states_exact_pruned: 0,
                prefix_states_heuristic_dropped: 0,
                incremental_legality_cache_hits: 0,
                incremental_connectivity_shortcuts: 0,
                incremental_connectivity_fallbacks: 0,
                incremental_connectivity_prunes: 0,
                incremental_clause_family_filter_hits: 0,
                incremental_clause_family_prunes: 0,
                incremental_terminal_admissibility_hits: 0,
                incremental_terminal_admissibility_rejections: 0,
                prefix_frontier_hot_states: 0,
                prefix_frontier_cold_states: 0,
                retained_candidates: 1,
                frontier_hot_states: 0,
                frontier_cold_states: 0,
                frontier_resident_cold_states: 0,
                frontier_spill_states: 0,
                frontier_drops: 0,
                scored_candidate_distribution: single_candidate_distribution(
                    &evaluated,
                    objective_bar,
                ),
            },
            candidate_reports: vec![candidate_report(
                objective_bar,
                &evaluated,
                true,
                RetentionPolicy::default(),
            )],
            prune_reports: Vec::new(),
            frontier_policy: RetentionPolicy::default(),
            frontier_pressure: FrontierPressure::default(),
            prefix_frontier: PrefixFrontierArtifact::default(),
            provenance: StepProvenance::ReferenceReplay,
            canon_evidence,
            replay_ablation: ReplayAblation::default(),
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

#[cfg_attr(not(test), allow(dead_code))]
pub fn search_atomic_bootstrap_steps(
    until_step: u32,
    window_depth: u16,
) -> Result<Vec<StepReport>> {
    search_atomic_bootstrap_steps_with_runtime(
        until_step,
        window_depth,
        FrontierRuntimeLimits::unlimited(),
    )
}

pub fn search_atomic_bootstrap_steps_with_runtime(
    until_step: u32,
    window_depth: u16,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<StepReport>> {
    search_atomic_bootstrap_steps_for_profile_with_runtime(
        until_step,
        window_depth,
        SearchProfile::StrictCanonGuarded,
        retention_runtime,
    )
}

pub fn search_atomic_bootstrap_steps_for_profile_with_runtime(
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<StepReport>> {
    let steps = search_bootstrap_prefix_for_profile_with_runtime(
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
    )?
    .into_iter()
    .map(step_to_report)
    .collect();
    finalize_step_reports(steps, window_depth)
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

pub fn annotate_search_profile(steps: &mut [StepReport], search_profile: SearchProfile) {
    for step in steps {
        step.search_profile = search_profile;
    }
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

    let late_step_claim = render_late_step_claim(&last.canon_evidence.late_step_claim)
        .map(|claim| format!("\nlate_step_claim: {claim}"))
        .unwrap_or_default();

    format!(
        "run {run_id}\ncompleted_step: {}\nsearch_profile: {}\nlatest: step {} ({})\nnu: {}\nkappa: {}\ncharged_kappa: {}\nrho: {}\nbar: {}\nbar_distance: {}\nprovenance: {}\nrun_provenance: {}\nreplay_ablation: {}\nminimal_overshoot: {}{}",
        last.step_index,
        report_search_profile(steps),
        last.step_index,
        last.label,
        last.accepted.nu,
        last.accepted.clause_kappa,
        last.canon_evidence.charged_clause_kappa,
        last.accepted.rho,
        last.objective_bar,
        render_signed_rational(last.canon_evidence.bar_distance),
        last.provenance.as_str(),
        summarize_run_provenance(steps),
        summarize_replay_ablation(steps),
        last.accepted.overshoot,
        late_step_claim,
    )
}

pub fn render_debug_report(run_id: &str, steps: &[StepReport]) -> String {
    let mut lines = vec![
        format!("run {run_id} debug"),
        format!("search_profile: {}", report_search_profile(steps)),
        String::new(),
        "translation guide:".to_owned(),
    ];
    lines.extend(
        translation_guide()
            .into_iter()
            .map(|line| format!("  - {line}")),
    );
    if !steps.is_empty() {
        lines.push(String::new());
        lines.push(format!(
            "run provenance: {}",
            summarize_run_provenance(steps)
        ));
        lines.push(format!(
            "replay ablation: {}",
            summarize_replay_ablation(steps)
        ));
    }

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
        lines.push(format!("  provenance: {}", step.provenance.as_str()));
        if let Some(replay_ablation) = render_replay_ablation(&step.replay_ablation) {
            lines.push(format!("  replay ablation: {replay_ablation}"));
        }
        lines.push(format!(
            "  accepted minimal overshoot: {}",
            step.accepted.overshoot
        ));
        lines.push(format!(
            "  canon evidence: charged_kappa={} clears_bar={} bar_distance={}",
            step.canon_evidence.charged_clause_kappa,
            step.canon_evidence.clears_bar,
            render_signed_rational(step.canon_evidence.bar_distance)
        ));
        if let Some(claim) = render_late_step_claim(&step.canon_evidence.late_step_claim) {
            lines.push(format!("  late_step_claim: {claim}"));
        }
        lines.push(format!(
            "  search stats: full_evaluated={} canonical_dedupe_prunes={} semantic_minimality_prunes={} heuristic_drops={} retained_valid={} frontier_hot={} frontier_cold={} frontier_resident={} frontier_spill={} frontier_drops={}",
            step.search_stats.full_telescopes_evaluated,
            step.search_stats.canonical_dedupe_prunes,
            step.search_stats.semantic_minimality_prunes,
            step.search_stats.heuristic_drops,
            step.search_stats.retained_candidates,
            step.search_stats.frontier_hot_states,
            step.search_stats.frontier_cold_states,
            step.search_stats.frontier_resident_cold_states,
            step.search_stats.frontier_spill_states,
            step.search_stats.frontier_drops
        ));
        lines.push(format!(
            "  candidate funnel: enumerated={} well_formed={} malformed={} admissibility_rejected={} scored={} canonical={} semantically_minimal={} frontier_kept={} frontier_dropped={}",
            step.search_stats.enumerated_candidates,
            step.search_stats.well_formed_candidates,
            step.search_stats.malformed_rejections,
            step.search_stats.admissibility_rejections,
            step.search_stats.full_telescopes_evaluated,
            step.search_stats.canonical_candidates,
            step.search_stats.semantically_minimal_candidates,
            step.search_stats.frontier_hot_states + step.search_stats.frontier_cold_states,
            step.search_stats.frontier_drops
        ));
        if step.search_stats.prefix_states_explored > 0 {
            lines.push(format!(
                "  prefix frontier: created={} explored={} merged={} exact_pruned={} heuristic_dropped={} hot={} cold={}",
                step.search_stats.prefixes_created,
                step.search_stats.prefix_states_explored,
                step.search_stats.prefix_states_merged_by_signature,
                step.search_stats.prefix_states_exact_pruned,
                step.search_stats.prefix_states_heuristic_dropped,
                step.search_stats.prefix_frontier_hot_states,
                step.search_stats.prefix_frontier_cold_states
            ));
            if step.search_stats.incremental_legality_cache_hits > 0
                || step.search_stats.incremental_connectivity_shortcuts > 0
                || step.search_stats.incremental_connectivity_fallbacks > 0
                || step.search_stats.incremental_connectivity_prunes > 0
                || step.search_stats.incremental_clause_family_filter_hits > 0
                || step.search_stats.incremental_clause_family_prunes > 0
                || step.search_stats.incremental_terminal_admissibility_hits > 0
                || step
                    .search_stats
                    .incremental_terminal_admissibility_rejections
                    > 0
            {
                lines.push(format!(
                    "  prefix memo: legality_hits={} connectivity_shortcuts={} connectivity_fallbacks={} connectivity_prunes={} clause_family_hits={} clause_family_prunes={} terminal_admissibility_hits={} terminal_admissibility_rejections={}",
                    step.search_stats.incremental_legality_cache_hits,
                    step.search_stats.incremental_connectivity_shortcuts,
                    step.search_stats.incremental_connectivity_fallbacks,
                    step.search_stats.incremental_connectivity_prunes,
                    step.search_stats.incremental_clause_family_filter_hits,
                    step.search_stats.incremental_clause_family_prunes,
                    step.search_stats.incremental_terminal_admissibility_hits,
                    step.search_stats.incremental_terminal_admissibility_rejections
                ));
            }
        }
        lines.push(format!(
            "  admissibility diagnostics: exact_legality={} structural_cap={} de-prioritized={} focus_aligned={}",
            step.search_stats
                .admissibility_diagnostics
                .exact_legality_rejections,
            step.search_stats
                .admissibility_diagnostics
                .structural_debt_cap_rejections,
            step.search_stats.admissibility_diagnostics.admitted_deprioritized,
            step.search_stats.admissibility_diagnostics.admitted_focus_aligned
        ));
        if !step
            .search_stats
            .admissibility_diagnostics
            .reason_counts
            .is_empty()
        {
            lines.push(format!(
                "  admissibility reasons: {}",
                summarize_named_counts(&step.search_stats.admissibility_diagnostics.reason_counts)
            ));
        }
        if !step.search_stats.malformed_rejection_reasons.is_empty() {
            lines.push(format!(
                "  malformed rejects: {}",
                summarize_named_counts(&step.search_stats.malformed_rejection_reasons)
            ));
        }
        if step
            .search_stats
            .scored_candidate_distribution
            .candidate_count
            > 0
        {
            lines.push(format!(
                "  scored candidates: clears_bar={} below_bar={} clause_kappa={}",
                step.search_stats.scored_candidate_distribution.clears_bar,
                step.search_stats.scored_candidate_distribution.below_bar,
                summarize_u16_counts(
                    &step
                        .search_stats
                        .scored_candidate_distribution
                        .clause_kappa_counts
                )
            ));
            lines.push(format!(
                "  scored nu summary: min={} median={} max={} avg={}",
                step.search_stats
                    .scored_candidate_distribution
                    .nu_summary
                    .min,
                step.search_stats
                    .scored_candidate_distribution
                    .nu_summary
                    .median,
                step.search_stats
                    .scored_candidate_distribution
                    .nu_summary
                    .max,
                step.search_stats
                    .scored_candidate_distribution
                    .nu_summary
                    .average
            ));
            lines.push(format!(
                "  scored rho summary: min={} median={} max={} avg={}",
                step.search_stats
                    .scored_candidate_distribution
                    .rho_summary
                    .min,
                step.search_stats
                    .scored_candidate_distribution
                    .rho_summary
                    .median,
                step.search_stats
                    .scored_candidate_distribution
                    .rho_summary
                    .max,
                step.search_stats
                    .scored_candidate_distribution
                    .rho_summary
                    .average
            ));
        }
        lines.push(format!(
            "  retention policy: focus={} quotas(focus={}, bridge={}, support={}, macro={}) cold_limit={}",
            retention_focus_label(step.frontier_policy.focus),
            step.frontier_policy.focus_quota,
            step.frontier_policy.bridge_quota,
            step.frontier_policy.support_quota,
            step.frontier_policy.macro_quota,
            step.frontier_policy.cold_limit
        ));
        lines.push(format!(
            "  frontier pressure: state={} action={} rss_bytes={} requested_cold={} retained_cold={} resident_cold={} spill_backed={} dropped={}",
            step.frontier_pressure.governor_state.as_str(),
            step.frontier_pressure.pressure_action.as_str(),
            step.frontier_pressure.rss_bytes,
            step.frontier_pressure.requested_cold_limit,
            step.frontier_pressure.retained_cold_limit,
            step.frontier_pressure.resident_cold_limit,
            step.frontier_pressure.spill_backed_cold_records,
            step.frontier_pressure.dropped_cold_records
        ));
        lines.push(format!(
            "  prune classes: quotient_dedupe={} sound_minimality={} heuristic_shaping={}",
            step.search_stats.dedupe_prunes,
            step.search_stats.minimality_prunes,
            step.search_stats.heuristic_drops
        ));
        let prune_samples = step
            .prune_reports
            .iter()
            .filter(|report| report.prune_class != PruneReportClass::HeuristicShaping)
            .collect::<Vec<_>>();
        if !prune_samples.is_empty() {
            lines.push("  prune samples:".to_owned());
            for prune in prune_samples {
                lines.push(render_prune_report(prune));
            }
        }
        lines.push(format!(
            "  frontier retention: hot={} resident_cold={} spill_backed={} dropped={} delta={:+}",
            step.search_stats.frontier_hot_states,
            step.search_stats.frontier_resident_cold_states,
            step.search_stats.frontier_spill_states,
            step.search_stats.frontier_drops,
            frontier_retention_delta(step)
        ));
        lines.push(format!(
            "  retention classes (hot/resident/spill/dropped): {}",
            retention_class_breakdown(step)
        ));
        let frontier_drop_samples = step
            .prune_reports
            .iter()
            .filter(|report| report.prune_class == PruneReportClass::HeuristicShaping)
            .collect::<Vec<_>>();
        if !frontier_drop_samples.is_empty() {
            lines.push("  frontier drop samples:".to_owned());
            for prune in frontier_drop_samples {
                lines.push(render_prune_report(prune));
            }
        }

        if !step.near_misses.is_empty() {
            lines.push("  near misses:".to_owned());
            for miss in &step.near_misses {
                lines.push(format!(
                    "    - {} | status={} | nu={} kappa={} bits={} canonical={}",
                    miss.candidate_hash,
                    miss.status,
                    miss.nu,
                    miss.clause_kappa,
                    miss.bit_kappa,
                    miss.canonical_hash
                ));
            }
        }

        if step.candidate_reports.is_empty() {
            lines.push("  retained valid candidates: none recorded".to_owned());
            continue;
        }

        lines.push("  retained valid candidates:".to_owned());
        for candidate in &step.candidate_reports {
            lines.push(format!(
                "    - {} | frontier={} | rho={} | nu={} | kappa={} | bits={} | {}",
                candidate_status_heading(candidate.status, candidate.distance_to_bar),
                frontier_retention_label(candidate.frontier_retention),
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
    step_to_report_with_provenance(step, StepProvenance::AtomicBootstrapSearch)
}

fn step_to_report_with_provenance(
    step: AtomicSearchStep,
    provenance: StepProvenance,
) -> StepReport {
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
    let canon_evidence = step_canon_evidence(step.step_index, &accepted, step.objective_bar);
    let mut candidate_reports = step
        .retained_candidates
        .iter()
        .map(|candidate| {
            candidate_report(
                step.objective_bar,
                candidate,
                candidate.candidate_hash == accepted.candidate_hash,
                step.retention_policy,
            )
        })
        .collect::<Vec<_>>();
    candidate_reports.sort_by_key(candidate_report_rank);
    let frontier_stats =
        annotate_frontier_retention(&step.frontier_retention, &mut candidate_reports);
    let prune_reports = build_prune_reports(
        step.objective_bar,
        step.retention_policy,
        &step.dedupe_pruned_candidates,
        &step.minimality_pruned_candidates,
        &candidate_reports,
    );

    StepReport {
        step_index: step.step_index,
        label: step_label(step.step_index).to_owned(),
        search_profile: SearchProfile::Unknown,
        objective_bar: step.objective_bar,
        accepted,
        telescope: step.telescope,
        trace: step.accepted.trace,
        near_misses: step.near_misses,
        search_stats: StepSearchStats {
            enumerated_candidates: step.enumerated_candidates,
            well_formed_candidates: step.well_formed_candidates,
            malformed_rejections: step.malformed_rejections,
            malformed_rejection_reasons: step.malformed_rejection_reasons,
            admissibility_rejections: step.admissibility_rejections,
            admissibility_diagnostics: step.admissibility_diagnostics,
            full_telescopes_evaluated: step.full_telescopes_evaluated,
            evaluated_candidates: step.evaluated_candidates,
            canonical_candidates: step.canonical_candidates,
            semantically_minimal_candidates: step.semantically_minimal_candidates,
            canonical_dedupe_prunes: step.canonical_dedupe_prunes,
            semantic_minimality_prunes: step.semantic_minimality_prunes,
            dedupe_prunes: step.dedupe_prunes,
            minimality_prunes: step.minimality_prunes,
            heuristic_drops: step.heuristic_drops,
            prefixes_created: step.prefixes_created,
            prefix_states_explored: step.prefix_states_explored,
            prefix_states_merged_by_signature: step.prefix_states_merged_by_signature,
            prefix_states_exact_pruned: step.prefix_states_exact_pruned,
            prefix_states_heuristic_dropped: step.prefix_states_heuristic_dropped,
            incremental_legality_cache_hits: step.incremental_legality_cache_hits,
            incremental_connectivity_shortcuts: step.incremental_connectivity_shortcuts,
            incremental_connectivity_fallbacks: step.incremental_connectivity_fallbacks,
            incremental_connectivity_prunes: step.incremental_connectivity_prunes,
            incremental_clause_family_filter_hits: step.incremental_clause_family_filter_hits,
            incremental_clause_family_prunes: step.incremental_clause_family_prunes,
            incremental_terminal_admissibility_hits: step.incremental_terminal_admissibility_hits,
            incremental_terminal_admissibility_rejections: step
                .incremental_terminal_admissibility_rejections,
            prefix_frontier_hot_states: step.prefix_frontier_hot_states,
            prefix_frontier_cold_states: step.prefix_frontier_cold_states,
            retained_candidates: candidate_reports.len(),
            frontier_hot_states: frontier_stats.frontier_hot_states,
            frontier_cold_states: frontier_stats.frontier_cold_states,
            frontier_resident_cold_states: frontier_stats.frontier_resident_cold_states,
            frontier_spill_states: frontier_stats.frontier_spill_states,
            frontier_drops: frontier_stats.frontier_drops,
            scored_candidate_distribution: step.scored_candidate_distribution,
        },
        candidate_reports,
        prune_reports,
        frontier_policy: step.retention_policy,
        frontier_pressure: step.frontier_pressure,
        prefix_frontier: PrefixFrontierArtifact {
            hot_states: step
                .frontier_window
                .hot
                .iter()
                .copied()
                .map(FrontierStateSnapshot::from)
                .collect(),
            cold_states: step
                .frontier_window
                .cold
                .iter()
                .copied()
                .map(FrontierStateSnapshot::from)
                .collect(),
            dedupe_keys: step.frontier_dedupe_keys.iter().cloned().collect(),
        },
        provenance,
        canon_evidence,
        replay_ablation: ReplayAblation::default(),
    }
}

fn candidate_report(
    objective_bar: Rational,
    candidate: &pen_search::expand::ExpandedCandidate,
    accepted: bool,
    retention_policy: RetentionPolicy,
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
        frontier_retention: FrontierRetention::NotRecorded,
        retention_class: retention_policy.classify(candidate.retention_signals()),
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

fn build_prune_reports(
    objective_bar: Rational,
    retention_policy: RetentionPolicy,
    dedupe_pruned_candidates: &[DedupePruneEvidence],
    minimality_pruned_candidates: &[MinimalityPruneEvidence],
    candidate_reports: &[CandidateReport],
) -> Vec<PruneReport> {
    let mut reports = Vec::new();

    reports.extend(dedupe_pruned_candidates.iter().map(|evidence| {
        prune_report_from_candidate(
            objective_bar,
            &evidence.candidate,
            retention_policy,
            PruneReportClass::QuotientDedupe,
            format!(
                "duplicate canonical representative; first_seen={}",
                evidence.first_seen_candidate_hash
            ),
        )
    }));
    reports.extend(minimality_pruned_candidates.iter().map(|evidence| {
        prune_report_from_candidate(
            objective_bar,
            &evidence.candidate,
            retention_policy,
            PruneReportClass::SoundMinimality,
            format!(
                "admissible_bar_clear_subbundles={}",
                evidence.admissible_bar_clear_subbundles
            ),
        )
    }));
    reports.extend(
        candidate_reports
            .iter()
            .filter(|candidate| candidate.frontier_retention == FrontierRetention::Dropped)
            .take(3)
            .map(|candidate| {
                prune_report_from_candidate_report(
                    candidate,
                    PruneReportClass::HeuristicShaping,
                    format!(
                        "dropped by bounded frontier retention focus={}",
                        retention_focus_label(retention_policy.focus)
                    ),
                )
            }),
    );

    reports
}

fn prune_report_from_candidate(
    objective_bar: Rational,
    candidate: &pen_search::expand::ExpandedCandidate,
    retention_policy: RetentionPolicy,
    prune_class: PruneReportClass,
    note: String,
) -> PruneReport {
    PruneReport {
        prune_class,
        candidate_hash: candidate.candidate_hash.clone(),
        canonical_hash: candidate.canonical_hash.clone(),
        bit_kappa: candidate.bit_kappa,
        clause_kappa: candidate.clause_kappa,
        nu: candidate.nu,
        rho: candidate.rho,
        bar_delta: candidate.rho - objective_bar,
        retention_class: retention_policy.classify(candidate.retention_signals()),
        headline: describe_candidate(&candidate.telescope, candidate.telescope_class),
        note,
    }
}

fn prune_report_from_candidate_report(
    candidate: &CandidateReport,
    prune_class: PruneReportClass,
    note: String,
) -> PruneReport {
    let bar_delta = match candidate.status {
        CandidateStatus::BelowBar => Rational::zero() - candidate.distance_to_bar,
        CandidateStatus::AcceptedMinimalOvershoot | CandidateStatus::ClearsBarHigherOvershoot => {
            candidate.distance_to_bar
        }
    };

    PruneReport {
        prune_class,
        candidate_hash: candidate.candidate_hash.clone(),
        canonical_hash: candidate.canonical_hash.clone(),
        bit_kappa: candidate.bit_kappa,
        clause_kappa: candidate.clause_kappa,
        nu: candidate.nu,
        rho: candidate.rho,
        bar_delta,
        retention_class: candidate.retention_class,
        headline: candidate.headline.clone(),
        note,
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct FrontierRetentionStats {
    frontier_hot_states: usize,
    frontier_cold_states: usize,
    frontier_resident_cold_states: usize,
    frontier_spill_states: usize,
    frontier_drops: usize,
}

fn candidate_report_rank(
    candidate: &CandidateReport,
) -> (u8, Rational, u16, std::cmp::Reverse<u16>, String) {
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

fn single_candidate_distribution(
    candidate: &pen_search::expand::ExpandedCandidate,
    objective_bar: Rational,
) -> CandidateScoreDistribution {
    let clears_bar = usize::from(candidate.rho >= objective_bar);
    let below_bar = 1usize.saturating_sub(clears_bar);
    CandidateScoreDistribution {
        candidate_count: 1,
        clears_bar,
        below_bar,
        clause_kappa_counts: BTreeMap::from([(candidate.clause_kappa, 1usize)]),
        nu_summary: pen_search::engine::IntegerDistributionSummary {
            min: candidate.nu,
            median: candidate.nu,
            max: candidate.nu,
            average: Rational::from_integer(i64::from(candidate.nu)),
        },
        rho_summary: pen_search::engine::RationalDistributionSummary {
            min: candidate.rho,
            median: candidate.rho,
            max: candidate.rho,
            average: candidate.rho,
        },
    }
}

fn reevaluate_prefix_steps(telescopes: &[Telescope], window_depth: u16) -> Result<Vec<StepReport>> {
    let mut library: Library = Vec::new();
    let mut history: Vec<DiscoveryRecord> = Vec::new();
    let mut steps = Vec::new();

    for (offset, telescope) in telescopes.iter().enumerate() {
        let step_index = u32::try_from(offset + 1).expect("reevaluated step count exceeded u32");
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
        let canon_evidence = step_canon_evidence(step_index, &accepted, objective_bar);

        steps.push(StepReport {
            step_index,
            label: step_label(step_index).to_owned(),
            search_profile: SearchProfile::Unknown,
            objective_bar,
            accepted,
            telescope: telescope.clone(),
            trace: evaluated.trace.clone(),
            near_misses: Vec::new(),
            search_stats: StepSearchStats {
                enumerated_candidates: 1,
                well_formed_candidates: 1,
                malformed_rejections: 0,
                malformed_rejection_reasons: BTreeMap::new(),
                admissibility_rejections: 0,
                admissibility_diagnostics: AdmissibilityDiagnostics::default(),
                full_telescopes_evaluated: 1,
                evaluated_candidates: 1,
                canonical_candidates: 1,
                semantically_minimal_candidates: 1,
                canonical_dedupe_prunes: 0,
                semantic_minimality_prunes: 0,
                dedupe_prunes: 0,
                minimality_prunes: 0,
                heuristic_drops: 0,
                prefixes_created: 0,
                prefix_states_explored: 0,
                prefix_states_merged_by_signature: 0,
                prefix_states_exact_pruned: 0,
                prefix_states_heuristic_dropped: 0,
                incremental_legality_cache_hits: 0,
                incremental_connectivity_shortcuts: 0,
                incremental_connectivity_fallbacks: 0,
                incremental_connectivity_prunes: 0,
                incremental_clause_family_filter_hits: 0,
                incremental_clause_family_prunes: 0,
                incremental_terminal_admissibility_hits: 0,
                incremental_terminal_admissibility_rejections: 0,
                prefix_frontier_hot_states: 0,
                prefix_frontier_cold_states: 0,
                retained_candidates: 1,
                frontier_hot_states: 0,
                frontier_cold_states: 0,
                frontier_resident_cold_states: 0,
                frontier_spill_states: 0,
                frontier_drops: 0,
                scored_candidate_distribution: single_candidate_distribution(
                    &evaluated,
                    objective_bar,
                ),
            },
            candidate_reports: vec![candidate_report(
                objective_bar,
                &evaluated,
                true,
                RetentionPolicy::default(),
            )],
            prune_reports: Vec::new(),
            frontier_policy: RetentionPolicy::default(),
            frontier_pressure: FrontierPressure::default(),
            prefix_frontier: PrefixFrontierArtifact::default(),
            provenance: StepProvenance::StepCheckpointReevaluate,
            canon_evidence,
            replay_ablation: ReplayAblation::default(),
        });

        history.push(DiscoveryRecord::new(
            step_index,
            u32::from(evaluated.nu),
            u32::from(evaluated.clause_kappa),
        ));
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }

    Ok(steps)
}

fn ensure_contiguous_steps(existing_steps: &[StepReport]) -> Result<()> {
    for (offset, step) in existing_steps.iter().enumerate() {
        let expected = u32::try_from(offset + 1).expect("step count exceeded u32");
        if step.step_index != expected {
            anyhow::bail!(
                "stored step artifacts are not a contiguous 1-based prefix: expected step {expected}, found {}",
                step.step_index
            );
        }
    }
    Ok(())
}

fn annotate_frontier_retention(
    outcome: &FrontierRetentionOutcome,
    candidate_reports: &mut [CandidateReport],
) -> FrontierRetentionStats {
    let mut stats = FrontierRetentionStats::default();
    for candidate in candidate_reports.iter_mut() {
        candidate.frontier_retention = match candidate.status {
            CandidateStatus::AcceptedMinimalOvershoot
            | CandidateStatus::ClearsBarHigherOvershoot => {
                stats.frontier_hot_states += 1;
                FrontierRetention::Hot
            }
            CandidateStatus::BelowBar
                if outcome
                    .resident_cold_candidates
                    .contains(&candidate.candidate_hash) =>
            {
                stats.frontier_cold_states += 1;
                stats.frontier_resident_cold_states += 1;
                FrontierRetention::ColdResident
            }
            CandidateStatus::BelowBar
                if outcome
                    .spill_cold_candidates
                    .contains(&candidate.candidate_hash) =>
            {
                stats.frontier_cold_states += 1;
                stats.frontier_spill_states += 1;
                FrontierRetention::SpillBacked
            }
            CandidateStatus::BelowBar => {
                stats.frontier_drops += 1;
                FrontierRetention::Dropped
            }
        };
    }
    stats
}

fn step_canon_evidence(
    step_index: u32,
    accepted: &AcceptedCandidate,
    objective_bar: Rational,
) -> CanonEvidence {
    CanonEvidence {
        charged_clause_kappa: accepted.clause_kappa,
        bar_distance: accepted.rho - objective_bar,
        clears_bar: accepted.rho >= objective_bar,
        late_step_claim: late_step_claim(step_index, accepted.nu),
    }
}

fn late_step_claim(step_index: u32, accepted_nu: u16) -> LateStepClaim {
    if step_index == 15 {
        return LateStepClaim {
            status: LateStepClaimStatus::ExecutableCanon,
            adopted_step: 15,
            adopted_label: "DCT".to_owned(),
            adopted_nu: 103,
            matches_accepted: accepted_nu == 103,
            note: "Historical late-step alternatives remain provenance only.".to_owned(),
        };
    }

    LateStepClaim::default()
}

fn render_signed_rational(value: Rational) -> String {
    if value.is_positive() {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

fn render_late_step_claim(claim: &LateStepClaim) -> Option<String> {
    match claim.status {
        LateStepClaimStatus::NotApplicable => None,
        LateStepClaimStatus::ExecutableCanon => Some(format!(
            "executable_canon step {} {} nu={} matches_current_acceptance={} ({})",
            claim.adopted_step,
            claim.adopted_label,
            claim.adopted_nu,
            claim.matches_accepted,
            claim.note
        )),
        LateStepClaimStatus::HistoricalProvenanceOnly => Some(format!(
            "historical_provenance_only step {} {} nu={} ({})",
            claim.adopted_step, claim.adopted_label, claim.adopted_nu, claim.note
        )),
    }
}

fn finalize_step_reports(mut steps: Vec<StepReport>, window_depth: u16) -> Result<Vec<StepReport>> {
    annotate_replay_ablation(&mut steps, window_depth)?;
    Ok(steps)
}

fn annotate_replay_ablation(steps: &mut [StepReport], window_depth: u16) -> Result<()> {
    let Some(last_step) = steps.last().map(|step| step.step_index) else {
        return Ok(());
    };
    let reference_steps = replay_reference_steps_raw(last_step.min(15), window_depth)?;
    let reference_by_step = reference_steps
        .into_iter()
        .map(|step| (step.step_index, step))
        .collect::<BTreeMap<_, _>>();

    for step in steps {
        let Some(reference) = reference_by_step.get(&step.step_index) else {
            step.replay_ablation = ReplayAblation {
                status: ReplayAblationStatus::NotApplicable,
                note: "reference replay baseline unavailable beyond the current 15-step corpus"
                    .to_owned(),
                ..ReplayAblation::default()
            };
            continue;
        };

        let status = if step.accepted.candidate_hash == reference.accepted.candidate_hash
            && step.accepted.canonical_hash == reference.accepted.canonical_hash
            && step.accepted.nu == reference.accepted.nu
            && step.accepted.clause_kappa == reference.accepted.clause_kappa
            && step.accepted.rho == reference.accepted.rho
            && step.objective_bar == reference.objective_bar
            && step.accepted.overshoot == reference.accepted.overshoot
        {
            ReplayAblationStatus::MatchesReferenceReplay
        } else {
            ReplayAblationStatus::DivergesFromReferenceReplay
        };

        step.replay_ablation = ReplayAblation {
            status,
            reference_candidate_hash: reference.accepted.candidate_hash.clone(),
            reference_canonical_hash: reference.accepted.canonical_hash.clone(),
            rho_delta: step.accepted.rho - reference.accepted.rho,
            objective_bar_delta: step.objective_bar - reference.objective_bar,
            overshoot_delta: step.accepted.overshoot - reference.accepted.overshoot,
            nu_delta: i32::from(step.accepted.nu) - i32::from(reference.accepted.nu),
            clause_kappa_delta: i32::from(step.accepted.clause_kappa)
                - i32::from(reference.accepted.clause_kappa),
            note: if status == ReplayAblationStatus::MatchesReferenceReplay {
                "matches reference replay baseline".to_owned()
            } else {
                format!(
                    "current provenance {} diverges from reference replay",
                    step.provenance.as_str()
                )
            },
        };
    }

    Ok(())
}

pub(crate) fn render_replay_ablation(ablation: &ReplayAblation) -> Option<String> {
    match ablation.status {
        ReplayAblationStatus::NotRecorded => None,
        ReplayAblationStatus::NotApplicable => Some(ablation.status.as_str().to_owned()),
        ReplayAblationStatus::MatchesReferenceReplay
        | ReplayAblationStatus::DivergesFromReferenceReplay => Some(format!(
            "{} rho_delta={} bar_delta={} overshoot_delta={} nu_delta={} kappa_delta={} reference={} ({})",
            ablation.status.as_str(),
            render_signed_rational(ablation.rho_delta),
            render_signed_rational(ablation.objective_bar_delta),
            render_signed_rational(ablation.overshoot_delta),
            ablation.nu_delta,
            ablation.clause_kappa_delta,
            ablation.reference_candidate_hash,
            ablation.note
        )),
    }
}

fn render_prune_report(prune: &PruneReport) -> String {
    format!(
        "    - {} | bar_delta={} | rho={} | nu={} | kappa={} | retention={:?} | {} | {}",
        prune.prune_class.as_str(),
        render_signed_rational(prune.bar_delta),
        prune.rho,
        prune.nu,
        prune.clause_kappa,
        prune.retention_class,
        prune.candidate_hash,
        prune.note
    )
}

pub(crate) fn summarize_prune_reports(prunes: &[PruneReport]) -> String {
    if prunes.is_empty() {
        return "none recorded".to_owned();
    }

    let mut counts = BTreeMap::new();
    for prune in prunes {
        *counts.entry(prune.prune_class.as_str()).or_insert(0_usize) += 1;
    }

    counts
        .into_iter()
        .map(|(label, count)| format!("{label} x{count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn frontier_retention_label(retention: FrontierRetention) -> &'static str {
    match retention {
        FrontierRetention::NotRecorded => "not-recorded",
        FrontierRetention::Hot => "hot",
        FrontierRetention::ColdResident => "cold-resident",
        FrontierRetention::SpillBacked => "spill-backed",
        FrontierRetention::Dropped => "dropped",
    }
}

fn retention_focus_label(focus: pen_type::obligations::RetentionFocus) -> &'static str {
    match focus {
        pen_type::obligations::RetentionFocus::OpenBand => "open_band",
        pen_type::obligations::RetentionFocus::Former => "former",
        pen_type::obligations::RetentionFocus::Hit => "hit",
        pen_type::obligations::RetentionFocus::Axiomatic => "axiomatic",
        pen_type::obligations::RetentionFocus::Modal => "modal",
        pen_type::obligations::RetentionFocus::Temporal => "temporal",
    }
}

fn summarize_run_provenance(steps: &[StepReport]) -> String {
    let mut counts = BTreeMap::new();
    for step in steps {
        *counts.entry(step.provenance.as_str()).or_insert(0_usize) += 1;
    }

    counts
        .into_iter()
        .map(|(label, count)| format!("{label} x{count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn summarize_replay_ablation(steps: &[StepReport]) -> String {
    let mut counts = BTreeMap::new();
    for step in steps {
        if step.replay_ablation.status == ReplayAblationStatus::NotRecorded {
            continue;
        }
        *counts
            .entry(step.replay_ablation.status.as_str())
            .or_insert(0_usize) += 1;
    }

    if counts.is_empty() {
        return "not recorded".to_owned();
    }

    counts
        .into_iter()
        .map(|(label, count)| format!("{label} x{count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn summarize_named_counts(counts: &BTreeMap<String, usize>) -> String {
    counts
        .iter()
        .map(|(label, count)| format!("{label}={count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn summarize_u16_counts(counts: &BTreeMap<u16, usize>) -> String {
    counts
        .iter()
        .map(|(label, count)| format!("{label}:{count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn frontier_retention_delta(step: &StepReport) -> i64 {
    let kept = step.search_stats.frontier_hot_states + step.search_stats.frontier_cold_states;
    let dropped = step.search_stats.frontier_drops;
    kept as i64 - dropped as i64
}

fn report_search_profile(steps: &[StepReport]) -> &'static str {
    steps
        .iter()
        .rev()
        .map(|step| step.search_profile)
        .find(|profile| *profile != SearchProfile::Unknown)
        .unwrap_or(SearchProfile::Unknown)
        .as_str()
}

fn retention_class_breakdown(step: &StepReport) -> String {
    if !step
        .candidate_reports
        .iter()
        .any(|candidate| candidate.frontier_retention != FrontierRetention::NotRecorded)
    {
        return "not recorded".to_owned();
    }

    let mut counts = RetentionClassCounts::default();
    for candidate in &step.candidate_reports {
        let bucket = match candidate.retention_class {
            RetentionClass::RareFocusHead => &mut counts.rare_focus,
            RetentionClass::RareBridgeHead => &mut counts.rare_bridge,
            RetentionClass::StructuralSupport => &mut counts.structural_support,
            RetentionClass::GenericMacro => &mut counts.generic_macro,
        };
        match candidate.frontier_retention {
            FrontierRetention::Hot => bucket.hot += 1,
            FrontierRetention::ColdResident => bucket.cold += 1,
            FrontierRetention::SpillBacked => bucket.spill += 1,
            FrontierRetention::Dropped => bucket.dropped += 1,
            FrontierRetention::NotRecorded => {}
        }
    }

    format!(
        "focus={} bridge={} support={} macro={}",
        counts.rare_focus.render(),
        counts.rare_bridge.render(),
        counts.structural_support.render(),
        counts.generic_macro.render()
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RetentionClassCount {
    hot: usize,
    cold: usize,
    spill: usize,
    dropped: usize,
}

impl RetentionClassCount {
    fn render(self) -> String {
        format!("{}/{}/{}/{}", self.hot, self.cold, self.spill, self.dropped)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RetentionClassCounts {
    rare_focus: RetentionClassCount,
    rare_bridge: RetentionClassCount,
    structural_support: RetentionClassCount,
    generic_macro: RetentionClassCount,
}

#[cfg(test)]
mod tests {
    use super::{
        CandidateStatus, FrontierRetention, StepGenerationMode, StepProvenance,
        extend_steps_from_reports, generate_steps, reevaluate_steps_from_reports,
        render_debug_report, render_standard_report, replay_reference_steps,
        search_atomic_bootstrap_steps, search_atomic_bootstrap_steps_with_runtime,
    };
    use pen_search::diversify::FrontierRuntimeLimits;
    use pen_store::{
        memory::{GovernorConfig, PressureAction},
        spill::SpillConfig,
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
        assert!(render_standard_report("run-1", &steps).contains("provenance: reference_replay"));
        assert!(render_standard_report("run-1", &steps).contains("bar_distance:"));
        assert!(render_standard_report("run-1", &steps).contains("run_provenance:"));
        assert!(render_standard_report("run-1", &steps).contains("replay_ablation:"));
        assert!(render_standard_report("run-1", &steps).contains("minimal_overshoot:"));
        assert!(render_debug_report("run-1", &steps).contains("translation guide:"));
        assert!(render_debug_report("run-1", &steps).contains("run provenance:"));
        assert!(render_debug_report("run-1", &steps).contains("replay ablation:"));
        assert!(render_debug_report("run-1", &steps).contains("canon evidence:"));
        assert!(
            render_debug_report("run-1", &steps)
                .contains("ACCEPTED clears bar with minimal overshoot")
        );
    }

    #[test]
    fn live_search_reports_candidate_breakdowns() {
        let steps = search_atomic_bootstrap_steps(4, 2).expect("bootstrap search should succeed");
        let debug = render_debug_report("run-1", &steps);
        let step = &steps[3];

        assert!(
            step.candidate_reports
                .iter()
                .any(|candidate| candidate.status == CandidateStatus::AcceptedMinimalOvershoot)
        );
        assert_eq!(
            step.search_stats.enumerated_candidates,
            step.search_stats.well_formed_candidates + step.search_stats.malformed_rejections
        );
        assert!(step.search_stats.well_formed_candidates >= step.search_stats.evaluated_candidates);
        assert_eq!(
            step.search_stats
                .scored_candidate_distribution
                .candidate_count,
            step.search_stats.evaluated_candidates
        );
        assert!(debug.contains("retained valid candidates:"));
        assert!(debug.contains("candidate funnel:"));
        assert!(debug.contains("scored candidates:"));
        assert!(debug.contains("scored nu summary:"));
        assert!(debug.contains("scored rho summary:"));
        assert!(debug.contains("class: Former"));
        assert!(debug.contains("c01 [introduction]"));
        assert!(debug.contains("fun x1 ->"));
    }

    #[test]
    fn resume_extension_continues_from_stored_step_reports() {
        let prefix = search_atomic_bootstrap_steps(4, 2).expect("bootstrap search should succeed");
        let generated = extend_steps_from_reports(&prefix, 6, 2)
            .expect("artifact-backed extension should succeed");

        assert_eq!(generated.mode, StepGenerationMode::StepCheckpointResume);
        assert_eq!(generated.steps.len(), 6);
        assert_eq!(
            generated.steps[4].provenance,
            StepProvenance::StepCheckpointResume
        );
        assert_eq!(
            generated.steps[5].telescope,
            pen_core::telescope::Telescope::reference(6)
        );
    }

    #[test]
    fn reevaluate_extension_rebuilds_prefix_reports_before_continuing() {
        let prefix = search_atomic_bootstrap_steps(4, 2).expect("bootstrap search should succeed");
        let generated = reevaluate_steps_from_reports(&prefix, 6, 2)
            .expect("reevaluated extension should succeed");

        assert_eq!(generated.mode, StepGenerationMode::StepCheckpointReevaluate);
        assert_eq!(generated.steps.len(), 6);
        assert_eq!(
            generated.steps[0].provenance,
            StepProvenance::StepCheckpointReevaluate
        );
        assert_eq!(
            generated.steps[3].telescope,
            pen_core::telescope::Telescope::reference(4)
        );
        assert_eq!(
            generated.steps[5].telescope,
            pen_core::telescope::Telescope::reference(6)
        );
    }

    #[test]
    fn late_bootstrap_reports_frontier_retention_annotations() {
        let steps = search_atomic_bootstrap_steps(15, 2).expect("bootstrap search should succeed");
        let late = &steps[14];
        let debug = render_debug_report("run-1", &steps);

        assert!(late.search_stats.frontier_hot_states > 0);
        assert!(
            late.candidate_reports
                .iter()
                .all(|candidate| candidate.frontier_retention != FrontierRetention::NotRecorded)
        );
        assert!(debug.contains("frontier=hot"));
        assert!(debug.contains("frontier retention: hot="));
        assert!(debug.contains("retention classes (hot/resident/spill/dropped):"));
        assert!(debug.contains("provenance: atomic_bootstrap_search"));
        assert!(debug.contains("replay ablation: matches_reference_replay"));
        assert!(debug.contains("retention policy: focus=temporal"));
        assert!(debug.contains("frontier pressure: state=green action=none"));
        assert!(debug.contains("late_step_claim: executable_canon step 15 DCT nu=103"));
        assert!(
            render_standard_report("run-1", &steps)
                .contains("late_step_claim: executable_canon step 15 DCT nu=103")
        );
        assert_eq!(
            late.replay_ablation.status,
            super::ReplayAblationStatus::MatchesReferenceReplay
        );
    }

    #[test]
    fn pressure_hardened_bootstrap_surfaces_spill_pressure() {
        let steps = search_atomic_bootstrap_steps_with_runtime(
            15,
            2,
            FrontierRuntimeLimits {
                worker_count: 1,
                governor: GovernorConfig {
                    green_limit_bytes: 1,
                    yellow_limit_bytes: 2,
                    orange_limit_bytes: 1_000_000,
                    red_limit_bytes: 2_000_000,
                    hard_limit_bytes: 4_000_000,
                },
                spill: SpillConfig {
                    max_records_per_shard: 32,
                    max_dedupe_keys_per_segment: 32,
                    resident_cold_records: 1,
                },
                record_bytes: 64,
                dedupe_bytes_per_record: 32,
                worker_scratch_bytes: 0,
                checkpoint_bytes: 0,
                spill_buffer_bytes: 0,
            },
        )
        .expect("pressure-aware bootstrap should succeed");
        let late = &steps[14];
        let debug = render_debug_report("run-1", &steps);

        assert_eq!(
            late.frontier_pressure.pressure_action,
            PressureAction::SpillCold
        );
        assert!(debug.contains("frontier pressure: state=orange action=spill_cold"));
        assert!(debug.contains("retention policy: focus=temporal"));
    }
}
