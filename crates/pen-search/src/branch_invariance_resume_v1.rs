//! BI-1b branch-local continuation from the sealed BI-1 Stage-8 window.
//!
//! The runner has two deliberately different cone sources:
//!
//! * Stage 8 is projected from one replayed BI-1 branch certificate.  It is
//!   never enumerated again.
//! * A later window is enumerated only after the preceding window selected one
//!   unique typed total discharger and extended this branch's own prefix.
//!
//! Candidate classification precedes the discharger census.  If even one
//! candidate remains `Unknown`, `census` is `None`; a known-subset count is
//! never serialized as though it were a complete census.

use crate::branch_invariance::{
    BRANCH_INVARIANCE_CORE_SCHEMA, BranchContinuationLimits, BranchDemandAudit,
    BranchR2GeneratedActionEvidence, BranchStructuralDischargeEvidence, CertifiedStage4BranchSeed,
};
use crate::branch_invariance_sweep_v3::Bi1BranchCertificateV3;
use crate::branch_prefix_general_provenance_v4::{
    BranchPrefixGeneralProvenanceV4Token, issue_branch_prefix_general_provenance_v4,
    issue_branch_prefix_general_provenance_v4_authority,
    replay_branch_prefix_general_provenance_v4,
    replay_branch_prefix_general_provenance_v4_authority,
    validate_branch_prefix_general_provenance_v4_token_integrity,
};
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use pen_core::canonical::{canonical_key_telescope, canonicalize_telescope};
use pen_core::encode::telescope_bit_cost;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
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

pub const BRANCH_INVARIANCE_RESUME_V1_SCHEMA: &str = "bi1b-prefix-general-sealed-stage8-resume-v1";
pub const BRANCH_INVARIANCE_RESUME_V1_DATE: &str = "2026-07-23";
pub const BI1B_FIRST_RESUMED_STAGE: u32 = 8;
pub const BI1B_KERNEL_IMPOSSIBILITY_THEOREM: &str =
    "typed-total-discharge-requires-kernel-elaboration-v1";
pub const BI1B_PREFIX_IDENTITY_IMPOSSIBILITY_THEOREM: &str =
    "marginal-lawful-act-requires-canonical-prefix-novelty-v1";
pub const BI1B_SEALED_WINDOW_AUTHORIZATION_V1_SCHEMA: &str =
    "bi1b-global-preflight-sealed-window-authorization-v1";

const BI1B_WINDOW_DEPTH: u16 = 2;
const BI1B_DEMAND_MECHANISM_COUNT: u32 = 10;
const BI1B_R2_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/e2_quotient_adjudications.md");

struct Bi1bDemandContext {
    window: A3HistoricalWindow,
    proof: A3WindowRuleInventoryProof,
    structural_pairs: Vec<(A3TypedDemandScheme, A3TypedDemandInstance, String)>,
    audit: BranchDemandAudit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bSealedWindowAuthorizationV1 {
    pub schema: String,
    pub global_bi1_sweep_digest: String,
    pub sealed_global_stage4_cone_digest: String,
    pub enacted_regression_authorization_digest: String,
    pub extraction_procedure_digest: String,
    pub branch_root_hash: String,
    pub sealed_bi1_certificate_digest: String,
    pub authorized_sealed_branch_certificate_count: usize,
    pub sealed_certificate_member_of_global_sweep: bool,
    pub cone_digest: String,
    pub stage8_cone_digest: String,
    pub stage8_demand_derivation_hash: String,
    pub stage8_candidate_hashes: Vec<String>,
    pub sealed_global_bi1_sweep_content_authenticated: bool,
    pub enacted_regression_passed: bool,
    pub regression_gate_row_free: bool,
    pub enacted_outcomes_exported: bool,
    pub selector_capability_exported: bool,
    pub derivation_hash: String,
}

/// Typed, row-free hard-gate surface accepted by the resume kernel.
///
/// The orchestration layer implements this for its sealed enacted-regression
/// authorization.  Passing three unrelated digest strings is deliberately not
/// an issuance API: the gate must also attest the exact regression closure and
/// the absence of enacted rows, vectors, winners, outcomes, or selectors.
pub(crate) trait Bi1bRegressionGate {
    fn replay_valid(&self) -> bool;
    fn sealed_global_sweep_content_authenticated(&self) -> bool;
    fn sealed_sweep_digest(&self) -> &str;
    fn sealed_stage4_cone_digest(&self) -> &str;
    fn authorization_digest(&self) -> &str;
    fn extraction_procedure_digest(&self) -> &str;
    fn authorized_branch_certificate_count(&self) -> usize;
    fn authorizes_branch_certificate_digest(&self, digest: &str) -> bool;
    fn hard_regression_passed(&self) -> bool;
    fn exact_proved_rows_reproduced(&self) -> bool;
    fn exact_theorem_impossibility_rows_reproduced(&self) -> bool;
    fn zero_residuals_reproduced(&self) -> bool;
    fn one_prefix_parametric_procedure(&self) -> bool;
    fn family_ids_exported(&self) -> bool;
    fn semantic_vectors_exported(&self) -> bool;
    fn enacted_winners_exported(&self) -> bool;
    fn enacted_outcomes_exported(&self) -> bool;
    fn selector_capability_exported(&self) -> bool;
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi1bConeOriginV1 {
    SealedBi1Stage8,
    NewlyEnumeratedBranchWindow,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Bi1bCandidateClassificationV1 {
    ProvedSemanticPackage {
        provenance: BranchPrefixGeneralProvenanceV4Token,
    },
    TheoremImpossible {
        theorem_id: String,
        exact_reason: String,
        evidence_hash: String,
    },
    Unknown {
        gap_id: String,
        exact_error: String,
    },
}

impl Bi1bCandidateClassificationV1 {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }

    pub fn provenance(&self) -> Option<&BranchPrefixGeneralProvenanceV4Token> {
        match self {
            Self::ProvedSemanticPackage { provenance } => Some(provenance),
            Self::TheoremImpossible { .. } | Self::Unknown { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bCandidateAssessmentV1 {
    pub candidate_hash: String,
    /// Collision-free serialization of the canonical telescope, including
    /// clause roles; used for marginality and later-window cone quotienting.
    pub canonical_encoding: String,
    /// Legacy FNV projection retained only for correspondence diagnostics.
    pub canonical_key: String,
    pub telescope: Telescope,
    pub kappa: u16,
    pub bit_kappa: u16,
    pub kernel_typed: bool,
    pub kernel_failure: Option<String>,
    pub elaboration_derivation_hash: String,
    pub identified_with_prefix: bool,
    pub structural_formula_total: u32,
    pub semantic_nu: Option<u32>,
    pub extraction_procedure_digest: Option<String>,
    pub classification: Bi1bCandidateClassificationV1,
    pub ordinary_charge_provenance_hash: Option<String>,
    pub structural_discharge_evidence: Vec<BranchStructuralDischargeEvidence>,
    pub every_live_structural_demand_realized: bool,
    pub structural_semantic_gap_errors: Vec<String>,
    pub resolved_before_census: bool,
    pub guarded_total_discharger: bool,
    pub diagnostic_rho: Option<String>,
    pub diagnostic_clears_bar: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi1bCensusDispositionV1 {
    UniqueTypedTotalDischarger,
    NoGuardedDischarger,
    MultipleGuardedDischargers,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bCompleteCensusV1 {
    pub classification_complete_before_census: bool,
    pub discharger_count: usize,
    pub discharger_hashes: Vec<String>,
    pub unique_discharger_hash: Option<String>,
    pub disposition: Bi1bCensusDispositionV1,
    pub semantic_value_used_as_selector: bool,
    pub bar_used_as_selector: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub no_improvised_selection: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bWinnerV1 {
    pub candidate_hash: String,
    pub canonical_encoding: String,
    pub canonical_key: String,
    pub telescope: Telescope,
    pub kappa: u16,
    pub semantic_nu: u32,
    pub provenance_derivation_hash: String,
    pub selected_by: String,
    pub diagnostic_clears_bar: bool,
    pub r2_generated_action_evidence: Vec<BranchR2GeneratedActionEvidence>,
    pub r2_candidate_action_check_applicable: bool,
    pub r2_candidate_actions_replayed_and_cardinality_matches_provenance: bool,
    pub r2_occurrence_to_action_injective_join_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bStageRecordV1 {
    pub stage: u32,
    pub cone_origin: Bi1bConeOriginV1,
    pub stage8_reenumeration_performed: bool,
    pub demand: BranchDemandAudit,
    pub diagnostic_bar: String,
    pub bar_used_as_gate_or_selector: bool,
    pub semantic_value_used_as_selector: bool,
    pub cone_enumerated: usize,
    pub cone_admitted: usize,
    pub cone_deduped: usize,
    pub cone_digest: String,
    pub assessments: Vec<Bi1bCandidateAssessmentV1>,
    pub proved_semantic_package_count: usize,
    pub theorem_impossibility_count: usize,
    pub unknown_count: usize,
    pub every_candidate_classified_before_census: bool,
    /// `None` means that the census was forbidden because at least one
    /// candidate was Unknown.  It never means a zero-discharger census.
    pub census: Option<Bi1bCompleteCensusV1>,
    pub winner: Option<Bi1bWinnerV1>,
    pub selection_or_stop_is_lawful: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bLedgerRowV1 {
    pub stage: u32,
    pub candidate_hash: String,
    pub telescope: Telescope,
    pub kappa: u16,
    pub semantic_nu: u32,
    pub provenance_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Bi1bResumeOutcomeV1 {
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
        phase: String,
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
pub struct BranchInvarianceResumeV1 {
    pub schema: String,
    pub date: String,
    pub branch_root_hash: String,
    pub branch_seed: CertifiedStage4BranchSeed,
    pub sealed_bi1_certificate_digest: String,
    pub sealed_bi1_certificate_authenticated_in_global_preflight: bool,
    /// Opaque authorization minted by the enacted regression rung.  The
    /// runner can test only that it is a content digest; no enacted family,
    /// winner, value, or continuation surface is represented here.
    pub enacted_regression_authorization_digest: String,
    pub sealed_window_authorization_digest: String,
    pub authorized_extraction_procedure_digest: String,
    pub sealed_stage8_cone_digest: String,
    pub sealed_stage8_candidate_hashes: Vec<String>,
    pub sealed_stage8_window_consumed_without_reenumeration: bool,
    pub initial_prefix_signature_digest: String,
    pub limits: BranchContinuationLimits,
    pub stages: Vec<Bi1bStageRecordV1>,
    pub terminal_demand: Option<BranchDemandAudit>,
    pub outcome: Bi1bResumeOutcomeV1,
    pub complete_ledger: Vec<Bi1bLedgerRowV1>,
    pub extraction_procedure_digests: Vec<String>,
    pub every_observed_extraction_procedure_matches_authorization: bool,
    pub exactly_one_extraction_procedure_authorized: bool,
    pub every_completed_census_followed_total_classification: bool,
    pub unknown_never_counted_as_discharger_or_exclusion: bool,
    pub semantic_value_never_used_as_selector: bool,
    pub bar_never_used_as_selector: bool,
    pub hash_or_enumeration_order_never_used_as_selector: bool,
    pub no_enacted_classification_or_outcome_accepted_as_input: bool,
    pub byte_identity_never_used_as_provenance_or_selector: bool,
    pub resource_limit_used_as_halt_claim: bool,
    /// False only for `ResourceLimitReached`: that outcome is a serialized
    /// incomplete computation, never a lawful branch disposition.
    pub branch_lawfully_disposed: bool,
    pub branch_indexed_only: bool,
    pub bi2_finale_issued: bool,
    pub bi4_cone_verdict_issued: bool,
    pub uc1_scored: bool,
    pub unindexed_claim_issued: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchInvarianceResumeV1Replay {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BranchInvarianceResumeV1Error {
    #[error("BI-1b sealed-window prerequisite failed: {0}")]
    SealedWindow(String),
    #[error("BI-1b branch continuation failed: {0}")]
    Continuation(String),
    #[error("BI-1b invariant failed: {0}")]
    Invariant(String),
}

#[derive(Clone)]
struct ResumePrefix {
    entries: Vec<(u32, Telescope)>,
    ledger: Vec<Bi1bLedgerRowV1>,
}

#[derive(Clone)]
struct SealedStage8Cone {
    cone_enumerated: usize,
    cone_admitted: usize,
    cone_deduped: usize,
    cone_digest: String,
    demand: BranchDemandAudit,
    candidates: Vec<Telescope>,
    candidate_hashes: Vec<String>,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BRANCH_INVARIANCE_RESUME_V1_SCHEMA, domain, value))
        .expect("BI-1b branch-resume evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn legacy_core_tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BRANCH_INVARIANCE_CORE_SCHEMA, domain, value))
        .expect("frozen BI-1 core evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn exact_canonical_telescope_key(telescope: &Telescope) -> String {
    serde_json::to_string(&canonicalize_telescope(telescope))
        .expect("canonical telescope serializes deterministically")
}

fn resume_cone_digest(candidates: &[Telescope]) -> String {
    tagged_hash(
        "complete-canonical-cone",
        &candidates.iter().map(candidate_hash).collect::<Vec<_>>(),
    )
}

fn legacy_core_cone_digest(candidates: &[Telescope]) -> String {
    legacy_core_tagged_hash(
        "complete-canonical-cone",
        &candidates.iter().map(candidate_hash).collect::<Vec<_>>(),
    )
}

fn legacy_core_demand_audit_digest(audit: &BranchDemandAudit) -> String {
    let mut projection = audit.clone();
    projection.derivation_hash.clear();
    legacy_core_tagged_hash("branch-demand-audit", &projection)
}

fn resume_demand_audit_digest(audit: &BranchDemandAudit) -> String {
    let mut projection = audit.clone();
    projection.derivation_hash.clear();
    tagged_hash("branch-demand-audit", &projection)
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
        .filter(|(step, _)| *step + u32::from(BI1B_WINDOW_DEPTH) >= stage && *step < stage)
        .map(|(_, telescope)| telescope.clauses.len() as u32)
        .sum::<u32>();
    let unary = source_count.saturating_mul(BI1B_DEMAND_MECHANISM_COUNT);
    let binary = source_count
        .saturating_mul(source_count)
        .saturating_mul(BI1B_DEMAND_MECHANISM_COUNT);
    unary.saturating_add(binary.saturating_mul(2))
}

fn demand_context(
    signature: &SealedSignature,
    stage: u32,
    coarse_required_packages: Vec<String>,
) -> Result<Bi1bDemandContext, String> {
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
    audit.derivation_hash = if stage == BI1B_FIRST_RESUMED_STAGE {
        legacy_core_demand_audit_digest(&audit)
    } else {
        resume_demand_audit_digest(&audit)
    };
    Ok(Bi1bDemandContext {
        window,
        proof,
        structural_pairs,
        audit,
    })
}

fn is_blake3_digest(value: &str) -> bool {
    value.starts_with("blake3:")
        && value.len() == "blake3:".len() + 64
        && value["blake3:".len()..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
}

fn extraction_procedure_authority_replays_exactly(digest: &str) -> bool {
    let authority = issue_branch_prefix_general_provenance_v4_authority();
    replay_branch_prefix_general_provenance_v4_authority(&authority).is_empty()
        && authority.proved
        && authority.one_procedure_for_every_prefix
        && authority.candidate_and_exact_prefix_are_only_semantic_inputs
        && !authority.archive_input_read
        && !authority.historical_registry_input_read
        && !authority.structural_nu_input_read
        && !authority.bar_input_read
        && !authority.verdict_input_read
        && !authority.enacted_future_input_read
        && !authority.enacted_classification_input_read
        && !authority.byte_identity_input_read
        && digest == authority.derivation_hash
}

fn sealed_window_authorization_digest(authorization: &Bi1bSealedWindowAuthorizationV1) -> String {
    let mut projection = authorization.clone();
    projection.derivation_hash.clear();
    tagged_hash("sealed-window-authorization", &projection)
}

/// Mint the narrow capability consumed by one branch resume.  Callers must do
/// so only after the frozen global BI-1 sweep has been content-authenticated
/// and the enacted regression has replayed.
/// The capability carries no enacted candidate, family, winner, value, or
/// continuation surface.
pub(crate) fn issue_bi1b_sealed_window_authorization_v1<G: Bi1bRegressionGate + ?Sized>(
    sealed: &Bi1BranchCertificateV3,
    gate: &G,
) -> Result<Bi1bSealedWindowAuthorizationV1, BranchInvarianceResumeV1Error> {
    let regression_gate_row_free = !gate.family_ids_exported()
        && !gate.semantic_vectors_exported()
        && !gate.enacted_winners_exported()
        && !gate.enacted_outcomes_exported();
    let sealed_certificate_member_of_global_sweep =
        gate.authorizes_branch_certificate_digest(&sealed.result_digest);
    if !is_blake3_digest(gate.sealed_sweep_digest())
        || !is_blake3_digest(gate.sealed_stage4_cone_digest())
        || !is_blake3_digest(gate.authorization_digest())
        || !is_blake3_digest(gate.extraction_procedure_digest())
        || !extraction_procedure_authority_replays_exactly(gate.extraction_procedure_digest())
        || !is_blake3_digest(&sealed.result_digest)
        || !is_blake3_digest(&sealed.cone_digest)
    {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "sealed-window authorization inputs are not content digests".to_owned(),
        ));
    }
    if !gate.replay_valid()
        || !gate.sealed_global_sweep_content_authenticated()
        || !gate.hard_regression_passed()
        || !gate.exact_proved_rows_reproduced()
        || !gate.exact_theorem_impossibility_rows_reproduced()
        || !gate.zero_residuals_reproduced()
        || !gate.one_prefix_parametric_procedure()
        || gate.authorized_branch_certificate_count() != 4
        || !sealed_certificate_member_of_global_sweep
        || !regression_gate_row_free
        || gate.selector_capability_exported()
    {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "typed enacted-regression gate is not closed, row-free, and selector-free".to_owned(),
        ));
    }
    if sealed.cone_digest != gate.sealed_stage4_cone_digest() {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "sealed branch is not bound to the authenticated global Stage-4 cone".to_owned(),
        ));
    }
    let stage8 = sealed
        .continuation
        .stages
        .iter()
        .find(|stage| stage.stage == BI1B_FIRST_RESUMED_STAGE)
        .ok_or_else(|| {
            BranchInvarianceResumeV1Error::SealedWindow(
                "cannot authorize a branch certificate without a sealed Stage-8 window".to_owned(),
            )
        })?;
    let stage8_candidate_hashes = stage8
        .assessments
        .iter()
        .map(|assessment| assessment.candidate_hash.clone())
        .collect::<Vec<_>>();
    let mut authorization = Bi1bSealedWindowAuthorizationV1 {
        schema: BI1B_SEALED_WINDOW_AUTHORIZATION_V1_SCHEMA.to_owned(),
        global_bi1_sweep_digest: gate.sealed_sweep_digest().to_owned(),
        sealed_global_stage4_cone_digest: gate.sealed_stage4_cone_digest().to_owned(),
        enacted_regression_authorization_digest: gate.authorization_digest().to_owned(),
        extraction_procedure_digest: gate.extraction_procedure_digest().to_owned(),
        branch_root_hash: sealed.branch_root_hash.clone(),
        sealed_bi1_certificate_digest: sealed.result_digest.clone(),
        authorized_sealed_branch_certificate_count: gate.authorized_branch_certificate_count(),
        sealed_certificate_member_of_global_sweep,
        cone_digest: sealed.cone_digest.clone(),
        stage8_cone_digest: stage8.cone_digest.clone(),
        stage8_demand_derivation_hash: stage8.demand.derivation_hash.clone(),
        stage8_candidate_hashes,
        sealed_global_bi1_sweep_content_authenticated: true,
        enacted_regression_passed: true,
        regression_gate_row_free,
        enacted_outcomes_exported: gate.enacted_outcomes_exported(),
        selector_capability_exported: gate.selector_capability_exported(),
        derivation_hash: String::new(),
    };
    authorization.derivation_hash = sealed_window_authorization_digest(&authorization);
    Ok(authorization)
}

pub fn replay_bi1b_sealed_window_authorization_v1(
    sealed: &Bi1BranchCertificateV3,
    claimed: &Bi1bSealedWindowAuthorizationV1,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != sealed_window_authorization_digest(claimed) {
        errors.push("sealed-window authorization digest mismatch".to_owned());
    }
    if claimed.schema != BI1B_SEALED_WINDOW_AUTHORIZATION_V1_SCHEMA
        || claimed.branch_root_hash != sealed.branch_root_hash
        || claimed.sealed_bi1_certificate_digest != sealed.result_digest
        || claimed.authorized_sealed_branch_certificate_count != 4
        || !claimed.sealed_certificate_member_of_global_sweep
        || claimed.cone_digest != sealed.cone_digest
        || claimed.sealed_global_stage4_cone_digest != sealed.cone_digest
        || !claimed.sealed_global_bi1_sweep_content_authenticated
        || !claimed.enacted_regression_passed
        || !claimed.regression_gate_row_free
        || claimed.enacted_outcomes_exported
        || claimed.selector_capability_exported
        || !is_blake3_digest(&claimed.global_bi1_sweep_digest)
        || !is_blake3_digest(&claimed.enacted_regression_authorization_digest)
        || !is_blake3_digest(&claimed.extraction_procedure_digest)
        || !extraction_procedure_authority_replays_exactly(&claimed.extraction_procedure_digest)
    {
        errors.push("sealed-window authorization surface is invalid".to_owned());
    }
    let Some(stage8) = sealed
        .continuation
        .stages
        .iter()
        .find(|stage| stage.stage == BI1B_FIRST_RESUMED_STAGE)
    else {
        errors.push("sealed BI-1 certificate has no Stage-8 window".to_owned());
        return errors;
    };
    if claimed.stage8_cone_digest != stage8.cone_digest
        || claimed.stage8_demand_derivation_hash != stage8.demand.derivation_hash
        || claimed.stage8_candidate_hashes
            != stage8
                .assessments
                .iter()
                .map(|assessment| assessment.candidate_hash.clone())
                .collect::<Vec<_>>()
    {
        errors
            .push("sealed-window authorization does not bind the exact Stage-8 surface".to_owned());
    }
    errors
}

fn assessment_digest(row: &Bi1bCandidateAssessmentV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("candidate-assessment", &projection)
}

fn census_digest(row: &Bi1bCompleteCensusV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("complete-discharger-census", &projection)
}

fn winner_digest(row: &Bi1bWinnerV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("winner", &projection)
}

fn stage_digest(row: &Bi1bStageRecordV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("stage", &projection)
}

fn ledger_row_digest(row: &Bi1bLedgerRowV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("ledger-row", &projection)
}

fn result_digest(result: &BranchInvarianceResumeV1) -> String {
    let mut projection = result.clone();
    projection.result_digest.clear();
    tagged_hash("branch-resume", &projection)
}

fn theorem_impossibility(
    theorem_id: &str,
    candidate_digest: &str,
    exact_reason: String,
) -> Bi1bCandidateClassificationV1 {
    let evidence_hash = tagged_hash(
        "candidate-theorem-impossibility",
        &(theorem_id, candidate_digest, &exact_reason),
    );
    Bi1bCandidateClassificationV1::TheoremImpossible {
        theorem_id: theorem_id.to_owned(),
        exact_reason,
        evidence_hash,
    }
}

fn unknown(gap_id: &str, exact_error: String) -> Bi1bCandidateClassificationV1 {
    Bi1bCandidateClassificationV1::Unknown {
        gap_id: gap_id.to_owned(),
        exact_error,
    }
}

fn row_for_prefix(
    stage: u32,
    telescope: Telescope,
    semantic_nu: u32,
    provenance_derivation_hash: String,
) -> Bi1bLedgerRowV1 {
    let mut row = Bi1bLedgerRowV1 {
        stage,
        candidate_hash: candidate_hash(&telescope),
        kappa: telescope.kappa() as u16,
        telescope,
        semantic_nu,
        provenance_derivation_hash,
        derivation_hash: String::new(),
    };
    row.derivation_hash = ledger_row_digest(&row);
    row
}

fn reconstruct_branch_prefix_through_stage7(
    sealed: &Bi1BranchCertificateV3,
) -> Result<ResumePrefix, BranchInvarianceResumeV1Error> {
    let genesis = SealedSignature::genesis_del_h15();
    let mut entries = (1..=3)
        .map(|stage| {
            genesis
                .entry(stage)
                .map(|entry| (stage, entry.telescope.clone()))
                .ok_or_else(|| {
                    BranchInvarianceResumeV1Error::SealedWindow(format!(
                        "frozen Genesis seal omits common-stem Stage {stage}"
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let seed = &sealed.continuation.branch;
    if seed.candidate_hash != sealed.branch_root_hash
        || candidate_hash(&seed.telescope) != sealed.branch_root_hash
    {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "globally authorized BI-1 certificate does not bind its Stage-4 branch root".to_owned(),
        ));
    }
    entries.push((4, seed.telescope.clone()));
    let stage4_prefix = SealedSignature::from_telescopes(entries.clone());
    if stage4_prefix.digest() != sealed.continuation.initial_prefix_signature_digest {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "sealed continuation's Stage-4 prefix digest does not match its root and common stem"
                .to_owned(),
        ));
    }
    for stage in 5..=7 {
        let winner = sealed
            .continuation
            .stages
            .iter()
            .find(|row| row.stage == stage)
            .and_then(|row| row.winner.as_ref())
            .ok_or_else(|| {
                BranchInvarianceResumeV1Error::SealedWindow(format!(
                    "sealed BI-1 branch has no unique Stage-{stage} winner before its Stage-8 window"
                ))
            })?;
        entries.push((stage, winner.telescope.clone()));
    }
    if entries.iter().map(|(stage, _)| *stage).ne(1..=7) {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "reconstructed resume prefix is not exactly Stage 1 through Stage 7".to_owned(),
        ));
    }
    let sealed_rows = sealed
        .continuation
        .complete_ledger
        .iter()
        .filter(|row| row.stage <= 7)
        .collect::<Vec<_>>();
    if sealed_rows.len() != 7 {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "sealed BI-1 branch does not carry exactly seven prefix ledger rows".to_owned(),
        ));
    }
    let mut ledger = Vec::with_capacity(7);
    for ((stage, telescope), sealed_row) in entries.iter().zip(sealed_rows) {
        if *stage != sealed_row.stage
            || candidate_hash(telescope) != sealed_row.candidate_hash
            || telescope.kappa() as u16 != sealed_row.kappa
        {
            return Err(BranchInvarianceResumeV1Error::SealedWindow(format!(
                "sealed BI-1 ledger does not join the reconstructed Stage-{stage} act"
            )));
        }
        ledger.push(row_for_prefix(
            *stage,
            telescope.clone(),
            sealed_row.semantic_nu,
            sealed_row.nu_provenance.derivation_hash.clone(),
        ));
    }
    Ok(ResumePrefix { entries, ledger })
}

fn project_sealed_stage8_cone(
    sealed: &Bi1BranchCertificateV3,
    prefix: &ResumePrefix,
) -> Result<SealedStage8Cone, BranchInvarianceResumeV1Error> {
    let stage8 = sealed
        .continuation
        .stages
        .iter()
        .find(|row| row.stage == BI1B_FIRST_RESUMED_STAGE)
        .ok_or_else(|| {
            BranchInvarianceResumeV1Error::SealedWindow(
                "sealed BI-1 branch does not contain its Stage-8 cone".to_owned(),
            )
        })?;
    let prefix_signature = SealedSignature::from_telescopes(prefix.entries.clone());
    if stage8.demand.prefix_signature_digest != prefix_signature.digest() {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "sealed Stage-8 demand does not bind the branch's own reconstructed B7".to_owned(),
        ));
    }
    let candidates = stage8
        .assessments
        .iter()
        .map(|assessment| assessment.telescope.clone())
        .collect::<Vec<_>>();
    let candidate_hashes = candidates.iter().map(candidate_hash).collect::<Vec<_>>();
    if candidates.len() != stage8.cone_deduped
        || stage8.assessments.len() != stage8.cone_deduped
        || candidate_hashes
            .iter()
            .zip(&stage8.assessments)
            .any(|(digest, assessment)| {
                digest != &assessment.candidate_hash
                    || canonical_key_telescope(&assessment.telescope).0 != assessment.canonical_key
            })
        || candidate_hashes.iter().collect::<BTreeSet<_>>().len() != candidate_hashes.len()
        || candidates
            .iter()
            .map(exact_canonical_telescope_key)
            .collect::<BTreeSet<_>>()
            .len()
            != candidates.len()
        || !candidate_hashes.windows(2).all(|pair| pair[0] <= pair[1])
        || legacy_core_cone_digest(&candidates) != stage8.cone_digest
    {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "sealed Stage-8 candidate surface failed its exact cone projection".to_owned(),
        ));
    }
    Ok(SealedStage8Cone {
        cone_enumerated: stage8.cone_enumerated,
        cone_admitted: stage8.cone_admitted,
        cone_deduped: stage8.cone_deduped,
        cone_digest: stage8.cone_digest.clone(),
        demand: stage8.demand.clone(),
        candidates,
        candidate_hashes,
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
        named_gap_id: Some(format!("bi1b_{phase}_error")),
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
    demand: &Bi1bDemandContext,
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

fn structural_gap_errors(rows: &[BranchStructuralDischargeEvidence]) -> Vec<String> {
    rows.iter()
        .filter(|row| row.semantic_evidence_gap)
        .map(|row| {
            row.named_gap_error.clone().unwrap_or_else(|| {
                format!(
                    "{}:{} is an unnamed structural semantic gap",
                    row.a3_scheme_id, row.a3_instance_id
                )
            })
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn assess_candidate(
    stage: u32,
    candidate: &Telescope,
    prefix_entries: &[(u32, Telescope)],
    signature: &SealedSignature,
    library: &Library,
    history: &[(u32, u32)],
    accepted_encodings: &BTreeSet<String>,
    diagnostic_bar: pen_core::rational::Rational,
    _demand_capacity: u32,
    demand: &Bi1bDemandContext,
    authorized_extraction_procedure_digest: &str,
) -> Bi1bCandidateAssessmentV1 {
    let candidate_digest = candidate_hash(candidate);
    let canonical_encoding = exact_canonical_telescope_key(candidate);
    let canonical_key = canonical_key_telescope(candidate).0;
    let kappa = candidate.kappa() as u16;
    let bit_kappa = u16::try_from(telescope_bit_cost(candidate)).expect("bit kappa fits u16");
    let identified_with_prefix = accepted_encodings.contains(&canonical_encoding);
    let (kernel_typed, kernel_failure, elaboration_derivation_hash) =
        match elaborate_telescope(signature, candidate, stage - 1) {
            Ok(elaboration) => (true, None, elaboration.derivation_hash),
            Err(error) => (false, Some(error.to_string()), String::new()),
        };
    let structural_formula_total = structural_nu(candidate, library, history).total;

    let mut classification = if !kernel_typed {
        theorem_impossibility(
            BI1B_KERNEL_IMPOSSIBILITY_THEOREM,
            &candidate_digest,
            format!(
                "candidate cannot be a typed total discharger because exact-prefix elaboration failed: {}",
                kernel_failure
                    .as_deref()
                    .unwrap_or("unspecified elaboration failure")
            ),
        )
    } else if identified_with_prefix {
        theorem_impossibility(
            BI1B_PREFIX_IDENTITY_IMPOSSIBILITY_THEOREM,
            &candidate_digest,
            format!(
                "the exact canonical encoding (legacy diagnostic key `{canonical_key}`) is already present in the candidate's own prefix, so marginality is false"
            ),
        )
    } else {
        match issue_branch_prefix_general_provenance_v4(prefix_entries, stage, candidate) {
            Ok(provenance) => {
                let replay = replay_branch_prefix_general_provenance_v4(
                    prefix_entries,
                    candidate,
                    &provenance,
                );
                let integrity =
                    validate_branch_prefix_general_provenance_v4_token_integrity(&provenance);
                if replay.is_empty()
                    && integrity.is_empty()
                    && provenance.stage == stage
                    && provenance.candidate_hash == candidate_digest
                    && provenance.predecessor_signature_digest == signature.digest()
                    && provenance.proved
                    && provenance.all_residual_counts_zero
                    && provenance.every_role_declaration_resolved
                    && provenance.every_marginal_family_credited_or_theorem_impossible
                    && provenance.no_archive_or_historical_registry_input
                    && provenance.extraction_procedure_digest
                        == authorized_extraction_procedure_digest
                    && !provenance.bar_input_read
                    && !provenance.structural_nu_input_read
                    && !provenance.verdict_input_read
                    && !provenance.enacted_future_input_read
                    && !provenance.byte_identity_input_read
                    && !provenance.enacted_classification_input_read
                {
                    Bi1bCandidateClassificationV1::ProvedSemanticPackage { provenance }
                } else {
                    unknown(
                        "BI1B_PREFIX_GENERAL_PROVENANCE_REPLAY_GAP",
                        format!(
                            "Stage-{stage} prefix-general provenance did not replay or close: replay={replay:?}; integrity={integrity:?}"
                        ),
                    )
                }
            }
            Err(error) => unknown(
                "BI1B_PREFIX_GENERAL_PROVENANCE_ISSUANCE_GAP",
                error.to_string(),
            ),
        }
    };

    let provenance = classification.provenance();
    let semantic_nu = provenance.map(|token| token.semantic_nu);
    let extraction_procedure_digest =
        provenance.map(|token| token.extraction_procedure_digest.clone());
    let family_row_hashes = provenance.map_or_else(Vec::new, |token| {
        token
            .credited_family_rows
            .iter()
            .map(|row| row.authoritative_family_row_derivation_hash.clone())
            .collect()
    });
    let charge = provenance.map(|token| {
        future_v2::issue_filler_ordinary_charge_provenance_v2(
            stage,
            candidate,
            u32::from(kappa),
            token.semantic_nu,
            candidate.bit_cost(),
            token.derivation_hash.clone(),
            family_row_hashes,
        )
    });
    let structural_discharge_evidence = charge.as_ref().map_or_else(Vec::new, |charge| {
        structural_discharge_evidence(signature, stage, candidate, demand, charge)
    });
    let every_live_structural_demand_realized = provenance.is_some()
        && !demand.structural_pairs.is_empty()
        && structural_discharge_evidence.len() == demand.structural_pairs.len()
        && structural_discharge_evidence
            .iter()
            .all(|row| row.typed_total_discharge_replayed);
    let structural_semantic_gap_errors = structural_gap_errors(&structural_discharge_evidence);
    let provenance_resolved = provenance.is_some();
    if !structural_semantic_gap_errors.is_empty() {
        classification = unknown(
            "BI1B_STRUCTURAL_DISCHARGE_SEMANTIC_GAP",
            structural_semantic_gap_errors.join("; "),
        );
    }
    let resolved_before_census = !classification.is_unknown();
    let guarded_total_discharger =
        provenance_resolved && resolved_before_census && every_live_structural_demand_realized;
    let diagnostic_rho = semantic_nu.map(|nu| {
        pen_core::rational::Rational::new(i64::from(nu), i64::from(kappa.max(1))).to_string()
    });
    let diagnostic_clears_bar = semantic_nu.is_some_and(|nu| {
        pen_core::rational::Rational::new(i64::from(nu), i64::from(kappa.max(1))) >= diagnostic_bar
    });
    let mut row = Bi1bCandidateAssessmentV1 {
        candidate_hash: candidate_digest,
        canonical_encoding,
        canonical_key,
        telescope: candidate.clone(),
        kappa,
        bit_kappa,
        kernel_typed,
        kernel_failure,
        elaboration_derivation_hash,
        identified_with_prefix,
        structural_formula_total,
        semantic_nu,
        extraction_procedure_digest,
        classification,
        ordinary_charge_provenance_hash: charge.map(|token| token.provenance_hash),
        structural_discharge_evidence,
        every_live_structural_demand_realized,
        structural_semantic_gap_errors,
        resolved_before_census,
        guarded_total_discharger,
        diagnostic_rho,
        diagnostic_clears_bar,
        derivation_hash: String::new(),
    };
    row.derivation_hash = assessment_digest(&row);
    row
}

fn complete_census(assessments: &[Bi1bCandidateAssessmentV1]) -> Option<Bi1bCompleteCensusV1> {
    if assessments
        .iter()
        .any(|assessment| !assessment.resolved_before_census)
    {
        return None;
    }
    let mut discharger_hashes = assessments
        .iter()
        .filter(|assessment| assessment.guarded_total_discharger)
        .map(|assessment| assessment.candidate_hash.clone())
        .collect::<Vec<_>>();
    discharger_hashes.sort();
    let (unique_discharger_hash, disposition) = match discharger_hashes.as_slice() {
        [only] => (
            Some(only.clone()),
            Bi1bCensusDispositionV1::UniqueTypedTotalDischarger,
        ),
        [] => (None, Bi1bCensusDispositionV1::NoGuardedDischarger),
        _ => (None, Bi1bCensusDispositionV1::MultipleGuardedDischargers),
    };
    let mut census = Bi1bCompleteCensusV1 {
        classification_complete_before_census: true,
        discharger_count: discharger_hashes.len(),
        discharger_hashes,
        unique_discharger_hash,
        disposition,
        semantic_value_used_as_selector: false,
        bar_used_as_selector: false,
        hash_or_enumeration_order_used_as_selector: false,
        no_improvised_selection: true,
        derivation_hash: String::new(),
    };
    census.derivation_hash = census_digest(&census);
    Some(census)
}

fn r2_rule_source_hash() -> Result<String, String> {
    let text =
        std::str::from_utf8(BI1B_R2_ADJUDICATION_BYTES).map_err(|error| error.to_string())?;
    if !text.contains("derived-action-generator-membership-rule-v1")
        || !text.contains("**R2 adopted**")
        || !text.contains("position, label, or archived cardinality")
    {
        return Err("adopted count-blind R2 source did not replay".to_owned());
    }
    Ok(format!("blake3:{}", blake3_hex(BI1B_R2_ADJUDICATION_BYTES)))
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

fn prove_r2_after_unique_discharge(
    stage: u32,
    assessment: &Bi1bCandidateAssessmentV1,
    library: &Library,
    signature: &SealedSignature,
) -> Result<Vec<BranchR2GeneratedActionEvidence>, String> {
    if stage != BI1B_FIRST_RESUMED_STAGE {
        return Ok(Vec::new());
    }
    let provenance = assessment.classification.provenance().ok_or_else(|| {
        "BI1B_R2_POSTSELECTION_GAP: unique discharger lacks prefix-general provenance".to_owned()
    })?;
    let elaboration = elaborate_telescope(signature, &assessment.telescope, stage - 1)
        .map_err(|error| error.to_string())?;
    let actions =
        prove_candidate_generated_actions(stage, &assessment.telescope, library, &elaboration)?;
    if provenance.r2_removed_occurrence_hashes.len() != actions.len()
        || provenance.r2_generated_instance_removed_count != actions.len()
        || provenance
            .r2_step8_typed_signature_derivation_hash
            .is_none()
        || provenance
            .r2_m1_generated_membership_derivation_hash
            .is_none()
    {
        return Err(format!(
            "BI1B_R2_POSTSELECTION_GAP: prefix-general R2 projection does not join the {} candidate-bound generated actions",
            actions.len()
        ));
    }
    Ok(actions)
}

fn stage_record(
    stage: u32,
    cone_origin: Bi1bConeOriginV1,
    cone_enumerated: usize,
    cone_admitted: usize,
    cone_deduped: usize,
    stage_cone_digest: String,
    demand: BranchDemandAudit,
    diagnostic_bar: String,
    assessments: Vec<Bi1bCandidateAssessmentV1>,
    census: Option<Bi1bCompleteCensusV1>,
    winner: Option<Bi1bWinnerV1>,
) -> Bi1bStageRecordV1 {
    let proved_semantic_package_count = assessments
        .iter()
        .filter(|assessment| {
            matches!(
                &assessment.classification,
                Bi1bCandidateClassificationV1::ProvedSemanticPackage { .. }
            )
        })
        .count();
    let theorem_impossibility_count = assessments
        .iter()
        .filter(|assessment| {
            matches!(
                &assessment.classification,
                Bi1bCandidateClassificationV1::TheoremImpossible { .. }
            )
        })
        .count();
    let unknown_count = assessments
        .iter()
        .filter(|assessment| !assessment.resolved_before_census)
        .count();
    let every_candidate_classified_before_census = unknown_count == 0;
    let selection_or_stop_is_lawful = match (&census, &winner) {
        (None, None) => unknown_count > 0,
        (Some(census), Some(winner)) => {
            census.discharger_count == 1
                && census.unique_discharger_hash.as_deref() == Some(&winner.candidate_hash)
                && census.no_improvised_selection
        }
        (Some(census), None) => census.discharger_count != 1 && census.no_improvised_selection,
        (None, Some(_)) => false,
    };
    let mut row = Bi1bStageRecordV1 {
        stage,
        stage8_reenumeration_performed: stage == BI1B_FIRST_RESUMED_STAGE
            && cone_origin != Bi1bConeOriginV1::SealedBi1Stage8,
        cone_origin,
        demand,
        diagnostic_bar,
        bar_used_as_gate_or_selector: false,
        semantic_value_used_as_selector: false,
        cone_enumerated,
        cone_admitted,
        cone_deduped,
        cone_digest: stage_cone_digest,
        assessments,
        proved_semantic_package_count,
        theorem_impossibility_count,
        unknown_count,
        every_candidate_classified_before_census,
        census,
        winner,
        selection_or_stop_is_lawful,
        derivation_hash: String::new(),
    };
    row.derivation_hash = stage_digest(&row);
    row
}

fn finish(
    branch_seed: CertifiedStage4BranchSeed,
    sealed: &Bi1BranchCertificateV3,
    authorization: &Bi1bSealedWindowAuthorizationV1,
    sealed_stage8: &SealedStage8Cone,
    initial_prefix_signature_digest: String,
    limits: BranchContinuationLimits,
    stages: Vec<Bi1bStageRecordV1>,
    terminal_demand: Option<BranchDemandAudit>,
    outcome: Bi1bResumeOutcomeV1,
    complete_ledger: Vec<Bi1bLedgerRowV1>,
) -> Result<BranchInvarianceResumeV1, BranchInvarianceResumeV1Error> {
    let branch_lawfully_disposed =
        !matches!(&outcome, Bi1bResumeOutcomeV1::ResourceLimitReached { .. });
    let extraction_procedure_digests = stages
        .iter()
        .flat_map(|stage| &stage.assessments)
        .filter_map(|assessment| assessment.extraction_procedure_digest.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let every_observed_extraction_procedure_matches_authorization = extraction_procedure_digests
        .iter()
        .all(|digest| digest == &authorization.extraction_procedure_digest);
    let exactly_one_extraction_procedure_authorized =
        extraction_procedure_authority_replays_exactly(&authorization.extraction_procedure_digest);
    let every_completed_census_followed_total_classification = stages.iter().all(|stage| {
        stage
            .census
            .as_ref()
            .is_none_or(|census| census.classification_complete_before_census)
    });
    let unknown_never_counted_as_discharger_or_exclusion = stages.iter().all(|stage| {
        (stage.unknown_count == 0 && stage.census.is_some())
            || (stage.unknown_count > 0 && stage.census.is_none())
    });
    let semantic_value_never_used_as_selector = stages
        .iter()
        .all(|stage| !stage.semantic_value_used_as_selector);
    let bar_never_used_as_selector = stages
        .iter()
        .all(|stage| !stage.bar_used_as_gate_or_selector);
    let hash_or_enumeration_order_never_used_as_selector = stages.iter().all(|stage| {
        stage
            .census
            .as_ref()
            .is_none_or(|census| !census.hash_or_enumeration_order_used_as_selector)
    });
    let mut result = BranchInvarianceResumeV1 {
        schema: BRANCH_INVARIANCE_RESUME_V1_SCHEMA.to_owned(),
        date: BRANCH_INVARIANCE_RESUME_V1_DATE.to_owned(),
        branch_root_hash: branch_seed.candidate_hash.clone(),
        branch_seed,
        sealed_bi1_certificate_digest: sealed.result_digest.clone(),
        sealed_bi1_certificate_authenticated_in_global_preflight: authorization
            .sealed_global_bi1_sweep_content_authenticated
            && authorization.sealed_certificate_member_of_global_sweep
            && authorization.authorized_sealed_branch_certificate_count == 4
            && authorization.sealed_global_stage4_cone_digest == sealed.cone_digest,
        enacted_regression_authorization_digest: authorization
            .enacted_regression_authorization_digest
            .clone(),
        sealed_window_authorization_digest: authorization.derivation_hash.clone(),
        authorized_extraction_procedure_digest: authorization.extraction_procedure_digest.clone(),
        sealed_stage8_cone_digest: sealed_stage8.cone_digest.clone(),
        sealed_stage8_candidate_hashes: sealed_stage8.candidate_hashes.clone(),
        sealed_stage8_window_consumed_without_reenumeration: stages.first().is_some_and(|stage| {
            stage.stage == BI1B_FIRST_RESUMED_STAGE
                && stage.cone_origin == Bi1bConeOriginV1::SealedBi1Stage8
                && !stage.stage8_reenumeration_performed
        }),
        initial_prefix_signature_digest,
        limits,
        stages,
        terminal_demand,
        outcome,
        complete_ledger,
        extraction_procedure_digests,
        every_observed_extraction_procedure_matches_authorization,
        exactly_one_extraction_procedure_authorized,
        every_completed_census_followed_total_classification,
        unknown_never_counted_as_discharger_or_exclusion,
        semantic_value_never_used_as_selector,
        bar_never_used_as_selector,
        hash_or_enumeration_order_never_used_as_selector,
        no_enacted_classification_or_outcome_accepted_as_input: !authorization
            .enacted_outcomes_exported
            && authorization.branch_root_hash == sealed.branch_root_hash
            && authorization.sealed_bi1_certificate_digest == sealed.result_digest,
        byte_identity_never_used_as_provenance_or_selector: true,
        resource_limit_used_as_halt_claim: false,
        branch_lawfully_disposed,
        branch_indexed_only: true,
        bi2_finale_issued: false,
        bi4_cone_verdict_issued: false,
        uc1_scored: false,
        unindexed_claim_issued: false,
        result_digest: String::new(),
    };
    let gate = result.sealed_bi1_certificate_authenticated_in_global_preflight
        && result.sealed_stage8_window_consumed_without_reenumeration
        && result.every_observed_extraction_procedure_matches_authorization
        && result.exactly_one_extraction_procedure_authorized
        && result.every_completed_census_followed_total_classification
        && result.unknown_never_counted_as_discharger_or_exclusion
        && result.semantic_value_never_used_as_selector
        && result.bar_never_used_as_selector
        && result.hash_or_enumeration_order_never_used_as_selector
        && result.no_enacted_classification_or_outcome_accepted_as_input
        && result.byte_identity_never_used_as_provenance_or_selector
        && !result.resource_limit_used_as_halt_claim
        && (result.branch_lawfully_disposed
            == !matches!(
                &result.outcome,
                Bi1bResumeOutcomeV1::ResourceLimitReached { .. }
            ))
        && result.branch_indexed_only
        && !result.bi2_finale_issued
        && !result.bi4_cone_verdict_issued
        && !result.uc1_scored
        && !result.unindexed_claim_issued;
    if !gate {
        return Err(BranchInvarianceResumeV1Error::Invariant(
            "branch resume failed the sealed-cone/classification-before-census boundary".to_owned(),
        ));
    }
    result.result_digest = result_digest(&result);
    Ok(result)
}

/// Resume one globally authorized branch at its already-sealed Stage-8 cone.
/// No current cone reissuance is admitted here: the typed gate proves this
/// certificate is one of the four globally replayed sealed members, while the
/// per-branch authorization binds its cone, root, and exact Stage-8 surface.
fn issue_branch_invariance_resume_v1_from_sealed_window(
    sealed: &Bi1BranchCertificateV3,
    authorization: &Bi1bSealedWindowAuthorizationV1,
    limits: &BranchContinuationLimits,
) -> Result<BranchInvarianceResumeV1, BranchInvarianceResumeV1Error> {
    let authorization_errors = replay_bi1b_sealed_window_authorization_v1(sealed, authorization);
    if !authorization_errors.is_empty() {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(format!(
            "sealed-window authorization did not replay: {}",
            authorization_errors.join("; ")
        )));
    }
    if limits.max_inspected_stage < BI1B_FIRST_RESUMED_STAGE
        || limits.max_enumerated_candidates_per_stage == 0
    {
        return Err(BranchInvarianceResumeV1Error::SealedWindow(
            "resume resource limits are empty or precede Stage 8".to_owned(),
        ));
    }
    let branch_seed = sealed.continuation.branch.clone();
    let prefix = reconstruct_branch_prefix_through_stage7(sealed)?;
    let sealed_stage8 = project_sealed_stage8_cone(sealed, &prefix)?;
    let initial_prefix_signature_digest = SealedSignature::from_telescopes(prefix.entries.clone())
        .digest()
        .to_owned();

    let mut winners = prefix.entries;
    let mut complete_ledger = prefix.ledger;
    let mut library: Library = Vec::new();
    let mut accepted_encodings = BTreeSet::new();
    for (_, telescope) in &winners {
        accepted_encodings.insert(exact_canonical_telescope_key(telescope));
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }
    let mut discovery_records = complete_ledger
        .iter()
        .map(|row| DiscoveryRecord::new(row.stage, row.semantic_nu, u32::from(row.kappa)))
        .collect::<Vec<_>>();
    let mut semantic_history = complete_ledger
        .iter()
        .map(|row| (row.stage, row.semantic_nu))
        .collect::<Vec<_>>();
    let mut stages = Vec::new();
    let mut stage = BI1B_FIRST_RESUMED_STAGE;

    loop {
        let debt = summarize_structural_debt(&library, 2);
        let required_packages = required_packages_for(debt)
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let signature = SealedSignature::from_telescopes(winners.clone());
        let demand = match demand_context(&signature, stage, required_packages.clone()) {
            Ok(demand) => demand,
            Err(exact_error) => {
                return finish(
                    branch_seed,
                    sealed,
                    authorization,
                    &sealed_stage8,
                    initial_prefix_signature_digest,
                    limits.clone(),
                    stages,
                    None,
                    Bi1bResumeOutcomeV1::ExpressivityGap {
                        stage,
                        phase: "demand_context".to_owned(),
                        exact_error,
                    },
                    complete_ledger,
                );
            }
        };
        if stage == BI1B_FIRST_RESUMED_STAGE && demand.audit != sealed_stage8.demand {
            return Err(BranchInvarianceResumeV1Error::SealedWindow(
                "fresh Stage-8 demand audit differs from the sealed BI-1 Stage-8 demand".to_owned(),
            ));
        }
        if demand.audit.debt_free {
            let terminal = demand.audit.clone();
            return finish(
                branch_seed,
                sealed,
                authorization,
                &sealed_stage8,
                initial_prefix_signature_digest,
                limits.clone(),
                stages,
                Some(terminal),
                Bi1bResumeOutcomeV1::DebtFreeHalt {
                    halt_stage: stage - 1,
                    next_stage: stage,
                },
                complete_ledger,
            );
        }
        if stage >= limits.max_inspected_stage {
            let terminal = demand.audit.clone();
            return finish(
                branch_seed,
                sealed,
                authorization,
                &sealed_stage8,
                initial_prefix_signature_digest,
                limits.clone(),
                stages,
                Some(terminal),
                Bi1bResumeOutcomeV1::ResourceLimitReached {
                    stage,
                    limit_kind: "max_inspected_stage".to_owned(),
                    configured_limit: u64::from(limits.max_inspected_stage),
                    observed: u64::from(stage),
                    required_packages,
                },
                complete_ledger,
            );
        }

        let (cone_origin, cone_enumerated, cone_admitted, candidates) =
            if stage == BI1B_FIRST_RESUMED_STAGE {
                (
                    Bi1bConeOriginV1::SealedBi1Stage8,
                    sealed_stage8.cone_enumerated,
                    sealed_stage8.cone_admitted,
                    sealed_stage8.candidates.clone(),
                )
            } else {
                let admissibility =
                    strict_admissibility_for_mode(stage, 2, &library, AdmissibilityMode::Guarded);
                let context = EnumerationContext::from_admissibility(&library, admissibility);
                let mut enumerated = Vec::new();
                for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
                    enumerated.extend(enumerate_telescopes(&library, context, kappa));
                }
                if enumerated.len() > limits.max_enumerated_candidates_per_stage {
                    let observed = enumerated.len() as u64;
                    let terminal = demand.audit.clone();
                    return finish(
                        branch_seed,
                        sealed,
                        authorization,
                        &sealed_stage8,
                        initial_prefix_signature_digest,
                        limits.clone(),
                        stages,
                        Some(terminal),
                        Bi1bResumeOutcomeV1::ResourceLimitReached {
                            stage,
                            limit_kind: "max_enumerated_candidates_per_stage".to_owned(),
                            configured_limit: limits.max_enumerated_candidates_per_stage as u64,
                            observed,
                            required_packages,
                        },
                        complete_ledger,
                    );
                }
                let cone_enumerated = enumerated.len();
                let admitted = enumerated
                    .into_iter()
                    .filter(|candidate| {
                        passes_strict_admissibility(stage, &library, candidate, admissibility)
                    })
                    .collect::<Vec<_>>();
                let cone_admitted = admitted.len();
                let mut encoded = admitted
                    .into_iter()
                    .map(|candidate| {
                        (
                            exact_canonical_telescope_key(&candidate),
                            serde_json::to_vec(&candidate)
                                .expect("enumerated candidate serializes deterministically"),
                            candidate,
                        )
                    })
                    .collect::<Vec<_>>();
                // Sort before quotienting so enumeration order cannot choose
                // the concrete representative of a canonical act.  The byte
                // tie-break is used only within one exact canonical encoding.
                encoded.sort_by(|left, right| (&left.0, &left.1).cmp(&(&right.0, &right.1)));
                encoded.dedup_by(|left, right| left.0 == right.0);
                // Preserve the collision-free canonical order after quotient.
                encoded.sort_by(|left, right| left.0.cmp(&right.0));
                let candidates = encoded
                    .into_iter()
                    .map(|(_, _, candidate)| candidate)
                    .collect::<Vec<_>>();
                (
                    Bi1bConeOriginV1::NewlyEnumeratedBranchWindow,
                    cone_enumerated,
                    cone_admitted,
                    candidates,
                )
            };
        let cone_deduped = candidates.len();
        let stage_cone_digest = if stage == BI1B_FIRST_RESUMED_STAGE {
            legacy_core_cone_digest(&candidates)
        } else {
            resume_cone_digest(&candidates)
        };
        if stage == BI1B_FIRST_RESUMED_STAGE
            && (cone_deduped != sealed_stage8.cone_deduped
                || stage_cone_digest != sealed_stage8.cone_digest)
        {
            return Err(BranchInvarianceResumeV1Error::Invariant(
                "Stage-8 sealed cone drifted after projection".to_owned(),
            ));
        }
        let diagnostic_bar = compute_bar(2, stage, &discovery_records).bar;
        let demand_capacity = demand_output_capacity(stage, &winners, &required_packages);
        let assessments = candidates
            .iter()
            .map(|candidate| {
                assess_candidate(
                    stage,
                    candidate,
                    &winners,
                    &signature,
                    &library,
                    &semantic_history,
                    &accepted_encodings,
                    diagnostic_bar,
                    demand_capacity,
                    &demand,
                    &authorization.extraction_procedure_digest,
                )
            })
            .collect::<Vec<_>>();
        let census = complete_census(&assessments);

        let Some(census_value) = census.clone() else {
            let exact_error = assessments
                .iter()
                .find(|assessment| !assessment.resolved_before_census)
                .map(|assessment| match &assessment.classification {
                    Bi1bCandidateClassificationV1::Unknown { exact_error, .. } => {
                        format!("candidate {}: {exact_error}", assessment.candidate_hash)
                    }
                    _ if !assessment.structural_semantic_gap_errors.is_empty() => format!(
                        "candidate {} structural gaps: {}",
                        assessment.candidate_hash,
                        assessment.structural_semantic_gap_errors.join("; ")
                    ),
                    _ => format!(
                        "candidate {} was not resolved before census",
                        assessment.candidate_hash
                    ),
                })
                .unwrap_or_else(|| {
                    "classification completeness failed without a witness".to_owned()
                });
            let terminal = demand.audit.clone();
            stages.push(stage_record(
                stage,
                cone_origin,
                cone_enumerated,
                cone_admitted,
                cone_deduped,
                stage_cone_digest,
                demand.audit,
                diagnostic_bar.to_string(),
                assessments,
                None,
                None,
            ));
            return finish(
                branch_seed,
                sealed,
                authorization,
                &sealed_stage8,
                initial_prefix_signature_digest,
                limits.clone(),
                stages,
                Some(terminal),
                Bi1bResumeOutcomeV1::ExpressivityGap {
                    stage,
                    phase: "candidate_classification".to_owned(),
                    exact_error,
                },
                complete_ledger,
            );
        };

        let unique_hash = census_value.unique_discharger_hash.clone();
        if unique_hash.is_none() {
            let outcome = if census_value.discharger_count == 0 {
                Bi1bResumeOutcomeV1::HaltedNoGuardedDischarger { stage }
            } else {
                Bi1bResumeOutcomeV1::HaltedMultipleGuardedDischargers {
                    stage,
                    count: census_value.discharger_count,
                }
            };
            let terminal = demand.audit.clone();
            stages.push(stage_record(
                stage,
                cone_origin,
                cone_enumerated,
                cone_admitted,
                cone_deduped,
                stage_cone_digest,
                demand.audit,
                diagnostic_bar.to_string(),
                assessments,
                Some(census_value),
                None,
            ));
            return finish(
                branch_seed,
                sealed,
                authorization,
                &sealed_stage8,
                initial_prefix_signature_digest,
                limits.clone(),
                stages,
                Some(terminal),
                outcome,
                complete_ledger,
            );
        }
        let unique_hash = unique_hash.expect("checked");
        let selected = assessments
            .iter()
            .find(|assessment| assessment.candidate_hash == unique_hash)
            .expect("complete census points into assessed cone");
        let r2_generated_action_evidence =
            match prove_r2_after_unique_discharge(stage, selected, &library, &signature) {
                Ok(rows) => rows,
                Err(exact_error) => {
                    let terminal = demand.audit.clone();
                    stages.push(stage_record(
                        stage,
                        cone_origin,
                        cone_enumerated,
                        cone_admitted,
                        cone_deduped,
                        stage_cone_digest,
                        demand.audit,
                        diagnostic_bar.to_string(),
                        assessments,
                        Some(census_value),
                        None,
                    ));
                    return finish(
                        branch_seed,
                        sealed,
                        authorization,
                        &sealed_stage8,
                        initial_prefix_signature_digest,
                        limits.clone(),
                        stages,
                        Some(terminal),
                        Bi1bResumeOutcomeV1::ExpressivityGap {
                            stage,
                            phase: "postselection_r2".to_owned(),
                            exact_error,
                        },
                        complete_ledger,
                    );
                }
            };
        let provenance = selected
            .classification
            .provenance()
            .expect("unique typed discharger has semantic provenance");
        let mut winner = Bi1bWinnerV1 {
            candidate_hash: selected.candidate_hash.clone(),
            canonical_encoding: selected.canonical_encoding.clone(),
            canonical_key: selected.canonical_key.clone(),
            telescope: selected.telescope.clone(),
            kappa: selected.kappa,
            semantic_nu: provenance.semantic_nu,
            provenance_derivation_hash: provenance.derivation_hash.clone(),
            selected_by: "unique_typed_total_discharger_after_complete_classification".to_owned(),
            diagnostic_clears_bar: selected.diagnostic_clears_bar,
            r2_candidate_action_check_applicable: stage == BI1B_FIRST_RESUMED_STAGE,
            r2_candidate_actions_replayed_and_cardinality_matches_provenance: stage
                == BI1B_FIRST_RESUMED_STAGE,
            r2_occurrence_to_action_injective_join_proved: false,
            r2_generated_action_evidence,
            derivation_hash: String::new(),
        };
        winner.derivation_hash = winner_digest(&winner);
        let winner_telescope = winner.telescope.clone();
        let winner_semantic_nu = winner.semantic_nu;
        let winner_kappa = winner.kappa;
        let winner_provenance_hash = winner.provenance_derivation_hash.clone();
        let winner_hash = winner.candidate_hash.clone();
        let winner_encoding = winner.canonical_encoding.clone();
        stages.push(stage_record(
            stage,
            cone_origin,
            cone_enumerated,
            cone_admitted,
            cone_deduped,
            stage_cone_digest,
            demand.audit,
            diagnostic_bar.to_string(),
            assessments,
            Some(census_value),
            Some(winner),
        ));
        accepted_encodings.insert(winner_encoding);
        discovery_records.push(DiscoveryRecord::new(
            stage,
            winner_semantic_nu,
            u32::from(winner_kappa),
        ));
        semantic_history.push((stage, winner_semantic_nu));
        library.push(LibraryEntry::from_telescope(&winner_telescope, &library));
        winners.push((stage, winner_telescope.clone()));
        complete_ledger.push(row_for_prefix(
            stage,
            winner_telescope,
            winner_semantic_nu,
            winner_provenance_hash,
        ));
        if complete_ledger
            .last()
            .is_none_or(|row| row.candidate_hash != winner_hash)
        {
            return Err(BranchInvarianceResumeV1Error::Invariant(
                "winner did not extend the branch ledger exactly".to_owned(),
            ));
        }
        stage = stage.checked_add(1).ok_or_else(|| {
            BranchInvarianceResumeV1Error::Continuation("stage counter overflowed".to_owned())
        })?;
    }
}

/// Resume one branch from its globally authorized sealed BI-1 certificate.
/// The current Stage-4 machinery is not reissued: BI-1b consumes the frozen
/// cone testimony after the row-free typed gate proves exact four-member
/// membership. No enacted certificate, classification, or outcome is an
/// argument.
pub fn issue_branch_invariance_resume_v1(
    sealed: &Bi1BranchCertificateV3,
    authorization: &Bi1bSealedWindowAuthorizationV1,
    limits: &BranchContinuationLimits,
) -> Result<BranchInvarianceResumeV1, BranchInvarianceResumeV1Error> {
    issue_branch_invariance_resume_v1_from_sealed_window(sealed, authorization, limits)
}

pub fn replay_branch_invariance_resume_v1(
    sealed: &Bi1BranchCertificateV3,
    authorization: &Bi1bSealedWindowAuthorizationV1,
    claimed: &BranchInvarianceResumeV1,
) -> BranchInvarianceResumeV1Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != result_digest(claimed) {
        errors.push("BI-1b branch-resume digest mismatch".to_owned());
    }
    for stage in &claimed.stages {
        if stage.derivation_hash != stage_digest(stage) {
            errors.push(format!("Stage {} digest mismatch", stage.stage));
        }
        for assessment in &stage.assessments {
            if assessment.derivation_hash != assessment_digest(assessment) {
                errors.push(format!(
                    "Stage {} candidate {} digest mismatch",
                    stage.stage, assessment.candidate_hash
                ));
            }
        }
        if stage
            .census
            .as_ref()
            .is_some_and(|census| census.derivation_hash != census_digest(census))
        {
            errors.push(format!("Stage {} census digest mismatch", stage.stage));
        }
        if stage
            .winner
            .as_ref()
            .is_some_and(|winner| winner.derivation_hash != winner_digest(winner))
        {
            errors.push(format!("Stage {} winner digest mismatch", stage.stage));
        }
    }
    for row in &claimed.complete_ledger {
        if row.derivation_hash != ledger_row_digest(row) {
            errors.push(format!("Stage {} ledger-row digest mismatch", row.stage));
        }
    }
    if claimed.enacted_regression_authorization_digest
        != authorization.enacted_regression_authorization_digest
        || claimed.sealed_window_authorization_digest != authorization.derivation_hash
        || claimed.authorized_extraction_procedure_digest
            != authorization.extraction_procedure_digest
    {
        errors.push("BI-1b resume authorization digest mismatch".to_owned());
    }
    match issue_branch_invariance_resume_v1(sealed, authorization, &claimed.limits) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => {
            errors.push("BI-1b branch resume differs from deterministic reissuance".to_owned())
        }
        Err(error) => errors.push(error.to_string()),
    }
    BranchInvarianceResumeV1Replay {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bi1b_does_not_drift_the_frozen_bi1_core_source_binding() {
        assert_eq!(
            format!(
                "blake3:{}",
                blake3_hex(include_bytes!("branch_invariance.rs"))
            ),
            "blake3:fc1ce77dec9200f2f1151f630808f7e939b80e5c06e69a49b5da5f4daeee552c"
        );
    }

    #[test]
    fn forged_extraction_procedure_digest_is_rejected() {
        let authority = issue_branch_prefix_general_provenance_v4_authority();
        assert!(extraction_procedure_authority_replays_exactly(
            &authority.derivation_hash
        ));
        assert!(!extraction_procedure_authority_replays_exactly(
            "blake3:0000000000000000000000000000000000000000000000000000000000000000"
        ));
    }

    #[test]
    fn all_four_sealed_stage8_rows_replay_in_the_legacy_core_hash_domain() {
        let certificates: [&[u8]; 4] = [
            include_bytes!("../../../docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json"),
            include_bytes!("../../../docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json"),
            include_bytes!("../../../docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json"),
            include_bytes!("../../../docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json"),
        ];
        for bytes in certificates {
            let sealed: Bi1BranchCertificateV3 =
                serde_json::from_slice(bytes).expect("sealed BI-1 branch certificate parses");
            let stage8 = sealed
                .continuation
                .stages
                .iter()
                .find(|stage| stage.stage == BI1B_FIRST_RESUMED_STAGE)
                .expect("sealed branch contains Stage 8");
            let candidates = stage8
                .assessments
                .iter()
                .map(|assessment| assessment.telescope.clone())
                .collect::<Vec<_>>();
            assert_eq!(legacy_core_cone_digest(&candidates), stage8.cone_digest);
            assert_eq!(
                legacy_core_demand_audit_digest(&stage8.demand),
                stage8.demand.derivation_hash
            );

            let genesis = SealedSignature::genesis_del_h15();
            let mut prefix_entries = (1..=3)
                .map(|stage| {
                    (
                        stage,
                        genesis
                            .entry(stage)
                            .expect("Genesis seal contains common stem")
                            .telescope
                            .clone(),
                    )
                })
                .collect::<Vec<_>>();
            prefix_entries.push((4, sealed.continuation.branch.telescope.clone()));
            for stage in 5..=7 {
                prefix_entries.push((
                    stage,
                    sealed
                        .continuation
                        .stages
                        .iter()
                        .find(|row| row.stage == stage)
                        .and_then(|row| row.winner.as_ref())
                        .expect("sealed branch contains its pre-Stage8 winner")
                        .telescope
                        .clone(),
                ));
            }
            assert!(prefix_entries.iter().map(|(stage, _)| *stage).eq(1..=7));
            let signature = SealedSignature::from_telescopes(prefix_entries.clone());
            let mut library: Library = Vec::new();
            for (_, telescope) in &prefix_entries {
                library.push(LibraryEntry::from_telescope(telescope, &library));
            }
            let coarse_required_packages =
                required_packages_for(summarize_structural_debt(&library, 2))
                    .into_iter()
                    .map(str::to_owned)
                    .collect::<Vec<_>>();
            let fresh = demand_context(
                &signature,
                BI1B_FIRST_RESUMED_STAGE,
                coarse_required_packages,
            )
            .expect("fresh exact-prefix Stage-8 demand audit closes");
            assert_eq!(fresh.audit, stage8.demand);
        }
    }
}
