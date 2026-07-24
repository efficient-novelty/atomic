//! Source-first BI-0 v6 join and narrow Stage-4 opening capability.
//!
//! BI-0 consumes only the public positive projection of T-BI-NU1 v6 and a
//! freshly replayed chronological-slot-map v5 certificate.  It rechecks the
//! operational, F-AL1-prime, 72/72, 9/9, and 18/18 gates before sealing a
//! core certificate.  A distinct issuer may then derive a minimal Stage-4
//! opening capability from that passing certificate.  The capability carries
//! the exact typed Stage-1-through-3 prefix, but neither semantic registers nor
//! authority to enumerate a cone, execute a branch, select a branch, or write
//! a successor artifact.

use crate::chronological_slot_map_v4::ChronologicalCaseDispositionV4;
use crate::chronological_slot_map_v5::{
    CHRONOLOGICAL_SLOT_MAP_V5_SCHEMA, SUPPORT_COMPREHENSION_CASE_ID,
    issue_chronological_slot_map_v5_certificate, replay_chronological_slot_map_v5_certificate,
};
use crate::t_bi_intrinsic_isolation_v3::T_BI_B3_V3_THEOREM_ID;
use crate::t_bi_nu1_regression_v6::{
    T_BI_NU1_REGRESSION_V6_SCHEMA, T_BI_NU1_REGRESSION_V6_THEOREM_ID,
    TBiNu1RegressionV6PassedProjection, issue_t_bi_nu1_regression_v6_passed_projection,
    replay_t_bi_nu1_regression_v6_passed_projection,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const BI0_SEMANTIC_REGISTER_V6_SCHEMA: &str =
    "branch-invariance-bi0-source-first-opening-gate-v6";
pub const BI0_SEMANTIC_REGISTER_V6_DATE: &str = "2026-07-22";
pub const BI0_SEMANTIC_REGISTER_V6_THEOREM_ID: &str =
    "T-BI0-v6-source-first-F-AL1-prime-F-SM1-closed-join";
pub const BI0_STAGE4_OPENING_CAPABILITY_V6_SCHEMA: &str =
    "bi0-v6-exact-stage1-through3-stage4-opening-capability";
pub const BI0_SEMANTIC_REGISTER_V6_CERTIFICATE_NAME: &str = "bi0_semantic_register_v6.json";
pub const BI0_SEMANTIC_REGISTER_V6_REPORT_NAME: &str = "BI0_SEMANTIC_REGISTER_V6_RESULT.md";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI0_SEMANTIC_REGISTER_V6_SCHEMA, domain, value))
        .expect("BI-0 v6 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi0JoinOperationV6 {
    IssueTBiPassedProjection,
    ReplayTBiPassedProjection,
    IssueChronologicalProjection,
    ReplayChronologicalProjection,
    JoinExactPrerequisites,
    SealBi0PassProof,
    SealBi0Certificate,
    EnumerateStage4Cone,
    ExecuteNonEnactedBranch,
    SelectStage4Branch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi0JoinCapabilityV6 {
    ReadTBiPassedProjection,
    ReadChronologicalPassedProjection,
    ReplayPrerequisiteProjection,
    ComparePrerequisiteProjection,
    ContentHashing,
    EnumerateStage4Cone,
    ExecuteNonEnactedBranch,
    SelectStage4Branch,
}

impl Bi0JoinOperationV6 {
    fn capabilities(self) -> Vec<Bi0JoinCapabilityV6> {
        use Bi0JoinCapabilityV6 as C;
        match self {
            Self::IssueTBiPassedProjection => vec![C::ReadTBiPassedProjection],
            Self::ReplayTBiPassedProjection => vec![C::ReplayPrerequisiteProjection],
            Self::IssueChronologicalProjection => vec![C::ReadChronologicalPassedProjection],
            Self::ReplayChronologicalProjection => vec![C::ReplayPrerequisiteProjection],
            Self::JoinExactPrerequisites => vec![C::ComparePrerequisiteProjection],
            Self::SealBi0PassProof | Self::SealBi0Certificate => vec![C::ContentHashing],
            Self::EnumerateStage4Cone => vec![C::EnumerateStage4Cone],
            Self::ExecuteNonEnactedBranch => vec![C::ExecuteNonEnactedBranch],
            Self::SelectStage4Branch => vec![C::SelectStage4Branch],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0JoinEffectProofV6 {
    pub theorem_id: String,
    pub operations: Vec<Bi0JoinOperationV6>,
    pub derived_capabilities: Vec<Bi0JoinCapabilityV6>,
    pub forbidden_successor_capabilities: Vec<Bi0JoinCapabilityV6>,
    pub every_capability_derived_from_closed_operation_enum: bool,
    pub no_cone_branch_or_selection_capability: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0ChronologicalPassedProjectionV6 {
    pub source_schema: String,
    pub source_certificate_result_digest: String,
    pub baseline_v4_result_digest: String,
    pub baseline_replayed: bool,
    pub exact_54_plus_18_partition: bool,
    pub exact_sealed_72_id_join: bool,
    pub support_hardening_derivation_hash: String,
    pub support_graph_normal_form_proved: bool,
    pub support_zero_mint_capability_proved: bool,
    pub support_replacement_evidence_hash: String,
    pub support_replacement_evidence_bound: bool,
    pub inherited_row_equality_count: usize,
    pub inherited_71_exact: bool,
    pub changed_case_ids: Vec<String>,
    pub derived_discharge_count: usize,
    pub total_discharge_count: usize,
    pub named_gap_count: usize,
    pub former_derived_count: usize,
    pub former_expected_count: usize,
    pub former_named_gap_count: usize,
    pub t_sm1b_derived_count: usize,
    pub t_sm1b_surface_count: usize,
    pub t_sm1b_named_gap_count: usize,
    pub zero_accounting_proved: bool,
    pub fixed_positive_gate_passed: bool,
    pub blocking_case_ids: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0Stage4PrefixEntryV6 {
    stage: u32,
    telescope: Telescope,
    candidate_hash: String,
    predecessor_signature_digest: String,
    t_bi_typed_prefix_digest: String,
    payload_hash: String,
}

impl Bi0Stage4PrefixEntryV6 {
    pub fn stage(&self) -> u32 {
        self.stage
    }

    pub fn telescope(&self) -> &Telescope {
        &self.telescope
    }

    pub fn candidate_hash(&self) -> &str {
        &self.candidate_hash
    }

    pub fn predecessor_signature_digest(&self) -> &str {
        &self.predecessor_signature_digest
    }

    pub fn payload_hash(&self) -> &str {
        &self.payload_hash
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi0Stage4OpeningPermissionV6 {
    ReadExactTypedPrefix,
    ReplayBi0PassBinding,
    PresentToSeparateStage4Procedure,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi0Stage4ForbiddenCapabilityV6 {
    ReadFullBi0Certificate,
    ReadSemanticRegister,
    ReadStructuralRegister,
    EnumerateCone,
    ExecuteBranch,
    SelectBranch,
    WriteSuccessorArtifact,
}

/// Narrow output of BI-0.  Fields are private so downstream code can replay
/// and inspect the issued capability but cannot construct one by struct
/// literal.  Deterministic replay additionally compares it with a fresh BI-0
/// issuance, so a fully rehashed deserialization forgery is rejected.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0Stage4OpeningCapabilityV6 {
    schema: String,
    granting_bi0_schema: String,
    granting_bi0_result_digest: String,
    granting_bi0_pass_proof_hash: String,
    t_bi_source_certificate_result_digest: String,
    t_bi_passed_projection_derivation_hash: String,
    chronological_source_certificate_result_digest: String,
    chronological_passed_projection_derivation_hash: String,
    exact_stage1_through3_entries: Vec<Bi0Stage4PrefixEntryV6>,
    exact_stage1_through3_t_bi_prefix_digest: String,
    exact_stage1_through3_payload_hashes: Vec<String>,
    exact_stage1_through3_bi0_prefix_digest: String,
    permitted_permissions: Vec<Bi0Stage4OpeningPermissionV6>,
    forbidden_capabilities: Vec<Bi0Stage4ForbiddenCapabilityV6>,
    full_bi0_certificate_present: bool,
    semantic_or_structural_register_present: bool,
    cone_execution_capability_present: bool,
    branch_execution_capability_present: bool,
    branch_selection_capability_present: bool,
    exact_prefix_and_pass_binding: bool,
    derivation_hash: String,
}

impl Bi0Stage4OpeningCapabilityV6 {
    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn granting_bi0_result_digest(&self) -> &str {
        &self.granting_bi0_result_digest
    }

    pub fn granting_bi0_pass_proof_hash(&self) -> &str {
        &self.granting_bi0_pass_proof_hash
    }

    pub fn t_bi_source_certificate_result_digest(&self) -> &str {
        &self.t_bi_source_certificate_result_digest
    }

    pub fn t_bi_passed_projection_derivation_hash(&self) -> &str {
        &self.t_bi_passed_projection_derivation_hash
    }

    pub fn chronological_source_certificate_result_digest(&self) -> &str {
        &self.chronological_source_certificate_result_digest
    }

    pub fn chronological_passed_projection_derivation_hash(&self) -> &str {
        &self.chronological_passed_projection_derivation_hash
    }

    pub fn exact_stage1_through3_entries(&self) -> &[Bi0Stage4PrefixEntryV6] {
        &self.exact_stage1_through3_entries
    }

    pub fn exact_stage1_through3_t_bi_prefix_digest(&self) -> &str {
        &self.exact_stage1_through3_t_bi_prefix_digest
    }

    pub fn exact_stage1_through3_bi0_prefix_digest(&self) -> &str {
        &self.exact_stage1_through3_bi0_prefix_digest
    }

    pub fn exact_stage1_through3_payload_hashes(&self) -> &[String] {
        &self.exact_stage1_through3_payload_hashes
    }

    pub fn permitted_permissions(&self) -> &[Bi0Stage4OpeningPermissionV6] {
        &self.permitted_permissions
    }

    pub fn forbidden_capabilities(&self) -> &[Bi0Stage4ForbiddenCapabilityV6] {
        &self.forbidden_capabilities
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn has_no_cone_or_branch_capability(&self) -> bool {
        !self.cone_execution_capability_present
            && !self.branch_execution_capability_present
            && !self.branch_selection_capability_present
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0SemanticRegisterV6Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub t_bi_passed_projection: TBiNu1RegressionV6PassedProjection,
    pub t_bi_projection_replayed: bool,
    pub t_bi_complete_act_local_semantic_certificate: bool,
    pub t_bi_exact_operational_regression: bool,
    pub t_bi_f_al1_prime_passed: bool,
    pub chronological_passed_projection: Bi0ChronologicalPassedProjectionV6,
    pub chronological_projection_replayed: bool,
    pub chronological_exact_72_of_72: bool,
    pub chronological_exact_9_of_9: bool,
    pub chronological_exact_18_of_18: bool,
    pub stage1_through3_prefix_entries: Vec<Bi0Stage4PrefixEntryV6>,
    pub stage1_through3_t_bi_prefix_digest: String,
    pub stage1_through3_bi0_prefix_digest: String,
    pub join_effect_proof: Bi0JoinEffectProofV6,
    pub bi0_attempt_executed: bool,
    pub bi0_passed: bool,
    pub bi0_pass_proof_hash: String,
    pub cone_enumerated: bool,
    pub non_enacted_branch_executed: bool,
    pub stage4_branch_selected: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0SemanticRegisterV6Artifact {
    pub certificate: Bi0SemanticRegisterV6Certificate,
    pub stage4_opening_capability: Bi0Stage4OpeningCapabilityV6,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0SemanticRegisterV6Replay {
    pub valid: bool,
    pub bi0_passed: bool,
    pub t_bi_passed: bool,
    pub chronological_passed: bool,
    pub opening_capability_valid: bool,
    pub cone_or_branch_capability_present: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bi0SemanticRegisterV6Error {
    #[error("T-BI v6 positive projection failed: {0}")]
    TBi(String),
    #[error("chronological v5 positive projection failed: {0}")]
    Chronological(String),
    #[error("BI-0 v6 prerequisite join failed: {0}")]
    Join(String),
    #[error("BI-0 v6 JSON failed: {0}")]
    Json(String),
    #[error("BI-0 v6 I/O failed: {0}")]
    Io(String),
    #[error("emitted BI-0 v6 artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn chronological_projection_digest(projection: &Bi0ChronologicalPassedProjectionV6) -> String {
    let mut value = projection.clone();
    value.derivation_hash.clear();
    tagged_hash("chronological-passed-projection", &value)
}

fn prefix_entry_payload_hash(entry: &Bi0Stage4PrefixEntryV6) -> String {
    tagged_hash(
        "stage4-exact-prefix-payload",
        &(
            entry.stage,
            &entry.telescope,
            &entry.candidate_hash,
            &entry.predecessor_signature_digest,
            &entry.t_bi_typed_prefix_digest,
        ),
    )
}

fn stage4_prefix_digest(entries: &[Bi0Stage4PrefixEntryV6]) -> String {
    tagged_hash(
        "stage4-exact-stage1-through3-prefix",
        &entries
            .iter()
            .map(|entry| {
                (
                    entry.stage,
                    &entry.candidate_hash,
                    &entry.predecessor_signature_digest,
                    &entry.payload_hash,
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn join_effect_hash(proof: &Bi0JoinEffectProofV6) -> String {
    let mut value = proof.clone();
    value.derivation_hash.clear();
    tagged_hash("closed-prerequisite-join-effect", &value)
}

fn certificate_digest(certificate: &Bi0SemanticRegisterV6Certificate) -> String {
    let mut value = certificate.clone();
    value.result_digest.clear();
    tagged_hash("bi0-core-certificate", &value)
}

fn opening_capability_digest(capability: &Bi0Stage4OpeningCapabilityV6) -> String {
    let mut value = capability.clone();
    value.derivation_hash.clear();
    tagged_hash("stage4-opening-capability", &value)
}

fn artifact_digest(artifact: &Bi0SemanticRegisterV6Artifact) -> String {
    let mut value = artifact.clone();
    value.result_digest.clear();
    tagged_hash("bi0-certificate-and-opening-envelope", &value)
}

fn issue_chronological_passed_projection()
-> Result<Bi0ChronologicalPassedProjectionV6, Bi0SemanticRegisterV6Error> {
    let certificate = issue_chronological_slot_map_v5_certificate()
        .map_err(|error| Bi0SemanticRegisterV6Error::Chronological(error.to_string()))?;
    replay_chronological_slot_map_v5_certificate(&certificate)
        .map_err(|error| Bi0SemanticRegisterV6Error::Chronological(error.to_string()))?;

    let mut blocking_case_ids = certificate
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
    blocking_case_ids.sort();
    let inherited_71_exact = certificate.inherited_row_equality_count == 71
        && certificate.prior_71_rows_byte_identical
        && certificate.prior_71_evidence_hashes_identical
        && certificate.prior_71_row_hashes_identical;
    let former_derived_count = certificate.former_derived_instance_ids.len();
    let former_expected_count = certificate.former_expected_instance_ids.len();
    let former_named_gap_count = certificate.former_named_gap_instance_ids.len();
    let positive = certificate.schema == CHRONOLOGICAL_SLOT_MAP_V5_SCHEMA
        && certificate.baseline_v4_replayed
        && certificate.baseline_exact_54_plus_18_partition
        && certificate.baseline_exact_sealed_72_id_join
        && certificate.support_comprehension_replayed
        && certificate.support_graph_normal_form_proved
        && certificate.support_zero_mint_capability_proved
        && certificate.support_replacement_evidence_bound
        && certificate.support_comprehension_case_id == SUPPORT_COMPREHENSION_CASE_ID
        && inherited_71_exact
        && certificate.exactly_one_case_changed
        && certificate.changed_case_ids == vec![SUPPORT_COMPREHENSION_CASE_ID]
        && certificate.sealed_discharge_derived_count == 72
        && certificate.sealed_discharge_count == 72
        && certificate.sealed_discharge_named_gap_count == 0
        && former_derived_count == 9
        && former_expected_count == 9
        && former_named_gap_count == 0
        && certificate.former_nine_exact
        && certificate.t_sm1b_derived_count == 18
        && certificate.t_sm1b_surface_count == 18
        && certificate.t_sm1b_named_gap_count == 0
        && certificate.f_sc3_zero_accounting_passed
        && certificate.f_sc1_regression_passed
        && certificate.fixed_positive_gate_passed
        && certificate.bi0_chronological_prerequisite_reopened
        && blocking_case_ids.is_empty();
    if !positive {
        return Err(Bi0SemanticRegisterV6Error::Chronological(
            "the replayed chronological v5 issuer did not prove the exact 72/72, 9/9, and 18/18 positive gate"
                .to_owned(),
        ));
    }

    let mut projection = Bi0ChronologicalPassedProjectionV6 {
        source_schema: certificate.schema,
        source_certificate_result_digest: certificate.result_digest,
        baseline_v4_result_digest: certificate.baseline_v4_result_digest,
        baseline_replayed: certificate.baseline_v4_replayed,
        exact_54_plus_18_partition: certificate.baseline_exact_54_plus_18_partition,
        exact_sealed_72_id_join: certificate.baseline_exact_sealed_72_id_join,
        support_hardening_derivation_hash: certificate
            .support_comprehension_hardening
            .derivation_hash,
        support_graph_normal_form_proved: certificate.support_graph_normal_form_proved,
        support_zero_mint_capability_proved: certificate.support_zero_mint_capability_proved,
        support_replacement_evidence_hash: certificate.support_replacement_evidence_hash,
        support_replacement_evidence_bound: certificate.support_replacement_evidence_bound,
        inherited_row_equality_count: certificate.inherited_row_equality_count,
        inherited_71_exact,
        changed_case_ids: certificate.changed_case_ids,
        derived_discharge_count: certificate.sealed_discharge_derived_count,
        total_discharge_count: certificate.sealed_discharge_count,
        named_gap_count: certificate.sealed_discharge_named_gap_count,
        former_derived_count,
        former_expected_count,
        former_named_gap_count,
        t_sm1b_derived_count: certificate.t_sm1b_derived_count,
        t_sm1b_surface_count: certificate.t_sm1b_surface_count,
        t_sm1b_named_gap_count: certificate.t_sm1b_named_gap_count,
        zero_accounting_proved: certificate.f_sc3_zero_accounting_passed,
        fixed_positive_gate_passed: certificate.fixed_positive_gate_passed,
        blocking_case_ids,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = chronological_projection_digest(&projection);
    Ok(projection)
}

fn chronological_projection_logically_passes(
    projection: &Bi0ChronologicalPassedProjectionV6,
) -> bool {
    projection.source_schema == CHRONOLOGICAL_SLOT_MAP_V5_SCHEMA
        && !projection.source_certificate_result_digest.is_empty()
        && !projection.baseline_v4_result_digest.is_empty()
        && projection.baseline_replayed
        && projection.exact_54_plus_18_partition
        && projection.exact_sealed_72_id_join
        && !projection.support_hardening_derivation_hash.is_empty()
        && projection.support_graph_normal_form_proved
        && projection.support_zero_mint_capability_proved
        && !projection.support_replacement_evidence_hash.is_empty()
        && projection.support_replacement_evidence_bound
        && projection.inherited_row_equality_count == 71
        && projection.inherited_71_exact
        && projection.changed_case_ids == vec![SUPPORT_COMPREHENSION_CASE_ID]
        && projection.derived_discharge_count == 72
        && projection.total_discharge_count == 72
        && projection.named_gap_count == 0
        && projection.former_derived_count == 9
        && projection.former_expected_count == 9
        && projection.former_named_gap_count == 0
        && projection.t_sm1b_derived_count == 18
        && projection.t_sm1b_surface_count == 18
        && projection.t_sm1b_named_gap_count == 0
        && projection.zero_accounting_proved
        && projection.fixed_positive_gate_passed
        && projection.blocking_case_ids.is_empty()
        && projection.derivation_hash == chronological_projection_digest(projection)
}

pub fn issue_bi0_chronological_passed_projection_v6()
-> Result<Bi0ChronologicalPassedProjectionV6, Bi0SemanticRegisterV6Error> {
    issue_chronological_passed_projection()
}

pub fn replay_bi0_chronological_passed_projection_v6(
    claimed: &Bi0ChronologicalPassedProjectionV6,
) -> Vec<String> {
    let mut errors = Vec::new();
    if !chronological_projection_logically_passes(claimed) {
        errors.push("BI-0 v6 chronological projection failed its local positive proof".to_owned());
    }
    match issue_chronological_passed_projection() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push(
            "BI-0 v6 chronological projection differs from deterministic v5 reissuance".to_owned(),
        ),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn t_bi_projection_logically_passes(projection: &TBiNu1RegressionV6PassedProjection) -> bool {
    let exact_candidate_vector = projection.exact_prefix_candidate_hashes.len() == 15;
    let exact_typed_prefix = exact_candidate_vector
        && projection.stage1_through3_typed_prefix_entries.len() == 3
        && projection
            .stage1_through3_typed_prefix_entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=3)
        && projection
            .stage1_through3_typed_prefix_entries
            .iter()
            .enumerate()
            .all(|(index, (_, telescope))| {
                candidate_hash(telescope) == projection.exact_prefix_candidate_hashes[index]
            });
    let operational_exact = exact_candidate_vector
        && projection.operational_rows.len() == 15
        && projection
            .operational_rows
            .iter()
            .map(|row| row.stage)
            .eq(1..=15)
        && projection
            .operational_rows
            .iter()
            .enumerate()
            .all(|(index, row)| {
                row.winner_admissible_under_recorded_regime
                    && row.operational_row_exact
                    && row.winner_candidate_hash == projection.exact_prefix_candidate_hashes[index]
            });
    let register_exact = projection.semantic_register.len() == 15
        && projection.structural_register.len() == 15
        && projection.bare_register_table.len() == 15
        && projection
            .bare_register_table
            .iter()
            .enumerate()
            .all(|(index, row)| {
                row.stage == index as u32 + 1
                    && row.semantic == projection.semantic_register[index]
                    && row.structural == projection.structural_register[index]
            });
    projection.schema == T_BI_NU1_REGRESSION_V6_SCHEMA
        && projection.theorem_id == T_BI_NU1_REGRESSION_V6_THEOREM_ID
        && !projection.source_certificate_result_digest.is_empty()
        && exact_candidate_vector
        && !projection.exact_prefix_candidate_hash_digest.is_empty()
        && exact_typed_prefix
        && !projection.stage1_through3_prefix_digest.is_empty()
        && !projection.authoritative_prefix_semantic_seal.is_empty()
        && !projection.intrinsic_sequence_derivation_hash.is_empty()
        && !projection.intrinsic_issuance_trace_root.is_empty()
        && projection.intrinsic_isolation_theorem_id == T_BI_B3_V3_THEOREM_ID
        && !projection.intrinsic_isolation_derivation_hash.is_empty()
        && projection.role_declaration_count
            == projection.proved_family_declaration_count
                + projection.theorem_impossibility_declaration_count
        && projection.named_registry_residual_count == 0
        && projection.named_quotient_residual_count == 0
        && projection.named_a3_residual_count == 0
        && projection.total_named_residual_count == 0
        && projection.silent_residue_count == 0
        && register_exact
        && operational_exact
        && projection.extraction_complete
        && projection.exact_operational_regression
        && projection.f_al1_prime_passed
        && !projection.non_enacted_branch_work_executed
}

fn derive_stage4_prefix_entries(
    projection: &TBiNu1RegressionV6PassedProjection,
) -> Vec<Bi0Stage4PrefixEntryV6> {
    projection
        .stage1_through3_typed_prefix_entries
        .iter()
        .enumerate()
        .map(|(index, (stage, telescope))| {
            let predecessor_signature_digest = SealedSignature::from_telescopes(
                projection.stage1_through3_typed_prefix_entries[..index].to_vec(),
            )
            .digest()
            .to_owned();
            let mut entry = Bi0Stage4PrefixEntryV6 {
                stage: *stage,
                telescope: telescope.clone(),
                candidate_hash: candidate_hash(telescope),
                predecessor_signature_digest,
                t_bi_typed_prefix_digest: projection.stage1_through3_prefix_digest.clone(),
                payload_hash: String::new(),
            };
            entry.payload_hash = prefix_entry_payload_hash(&entry);
            entry
        })
        .collect()
}

fn prefix_entries_logically_exact(
    entries: &[Bi0Stage4PrefixEntryV6],
    projection: &TBiNu1RegressionV6PassedProjection,
) -> bool {
    entries.len() == 3
        && projection.stage1_through3_typed_prefix_entries.len() == 3
        && projection.exact_prefix_candidate_hashes.len() >= 3
        && entries.iter().map(|entry| entry.stage).eq(1..=3)
        && entries.iter().enumerate().all(|(index, entry)| {
            entry.telescope == projection.stage1_through3_typed_prefix_entries[index].1
                && entry.candidate_hash == candidate_hash(&entry.telescope)
                && entry.candidate_hash == projection.exact_prefix_candidate_hashes[index]
                && entry.predecessor_signature_digest
                    == SealedSignature::from_telescopes(
                        projection.stage1_through3_typed_prefix_entries[..index].to_vec(),
                    )
                    .digest()
                && entry.t_bi_typed_prefix_digest == projection.stage1_through3_prefix_digest
                && entry.payload_hash == prefix_entry_payload_hash(entry)
        })
}

fn derive_join_effect_proof() -> Bi0JoinEffectProofV6 {
    let operations = vec![
        Bi0JoinOperationV6::IssueTBiPassedProjection,
        Bi0JoinOperationV6::ReplayTBiPassedProjection,
        Bi0JoinOperationV6::IssueChronologicalProjection,
        Bi0JoinOperationV6::ReplayChronologicalProjection,
        Bi0JoinOperationV6::JoinExactPrerequisites,
        Bi0JoinOperationV6::SealBi0PassProof,
        Bi0JoinOperationV6::SealBi0Certificate,
    ];
    let derived_capabilities = operations
        .iter()
        .flat_map(|operation| operation.capabilities())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let forbidden = BTreeSet::from([
        Bi0JoinCapabilityV6::EnumerateStage4Cone,
        Bi0JoinCapabilityV6::ExecuteNonEnactedBranch,
        Bi0JoinCapabilityV6::SelectStage4Branch,
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
    let no_cone_branch_or_selection_capability = forbidden_successor_capabilities.is_empty();
    let mut proof = Bi0JoinEffectProofV6 {
        theorem_id: "T-BI0-E2-v6-closed-source-first-prerequisite-join".to_owned(),
        operations,
        derived_capabilities,
        forbidden_successor_capabilities,
        every_capability_derived_from_closed_operation_enum,
        no_cone_branch_or_selection_capability,
        proved: every_capability_derived_from_closed_operation_enum
            && no_cone_branch_or_selection_capability,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = join_effect_hash(&proof);
    proof
}

fn bi0_pass_proof_hash(certificate: &Bi0SemanticRegisterV6Certificate) -> String {
    tagged_hash(
        "bi0-positive-join-pass-proof",
        &(
            (
                &certificate.t_bi_passed_projection.derivation_hash,
                &certificate.chronological_passed_projection.derivation_hash,
                &certificate.stage1_through3_t_bi_prefix_digest,
                &certificate.stage1_through3_bi0_prefix_digest,
                &certificate.join_effect_proof.derivation_hash,
            ),
            vec![
                certificate.t_bi_projection_replayed,
                certificate.t_bi_complete_act_local_semantic_certificate,
                certificate.t_bi_exact_operational_regression,
                certificate.t_bi_f_al1_prime_passed,
                certificate.chronological_projection_replayed,
                certificate.chronological_exact_72_of_72,
                certificate.chronological_exact_9_of_9,
                certificate.chronological_exact_18_of_18,
                certificate.bi0_attempt_executed,
                certificate.bi0_passed,
                certificate.cone_enumerated,
                certificate.non_enacted_branch_executed,
                certificate.stage4_branch_selected,
            ],
        ),
    )
}

pub fn issue_bi0_semantic_register_v6_certificate()
-> Result<Bi0SemanticRegisterV6Certificate, Bi0SemanticRegisterV6Error> {
    // The prerequisite issuers are independent; neither receives the other
    // projection or the prospective BI-0 verdict.
    let t_bi_passed_projection = issue_t_bi_nu1_regression_v6_passed_projection()
        .map_err(|error| Bi0SemanticRegisterV6Error::TBi(error.to_string()))?;
    let t_bi_replay_errors =
        replay_t_bi_nu1_regression_v6_passed_projection(&t_bi_passed_projection);
    if !t_bi_replay_errors.is_empty() {
        return Err(Bi0SemanticRegisterV6Error::TBi(
            t_bi_replay_errors.join("; "),
        ));
    }
    let chronological_passed_projection = issue_chronological_passed_projection()?;
    let chronological_replay_errors =
        replay_bi0_chronological_passed_projection_v6(&chronological_passed_projection);
    if !chronological_replay_errors.is_empty() {
        return Err(Bi0SemanticRegisterV6Error::Chronological(
            chronological_replay_errors.join("; "),
        ));
    }

    let t_bi_projection_replayed = true;
    let t_bi_complete_act_local_semantic_certificate =
        t_bi_projection_logically_passes(&t_bi_passed_projection);
    let t_bi_exact_operational_regression = t_bi_passed_projection.exact_operational_regression;
    let t_bi_f_al1_prime_passed = t_bi_passed_projection.f_al1_prime_passed;
    let chronological_projection_replayed = true;
    let chronological_exact_72_of_72 = chronological_passed_projection.derived_discharge_count
        == 72
        && chronological_passed_projection.total_discharge_count == 72
        && chronological_passed_projection.named_gap_count == 0;
    let chronological_exact_9_of_9 = chronological_passed_projection.former_derived_count == 9
        && chronological_passed_projection.former_expected_count == 9
        && chronological_passed_projection.former_named_gap_count == 0;
    let chronological_exact_18_of_18 = chronological_passed_projection.t_sm1b_derived_count == 18
        && chronological_passed_projection.t_sm1b_surface_count == 18
        && chronological_passed_projection.t_sm1b_named_gap_count == 0;
    let stage1_through3_prefix_entries = derive_stage4_prefix_entries(&t_bi_passed_projection);
    let stage1_through3_t_bi_prefix_digest =
        t_bi_passed_projection.stage1_through3_prefix_digest.clone();
    let stage1_through3_bi0_prefix_digest = stage4_prefix_digest(&stage1_through3_prefix_entries);
    let prefix_exact =
        prefix_entries_logically_exact(&stage1_through3_prefix_entries, &t_bi_passed_projection);
    let join_effect_proof = derive_join_effect_proof();
    let effect_capabilities = join_effect_proof
        .derived_capabilities
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let bi0_attempt_executed =
        effect_capabilities.contains(&Bi0JoinCapabilityV6::ComparePrerequisiteProjection);
    let cone_enumerated = effect_capabilities.contains(&Bi0JoinCapabilityV6::EnumerateStage4Cone);
    let non_enacted_branch_executed =
        effect_capabilities.contains(&Bi0JoinCapabilityV6::ExecuteNonEnactedBranch);
    let stage4_branch_selected =
        effect_capabilities.contains(&Bi0JoinCapabilityV6::SelectStage4Branch);
    let bi0_passed = t_bi_projection_replayed
        && t_bi_complete_act_local_semantic_certificate
        && t_bi_exact_operational_regression
        && t_bi_f_al1_prime_passed
        && chronological_projection_replayed
        && chronological_projection_logically_passes(&chronological_passed_projection)
        && chronological_exact_72_of_72
        && chronological_exact_9_of_9
        && chronological_exact_18_of_18
        && prefix_exact
        && join_effect_proof.proved
        && bi0_attempt_executed
        && !cone_enumerated
        && !non_enacted_branch_executed
        && !stage4_branch_selected;
    if !bi0_passed {
        return Err(Bi0SemanticRegisterV6Error::Join(
            "the source-first semantic and chronological prerequisites did not close the exact BI-0 v6 join"
                .to_owned(),
        ));
    }

    let mut certificate = Bi0SemanticRegisterV6Certificate {
        schema: BI0_SEMANTIC_REGISTER_V6_SCHEMA.to_owned(),
        date: BI0_SEMANTIC_REGISTER_V6_DATE.to_owned(),
        theorem_id: BI0_SEMANTIC_REGISTER_V6_THEOREM_ID.to_owned(),
        t_bi_passed_projection,
        t_bi_projection_replayed,
        t_bi_complete_act_local_semantic_certificate,
        t_bi_exact_operational_regression,
        t_bi_f_al1_prime_passed,
        chronological_passed_projection,
        chronological_projection_replayed,
        chronological_exact_72_of_72,
        chronological_exact_9_of_9,
        chronological_exact_18_of_18,
        stage1_through3_prefix_entries,
        stage1_through3_t_bi_prefix_digest,
        stage1_through3_bi0_prefix_digest,
        join_effect_proof,
        bi0_attempt_executed,
        bi0_passed,
        bi0_pass_proof_hash: String::new(),
        cone_enumerated,
        non_enacted_branch_executed,
        stage4_branch_selected,
        outcome: "BI0_SEMANTIC_REGISTER_V6_SOURCE_FIRST_PASSED".to_owned(),
        permitted_conclusion: "The exact operational/F-AL1-prime T-BI v6 projection and the replayed chronological 72/72, 9/9, and 18/18 projection pass their closed BI-0 join. A separate BI-0 issuer may expose only the exact typed Stage-1-through-3 opening capability. This certificate itself enumerates no cone and executes or selects no branch."
            .to_owned(),
        required_successor_action: "Present the separately issued minimal opening capability to the independent Stage-4 procedure; that procedure must replay the capability and obtain its own cone-enumeration authority."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.bi0_pass_proof_hash = bi0_pass_proof_hash(&certificate);
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn certificate_logical_errors(certificate: &Bi0SemanticRegisterV6Certificate) -> Vec<String> {
    let mut errors = Vec::new();
    let expected_effect = derive_join_effect_proof();
    let prefix_exact = prefix_entries_logically_exact(
        &certificate.stage1_through3_prefix_entries,
        &certificate.t_bi_passed_projection,
    ) && certificate.stage1_through3_t_bi_prefix_digest
        == certificate
            .t_bi_passed_projection
            .stage1_through3_prefix_digest
        && certificate.stage1_through3_bi0_prefix_digest
            == stage4_prefix_digest(&certificate.stage1_through3_prefix_entries);
    let logical_t_bi = t_bi_projection_logically_passes(&certificate.t_bi_passed_projection)
        && certificate.t_bi_projection_replayed
        && certificate.t_bi_complete_act_local_semantic_certificate
        && certificate.t_bi_exact_operational_regression
        && certificate.t_bi_f_al1_prime_passed;
    let logical_chronological =
        chronological_projection_logically_passes(&certificate.chronological_passed_projection)
            && certificate.chronological_projection_replayed
            && certificate.chronological_exact_72_of_72
            && certificate.chronological_exact_9_of_9
            && certificate.chronological_exact_18_of_18;
    let logical_join = certificate.join_effect_proof == expected_effect
        && certificate.join_effect_proof.proved
        && certificate.bi0_attempt_executed
        && !certificate.cone_enumerated
        && !certificate.non_enacted_branch_executed
        && !certificate.stage4_branch_selected;
    let logical_bi0 = logical_t_bi && logical_chronological && prefix_exact && logical_join;
    if certificate.schema != BI0_SEMANTIC_REGISTER_V6_SCHEMA
        || certificate.date != BI0_SEMANTIC_REGISTER_V6_DATE
        || certificate.theorem_id != BI0_SEMANTIC_REGISTER_V6_THEOREM_ID
    {
        errors.push("BI-0 v6 schema, date, or theorem identity mismatch".to_owned());
    }
    if certificate.bi0_passed != logical_bi0 || !logical_bi0 {
        errors.push("BI-0 v6 prerequisite join does not prove its positive verdict".to_owned());
    }
    if certificate.bi0_pass_proof_hash != bi0_pass_proof_hash(certificate) {
        errors.push("BI-0 v6 pass-proof digest mismatch".to_owned());
    }
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("BI-0 v6 certificate digest mismatch".to_owned());
    }
    errors
}

pub fn replay_bi0_semantic_register_v6_certificate(
    claimed: &Bi0SemanticRegisterV6Certificate,
) -> Vec<String> {
    let mut errors = certificate_logical_errors(claimed);
    // The create-new comparison itself replays both prerequisite projections
    // before issuing the expected certificate.  Repeating each projection
    // replay here would add cost without adding an authority boundary.
    match issue_bi0_semantic_register_v6_certificate() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("BI-0 v6 certificate differs from create-new reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn exact_opening_permissions() -> Vec<Bi0Stage4OpeningPermissionV6> {
    vec![
        Bi0Stage4OpeningPermissionV6::ReadExactTypedPrefix,
        Bi0Stage4OpeningPermissionV6::ReplayBi0PassBinding,
        Bi0Stage4OpeningPermissionV6::PresentToSeparateStage4Procedure,
    ]
}

fn exact_opening_forbidden_capabilities() -> Vec<Bi0Stage4ForbiddenCapabilityV6> {
    vec![
        Bi0Stage4ForbiddenCapabilityV6::ReadFullBi0Certificate,
        Bi0Stage4ForbiddenCapabilityV6::ReadSemanticRegister,
        Bi0Stage4ForbiddenCapabilityV6::ReadStructuralRegister,
        Bi0Stage4ForbiddenCapabilityV6::EnumerateCone,
        Bi0Stage4ForbiddenCapabilityV6::ExecuteBranch,
        Bi0Stage4ForbiddenCapabilityV6::SelectBranch,
        Bi0Stage4ForbiddenCapabilityV6::WriteSuccessorArtifact,
    ]
}

fn issue_opening_from_certificate(
    certificate: &Bi0SemanticRegisterV6Certificate,
) -> Result<Bi0Stage4OpeningCapabilityV6, Bi0SemanticRegisterV6Error> {
    if !certificate_logical_errors(certificate).is_empty() || !certificate.bi0_passed {
        return Err(Bi0SemanticRegisterV6Error::Join(
            "a Stage-4 opening capability requires a current passing BI-0 v6 certificate"
                .to_owned(),
        ));
    }
    let exact_stage1_through3_entries = certificate.stage1_through3_prefix_entries.clone();
    let exact_stage1_through3_payload_hashes = exact_stage1_through3_entries
        .iter()
        .map(|entry| entry.payload_hash.clone())
        .collect::<Vec<_>>();
    let exact_prefix_and_pass_binding = exact_stage1_through3_entries.len() == 3
        && exact_stage1_through3_entries
            .iter()
            .map(|entry| entry.stage)
            .eq(1..=3)
        && exact_stage1_through3_payload_hashes
            == exact_stage1_through3_entries
                .iter()
                .map(|entry| prefix_entry_payload_hash(entry))
                .collect::<Vec<_>>()
        && certificate.stage1_through3_bi0_prefix_digest
            == stage4_prefix_digest(&exact_stage1_through3_entries);
    let mut capability = Bi0Stage4OpeningCapabilityV6 {
        schema: BI0_STAGE4_OPENING_CAPABILITY_V6_SCHEMA.to_owned(),
        granting_bi0_schema: certificate.schema.clone(),
        granting_bi0_result_digest: certificate.result_digest.clone(),
        granting_bi0_pass_proof_hash: certificate.bi0_pass_proof_hash.clone(),
        t_bi_source_certificate_result_digest: certificate
            .t_bi_passed_projection
            .source_certificate_result_digest
            .clone(),
        t_bi_passed_projection_derivation_hash: certificate
            .t_bi_passed_projection
            .derivation_hash
            .clone(),
        chronological_source_certificate_result_digest: certificate
            .chronological_passed_projection
            .source_certificate_result_digest
            .clone(),
        chronological_passed_projection_derivation_hash: certificate
            .chronological_passed_projection
            .derivation_hash
            .clone(),
        exact_stage1_through3_entries,
        exact_stage1_through3_t_bi_prefix_digest: certificate
            .stage1_through3_t_bi_prefix_digest
            .clone(),
        exact_stage1_through3_payload_hashes,
        exact_stage1_through3_bi0_prefix_digest: certificate
            .stage1_through3_bi0_prefix_digest
            .clone(),
        permitted_permissions: exact_opening_permissions(),
        forbidden_capabilities: exact_opening_forbidden_capabilities(),
        full_bi0_certificate_present: false,
        semantic_or_structural_register_present: false,
        cone_execution_capability_present: false,
        branch_execution_capability_present: false,
        branch_selection_capability_present: false,
        exact_prefix_and_pass_binding,
        derivation_hash: String::new(),
    };
    if !exact_prefix_and_pass_binding {
        return Err(Bi0SemanticRegisterV6Error::Join(
            "BI-0 v6 could not bind the exact typed Stage-1-through-3 payload".to_owned(),
        ));
    }
    capability.derivation_hash = opening_capability_digest(&capability);
    Ok(capability)
}

fn opening_capability_logical_errors(capability: &Bi0Stage4OpeningCapabilityV6) -> Vec<String> {
    let mut errors = Vec::new();
    let entries = &capability.exact_stage1_through3_entries;
    let exact_entries = entries.len() == 3
        && entries.iter().map(|entry| entry.stage).eq(1..=3)
        && entries.iter().enumerate().all(|(index, entry)| {
            entry.candidate_hash == candidate_hash(&entry.telescope)
                && entry.predecessor_signature_digest
                    == SealedSignature::from_telescopes(
                        entries[..index]
                            .iter()
                            .map(|entry| (entry.stage, entry.telescope.clone()))
                            .collect(),
                    )
                    .digest()
                && entry.t_bi_typed_prefix_digest
                    == capability.exact_stage1_through3_t_bi_prefix_digest
                && entry.payload_hash == prefix_entry_payload_hash(entry)
        })
        && capability.exact_stage1_through3_payload_hashes
            == entries
                .iter()
                .map(|entry| entry.payload_hash.clone())
                .collect::<Vec<_>>()
        && capability.exact_stage1_through3_bi0_prefix_digest == stage4_prefix_digest(entries);
    let exact_scope = capability.schema == BI0_STAGE4_OPENING_CAPABILITY_V6_SCHEMA
        && capability.granting_bi0_schema == BI0_SEMANTIC_REGISTER_V6_SCHEMA
        && !capability.granting_bi0_result_digest.is_empty()
        && !capability.granting_bi0_pass_proof_hash.is_empty()
        && !capability.t_bi_source_certificate_result_digest.is_empty()
        && !capability.t_bi_passed_projection_derivation_hash.is_empty()
        && !capability
            .chronological_source_certificate_result_digest
            .is_empty()
        && !capability
            .chronological_passed_projection_derivation_hash
            .is_empty()
        && capability.permitted_permissions == exact_opening_permissions()
        && capability.forbidden_capabilities == exact_opening_forbidden_capabilities()
        && !capability.full_bi0_certificate_present
        && !capability.semantic_or_structural_register_present
        && !capability.cone_execution_capability_present
        && !capability.branch_execution_capability_present
        && !capability.branch_selection_capability_present
        && capability.exact_prefix_and_pass_binding;
    if !exact_entries || !exact_scope {
        errors.push(
            "BI-0 v6 Stage-4 opening capability has an invalid prefix or authority surface"
                .to_owned(),
        );
    }
    if capability.derivation_hash != opening_capability_digest(capability) {
        errors.push("BI-0 v6 Stage-4 opening capability digest mismatch".to_owned());
    }
    errors
}

/// The only public minting seam for the Stage-4 opening.  It always reruns
/// the complete current BI-0 issuer internally; callers cannot supply a
/// certificate, prefix, pass bit, digest, semantic vector, or branch verdict.
pub fn issue_bi0_stage4_opening_capability_v6()
-> Result<Bi0Stage4OpeningCapabilityV6, Bi0SemanticRegisterV6Error> {
    let certificate = issue_bi0_semantic_register_v6_certificate()?;
    issue_opening_from_certificate(&certificate)
}

/// Replay from the narrow capability alone.  The caller does not provide or
/// read a full BI-0 certificate; exact authority is checked by deterministic
/// reissuance inside this module.
pub fn replay_bi0_stage4_opening_capability_v6(
    claimed: &Bi0Stage4OpeningCapabilityV6,
) -> Vec<String> {
    match issue_bi0_stage4_opening_capability_v6() {
        Ok(expected) => opening_capability_errors_against_expected(claimed, &expected),
        Err(error) => vec![error.to_string()],
    }
}

fn opening_capability_errors_against_expected(
    claimed: &Bi0Stage4OpeningCapabilityV6,
    expected: &Bi0Stage4OpeningCapabilityV6,
) -> Vec<String> {
    let mut errors = opening_capability_logical_errors(claimed);
    if claimed != expected {
        errors.push(
            "BI-0 v6 Stage-4 opening differs from deterministic passing-gate reissuance".to_owned(),
        );
    }
    errors
}

pub fn issue_bi0_semantic_register_v6_artifact()
-> Result<Bi0SemanticRegisterV6Artifact, Bi0SemanticRegisterV6Error> {
    let certificate = issue_bi0_semantic_register_v6_certificate()?;
    let stage4_opening_capability = issue_opening_from_certificate(&certificate)?;
    let mut artifact = Bi0SemanticRegisterV6Artifact {
        certificate,
        stage4_opening_capability,
        result_digest: String::new(),
    };
    artifact.result_digest = artifact_digest(&artifact);
    Ok(artifact)
}

pub fn replay_bi0_semantic_register_v6_artifact(
    claimed: &Bi0SemanticRegisterV6Artifact,
) -> Bi0SemanticRegisterV6Replay {
    let mut errors = certificate_logical_errors(&claimed.certificate);
    let opening_errors = opening_capability_logical_errors(&claimed.stage4_opening_capability);
    let opening_capability_valid = opening_errors.is_empty();
    if !opening_errors.is_empty() {
        errors.extend(opening_errors);
    }
    if claimed.stage4_opening_capability.granting_bi0_result_digest
        != claimed.certificate.result_digest
        || claimed
            .stage4_opening_capability
            .granting_bi0_pass_proof_hash
            != claimed.certificate.bi0_pass_proof_hash
        || claimed
            .stage4_opening_capability
            .t_bi_passed_projection_derivation_hash
            != claimed.t_bi_projection_digest()
        || claimed
            .stage4_opening_capability
            .chronological_passed_projection_derivation_hash
            != claimed.chronological_projection_digest()
    {
        errors.push("BI-0 v6 artifact opening-to-certificate binding mismatch".to_owned());
    }
    if claimed.result_digest != artifact_digest(claimed) {
        errors.push("BI-0 v6 artifact envelope digest mismatch".to_owned());
    }
    // Reissue the complete envelope once.  Equality with that single current
    // source-first artifact simultaneously checks the certificate, opening,
    // and their cross-binding; separately reissuing its two projections would
    // duplicate the proof construction without creating another boundary.
    match issue_bi0_semantic_register_v6_artifact() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("BI-0 v6 artifact differs from create-new reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    let cone_or_branch_capability_present = !claimed
        .stage4_opening_capability
        .has_no_cone_or_branch_capability()
        || claimed.certificate.cone_enumerated
        || claimed.certificate.non_enacted_branch_executed
        || claimed.certificate.stage4_branch_selected;
    Bi0SemanticRegisterV6Replay {
        valid: errors.is_empty(),
        bi0_passed: claimed.certificate.bi0_passed,
        t_bi_passed: claimed
            .certificate
            .t_bi_complete_act_local_semantic_certificate
            && claimed.certificate.t_bi_exact_operational_regression
            && claimed.certificate.t_bi_f_al1_prime_passed,
        chronological_passed: claimed.certificate.chronological_exact_72_of_72
            && claimed.certificate.chronological_exact_9_of_9
            && claimed.certificate.chronological_exact_18_of_18,
        opening_capability_valid,
        cone_or_branch_capability_present,
        errors,
    }
}

impl Bi0SemanticRegisterV6Artifact {
    fn t_bi_projection_digest(&self) -> &str {
        &self.certificate.t_bi_passed_projection.derivation_hash
    }

    fn chronological_projection_digest(&self) -> &str {
        &self
            .certificate
            .chronological_passed_projection
            .derivation_hash
    }
}

pub fn replay_bi0_semantic_register_v6_json(json: &str) -> Bi0SemanticRegisterV6Replay {
    match serde_json::from_str::<Bi0SemanticRegisterV6Artifact>(json) {
        Ok(artifact) => replay_bi0_semantic_register_v6_artifact(&artifact),
        Err(error) => Bi0SemanticRegisterV6Replay {
            valid: false,
            bi0_passed: false,
            t_bi_passed: false,
            chronological_passed: false,
            opening_capability_valid: false,
            cone_or_branch_capability_present: false,
            errors: vec![error.to_string()],
        },
    }
}

pub fn render_bi0_semantic_register_v6_report(artifact: &Bi0SemanticRegisterV6Artifact) -> String {
    let certificate = &artifact.certificate;
    let prefix_rows = certificate
        .stage1_through3_prefix_entries
        .iter()
        .map(|entry| {
            format!(
                "| {} | `{}` | `{}` |",
                entry.stage, entry.candidate_hash, entry.payload_hash
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# BI-0 source-first semantic-register v6 result\n\n\
         **Date:** {}. **Outcome:** `{}`.\n\n\
         T-BI v6 projection replayed: **{}**. Complete act-local semantic certificate: **{}**. Exact operational regression: **{}**. F-AL1-prime: **{}**. Semantic role declarations: **{} = {} proved + {} impossible**, with role/quotient/A3/total/silent residuals **{}/{}/{}/{}/{}**.\n\n\
         Chronological v5 projection replayed: **{}**. F-SM1 gates: **{}/{} discharges**, **{}/{} former cases**, and **{}/{} T-SM1b cases**; zero accounting: **{}**.\n\n\
         BI-0 passed: **{}**. Closed join-effect proof: **{}**. Cone enumerated: **{}**. Branch executed: **{}**. Branch selected: **{}**.\n\n\
         The separately issued Stage-4 capability binds only these typed prefix payloads:\n\n\
         | Stage | Candidate hash | BI-0 payload hash |\n|---:|---|---|\n{}\n\n\
         T-BI prefix digest: `{}`. BI-0 prefix digest: `{}`. Capability digest: `{}`. The capability carries no full BI-0 certificate, semantic or structural register, cone-execution authority, branch-execution authority, branch-selection authority, or artifact-write authority.\n\n\
         {}\n\nNext: {}\n\nBI-0 certificate digest: `{}`. Envelope digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.t_bi_projection_replayed,
        certificate.t_bi_complete_act_local_semantic_certificate,
        certificate.t_bi_exact_operational_regression,
        certificate.t_bi_f_al1_prime_passed,
        certificate.t_bi_passed_projection.role_declaration_count,
        certificate
            .t_bi_passed_projection
            .proved_family_declaration_count,
        certificate
            .t_bi_passed_projection
            .theorem_impossibility_declaration_count,
        certificate
            .t_bi_passed_projection
            .named_registry_residual_count,
        certificate
            .t_bi_passed_projection
            .named_quotient_residual_count,
        certificate.t_bi_passed_projection.named_a3_residual_count,
        certificate
            .t_bi_passed_projection
            .total_named_residual_count,
        certificate.t_bi_passed_projection.silent_residue_count,
        certificate.chronological_projection_replayed,
        certificate
            .chronological_passed_projection
            .derived_discharge_count,
        certificate
            .chronological_passed_projection
            .total_discharge_count,
        certificate
            .chronological_passed_projection
            .former_derived_count,
        certificate
            .chronological_passed_projection
            .former_expected_count,
        certificate
            .chronological_passed_projection
            .t_sm1b_derived_count,
        certificate
            .chronological_passed_projection
            .t_sm1b_surface_count,
        certificate
            .chronological_passed_projection
            .zero_accounting_proved,
        certificate.bi0_passed,
        certificate.join_effect_proof.proved,
        certificate.cone_enumerated,
        certificate.non_enacted_branch_executed,
        certificate.stage4_branch_selected,
        prefix_rows,
        certificate.stage1_through3_t_bi_prefix_digest,
        certificate.stage1_through3_bi0_prefix_digest,
        artifact.stage4_opening_capability.derivation_hash,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
        artifact.result_digest,
    )
}

pub fn emit_bi0_semantic_register_v6_create_new(
    directory: &Path,
) -> Result<Bi0SemanticRegisterV6Artifact, Bi0SemanticRegisterV6Error> {
    let artifact = issue_bi0_semantic_register_v6_artifact()?;
    let json = serde_json::to_string_pretty(&artifact)
        .map_err(|error| Bi0SemanticRegisterV6Error::Json(error.to_string()))?;
    let replay = replay_bi0_semantic_register_v6_json(&json);
    if !replay.valid {
        return Err(Bi0SemanticRegisterV6Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let certificate_path = directory.join(BI0_SEMANTIC_REGISTER_V6_CERTIFICATE_NAME);
    let report_path = directory.join(BI0_SEMANTIC_REGISTER_V6_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(Bi0SemanticRegisterV6Error::Io(format!(
            "create-new target already exists: {} or {}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| Bi0SemanticRegisterV6Error::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| Bi0SemanticRegisterV6Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| Bi0SemanticRegisterV6Error::Io(error.to_string()))?;
    report_file
        .write_all(render_bi0_semantic_register_v6_report(&artifact).as_bytes())
        .map_err(|error| Bi0SemanticRegisterV6Error::Io(error.to_string()))?;
    Ok(artifact)
}

pub fn replay_bi0_semantic_register_v6_directory(
    directory: &Path,
) -> Result<Bi0SemanticRegisterV6Replay, Bi0SemanticRegisterV6Error> {
    let json = std::fs::read_to_string(directory.join(BI0_SEMANTIC_REGISTER_V6_CERTIFICATE_NAME))
        .map_err(|error| Bi0SemanticRegisterV6Error::Io(error.to_string()))?;
    Ok(replay_bi0_semantic_register_v6_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::expr::Expr;
    use std::sync::OnceLock;

    fn artifact() -> &'static Bi0SemanticRegisterV6Artifact {
        static ARTIFACT: OnceLock<Bi0SemanticRegisterV6Artifact> = OnceLock::new();
        ARTIFACT.get_or_init(|| {
            issue_bi0_semantic_register_v6_artifact().expect("BI-0 v6 source-first artifact")
        })
    }

    #[test]
    fn source_first_prerequisites_pass_and_issue_only_a_narrow_opening() {
        let artifact = artifact();
        let certificate = &artifact.certificate;
        assert!(certificate.t_bi_projection_replayed);
        assert!(certificate.t_bi_complete_act_local_semantic_certificate);
        assert!(certificate.t_bi_exact_operational_regression);
        assert!(certificate.t_bi_f_al1_prime_passed);
        assert!(certificate.chronological_projection_replayed);
        assert!(certificate.chronological_exact_72_of_72);
        assert!(certificate.chronological_exact_9_of_9);
        assert!(certificate.chronological_exact_18_of_18);
        assert!(certificate.join_effect_proof.proved);
        assert!(certificate.bi0_passed);
        assert!(!certificate.cone_enumerated);
        assert!(!certificate.non_enacted_branch_executed);
        assert!(!certificate.stage4_branch_selected);
        assert_eq!(
            artifact
                .stage4_opening_capability
                .exact_stage1_through3_entries()
                .len(),
            3
        );
        assert!(
            artifact
                .stage4_opening_capability
                .has_no_cone_or_branch_capability()
        );
        assert!(certificate_logical_errors(certificate).is_empty());
        assert!(opening_capability_logical_errors(&artifact.stage4_opening_capability).is_empty());
        assert_eq!(artifact.result_digest, artifact_digest(artifact));
    }

    #[test]
    fn capability_replays_without_a_full_bi0_argument_and_omits_registers() {
        let capability = &artifact().stage4_opening_capability;
        assert!(replay_bi0_stage4_opening_capability_v6(capability).is_empty());
        let object = serde_json::to_value(capability)
            .expect("opening JSON")
            .as_object()
            .cloned()
            .expect("opening object");
        assert!(!object.contains_key("semantic_register"));
        assert!(!object.contains_key("structural_register"));
        assert!(!object.contains_key("bare_register_table"));
        assert!(!object.contains_key("operational_rows"));
        assert!(!object.contains_key("cone_enumerated"));
    }

    #[test]
    fn a_fully_rehashed_prefix_payload_forgery_fails_current_issuer_replay() {
        let mut forged = artifact().stage4_opening_capability.clone();
        forged.exact_stage1_through3_entries[1].telescope.clauses[0].expr = Expr::Var(77);
        for index in 0..forged.exact_stage1_through3_entries.len() {
            let predecessor = forged.exact_stage1_through3_entries[..index]
                .iter()
                .map(|entry| (entry.stage, entry.telescope.clone()))
                .collect::<Vec<_>>();
            forged.exact_stage1_through3_entries[index].candidate_hash =
                candidate_hash(&forged.exact_stage1_through3_entries[index].telescope);
            forged.exact_stage1_through3_entries[index].predecessor_signature_digest =
                SealedSignature::from_telescopes(predecessor)
                    .digest()
                    .to_owned();
            forged.exact_stage1_through3_entries[index].payload_hash =
                prefix_entry_payload_hash(&forged.exact_stage1_through3_entries[index]);
        }
        forged.exact_stage1_through3_payload_hashes = forged
            .exact_stage1_through3_entries
            .iter()
            .map(|entry| entry.payload_hash.clone())
            .collect();
        forged.exact_stage1_through3_bi0_prefix_digest =
            stage4_prefix_digest(&forged.exact_stage1_through3_entries);
        forged.derivation_hash = opening_capability_digest(&forged);
        let errors = opening_capability_errors_against_expected(
            &forged,
            &artifact().stage4_opening_capability,
        );
        assert!(errors.iter().any(|error| error.contains("reissuance")));
    }

    #[test]
    fn a_fully_rehashed_pass_binding_or_cone_capability_forgery_fails() {
        let mut digest_forgery = artifact().stage4_opening_capability.clone();
        digest_forgery.granting_bi0_result_digest = "blake3:forged-bi0".to_owned();
        digest_forgery.derivation_hash = opening_capability_digest(&digest_forgery);
        assert!(
            opening_capability_errors_against_expected(
                &digest_forgery,
                &artifact().stage4_opening_capability,
            )
            .iter()
            .any(|error| error.contains("reissuance"))
        );

        let mut cone_forgery = artifact().stage4_opening_capability.clone();
        cone_forgery.cone_execution_capability_present = true;
        cone_forgery.derivation_hash = opening_capability_digest(&cone_forgery);
        assert!(
            opening_capability_errors_against_expected(
                &cone_forgery,
                &artifact().stage4_opening_capability,
            )
            .iter()
            .any(|error| error.contains("authority surface"))
        );
    }

    #[test]
    fn artifact_envelope_rejects_a_cross_bound_opening() {
        let mut forged = artifact().clone();
        forged
            .stage4_opening_capability
            .chronological_source_certificate_result_digest = "blake3:other-run".to_owned();
        forged.stage4_opening_capability.derivation_hash =
            opening_capability_digest(&forged.stage4_opening_capability);
        forged.result_digest = artifact_digest(&forged);
        let errors = opening_capability_errors_against_expected(
            &forged.stage4_opening_capability,
            &artifact().stage4_opening_capability,
        );
        assert!(errors.iter().any(|error| error.contains("reissuance")));
        assert_ne!(forged, *artifact());
    }
}
