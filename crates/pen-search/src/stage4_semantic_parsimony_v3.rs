//! Exact BI-0/preseal cross-binding successor for the Stage-4 audit.
//!
//! V2 is accepted only as a caller-supplied claim.  This module first fully
//! replays that claim, then proves that the exact typed Stage-1-through-3
//! payload exported by BI-0 is byte-for-byte the payload consumed by the V1
//! blind audit's preseal opening.  Only the resulting V3 certificate asserts
//! that the already-computed blind cone audit is authorized.  No API in this
//! module can issue BI-0, V1, or V2, and no branch is selected or executed.

use crate::stage4_semantic_parsimony_v1::{
    STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA, Stage4SemanticParsimonyOutcomeV1,
    Stage4SemanticRootAuditV1, Stage4SemanticSelectionRootV1,
};
use crate::stage4_semantic_parsimony_v2::{
    STAGE4_SEMANTIC_PARSIMONY_V2_SCHEMA, Stage4SemanticParsimonyV2Certificate,
    replay_stage4_semantic_parsimony_v2,
};
use pen_core::hash::blake3_hex;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const STAGE4_SEMANTIC_PARSIMONY_V3_SCHEMA: &str =
    "stage4-semantic-parsimony-bi0-preseal-cross-binding-v3";
pub const STAGE4_SEMANTIC_PARSIMONY_V3_DATE: &str = "2026-07-22";
pub const STAGE4_SEMANTIC_PARSIMONY_V3_THEOREM_ID: &str =
    "T-SP4-v3-exact-BI0-to-blind-preseal-prefix-cross-binding";
pub const STAGE4_SEMANTIC_PARSIMONY_V3_CERTIFICATE_NAME: &str = "stage4_semantic_parsimony_v3.json";
pub const STAGE4_SEMANTIC_PARSIMONY_V3_REPORT_NAME: &str = "STAGE4_SEMANTIC_PARSIMONY_V3_RESULT.md";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(STAGE4_SEMANTIC_PARSIMONY_V3_SCHEMA, domain, value))
        .expect("Stage-4 v3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4PrefixCrossBindingRowV3 {
    pub position: usize,
    pub stage: u32,
    pub external_bi0_payload_hash: String,
    pub external_telescope_hash: String,
    pub blind_preseal_telescope_hash: String,
    pub external_candidate_hash: String,
    pub blind_preseal_candidate_hash: String,
    pub external_predecessor_signature_digest: String,
    pub blind_preseal_predecessor_signature_digest: String,
    pub recomputed_external_predecessor_signature_digest: String,
    pub recomputed_blind_predecessor_signature_digest: String,
    pub same_stage: bool,
    pub same_telescope: bool,
    pub same_candidate_hash: bool,
    pub same_predecessor_signature_digest: bool,
    pub external_predecessor_signature_recomputed: bool,
    pub blind_predecessor_signature_recomputed: bool,
    pub row_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4RootPrefixExtensionBindingV3 {
    pub position: usize,
    pub candidate_hash: String,
    pub geometry_telescope_hash: String,
    pub semantic_root_geometry_telescope_hash: String,
    pub recomputed_extended_prefix_signature_digest: String,
    pub semantic_root_prefix_signature_digest: String,
    pub matching_geometry_root_count: usize,
    pub exact_candidate_join: bool,
    pub exact_geometry_telescope_hash: bool,
    pub exact_stage4_extension_signature: bool,
    pub exact_candidate_prefix_and_package_bindings: bool,
    pub semantic_root_audit_proved: bool,
    pub row_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SelectionRootJoinV3 {
    pub position: usize,
    pub candidate_hash: String,
    pub matching_semantic_root_count: usize,
    pub semantic_root_prefix_signature_digest: String,
    pub selection_root_prefix_signature_digest: String,
    pub exact_semantic_root_projection: bool,
    pub proved_root_extension_joined: bool,
    pub row_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4PrefixCrossBindingProofV3 {
    pub source_v2_schema: String,
    pub source_v2_result_digest: String,
    pub external_bi0_opening_derivation_hash: String,
    pub blind_preseal_result_digest: String,
    pub blind_preseal_opening_derivation_hash: String,
    pub rows: Vec<Stage4PrefixCrossBindingRowV3>,
    pub exact_three_rows: bool,
    pub external_steps: Vec<u32>,
    pub blind_opening_steps: Vec<u32>,
    pub preseal_common_prefix_steps: Vec<u32>,
    pub external_candidate_hashes: Vec<String>,
    pub blind_opening_candidate_hashes: Vec<String>,
    pub preseal_common_prefix_candidate_hashes: Vec<String>,
    pub computed_external_full_prefix_signature_digest: String,
    pub computed_blind_full_prefix_signature_digest: String,
    pub blind_opening_full_prefix_signature_digest: String,
    pub preseal_common_prefix_signature_digest: String,
    pub cone_geometry_common_prefix_signature_digest: String,
    pub root_prefix_extensions: Vec<Stage4RootPrefixExtensionBindingV3>,
    pub selection_root_joins: Vec<Stage4SelectionRootJoinV3>,
    pub exact_entrywise_cross_binding: bool,
    pub opening_steps_copy_exact: bool,
    pub opening_candidate_hashes_copy_exact: bool,
    pub opening_signature_digest_exact: bool,
    pub preseal_steps_copy_exact: bool,
    pub preseal_candidate_hashes_copy_exact: bool,
    pub preseal_signature_digest_exact: bool,
    pub cone_geometry_signature_digest_exact: bool,
    pub every_root_extends_cross_bound_prefix: bool,
    pub every_selection_root_joins_proved_root_extension: bool,
    pub exact_bi0_to_blind_preseal_cross_binding_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticParsimonyV3Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub replayed_v2_claim: Stage4SemanticParsimonyV2Certificate,
    pub v2_claim_fully_replayed_before_cross_binding: bool,
    pub v2_result_digest: String,
    pub cross_binding: Stage4PrefixCrossBindingProofV3,
    pub v2_cone_authorization_field_used_as_authority: bool,
    pub blind_audit_completed_before_v3_authorization: bool,
    pub authorized_stage4_cone_audit: bool,
    pub authorized_bi1_branch_execution: bool,
    pub no_branch_selected_or_executed: bool,
    pub divergence_outcome_preserved: bool,
    pub divergence_requires_versioned_adjudication: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticParsimonyV3Replay {
    pub valid: bool,
    pub v2_claim_replayed: bool,
    pub exact_cross_binding_proved: bool,
    pub authorized_stage4_cone_audit: bool,
    pub branch_execution_authorized: bool,
    pub divergence_requires_adjudication: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Stage4SemanticParsimonyV3Error {
    #[error("Stage-4 v3 rejected the V2 claim: {0}")]
    V2Replay(String),
    #[error("Stage-4 v3 prefix cross-binding failed: {0}")]
    CrossBinding(String),
    #[error("Stage-4 v3 invariant failed: {0}")]
    Invariant(String),
    #[error("Stage-4 v3 JSON failed: {0}")]
    Json(String),
    #[error("Stage-4 v3 I/O failed: {0}")]
    Io(String),
    #[error("emitted Stage-4 v3 artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn row_hash(row: &Stage4PrefixCrossBindingRowV3) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("exact-prefix-cross-binding-row", &projection)
}

fn root_extension_hash(row: &Stage4RootPrefixExtensionBindingV3) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("root-extends-cross-bound-prefix", &projection)
}

fn selection_root_join_hash(row: &Stage4SelectionRootJoinV3) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("selection-root-joins-proved-root-extension", &projection)
}

fn v1_geometry_telescope_hash<T: Serialize + ?Sized>(telescope: &T) -> String {
    let bytes = serde_json::to_vec(&(
        STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA,
        "certified-root-geometry",
        telescope,
    ))
    .expect("Stage-4 v1 geometry telescope serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn selection_is_exact_projection(
    selection: &Stage4SemanticSelectionRootV1,
    root: &Stage4SemanticRootAuditV1,
) -> bool {
    selection.candidate_hash == root.candidate_hash
        && selection.geometry_telescope_hash == root.geometry_telescope_hash
        && selection.kappa == root.kappa
        && selection.prefix_signature_digest == root.prefix_signature_digest
        && selection.authoritative_prefix_semantic_v3_seal
            == root.authoritative_prefix_semantic_v3_seal
        && selection.prefix_semantic_nu_vector == root.prefix_semantic_nu_vector
        && selection.stage4_semantic_nu == root.stage4_semantic_nu
        && selection.registry_extension_invariance_proved
            == root.b3_v3_registry_extension_invariance_proved
        && selection.prefix_generic_transitive_isolation_proved
            == root.b3_v3_prefix_generic_isolation_proved
        && selection.no_forbidden_or_future_semantic_input
            == root.b3_v3_no_forbidden_or_future_semantic_input
}

fn cross_binding_hash(proof: &Stage4PrefixCrossBindingProofV3) -> String {
    let mut projection = proof.clone();
    projection.derivation_hash.clear();
    tagged_hash("exact-BI0-to-blind-preseal-cross-binding", &projection)
}

fn certificate_hash(certificate: &Stage4SemanticParsimonyV3Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("authorized-cross-bound-stage4-audit", &projection)
}

fn derive_prefix_cross_binding(
    v2: &Stage4SemanticParsimonyV2Certificate,
) -> Result<Stage4PrefixCrossBindingProofV3, Stage4SemanticParsimonyV3Error> {
    let external = v2
        .external_bi0_opening_capability
        .exact_stage1_through3_entries();
    let blind = &v2.blind_audit.preseal.opening_token.common_prefix_entries;
    if external.len() != 3 || blind.len() != 3 {
        return Err(Stage4SemanticParsimonyV3Error::CrossBinding(format!(
            "expected exactly three external and blind prefix entries; observed {}/{}",
            external.len(),
            blind.len()
        )));
    }

    let external_telescopes = external
        .iter()
        .map(|entry| (entry.stage(), entry.telescope().clone()))
        .collect::<Vec<_>>();
    let blind_telescopes = blind
        .iter()
        .map(|entry| (entry.stage, entry.telescope.clone()))
        .collect::<Vec<_>>();
    let computed_external_full_prefix_signature_digest =
        SealedSignature::from_telescopes(external_telescopes.clone())
            .digest()
            .to_owned();
    let computed_blind_full_prefix_signature_digest =
        SealedSignature::from_telescopes(blind_telescopes.clone())
            .digest()
            .to_owned();

    let mut rows = Vec::with_capacity(3);
    for index in 0..3 {
        let external_entry = &external[index];
        let blind_entry = &blind[index];
        let recomputed_external_predecessor_signature_digest =
            SealedSignature::from_telescopes(external_telescopes[..index].to_vec())
                .digest()
                .to_owned();
        let recomputed_blind_predecessor_signature_digest =
            SealedSignature::from_telescopes(blind_telescopes[..index].to_vec())
                .digest()
                .to_owned();
        let external_telescope_hash = candidate_hash(external_entry.telescope());
        let blind_preseal_telescope_hash = candidate_hash(&blind_entry.telescope);
        let same_stage = external_entry.stage() == blind_entry.stage;
        let same_telescope = external_entry.telescope() == &blind_entry.telescope;
        let same_candidate_hash = external_entry.candidate_hash() == blind_entry.candidate_hash;
        let same_predecessor_signature_digest = external_entry.predecessor_signature_digest()
            == blind_entry.predecessor_signature_digest;
        let external_predecessor_signature_recomputed = external_entry
            .predecessor_signature_digest()
            == recomputed_external_predecessor_signature_digest;
        let blind_predecessor_signature_recomputed = blind_entry.predecessor_signature_digest
            == recomputed_blind_predecessor_signature_digest;
        let row_proved = same_stage
            && same_telescope
            && same_candidate_hash
            && same_predecessor_signature_digest
            && external_entry.candidate_hash() == external_telescope_hash
            && blind_entry.candidate_hash == blind_preseal_telescope_hash
            && external_predecessor_signature_recomputed
            && blind_predecessor_signature_recomputed;
        let mut row = Stage4PrefixCrossBindingRowV3 {
            position: index,
            stage: external_entry.stage(),
            external_bi0_payload_hash: external_entry.payload_hash().to_owned(),
            external_telescope_hash,
            blind_preseal_telescope_hash,
            external_candidate_hash: external_entry.candidate_hash().to_owned(),
            blind_preseal_candidate_hash: blind_entry.candidate_hash.clone(),
            external_predecessor_signature_digest: external_entry
                .predecessor_signature_digest()
                .to_owned(),
            blind_preseal_predecessor_signature_digest: blind_entry
                .predecessor_signature_digest
                .clone(),
            recomputed_external_predecessor_signature_digest,
            recomputed_blind_predecessor_signature_digest,
            same_stage,
            same_telescope,
            same_candidate_hash,
            same_predecessor_signature_digest,
            external_predecessor_signature_recomputed,
            blind_predecessor_signature_recomputed,
            row_proved,
            derivation_hash: String::new(),
        };
        row.derivation_hash = row_hash(&row);
        rows.push(row);
    }

    let external_steps = external
        .iter()
        .map(|entry| entry.stage())
        .collect::<Vec<_>>();
    let blind_opening_steps = blind.iter().map(|entry| entry.stage).collect::<Vec<_>>();
    let preseal_common_prefix_steps = v2.blind_audit.preseal.common_prefix_steps.clone();
    let external_candidate_hashes = external
        .iter()
        .map(|entry| entry.candidate_hash().to_owned())
        .collect::<Vec<_>>();
    let blind_opening_candidate_hashes = blind
        .iter()
        .map(|entry| entry.candidate_hash.clone())
        .collect::<Vec<_>>();
    let preseal_common_prefix_candidate_hashes = v2
        .blind_audit
        .preseal
        .common_prefix_candidate_hashes
        .clone();
    let blind_opening_full_prefix_signature_digest = v2
        .blind_audit
        .preseal
        .opening_token
        .common_prefix_signature_digest
        .clone();
    let preseal_common_prefix_signature_digest = v2
        .blind_audit
        .preseal
        .common_prefix_signature_digest
        .clone();
    let cone_geometry_common_prefix_signature_digest = v2
        .blind_audit
        .preseal
        .live_strict_cone_geometry
        .common_prefix_signature_digest
        .clone();
    let exact_three_rows = rows.len() == 3
        && rows.iter().map(|row| row.stage).eq(1..=3)
        && rows
            .iter()
            .all(|row| row.row_proved && row.derivation_hash == row_hash(row));
    let exact_entrywise_cross_binding = exact_three_rows;
    let opening_steps_copy_exact = external_steps == blind_opening_steps
        && external_steps == v2.blind_audit.preseal.opening_token.common_prefix_steps;
    let opening_candidate_hashes_copy_exact = external_candidate_hashes
        == blind_opening_candidate_hashes
        && external_candidate_hashes
            == v2
                .blind_audit
                .preseal
                .opening_token
                .common_prefix_candidate_hashes;
    let opening_signature_digest_exact = computed_external_full_prefix_signature_digest
        == computed_blind_full_prefix_signature_digest
        && computed_external_full_prefix_signature_digest
            == blind_opening_full_prefix_signature_digest;
    let preseal_steps_copy_exact = external_steps == preseal_common_prefix_steps;
    let preseal_candidate_hashes_copy_exact =
        external_candidate_hashes == preseal_common_prefix_candidate_hashes;
    let preseal_signature_digest_exact =
        computed_external_full_prefix_signature_digest == preseal_common_prefix_signature_digest;
    let cone_geometry_signature_digest_exact = computed_external_full_prefix_signature_digest
        == cone_geometry_common_prefix_signature_digest;
    let root_prefix_extensions = v2
        .blind_audit
        .preseal
        .roots
        .iter()
        .enumerate()
        .map(|(position, root)| {
            let matching_geometry = v2
                .blind_audit
                .preseal
                .live_strict_cone_geometry
                .roots
                .iter()
                .filter(|geometry| {
                    geometry.candidate_hash == root.candidate_hash
                        && candidate_hash(&geometry.telescope) == root.candidate_hash
                })
                .collect::<Vec<_>>();
            let matching_geometry_root_count = matching_geometry.len();
            let (geometry_telescope_hash, recomputed_extended_prefix_signature_digest) =
                matching_geometry.first().map_or_else(
                    || (String::new(), String::new()),
                    |geometry| {
                        let mut extended = external_telescopes.clone();
                        extended.push((4, geometry.telescope.clone()));
                        (
                            v1_geometry_telescope_hash(&geometry.telescope),
                            SealedSignature::from_telescopes(extended)
                                .digest()
                                .to_owned(),
                        )
                    },
                );
            let exact_candidate_join = matching_geometry_root_count == 1
                && v2.blind_audit.preseal.live_strict_cone_geometry.stage == 4;
            let exact_geometry_telescope_hash =
                geometry_telescope_hash == root.geometry_telescope_hash;
            let exact_stage4_extension_signature =
                recomputed_extended_prefix_signature_digest == root.prefix_signature_digest;
            let row_proved = exact_candidate_join
                && exact_geometry_telescope_hash
                && exact_stage4_extension_signature
                && root.exact_candidate_prefix_and_package_bindings
                && root.root_semantic_audit_proved;
            let mut row = Stage4RootPrefixExtensionBindingV3 {
                position,
                candidate_hash: root.candidate_hash.clone(),
                geometry_telescope_hash,
                semantic_root_geometry_telescope_hash: root.geometry_telescope_hash.clone(),
                recomputed_extended_prefix_signature_digest,
                semantic_root_prefix_signature_digest: root.prefix_signature_digest.clone(),
                matching_geometry_root_count,
                exact_candidate_join,
                exact_geometry_telescope_hash,
                exact_stage4_extension_signature,
                exact_candidate_prefix_and_package_bindings: root
                    .exact_candidate_prefix_and_package_bindings,
                semantic_root_audit_proved: root.root_semantic_audit_proved,
                row_proved,
                derivation_hash: String::new(),
            };
            row.derivation_hash = root_extension_hash(&row);
            row
        })
        .collect::<Vec<_>>();
    let every_root_extends_cross_bound_prefix = !root_prefix_extensions.is_empty()
        && root_prefix_extensions.len()
            == v2.blind_audit.preseal.live_strict_cone_geometry.roots.len()
        && root_prefix_extensions
            .iter()
            .all(|row| row.row_proved && row.derivation_hash == root_extension_hash(row));
    let selection_root_joins = v2
        .blind_audit
        .preseal
        .semantic_selection_roots
        .iter()
        .enumerate()
        .map(|(position, selection)| {
            let matching_roots = v2
                .blind_audit
                .preseal
                .roots
                .iter()
                .filter(|root| root.candidate_hash == selection.candidate_hash)
                .collect::<Vec<_>>();
            let matching_semantic_root_count = matching_roots.len();
            let semantic_root_prefix_signature_digest = matching_roots
                .first()
                .map_or_else(String::new, |root| root.prefix_signature_digest.clone());
            let exact_semantic_root_projection = matching_roots
                .first()
                .is_some_and(|root| selection_is_exact_projection(selection, root));
            let proved_root_extension_joined = root_prefix_extensions.iter().any(|extension| {
                extension.candidate_hash == selection.candidate_hash
                    && extension.semantic_root_prefix_signature_digest
                        == selection.prefix_signature_digest
                    && extension.row_proved
            });
            let row_proved = matching_semantic_root_count == 1
                && exact_semantic_root_projection
                && proved_root_extension_joined;
            let mut row = Stage4SelectionRootJoinV3 {
                position,
                candidate_hash: selection.candidate_hash.clone(),
                matching_semantic_root_count,
                semantic_root_prefix_signature_digest,
                selection_root_prefix_signature_digest: selection.prefix_signature_digest.clone(),
                exact_semantic_root_projection,
                proved_root_extension_joined,
                row_proved,
                derivation_hash: String::new(),
            };
            row.derivation_hash = selection_root_join_hash(&row);
            row
        })
        .collect::<Vec<_>>();
    let every_selection_root_joins_proved_root_extension = !selection_root_joins.is_empty()
        && selection_root_joins.len() == root_prefix_extensions.len()
        && selection_root_joins
            .iter()
            .all(|row| row.row_proved && row.derivation_hash == selection_root_join_hash(row));
    let exact_bi0_to_blind_preseal_cross_binding_proved = exact_entrywise_cross_binding
        && opening_steps_copy_exact
        && opening_candidate_hashes_copy_exact
        && opening_signature_digest_exact
        && preseal_steps_copy_exact
        && preseal_candidate_hashes_copy_exact
        && preseal_signature_digest_exact
        && cone_geometry_signature_digest_exact
        && every_root_extends_cross_bound_prefix
        && every_selection_root_joins_proved_root_extension;
    let mut proof = Stage4PrefixCrossBindingProofV3 {
        source_v2_schema: v2.schema.clone(),
        source_v2_result_digest: v2.result_digest.clone(),
        external_bi0_opening_derivation_hash: v2
            .external_bi0_opening_capability
            .derivation_hash()
            .to_owned(),
        blind_preseal_result_digest: v2.blind_audit.preseal.result_digest.clone(),
        blind_preseal_opening_derivation_hash: v2
            .blind_audit
            .preseal
            .opening_token
            .derivation_hash
            .clone(),
        rows,
        exact_three_rows,
        external_steps,
        blind_opening_steps,
        preseal_common_prefix_steps,
        external_candidate_hashes,
        blind_opening_candidate_hashes,
        preseal_common_prefix_candidate_hashes,
        computed_external_full_prefix_signature_digest,
        computed_blind_full_prefix_signature_digest,
        blind_opening_full_prefix_signature_digest,
        preseal_common_prefix_signature_digest,
        cone_geometry_common_prefix_signature_digest,
        root_prefix_extensions,
        selection_root_joins,
        exact_entrywise_cross_binding,
        opening_steps_copy_exact,
        opening_candidate_hashes_copy_exact,
        opening_signature_digest_exact,
        preseal_steps_copy_exact,
        preseal_candidate_hashes_copy_exact,
        preseal_signature_digest_exact,
        cone_geometry_signature_digest_exact,
        every_root_extends_cross_bound_prefix,
        every_selection_root_joins_proved_root_extension,
        exact_bi0_to_blind_preseal_cross_binding_proved,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = cross_binding_hash(&proof);
    if !proof.exact_bi0_to_blind_preseal_cross_binding_proved {
        return Err(Stage4SemanticParsimonyV3Error::CrossBinding(format!(
            "the BI-0 typed prefix and blind preseal prefix are not exactly cross-bound: entrywise={}, opening-steps={}, opening-candidates={}, opening-signature={}, preseal-steps={}, preseal-candidates={}, preseal-signature={}, geometry-signature={}, roots-extend={}, selections-join={}",
            proof.exact_entrywise_cross_binding,
            proof.opening_steps_copy_exact,
            proof.opening_candidate_hashes_copy_exact,
            proof.opening_signature_digest_exact,
            proof.preseal_steps_copy_exact,
            proof.preseal_candidate_hashes_copy_exact,
            proof.preseal_signature_digest_exact,
            proof.cone_geometry_signature_digest_exact,
            proof.every_root_extends_cross_bound_prefix,
            proof.every_selection_root_joins_proved_root_extension,
        )));
    }
    Ok(proof)
}

pub fn issue_stage4_semantic_parsimony_v3(
    claimed_v2: &Stage4SemanticParsimonyV2Certificate,
) -> Result<Stage4SemanticParsimonyV3Certificate, Stage4SemanticParsimonyV3Error> {
    // This full replay is the first substantive operation.  V3 has no issuer
    // for BI-0 or V2 and cannot repair or replace the caller's claim.
    let v2_replay = replay_stage4_semantic_parsimony_v2(claimed_v2);
    if !v2_replay.valid {
        return Err(Stage4SemanticParsimonyV3Error::V2Replay(
            v2_replay.errors.join("; "),
        ));
    }
    issue_stage4_semantic_parsimony_v3_after_replayed_v2(claimed_v2)
}

fn issue_stage4_semantic_parsimony_v3_after_replayed_v2(
    claimed_v2: &Stage4SemanticParsimonyV2Certificate,
) -> Result<Stage4SemanticParsimonyV3Certificate, Stage4SemanticParsimonyV3Error> {
    let v2_claim_fully_replayed_before_cross_binding = true;
    let cross_binding = derive_prefix_cross_binding(claimed_v2)?;
    let blind_audit_completed_before_v3_authorization = claimed_v2.gate_binding.proved
        && claimed_v2.gate_sealed_before_blind_audit
        && claimed_v2.blind_audit_issued_after_gate;
    let no_branch_selected_or_executed = claimed_v2.no_branch_selected_or_executed
        && claimed_v2.blind_audit.no_branch_executed
        && !claimed_v2.authorized_bi1_branch_execution;
    let blind_divergence = matches!(
        claimed_v2.blind_audit.outcome,
        Stage4SemanticParsimonyOutcomeV1::TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired
    ) && claimed_v2
        .blind_audit
        .exact_two_minimizer_enacted_nonminimal_divergence;
    let divergence_outcome_preserved =
        claimed_v2.divergence_requires_versioned_adjudication == blind_divergence;
    let divergence_requires_versioned_adjudication =
        claimed_v2.divergence_requires_versioned_adjudication;
    // V2's same-named field is intentionally not a premise.  V3 earns this
    // assertion from full V2 replay plus the new exact cross-binding.
    let v2_cone_authorization_field_used_as_authority = false;
    let authorized_stage4_cone_audit = v2_claim_fully_replayed_before_cross_binding
        && claimed_v2.schema == STAGE4_SEMANTIC_PARSIMONY_V2_SCHEMA
        && cross_binding.exact_bi0_to_blind_preseal_cross_binding_proved
        && blind_audit_completed_before_v3_authorization
        && no_branch_selected_or_executed
        && divergence_outcome_preserved
        && !v2_cone_authorization_field_used_as_authority;
    let authorized_bi1_branch_execution = false;
    if !authorized_stage4_cone_audit {
        return Err(Stage4SemanticParsimonyV3Error::Invariant(
            "full V2 replay and exact BI-0/preseal cross-binding did not authorize the cone audit"
                .to_owned(),
        ));
    }

    let mut certificate = Stage4SemanticParsimonyV3Certificate {
        schema: STAGE4_SEMANTIC_PARSIMONY_V3_SCHEMA.to_owned(),
        date: STAGE4_SEMANTIC_PARSIMONY_V3_DATE.to_owned(),
        theorem_id: STAGE4_SEMANTIC_PARSIMONY_V3_THEOREM_ID.to_owned(),
        replayed_v2_claim: claimed_v2.clone(),
        v2_claim_fully_replayed_before_cross_binding,
        v2_result_digest: claimed_v2.result_digest.clone(),
        cross_binding,
        v2_cone_authorization_field_used_as_authority,
        blind_audit_completed_before_v3_authorization,
        authorized_stage4_cone_audit,
        authorized_bi1_branch_execution,
        no_branch_selected_or_executed,
        divergence_outcome_preserved,
        divergence_requires_versioned_adjudication,
        outcome: if divergence_requires_versioned_adjudication {
            "STAGE4_V3_CROSS_BOUND_SEMANTIC_DIVERGENCE_ADJUDICATION_REQUIRED"
        } else {
            "STAGE4_V3_CROSS_BOUND_CONE_AUDIT_AUTHORIZED"
        }
        .to_owned(),
        permitted_conclusion: "The caller-supplied V2 claim replayed in full, and the exact typed Stage-1-through-3 payload exported by BI-0 is entrywise identical to the blind audit's preseal opening and every common-prefix digest copy. V3 therefore authorizes the completed Stage-4 cone audit. No branch is selected or executed."
            .to_owned(),
        required_successor_action: if divergence_requires_versioned_adjudication {
            "Publish and adjudicate the preserved two-minimizer semantic divergence before any BI-1 branch execution, UC-1 scoring, bridge, or final certificate."
        } else {
            "Continue only through the successor action named by the fully replayed blind audit."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

pub fn replay_stage4_semantic_parsimony_v3(
    claimed: &Stage4SemanticParsimonyV3Certificate,
) -> Stage4SemanticParsimonyV3Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("Stage-4 v3 certificate digest mismatch".to_owned());
    }
    let v2_replay = replay_stage4_semantic_parsimony_v2(&claimed.replayed_v2_claim);
    let v2_claim_replayed = v2_replay.valid;
    if !v2_claim_replayed {
        errors.push(format!(
            "Stage-4 v3 embedded V2 claim failed full replay: {}",
            v2_replay.errors.join("; ")
        ));
    }
    if v2_claim_replayed {
        match issue_stage4_semantic_parsimony_v3_after_replayed_v2(&claimed.replayed_v2_claim) {
            Ok(expected) if expected == *claimed => {}
            Ok(_) => errors.push(
                "Stage-4 v3 certificate differs from deterministic cross-binding reissuance"
                    .to_owned(),
            ),
            Err(error) => errors.push(error.to_string()),
        }
    }
    Stage4SemanticParsimonyV3Replay {
        valid: errors.is_empty(),
        v2_claim_replayed,
        exact_cross_binding_proved: claimed
            .cross_binding
            .exact_bi0_to_blind_preseal_cross_binding_proved,
        authorized_stage4_cone_audit: claimed.authorized_stage4_cone_audit,
        branch_execution_authorized: claimed.authorized_bi1_branch_execution,
        divergence_requires_adjudication: claimed.divergence_requires_versioned_adjudication,
        errors,
    }
}

pub fn replay_stage4_semantic_parsimony_v3_json(json: &str) -> Stage4SemanticParsimonyV3Replay {
    match serde_json::from_str::<Stage4SemanticParsimonyV3Certificate>(json) {
        Ok(certificate) => replay_stage4_semantic_parsimony_v3(&certificate),
        Err(error) => Stage4SemanticParsimonyV3Replay {
            valid: false,
            v2_claim_replayed: false,
            exact_cross_binding_proved: false,
            authorized_stage4_cone_audit: false,
            branch_execution_authorized: false,
            divergence_requires_adjudication: false,
            errors: vec![format!("Stage-4 v3 JSON did not deserialize: {error}")],
        },
    }
}

pub fn render_stage4_semantic_parsimony_v3_report(
    certificate: &Stage4SemanticParsimonyV3Certificate,
) -> String {
    let rows = certificate
        .cross_binding
        .rows
        .iter()
        .map(|row| {
            format!(
                "| {} | {} | `{}` | `{}` | `{}` | {} |",
                row.position,
                row.stage,
                row.external_candidate_hash,
                row.external_predecessor_signature_digest,
                row.external_bi0_payload_hash,
                row.row_proved,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# Stage-4 semantic parsimony v3 cross-binding result\n\n\
         **Date:** {}. **Outcome:** `{}`.\n\n\
         The caller-supplied V2 claim replayed before cross-binding: **{}**. Exact BI-0-to-blind-preseal prefix cross-binding: **{}**. V2's pre-cross-binding cone field used as authority: **{}**.\n\n\
         | Position | Stage | Candidate hash | Predecessor signature | BI-0 payload hash | Cross-bound |\n|---:|---:|---|---|---|---|\n{}\n\n\
         Computed full prefix signature: `{}`. Blind opening copy: `{}`. Preseal common-prefix copy: `{}`. Cone-geometry copy: `{}`.\n\n\
         Authorized Stage-4 cone audit: **{}**. Authorized branch execution: **{}**. No branch selected or executed: **{}**. Divergence outcome preserved: **{}**. Divergence requires adjudication: **{}**.\n\n\
         {}\n\nNext: {}\n\nV2 claim digest: `{}`. V3 certificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.v2_claim_fully_replayed_before_cross_binding,
        certificate
            .cross_binding
            .exact_bi0_to_blind_preseal_cross_binding_proved,
        certificate.v2_cone_authorization_field_used_as_authority,
        rows,
        certificate
            .cross_binding
            .computed_external_full_prefix_signature_digest,
        certificate
            .cross_binding
            .blind_opening_full_prefix_signature_digest,
        certificate
            .cross_binding
            .preseal_common_prefix_signature_digest,
        certificate
            .cross_binding
            .cone_geometry_common_prefix_signature_digest,
        certificate.authorized_stage4_cone_audit,
        certificate.authorized_bi1_branch_execution,
        certificate.no_branch_selected_or_executed,
        certificate.divergence_outcome_preserved,
        certificate.divergence_requires_versioned_adjudication,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.v2_result_digest,
        certificate.result_digest,
    )
}

pub fn emit_stage4_semantic_parsimony_v3_create_new(
    claimed_v2: &Stage4SemanticParsimonyV2Certificate,
    directory: &Path,
) -> Result<Stage4SemanticParsimonyV3Certificate, Stage4SemanticParsimonyV3Error> {
    let certificate = issue_stage4_semantic_parsimony_v3(claimed_v2)?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Stage4SemanticParsimonyV3Error::Json(error.to_string()))?;
    let replay = replay_stage4_semantic_parsimony_v3_json(&json);
    if !replay.valid {
        return Err(Stage4SemanticParsimonyV3Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let json_path = directory.join(STAGE4_SEMANTIC_PARSIMONY_V3_CERTIFICATE_NAME);
    let report_path = directory.join(STAGE4_SEMANTIC_PARSIMONY_V3_REPORT_NAME);
    if json_path.exists() || report_path.exists() {
        return Err(Stage4SemanticParsimonyV3Error::Io(format!(
            "create-new target already exists: {} or {}",
            json_path.display(),
            report_path.display()
        )));
    }
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&json_path)
        .map_err(|error| Stage4SemanticParsimonyV3Error::Io(error.to_string()))?;
    json_file
        .write_all(json.as_bytes())
        .map_err(|error| Stage4SemanticParsimonyV3Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| Stage4SemanticParsimonyV3Error::Io(error.to_string()))?;
    report_file
        .write_all(render_stage4_semantic_parsimony_v3_report(&certificate).as_bytes())
        .map_err(|error| Stage4SemanticParsimonyV3Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_stage4_semantic_parsimony_v3_directory(
    directory: &Path,
) -> Result<Stage4SemanticParsimonyV3Replay, Stage4SemanticParsimonyV3Error> {
    let json =
        std::fs::read_to_string(directory.join(STAGE4_SEMANTIC_PARSIMONY_V3_CERTIFICATE_NAME))
            .map_err(|error| Stage4SemanticParsimonyV3Error::Io(error.to_string()))?;
    Ok(replay_stage4_semantic_parsimony_v3_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;

    const V2_CLAIM: &str = include_str!("../../../docs/stage4_semantic_parsimony_v2.json");

    fn archived_v2_claim() -> Stage4SemanticParsimonyV2Certificate {
        serde_json::from_str(V2_CLAIM).expect("archived V2 claim JSON")
    }

    #[test]
    fn production_surface_can_only_replay_a_caller_supplied_v2_claim() {
        let source = include_str!("stage4_semantic_parsimony_v3.rs");
        let production = source.split("#[cfg(test)]").next().unwrap_or(source);
        assert!(production.contains("replay_stage4_semantic_parsimony_v2"));
        assert!(!production.contains("issue_bi0_stage4_opening_capability_v6"));
        assert!(!production.contains("issue_stage4_semantic_parsimony_v2("));
        assert!(!production.contains("issue_stage4_semantic_parsimony_v1("));
        assert!(!production.contains("Telescope::reference"));
        assert!(!production.contains("include_bytes!"));
        assert!(!production.contains("include_str!(\"../../../docs/"));
    }

    #[test]
    fn archived_v2_claim_has_exact_cross_binding_shape_without_reissuing_v2() {
        let claim = archived_v2_claim();
        let proof = derive_prefix_cross_binding(&claim).expect("exact cross-binding");
        assert_eq!(proof.rows.len(), 3);
        assert!(proof.rows.iter().all(|row| row.row_proved));
        assert!(proof.exact_bi0_to_blind_preseal_cross_binding_proved);
    }

    #[test]
    fn a_blind_preseal_telescope_mutation_breaks_cross_binding() {
        let mut claim = archived_v2_claim();
        claim
            .blind_audit
            .preseal
            .opening_token
            .common_prefix_entries[1]
            .telescope
            .clauses
            .clear();
        assert!(derive_prefix_cross_binding(&claim).is_err());
    }
}
