//! T-SM1a: genuine contextual-Formation `Internal` evidence.
//!
//! This is an additive successor to the negative v3 chronological audit.  It
//! does not modify or reinterpret any v2/v3 evidence.  The theorem proved here
//! is deliberately narrower than "well typed implies Internal":
//!
//! > An exact sealed Formation family is zero-credit `Internal` when its
//! > canonical open judgment is reconstructed from a formable dependent
//! > ambient context solely by the already-adopted transparent-former rules;
//! > and every exact chronological identity-tail specialization is `Internal`
//! > when the same constructor derivation reissues for the structural
//! > substitution result.
//!
//! The evidence is a recursive constructor derivation, not a typing/scoping
//! flag.  Ambient hypotheses, local binders, sealed constants, every premise,
//! the kernel rule at every node, the dependent declaration, normalization,
//! and the post-substitution derivation all replay.  Candidate-fresh
//! Formation is not admitted: the public source issuer accepts only an exact
//! public family from the sealed predecessor signature, and the public
//! specialization issuer accepts only the frozen chronological slot map.

use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3ChronologicalInterfaceSlotMap, A3DemandOutputType,
    A3RuleConstructor, A3TypedClauseSource, generate_a3_window_for_exact_prefix_unbounded,
    replay_chronological_interface_slot_map,
};
use pen_eval::typed_families::ParamSort;
use pen_search::chronological_slot_map_v3::{
    CHRONOLOGICAL_FORMATION_GAP_V3, replay_chronological_slot_map_v3_json,
};
use pen_search::contextual_formation_coherence_v3::{
    SealedNaturalFamilyProjectionV3, issue_sealed_natural_family_v3,
    kernel_context_from_parameter_sorts, replay_sealed_natural_family_v3,
};
use pen_type::ambient_former_internality::{TransparentFormer, registered_transparent_formers};
use pen_type::contextual_internality::ContextualMotive;
use pen_type::dependent_context::{
    DependentAmbientContextDeclarationProjection, DependentContextMotive,
    issue_dependent_ambient_context_declaration, replay_dependent_ambient_context_declaration,
};
use pen_type::elaborate::{
    DerivationNode, KernelTy, SealedSignature, SingleClauseElaboration, candidate_hash,
    elaborate_single_clause_with_typed_ambient,
};
use pen_type::equality::{EqualityWitness, univalent_equality};
use pen_type::motive_parametric_coherence_v2::KernelTyProjectionV2;
use pen_type::normalize::{normalize, substitute_level, whnf};
use pen_type::substitution::{
    ParameterSort, SortedParameterContext, SubstitutionImage, issue_structural_substitution,
    replay_structural_substitution,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION: &str =
    "t-sm1a-genuine-contextual-formation-internal-v4";
pub const T_SM1A_THEOREM_STATEMENT: &str =
    "an exact sealed contextual Formation assembled only from formable hypotheses, sealed constants, local binders, and registered transparent formers is zero-credit Internal; exact chronological identity-tail substitution preserves that Internal derivation by reissuance, without admitting candidate-fresh Formation";
pub const T_SM1A_EXPECTED_CASES: usize = 54;
const NORMALIZATION_FUEL: u32 = 512;

const NU_REGISTER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/nu_register_adjudication.md");
const CONTEXTUAL_INTERNALITY_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/contextual_internality_adjudication.md");
const MOTIVE_COHERENCE_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/motive_parametric_coherence_adjudication.md");
const DEPENDENT_CONTEXT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/dependent_context_adjudication.md");
const FROZEN_V3_THEOREM_SOURCE_BYTES: &[u8] =
    include_bytes!("contextual_formation_coherence_v3.rs");
const FROZEN_V3_WRAPPER_SOURCE_BYTES: &[u8] = include_bytes!("chronological_slot_map_v3.rs");
const FROZEN_V3_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/chronological_interface_slot_map_v3.json");

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "provenance")]
pub enum ContextualFormationTermProvenanceV4 {
    AmbientHypothesis {
        parameter: u32,
        classifier: KernelTyProjectionV2,
        declaration_hash: String,
    },
    LocalBinder {
        binder: u32,
        classifier: KernelTyProjectionV2,
    },
    AmbientUniverse,
    SealedLibraryConstant {
        step: u32,
    },
    TransparentFormer {
        former: TransparentFormer,
        premise_hashes: Vec<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualFormationTermDerivationV4 {
    pub expression: Expr,
    pub scope_len: u32,
    pub derivation_rule: String,
    pub kernel_type: KernelTyProjectionV2,
    pub coarse_kernel_step: bool,
    pub former: TransparentFormer,
    pub provenance: ContextualFormationTermProvenanceV4,
    pub premises: Vec<ContextualFormationTermDerivationV4>,
    pub normal_form: Expr,
    pub normalization_steps: u32,
    pub exact_kernel_rule_replayed: bool,
    pub registered_transparent_former: bool,
    pub every_premise_internal: bool,
    pub no_candidate_fresh_head: bool,
    pub full_provenance_retained: bool,
    pub independent_demand_orbit_exported: bool,
    pub internal_term_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualFormationBodyDerivationV4 {
    pub version: String,
    pub theorem_statement: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate: Telescope,
    pub candidate_hash: String,
    pub declared_context: Vec<KernelTyProjectionV2>,
    pub dependent_context: DependentAmbientContextDeclarationProjection,
    pub clause_index: u16,
    pub declared_role: ClauseRole,
    pub expression: Expr,
    pub normal_form: Expr,
    pub kernel_type: KernelTyProjectionV2,
    pub inferred_motive: ContextualMotive,
    pub kernel_coarse_assumptions: u32,
    pub term_evidence: ContextualFormationTermDerivationV4,
    pub dependent_context_replayed: bool,
    pub exact_kernel_typing_replayed: bool,
    pub every_transparent_premise_replayed: bool,
    pub candidate_fresh_formation_admitted: bool,
    pub independent_demand_orbit_exported: bool,
    pub internal_closure_issued: bool,
    pub marginal_kappa: u32,
    pub marginal_nu: u32,
    pub anchors_minted: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SealedContextualFormationInternalProjectionV4 {
    pub version: String,
    pub sealed_family: SealedNaturalFamilyProjectionV3,
    pub body_derivation: ContextualFormationBodyDerivationV4,
    pub exact_sealed_source_replayed: bool,
    pub sealed_source_is_contextual_formation: bool,
    pub zero_credit: bool,
    pub internal_closure_issued: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedContextualFormationInternalTokenV4 {
    projection: SealedContextualFormationInternalProjectionV4,
}

impl SealedContextualFormationInternalTokenV4 {
    pub fn projection(&self) -> &SealedContextualFormationInternalProjectionV4 {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "origin")]
pub enum ChronologicalFirstImageEvidenceV4 {
    DirectSealedTypeFamily {
        evidence: SealedNaturalFamilyProjectionV3,
    },
    PointwiseSealedTypeValuedFamily {
        evidence: SealedNaturalFamilyProjectionV3,
        argument_parameter: u32,
    },
}

/// Reusable Internal evidence for the raw first image of a frozen
/// chronological substitution.  In the pointwise case `raw_expression` is
/// intentionally the registered beta-redex; `normal_form` and
/// `raw_to_normal_equality` prove its reduction without replacing it.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalImageInternalProjectionV4 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source: A3TypedClauseSource,
    pub source_evidence: SealedNaturalFamilyProjectionV3,
    pub interface_mode: A3ChronologicalInterfaceMode,
    pub target_context: Vec<KernelTyProjectionV2>,
    pub raw_expression: Expr,
    pub raw_expression_role: ClauseRole,
    pub normal_form: Expr,
    pub raw_to_normal_equality: EqualityWitness,
    pub body_derivation: ContextualFormationBodyDerivationV4,
    pub exact_sealed_source_replayed: bool,
    pub raw_registered_expression_retained: bool,
    pub exact_beta_child_replayed: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ChronologicalImageInternalTokenV4 {
    projection: ChronologicalImageInternalProjectionV4,
}

impl ChronologicalImageInternalTokenV4 {
    pub fn projection(&self) -> &ChronologicalImageInternalProjectionV4 {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactContextualImageTypingV4 {
    pub source_parameter: u32,
    pub expected_classifier: KernelTyProjectionV2,
    pub term: Expr,
    pub target_context: Vec<KernelTyProjectionV2>,
    pub elaboration: SingleClauseElaboration,
    pub derivation: DerivationNode,
    pub exact_classifier_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalFormationSpecializationProjectionV4 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source: SealedContextualFormationInternalProjectionV4,
    pub older_source: A3TypedClauseSource,
    pub first_image_evidence: ChronologicalFirstImageEvidenceV4,
    pub interface_mode: A3ChronologicalInterfaceMode,
    pub slot_map: A3ChronologicalInterfaceSlotMap,
    pub source_context: Vec<KernelTyProjectionV2>,
    pub target_context: Vec<KernelTyProjectionV2>,
    pub images: Vec<SubstitutionImage>,
    pub image_typings: Vec<ExactContextualImageTypingV4>,
    pub every_image_exactly_typed: bool,
    pub ordered_identity_tail_exact: bool,
    pub no_permutation_or_instance_override: bool,
    pub substitution_body: Expr,
    pub substitution_result: Expr,
    pub substitution_derivation_hash: String,
    pub structural_substitution_replayed: bool,
    pub source_normal_form_substitution_result: Expr,
    pub normalization_commutes: EqualityWitness,
    pub specialized_internal: ContextualFormationBodyDerivationV4,
    pub post_substitution_internal_replayed: bool,
    pub exact_open_substitution_result_replayed: bool,
    pub candidate_fresh_formation_admitted: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ChronologicalFormationSpecializationTokenV4 {
    projection: ChronologicalFormationSpecializationProjectionV4,
}

impl ChronologicalFormationSpecializationTokenV4 {
    pub fn projection(&self) -> &ChronologicalFormationSpecializationProjectionV4 {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TSm1aNamedGapV4 {
    pub gap_id: String,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TSm1aCaseAuditV4 {
    pub instance_id: String,
    pub older_step: u32,
    pub older_clause: u16,
    pub newest_step: u32,
    pub newest_clause: u16,
    pub source_derivation_hash: Option<String>,
    pub specialization_derivation_hash: Option<String>,
    pub exact_open_substitution_result_replayed: bool,
    pub post_substitution_internal_replayed: bool,
    pub replayed: bool,
    pub gap: Option<TSm1aNamedGapV4>,
    pub row_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TSm1aAuditV4 {
    pub version: String,
    pub theorem_statement: String,
    pub adjudication_bindings: BTreeMap<String, String>,
    pub frozen_v3_source_bindings: BTreeMap<String, String>,
    pub frozen_v3_artifact_digest: String,
    pub frozen_v3_artifact_replayed: bool,
    pub live_case_ids_fixed_before_v3_comparison: bool,
    pub live_case_count: usize,
    pub live_case_ids_unique: bool,
    pub exact_v3_formation_gap_surface_recovered: bool,
    pub source_formation_coordinates: Vec<(u32, u16)>,
    pub older_interface_coordinates: Vec<(u32, u16)>,
    pub cases: Vec<TSm1aCaseAuditV4>,
    pub derived_count: usize,
    pub named_gap_count: usize,
    pub named_gap_counts: BTreeMap<String, usize>,
    pub every_source_context_replayed: bool,
    pub every_post_substitution_internal_replayed: bool,
    pub every_exact_open_substitution_result_replayed: bool,
    pub candidate_fresh_formation_admitted: bool,
    pub t_sm1a_passed: bool,
    pub fixed_f_sm1_positive_gate_unchanged: bool,
    pub full_f_sm1_passed: bool,
    pub bi0_prerequisite_reopened: bool,
    pub conclusion: String,
    pub audit_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TSm1aReplayV4 {
    pub valid: bool,
    pub t_sm1a_passed: bool,
    pub derived_count: usize,
    pub named_gap_count: usize,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TSm1aErrorV4 {
    #[error("sealed contextual Formation source failed: {0}")]
    SealedSource(String),
    #[error("dependent contextual Formation derivation failed: {0}")]
    Formation(String),
    #[error("chronological Formation specialization failed: {0}")]
    Specialization(String),
    #[error("T-SM1a corpus audit failed: {0}")]
    Audit(String),
    #[error("T-SM1a replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION, domain, value))
        .expect("T-SM1a evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn kernel_ty_projection(ty: &KernelTy) -> KernelTyProjectionV2 {
    KernelTyProjectionV2::from(ty)
}

fn kernel_ty_from_projection(ty: &KernelTyProjectionV2) -> KernelTy {
    match ty {
        KernelTyProjectionV2::Type => KernelTy::Type,
        KernelTyProjectionV2::Element(expression) => KernelTy::El(expression.clone()),
        KernelTyProjectionV2::Function { domain, codomain } => KernelTy::Fun(
            Box::new(kernel_ty_from_projection(domain)),
            Box::new(kernel_ty_from_projection(codomain)),
        ),
        KernelTyProjectionV2::PathDeclaration { dimension } => KernelTy::PathDecl {
            dimension: *dimension,
        },
        KernelTyProjectionV2::Neutral => KernelTy::Neutral,
    }
}

fn contextual_motive(ty: &KernelTy) -> Option<ContextualMotive> {
    match ty {
        KernelTy::Type => Some(ContextualMotive::Type),
        KernelTy::El(expression) => Some(ContextualMotive::Element(expression.clone())),
        KernelTy::Fun(domain, codomain) => Some(ContextualMotive::Function {
            domain: Box::new(contextual_motive(domain)?),
            codomain: Box::new(contextual_motive(codomain)?),
        }),
        KernelTy::PathDecl { .. } | KernelTy::Neutral => None,
    }
}

fn structural_context(types: &[KernelTy]) -> SortedParameterContext {
    SortedParameterContext::new(
        types
            .iter()
            .map(|ty| {
                if *ty == KernelTy::Type {
                    ParameterSort::Type
                } else {
                    ParameterSort::Opaque
                }
            })
            .collect(),
    )
}

fn parse_rule_index(rule: &str, prefix: &str) -> Option<u32> {
    rule.strip_prefix(prefix)?.parse().ok()
}

fn candidate_field_head(expression: &Expr, derivation: &DerivationNode) -> bool {
    match expression {
        Expr::Var(_) => derivation.rule.starts_with("field-ref-"),
        Expr::App(function, _) => derivation
            .children
            .first()
            .is_some_and(|child| candidate_field_head(function, child)),
        _ => false,
    }
}

fn finish_term_node(
    expression: &Expr,
    derivation: &DerivationNode,
    scope_len: u32,
    former: TransparentFormer,
    provenance: ContextualFormationTermProvenanceV4,
    premises: Vec<ContextualFormationTermDerivationV4>,
    no_candidate_fresh_head: bool,
) -> Result<ContextualFormationTermDerivationV4, TSm1aErrorV4> {
    let normalized = normalize(expression, scope_len, NORMALIZATION_FUEL)
        .map_err(|error| TSm1aErrorV4::Formation(format!("normalization failed: {error}")))?;
    let registered_transparent_former = registered_transparent_formers().contains(&former);
    let every_premise_internal = premises.iter().all(|premise| premise.internal_term_issued);
    let full_provenance_retained = match &provenance {
        ContextualFormationTermProvenanceV4::TransparentFormer {
            former: recorded,
            premise_hashes,
        } => {
            *recorded == former
                && *premise_hashes
                    == premises
                        .iter()
                        .map(|premise| premise.derivation_hash.clone())
                        .collect::<Vec<_>>()
        }
        ContextualFormationTermProvenanceV4::AmbientHypothesis {
            declaration_hash, ..
        } => !declaration_hash.is_empty() && premises.is_empty(),
        ContextualFormationTermProvenanceV4::LocalBinder { .. }
        | ContextualFormationTermProvenanceV4::AmbientUniverse
        | ContextualFormationTermProvenanceV4::SealedLibraryConstant { .. } => {
            premises.is_empty()
        }
    } && premises
        .iter()
        .all(|premise| premise.full_provenance_retained);
    let exact_kernel_rule_replayed = true;
    let independent_demand_orbit_exported = false;
    let internal_term_issued = exact_kernel_rule_replayed
        && registered_transparent_former
        && every_premise_internal
        && no_candidate_fresh_head
        && full_provenance_retained
        && !independent_demand_orbit_exported;
    let mut projection = ContextualFormationTermDerivationV4 {
        expression: expression.clone(),
        scope_len,
        derivation_rule: derivation.rule.clone(),
        kernel_type: kernel_ty_projection(&derivation.kernel_ty),
        coarse_kernel_step: derivation.coarse,
        former,
        provenance,
        premises,
        normal_form: normalized.expr,
        normalization_steps: normalized.steps,
        exact_kernel_rule_replayed,
        registered_transparent_former,
        every_premise_internal,
        no_candidate_fresh_head,
        full_provenance_retained,
        independent_demand_orbit_exported,
        internal_term_issued,
        marginal_nu: 0,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("contextual-formation-term", &projection);
    Ok(projection)
}

fn issue_unary_term_node(
    expression: &Expr,
    inner: &Expr,
    derivation: &DerivationNode,
    expected_rule: &str,
    former: TransparentFormer,
    ambient_types: &[KernelTy],
    local_types: &[KernelTy],
    visible_library: u32,
    declaration_hash: &str,
) -> Result<ContextualFormationTermDerivationV4, TSm1aErrorV4> {
    if derivation.rule != expected_rule || derivation.children.len() != 1 {
        return Err(TSm1aErrorV4::Formation(format!(
            "transparent unary rule mismatch: expression {expression:?}, observed {}, expected {expected_rule}",
            derivation.rule
        )));
    }
    let premise = issue_contextual_internal_term(
        inner,
        &derivation.children[0],
        ambient_types,
        local_types,
        visible_library,
        declaration_hash,
    )?;
    let premise_hashes = vec![premise.derivation_hash.clone()];
    finish_term_node(
        expression,
        derivation,
        ambient_types.len() as u32 + local_types.len() as u32,
        former,
        ContextualFormationTermProvenanceV4::TransparentFormer {
            former,
            premise_hashes,
        },
        vec![premise],
        true,
    )
}

fn issue_contextual_internal_term(
    expression: &Expr,
    derivation: &DerivationNode,
    ambient_types: &[KernelTy],
    local_types: &[KernelTy],
    visible_library: u32,
    declaration_hash: &str,
) -> Result<ContextualFormationTermDerivationV4, TSm1aErrorV4> {
    let scope_len = ambient_types.len() as u32 + local_types.len() as u32;
    match expression {
        Expr::Univ if derivation.rule == "univ-form" && derivation.children.is_empty() => {
            finish_term_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::AmbientUniverse,
                ContextualFormationTermProvenanceV4::AmbientUniverse,
                Vec::new(),
                true,
            )
        }
        Expr::Lib(step)
            if (1..=visible_library).contains(step)
                && derivation.rule == "library-constant"
                && derivation.children.is_empty() =>
        {
            finish_term_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::SealedLibraryConstant,
                ContextualFormationTermProvenanceV4::SealedLibraryConstant { step: *step },
                Vec::new(),
                true,
            )
        }
        Expr::Var(_) if derivation.children.is_empty() => {
            if let Some(parameter) = parse_rule_index(&derivation.rule, "ambient-param-") {
                let classifier = ambient_types
                    .get(parameter.saturating_sub(1) as usize)
                    .ok_or_else(|| {
                        TSm1aErrorV4::Formation(format!(
                            "ambient hypothesis {parameter} is outside the exact dependent context"
                        ))
                    })?;
                if &derivation.kernel_ty != classifier {
                    return Err(TSm1aErrorV4::Formation(format!(
                        "ambient hypothesis {parameter} classifier drifted: observed {:?}, declared {:?}",
                        derivation.kernel_ty, classifier
                    )));
                }
                return finish_term_node(
                    expression,
                    derivation,
                    scope_len,
                    TransparentFormer::VariableReference,
                    ContextualFormationTermProvenanceV4::AmbientHypothesis {
                        parameter,
                        classifier: kernel_ty_projection(classifier),
                        declaration_hash: declaration_hash.to_owned(),
                    },
                    Vec::new(),
                    true,
                );
            }
            if let Some(binder) = parse_rule_index(&derivation.rule, "local-var-") {
                let classifier = local_types
                    .get(binder.saturating_sub(1) as usize)
                    .ok_or_else(|| {
                        TSm1aErrorV4::Formation(format!(
                            "local binder {binder} is outside the exact local context"
                        ))
                    })?;
                if &derivation.kernel_ty != classifier {
                    return Err(TSm1aErrorV4::Formation(format!(
                        "local binder {binder} classifier drifted: observed {:?}, declared {:?}",
                        derivation.kernel_ty, classifier
                    )));
                }
                return finish_term_node(
                    expression,
                    derivation,
                    scope_len,
                    TransparentFormer::VariableReference,
                    ContextualFormationTermProvenanceV4::LocalBinder {
                        binder,
                        classifier: kernel_ty_projection(classifier),
                    },
                    Vec::new(),
                    true,
                );
            }
            if derivation.rule.starts_with("field-ref-") {
                return Err(TSm1aErrorV4::Formation(
                    "candidate-field reference survived canonicalization; candidate-fresh Formation is not admitted"
                        .to_owned(),
                ));
            }
            Err(TSm1aErrorV4::Formation(format!(
                "unrecognized variable derivation rule {}",
                derivation.rule
            )))
        }
        Expr::Lam(body) => {
            if derivation.rule != "lam-intro" || derivation.children.len() != 1 {
                return Err(TSm1aErrorV4::Formation(format!(
                    "lambda derivation shape mismatch under {}",
                    derivation.rule
                )));
            }
            let mut child_locals = local_types.to_vec();
            child_locals.push(KernelTy::Neutral);
            let premise = issue_contextual_internal_term(
                body,
                &derivation.children[0],
                ambient_types,
                &child_locals,
                visible_library,
                declaration_hash,
            )?;
            let premise_hashes = vec![premise.derivation_hash.clone()];
            finish_term_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::LambdaIntroduction,
                ContextualFormationTermProvenanceV4::TransparentFormer {
                    former: TransparentFormer::LambdaIntroduction,
                    premise_hashes,
                },
                vec![premise],
                true,
            )
        }
        Expr::App(function, argument) => {
            if candidate_field_head(function, derivation.children.first().unwrap_or(derivation)) {
                return Err(TSm1aErrorV4::Formation(
                    "candidate-fresh application head is excluded".to_owned(),
                ));
            }
            let (expected_children, reduced) = match derivation.rule.as_str() {
                "app-beta" => {
                    let function_whnf = whnf(function, scope_len, NORMALIZATION_FUEL).map_err(
                        |error| TSm1aErrorV4::Formation(format!("function WHNF failed: {error}")),
                    )?;
                    let Expr::Lam(body) = function_whnf.expr else {
                        return Err(TSm1aErrorV4::Formation(
                            "app-beta head did not replay to a lambda".to_owned(),
                        ));
                    };
                    (3, Some(substitute_level(&body, scope_len + 1, argument)))
                }
                "univ-app-form" | "app-fun" | "app-el-pi" | "app-stuck" => (2, None),
                rule => {
                    return Err(TSm1aErrorV4::Formation(format!(
                        "application derivation rule {rule} is outside the replayed transparent inventory"
                    )));
                }
            };
            if derivation.children.len() != expected_children {
                return Err(TSm1aErrorV4::Formation(format!(
                    "application child count {}, expected {expected_children}",
                    derivation.children.len()
                )));
            }
            let mut premises = vec![
                issue_contextual_internal_term(
                    function,
                    &derivation.children[0],
                    ambient_types,
                    local_types,
                    visible_library,
                    declaration_hash,
                )?,
                issue_contextual_internal_term(
                    argument,
                    &derivation.children[1],
                    ambient_types,
                    local_types,
                    visible_library,
                    declaration_hash,
                )?,
            ];
            if let Some(reduced) = reduced {
                premises.push(issue_contextual_internal_term(
                    &reduced,
                    &derivation.children[2],
                    ambient_types,
                    local_types,
                    visible_library,
                    declaration_hash,
                )?);
            }
            let premise_hashes = premises
                .iter()
                .map(|premise| premise.derivation_hash.clone())
                .collect();
            finish_term_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::Application,
                ContextualFormationTermProvenanceV4::TransparentFormer {
                    former: TransparentFormer::Application,
                    premise_hashes,
                },
                premises,
                true,
            )
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            let (expected_rule, former) = if matches!(expression, Expr::Pi(_, _)) {
                ("pi-form", TransparentFormer::PiFormation)
            } else {
                ("sigma-form", TransparentFormer::SigmaFormation)
            };
            if derivation.rule != expected_rule || derivation.children.len() != 2 {
                return Err(TSm1aErrorV4::Formation(format!(
                    "binary former derivation mismatch: observed {}, expected {expected_rule}",
                    derivation.rule
                )));
            }
            let domain_premise = issue_contextual_internal_term(
                domain,
                &derivation.children[0],
                ambient_types,
                local_types,
                visible_library,
                declaration_hash,
            )?;
            let domain_whnf = whnf(domain, scope_len, NORMALIZATION_FUEL).map_err(|error| {
                TSm1aErrorV4::Formation(format!("formation domain WHNF failed: {error}"))
            })?;
            let mut codomain_locals = local_types.to_vec();
            codomain_locals.push(KernelTy::El(domain_whnf.expr));
            let codomain_premise = issue_contextual_internal_term(
                codomain,
                &derivation.children[1],
                ambient_types,
                &codomain_locals,
                visible_library,
                declaration_hash,
            )?;
            let premises = vec![domain_premise, codomain_premise];
            let premise_hashes = premises
                .iter()
                .map(|premise| premise.derivation_hash.clone())
                .collect();
            finish_term_node(
                expression,
                derivation,
                scope_len,
                former,
                ContextualFormationTermProvenanceV4::TransparentFormer {
                    former,
                    premise_hashes,
                },
                premises,
                true,
            )
        }
        Expr::Id(ty, left, right) => {
            if derivation.rule != "id-form" || derivation.children.len() != 3 {
                return Err(TSm1aErrorV4::Formation(format!(
                    "identity formation derivation mismatch under {}",
                    derivation.rule
                )));
            }
            let premises = vec![
                issue_contextual_internal_term(
                    ty,
                    &derivation.children[0],
                    ambient_types,
                    local_types,
                    visible_library,
                    declaration_hash,
                )?,
                issue_contextual_internal_term(
                    left,
                    &derivation.children[1],
                    ambient_types,
                    local_types,
                    visible_library,
                    declaration_hash,
                )?,
                issue_contextual_internal_term(
                    right,
                    &derivation.children[2],
                    ambient_types,
                    local_types,
                    visible_library,
                    declaration_hash,
                )?,
            ];
            let premise_hashes = premises
                .iter()
                .map(|premise| premise.derivation_hash.clone())
                .collect();
            finish_term_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::IdentityFormation,
                ContextualFormationTermProvenanceV4::TransparentFormer {
                    former: TransparentFormer::IdentityFormation,
                    premise_hashes,
                },
                premises,
                true,
            )
        }
        Expr::Refl(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "refl-intro",
            TransparentFormer::ReflexivityIntroduction,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Susp(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "susp-form",
            TransparentFormer::SuspensionFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Trunc(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "trunc-form",
            TransparentFormer::TruncationFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Flat(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "flat-form",
            TransparentFormer::FlatFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Sharp(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "sharp-form",
            TransparentFormer::SharpFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Disc(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "disc-form",
            TransparentFormer::DiscreteFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Shape(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "shape-form",
            TransparentFormer::ShapeFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Next(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "next-form",
            TransparentFormer::NextFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::Eventually(inner) => issue_unary_term_node(
            expression,
            inner,
            derivation,
            "eventually-form",
            TransparentFormer::EventuallyFormation,
            ambient_types,
            local_types,
            visible_library,
            declaration_hash,
        ),
        Expr::PathCon(_) => Err(TSm1aErrorV4::Formation(
            "charged PathCon remains outside zero-credit contextual Formation".to_owned(),
        )),
        Expr::Bang(_) | Expr::WhyNot(_) => Err(TSm1aErrorV4::Formation(
            "linear-exponential constructor is outside the frozen transparent grammar".to_owned(),
        )),
        _ => Err(TSm1aErrorV4::Formation(format!(
            "expression/derivation shape mismatch for {expression:?} under {}",
            derivation.rule
        ))),
    }
}
