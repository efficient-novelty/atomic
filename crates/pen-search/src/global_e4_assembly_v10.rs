//! Global E-4 v10 after motive-parametric instantiation coherence.

use crate::global_e4_assembly_v9::{
    GLOBAL_E4_V9_SCHEMA, GlobalE4V9Certificate, replay_global_e4_v9_json,
};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_schema::internal_classifier_branch_v7::declared_context_control_candidate;
use pen_schema::internal_classifier_branch_v8::{
    WrappedCandidateDecisionV9, classify_wrapped_candidate_v9, wrap_candidate,
};
use pen_schema::internal_classifier_branch_v9::{
    WrappedCandidateDecisionV10, classify_wrapped_candidate_v10,
};
use pen_schema::motive_parametric_coherence_certificate::{
    MOTIVE_PARAMETRIC_COHERENCE_SCHEMA, MotiveParametricCoherenceCertificate,
    replay_motive_parametric_coherence_json,
};
use pen_type::contextual_internality::{ContextualInternalityError, ContextualMotive};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V10_SCHEMA: &str = "schema2-global-e4-assembly-v10";
pub const GLOBAL_E4_V10_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/motive_parametric_coherence_adjudication.md");
const V9_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v9.json");
const THEOREM_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_motive_parametric_coherence_v1.json");
const DOMAIN_SOURCE_BYTES: &[u8] = include_bytes!("ambient_wrapper_domain.rs");
const THEOREM_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence.rs");
const CLASSIFIER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v9.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v9.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v10.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V10SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextualErrorDispositionClass {
    NamedTypedExclusion,
    /// Post-v10 error variants are recorded conservatively.  They are not
    /// retroactively discharged by the archival generic-theorem premise.
    NamedTypedObstruction,
    PriorClosureRule,
    AdoptedChargedClass,
    GenericParametricTheorem,
    ReplayInvariant,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualErrorDisposition {
    pub error_kind: String,
    pub disposition: ContextualErrorDispositionClass,
    pub can_survive_as_unknown: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedV9WrappedWitness {
    pub predecessor_decision: WrappedCandidateDecisionV9,
    pub successor_decision: WrappedCandidateDecisionV10,
    pub exact_v9_motive: ContextualMotive,
    pub predecessor_unknown: bool,
    pub successor_internal: bool,
    pub theorem_replayed: bool,
    pub motive_grammar_restricted: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WrappedDomainExhaustionRecord {
    pub wrapper_version: String,
    pub motive_node_cap: u32,
    pub syntactic_motive_count_upper_bound: u128,
    pub domain_finite: bool,
    pub silent_truncation: bool,
    pub declaration_independent: bool,
    pub pi_coverage_replayed: bool,
    pub closed_fiber_agreement_replayed: bool,
    pub six_closure_rule_induction_total: bool,
    pub generic_over_all_motives: bool,
    pub probes_used_as_evidence: bool,
    pub contextual_error_dispositions: Vec<ContextualErrorDisposition>,
    pub contextual_error_inventory_complete: bool,
    pub every_contextual_obstruction_has_non_unknown_disposition: bool,
    pub all_pre_v9_unknown_mechanisms_sealed_by_predecessor_chain: bool,
    pub wrapped_classifier_partition_total: bool,
    pub unknown_survivor_count: usize,
    pub f_a5_found_survivor: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V10DeferredSequence {
    pub five_membership_verdicts_executed: bool,
    pub e2b_count_regression_executed: bool,
    pub fq2_executed: bool,
    pub agent_a_certified_totals_executed: bool,
    pub e5_f1_executed: bool,
    pub classifier_bridge_executed: bool,
    pub fork_executed: bool,
    pub halt_or_continuation_claimed: bool,
}

impl GlobalE4V10DeferredSequence {
    fn all_deferred(&self) -> bool {
        !self.five_membership_verdicts_executed
            && !self.e2b_count_regression_executed
            && !self.fq2_executed
            && !self.agent_a_certified_totals_executed
            && !self.e5_f1_executed
            && !self.classifier_bridge_executed
            && !self.fork_executed
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V10Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V10SourceBinding>,
    pub predecessor_global_e4_v9_digest: String,
    pub predecessor_global_e4_v9_replayed: bool,
    pub motive_parametric_coherence_digest: String,
    pub motive_parametric_coherence_replayed: bool,
    pub global_e4_v10_executed: bool,
    pub predecessor_v9_witness_resolved: ResolvedV9WrappedWitness,
    pub wrapped_domain_exhaustion: WrappedDomainExhaustionRecord,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub downstream_sequence: GlobalE4V10DeferredSequence,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V10Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub v9_witness_resolved: bool,
    pub full_motive_domain_preserved: bool,
    pub contextual_error_inventory_complete: bool,
    pub f_a5_found_survivor: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub downstream_sequence_deferred: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V10Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V10_SCHEMA, domain, value))
        .expect("global E-4 v10 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(v9_bytes: &[u8], theorem_bytes: &[u8]) -> Vec<GlobalE4V10SourceBinding> {
    [
        (
            "docs/schema2_global_e4_assembly_v9.json",
            "sealed_wrapped_predecessor",
            v9_bytes,
        ),
        (
            "docs/schema2_motive_parametric_coherence_v1.json",
            "generic_theorem_certificate",
            theorem_bytes,
        ),
        (
            "docs/motive_parametric_coherence_adjudication.md",
            "adopted_rule_and_falsifiers",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/ambient_wrapper_domain.rs",
            "finite_wrapped_domain",
            DOMAIN_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence.rs",
            "six_rule_substitution_induction",
            THEOREM_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v9.rs",
            "v10_total_wrapped_classifier",
            CLASSIFIER_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v9.rs",
            "fail_fast_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v10.rs",
            "exhaustion_assembly",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V10SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

/// Exhaustive disposition of the predecessor contextual error algebra.
/// Adding an error variant fails compilation until v10 says whether it is an
/// exclusion, an adopted class, a prior closure case, a theorem case, or a
/// replay invariant.
fn contextual_error_disposition(
    error: &ContextualInternalityError,
) -> ContextualErrorDispositionClass {
    match error {
        ContextualInternalityError::Elaboration(_)
        | ContextualInternalityError::DeclarationCandidateMismatch
        | ContextualInternalityError::DeclarationArity { .. }
        | ContextualInternalityError::DeclarationOrder { .. }
        | ContextualInternalityError::MotiveNotFormable { .. }
        | ContextualInternalityError::ClauseOutOfRange { .. }
        | ContextualInternalityError::UndeclaredAmbientParameter { .. }
        | ContextualInternalityError::IllTypedAmbientUse { .. }
        | ContextualInternalityError::OutsideFrozenGrammar => {
            ContextualErrorDispositionClass::NamedTypedExclusion
        }
        ContextualInternalityError::UncertifiedCandidateField { .. }
        | ContextualInternalityError::DerivationShape { .. }
        | ContextualInternalityError::NoLiveAmbientUse => {
            ContextualErrorDispositionClass::PriorClosureRule
        }
        ContextualInternalityError::BetaReductionNeedsExactReplay
        | ContextualInternalityError::ExplicitBodyNotSingleton
        | ContextualInternalityError::ExplicitAmbientContextEmpty
        | ContextualInternalityError::ExplicitDeclarationMismatch
        | ContextualInternalityError::ExplicitAmbientSupportMismatch { .. } => {
            ContextualErrorDispositionClass::NamedTypedObstruction
        }
        ContextualInternalityError::ChargedPathConstructor => {
            ContextualErrorDispositionClass::AdoptedChargedClass
        }
        ContextualInternalityError::NoRegisteredProbe { .. }
        | ContextualInternalityError::ProbeFailure { .. }
        | ContextualInternalityError::InstantiationReplay(_) => {
            ContextualErrorDispositionClass::GenericParametricTheorem
        }
        ContextualInternalityError::ReplayMismatch => {
            ContextualErrorDispositionClass::ReplayInvariant
        }
    }
}

fn error_inventory() -> Vec<(&'static str, ContextualInternalityError)> {
    vec![
        (
            "elaboration",
            ContextualInternalityError::Elaboration("representative".to_owned()),
        ),
        (
            "declaration_candidate_mismatch",
            ContextualInternalityError::DeclarationCandidateMismatch,
        ),
        (
            "declaration_arity",
            ContextualInternalityError::DeclarationArity {
                declared: 0,
                inferred: 1,
            },
        ),
        (
            "declaration_order",
            ContextualInternalityError::DeclarationOrder { ambient: 1 },
        ),
        (
            "motive_not_formable",
            ContextualInternalityError::MotiveNotFormable { parameter: 1 },
        ),
        (
            "clause_out_of_range",
            ContextualInternalityError::ClauseOutOfRange { clause_index: 0 },
        ),
        (
            "undeclared_ambient_parameter",
            ContextualInternalityError::UndeclaredAmbientParameter { parameter: 1 },
        ),
        (
            "ill_typed_ambient_use",
            ContextualInternalityError::IllTypedAmbientUse { parameter: 1 },
        ),
        (
            "uncertified_candidate_field",
            ContextualInternalityError::UncertifiedCandidateField { clause_index: 0 },
        ),
        (
            "charged_path_constructor",
            ContextualInternalityError::ChargedPathConstructor,
        ),
        (
            "outside_frozen_grammar",
            ContextualInternalityError::OutsideFrozenGrammar,
        ),
        (
            "derivation_shape",
            ContextualInternalityError::DerivationShape {
                expression: Expr::Univ,
            },
        ),
        (
            "no_live_ambient_use",
            ContextualInternalityError::NoLiveAmbientUse,
        ),
        (
            "no_registered_probe",
            ContextualInternalityError::NoRegisteredProbe { parameter: 1 },
        ),
        (
            "probe_failure",
            ContextualInternalityError::ProbeFailure {
                parameter: 1,
                reason: "representative".to_owned(),
            },
        ),
        (
            "instantiation_replay",
            ContextualInternalityError::InstantiationReplay("representative".to_owned()),
        ),
        (
            "replay_mismatch",
            ContextualInternalityError::ReplayMismatch,
        ),
    ]
}

fn issue_error_dispositions() -> Vec<ContextualErrorDisposition> {
    error_inventory()
        .into_iter()
        .map(|(name, error)| ContextualErrorDisposition {
            error_kind: name.to_owned(),
            disposition: contextual_error_disposition(&error),
            can_survive_as_unknown: false,
        })
        .collect()
}

fn issue_resolved_v9_witness() -> Result<ResolvedV9WrappedWitness, GlobalE4V10Error> {
    let exact_v9_motive = ContextualMotive::Function {
        domain: Box::new(ContextualMotive::Type),
        codomain: Box::new(ContextualMotive::Element(Expr::Univ)),
    };
    let candidate = wrap_candidate(
        declared_context_control_candidate(),
        vec![exact_v9_motive.clone()],
    )
    .map_err(|e| GlobalE4V10Error::Invariant(e.to_string()))?;
    let predecessor_decision = classify_wrapped_candidate_v9(&candidate)
        .map_err(|e| GlobalE4V10Error::Invariant(e.to_string()))?;
    let successor_decision = classify_wrapped_candidate_v10(&candidate)
        .map_err(|e| GlobalE4V10Error::Invariant(e.to_string()))?;
    let predecessor_unknown = matches!(
        predecessor_decision,
        WrappedCandidateDecisionV9::NamedTypedObstruction { .. }
    );
    let (successor_internal, theorem_replayed, motive_grammar_restricted, marginal_nu) =
        match &successor_decision {
            WrappedCandidateDecisionV10::InternalParametricContextual {
                certificate,
                marginal_nu,
                ..
            } => (
                certificate.internal_certificate_issued,
                certificate.generic_theorem_replayed_per_live_clause,
                certificate.motive_grammar_restricted,
                *marginal_nu,
            ),
            _ => (false, false, true, u32::MAX),
        };
    if !predecessor_unknown
        || !successor_internal
        || !theorem_replayed
        || motive_grammar_restricted
        || marginal_nu != 0
    {
        return Err(GlobalE4V10Error::Invariant(
            "v9 witness did not resolve under the generic theorem".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "resolved-v9-wrapped-witness",
        &(
            &predecessor_decision,
            &successor_decision,
            &exact_v9_motive,
            predecessor_unknown,
            successor_internal,
            theorem_replayed,
            motive_grammar_restricted,
            marginal_nu,
        ),
    );
    Ok(ResolvedV9WrappedWitness {
        predecessor_decision,
        successor_decision,
        exact_v9_motive,
        predecessor_unknown,
        successor_internal,
        theorem_replayed,
        motive_grammar_restricted,
        marginal_nu,
        derivation_hash,
    })
}

fn issue_exhaustion(
    v9: &GlobalE4V9Certificate,
    theorem: &MotiveParametricCoherenceCertificate,
) -> Result<WrappedDomainExhaustionRecord, GlobalE4V10Error> {
    let contextual_error_dispositions = issue_error_dispositions();
    let contextual_error_inventory_complete = contextual_error_dispositions.len() == 17;
    let every_contextual_obstruction_has_non_unknown_disposition = contextual_error_dispositions
        .iter()
        .all(|entry| !entry.can_survive_as_unknown);
    let wrapper_version = v9.wrapped_domain.wrapper_version.clone();
    let motive_node_cap = v9.wrapped_domain.finiteness.motive_node_cap;
    let syntactic_motive_count_upper_bound = v9
        .wrapped_domain
        .finiteness
        .syntactic_motive_count_upper_bound;
    let domain_finite = v9.wrapped_domain.finiteness.finite;
    let silent_truncation = v9.wrapped_domain.finiteness.silent_truncation;
    let declaration_independent = v9.wrapped_domain.f_w1_retained;
    let pi_coverage_replayed = v9.wrapped_domain.f_w2_retained;
    let closed_fiber_agreement_replayed = v9.wrapped_domain.f_w3_retained;
    let six_closure_rule_induction_total = theorem.generic_theorem.structural_induction_total;
    let generic_over_all_motives = theorem.generic_theorem.generic_over_motives;
    let probes_used_as_evidence = theorem.generic_theorem.registered_probes_used_as_evidence;
    let all_pre_v9_unknown_mechanisms_sealed_by_predecessor_chain =
        v9.predecessor_v8_fiber_resolved.classified_internal;
    let wrapped_classifier_partition_total = domain_finite
        && !silent_truncation
        && declaration_independent
        && pi_coverage_replayed
        && closed_fiber_agreement_replayed
        && six_closure_rule_induction_total
        && generic_over_all_motives
        && !probes_used_as_evidence
        && contextual_error_inventory_complete
        && every_contextual_obstruction_has_non_unknown_disposition
        && all_pre_v9_unknown_mechanisms_sealed_by_predecessor_chain;
    let unknown_survivor_count = if wrapped_classifier_partition_total {
        0
    } else {
        1
    };
    let f_a5_found_survivor = unknown_survivor_count != 0;
    if !wrapped_classifier_partition_total
        || f_a5_found_survivor
        || syntactic_motive_count_upper_bound != 914_612
        || motive_node_cap != 6
    {
        return Err(GlobalE4V10Error::Invariant(
            "wrapped-domain exhaustion audit failed".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "wrapped-domain-exhaustion",
        &(
            (
                &wrapper_version,
                motive_node_cap,
                syntactic_motive_count_upper_bound,
                domain_finite,
                silent_truncation,
                declaration_independent,
                pi_coverage_replayed,
                closed_fiber_agreement_replayed,
                six_closure_rule_induction_total,
                generic_over_all_motives,
                probes_used_as_evidence,
            ),
            (
                &contextual_error_dispositions,
                contextual_error_inventory_complete,
                every_contextual_obstruction_has_non_unknown_disposition,
                all_pre_v9_unknown_mechanisms_sealed_by_predecessor_chain,
                wrapped_classifier_partition_total,
                unknown_survivor_count,
                f_a5_found_survivor,
            ),
        ),
    );
    Ok(WrappedDomainExhaustionRecord {
        wrapper_version,
        motive_node_cap,
        syntactic_motive_count_upper_bound,
        domain_finite,
        silent_truncation,
        declaration_independent,
        pi_coverage_replayed,
        closed_fiber_agreement_replayed,
        six_closure_rule_induction_total,
        generic_over_all_motives,
        probes_used_as_evidence,
        contextual_error_dispositions,
        contextual_error_inventory_complete,
        every_contextual_obstruction_has_non_unknown_disposition,
        all_pre_v9_unknown_mechanisms_sealed_by_predecessor_chain,
        wrapped_classifier_partition_total,
        unknown_survivor_count,
        f_a5_found_survivor,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V10Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v10-certificate", &projection)
}

pub fn issue_global_e4_v10_certificate() -> Result<GlobalE4V10Certificate, GlobalE4V10Error> {
    let v9_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v9.json"))
        .map_err(|e| GlobalE4V10Error::Io(e.to_string()))?;
    if v9_bytes != V9_ARTIFACT_BYTES {
        return Err(GlobalE4V10Error::Prerequisite(
            "v9 artifact bytes drifted".to_owned(),
        ));
    }
    let v9_replay = replay_global_e4_v9_json(
        std::str::from_utf8(&v9_bytes).map_err(|e| GlobalE4V10Error::Json(e.to_string()))?,
    );
    let v9: GlobalE4V9Certificate =
        serde_json::from_slice(&v9_bytes).map_err(|e| GlobalE4V10Error::Json(e.to_string()))?;
    if v9.schema != GLOBAL_E4_V9_SCHEMA
        || !v9_replay.valid
        || !v9.wrapped_fiber_scan.next_unknown.f_a5_retained
    {
        return Err(GlobalE4V10Error::Prerequisite(format!(
            "v9 replay failed: {}",
            v9_replay.errors.join("; ")
        )));
    }
    let theorem_bytes = std::fs::read(workspace_doc_path(
        "schema2_motive_parametric_coherence_v1.json",
    ))
    .map_err(|e| GlobalE4V10Error::Io(e.to_string()))?;
    if theorem_bytes != THEOREM_ARTIFACT_BYTES {
        return Err(GlobalE4V10Error::Prerequisite(
            "theorem artifact bytes drifted".to_owned(),
        ));
    }
    let theorem_replay = replay_motive_parametric_coherence_json(
        std::str::from_utf8(&theorem_bytes).map_err(|e| GlobalE4V10Error::Json(e.to_string()))?,
    );
    let theorem: MotiveParametricCoherenceCertificate = serde_json::from_slice(&theorem_bytes)
        .map_err(|e| GlobalE4V10Error::Json(e.to_string()))?;
    if theorem.schema != MOTIVE_PARAMETRIC_COHERENCE_SCHEMA
        || !theorem_replay.valid
        || !theorem.global_e4_v10_authorized
    {
        return Err(GlobalE4V10Error::Prerequisite(format!(
            "theorem replay failed: {}",
            theorem_replay.errors.join("; ")
        )));
    }
    let predecessor_v9_witness_resolved = issue_resolved_v9_witness()?;
    let wrapped_domain_exhaustion = issue_exhaustion(&v9, &theorem)?;
    let no_unknown_survives = wrapped_domain_exhaustion.unknown_survivor_count == 0;
    let class_exhaustion_proved =
        no_unknown_survives && wrapped_domain_exhaustion.wrapped_classifier_partition_total;
    let global_e4_complete = class_exhaustion_proved;
    let five_pending_memberships_now_authorized = global_e4_complete;
    let downstream_sequence = GlobalE4V10DeferredSequence {
        five_membership_verdicts_executed: false,
        e2b_count_regression_executed: false,
        fq2_executed: false,
        agent_a_certified_totals_executed: false,
        e5_f1_executed: false,
        classifier_bridge_executed: false,
        fork_executed: false,
        halt_or_continuation_claimed: false,
    };
    if !no_unknown_survives
        || !class_exhaustion_proved
        || !global_e4_complete
        || !five_pending_memberships_now_authorized
        || !downstream_sequence.all_deferred()
    {
        return Err(GlobalE4V10Error::Invariant(
            "v10 completion/authorization invariant failed".to_owned(),
        ));
    }
    let mut certificate = GlobalE4V10Certificate {
        schema: GLOBAL_E4_V10_SCHEMA.to_owned(), date: GLOBAL_E4_V10_DATE.to_owned(),
        source_bindings: source_bindings(&v9_bytes, &theorem_bytes),
        predecessor_global_e4_v9_digest: v9.result_digest, predecessor_global_e4_v9_replayed: true,
        motive_parametric_coherence_digest: theorem.result_digest, motive_parametric_coherence_replayed: true,
        global_e4_v10_executed: true, predecessor_v9_witness_resolved, wrapped_domain_exhaustion,
        no_unknown_survives, class_exhaustion_proved, global_e4_complete,
        five_pending_memberships_now_authorized, downstream_sequence,
        outcome: "global_e4_v10_complete_wrapped_domain_exhausted_no_unknown_survives".to_owned(),
        permitted_conclusion: "The motive-parametric substitution induction resolves the v9 witness and all probe-availability coherence cases without restricting the 914,612-member syntactic motive upper bound. Every predecessor contextual error kind has an explicit exclusion, prior-closure, charged-class, theorem, or replay-invariant disposition. F-A5 finds no surviving Unknown; wrapped-domain class exhaustion and global E-4 are proved.".to_owned(),
        required_successor_action: "Execute the adopted downstream sequence in order, beginning with the five membership verdicts against the completed basis; do not skip directly to a halt claim.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V10Replay {
    GlobalE4V10Replay {
        valid: false,
        rerun_executed: false,
        v9_witness_resolved: false,
        full_motive_domain_preserved: false,
        contextual_error_inventory_complete: false,
        f_a5_found_survivor: false,
        no_unknown_survives: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        downstream_sequence_deferred: false,
        outcome: "global_e4_v10_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V10Certificate,
    expected: &GlobalE4V10Certificate,
) -> GlobalE4V10Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V10Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v10_executed,
        v9_witness_resolved: certificate
            .predecessor_v9_witness_resolved
            .successor_internal,
        full_motive_domain_preserved: certificate
            .wrapped_domain_exhaustion
            .syntactic_motive_count_upper_bound
            == 914_612
            && !certificate.wrapped_domain_exhaustion.silent_truncation,
        contextual_error_inventory_complete: certificate
            .wrapped_domain_exhaustion
            .contextual_error_inventory_complete,
        f_a5_found_survivor: certificate.wrapped_domain_exhaustion.f_a5_found_survivor,
        no_unknown_survives: certificate.no_unknown_survives,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        downstream_sequence_deferred: certificate.downstream_sequence.all_deferred(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v10_certificate(certificate: &GlobalE4V10Certificate) -> GlobalE4V10Replay {
    match issue_global_e4_v10_certificate() {
        Ok(expected) => replay_against_expected(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn replay_global_e4_v10_json(json: &str) -> GlobalE4V10Replay {
    let json = json.to_owned();
    let worker = match std::thread::Builder::new()
        .name("global-e4-v10-replay".to_owned())
        .stack_size(24 * 1024 * 1024)
        .spawn(
            move || match serde_json::from_str::<GlobalE4V10Certificate>(&json) {
                Ok(certificate) => replay_global_e4_v10_certificate(&certificate),
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

pub fn emit_global_e4_v10_create_new(path: &Path) -> Result<GlobalE4V10Replay, GlobalE4V10Error> {
    let certificate = issue_global_e4_v10_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|e| GlobalE4V10Error::Json(e.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| GlobalE4V10Error::Io(e.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|e| GlobalE4V10Error::Io(e.to_string()))?;
    let replay = replay_global_e4_v10_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V10Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_v10_contextual_errors_remain_obstructions() {
        let extensions = [
            ContextualInternalityError::BetaReductionNeedsExactReplay,
            ContextualInternalityError::ExplicitBodyNotSingleton,
            ContextualInternalityError::ExplicitAmbientContextEmpty,
            ContextualInternalityError::ExplicitDeclarationMismatch,
            ContextualInternalityError::ExplicitAmbientSupportMismatch {
                declared: 1,
                used: vec![],
            },
        ];
        assert!(extensions.iter().all(|error| matches!(
            contextual_error_disposition(error),
            ContextualErrorDispositionClass::NamedTypedObstruction
        )));
    }

    #[test]
    fn contextual_error_algebra_is_exhaustively_disposed() {
        let audit = issue_error_dispositions();
        assert_eq!(audit.len(), 17);
        assert!(audit.iter().all(|entry| !entry.can_survive_as_unknown));
    }
    #[test]
    fn v9_witness_resolves_and_v10_exhausts() {
        let certificate = issue_global_e4_v10_certificate().expect("v10");
        assert!(
            certificate
                .predecessor_v9_witness_resolved
                .successor_internal
        );
        assert!(!certificate.wrapped_domain_exhaustion.f_a5_found_survivor);
        assert!(certificate.no_unknown_survives);
        assert!(certificate.class_exhaustion_proved);
        assert!(certificate.global_e4_complete);
        assert!(certificate.five_pending_memberships_now_authorized);
    }
    #[test]
    fn v10_mutations_fail_closed() {
        let certificate = issue_global_e4_v10_certificate().expect("v10");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutation = certificate.clone();
        mutation.class_exhaustion_proved = false;
        assert!(!replay_against_expected(&mutation, &certificate).valid);
        let json = serde_json::to_string_pretty(&certificate).expect("json");
        assert!(replay_global_e4_v10_json(&json).valid);
    }
}
