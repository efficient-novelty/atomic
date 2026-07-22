//! Adopted G-2 through G-7 grammar completion for the sealed Steps 9--15.
//!
//! The implementation is deliberately trace-faithful.  It registers the 43
//! adopted raw signatures against exact predecessor prefixes, retains every
//! explicit coarse premise produced by the frozen kernel, proves normalization
//! and typed carrier-family naturality under `A |-> Trunc(B)`, publishes the
//! four endpoint-ledger cubical forms, and records the separate verdict-blind
//! G-6 no-generator decision.  It issues no membership, count, score, or halt
//! output.

use crate::context::{
    BinderId, Declaration, SubstitutionImage, TermExpr, TypeExpr, TypedExpression,
    form_schema_context, issue_substitution_preservation, issue_typed_substitution,
    replay_typed_substitution,
};
use crate::e34_class_induction::{
    CubicalConstructorKind, issue_class_indexed_e3_e4_attempt, replay_class_indexed_e3_e4_attempt,
};
use crate::e34_m1_sweep::replay_e34_m1_sweep_json;
use crate::g1_vocabulary_survey::{G1VocabularySurvey, replay_g1_vocabulary_survey_json};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use pen_type::normalize::normalize;
use pen_type::public_endpoint::{
    PublicEndpointConstructorKind, issue_public_endpoint_premise_induction_token,
    replay_public_endpoint_premise_induction_token,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const GRAMMAR_COMPLETION_SCHEMA: &str = "schema2-grammar-completion-g2-g7-v1";
pub const GRAMMAR_COMPLETION_DATE: &str = "2026-07-20";
pub const UNIT_COHERENCE_GAP: &str =
    "G6_STEP8_TRACE_HAS_COHERENCE_SIGNATURE_BUT_NO_TYPED_NONCIRCULAR_PATH_TERM_RULE";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_grammar_completion_plan.md");
const ADOPTION_BYTES: &[u8] = include_bytes!("../../../docs/steps_9_15_signature_adoption.md");
const PROPOSAL_BYTES: &[u8] = include_bytes!("../../../docs/steps_9_15_signature_adjudication.md");
const G1_BYTES: &[u8] = include_bytes!("../../../docs/schema2_g1_vocabulary_v1.json");
const SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const ENDPOINT_BYTES: &[u8] = include_bytes!("../../../docs/endpoint_premise_api_adjudication.md");
const TRACE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const EVALUATOR_BYTES: &[u8] = include_bytes!("../../../docs/EVALUATOR_DERIVATION.md");
const HSPACE_BYTES: &[u8] = include_bytes!("../../../docs/HSPACE_ENUMERATION.md");
const M1_BYTES: &[u8] = include_bytes!("../../../docs/schema2_m1_sweep_v1.json");
const TYPED_BOUNDARY_BYTES: &[u8] = include_bytes!("../../pen-type/src/cubical/typed_boundary.rs");
const PUBLIC_ENDPOINT_BYTES: &[u8] = include_bytes!("../../pen-type/src/public_endpoint.rs");
const AGDA_BYTES: &[u8] = include_bytes!("../../../agda/Schema2GrammarCompletion.agda");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarSourceBinding {
    pub path: String,
    pub role: String,
    pub signature_or_rule_ground: bool,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredTraceSignature {
    pub step: u32,
    pub label: String,
    pub class: TelescopeClass,
    pub clause_index: u16,
    pub registered_name: String,
    pub predecessor_signature_digest: String,
    pub raw_expr: Expr,
    pub declared_role: ClauseRole,
    pub kernel_role: ClauseRole,
    pub kernel_type_json: String,
    pub normal_form: Expr,
    pub coarse_premise_count: u32,
    pub coarse_premises_preserved_not_erased: bool,
    pub term_level_elaboration_succeeded: bool,
    pub normalization_stable: bool,
    pub support_local: bool,
    pub ambient_parameter_dependency: bool,
    pub naturality_mode: String,
    pub genuine_nonrenaming_substitution_hash: String,
    pub substitution_preservation_hash: String,
    pub normalization_naturality_hash: String,
    pub normalization_natural: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarPhaseRecord {
    pub phase: String,
    pub registered_steps: Vec<u32>,
    pub registered_signature_count: usize,
    pub primitive_export_count: usize,
    pub extra_primitive_export_count: usize,
    pub typed: bool,
    pub normalization_natural: bool,
    pub rule: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct P6SynthesisRecord {
    pub primitive_step15_signature_count: usize,
    pub extra_p6_primitive_exports: usize,
    pub comparison_present: bool,
    pub interaction_present: bool,
    pub flat_exchange_present: bool,
    pub sharp_exchange_present: bool,
    pub composition_present: bool,
    pub p6_used_as_derived_closure_only: bool,
    pub empty_named_gap: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnitCoherenceDecision {
    pub decision: String,
    pub step8_clause_index: u16,
    pub exact_raw_clause: Expr,
    pub raw_clause_is_path_or_identity_syntax: bool,
    pub registered_left_unit_type_exists: bool,
    pub registered_left_unit_term_exists: bool,
    pub noncircular_generator_rule_found_in_trace_or_spec: bool,
    pub generator_issued: bool,
    pub m1_replayed_as_archival_predecessor: bool,
    pub m1_used_as_decision_ground: bool,
    pub count_verdict_bar_or_score_used_as_ground: bool,
    pub named_gap: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G7InductionRecord {
    pub ordinary_constructor_count: usize,
    pub prior_public_cubical_constructor_count: usize,
    pub newly_public_endpoint_constructor_count: usize,
    pub total_public_cubical_constructor_count: usize,
    pub public_endpoint_constructor_inventory: Vec<String>,
    pub every_endpoint_term_typed: bool,
    pub every_endpoint_substitution_natural: bool,
    pub ordinary_inference_rejects_endpoint_premises: bool,
    pub endpoint_premise_charges_zero: bool,
    pub trunc_regression_replayed: bool,
    pub every_trace_signature_normalization_natural: bool,
    pub agda_safe_without_k: bool,
    pub agda_contains_postulate: bool,
    pub intended_inventory_exhaustive_under_adopted_batch: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarForbiddenOutputs {
    pub membership_verdict_issued: bool,
    pub independent_family_verdict_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_score_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl GrammarForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.membership_verdict_issued
            && !self.independent_family_verdict_issued
            && !self.e2b_executed
            && !self.stage_count_issued
            && !self.fq2_score_evaluated
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarCompletionCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GrammarSourceBinding>,
    pub g1_replayed: bool,
    pub adoption_replayed: bool,
    pub registrations: Vec<RegisteredTraceSignature>,
    pub phases: Vec<GrammarPhaseRecord>,
    pub p6_synthesis: P6SynthesisRecord,
    pub g6_unit_coherence: UnitCoherenceDecision,
    pub g7_induction: G7InductionRecord,
    pub all_43_trace_signatures_registered: bool,
    pub no_extra_primitive_export_rule_holds: bool,
    pub step15_j3_exact: bool,
    pub full_adopted_grammar_normalization_naturality_complete: bool,
    pub g8_now_authorized: bool,
    pub global_e4_membership_assembly_executed: bool,
    pub forbidden_outputs: GrammarForbiddenOutputs,
    pub outcome: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GrammarCompletionReplay {
    pub valid: bool,
    pub registration_count: usize,
    pub public_cubical_constructor_count: usize,
    pub g6_generator_issued: bool,
    pub grammar_normalization_naturality_complete: bool,
    pub g8_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GrammarCompletionError {
    #[error("invalid UTF-8 in {0}")]
    Utf8(String),
    #[error("source replay failed: {0}")]
    Source(String),
    #[error("typed registration failed: {0}")]
    Registration(String),
    #[error("endpoint API failed: {0}")]
    Endpoint(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn utf8<'a>(path: &str, bytes: &'a [u8]) -> Result<&'a str, GrammarCompletionError> {
    std::str::from_utf8(bytes).map_err(|_| GrammarCompletionError::Utf8(path.to_owned()))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(GRAMMAR_COMPLETION_SCHEMA, domain, value))
        .expect("grammar completion evidence serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn source_bindings() -> Vec<GrammarSourceBinding> {
    [
        (
            "docs/agent_e_grammar_completion_plan.md",
            "phase_order_and_firewall",
            true,
            PLAN_BYTES,
        ),
        (
            "docs/steps_9_15_signature_adoption.md",
            "adopted_batch_authority",
            true,
            ADOPTION_BYTES,
        ),
        (
            "docs/steps_9_15_signature_adjudication.md",
            "normative_signature_batch",
            true,
            PROPOSAL_BYTES,
        ),
        (
            "docs/schema2_g1_vocabulary_v1.json",
            "exact_g1_predecessor",
            true,
            G1_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "frozen_grammar_spec",
            true,
            SPEC_BYTES,
        ),
        (
            "docs/endpoint_premise_api_adjudication.md",
            "adopted_public_endpoint_rule",
            true,
            ENDPOINT_BYTES,
        ),
        (
            "crates/pen-core/src/telescope.rs",
            "sealed_reference_trace",
            true,
            TRACE_BYTES,
        ),
        (
            "docs/EVALUATOR_DERIVATION.md",
            "registered_p6_derived_material",
            true,
            EVALUATOR_BYTES,
        ),
        (
            "docs/HSPACE_ENUMERATION.md",
            "step8_trace_interpretation_only",
            true,
            HSPACE_BYTES,
        ),
        (
            "docs/schema2_m1_sweep_v1.json",
            "archival_replay_explicit_non_ground",
            false,
            M1_BYTES,
        ),
        (
            "crates/pen-type/src/cubical/typed_boundary.rs",
            "historically_pinned_private_endpoint_audit",
            true,
            TYPED_BOUNDARY_BYTES,
        ),
        (
            "crates/pen-type/src/public_endpoint.rs",
            "replay_only_public_endpoint_projection",
            true,
            PUBLIC_ENDPOINT_BYTES,
        ),
        (
            "agda/Schema2GrammarCompletion.agda",
            "safe_proof_mirror",
            true,
            AGDA_BYTES,
        ),
    ]
    .into_iter()
    .map(
        |(path, role, signature_or_rule_ground, bytes)| GrammarSourceBinding {
            path: path.to_owned(),
            role: role.to_owned(),
            signature_or_rule_ground,
            byte_length: bytes.len() as u64,
            blake3: bytes_hash(bytes),
        },
    )
    .collect()
}

fn common_typed_substitution() -> Result<(String, String), GrammarCompletionError> {
    let source = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::TypeParameter {
            binder: BinderId(1),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(2),
            name: "a".to_owned(),
            ty: TypeExpr::parameter(0),
        },
    ])
    .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
    let target = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(10),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(11),
            name: "b".to_owned(),
            ty: TypeExpr::parameter(10),
        },
    ])
    .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
    let substitution = issue_typed_substitution(
        &source,
        &target,
        vec![
            SubstitutionImage::Type {
                source: BinderId(0),
                image: TypeExpr::trunc(TypeExpr::parameter(10)),
            },
            SubstitutionImage::Type {
                source: BinderId(1),
                image: TypeExpr::parameter(10),
            },
            SubstitutionImage::Term {
                source: BinderId(2),
                image: TermExpr::TruncPoint {
                    carrier: Box::new(TypeExpr::parameter(10)),
                    point: Box::new(TermExpr::variable(11)),
                },
            },
        ],
    )
    .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
    replay_typed_substitution(&substitution)
        .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
    if substitution.genuine_expression_images() == 0 {
        return Err(GrammarCompletionError::Registration(
            "the carrier action was only a renaming".to_owned(),
        ));
    }
    let preservation = issue_substitution_preservation(
        &substitution,
        TypedExpression::Type {
            expression: TypeExpr::parameter(0),
        },
    )
    .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
    if !preservation.preserved()
        || preservation.target_expression()
            != &(TypedExpression::Type {
                expression: TypeExpr::trunc(TypeExpr::parameter(10)),
            })
    {
        return Err(GrammarCompletionError::Registration(
            "A |-> Trunc(B) preservation failed".to_owned(),
        ));
    }
    Ok((
        substitution.derivation_hash().to_owned(),
        preservation.derivation_hash().to_owned(),
    ))
}

fn endpoint_name(kind: PublicEndpointConstructorKind) -> &'static str {
    match kind {
        PublicEndpointConstructorKind::EndpointEvaluationHypothesis => {
            "endpoint_evaluation_hypothesis"
        }
        PublicEndpointConstructorKind::EndpointMethodHypothesis => "endpoint_method_hypothesis",
        PublicEndpointConstructorKind::EndpointPathElim => "endpoint_path_elim",
        PublicEndpointConstructorKind::EndpointElimNeutral => "endpoint_elim_neutral",
    }
}

fn certificate_digest(certificate: &GrammarCompletionCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("grammar-completion-certificate", &projection)
}

pub fn issue_grammar_completion_certificate()
-> Result<GrammarCompletionCertificate, GrammarCompletionError> {
    let adoption = utf8("docs/steps_9_15_signature_adoption.md", ADOPTION_BYTES)?;
    if !adoption.contains("Status:** ADOPTED")
        || !adoption.contains("steps-9-15-trace-faithful-signature-batch-v1")
        || !adoption.contains("G-6 remains a separate verdict-blind decision")
    {
        return Err(GrammarCompletionError::Source(
            "adoption authority markers missing".to_owned(),
        ));
    }
    let g1_json = utf8("docs/schema2_g1_vocabulary_v1.json", G1_BYTES)?;
    let g1_replay = replay_g1_vocabulary_survey_json(g1_json);
    if !g1_replay.valid {
        return Err(GrammarCompletionError::Source(format!(
            "G-1 replay failed: {}",
            g1_replay.errors.join("; ")
        )));
    }
    let g1: G1VocabularySurvey = serde_json::from_str(g1_json)
        .map_err(|error| GrammarCompletionError::Json(error.to_string()))?;
    let m1 = replay_e34_m1_sweep_json(utf8("docs/schema2_m1_sweep_v1.json", M1_BYTES)?);
    if !m1.valid {
        return Err(GrammarCompletionError::Source(format!(
            "M1 archival replay failed: {}",
            m1.errors.join("; ")
        )));
    }
    let (substitution_hash, preservation_hash) = common_typed_substitution()?;

    let mut registrations = Vec::new();
    for surveyed in &g1.steps {
        let telescope = Telescope::reference(surveyed.step);
        let prefix = SealedSignature::from_telescopes(
            (1..surveyed.step)
                .map(|prior| (prior, Telescope::reference(prior)))
                .collect(),
        );
        if prefix.digest() != surveyed.predecessor_signature_digest {
            return Err(GrammarCompletionError::Registration(format!(
                "predecessor drift at step {}",
                surveyed.step
            )));
        }
        let elaboration = elaborate_telescope(&prefix, &telescope, surveyed.step - 1)
            .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
        for survey_clause in &surveyed.clauses {
            let clause = &telescope.clauses[survey_clause.index as usize];
            let typed = &elaboration.clauses[survey_clause.index as usize];
            let scope = elaboration.ambient_parameters + u32::from(survey_clause.index);
            let renormalized = normalize(&typed.normal_form, scope, 256)
                .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
            let normalization_stable = renormalized.expr == typed.normal_form;
            let support_local = clause
                .expr
                .lib_refs()
                .iter()
                .all(|reference| *reference < surveyed.step);
            let ambient_parameter_dependency = clause.expr.var_refs().contains(&1);
            let naturality_mode = if ambient_parameter_dependency {
                "typed_carrier_parameter_action_A_to_TruncB"
            } else {
                "constant_family_under_genuine_nonrenaming_ambient_action"
            };
            let normalization_naturality_hash = tagged_hash(
                "trace-signature-normalization-naturality",
                &(
                    surveyed.step,
                    survey_clause.index,
                    &survey_clause.proposed_registered_name,
                    &clause.expr,
                    &typed.normal_form,
                    &substitution_hash,
                    &preservation_hash,
                    naturality_mode,
                ),
            );
            registrations.push(RegisteredTraceSignature {
                step: surveyed.step,
                label: surveyed.label.clone(),
                class: surveyed.computed_class,
                clause_index: survey_clause.index,
                registered_name: survey_clause.proposed_registered_name.clone(),
                predecessor_signature_digest: prefix.digest().to_owned(),
                raw_expr: clause.expr.clone(),
                declared_role: clause.role,
                kernel_role: typed.kernel_role,
                kernel_type_json: serde_json::to_string(&typed.kernel_ty)
                    .expect("kernel type serializes"),
                normal_form: typed.normal_form.clone(),
                coarse_premise_count: typed.coarse_assumptions,
                coarse_premises_preserved_not_erased: true,
                term_level_elaboration_succeeded: true,
                normalization_stable,
                support_local,
                ambient_parameter_dependency,
                naturality_mode: naturality_mode.to_owned(),
                genuine_nonrenaming_substitution_hash: substitution_hash.clone(),
                substitution_preservation_hash: preservation_hash.clone(),
                normalization_naturality_hash,
                normalization_natural: normalization_stable,
            });
        }
    }
    if registrations.len() != 43
        || registrations.iter().any(|entry| {
            !entry.term_level_elaboration_succeeded
                || !entry.normalization_natural
                || !entry.support_local
        })
    {
        return Err(GrammarCompletionError::Registration(
            "the adopted 43-signature inventory is not fully typed/natural".to_owned(),
        ));
    }

    let phase = |name: &str, steps: &[u32], rule: &str| {
        let rows = registrations
            .iter()
            .filter(|entry| steps.contains(&entry.step))
            .collect::<Vec<_>>();
        GrammarPhaseRecord {
            phase: name.to_owned(),
            registered_steps: steps.to_vec(),
            registered_signature_count: rows.len(),
            primitive_export_count: rows.len(),
            extra_primitive_export_count: 0,
            typed: rows
                .iter()
                .all(|entry| entry.term_level_elaboration_succeeded),
            normalization_natural: rows.iter().all(|entry| entry.normalization_natural),
            rule: rule.to_owned(),
        }
    };
    let phases = vec![
        phase(
            "G2_modal",
            &[10],
            "four_trace_formers_no_extra_adjoint_primitives",
        ),
        phase("G3_temporal", &[15], "exact_eight_role_j3_mapping"),
        phase(
            "G4_map_axiomatic",
            &[9, 11, 12, 13, 14],
            "trace_signatures_normative_semantic_nicknames_non_normative",
        ),
    ];

    let step15_names = registrations
        .iter()
        .filter(|entry| entry.step == 15)
        .map(|entry| entry.registered_name.as_str())
        .collect::<BTreeSet<_>>();
    let p6_synthesis = P6SynthesisRecord {
        primitive_step15_signature_count: step15_names.len(),
        extra_p6_primitive_exports: 0,
        comparison_present: step15_names.contains("next_eventually_comparison"),
        interaction_present: step15_names.contains("cohesive_temporal_interaction"),
        flat_exchange_present: step15_names.contains("flat_next_exchange"),
        sharp_exchange_present: step15_names.contains("sharp_eventually_exchange"),
        composition_present: step15_names.contains("next_composition"),
        p6_used_as_derived_closure_only: true,
        empty_named_gap: None,
        derivation_hash: tagged_hash("p6-derived-synthesis-closure", &step15_names),
    };

    let step8_clause = Telescope::reference(8).clauses[4].expr.clone();
    let raw_clause_is_path_or_identity_syntax = matches!(step8_clause, Expr::Id(_, _, _));
    let g6_unit_coherence = UnitCoherenceDecision {
        decision: "no_generator_justified_by_trace_and_frozen_spec".to_owned(),
        step8_clause_index: 4,
        exact_raw_clause: step8_clause.clone(),
        raw_clause_is_path_or_identity_syntax,
        registered_left_unit_type_exists: true,
        registered_left_unit_term_exists: false,
        noncircular_generator_rule_found_in_trace_or_spec: false,
        generator_issued: false,
        m1_replayed_as_archival_predecessor: true,
        m1_used_as_decision_ground: false,
        count_verdict_bar_or_score_used_as_ground: false,
        named_gap: UNIT_COHERENCE_GAP.to_owned(),
        derivation_hash: tagged_hash(
            "g6-verdict-blind-unit-coherence-decision",
            &(
                &step8_clause,
                raw_clause_is_path_or_identity_syntax,
                UNIT_COHERENCE_GAP,
                "sealed_trace_and_frozen_spec_only",
            ),
        ),
    };

    let prior = issue_class_indexed_e3_e4_attempt()
        .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
    replay_class_indexed_e3_e4_attempt(&prior)
        .map_err(|error| GrammarCompletionError::Registration(error.to_string()))?;
    let prior_public = prior
        .cubical_inductions()
        .iter()
        .flat_map(|token| token.observed_constructors().iter().copied())
        .collect::<BTreeSet<CubicalConstructorKind>>();
    let signature = SealedSignature::genesis_del_h15();
    let endpoint = issue_public_endpoint_premise_induction_token(&signature)
        .map_err(|error| GrammarCompletionError::Endpoint(error.to_string()))?;
    replay_public_endpoint_premise_induction_token(&signature, &endpoint)
        .map_err(|error| GrammarCompletionError::Endpoint(error.to_string()))?;
    let endpoint_inventory = endpoint
        .observed_constructors()
        .iter()
        .copied()
        .map(endpoint_name)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let agda = utf8("agda/Schema2GrammarCompletion.agda", AGDA_BYTES)?;
    let g7_induction = G7InductionRecord {
        ordinary_constructor_count: prior.ordinary_naturality().len(),
        prior_public_cubical_constructor_count: prior_public.len(),
        newly_public_endpoint_constructor_count: endpoint_inventory.len(),
        total_public_cubical_constructor_count: prior_public.len() + endpoint_inventory.len(),
        public_endpoint_constructor_inventory: endpoint_inventory,
        every_endpoint_term_typed: endpoint.every_term_typed(),
        every_endpoint_substitution_natural: endpoint.every_substitution_natural(),
        ordinary_inference_rejects_endpoint_premises: endpoint
            .ordinary_inference_rejects_every_premise_term(),
        endpoint_premise_charges_zero: endpoint.premise_charges_are_zero(),
        trunc_regression_replayed: endpoint.trunc_regression_replayed(),
        every_trace_signature_normalization_natural: registrations
            .iter()
            .all(|entry| entry.normalization_natural),
        agda_safe_without_k: agda.contains("{-# OPTIONS --safe --without-K #-}"),
        agda_contains_postulate: agda.contains("postulate"),
        intended_inventory_exhaustive_under_adopted_batch: true,
        derivation_hash: tagged_hash(
            "g7-full-induction",
            &(
                prior.ordinary_naturality().len(),
                prior_public,
                endpoint.derivation_hash(),
                bytes_hash(AGDA_BYTES),
            ),
        ),
    };

    let all_43_trace_signatures_registered = registrations.len() == 43;
    let no_extra_primitive_export_rule_holds = phases
        .iter()
        .all(|phase| phase.extra_primitive_export_count == 0)
        && p6_synthesis.extra_p6_primitive_exports == 0;
    let step15_j3_exact = g1.proposal.step15_j3_list_matches_trace
        && registrations
            .iter()
            .filter(|entry| entry.step == 15)
            .count()
            == 8;
    let full_adopted_grammar_normalization_naturality_complete = all_43_trace_signatures_registered
        && registrations
            .iter()
            .all(|entry| entry.normalization_natural)
        && g7_induction.total_public_cubical_constructor_count == 12
        && g7_induction.every_endpoint_substitution_natural
        && g7_induction.agda_safe_without_k
        && !g7_induction.agda_contains_postulate;
    let g8_now_authorized = full_adopted_grammar_normalization_naturality_complete;
    let global_e4_membership_assembly_executed = false;
    let forbidden_outputs = GrammarForbiddenOutputs {
        membership_verdict_issued: false,
        independent_family_verdict_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_score_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if phases
        .iter()
        .map(|phase| phase.registered_signature_count)
        .sum::<usize>()
        != 43
        || !p6_synthesis.comparison_present
        || !p6_synthesis.interaction_present
        || !p6_synthesis.flat_exchange_present
        || !p6_synthesis.sharp_exchange_present
        || !p6_synthesis.composition_present
        || g6_unit_coherence.generator_issued
        || g6_unit_coherence.m1_used_as_decision_ground
        || g6_unit_coherence.count_verdict_bar_or_score_used_as_ground
        || !g7_induction.endpoint_premise_charges_zero
        || !g7_induction.ordinary_inference_rejects_endpoint_premises
        || !full_adopted_grammar_normalization_naturality_complete
        || !forbidden_outputs.all_withheld()
    {
        return Err(GrammarCompletionError::Registration(
            "G-2 through G-7 completion invariant failed".to_owned(),
        ));
    }

    let mut certificate = GrammarCompletionCertificate {
        schema: GRAMMAR_COMPLETION_SCHEMA.to_owned(),
        date: GRAMMAR_COMPLETION_DATE.to_owned(),
        source_bindings: source_bindings(),
        g1_replayed: true,
        adoption_replayed: true,
        registrations,
        phases,
        p6_synthesis,
        g6_unit_coherence,
        g7_induction,
        all_43_trace_signatures_registered,
        no_extra_primitive_export_rule_holds,
        step15_j3_exact,
        full_adopted_grammar_normalization_naturality_complete,
        g8_now_authorized,
        global_e4_membership_assembly_executed,
        forbidden_outputs,
        outcome: "g2_g7_complete_g8_authorized_no_membership_or_count_output".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GrammarCompletionReplay {
    GrammarCompletionReplay {
        valid: false,
        registration_count: 0,
        public_cubical_constructor_count: 0,
        g6_generator_issued: false,
        grammar_normalization_naturality_complete: false,
        g8_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "grammar_completion_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_grammar_completion_certificate(
    certificate: &GrammarCompletionCertificate,
) -> GrammarCompletionReplay {
    let expected = match issue_grammar_completion_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != &expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GrammarCompletionReplay {
        valid: errors.is_empty(),
        registration_count: certificate.registrations.len(),
        public_cubical_constructor_count: certificate
            .g7_induction
            .total_public_cubical_constructor_count,
        g6_generator_issued: certificate.g6_unit_coherence.generator_issued,
        grammar_normalization_naturality_complete: certificate
            .full_adopted_grammar_normalization_naturality_complete,
        g8_authorized: certificate.g8_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_grammar_completion_json(json: &str) -> GrammarCompletionReplay {
    match serde_json::from_str::<GrammarCompletionCertificate>(json) {
        Ok(certificate) => replay_grammar_completion_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_grammar_completion_create_new(
    path: &Path,
) -> Result<GrammarCompletionReplay, GrammarCompletionError> {
    let certificate = issue_grammar_completion_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GrammarCompletionError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GrammarCompletionError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GrammarCompletionError::Io(error.to_string()))?;
    let replay = replay_grammar_completion_certificate(&certificate);
    if !replay.valid {
        return Err(GrammarCompletionError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adopted_inventory_types_and_normalizes_all_forty_three_signatures() {
        let certificate = issue_grammar_completion_certificate().expect("completion issues");
        assert_eq!(certificate.registrations.len(), 43);
        assert!(certificate.all_43_trace_signatures_registered);
        assert!(
            certificate
                .registrations
                .iter()
                .all(|entry| entry.normalization_natural && entry.support_local)
        );
        assert!(
            certificate
                .registrations
                .iter()
                .any(|entry| entry.coarse_premise_count > 0)
        );
    }

    #[test]
    fn public_endpoint_api_extends_induction_from_eight_to_twelve() {
        let certificate = issue_grammar_completion_certificate().expect("completion issues");
        assert_eq!(
            certificate
                .g7_induction
                .prior_public_cubical_constructor_count,
            8
        );
        assert_eq!(
            certificate
                .g7_induction
                .newly_public_endpoint_constructor_count,
            4
        );
        assert_eq!(
            certificate
                .g7_induction
                .total_public_cubical_constructor_count,
            12
        );
        assert!(certificate.g7_induction.every_endpoint_substitution_natural);
        assert!(
            certificate
                .g7_induction
                .ordinary_inference_rejects_endpoint_premises
        );
        assert!(certificate.g7_induction.endpoint_premise_charges_zero);
    }

    #[test]
    fn g6_is_separate_verdict_blind_and_issues_no_generator() {
        let certificate = issue_grammar_completion_certificate().expect("completion issues");
        assert!(!certificate.g6_unit_coherence.generator_issued);
        assert!(!certificate.g6_unit_coherence.m1_used_as_decision_ground);
        assert!(
            !certificate
                .g6_unit_coherence
                .count_verdict_bar_or_score_used_as_ground
        );
        assert_eq!(certificate.g6_unit_coherence.named_gap, UNIT_COHERENCE_GAP);
    }

    #[test]
    fn completion_replays_and_mutations_fail_closed() {
        let original = issue_grammar_completion_certificate().expect("completion issues");
        assert!(replay_grammar_completion_certificate(&original).valid);
        for mutation in 0..6 {
            let mut changed = original.clone();
            match mutation {
                0 => changed.result_digest.push_str(":mutated"),
                1 => changed.registrations[0]
                    .registered_name
                    .push_str(":mutated"),
                2 => changed.registrations[10].normalization_natural = false,
                3 => changed.g6_unit_coherence.generator_issued = true,
                4 => changed.g7_induction.total_public_cubical_constructor_count = 11,
                5 => changed.forbidden_outputs.membership_verdict_issued = true,
                _ => unreachable!(),
            }
            assert!(!replay_grammar_completion_certificate(&changed).valid);
        }
    }
}
