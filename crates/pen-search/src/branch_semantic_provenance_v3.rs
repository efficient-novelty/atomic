//! Compact, replayable source-first semantic evidence for one branch candidate.
//!
//! The caller supplies only an exact contiguous predecessor prefix and the
//! next `(stage, candidate)`.  Issuance first proves T-BI-B3 over that exact
//! extension and recovers the v5 packages from B3's combined two-issuance
//! replay context.  The already-replayed authoritative sequence is consumed
//! directly, so no third adapter reissuance occurs.  The resulting token
//! contains no structural novelty scalar, bar, archive, enacted suffix, or
//! desired semantic vector.

use crate::act_local_provenance_v3::{
    issue_act_local_prefix_declaration_sequence_v3, issue_act_local_v3_typed_r2_rule_token,
};
use crate::act_local_semantic_provenance_v4::{
    issue_v4_prefix_local_source_surface, issue_v4_prefix_local_typed_r1_rule_token,
    replay_v4_prefix_local_source_surface,
};
use crate::act_local_semantic_provenance_v5::V5PrefixLocalSemanticPackageProof;
use crate::t_bi_intrinsic_isolation_v3::{
    T_BI_B3_V3_THEOREM_ID, TBiIntrinsicIsolationV3Error,
    issue_replayed_t_bi_intrinsic_isolation_v3_context,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const BRANCH_SEMANTIC_PROVENANCE_V3_SCHEMA: &str =
    "branch-prefix-candidate-semantic-provenance-v3";
pub const BRANCH_SEMANTIC_PROVENANCE_V3_DATE: &str = "2026-07-23";
pub const BRANCH_SEMANTIC_PROVENANCE_V3_THEOREM_ID: &str =
    "T-BI-branch-v3-exact-prefix-candidate-source-first-semantic-evidence";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BRANCH_SEMANTIC_PROVENANCE_V3_SCHEMA, domain, value))
        .expect("branch semantic evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

/// The exact authoritative v5 family-row digest corresponding to one unit of
/// semantic credit.  Ordering follows v5's sorted `credited_family_ids`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchSemanticFamilyCreditV3 {
    pub family_id: String,
    pub authoritative_family_row_derivation_hash: String,
}

/// Additive Stage-8 R2 premise projection.  This is reconstructed from the
/// public prefix-local v3/v4 issuers and joined to the isolated v5 package by
/// the exact v4 source hash.  It is zero/none/trivial at every non-Stage-8
/// position.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchSemanticR2PremiseV3 {
    pub stage: u32,
    pub v3_r2_rule_authority_hash: String,
    pub v4_r1_rule_authority_hash: String,
    pub v3_declaration_surface_hash: String,
    pub v4_prefix_local_source_hash: String,
    pub v4_source_matches_isolated_package: bool,
    pub v4_source_surface_replayed: bool,
    pub r2_premises_derivation_hash: String,
    pub r2_generated_instance_removed_count: usize,
    pub r2_removed_occurrence_hashes: Vec<String>,
    pub r2_parent_slot_label_join_holds: bool,
    pub r2_exact_v3_occurrence_surface_holds: bool,
    pub r2_step8_typed_signature_derivation_hash: Option<String>,
    pub r2_m1_generated_membership_derivation_hash: Option<String>,
    pub r2_m1_generated_membership_replayed: bool,
    pub r2_parent_membership_replayed: bool,
    pub r2_removed_occurrences_absent_from_unified_membership: bool,
    pub r2_removed_occurrences_not_exported_as_families: bool,
    pub r2_generated_instance_not_multiplied: bool,
    pub r2_local_premises_replayed: bool,
    pub non_stage8_projection_is_exact_zero_none_trivial: bool,
    pub v4_forbidden_inputs_absent: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchSemanticProvenanceV3Token {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_entry_count: usize,
    pub predecessor_stages: Vec<u32>,
    pub predecessor_candidate_hashes: Vec<String>,
    pub predecessor_signature_digest: String,
    pub semantic_nu: u32,
    pub credited_family_ids: Vec<String>,
    pub credited_family_rows: Vec<BranchSemanticFamilyCreditV3>,
    pub exact_credited_family_row_join: bool,
    pub package_derivation_hash: String,
    pub package_v3_declaration_hash: String,
    pub package_v4_candidate_local_source_hash: String,
    pub sequence_derivation_hash: String,
    pub sequence_authoritative_seal: String,
    pub sequence_issuance_trace_root: String,
    pub b3_theorem_id: String,
    pub b3_token_derivation_hash: String,
    pub b3_authoritative_prefix_semantic_seal: String,
    pub b3_issuance_trace_root: String,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_role_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub silent_residue_count: usize,
    pub t_bi_b1_proved: bool,
    pub t_bi_b2_proved: bool,
    pub t_bi_b3_proved: bool,
    pub package_proved: bool,
    pub every_role_declaration_resolved: bool,
    pub every_marginal_family_credited_or_theorem_impossible: bool,
    pub local_anchor_nonreuse_holds: bool,
    pub no_historical_registry_consulted: bool,
    pub no_archive_structural_bar_verdict_or_future_input: bool,
    pub sequence_no_historical_registry_or_future_input: bool,
    pub b3_no_historical_registry_or_legacy_v5_authority: bool,
    pub b3_no_archive_structural_bar_verdict_or_future_input: bool,
    pub exact_prefix_candidate_package_binding: bool,
    pub all_residual_counts_zero: bool,
    pub r2_premise: BranchSemanticR2PremiseV3,
    pub no_history_or_forbidden_input: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BranchSemanticProvenanceV3Error {
    #[error("branch semantic v3 invalid input: {0}")]
    Input(String),
    #[error("branch semantic v3 intrinsic isolation failed: {0}")]
    Isolation(String),
    #[error("branch semantic v3 projection failed: {0}")]
    Projection(String),
}

impl From<TBiIntrinsicIsolationV3Error> for BranchSemanticProvenanceV3Error {
    fn from(error: TBiIntrinsicIsolationV3Error) -> Self {
        Self::Isolation(error.to_string())
    }
}

fn r2_premise_hash(premise: &BranchSemanticR2PremiseV3) -> String {
    let mut projection = premise.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-r2-premise-projection", &projection)
}

fn token_hash(token: &BranchSemanticProvenanceV3Token) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("branch-semantic-provenance-token", &projection)
}

fn validate_input(
    predecessor_entries: &[(u32, Telescope)],
    stage: u32,
) -> Result<(), BranchSemanticProvenanceV3Error> {
    if stage == 0 || stage as usize != predecessor_entries.len() + 1 {
        return Err(BranchSemanticProvenanceV3Error::Input(format!(
            "candidate Stage {stage} requires exactly {} predecessor entries",
            stage.saturating_sub(1)
        )));
    }
    if !predecessor_entries
        .iter()
        .map(|(entry_stage, _)| *entry_stage)
        .eq(1..stage)
    {
        return Err(BranchSemanticProvenanceV3Error::Input(
            "predecessor entries must be the exact contiguous Stage-1-through-(N-1) prefix"
                .to_owned(),
        ));
    }
    Ok(())
}

fn project_credited_rows(
    package: &V5PrefixLocalSemanticPackageProof,
) -> Result<(Vec<BranchSemanticFamilyCreditV3>, bool), BranchSemanticProvenanceV3Error> {
    let mut rows = Vec::with_capacity(package.credited_family_ids.len());
    for family_id in &package.credited_family_ids {
        let matches = package
            .family_rows
            .iter()
            .filter(|row| row.family_id == *family_id && row.credited)
            .collect::<Vec<_>>();
        if matches.len() != 1 || matches[0].derivation_hash.is_empty() {
            return Err(BranchSemanticProvenanceV3Error::Projection(format!(
                "credited family {family_id} does not have exactly one authoritative credited row"
            )));
        }
        rows.push(BranchSemanticFamilyCreditV3 {
            family_id: family_id.clone(),
            authoritative_family_row_derivation_hash: matches[0].derivation_hash.clone(),
        });
    }
    let credited_row_ids = package
        .family_rows
        .iter()
        .filter(|row| row.credited)
        .map(|row| row.family_id.as_str())
        .collect::<BTreeSet<_>>();
    let credited_id_set = package
        .credited_family_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let exact = credited_row_ids == credited_id_set
        && rows.len() == package.credited_family_ids.len()
        && u32::try_from(rows.len()).ok() == Some(package.semantic_nu);
    if !exact {
        return Err(BranchSemanticProvenanceV3Error::Projection(
            "credited family IDs, authoritative family rows, and semantic nu do not coincide"
                .to_owned(),
        ));
    }
    Ok((rows, exact))
}

fn issue_r2_premise(
    full_entries: &[(u32, Telescope)],
    predecessor_signature: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    package: &V5PrefixLocalSemanticPackageProof,
) -> Result<BranchSemanticR2PremiseV3, BranchSemanticProvenanceV3Error> {
    let v3_r2_rule = issue_act_local_v3_typed_r2_rule_token();
    let declarations = issue_act_local_prefix_declaration_sequence_v3(full_entries, &v3_r2_rule)
        .map_err(|error| BranchSemanticProvenanceV3Error::Projection(error.to_string()))?;
    let (current, predecessors) = declarations.split_last().ok_or_else(|| {
        BranchSemanticProvenanceV3Error::Projection(
            "v3 prefix declaration sequence was unexpectedly empty".to_owned(),
        )
    })?;
    let v4_r1_rule = issue_v4_prefix_local_typed_r1_rule_token();
    let surface = issue_v4_prefix_local_source_surface(
        predecessor_signature,
        stage,
        candidate,
        predecessors,
        current,
        &v3_r2_rule,
        &v4_r1_rule,
    )
    .map_err(|error| BranchSemanticProvenanceV3Error::Projection(error.to_string()))?;
    let v4_source_surface_replayed = replay_v4_prefix_local_source_surface(
        predecessor_signature,
        candidate,
        predecessors,
        current,
        &v3_r2_rule,
        &v4_r1_rule,
        &surface,
    )
    .is_empty();
    let v4_source_matches_isolated_package =
        surface.source_hash == package.v4_candidate_local_source_hash;
    if !v4_source_surface_replayed || !v4_source_matches_isolated_package {
        return Err(BranchSemanticProvenanceV3Error::Projection(
            "reissued v4 source surface does not replay or does not match the isolated v5 package"
                .to_owned(),
        ));
    }

    let premises = &surface.r1_r2_premises;
    let removed_not_exported = premises.r2_removed_occurrences_emitted_as_families == 0;
    let not_multiplied = !premises.r2_generated_instance_multiplied;
    let v4_forbidden_inputs_absent = !surface.archive_read
        && !surface.structural_nu_read
        && !surface.bar_read
        && !surface.verdict_read
        && !surface.enacted_future_read
        && !surface.historical_kind_surface_read
        && !surface.declaration_resolution_read
        && !surface.anchor_assignment_read;
    let trivial_projection = premises.r2_generated_instance_removed_count == 0
        && premises.r2_removed_occurrence_hashes.is_empty()
        && premises.r2_step8_typed_signature_derivation_hash.is_none()
        && premises
            .r2_m1_generated_membership_derivation_hash
            .is_none()
        && premises.r2_parent_slot_label_join_holds
        && premises.r2_exact_v3_occurrence_surface_holds
        && premises.r2_m1_generated_membership_replayed
        && premises.r2_parent_membership_replayed
        && premises.r2_removed_occurrences_absent_from_unified_membership
        && removed_not_exported
        && not_multiplied
        && premises.r2_local_premises_replayed;
    let non_stage8_projection_is_exact_zero_none_trivial = stage == 8 || trivial_projection;
    let hash_surface_exact = if premises.r2_removed_occurrence_hashes.is_empty() {
        premises.r2_generated_instance_removed_count == 0
            && premises.r2_step8_typed_signature_derivation_hash.is_none()
            && premises
                .r2_m1_generated_membership_derivation_hash
                .is_none()
    } else {
        premises.r2_generated_instance_removed_count == premises.r2_removed_occurrence_hashes.len()
            && premises.r2_step8_typed_signature_derivation_hash.is_some()
            && premises
                .r2_m1_generated_membership_derivation_hash
                .is_some()
    };
    let proved = v4_source_matches_isolated_package
        && v4_source_surface_replayed
        && premises.stage == stage
        && premises.candidate_hash == package.candidate_hash
        && premises.predecessor_signature_digest == package.predecessor_signature_digest
        && surface.v3_r2_rule_authority_hash == v3_r2_rule.derivation_hash
        && premises.r1_rule_authority_hash == v4_r1_rule.derivation_hash
        && premises.r2_parent_slot_label_join_holds
        && premises.r2_exact_v3_occurrence_surface_holds
        && premises.r2_m1_generated_membership_replayed
        && premises.r2_parent_membership_replayed
        && premises.r2_removed_occurrences_absent_from_unified_membership
        && removed_not_exported
        && not_multiplied
        && premises.r2_local_premises_replayed
        && non_stage8_projection_is_exact_zero_none_trivial
        && hash_surface_exact
        && v4_forbidden_inputs_absent;
    if !proved {
        return Err(BranchSemanticProvenanceV3Error::Projection(format!(
            "Stage {stage} prefix-local R2 premise projection did not close"
        )));
    }

    let mut projection = BranchSemanticR2PremiseV3 {
        stage,
        v3_r2_rule_authority_hash: v3_r2_rule.derivation_hash,
        v4_r1_rule_authority_hash: v4_r1_rule.derivation_hash,
        v3_declaration_surface_hash: current.surface_hash.clone(),
        v4_prefix_local_source_hash: surface.source_hash,
        v4_source_matches_isolated_package,
        v4_source_surface_replayed,
        r2_premises_derivation_hash: premises.derivation_hash.clone(),
        r2_generated_instance_removed_count: premises.r2_generated_instance_removed_count,
        r2_removed_occurrence_hashes: premises.r2_removed_occurrence_hashes.clone(),
        r2_parent_slot_label_join_holds: premises.r2_parent_slot_label_join_holds,
        r2_exact_v3_occurrence_surface_holds: premises.r2_exact_v3_occurrence_surface_holds,
        r2_step8_typed_signature_derivation_hash: premises
            .r2_step8_typed_signature_derivation_hash
            .clone(),
        r2_m1_generated_membership_derivation_hash: premises
            .r2_m1_generated_membership_derivation_hash
            .clone(),
        r2_m1_generated_membership_replayed: premises.r2_m1_generated_membership_replayed,
        r2_parent_membership_replayed: premises.r2_parent_membership_replayed,
        r2_removed_occurrences_absent_from_unified_membership: premises
            .r2_removed_occurrences_absent_from_unified_membership,
        r2_removed_occurrences_not_exported_as_families: removed_not_exported,
        r2_generated_instance_not_multiplied: not_multiplied,
        r2_local_premises_replayed: premises.r2_local_premises_replayed,
        non_stage8_projection_is_exact_zero_none_trivial,
        v4_forbidden_inputs_absent,
        proved,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = r2_premise_hash(&projection);
    Ok(projection)
}

/// Issue compact source-first evidence for one candidate over its exact
/// predecessor prefix.  The v5 sequence is never issued directly here: it is
/// recovered from the combined T-BI-B3-v3 issue/replay context after exactly
/// one independent replay, with no third adapter reissuance.
pub fn issue_branch_semantic_provenance_v3(
    predecessor_entries: &[(u32, Telescope)],
    stage: u32,
    candidate: &Telescope,
) -> Result<BranchSemanticProvenanceV3Token, BranchSemanticProvenanceV3Error> {
    validate_input(predecessor_entries, stage)?;
    let predecessor_signature = SealedSignature::from_telescopes(predecessor_entries.to_vec());
    let predecessor_signature_digest = predecessor_signature.digest().to_owned();
    let mut full_entries = predecessor_entries.to_vec();
    full_entries.push((stage, candidate.clone()));

    let replay_context = issue_replayed_t_bi_intrinsic_isolation_v3_context(&full_entries)?;
    let b3 = replay_context.token;
    let sequence = replay_context.sequence;
    let package = sequence.packages.last().ok_or_else(|| {
        BranchSemanticProvenanceV3Error::Projection(
            "isolated semantic sequence contained no candidate package".to_owned(),
        )
    })?;
    let candidate_digest = candidate_hash(candidate);
    let exact_prefix_candidate_package_binding = sequence.packages.len() == full_entries.len()
        && package.stage == stage
        && package.candidate_hash == candidate_digest
        && package.predecessor_signature_digest == predecessor_signature_digest
        && b3.prefix_len == full_entries.len()
        && b3.last_stage == stage
        && b3.prefix_bindings.last().is_some_and(|binding| {
            binding.stage == stage
                && binding.candidate_hash == candidate_digest
                && binding.predecessor_signature_digest == predecessor_signature_digest
                && binding.semantic_package_derivation_hash == package.derivation_hash
                && binding.exact_candidate_and_prefix_binding
                && binding.exact_package_source_v3_grammar_predecessor_commitments
        });
    if !exact_prefix_candidate_package_binding {
        return Err(BranchSemanticProvenanceV3Error::Projection(
            "last isolated package is not the exact requested prefix extension".to_owned(),
        ));
    }
    let (credited_family_rows, exact_credited_family_row_join) = project_credited_rows(package)?;
    let r2_premise = issue_r2_premise(
        &full_entries,
        &predecessor_signature,
        stage,
        candidate,
        package,
    )?;

    let all_residual_counts_zero = package.named_role_residual_count == 0
        && package.named_quotient_residual_count == 0
        && package.named_a3_residual_count == 0
        && package.silent_residue_count == 0;
    let t_bi_b3_proved = b3.prefix_generic_transitive_isolation_proved
        && b3.prefix_local_sequence_replayed
        && b3.every_local_package_proved_b1_b2
        && b3.every_registry_extension_projection_equal
        && b3.exact_package_source_v3_grammar_predecessor_commitments
        && b3.issuance_trace_root_exact;
    let no_historical_registry_consulted = !package.historical_registry_consulted;
    let no_archive_structural_bar_verdict_or_future_input =
        !package.archive_structural_bar_verdict_or_future_read;
    let b3_no_historical_registry_or_legacy_v5_authority =
        b3.no_historical_registry_or_legacy_v5_authority;
    let b3_no_archive_structural_bar_verdict_or_future_input =
        b3.no_archive_structural_bar_verdict_or_future_input;
    let no_history_or_forbidden_input = no_historical_registry_consulted
        && no_archive_structural_bar_verdict_or_future_input
        && sequence.no_historical_registry_or_future_input
        && b3_no_historical_registry_or_legacy_v5_authority
        && b3_no_archive_structural_bar_verdict_or_future_input
        && r2_premise.v4_forbidden_inputs_absent;
    let proved = exact_prefix_candidate_package_binding
        && exact_credited_family_row_join
        && package.proved
        && package.t_bi_b1_proved
        && package.t_bi_b2_proved
        && t_bi_b3_proved
        && package.every_role_declaration_resolved
        && package.every_marginal_family_credited_or_theorem_impossible
        && package.local_anchor_nonreuse_holds
        && all_residual_counts_zero
        && r2_premise.proved
        && no_history_or_forbidden_input;
    if !proved {
        return Err(BranchSemanticProvenanceV3Error::Projection(format!(
            "Stage {stage} exact candidate semantic evidence did not close"
        )));
    }

    let mut token = BranchSemanticProvenanceV3Token {
        schema: BRANCH_SEMANTIC_PROVENANCE_V3_SCHEMA.to_owned(),
        date: BRANCH_SEMANTIC_PROVENANCE_V3_DATE.to_owned(),
        theorem_id: BRANCH_SEMANTIC_PROVENANCE_V3_THEOREM_ID.to_owned(),
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
        semantic_nu: package.semantic_nu,
        credited_family_ids: package.credited_family_ids.clone(),
        credited_family_rows,
        exact_credited_family_row_join,
        package_derivation_hash: package.derivation_hash.clone(),
        package_v3_declaration_hash: package.v3_declaration_hash.clone(),
        package_v4_candidate_local_source_hash: package.v4_candidate_local_source_hash.clone(),
        sequence_derivation_hash: sequence.derivation_hash,
        sequence_authoritative_seal: sequence.authoritative_sequence_seal,
        sequence_issuance_trace_root: sequence.issuance_trace_root,
        b3_theorem_id: b3.theorem_id,
        b3_token_derivation_hash: b3.derivation_hash,
        b3_authoritative_prefix_semantic_seal: b3.authoritative_prefix_semantic_seal,
        b3_issuance_trace_root: b3.issuance_trace_root,
        role_declaration_count: package.role_declaration_count,
        proved_family_declaration_count: package.proved_family_declaration_count,
        theorem_impossibility_declaration_count: package.theorem_impossibility_declaration_count,
        named_role_residual_count: package.named_role_residual_count,
        named_quotient_residual_count: package.named_quotient_residual_count,
        named_a3_residual_count: package.named_a3_residual_count,
        silent_residue_count: package.silent_residue_count,
        t_bi_b1_proved: package.t_bi_b1_proved,
        t_bi_b2_proved: package.t_bi_b2_proved,
        t_bi_b3_proved,
        package_proved: package.proved,
        every_role_declaration_resolved: package.every_role_declaration_resolved,
        every_marginal_family_credited_or_theorem_impossible: package
            .every_marginal_family_credited_or_theorem_impossible,
        local_anchor_nonreuse_holds: package.local_anchor_nonreuse_holds,
        no_historical_registry_consulted,
        no_archive_structural_bar_verdict_or_future_input,
        sequence_no_historical_registry_or_future_input: sequence
            .no_historical_registry_or_future_input,
        b3_no_historical_registry_or_legacy_v5_authority,
        b3_no_archive_structural_bar_verdict_or_future_input,
        exact_prefix_candidate_package_binding,
        all_residual_counts_zero,
        r2_premise,
        no_history_or_forbidden_input,
        proved,
        derivation_hash: String::new(),
    };
    token.derivation_hash = token_hash(&token);
    Ok(token)
}

/// Deterministic replay from the exact predecessor entries and candidate.
/// Rehashing a mutated token cannot make it authoritative because the full
/// source-first evidence is reissued before equality is checked.
pub fn replay_branch_semantic_provenance_v3(
    predecessor_entries: &[(u32, Telescope)],
    stage: u32,
    candidate: &Telescope,
    claimed: &BranchSemanticProvenanceV3Token,
) -> Vec<String> {
    let mut errors = validate_branch_semantic_provenance_v3_token_integrity(claimed);
    match issue_branch_semantic_provenance_v3(predecessor_entries, stage, candidate) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push(
            "branch semantic v3 token differs from deterministic source-first reissuance"
                .to_owned(),
        ),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

/// Cheap self-integrity check for an embedded token.  This does not replace
/// exact-entry replay and cannot establish that a caller supplied the right
/// telescope.  It lets an outer continuation reject a malformed projection,
/// broken digest, mismatched v5/B3 seal, or impossible flag/count combination
/// before paying for deterministic source-first reissuance.
pub fn validate_branch_semantic_provenance_v3_token_integrity(
    claimed: &BranchSemanticProvenanceV3Token,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != token_hash(claimed) {
        errors.push("branch semantic v3 token digest mismatch".to_owned());
    }
    if claimed.r2_premise.derivation_hash != r2_premise_hash(&claimed.r2_premise) {
        errors.push("branch semantic v3 R2 premise digest mismatch".to_owned());
    }
    if claimed.schema != BRANCH_SEMANTIC_PROVENANCE_V3_SCHEMA
        || claimed.date != BRANCH_SEMANTIC_PROVENANCE_V3_DATE
        || claimed.theorem_id != BRANCH_SEMANTIC_PROVENANCE_V3_THEOREM_ID
        || claimed.b3_theorem_id != T_BI_B3_V3_THEOREM_ID
    {
        errors.push("branch semantic v3 schema/date/theorem mismatch".to_owned());
    }
    if claimed.stage == 0
        || claimed.predecessor_entry_count + 1 != claimed.stage as usize
        || claimed.predecessor_stages != (1..claimed.stage).collect::<Vec<_>>()
        || claimed.predecessor_candidate_hashes.len() != claimed.predecessor_entry_count
    {
        errors.push("branch semantic v3 embedded predecessor shape is not contiguous".to_owned());
    }
    if claimed.candidate_hash.is_empty()
        || claimed.predecessor_signature_digest.is_empty()
        || claimed.package_derivation_hash.is_empty()
        || claimed.package_v3_declaration_hash.is_empty()
        || claimed.package_v4_candidate_local_source_hash.is_empty()
        || claimed.sequence_derivation_hash.is_empty()
        || claimed.sequence_authoritative_seal.is_empty()
        || claimed.sequence_issuance_trace_root.is_empty()
        || claimed.b3_token_derivation_hash.is_empty()
        || claimed.b3_authoritative_prefix_semantic_seal.is_empty()
        || claimed.b3_issuance_trace_root.is_empty()
    {
        errors.push("branch semantic v3 embedded binding hash is empty".to_owned());
    }
    let credited_ids = claimed
        .credited_family_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let credited_row_ids = claimed
        .credited_family_rows
        .iter()
        .map(|row| row.family_id.as_str())
        .collect::<BTreeSet<_>>();
    let exact_credit_projection = claimed.exact_credited_family_row_join
        && claimed.credited_family_ids.len() == credited_ids.len()
        && claimed.credited_family_rows.len() == credited_row_ids.len()
        && credited_ids == credited_row_ids
        && claimed
            .credited_family_ids
            .iter()
            .zip(&claimed.credited_family_rows)
            .all(|(family_id, row)| {
                family_id == &row.family_id
                    && !row.authoritative_family_row_derivation_hash.is_empty()
            })
        && u32::try_from(claimed.credited_family_rows.len()).ok() == Some(claimed.semantic_nu);
    if !exact_credit_projection {
        errors.push("branch semantic v3 embedded credited-family row join is not exact".to_owned());
    }
    let residuals_zero = claimed.named_role_residual_count == 0
        && claimed.named_quotient_residual_count == 0
        && claimed.named_a3_residual_count == 0
        && claimed.silent_residue_count == 0;
    if claimed.all_residual_counts_zero != residuals_zero {
        errors.push("branch semantic v3 residual summary disagrees with its counts".to_owned());
    }
    if claimed.sequence_issuance_trace_root != claimed.b3_issuance_trace_root {
        errors.push("branch semantic v3 v5 issuance trace does not join B3".to_owned());
    }
    let r2 = &claimed.r2_premise;
    let removed_not_exported = r2.r2_removed_occurrences_not_exported_as_families;
    let hash_surface_exact = r2.r2_generated_instance_removed_count
        == r2.r2_removed_occurrence_hashes.len()
        && if r2.r2_removed_occurrence_hashes.is_empty() {
            r2.r2_step8_typed_signature_derivation_hash.is_none()
                && r2.r2_m1_generated_membership_derivation_hash.is_none()
        } else {
            r2.r2_step8_typed_signature_derivation_hash.is_some()
                && r2.r2_m1_generated_membership_derivation_hash.is_some()
        };
    let trivial_r2 = r2.r2_generated_instance_removed_count == 0
        && r2.r2_removed_occurrence_hashes.is_empty()
        && r2.r2_step8_typed_signature_derivation_hash.is_none()
        && r2.r2_m1_generated_membership_derivation_hash.is_none()
        && r2.r2_parent_slot_label_join_holds
        && r2.r2_exact_v3_occurrence_surface_holds
        && r2.r2_m1_generated_membership_replayed
        && r2.r2_parent_membership_replayed
        && r2.r2_removed_occurrences_absent_from_unified_membership
        && removed_not_exported
        && r2.r2_generated_instance_not_multiplied
        && r2.r2_local_premises_replayed;
    if r2.stage != claimed.stage
        || r2.v3_declaration_surface_hash != claimed.package_v3_declaration_hash
        || r2.v4_prefix_local_source_hash != claimed.package_v4_candidate_local_source_hash
        || !r2.v4_source_matches_isolated_package
        || !r2.v4_source_surface_replayed
        || !r2.r2_parent_slot_label_join_holds
        || !r2.r2_exact_v3_occurrence_surface_holds
        || !r2.r2_m1_generated_membership_replayed
        || !r2.r2_parent_membership_replayed
        || !r2.r2_removed_occurrences_absent_from_unified_membership
        || !removed_not_exported
        || !r2.r2_generated_instance_not_multiplied
        || !r2.r2_local_premises_replayed
        || !hash_surface_exact
        || r2.non_stage8_projection_is_exact_zero_none_trivial != (claimed.stage == 8 || trivial_r2)
        || !r2.v4_forbidden_inputs_absent
        || !r2.proved
    {
        errors.push("branch semantic v3 embedded R2 premise projection is inconsistent".to_owned());
    }
    let no_history = claimed.no_historical_registry_consulted
        && claimed.no_archive_structural_bar_verdict_or_future_input
        && claimed.sequence_no_historical_registry_or_future_input
        && claimed.b3_no_historical_registry_or_legacy_v5_authority
        && claimed.b3_no_archive_structural_bar_verdict_or_future_input
        && claimed.r2_premise.v4_forbidden_inputs_absent;
    if claimed.no_history_or_forbidden_input != no_history {
        errors.push("branch semantic v3 no-history summary disagrees with its premises".to_owned());
    }
    if !claimed.proved
        || !claimed.package_proved
        || !claimed.t_bi_b1_proved
        || !claimed.t_bi_b2_proved
        || !claimed.t_bi_b3_proved
        || !claimed.every_role_declaration_resolved
        || !claimed.every_marginal_family_credited_or_theorem_impossible
        || !claimed.local_anchor_nonreuse_holds
        || !claimed.exact_prefix_candidate_package_binding
        || !claimed.all_residual_counts_zero
        || !claimed.r2_premise.proved
        || !claimed.no_history_or_forbidden_input
    {
        errors.push("branch semantic v3 embedded closing flags do not prove the token".to_owned());
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_predecessors(stage: u32) -> Vec<(u32, Telescope)> {
        (1..stage)
            .map(|entry_stage| (entry_stage, Telescope::reference(entry_stage)))
            .collect()
    }

    #[test]
    fn exact_stage4_reference_candidate_issues_and_replays() {
        let predecessors = reference_predecessors(4);
        let candidate = Telescope::reference(4);
        let token = issue_branch_semantic_provenance_v3(&predecessors, 4, &candidate)
            .expect("Stage-4 branch semantic evidence");
        assert!(token.proved);
        assert!(token.t_bi_b1_proved && token.t_bi_b2_proved && token.t_bi_b3_proved);
        assert!(token.no_history_or_forbidden_input);
        assert_eq!(token.semantic_nu as usize, token.credited_family_rows.len());
        assert!(
            token
                .r2_premise
                .non_stage8_projection_is_exact_zero_none_trivial
        );
        assert_eq!(token.r2_premise.r2_generated_instance_removed_count, 0);
        assert!(token.r2_premise.r2_removed_occurrence_hashes.is_empty());
        assert!(
            token
                .r2_premise
                .r2_step8_typed_signature_derivation_hash
                .is_none()
        );
        assert!(
            replay_branch_semantic_provenance_v3(&predecessors, 4, &candidate, &token).is_empty()
        );
    }

    #[test]
    fn prefix_candidate_and_rehashed_token_mutations_are_rejected() {
        let predecessors = reference_predecessors(4);
        let candidate = Telescope::reference(4);
        let token = issue_branch_semantic_provenance_v3(&predecessors, 4, &candidate)
            .expect("Stage-4 branch semantic evidence");

        let mut changed_prefix = predecessors.clone();
        changed_prefix[0].1 = Telescope::reference(2);
        assert!(
            !replay_branch_semantic_provenance_v3(&changed_prefix, 4, &candidate, &token)
                .is_empty()
        );

        let changed_candidate = Telescope::reference(3);
        assert!(
            !replay_branch_semantic_provenance_v3(&predecessors, 4, &changed_candidate, &token)
                .is_empty()
        );

        let mut rehashed = token.clone();
        rehashed.semantic_nu = rehashed.semantic_nu.saturating_add(1);
        rehashed.derivation_hash = token_hash(&rehashed);
        assert!(!validate_branch_semantic_provenance_v3_token_integrity(&rehashed).is_empty());
        assert!(
            !replay_branch_semantic_provenance_v3(&predecessors, 4, &candidate, &rehashed)
                .is_empty()
        );
    }

    #[test]
    fn stage8_binds_the_typed_r2_and_m1_premises() {
        let predecessors = reference_predecessors(8);
        let candidate = Telescope::reference(8);
        let token = issue_branch_semantic_provenance_v3(&predecessors, 8, &candidate)
            .expect("Stage-8 branch semantic evidence");
        let r2 = &token.r2_premise;
        assert!(token.proved && r2.proved);
        assert_eq!(r2.r2_generated_instance_removed_count, 1);
        assert_eq!(r2.r2_removed_occurrence_hashes.len(), 1);
        assert!(r2.r2_parent_slot_label_join_holds);
        assert!(r2.r2_exact_v3_occurrence_surface_holds);
        assert!(r2.r2_m1_generated_membership_replayed);
        assert!(r2.r2_parent_membership_replayed);
        assert!(r2.r2_removed_occurrences_absent_from_unified_membership);
        assert!(r2.r2_removed_occurrences_not_exported_as_families);
        assert!(r2.r2_generated_instance_not_multiplied);
        assert!(r2.r2_step8_typed_signature_derivation_hash.is_some());
        assert!(r2.r2_m1_generated_membership_derivation_hash.is_some());
        assert!(validate_branch_semantic_provenance_v3_token_integrity(&token).is_empty());
    }

    /// This expensive integration check exercises the same API over every
    /// live strict Stage-4 geometry root, without importing an enacted root or
    /// a semantic/structural selection result.
    #[test]
    #[ignore = "full four-root source-first semantic replay is intentionally expensive"]
    fn every_live_stage4_strict_root_is_an_arbitrary_supported_candidate() {
        use crate::stage4_semantic_parsimony_v1::{
            issue_stage4_preseal_opening_token_v1, issue_stage4_strict_cone_geometry_v1,
        };

        let opening = issue_stage4_preseal_opening_token_v1().expect("typed Stage-4 opening");
        let geometry = issue_stage4_strict_cone_geometry_v1().expect("live strict cone");
        let predecessors = opening
            .common_prefix_entries
            .iter()
            .map(|entry| (entry.stage, entry.telescope.clone()))
            .collect::<Vec<_>>();
        assert_eq!(geometry.stage, 4);
        assert!(!geometry.roots.is_empty());
        for root in geometry.roots {
            let token =
                issue_branch_semantic_provenance_v3(&predecessors, geometry.stage, &root.telescope)
                    .expect("arbitrary Stage-4 root semantic evidence");
            assert_eq!(token.candidate_hash, root.candidate_hash);
            assert!(token.proved);
        }
    }
}
