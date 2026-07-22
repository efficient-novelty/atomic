//! Branch-parametric continuation kernel for the adopted Stage-4 cone.
//!
//! This module deliberately does not modify or wrap the frozen Phase-5b v3
//! source. Its input is one member of the independently certified Stage-4
//! cone, not an enacted continuation, expected winner, score vector, or bar.
//! Starting at Stage 5 it rebuilds the library and every candidate cone from
//! that branch. At a guarded stage a candidate is a total discharger only if
//! candidate-local provenance is ExactCertified and each live A3 structural-
//! completion instance registers and realizes against the candidate with
//! deterministic replay. The diagnostic bar is computed and recorded but is
//! never read by selection.

use crate::act_local_provenance::{
    ActLocalNuProvenanceCertificate, T_BI_NU1_THEOREM_ID, issue_act_local_provenance,
    issue_act_local_sequence,
};
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::naturality_orbit_transport::{
    issue_stage4_r_t1_orbit_audit, replay_stage4_r_t1_orbit_audit,
};
use crate::r_t2_future_hole_confluence_v2::{
    Rt2FutureHoleConfluenceV2Certificate, Rt2FutureHoleConfluenceV2Outcome,
    replay_archived_stage4_fork_projection,
};
use pen_core::canonical::canonical_key_telescope;
use pen_core::encode::telescope_bit_cost;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3HistoricalWindow, A3RuleConstructor, A3TypedDemandInstance,
    A3TypedDemandScheme, generate_a3_window_for_exact_prefix_unbounded,
};
use pen_eval::a3_rule_inventory_exhaustiveness::{
    A3WindowRuleInventoryProof, prove_a3_window_inventory_for_exact_prefix_unbounded,
};
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_eval::debt_guard::required_packages_for;
use pen_eval::future_hole_hypothesis_v2 as future_v2;
use pen_eval::nu::structural_nu;
use pen_schema::context::{BinderId, Declaration, TermExpr, TypeExpr, form_schema_context};
use pen_schema::e4_generator_basis::{
    issue_parent_cube_action_generated, replay_parent_cube_action_generated,
};
use pen_schema::grammar::{
    DerivationRef, RegisteredBoundaryBundleRef, SchemaBinderId, SchemaDimBinderId,
    SchemaLocalDeclaration, SchemaTermExpr, SchemaTypeExpr, SupportWindow,
};
use pen_type::admissibility::{
    AdmissibilityMode, passes_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::elaborate::{
    KernelTy, SealedSignature, TelescopeElaboration, candidate_hash, elaborate_telescope,
};
use pen_type::obligations::summarize_structural_debt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const BRANCH_INVARIANCE_CORE_SCHEMA: &str = "branch-invariance-continuation-core-v2";
pub const BRANCH_INVARIANCE_CORE_DATE: &str = "2026-07-22";
pub const BRANCH_FIRST_SELECTION_STAGE: u32 = 5;
pub const BRANCH_WINDOW_DEPTH: u16 = 2;
pub const BRANCH_NU_PROVENANCE_SCHEMA: &str = "branch-local-nu-provenance-v2";
pub const BRANCH_NU_DECOMPOSITION_THEOREM_ID: &str =
    "T-BI-NU1-candidate-local-structural-nu-family-orbit-decomposition";
pub const BRANCH_NU_DECOMPOSITION_GAP: &str = "No adopted theorem currently bijects the summands of structural_nu with candidate-local typed, normalized, natural, marginal family IDs or independently exported live A3 demand-output positions. The missing theorem must also join every R2 generated-instance subtraction injectively to an actually counted family ID and its parent family.";
pub const BRANCH_ACT_LOCAL_PROVENANCE_ISSUANCE_GAP_PREFIX: &str =
    "BI_ACT_LOCAL_PROVENANCE_ISSUANCE_GAP: ";

const DEMAND_MECHANISM_COUNT: u32 = 10;
const R_T2_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json");
const R_T3_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/r_t3_stage4_adjudication.md");
const R2_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_quotient_adjudications.md");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchContinuationLimits {
    /// Law-independent safety ceiling: the runner may inspect demand at this
    /// stage, but reaching it with live debt is a resource stop, never a halt.
    pub max_inspected_stage: u32,
    /// Law-independent memory/time ceiling checked before candidate analysis.
    pub max_enumerated_candidates_per_stage: usize,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BRANCH_INVARIANCE_CORE_SCHEMA, domain, value))
        .expect("branch-invariance evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn nu_provenance_digest(token: &BranchNuProvenanceToken) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("branch-nu-provenance-token", &projection)
}

fn diagnostic_nu_provenance_token(
    stage: u32,
    candidate_hash: String,
    predecessor_signature_digest: String,
    structural_formula_total: u32,
    r2_generated_instance_adjustment: i32,
) -> BranchNuProvenanceToken {
    let diagnostic_nu = if r2_generated_instance_adjustment < 0 {
        structural_formula_total.saturating_sub(r2_generated_instance_adjustment.unsigned_abs())
    } else {
        structural_formula_total.saturating_add(r2_generated_instance_adjustment as u32)
    };
    let mut token = BranchNuProvenanceToken {
        schema: BRANCH_NU_PROVENANCE_SCHEMA.to_owned(),
        stage,
        candidate_hash,
        predecessor_signature_digest,
        structural_formula_total,
        r2_generated_instance_adjustment,
        diagnostic_nu,
        structural_formula_replayed_from_branch_prefix: true,
        historical_score_vector_or_bar_used_as_input: false,
        disposition: BranchNuProvenanceDisposition::DiagnosticFormulaOnly {
            missing_theorem_id: BRANCH_NU_DECOMPOSITION_THEOREM_ID.to_owned(),
            exact_gap: BRANCH_NU_DECOMPOSITION_GAP.to_owned(),
        },
        derivation_hash: String::new(),
    };
    token.derivation_hash = nu_provenance_digest(&token);
    token
}

fn act_local_provenance_gap_token(
    stage: u32,
    candidate_hash: String,
    predecessor_signature_digest: String,
    structural_formula_total: u32,
    exact_error: &str,
) -> BranchNuProvenanceToken {
    let mut token = diagnostic_nu_provenance_token(
        stage,
        candidate_hash,
        predecessor_signature_digest,
        structural_formula_total,
        0,
    );
    token.disposition = BranchNuProvenanceDisposition::DiagnosticFormulaOnly {
        missing_theorem_id: T_BI_NU1_THEOREM_ID.to_owned(),
        exact_gap: format!("{BRANCH_ACT_LOCAL_PROVENANCE_ISSUANCE_GAP_PREFIX}{exact_error}"),
    };
    token.derivation_hash = nu_provenance_digest(&token);
    token
}

fn exact_nu_provenance_token(package: &ActLocalNuProvenanceCertificate) -> BranchNuProvenanceToken {
    let mut token = BranchNuProvenanceToken {
        schema: BRANCH_NU_PROVENANCE_SCHEMA.to_owned(),
        stage: package.stage,
        candidate_hash: package.candidate_hash.clone(),
        predecessor_signature_digest: package.predecessor_signature_digest.clone(),
        structural_formula_total: package.structural_formula_total,
        r2_generated_instance_adjustment: package.generated_instance_adjustment,
        diagnostic_nu: package.exact_certified_nu,
        structural_formula_replayed_from_branch_prefix: true,
        historical_score_vector_or_bar_used_as_input: false,
        disposition: BranchNuProvenanceDisposition::ExactCertified {
            theorem_id: T_BI_NU1_THEOREM_ID.to_owned(),
            certificate_hash: package.derivation_hash.clone(),
            counted_family_ids: package.counted_family_ids(),
            authoritative_family_token_hashes: package.authoritative_token_hashes(),
            generated_instance_family_ids_removed_by_quotient: package
                .generated_instance_family_ids_removed_by_quotient
                .clone(),
        },
        derivation_hash: String::new(),
    };
    token.derivation_hash = nu_provenance_digest(&token);
    token
}

/// Replay the adopted branch-level projection of T-BI-NU1. Full semantic
/// replay occurs from the exact act and prefix in continuation replay; this
/// local check rejects malformed projections even after outer re-signing.
pub fn replay_branch_nu_provenance_token(token: &BranchNuProvenanceToken) -> Vec<String> {
    let mut errors = Vec::new();
    if token.derivation_hash != nu_provenance_digest(token) {
        errors.push("branch nu-provenance token digest mismatch".to_owned());
    }
    if token.schema != BRANCH_NU_PROVENANCE_SCHEMA
        || token.candidate_hash.is_empty()
        || token.predecessor_signature_digest.is_empty()
        || !token.structural_formula_replayed_from_branch_prefix
        || token.historical_score_vector_or_bar_used_as_input
    {
        errors.push("branch nu-provenance token has an invalid replay surface".to_owned());
    }
    let expected_diagnostic = if token.r2_generated_instance_adjustment < 0 {
        token
            .structural_formula_total
            .saturating_sub(token.r2_generated_instance_adjustment.unsigned_abs())
    } else {
        token
            .structural_formula_total
            .saturating_add(token.r2_generated_instance_adjustment as u32)
    };
    if token.diagnostic_nu != expected_diagnostic {
        errors.push("branch nu-provenance diagnostic arithmetic mismatch".to_owned());
    }
    match &token.disposition {
        BranchNuProvenanceDisposition::DiagnosticFormulaOnly {
            missing_theorem_id,
            exact_gap,
        } if (missing_theorem_id == BRANCH_NU_DECOMPOSITION_THEOREM_ID
            && exact_gap == BRANCH_NU_DECOMPOSITION_GAP)
            || (missing_theorem_id == T_BI_NU1_THEOREM_ID
                && exact_gap
                    .strip_prefix(BRANCH_ACT_LOCAL_PROVENANCE_ISSUANCE_GAP_PREFIX)
                    .is_some_and(|error| !error.is_empty())) => {}
        BranchNuProvenanceDisposition::ExactCertified {
            theorem_id,
            certificate_hash,
            counted_family_ids,
            authoritative_family_token_hashes,
            generated_instance_family_ids_removed_by_quotient,
        } => {
            let token_hashes = authoritative_family_token_hashes
                .iter()
                .collect::<BTreeSet<_>>();
            let removed_ids = generated_instance_family_ids_removed_by_quotient
                .iter()
                .collect::<BTreeSet<_>>();
            let removed_count = token
                .r2_generated_instance_adjustment
                .checked_neg()
                .and_then(|count| usize::try_from(count).ok());
            if theorem_id != T_BI_NU1_THEOREM_ID
                || certificate_hash.is_empty()
                || counted_family_ids.len() != token.diagnostic_nu as usize
                || authoritative_family_token_hashes.len() != token.diagnostic_nu as usize
                || token_hashes.len() != authoritative_family_token_hashes.len()
                || removed_count != Some(generated_instance_family_ids_removed_by_quotient.len())
                || removed_ids.len() != generated_instance_family_ids_removed_by_quotient.len()
            {
                errors.push("exact T-BI-NU1 projection is malformed".to_owned());
            }
        }
        _ => errors.push("branch nu-provenance gap statement drifted".to_owned()),
    }
    errors
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedStage4BranchSeed {
    pub candidate_hash: String,
    pub telescope: Telescope,
    pub r_t1_class_id: String,
    /// The two-class arity-3 key from R-T2. It is scheduling geometry only;
    /// membership in this class is never used as a continuation transport.
    pub economy_probe_class_key: String,
    pub prefix_signature_digest: String,
    pub certified_kappa: u16,
    pub certified_nu: u32,
    pub seed_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedStage4BranchCone {
    pub schema: String,
    pub date: String,
    pub r_t3_option_b_replayed: bool,
    pub r_t2_certificate_digest: String,
    pub r_t2_replay_valid: bool,
    pub naturality_transport_digest: String,
    pub naturality_transport_replay_valid: bool,
    pub minimum_kappa: u16,
    pub minimum_certified_nu: u32,
    /// The common pre-fork stem, projected from the sealed signature.  No
    /// post-fork enacted entry is exposed to a branch runner.
    pub common_stem: Vec<(u32, Telescope)>,
    pub common_stem_signature_digest: String,
    pub branches: Vec<CertifiedStage4BranchSeed>,
    pub branch_count: usize,
    pub economy_probe_class_count: usize,
    pub no_branch_selected: bool,
    pub ordering_used_only_for_serialization: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchLedgerRow {
    pub stage: u32,
    pub candidate_hash: String,
    pub kappa: u16,
    /// The legacy structural formula, retained as a diagnostic only until
    /// `T-BI-NU1` supplies the missing family/orbit decomposition theorem.
    pub semantic_nu: u32,
    pub nu_provenance: BranchNuProvenanceToken,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum BranchNuProvenanceDisposition {
    ExactCertified {
        theorem_id: String,
        certificate_hash: String,
        counted_family_ids: Vec<String>,
        authoritative_family_token_hashes: Vec<String>,
        generated_instance_family_ids_removed_by_quotient: Vec<String>,
    },
    DiagnosticFormulaOnly {
        missing_theorem_id: String,
        exact_gap: String,
    },
}

impl BranchNuProvenanceDisposition {
    pub fn is_exact_certified(&self) -> bool {
        matches!(self, Self::ExactCertified { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchNuProvenanceToken {
    pub schema: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub structural_formula_total: u32,
    pub r2_generated_instance_adjustment: i32,
    pub diagnostic_nu: u32,
    pub structural_formula_replayed_from_branch_prefix: bool,
    pub historical_score_vector_or_bar_used_as_input: bool,
    pub disposition: BranchNuProvenanceDisposition,
    pub derivation_hash: String,
}

impl BranchNuProvenanceToken {
    pub fn is_exact_certified(&self) -> bool {
        self.disposition.is_exact_certified()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchDemandAudit {
    pub stage: u32,
    pub prefix_signature_digest: String,
    pub coarse_required_packages: Vec<String>,
    pub a3_required_packages: Vec<String>,
    pub structural_constructors: Vec<String>,
    pub structural_scheme_ids: Vec<String>,
    pub structural_instance_ids: Vec<String>,
    pub a3_window_derivation_hash: String,
    pub a3_inventory_derivation_hash: String,
    pub exact_prefix_inventory_exhaustive: bool,
    pub coarse_and_a3_demands_agree: bool,
    pub debt_free: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchStructuralDischargeEvidence {
    pub constructor: String,
    pub a3_scheme_id: String,
    pub a3_instance_id: String,
    pub registration_disposition: String,
    pub registration_formation_hash: Option<String>,
    pub registration_replay_valid: bool,
    pub realization_disposition: String,
    pub realization_hash: Option<String>,
    pub realization_replay_valid: bool,
    pub provider_relation_satisfied: bool,
    pub constructor_absent_after_filler: bool,
    pub specialized_expression_is_filler_reference: bool,
    pub zero_marginal_discharge_charge: bool,
    pub typed_total_discharge_replayed: bool,
    pub named_gap_id: Option<String>,
    pub named_gap_phase: Option<String>,
    pub named_gap_error: Option<String>,
    pub semantic_evidence_gap: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchR2GeneratedActionEvidence {
    pub path_clause_index: u16,
    pub parent_operation_clause_index: u16,
    pub path_dimension: u32,
    pub candidate_elaboration_hash: String,
    pub path_clause_is_typed_path: bool,
    pub parent_operation_clause_is_typed_introduction: bool,
    pub canonical_operation_slot: bool,
    pub candidate_bound_registered_cube_hash: String,
    pub generic_parent_cube_action_hash: String,
    pub generic_parent_cube_action_replay_valid: bool,
    pub adopted_r2_rule_source_hash: String,
    pub count_or_bar_used: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchCandidateAssessment {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub telescope: Telescope,
    pub kappa: u16,
    pub bit_kappa: u16,
    pub kernel_typed: bool,
    pub kernel_failure: Option<String>,
    pub elaboration_derivation_hash: String,
    pub identified_with_prefix: bool,
    pub structural_formula_total: u32,
    pub r2_generated_instance_adjustment: i32,
    pub r2_generated_action_evidence: Vec<BranchR2GeneratedActionEvidence>,
    pub r2_applied_after_unique_discharge: bool,
    pub semantic_nu: u32,
    pub local_role_capacity: u32,
    pub live_demand_output_capacity: u32,
    /// This is only the replay/bound status of the diagnostic structural
    /// formula. It is deliberately not an exact semantic-nu certificate.
    pub diagnostic_formula_replayed_and_capacity_bound_satisfied: bool,
    pub nu_provenance: BranchNuProvenanceToken,
    pub ordinary_charge_provenance_hash: String,
    pub structural_discharge_evidence: Vec<BranchStructuralDischargeEvidence>,
    pub every_live_structural_demand_realized: bool,
    pub semantic_evidence_gap: bool,
    pub guarded_total_discharger: bool,
    pub rho: String,
    pub diagnostic_clears_bar: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchWinner {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub telescope: Telescope,
    pub kappa: u16,
    pub semantic_nu: u32,
    pub nu_provenance: BranchNuProvenanceToken,
    pub selected_by: String,
    pub diagnostic_clears_bar: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchStageRecord {
    pub stage: u32,
    pub demand: BranchDemandAudit,
    pub diagnostic_bar: String,
    pub bar_used_as_gate_or_selector: bool,
    pub score_used_as_selector: bool,
    pub cone_enumerated: usize,
    pub cone_admitted: usize,
    pub cone_deduped: usize,
    pub cone_digest: String,
    pub assessments: Vec<BranchCandidateAssessment>,
    pub discharger_hashes: Vec<String>,
    pub discharger_count: usize,
    pub diagnostic_bar_clearing_count: usize,
    pub r2_rule_evidence_hash: Option<String>,
    pub r2_applied_only_after_unique_discharge: bool,
    pub winner: Option<BranchWinner>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum BranchContinuationOutcome {
    DebtFreeHalt {
        halt_stage: u32,
        next_stage: u32,
    },
    HaltedNoGuardedDischarger {
        stage: u32,
    },
    HaltedMultipleGuardedDischargers {
        stage: u32,
        count: usize,
    },
    ExpressivityGap {
        stage: u32,
        exact_error: String,
    },
    ResourceLimitReached {
        stage: u32,
        limit_kind: String,
        configured_limit: u64,
        observed: u64,
        required_packages: Vec<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchContinuation {
    pub schema: String,
    pub date: String,
    pub branch: CertifiedStage4BranchSeed,
    pub limits: BranchContinuationLimits,
    pub initial_prefix_signature_digest: String,
    pub initial_prefix_ledger: Vec<BranchLedgerRow>,
    pub stages: Vec<BranchStageRecord>,
    pub terminal_demand: BranchDemandAudit,
    pub outcome: BranchContinuationOutcome,
    pub complete_ledger: Vec<BranchLedgerRow>,
    pub completed_through_stage15: bool,
    pub debt_free_halt_at_stage15: bool,
    pub every_reached_guarded_stage_unique: bool,
    pub bar_never_used_as_gate_or_selector: bool,
    pub score_never_used_as_selector: bool,
    pub expected_winner_score_ledger_or_bar_accepted_as_runner_input: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub resource_limit_used_as_halt_claim: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BranchInvarianceError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("branch-invariance invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
}

struct DemandContext {
    window: A3HistoricalWindow,
    proof: A3WindowRuleInventoryProof,
    structural_pairs: Vec<(A3TypedDemandScheme, A3TypedDemandInstance, String)>,
    audit: BranchDemandAudit,
}

fn cone_digest(candidates: &[Telescope]) -> String {
    tagged_hash(
        "complete-canonical-cone",
        &candidates.iter().map(candidate_hash).collect::<Vec<_>>(),
    )
}

fn result_digest(result: &BranchContinuation) -> String {
    let mut projection = result.clone();
    projection.derivation_hash.clear();
    tagged_hash("branch-continuation", &projection)
}

fn cone_result_digest(cone: &CertifiedStage4BranchCone) -> String {
    let mut projection = cone.clone();
    projection.derivation_hash.clear();
    tagged_hash("certified-stage4-cone", &projection)
}

fn seed_digest(seed: &CertifiedStage4BranchSeed) -> String {
    let mut projection = seed.clone();
    projection.seed_derivation_hash.clear();
    tagged_hash("stage4-branch-seed", &projection)
}

fn certified_common_stem() -> Result<Vec<(u32, Telescope)>, BranchInvarianceError> {
    let sealed = SealedSignature::genesis_del_h15();
    let entries = (1..=3_u32)
        .map(|stage| {
            sealed
                .entry(stage)
                .map(|entry| (stage, entry.telescope.clone()))
                .ok_or_else(|| {
                    BranchInvarianceError::Prerequisite(format!(
                        "sealed pre-fork signature omits Stage {stage}"
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if entries.iter().map(|(stage, _)| *stage).ne(1..=3) {
        return Err(BranchInvarianceError::Prerequisite(
            "sealed pre-fork signature is not exactly Stages 1–3".to_owned(),
        ));
    }
    Ok(entries)
}

fn exact_stage4_prefix(stem: &[(u32, Telescope)], telescope: &Telescope) -> SealedSignature {
    let mut entries = stem.to_vec();
    entries.push((4, telescope.clone()));
    SealedSignature::from_telescopes(entries)
}

/// Rebuild the four branch seeds from the two independently replayed cone
/// certificates. Sorting makes serialization deterministic; no member is
/// selected and the order is unavailable to the continuation selector.
pub fn issue_certified_stage4_branch_cone()
-> Result<CertifiedStage4BranchCone, BranchInvarianceError> {
    let adjudication = std::str::from_utf8(R_T3_ADJUDICATION_BYTES)
        .map_err(|error| BranchInvarianceError::Prerequisite(error.to_string()))?;
    let r_t3_option_b_replayed = adjudication.contains("Option B adopted")
        && adjudication.contains("explicit non-determinism")
        && adjudication.contains("F-R3-B1");
    if !r_t3_option_b_replayed {
        return Err(BranchInvarianceError::Prerequisite(
            "R-T3 Option B or F-R3-B1 is absent".to_owned(),
        ));
    }

    let r_t2: Rt2FutureHoleConfluenceV2Certificate = serde_json::from_slice(R_T2_ARTIFACT_BYTES)
        .map_err(|error| BranchInvarianceError::Json(error.to_string()))?;
    let r_t2_replay_errors = replay_archived_stage4_fork_projection(&r_t2);
    let r_t2_replay_valid = r_t2_replay_errors.is_empty();
    if !r_t2_replay_valid
        || r_t2.outcome != Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened
        || !r_t2.r_t2_confluence_refuted
        || r_t2.selected_candidate_hash.is_some()
        || r_t2.hash_or_enumeration_order_used_as_selector
        || r_t2.branch_count != 4
        || r_t2.branches.len() != 4
    {
        return Err(BranchInvarianceError::Prerequisite(format!(
            "authoritative R-T2 cone did not replay as an unselected four-way fork: {:?}",
            r_t2_replay_errors
        )));
    }

    let stage4_transport = issue_stage4_r_t1_orbit_audit()
        .map_err(|error| BranchInvarianceError::Prerequisite(error.to_string()))?;
    let transport_replay_errors = replay_stage4_r_t1_orbit_audit(&stage4_transport);
    let naturality_transport_replay_valid = transport_replay_errors.is_empty();
    if !naturality_transport_replay_valid
        || !stage4_transport.parsimony_minimizer_hashes_joined
        || stage4_transport.minimum_kappa != 3
        || stage4_transport.minimum_certified_nu != 5
        || stage4_transport.minimizer_count != 4
        || stage4_transport.packages.len() != 4
        || !stage4_transport.no_selector_or_presentation_order_used
    {
        return Err(BranchInvarianceError::Prerequisite(format!(
            "R-T1 transport did not replay the exact unselected (3,5) four-way cone: {:?}",
            transport_replay_errors
        )));
    }

    let common_stem = certified_common_stem()?;
    let common_stem_signature_digest = SealedSignature::from_telescopes(common_stem.clone())
        .digest()
        .to_owned();
    let package_by_hash = stage4_transport
        .packages
        .iter()
        .map(|package| (package.candidate_hash.as_str(), package))
        .collect::<BTreeMap<_, _>>();
    let mut branches = Vec::new();
    for branch in &r_t2.branches {
        let package = package_by_hash
            .get(branch.stage4_candidate_hash.as_str())
            .ok_or_else(|| {
                BranchInvarianceError::Invariant(format!(
                    "R-T2 branch {} is absent from R-T1",
                    branch.stage4_candidate_hash
                ))
            })?;
        if package.telescope != branch.stage4_telescope
            || candidate_hash(&branch.stage4_telescope) != branch.stage4_candidate_hash
        {
            return Err(BranchInvarianceError::Invariant(format!(
                "R-T1/R-T2 telescope join failed for {}",
                branch.stage4_candidate_hash
            )));
        }
        let arity_three = branch
            .future_hole_aggregate
            .registrations
            .iter()
            .filter(|row| row.dependent_context_declared_arity == Some(3))
            .collect::<Vec<_>>();
        if arity_three.len() != 1
            || arity_three[0].branch_independent_semantic_key.is_empty()
            || arity_three[0].branch_local_family_id_used_in_semantic_key
        {
            return Err(BranchInvarianceError::Invariant(format!(
                "branch {} does not have its unique branch-independent arity-3 economy key",
                branch.stage4_candidate_hash
            )));
        }
        let signature = exact_stage4_prefix(&common_stem, &branch.stage4_telescope);
        if signature.digest() != branch.prefix_signature_digest {
            return Err(BranchInvarianceError::Invariant(format!(
                "branch {} exact prefix digest drifted",
                branch.stage4_candidate_hash
            )));
        }
        let mut seed = CertifiedStage4BranchSeed {
            candidate_hash: branch.stage4_candidate_hash.clone(),
            telescope: branch.stage4_telescope.clone(),
            r_t1_class_id: branch.r_t1_class_id.clone(),
            economy_probe_class_key: arity_three[0].branch_independent_semantic_key.clone(),
            prefix_signature_digest: branch.prefix_signature_digest.clone(),
            certified_kappa: stage4_transport.minimum_kappa,
            certified_nu: stage4_transport.minimum_certified_nu,
            seed_derivation_hash: String::new(),
        };
        if seed.telescope.kappa() as u16 != seed.certified_kappa {
            return Err(BranchInvarianceError::Invariant(format!(
                "branch {} telescope kappa differs from R-T1",
                seed.candidate_hash
            )));
        }
        seed.seed_derivation_hash = seed_digest(&seed);
        branches.push(seed);
    }
    branches.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    let economy_probe_class_count = branches
        .iter()
        .map(|branch| branch.economy_probe_class_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    if economy_probe_class_count != 2 {
        return Err(BranchInvarianceError::Invariant(format!(
            "R-T2 arity-3 economy geometry has {economy_probe_class_count} classes, expected 2"
        )));
    }
    let mut cone = CertifiedStage4BranchCone {
        schema: BRANCH_INVARIANCE_CORE_SCHEMA.to_owned(),
        date: BRANCH_INVARIANCE_CORE_DATE.to_owned(),
        r_t3_option_b_replayed,
        r_t2_certificate_digest: r_t2.result_digest,
        r_t2_replay_valid,
        naturality_transport_digest: stage4_transport.derivation_hash,
        naturality_transport_replay_valid,
        minimum_kappa: 3,
        minimum_certified_nu: 5,
        common_stem,
        common_stem_signature_digest,
        branch_count: branches.len(),
        branches,
        economy_probe_class_count,
        no_branch_selected: true,
        ordering_used_only_for_serialization: true,
        derivation_hash: String::new(),
    };
    cone.derivation_hash = cone_result_digest(&cone);
    Ok(cone)
}

pub fn replay_certified_stage4_branch_cone(cone: &CertifiedStage4BranchCone) -> Vec<String> {
    let mut errors = Vec::new();
    if cone.derivation_hash != cone_result_digest(cone) {
        errors.push("Stage-4 branch cone digest mismatch".to_owned());
    }
    match issue_certified_stage4_branch_cone() {
        Ok(expected) if &expected == cone => {}
        Ok(_) => errors.push("Stage-4 branch cone differs from independent reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn demand_output_capacity(
    stage: u32,
    winners: &[(u32, Telescope)],
    required_packages: &[String],
) -> u32 {
    if required_packages.is_empty() {
        return 0;
    }
    let source_count = winners
        .iter()
        .filter(|(step, _)| *step + u32::from(BRANCH_WINDOW_DEPTH) >= stage && *step < stage)
        .map(|(_, telescope)| telescope.clauses.len() as u32)
        .sum::<u32>();
    let unary = source_count.saturating_mul(DEMAND_MECHANISM_COUNT);
    let binary = source_count
        .saturating_mul(source_count)
        .saturating_mul(DEMAND_MECHANISM_COUNT);
    unary.saturating_add(binary.saturating_mul(2))
}

fn demand_context(
    signature: &SealedSignature,
    stage: u32,
    coarse_required_packages: Vec<String>,
) -> Result<DemandContext, String> {
    let window = generate_a3_window_for_exact_prefix_unbounded(signature, stage)
        .map_err(|error| error.to_string())?;
    let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(signature, stage)
        .map_err(|error| error.to_string())?;
    if window.stage != stage
        || proof.stage != stage
        || proof.exact_prefix_signature_digest != signature.digest()
        || !proof.relative_rule_inventory_exhaustive_for_window
        || !proof.every_independent_seed_has_exactly_one_disposition
        || !proof.every_operational_instance_has_exactly_one_independent_preimage
        || !proof.every_operational_scheme_and_orbit_is_consumed
    {
        return Err(format!(
            "Stage {stage} exact-prefix A3 inventory theorem did not close"
        ));
    }

    let scheme_by_id = window
        .schemes
        .iter()
        .map(|scheme| (scheme.scheme_id.as_str(), scheme))
        .collect::<BTreeMap<_, _>>();
    let mut structural_pairs = Vec::new();
    for instance in &window.instances {
        let scheme = scheme_by_id
            .get(instance.scheme_id.as_str())
            .ok_or_else(|| {
                format!(
                    "Stage {stage} A3 instance {} lacks its scheme",
                    instance.instance_id
                )
            })?;
        if scheme.rule_constructor != A3RuleConstructor::StructuralCompletionHole {
            continue;
        }
        let A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } = &scheme.origin else {
            return Err(format!(
                "Stage {stage} structural scheme {} has a non-structural origin",
                scheme.scheme_id
            ));
        };
        structural_pairs.push((
            (*scheme).clone(),
            instance.clone(),
            constructor.slug().to_owned(),
        ));
    }
    structural_pairs.sort_by(|left, right| {
        (&left.2, &left.0.scheme_id, &left.1.instance_id).cmp(&(
            &right.2,
            &right.0.scheme_id,
            &right.1.instance_id,
        ))
    });
    let structural_constructors = structural_pairs
        .iter()
        .map(|(_, _, constructor)| constructor.clone())
        .collect::<Vec<_>>();
    let structural_scheme_ids = structural_pairs
        .iter()
        .map(|(scheme, _, _)| scheme.scheme_id.clone())
        .collect::<Vec<_>>();
    let structural_instance_ids = structural_pairs
        .iter()
        .map(|(_, instance, _)| instance.instance_id.clone())
        .collect::<Vec<_>>();
    let mut a3_required_packages = proof.required_packages_from_raw_debt.clone();
    let mut coarse_sorted = coarse_required_packages.clone();
    a3_required_packages.sort();
    coarse_sorted.sort();
    let mut constructor_sorted = structural_constructors.clone();
    constructor_sorted.sort();
    let coarse_and_a3_demands_agree = coarse_sorted == a3_required_packages
        && constructor_sorted == a3_required_packages
        && structural_pairs.len() == window.constructor_evidence.len();
    if !coarse_and_a3_demands_agree {
        return Err(format!(
            "Stage {stage} coarse debt, A3 debt, and structural instances disagree: coarse={coarse_sorted:?}, A3={a3_required_packages:?}, structural={constructor_sorted:?}"
        ));
    }
    let mut audit = BranchDemandAudit {
        stage,
        prefix_signature_digest: signature.digest().to_owned(),
        coarse_required_packages,
        a3_required_packages,
        structural_constructors,
        structural_scheme_ids,
        structural_instance_ids,
        a3_window_derivation_hash: window.window_derivation_hash.clone(),
        a3_inventory_derivation_hash: proof.derivation_hash.clone(),
        exact_prefix_inventory_exhaustive: true,
        coarse_and_a3_demands_agree,
        debt_free: structural_pairs.is_empty(),
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("branch-demand-audit", &audit);
    Ok(DemandContext {
        window,
        proof,
        structural_pairs,
        audit,
    })
}

fn gap_is_ordinary_candidate_rejection(id: &str) -> bool {
    matches!(
        id,
        "a3_v2_wrong_structural_provider" | "a3_v2_structural_discharge_projection_failed"
    )
}

fn gap_summary(
    constructor: &str,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    registration_disposition: &str,
    registration_formation_hash: Option<String>,
    registration_replay_valid: bool,
    gap: &future_v2::NamedFutureHoleGapV2,
    semantic_evidence_gap: bool,
) -> BranchStructuralDischargeEvidence {
    let mut evidence = BranchStructuralDischargeEvidence {
        constructor: constructor.to_owned(),
        a3_scheme_id: scheme.scheme_id.clone(),
        a3_instance_id: instance.instance_id.clone(),
        registration_disposition: registration_disposition.to_owned(),
        registration_formation_hash,
        registration_replay_valid,
        realization_disposition: "gap".to_owned(),
        realization_hash: None,
        realization_replay_valid: gap.replays(),
        provider_relation_satisfied: false,
        constructor_absent_after_filler: false,
        specialized_expression_is_filler_reference: false,
        zero_marginal_discharge_charge: false,
        typed_total_discharge_replayed: false,
        named_gap_id: Some(gap.id.clone()),
        named_gap_phase: Some(format!("{:?}", gap.phase)),
        named_gap_error: Some(gap.exact_error.clone()),
        semantic_evidence_gap,
        derivation_hash: String::new(),
    };
    evidence.derivation_hash = tagged_hash("structural-discharge-evidence", &evidence);
    evidence
}

fn error_summary(
    constructor: &str,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    phase: &str,
    error: impl Into<String>,
) -> BranchStructuralDischargeEvidence {
    let mut evidence = BranchStructuralDischargeEvidence {
        constructor: constructor.to_owned(),
        a3_scheme_id: scheme.scheme_id.clone(),
        a3_instance_id: instance.instance_id.clone(),
        registration_disposition: if phase == "registration" {
            "error".to_owned()
        } else {
            "registered".to_owned()
        },
        registration_formation_hash: None,
        registration_replay_valid: false,
        realization_disposition: "error".to_owned(),
        realization_hash: None,
        realization_replay_valid: false,
        provider_relation_satisfied: false,
        constructor_absent_after_filler: false,
        specialized_expression_is_filler_reference: false,
        zero_marginal_discharge_charge: false,
        typed_total_discharge_replayed: false,
        named_gap_id: Some(format!("branch_core_{phase}_error")),
        named_gap_phase: Some(phase.to_owned()),
        named_gap_error: Some(error.into()),
        semantic_evidence_gap: true,
        derivation_hash: String::new(),
    };
    evidence.derivation_hash = tagged_hash("structural-discharge-evidence", &evidence);
    evidence
}

fn structural_discharge_evidence(
    signature: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    demand: &DemandContext,
    charge: &future_v2::FillerOrdinaryChargeProvenanceV2,
) -> Vec<BranchStructuralDischargeEvidence> {
    let mut evidence = Vec::new();
    for (scheme, instance, constructor) in &demand.structural_pairs {
        let raw = match future_v2::register_structural_future_hole_v2(
            signature,
            &demand.window,
            scheme,
            instance,
        ) {
            Ok(value) => value,
            Err(error) => {
                evidence.push(error_summary(
                    constructor,
                    scheme,
                    instance,
                    "registration",
                    error.to_string(),
                ));
                continue;
            }
        };
        let registered = match raw {
            future_v2::FutureHoleRegistrationDispositionV2::Gap(gap) => {
                let claimed = future_v2::FutureHoleRegistrationDispositionV2::Gap(gap.clone());
                let replay = future_v2::replay_future_hole_registration_v2(
                    signature,
                    &demand.window,
                    scheme,
                    instance,
                    &claimed,
                );
                evidence.push(gap_summary(
                    constructor,
                    scheme,
                    instance,
                    "gap",
                    None,
                    replay.valid,
                    &gap,
                    true,
                ));
                continue;
            }
            future_v2::FutureHoleRegistrationDispositionV2::Registered(value) => value,
        };
        let registered = match future_v2::attach_external_exhaustiveness_evidence_v2(
            &registered,
            demand.proof.derivation_hash.clone(),
        ) {
            Ok(value) => value,
            Err(error) => {
                evidence.push(error_summary(
                    constructor,
                    scheme,
                    instance,
                    "exhaustiveness_join",
                    error.to_string(),
                ));
                continue;
            }
        };
        let claimed_registration =
            future_v2::FutureHoleRegistrationDispositionV2::Registered(registered.clone());
        let registration_replay = future_v2::replay_future_hole_registration_v2(
            signature,
            &demand.window,
            scheme,
            instance,
            &claimed_registration,
        );
        if !registration_replay.valid {
            evidence.push(error_summary(
                constructor,
                scheme,
                instance,
                "registration_replay",
                registration_replay.errors.join("; "),
            ));
            continue;
        }
        let realization = match future_v2::realize_structural_future_hole_v2(
            signature,
            &registered,
            stage,
            candidate,
            charge,
        ) {
            Ok(value) => value,
            Err(error) => {
                evidence.push(error_summary(
                    constructor,
                    scheme,
                    instance,
                    "realization",
                    error.to_string(),
                ));
                continue;
            }
        };
        let realization_replay = future_v2::replay_structural_realization_v2(
            signature,
            &registered,
            stage,
            candidate,
            charge,
            &realization,
        );
        match realization {
            future_v2::FutureHoleRealizationDispositionV2::Gap(gap) => {
                let semantic_gap = !gap_is_ordinary_candidate_rejection(&gap.id);
                evidence.push(gap_summary(
                    constructor,
                    scheme,
                    instance,
                    "registered",
                    Some(registered.formation_hash.clone()),
                    registration_replay.valid,
                    &gap,
                    semantic_gap,
                ));
                if let Some(last) = evidence.last_mut() {
                    last.realization_replay_valid = realization_replay.valid;
                    last.derivation_hash.clear();
                    last.derivation_hash = tagged_hash("structural-discharge-evidence", last);
                }
            }
            future_v2::FutureHoleRealizationDispositionV2::Realized(realized) => {
                let typed_total_discharge_replayed = registration_replay.valid
                    && realization_replay.valid
                    && realized.provider_relation_satisfied
                    && realized.constructor_live_before_filler
                    && realized.constructor_absent_after_filler
                    && realized.prefix_extension_preserved
                    && realized.specialized_expression_is_filler_reference
                    && realized.clause4_prime_instantiation_replayed
                    && realized.discharge_marginal_charge.replays_as_zero();
                let mut row = BranchStructuralDischargeEvidence {
                    constructor: constructor.clone(),
                    a3_scheme_id: scheme.scheme_id.clone(),
                    a3_instance_id: instance.instance_id.clone(),
                    registration_disposition: "registered".to_owned(),
                    registration_formation_hash: Some(registered.formation_hash.clone()),
                    registration_replay_valid: registration_replay.valid,
                    realization_disposition: "realized".to_owned(),
                    realization_hash: Some(realized.realization_hash.clone()),
                    realization_replay_valid: realization_replay.valid,
                    provider_relation_satisfied: realized.provider_relation_satisfied,
                    constructor_absent_after_filler: realized.constructor_absent_after_filler,
                    specialized_expression_is_filler_reference: realized
                        .specialized_expression_is_filler_reference,
                    zero_marginal_discharge_charge: realized
                        .discharge_marginal_charge
                        .replays_as_zero(),
                    typed_total_discharge_replayed,
                    named_gap_id: None,
                    named_gap_phase: None,
                    named_gap_error: None,
                    semantic_evidence_gap: false,
                    derivation_hash: String::new(),
                };
                row.derivation_hash = tagged_hash("structural-discharge-evidence", &row);
                evidence.push(row);
            }
        }
    }
    evidence.sort_by(|left, right| {
        (&left.constructor, &left.a3_instance_id).cmp(&(&right.constructor, &right.a3_instance_id))
    });
    evidence
}

fn candidate_semantic_discharge_status(
    kernel_typed: bool,
    identified_with_prefix: bool,
    every_live_structural_demand_realized: bool,
    structural_semantic_evidence_gap: bool,
    nu_provenance: &BranchNuProvenanceToken,
) -> (bool, bool) {
    let provenance_required = kernel_typed && !identified_with_prefix;
    let exact_provenance_replays = nu_provenance.is_exact_certified()
        && replay_branch_nu_provenance_token(nu_provenance).is_empty();
    let provenance_semantic_evidence_gap = provenance_required && !exact_provenance_replays;
    let semantic_evidence_gap =
        structural_semantic_evidence_gap || provenance_semantic_evidence_gap;
    let guarded_total_discharger = provenance_required
        && exact_provenance_replays
        && every_live_structural_demand_realized
        && !semantic_evidence_gap;
    (semantic_evidence_gap, guarded_total_discharger)
}

#[allow(clippy::too_many_arguments)]
fn assess_candidate(
    stage: u32,
    candidate: &Telescope,
    signature: &SealedSignature,
    library: &Library,
    history: &[(u32, u32)],
    accepted_keys: &BTreeSet<String>,
    bar: Rational,
    demand_capacity: u32,
    demand: &DemandContext,
) -> BranchCandidateAssessment {
    let candidate_digest = candidate_hash(candidate);
    let canonical_key = canonical_key_telescope(candidate).0;
    let kappa = candidate.kappa() as u16;
    let bit_kappa = u16::try_from(telescope_bit_cost(candidate)).expect("bit kappa fits u16");
    let identified_with_prefix = accepted_keys.contains(&canonical_key);
    let (kernel_typed, kernel_failure, elaboration_derivation_hash) =
        match elaborate_telescope(signature, candidate, stage - 1) {
            Ok(value) => (true, None, value.derivation_hash),
            Err(error) => (false, Some(error.to_string()), String::new()),
        };
    let structural_formula_total = structural_nu(candidate, library, history).total;
    // Exact act-local provenance is an admissibility premise, not a score.
    // It is issued for every typed, non-internal candidate before the
    // discharger census. Stage 8 still delays the typed MapCube/R2 action
    // replay until the census is a singleton; only its intrinsic T-BI-NU1
    // projection is available here.
    let (exact_package, provenance_issuance_error) = if kernel_typed && !identified_with_prefix {
        match issue_act_local_provenance(signature, stage, candidate) {
            Ok(package) => (Some(package), None),
            Err(error) => (None, Some(error.to_string())),
        }
    } else {
        (None, None)
    };
    let r2_generated_instance_adjustment = exact_package
        .as_ref()
        .map_or(0, |package| package.generated_instance_adjustment);
    let semantic_nu = if identified_with_prefix {
        0
    } else {
        exact_package
            .as_ref()
            .map_or(structural_formula_total, |package| {
                package.exact_certified_nu
            })
    };
    let local_role_capacity = 4 * u32::from(kappa);
    let diagnostic_formula_replayed_and_capacity_bound_satisfied = kernel_typed
        && !identified_with_prefix
        && semantic_nu <= local_role_capacity.saturating_add(demand_capacity);
    let nu_provenance = match (&exact_package, &provenance_issuance_error) {
        (Some(package), _) => exact_nu_provenance_token(package),
        (None, Some(error)) => act_local_provenance_gap_token(
            stage,
            candidate_digest.clone(),
            signature.digest().to_owned(),
            structural_formula_total,
            error,
        ),
        (None, None) => diagnostic_nu_provenance_token(
            stage,
            candidate_digest.clone(),
            signature.digest().to_owned(),
            structural_formula_total,
            r2_generated_instance_adjustment,
        ),
    };
    let ordinary_family_token_hashes = exact_package.as_ref().map_or_else(
        Vec::new,
        ActLocalNuProvenanceCertificate::authoritative_token_hashes,
    );
    // The realization preserves this already-certified ordinary charge and
    // mints no family credit.
    let charge = future_v2::issue_filler_ordinary_charge_provenance_v2(
        stage,
        candidate,
        u32::from(kappa),
        semantic_nu,
        candidate.bit_cost(),
        nu_provenance.derivation_hash.clone(),
        ordinary_family_token_hashes,
    );
    let exact_provenance_replays = nu_provenance.is_exact_certified()
        && replay_branch_nu_provenance_token(&nu_provenance).is_empty();
    let structural_discharge_evidence =
        if kernel_typed && !identified_with_prefix && exact_provenance_replays {
            structural_discharge_evidence(signature, stage, candidate, demand, &charge)
        } else {
            Vec::new()
        };
    let every_live_structural_demand_realized = !demand.structural_pairs.is_empty()
        && structural_discharge_evidence.len() == demand.structural_pairs.len()
        && structural_discharge_evidence
            .iter()
            .all(|row| row.typed_total_discharge_replayed);
    let structural_semantic_evidence_gap = structural_discharge_evidence
        .iter()
        .any(|row| row.semantic_evidence_gap);
    let (semantic_evidence_gap, guarded_total_discharger) = candidate_semantic_discharge_status(
        kernel_typed,
        identified_with_prefix,
        every_live_structural_demand_realized,
        structural_semantic_evidence_gap,
        &nu_provenance,
    );
    let rho = Rational::new(i64::from(semantic_nu), i64::from(kappa.max(1)));
    let diagnostic_clears_bar = kernel_typed && !identified_with_prefix && rho >= bar;
    let mut assessment = BranchCandidateAssessment {
        candidate_hash: candidate_digest,
        canonical_key,
        telescope: candidate.clone(),
        kappa,
        bit_kappa,
        kernel_typed,
        kernel_failure,
        elaboration_derivation_hash,
        identified_with_prefix,
        structural_formula_total,
        r2_generated_instance_adjustment,
        r2_generated_action_evidence: Vec::new(),
        r2_applied_after_unique_discharge: false,
        semantic_nu,
        local_role_capacity,
        live_demand_output_capacity: demand_capacity,
        diagnostic_formula_replayed_and_capacity_bound_satisfied,
        nu_provenance,
        ordinary_charge_provenance_hash: charge.provenance_hash,
        structural_discharge_evidence,
        every_live_structural_demand_realized,
        semantic_evidence_gap,
        guarded_total_discharger,
        rho: rho.to_string(),
        diagnostic_clears_bar,
        derivation_hash: String::new(),
    };
    assessment.derivation_hash = tagged_hash("branch-candidate-assessment", &assessment);
    assessment
}

fn r2_rule_source_hash() -> Result<String, String> {
    let text = std::str::from_utf8(R2_ADJUDICATION_BYTES).map_err(|error| error.to_string())?;
    if !text.contains("derived-action-generator-membership-rule-v1")
        || !text.contains("**R2 adopted**")
        || !text.contains("position, label, or archived cardinality")
    {
        return Err("adopted count-blind R2 source did not replay".to_owned());
    }
    Ok(format!("blake3:{}", blake3_hex(R2_ADJUDICATION_BYTES)))
}

fn is_hit_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Trunc(_))
        || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ))
}

fn prove_candidate_generated_actions(
    stage: u32,
    candidate: &Telescope,
    library: &Library,
    elaboration: &TelescopeElaboration,
) -> Result<Vec<BranchR2GeneratedActionEvidence>, String> {
    if candidate.classify(library) != TelescopeClass::Hit
        || !candidate
            .clauses
            .iter()
            .any(|clause| is_hit_formation(&clause.expr))
    {
        return Ok(Vec::new());
    }
    let Some(path_index) = candidate
        .clauses
        .iter()
        .position(|clause| matches!(clause.expr, Expr::PathCon(_)))
    else {
        return Ok(Vec::new());
    };
    let path_clause = elaboration
        .clauses
        .get(path_index)
        .ok_or_else(|| "typed candidate omits its first path clause".to_owned())?;
    let KernelTy::PathDecl { dimension } = path_clause.kernel_ty else {
        return Err(format!(
            "BI_R2_CANDIDATE_PATH_TYPING_GAP: clause {path_index} is not a typed path declaration"
        ));
    };
    let source_hash = r2_rule_source_hash()?;
    let post_count = candidate.clauses.len().saturating_sub(path_index + 1);
    let expected_action_count = post_count.div_ceil(2);
    let mut rows = Vec::new();
    for operation_offset in 0..expected_action_count {
        let operation_index = path_index + 1 + 2 * operation_offset;
        let operation_clause = elaboration.clauses.get(operation_index).ok_or_else(|| {
            format!(
                "BI_R2_PARENT_OPERATION_TYPING_GAP: canonical operation slot {operation_index} is absent"
            )
        })?;
        let parent_operation_clause_is_typed_introduction =
            operation_clause.kernel_role == pen_core::clause::ClauseRole::Introduction;
        if !parent_operation_clause_is_typed_introduction {
            return Err(format!(
                "BI_R2_PARENT_OPERATION_TYPING_GAP: canonical operation slot {operation_index} has kernel role {:?}",
                operation_clause.kernel_role
            ));
        }

        // Candidate binding enters through the registered-cube derivation and
        // the typed path/parent clause indices.  The MapCube theorem itself is
        // parametric in the carrier and parent operation; no historical
        // telescope, winner hash, score, count, or bar is an input.
        let carrier = TypeExpr::parameter(0);
        let base = TermExpr::variable(1);
        let context = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(0),
                name: "R2Carrier".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(1),
                name: "r2_base".to_owned(),
                ty: carrier.clone(),
            },
        ])
        .map_err(|error| error.to_string())?;
        let candidate_bound_registered_cube_hash = tagged_hash(
            "candidate-bound-r2-cube",
            &(
                stage,
                &elaboration.subject_hash,
                &elaboration.derivation_hash,
                path_index,
                operation_index,
                dimension,
                &path_clause.derivation,
                &operation_clause.derivation,
            ),
        );
        let bundle = RegisteredBoundaryBundleRef {
            derivation: DerivationRef::parse(candidate_bound_registered_cube_hash.clone())
                .map_err(|error| error.to_string())?,
            source_step: stage,
            carrier: carrier.clone(),
            dimension,
        };
        let element = SchemaTypeExpr::Element {
            carrier: carrier.clone(),
        };
        let path_cell = SchemaBinderId(0);
        let operation = SchemaBinderId(1);
        let pair_argument = SchemaBinderId(2);
        let family_argument = SchemaBinderId(3);
        let translated_point = SchemaBinderId(4);
        let locals = vec![
            SchemaLocalDeclaration {
                binder: path_cell,
                ty: SchemaTypeExpr::Cube {
                    carrier: carrier.clone(),
                    boundary: Box::new(SchemaTermExpr::Ambient { term: base }),
                    dimension,
                    source_bundle: bundle,
                },
            },
            SchemaLocalDeclaration {
                binder: operation,
                ty: SchemaTypeExpr::Pi {
                    binder: pair_argument,
                    domain: Box::new(SchemaTypeExpr::Product {
                        left: Box::new(element.clone()),
                        right: Box::new(element.clone()),
                    }),
                    codomain: Box::new(element.clone()),
                },
            },
            SchemaLocalDeclaration {
                binder: family_argument,
                ty: element.clone(),
            },
        ];
        let parent_function = SchemaTermExpr::Lam {
            binder: translated_point,
            domain: Box::new(element),
            body: Box::new(SchemaTermExpr::App {
                function: Box::new(SchemaTermExpr::Variable { binder: operation }),
                argument: Box::new(SchemaTermExpr::Pair {
                    left: Box::new(SchemaTermExpr::Variable {
                        binder: translated_point,
                    }),
                    right: Box::new(SchemaTermExpr::Variable {
                        binder: family_argument,
                    }),
                }),
            }),
        };
        let registered_cube = SchemaTermExpr::Variable { binder: path_cell };
        let candidate_row = SchemaTermExpr::MapCube {
            function: Box::new(parent_function.clone()),
            cube: Box::new(registered_cube.clone()),
        };
        let coordinate_dimensions = (0..dimension)
            .map(|axis| SchemaDimBinderId(10 + axis))
            .collect::<Vec<_>>();
        let window = SupportWindow::new(stage.saturating_sub(1), stage)
            .map_err(|error| error.to_string())?;
        let generated = issue_parent_cube_action_generated(
            context,
            window,
            locals,
            coordinate_dimensions,
            parent_function,
            registered_cube,
            candidate_row,
        )
        .map_err(|error| error.to_string())?;
        replay_parent_cube_action_generated(&generated).map_err(|error| error.to_string())?;
        let mut row = BranchR2GeneratedActionEvidence {
            path_clause_index: u16::try_from(path_index)
                .map_err(|_| "path clause index exceeds u16".to_owned())?,
            parent_operation_clause_index: u16::try_from(operation_index)
                .map_err(|_| "operation clause index exceeds u16".to_owned())?,
            path_dimension: dimension,
            candidate_elaboration_hash: elaboration.derivation_hash.clone(),
            path_clause_is_typed_path: true,
            parent_operation_clause_is_typed_introduction,
            canonical_operation_slot: true,
            candidate_bound_registered_cube_hash,
            generic_parent_cube_action_hash: generated.derivation_hash().to_owned(),
            generic_parent_cube_action_replay_valid: true,
            adopted_r2_rule_source_hash: source_hash.clone(),
            count_or_bar_used: false,
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("branch-r2-generated-action", &row);
        rows.push(row);
    }
    if rows.len() != expected_action_count {
        return Err("BI_R2_GENERATED_ACTION_CARDINALITY_GAP".to_owned());
    }
    Ok(rows)
}

/// Apply the adopted R2 quotient only after exact A3 discharge has already
/// produced a singleton.  Failure of candidate-local typed MapCube replay is
/// an expressivity gap; it can never cause a different candidate to be chosen.
fn apply_postselection_r2(
    stage: u32,
    assessment: &mut BranchCandidateAssessment,
    signature: &SealedSignature,
    library: &Library,
    bar: Rational,
    demand_capacity: u32,
    demand: &DemandContext,
) -> Result<(), String> {
    if stage != 8 {
        return Ok(());
    }
    let elaboration = elaborate_telescope(signature, &assessment.telescope, stage - 1)
        .map_err(|error| error.to_string())?;
    let generated_actions =
        prove_candidate_generated_actions(stage, &assessment.telescope, library, &elaboration)?;
    let exact_package = issue_act_local_provenance(signature, stage, &assessment.telescope)
        .map_err(|error| format!("T-BI-NU1 Stage-{stage} postselection replay: {error}"))?;
    if exact_package.structural_formula_total != assessment.structural_formula_total
        || exact_package.generated_instance_adjustment != -(generated_actions.len() as i32)
        || exact_package.generated_instance_removals.len() != generated_actions.len()
    {
        return Err(format!(
            "T-BI-NU1 Stage-{stage} quotient projection does not join the typed R2 action census"
        ));
    }
    assessment.r2_applied_after_unique_discharge = true;
    assessment.r2_generated_instance_adjustment = exact_package.generated_instance_adjustment;
    assessment.r2_generated_action_evidence = generated_actions;
    assessment.semantic_nu = exact_package.exact_certified_nu;
    assessment.diagnostic_formula_replayed_and_capacity_bound_satisfied = assessment.kernel_typed
        && !assessment.identified_with_prefix
        && assessment.semantic_nu
            <= assessment
                .local_role_capacity
                .saturating_add(demand_capacity);
    assessment.nu_provenance = exact_nu_provenance_token(&exact_package);
    let charge = future_v2::issue_filler_ordinary_charge_provenance_v2(
        stage,
        &assessment.telescope,
        u32::from(assessment.kappa),
        assessment.semantic_nu,
        assessment.telescope.bit_cost(),
        assessment.nu_provenance.derivation_hash.clone(),
        exact_package.authoritative_token_hashes(),
    );
    assessment.ordinary_charge_provenance_hash = charge.provenance_hash.clone();
    assessment.structural_discharge_evidence =
        structural_discharge_evidence(signature, stage, &assessment.telescope, demand, &charge);
    assessment.every_live_structural_demand_realized = !demand.structural_pairs.is_empty()
        && assessment.structural_discharge_evidence.len() == demand.structural_pairs.len()
        && assessment
            .structural_discharge_evidence
            .iter()
            .all(|row| row.typed_total_discharge_replayed);
    let structural_semantic_evidence_gap = assessment
        .structural_discharge_evidence
        .iter()
        .any(|row| row.semantic_evidence_gap);
    (
        assessment.semantic_evidence_gap,
        assessment.guarded_total_discharger,
    ) = candidate_semantic_discharge_status(
        assessment.kernel_typed,
        assessment.identified_with_prefix,
        assessment.every_live_structural_demand_realized,
        structural_semantic_evidence_gap,
        &assessment.nu_provenance,
    );
    let rho = Rational::new(
        i64::from(assessment.semantic_nu),
        i64::from(assessment.kappa.max(1)),
    );
    assessment.rho = rho.to_string();
    assessment.diagnostic_clears_bar =
        assessment.kernel_typed && !assessment.identified_with_prefix && rho >= bar;
    assessment.derivation_hash.clear();
    assessment.derivation_hash = tagged_hash("branch-candidate-assessment", assessment);

    if !assessment.every_live_structural_demand_realized || assessment.semantic_evidence_gap {
        return Err(format!(
            "BI_R2_POSTSELECTION_REPLAY_GAP: Stage-{stage} unique discharger lost exact A3 discharge replay after the operational generated-action audit"
        ));
    }
    Ok(())
}

fn common_prefix(
    cone: &CertifiedStage4BranchCone,
    seed: &CertifiedStage4BranchSeed,
) -> Result<(Vec<(u32, Telescope)>, Vec<BranchLedgerRow>), BranchInvarianceError> {
    let mut telescopes = cone.common_stem.clone();
    telescopes.push((4, seed.telescope.clone()));
    // Re-derive the shared stem from its actual telescopes. No historical
    // score vector is accepted as input. Stage 4 is then checked against the
    // independent R-T1 transport value, but that value does not create the
    // formula output.
    let mut library: Library = Vec::new();
    let mut history = Vec::<(u32, u32)>::new();
    let mut prior = Vec::<(u32, Telescope)>::new();
    let mut ledger = Vec::new();
    let exact_packages = issue_act_local_sequence(&telescopes).map_err(|error| {
        BranchInvarianceError::Invariant(format!("T-BI-NU1 shared-prefix issuance failed: {error}"))
    })?;
    for ((stage, telescope), exact_package) in telescopes.iter().zip(&exact_packages) {
        let formula = structural_nu(telescope, &library, &history).total;
        if *stage == 4 && formula != seed.certified_nu {
            return Err(BranchInvarianceError::Invariant(format!(
                "branch-local Stage-4 structural formula {formula} differs from independent R-T1 value {}",
                seed.certified_nu
            )));
        }
        let digest = candidate_hash(telescope);
        if exact_package.stage != *stage
            || exact_package.candidate_hash != digest
            || exact_package.structural_formula_total != formula
            || exact_package.generated_instance_adjustment != 0
            || exact_package.exact_certified_nu != formula
        {
            return Err(BranchInvarianceError::Invariant(format!(
                "T-BI-NU1 shared-prefix projection drifted at Stage {stage}"
            )));
        }
        let token = exact_nu_provenance_token(exact_package);
        ledger.push(BranchLedgerRow {
            stage: *stage,
            candidate_hash: digest,
            kappa: telescope.kappa() as u16,
            semantic_nu: formula,
            nu_provenance: token,
        });
        library.push(LibraryEntry::from_telescope(telescope, &library));
        history.push((*stage, formula));
        prior.push((*stage, telescope.clone()));
    }
    Ok((telescopes, ledger))
}

fn finish_continuation(
    branch: CertifiedStage4BranchSeed,
    limits: BranchContinuationLimits,
    initial_prefix_signature_digest: String,
    initial_prefix_ledger: Vec<BranchLedgerRow>,
    stages: Vec<BranchStageRecord>,
    terminal_demand: BranchDemandAudit,
    outcome: BranchContinuationOutcome,
    complete_ledger: Vec<BranchLedgerRow>,
) -> BranchContinuation {
    let completed_through_stage15 = complete_ledger.iter().any(|row| row.stage == 15);
    let debt_free_halt_at_stage15 = matches!(
        &outcome,
        BranchContinuationOutcome::DebtFreeHalt {
            halt_stage: 15,
            next_stage: 16
        }
    );
    let every_reached_guarded_stage_unique = stages
        .iter()
        .filter(|stage| !stage.demand.debt_free)
        .all(|stage| stage.discharger_count == 1);
    let bar_never_used_as_gate_or_selector = stages
        .iter()
        .all(|stage| !stage.bar_used_as_gate_or_selector);
    let score_never_used_as_selector = stages.iter().all(|stage| !stage.score_used_as_selector);
    let mut result = BranchContinuation {
        schema: BRANCH_INVARIANCE_CORE_SCHEMA.to_owned(),
        date: BRANCH_INVARIANCE_CORE_DATE.to_owned(),
        branch,
        limits,
        initial_prefix_signature_digest,
        initial_prefix_ledger,
        stages,
        terminal_demand,
        outcome,
        complete_ledger,
        completed_through_stage15,
        debt_free_halt_at_stage15,
        every_reached_guarded_stage_unique,
        bar_never_used_as_gate_or_selector,
        score_never_used_as_selector,
        expected_winner_score_ledger_or_bar_accepted_as_runner_input: false,
        hash_or_enumeration_order_used_as_selector: false,
        resource_limit_used_as_halt_claim: false,
        derivation_hash: String::new(),
    };
    result.derivation_hash = result_digest(&result);
    result
}

fn fail_closed_unique_discharger(
    stage: u32,
    discharger_indices: &[usize],
    first_semantic_gap_candidate: Option<&str>,
) -> Result<Option<usize>, String> {
    if let Some(candidate) = first_semantic_gap_candidate {
        return Err(format!(
            "BI_CANDIDATE_CONE_SEMANTIC_GAP: Stage-{stage} candidate {candidate} is Unknown, so the known discharger census cannot establish uniqueness"
        ));
    }
    Ok(match discharger_indices {
        [only] => Some(*only),
        _ => None,
    })
}

/// Execute one branch from Stage 5 through the first debt-free stage or a
/// fail-closed stop. `branch_hash` is an index, never a preference.
pub fn execute_branch_continuation(
    cone: &CertifiedStage4BranchCone,
    branch_hash: &str,
    limits: &BranchContinuationLimits,
) -> Result<BranchContinuation, BranchInvarianceError> {
    if cone.derivation_hash != cone_result_digest(cone)
        || cone.schema != BRANCH_INVARIANCE_CORE_SCHEMA
        || !cone.r_t3_option_b_replayed
        || !cone.r_t2_replay_valid
        || !cone.naturality_transport_replay_valid
        || !cone.no_branch_selected
        || cone.branch_count != 4
        || cone.branches.len() != 4
        || SealedSignature::from_telescopes(cone.common_stem.clone()).digest()
            != cone.common_stem_signature_digest
        || cone.common_stem.iter().map(|(stage, _)| *stage).ne(1..=3)
    {
        return Err(BranchInvarianceError::Prerequisite(
            "Stage-4 branch cone is not a self-consistent replay gate".to_owned(),
        ));
    }
    if limits.max_inspected_stage < BRANCH_FIRST_SELECTION_STAGE
        || limits.max_enumerated_candidates_per_stage == 0
    {
        return Err(BranchInvarianceError::Prerequisite(
            "branch continuation resource limits are empty or precede Stage 5".to_owned(),
        ));
    }
    let branch = cone
        .branches
        .iter()
        .find(|branch| branch.candidate_hash == branch_hash)
        .cloned()
        .ok_or_else(|| {
            BranchInvarianceError::Prerequisite(format!(
                "{branch_hash} is not a member of the certified Stage-4 cone"
            ))
        })?;
    if branch.seed_derivation_hash != seed_digest(&branch)
        || candidate_hash(&branch.telescope) != branch.candidate_hash
        || branch.certified_kappa != cone.minimum_kappa
        || branch.certified_nu != cone.minimum_certified_nu
    {
        return Err(BranchInvarianceError::Prerequisite(
            "selected branch seed failed its cone join".to_owned(),
        ));
    }

    let (mut winners, initial_prefix_ledger) = common_prefix(cone, &branch)?;
    let initial_signature = SealedSignature::from_telescopes(winners.clone());
    if initial_signature.digest() != branch.prefix_signature_digest {
        return Err(BranchInvarianceError::Invariant(
            "Stage-4 prefix digest differs from its certified seed".to_owned(),
        ));
    }
    let mut library: Library = Vec::new();
    let mut accepted_keys = BTreeSet::new();
    for (_, telescope) in &winners {
        accepted_keys.insert(canonical_key_telescope(telescope).0);
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }
    let mut discovery_records = initial_prefix_ledger
        .iter()
        .map(|row| DiscoveryRecord::new(row.stage, row.semantic_nu, u32::from(row.kappa)))
        .collect::<Vec<_>>();
    let mut score_history = initial_prefix_ledger
        .iter()
        .map(|row| (row.stage, row.semantic_nu))
        .collect::<Vec<_>>();
    let mut complete_ledger = initial_prefix_ledger.clone();
    let mut stages = Vec::new();

    let mut stage = BRANCH_FIRST_SELECTION_STAGE;
    loop {
        let debt = summarize_structural_debt(&library, BRANCH_WINDOW_DEPTH);
        let required_packages = required_packages_for(debt)
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let signature = SealedSignature::from_telescopes(winners.clone());
        let demand = match demand_context(&signature, stage, required_packages.clone()) {
            Ok(value) => value,
            Err(error) => {
                let mut terminal = BranchDemandAudit {
                    stage,
                    prefix_signature_digest: signature.digest().to_owned(),
                    coarse_required_packages: required_packages,
                    a3_required_packages: Vec::new(),
                    structural_constructors: Vec::new(),
                    structural_scheme_ids: Vec::new(),
                    structural_instance_ids: Vec::new(),
                    a3_window_derivation_hash: String::new(),
                    a3_inventory_derivation_hash: String::new(),
                    exact_prefix_inventory_exhaustive: false,
                    coarse_and_a3_demands_agree: false,
                    debt_free: false,
                    derivation_hash: String::new(),
                };
                terminal.derivation_hash = tagged_hash("branch-demand-audit", &terminal);
                return Ok(finish_continuation(
                    branch,
                    limits.clone(),
                    initial_signature.digest().to_owned(),
                    initial_prefix_ledger,
                    stages,
                    terminal,
                    BranchContinuationOutcome::ExpressivityGap {
                        stage,
                        exact_error: error,
                    },
                    complete_ledger,
                ));
            }
        };
        if demand.audit.debt_free {
            return Ok(finish_continuation(
                branch,
                limits.clone(),
                initial_signature.digest().to_owned(),
                initial_prefix_ledger,
                stages,
                demand.audit,
                BranchContinuationOutcome::DebtFreeHalt {
                    halt_stage: stage - 1,
                    next_stage: stage,
                },
                complete_ledger,
            ));
        }
        if stage >= limits.max_inspected_stage {
            return Ok(finish_continuation(
                branch,
                limits.clone(),
                initial_signature.digest().to_owned(),
                initial_prefix_ledger,
                stages,
                demand.audit,
                BranchContinuationOutcome::ResourceLimitReached {
                    stage,
                    limit_kind: "max_inspected_stage".to_owned(),
                    configured_limit: u64::from(limits.max_inspected_stage),
                    observed: u64::from(stage),
                    required_packages,
                },
                complete_ledger,
            ));
        }

        let admissibility = strict_admissibility_for_mode(
            stage,
            BRANCH_WINDOW_DEPTH,
            &library,
            AdmissibilityMode::Guarded,
        );
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let mut enumerated = Vec::new();
        for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            enumerated.extend(enumerate_telescopes(&library, context, kappa));
        }
        let cone_enumerated = enumerated.len();
        if cone_enumerated > limits.max_enumerated_candidates_per_stage {
            return Ok(finish_continuation(
                branch,
                limits.clone(),
                initial_signature.digest().to_owned(),
                initial_prefix_ledger,
                stages,
                demand.audit,
                BranchContinuationOutcome::ResourceLimitReached {
                    stage,
                    limit_kind: "max_enumerated_candidates_per_stage".to_owned(),
                    configured_limit: limits.max_enumerated_candidates_per_stage as u64,
                    observed: cone_enumerated as u64,
                    required_packages,
                },
                complete_ledger,
            ));
        }
        let admitted = enumerated
            .into_iter()
            .filter(|candidate| {
                passes_strict_admissibility(stage, &library, candidate, admissibility)
            })
            .collect::<Vec<_>>();
        let cone_admitted = admitted.len();
        let mut seen = BTreeSet::new();
        let mut candidates = admitted
            .into_iter()
            .filter(|candidate| seen.insert(canonical_key_telescope(candidate).0))
            .collect::<Vec<_>>();
        // Canonical output order is not consulted by the selector.
        candidates.sort_by_key(candidate_hash);
        let cone_deduped = candidates.len();
        let stage_cone_digest = cone_digest(&candidates);
        let bar = compute_bar(usize::from(BRANCH_WINDOW_DEPTH), stage, &discovery_records).bar;
        let demand_capacity = demand_output_capacity(stage, &winners, &required_packages);
        let mut assessments = candidates
            .iter()
            .map(|candidate| {
                assess_candidate(
                    stage,
                    candidate,
                    &signature,
                    &library,
                    &score_history,
                    &accepted_keys,
                    bar,
                    demand_capacity,
                    &demand,
                )
            })
            .collect::<Vec<_>>();
        let discharger_indices = assessments
            .iter()
            .enumerate()
            .filter_map(|(index, row)| row.guarded_total_discharger.then_some(index))
            .collect::<Vec<_>>();
        let mut discharger_hashes = discharger_indices
            .iter()
            .map(|index| assessments[*index].candidate_hash.clone())
            .collect::<Vec<_>>();
        discharger_hashes.sort();
        // Unknown losing candidates may still hide a second total
        // discharger. Uniqueness is therefore defined only over a
        // semantically closed assessed cone, never over the known subset.
        let first_semantic_gap_candidate = assessments
            .iter()
            .find(|row| row.semantic_evidence_gap)
            .map(|row| row.candidate_hash.as_str());
        let selection =
            fail_closed_unique_discharger(stage, &discharger_indices, first_semantic_gap_candidate);
        let preselection_semantic_error = selection.as_ref().err().cloned();
        let selected = selection.ok().flatten();
        let r2_error = if let Some(index) = selected {
            apply_postselection_r2(
                stage,
                &mut assessments[index],
                &signature,
                &library,
                bar,
                demand_capacity,
                &demand,
            )
            .err()
        } else {
            None
        };
        let diagnostic_bar_clearing_count = assessments
            .iter()
            .filter(|row| row.diagnostic_clears_bar)
            .count();
        let r2_rule_evidence_hash = selected.filter(|_| stage == 8).map(|index| {
            tagged_hash(
                "branch-stage-r2-rule-evidence",
                &assessments[index]
                    .r2_generated_action_evidence
                    .iter()
                    .map(|row| row.derivation_hash.as_str())
                    .collect::<Vec<_>>(),
            )
        });
        let winner = selected.filter(|_| r2_error.is_none()).map(|index| {
            let row = &assessments[index];
            let mut winner = BranchWinner {
                candidate_hash: row.candidate_hash.clone(),
                canonical_key: row.canonical_key.clone(),
                telescope: row.telescope.clone(),
                kappa: row.kappa,
                semantic_nu: row.semantic_nu,
                nu_provenance: row.nu_provenance.clone(),
                selected_by: "unique_typed_a3_total_discharger".to_owned(),
                diagnostic_clears_bar: row.diagnostic_clears_bar,
                derivation_hash: String::new(),
            };
            winner.derivation_hash = tagged_hash("branch-winner", &winner);
            winner
        });
        let mut record = BranchStageRecord {
            stage,
            demand: demand.audit,
            diagnostic_bar: bar.to_string(),
            bar_used_as_gate_or_selector: false,
            score_used_as_selector: false,
            cone_enumerated,
            cone_admitted,
            cone_deduped,
            cone_digest: stage_cone_digest,
            assessments,
            discharger_hashes,
            discharger_count: discharger_indices.len(),
            diagnostic_bar_clearing_count,
            r2_rule_evidence_hash,
            r2_applied_only_after_unique_discharge: stage != 8
                || selected.is_none()
                || discharger_indices.len() == 1,
            winner: winner.clone(),
            derivation_hash: String::new(),
        };
        record.derivation_hash = tagged_hash("branch-stage", &record);
        stages.push(record);

        let Some(winner) = winner else {
            let last = stages.last().expect("just pushed");
            let terminal_demand = last.demand.clone();
            let semantic_gap = last.assessments.iter().any(|row| row.semantic_evidence_gap);
            let outcome = if let Some(exact_error) = preselection_semantic_error.or(r2_error) {
                BranchContinuationOutcome::ExpressivityGap { stage, exact_error }
            } else if semantic_gap {
                let exact_error = last
                    .assessments
                    .iter()
                    .flat_map(|row| &row.structural_discharge_evidence)
                    .find(|row| row.semantic_evidence_gap)
                    .and_then(|row| row.named_gap_error.clone())
                    .unwrap_or_else(|| "candidate-level A3 discharge evidence gap".to_owned());
                BranchContinuationOutcome::ExpressivityGap { stage, exact_error }
            } else if discharger_indices.is_empty() {
                BranchContinuationOutcome::HaltedNoGuardedDischarger { stage }
            } else {
                BranchContinuationOutcome::HaltedMultipleGuardedDischargers {
                    stage,
                    count: discharger_indices.len(),
                }
            };
            return Ok(finish_continuation(
                branch,
                limits.clone(),
                initial_signature.digest().to_owned(),
                initial_prefix_ledger,
                stages,
                terminal_demand,
                outcome,
                complete_ledger,
            ));
        };
        accepted_keys.insert(winner.canonical_key.clone());
        discovery_records.push(DiscoveryRecord::new(
            stage,
            winner.semantic_nu,
            u32::from(winner.kappa),
        ));
        score_history.push((stage, winner.semantic_nu));
        library.push(LibraryEntry::from_telescope(&winner.telescope, &library));
        winners.push((stage, winner.telescope.clone()));
        complete_ledger.push(BranchLedgerRow {
            stage,
            candidate_hash: winner.candidate_hash,
            kappa: winner.kappa,
            semantic_nu: winner.semantic_nu,
            nu_provenance: winner.nu_provenance,
        });
        stage = stage.checked_add(1).ok_or_else(|| {
            BranchInvarianceError::Invariant("stage counter overflowed".to_owned())
        })?;
    }
}

pub fn replay_branch_continuation(
    cone: &CertifiedStage4BranchCone,
    result: &BranchContinuation,
) -> Vec<String> {
    let mut errors = Vec::new();
    if result.derivation_hash != result_digest(result) {
        errors.push("branch continuation digest mismatch".to_owned());
    }
    for row in &result.complete_ledger {
        errors.extend(
            replay_branch_nu_provenance_token(&row.nu_provenance)
                .into_iter()
                .map(|error| format!("Stage {} ledger: {error}", row.stage)),
        );
        if row.nu_provenance.stage != row.stage
            || row.nu_provenance.candidate_hash != row.candidate_hash
            || row.nu_provenance.diagnostic_nu != row.semantic_nu
        {
            errors.push(format!(
                "Stage {} ledger does not join its nu-provenance token",
                row.stage
            ));
        }
    }
    for stage in &result.stages {
        for assessment in &stage.assessments {
            errors.extend(
                replay_branch_nu_provenance_token(&assessment.nu_provenance)
                    .into_iter()
                    .map(|error| format!("Stage {} assessment: {error}", stage.stage)),
            );
        }
    }
    match execute_branch_continuation(cone, &result.branch.candidate_hash, &result.limits) {
        Ok(expected) if &expected == result => {}
        Ok(_) => errors.push("branch continuation differs from independent replay".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certified_cone_has_four_unselected_branches_and_two_economy_classes() {
        let cone = issue_certified_stage4_branch_cone().expect("certified cone");
        assert!(replay_certified_stage4_branch_cone(&cone).is_empty());
        assert_eq!(cone.branch_count, 4);
        assert_eq!(cone.economy_probe_class_count, 2);
        assert!(cone.no_branch_selected);
        assert_eq!(cone.minimum_kappa, 3);
        assert_eq!(cone.minimum_certified_nu, 5);
    }

    #[test]
    #[ignore = "F-BI requires a sealed passing BI-0 artifact before any non-enacted branch run"]
    fn all_certified_branches_replay_without_reference_or_expected_outcome_input() {
        let cone = issue_certified_stage4_branch_cone().expect("certified cone");
        let limits = BranchContinuationLimits {
            max_inspected_stage: 20,
            max_enumerated_candidates_per_stage: 1_000_000,
        };
        for branch in &cone.branches {
            let result = execute_branch_continuation(&cone, &branch.candidate_hash, &limits)
                .expect("continuation executes");
            assert!(replay_branch_continuation(&cone, &result).is_empty());
            assert!(matches!(
                &result.outcome,
                BranchContinuationOutcome::DebtFreeHalt {
                    halt_stage: 15,
                    next_stage: 16
                }
            ));
            assert!(result.every_reached_guarded_stage_unique);
            assert!(result.debt_free_halt_at_stage15);
            assert!(result.bar_never_used_as_gate_or_selector);
            assert!(result.score_never_used_as_selector);
            assert!(!result.resource_limit_used_as_halt_claim);
            assert_eq!(
                result
                    .initial_prefix_ledger
                    .iter()
                    .map(|row| row.semantic_nu)
                    .collect::<Vec<_>>(),
                vec![1, 1, 2, 5]
            );
            assert!(result.complete_ledger.iter().all(|row| {
                row.nu_provenance.is_exact_certified()
                    && replay_branch_nu_provenance_token(&row.nu_provenance).is_empty()
            }));
            let stage8 = result
                .stages
                .iter()
                .find(|stage| stage.stage == 8)
                .expect("Stage 8 reached");
            let winner = stage8.winner.as_ref().expect("unique Stage-8 winner");
            let assessment = stage8
                .assessments
                .iter()
                .find(|row| row.candidate_hash == winner.candidate_hash)
                .expect("winner assessment retained");
            assert_eq!(assessment.r2_generated_instance_adjustment, -1);
            assert_eq!(assessment.r2_generated_action_evidence.len(), 1);
            assert!(assessment.r2_applied_after_unique_discharge);
        }
    }

    #[test]
    fn redigested_exact_nu_promotion_is_rejected_without_the_theorem_certificate() {
        let mut token = diagnostic_nu_provenance_token(
            5,
            "blake3:candidate".to_owned(),
            "blake3:prefix".to_owned(),
            7,
            0,
        );
        token.disposition = BranchNuProvenanceDisposition::ExactCertified {
            theorem_id: BRANCH_NU_DECOMPOSITION_THEOREM_ID.to_owned(),
            certificate_hash: "blake3:forged".to_owned(),
            counted_family_ids: vec!["family:forged".to_owned()],
            authoritative_family_token_hashes: Vec::new(),
            generated_instance_family_ids_removed_by_quotient: Vec::new(),
        };
        token.derivation_hash = nu_provenance_digest(&token);
        let errors = replay_branch_nu_provenance_token(&token);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("projection is malformed"))
        );
        let (semantic_gap, guarded_total_discharger) =
            candidate_semantic_discharge_status(true, false, true, false, &token);
        assert!(semantic_gap);
        assert!(!guarded_total_discharger);
    }

    #[test]
    fn act_local_issuance_gap_is_named_replayable_and_never_a_discharger() {
        let exact_error = "formula decomposition failed: missing typed family orbit";
        let token = act_local_provenance_gap_token(
            9,
            "blake3:candidate".to_owned(),
            "blake3:prefix".to_owned(),
            17,
            exact_error,
        );
        assert!(replay_branch_nu_provenance_token(&token).is_empty());
        assert!(matches!(
            &token.disposition,
            BranchNuProvenanceDisposition::DiagnosticFormulaOnly {
                missing_theorem_id,
                exact_gap,
            } if missing_theorem_id == T_BI_NU1_THEOREM_ID
                && exact_gap
                    == &format!("{BRANCH_ACT_LOCAL_PROVENANCE_ISSUANCE_GAP_PREFIX}{exact_error}")
        ));

        // Even granting every other typed-discharge premise, the missing
        // exact provenance is a candidate-level semantic gap before census.
        let (semantic_gap, guarded_total_discharger) =
            candidate_semantic_discharge_status(true, false, true, false, &token);
        assert!(semantic_gap);
        assert!(!guarded_total_discharger);
    }

    #[test]
    fn provenance_gap_cannot_be_omitted_to_create_a_false_singleton() {
        let token = act_local_provenance_gap_token(
            9,
            "blake3:provenance-gap".to_owned(),
            "blake3:prefix".to_owned(),
            17,
            "provenance injection failed: no authoritative token",
        );
        let (semantic_gap, guarded_total_discharger) =
            candidate_semantic_discharge_status(true, false, true, false, &token);
        let guarded_flags = [true, guarded_total_discharger];
        let discharger_indices = guarded_flags
            .iter()
            .enumerate()
            .filter_map(|(index, guarded)| guarded.then_some(index))
            .collect::<Vec<_>>();
        assert_eq!(discharger_indices, vec![0]);
        let error = fail_closed_unique_discharger(
            9,
            &discharger_indices,
            semantic_gap.then_some("blake3:provenance-gap"),
        )
        .expect_err("a provenance-Unknown candidate must block false singleton selection");
        assert!(error.contains("known discharger census cannot establish uniqueness"));
    }

    #[test]
    fn unknown_losing_candidate_blocks_singleton_known_discharger() {
        let error = fail_closed_unique_discharger(9, &[0], Some("blake3:unknown"))
            .expect_err("Unknown candidate must block uniqueness");
        assert!(error.contains("known discharger census cannot establish uniqueness"));
        assert_eq!(
            fail_closed_unique_discharger(9, &[0], None).expect("closed cone"),
            Some(0)
        );
    }
}
