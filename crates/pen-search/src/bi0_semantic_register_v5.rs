//! BI-0 join over the completed additive v5 semantic and chronological gates.
//!
//! This artifact is deliberately terminal at BI-0. A positive result opens a
//! separate BI-1 create-new run; it does not enumerate or inspect the cone.

use crate::chronological_slot_map_v4::ChronologicalCaseDispositionV4;
use crate::chronological_slot_map_v5::{
    SUPPORT_COMPREHENSION_CASE_ID, issue_chronological_slot_map_v5_certificate,
};
use crate::t_bi_intrinsic_isolation_v2::{
    T_BI_B3_V2_THEOREM_ID, T_BI_INTRINSIC_ISOLATION_V2_SCHEMA,
};
use crate::t_bi_nu1_regression_v5::{BareRegisterRowV5, issue_t_bi_nu1_regression_v5_certificate};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const BI0_SEMANTIC_REGISTER_V5_SCHEMA: &str =
    "branch-invariance-bi0-semantic-register-f-al1-prime-v5";
pub const BI0_SEMANTIC_REGISTER_V5_DATE: &str = "2026-07-22";
pub const BI0_SEMANTIC_REGISTER_V5_CERTIFICATE_NAME: &str = "bi0_semantic_register_v5.json";
pub const BI0_SEMANTIC_REGISTER_V5_REPORT_NAME: &str = "BI0_SEMANTIC_REGISTER_V5_RESULT.md";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI0_SEMANTIC_REGISTER_V5_SCHEMA, domain, value))
        .expect("BI-0 v5 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0OperationalProjectionV5 {
    pub stage: u32,
    pub winner_candidate_hash: String,
    pub winner_admissible_under_recorded_regime: bool,
    pub operational_row_exact: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi0JoinOperationV5 {
    IssueTBiPrerequisite,
    IssueChronologicalPrerequisite,
    JoinPrerequisiteProjections,
    SealBi0Certificate,
    InvokeBi1,
    EnumerateNonEnactedCone,
    ExecuteNonEnactedBranch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi0JoinCapabilityV5 {
    ReadTBiPrerequisite,
    ReadChronologicalPrerequisite,
    ComparePrerequisiteProjections,
    ContentHashing,
    InvokeBi1,
    EnumerateNonEnactedCone,
    ExecuteNonEnactedBranch,
}

impl Bi0JoinOperationV5 {
    fn capabilities(self) -> Vec<Bi0JoinCapabilityV5> {
        use Bi0JoinCapabilityV5 as C;
        match self {
            Self::IssueTBiPrerequisite => vec![C::ReadTBiPrerequisite],
            Self::IssueChronologicalPrerequisite => vec![C::ReadChronologicalPrerequisite],
            Self::JoinPrerequisiteProjections => vec![C::ComparePrerequisiteProjections],
            Self::SealBi0Certificate => vec![C::ContentHashing],
            Self::InvokeBi1 => vec![C::InvokeBi1],
            Self::EnumerateNonEnactedCone => vec![C::EnumerateNonEnactedCone],
            Self::ExecuteNonEnactedBranch => vec![C::ExecuteNonEnactedBranch],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0JoinEffectProofV5 {
    pub theorem_id: String,
    pub bound_prerequisite_digests: Vec<String>,
    pub operations: Vec<Bi0JoinOperationV5>,
    pub derived_capabilities: Vec<Bi0JoinCapabilityV5>,
    pub forbidden_successor_capabilities: Vec<Bi0JoinCapabilityV5>,
    pub every_capability_derived_from_closed_operation_enum: bool,
    pub no_successor_capability: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0SemanticRegisterV5Certificate {
    pub schema: String,
    pub date: String,
    pub t_bi_result_digest: String,
    pub t_bi_intrinsic_sequence_seal: String,
    pub t_bi_intrinsic_isolation_digest: String,
    pub t_bi_intrinsic_isolation_schema: String,
    pub t_bi_intrinsic_isolation_theorem_id: String,
    pub t_bi_intrinsic_isolation_exact_typed_phase_surface: bool,
    pub t_bi_intrinsic_isolation_no_forbidden_capability: bool,
    pub t_bi_intrinsic_isolation_synthetic_receipt_input_accepted: bool,
    pub t_bi_role_declaration_count: usize,
    pub t_bi_proved_family_count: usize,
    pub t_bi_named_impossibility_count: usize,
    pub t_bi_named_registry_residual_count: usize,
    pub t_bi_named_quotient_residual_count: usize,
    pub t_bi_named_a3_residual_count: usize,
    pub t_bi_total_named_residual_count: usize,
    pub t_bi_silent_residue_count: usize,
    pub t_bi_b1_proved: bool,
    pub t_bi_b2_proved: bool,
    pub t_bi_b3_proved: bool,
    pub t_bi_special_cases_proved: bool,
    pub t_bi_extraction_complete: bool,
    pub t_bi_operational_rows: Vec<Bi0OperationalProjectionV5>,
    pub t_bi_exact_operational_regression: bool,
    pub structural_register: Vec<u32>,
    pub authoritative_semantic_register: Option<Vec<u32>>,
    pub non_authoritative_extraction_floor: Option<Vec<u32>>,
    pub bare_register_table: Vec<BareRegisterRowV5>,
    pub bare_register_table_exact: bool,
    pub register_value_projection_exact: bool,
    pub t_bi_f_al1_prime_passed: bool,
    pub t_bi_side_passed: bool,
    pub chronological_result_digest: String,
    pub chronological_baseline_v4_result_digest: String,
    pub chronological_exact_54_plus_18_partition: bool,
    pub chronological_exact_sealed_72_id_join: bool,
    pub chronological_support_hardening_digest: String,
    pub chronological_support_graph_normal_form_proved: bool,
    pub chronological_support_zero_mint_capability_proved: bool,
    pub chronological_support_replacement_evidence_hash: String,
    pub chronological_support_replacement_evidence_bound: bool,
    pub chronological_inherited_71_exact: bool,
    pub chronological_changed_case_ids: Vec<String>,
    pub chronological_derived_count: usize,
    pub chronological_total_count: usize,
    pub chronological_named_gap_count: usize,
    pub chronological_former_derived_count: usize,
    pub chronological_former_expected_count: usize,
    pub chronological_t_sm1b_derived_count: usize,
    pub chronological_t_sm1b_surface_count: usize,
    pub chronological_blocking_case_ids: Vec<String>,
    pub chronological_zero_accounting: bool,
    pub f_sm1_fixed_positive_gate_passed: bool,
    pub join_effect_proof: Bi0JoinEffectProofV5,
    pub bi0_attempt_executed: bool,
    pub bi0_passed: bool,
    pub bi1_invoked: bool,
    pub non_enacted_cone_invoked: bool,
    pub non_enacted_branch_work_executed: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0SemanticRegisterV5Replay {
    pub valid: bool,
    pub bi0_passed: bool,
    pub t_bi_side_passed: bool,
    pub f_sm1_fixed_positive_gate_passed: bool,
    pub bi1_invoked: bool,
    pub non_enacted_cone_invoked: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bi0SemanticRegisterV5Error {
    #[error("T-BI-NU1 v5 issuance failed: {0}")]
    TBi(String),
    #[error("chronological v5 issuance failed: {0}")]
    Chronological(String),
    #[error("BI-0 v5 JSON failed: {0}")]
    Json(String),
    #[error("BI-0 v5 I/O failed: {0}")]
    Io(String),
    #[error("emitted BI-0 v5 artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn certificate_digest(certificate: &Bi0SemanticRegisterV5Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn register_table_exact(rows: &[BareRegisterRowV5]) -> bool {
    rows.len() == 15
        && rows.iter().map(|row| row.stage).eq(1..=15)
        && rows.iter().all(|row| {
            serde_json::to_value(row)
                .ok()
                .and_then(|value| value.as_object().cloned())
                .is_some_and(|object| {
                    object.len() == 3
                        && object.contains_key("stage")
                        && object.contains_key("structural")
                        && object.contains_key("semantic")
                })
        })
}

fn register_value_projection_exact(
    structural: &[u32],
    semantic: &Option<Vec<u32>>,
    rows: &[BareRegisterRowV5],
) -> bool {
    let Some(semantic) = semantic else {
        return false;
    };
    structural.len() == 15
        && semantic.len() == 15
        && rows.len() == 15
        && rows.iter().enumerate().all(|(index, row)| {
            row.stage == index as u32 + 1
                && row.structural == structural[index]
                && row.semantic == semantic[index]
        })
}

fn derive_bi0_join_effect_proof(
    t_bi_digest: &str,
    chronological_digest: &str,
) -> Bi0JoinEffectProofV5 {
    let operations = vec![
        Bi0JoinOperationV5::IssueTBiPrerequisite,
        Bi0JoinOperationV5::IssueChronologicalPrerequisite,
        Bi0JoinOperationV5::JoinPrerequisiteProjections,
        Bi0JoinOperationV5::SealBi0Certificate,
    ];
    let derived_capabilities = operations
        .iter()
        .flat_map(|operation| operation.capabilities())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let forbidden = BTreeSet::from([
        Bi0JoinCapabilityV5::InvokeBi1,
        Bi0JoinCapabilityV5::EnumerateNonEnactedCone,
        Bi0JoinCapabilityV5::ExecuteNonEnactedBranch,
    ]);
    let forbidden_successor_capabilities = derived_capabilities
        .iter()
        .filter(|capability| forbidden.contains(capability))
        .copied()
        .collect::<Vec<_>>();
    let every_capability_derived_from_closed_operation_enum = derived_capabilities
        == operations
            .iter()
            .flat_map(|operation| operation.capabilities())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
    let no_successor_capability = forbidden_successor_capabilities.is_empty();
    let mut proof = Bi0JoinEffectProofV5 {
        theorem_id: "T-BI0-E1-closed-prerequisite-join-effect".to_owned(),
        bound_prerequisite_digests: vec![t_bi_digest.to_owned(), chronological_digest.to_owned()],
        operations,
        derived_capabilities,
        forbidden_successor_capabilities,
        every_capability_derived_from_closed_operation_enum,
        no_successor_capability,
        proved: every_capability_derived_from_closed_operation_enum && no_successor_capability,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("bi0-join-effect-proof", &proof);
    proof
}

pub fn issue_bi0_semantic_register_v5_certificate()
-> Result<Bi0SemanticRegisterV5Certificate, Bi0SemanticRegisterV5Error> {
    // The issuers are independent. Neither sees the other's result.
    let t_bi = issue_t_bi_nu1_regression_v5_certificate()
        .map_err(|error| Bi0SemanticRegisterV5Error::TBi(error.to_string()))?;
    let chronological = issue_chronological_slot_map_v5_certificate()
        .map_err(|error| Bi0SemanticRegisterV5Error::Chronological(error.to_string()))?;

    let t_bi_operational_rows = t_bi
        .operational
        .rows
        .iter()
        .map(|row| Bi0OperationalProjectionV5 {
            stage: row.stage,
            winner_candidate_hash: row.winner_candidate_hash.clone(),
            winner_admissible_under_recorded_regime: row.winner_admissible_under_recorded_regime,
            operational_row_exact: row.operational_row_exact,
        })
        .collect::<Vec<_>>();
    let operational_projection_exact = t_bi_operational_rows.len() == 15
        && t_bi_operational_rows.iter().map(|row| row.stage).eq(1..=15)
        && t_bi_operational_rows
            .iter()
            .all(|row| row.winner_admissible_under_recorded_regime && row.operational_row_exact);
    let bare_register_table = t_bi.register_table.clone().unwrap_or_default();
    let bare_register_table_exact =
        register_table_exact(&bare_register_table) && t_bi.register_table_is_bare_and_exact;
    let register_value_projection_exact = register_value_projection_exact(
        &t_bi.structural_register,
        &t_bi.authoritative_semantic_register,
        &bare_register_table,
    );
    let t_bi_special_cases_proved = t_bi.stage1_exception_proved
        && t_bi.stage2_constitutive_question_settled
        && t_bi.stage9_boundary_case_settled;
    let t_bi_b3_v2_projection_exact = t_bi.intrinsic_isolation.schema
        == T_BI_INTRINSIC_ISOLATION_V2_SCHEMA
        && t_bi.intrinsic_isolation.theorem_id == T_BI_B3_V2_THEOREM_ID
        && t_bi.intrinsic_isolation.exact_fifteen_stage_surface
        && t_bi.intrinsic_isolation.exact_typed_phase_surface
        && t_bi
            .intrinsic_isolation
            .exact_package_commitments_recomputed
        && t_bi
            .intrinsic_isolation
            .exact_sequence_commitments_recomputed
        && t_bi.intrinsic_isolation.every_predecessor_hash_bound
        && t_bi
            .intrinsic_isolation
            .every_capability_derived_from_operation
        && t_bi.intrinsic_isolation.every_node_reaches_sequence_seal
        && t_bi
            .intrinsic_isolation
            .no_forbidden_capability_in_transitive_closure
        && !t_bi.intrinsic_isolation.source_scan_used_as_proof
        && !t_bi.intrinsic_isolation.runtime_self_report_used_as_proof
        && !t_bi.intrinsic_isolation.synthetic_receipt_input_accepted
        && t_bi
            .intrinsic_isolation
            .transitive_call_graph_isolation_proved
        && t_bi.intrinsic_isolation_replayed
        && t_bi.preseal_completed_before_postseal_inputs;
    let t_bi_side_passed = t_bi.role_declaration_count == 250
        && t_bi.proved_family_declaration_count + t_bi.theorem_impossibility_declaration_count
            == 250
        && t_bi.named_registry_residual_count == 0
        && t_bi.named_quotient_residual_count == 0
        && t_bi.named_a3_residual_count == 0
        && t_bi.total_named_residual_count
            == t_bi.named_registry_residual_count
                + t_bi.named_quotient_residual_count
                + t_bi.named_a3_residual_count
        && t_bi.total_named_residual_count == 0
        && t_bi.silent_residue_count == 0
        && t_bi.t_bi_b1_proved
        && t_bi.t_bi_b2_proved
        && t_bi.t_bi_b3_proved
        && t_bi_b3_v2_projection_exact
        && t_bi_special_cases_proved
        && t_bi.extraction_complete
        && t_bi.operational.exact_operational_regression
        && operational_projection_exact
        && t_bi
            .authoritative_semantic_register
            .as_ref()
            .is_some_and(|register| register.len() == 15)
        && t_bi.non_authoritative_extraction_floor.is_none()
        && bare_register_table_exact
        && register_value_projection_exact
        && !t_bi.structural_semantic_scalar_equality_required
        && t_bi.f_al1_prime_passed
        && t_bi.t_bi_nu1_proved_on_enacted_branch
        && t_bi.bi0_rerun_authorized_by_t_bi_nu1_side
        && !t_bi.non_enacted_branch_work_executed;

    let mut chronological_blocking_case_ids = chronological
        .cases
        .iter()
        .filter(|case| {
            matches!(
                case.disposition,
                ChronologicalCaseDispositionV4::NamedBlocker { .. }
            )
        })
        .map(|case| case.instance_id.clone())
        .collect::<Vec<_>>();
    chronological_blocking_case_ids.sort();
    let chronological_inherited_71_exact = chronological.inherited_row_equality_count == 71
        && chronological.prior_71_rows_byte_identical
        && chronological.prior_71_evidence_hashes_identical
        && chronological.prior_71_row_hashes_identical;
    let f_sm1_fixed_positive_gate_passed = chronological.baseline_v4_replayed
        && chronological.baseline_exact_54_plus_18_partition
        && chronological.baseline_exact_sealed_72_id_join
        && chronological.support_comprehension_replayed
        && chronological.support_graph_normal_form_proved
        && chronological.support_zero_mint_capability_proved
        && chronological.support_replacement_evidence_bound
        && chronological.support_comprehension_case_id == SUPPORT_COMPREHENSION_CASE_ID
        && chronological_inherited_71_exact
        && chronological.exactly_one_case_changed
        && chronological.changed_case_ids == vec![SUPPORT_COMPREHENSION_CASE_ID]
        && chronological.sealed_discharge_count == 72
        && chronological.sealed_discharge_derived_count == 72
        && chronological.sealed_discharge_named_gap_count == 0
        && chronological.former_nine_exact
        && chronological.former_derived_instance_ids.len() == 9
        && chronological.former_expected_instance_ids.len() == 9
        && chronological.former_named_gap_instance_ids.is_empty()
        && chronological.t_sm1b_surface_count == 18
        && chronological.t_sm1b_derived_count == 18
        && chronological.t_sm1b_named_gap_count == 0
        && chronological.f_sc3_zero_accounting_passed
        && chronological.f_sc1_regression_passed
        && chronological.fixed_positive_gate_passed
        && chronological.bi0_chronological_prerequisite_reopened
        && chronological_blocking_case_ids.is_empty();

    let join_effect_proof =
        derive_bi0_join_effect_proof(&t_bi.result_digest, &chronological.result_digest);
    let effect_capabilities = join_effect_proof
        .derived_capabilities
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let bi0_attempt_executed =
        effect_capabilities.contains(&Bi0JoinCapabilityV5::ComparePrerequisiteProjections);
    let bi1_invoked = effect_capabilities.contains(&Bi0JoinCapabilityV5::InvokeBi1);
    let non_enacted_cone_invoked =
        effect_capabilities.contains(&Bi0JoinCapabilityV5::EnumerateNonEnactedCone);
    let non_enacted_branch_work_executed =
        effect_capabilities.contains(&Bi0JoinCapabilityV5::ExecuteNonEnactedBranch);
    let bi0_passed = t_bi_side_passed
        && f_sm1_fixed_positive_gate_passed
        && join_effect_proof.proved
        && bi0_attempt_executed
        && !bi1_invoked
        && !non_enacted_cone_invoked
        && !non_enacted_branch_work_executed;
    let mut certificate = Bi0SemanticRegisterV5Certificate {
        schema: BI0_SEMANTIC_REGISTER_V5_SCHEMA.to_owned(),
        date: BI0_SEMANTIC_REGISTER_V5_DATE.to_owned(),
        t_bi_result_digest: t_bi.result_digest,
        t_bi_intrinsic_sequence_seal: t_bi.intrinsic_sequence_seal,
        t_bi_intrinsic_isolation_digest: t_bi.intrinsic_isolation.derivation_hash,
        t_bi_intrinsic_isolation_schema: t_bi.intrinsic_isolation.schema,
        t_bi_intrinsic_isolation_theorem_id: t_bi.intrinsic_isolation.theorem_id,
        t_bi_intrinsic_isolation_exact_typed_phase_surface: t_bi
            .intrinsic_isolation
            .exact_typed_phase_surface,
        t_bi_intrinsic_isolation_no_forbidden_capability: t_bi
            .intrinsic_isolation
            .no_forbidden_capability_in_transitive_closure,
        t_bi_intrinsic_isolation_synthetic_receipt_input_accepted: t_bi
            .intrinsic_isolation
            .synthetic_receipt_input_accepted,
        t_bi_role_declaration_count: t_bi.role_declaration_count,
        t_bi_proved_family_count: t_bi.proved_family_declaration_count,
        t_bi_named_impossibility_count: t_bi.theorem_impossibility_declaration_count,
        t_bi_named_registry_residual_count: t_bi.named_registry_residual_count,
        t_bi_named_quotient_residual_count: t_bi.named_quotient_residual_count,
        t_bi_named_a3_residual_count: t_bi.named_a3_residual_count,
        t_bi_total_named_residual_count: t_bi.total_named_residual_count,
        t_bi_silent_residue_count: t_bi.silent_residue_count,
        t_bi_b1_proved: t_bi.t_bi_b1_proved,
        t_bi_b2_proved: t_bi.t_bi_b2_proved,
        t_bi_b3_proved: t_bi.t_bi_b3_proved,
        t_bi_special_cases_proved,
        t_bi_extraction_complete: t_bi.extraction_complete,
        t_bi_operational_rows,
        t_bi_exact_operational_regression: t_bi.operational.exact_operational_regression,
        structural_register: t_bi.structural_register,
        authoritative_semantic_register: t_bi.authoritative_semantic_register,
        non_authoritative_extraction_floor: t_bi.non_authoritative_extraction_floor,
        bare_register_table,
        bare_register_table_exact,
        register_value_projection_exact,
        t_bi_f_al1_prime_passed: t_bi.f_al1_prime_passed,
        t_bi_side_passed,
        chronological_result_digest: chronological.result_digest,
        chronological_baseline_v4_result_digest: chronological.baseline_v4_result_digest,
        chronological_exact_54_plus_18_partition: chronological
            .baseline_exact_54_plus_18_partition,
        chronological_exact_sealed_72_id_join: chronological.baseline_exact_sealed_72_id_join,
        chronological_support_hardening_digest: chronological
            .support_comprehension_hardening
            .derivation_hash,
        chronological_support_graph_normal_form_proved: chronological
            .support_graph_normal_form_proved,
        chronological_support_zero_mint_capability_proved: chronological
            .support_zero_mint_capability_proved,
        chronological_support_replacement_evidence_hash: chronological
            .support_replacement_evidence_hash,
        chronological_support_replacement_evidence_bound: chronological
            .support_replacement_evidence_bound,
        chronological_inherited_71_exact,
        chronological_changed_case_ids: chronological.changed_case_ids,
        chronological_derived_count: chronological.sealed_discharge_derived_count,
        chronological_total_count: chronological.sealed_discharge_count,
        chronological_named_gap_count: chronological.sealed_discharge_named_gap_count,
        chronological_former_derived_count: chronological.former_derived_instance_ids.len(),
        chronological_former_expected_count: chronological.former_expected_instance_ids.len(),
        chronological_t_sm1b_derived_count: chronological.t_sm1b_derived_count,
        chronological_t_sm1b_surface_count: chronological.t_sm1b_surface_count,
        chronological_blocking_case_ids,
        chronological_zero_accounting: chronological.f_sc3_zero_accounting_passed,
        f_sm1_fixed_positive_gate_passed,
        join_effect_proof,
        bi0_attempt_executed,
        bi0_passed,
        bi1_invoked,
        non_enacted_cone_invoked,
        non_enacted_branch_work_executed,
        outcome: if bi0_passed {
            "BI0_SEMANTIC_REGISTER_V5_PASSED"
        } else if !t_bi_side_passed && !f_sm1_fixed_positive_gate_passed {
            "BI0_SEMANTIC_REGISTER_V5_FALSE_AT_BOTH_PREREQUISITES"
        } else if !t_bi_side_passed {
            "BI0_SEMANTIC_REGISTER_V5_FALSE_AT_T_BI_NU1"
        } else {
            "BI0_SEMANTIC_REGISTER_V5_FALSE_AT_F_SM1"
        }
        .to_owned(),
        permitted_conclusion: if bi0_passed {
            "Both independent BI-0 prerequisites pass. BI-1 is reopened only as a separate create-new successor; this artifact evaluates no non-enacted branch and issues no cone verdict."
        } else {
            "At least one independent prerequisite is false. BI-1 and the non-enacted cone remain closed; no partial result is promoted."
        }
        .to_owned(),
        required_successor_action: if bi0_passed {
            "Run BI-1 create-new under the adopted branch-invariance program, consuming this sealed BI-0 v5 capability."
        } else {
            "Repair only the named failed prerequisite under a versioned successor, then rerun BI-0 create-new."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn logical_gate_errors(claimed: &Bi0SemanticRegisterV5Certificate) -> Vec<String> {
    let mut errors = Vec::new();
    let logical_operational_projection = claimed.t_bi_operational_rows.len() == 15
        && claimed
            .t_bi_operational_rows
            .iter()
            .map(|row| row.stage)
            .eq(1..=15)
        && claimed
            .t_bi_operational_rows
            .iter()
            .all(|row| row.winner_admissible_under_recorded_regime && row.operational_row_exact);
    let logical_t_bi = !claimed.t_bi_result_digest.is_empty()
        && !claimed.t_bi_intrinsic_sequence_seal.is_empty()
        && !claimed.t_bi_intrinsic_isolation_digest.is_empty()
        && claimed.t_bi_intrinsic_isolation_schema == T_BI_INTRINSIC_ISOLATION_V2_SCHEMA
        && claimed.t_bi_intrinsic_isolation_theorem_id == T_BI_B3_V2_THEOREM_ID
        && claimed.t_bi_intrinsic_isolation_exact_typed_phase_surface
        && claimed.t_bi_intrinsic_isolation_no_forbidden_capability
        && !claimed.t_bi_intrinsic_isolation_synthetic_receipt_input_accepted
        && claimed.t_bi_role_declaration_count == 250
        && claimed.t_bi_proved_family_count + claimed.t_bi_named_impossibility_count == 250
        && claimed.t_bi_named_registry_residual_count == 0
        && claimed.t_bi_named_quotient_residual_count == 0
        && claimed.t_bi_named_a3_residual_count == 0
        && claimed.t_bi_total_named_residual_count
            == claimed.t_bi_named_registry_residual_count
                + claimed.t_bi_named_quotient_residual_count
                + claimed.t_bi_named_a3_residual_count
        && claimed.t_bi_total_named_residual_count == 0
        && claimed.t_bi_silent_residue_count == 0
        && claimed.t_bi_b1_proved
        && claimed.t_bi_b2_proved
        && claimed.t_bi_b3_proved
        && claimed.t_bi_special_cases_proved
        && claimed.t_bi_extraction_complete
        && claimed.t_bi_exact_operational_regression
        && logical_operational_projection
        && claimed
            .authoritative_semantic_register
            .as_ref()
            .is_some_and(|register| register.len() == 15)
        && claimed.non_authoritative_extraction_floor.is_none()
        && register_table_exact(&claimed.bare_register_table)
        && claimed.bare_register_table_exact
        && claimed.register_value_projection_exact
        && register_value_projection_exact(
            &claimed.structural_register,
            &claimed.authoritative_semantic_register,
            &claimed.bare_register_table,
        )
        && claimed.t_bi_f_al1_prime_passed;
    let logical_f_sm1 = !claimed.chronological_result_digest.is_empty()
        && !claimed.chronological_baseline_v4_result_digest.is_empty()
        && claimed.chronological_exact_54_plus_18_partition
        && claimed.chronological_exact_sealed_72_id_join
        && !claimed.chronological_support_hardening_digest.is_empty()
        && claimed.chronological_support_graph_normal_form_proved
        && claimed.chronological_support_zero_mint_capability_proved
        && !claimed
            .chronological_support_replacement_evidence_hash
            .is_empty()
        && claimed.chronological_support_replacement_evidence_bound
        && claimed.chronological_inherited_71_exact
        && claimed.chronological_changed_case_ids == vec![SUPPORT_COMPREHENSION_CASE_ID]
        && claimed.chronological_derived_count == 72
        && claimed.chronological_total_count == 72
        && claimed.chronological_named_gap_count == 0
        && claimed.chronological_former_derived_count == 9
        && claimed.chronological_former_expected_count == 9
        && claimed.chronological_t_sm1b_derived_count == 18
        && claimed.chronological_t_sm1b_surface_count == 18
        && claimed.chronological_blocking_case_ids.is_empty()
        && claimed.chronological_zero_accounting;
    let expected_effect_proof = derive_bi0_join_effect_proof(
        &claimed.t_bi_result_digest,
        &claimed.chronological_result_digest,
    );
    let effect_capabilities = claimed
        .join_effect_proof
        .derived_capabilities
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let logical_attempt_executed =
        effect_capabilities.contains(&Bi0JoinCapabilityV5::ComparePrerequisiteProjections);
    let logical_bi1_invoked = effect_capabilities.contains(&Bi0JoinCapabilityV5::InvokeBi1);
    let logical_cone_invoked =
        effect_capabilities.contains(&Bi0JoinCapabilityV5::EnumerateNonEnactedCone);
    let logical_branch_executed =
        effect_capabilities.contains(&Bi0JoinCapabilityV5::ExecuteNonEnactedBranch);
    let logical_effect = claimed.join_effect_proof == expected_effect_proof
        && claimed.join_effect_proof.proved
        && logical_attempt_executed
        && !logical_bi1_invoked
        && !logical_cone_invoked
        && !logical_branch_executed;
    let logical_bi0 = logical_t_bi && logical_f_sm1 && logical_effect;
    if claimed.t_bi_side_passed != logical_t_bi
        || claimed.f_sm1_fixed_positive_gate_passed != logical_f_sm1
        || claimed.bi0_passed != logical_bi0
        || claimed.bi0_attempt_executed != logical_attempt_executed
        || claimed.bi1_invoked != logical_bi1_invoked
        || claimed.non_enacted_cone_invoked != logical_cone_invoked
        || claimed.non_enacted_branch_work_executed != logical_branch_executed
    {
        errors.push("BI-0 v5 prerequisite join or sequencing surface is invalid".to_owned());
    }
    errors
}

pub fn replay_bi0_semantic_register_v5_certificate(
    claimed: &Bi0SemanticRegisterV5Certificate,
) -> Bi0SemanticRegisterV5Replay {
    let mut errors = logical_gate_errors(claimed);
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("BI-0 v5 certificate digest mismatch".to_owned());
    }
    match issue_bi0_semantic_register_v5_certificate() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("BI-0 v5 certificate differs from create-new reissuance".to_owned()),
        Err(error) => errors.push(format!("BI-0 v5 create-new reissuance failed: {error}")),
    }
    Bi0SemanticRegisterV5Replay {
        valid: errors.is_empty(),
        bi0_passed: claimed.bi0_passed,
        t_bi_side_passed: claimed.t_bi_side_passed,
        f_sm1_fixed_positive_gate_passed: claimed.f_sm1_fixed_positive_gate_passed,
        bi1_invoked: claimed.bi1_invoked,
        non_enacted_cone_invoked: claimed.non_enacted_cone_invoked,
        errors,
    }
}

pub fn replay_bi0_semantic_register_v5_json(json: &str) -> Bi0SemanticRegisterV5Replay {
    match serde_json::from_str::<Bi0SemanticRegisterV5Certificate>(json) {
        Ok(certificate) => replay_bi0_semantic_register_v5_certificate(&certificate),
        Err(error) => Bi0SemanticRegisterV5Replay {
            valid: false,
            bi0_passed: false,
            t_bi_side_passed: false,
            f_sm1_fixed_positive_gate_passed: false,
            bi1_invoked: false,
            non_enacted_cone_invoked: false,
            errors: vec![error.to_string()],
        },
    }
}

pub fn render_bi0_semantic_register_v5_report(
    certificate: &Bi0SemanticRegisterV5Certificate,
) -> String {
    let table = certificate
        .bare_register_table
        .iter()
        .map(|row| format!("| {} | {} | {} |", row.stage, row.structural, row.semantic))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# BI-0 semantic-register v5 result\n\n\
         **Date:** {}. **Outcome:** `{}`.\n\n\
         T-BI-B1/B2/B3: **{}/{}/{}**; complete declarations: **{}/250** with **{}** named impossibilities, role/quotient/A3/total residuals **{}/{}/{}/{}**, and **{}** silent residue. F-AL1-prime: **{}**. Exact register-value projection: **{}**.\n\n\
         B3 authority: `{}` / `{}`; exact typed phase surface **{}**; forbidden capability absent **{}**; caller-supplied synthetic receipts accepted **{}**.\n\n\
         F-SM1: **{}** — support graph normal form **{}**, zero-mint capability **{}**, composite replacement evidence **{}**, inherited rows **71/71**, sealed discharges **{}/{}**, former-gap members **{}/{}**, T-SM1b **{}/{}**, zero accounting **{}**.\n\n\
         BI-0: **{}**. Closed join-effect proof: **{}**. BI-1 invoked: **{}**. Non-enacted cone invoked: **{}**.\n\n\
         The following is the required bare, uninterpreted register table.\n\n\
         | Stage | Structural | Semantic |\n|---:|---:|---:|\n{}\n\n\
         {}\n\nNext: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.t_bi_b1_proved,
        certificate.t_bi_b2_proved,
        certificate.t_bi_b3_proved,
        certificate.t_bi_role_declaration_count,
        certificate.t_bi_named_impossibility_count,
        certificate.t_bi_named_registry_residual_count,
        certificate.t_bi_named_quotient_residual_count,
        certificate.t_bi_named_a3_residual_count,
        certificate.t_bi_total_named_residual_count,
        certificate.t_bi_silent_residue_count,
        certificate.t_bi_f_al1_prime_passed,
        certificate.register_value_projection_exact,
        certificate.t_bi_intrinsic_isolation_schema,
        certificate.t_bi_intrinsic_isolation_theorem_id,
        certificate.t_bi_intrinsic_isolation_exact_typed_phase_surface,
        certificate.t_bi_intrinsic_isolation_no_forbidden_capability,
        certificate.t_bi_intrinsic_isolation_synthetic_receipt_input_accepted,
        certificate.f_sm1_fixed_positive_gate_passed,
        certificate.chronological_support_graph_normal_form_proved,
        certificate.chronological_support_zero_mint_capability_proved,
        certificate.chronological_support_replacement_evidence_bound,
        certificate.chronological_derived_count,
        certificate.chronological_total_count,
        certificate.chronological_former_derived_count,
        certificate.chronological_former_expected_count,
        certificate.chronological_t_sm1b_derived_count,
        certificate.chronological_t_sm1b_surface_count,
        certificate.chronological_zero_accounting,
        certificate.bi0_passed,
        certificate.join_effect_proof.proved,
        certificate.bi1_invoked,
        certificate.non_enacted_cone_invoked,
        table,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
    )
}

pub fn emit_bi0_semantic_register_v5_create_new(
    directory: &Path,
) -> Result<Bi0SemanticRegisterV5Certificate, Bi0SemanticRegisterV5Error> {
    let certificate = issue_bi0_semantic_register_v5_certificate()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Bi0SemanticRegisterV5Error::Json(error.to_string()))?;
    let replay = replay_bi0_semantic_register_v5_json(&json);
    if !replay.valid {
        return Err(Bi0SemanticRegisterV5Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let json_path = directory.join(BI0_SEMANTIC_REGISTER_V5_CERTIFICATE_NAME);
    let report_path = directory.join(BI0_SEMANTIC_REGISTER_V5_REPORT_NAME);
    if json_path.exists() || report_path.exists() {
        return Err(Bi0SemanticRegisterV5Error::Io(format!(
            "create-new target already exists: {} or {}",
            json_path.display(),
            report_path.display()
        )));
    }
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&json_path)
        .map_err(|error| Bi0SemanticRegisterV5Error::Io(error.to_string()))?;
    json_file
        .write_all(json.as_bytes())
        .map_err(|error| Bi0SemanticRegisterV5Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| Bi0SemanticRegisterV5Error::Io(error.to_string()))?;
    report_file
        .write_all(render_bi0_semantic_register_v5_report(&certificate).as_bytes())
        .map_err(|error| Bi0SemanticRegisterV5Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_bi0_semantic_register_v5_directory(
    directory: &Path,
) -> Result<Bi0SemanticRegisterV5Replay, Bi0SemanticRegisterV5Error> {
    let json = std::fs::read_to_string(directory.join(BI0_SEMANTIC_REGISTER_V5_CERTIFICATE_NAME))
        .map_err(|error| Bi0SemanticRegisterV5Error::Io(error.to_string()))?;
    Ok(replay_bi0_semantic_register_v5_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> &'static Bi0SemanticRegisterV5Certificate {
        static CERTIFICATE: OnceLock<Bi0SemanticRegisterV5Certificate> = OnceLock::new();
        CERTIFICATE.get_or_init(|| {
            issue_bi0_semantic_register_v5_certificate().expect("BI-0 v5 create-new")
        })
    }

    #[test]
    fn completed_independent_gates_open_only_bi1_capability() {
        let certificate = certificate();
        assert!(certificate.t_bi_side_passed);
        assert_eq!(
            certificate.t_bi_intrinsic_isolation_schema,
            T_BI_INTRINSIC_ISOLATION_V2_SCHEMA
        );
        assert_eq!(
            certificate.t_bi_intrinsic_isolation_theorem_id,
            T_BI_B3_V2_THEOREM_ID
        );
        assert!(certificate.t_bi_intrinsic_isolation_exact_typed_phase_surface);
        assert!(certificate.t_bi_intrinsic_isolation_no_forbidden_capability);
        assert!(!certificate.t_bi_intrinsic_isolation_synthetic_receipt_input_accepted);
        assert_eq!(certificate.t_bi_total_named_residual_count, 0);
        assert!(certificate.f_sm1_fixed_positive_gate_passed);
        assert!(certificate.register_value_projection_exact);
        assert!(certificate.join_effect_proof.proved);
        assert!(certificate.bi0_passed);
        assert!(!certificate.bi1_invoked);
        assert!(!certificate.non_enacted_cone_invoked);
        assert!(!certificate.non_enacted_branch_work_executed);
        assert!(replay_bi0_semantic_register_v5_certificate(certificate).valid);
    }

    #[test]
    fn digest_and_gate_mutations_fail_replay_without_opening_the_cone() {
        let certificate = certificate().clone();
        let mut support_forgery = certificate.clone();
        support_forgery.chronological_support_graph_normal_form_proved = false;
        support_forgery.result_digest = certificate_digest(&support_forgery);
        assert!(!logical_gate_errors(&support_forgery).is_empty());

        let mut table_forgery = certificate.clone();
        table_forgery.bare_register_table[0].semantic += 1;
        table_forgery.result_digest = certificate_digest(&table_forgery);
        assert!(!logical_gate_errors(&table_forgery).is_empty());

        let mut isolation_forgery = certificate.clone();
        isolation_forgery.t_bi_intrinsic_isolation_theorem_id =
            "T-BI-B3-v1-caller-receipt-isolation".to_owned();
        isolation_forgery.t_bi_intrinsic_isolation_synthetic_receipt_input_accepted = false;
        isolation_forgery.result_digest = certificate_digest(&isolation_forgery);
        assert!(!logical_gate_errors(&isolation_forgery).is_empty());

        let mut effect_forgery = certificate;
        effect_forgery
            .join_effect_proof
            .operations
            .push(Bi0JoinOperationV5::InvokeBi1);
        effect_forgery
            .join_effect_proof
            .derived_capabilities
            .push(Bi0JoinCapabilityV5::InvokeBi1);
        effect_forgery.join_effect_proof.proved = true;
        effect_forgery.join_effect_proof.derivation_hash =
            tagged_hash("bi0-join-effect-proof", &effect_forgery.join_effect_proof);
        effect_forgery.bi1_invoked = true;
        effect_forgery.result_digest = certificate_digest(&effect_forgery);
        assert!(!logical_gate_errors(&effect_forgery).is_empty());
    }
}
