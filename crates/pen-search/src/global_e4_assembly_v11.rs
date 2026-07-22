//! Proof-strength Global E-4 successor over the exact v9 wrapped surface.
//!
//! v10 is archival and is deliberately not a premise of this module.  The
//! v9 artifact supplies the frozen, verdict-blind surface and its first
//! witness only.  Exact clause sources are reissued by the v11 theorem
//! program from contextual typed-structure evidence and the candidate-specific
//! motive-parametric v2 API.  Candidate-level Internal and exhaustion remain
//! withheld until that API earns universal totality on its issuable domain.

use crate::ambient_wrapper_domain::{
    MotiveGrammarFinitenessCertificate, PiCoverageRecord, issue_pi_coverage,
    motive_candidates_through, motive_grammar_finiteness_certificate,
};
use crate::global_e4_assembly_v9::{
    GLOBAL_E4_V9_SCHEMA, GlobalE4V9Certificate, NextWrappedUnknownRecord, replay_global_e4_v9_json,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v7::declared_context_control_candidate;
use pen_schema::internal_classifier_branch_v8::{
    AMBIENT_WRAPPER_VERSION, AmbientWrappedCandidate, AmbientWrapperError,
};
use pen_schema::internal_classifier_branch_v10::{
    ProofStrengthWrappedLiveSourceCertificateV11, WrappedCandidateDecisionV11,
    classify_wrapped_candidate_v11, replay_wrapped_candidate_decision_v11,
};
use pen_type::contextual_internality::{ContextualMotive, issue_ambient_context_declaration_token};
use pen_type::elaborate::SealedSignature;
use pen_type::motive_parametric_coherence::ClosureRuleKind;
use pen_type::motive_parametric_coherence_v2::{
    CLOSURE_RULE_INVENTORY_V2, MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION,
    MotiveTypedClosedAssignmentProjectionV2, SpecializedClosureDerivationV2,
    VerifiedClosureDerivationV2, issue_actual_body_closure_derivation_v2,
    issue_closed_internal_evidence_v2, issue_motive_typed_closed_assignment_v2,
    replay_specialized_closure_derivation_v2, specialize_verified_closure_derivation_v2,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V11_SCHEMA: &str = "schema2-global-e4-assembly-v11";
pub const GLOBAL_E4_V11_DATE: &str = "2026-07-22";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/motive_parametric_coherence_adjudication.md");
const V9_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v9.json");
const DOMAIN_SOURCE_BYTES: &[u8] = include_bytes!("ambient_wrapper_domain.rs");
const CONTEXTUAL_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/contextual_internality.rs");
const MOTIVE_V2_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence_v2.rs");
const CLASSIFIER_V11_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v10.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v9.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v11.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V11SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactV9WitnessResolutionV11 {
    pub frozen_v9_witness: NextWrappedUnknownRecord,
    pub exact_candidate: AmbientWrappedCandidate,
    pub successor_decision: WrappedCandidateDecisionV11,
    pub exact_contextual_source: VerifiedClosureDerivationV2,
    pub exact_used_prior_clause_indices: Vec<u16>,
    pub frozen_projection_and_motive_match: bool,
    pub v9_record_used_only_as_enumeration_witness: bool,
    pub successor_source_replayed: bool,
    pub successor_live_source_established: bool,
    pub candidate_level_internality_authorized: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactEliminatorRegressionV11 {
    pub source_derivation: VerifiedClosureDerivationV2,
    pub assignment: MotiveTypedClosedAssignmentProjectionV2,
    pub specialization: SpecializedClosureDerivationV2,
    pub source_is_exact_singleton_ambient_projection: bool,
    pub image_carries_replayed_closed_internal_evidence: bool,
    pub specialization_is_exact_closed_universe_body: bool,
    pub specialized_replay_succeeded: bool,
    pub representative_promoted_to_generic_theorem: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WrappedScanDispositionV11 {
    SyntacticNonmember { reason: String },
    ExactLiveSourceEstablished { certificate_hash: String },
    ExactTypedExclusion { clause_index: u16, reason: String },
    ExactNamedObstruction { clause_index: u16, reason: String },
    OutsideFrozenSurface { reason: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FirstWrappedObstructionV11 {
    pub syntactic_index: usize,
    pub motive: ContextualMotive,
    pub disposition: WrappedScanDispositionV11,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProofStrengthWrappedDomainScanV11 {
    pub finiteness: MotiveGrammarFinitenessCertificate,
    pub pi_coverage: PiCoverageRecord,
    pub motive_node_cap: u32,
    pub syntactic_motives_expected: u128,
    pub syntactic_motives_generated: usize,
    pub syntactic_motives_visited: usize,
    pub syntactic_nonmembers: usize,
    pub admissible_wrappers: usize,
    pub exact_live_source_wrappers: usize,
    pub exact_typed_exclusions: usize,
    pub outside_frozen_surface: usize,
    pub named_obstructions: usize,
    pub first_obstruction: Option<FirstWrappedObstructionV11>,
    pub fail_fast_at_first_obstruction: bool,
    pub scan_completed: bool,
    pub admissible_partition_exact: bool,
    pub rolling_scan_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V11Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V11SourceBinding>,
    pub predecessor_global_e4_v9_digest: String,
    pub predecessor_global_e4_v9_replayed: bool,
    pub motive_v2_version: String,
    pub closure_rule_inventory: Vec<ClosureRuleKind>,
    pub motive_v2_universal_totality_established: bool,
    pub exact_eliminator_regression: ExactEliminatorRegressionV11,
    pub predecessor_v9_witness_resolved: ExactV9WitnessResolutionV11,
    pub wrapped_domain_scan: ProofStrengthWrappedDomainScanV11,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub proof_strength_downstream_replay_authorized: bool,
    pub blocking_obligations: Vec<String>,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V11Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub v9_witness_live_source_established: bool,
    pub exact_eliminator_regression_replayed: bool,
    pub full_syntactic_domain_visited: bool,
    pub f_a5_found_survivor: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub downstream_replay_authorized: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V11Error {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn workspace_doc_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs")
        .join(name)
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V11_SCHEMA, domain, value))
        .expect("global E-4 v11 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(v9_bytes: &[u8]) -> Vec<GlobalE4V11SourceBinding> {
    [
        (
            "docs/schema2_global_e4_assembly_v9.json",
            "frozen_verdict_blind_wrapped_surface_and_first_witness",
            v9_bytes,
        ),
        (
            "docs/motive_parametric_coherence_adjudication.md",
            "adopted_universal_conditional_and_vacuous_case",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/ambient_wrapper_domain.rs",
            "finite_six_node_motive_enumerator",
            DOMAIN_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "probe_free_exact_typed_structure_judgment",
            CONTEXTUAL_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence_v2.rs",
            "candidate_specific_six_rule_specialization_eliminator",
            MOTIVE_V2_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v10.rs",
            "blocked_proof_strength_live_source_classifier_v11",
            CLASSIFIER_V11_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v9.rs",
            "archival_fail_fast_surface_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v11.rs",
            "proof_strength_global_exhaustion_assembly",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V11SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn issue_exact_eliminator_regression() -> Result<ExactEliminatorRegressionV11, GlobalE4V11Error> {
    let signature = SealedSignature::genesis_del_h15();
    let source_candidate =
        Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, Expr::Var(1))]);
    let declaration = issue_ambient_context_declaration_token(
        &signature,
        &source_candidate,
        15,
        vec![ContextualMotive::Type],
    )
    .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    let source = issue_actual_body_closure_derivation_v2(
        &signature,
        &source_candidate,
        15,
        0,
        Some(&declaration),
        &BTreeMap::new(),
    )
    .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;

    let closed_candidate =
        Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, Expr::Univ)]);
    let closed_relation = issue_actual_body_closure_derivation_v2(
        &signature,
        &closed_candidate,
        15,
        0,
        None,
        &BTreeMap::new(),
    )
    .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    let closed_evidence = issue_closed_internal_evidence_v2(&signature, &closed_relation)
        .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    let assignment = issue_motive_typed_closed_assignment_v2(
        &signature,
        15,
        vec![ContextualMotive::Type],
        vec![closed_evidence.projection().clone()],
    )
    .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    let specialization =
        specialize_verified_closure_derivation_v2(&signature, &source, &assignment)
            .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    replay_specialized_closure_derivation_v2(&signature, specialization.projection())
        .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;

    let source_is_exact_singleton_ambient_projection = source.projection().candidate
        == source_candidate
        && source.projection().expression == Expr::Var(1)
        && source.projection().ambient_arity == 1
        && source.projection().rule == ClosureRuleKind::Projection;
    let image_carries_replayed_closed_internal_evidence = assignment
        .projection()
        .images
        .iter()
        .all(|image| image.evidence_replayed && image.evidence.closed_candidate);
    let specialization_is_exact_closed_universe_body =
        specialization.projection().specialized_candidate == closed_candidate
            && specialization.projection().specialized_expression == Expr::Univ
            && specialization
                .projection()
                .specialized_relation
                .relation
                .candidate
                == closed_candidate;
    let specialized_replay_succeeded = true;
    let representative_promoted_to_generic_theorem = false;
    if !source_is_exact_singleton_ambient_projection
        || !image_carries_replayed_closed_internal_evidence
        || !specialization_is_exact_closed_universe_body
        || representative_promoted_to_generic_theorem
    {
        return Err(GlobalE4V11Error::Invariant(
            "exact eliminator regression drifted".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "exact-eliminator-regression",
        &(
            source.projection(),
            assignment.projection(),
            specialization.projection(),
            source_is_exact_singleton_ambient_projection,
            image_carries_replayed_closed_internal_evidence,
            specialization_is_exact_closed_universe_body,
            specialized_replay_succeeded,
            representative_promoted_to_generic_theorem,
        ),
    );
    Ok(ExactEliminatorRegressionV11 {
        source_derivation: source.projection().clone(),
        assignment: assignment.projection().clone(),
        specialization: specialization.projection().clone(),
        source_is_exact_singleton_ambient_projection,
        image_carries_replayed_closed_internal_evidence,
        specialization_is_exact_closed_universe_body,
        specialized_replay_succeeded,
        representative_promoted_to_generic_theorem,
        derivation_hash,
    })
}

fn live_source_certificate(
    decision: &WrappedCandidateDecisionV11,
) -> Option<&ProofStrengthWrappedLiveSourceCertificateV11> {
    let WrappedCandidateDecisionV11::ExactLiveContextualSourceEstablished { certificate, .. } =
        decision
    else {
        return None;
    };
    Some(certificate)
}

fn issue_exact_v9_resolution(
    v9: &GlobalE4V9Certificate,
) -> Result<ExactV9WitnessResolutionV11, GlobalE4V11Error> {
    let frozen_v9_witness = v9.wrapped_fiber_scan.next_unknown.clone();
    let exact_candidate = frozen_v9_witness.wrapped_candidate.clone();
    let frozen_projection_and_motive_match = exact_candidate.clauses
        == declared_context_control_candidate()
        && exact_candidate.ambient == vec![frozen_v9_witness.exact_motive.clone()]
        && frozen_v9_witness.exact_raw_projection;
    let successor_decision = classify_wrapped_candidate_v11(&exact_candidate)
        .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    replay_wrapped_candidate_decision_v11(&exact_candidate, &successor_decision)
        .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    let certificate = live_source_certificate(&successor_decision).ok_or_else(|| {
        GlobalE4V11Error::Invariant(format!(
            "exact v9 witness did not establish its v11 live source: {successor_decision:?}"
        ))
    })?;
    let live = certificate
        .exact_contextual_clauses
        .iter()
        .find(|record| record.clause_index == 1)
        .ok_or_else(|| {
            GlobalE4V11Error::Invariant(
                "exact v9 witness lacks clause-1 contextual source".to_owned(),
            )
        })?;
    let exact_contextual_source = live.exact_closure_derivation.clone();
    let exact_used_prior_clause_indices = live.exact_used_prior_clause_indices.clone();
    let v9_record_used_only_as_enumeration_witness = true;
    let successor_source_replayed = live.source_replayed;
    let successor_live_source_established = true;
    let candidate_level_internality_authorized = certificate.candidate_level_internality_authorized;
    let marginal_nu = certificate.marginal_nu;
    if !frozen_projection_and_motive_match
        || !exact_used_prior_clause_indices.is_empty()
        || !successor_source_replayed
        || candidate_level_internality_authorized
        || marginal_nu != 0
    {
        return Err(GlobalE4V11Error::Invariant(
            "exact v9 witness resolution drifted".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "exact-v9-witness-resolution",
        &(
            &frozen_v9_witness,
            &exact_candidate,
            &successor_decision,
            &exact_contextual_source,
            &exact_used_prior_clause_indices,
            frozen_projection_and_motive_match,
            v9_record_used_only_as_enumeration_witness,
            successor_source_replayed,
            successor_live_source_established,
            candidate_level_internality_authorized,
            marginal_nu,
        ),
    );
    Ok(ExactV9WitnessResolutionV11 {
        frozen_v9_witness,
        exact_candidate,
        successor_decision,
        exact_contextual_source,
        exact_used_prior_clause_indices,
        frozen_projection_and_motive_match,
        v9_record_used_only_as_enumeration_witness,
        successor_source_replayed,
        successor_live_source_established,
        candidate_level_internality_authorized,
        marginal_nu,
        derivation_hash,
    })
}

fn scan_disposition_hash(
    rolling: &str,
    index: usize,
    motive: &ContextualMotive,
    disposition: &WrappedScanDispositionV11,
) -> String {
    tagged_hash("wrapped-scan-step", &(rolling, index, motive, disposition))
}

fn issue_wrapped_domain_scan(
    motive_node_cap: u32,
) -> Result<ProofStrengthWrappedDomainScanV11, GlobalE4V11Error> {
    let finiteness = motive_grammar_finiteness_certificate();
    let raw_candidate = declared_context_control_candidate();
    let pi_coverage = issue_pi_coverage(&raw_candidate)
        .map_err(|error| GlobalE4V11Error::Invariant(error.to_string()))?;
    let motives = motive_candidates_through(motive_node_cap);
    let syntactic_motives_generated = motives.len();
    let syntactic_motives_expected = finiteness
        .syntactic_motive_counts_by_exact_nodes
        .iter()
        .take(motive_node_cap as usize + 1)
        .copied()
        .sum::<u128>();
    let mut syntactic_motives_visited = 0usize;
    let mut syntactic_nonmembers = 0usize;
    let mut admissible_wrappers = 0usize;
    let mut exact_live_source_wrappers = 0usize;
    let mut exact_typed_exclusions = 0usize;
    let mut outside_frozen_surface = 0usize;
    let mut named_obstructions = 0usize;
    let mut first_obstruction = None;
    let fail_fast_at_first_obstruction = true;
    let mut rolling_scan_hash = tagged_hash(
        "wrapped-scan-seed",
        &(&raw_candidate, motive_node_cap, syntactic_motives_expected),
    );

    for (index, motive) in motives.into_iter().enumerate() {
        syntactic_motives_visited += 1;
        let candidate = AmbientWrappedCandidate {
            version: AMBIENT_WRAPPER_VERSION.to_owned(),
            clauses: raw_candidate.clone(),
            ambient: vec![motive.clone()],
        };
        let disposition = match classify_wrapped_candidate_v11(&candidate) {
            Err(AmbientWrapperError::Declaration(reason)) => {
                syntactic_nonmembers += 1;
                WrappedScanDispositionV11::SyntacticNonmember { reason }
            }
            Err(error) => {
                named_obstructions += 1;
                WrappedScanDispositionV11::ExactNamedObstruction {
                    clause_index: u16::MAX,
                    reason: error.to_string(),
                }
            }
            Ok(WrappedCandidateDecisionV11::ExactLiveContextualSourceEstablished {
                certificate,
                ..
            }) => {
                admissible_wrappers += 1;
                exact_live_source_wrappers += 1;
                WrappedScanDispositionV11::ExactLiveSourceEstablished {
                    certificate_hash: certificate.derivation_hash,
                }
            }
            Ok(WrappedCandidateDecisionV11::NamedTypedExclusion {
                clause_index,
                error,
                ..
            }) => {
                admissible_wrappers += 1;
                exact_typed_exclusions += 1;
                WrappedScanDispositionV11::ExactTypedExclusion {
                    clause_index,
                    reason: error,
                }
            }
            Ok(WrappedCandidateDecisionV11::NamedTypedObstruction {
                clause_index,
                reason,
                ..
            }) => {
                admissible_wrappers += 1;
                named_obstructions += 1;
                WrappedScanDispositionV11::ExactNamedObstruction {
                    clause_index,
                    reason,
                }
            }
            Ok(WrappedCandidateDecisionV11::OutsideFrozenSuccessorSurface { code, .. }) => {
                admissible_wrappers += 1;
                outside_frozen_surface += 1;
                WrappedScanDispositionV11::OutsideFrozenSurface { reason: code }
            }
        };
        rolling_scan_hash = scan_disposition_hash(&rolling_scan_hash, index, &motive, &disposition);
        if matches!(
            disposition,
            WrappedScanDispositionV11::ExactNamedObstruction { .. }
                | WrappedScanDispositionV11::OutsideFrozenSurface { .. }
        ) {
            let derivation_hash = tagged_hash(
                "first-wrapped-obstruction",
                &(index, &motive, &disposition, &rolling_scan_hash),
            );
            first_obstruction = Some(FirstWrappedObstructionV11 {
                syntactic_index: index,
                motive,
                disposition,
                derivation_hash,
            });
            break;
        }
    }

    let scan_completed = syntactic_motives_visited == syntactic_motives_generated;
    let admissible_partition_exact = admissible_wrappers
        == exact_live_source_wrappers
            + exact_typed_exclusions
            + outside_frozen_surface
            + named_obstructions;
    let derivation_hash = tagged_hash(
        "proof-strength-wrapped-domain-scan",
        &(
            (
                &finiteness,
                &pi_coverage,
                motive_node_cap,
                syntactic_motives_expected,
                syntactic_motives_generated,
                syntactic_motives_visited,
                syntactic_nonmembers,
                admissible_wrappers,
            ),
            (
                exact_live_source_wrappers,
                exact_typed_exclusions,
                outside_frozen_surface,
                named_obstructions,
                &first_obstruction,
                fail_fast_at_first_obstruction,
                scan_completed,
                admissible_partition_exact,
            ),
            &rolling_scan_hash,
        ),
    );
    Ok(ProofStrengthWrappedDomainScanV11 {
        finiteness,
        pi_coverage,
        motive_node_cap,
        syntactic_motives_expected,
        syntactic_motives_generated,
        syntactic_motives_visited,
        syntactic_nonmembers,
        admissible_wrappers,
        exact_live_source_wrappers,
        exact_typed_exclusions,
        outside_frozen_surface,
        named_obstructions,
        first_obstruction,
        fail_fast_at_first_obstruction,
        scan_completed,
        admissible_partition_exact,
        rolling_scan_hash,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V11Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v11-certificate", &projection)
}

pub fn issue_global_e4_v11_certificate() -> Result<GlobalE4V11Certificate, GlobalE4V11Error> {
    let v9_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v9.json"))
        .map_err(|error| GlobalE4V11Error::Io(error.to_string()))?;
    if v9_bytes != V9_ARTIFACT_BYTES {
        return Err(GlobalE4V11Error::Prerequisite(
            "v9 artifact bytes drifted".to_owned(),
        ));
    }
    let v9_json = std::str::from_utf8(&v9_bytes)
        .map_err(|error| GlobalE4V11Error::Json(error.to_string()))?;
    let v9_replay = replay_global_e4_v9_json(v9_json);
    let v9: GlobalE4V9Certificate = serde_json::from_slice(&v9_bytes)
        .map_err(|error| GlobalE4V11Error::Json(error.to_string()))?;
    if v9.schema != GLOBAL_E4_V9_SCHEMA
        || !v9_replay.valid
        || !v9.wrapped_fiber_scan.next_unknown.f_a5_retained
    {
        return Err(GlobalE4V11Error::Prerequisite(format!(
            "v9 frozen surface replay failed: {}",
            v9_replay.errors.join("; ")
        )));
    }

    let exact_eliminator_regression = issue_exact_eliminator_regression()?;
    let predecessor_v9_witness_resolved = issue_exact_v9_resolution(&v9)?;
    let wrapped_domain_scan = issue_wrapped_domain_scan(6)?;
    let closure_rule_inventory = CLOSURE_RULE_INVENTORY_V2.to_vec();
    let motive_v2_universal_totality_established = false;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let proof_strength_downstream_replay_authorized = false;
    let blocking_obligations = vec![
        "release a versioned motive-v2 eliminator theorem whose issuable-source domain excludes or repairs every registered F-M1 counterexample".to_owned(),
        "bind that released source version and hash before promoting exact live-source evidence to candidate-level contextual Internal".to_owned(),
    ];
    let outcome =
        "global_e4_v11_theorem_program_blocked_motive_v2_universal_totality_not_established";
    let permitted_conclusion = "The exact v9 two-clause candidate and declaration replay. Clause 0 carries a reissued singleton AmbientArenaReference Internal derivation; clause 1 carries an exact whole-candidate contextual v2 source with empty used-field support. A separate universe-image specialization replays. These facts do not establish candidate-level Internal or wrapped-domain exhaustion: the current motive-v2 issuer domain still has registered substitution-stability counterexamples, so its six-case function cannot yet be used as the adopted universal theorem.";
    let required_successor_action = "Repair or restrict motive-v2 by a versioned, source-bound theorem covering its exact issuable domain, rerun its adversarial audit, and only then rerun Global E-4 v11. Do not emit create-new, authorize downstream replays, or bridge from this theorem-program result.";
    let mut certificate = GlobalE4V11Certificate {
        schema: GLOBAL_E4_V11_SCHEMA.to_owned(),
        date: GLOBAL_E4_V11_DATE.to_owned(),
        source_bindings: source_bindings(&v9_bytes),
        predecessor_global_e4_v9_digest: v9.result_digest,
        predecessor_global_e4_v9_replayed: true,
        motive_v2_version: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        closure_rule_inventory,
        motive_v2_universal_totality_established,
        exact_eliminator_regression,
        predecessor_v9_witness_resolved,
        wrapped_domain_scan,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        proof_strength_downstream_replay_authorized,
        blocking_obligations,
        outcome: outcome.to_owned(),
        permitted_conclusion: permitted_conclusion.to_owned(),
        required_successor_action: required_successor_action.to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V11Replay {
    GlobalE4V11Replay {
        valid: false,
        rerun_executed: false,
        v9_witness_live_source_established: false,
        exact_eliminator_regression_replayed: false,
        full_syntactic_domain_visited: false,
        f_a5_found_survivor: false,
        no_unknown_survives: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        downstream_replay_authorized: false,
        outcome: "global_e4_v11_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V11Certificate,
    expected: &GlobalE4V11Certificate,
) -> GlobalE4V11Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V11Replay {
        valid: errors.is_empty(),
        rerun_executed: true,
        v9_witness_live_source_established: certificate
            .predecessor_v9_witness_resolved
            .successor_live_source_established,
        exact_eliminator_regression_replayed: certificate
            .exact_eliminator_regression
            .specialized_replay_succeeded,
        full_syntactic_domain_visited: certificate.wrapped_domain_scan.scan_completed,
        f_a5_found_survivor: !certificate.motive_v2_universal_totality_established
            || certificate.wrapped_domain_scan.named_obstructions != 0
            || certificate.wrapped_domain_scan.outside_frozen_surface != 0,
        no_unknown_survives: certificate.no_unknown_survives,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        downstream_replay_authorized: certificate.proof_strength_downstream_replay_authorized,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v11_certificate(certificate: &GlobalE4V11Certificate) -> GlobalE4V11Replay {
    match issue_global_e4_v11_certificate() {
        Ok(expected) => replay_against_expected(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn replay_global_e4_v11_json(json: &str) -> GlobalE4V11Replay {
    let json = json.to_owned();
    let worker = match std::thread::Builder::new()
        .name("global-e4-v11-replay".to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(
            move || match serde_json::from_str::<GlobalE4V11Certificate>(&json) {
                Ok(certificate) => replay_global_e4_v11_certificate(&certificate),
                Err(error) => failed_replay(format!("JSON parse failed: {error}")),
            },
        ) {
        Ok(worker) => worker,
        Err(error) => return failed_replay(format!("replay worker spawn failed: {error}")),
    };
    worker
        .join()
        .unwrap_or_else(|_| failed_replay("replay worker panicked"))
}

pub fn emit_global_e4_v11_create_new(path: &Path) -> Result<GlobalE4V11Replay, GlobalE4V11Error> {
    let certificate = issue_global_e4_v11_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V11Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V11Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V11Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v11_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V11Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_v9_witness_and_concrete_eliminator_replay() {
        let v9: GlobalE4V9Certificate =
            serde_json::from_slice(V9_ARTIFACT_BYTES).expect("v9 artifact");
        let witness = issue_exact_v9_resolution(&v9).expect("exact v9 resolution");
        assert!(witness.successor_live_source_established);
        assert!(!witness.candidate_level_internality_authorized);
        assert!(witness.exact_used_prior_clause_indices.is_empty());
        let regression = issue_exact_eliminator_regression().expect("exact regression");
        assert!(regression.specialized_replay_succeeded);
        assert!(!regression.representative_promoted_to_generic_theorem);
    }

    #[test]
    fn bounded_scan_partitions_without_promoting_nonmembers() {
        let scan = issue_wrapped_domain_scan(4).expect("four-node scan");
        assert!(scan.scan_completed);
        assert_eq!(scan.named_obstructions, 0);
        assert_eq!(scan.outside_frozen_surface, 0);
        assert!(scan.admissible_partition_exact);
        assert_eq!(
            scan.syntactic_motives_generated as u128,
            scan.syntactic_motives_expected
        );
    }

    #[test]
    fn component_mutations_fail_exact_replay() {
        let v9: GlobalE4V9Certificate =
            serde_json::from_slice(V9_ARTIFACT_BYTES).expect("v9 artifact");
        let mut witness = issue_exact_v9_resolution(&v9).expect("exact v9 resolution");
        witness.exact_contextual_source.expression = Expr::Univ;
        let expected = issue_exact_v9_resolution(&v9).expect("expected");
        assert_ne!(witness, expected);

        let mut regression = issue_exact_eliminator_regression().expect("exact regression");
        regression.specialization.specialized_expression = Expr::Lib(15);
        assert!(
            replay_specialized_closure_derivation_v2(
                &SealedSignature::genesis_del_h15(),
                &regression.specialization,
            )
            .is_err()
        );
    }
}
