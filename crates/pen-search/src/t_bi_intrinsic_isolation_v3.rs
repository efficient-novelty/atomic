//! Prefix-generic typed isolation for act-local semantic provenance.
//!
//! V2 deliberately stated its theorem only on the exact fifteen-stage
//! historical surface.  Stage-4 branch comparison needs the same theorem on
//! four independently reissued Stage-1-through-4 prefixes.  This successor
//! changes only that surface quantifier: it accepts no receipts, archive
//! facts, verdicts, structural scores, bars, or future suffixes.

use crate::act_local_semantic_provenance_v5::{
    ActLocalSemanticSequenceV5, V5PrefixLocalRegistryErasureProof,
    prove_prefix_local_role_registry_erasure_v5, replay_act_local_semantic_sequence_v5,
    replay_prefix_local_role_registry_erasure_v5,
};
use crate::act_local_provenance_v3::issue_act_local_sequence_v3;
use crate::act_local_semantic_provenance_v4::issue_act_local_semantic_sequence_v4;
use crate::t_bi_intrinsic_isolation_v2::{
    IntrinsicCapabilityV2, TBiIntrinsicIsolationV2Token,
    issue_t_bi_intrinsic_isolation_prefix_core_v2,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const T_BI_INTRINSIC_ISOLATION_V3_SCHEMA: &str =
    "t-bi-intrinsic-prefix-generic-typed-isolation-v3";
pub const T_BI_INTRINSIC_ISOLATION_V3_DATE: &str = "2026-07-22";
pub const T_BI_B3_V3_THEOREM_ID: &str =
    "T-BI-B3-v3-prefix-generic-typed-evidence-bound-transitive-isolation";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_INTRINSIC_ISOLATION_V3_SCHEMA, domain, value))
        .expect("T-BI-B3 v3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiIntrinsicIsolationV3Token {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub prefix_len: usize,
    pub last_stage: u32,
    pub non_authoritative_legacy_sequence_schema: String,
    pub non_authoritative_legacy_sequence_derivation_hash: String,
    pub non_authoritative_legacy_package_derivation_hashes: Vec<String>,
    pub non_authoritative_legacy_intrinsic_sequence_seal: String,
    /// Non-authoritative reuse of v2's audited evidence vocabulary.  This is
    /// not an issuance of the exact-fifteen v2 theorem; v3 alone gives the
    /// prefix-generic theorem its authority.
    pub non_authoritative_v2_evidence_dag: TBiIntrinsicIsolationV2Token,
    pub embedded_v2_exact_fifteen_theorem_authority_claimed: bool,
    pub v3_is_sole_prefix_generic_theorem_authority: bool,
    pub prefix_local_registry_erasure_proofs: Vec<V5PrefixLocalRegistryErasureProof>,
    pub sorted_unique_prefix_observed_kinds: Vec<String>,
    pub authoritative_semantic_nu_vector: Vec<u32>,
    pub authoritative_registry_erased_prefix_semantic_seal: String,
    pub registry_extension_invariance_proved: bool,
    pub old_v4_v5_full_hashes_authoritative_for_prefix_theorem: bool,
    pub nonempty_contiguous_stage1_through_n: bool,
    pub exact_typed_phase_surface: bool,
    pub non_authoritative_legacy_package_commitments_recomputed: bool,
    pub non_authoritative_legacy_sequence_commitments_recomputed: bool,
    pub every_predecessor_hash_bound: bool,
    pub every_capability_derived_from_operation: bool,
    pub every_node_reaches_sequence_seal: bool,
    pub no_forbidden_capability_in_transitive_closure: bool,
    pub no_caller_receipt_parameter: bool,
    pub no_archive_structural_bar_verdict_or_future_input: bool,
    pub source_scan_used_as_proof: bool,
    pub runtime_self_report_used_as_proof: bool,
    pub synthetic_receipt_input_accepted: bool,
    pub prefix_generic_transitive_isolation_proved: bool,
    pub proof_scope: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiIntrinsicIsolationV3Error {
    #[error("T-BI-B3 v3 invalid input: {0}")]
    Input(String),
    #[error("T-BI-B3 v3 v5 binding failed: {0}")]
    Binding(String),
    #[error("T-BI-B3 v3 typed isolation failed: {0}")]
    Isolation(String),
}

fn token_hash(token: &TBiIntrinsicIsolationV3Token) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-generic-isolation-token", &projection)
}

fn exact_prefix_surface(
    entries: &[(u32, Telescope)],
    sequence: &ActLocalSemanticSequenceV5,
) -> bool {
    let Ok(prefix_len) = u32::try_from(entries.len()) else {
        return false;
    };
    !entries.is_empty()
        && entries.iter().map(|(stage, _)| *stage).eq(1..=prefix_len)
        && sequence.packages.len() == entries.len()
        && sequence
            .packages
            .iter()
            .map(|package| package.stage)
            .eq(1..=prefix_len)
}

fn forbidden_capabilities() -> BTreeSet<IntrinsicCapabilityV2> {
    use IntrinsicCapabilityV2 as C;
    [
        C::Archive,
        C::StructuralNu,
        C::Bar,
        C::Verdict,
        C::EnactedFuture,
    ]
    .into_iter()
    .collect()
}

/// Prove intrinsic isolation for an arbitrary nonempty contiguous
/// Stage-1-through-N semantic sequence.  The only inputs are the acts and the
/// v5 sequence claimed to be their deterministic reissuance.
pub fn issue_t_bi_intrinsic_isolation_v3(
    entries: &[(u32, Telescope)],
    sequence: &ActLocalSemanticSequenceV5,
) -> Result<TBiIntrinsicIsolationV3Token, TBiIntrinsicIsolationV3Error> {
    if !exact_prefix_surface(entries, sequence) {
        return Err(TBiIntrinsicIsolationV3Error::Input(
            "requires a nonempty contiguous Stage-1-through-N act/package surface".to_owned(),
        ));
    }
    let replay_errors = replay_act_local_semantic_sequence_v5(entries, sequence);
    if !replay_errors.is_empty() {
        return Err(TBiIntrinsicIsolationV3Error::Binding(
            replay_errors.join("; "),
        ));
    }
    let v3_packages = issue_act_local_sequence_v3(entries)
        .map_err(|error| TBiIntrinsicIsolationV3Error::Binding(error.to_string()))?;
    let v4_packages = issue_act_local_semantic_sequence_v4(entries)
        .map_err(|error| TBiIntrinsicIsolationV3Error::Binding(error.to_string()))?;
    if v3_packages.len() != entries.len() || v4_packages.len() != entries.len() {
        return Err(TBiIntrinsicIsolationV3Error::Binding(
            "prefix-local predecessor package vectors are not exact".to_owned(),
        ));
    }
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut prefix_local_registry_erasure_proofs = Vec::new();
    for ((((stage, candidate), package), v3), v4) in entries
        .iter()
        .zip(&sequence.packages)
        .zip(&v3_packages)
        .zip(&v4_packages)
    {
        if package.stage != *stage || v3.stage != *stage || v4.stage != *stage {
            return Err(TBiIntrinsicIsolationV3Error::Binding(format!(
                "Stage {stage} predecessor package join drifted"
            )));
        }
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let proof = prove_prefix_local_role_registry_erasure_v5(
            &prefix, candidate, v3, v4, package,
        )
        .map_err(|error| TBiIntrinsicIsolationV3Error::Isolation(error.to_string()))?;
        let erasure_errors = replay_prefix_local_role_registry_erasure_v5(
            &prefix, candidate, v3, v4, package, &proof,
        );
        if !erasure_errors.is_empty() {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(
                erasure_errors.join("; "),
            ));
        }
        prefix_local_registry_erasure_proofs.push(proof);
        accepted.push((*stage, candidate.clone()));
    }
    let sorted_unique_prefix_observed_kinds = prefix_local_registry_erasure_proofs
        .iter()
        .flat_map(|proof| proof.sorted_unique_observed_kinds.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let authoritative_semantic_nu_vector = prefix_local_registry_erasure_proofs
        .iter()
        .map(|proof| proof.recomputed_semantic_nu)
        .collect::<Vec<_>>();
    let authoritative_registry_erased_prefix_semantic_seal = tagged_hash(
        "registry-erased-prefix-semantic-seal",
        &(
            entries
                .iter()
                .map(|(stage, candidate)| (*stage, candidate_hash(candidate)))
                .collect::<Vec<_>>(),
            prefix_local_registry_erasure_proofs
                .iter()
                .map(|proof| {
                    (
                        proof.stage,
                        proof.candidate_hash.as_str(),
                        proof.predecessor_signature_digest.as_str(),
                        proof.derivation_hash.as_str(),
                    )
                })
                .collect::<Vec<_>>(),
            &authoritative_semantic_nu_vector,
        ),
    );
    let registry_extension_invariance_proved = prefix_local_registry_erasure_proofs
        .iter()
        .all(|proof| {
            proof.proved
                && proof.v3_occurrence_surface_replayed
                && proof.every_occurrence_has_one_direct_grammar_entry
                && proof.target_computation_registry_extension_invariant
                && proof.resolution_registry_extension_invariant
                && proof.prefix_local_finite_closure_proved
                && proof.prefix_local_b1_proved
                && proof.prefix_local_b2_proved
                && proof.every_marginal_family_locally_credited_or_theorem_impossible
                && proof.credited_family_projection_exact
                && proof.semantic_nu_registry_extension_invariant
                && proof.package_projection_exact
                && !proof.historical_registry_suffix_used_as_semantic_premise
                && !proof.old_v4_v5_full_hashes_authoritative_for_prefix_theorem
                && !proof.stage9_historical_boundary_hash_in_authoritative_projection
        });
    if !registry_extension_invariance_proved
        || authoritative_registry_erased_prefix_semantic_seal.is_empty()
    {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "prefix-local registry erasure did not close the semantic projection".to_owned(),
        ));
    }
    let typed_phase_dag = issue_t_bi_intrinsic_isolation_prefix_core_v2(entries, sequence)
        .map_err(|error| TBiIntrinsicIsolationV3Error::Isolation(error.to_string()))?;

    let prefix_len = entries.len();
    let last_stage = u32::try_from(prefix_len)
        .map_err(|_| TBiIntrinsicIsolationV3Error::Input("prefix length exceeds u32".to_owned()))?;
    let nonempty_contiguous_stage1_through_n = exact_prefix_surface(entries, sequence);
    let exact_typed_phase_surface = typed_phase_dag.phase_receipts.len() == prefix_len * 17 + 1
        && (1..=last_stage).all(|stage| {
            typed_phase_dag
                .phase_receipts
                .iter()
                .filter(|receipt| receipt.stage == Some(stage))
                .count()
                == 17
        });
    let forbidden = forbidden_capabilities();
    let no_archive_structural_bar_verdict_or_future_input =
        typed_phase_dag.closure_rows.iter().all(|row| {
            row.transitive_capabilities
                .iter()
                .all(|capability| !forbidden.contains(capability))
        }) && sequence.packages.iter().all(|package| {
            !package.archive_read
                && !package.structural_nu_read
                && !package.bar_read
                && !package.verdict_read
                && !package.enacted_future_read
        }) && registry_extension_invariance_proved
            && prefix_local_registry_erasure_proofs.iter().all(|proof| {
                !proof.historical_registry_suffix_used_as_semantic_premise
                    && !proof.old_v4_v5_full_hashes_authoritative_for_prefix_theorem
                    && !proof.stage9_historical_boundary_hash_in_authoritative_projection
            });
    let no_caller_receipt_parameter = true;
    let prefix_generic_transitive_isolation_proved = nonempty_contiguous_stage1_through_n
        && exact_typed_phase_surface
        && registry_extension_invariance_proved
        && authoritative_semantic_nu_vector.len() == prefix_len
        && !authoritative_registry_erased_prefix_semantic_seal.is_empty()
        && typed_phase_dag.every_predecessor_hash_bound
        && typed_phase_dag.every_capability_derived_from_operation
        && typed_phase_dag.every_node_reaches_sequence_seal
        && typed_phase_dag.no_forbidden_capability_in_transitive_closure
        && typed_phase_dag.named_gaps.is_empty()
        && no_caller_receipt_parameter
        && no_archive_structural_bar_verdict_or_future_input
        && !typed_phase_dag.source_scan_used_as_proof
        && !typed_phase_dag.runtime_self_report_used_as_proof
        && !typed_phase_dag.synthetic_receipt_input_accepted
        && typed_phase_dag.transitive_call_graph_isolation_proved;
    if !prefix_generic_transitive_isolation_proved {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "the reissued prefix did not close its typed evidence/capability DAG".to_owned(),
        ));
    }

    let mut token = TBiIntrinsicIsolationV3Token {
        schema: T_BI_INTRINSIC_ISOLATION_V3_SCHEMA.to_owned(),
        date: T_BI_INTRINSIC_ISOLATION_V3_DATE.to_owned(),
        theorem_id: T_BI_B3_V3_THEOREM_ID.to_owned(),
        prefix_len,
        last_stage,
        non_authoritative_legacy_sequence_schema: sequence.schema.clone(),
        non_authoritative_legacy_sequence_derivation_hash: sequence.derivation_hash.clone(),
        non_authoritative_legacy_package_derivation_hashes: sequence
            .exact_package_derivation_hashes
            .clone(),
        non_authoritative_legacy_intrinsic_sequence_seal: sequence
            .intrinsic_sequence_seal
            .clone(),
        prefix_local_registry_erasure_proofs,
        sorted_unique_prefix_observed_kinds,
        authoritative_semantic_nu_vector,
        authoritative_registry_erased_prefix_semantic_seal,
        registry_extension_invariance_proved,
        old_v4_v5_full_hashes_authoritative_for_prefix_theorem: false,
        non_authoritative_legacy_package_commitments_recomputed: typed_phase_dag
            .exact_package_commitments_recomputed,
        non_authoritative_legacy_sequence_commitments_recomputed: typed_phase_dag
            .exact_sequence_commitments_recomputed,
        every_predecessor_hash_bound: typed_phase_dag.every_predecessor_hash_bound,
        every_capability_derived_from_operation: typed_phase_dag
            .every_capability_derived_from_operation,
        every_node_reaches_sequence_seal: typed_phase_dag.every_node_reaches_sequence_seal,
        no_forbidden_capability_in_transitive_closure: typed_phase_dag
            .no_forbidden_capability_in_transitive_closure,
        source_scan_used_as_proof: typed_phase_dag.source_scan_used_as_proof,
        runtime_self_report_used_as_proof: typed_phase_dag.runtime_self_report_used_as_proof,
        synthetic_receipt_input_accepted: typed_phase_dag.synthetic_receipt_input_accepted,
        non_authoritative_v2_evidence_dag: typed_phase_dag,
        embedded_v2_exact_fifteen_theorem_authority_claimed: false,
        v3_is_sole_prefix_generic_theorem_authority: true,
        nonempty_contiguous_stage1_through_n,
        exact_typed_phase_surface,
        no_caller_receipt_parameter,
        no_archive_structural_bar_verdict_or_future_input,
        prefix_generic_transitive_isolation_proved,
        proof_scope: "For every supplied nonempty contiguous Stage-1-through-N prefix: rederive the v3 occurrence surface from the exact candidate and prefix; erase the historical role-registry index, size, route hash, and suffix; recompute the four-surface targets, resolution classes, local-role injection, credited-family set, and semantic nu relative to the replayed candidate-local v4 term, typing, ordinary, cubical, quotient, and A3 source fields; seal that registry-erased projection; and combine it with the non-role typed phase/capability structure. This theorem proves extension-invariance of the role grammar and its semantic projection, not byte-independence from every v4 field. Exact v4/v5 and v2 hashes remain replayed compatibility witnesses, not prefix-semantic authority. No caller receipt surface exists."
            .to_owned(),
        derivation_hash: String::new(),
    };
    token.derivation_hash = token_hash(&token);
    Ok(token)
}

pub fn replay_t_bi_intrinsic_isolation_v3(
    entries: &[(u32, Telescope)],
    sequence: &ActLocalSemanticSequenceV5,
    claimed: &TBiIntrinsicIsolationV3Token,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != token_hash(claimed) {
        errors.push("T-BI-B3 v3 token digest mismatch".to_owned());
    }
    match issue_t_bi_intrinsic_isolation_v3(entries, sequence) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => {
            errors.push("T-BI-B3 v3 token differs from deterministic prefix reissuance".to_owned())
        }
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::act_local_semantic_provenance_v5::issue_act_local_semantic_sequence_v5;
    use crate::t_bi_intrinsic_isolation_v2::issue_t_bi_intrinsic_isolation_v2;

    fn reference_prefix(n: u32) -> (Vec<(u32, Telescope)>, ActLocalSemanticSequenceV5) {
        let entries = (1..=n)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let sequence = issue_act_local_semantic_sequence_v5(&entries).expect("v5 prefix");
        (entries, sequence)
    }

    #[test]
    fn stage4_prefix_closes_without_future_capabilities() {
        let (entries, sequence) = reference_prefix(4);
        let token = issue_t_bi_intrinsic_isolation_v3(&entries, &sequence).expect("B3 v3");
        assert_eq!(token.prefix_len, 4);
        assert_eq!(
            token.non_authoritative_v2_evidence_dag.phase_receipts.len(),
            69
        );
        assert!(!token.embedded_v2_exact_fifteen_theorem_authority_claimed);
        assert!(token.v3_is_sole_prefix_generic_theorem_authority);
        assert_eq!(token.authoritative_semantic_nu_vector, vec![1, 0, 1, 3]);
        assert_eq!(
            token.sorted_unique_prefix_observed_kinds,
            vec![
                "former_adjoint".to_owned(),
                "former_eliminator".to_owned(),
                "former_introduction".to_owned(),
                "foundation_completion".to_owned(),
                "foundation_formation".to_owned(),
                "map_single_action".to_owned(),
                "map_single_head".to_owned(),
            ]
        );
        assert_eq!(token.prefix_local_registry_erasure_proofs.len(), 4);
        assert!(token.prefix_local_registry_erasure_proofs.iter().all(|proof| {
            proof.proved
                && proof.target_computation_registry_extension_invariant
                && proof.resolution_registry_extension_invariant
                && proof.semantic_nu_registry_extension_invariant
                && proof.every_marginal_family_locally_credited_or_theorem_impossible
                && !proof.historical_registry_suffix_used_as_semantic_premise
                && !proof.old_v4_v5_full_hashes_authoritative_for_prefix_theorem
        }));
        assert!(token.registry_extension_invariance_proved);
        assert!(!token.old_v4_v5_full_hashes_authoritative_for_prefix_theorem);
        assert!(token.prefix_generic_transitive_isolation_proved);
        assert!(token.no_archive_structural_bar_verdict_or_future_input);
        assert!(replay_t_bi_intrinsic_isolation_v3(&entries, &sequence, &token).is_empty());
    }

    #[test]
    fn exact_fifteen_v2_surface_still_issues_and_v3_generalizes_it() {
        let (entries, sequence) = reference_prefix(15);
        let v2 = issue_t_bi_intrinsic_isolation_v2(&entries, &sequence).expect("unchanged v2");
        let v3 = issue_t_bi_intrinsic_isolation_v3(&entries, &sequence).expect("v3 at N=15");
        assert!(v2.exact_fifteen_stage_surface);
        assert!(v2.transitive_call_graph_isolation_proved);
        assert!(
            v3.non_authoritative_v2_evidence_dag
                .exact_fifteen_stage_surface
        );
        assert!(!v3.embedded_v2_exact_fifteen_theorem_authority_claimed);
        assert!(v3.prefix_generic_transitive_isolation_proved);
    }

    #[test]
    fn empty_and_gapped_surfaces_are_rejected() {
        let empty = Vec::<(u32, Telescope)>::new();
        let (_, mut empty_sequence) = reference_prefix(1);
        empty_sequence.packages.clear();
        empty_sequence.exact_package_derivation_hashes.clear();
        assert!(issue_t_bi_intrinsic_isolation_v3(&empty, &empty_sequence).is_err());

        let (mut entries, _) = reference_prefix(4);
        entries[3].0 = 5;
        let sequence = issue_act_local_semantic_sequence_v5(&entries);
        assert!(
            sequence.is_err()
                || sequence.is_ok_and(|sequence| {
                    issue_t_bi_intrinsic_isolation_v3(&entries, &sequence).is_err()
                })
        );
    }

    #[test]
    fn fully_rehashed_claim_mutation_still_fails_reissuance() {
        let (entries, sequence) = reference_prefix(4);
        let token = issue_t_bi_intrinsic_isolation_v3(&entries, &sequence).expect("B3 v3");
        let mut forged = token.clone();
        forged.prefix_len = 3;
        forged.derivation_hash = token_hash(&forged);
        let errors = replay_t_bi_intrinsic_isolation_v3(&entries, &sequence, &forged);
        assert!(errors.iter().any(|error| error.contains("reissuance")));
    }

    #[test]
    fn fully_rehashed_inner_erasure_mutation_still_fails_reissuance() {
        let (entries, sequence) = reference_prefix(4);
        let token = issue_t_bi_intrinsic_isolation_v3(&entries, &sequence).expect("B3 v3");
        let mut forged = token.clone();
        let proof = forged
            .prefix_local_registry_erasure_proofs
            .last_mut()
            .expect("Stage-4 erasure proof");
        let row = proof.rows.first_mut().expect("Stage-4 erasure row");
        row.unused_registry_extension_projection_equal = false;
        row.derivation_hash = super::super::act_local_semantic_provenance_v5::test_only_prefix_local_erasure_row_hash(row);
        proof.derivation_hash = super::super::act_local_semantic_provenance_v5::test_only_prefix_local_erasure_proof_hash(proof);
        forged.authoritative_semantic_nu_vector[3] = forged.authoritative_semantic_nu_vector[3].saturating_add(1);
        forged.derivation_hash = token_hash(&forged);
        let errors = replay_t_bi_intrinsic_isolation_v3(&entries, &sequence, &forged);
        assert!(errors.iter().any(|error| error.contains("reissuance")));
    }
}
