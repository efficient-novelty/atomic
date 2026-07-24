//! Prefix-parametric source-first semantic provenance for BI-1b.
//!
//! This successor leaves `branch_semantic_provenance_v3` byte-for-byte
//! untouched.  It uses the same v3 declaration grammar and v5 B1/B2
//! classifier, but replaces the one historical-B7 premise at Stage 8 with a
//! theorem that re-elaborates the supplied seven-act prefix.  No enacted
//! classification, candidate byte-correspondence, score, bar, verdict, or
//! structural novelty scalar is an input.

use crate::act_local_provenance_v3::{
    issue_act_local_prefix_declaration_sequence_v3, issue_act_local_v3_typed_r2_rule_token,
};
use crate::act_local_semantic_provenance_v4::{
    V4_PREFIX_GENERAL_SOURCE_SURFACE_SCHEMA, V4_PREFIX_GENERAL_SOURCE_SURFACE_THEOREM_ID,
    issue_v4_prefix_general_source_surface, issue_v4_prefix_local_typed_r1_rule_token,
    replay_v4_prefix_general_source_surface,
};
use crate::act_local_semantic_provenance_v5::{
    T_BI_B1_B2_PREFIX_GENERAL_THEOREM_ID, V5_PREFIX_GENERAL_SEMANTIC_SEQUENCE_SCHEMA,
    V5PrefixLocalAuthoritativeRoleRow, V5PrefixLocalResolutionClass,
    V5PrefixLocalSemanticPackageProof, issue_prefix_general_semantic_sequence_v6,
    issue_v5_prefix_local_rule_authority, replay_prefix_local_issuance_receipt,
    replay_v5_prefix_local_rule_authority,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::e4_generator_basis::E4_PREFIX_GENERAL_M1_VERSION;
use pen_schema::step8_r2::{
    STEP8_R2_PREFIX_GENERAL_TOKEN_VERSION, STEP8_R2_PREFIX_GENERAL_TYPED_BOUNDARY_API,
};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const BRANCH_PREFIX_GENERAL_PROVENANCE_V4_SCHEMA: &str =
    "branch-prefix-general-semantic-provenance-v4";
pub const BRANCH_PREFIX_GENERAL_PROVENANCE_V4_DATE: &str = "2026-07-23";
pub const BRANCH_PREFIX_GENERAL_PROVENANCE_V4_THEOREM_ID: &str =
    "T-B1b-prefix-general-candidate-source-first-semantic-provenance-v1";
pub const BRANCH_PREFIX_GENERAL_PROVENANCE_V4_PROCEDURE_ID: &str =
    "BI-1b-one-parametric-v3-v4-v5-B1-B2-prefix-general-extraction-v1";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BRANCH_PREFIX_GENERAL_PROVENANCE_V4_SCHEMA, domain, value))
        .expect("BI-1b prefix-general provenance serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchPrefixGeneralProvenanceV4Authority {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub procedure_id: String,
    pub v4_source_schema: String,
    pub v4_source_theorem_id: String,
    pub v5_sequence_schema: String,
    pub v5_sequence_theorem_id: String,
    pub v5_rule_authority_derivation_hash: String,
    pub step8_r2_version: String,
    pub step8_typed_boundary_api: String,
    pub step8_m1_version: String,
    pub one_procedure_for_every_prefix: bool,
    pub candidate_and_exact_prefix_are_only_semantic_inputs: bool,
    pub archive_input_read: bool,
    pub historical_registry_input_read: bool,
    pub structural_nu_input_read: bool,
    pub bar_input_read: bool,
    pub verdict_input_read: bool,
    pub enacted_future_input_read: bool,
    pub enacted_classification_input_read: bool,
    pub byte_identity_input_read: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchPrefixGeneralFamilyCreditV4 {
    pub family_id: String,
    pub authoritative_family_row_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchPrefixGeneralRoleClassificationV4 {
    pub declaration_id: String,
    pub role_kind: String,
    pub resolution_class: V5PrefixLocalResolutionClass,
    pub resolved_family_id: Option<String>,
    pub target_family_ids: Vec<String>,
    pub classification_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchPrefixGeneralProvenanceV4Token {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_entry_count: usize,
    pub predecessor_stages: Vec<u32>,
    pub predecessor_candidate_hashes: Vec<String>,
    pub predecessor_signature_digest: String,
    pub extraction_procedure_digest: String,
    pub rule_authority_derivation_hash: String,
    pub sequence_schema: String,
    pub sequence_theorem_id: String,
    pub sequence_derivation_hash: String,
    pub sequence_authoritative_seal: String,
    pub sequence_issuance_trace_root: String,
    pub package_derivation_hash: String,
    pub package_v3_declaration_hash: String,
    pub package_v4_candidate_local_source_hash: String,
    pub semantic_nu: u32,
    pub credited_family_ids: Vec<String>,
    pub credited_family_rows: Vec<BranchPrefixGeneralFamilyCreditV4>,
    pub proved_family_rows: Vec<BranchPrefixGeneralRoleClassificationV4>,
    pub theorem_impossibility_rows: Vec<BranchPrefixGeneralRoleClassificationV4>,
    pub classification_evidence_hashes: Vec<String>,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_role_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub silent_residue_count: usize,
    pub r2_generated_instance_removed_count: usize,
    pub r2_removed_occurrence_hashes: Vec<String>,
    pub r2_step8_typed_signature_derivation_hash: Option<String>,
    pub r2_m1_generated_membership_derivation_hash: Option<String>,
    pub r2_local_premises_replayed: bool,
    pub exact_prefix_candidate_package_binding: bool,
    pub v4_source_replayed_and_matches_package: bool,
    pub issuance_trace_replayed: bool,
    pub independent_reissuance_equal: bool,
    pub all_residual_counts_zero: bool,
    pub every_role_declaration_resolved: bool,
    pub every_marginal_family_credited_or_theorem_impossible: bool,
    pub local_anchor_nonreuse_holds: bool,
    pub no_archive_or_historical_registry_input: bool,
    pub structural_nu_input_read: bool,
    pub bar_input_read: bool,
    pub verdict_input_read: bool,
    pub enacted_future_input_read: bool,
    pub enacted_classification_input_read: bool,
    pub byte_identity_input_read: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BranchPrefixGeneralProvenanceV4Error {
    #[error("BI-1b prefix-general provenance invalid input: {0}")]
    Input(String),
    #[error("BI-1b prefix-general semantic construction failed: {0}")]
    Construction(String),
    #[error("BI-1b prefix-general projection failed: {0}")]
    Projection(String),
}

fn authority_hash(authority: &BranchPrefixGeneralProvenanceV4Authority) -> String {
    let mut projection = authority.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-general-extraction-authority", &projection)
}

fn token_hash(token: &BranchPrefixGeneralProvenanceV4Token) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-general-provenance-token", &projection)
}

fn classification_hash(row: &BranchPrefixGeneralRoleClassificationV4) -> String {
    let mut projection = row.clone();
    projection.classification_hash.clear();
    tagged_hash("semantic-role-classification", &projection)
}

fn project_one_classification(
    row: &V5PrefixLocalAuthoritativeRoleRow,
) -> BranchPrefixGeneralRoleClassificationV4 {
    let mut projected = BranchPrefixGeneralRoleClassificationV4 {
        declaration_id: row.declaration_id.clone(),
        role_kind: row.occurrence.kind.clone(),
        resolution_class: row.resolution_class.clone(),
        resolved_family_id: row.resolved_family_id.clone(),
        target_family_ids: row
            .target_relations
            .iter()
            .map(|pair| pair.family_id.clone())
            .collect(),
        classification_hash: String::new(),
    };
    projected.classification_hash = classification_hash(&projected);
    projected
}

/// Count-blind semantic projection used by the BI-1b-0 regression rung.
/// Proof hashes are deliberately excluded: the historical and prefix-general
/// derivations have different source-bound evidence while proving the same
/// family/impossibility classification.
pub fn project_branch_prefix_general_role_classifications_v4(
    package: &V5PrefixLocalSemanticPackageProof,
) -> Result<
    (
        Vec<BranchPrefixGeneralRoleClassificationV4>,
        Vec<BranchPrefixGeneralRoleClassificationV4>,
    ),
    BranchPrefixGeneralProvenanceV4Error,
> {
    if package
        .role_rows
        .iter()
        .any(|row| row.resolution_class == V5PrefixLocalResolutionClass::NamedResidual)
    {
        return Err(BranchPrefixGeneralProvenanceV4Error::Projection(
            "role classification surface contains a named residual".to_owned(),
        ));
    }
    let proved = package
        .role_rows
        .iter()
        .filter(|row| row.resolution_class == V5PrefixLocalResolutionClass::ProvedFamily)
        .map(project_one_classification)
        .collect::<Vec<_>>();
    let impossible = package
        .role_rows
        .iter()
        .filter(|row| {
            row.resolution_class == V5PrefixLocalResolutionClass::TheoremBackedImpossibility
        })
        .map(project_one_classification)
        .collect::<Vec<_>>();
    if proved.len() + impossible.len() != package.role_declaration_count {
        return Err(BranchPrefixGeneralProvenanceV4Error::Projection(
            "proved-family and theorem-impossibility rows do not partition declarations".to_owned(),
        ));
    }
    Ok((proved, impossible))
}

/// Zero-input authority for F-B1b-2.  A sweep can seal this digest before any
/// candidate is classified, including on a window whose every candidate
/// returns a named construction gap.
pub fn issue_branch_prefix_general_provenance_v4_authority()
-> BranchPrefixGeneralProvenanceV4Authority {
    let v5 = issue_v5_prefix_local_rule_authority();
    let no_forbidden_input =
        !v5.source_document_read && !v5.structural_scalar_read && !v5.desired_vector_read;
    let mut authority = BranchPrefixGeneralProvenanceV4Authority {
        schema: BRANCH_PREFIX_GENERAL_PROVENANCE_V4_SCHEMA.to_owned(),
        date: BRANCH_PREFIX_GENERAL_PROVENANCE_V4_DATE.to_owned(),
        theorem_id: BRANCH_PREFIX_GENERAL_PROVENANCE_V4_THEOREM_ID.to_owned(),
        procedure_id: BRANCH_PREFIX_GENERAL_PROVENANCE_V4_PROCEDURE_ID.to_owned(),
        v4_source_schema: V4_PREFIX_GENERAL_SOURCE_SURFACE_SCHEMA.to_owned(),
        v4_source_theorem_id: V4_PREFIX_GENERAL_SOURCE_SURFACE_THEOREM_ID.to_owned(),
        v5_sequence_schema: V5_PREFIX_GENERAL_SEMANTIC_SEQUENCE_SCHEMA.to_owned(),
        v5_sequence_theorem_id: T_BI_B1_B2_PREFIX_GENERAL_THEOREM_ID.to_owned(),
        v5_rule_authority_derivation_hash: v5.derivation_hash,
        step8_r2_version: STEP8_R2_PREFIX_GENERAL_TOKEN_VERSION.to_owned(),
        step8_typed_boundary_api: STEP8_R2_PREFIX_GENERAL_TYPED_BOUNDARY_API.to_owned(),
        step8_m1_version: E4_PREFIX_GENERAL_M1_VERSION.to_owned(),
        one_procedure_for_every_prefix: true,
        candidate_and_exact_prefix_are_only_semantic_inputs: true,
        archive_input_read: false,
        historical_registry_input_read: false,
        structural_nu_input_read: false,
        bar_input_read: false,
        verdict_input_read: false,
        enacted_future_input_read: false,
        enacted_classification_input_read: false,
        byte_identity_input_read: false,
        proved: no_forbidden_input,
        derivation_hash: String::new(),
    };
    authority.derivation_hash = authority_hash(&authority);
    authority
}

pub fn replay_branch_prefix_general_provenance_v4_authority(
    claimed: &BranchPrefixGeneralProvenanceV4Authority,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != authority_hash(claimed) {
        errors.push("BI-1b extraction authority digest mismatch".to_owned());
    }
    let expected = issue_branch_prefix_general_provenance_v4_authority();
    if expected != *claimed {
        errors.push("BI-1b extraction authority differs from deterministic reissuance".to_owned());
    }
    errors
}

fn validate_input(
    predecessor_entries: &[(u32, Telescope)],
    stage: u32,
) -> Result<(), BranchPrefixGeneralProvenanceV4Error> {
    if stage == 0 || stage as usize != predecessor_entries.len() + 1 {
        return Err(BranchPrefixGeneralProvenanceV4Error::Input(format!(
            "candidate Stage {stage} requires exactly {} predecessor entries",
            stage.saturating_sub(1)
        )));
    }
    if !predecessor_entries
        .iter()
        .map(|(entry_stage, _)| *entry_stage)
        .eq(1..stage)
    {
        return Err(BranchPrefixGeneralProvenanceV4Error::Input(
            "predecessor entries must be the exact contiguous Stage-1-through-(N-1) prefix"
                .to_owned(),
        ));
    }
    Ok(())
}

pub fn issue_branch_prefix_general_provenance_v4(
    predecessor_entries: &[(u32, Telescope)],
    stage: u32,
    candidate: &Telescope,
) -> Result<BranchPrefixGeneralProvenanceV4Token, BranchPrefixGeneralProvenanceV4Error> {
    validate_input(predecessor_entries, stage)?;
    let authority = issue_branch_prefix_general_provenance_v4_authority();
    if !authority.proved
        || !replay_branch_prefix_general_provenance_v4_authority(&authority).is_empty()
    {
        return Err(BranchPrefixGeneralProvenanceV4Error::Construction(
            "the one prefix-general extraction authority did not replay".to_owned(),
        ));
    }

    let predecessor_signature = SealedSignature::from_telescopes(predecessor_entries.to_vec());
    let predecessor_signature_digest = predecessor_signature.digest().to_owned();
    let candidate_digest = candidate_hash(candidate);
    let mut full_entries = predecessor_entries.to_vec();
    full_entries.push((stage, candidate.clone()));

    let sequence = issue_prefix_general_semantic_sequence_v6(&full_entries)
        .map_err(|error| BranchPrefixGeneralProvenanceV4Error::Construction(error.to_string()))?;
    let independent = issue_prefix_general_semantic_sequence_v6(&full_entries)
        .map_err(|error| BranchPrefixGeneralProvenanceV4Error::Construction(error.to_string()))?;
    let independent_reissuance_equal = independent == sequence;
    if !independent_reissuance_equal {
        return Err(BranchPrefixGeneralProvenanceV4Error::Construction(
            "two independent prefix-general semantic issuances differ".to_owned(),
        ));
    }
    let package = sequence.packages.last().ok_or_else(|| {
        BranchPrefixGeneralProvenanceV4Error::Projection(
            "prefix-general sequence contained no candidate package".to_owned(),
        )
    })?;
    let exact_prefix_candidate_package_binding = sequence.packages.len() == full_entries.len()
        && package.stage == stage
        && package.candidate_hash == candidate_digest
        && package.predecessor_signature_digest == predecessor_signature_digest;
    if !exact_prefix_candidate_package_binding {
        return Err(BranchPrefixGeneralProvenanceV4Error::Projection(
            "last package is not the exact requested candidate/prefix extension".to_owned(),
        ));
    }

    let v3_rule = issue_act_local_v3_typed_r2_rule_token();
    let declarations = issue_act_local_prefix_declaration_sequence_v3(&full_entries, &v3_rule)
        .map_err(|error| BranchPrefixGeneralProvenanceV4Error::Construction(error.to_string()))?;
    let (current_declaration, predecessor_declarations) =
        declarations.split_last().ok_or_else(|| {
            BranchPrefixGeneralProvenanceV4Error::Projection(
                "prefix declaration sequence was empty".to_owned(),
            )
        })?;
    let v4_r1_rule = issue_v4_prefix_local_typed_r1_rule_token();
    let v4_source = issue_v4_prefix_general_source_surface(
        &predecessor_signature,
        stage,
        candidate,
        predecessor_declarations,
        current_declaration,
        &v3_rule,
        &v4_r1_rule,
    )
    .map_err(|error| BranchPrefixGeneralProvenanceV4Error::Construction(error.to_string()))?;
    let v4_source_replayed_and_matches_package = replay_v4_prefix_general_source_surface(
        &predecessor_signature,
        candidate,
        predecessor_declarations,
        current_declaration,
        &v3_rule,
        &v4_r1_rule,
        &v4_source,
    )
    .is_empty()
        && v4_source.source_hash == package.v4_candidate_local_source_hash;
    if !v4_source_replayed_and_matches_package {
        return Err(BranchPrefixGeneralProvenanceV4Error::Projection(
            "replayed v4 prefix-general source does not match the v5 package".to_owned(),
        ));
    }

    let issuance_trace_replayed =
        sequence
            .issuance_receipts
            .iter()
            .enumerate()
            .all(|(index, receipt)| {
                replay_prefix_local_issuance_receipt(&sequence.issuance_receipts[..index], receipt)
            });
    let mut credited_family_rows = package
        .family_rows
        .iter()
        .filter(|row| row.credited)
        .map(|row| BranchPrefixGeneralFamilyCreditV4 {
            family_id: row.family_id.clone(),
            authoritative_family_row_derivation_hash: row.derivation_hash.clone(),
        })
        .collect::<Vec<_>>();
    credited_family_rows.sort_by(|left, right| left.family_id.cmp(&right.family_id));
    let credited_family_ids = credited_family_rows
        .iter()
        .map(|row| row.family_id.clone())
        .collect::<Vec<_>>();
    if credited_family_ids != package.credited_family_ids
        || credited_family_ids.len() != package.semantic_nu as usize
    {
        return Err(BranchPrefixGeneralProvenanceV4Error::Projection(
            "credited family rows do not exactly project the semantic nu".to_owned(),
        ));
    }

    let (proved_family_rows, theorem_impossibility_rows) =
        project_branch_prefix_general_role_classifications_v4(package)?;
    let classification_evidence_hashes = package
        .role_rows
        .iter()
        .map(|row| row.derivation_hash.clone())
        .collect::<Vec<_>>();
    let all_residual_counts_zero = package.named_role_residual_count == 0
        && package.named_quotient_residual_count == 0
        && package.named_a3_residual_count == 0
        && package.silent_residue_count == 0;
    let no_archive_or_historical_registry_input = sequence.no_historical_registry_or_future_input
        && !package.historical_registry_consulted
        && !package.archive_structural_bar_verdict_or_future_read
        && !v4_source.archive_read
        && !v4_source.historical_kind_surface_read;
    let no_named_residual_class = package
        .role_rows
        .iter()
        .all(|row| row.resolution_class != V5PrefixLocalResolutionClass::NamedResidual);
    let classifications_partition = proved_family_rows.len() + theorem_impossibility_rows.len()
        == package.role_declaration_count
        && no_named_residual_class;
    let r2 = &v4_source.r1_r2_premises;
    let proved = authority.proved
        && replay_v5_prefix_local_rule_authority(&sequence.rule_authority)
        && sequence.schema == V5_PREFIX_GENERAL_SEMANTIC_SEQUENCE_SCHEMA
        && sequence.theorem_id == T_BI_B1_B2_PREFIX_GENERAL_THEOREM_ID
        && sequence.rule_authority.derivation_hash == authority.v5_rule_authority_derivation_hash
        && sequence.every_package_closed_observed_grammar
        && sequence.every_package_registry_extension_invariant
        && sequence.t_bi_b1_proved_on_sequence
        && sequence.t_bi_b2_proved_on_sequence
        && package.proved
        && package.t_bi_b1_proved
        && package.t_bi_b2_proved
        && package.every_role_declaration_resolved
        && package.every_marginal_family_credited_or_theorem_impossible
        && package.local_anchor_nonreuse_holds
        && exact_prefix_candidate_package_binding
        && v4_source_replayed_and_matches_package
        && issuance_trace_replayed
        && independent_reissuance_equal
        && all_residual_counts_zero
        && classifications_partition
        && r2.r2_local_premises_replayed
        && no_archive_or_historical_registry_input;
    if !proved {
        return Err(BranchPrefixGeneralProvenanceV4Error::Projection(
            "prefix-general semantic classification did not close".to_owned(),
        ));
    }

    let mut token = BranchPrefixGeneralProvenanceV4Token {
        schema: BRANCH_PREFIX_GENERAL_PROVENANCE_V4_SCHEMA.to_owned(),
        date: BRANCH_PREFIX_GENERAL_PROVENANCE_V4_DATE.to_owned(),
        theorem_id: BRANCH_PREFIX_GENERAL_PROVENANCE_V4_THEOREM_ID.to_owned(),
        stage,
        candidate_hash: candidate_digest,
        predecessor_entry_count: predecessor_entries.len(),
        predecessor_stages: predecessor_entries
            .iter()
            .map(|(entry_stage, _)| *entry_stage)
            .collect(),
        predecessor_candidate_hashes: predecessor_entries
            .iter()
            .map(|(_, predecessor)| candidate_hash(predecessor))
            .collect(),
        predecessor_signature_digest,
        extraction_procedure_digest: authority.derivation_hash,
        rule_authority_derivation_hash: sequence.rule_authority.derivation_hash.clone(),
        sequence_schema: sequence.schema,
        sequence_theorem_id: sequence.theorem_id,
        sequence_derivation_hash: sequence.derivation_hash,
        sequence_authoritative_seal: sequence.authoritative_sequence_seal,
        sequence_issuance_trace_root: sequence.issuance_trace_root,
        package_derivation_hash: package.derivation_hash.clone(),
        package_v3_declaration_hash: package.v3_declaration_hash.clone(),
        package_v4_candidate_local_source_hash: package.v4_candidate_local_source_hash.clone(),
        semantic_nu: package.semantic_nu,
        credited_family_ids,
        credited_family_rows,
        proved_family_rows,
        theorem_impossibility_rows,
        classification_evidence_hashes,
        role_declaration_count: package.role_declaration_count,
        proved_family_declaration_count: package.proved_family_declaration_count,
        theorem_impossibility_declaration_count: package.theorem_impossibility_declaration_count,
        named_role_residual_count: package.named_role_residual_count,
        named_quotient_residual_count: package.named_quotient_residual_count,
        named_a3_residual_count: package.named_a3_residual_count,
        silent_residue_count: package.silent_residue_count,
        r2_generated_instance_removed_count: r2.r2_generated_instance_removed_count,
        r2_removed_occurrence_hashes: r2.r2_removed_occurrence_hashes.clone(),
        r2_step8_typed_signature_derivation_hash: r2
            .r2_step8_typed_signature_derivation_hash
            .clone(),
        r2_m1_generated_membership_derivation_hash: r2
            .r2_m1_generated_membership_derivation_hash
            .clone(),
        r2_local_premises_replayed: r2.r2_local_premises_replayed,
        exact_prefix_candidate_package_binding,
        v4_source_replayed_and_matches_package,
        issuance_trace_replayed,
        independent_reissuance_equal,
        all_residual_counts_zero,
        every_role_declaration_resolved: package.every_role_declaration_resolved,
        every_marginal_family_credited_or_theorem_impossible: package
            .every_marginal_family_credited_or_theorem_impossible,
        local_anchor_nonreuse_holds: package.local_anchor_nonreuse_holds,
        no_archive_or_historical_registry_input,
        structural_nu_input_read: false,
        bar_input_read: false,
        verdict_input_read: false,
        enacted_future_input_read: false,
        enacted_classification_input_read: false,
        byte_identity_input_read: false,
        proved,
        derivation_hash: String::new(),
    };
    token.derivation_hash = token_hash(&token);
    Ok(token)
}

pub fn validate_branch_prefix_general_provenance_v4_token_integrity(
    claimed: &BranchPrefixGeneralProvenanceV4Token,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != token_hash(claimed) {
        errors.push("BI-1b prefix-general token digest mismatch".to_owned());
    }
    let authority = issue_branch_prefix_general_provenance_v4_authority();
    if claimed.extraction_procedure_digest != authority.derivation_hash
        || claimed.rule_authority_derivation_hash != authority.v5_rule_authority_derivation_hash
    {
        errors.push("BI-1b extraction procedure/rule authority mismatch".to_owned());
    }
    if claimed.schema != BRANCH_PREFIX_GENERAL_PROVENANCE_V4_SCHEMA
        || claimed.date != BRANCH_PREFIX_GENERAL_PROVENANCE_V4_DATE
        || claimed.theorem_id != BRANCH_PREFIX_GENERAL_PROVENANCE_V4_THEOREM_ID
        || claimed.sequence_schema != V5_PREFIX_GENERAL_SEMANTIC_SEQUENCE_SCHEMA
        || claimed.sequence_theorem_id != T_BI_B1_B2_PREFIX_GENERAL_THEOREM_ID
    {
        errors.push("BI-1b token schema/date/theorem mismatch".to_owned());
    }
    if claimed.stage == 0
        || claimed.predecessor_entry_count + 1 != claimed.stage as usize
        || claimed.predecessor_stages != (1..claimed.stage).collect::<Vec<_>>()
        || claimed.predecessor_candidate_hashes.len() != claimed.predecessor_entry_count
    {
        errors.push("BI-1b token predecessor shape is not exact and contiguous".to_owned());
    }
    let credited_ids = claimed
        .credited_family_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let credited_rows = claimed
        .credited_family_rows
        .iter()
        .map(|row| row.family_id.as_str())
        .collect::<BTreeSet<_>>();
    if credited_ids != credited_rows
        || credited_ids.len() != claimed.credited_family_ids.len()
        || credited_rows.len() != claimed.credited_family_rows.len()
        || claimed.semantic_nu as usize != claimed.credited_family_rows.len()
    {
        errors.push("BI-1b credited family projection is not exact".to_owned());
    }
    let residuals_zero = claimed.named_role_residual_count == 0
        && claimed.named_quotient_residual_count == 0
        && claimed.named_a3_residual_count == 0
        && claimed.silent_residue_count == 0;
    let declaration_ids = claimed
        .proved_family_rows
        .iter()
        .chain(&claimed.theorem_impossibility_rows)
        .map(|row| row.declaration_id.as_str())
        .collect::<BTreeSet<_>>();
    let classification_partition_exact = claimed.proved_family_rows.len()
        == claimed.proved_family_declaration_count
        && claimed.theorem_impossibility_rows.len()
            == claimed.theorem_impossibility_declaration_count
        && claimed.proved_family_declaration_count
            + claimed.theorem_impossibility_declaration_count
            == claimed.role_declaration_count
        && declaration_ids.len() == claimed.role_declaration_count
        && claimed
            .proved_family_rows
            .iter()
            .all(|row| row.resolution_class == V5PrefixLocalResolutionClass::ProvedFamily)
        && claimed.theorem_impossibility_rows.iter().all(|row| {
            row.resolution_class == V5PrefixLocalResolutionClass::TheoremBackedImpossibility
        });
    if claimed.all_residual_counts_zero != residuals_zero
        || !classification_partition_exact
        || claimed.classification_evidence_hashes.len() != claimed.role_declaration_count
        || claimed
            .proved_family_rows
            .iter()
            .chain(&claimed.theorem_impossibility_rows)
            .any(|row| row.classification_hash != classification_hash(row))
        || claimed.r2_generated_instance_removed_count != claimed.r2_removed_occurrence_hashes.len()
    {
        errors.push("BI-1b classification/residual/R2 count projection is inconsistent".to_owned());
    }
    if claimed.structural_nu_input_read
        || claimed.bar_input_read
        || claimed.verdict_input_read
        || claimed.enacted_future_input_read
        || claimed.enacted_classification_input_read
        || claimed.byte_identity_input_read
        || !claimed.no_archive_or_historical_registry_input
    {
        errors.push("BI-1b forbidden input capability is present".to_owned());
    }
    if !claimed.proved
        || !claimed.exact_prefix_candidate_package_binding
        || !claimed.v4_source_replayed_and_matches_package
        || !claimed.issuance_trace_replayed
        || !claimed.independent_reissuance_equal
        || !claimed.all_residual_counts_zero
        || !claimed.every_role_declaration_resolved
        || !claimed.every_marginal_family_credited_or_theorem_impossible
        || !claimed.local_anchor_nonreuse_holds
        || !claimed.r2_local_premises_replayed
    {
        errors.push("BI-1b closing flags do not prove the token".to_owned());
    }
    errors
}

pub fn replay_branch_prefix_general_provenance_v4(
    predecessor_entries: &[(u32, Telescope)],
    candidate: &Telescope,
    claimed: &BranchPrefixGeneralProvenanceV4Token,
) -> Vec<String> {
    let mut errors = validate_branch_prefix_general_provenance_v4_token_integrity(claimed);
    match issue_branch_prefix_general_provenance_v4(predecessor_entries, claimed.stage, candidate) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors
            .push("BI-1b token differs from deterministic prefix-general reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::branch_semantic_provenance_v3::issue_branch_semantic_provenance_v3;
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;

    fn predecessor_prefix(stage: u32, alternate_stage4: bool) -> Vec<(u32, Telescope)> {
        let alternate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                ))),
            ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(
                    Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(3)))),
                    Box::new(Expr::Var(2)),
                ),
            ),
            ClauseRec::new(
                ClauseRole::Elimination,
                Expr::App(
                    Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Var(2)),
                ),
            ),
        ]);
        (1..stage)
            .map(|entry_stage| {
                (
                    entry_stage,
                    if alternate_stage4 && entry_stage == 4 {
                        alternate.clone()
                    } else {
                        Telescope::reference(entry_stage)
                    },
                )
            })
            .collect()
    }

    #[test]
    fn zero_input_authority_is_stable_and_forbids_every_b1b_input() {
        let authority = issue_branch_prefix_general_provenance_v4_authority();
        assert!(authority.proved);
        assert!(authority.one_procedure_for_every_prefix);
        assert!(authority.candidate_and_exact_prefix_are_only_semantic_inputs);
        assert!(!authority.archive_input_read);
        assert!(!authority.enacted_classification_input_read);
        assert!(!authority.byte_identity_input_read);
        assert!(replay_branch_prefix_general_provenance_v4_authority(&authority).is_empty());
    }

    #[test]
    #[ignore = "source-first Stage-8 double issuance is intentionally expensive"]
    fn enacted_stage8_classification_regresses_exactly_to_v3() {
        let predecessors = predecessor_prefix(8, false);
        let candidate = Telescope::reference(8);
        let legacy = issue_branch_semantic_provenance_v3(&predecessors, 8, &candidate).unwrap();
        let mut full_entries = predecessors.clone();
        full_entries.push((8, candidate.clone()));
        let legacy_sequence =
            crate::act_local_semantic_provenance_v5::issue_prefix_local_semantic_sequence_v5(
                &full_entries,
            )
            .unwrap();
        let (legacy_proved_rows, legacy_impossible_rows) =
            project_branch_prefix_general_role_classifications_v4(
                legacy_sequence.packages.last().unwrap(),
            )
            .unwrap();
        let general =
            issue_branch_prefix_general_provenance_v4(&predecessors, 8, &candidate).unwrap();
        assert_eq!(general.semantic_nu, legacy.semantic_nu);
        assert_eq!(general.credited_family_ids, legacy.credited_family_ids);
        assert_eq!(general.proved_family_rows, legacy_proved_rows);
        assert_eq!(general.theorem_impossibility_rows, legacy_impossible_rows);
        assert_eq!(
            general.proved_family_declaration_count,
            legacy.proved_family_declaration_count
        );
        assert_eq!(
            general.theorem_impossibility_declaration_count,
            legacy.theorem_impossibility_declaration_count
        );
        assert!(general.all_residual_counts_zero);
    }

    #[test]
    #[ignore = "source-first Stage-8 double issuance is intentionally expensive"]
    fn alternate_stage8_closes_where_legacy_historical_b7_gate_stops() {
        let predecessors = predecessor_prefix(8, true);
        let candidate = Telescope::reference(8);
        let legacy = issue_branch_semantic_provenance_v3(&predecessors, 8, &candidate).unwrap_err();
        assert!(legacy.to_string().contains("exact sealed B_7"));
        let general =
            issue_branch_prefix_general_provenance_v4(&predecessors, 8, &candidate).unwrap();
        assert!(general.proved);
        assert_eq!(general.candidate_hash, candidate_hash(&candidate));
        assert_eq!(general.semantic_nu, 1);
        assert_eq!(general.r2_generated_instance_removed_count, 1);
        assert!(general.all_residual_counts_zero);
        assert!(general.no_archive_or_historical_registry_input);
        assert!(
            replay_branch_prefix_general_provenance_v4(&predecessors, &candidate, &general)
                .is_empty()
        );
    }

    #[test]
    #[ignore = "source-first Stage-8 double issuance is intentionally expensive"]
    fn prefix_candidate_and_fully_rehashed_mutations_fail_replay() {
        let predecessors = predecessor_prefix(8, true);
        let candidate = Telescope::reference(8);
        let token =
            issue_branch_prefix_general_provenance_v4(&predecessors, 8, &candidate).unwrap();

        let mut altered_prefix = predecessors.clone();
        altered_prefix[0].1 = Telescope::reference(2);
        assert!(
            !replay_branch_prefix_general_provenance_v4(&altered_prefix, &candidate, &token)
                .is_empty()
        );

        let altered_candidate = Telescope::reference(7);
        assert!(
            !replay_branch_prefix_general_provenance_v4(&predecessors, &altered_candidate, &token)
                .is_empty()
        );

        let mut fully_rehashed = token.clone();
        fully_rehashed.package_derivation_hash.push('0');
        fully_rehashed.classification_evidence_hashes[0].push('0');
        fully_rehashed.derivation_hash = token_hash(&fully_rehashed);
        assert_eq!(fully_rehashed.derivation_hash, token_hash(&fully_rehashed));
        assert!(
            !replay_branch_prefix_general_provenance_v4(&predecessors, &candidate, &fully_rehashed)
                .is_empty()
        );
    }
}
