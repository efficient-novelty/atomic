//! PA-1b: mutation-guarded open-competition audit.
//!
//! This module never writes or refreshes an input certificate.  Its live probes
//! run in memory against the current source; historical dirty-tree evidence is
//! kept in a separate register and is never represented as an exact replay.

use crate::config::RuntimeConfig;
use crate::diversify::FrontierRuntimeLimits;
use crate::engine::search_bootstrap_prefix_for_config_with_runtime;
use crate::halting_probe::run_adversarial_probe;
use crate::t_bf1_prefix::{Tbf1PrefixCertificate, issue_t_bf1_prefix_certificate};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const PA1B_SCHEMA: &str = "pa1-open-competition-v1";
pub const PA1B_DATE: &str = "2026-07-27";
pub const PA1B_CERTIFICATE_NAME: &str = "pa1_open_competition_v1.json";
pub const PA1B_REPORT_NAME: &str = "PA1_OPEN_COMPETITION_RESULT.md";
pub const PA1B_GLOBAL_FINDING: &str =
    "No certified run in the record contains a genuinely open competition with a lawful winner.";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/pa1_provenance_audit_plan.md");
const EXECUTION_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/PA1B_EXECUTION_PLAN.md");
const DISCLOSURE_BYTES: &[u8] = include_bytes!("../../../docs/provenance_disclosure_v1.md");

const CLAIM_RUN_BYTES: &[u8] = include_bytes!(
    "../../../runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/run.json"
);
const CLAIM_CONFIG_BYTES: &[u8] = include_bytes!(
    "../../../runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/config.toml"
);
const CLAIM_COMPARE_BYTES: &[u8] = include_bytes!(
    "../../../runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/claim-compare.json"
);
const CLAIM_STEP1_BYTES: &[u8] = include_bytes!(
    "../../../runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/reports/steps/step-01-summary.json"
);
const CLAIM_STEP15_BYTES: &[u8] = include_bytes!(
    "../../../runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/reports/steps/step-15-summary.json"
);

const V2_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v2.json");
const V2_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const V3_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v3.json");
const V3_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v3.json");
const TBF1_BYTES: &[u8] = include_bytes!("../../../docs/t_bf1_prefix_v1.json");

const BI1_SWEEP_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1_OPTION_A_SWEEP_V3_CERTIFICATE.json");
const BI1_ENACTED_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json");
const BI1_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json");
const BI1_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json");
const BI1_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json");
const BI1B_SWEEP_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_PREFIX_GENERAL_SWEEP_V1_CERTIFICATE.json");
const BI1B_REGRESSION_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_REGRESSION_CERTIFICATE.json");
const BI1B_ENACTED_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json");
const BI1B_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json");
const BI1B_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json");
const BI1B_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json");
const BI1B_CORRESPONDENCE_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json");

const HALTING_SURFACE_BYTES: &[u8] =
    include_bytes!("../../../docs/halting_surface_diagnostics.json");
const STEP16_EXHAUSTION_BYTES: &[u8] =
    include_bytes!("../../../docs/step16_semantic_exhaustion.json");
const CERTIFIED_HALT_BYTES: &[u8] =
    include_bytes!("../../../docs/certified_halt_verification.json");
const SH1_BYTES: &[u8] = include_bytes!("../../../docs/sh1_experiment_burned.json");

const ENGINE_SOURCE_BYTES: &[u8] = include_bytes!("engine.rs");
const ENUMERATE_SOURCE_BYTES: &[u8] = include_bytes!("enumerate.rs");
const ACCEPT_SOURCE_BYTES: &[u8] = include_bytes!("accept.rs");
const TBF1_SOURCE_BYTES: &[u8] = include_bytes!("t_bf1_prefix.rs");
const BI1_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance.rs");
const BI1B_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_resume_v1.rs");
const HALTING_SOURCE_BYTES: &[u8] = include_bytes!("halting_probe.rs");
const AUTOMATON_SOURCE_BYTES: &[u8] = include_bytes!("step16_automaton.rs");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const TELESCOPE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("pa1_open_competition_v1.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Pa1bEvidenceMode {
    HistoricalArtifact,
    HistoricalArtifactAndHomologousProbe,
    DirectReenumeration,
    FrozenTestimonyAndCodePath,
    CountOnly,
    AuthoredWitnessReplay,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Pa1bVerdict {
    PinnedBeforeCompetition,
    IncompleteTargetFencedShadow,
    OpenConeNoLawfulWinner,
    PositionPinnedSingletons,
    OpenSurfaceNotExecuted,
    AuthoredWitnessesNoFullSelection,
    ExplicitlyDesignatedWinner,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bStageCensus {
    pub stage: u32,
    pub generated_or_enumerated: Option<String>,
    pub checker_valid_or_admitted: Option<String>,
    pub selector_eligible: Option<String>,
    pub fully_evaluated: Option<String>,
    pub winner_hash: Option<String>,
    pub disposition: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bSurfaceAudit {
    pub audit_key: String,
    pub record_path: String,
    pub historical_run_id: Option<String>,
    pub evidence_mode: Pa1bEvidenceMode,
    pub count_semantics: String,
    pub stages: Vec<Pa1bStageCensus>,
    pub candidate_origin: String,
    pub raw_competitors_real: bool,
    pub admission_target_independent: bool,
    pub complete_selection_input: bool,
    pub winner_selected: bool,
    pub winner_autonomous: bool,
    pub pinning_or_incompleteness: String,
    pub verdict: Pa1bVerdict,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bReplayStatus {
    pub attempted: bool,
    pub valid: bool,
    pub exact_historical_replay: bool,
    pub summary: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bClaimProbe {
    pub step: u32,
    pub generated_raw_prefixes: String,
    pub well_formed_terminals: String,
    pub hard_admissible: String,
    pub exact_pruned: String,
    pub heuristic_dropped: String,
    pub full_telescopes_evaluated: String,
    pub winner_hash: String,
    pub matches_stored_counts: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bAdversarialCandidate {
    pub name: String,
    pub clause_kappa: u32,
    pub raw_surface_member: bool,
    pub admitted: bool,
    pub type_checks: bool,
    pub connectivity_passes: bool,
    pub semantically_minimal: bool,
    pub clears_bar: bool,
    pub survives_all_gates_and_clears: bool,
    pub nu_total: u32,
    pub rho: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bLiveProbes {
    pub claim_step1: Pa1bClaimProbe,
    pub claim_step15: Pa1bReplayStatus,
    pub claim_step15_observed: Option<Pa1bClaimProbe>,
    pub phase5b_v2_replay: Pa1bReplayStatus,
    pub phase5b_v3_replay: Pa1bReplayStatus,
    pub t_bf1_reissue: Pa1bReplayStatus,
    pub t_bf1_stage4_minimizers: String,
    pub step16_semantic_exhaustion_artifact_complete: bool,
    pub step16_adversarial_candidates: Vec<Pa1bAdversarialCandidate>,
    pub step16_raw_members: String,
    pub step16_lawful_clearers: String,
    pub certified_halt_replay: Pa1bReplayStatus,
    pub probes_in_memory_only: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bChronology {
    pub reference_table: String,
    pub early_admissibility_gates: String,
    pub step1_raw_catalog: String,
    pub t_bf1_and_bi_artifacts: String,
    pub consequence: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bFalsifierCompliance {
    pub no_pa1c_grades_assigned: bool,
    pub no_narrative_ground_used: bool,
    pub sealed_inputs_written: bool,
    pub every_requested_surface_explicit: bool,
    pub disclosure_weakened_or_superseded: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1OpenCompetitionV1Certificate {
    pub schema: String,
    pub date: String,
    pub question: String,
    pub positive_criterion: Vec<String>,
    pub source_bindings: Vec<Pa1bSourceBinding>,
    pub chronology: Pa1bChronology,
    pub exact_historical_claim_source_available: bool,
    pub current_reissues_are_historical_replays: bool,
    pub probes: Pa1bLiveProbes,
    pub surfaces: Vec<Pa1bSurfaceAudit>,
    pub qualifying_surface_count: usize,
    pub strongest_autonomy_evidence: Option<String>,
    pub strongest_gate_openness_evidence: Vec<String>,
    pub global_finding: String,
    pub falsifier_compliance: Pa1bFalsifierCompliance,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pa1bReplay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub qualifying_surface_count: usize,
    pub global_finding: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Pa1bError {
    #[error("JSON error: {0}")]
    Json(String),
    #[error("probe error: {0}")]
    Probe(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("I/O error: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn binding(path: &str, role: &str, bytes: &[u8]) -> Pa1bSourceBinding {
    Pa1bSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: u64::try_from(bytes.len()).expect("source byte length fits u64"),
        blake3: bytes_hash(bytes),
    }
}

fn source_bindings() -> Vec<Pa1bSourceBinding> {
    [
        (
            "docs/pa1_provenance_audit_plan.md",
            "governing_pa1_brief",
            PLAN_BYTES,
        ),
        (
            "docs/PA1B_EXECUTION_PLAN.md",
            "detailed_execution_plan",
            EXECUTION_PLAN_BYTES,
        ),
        (
            "docs/provenance_disclosure_v1.md",
            "standing_disclosure_not_superseded",
            DISCLOSURE_BYTES,
        ),
        (
            "runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/run.json",
            "claim_historical_run_metadata",
            CLAIM_RUN_BYTES,
        ),
        (
            "runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/config.toml",
            "claim_historical_config",
            CLAIM_CONFIG_BYTES,
        ),
        (
            "runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/claim-compare.json",
            "claim_comparison_certificate",
            CLAIM_COMPARE_BYTES,
        ),
        (
            "runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/reports/steps/step-01-summary.json",
            "claim_step1_census",
            CLAIM_STEP1_BYTES,
        ),
        (
            "runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/reports/steps/step-15-summary.json",
            "claim_step15_census",
            CLAIM_STEP15_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v2.json",
            "v2_preregistered_program",
            V2_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v2.json",
            "v2_burned_census",
            V2_BURN_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v3.json",
            "v3_preregistered_program",
            V3_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v3.json",
            "v3_burned_census",
            V3_BURN_BYTES,
        ),
        (
            "docs/t_bf1_prefix_v1.json",
            "t_bf1_direct_stage4_census",
            TBF1_BYTES,
        ),
        (
            "docs/BI1_OPTION_A_SWEEP_V3_CERTIFICATE.json",
            "bi1_four_branch_manifest",
            BI1_SWEEP_BYTES,
        ),
        (
            "docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json",
            "bi1_enacted_branch",
            BI1_ENACTED_BYTES,
        ),
        (
            "docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json",
            "bi1_alternate_branch",
            BI1_43_BYTES,
        ),
        (
            "docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json",
            "bi1_alternate_branch",
            BI1_4B_BYTES,
        ),
        (
            "docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json",
            "bi1_alternate_branch",
            BI1_B4_BYTES,
        ),
        (
            "docs/BI1B_PREFIX_GENERAL_SWEEP_V1_CERTIFICATE.json",
            "bi1b_four_branch_manifest",
            BI1B_SWEEP_BYTES,
        ),
        (
            "docs/BI1B_REGRESSION_CERTIFICATE.json",
            "bi1b_current_reissue_drift_disclosure",
            BI1B_REGRESSION_BYTES,
        ),
        (
            "docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json",
            "bi1b_enacted_revalidation",
            BI1B_ENACTED_BYTES,
        ),
        (
            "docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json",
            "bi1b_alternate_resume",
            BI1B_43_BYTES,
        ),
        (
            "docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json",
            "bi1b_alternate_resume",
            BI1B_4B_BYTES,
        ),
        (
            "docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json",
            "bi1b_alternate_resume",
            BI1B_B4_BYTES,
        ),
        (
            "docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json",
            "bi1b_24_row_correspondence",
            BI1B_CORRESPONDENCE_BYTES,
        ),
        (
            "docs/halting_surface_diagnostics.json",
            "historical_step16_count_only_diagnostic",
            HALTING_SURFACE_BYTES,
        ),
        (
            "docs/step16_semantic_exhaustion.json",
            "corrected_step16_exact_product_partition",
            STEP16_EXHAUSTION_BYTES,
        ),
        (
            "docs/certified_halt_verification.json",
            "step16_four_witness_certificate",
            CERTIFIED_HALT_BYTES,
        ),
        (
            "docs/sh1_experiment_burned.json",
            "step16_named_seal_and_continue",
            SH1_BYTES,
        ),
        (
            "crates/pen-search/src/engine.rs",
            "claim_search_and_pruning_paths",
            ENGINE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "enumeration_and_position_gates",
            ENUMERATE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/accept.rs",
            "legacy_exact_acceptance_rank",
            ACCEPT_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "admissibility_paths",
            ADMISSIBILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-core/src/telescope.rs",
            "authored_reference_table",
            TELESCOPE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf1_prefix.rs",
            "bar_free_stage4_direct_reenumeration",
            TBF1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance.rs",
            "bi1_guarded_branch_cones",
            BI1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance_resume_v1.rs",
            "bi1b_resume_cones",
            BI1B_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/halting_probe.rs",
            "step16_adversarial_and_named_selection_paths",
            HALTING_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/step16_automaton.rs",
            "step16_authored_four_witness_basis",
            AUTOMATON_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/pa1_open_competition_v1.rs",
            "pa1b_audit_issuer_and_replay",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| binding(path, role, bytes))
    .collect()
}

fn parse_value(label: &str, bytes: &[u8]) -> Result<Value, Pa1bError> {
    serde_json::from_slice(bytes).map_err(|error| Pa1bError::Json(format!("{label}: {error}")))
}

fn field<'a>(value: &'a Value, name: &str, label: &str) -> Result<&'a Value, Pa1bError> {
    value
        .get(name)
        .ok_or_else(|| Pa1bError::Invariant(format!("{label} lacks `{name}`")))
}

fn array<'a>(value: &'a Value, name: &str, label: &str) -> Result<&'a [Value], Pa1bError> {
    field(value, name, label)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| Pa1bError::Invariant(format!("{label}.{name} is not an array")))
}

fn unsigned(value: &Value, name: &str, label: &str) -> Result<u64, Pa1bError> {
    field(value, name, label)?
        .as_u64()
        .ok_or_else(|| Pa1bError::Invariant(format!("{label}.{name} is not unsigned")))
}

fn text_field(value: &Value, name: &str, label: &str) -> Result<String, Pa1bError> {
    field(value, name, label)?
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| Pa1bError::Invariant(format!("{label}.{name} is not text")))
}

fn option_count(value: Option<u64>) -> Option<String> {
    value.map(|count| count.to_string())
}

fn winner_hash(value: &Value) -> Option<String> {
    value
        .get("winner")
        .and_then(Value::as_object)
        .and_then(|winner| winner.get("candidate_hash"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn claim_probe_from_step(step: &crate::engine::AtomicSearchStep) -> Pa1bClaimProbe {
    Pa1bClaimProbe {
        step: step.step_index,
        generated_raw_prefixes: step.demo_funnel.generated_raw_prefixes.to_string(),
        well_formed_terminals: step.demo_funnel.well_formed_terminals.to_string(),
        hard_admissible: step.demo_funnel.hard_admissible.to_string(),
        exact_pruned: step.demo_funnel.exact_bound_pruned.to_string(),
        heuristic_dropped: step.demo_funnel.heuristic_dropped.to_string(),
        full_telescopes_evaluated: step.demo_funnel.full_telescopes_evaluated.to_string(),
        winner_hash: step.accepted.candidate_hash.clone(),
        matches_stored_counts: false,
    }
}

fn stored_claim_counts(
    bytes: &[u8],
) -> Result<(String, String, String, String, String, String, String), Pa1bError> {
    let value = parse_value("claim step", bytes)?;
    let stats = field(&value, "search_stats", "claim step")?;
    let funnel = field(stats, "demo_funnel", "claim step search_stats")?;
    let accepted = field(&value, "accepted", "claim step")?;
    Ok((
        unsigned(funnel, "generated_raw_prefixes", "claim funnel")?.to_string(),
        unsigned(funnel, "well_formed_terminals", "claim funnel")?.to_string(),
        unsigned(funnel, "hard_admissible", "claim funnel")?.to_string(),
        unsigned(funnel, "exact_bound_pruned", "claim funnel")?.to_string(),
        unsigned(funnel, "heuristic_dropped", "claim funnel")?.to_string(),
        unsigned(funnel, "full_telescopes_evaluated", "claim funnel")?.to_string(),
        text_field(accepted, "candidate_hash", "claim accepted")?,
    ))
}

fn probe_matches_stored(probe: &Pa1bClaimProbe, stored: &[u8]) -> Result<bool, Pa1bError> {
    let expected = stored_claim_counts(stored)?;
    Ok((
        &probe.generated_raw_prefixes,
        &probe.well_formed_terminals,
        &probe.hard_admissible,
        &probe.exact_pruned,
        &probe.heuristic_dropped,
        &probe.full_telescopes_evaluated,
        &probe.winner_hash,
    ) == (
        &expected.0,
        &expected.1,
        &expected.2,
        &expected.3,
        &expected.4,
        &expected.5,
        &expected.6,
    ))
}

fn replay_status(
    valid: bool,
    exact_historical_replay: bool,
    summary: impl Into<String>,
    errors: Vec<String>,
) -> Pa1bReplayStatus {
    Pa1bReplayStatus {
        attempted: true,
        valid,
        exact_historical_replay,
        summary: summary.into(),
        errors,
    }
}

fn run_live_probes_uncached() -> Result<Pa1bLiveProbes, Pa1bError> {
    let config_source = std::str::from_utf8(CLAIM_CONFIG_BYTES)
        .map_err(|error| Pa1bError::Probe(format!("claim config UTF-8: {error}")))?;
    let config = RuntimeConfig::from_toml_str(config_source)
        .map_err(|error| Pa1bError::Probe(format!("claim config parse: {error}")))?;

    let mut step1 = search_bootstrap_prefix_for_config_with_runtime(
        1,
        2,
        &config,
        FrontierRuntimeLimits::unlimited(),
    )
    .map_err(|error| Pa1bError::Probe(format!("claim Step 1 homologous rerun: {error:#}")))?
    .into_iter()
    .last()
    .map(|step| claim_probe_from_step(&step))
    .ok_or_else(|| Pa1bError::Probe("claim Step 1 produced no row".to_owned()))?;
    step1.matches_stored_counts = probe_matches_stored(&step1, CLAIM_STEP1_BYTES)?;

    let claim_step15 = replay_status(
        false,
        false,
        "two bounded current-source in-memory homologous Step-15 attempts exceeded their 2-minute and 3-minute audit bounds and were terminated without an artifact",
        vec![
            "the exact historical dirty source is unavailable; PA-1b therefore uses the sealed counters and code path rather than converting a resource-bound probe into evidence"
                .to_owned(),
        ],
    );
    let claim_step15_observed = None;

    let v2_status = replay_status(
        false,
        false,
        "bounded public replay executed during PA-1b failed closed before burn reissuance because the current upstream global E-4 v10 certificate differs from definition replay",
        vec![
            "current-definition prerequisite drift; immutable v2 program/burn retained as historical evidence"
                .to_owned(),
        ],
    );

    let v3_status = replay_status(
        false,
        false,
        "bounded public replay executed during PA-1b failed closed transitively on the current v2/upstream prerequisite drift",
        vec![
            "current-definition prerequisite drift; immutable v3 program/burn retained as historical evidence"
                .to_owned(),
        ],
    );

    let frozen_tbf1: Tbf1PrefixCertificate = serde_json::from_slice(TBF1_BYTES)
        .map_err(|error| Pa1bError::Json(format!("T-BF1 certificate: {error}")))?;
    let (tbf1_status, tbf1_stage4_minimizers) = match issue_t_bf1_prefix_certificate() {
        Ok(current) => {
            let stage4 = current
                .stages
                .iter()
                .find(|stage| stage.stage == 4)
                .ok_or_else(|| Pa1bError::Invariant("current T-BF1 lacks Stage 4".to_owned()))?;
            (
                replay_status(
                    current == frozen_tbf1,
                    false,
                    "current T-BF1 reissue preserves the live four-way stop; exact certificate equality is recorded separately",
                    (current != frozen_tbf1)
                        .then(|| {
                            "certificate differs from independent current reissuance".to_owned()
                        })
                        .into_iter()
                        .collect(),
                ),
                stage4.minimizer_hashes.len().to_string(),
            )
        }
        Err(error) => (
            replay_status(
                false,
                false,
                "current T-BF1 reissue failed closed",
                vec![error.to_string()],
            ),
            "4".to_owned(),
        ),
    };

    let frozen_exhaustion: Value =
        parse_value("Step-16 semantic exhaustion", STEP16_EXHAUSTION_BYTES)?;
    let step16_semantic_exhaustion_artifact_complete = field(
        &frozen_exhaustion,
        "every_stratum_classified",
        "Step-16 semantic exhaustion",
    )?
    .as_bool()
        == Some(true)
        && array(&frozen_exhaustion, "strata", "Step-16 semantic exhaustion")?
            .iter()
            .all(|stratum| {
                stratum.get("sums_match").and_then(Value::as_bool) == Some(true)
                    && stratum.get("unclassified_is_zero").and_then(Value::as_bool) == Some(true)
            });

    let adversarial = run_adversarial_probe();
    let step16_raw_members = adversarial
        .candidates
        .iter()
        .filter(|candidate| candidate.raw_surface_member)
        .count();
    let adversarial_candidates = adversarial
        .candidates
        .into_iter()
        .map(|candidate| Pa1bAdversarialCandidate {
            name: candidate.name,
            clause_kappa: candidate.clause_kappa,
            raw_surface_member: candidate.raw_surface_member,
            admitted: candidate.admitted,
            type_checks: candidate.type_checks,
            connectivity_passes: candidate.connectivity_passes,
            semantically_minimal: candidate.semantically_minimal,
            clears_bar: candidate.clears_bar,
            survives_all_gates_and_clears: candidate.survives_all_gates_and_clears,
            nu_total: candidate.nu_total,
            rho: candidate.rho,
        })
        .collect::<Vec<_>>();

    let halt_replay = crate::certified_halt::replay_certified_halt_certificate_json(
        std::str::from_utf8(CERTIFIED_HALT_BYTES)
            .map_err(|error| Pa1bError::Json(format!("certified halt UTF-8: {error}")))?,
    );
    let halt_status = replay_status(
        halt_replay.valid,
        true,
        "combined Step-16 certificate replay",
        halt_replay.errors,
    );

    Ok(Pa1bLiveProbes {
        claim_step1: step1,
        claim_step15,
        claim_step15_observed,
        phase5b_v2_replay: v2_status,
        phase5b_v3_replay: v3_status,
        t_bf1_reissue: tbf1_status,
        t_bf1_stage4_minimizers: tbf1_stage4_minimizers,
        step16_semantic_exhaustion_artifact_complete,
        step16_adversarial_candidates: adversarial_candidates,
        step16_raw_members: step16_raw_members.to_string(),
        step16_lawful_clearers: adversarial.clearing_survivors.to_string(),
        certified_halt_replay: halt_status,
        probes_in_memory_only: true,
    })
}

static LIVE_PROBES: OnceLock<Result<Pa1bLiveProbes, Pa1bError>> = OnceLock::new();

fn run_live_probes() -> Result<Pa1bLiveProbes, Pa1bError> {
    LIVE_PROBES.get_or_init(run_live_probes_uncached).clone()
}

fn stage_census_v2(stage: &Value) -> Result<Pa1bStageCensus, Pa1bError> {
    let stage_index = unsigned(stage, "stage", "v2 stage")?;
    let scored = array(stage, "scored", "v2 stage")?;
    Ok(Pa1bStageCensus {
        stage: u32::try_from(stage_index).expect("stage fits u32"),
        generated_or_enumerated: option_count(Some(unsigned(
            stage,
            "cone_enumerated",
            "v2 stage",
        )?)),
        checker_valid_or_admitted: option_count(Some(unsigned(
            stage,
            "cone_admitted",
            "v2 stage",
        )?)),
        selector_eligible: option_count(Some(unsigned(stage, "clearing_count", "v2 stage")?)),
        fully_evaluated: option_count(Some(
            u64::try_from(scored.len()).expect("scored count fits u64"),
        )),
        winner_hash: winner_hash(stage),
        disposition: if winner_hash(stage).is_some() {
            "legacy_exact_acceptance_rank_selected".to_owned()
        } else {
            "no_bar_clearer".to_owned()
        },
    })
}

fn stage_census_v3(stage: &Value) -> Result<Pa1bStageCensus, Pa1bError> {
    let stage_index = unsigned(stage, "stage", "v3 stage")?;
    let assessments = array(stage, "assessments", "v3 stage")?;
    Ok(Pa1bStageCensus {
        stage: u32::try_from(stage_index).expect("stage fits u32"),
        generated_or_enumerated: option_count(Some(unsigned(
            stage,
            "cone_enumerated",
            "v3 stage",
        )?)),
        checker_valid_or_admitted: option_count(Some(unsigned(
            stage,
            "cone_admitted",
            "v3 stage",
        )?)),
        selector_eligible: option_count(Some(unsigned(stage, "discharger_count", "v3 stage")?)),
        fully_evaluated: option_count(Some(
            u64::try_from(assessments.len()).expect("assessment count fits u64"),
        )),
        winner_hash: winner_hash(stage),
        disposition: "unique_guarded_total_discharger".to_owned(),
    })
}

fn stage_census_tbf1(stage: &Value) -> Result<Pa1bStageCensus, Pa1bError> {
    let candidates = array(stage, "candidates_seen_by_parsimony", "T-BF1 stage")?;
    let minimizers = array(stage, "minimizer_hashes", "T-BF1 stage")?;
    Ok(Pa1bStageCensus {
        stage: u32::try_from(unsigned(stage, "stage", "T-BF1 stage")?).expect("stage fits u32"),
        generated_or_enumerated: option_count(Some(unsigned(
            stage,
            "cone_enumerated",
            "T-BF1 stage",
        )?)),
        checker_valid_or_admitted: option_count(Some(unsigned(
            stage,
            "strict_cone_admitted",
            "T-BF1 stage",
        )?)),
        selector_eligible: option_count(Some(
            u64::try_from(minimizers.len()).expect("minimizer count fits u64"),
        )),
        fully_evaluated: option_count(Some(
            u64::try_from(candidates.len()).expect("candidate count fits u64"),
        )),
        winner_hash: field(stage, "selected_hash", "T-BF1 stage")?
            .as_str()
            .map(ToOwned::to_owned),
        disposition: "stopped_equal_minimum_pair".to_owned(),
    })
}

fn stage_census_bi1(stage: &Value) -> Result<Pa1bStageCensus, Pa1bError> {
    let dischargers = unsigned(stage, "discharger_count", "BI-1 stage")?;
    Ok(Pa1bStageCensus {
        stage: u32::try_from(unsigned(stage, "stage", "BI-1 stage")?).expect("stage fits u32"),
        generated_or_enumerated: option_count(Some(unsigned(
            stage,
            "cone_enumerated",
            "BI-1 stage",
        )?)),
        checker_valid_or_admitted: option_count(Some(unsigned(
            stage,
            "cone_admitted",
            "BI-1 stage",
        )?)),
        selector_eligible: option_count(Some(dischargers)),
        fully_evaluated: option_count(Some(unsigned(stage, "cone_deduped", "BI-1 stage")?)),
        winner_hash: winner_hash(stage),
        disposition: if dischargers == 1 {
            "unique_typed_total_discharger".to_owned()
        } else {
            "singleton_unknown_no_discharger".to_owned()
        },
    })
}

fn stage_census_bi1b(stage: &Value) -> Result<Pa1bStageCensus, Pa1bError> {
    let census = field(stage, "census", "BI-1b stage")?;
    Ok(Pa1bStageCensus {
        stage: u32::try_from(unsigned(stage, "stage", "BI-1b stage")?).expect("stage fits u32"),
        generated_or_enumerated: option_count(Some(unsigned(
            stage,
            "cone_enumerated",
            "BI-1b stage",
        )?)),
        checker_valid_or_admitted: option_count(Some(unsigned(
            stage,
            "cone_admitted",
            "BI-1b stage",
        )?)),
        selector_eligible: option_count(Some(unsigned(
            census,
            "discharger_count",
            "BI-1b census",
        )?)),
        fully_evaluated: option_count(Some(unsigned(stage, "cone_deduped", "BI-1b stage")?)),
        winner_hash: winner_hash(stage),
        disposition: "unique_typed_total_discharger_after_complete_classification".to_owned(),
    })
}

fn collect_stages<F>(
    value: &Value,
    path: &[&str],
    label: &str,
    map: F,
) -> Result<Vec<Pa1bStageCensus>, Pa1bError>
where
    F: Fn(&Value) -> Result<Pa1bStageCensus, Pa1bError>,
{
    let mut current = value;
    for segment in path {
        current = field(current, segment, label)?;
    }
    current
        .as_array()
        .ok_or_else(|| Pa1bError::Invariant(format!("{label} stage path is not an array")))?
        .iter()
        .map(map)
        .collect()
}

fn surface(
    audit_key: &str,
    record_path: &str,
    historical_run_id: Option<&str>,
    evidence_mode: Pa1bEvidenceMode,
    count_semantics: &str,
    stages: Vec<Pa1bStageCensus>,
    candidate_origin: &str,
    raw_competitors_real: bool,
    admission_target_independent: bool,
    complete_selection_input: bool,
    winner_selected: bool,
    winner_autonomous: bool,
    pinning_or_incompleteness: &str,
    verdict: Pa1bVerdict,
    evidence: &[&str],
) -> Pa1bSurfaceAudit {
    Pa1bSurfaceAudit {
        audit_key: audit_key.to_owned(),
        record_path: record_path.to_owned(),
        historical_run_id: historical_run_id.map(ToOwned::to_owned),
        evidence_mode,
        count_semantics: count_semantics.to_owned(),
        stages,
        candidate_origin: candidate_origin.to_owned(),
        raw_competitors_real,
        admission_target_independent,
        complete_selection_input,
        winner_selected,
        winner_autonomous,
        pinning_or_incompleteness: pinning_or_incompleteness.to_owned(),
        verdict,
        evidence: evidence.iter().map(|entry| (*entry).to_owned()).collect(),
    }
}

fn claim_stage_census(bytes: &[u8], stage: u32) -> Result<Pa1bStageCensus, Pa1bError> {
    let value = parse_value("claim stage summary", bytes)?;
    let stats = field(&value, "search_stats", "claim stage")?;
    let funnel = field(stats, "demo_funnel", "claim stage stats")?;
    let accepted = field(&value, "accepted", "claim stage")?;
    Ok(Pa1bStageCensus {
        stage,
        generated_or_enumerated: option_count(Some(unsigned(
            funnel,
            "generated_raw_prefixes",
            "claim funnel",
        )?)),
        checker_valid_or_admitted: option_count(Some(unsigned(
            funnel,
            "hard_admissible",
            "claim funnel",
        )?)),
        selector_eligible: option_count(Some(unsigned(funnel, "bar_clearers", "claim funnel")?)),
        fully_evaluated: option_count(Some(unsigned(
            funnel,
            "full_telescopes_evaluated",
            "claim funnel",
        )?)),
        winner_hash: Some(text_field(accepted, "candidate_hash", "claim accepted")?),
        disposition: "minimal_positive_overshoot".to_owned(),
    })
}

fn derive_surfaces(probes: &Pa1bLiveProbes) -> Result<Vec<Pa1bSurfaceAudit>, Pa1bError> {
    let run = parse_value("claim run", CLAIM_RUN_BYTES)?;
    let run_id = text_field(&run, "run_id", "claim run")?;
    let build = field(&run, "build", "claim run")?;
    if field(build, "dirty_tree", "claim build")?.as_bool() != Some(true) {
        return Err(Pa1bError::Invariant(
            "claim historical run no longer records dirty_tree=true".to_owned(),
        ));
    }

    let claim1_value = parse_value("claim Step 1", CLAIM_STEP1_BYTES)?;
    let claim1_stats = field(&claim1_value, "search_stats", "claim Step 1")?;
    if unsigned(
        field(claim1_stats, "demo_funnel", "claim Step 1 stats")?,
        "well_formed_terminals",
        "claim Step 1 funnel",
    )? != 1285
    {
        return Err(Pa1bError::Invariant(
            "claim Step 1 well-formed count drifted".to_owned(),
        ));
    }

    let claim15_value = parse_value("claim Step 15", CLAIM_STEP15_BYTES)?;
    let claim15_stats = field(&claim15_value, "search_stats", "claim Step 15")?;
    let claim15_funnel = field(claim15_stats, "demo_funnel", "claim Step 15 stats")?;
    if unsigned(claim15_funnel, "heuristic_dropped", "claim Step 15 funnel")? != 257 {
        return Err(Pa1bError::Invariant(
            "claim Step 15 prefix heuristic-drop count drifted".to_owned(),
        ));
    }

    let v2 = parse_value("v2 burn", V2_BURN_BYTES)?;
    let v2_all = collect_stages(&v2, &["stages"], "v2", stage_census_v2)?;
    if v2_all.len() != 8 {
        return Err(Pa1bError::Invariant(
            "v2 must contain eight reached stages".to_owned(),
        ));
    }
    let v2_early = v2_all[0..3].to_vec();
    let v2_stage4 = vec![v2_all[3].clone()];
    let v2_late = v2_all[4..8].to_vec();

    let v3 = parse_value("v3 burn", V3_BURN_BYTES)?;
    let v3_stages = collect_stages(&v3, &["stages"], "v3", stage_census_v3)?;
    if v3_stages.len() != 8
        || v3_stages
            .iter()
            .any(|stage| stage.generated_or_enumerated.as_deref() != Some("1"))
    {
        return Err(Pa1bError::Invariant(
            "v3 recomputed stages are not the expected eight singleton cones".to_owned(),
        ));
    }

    let tbf1 = parse_value("T-BF1", TBF1_BYTES)?;
    let tbf1_stage4 = array(&tbf1, "stages", "T-BF1")?
        .iter()
        .find(|stage| stage.get("stage").and_then(Value::as_u64) == Some(4))
        .ok_or_else(|| Pa1bError::Invariant("T-BF1 lacks Stage 4".to_owned()))?;
    let tbf1_census = vec![stage_census_tbf1(tbf1_stage4)?];

    let bi1_enacted = parse_value("BI-1 enacted", BI1_ENACTED_BYTES)?;
    let bi1_43 = parse_value("BI-1 43", BI1_43_BYTES)?;
    let bi1_4b = parse_value("BI-1 4b", BI1_4B_BYTES)?;
    let bi1_b4 = parse_value("BI-1 b4", BI1_B4_BYTES)?;
    let bi1_enacted_stages = collect_stages(
        &bi1_enacted,
        &["continuation", "stages"],
        "BI-1 enacted",
        stage_census_bi1,
    )?;
    let bi1_43_stages = collect_stages(
        &bi1_43,
        &["continuation", "stages"],
        "BI-1 43",
        stage_census_bi1,
    )?;
    let bi1_4b_stages = collect_stages(
        &bi1_4b,
        &["continuation", "stages"],
        "BI-1 4b",
        stage_census_bi1,
    )?;
    let bi1_b4_stages = collect_stages(
        &bi1_b4,
        &["continuation", "stages"],
        "BI-1 b4",
        stage_census_bi1,
    )?;

    let bi1b_43 = parse_value("BI-1b 43", BI1B_43_BYTES)?;
    let bi1b_4b = parse_value("BI-1b 4b", BI1B_4B_BYTES)?;
    let bi1b_b4 = parse_value("BI-1b b4", BI1B_B4_BYTES)?;
    let bi1b_43_stages = collect_stages(
        &bi1b_43,
        &["resume", "stages"],
        "BI-1b 43",
        stage_census_bi1b,
    )?;
    let bi1b_4b_stages = collect_stages(
        &bi1b_4b,
        &["resume", "stages"],
        "BI-1b 4b",
        stage_census_bi1b,
    )?;
    let bi1b_b4_stages = collect_stages(
        &bi1b_b4,
        &["resume", "stages"],
        "BI-1b b4",
        stage_census_bi1b,
    )?;

    let exhaustion = parse_value("Step-16 exhaustion", STEP16_EXHAUSTION_BYTES)?;
    let step16_products = array(&exhaustion, "strata", "Step-16 exhaustion")?
        .iter()
        .map(|stratum| {
            let kappa = unsigned(stratum, "kappa", "Step-16 stratum")?;
            let raw = text_field(stratum, "raw_total", "Step-16 stratum")?;
            let internal = text_field(stratum, "internal", "Step-16 stratum")?
                .parse::<u128>()
                .map_err(|error| Pa1bError::Invariant(format!("Step-16 internal: {error}")))?;
            let marginal = text_field(stratum, "egp_marginal", "Step-16 stratum")?
                .parse::<u128>()
                .map_err(|error| Pa1bError::Invariant(format!("Step-16 marginal: {error}")))?;
            Ok(Pa1bStageCensus {
                stage: 16,
                generated_or_enumerated: Some(raw),
                checker_valid_or_admitted: Some((internal + marginal).to_string()),
                selector_eligible: None,
                fully_evaluated: None,
                winner_hash: None,
                disposition: format!("kappa_{kappa}_counted_not_materialized"),
            })
        })
        .collect::<Result<Vec<_>, Pa1bError>>()?;

    let sh1 = parse_value("SH-1", SH1_BYTES)?;
    let first_sh1_step = array(field(&sh1, "t1", "SH-1")?, "steps", "SH-1 t1")?
        .first()
        .ok_or_else(|| Pa1bError::Invariant("SH-1 has no Step 16 row".to_owned()))?;
    let sh1_candidates = array(first_sh1_step, "candidates", "SH-1 Step 16")?;
    let sh1_selected = field(first_sh1_step, "selected", "SH-1 Step 16")?;
    let sh1_name = text_field(sh1_selected, "name", "SH-1 selected")?;

    let mut surfaces = vec![
        surface(
            "claim_lane_step_1",
            "runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/reports/steps/step-01-summary.json",
            Some(&run_id),
            Pa1bEvidenceMode::HistoricalArtifactAndHomologousProbe,
            "generated is the 18x120-16 raw telescope catalog; hard-admitted and selected counts are downstream",
            vec![claim_stage_census(CLAIM_STEP1_BYTES, 1)?],
            "real raw grammar cross-product added after the reference telescope",
            true,
            false,
            true,
            true,
            false,
            "1,285 checker-valid/connected terminals reach a Step-1-specific bootstrap predicate; 1,284 are rejected by the post-reference `bootstrap_universe_shape` gate, leaving the authored universe pair alone",
            Pa1bVerdict::PinnedBeforeCompetition,
            &[
                "stored counters: 2,144 raw; 384 malformed; 475 connectivity-pruned; 1,285 well formed; 1 admitted/evaluated/clearer",
                "the reference table first appears 2026-03-13; the bootstrap predicate follows 2026-03-14/15; the 2,144 catalog follows 2026-03-18",
                "the current in-memory homologous probe matches the stored Step-1 counts but cannot reconstruct the historical dirty source",
            ],
        ),
        surface(
            "claim_lane_step_15",
            "runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15/reports/steps/step-15-summary.json",
            Some(&run_id),
            Pa1bEvidenceMode::HistoricalArtifactAndHomologousProbe,
            "7,211 is a raw prefix-event counter, not 7,211 complete telescopes; the authored 3-choice-per-position cube has raw product 6,561",
            vec![claim_stage_census(CLAIM_STEP15_BYTES, 15)?],
            "claim-generic authored terminal cube with reference-aware path logic",
            true,
            false,
            false,
            true,
            false,
            "261 terminals are reported hard-admissible, but 257 prefix groups are heuristically dropped, 553 prefixes are exact-pruned, three terminal ranks are incumbent-pruned, and only one telescope is fully evaluated",
            Pa1bVerdict::IncompleteTargetFencedShadow,
            &[
                "the 5,000 value is a configured reporting floor, not an admitted-candidate count",
                "the historical run is dirty_tree=true and its source diff/binary is absent, so current source cannot be represented as an exact replay",
                "reference-root and claim-open-band special paths prevent this incomplete shadow from establishing an autonomous winner",
            ],
        ),
        surface(
            "phase5b_reselection_v2_stages_1_3",
            "docs/phase5b_reselection_burn_v2.json",
            None,
            Pa1bEvidenceMode::HistoricalArtifact,
            "complete Guarded cone enumeration / admission / legacy bar-clearer / scored candidate",
            v2_early,
            "Guarded early-family cones",
            true,
            false,
            true,
            true,
            false,
            "the reported 288, 33, and 56 raw cones each collapse to one admitted candidate under Step-specific gates",
            Pa1bVerdict::PinnedBeforeCompetition,
            &["counts are respectively 288/1, 33/1, and 56/1 before the legacy selector"],
        ),
        surface(
            "phase5b_reselection_v2_stage_4",
            "docs/phase5b_reselection_burn_v2.json",
            None,
            Pa1bEvidenceMode::HistoricalArtifact,
            "complete Guarded cone; all four admitted candidates clear the legacy bar and are ranked",
            v2_stage4,
            "real four-candidate former-eliminator cone",
            true,
            true,
            true,
            true,
            false,
            "all four candidates tie at (kappa, nu)=(3,5) under the adopted two-register/parsimony law; v2's reference winner comes from the older target-derived acceptance tuple, ending in canonical order",
            Pa1bVerdict::OpenConeNoLawfulWinner,
            &[
                "T-BF1 independently recomputes the same four hashes and lawfully stops with selected_hash=null",
                "v2's selected reference is not a lawful winner under the audited two-register law",
            ],
        ),
        surface(
            "phase5b_reselection_v2_stages_5_8",
            "docs/phase5b_reselection_burn_v2.json",
            None,
            Pa1bEvidenceMode::HistoricalArtifact,
            "complete Guarded cone / admission / legacy bar-clearer / scored candidate",
            v2_late,
            "Guarded family cones",
            true,
            false,
            true,
            true,
            false,
            "Stages 5-8 enumerate one admitted canonical candidate each; Stage 8 has no legacy bar clearer and no winner",
            Pa1bVerdict::PositionPinnedSingletons,
            &[
                "the position recognizers and late-family None surface copy the authored family clauses",
            ],
        ),
        surface(
            "phase5b_reselection_v3_stages_8_15",
            "docs/phase5b_reselection_burn_v3.json",
            None,
            Pa1bEvidenceMode::HistoricalArtifact,
            "complete Guarded cone / admission / typed total discharger / assessed candidate",
            v3_stages,
            "Guarded family cones",
            true,
            false,
            true,
            true,
            false,
            "every recomputed stage is 1 enumerated / 1 admitted / 1 deduped / 1 total discharger",
            Pa1bVerdict::PositionPinnedSingletons,
            &[
                "Guarded mode maps to LateFamilySurface::None",
                "Stages 13-15 directly return one reference expression per slot; Stages 8-12 use exact slot recognizers",
            ],
        ),
        surface(
            "t_bf1_stage_4",
            "docs/t_bf1_prefix_v1.json",
            None,
            Pa1bEvidenceMode::DirectReenumeration,
            "direct cone enumeration / strict admission / parsimony minimizer / evaluated candidate",
            tbf1_census,
            "real 2x2 cross-product of two clause-0 and two clause-1 alternatives",
            true,
            true,
            true,
            false,
            false,
            "all four typed, admitted candidates have the same lawful minimum pair (3,5); the selector cannot read hash, canonical key, bit cost, reference, or diagnostic bar",
            Pa1bVerdict::OpenConeNoLawfulWinner,
            &[
                "selected_hash is null and the terminal decision is stopped_f_bf1_equal_minimum_pair",
                "reissuance is certificate-gated to the frozen four-vector, so this proves multiplicity inside the certified cone, not openness to an unknown fifth shape",
            ],
        ),
        surface(
            "bi1_stage_4_option_a_roots",
            "docs/BI1_OPTION_A_SWEEP_V3_CERTIFICATE.json",
            None,
            Pa1bEvidenceMode::FrozenTestimonyAndCodePath,
            "four authorized Stage-4 branch roots; no root is selected",
            vec![Pa1bStageCensus {
                stage: 4,
                generated_or_enumerated: Some("4".to_owned()),
                checker_valid_or_admitted: Some("4".to_owned()),
                selector_eligible: Some("4".to_owned()),
                fully_evaluated: Some("4".to_owned()),
                winner_hash: None,
                disposition: "four_option_a_execution_branches".to_owned(),
            }],
            "the T-BF1 four-root cone",
            true,
            true,
            true,
            false,
            false,
            "Option A authorizes all four roots as branches; semantic nu is diagnostic and no enacted-root outcome is used as a selector",
            Pa1bVerdict::OpenConeNoLawfulWinner,
            &[
                "the enacted root is issued first operationally but is not selected as a Stage-4 winner",
            ],
        ),
    ];

    let bi1_specs = [
        (
            "bi1_enacted_2016726758f3_stages_5_15",
            "docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json",
            bi1_enacted_stages,
            "enacted",
        ),
        (
            "bi1_alternate_43a0ed707770_stages_5_8",
            "docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json",
            bi1_43_stages,
            "alternate",
        ),
        (
            "bi1_alternate_4b2211ecae25_stages_5_8",
            "docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json",
            bi1_4b_stages,
            "alternate",
        ),
        (
            "bi1_alternate_b4f821d9bb28_stages_5_8",
            "docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json",
            bi1_b4_stages,
            "alternate",
        ),
    ];
    for (key, path, stages, branch_kind) in bi1_specs {
        let has_winner = stages.iter().any(|stage| stage.winner_hash.is_some());
        surfaces.push(surface(
            key,
            path,
            None,
            Pa1bEvidenceMode::FrozenTestimonyAndCodePath,
            "Guarded cone / admission / typed discharger / canonical singleton",
            stages,
            "branch-local Guarded family cones",
            true,
            false,
            true,
            has_winner,
            false,
            if branch_kind == "enacted" {
                "Stages 5-15 are eleven successive 1/1/1 singleton cones"
            } else {
                "Stages 5-7 are singleton winners; the same singleton at Stage 8 has zero dischargers/Unknown and stops"
            },
            Pa1bVerdict::PositionPinnedSingletons,
            &[
                "Guarded exact position recognizers admit no same-stage competitor",
                "current BI-1 reissuance is blocked by upstream drift; frozen rows and code path are reported without claiming successful current replay",
            ],
        ));
    }

    surfaces.push(surface(
        "bi1b_enacted_2016726758f3_revalidation",
        "docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json",
        None,
        Pa1bEvidenceMode::FrozenTestimonyAndCodePath,
        "zero newly resumed stages; the prior enacted Stage-15 halt is authenticated",
        Vec::new(),
        "sealed BI-1 enacted branch replay",
        false,
        false,
        false,
        false,
        false,
        "BI-1b performs no new enacted-branch candidate competition",
        Pa1bVerdict::PositionPinnedSingletons,
        &["resumed_stage_count=0 and prior_enacted_halt_revalidated=true"],
    ));

    for (key, path, stages) in [
        (
            "bi1b_alternate_43a0ed707770_stages_8_15",
            "docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json",
            bi1b_43_stages,
        ),
        (
            "bi1b_alternate_4b2211ecae25_stages_8_15",
            "docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json",
            bi1b_4b_stages,
        ),
        (
            "bi1b_alternate_b4f821d9bb28_stages_8_15",
            "docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json",
            bi1b_b4_stages,
        ),
    ] {
        surfaces.push(surface(
            key,
            path,
            None,
            Pa1bEvidenceMode::FrozenTestimonyAndCodePath,
            "sealed Stage-8 singleton consumption, then Guarded enumeration for Stages 9-15",
            stages,
            "branch-local prefix-general classification over singleton cones",
            true,
            false,
            true,
            true,
            false,
            "each stage is 1 enumerated / 1 admitted / 1 deduped / 1 discharger; Stage 8 is consumed from the sealed cone, not re-enumerated",
            Pa1bVerdict::PositionPinnedSingletons,
            &[
                "all three never-enacted branches are listed separately",
                "the 24-row correspondence records identical singleton candidates and winners across three branches x eight stages",
            ],
        ));
    }

    surfaces.extend([
        surface(
            "step16_unclamped_full_lane",
            "docs/halting_surface_diagnostics.json",
            None,
            Pa1bEvidenceMode::CountOnly,
            "each row is one kappa stratum: exact per-position raw product and checker-valid product; no telescope pool was materialized",
            step16_products,
            "generic Guarded open-band grammar with no focus family or historical anchor",
            true,
            true,
            false,
            false,
            false,
            "both full materializing attempts exhausted memory; the safe replacement counts the surface and classifies expression products but does not execute candidate selection",
            Pa1bVerdict::OpenSurfaceNotExecuted,
            &[
                "the older 2026-07-05 widest-scope diagnostic is retained as historical testimony; the exact position-specific products come from step16_semantic_exhaustion.json",
                "an OOM/probe failure is not converted into a halt or winner verdict",
            ],
        ),
        surface(
            "step16_corrected_adversarial_probe",
            "docs/certified_halt_verification.json",
            None,
            Pa1bEvidenceMode::AuthoredWitnessReplay,
            "17 authored shapes; 9 raw-surface members; 4 pass the complete gate conjunction and clear",
            vec![Pa1bStageCensus {
                stage: 16,
                generated_or_enumerated: Some("17".to_owned()),
                checker_valid_or_admitted: Some(probes.step16_raw_members.clone()),
                selector_eligible: Some(probes.step16_lawful_clearers.clone()),
                fully_evaluated: Some("17".to_owned()),
                winner_hash: None,
                disposition: "membership_replay_no_selection".to_owned(),
            }],
            "hand-authored adversarial table replayed through the real gates",
            true,
            true,
            false,
            false,
            false,
            "the table is deliberately non-exhaustive and performs no winner selection",
            Pa1bVerdict::AuthoredWitnessesNoFullSelection,
            &[
                "four real raw-generable, admitted, typed, connected, minimal bar clearers are disclosed by name",
                "five further raw expressions are lawfully rejected; eight diagnostic shapes are outside the raw lane",
            ],
        ),
        surface(
            "step16_semantic_exhaustion_and_automaton",
            "docs/step16_semantic_exhaustion.json",
            None,
            Pa1bEvidenceMode::AuthoredWitnessReplay,
            "exhaustive expression-product partition plus an authored four-witness SAT basis",
            Vec::new(),
            "dynamic-programming quotient counts and handcrafted witness membership",
            true,
            true,
            false,
            false,
            false,
            "the semantic partition does not materialize telescopes or rank candidates; the automaton explicitly has no exhaustive telescope quotient or certified global maximum",
            Pa1bVerdict::AuthoredWitnessesNoFullSelection,
            &[
                "the four-witness basis proves that lawful clearers exist, not which expression wins the full surface",
                "the combined certified-halt replay remains a verification of its stated conditional boundary",
            ],
        ),
        surface(
            "sh1_step16_named_seal_and_continue",
            "docs/sh1_experiment_burned.json",
            None,
            Pa1bEvidenceMode::AuthoredWitnessReplay,
            "selection from the 17-shape registered adversarial table, not the full raw catalog",
            vec![Pa1bStageCensus {
                stage: 16,
                generated_or_enumerated: Some(
                    u64::try_from(sh1_candidates.len())
                        .expect("SH-1 candidate count fits u64")
                        .to_string(),
                ),
                checker_valid_or_admitted: None,
                selector_eligible: None,
                fully_evaluated: Some(
                    u64::try_from(sh1_candidates.len())
                        .expect("SH-1 candidate count fits u64")
                        .to_string(),
                ),
                winner_hash: Some(format!("name:{sh1_name}")),
                disposition: "designated_step16_survivor".to_owned(),
            }],
            "registered authored adversarial table",
            true,
            true,
            false,
            true,
            false,
            "offset zero looks up `designated_step16_survivor` by name; acceptance-rank selection begins only on later continuation rounds",
            Pa1bVerdict::ExplicitlyDesignatedWinner,
            &[
                "the designated name is hit_no_formation_d1",
                "no certificate proves that this witness beats the complete lawful Step-16 surface",
            ],
        ),
    ]);

    Ok(surfaces)
}

fn certificate_projection(
    certificate: &Pa1OpenCompetitionV1Certificate,
) -> Pa1OpenCompetitionV1Certificate {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    projection
}

fn certificate_digest(certificate: &Pa1OpenCompetitionV1Certificate) -> String {
    let projection = certificate_projection(certificate);
    let bytes = serde_json::to_vec(&(PA1B_SCHEMA, "certificate", projection))
        .expect("PA-1b certificate serializes");
    bytes_hash(&bytes)
}

pub fn issue_pa1_open_competition_v1() -> Result<Pa1OpenCompetitionV1Certificate, Pa1bError> {
    let probes = run_live_probes()?;
    let surfaces = derive_surfaces(&probes)?;
    let qualifying_surface_count = surfaces
        .iter()
        .filter(|row| {
            row.raw_competitors_real
                && row.admission_target_independent
                && row.complete_selection_input
                && row.winner_selected
                && row.winner_autonomous
        })
        .count();
    if qualifying_surface_count != 0 {
        return Err(Pa1bError::Invariant(
            "a qualifying surface requires an explicit strongest-autonomy disposition".to_owned(),
        ));
    }
    if surfaces.len() != 20 {
        return Err(Pa1bError::Invariant(format!(
            "expected 20 explicit audit rows, derived {}",
            surfaces.len()
        )));
    }

    let mut certificate = Pa1OpenCompetitionV1Certificate {
        schema: PA1B_SCHEMA.to_owned(),
        date: PA1B_DATE.to_owned(),
        question: "Does any certified run place real candidates before target-independent law, account for the complete outcome-relevant pool, and lawfully select a winner?".to_owned(),
        positive_criterion: vec![
            "candidate_origin_is_actual_enumeration".to_owned(),
            "admission_is_target_independent".to_owned(),
            "full_surface_or_soundly_complete_pool".to_owned(),
            "winner_is_unique_under_unpinned_law".to_owned(),
        ],
        source_bindings: source_bindings(),
        chronology: Pa1bChronology {
            reference_table: "Telescope::reference(1..15): commit 5dd8447, 2026-03-13; the target sequence was predeclared earlier".to_owned(),
            early_admissibility_gates: "bootstrap and family gates follow on 2026-03-14/15 and later March commits".to_owned(),
            step1_raw_catalog: "step_one_demo_raw_catalog: commit a8a1935, 2026-03-18".to_owned(),
            t_bf1_and_bi_artifacts: "T-BF1 first lands 2026-07-22; BI-1/BI-1b v3 land 2026-07-24".to_owned(),
            consequence: "authored-reference-first, recognizing/enumerating machinery second; replay equality cannot establish discovery".to_owned(),
        },
        exact_historical_claim_source_available: false,
        current_reissues_are_historical_replays: false,
        probes,
        surfaces,
        qualifying_surface_count,
        strongest_autonomy_evidence: None,
        strongest_gate_openness_evidence: vec![
            "T-BF1 Stage 4: four real admitted typed co-minimizers, lawful stop, no winner".to_owned(),
            "Step 16: generic open gate with four known lawful clearers, but no complete executed selection".to_owned(),
        ],
        global_finding: PA1B_GLOBAL_FINDING.to_owned(),
        falsifier_compliance: Pa1bFalsifierCompliance {
            no_pa1c_grades_assigned: true,
            no_narrative_ground_used: true,
            sealed_inputs_written: false,
            every_requested_surface_explicit: true,
            disclosure_weakened_or_superseded: false,
        },
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn verdict_label(verdict: Pa1bVerdict) -> &'static str {
    match verdict {
        Pa1bVerdict::PinnedBeforeCompetition => "pinned_before_competition",
        Pa1bVerdict::IncompleteTargetFencedShadow => "incomplete_target_fenced_shadow",
        Pa1bVerdict::OpenConeNoLawfulWinner => "open_cone_no_lawful_winner",
        Pa1bVerdict::PositionPinnedSingletons => "position_pinned_singletons",
        Pa1bVerdict::OpenSurfaceNotExecuted => "open_surface_not_executed",
        Pa1bVerdict::AuthoredWitnessesNoFullSelection => "authored_witnesses_no_full_selection",
        Pa1bVerdict::ExplicitlyDesignatedWinner => "explicitly_designated_winner",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn count_cell(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("—")
}

fn stage_vector(stages: &[Pa1bStageCensus]) -> String {
    if stages.is_empty() {
        return "no new census".to_owned();
    }
    stages
        .iter()
        .map(|stage| {
            format!(
                "S{}:{}/{}/{}/{}",
                stage.stage,
                count_cell(&stage.generated_or_enumerated),
                count_cell(&stage.checker_valid_or_admitted),
                count_cell(&stage.selector_eligible),
                count_cell(&stage.fully_evaluated),
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn replay_status_line(label: &str, status: &Pa1bReplayStatus) -> String {
    let errors = if status.errors.is_empty() {
        "none".to_owned()
    } else {
        status.errors.join("; ")
    };
    format!(
        "- **{label}:** valid `{}`; exact historical replay `{}`. {} Errors: `{}`.\n",
        status.valid, status.exact_historical_replay, status.summary, errors
    )
}

pub fn render_pa1_open_competition_v1(certificate: &Pa1OpenCompetitionV1Certificate) -> String {
    let mut out = String::new();
    out.push_str("# PA-1b open-competition result\n\n");
    out.push_str(&format!(
        "**Date:** {}. **Status:** executed. **Certificate:** `{}`. **Scope:** PA-1b only; no PA-1c grades are assigned.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("## Finding\n\n");
    out.push_str("> ");
    out.push_str(&certificate.global_finding);
    out.push_str("\n\n");
    out.push_str(
        "No run therefore becomes the program's strongest autonomy evidence. This does **not** mean every audited surface is syntactically closed. T-BF1 Stage 4 is a real four-way lawful cone that correctly stops without a winner, and Step 16 has a generic open gate with at least four real lawful bar-clearing expressions. Those are evidence of gate openness and multiplicity, not evidence that a complete open run lawfully found a winner.\n\n",
    );

    out.push_str("## Decision rule\n\n");
    out.push_str(
        "A positive row requires all four facts at once: actual enumeration, target-independent admission, a complete outcome-relevant selection pool, and a unique winner under an unpinned law. Raw breadth alone, a complete census of a singleton gate, an authored witness table, or a deterministic but target-derived tie break is non-positive.\n\n",
    );

    out.push_str("## Complete run matrix\n\n");
    out.push_str(
        "Counts are `generated-or-enumerated / checker-valid-or-admitted / selector-eligible / fully-evaluated`. Repeated `S16` entries are the separate κ strata named in their dispositions. Every requested run family and every BI branch is explicit.\n\n",
    );
    out.push_str("| audit key | stage counts | target-independent admission | complete selector pool | winner | autonomous winner | verdict |\n");
    out.push_str("|---|---|:---:|:---:|:---:|:---:|---|\n");
    for row in &certificate.surfaces {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | `{}` |\n",
            row.audit_key,
            stage_vector(&row.stages),
            yes_no(row.admission_target_independent),
            yes_no(row.complete_selection_input),
            yes_no(row.winner_selected),
            yes_no(row.winner_autonomous),
            verdict_label(row.verdict),
        ));
    }
    out.push('\n');

    out.push_str("## Claim lane\n\n");
    out.push_str(
        "### Step 1 — real raw expressions, pinned finalist\n\nThe stored run reports 2,144 raw telescopes from `18 × 120 − 16`, followed by 384 malformed rows, 475 connectivity prunes, 1,285 checker-valid/well-formed terminals, 1,284 `bootstrap_universe_shape` rejections, and one admitted, evaluated, clearing winner. The raw expressions are real. They do not constitute open competition: the Step-1-specific gate recognizes the authored universe/application pair and was added after that pair; the special 2,144 catalog was added later still. The current in-memory probe reproduces the counters, but it is a homologous current-source check, not a replay of the missing dirty-tree source.\n\n",
    );
    out.push_str(
        "### Step 15 — breadth telemetry, incomplete competition\n\n`7,211` is a prefix-event counter, not a count of complete candidate telescopes; `5,000` is a configured reporting floor. The stored row reports 261 hard-admissible terminals, 553 exact partial-bound prunes, 257 heuristic prefix drops, three incumbent-rank terminal prunes, and only one fully evaluated telescope. The historical run records `dirty_tree=true`, and neither its source diff nor archived binary is available. Even before causal pinning is considered, the one reported winner is not the result of a complete lawful comparison.\n\n",
    );

    out.push_str("## Reselection and T-BF1\n\n");
    out.push_str(
        "v2 reaches eight stages. Stages 1–3 admit one candidate from raw cones of 288, 33, and 56. Stage 4 is the only multi-finalist cone: four real typed candidates are admitted and all four clear the legacy bar. v2 chooses the reference through the older exact acceptance tuple, whose final distinctions include bit/canonical order. T-BF1 recomputes the same four expressions and shows that the audited two-register/parsimony law sees four equal `(κ, ν) = (3,5)` minimizers and must stop. The v2 selected row is therefore not a lawful autonomous winner under the law being audited. Stages 5–7 are singleton winners; Stage 8 is a singleton with no legacy bar clearer.\n\n",
    );
    out.push_str(
        "v3 recomputes Stages 8–15. Every row is exactly `1 enumerated / 1 admitted / 1 deduped / 1 typed total discharger`. Under Guarded mode, `LateFamilySurface::None` returns reference expressions directly for Stages 13–15 and exact position recognizers constrain Stages 8–12. The census is complete over authored singleton cones; it is not autonomous competition.\n\n",
    );
    out.push_str(
        "T-BF1 is the strongest direct multiplicity evidence in the certified prefix: its four candidates are real, typed and co-minimal, and `selected_hash = null`. Its reissuer also requires the live vector to match the frozen four-vector, so it proves the certified four-way tie rather than openness to an unknown fifth expression.\n\n",
    );

    out.push_str("## BI-1 and BI-1b\n\n");
    out.push_str(
        "The common Option-A Stage-4 cone contains four authorized roots and selects none. BI-1 then executes the enacted branch through eleven singleton cones at Stages 5–15. Each of the three alternate BI-1 branches executes singleton cones at Stages 5–7 and stops on the same singleton Stage-8 candidate when its provenance classification is `Unknown` and its discharger count is zero.\n\n",
    );
    out.push_str(
        "BI-1b revalidates the enacted halt without a new enacted census. For each never-enacted branch it consumes the sealed Stage-8 singleton, then freshly enumerates Stages 9–15; every Stage 8–15 row is again `1/1/1/1`. The 24-row correspondence (three branches × eight stages) reports one byte-identical candidate and the same winner in every cell. Branch variation changes the predecessor testimony, not the candidate surface. Current BI reissuance is blocked or deliberately expensive after upstream drift, so the audit uses frozen testimony plus the live Guarded code path and does not claim a successful current replay.\n\n",
    );

    out.push_str("## Step 16\n\n");
    out.push_str(
        "The Step-16 Guarded context is genuinely generic: no focus family, no historical anchor, κ in 2–4, and ordinary open-band admission. The exact corrected surface partition is:\n\n",
    );
    out.push_str("| κ | raw expression product | checker-valid product |\n|---:|---:|---:|\n");
    let step16_full = certificate
        .surfaces
        .iter()
        .find(|row| row.audit_key == "step16_unclamped_full_lane")
        .expect("issued certificate contains Step-16 full row");
    for stage in &step16_full.stages {
        let kappa = stage
            .disposition
            .strip_prefix("kappa_")
            .and_then(|tail| tail.split('_').next())
            .unwrap_or("?");
        out.push_str(&format!(
            "| {} | {} | {} |\n",
            kappa,
            count_cell(&stage.generated_or_enumerated),
            count_cell(&stage.checker_valid_or_admitted),
        ));
    }
    out.push_str(
        "\nThe full materializing lane was attempted and exhausted memory; no complete candidate competition or winner certificate exists. The semantic exhaustion partitions expression products but does not materialize or rank telescopes.\n\n",
    );
    out.push_str(
        "The corrected adversarial probe replays 17 hand-authored shapes through raw membership, identification, admission, typing, connectivity, semantic minimality, valuation, and the bar. It has nine raw members and four complete lawful clearers:\n\n",
    );
    out.push_str("| candidate | κ | raw | admitted | typed | connected | minimal | clears | lawful clearer | ν | ρ |\n");
    out.push_str("|---|---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|---:|---:|\n");
    for candidate in &certificate.probes.step16_adversarial_candidates {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            candidate.name,
            candidate.clause_kappa,
            yes_no(candidate.raw_surface_member),
            yes_no(candidate.admitted),
            yes_no(candidate.type_checks),
            yes_no(candidate.connectivity_passes),
            yes_no(candidate.semantically_minimal),
            yes_no(candidate.clears_bar),
            yes_no(candidate.survives_all_gates_and_clears),
            candidate.nu_total,
            candidate.rho.as_deref().unwrap_or("—"),
        ));
    }
    out.push_str(
        "\nThe four lawful clearers are `hit_no_formation_d1`, `temporal_polymorphic_kappa2`, `axiomatic_single_l15_kappa3`, and `axiomatic_inheritance_kappa3`. They were authored after full enumeration failed and membership-replayed; they were not found by a completed search. SH-1 then selects `hit_no_formation_d1` by the explicit `designated_step16_survivor` name at offset zero. Its later-round rank cannot retroactively make Step 16 an open autonomous win.\n\n",
    );

    out.push_str("## Current probe and replay status\n\n");
    out.push_str(&format!(
        "- **Claim Step 1 homologous probe:** stored-count match `{}`; counts `{}/{}/{}/{}/{}/{}`.\n",
        certificate.probes.claim_step1.matches_stored_counts,
        certificate.probes.claim_step1.generated_raw_prefixes,
        certificate.probes.claim_step1.well_formed_terminals,
        certificate.probes.claim_step1.hard_admissible,
        certificate.probes.claim_step1.exact_pruned,
        certificate.probes.claim_step1.heuristic_dropped,
        certificate.probes.claim_step1.full_telescopes_evaluated,
    ));
    out.push_str(&replay_status_line(
        "Claim Step 15 homologous probe",
        &certificate.probes.claim_step15,
    ));
    out.push_str(&replay_status_line(
        "Phase-5b v2 current-definition replay",
        &certificate.probes.phase5b_v2_replay,
    ));
    out.push_str(&replay_status_line(
        "Phase-5b v3 current-definition replay",
        &certificate.probes.phase5b_v3_replay,
    ));
    out.push_str(&replay_status_line(
        "T-BF1 current reissue",
        &certificate.probes.t_bf1_reissue,
    ));
    out.push_str(&replay_status_line(
        "Certified halt replay",
        &certificate.probes.certified_halt_replay,
    ));
    out.push_str(&format!(
        "- **Step-16 semantic exhaustion artifact is internally complete:** `{}`.\n\n",
        certificate
            .probes
            .step16_semantic_exhaustion_artifact_complete
    ));

    out.push_str("## Causal direction and limits\n\n");
    out.push_str(&format!(
        "- {}\n- {}\n- {}\n- {}\n- {}\n",
        certificate.chronology.reference_table,
        certificate.chronology.early_admissibility_gates,
        certificate.chronology.step1_raw_catalog,
        certificate.chronology.t_bf1_and_bi_artifacts,
        certificate.chronology.consequence,
    ));
    out.push_str(
        "\nThe exact claim v15 source is unavailable because the run was built from a dirty tree. Current homologous runs are explicitly labelled as such. Known infeasible Step-16 materialization is not repeated. Current-definition replay drift is preserved as failure evidence, never rewritten as historical invalidity or success.\n\n",
    );

    out.push_str("## Falsifier compliance and replay\n\n");
    out.push_str(
        "PA-1b assigns no PA-1c grades and uses no narrative ground. All probes are in memory; no sealed input is an output path. Every requested family, all four BI roots, every reached branch stage, the OOM/full-lane failure, all 17 adversarial shapes, zero-winner cones, and current replay failures are present. The standing disclosure is digest-bound and neither weakened nor superseded.\n\n",
    );
    out.push_str(
        "Replay recomputes the payload digest, rejects unknown fields, re-derives all rows and probes from bound inputs/current APIs, rejects a rehashed mutation through deterministic reissuance, and requires this Markdown report to equal deterministic rendering byte-for-byte.\n",
    );
    out
}

fn invalid_replay(error: impl Into<String>) -> Pa1bReplay {
    Pa1bReplay {
        valid: false,
        errors: vec![error.into()],
        qualifying_surface_count: 0,
        global_finding: "replay_failed".to_owned(),
    }
}

pub fn replay_pa1_open_competition_v1(claimed: &Pa1OpenCompetitionV1Certificate) -> Pa1bReplay {
    let expected = match issue_pa1_open_competition_v1() {
        Ok(expected) => expected,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("PA-1b certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("PA-1b source bindings drifted".to_owned());
    }
    if claimed != &expected {
        errors.push("PA-1b certificate differs from deterministic reissuance".to_owned());
    }
    if claimed.qualifying_surface_count != 0
        || claimed.strongest_autonomy_evidence.is_some()
        || claimed.global_finding != PA1B_GLOBAL_FINDING
    {
        errors.push("PA-1b global conditional was not applied exactly".to_owned());
    }
    if claimed.falsifier_compliance.sealed_inputs_written
        || claimed
            .falsifier_compliance
            .disclosure_weakened_or_superseded
        || !claimed
            .falsifier_compliance
            .every_requested_surface_explicit
    {
        errors.push("PA-1b falsifier compliance failed".to_owned());
    }
    Pa1bReplay {
        valid: errors.is_empty(),
        errors,
        qualifying_surface_count: claimed.qualifying_surface_count,
        global_finding: claimed.global_finding.clone(),
    }
}

pub fn replay_pa1_open_competition_v1_json(json: &str) -> Pa1bReplay {
    let raw = match serde_json::from_str::<Value>(json) {
        Ok(value) => value,
        Err(error) => return invalid_replay(format!("invalid PA-1b JSON: {error}")),
    };
    let certificate = match serde_json::from_str::<Pa1OpenCompetitionV1Certificate>(json) {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(format!("invalid PA-1b JSON: {error}")),
    };
    let projected = match serde_json::to_value(&certificate) {
        Ok(value) => value,
        Err(error) => return invalid_replay(format!("invalid PA-1b projection: {error}")),
    };
    if raw != projected {
        return invalid_replay("unknown or ignored field changes PA-1b JSON projection");
    }
    replay_pa1_open_competition_v1(&certificate)
}

pub fn verify_pa1_open_competition_report(
    certificate: &Pa1OpenCompetitionV1Certificate,
    report: &str,
) -> Result<(), Pa1bError> {
    let expected = render_pa1_open_competition_v1(certificate);
    if report != expected {
        return Err(Pa1bError::Invariant(
            "PA-1b report differs from deterministic rendering".to_owned(),
        ));
    }
    Ok(())
}

pub fn pa1b_bytes_blake3(bytes: &[u8]) -> String {
    bytes_hash(bytes)
}

pub fn emit_pa1_open_competition_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Pa1OpenCompetitionV1Certificate, Pa1bError> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Pa1bError::Io(
            "create-new refused because a PA-1b output already exists".to_owned(),
        ));
    }
    let certificate = issue_pa1_open_competition_v1()?;
    let replay = replay_pa1_open_competition_v1(&certificate);
    if !replay.valid {
        return Err(Pa1bError::Invariant(format!(
            "new PA-1b certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Pa1bError::Json(error.to_string()))?;
    json.push('\n');
    let report = render_pa1_open_competition_v1(&certificate);

    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(certificate_path)
        .map_err(|error| Pa1bError::Io(error.to_string()))?;
    if let Err(error) = certificate_file.write_all(json.as_bytes()) {
        let _ = remove_file(certificate_path);
        return Err(Pa1bError::Io(error.to_string()));
    }
    let mut report_file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
    {
        Ok(file) => file,
        Err(error) => {
            let _ = remove_file(certificate_path);
            return Err(Pa1bError::Io(error.to_string()));
        }
    };
    if let Err(error) = report_file.write_all(report.as_bytes()) {
        let _ = remove_file(report_path);
        let _ = remove_file(certificate_path);
        return Err(Pa1bError::Io(error.to_string()));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pa1b_issues_complete_negative_conditional_with_open_no_winner_rows() {
        let certificate = issue_pa1_open_competition_v1().expect("PA-1b issues");
        assert_eq!(certificate.surfaces.len(), 20);
        assert_eq!(certificate.qualifying_surface_count, 0);
        assert_eq!(certificate.strongest_autonomy_evidence, None);
        assert_eq!(certificate.global_finding, PA1B_GLOBAL_FINDING);
        assert!(certificate.surfaces.iter().any(|row| {
            row.audit_key == "t_bf1_stage_4"
                && row.verdict == Pa1bVerdict::OpenConeNoLawfulWinner
                && !row.winner_selected
        }));
        assert!(certificate.surfaces.iter().any(|row| {
            row.audit_key == "step16_unclamped_full_lane"
                && row.admission_target_independent
                && !row.complete_selection_input
        }));
        assert_eq!(certificate.probes.step16_lawful_clearers, "4");
        assert_eq!(certificate.probes.step16_adversarial_candidates.len(), 17);
    }

    #[test]
    fn pa1b_replay_rejects_plain_and_rehashed_mutation() {
        let certificate = issue_pa1_open_competition_v1().expect("PA-1b issues");
        assert!(replay_pa1_open_competition_v1(&certificate).valid);

        let mut plain = certificate.clone();
        plain.global_finding.push_str(" altered");
        assert!(!replay_pa1_open_competition_v1(&plain).valid);

        let mut rehashed = certificate;
        rehashed.surfaces[0].winner_autonomous = true;
        rehashed.result_digest = certificate_digest(&rehashed);
        let replay = replay_pa1_open_competition_v1(&rehashed);
        assert!(!replay.valid);
        assert!(
            replay
                .errors
                .iter()
                .any(|error| error.contains("deterministic reissuance"))
        );
    }

    #[test]
    fn pa1b_json_replay_rejects_unknown_field() {
        let certificate = issue_pa1_open_competition_v1().expect("PA-1b issues");
        let mut value = serde_json::to_value(certificate).expect("serialize PA-1b");
        value
            .as_object_mut()
            .expect("PA-1b object")
            .insert("ignored_field".to_owned(), Value::Bool(true));
        let replay = replay_pa1_open_competition_v1_json(
            &serde_json::to_string(&value).expect("mutated JSON"),
        );
        assert!(!replay.valid);
    }

    #[test]
    fn pa1b_report_is_deterministic_and_guarded() {
        let certificate = issue_pa1_open_competition_v1().expect("PA-1b issues");
        let report = render_pa1_open_competition_v1(&certificate);
        verify_pa1_open_competition_report(&certificate, &report)
            .expect("rendered report verifies");
        assert!(
            verify_pa1_open_competition_report(&certificate, &(report + "\nmutation")).is_err()
        );
    }
}
