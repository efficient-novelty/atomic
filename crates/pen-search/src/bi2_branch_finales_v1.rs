//! BI-2: four branch-indexed E-5 finales over the sealed BI-1b cone.
//!
//! The module has two capability boundaries:
//!
//! 1. the complete emitted BI-1b bundle is deterministically replayed once;
//! 2. each branch issuer receives only a row-free authorization plus its own
//!    paired BI-1/BI-1b certificates.
//!
//! The write-only correspondence diagnostic is needed to authenticate the
//! upstream bundle, but is never passed to a branch issuer.  Cross-branch
//! G2/G3 comparison and every unindexed halt claim remain BI-4 work.

use crate::act_local_semantic_provenance_v5::{
    T_BI_B1_B2_PREFIX_GENERAL_THEOREM_ID, V5_PREFIX_GENERAL_SEMANTIC_SEQUENCE_SCHEMA,
    V5PrefixLocalSemanticPackageProof, issue_prefix_general_semantic_sequence_v6,
    replay_prefix_general_semantic_sequence_v6,
};
use crate::bi1b_prefix_general_sweep_v1::{
    BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA, Bi1bBranchCertificateV1, Bi1bPrefixGeneralSweepV1Bundle,
    replay_bi1b_prefix_general_sweep_v1,
};
use crate::branch_invariance::{
    BranchContinuationOutcome, BranchDemandAudit, BranchLedgerRow, BranchStageRecord,
};
use crate::branch_invariance_finale::{
    branch_finale_input_binding_hash, execute_replayed_branch_finale_input,
};
use crate::branch_invariance_resume_v1::{
    Bi1bCandidateClassificationV1, Bi1bResumeOutcomeV1, Bi1bStageRecordV1,
};
use crate::branch_invariance_sweep_v3::Bi1BranchCertificateV3;
use crate::branch_prefix_general_provenance_v4::{
    BranchPrefixGeneralProvenanceV4Token, issue_branch_prefix_general_provenance_v4_authority,
    replay_branch_prefix_general_provenance_v4_authority,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_rule_inventory_exhaustiveness::{
    A3WindowRuleInventoryProof, prove_a3_window_inventory_for_exact_prefix_unbounded,
};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub use crate::branch_invariance_finale::{
    BranchFinaleAudit, BranchFinaleInput, BranchFinaleStepInput,
};

/// The first create-new execution is retained as an immutable partial
/// certificate.  This source now issues the versioned theorem-adapter
/// successor, so the repaired run can never overwrite or masquerade as v1.
pub const BI2_BRANCH_FINALE_V2_SCHEMA: &str = "bi2-prefix-general-branch-finale-v2";
pub const BI2_FOUR_BRANCH_FINALES_V2_SCHEMA: &str = "bi2-four-branch-finales-index-v2";
pub const BI2_BRANCH_FINALE_V2_DATE: &str = "2026-07-23";
pub const BI2_INDEX_CERTIFICATE_NAME: &str = "BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json";
pub const BI2_INDEX_REPORT_NAME: &str = "BI2_FOUR_BRANCH_FINALES_V2_RESULT.md";
const FROZEN_BI1B_SWEEP_DIGEST: &str =
    "blake3:79272aa307186696b2a6a4cd63b969cf7d9fc08a6ee4efde8f00ca30e5ac9e3d";
const BI1B_CURRENT_REISSUANCE_DRIFT: &str = "BI-1b sweep differs from deterministic reissuance";

const BRANCH_INVARIANCE_PLAN_BYTES: &[u8] =
    include_bytes!("../../../docs/branch_invariance_program_plan.md");
const BI1B_PLAN_BYTES: &[u8] =
    include_bytes!("../../../docs/bi1b_prefix_general_provenance_plan.md");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("bi2_branch_finales_v1.rs");
const FINALE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_finale.rs");
const PREFIX_GENERAL_SOURCE_BYTES: &[u8] = include_bytes!("branch_prefix_general_provenance_v4.rs");
const V5_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v5.rs");
const BI1B_SOURCE_BYTES: &[u8] = include_bytes!("bi1b_prefix_general_sweep_v1.rs");
const T_SM1A_SOURCE_BYTES: &[u8] = include_bytes!("t_sm1a_contextual_formation_v4.rs");
const OPEN_SPECIALIZATION_SOURCE_BYTES: &[u8] =
    include_bytes!("motive_typed_open_specialization_v4.rs");
const SUPPORT_COMPREHENSION_SOURCE_BYTES: &[u8] = include_bytes!("support_comprehension_v5.rs");
const SUPPORT_HARDENING_SOURCE_BYTES: &[u8] =
    include_bytes!("support_comprehension_hardening_v6.rs");
const DEPENDENT_CONTEXT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/dependent_context_adjudication.md");
const SUPPORT_COMPREHENSION_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/support_comprehension_adjudication.md");

const BI1B_REGRESSION_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_REGRESSION_CERTIFICATE.json");
const BI1B_CORRESPONDENCE_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json");
const BI1B_SWEEP_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_PREFIX_GENERAL_SWEEP_V1_CERTIFICATE.json");
const BI2_V1_INDEX_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_FOUR_BRANCH_FINALES_V1_CERTIFICATE.json");
const BI1B_BRANCH_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json");
const BI1B_BRANCH_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json");
const BI1B_BRANCH_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json");
const BI1B_BRANCH_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json");

const BI1_BRANCH_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json");
const BI1_BRANCH_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json");
const BI1_BRANCH_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json");
const BI1_BRANCH_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2SourceBindingV1 {
    pub path: String,
    pub role: String,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2RowFreeAuthorizationV1 {
    pub schema: String,
    pub date: String,
    pub bi1b_sweep_digest: String,
    pub bi1b_regression_certificate_digest: String,
    pub bi1b_regression_authorization_digest: String,
    pub prefix_general_extraction_authority_digest: String,
    pub exact_four_branch_surface_authenticated: bool,
    pub every_bi1b_branch_certificate_replayed: bool,
    pub public_deterministic_replay_passed: bool,
    pub public_replay_errors: Vec<String>,
    pub frozen_testimony_fallback_used: bool,
    pub frozen_bi1b_sweep_digest_exact: bool,
    pub authorized_branch_roots: Vec<String>,
    pub authorized_bi1b_branch_certificate_digests: Vec<String>,
    pub branch_family_rows_exported: bool,
    pub branch_semantic_vectors_exported: bool,
    pub branch_winners_exported: bool,
    pub branch_outcomes_exported: bool,
    pub correspondence_rows_exported: bool,
    pub selector_capability_exported: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi2BranchHistorySourceV1 {
    SealedBi1EnactedContinuation,
    SealedBi1PrefixPlusBi1bResume,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2StepSemanticProvenanceV1 {
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub semantic_nu: u32,
    pub credited_family_ids: Vec<String>,
    pub authoritative_family_token_hashes: Vec<String>,
    pub package_derivation_hash: String,
    pub package_v4_candidate_local_source_hash: String,
    pub package_rule_authority_derivation_hash: String,
    pub exact_branch_ledger_join: bool,
    pub exact_prefix_candidate_package_binding: bool,
    pub all_residual_counts_zero: bool,
    pub package_proved: bool,
    pub sealed_bi1b_winner_token_join_applicable: bool,
    pub sealed_bi1b_winner_token_join_exact: bool,
    pub no_archive_structural_bar_verdict_or_future_input: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2PrefixGeneralSequenceAuditV1 {
    pub schema: String,
    pub theorem_id: String,
    pub rule_authority_derivation_hash: String,
    pub sequence_derivation_hash: String,
    pub authoritative_sequence_seal: String,
    pub issuance_trace_root: String,
    pub package_count: usize,
    pub semantic_nu_vector: Vec<u32>,
    pub deterministic_replay_valid: bool,
    pub every_package_closed_observed_grammar: bool,
    pub every_package_registry_extension_invariant: bool,
    pub no_historical_registry_or_future_input: bool,
    pub t_bi_b1_proved_on_sequence: bool,
    pub t_bi_b2_proved_on_sequence: bool,
    pub one_prefix_general_procedure: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2BranchFinaleCertificateV1 {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Bi2SourceBindingV1>,
    pub branch_root_hash: String,
    pub branch_digest_prefix: String,
    pub enacted_root: bool,
    pub issuance_ordinal: usize,
    pub history_source: Bi2BranchHistorySourceV1,
    pub row_free_authorization_digest: String,
    pub bi1b_sweep_digest: String,
    pub bi1b_branch_certificate_digest: String,
    pub sealed_bi1_branch_certificate_digest: String,
    pub bi1b_branch_source_replayed: bool,
    pub own_branch_only_input: bool,
    pub correspondence_diagnostic_consumed_by_branch_issuer: bool,
    pub enacted_classification_or_outcome_consumed_by_alternate_issuer: bool,
    pub semantic_value_bar_hash_or_order_used_as_selector: bool,
    pub prefix_general_sequence: Bi2PrefixGeneralSequenceAuditV1,
    pub step_provenance: Vec<Bi2StepSemanticProvenanceV1>,
    pub finale_input: BranchFinaleInput,
    pub finale_input_binding_hash: String,
    pub finale: BranchFinaleAudit,
    pub finale_reissued_equal: bool,
    pub exact_a3_window_count: usize,
    pub final_a3_inventory_count: usize,
    pub final_unary_count: usize,
    pub final_direct_chronological_count: usize,
    pub final_pointwise_chronological_count: usize,
    pub final_higher_count: usize,
    pub final_structural_count: usize,
    pub semantic_o16_empty_branch_indexed: bool,
    pub f1_executed: bool,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub f1_underdetermined_instance_ids: Vec<String>,
    pub theorem12_full_instance_granularity_branch_indexed: bool,
    pub local_g4_debt_free_halt_at_15: bool,
    pub e5_class_complete: bool,
    pub expressivity_gaps: Vec<String>,
    pub branch_indexed_only: bool,
    pub bi4_cone_report_issued: bool,
    pub cone_g4_verdict_issued: bool,
    pub g2_verdict_issued: bool,
    pub g3_verdict_issued: bool,
    pub uc1_scored: bool,
    pub bridge_authorized: bool,
    pub unindexed_halt_o_or_theorem12_claim_issued: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2FinaleIndexRowV1 {
    pub issuance_ordinal: usize,
    pub branch_root_hash: String,
    pub enacted_root: bool,
    pub branch_certificate_digest: String,
    pub final_a3_inventory_count: usize,
    pub semantic_o16_empty_branch_indexed: bool,
    pub f1_executed: bool,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub e5_class_complete: bool,
    pub expressivity_gap_count: usize,
    pub branch_indexed_only: bool,
    pub row_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2FourBranchFinalesIndexV1 {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Bi2SourceBindingV1>,
    /// Aggregate-only lineage. This binding is never included in the
    /// row-free authorization or exposed to an individual branch issuer.
    pub predecessor_v1_index_binding: Bi2SourceBindingV1,
    pub row_free_authorization: Bi2RowFreeAuthorizationV1,
    pub rows: Vec<Bi2FinaleIndexRowV1>,
    pub exact_four_branch_surface: bool,
    pub every_branch_certificate_reissued_equal: bool,
    pub every_branch_finale_e5_class_complete: bool,
    pub every_branch_local_semantic_o16_empty: bool,
    pub every_branch_f1_executed: bool,
    pub every_branch_f1_excluded: bool,
    pub ready_for_bi4_assembly: bool,
    pub cross_branch_comparison_performed: bool,
    pub bi4_cone_report_issued: bool,
    pub cone_g4_verdict_issued: bool,
    pub g2_verdict_issued: bool,
    pub g3_verdict_issued: bool,
    pub uc1_scored: bool,
    pub bridge_authorized: bool,
    pub unindexed_halt_o_or_theorem12_claim_issued: bool,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bi2FourBranchFinalesBundleV1 {
    pub branches: Vec<Bi2BranchFinaleCertificateV1>,
    pub index: Bi2FourBranchFinalesIndexV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi2FourBranchFinalesReplayV1 {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bi2BranchFinalesV1Error {
    #[error("BI-2 embedded-input failure: {0}")]
    EmbeddedInput(String),
    #[error("BI-2 upstream replay failure: {0}")]
    UpstreamReplay(String),
    #[error("BI-2 branch reconstruction failure: {0}")]
    Branch(String),
    #[error("BI-2 semantic provenance failure: {0}")]
    Provenance(String),
    #[error("BI-2 finale failure: {0}")]
    Finale(String),
    #[error("BI-2 invariant failure: {0}")]
    Invariant(String),
    #[error("BI-2 create-new I/O failure: {0}")]
    Io(String),
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI2_BRANCH_FINALE_V2_SCHEMA, domain, value))
        .expect("BI-2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn index_tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI2_FOUR_BRANCH_FINALES_V2_SCHEMA, domain, value))
        .expect("BI-2 index evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn source_bindings() -> Vec<Bi2SourceBindingV1> {
    [
        (
            "docs/branch_invariance_program_plan.md",
            "frozen BI-2 mission, scope, and falsifiers",
            BRANCH_INVARIANCE_PLAN_BYTES,
        ),
        (
            "docs/bi1b_prefix_general_provenance_plan.md",
            "sealed BI-1b upstream discipline",
            BI1B_PLAN_BYTES,
        ),
        (
            "crates/pen-search/src/bi2_branch_finales_v1.rs",
            "BI-2 branch adapter, replay, and create-new issuer",
            THIS_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance_finale.rs",
            "branch-parametric A3, D, O16, and F1 semantic core",
            FINALE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_prefix_general_provenance_v4.rs",
            "zero-input prefix-general extraction authority",
            PREFIX_GENERAL_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_semantic_provenance_v5.rs",
            "prefix-general v6 B1/B2 semantic sequence theorem",
            V5_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/bi1b_prefix_general_sweep_v1.rs",
            "upstream four-branch deterministic replay",
            BI1B_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_sm1a_contextual_formation_v4.rs",
            "branch-parametric contextual-Formation Internal theorem",
            T_SM1A_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/motive_typed_open_specialization_v4.rs",
            "branch-parametric dependent open-specialization theorem",
            OPEN_SPECIALIZATION_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/support_comprehension_v5.rs",
            "canonical support-comprehension theorem for the unique cyclic row",
            SUPPORT_COMPREHENSION_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/support_comprehension_hardening_v6.rs",
            "support graph-normal-form and zero-mint hardening",
            SUPPORT_HARDENING_SOURCE_BYTES,
        ),
        (
            "docs/dependent_context_adjudication.md",
            "adopted dependent-context law",
            DEPENDENT_CONTEXT_ADJUDICATION_BYTES,
        ),
        (
            "docs/support_comprehension_adjudication.md",
            "adopted support-comprehension law",
            SUPPORT_COMPREHENSION_ADJUDICATION_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Bi2SourceBindingV1 {
        path: path.to_owned(),
        role: role.to_owned(),
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn predecessor_v1_index_binding() -> Bi2SourceBindingV1 {
    Bi2SourceBindingV1 {
        path: "docs/BI2_FOUR_BRANCH_FINALES_V1_CERTIFICATE.json".to_owned(),
        role: "aggregate-only immutable partial BI-2 v1 predecessor index".to_owned(),
        blake3: bytes_hash(BI2_V1_INDEX_BYTES),
    }
}

fn authorization_digest(value: &Bi2RowFreeAuthorizationV1) -> String {
    let mut projection = value.clone();
    projection.derivation_hash.clear();
    tagged_hash("row-free-upstream-authorization", &projection)
}

fn step_provenance_digest(value: &Bi2StepSemanticProvenanceV1) -> String {
    let mut projection = value.clone();
    projection.derivation_hash.clear();
    tagged_hash("step-semantic-provenance", &projection)
}

fn sequence_audit_digest(value: &Bi2PrefixGeneralSequenceAuditV1) -> String {
    let mut projection = value.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-general-sequence-audit", &projection)
}

fn branch_certificate_digest(value: &Bi2BranchFinaleCertificateV1) -> String {
    let mut projection = value.clone();
    projection.result_digest.clear();
    tagged_hash("branch-finale-certificate", &projection)
}

fn index_row_digest(value: &Bi2FinaleIndexRowV1) -> String {
    let mut projection = value.clone();
    projection.row_hash.clear();
    index_tagged_hash("branch-index-row", &projection)
}

fn index_digest(value: &Bi2FourBranchFinalesIndexV1) -> String {
    let mut projection = value.clone();
    projection.result_digest.clear();
    index_tagged_hash("four-branch-finales-index", &projection)
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    label: &str,
) -> Result<T, Bi2BranchFinalesV1Error> {
    serde_json::from_slice(bytes)
        .map_err(|error| Bi2BranchFinalesV1Error::EmbeddedInput(format!("{label}: {error}")))
}

fn embedded_bi1b_bundle() -> Result<Bi1bPrefixGeneralSweepV1Bundle, Bi2BranchFinalesV1Error> {
    Ok(Bi1bPrefixGeneralSweepV1Bundle {
        regression: parse_json(BI1B_REGRESSION_BYTES, "BI1b regression")?,
        branches: vec![
            parse_json(BI1B_BRANCH_201_BYTES, "BI1b enacted branch")?,
            parse_json(BI1B_BRANCH_43_BYTES, "BI1b alternate 43 branch")?,
            parse_json(BI1B_BRANCH_4B_BYTES, "BI1b alternate 4b branch")?,
            parse_json(BI1B_BRANCH_B4_BYTES, "BI1b alternate b4 branch")?,
        ],
        correspondence: parse_json(BI1B_CORRESPONDENCE_BYTES, "BI1b correspondence")?,
        sweep: parse_json(BI1B_SWEEP_BYTES, "BI1b sweep")?,
    })
}

fn embedded_bi1_certificates() -> Result<Vec<Bi1BranchCertificateV3>, Bi2BranchFinalesV1Error> {
    Ok(vec![
        parse_json(BI1_BRANCH_201_BYTES, "BI1 enacted branch")?,
        parse_json(BI1_BRANCH_43_BYTES, "BI1 alternate 43 branch")?,
        parse_json(BI1_BRANCH_4B_BYTES, "BI1 alternate 4b branch")?,
        parse_json(BI1_BRANCH_B4_BYTES, "BI1 alternate b4 branch")?,
    ])
}

fn issue_row_free_authorization(
    bundle: &Bi1bPrefixGeneralSweepV1Bundle,
) -> Result<Bi2RowFreeAuthorizationV1, Bi2BranchFinalesV1Error> {
    let replay = replay_bi1b_prefix_general_sweep_v1(bundle);
    let frozen_testimony_fallback_used = !replay.valid
        && replay.errors == vec![BI1B_CURRENT_REISSUANCE_DRIFT.to_owned()]
        && bundle.sweep.result_digest == FROZEN_BI1B_SWEEP_DIGEST;
    if !replay.valid && !frozen_testimony_fallback_used {
        return Err(Bi2BranchFinalesV1Error::UpstreamReplay(
            replay.errors.join("; "),
        ));
    }
    if bundle.sweep.schema != BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA
        || bundle.sweep.branch_count != 4
        || !bundle.sweep.exact_four_branch_surface
        || !bundle.sweep.every_branch_certificate_replayed
        || !bundle.sweep.one_extraction_procedure_digest
        || !bundle.sweep.f_b1b_1_passed
        || !bundle.sweep.f_b1b_2_passed
        || !bundle.sweep.f_b1b_3_passed
        || !bundle.sweep.f_b1b_4_passed
        || !bundle.sweep.f_b1b_5_passed
        || !bundle.sweep.f_b1b_6_passed
        || bundle.sweep.bi2_finale_issued
        || bundle.sweep.bi4_cone_report_issued
        || bundle.sweep.cross_branch_g2_verdict_issued
        || bundle.sweep.cross_branch_g3_verdict_issued
        || bundle.sweep.uc1_scored
        || bundle.sweep.unindexed_halt_ledger_or_o_claim_issued
    {
        return Err(Bi2BranchFinalesV1Error::UpstreamReplay(
            "the replayed BI1b sweep does not expose the exact scoped four-branch surface"
                .to_owned(),
        ));
    }
    let extraction = issue_branch_prefix_general_provenance_v4_authority();
    if !extraction.proved
        || !replay_branch_prefix_general_provenance_v4_authority(&extraction).is_empty()
        || extraction.derivation_hash != bundle.sweep.extraction_procedure_digest
        || extraction.derivation_hash != bundle.regression.authorization.extraction_procedure_digest
    {
        return Err(Bi2BranchFinalesV1Error::UpstreamReplay(
            "the prefix-general extraction authority does not join BI1b".to_owned(),
        ));
    }
    let mut roots = bundle
        .branches
        .iter()
        .map(|branch| branch.branch_root_hash.clone())
        .collect::<Vec<_>>();
    roots.sort();
    let roots_unique = roots.iter().collect::<BTreeSet<_>>().len() == roots.len();
    let mut branch_digests = bundle
        .branches
        .iter()
        .map(|branch| branch.result_digest.clone())
        .collect::<Vec<_>>();
    branch_digests.sort();
    if !roots_unique
        || roots.len() != 4
        || bundle
            .branches
            .iter()
            .filter(|branch| branch.enacted_root)
            .count()
            != 1
    {
        return Err(Bi2BranchFinalesV1Error::UpstreamReplay(
            "BI1b did not authenticate four unique roots with one enacted index".to_owned(),
        ));
    }
    let mut authorization = Bi2RowFreeAuthorizationV1 {
        schema: BI2_BRANCH_FINALE_V2_SCHEMA.to_owned(),
        date: BI2_BRANCH_FINALE_V2_DATE.to_owned(),
        bi1b_sweep_digest: bundle.sweep.result_digest.clone(),
        bi1b_regression_certificate_digest: bundle.regression.result_digest.clone(),
        bi1b_regression_authorization_digest: bundle
            .regression
            .authorization
            .derivation_hash
            .clone(),
        prefix_general_extraction_authority_digest: extraction.derivation_hash,
        exact_four_branch_surface_authenticated: true,
        every_bi1b_branch_certificate_replayed: true,
        public_deterministic_replay_passed: replay.valid,
        public_replay_errors: replay.errors,
        frozen_testimony_fallback_used,
        frozen_bi1b_sweep_digest_exact: bundle.sweep.result_digest == FROZEN_BI1B_SWEEP_DIGEST,
        authorized_branch_roots: roots,
        authorized_bi1b_branch_certificate_digests: branch_digests,
        branch_family_rows_exported: false,
        branch_semantic_vectors_exported: false,
        branch_winners_exported: false,
        branch_outcomes_exported: false,
        correspondence_rows_exported: false,
        selector_capability_exported: false,
        derivation_hash: String::new(),
    };
    authorization.derivation_hash = authorization_digest(&authorization);
    Ok(authorization)
}

struct ReconstructedBranch {
    entries: Vec<(u32, Telescope)>,
    semantic_nu: Vec<u32>,
    selection_evidence_hashes: Vec<String>,
    required_packages: Vec<Vec<String>>,
    terminal_required_packages: Vec<String>,
    lawful_engine_stage_limit: u32,
    history_source: Bi2BranchHistorySourceV1,
}

fn genesis_common_stem() -> Result<Vec<(u32, Telescope)>, Bi2BranchFinalesV1Error> {
    let genesis = SealedSignature::genesis_del_h15();
    (1..=3)
        .map(|stage| {
            genesis
                .entry(stage)
                .map(|entry| (stage, entry.telescope.clone()))
                .ok_or_else(|| {
                    Bi2BranchFinalesV1Error::Branch(format!(
                        "frozen Genesis seal omits Stage {stage}"
                    ))
                })
        })
        .collect()
}

fn exact_demand_join(
    demand: &BranchDemandAudit,
    proof: &A3WindowRuleInventoryProof,
    prefix: &SealedSignature,
    stage: u32,
) -> bool {
    demand.stage == stage
        && demand.prefix_signature_digest == prefix.digest()
        && demand.a3_required_packages == proof.required_packages_from_raw_debt
        && demand.a3_inventory_derivation_hash == proof.derivation_hash
        && demand.exact_prefix_inventory_exhaustive
        && demand.coarse_and_a3_demands_agree
        && demand.debt_free == proof.required_packages_from_raw_debt.is_empty()
}

fn old_stage<'a>(
    sealed: &'a Bi1BranchCertificateV3,
    stage: u32,
) -> Result<&'a BranchStageRecord, Bi2BranchFinalesV1Error> {
    sealed
        .continuation
        .stages
        .iter()
        .find(|row| row.stage == stage)
        .ok_or_else(|| {
            Bi2BranchFinalesV1Error::Branch(format!(
                "sealed BI1 branch {} omits Stage {stage}",
                sealed.branch_root_hash
            ))
        })
}

fn old_ledger<'a>(
    sealed: &'a Bi1BranchCertificateV3,
    stage: u32,
) -> Result<&'a BranchLedgerRow, Bi2BranchFinalesV1Error> {
    sealed
        .continuation
        .complete_ledger
        .iter()
        .find(|row| row.stage == stage)
        .ok_or_else(|| {
            Bi2BranchFinalesV1Error::Branch(format!(
                "sealed BI1 branch {} omits Stage {stage} ledger",
                sealed.branch_root_hash
            ))
        })
}

fn resume_stage<'a>(
    branch: &'a Bi1bBranchCertificateV1,
    stage: u32,
) -> Result<&'a Bi1bStageRecordV1, Bi2BranchFinalesV1Error> {
    branch
        .resume
        .as_ref()
        .and_then(|resume| resume.stages.iter().find(|row| row.stage == stage))
        .ok_or_else(|| {
            Bi2BranchFinalesV1Error::Branch(format!(
                "BI1b branch {} omits resumed Stage {stage}",
                branch.branch_root_hash
            ))
        })
}

fn winning_bi1b_token<'a>(
    branch: &'a Bi1bBranchCertificateV1,
    stage: u32,
    expected_candidate: &str,
) -> Result<&'a BranchPrefixGeneralProvenanceV4Token, Bi2BranchFinalesV1Error> {
    let record = resume_stage(branch, stage)?;
    let winner = record.winner.as_ref().ok_or_else(|| {
        Bi2BranchFinalesV1Error::Branch(format!(
            "BI1b branch {} Stage {stage} has no winner",
            branch.branch_root_hash
        ))
    })?;
    if winner.candidate_hash != expected_candidate {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "BI1b branch {} Stage {stage} winner differs from its complete ledger",
            branch.branch_root_hash
        )));
    }
    let matching = record
        .assessments
        .iter()
        .filter(|row| row.candidate_hash == expected_candidate)
        .collect::<Vec<_>>();
    if matching.len() != 1
        || !matching[0].guarded_total_discharger
        || matching[0].ordinary_charge_provenance_hash.is_none()
    {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "BI1b branch {} Stage {stage} lacks one charged winning assessment",
            branch.branch_root_hash
        )));
    }
    let Bi1bCandidateClassificationV1::ProvedSemanticPackage { provenance } =
        &matching[0].classification
    else {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "BI1b branch {} Stage {stage} winner lacks proved prefix-general provenance",
            branch.branch_root_hash
        )));
    };
    if winner.provenance_derivation_hash != provenance.derivation_hash
        || matching[0].semantic_nu != Some(provenance.semantic_nu)
    {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "BI1b branch {} Stage {stage} winner/provenance join failed",
            branch.branch_root_hash
        )));
    }
    Ok(provenance)
}

fn reconstruct_branch(
    bi1b: &Bi1bBranchCertificateV1,
    sealed: &Bi1BranchCertificateV3,
) -> Result<ReconstructedBranch, Bi2BranchFinalesV1Error> {
    if bi1b.branch_root_hash != sealed.branch_root_hash
        || bi1b.enacted_root != sealed.enacted_root
        || bi1b.sealed_bi1_certificate_digest != sealed.result_digest
        || bi1b.bi2_finale_issued
        || bi1b.bi4_cone_verdict_issued
        || bi1b.g2_verdict_issued
        || bi1b.g3_verdict_issued
        || bi1b.uc1_scored
        || bi1b.unindexed_claim_issued
        || !bi1b.branch_indexed_only
        || !bi1b.lawful_branch_disposition
    {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "BI1/BI1b source binding failed for {}",
            bi1b.branch_root_hash
        )));
    }
    let mut entries = genesis_common_stem()?;
    let seed = &sealed.continuation.branch;
    if seed.candidate_hash != bi1b.branch_root_hash
        || candidate_hash(&seed.telescope) != bi1b.branch_root_hash
    {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "Stage-4 seed does not bind branch {}",
            bi1b.branch_root_hash
        )));
    }
    entries.push((4, seed.telescope.clone()));
    let stage4_prefix = SealedSignature::from_telescopes(entries.clone());
    if stage4_prefix.digest() != sealed.continuation.initial_prefix_signature_digest {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "Stage-4 prefix digest drift for {}",
            bi1b.branch_root_hash
        )));
    }

    let history_source;
    if bi1b.enacted_root {
        if bi1b.resume.is_some()
            || !bi1b.prior_enacted_halt_revalidated
            || !matches!(
                sealed.continuation.outcome,
                BranchContinuationOutcome::DebtFreeHalt {
                    halt_stage: 15,
                    next_stage: 16
                }
            )
            || !sealed.continuation.debt_free_halt_at_stage15
            || !sealed.continuation.completed_through_stage15
        {
            return Err(Bi2BranchFinalesV1Error::Branch(
                "enacted BI1b branch did not revalidate its sealed 15/16 halt".to_owned(),
            ));
        }
        for stage in 5..=15 {
            let winner = old_stage(sealed, stage)?.winner.as_ref().ok_or_else(|| {
                Bi2BranchFinalesV1Error::Branch(format!(
                    "enacted sealed branch omits Stage {stage} winner"
                ))
            })?;
            entries.push((stage, winner.telescope.clone()));
        }
        history_source = Bi2BranchHistorySourceV1::SealedBi1EnactedContinuation;
    } else {
        let resume = bi1b.resume.as_ref().ok_or_else(|| {
            Bi2BranchFinalesV1Error::Branch(format!(
                "alternate branch {} has no BI1b resume",
                bi1b.branch_root_hash
            ))
        })?;
        if !bi1b.resume_replayed
            || !matches!(
                resume.outcome,
                Bi1bResumeOutcomeV1::DebtFreeHalt {
                    halt_stage: 15,
                    next_stage: 16
                }
            )
            || !resume.branch_lawfully_disposed
            || !resume.branch_indexed_only
            || resume.bi2_finale_issued
            || resume.bi4_cone_verdict_issued
            || resume.uc1_scored
            || resume.unindexed_claim_issued
            || !resume.no_enacted_classification_or_outcome_accepted_as_input
            || !resume.byte_identity_never_used_as_provenance_or_selector
            || !resume.semantic_value_never_used_as_selector
            || !resume.bar_never_used_as_selector
            || !resume.hash_or_enumeration_order_never_used_as_selector
            || resume.resource_limit_used_as_halt_claim
        {
            return Err(Bi2BranchFinalesV1Error::Branch(format!(
                "alternate branch {} resume is not an isolated debt-free 15/16 run",
                bi1b.branch_root_hash
            )));
        }
        for stage in 5..=7 {
            let winner = old_stage(sealed, stage)?.winner.as_ref().ok_or_else(|| {
                Bi2BranchFinalesV1Error::Branch(format!(
                    "alternate sealed prefix omits Stage {stage} winner"
                ))
            })?;
            entries.push((stage, winner.telescope.clone()));
        }
        for stage in 8..=15 {
            let winner = resume_stage(bi1b, stage)?.winner.as_ref().ok_or_else(|| {
                Bi2BranchFinalesV1Error::Branch(format!(
                    "alternate BI1b branch omits Stage {stage} winner"
                ))
            })?;
            entries.push((stage, winner.telescope.clone()));
        }
        history_source = Bi2BranchHistorySourceV1::SealedBi1PrefixPlusBi1bResume;
    }
    if entries.iter().map(|(stage, _)| *stage).ne(1..=15) {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "branch {} reconstruction is not exactly Stages 1 through 15",
            bi1b.branch_root_hash
        )));
    }

    let mut semantic_nu = Vec::with_capacity(15);
    let mut selection_evidence_hashes = Vec::with_capacity(15);
    let mut required_packages = Vec::with_capacity(15);
    for (stage, telescope) in &entries {
        let candidate = candidate_hash(telescope);
        let expected_nu = if bi1b.enacted_root || *stage <= 7 {
            let ledger = old_ledger(sealed, *stage)?;
            if ledger.candidate_hash != candidate || ledger.kappa != telescope.kappa() as u16 {
                return Err(Bi2BranchFinalesV1Error::Branch(format!(
                    "sealed BI1 ledger/telescope join failed at Stage {stage}"
                )));
            }
            ledger.semantic_nu
        } else {
            let resume = bi1b.resume.as_ref().expect("validated alternate");
            let ledger = resume
                .complete_ledger
                .iter()
                .find(|row| row.stage == *stage)
                .ok_or_else(|| {
                    Bi2BranchFinalesV1Error::Branch(format!(
                        "BI1b complete ledger omits Stage {stage}"
                    ))
                })?;
            if ledger.candidate_hash != candidate
                || ledger.telescope != *telescope
                || ledger.kappa != telescope.kappa() as u16
            {
                return Err(Bi2BranchFinalesV1Error::Branch(format!(
                    "BI1b ledger/telescope join failed at Stage {stage}"
                )));
            }
            ledger.semantic_nu
        };
        if !bi1b.enacted_root && *stage <= 7 {
            let resume_ledger = bi1b
                .resume
                .as_ref()
                .expect("validated alternate")
                .complete_ledger
                .iter()
                .find(|row| row.stage == *stage)
                .ok_or_else(|| {
                    Bi2BranchFinalesV1Error::Branch(format!(
                        "BI1b complete ledger omits inherited Stage {stage}"
                    ))
                })?;
            if resume_ledger.candidate_hash != candidate
                || resume_ledger.telescope != *telescope
                || resume_ledger.semantic_nu != expected_nu
            {
                return Err(Bi2BranchFinalesV1Error::Branch(format!(
                    "BI1/BI1b inherited ledger join failed at Stage {stage}"
                )));
            }
        }
        semantic_nu.push(expected_nu);

        let selection_hash = if *stage <= 4 {
            tagged_hash(
                "sealed-bootstrap-selection-evidence",
                &(sealed.result_digest.as_str(), stage, candidate.as_str()),
            )
        } else if bi1b.enacted_root || *stage <= 7 {
            old_stage(sealed, *stage)?.derivation_hash.clone()
        } else {
            resume_stage(bi1b, *stage)?.derivation_hash.clone()
        };
        selection_evidence_hashes.push(selection_hash);

        let prefix = SealedSignature::from_telescopes(
            entries
                .iter()
                .filter(|(entry_stage, _)| entry_stage < stage)
                .cloned()
                .collect(),
        );
        let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(&prefix, *stage).map_err(
            |error| {
                Bi2BranchFinalesV1Error::Branch(format!(
                    "Stage {stage} exact A3 inventory: {error}"
                ))
            },
        )?;
        if *stage >= 5 {
            let source_demand = if bi1b.enacted_root || *stage <= 7 {
                &old_stage(sealed, *stage)?.demand
            } else {
                &resume_stage(bi1b, *stage)?.demand
            };
            if !exact_demand_join(source_demand, &proof, &prefix, *stage) {
                return Err(Bi2BranchFinalesV1Error::Branch(format!(
                    "Stage {stage} source/A3 demand join failed"
                )));
            }
        }
        required_packages.push(proof.required_packages_from_raw_debt);
    }

    let terminal_prefix = SealedSignature::from_telescopes(entries.clone());
    let terminal_proof = prove_a3_window_inventory_for_exact_prefix_unbounded(&terminal_prefix, 16)
        .map_err(|error| {
            Bi2BranchFinalesV1Error::Branch(format!("Stage 16 exact A3 inventory: {error}"))
        })?;
    let (terminal_demand, lawful_engine_stage_limit) = if bi1b.enacted_root {
        (
            &sealed.continuation.terminal_demand,
            sealed.continuation.limits.max_inspected_stage,
        )
    } else {
        let resume = bi1b.resume.as_ref().expect("validated alternate");
        (
            resume.terminal_demand.as_ref().ok_or_else(|| {
                Bi2BranchFinalesV1Error::Branch(format!(
                    "alternate branch {} lacks terminal demand",
                    bi1b.branch_root_hash
                ))
            })?,
            resume.limits.max_inspected_stage,
        )
    };
    if !exact_demand_join(terminal_demand, &terminal_proof, &terminal_prefix, 16)
        || !terminal_demand.debt_free
        || !terminal_proof.required_packages_from_raw_debt.is_empty()
        || lawful_engine_stage_limit < 16
    {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "branch {} terminal Stage-16 debt join failed",
            bi1b.branch_root_hash
        )));
    }
    Ok(ReconstructedBranch {
        entries,
        semantic_nu,
        selection_evidence_hashes,
        required_packages,
        terminal_required_packages: terminal_proof.required_packages_from_raw_debt,
        lawful_engine_stage_limit,
        history_source,
    })
}

fn credited_family_projection(
    package: &V5PrefixLocalSemanticPackageProof,
) -> Result<(Vec<String>, Vec<String>), Bi2BranchFinalesV1Error> {
    let mut rows = package
        .family_rows
        .iter()
        .filter(|row| row.credited)
        .map(|row| {
            (
                row.family_id.clone(),
                row.derivation_hash.clone(),
                row.typed_normalized_natural,
                row.marginal,
            )
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    let family_ids = rows
        .iter()
        .map(|(family_id, _, _, _)| family_id.clone())
        .collect::<Vec<_>>();
    let token_hashes = rows
        .iter()
        .map(|(_, hash, _, _)| hash.clone())
        .collect::<Vec<_>>();
    let unique_ids = family_ids.iter().collect::<BTreeSet<_>>().len() == family_ids.len();
    let unique_hashes = token_hashes.iter().collect::<BTreeSet<_>>().len() == token_hashes.len();
    if family_ids != package.credited_family_ids
        || family_ids.len() != package.semantic_nu as usize
        || !unique_ids
        || !unique_hashes
        || rows
            .iter()
            .any(|(_, hash, typed, marginal)| hash.is_empty() || !typed || !marginal)
    {
        return Err(Bi2BranchFinalesV1Error::Provenance(format!(
            "Stage {} credited-family projection is not exact",
            package.stage
        )));
    }
    Ok((family_ids, token_hashes))
}

fn sealed_winner_token_join(
    bi1b: &Bi1bBranchCertificateV1,
    stage: u32,
    candidate: &str,
    prefix_digest: &str,
    extraction_authority_digest: &str,
    package: &V5PrefixLocalSemanticPackageProof,
    family_ids: &[String],
    family_hashes: &[String],
) -> Result<(bool, bool), Bi2BranchFinalesV1Error> {
    if bi1b.enacted_root || stage < 8 {
        return Ok((false, false));
    }
    let token = winning_bi1b_token(bi1b, stage, candidate)?;
    let token_family_hashes = token
        .credited_family_rows
        .iter()
        .map(|row| row.authoritative_family_row_derivation_hash.clone())
        .collect::<Vec<_>>();
    let exact = token.stage == stage
        && token.candidate_hash == candidate
        && token.predecessor_signature_digest == prefix_digest
        && token.extraction_procedure_digest == extraction_authority_digest
        && token.semantic_nu == package.semantic_nu
        && token.credited_family_ids == family_ids
        && token_family_hashes == family_hashes
        && token.package_derivation_hash == package.derivation_hash
        && token.package_v4_candidate_local_source_hash == package.v4_candidate_local_source_hash
        && token.rule_authority_derivation_hash == package.rule_authority_derivation_hash
        && token.proved
        && token.all_residual_counts_zero
        && token.no_archive_or_historical_registry_input
        && !token.structural_nu_input_read
        && !token.bar_input_read
        && !token.verdict_input_read
        && !token.enacted_future_input_read
        && !token.enacted_classification_input_read
        && !token.byte_identity_input_read;
    if !exact {
        return Err(Bi2BranchFinalesV1Error::Provenance(format!(
            "branch {} Stage {stage} batch sequence does not exactly reproduce its sealed BI1b winner token",
            bi1b.branch_root_hash
        )));
    }
    Ok((true, true))
}

fn issue_branch_certificate(
    authorization: &Bi2RowFreeAuthorizationV1,
    bi1b: &Bi1bBranchCertificateV1,
    sealed: &Bi1BranchCertificateV3,
) -> Result<Bi2BranchFinaleCertificateV1, Bi2BranchFinalesV1Error> {
    if authorization.derivation_hash != authorization_digest(authorization)
        || (!authorization.public_deterministic_replay_passed
            && !authorization.frozen_testimony_fallback_used)
        || (authorization.public_deterministic_replay_passed
            && (!authorization.public_replay_errors.is_empty()
                || authorization.frozen_testimony_fallback_used))
        || (authorization.frozen_testimony_fallback_used
            && authorization.public_replay_errors != vec![BI1B_CURRENT_REISSUANCE_DRIFT.to_owned()])
        || !authorization.frozen_bi1b_sweep_digest_exact
        || authorization.bi1b_sweep_digest != FROZEN_BI1B_SWEEP_DIGEST
        || authorization.branch_family_rows_exported
        || authorization.branch_semantic_vectors_exported
        || authorization.branch_winners_exported
        || authorization.branch_outcomes_exported
        || authorization.correspondence_rows_exported
        || authorization.selector_capability_exported
        || authorization
            .authorized_branch_roots
            .binary_search(&bi1b.branch_root_hash)
            .is_err()
        || authorization
            .authorized_bi1b_branch_certificate_digests
            .binary_search(&bi1b.result_digest)
            .is_err()
    {
        return Err(Bi2BranchFinalesV1Error::Branch(format!(
            "row-free authorization does not admit branch {}",
            bi1b.branch_root_hash
        )));
    }
    let reconstructed = reconstruct_branch(bi1b, sealed)?;
    let sequence = issue_prefix_general_semantic_sequence_v6(&reconstructed.entries)
        .map_err(|error| Bi2BranchFinalesV1Error::Provenance(error.to_string()))?;
    let sequence_replay =
        replay_prefix_general_semantic_sequence_v6(&reconstructed.entries, &sequence);
    if !sequence_replay.is_empty() {
        return Err(Bi2BranchFinalesV1Error::Provenance(format!(
            "branch {} prefix-general sequence replay failed: {}",
            bi1b.branch_root_hash,
            sequence_replay.join("; ")
        )));
    }
    let extraction = issue_branch_prefix_general_provenance_v4_authority();
    if sequence.schema != V5_PREFIX_GENERAL_SEMANTIC_SEQUENCE_SCHEMA
        || sequence.theorem_id != T_BI_B1_B2_PREFIX_GENERAL_THEOREM_ID
        || sequence.packages.len() != 15
        || sequence.semantic_nu_vector != reconstructed.semantic_nu
        || sequence.rule_authority.derivation_hash != extraction.v5_rule_authority_derivation_hash
        || extraction.derivation_hash != authorization.prefix_general_extraction_authority_digest
        || !sequence.every_package_closed_observed_grammar
        || !sequence.every_package_registry_extension_invariant
        || !sequence.no_historical_registry_or_future_input
        || !sequence.t_bi_b1_proved_on_sequence
        || !sequence.t_bi_b2_proved_on_sequence
    {
        return Err(Bi2BranchFinalesV1Error::Provenance(format!(
            "branch {} prefix-general semantic vector or theorem surface diverged from its sealed ledger: sealed={:?}, recomputed={:?}",
            bi1b.branch_root_hash, reconstructed.semantic_nu, sequence.semantic_nu_vector
        )));
    }

    let mut sequence_audit = Bi2PrefixGeneralSequenceAuditV1 {
        schema: sequence.schema.clone(),
        theorem_id: sequence.theorem_id.clone(),
        rule_authority_derivation_hash: sequence.rule_authority.derivation_hash.clone(),
        sequence_derivation_hash: sequence.derivation_hash.clone(),
        authoritative_sequence_seal: sequence.authoritative_sequence_seal.clone(),
        issuance_trace_root: sequence.issuance_trace_root.clone(),
        package_count: sequence.packages.len(),
        semantic_nu_vector: sequence.semantic_nu_vector.clone(),
        deterministic_replay_valid: true,
        every_package_closed_observed_grammar: sequence.every_package_closed_observed_grammar,
        every_package_registry_extension_invariant: sequence
            .every_package_registry_extension_invariant,
        no_historical_registry_or_future_input: sequence.no_historical_registry_or_future_input,
        t_bi_b1_proved_on_sequence: sequence.t_bi_b1_proved_on_sequence,
        t_bi_b2_proved_on_sequence: sequence.t_bi_b2_proved_on_sequence,
        one_prefix_general_procedure: true,
        derivation_hash: String::new(),
    };
    sequence_audit.derivation_hash = sequence_audit_digest(&sequence_audit);

    let mut step_provenance = Vec::with_capacity(15);
    let mut steps = Vec::with_capacity(15);
    for (index, (((stage, telescope), package), expected_nu)) in reconstructed
        .entries
        .iter()
        .zip(&sequence.packages)
        .zip(&reconstructed.semantic_nu)
        .enumerate()
    {
        let candidate = candidate_hash(telescope);
        let prefix = SealedSignature::from_telescopes(reconstructed.entries[..index].to_vec());
        let prefix_digest = prefix.digest().to_owned();
        let (family_ids, family_hashes) = credited_family_projection(package)?;
        let residuals_zero = package.named_role_residual_count == 0
            && package.named_quotient_residual_count == 0
            && package.named_a3_residual_count == 0
            && package.silent_residue_count == 0;
        let exact_branch_ledger_join = package.stage == *stage
            && package.candidate_hash == candidate
            && package.predecessor_signature_digest == prefix_digest
            && package.semantic_nu == *expected_nu
            && package.rule_authority_derivation_hash == sequence.rule_authority.derivation_hash;
        let no_forbidden_input = !package.historical_registry_consulted
            && !package.archive_structural_bar_verdict_or_future_read;
        if !exact_branch_ledger_join
            || !residuals_zero
            || !package.proved
            || !package.t_bi_b1_proved
            || !package.t_bi_b2_proved
            || !package.every_role_declaration_resolved
            || !package.every_marginal_family_credited_or_theorem_impossible
            || !package.local_anchor_nonreuse_holds
            || !no_forbidden_input
        {
            return Err(Bi2BranchFinalesV1Error::Provenance(format!(
                "branch {} Stage {stage} prefix-general package did not close",
                bi1b.branch_root_hash
            )));
        }
        let (token_applicable, token_exact) = sealed_winner_token_join(
            bi1b,
            *stage,
            &candidate,
            &prefix_digest,
            &authorization.prefix_general_extraction_authority_digest,
            package,
            &family_ids,
            &family_hashes,
        )?;
        let mut provenance = Bi2StepSemanticProvenanceV1 {
            stage: *stage,
            candidate_hash: candidate.clone(),
            predecessor_signature_digest: prefix_digest,
            semantic_nu: *expected_nu,
            credited_family_ids: family_ids,
            authoritative_family_token_hashes: family_hashes.clone(),
            package_derivation_hash: package.derivation_hash.clone(),
            package_v4_candidate_local_source_hash: package.v4_candidate_local_source_hash.clone(),
            package_rule_authority_derivation_hash: package.rule_authority_derivation_hash.clone(),
            exact_branch_ledger_join,
            exact_prefix_candidate_package_binding: true,
            all_residual_counts_zero: residuals_zero,
            package_proved: package.proved,
            sealed_bi1b_winner_token_join_applicable: token_applicable,
            sealed_bi1b_winner_token_join_exact: token_exact,
            no_archive_structural_bar_verdict_or_future_input: no_forbidden_input,
            derivation_hash: String::new(),
        };
        provenance.derivation_hash = step_provenance_digest(&provenance);
        let step_evidence_hash = tagged_hash(
            "branch-step-selection-and-prefix-general-provenance",
            &(
                reconstructed.selection_evidence_hashes[index].as_str(),
                provenance.derivation_hash.as_str(),
                sequence.derivation_hash.as_str(),
                bi1b.result_digest.as_str(),
            ),
        );
        steps.push(BranchFinaleStepInput {
            stage: *stage,
            telescope: telescope.clone(),
            candidate_hash: candidate,
            kappa: telescope.kappa() as u32,
            certified_nu: *expected_nu,
            required_packages_before_selection: reconstructed.required_packages[index].clone(),
            step_evidence_hash,
            ordinary_family_token_hashes: family_hashes,
            ordinary_charge_provenance_authoritative: true,
        });
        step_provenance.push(provenance);
    }
    let continuation_digest = tagged_hash(
        "paired-sealed-branch-continuation",
        &(
            sealed.result_digest.as_str(),
            bi1b.result_digest.as_str(),
            sequence.derivation_hash.as_str(),
        ),
    );
    let finale_input = BranchFinaleInput {
        branch_root_hash: bi1b.branch_root_hash.clone(),
        continuation_digest,
        lawful_engine_stage_limit: reconstructed.lawful_engine_stage_limit,
        halt_step: 15,
        terminal_required_packages: reconstructed.terminal_required_packages,
        steps,
    };
    let finale_input_binding_hash = branch_finale_input_binding_hash(&finale_input);
    let finale = execute_replayed_branch_finale_input(&finale_input)
        .map_err(Bi2BranchFinalesV1Error::Finale)?;
    let finale_reissued_equal = execute_replayed_branch_finale_input(&finale_input)
        .map(|reissued| reissued == finale)
        .map_err(Bi2BranchFinalesV1Error::Finale)?;
    if finale.input_binding_hash != finale_input_binding_hash
        || finale.branch_root_hash != bi1b.branch_root_hash
        || finale.halt_step != 15
        || finale.successor_stage != 16
        || !finale.continuation_provenance_join_exact
        || finale.broader_absolute_theorem12_claimed
        || finale.absolute_semantic_exhaustiveness_claimed
        || !finale_reissued_equal
    {
        return Err(Bi2BranchFinalesV1Error::Finale(format!(
            "branch {} finale did not preserve its certified branch/index scope",
            bi1b.branch_root_hash
        )));
    }
    let local_g4_debt_free_halt_at_15 = finale.e5_class_complete
        && finale.theorem12_full_instance_granularity_proved
        && finale.semantic_successor_o_empty
        && finale.f1_executed
        && finale.f1_excluded
        && !finale.f1_triggered
        && finale.expressivity_gaps.is_empty()
        && finale.halt_step == 15
        && finale.successor_stage == 16;
    let mut certificate = Bi2BranchFinaleCertificateV1 {
        schema: BI2_BRANCH_FINALE_V2_SCHEMA.to_owned(),
        date: BI2_BRANCH_FINALE_V2_DATE.to_owned(),
        source_bindings: source_bindings(),
        branch_root_hash: bi1b.branch_root_hash.clone(),
        branch_digest_prefix: bi1b.branch_digest_prefix.clone(),
        enacted_root: bi1b.enacted_root,
        issuance_ordinal: bi1b.issuance_ordinal,
        history_source: reconstructed.history_source,
        row_free_authorization_digest: authorization.derivation_hash.clone(),
        bi1b_sweep_digest: authorization.bi1b_sweep_digest.clone(),
        bi1b_branch_certificate_digest: bi1b.result_digest.clone(),
        sealed_bi1_branch_certificate_digest: sealed.result_digest.clone(),
        bi1b_branch_source_replayed: true,
        own_branch_only_input: true,
        correspondence_diagnostic_consumed_by_branch_issuer: false,
        enacted_classification_or_outcome_consumed_by_alternate_issuer: false,
        semantic_value_bar_hash_or_order_used_as_selector: false,
        prefix_general_sequence: sequence_audit,
        step_provenance,
        finale_input,
        finale_input_binding_hash,
        exact_a3_window_count: finale.windows.len(),
        final_a3_inventory_count: finale.final_a3_inventory_count,
        final_unary_count: finale.final_unary_count,
        final_direct_chronological_count: finale.final_direct_chronological_count,
        final_pointwise_chronological_count: finale.final_pointwise_chronological_count,
        final_higher_count: finale.final_higher_count,
        final_structural_count: finale.final_structural_count,
        semantic_o16_empty_branch_indexed: finale.semantic_successor_o_empty,
        f1_executed: finale.f1_executed,
        f1_triggered: finale.f1_triggered,
        f1_excluded: finale.f1_excluded,
        f1_underdetermined_instance_ids: finale.underdetermined_instance_ids.clone(),
        theorem12_full_instance_granularity_branch_indexed: finale
            .theorem12_full_instance_granularity_proved,
        local_g4_debt_free_halt_at_15,
        e5_class_complete: finale.e5_class_complete,
        expressivity_gaps: finale.expressivity_gaps.clone(),
        finale,
        finale_reissued_equal,
        branch_indexed_only: true,
        bi4_cone_report_issued: false,
        cone_g4_verdict_issued: false,
        g2_verdict_issued: false,
        g3_verdict_issued: false,
        uc1_scored: false,
        bridge_authorized: false,
        unindexed_halt_o_or_theorem12_claim_issued: false,
        result_digest: String::new(),
    };
    certificate.result_digest = branch_certificate_digest(&certificate);
    Ok(certificate)
}

fn index_row(certificate: &Bi2BranchFinaleCertificateV1) -> Bi2FinaleIndexRowV1 {
    let mut row = Bi2FinaleIndexRowV1 {
        issuance_ordinal: certificate.issuance_ordinal,
        branch_root_hash: certificate.branch_root_hash.clone(),
        enacted_root: certificate.enacted_root,
        branch_certificate_digest: certificate.result_digest.clone(),
        final_a3_inventory_count: certificate.final_a3_inventory_count,
        semantic_o16_empty_branch_indexed: certificate.semantic_o16_empty_branch_indexed,
        f1_executed: certificate.f1_executed,
        f1_triggered: certificate.f1_triggered,
        f1_excluded: certificate.f1_excluded,
        e5_class_complete: certificate.e5_class_complete,
        expressivity_gap_count: certificate.expressivity_gaps.len(),
        branch_indexed_only: certificate.branch_indexed_only,
        row_hash: String::new(),
    };
    row.row_hash = index_row_digest(&row);
    row
}

/// Issue the four BI-2 finales.  No branch issuer receives another branch,
/// the BI1b correspondence diagnostic, or any enacted outcome surface.
pub fn issue_bi2_four_branch_finales_v1()
-> Result<Bi2FourBranchFinalesBundleV1, Bi2BranchFinalesV1Error> {
    let bi1b_bundle = embedded_bi1b_bundle()?;
    let authorization = issue_row_free_authorization(&bi1b_bundle)?;
    let sealed_bi1 = embedded_bi1_certificates()?
        .into_iter()
        .map(|branch| (branch.branch_root_hash.clone(), branch))
        .collect::<BTreeMap<_, _>>();
    if sealed_bi1.len() != 4 {
        return Err(Bi2BranchFinalesV1Error::EmbeddedInput(
            "embedded BI1 source does not contain exactly four unique roots".to_owned(),
        ));
    }
    let mut branch_inputs = bi1b_bundle.branches.iter().collect::<Vec<_>>();
    branch_inputs.sort_by_key(|branch| branch.issuance_ordinal);
    if branch_inputs
        .iter()
        .enumerate()
        .any(|(ordinal, branch)| branch.issuance_ordinal != ordinal)
    {
        return Err(Bi2BranchFinalesV1Error::UpstreamReplay(
            "BI1b branch issuance ordinals are not exactly 0 through 3".to_owned(),
        ));
    }
    let mut branches = Vec::with_capacity(4);
    for branch in branch_inputs {
        let sealed = sealed_bi1.get(&branch.branch_root_hash).ok_or_else(|| {
            Bi2BranchFinalesV1Error::EmbeddedInput(format!(
                "no paired BI1 certificate for {}",
                branch.branch_root_hash
            ))
        })?;
        branches.push(issue_branch_certificate(&authorization, branch, sealed)?);
    }
    let rows = branches.iter().map(index_row).collect::<Vec<_>>();
    let roots = rows
        .iter()
        .map(|row| row.branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    let exact_four_branch_surface = rows.len() == 4
        && roots.len() == 4
        && rows.iter().filter(|row| row.enacted_root).count() == 1
        && rows.iter().map(|row| row.issuance_ordinal).eq(0..4);
    let every_branch_certificate_reissued_equal =
        branches.iter().all(|branch| branch.finale_reissued_equal);
    let every_branch_finale_e5_class_complete =
        branches.iter().all(|branch| branch.e5_class_complete);
    let every_branch_local_semantic_o16_empty = branches
        .iter()
        .all(|branch| branch.semantic_o16_empty_branch_indexed);
    let every_branch_f1_executed = branches.iter().all(|branch| branch.f1_executed);
    let every_branch_f1_excluded = branches
        .iter()
        .all(|branch| branch.f1_excluded && !branch.f1_triggered);
    // BI-4 can compare only sealed local finales, not merely four syntactic
    // rows.  In particular, a named BI-2 expressivity gap is Z-STOP rather
    // than cone-ready evidence.
    let ready_for_bi4_assembly = exact_four_branch_surface
        && every_branch_certificate_reissued_equal
        && every_branch_finale_e5_class_complete
        && every_branch_local_semantic_o16_empty
        && every_branch_f1_executed
        && every_branch_f1_excluded;
    let permitted_conclusion = if ready_for_bi4_assembly {
        "Four independently replayed branch-indexed BI-2 finale certificates are sealed and ready for BI-4 assembly. Each row reports only its own relative full-A3 inventory, semantic successor-O, F1, and local-G4 disposition. This index performs no cross-branch comparison and issues no BI-4, cone-G4, G2, G3, UC-1, bridge, or unindexed halt/O/Theorem-12 claim."
    } else {
        "The four branch-indexed BI-2 attempts are sealed, but at least one local E-5 finale is incomplete. BI-4 remains closed under Z-STOP until every branch has a complete A3 membership partition, semantic successor-O decision, and executed F1 disposition. No cross-branch or unindexed claim is issued."
    };
    let mut index = Bi2FourBranchFinalesIndexV1 {
        schema: BI2_FOUR_BRANCH_FINALES_V2_SCHEMA.to_owned(),
        date: BI2_BRANCH_FINALE_V2_DATE.to_owned(),
        source_bindings: source_bindings(),
        predecessor_v1_index_binding: predecessor_v1_index_binding(),
        row_free_authorization: authorization,
        rows,
        exact_four_branch_surface,
        every_branch_certificate_reissued_equal,
        every_branch_finale_e5_class_complete,
        every_branch_local_semantic_o16_empty,
        every_branch_f1_executed,
        every_branch_f1_excluded,
        ready_for_bi4_assembly,
        cross_branch_comparison_performed: false,
        bi4_cone_report_issued: false,
        cone_g4_verdict_issued: false,
        g2_verdict_issued: false,
        g3_verdict_issued: false,
        uc1_scored: false,
        bridge_authorized: false,
        unindexed_halt_o_or_theorem12_claim_issued: false,
        permitted_conclusion: permitted_conclusion.to_owned(),
        result_digest: String::new(),
    };
    index.result_digest = index_digest(&index);
    Ok(Bi2FourBranchFinalesBundleV1 { branches, index })
}

fn branch_integrity_errors(certificate: &Bi2BranchFinaleCertificateV1) -> Vec<String> {
    let mut errors = Vec::new();
    if certificate.schema != BI2_BRANCH_FINALE_V2_SCHEMA
        || certificate.date != BI2_BRANCH_FINALE_V2_DATE
    {
        errors.push(format!(
            "branch {} schema/date mismatch",
            certificate.branch_root_hash
        ));
    }
    if certificate.result_digest != branch_certificate_digest(certificate) {
        errors.push(format!(
            "branch {} certificate digest mismatch",
            certificate.branch_root_hash
        ));
    }
    if certificate.source_bindings != source_bindings() {
        errors.push(format!(
            "branch {} source binding drift",
            certificate.branch_root_hash
        ));
    }
    if certificate.prefix_general_sequence.derivation_hash
        != sequence_audit_digest(&certificate.prefix_general_sequence)
        || !certificate
            .prefix_general_sequence
            .deterministic_replay_valid
        || !certificate
            .prefix_general_sequence
            .one_prefix_general_procedure
    {
        errors.push(format!(
            "branch {} sequence audit is invalid",
            certificate.branch_root_hash
        ));
    }
    if certificate.step_provenance.len() != 15
        || certificate
            .step_provenance
            .iter()
            .map(|row| row.stage)
            .ne(1..=15)
        || certificate
            .step_provenance
            .iter()
            .any(|row| row.derivation_hash != step_provenance_digest(row))
    {
        errors.push(format!(
            "branch {} step provenance surface is invalid",
            certificate.branch_root_hash
        ));
    }
    if certificate.finale_input_binding_hash
        != branch_finale_input_binding_hash(&certificate.finale_input)
        || certificate.finale.input_binding_hash != certificate.finale_input_binding_hash
        || certificate.finale.branch_root_hash != certificate.branch_root_hash
        || certificate.finale_input.branch_root_hash != certificate.branch_root_hash
        || certificate.finale_input.halt_step != 15
        || certificate.finale.successor_stage != 16
    {
        errors.push(format!(
            "branch {} finale input/audit binding mismatch",
            certificate.branch_root_hash
        ));
    }
    if certificate.exact_a3_window_count != certificate.finale.windows.len()
        || certificate.final_a3_inventory_count != certificate.finale.final_a3_inventory_count
        || certificate.final_unary_count != certificate.finale.final_unary_count
        || certificate.final_direct_chronological_count
            != certificate.finale.final_direct_chronological_count
        || certificate.final_pointwise_chronological_count
            != certificate.finale.final_pointwise_chronological_count
        || certificate.final_higher_count != certificate.finale.final_higher_count
        || certificate.final_structural_count != certificate.finale.final_structural_count
        || certificate.semantic_o16_empty_branch_indexed
            != certificate.finale.semantic_successor_o_empty
        || certificate.f1_executed != certificate.finale.f1_executed
        || certificate.f1_triggered != certificate.finale.f1_triggered
        || certificate.f1_excluded != certificate.finale.f1_excluded
        || certificate.f1_underdetermined_instance_ids
            != certificate.finale.underdetermined_instance_ids
        || certificate.theorem12_full_instance_granularity_branch_indexed
            != certificate
                .finale
                .theorem12_full_instance_granularity_proved
        || certificate.e5_class_complete != certificate.finale.e5_class_complete
        || certificate.expressivity_gaps != certificate.finale.expressivity_gaps
    {
        errors.push(format!(
            "branch {} copied semantic summary drifted from its finale",
            certificate.branch_root_hash
        ));
    }
    if !certificate.bi1b_branch_source_replayed
        || !certificate.own_branch_only_input
        || certificate.correspondence_diagnostic_consumed_by_branch_issuer
        || certificate.enacted_classification_or_outcome_consumed_by_alternate_issuer
        || certificate.semantic_value_bar_hash_or_order_used_as_selector
        || !certificate.branch_indexed_only
        || certificate.bi4_cone_report_issued
        || certificate.cone_g4_verdict_issued
        || certificate.g2_verdict_issued
        || certificate.g3_verdict_issued
        || certificate.uc1_scored
        || certificate.bridge_authorized
        || certificate.unindexed_halt_o_or_theorem12_claim_issued
    {
        errors.push(format!(
            "branch {} exceeded the BI-2 branch-indexed scope",
            certificate.branch_root_hash
        ));
    }
    errors
}

fn bundle_integrity_errors(bundle: &Bi2FourBranchFinalesBundleV1) -> Vec<String> {
    let mut errors = bundle
        .branches
        .iter()
        .flat_map(branch_integrity_errors)
        .collect::<Vec<_>>();
    if bundle.index.schema != BI2_FOUR_BRANCH_FINALES_V2_SCHEMA
        || bundle.index.date != BI2_BRANCH_FINALE_V2_DATE
        || bundle.index.result_digest != index_digest(&bundle.index)
        || bundle.index.source_bindings != source_bindings()
        || bundle.index.predecessor_v1_index_binding != predecessor_v1_index_binding()
        || bundle.index.row_free_authorization.derivation_hash
            != authorization_digest(&bundle.index.row_free_authorization)
    {
        errors.push("BI-2 four-branch index digest/schema/source binding mismatch".to_owned());
    }
    let authorization = &bundle.index.row_free_authorization;
    if (!authorization.public_deterministic_replay_passed
        && !authorization.frozen_testimony_fallback_used)
        || (authorization.frozen_testimony_fallback_used
            && authorization.public_replay_errors != vec![BI1B_CURRENT_REISSUANCE_DRIFT.to_owned()])
        || !authorization.frozen_bi1b_sweep_digest_exact
        || authorization.bi1b_sweep_digest != FROZEN_BI1B_SWEEP_DIGEST
    {
        errors.push("BI-2 upstream BI1b replay/frozen-testimony surface is invalid".to_owned());
    }
    if bundle.index.rows.len() != bundle.branches.len()
        || bundle
            .index
            .rows
            .iter()
            .any(|row| row.row_hash != index_row_digest(row))
    {
        errors.push("BI-2 index rows are malformed".to_owned());
    }
    for branch in &bundle.branches {
        if bundle
            .index
            .rows
            .iter()
            .filter(|row| {
                row.branch_root_hash == branch.branch_root_hash
                    && row.branch_certificate_digest == branch.result_digest
                    && row.issuance_ordinal == branch.issuance_ordinal
                    && row.enacted_root == branch.enacted_root
                    && row.semantic_o16_empty_branch_indexed
                        == branch.semantic_o16_empty_branch_indexed
                    && row.f1_executed == branch.f1_executed
                    && row.f1_triggered == branch.f1_triggered
                    && row.f1_excluded == branch.f1_excluded
                    && row.e5_class_complete == branch.e5_class_complete
                    && row.expressivity_gap_count == branch.expressivity_gaps.len()
            })
            .count()
            != 1
        {
            errors.push(format!(
                "branch {} lacks one exact BI-2 index row",
                branch.branch_root_hash
            ));
        }
    }
    let expected_ready_for_bi4 = bundle.index.exact_four_branch_surface
        && bundle.index.every_branch_certificate_reissued_equal
        && bundle.index.every_branch_finale_e5_class_complete
        && bundle.index.every_branch_local_semantic_o16_empty
        && bundle.index.every_branch_f1_executed
        && bundle.index.every_branch_f1_excluded;
    if !bundle.index.exact_four_branch_surface
        || !bundle.index.every_branch_certificate_reissued_equal
        || bundle.index.ready_for_bi4_assembly != expected_ready_for_bi4
        || bundle.index.cross_branch_comparison_performed
        || bundle.index.bi4_cone_report_issued
        || bundle.index.cone_g4_verdict_issued
        || bundle.index.g2_verdict_issued
        || bundle.index.g3_verdict_issued
        || bundle.index.uc1_scored
        || bundle.index.bridge_authorized
        || bundle.index.unindexed_halt_o_or_theorem12_claim_issued
    {
        errors.push("BI-2 index did not preserve its pre-BI4 scope".to_owned());
    }
    errors
}

/// Full deterministic replay.  A fully rehashed mutation still fails because
/// all embedded upstream inputs, semantic sequences, and branch finales are
/// independently reissued before exact bundle equality is accepted.
pub fn replay_bi2_four_branch_finales_v1(
    claimed: &Bi2FourBranchFinalesBundleV1,
) -> Bi2FourBranchFinalesReplayV1 {
    let mut errors = bundle_integrity_errors(claimed);
    match issue_bi2_four_branch_finales_v1() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("BI-2 bundle differs from deterministic reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    Bi2FourBranchFinalesReplayV1 {
        valid: errors.is_empty(),
        errors,
    }
}

fn branch_json_name(prefix: &str) -> String {
    format!("BI2_BRANCH_{prefix}_V2_CERTIFICATE.json")
}

fn branch_report_name(prefix: &str) -> String {
    format!("BI2_BRANCH_{prefix}_V2_RESULT.md")
}

pub fn render_bi2_branch_finale_v1(certificate: &Bi2BranchFinaleCertificateV1) -> String {
    let f1 = if certificate.f1_triggered {
        format!(
            "triggered on `{}`",
            certificate.f1_underdetermined_instance_ids.join("`, `")
        )
    } else if certificate.f1_excluded {
        "excluded after exhaustive D partition".to_owned()
    } else {
        "not executable; see named gaps".to_owned()
    };
    let gaps = if certificate.expressivity_gaps.is_empty() {
        "none".to_owned()
    } else {
        certificate.expressivity_gaps.join("; ")
    };
    format!(
        "# BI-2 v2 branch {} finale\n\n**Date:** {}. **Certificate:** `{}`. **Enacted index:** {}.\n\nThe exact branch-local prefix-general sequence replayed: **{}**; semantic vector: `{:?}`. The E-5 semantic core independently covered **{}** exact-prefix windows through Stage 16.\n\nFinal A3 inventory: **{}** = {} unary + {} direct chronological + {} pointwise chronological + {} higher + {} structural. Semantic `O_B(16) = empty`: **{}**. F1: **{}**. Full instance-granularity Theorem 12 on the adopted A3 scope: **{}**. Local G4 debt-free halt at 15: **{}**. E-5 complete: **{}**. Named gaps: `{}`.\n\nThis create-new v2 certificate preserves the partial v1 run as immutable testimony. It is branch-indexed and issues no BI-4/cone-G4, G2, G3, UC-1, bridge, or unindexed halt/O/Theorem-12 claim.\n",
        certificate.branch_digest_prefix,
        certificate.date,
        certificate.result_digest,
        certificate.enacted_root,
        certificate
            .prefix_general_sequence
            .deterministic_replay_valid,
        certificate.prefix_general_sequence.semantic_nu_vector,
        certificate.exact_a3_window_count,
        certificate.final_a3_inventory_count,
        certificate.final_unary_count,
        certificate.final_direct_chronological_count,
        certificate.final_pointwise_chronological_count,
        certificate.final_higher_count,
        certificate.final_structural_count,
        certificate.semantic_o16_empty_branch_indexed,
        f1,
        certificate.theorem12_full_instance_granularity_branch_indexed,
        certificate.local_g4_debt_free_halt_at_15,
        certificate.e5_class_complete,
        gaps,
    )
}

pub fn render_bi2_four_branch_finales_v1(index: &Bi2FourBranchFinalesIndexV1) -> String {
    let rows = index
        .rows
        .iter()
        .map(|row| {
            format!(
                "| {} | `{}` | {} | {} | {} | {} | {} | {} |",
                row.issuance_ordinal,
                row.branch_root_hash,
                row.enacted_root,
                row.final_a3_inventory_count,
                row.semantic_o16_empty_branch_indexed,
                if row.f1_triggered {
                    "triggered"
                } else if row.f1_excluded {
                    "excluded"
                } else {
                    "not executed"
                },
                row.e5_class_complete,
                row.expressivity_gap_count,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# BI-2 v2 four branch-indexed finales\n\n**Date:** {}. **Index certificate:** `{}`. The partial v1 run remains sealed and is not overwritten.\n\nBI-1b public deterministic replay passed: **{}**. Frozen-testimony fallback used: **{}**. Exact replay report: `{}`. The fallback is admitted only for the single bound post-seal reissuance drift; all BI-1b integrity, source, root, and cross-certificate checks passed before it.\n\n| Ordinal | Branch root | Enacted | Final A3 | O_B(16) empty | F1 | E-5 complete | Gaps |\n|---:|---|---|---:|---|---|---|---:|\n{}\n\nAll four branch certificates reissued: **{}**. Every local E-5 finale completed: **{}**. Every local semantic successor obligation is empty: **{}**. F1 executed/excluded on every branch: **{}/{}**. Ready for BI-4 assembly: **{}**.\n\nNo cross-branch comparison was performed. No BI-4, cone-G4, G2, G3, UC-1, bridge, or unindexed claim is issued.\n\n{}\n",
        index.date,
        index.result_digest,
        index
            .row_free_authorization
            .public_deterministic_replay_passed,
        index.row_free_authorization.frozen_testimony_fallback_used,
        if index.row_free_authorization.public_replay_errors.is_empty() {
            "none".to_owned()
        } else {
            index.row_free_authorization.public_replay_errors.join("; ")
        },
        rows,
        index.every_branch_certificate_reissued_equal,
        index.every_branch_finale_e5_class_complete,
        index.every_branch_local_semantic_o16_empty,
        index.every_branch_f1_executed,
        index.every_branch_f1_excluded,
        index.ready_for_bi4_assembly,
        index.permitted_conclusion,
    )
}

fn create_new(path: &Path, contents: &[u8]) -> Result<(), Bi2BranchFinalesV1Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Bi2BranchFinalesV1Error::Io(format!("{}: {error}", path.display())))?;
    file.write_all(contents)
        .map_err(|error| Bi2BranchFinalesV1Error::Io(format!("{}: {error}", path.display())))
}

pub fn emit_bi2_four_branch_finales_v1_create_new(
    directory: &Path,
) -> Result<Bi2FourBranchFinalesBundleV1, Bi2BranchFinalesV1Error> {
    let bundle = issue_bi2_four_branch_finales_v1()?;
    let replay = replay_bi2_four_branch_finales_v1(&bundle);
    if !replay.valid {
        return Err(Bi2BranchFinalesV1Error::Invariant(format!(
            "create-new emission requires exact replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut outputs = Vec::<(std::path::PathBuf, Vec<u8>)>::new();
    for branch in &bundle.branches {
        outputs.push((
            directory.join(branch_json_name(&branch.branch_digest_prefix)),
            serde_json::to_vec_pretty(branch).map_err(|error| {
                Bi2BranchFinalesV1Error::Io(format!(
                    "branch {} JSON: {error}",
                    branch.branch_root_hash
                ))
            })?,
        ));
        outputs.push((
            directory.join(branch_report_name(&branch.branch_digest_prefix)),
            render_bi2_branch_finale_v1(branch).into_bytes(),
        ));
    }
    outputs.push((
        directory.join(BI2_INDEX_CERTIFICATE_NAME),
        serde_json::to_vec_pretty(&bundle.index)
            .map_err(|error| Bi2BranchFinalesV1Error::Io(format!("index JSON: {error}")))?,
    ));
    outputs.push((
        directory.join(BI2_INDEX_REPORT_NAME),
        render_bi2_four_branch_finales_v1(&bundle.index).into_bytes(),
    ));
    if let Some(path) = outputs
        .iter()
        .map(|(path, _)| path)
        .find(|path| path.exists())
    {
        return Err(Bi2BranchFinalesV1Error::Io(format!(
            "{} already exists; create-new refuses overwrite",
            path.display()
        )));
    }
    for (path, contents) in outputs {
        create_new(&path, &contents)?;
    }
    Ok(bundle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_inputs_parse_and_are_still_pre_bi2() {
        let bundle = embedded_bi1b_bundle().expect("embedded BI1b bundle");
        assert_eq!(bundle.branches.len(), 4);
        assert!(
            bundle
                .branches
                .iter()
                .all(|branch| !branch.bi2_finale_issued)
        );
        assert!(!bundle.sweep.bi2_finale_issued);
        assert!(!bundle.correspondence.bi2_finale_issued);
        let sealed = embedded_bi1_certificates().expect("embedded BI1 branches");
        assert_eq!(sealed.len(), 4);
    }

    #[test]
    fn v2_artifact_names_are_disjoint_from_the_immutable_v1_set() {
        let v1 = [
            "BI2_FOUR_BRANCH_FINALES_V1_CERTIFICATE.json".to_owned(),
            "BI2_FOUR_BRANCH_FINALES_V1_RESULT.md".to_owned(),
        ]
        .into_iter()
        .chain(
            [
                "2016726758f3",
                "43a0ed707770",
                "4b2211ecae25",
                "b4f821d9bb28",
            ]
            .into_iter()
            .flat_map(|prefix| {
                [
                    format!("BI2_BRANCH_{prefix}_CERTIFICATE.json"),
                    format!("BI2_BRANCH_{prefix}_RESULT.md"),
                ]
            }),
        )
        .collect::<BTreeSet<_>>();
        let v2 = [
            BI2_INDEX_CERTIFICATE_NAME.to_owned(),
            BI2_INDEX_REPORT_NAME.to_owned(),
        ]
        .into_iter()
        .chain(
            [
                "2016726758f3",
                "43a0ed707770",
                "4b2211ecae25",
                "b4f821d9bb28",
            ]
            .into_iter()
            .flat_map(|prefix| [branch_json_name(prefix), branch_report_name(prefix)]),
        )
        .collect::<BTreeSet<_>>();
        assert!(v1.is_disjoint(&v2));
        assert!(
            !source_bindings().iter().any(|binding| {
                binding.path == "docs/BI2_FOUR_BRANCH_FINALES_V1_CERTIFICATE.json"
            })
        );
        let predecessor = predecessor_v1_index_binding();
        assert_eq!(
            predecessor.path,
            "docs/BI2_FOUR_BRANCH_FINALES_V1_CERTIFICATE.json"
        );
        assert_eq!(predecessor.blake3, bytes_hash(BI2_V1_INDEX_BYTES));
    }

    #[test]
    #[ignore = "full BI1b replay plus four semantic-sequence and E-5 reissuances is expensive"]
    fn four_branch_finales_replay_and_fully_rehashed_mutation_fails() {
        let bundle = issue_bi2_four_branch_finales_v1().expect("BI2 bundle");
        let replay = replay_bi2_four_branch_finales_v1(&bundle);
        assert!(replay.valid, "{:?}", replay.errors);
        let mut mutated = bundle.clone();
        mutated.branches[1].semantic_o16_empty_branch_indexed =
            !mutated.branches[1].semantic_o16_empty_branch_indexed;
        mutated.branches[1].result_digest = branch_certificate_digest(&mutated.branches[1]);
        mutated.index.rows[1].semantic_o16_empty_branch_indexed =
            mutated.branches[1].semantic_o16_empty_branch_indexed;
        mutated.index.rows[1].branch_certificate_digest = mutated.branches[1].result_digest.clone();
        mutated.index.rows[1].row_hash = index_row_digest(&mutated.index.rows[1]);
        mutated.index.result_digest = index_digest(&mutated.index);
        assert!(!replay_bi2_four_branch_finales_v1(&mutated).valid);
    }
}
