//! BI-1-only Option-A branch sweep.
//!
//! This module deliberately stops before BI-2 and BI-4.  It first consumes
//! the independently issued Option-A execution capability, executes and
//! seals the enacted branch, and only then reads the T-BI v6 positive
//! projection for a post-seal regression.  A passing regression exports a
//! narrow authorization containing no enacted winner or ledger values; that
//! authorization is the sole extra input admitted to the other three branch
//! issuers.

use crate::branch_invariance::{
    BranchContinuation, BranchContinuationLimits, CertifiedStage4BranchCone,
    execute_branch_continuation, issue_certified_stage4_branch_cone_from_replayed_option_a,
    replay_branch_continuation, replay_certified_stage4_branch_cone_from_replayed_option_a,
};
use crate::stage4_option_a_execution_v1::{
    Stage4OptionAReplayedExecutionGrantV1, issue_replayed_stage4_option_a_execution_v1,
};
use crate::t_bi_nu1_regression_v6::{
    TBiNu1RegressionV6PassedProjection, issue_t_bi_nu1_regression_v6_passed_projection,
    replay_t_bi_nu1_regression_v6_passed_projection,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const BI1_OPTION_A_SWEEP_V3_SCHEMA: &str = "bi1-option-a-enacted-first-sweep-v3";
pub const BI1_BRANCH_V3_SCHEMA: &str = "bi1-option-a-independent-branch-v3";
pub const BI1_ENACTED_REGRESSION_V3_SCHEMA: &str =
    "bi1-option-a-enacted-postseal-tbi-v6-regression-v3";
pub const BI1_OPTION_A_SWEEP_V3_DATE: &str = "2026-07-23";
pub const BI1_MAX_INSPECTED_STAGE: u32 = 64;
pub const BI1_MAX_ENUMERATED_CANDIDATES_PER_STAGE: usize = 10_000_000;
pub const BI1_SWEEP_CERTIFICATE_NAME: &str = "BI1_OPTION_A_SWEEP_V3_CERTIFICATE.json";
pub const BI1_SWEEP_REPORT_NAME: &str = "BI1_OPTION_A_SWEEP_V3_RESULT.md";

const OPTION_A_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/stage4_semantic_divergence_adjudication.md");
const BI_PROGRAM_PLAN_BYTES: &[u8] =
    include_bytes!("../../../docs/branch_invariance_program_plan.md");
const BRANCH_CORE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance.rs");
const OPTION_A_CAPABILITY_SOURCE_BYTES: &[u8] = include_bytes!("stage4_option_a_execution_v1.rs");
const T_BI_V6_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_nu1_regression_v6.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1SourceBindingV3 {
    pub path: String,
    pub role: String,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1GuardedStageSealV3 {
    pub stage: u32,
    pub demand_derivation_hash: String,
    pub discharger_count: usize,
    pub discharger_hashes: Vec<String>,
    pub winner_hash: Option<String>,
    pub winner_kernel_typed: bool,
    pub winner_exact_semantic_provenance: bool,
    pub winner_total_discharge_replayed: bool,
    pub candidate_semantic_gaps_absent: bool,
    pub unique_typed_total_discharger: bool,
    pub selection_disposition: String,
    pub no_improvised_selection: bool,
    pub value_used_as_selector: bool,
    pub bar_used_as_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1BranchCertificateV3 {
    pub schema: String,
    pub date: String,
    pub option_a_execution_capability_digest: String,
    pub cone_digest: String,
    pub branch_root_hash: String,
    pub branch_digest_prefix: String,
    pub enacted_root: bool,
    pub issuance_ordinal: usize,
    pub enacted_regression_authorization_digest: Option<String>,
    pub continuation: BranchContinuation,
    pub continuation_replayed: bool,
    pub candidate_hash_vector: Vec<String>,
    pub semantic_nu_vector: Vec<u32>,
    pub guarded_stages: Vec<Bi1GuardedStageSealV3>,
    pub every_reached_guarded_stage_has_unique_typed_discharger: bool,
    pub every_reached_live_demand_totally_discharged: bool,
    pub lawful_stop_sealed_without_improvised_selection: bool,
    pub semantic_value_never_used_as_selector: bool,
    pub bar_never_used_as_selector: bool,
    pub hash_or_enumeration_order_never_used_as_selector: bool,
    pub expected_or_enacted_outcomes_never_accepted_as_runner_input: bool,
    pub resource_limit_never_used_as_halt_claim: bool,
    pub enacted_outcomes_consumed_by_branch_issuer: bool,
    pub f_s4_3_preserved: bool,
    pub branch_indexed_only: bool,
    pub bi2_finale_issued: bool,
    pub bi4_cone_verdict_issued: bool,
    pub unindexed_halt_or_o_claim_issued: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1EnactedRegressionAuthorizationV3 {
    pub schema: String,
    pub option_a_execution_capability_digest: String,
    pub enacted_root_hash: String,
    pub enacted_branch_certificate_digest: String,
    pub t_bi_v6_projection_digest: String,
    pub enacted_certificate_sealed_before_t_bi_projection_issued: bool,
    pub enacted_certificate_replayed_before_comparison: bool,
    pub exact_candidate_hash_vector: bool,
    pub exact_semantic_nu_vector: bool,
    pub unique_typed_dischargers_replayed: bool,
    pub t_bi_v6_projection_replayed: bool,
    pub t_bi_v6_extraction_complete: bool,
    pub t_bi_v6_operational_regression_exact: bool,
    pub t_bi_v6_f_al1_prime_passed: bool,
    pub t_bi_v6_non_enacted_work_absent: bool,
    pub enacted_regression_passed: bool,
    /// This authorization deliberately contains no candidate or nu vector.
    pub enacted_outcome_vectors_exported: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1BranchManifestRowV3 {
    pub issuance_ordinal: usize,
    pub branch_root_hash: String,
    pub branch_digest_prefix: String,
    pub enacted_root: bool,
    pub certificate_digest: String,
    pub continuation_outcome: String,
    pub reached_guarded_stage_count: usize,
    pub every_reached_guarded_stage_has_unique_typed_discharger: bool,
    pub branch_indexed_only: bool,
    pub json_name: String,
    pub report_name: String,
    pub row_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1OptionASweepManifestV3 {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Bi1SourceBindingV3>,
    pub option_a_execution_capability_digest: String,
    pub option_a_capability_replayed_before_any_branch: bool,
    pub option_a_real_choice_preserved: bool,
    pub cone_digest: String,
    pub cone_replayed_before_any_branch: bool,
    pub enacted_root_hash: String,
    pub enacted_branch_issued_first: bool,
    pub enacted_regression: Bi1EnactedRegressionAuthorizationV3,
    pub enacted_regression_passed_before_non_enacted_issuance: bool,
    pub branch_rows: Vec<Bi1BranchManifestRowV3>,
    pub branch_count: usize,
    pub exact_four_root_surface: bool,
    pub all_branch_certificates_independently_replayed: bool,
    pub all_reached_guarded_stages_have_unique_typed_dischargers: bool,
    pub all_lawful_stops_sealed_without_improvised_selection: bool,
    pub non_enacted_branch_count: usize,
    pub non_enacted_branches_consumed_enacted_outcomes: bool,
    pub value_or_bar_used_as_selector: bool,
    pub divergence_suppression_or_repair_performed: bool,
    pub class_representative_substitution_used: bool,
    pub f_s4_3_preserved: bool,
    pub f_bi1_preserved: bool,
    pub f_bi2_preserved: bool,
    pub f_bi3_preserved: bool,
    pub f_bi4_preserved: bool,
    pub f_bi5_preserved: bool,
    pub f_bi6_preserved: bool,
    pub cross_branch_g2_verdict_issued: bool,
    pub cross_branch_g3_verdict_issued: bool,
    pub bi2_finale_issued: bool,
    pub bi4_cone_report_issued: bool,
    pub unindexed_halt_ledger_or_o_claim_issued: bool,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bi1OptionASweepBundleV3 {
    pub branches: Vec<Bi1BranchCertificateV3>,
    pub manifest: Bi1OptionASweepManifestV3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1ReplayV3 {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bi1SweepV3Error {
    #[error("Option-A execution capability failed: {0}")]
    Capability(String),
    #[error("BI-1 continuation failed: {0}")]
    Continuation(String),
    #[error("enacted post-seal regression failed: {0}")]
    Regression(String),
    #[error("BI-1 invariant failed: {0}")]
    Invariant(String),
    #[error("BI-1 JSON failed: {0}")]
    Json(String),
    #[error("BI-1 create-new I/O failed: {0}")]
    Io(String),
}

#[derive(Clone)]
struct OptionACapabilityViewV3 {
    grant: Stage4OptionAReplayedExecutionGrantV1,
    digest: String,
    enacted_root_hash: String,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI1_OPTION_A_SWEEP_V3_SCHEMA, domain, value))
        .expect("BI-1 v3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn branch_digest(certificate: &Bi1BranchCertificateV3) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(BI1_BRANCH_V3_SCHEMA, &projection)
}

fn regression_digest(regression: &Bi1EnactedRegressionAuthorizationV3) -> String {
    let mut projection = regression.clone();
    projection.derivation_hash.clear();
    tagged_hash(BI1_ENACTED_REGRESSION_V3_SCHEMA, &projection)
}

fn manifest_digest(manifest: &Bi1OptionASweepManifestV3) -> String {
    let mut projection = manifest.clone();
    projection.result_digest.clear();
    tagged_hash("sweep-manifest", &projection)
}

fn guarded_stage_digest(row: &Bi1GuardedStageSealV3) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("guarded-stage-seal", &projection)
}

fn manifest_row_digest(row: &Bi1BranchManifestRowV3) -> String {
    let mut projection = row.clone();
    projection.row_hash.clear();
    tagged_hash("branch-manifest-row", &projection)
}

fn digest_prefix(digest: &str) -> String {
    digest
        .strip_prefix("blake3:")
        .unwrap_or(digest)
        .chars()
        .take(12)
        .collect()
}

fn branch_json_name(prefix: &str) -> String {
    format!("BI_BRANCH_V3_{prefix}_CERTIFICATE.json")
}

fn branch_report_name(prefix: &str) -> String {
    format!("BI_BRANCH_V3_{prefix}_RESULT.md")
}

fn source_bindings() -> Vec<Bi1SourceBindingV3> {
    [
        (
            "docs/stage4_semantic_divergence_adjudication.md",
            "adopted Option A and F-S4-3",
            OPTION_A_ADJUDICATION_BYTES,
        ),
        (
            "docs/branch_invariance_program_plan.md",
            "F-BI1 through F-BI6 and BI-1/BI-2 separation",
            BI_PROGRAM_PLAN_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance.rs",
            "crate-private branch-parametric continuation core",
            BRANCH_CORE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/stage4_option_a_execution_v1.rs",
            "replayed four-root Option-A execution capability",
            OPTION_A_CAPABILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bi_nu1_regression_v6.rs",
            "post-seal enacted candidate and semantic-vector comparator",
            T_BI_V6_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Bi1SourceBindingV3 {
        path: path.to_owned(),
        role: role.to_owned(),
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn issue_option_a_capability_view() -> Result<OptionACapabilityViewV3, Bi1SweepV3Error> {
    let grant = issue_replayed_stage4_option_a_execution_v1()
        .map_err(|error| Bi1SweepV3Error::Capability(error.to_string()))?;
    let capability = grant.capability();
    let replay = grant.replay();
    if !replay.valid
        || replay.root_count != 4
        || !replay.all_four_execution_authorized
        || replay.selector_capability_present
        || !capability.all_four_execution_authorized()
        || !capability.complete_sweep_required_for_cone_verdict()
        || !capability.has_no_selector()
        || !capability.semantic_nu_is_diagnostic_only()
        || !capability.bar_is_diagnostic_only()
        || !capability.f_s4_3_armed()
    {
        return Err(Bi1SweepV3Error::Capability(format!(
            "capability did not replay as the exact selector-free four-root Option-A surface: {}",
            replay.errors.join("; ")
        )));
    }
    let digest = capability.derivation_hash().to_owned();
    let enacted_root_hash = capability.enacted_indexical_root_hash().to_owned();
    Ok(OptionACapabilityViewV3 {
        grant,
        digest,
        enacted_root_hash,
    })
}

fn issue_replayed_cone(
    capability: &OptionACapabilityViewV3,
) -> Result<CertifiedStage4BranchCone, Bi1SweepV3Error> {
    let cone = issue_certified_stage4_branch_cone_from_replayed_option_a(&capability.grant)
        .map_err(|error| Bi1SweepV3Error::Continuation(error.to_string()))?;
    let replay_errors =
        replay_certified_stage4_branch_cone_from_replayed_option_a(&capability.grant, &cone);
    if !replay_errors.is_empty() {
        return Err(Bi1SweepV3Error::Continuation(format!(
            "Option-A cone did not replay: {}",
            replay_errors.join("; ")
        )));
    }
    if cone.branches.len() != 4
        || cone.branch_count != 4
        || !cone.no_branch_selected
        || cone.option_a_execution_capability_digest != capability.digest
        || !cone.option_a_execution_capability_replay_valid
        || !cone.semantic_nu_diagnostic_only_at_guarded_stages
        || !cone.bar_diagnostic_only_at_guarded_stages
        || !cone.f_s4_3_armed
        || cone.guarded_value_selection_used
        || !cone
            .branches
            .iter()
            .any(|branch| branch.candidate_hash == capability.enacted_root_hash)
    {
        return Err(Bi1SweepV3Error::Invariant(
            "Option-A capability/cone join is not the exact unselected four-root surface"
                .to_owned(),
        ));
    }
    Ok(cone)
}

fn issue_guarded_stage_seals(
    continuation: &BranchContinuation,
) -> Result<Vec<Bi1GuardedStageSealV3>, Bi1SweepV3Error> {
    let mut seals = Vec::with_capacity(continuation.stages.len());
    for stage in &continuation.stages {
        let flagged = stage
            .assessments
            .iter()
            .filter(|assessment| assessment.guarded_total_discharger)
            .collect::<Vec<_>>();
        let mut replayed_discharger_hashes = flagged
            .iter()
            .map(|assessment| assessment.candidate_hash.clone())
            .collect::<Vec<_>>();
        replayed_discharger_hashes.sort();
        let discharger_surface_exact = stage.discharger_count == flagged.len()
            && stage.discharger_hashes == replayed_discharger_hashes;
        let winner_assessment = stage.winner.as_ref().and_then(|winner| {
            flagged
                .iter()
                .copied()
                .find(|assessment| assessment.candidate_hash == winner.candidate_hash)
        });
        let winner_kernel_typed = winner_assessment.is_some_and(|row| row.kernel_typed);
        let winner_exact_semantic_provenance =
            winner_assessment.is_some_and(|row| row.nu_provenance.is_exact_certified());
        let winner_total_discharge_replayed =
            winner_assessment.is_some_and(|row| row.every_live_structural_demand_realized);
        let candidate_semantic_gaps_absent = stage
            .assessments
            .iter()
            .all(|candidate| !candidate.semantic_evidence_gap);
        let unique_typed_total_discharger = stage.discharger_count == 1
            && flagged.len() == 1
            && discharger_surface_exact
            && stage
                .winner
                .as_ref()
                .is_some_and(|winner| winner.candidate_hash == flagged[0].candidate_hash)
            && winner_kernel_typed
            && winner_exact_semantic_provenance
            && winner_total_discharge_replayed
            && candidate_semantic_gaps_absent;
        let terminal_stop_kind = match &continuation.outcome {
            crate::branch_invariance::BranchContinuationOutcome::HaltedNoGuardedDischarger {
                stage: stop_stage,
            } if *stop_stage == stage.stage => Some("lawful_stop_no_guarded_discharger"),
            crate::branch_invariance::BranchContinuationOutcome::HaltedMultipleGuardedDischargers {
                stage: stop_stage,
                ..
            } if *stop_stage == stage.stage => Some("lawful_stop_multiple_guarded_dischargers"),
            crate::branch_invariance::BranchContinuationOutcome::ExpressivityGap {
                stage: stop_stage,
                ..
            } if *stop_stage == stage.stage => Some("lawful_stop_expressivity_gap"),
            _ => None,
        };
        let selection_disposition = if unique_typed_total_discharger {
            "unique_typed_total_discharger"
        } else {
            terminal_stop_kind.unwrap_or("invalid_unsealed_selection_state")
        };
        let no_improvised_selection = discharger_surface_exact
            && (unique_typed_total_discharger
                || (terminal_stop_kind.is_some() && stage.winner.is_none()));
        let mut seal = Bi1GuardedStageSealV3 {
            stage: stage.stage,
            demand_derivation_hash: stage.demand.derivation_hash.clone(),
            discharger_count: stage.discharger_count,
            discharger_hashes: replayed_discharger_hashes,
            winner_hash: stage
                .winner
                .as_ref()
                .map(|winner| winner.candidate_hash.clone()),
            winner_kernel_typed,
            winner_exact_semantic_provenance,
            winner_total_discharge_replayed,
            candidate_semantic_gaps_absent,
            unique_typed_total_discharger,
            selection_disposition: selection_disposition.to_owned(),
            no_improvised_selection,
            value_used_as_selector: stage.score_used_as_selector,
            bar_used_as_selector: stage.bar_used_as_gate_or_selector,
            derivation_hash: String::new(),
        };
        seal.derivation_hash = guarded_stage_digest(&seal);
        if !seal.no_improvised_selection || seal.value_used_as_selector || seal.bar_used_as_selector
        {
            return Err(Bi1SweepV3Error::Invariant(format!(
                "Stage {} failed the replayed demand-only selection/stop boundary",
                stage.stage
            )));
        }
        seals.push(seal);
    }
    Ok(seals)
}

fn issue_branch_certificate_from_cone(
    cone: &CertifiedStage4BranchCone,
    option_a_execution_capability_digest: &str,
    branch_root_hash: &str,
    enacted_root: bool,
    issuance_ordinal: usize,
    enacted_regression_authorization_digest: Option<String>,
) -> Result<Bi1BranchCertificateV3, Bi1SweepV3Error> {
    if enacted_root && (issuance_ordinal != 0 || enacted_regression_authorization_digest.is_some())
    {
        return Err(Bi1SweepV3Error::Invariant(
            "the enacted branch must be ordinal zero and cannot consume its own regression authorization"
                .to_owned(),
        ));
    }
    if !enacted_root && enacted_regression_authorization_digest.is_none() {
        return Err(Bi1SweepV3Error::Invariant(
            "a non-enacted branch was requested before the enacted regression gate".to_owned(),
        ));
    }
    if !enacted_root && issuance_ordinal == 0 {
        return Err(Bi1SweepV3Error::Invariant(
            "a non-enacted branch cannot occupy the enacted-first issuance slot".to_owned(),
        ));
    }
    let limits = BranchContinuationLimits {
        max_inspected_stage: BI1_MAX_INSPECTED_STAGE,
        max_enumerated_candidates_per_stage: BI1_MAX_ENUMERATED_CANDIDATES_PER_STAGE,
    };
    let continuation = execute_branch_continuation(cone, branch_root_hash, &limits)
        .map_err(|error| Bi1SweepV3Error::Continuation(error.to_string()))?;
    let continuation_errors = replay_branch_continuation(cone, &continuation);
    if !continuation_errors.is_empty() {
        return Err(Bi1SweepV3Error::Continuation(format!(
            "branch continuation did not replay: {}",
            continuation_errors.join("; ")
        )));
    }
    if continuation.branch.candidate_hash != branch_root_hash {
        return Err(Bi1SweepV3Error::Invariant(
            "continuation root differs from the requested Option-A root".to_owned(),
        ));
    }
    let guarded_stages = issue_guarded_stage_seals(&continuation)?;
    let every_reached_guarded_stage_has_unique_typed_discharger = !guarded_stages.is_empty()
        && guarded_stages
            .iter()
            .all(|stage| stage.unique_typed_total_discharger);
    let every_reached_live_demand_totally_discharged = guarded_stages
        .iter()
        .all(|stage| stage.winner_total_discharge_replayed);
    let lawful_stop_sealed_without_improvised_selection = guarded_stages
        .iter()
        .all(|stage| stage.no_improvised_selection);
    let semantic_value_never_used_as_selector = continuation.score_never_used_as_selector
        && guarded_stages
            .iter()
            .all(|stage| !stage.value_used_as_selector);
    let bar_never_used_as_selector = continuation.bar_never_used_as_gate_or_selector
        && guarded_stages
            .iter()
            .all(|stage| !stage.bar_used_as_selector);
    let hash_or_enumeration_order_never_used_as_selector =
        !continuation.hash_or_enumeration_order_used_as_selector;
    let expected_or_enacted_outcomes_never_accepted_as_runner_input =
        !continuation.expected_winner_score_ledger_or_bar_accepted_as_runner_input;
    let resource_limit_never_used_as_halt_claim = !continuation.resource_limit_used_as_halt_claim;
    let enacted_outcomes_consumed_by_branch_issuer = false;
    let f_s4_3_preserved = semantic_value_never_used_as_selector && bar_never_used_as_selector;
    if !lawful_stop_sealed_without_improvised_selection
        || !f_s4_3_preserved
        || !hash_or_enumeration_order_never_used_as_selector
        || !expected_or_enacted_outcomes_never_accepted_as_runner_input
        || !resource_limit_never_used_as_halt_claim
    {
        return Err(Bi1SweepV3Error::Invariant(
            "branch failed the BI-1 demand-only, value-blind continuation gate".to_owned(),
        ));
    }
    let candidate_hash_vector = continuation
        .complete_ledger
        .iter()
        .map(|row| row.candidate_hash.clone())
        .collect();
    let semantic_nu_vector = continuation
        .complete_ledger
        .iter()
        .map(|row| row.semantic_nu)
        .collect();
    let mut certificate = Bi1BranchCertificateV3 {
        schema: BI1_BRANCH_V3_SCHEMA.to_owned(),
        date: BI1_OPTION_A_SWEEP_V3_DATE.to_owned(),
        option_a_execution_capability_digest: option_a_execution_capability_digest.to_owned(),
        cone_digest: cone.derivation_hash.clone(),
        branch_root_hash: branch_root_hash.to_owned(),
        branch_digest_prefix: digest_prefix(branch_root_hash),
        enacted_root,
        issuance_ordinal,
        enacted_regression_authorization_digest,
        continuation,
        continuation_replayed: true,
        candidate_hash_vector,
        semantic_nu_vector,
        guarded_stages,
        every_reached_guarded_stage_has_unique_typed_discharger,
        every_reached_live_demand_totally_discharged,
        lawful_stop_sealed_without_improvised_selection,
        semantic_value_never_used_as_selector,
        bar_never_used_as_selector,
        hash_or_enumeration_order_never_used_as_selector,
        expected_or_enacted_outcomes_never_accepted_as_runner_input,
        resource_limit_never_used_as_halt_claim,
        enacted_outcomes_consumed_by_branch_issuer,
        f_s4_3_preserved,
        branch_indexed_only: true,
        bi2_finale_issued: false,
        bi4_cone_verdict_issued: false,
        unindexed_halt_or_o_claim_issued: false,
        result_digest: String::new(),
    };
    certificate.result_digest = branch_digest(&certificate);
    Ok(certificate)
}

fn issue_t_bi_v6_projection_postseal() -> Result<TBiNu1RegressionV6PassedProjection, Bi1SweepV3Error>
{
    let projection = issue_t_bi_nu1_regression_v6_passed_projection()
        .map_err(|error| Bi1SweepV3Error::Regression(error.to_string()))?;
    let replay_errors = replay_t_bi_nu1_regression_v6_passed_projection(&projection);
    if !replay_errors.is_empty() {
        return Err(Bi1SweepV3Error::Regression(format!(
            "T-BI v6 projection did not replay: {}",
            replay_errors.join("; ")
        )));
    }
    Ok(projection)
}

fn issue_enacted_regression(
    capability: &OptionACapabilityViewV3,
    enacted: &Bi1BranchCertificateV3,
) -> Result<Bi1EnactedRegressionAuthorizationV3, Bi1SweepV3Error> {
    // This is the first historical outcome read in the sweep.  `enacted` is
    // already content-addressed and its continuation has replayed above.
    let t_bi = issue_t_bi_v6_projection_postseal()?;
    let exact_candidate_hash_vector =
        enacted.candidate_hash_vector == t_bi.exact_prefix_candidate_hashes;
    let exact_semantic_nu_vector = enacted.semantic_nu_vector == t_bi.semantic_register;
    let enacted_certificate_replayed_before_comparison =
        enacted.continuation_replayed && enacted.result_digest == branch_digest(enacted);
    let unique_typed_dischargers_replayed =
        enacted.every_reached_guarded_stage_has_unique_typed_discharger;
    let t_bi_v6_projection_replayed = true;
    let t_bi_v6_extraction_complete = t_bi.extraction_complete;
    let t_bi_v6_operational_regression_exact = t_bi.exact_operational_regression;
    let t_bi_v6_f_al1_prime_passed = t_bi.f_al1_prime_passed;
    let t_bi_v6_non_enacted_work_absent = !t_bi.non_enacted_branch_work_executed;
    let enacted_regression_passed = enacted.enacted_root
        && enacted.branch_root_hash == capability.enacted_root_hash
        && enacted.candidate_hash_vector.len() == 15
        && exact_candidate_hash_vector
        && exact_semantic_nu_vector
        && enacted_certificate_replayed_before_comparison
        && unique_typed_dischargers_replayed
        && t_bi_v6_projection_replayed
        && t_bi_v6_extraction_complete
        && t_bi_v6_operational_regression_exact
        && t_bi_v6_f_al1_prime_passed
        && t_bi_v6_non_enacted_work_absent;
    let mut authorization = Bi1EnactedRegressionAuthorizationV3 {
        schema: BI1_ENACTED_REGRESSION_V3_SCHEMA.to_owned(),
        option_a_execution_capability_digest: capability.digest.clone(),
        enacted_root_hash: capability.enacted_root_hash.clone(),
        enacted_branch_certificate_digest: enacted.result_digest.clone(),
        t_bi_v6_projection_digest: t_bi.derivation_hash,
        enacted_certificate_sealed_before_t_bi_projection_issued: true,
        enacted_certificate_replayed_before_comparison,
        exact_candidate_hash_vector,
        exact_semantic_nu_vector,
        unique_typed_dischargers_replayed,
        t_bi_v6_projection_replayed,
        t_bi_v6_extraction_complete,
        t_bi_v6_operational_regression_exact,
        t_bi_v6_f_al1_prime_passed,
        t_bi_v6_non_enacted_work_absent,
        enacted_regression_passed,
        enacted_outcome_vectors_exported: false,
        derivation_hash: String::new(),
    };
    authorization.derivation_hash = regression_digest(&authorization);
    if !authorization.enacted_regression_passed {
        return Err(Bi1SweepV3Error::Regression(format!(
            "F-BI5 enacted regression failed: candidate-vector={}, semantic-vector={}, unique-dischargers={}, extraction={}, F-AL1-prime={}",
            authorization.exact_candidate_hash_vector,
            authorization.exact_semantic_nu_vector,
            authorization.unique_typed_dischargers_replayed,
            authorization.t_bi_v6_extraction_complete,
            authorization.t_bi_v6_f_al1_prime_passed,
        )));
    }
    Ok(authorization)
}

fn manifest_row(certificate: &Bi1BranchCertificateV3) -> Bi1BranchManifestRowV3 {
    let mut row = Bi1BranchManifestRowV3 {
        issuance_ordinal: certificate.issuance_ordinal,
        branch_root_hash: certificate.branch_root_hash.clone(),
        branch_digest_prefix: certificate.branch_digest_prefix.clone(),
        enacted_root: certificate.enacted_root,
        certificate_digest: certificate.result_digest.clone(),
        continuation_outcome: format!("{:?}", certificate.continuation.outcome),
        reached_guarded_stage_count: certificate.guarded_stages.len(),
        every_reached_guarded_stage_has_unique_typed_discharger: certificate
            .every_reached_guarded_stage_has_unique_typed_discharger,
        branch_indexed_only: certificate.branch_indexed_only,
        json_name: branch_json_name(&certificate.branch_digest_prefix),
        report_name: branch_report_name(&certificate.branch_digest_prefix),
        row_hash: String::new(),
    };
    row.row_hash = manifest_row_digest(&row);
    row
}

/// Execute BI-1 in the only authorized order.  The returned bundle carries
/// four independent branch certificates plus a digest-only sweep manifest.
pub fn issue_bi1_option_a_sweep_v3() -> Result<Bi1OptionASweepBundleV3, Bi1SweepV3Error> {
    let capability = issue_option_a_capability_view()?;
    let cone = issue_replayed_cone(&capability)?;

    let enacted = issue_branch_certificate_from_cone(
        &cone,
        &capability.digest,
        &capability.enacted_root_hash,
        true,
        0,
        None,
    )?;
    // The continuation issuer has already run the independent core replay.
    // Seal validation is deliberately cheap here so the enacted-first gate
    // does not execute the branch a second time before the T-BI comparison.
    if !enacted.continuation_replayed || enacted.result_digest != branch_digest(&enacted) {
        return Err(Bi1SweepV3Error::Invariant(
            "enacted branch did not replay and seal before comparison".to_owned(),
        ));
    }
    let enacted_regression = issue_enacted_regression(&capability, &enacted)?;
    let authorization_digest = enacted_regression.derivation_hash.clone();

    // Only the narrow authorization digest crosses this boundary.  No
    // enacted winner, candidate vector, nu vector, or continuation is an
    // argument to a non-enacted issuer.
    let mut remaining_roots = cone
        .branches
        .iter()
        .map(|branch| branch.candidate_hash.clone())
        .filter(|root| root != &capability.enacted_root_hash)
        .collect::<Vec<_>>();
    remaining_roots.sort();
    let mut branches = vec![enacted];
    for (index, root) in remaining_roots.iter().enumerate() {
        let certificate = issue_branch_certificate_from_cone(
            &cone,
            &capability.digest,
            root,
            false,
            index + 1,
            Some(authorization_digest.clone()),
        )?;
        branches.push(certificate);
    }

    let roots = branches
        .iter()
        .map(|branch| branch.branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    let cone_roots = cone
        .branches
        .iter()
        .map(|branch| branch.candidate_hash.as_str())
        .collect::<BTreeSet<_>>();
    let exact_four_root_surface = branches.len() == 4 && roots == cone_roots;
    let all_branch_certificates_independently_replayed = branches.iter().all(|branch| {
        branch.continuation_replayed
            && branch.result_digest == branch_digest(branch)
            && branch
                .guarded_stages
                .iter()
                .all(|stage| stage.derivation_hash == guarded_stage_digest(stage))
    });
    let all_reached_guarded_stages_have_unique_typed_dischargers = branches
        .iter()
        .all(|branch| branch.every_reached_guarded_stage_has_unique_typed_discharger);
    let all_lawful_stops_sealed_without_improvised_selection = branches
        .iter()
        .all(|branch| branch.lawful_stop_sealed_without_improvised_selection);
    let non_enacted_branch_count = branches
        .iter()
        .filter(|branch| !branch.enacted_root)
        .count();
    let non_enacted_branches_consumed_enacted_outcomes = branches
        .iter()
        .filter(|branch| !branch.enacted_root)
        .any(|branch| branch.enacted_outcomes_consumed_by_branch_issuer);
    let value_or_bar_used_as_selector = branches.iter().any(|branch| {
        !branch.semantic_value_never_used_as_selector || !branch.bar_never_used_as_selector
    });
    let f_s4_3_preserved = !value_or_bar_used_as_selector;
    let branch_rows = branches.iter().map(manifest_row).collect::<Vec<_>>();
    let f_bi1_preserved = !non_enacted_branches_consumed_enacted_outcomes;
    let f_bi2_preserved = all_lawful_stops_sealed_without_improvised_selection;
    let divergence_suppression_or_repair_performed = false;
    let class_representative_substitution_used = false;
    let f_bi3_preserved = !divergence_suppression_or_repair_performed;
    let f_bi4_preserved = exact_four_root_surface && !class_representative_substitution_used;
    let f_bi5_preserved = enacted_regression.enacted_regression_passed;
    let f_bi6_preserved = branches.iter().all(|branch| branch.branch_indexed_only);
    let mut manifest = Bi1OptionASweepManifestV3 {
        schema: BI1_OPTION_A_SWEEP_V3_SCHEMA.to_owned(),
        date: BI1_OPTION_A_SWEEP_V3_DATE.to_owned(),
        source_bindings: source_bindings(),
        option_a_execution_capability_digest: capability.digest.clone(),
        option_a_capability_replayed_before_any_branch: true,
        option_a_real_choice_preserved: true,
        cone_digest: cone.derivation_hash.clone(),
        cone_replayed_before_any_branch: true,
        enacted_root_hash: capability.enacted_root_hash.clone(),
        enacted_branch_issued_first: branches
            .first()
            .is_some_and(|branch| branch.enacted_root && branch.issuance_ordinal == 0),
        enacted_regression,
        enacted_regression_passed_before_non_enacted_issuance: true,
        branch_rows,
        branch_count: branches.len(),
        exact_four_root_surface,
        all_branch_certificates_independently_replayed,
        all_reached_guarded_stages_have_unique_typed_dischargers,
        all_lawful_stops_sealed_without_improvised_selection,
        non_enacted_branch_count,
        non_enacted_branches_consumed_enacted_outcomes,
        value_or_bar_used_as_selector,
        divergence_suppression_or_repair_performed,
        class_representative_substitution_used,
        f_s4_3_preserved,
        f_bi1_preserved,
        f_bi2_preserved,
        f_bi3_preserved,
        f_bi4_preserved,
        f_bi5_preserved,
        f_bi6_preserved,
        cross_branch_g2_verdict_issued: false,
        cross_branch_g3_verdict_issued: false,
        bi2_finale_issued: false,
        bi4_cone_report_issued: false,
        unindexed_halt_ledger_or_o_claim_issued: false,
        permitted_conclusion: "Four Option-A branch-indexed BI-1 continuations were independently issued after an enacted-first F-BI5 regression. This manifest issues no cross-branch G2/G3 verdict, BI-2 finale, BI-4 cone report, or unindexed halt, ledger, O, or F1 claim."
            .to_owned(),
        result_digest: String::new(),
    };
    let manifest_gate = manifest.enacted_branch_issued_first
        && manifest.enacted_regression_passed_before_non_enacted_issuance
        && manifest.exact_four_root_surface
        && manifest.all_branch_certificates_independently_replayed
        && manifest.all_lawful_stops_sealed_without_improvised_selection
        && manifest.non_enacted_branch_count == 3
        && manifest.f_s4_3_preserved
        && manifest.f_bi1_preserved
        && manifest.f_bi2_preserved
        && manifest.f_bi3_preserved
        && manifest.f_bi4_preserved
        && manifest.f_bi5_preserved
        && manifest.f_bi6_preserved
        && !manifest.divergence_suppression_or_repair_performed
        && !manifest.class_representative_substitution_used
        && !manifest.cross_branch_g2_verdict_issued
        && !manifest.cross_branch_g3_verdict_issued
        && !manifest.bi2_finale_issued
        && !manifest.bi4_cone_report_issued
        && !manifest.unindexed_halt_ledger_or_o_claim_issued;
    if !manifest_gate {
        return Err(Bi1SweepV3Error::Invariant(
            "BI-1-only sweep failed its exact four-root/F-BI boundary".to_owned(),
        ));
    }
    manifest.result_digest = manifest_digest(&manifest);
    Ok(Bi1OptionASweepBundleV3 { branches, manifest })
}

fn replay_bi1_branch_certificate_against(
    cone: &CertifiedStage4BranchCone,
    capability: &OptionACapabilityViewV3,
    enacted_regression: Option<&Bi1EnactedRegressionAuthorizationV3>,
    claimed: &Bi1BranchCertificateV3,
) -> Bi1ReplayV3 {
    let mut errors = Vec::new();
    if claimed.result_digest != branch_digest(claimed) {
        errors.push("branch certificate digest mismatch".to_owned());
    }
    let is_indexical_enacted_root = claimed.branch_root_hash == capability.enacted_root_hash;
    if claimed.enacted_root != is_indexical_enacted_root {
        errors.push("branch enacted-role flag differs from the sealed indexical root".to_owned());
    }
    let authorization = if claimed.enacted_root {
        None
    } else {
        match enacted_regression {
            Some(regression)
                if regression.enacted_regression_passed
                    && regression.derivation_hash == regression_digest(regression) =>
            {
                Some(regression.derivation_hash.clone())
            }
            _ => {
                errors.push("non-enacted replay lacks a valid enacted regression".to_owned());
                None
            }
        }
    };
    match issue_branch_certificate_from_cone(
        cone,
        &capability.digest,
        &claimed.branch_root_hash,
        claimed.enacted_root,
        claimed.issuance_ordinal,
        authorization,
    ) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("branch certificate differs from deterministic reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    Bi1ReplayV3 {
        valid: errors.is_empty(),
        errors,
    }
}

/// Reissue one branch.  A non-enacted replay first reissues the enacted
/// branch and its post-seal T-BI regression; no caller-supplied enacted
/// outcomes are accepted.
pub fn replay_bi1_branch_certificate_v3(claimed: &Bi1BranchCertificateV3) -> Bi1ReplayV3 {
    let capability = match issue_option_a_capability_view() {
        Ok(value) => value,
        Err(error) => {
            return Bi1ReplayV3 {
                valid: false,
                errors: vec![error.to_string()],
            };
        }
    };
    let cone = match issue_replayed_cone(&capability) {
        Ok(value) => value,
        Err(error) => {
            return Bi1ReplayV3 {
                valid: false,
                errors: vec![error.to_string()],
            };
        }
    };
    if claimed.enacted_root {
        return replay_bi1_branch_certificate_against(&cone, &capability, None, claimed);
    }
    let enacted = match issue_branch_certificate_from_cone(
        &cone,
        &capability.digest,
        &capability.enacted_root_hash,
        true,
        0,
        None,
    ) {
        Ok(value) => value,
        Err(error) => {
            return Bi1ReplayV3 {
                valid: false,
                errors: vec![error.to_string()],
            };
        }
    };
    let regression = match issue_enacted_regression(&capability, &enacted) {
        Ok(value) => value,
        Err(error) => {
            return Bi1ReplayV3 {
                valid: false,
                errors: vec![error.to_string()],
            };
        }
    };
    replay_bi1_branch_certificate_against(&cone, &capability, Some(&regression), claimed)
}

pub fn replay_bi1_option_a_sweep_v3(claimed: &Bi1OptionASweepBundleV3) -> Bi1ReplayV3 {
    let mut errors = Vec::new();
    if claimed.manifest.result_digest != manifest_digest(&claimed.manifest) {
        errors.push("BI-1 sweep manifest digest mismatch".to_owned());
    }
    // Deterministic sweep reissuance below independently executes and core-
    // replays each of the four expected branches exactly once.  Replaying
    // each claimed branch first would add seven redundant branch issuances
    // without strengthening the final exact bundle comparison.
    match issue_bi1_option_a_sweep_v3() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("BI-1 sweep differs from deterministic reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    Bi1ReplayV3 {
        valid: errors.is_empty(),
        errors,
    }
}

pub fn render_bi1_branch_v3(certificate: &Bi1BranchCertificateV3) -> String {
    let stages = certificate
        .guarded_stages
        .iter()
        .map(|row| {
            format!(
                "| {} | `{}` | {} | `{}` | {} |",
                row.stage,
                row.winner_hash.as_deref().unwrap_or("none"),
                row.discharger_count,
                row.selection_disposition,
                row.no_improvised_selection,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# BI-1 Option-A branch {}\n\n**Date:** {}. **Certificate:** `{}`. **Scope:** branch-indexed BI-1 only.\n\nRoot `{}` was issued at ordinal **{}** (enacted: **{}**). Its continuation outcome is `{:?}`. Every reached guarded stage has one typed total discharger: **{}**. Every lawful stop was sealed without improvised selection: **{}**. Semantic value was never a selector: **{}**. The bar was never a selector: **{}**.\n\n| Stage | Winner | Dischargers | Disposition | No improvised selection |\n|---:|---|---:|---|---|\n{}\n\nNo BI-2 finale, BI-4 comparison, or unindexed halt/O claim is issued.\n",
        certificate.branch_digest_prefix,
        certificate.date,
        certificate.result_digest,
        certificate.branch_root_hash,
        certificate.issuance_ordinal,
        certificate.enacted_root,
        certificate.continuation.outcome,
        certificate.every_reached_guarded_stage_has_unique_typed_discharger,
        certificate.lawful_stop_sealed_without_improvised_selection,
        certificate.semantic_value_never_used_as_selector,
        certificate.bar_never_used_as_selector,
        stages,
    )
}

pub fn render_bi1_option_a_sweep_v3(manifest: &Bi1OptionASweepManifestV3) -> String {
    let rows = manifest
        .branch_rows
        .iter()
        .map(|row| {
            format!(
                "| {} | `{}` | {} | `{}` | {} |",
                row.issuance_ordinal,
                row.branch_root_hash,
                row.enacted_root,
                row.certificate_digest,
                row.every_reached_guarded_stage_has_unique_typed_discharger,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# BI-1 Option-A sweep v3\n\n**Date:** {}. **Certificate:** `{}`.\n\nThe enacted branch was issued and replayed before the T-BI v6 comparison: **{}**. Its exact candidate-hash vector and semantic-nu vector regressed: **{}/{}**. Only then were the other three branches issued.\n\n| Ordinal | Root | Enacted | Certificate | Unique typed dischargers |\n|---:|---|---|---|---|\n{}\n\nFour-root surface exact: **{}**. All lawful stops sealed without improvised selection: **{}**. Non-enacted branches consumed enacted outcomes: **{}**. Value or bar used as selector: **{}**. F-S4-3 and F-BI1..6: **{}/{}/{}/{}/{}/{}/{}**.\n\nNo cross-branch G2/G3 verdict, BI-2 finale, BI-4 cone report, or unindexed halt, ledger, O, or F1 claim is issued.\n\n{}\n",
        manifest.date,
        manifest.result_digest,
        manifest
            .enacted_regression
            .enacted_certificate_replayed_before_comparison,
        manifest.enacted_regression.exact_candidate_hash_vector,
        manifest.enacted_regression.exact_semantic_nu_vector,
        rows,
        manifest.exact_four_root_surface,
        manifest.all_lawful_stops_sealed_without_improvised_selection,
        manifest.non_enacted_branches_consumed_enacted_outcomes,
        manifest.value_or_bar_used_as_selector,
        manifest.f_s4_3_preserved,
        manifest.f_bi1_preserved,
        manifest.f_bi2_preserved,
        manifest.f_bi3_preserved,
        manifest.f_bi4_preserved,
        manifest.f_bi5_preserved,
        manifest.f_bi6_preserved,
        manifest.permitted_conclusion,
    )
}

fn create_new(path: &Path, contents: &[u8]) -> Result<(), Bi1SweepV3Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Bi1SweepV3Error::Io(format!("{}: {error}", path.display())))?;
    file.write_all(contents)
        .map_err(|error| Bi1SweepV3Error::Io(format!("{}: {error}", path.display())))
}

pub fn emit_bi1_option_a_sweep_v3_create_new(
    directory: &Path,
) -> Result<Bi1OptionASweepBundleV3, Bi1SweepV3Error> {
    let bundle = issue_bi1_option_a_sweep_v3()?;
    let replay = replay_bi1_option_a_sweep_v3(&bundle);
    if !replay.valid {
        return Err(Bi1SweepV3Error::Invariant(format!(
            "create-new bundle failed replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut targets = Vec::<(PathBuf, Vec<u8>)>::new();
    for branch in &bundle.branches {
        let json = serde_json::to_vec_pretty(branch)
            .map_err(|error| Bi1SweepV3Error::Json(error.to_string()))?;
        targets.push((
            directory.join(branch_json_name(&branch.branch_digest_prefix)),
            json,
        ));
        targets.push((
            directory.join(branch_report_name(&branch.branch_digest_prefix)),
            render_bi1_branch_v3(branch).into_bytes(),
        ));
    }
    targets.push((
        directory.join(BI1_SWEEP_CERTIFICATE_NAME),
        serde_json::to_vec_pretty(&bundle.manifest)
            .map_err(|error| Bi1SweepV3Error::Json(error.to_string()))?,
    ));
    targets.push((
        directory.join(BI1_SWEEP_REPORT_NAME),
        render_bi1_option_a_sweep_v3(&bundle.manifest).into_bytes(),
    ));
    if let Some(path) = targets
        .iter()
        .map(|(path, _)| path)
        .find(|path| path.exists())
    {
        return Err(Bi1SweepV3Error::Io(format!(
            "create-new target already exists: {}",
            path.display()
        )));
    }
    for (path, contents) in targets {
        create_new(&path, &contents)?;
    }
    Ok(bundle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn bundle() -> &'static Bi1OptionASweepBundleV3 {
        static BUNDLE: OnceLock<Bi1OptionASweepBundleV3> = OnceLock::new();
        BUNDLE.get_or_init(|| issue_bi1_option_a_sweep_v3().expect("BI-1 Option-A sweep"))
    }

    fn rehash_branch(branch: &mut Bi1BranchCertificateV3) {
        branch.result_digest = branch_digest(branch);
    }

    #[test]
    fn enacted_first_four_root_sweep_stops_before_bi2_and_bi4() {
        let bundle = bundle();
        assert_eq!(bundle.branches.len(), 4);
        assert!(bundle.branches[0].enacted_root);
        assert_eq!(bundle.branches[0].issuance_ordinal, 0);
        assert!(bundle.manifest.enacted_regression.enacted_regression_passed);
        assert!(bundle.manifest.exact_four_root_surface);
        assert!(
            bundle
                .manifest
                .all_lawful_stops_sealed_without_improvised_selection
        );
        assert!(
            !bundle
                .manifest
                .non_enacted_branches_consumed_enacted_outcomes
        );
        assert!(!bundle.manifest.value_or_bar_used_as_selector);
        assert!(!bundle.manifest.cross_branch_g2_verdict_issued);
        assert!(!bundle.manifest.cross_branch_g3_verdict_issued);
        assert!(!bundle.manifest.bi2_finale_issued);
        assert!(!bundle.manifest.bi4_cone_report_issued);
        assert!(!bundle.manifest.unindexed_halt_ledger_or_o_claim_issued);
    }

    #[test]
    fn branch_root_winner_nu_and_verdict_mutations_fail_reissuance() {
        let original = &bundle().branches[0];

        let mut root = original.clone();
        root.branch_root_hash.push_str("-forged");
        rehash_branch(&mut root);
        assert!(!replay_bi1_branch_certificate_v3(&root).valid);

        let mut winner = original.clone();
        winner.continuation.stages[0]
            .winner
            .as_mut()
            .expect("reached winner")
            .candidate_hash
            .push_str("-forged");
        rehash_branch(&mut winner);
        assert!(!replay_bi1_branch_certificate_v3(&winner).valid);

        let mut nu = original.clone();
        nu.semantic_nu_vector[0] = nu.semantic_nu_vector[0].saturating_add(1);
        rehash_branch(&mut nu);
        assert!(!replay_bi1_branch_certificate_v3(&nu).valid);

        let mut verdict = original.clone();
        verdict.every_reached_guarded_stage_has_unique_typed_discharger = false;
        rehash_branch(&mut verdict);
        assert!(!replay_bi1_branch_certificate_v3(&verdict).valid);
    }

    #[test]
    fn manifest_mutation_fails_full_deterministic_replay() {
        let mut forged = bundle().clone();
        forged.manifest.cross_branch_g3_verdict_issued = true;
        forged.manifest.result_digest = manifest_digest(&forged.manifest);
        assert!(!replay_bi1_option_a_sweep_v3(&forged).valid);
    }
}
