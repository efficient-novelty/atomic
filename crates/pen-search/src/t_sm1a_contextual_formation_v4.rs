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

use crate::chronological_slot_map_v3::{
    CHRONOLOGICAL_FORMATION_GAP_V3, replay_chronological_slot_map_v3_json,
};
use crate::contextual_formation_coherence_v3::kernel_context_from_parameter_sorts;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3ChronologicalInterfaceSlotMap, A3DemandOutputType,
    A3RuleConstructor, A3TypedClauseSource, generate_a3_window_for_exact_prefix_unbounded,
    replay_chronological_interface_slot_map,
};
use pen_eval::typed_families::{ParamSort, clause_presentation};
use pen_type::ambient_former_internality::{TransparentFormer, registered_transparent_formers};
use pen_type::contextual_internality::ContextualMotive;
use pen_type::dependent_context::{
    DependentAmbientContextDeclarationProjection, DependentContextMotive,
    issue_dependent_ambient_context_declaration, replay_dependent_ambient_context_declaration,
};
use pen_type::elaborate::{
    DerivationNode, KernelTy, SealedSignature, SingleClauseElaboration, candidate_hash,
    elaborate_single_clause_with_typed_ambient, elaborate_telescope,
};
use pen_type::equality::{EqualityWitness, univalent_equality};
use pen_type::motive_parametric_coherence_v2::KernelTyProjectionV2;
use pen_type::normalize::{normalize, substitute_level, whnf};
use pen_type::substitution::{
    ParameterSort, SortedParameterContext, SubstitutionImage, is_well_scoped,
    issue_structural_substitution, replay_structural_substitution,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION: &str =
    "t-sm1a-genuine-contextual-formation-internal-v4";
pub const T_SM1A_THEOREM_STATEMENT: &str = "an exact sealed contextual Formation assembled only from formable hypotheses, sealed constants, local binders, and registered transparent formers is zero-credit Internal; exact chronological identity-tail substitution preserves that Internal derivation by reissuance, without admitting candidate-fresh Formation";
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

/// Exact public-family evidence used by T-SM1a.  Unlike the frozen v3
/// `SealedNaturalFamilyProjectionV3`, this projection does not require the
/// public occurrence to have a predecessor-closure preimage.  Public
/// ownership is replayed directly from the sealed telescope, while semantic
/// `Internal` authority is supplied separately by the recursive term proof.
/// This distinction is essential for Step 14 clause 4: it is an exact sealed
/// public family but is intentionally absent from the older closure quotient.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactPublicFamilyProjectionV4 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source: A3TypedClauseSource,
    pub canonical_context: Vec<KernelTy>,
    pub canonical_expression: Expr,
    pub canonical_kernel_role: ClauseRole,
    pub canonical_kernel_type: KernelTy,
    pub canonical_typing: SingleClauseElaboration,
    pub canonical_typing_derivation: DerivationNode,
    pub exact_live_source_identity_replayed: bool,
    pub sealed_telescope_replayed: bool,
    pub canonical_presentation_replayed: bool,
    pub canonical_typing_replayed: bool,
    pub exact_public_ownership_replayed: bool,
    pub predecessor_closure_assumed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SealedContextualFormationInternalProjectionV4 {
    pub version: String,
    pub sealed_family: ExactPublicFamilyProjectionV4,
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
        evidence: ExactPublicFamilyProjectionV4,
    },
    PointwiseSealedTypeValuedFamily {
        evidence: ExactPublicFamilyProjectionV4,
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
    pub source_evidence: ExactPublicFamilyProjectionV4,
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
    pub internal_derivation: ContextualFormationBodyDerivationV4,
    pub genuine_internal_replayed: bool,
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
    pub first_image_internal: ChronologicalImageInternalProjectionV4,
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
pub struct TSm1aCorpusCaseV4 {
    pub instance_id: String,
    pub older_step: u32,
    pub older_clause: u16,
    pub newest_step: u32,
    pub newest_clause: u16,
    pub specialization: ChronologicalFormationSpecializationProjectionV4,
    pub replayed: bool,
    pub case_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TSm1aCorpusSweepV4 {
    pub version: String,
    pub signature_digest: String,
    pub stage: u32,
    pub live_case_ids: Vec<String>,
    pub live_case_ids_unique: bool,
    pub live_surface_hash_before_archive_read: String,
    pub cases: Vec<TSm1aCorpusCaseV4>,
    pub every_case_replayed: bool,
    pub sweep_hash: String,
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
    pub live_surface_hash_before_v3_comparison: String,
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

fn replay_exact_live_source_identity(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
) -> Result<(), TSm1aErrorV4> {
    let stage = visible_library.saturating_add(1);
    let window = generate_a3_window_for_exact_prefix_unbounded(signature, stage)
        .map_err(|error| TSm1aErrorV4::SealedSource(error.to_string()))?;
    let expected = window
        .typed_sources
        .iter()
        .find(|candidate| {
            candidate.step == source.step && candidate.clause_index == source.clause_index
        })
        .ok_or_else(|| {
            TSm1aErrorV4::SealedSource(format!(
                "source {}:{} is absent from the exact live historical window",
                source.step, source.clause_index
            ))
        })?;
    if expected != source {
        return Err(TSm1aErrorV4::SealedSource(format!(
            "source identity/provenance drifted at {}:{}",
            source.step, source.clause_index
        )));
    }
    Ok(())
}

/// Reconstruct one exact public family from its sealed owner.  This is a
/// provenance theorem, not an `Internal` theorem: it proves that the source
/// occurred publicly with this exact typed canonical presentation.  No
/// predecessor-closure membership is requested or inferred.
pub fn issue_exact_public_family_v4(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
) -> Result<ExactPublicFamilyProjectionV4, TSm1aErrorV4> {
    replay_exact_live_source_identity(signature, visible_library, source)?;
    if source.step == 0 || source.step > visible_library || !source.exported_public_clause {
        return Err(TSm1aErrorV4::SealedSource(
            "source is not an exact public act in the visible sealed past".to_owned(),
        ));
    }
    let entry = signature.entry(source.step).ok_or_else(|| {
        TSm1aErrorV4::SealedSource(format!(
            "sealed step {} is absent from the supplied prefix",
            source.step
        ))
    })?;
    let elaboration = elaborate_telescope(signature, &entry.telescope, source.step - 1)
        .map_err(|error| TSm1aErrorV4::SealedSource(error.to_string()))?;
    let clause = elaboration
        .clauses
        .get(usize::from(source.clause_index))
        .ok_or_else(|| {
            TSm1aErrorV4::SealedSource(
                "public source clause is outside its sealed owner telescope".to_owned(),
            )
        })?;
    let roles = elaboration
        .clauses
        .iter()
        .map(|candidate| candidate.kernel_role)
        .collect::<Vec<_>>();
    let presentation = clause_presentation(
        &clause.normal_form,
        elaboration.ambient_parameters + u32::from(source.clause_index),
        &roles[..usize::from(source.clause_index)],
        elaboration.ambient_parameters,
    );
    let sealed_telescope_replayed = entry.candidate_hash == source.candidate_hash
        && elaboration.derivation_hash == source.telescope_elaboration_hash
        && clause.kernel_role == source.kernel_role
        && clause.kernel_ty == source.kernel_type
        && clause.normal_form == source.normal_form;
    let canonical_presentation_replayed = presentation == source.canonical_presentation;
    if !sealed_telescope_replayed || !canonical_presentation_replayed {
        return Err(TSm1aErrorV4::SealedSource(
            "exact public family did not replay from its sealed owner telescope".to_owned(),
        ));
    }

    let canonical_context =
        kernel_context_from_parameter_sorts(&source.canonical_presentation.parameters);
    let canonical_expression = source.canonical_presentation.canonical_normal_form.clone();
    if !is_well_scoped(&canonical_expression, canonical_context.len() as u32) {
        return Err(TSm1aErrorV4::SealedSource(
            "exact public canonical expression is not scoped by its declared context".to_owned(),
        ));
    }
    let (canonical_typing, canonical_typing_derivation) =
        elaborate_single_clause_with_typed_ambient(
            &canonical_expression,
            &canonical_context,
            &[],
            visible_library,
        )
        .map_err(|error| TSm1aErrorV4::SealedSource(error.to_string()))?;
    let canonical_typing_replayed = canonical_typing.kernel_role == source.kernel_role
        && canonical_typing.kernel_ty == source.kernel_type
        && canonical_typing.normal_form == canonical_expression;
    let exact_live_source_identity_replayed = true;
    let exact_public_ownership_replayed = source.exported_public_clause
        && source.step <= visible_library
        && source.clause_index < entry.telescope.clauses.len() as u16
        && !source.public_eligibility_hash.is_empty()
        && !source.typing_derivation_hash.is_empty();
    if !canonical_typing_replayed || !exact_public_ownership_replayed {
        return Err(TSm1aErrorV4::SealedSource(
            "exact public canonical typing or occurrence ownership did not replay".to_owned(),
        ));
    }
    let mut projection = ExactPublicFamilyProjectionV4 {
        version: T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        source: source.clone(),
        canonical_context,
        canonical_expression,
        canonical_kernel_role: canonical_typing.kernel_role,
        canonical_kernel_type: canonical_typing.kernel_ty.clone(),
        canonical_typing,
        canonical_typing_derivation,
        exact_live_source_identity_replayed,
        sealed_telescope_replayed,
        canonical_presentation_replayed,
        canonical_typing_replayed,
        exact_public_ownership_replayed,
        predecessor_closure_assumed: false,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("exact-public-family-v4", &projection);
    Ok(projection)
}

pub fn replay_exact_public_family_v4(
    signature: &SealedSignature,
    projection: &ExactPublicFamilyProjectionV4,
) -> Result<(), TSm1aErrorV4> {
    let reissued =
        issue_exact_public_family_v4(signature, projection.visible_library, &projection.source)?;
    if reissued == *projection {
        Ok(())
    } else {
        Err(TSm1aErrorV4::ReplayMismatch)
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

fn exact_kernel_rule_matches(
    expression: &Expr,
    derivation: &DerivationNode,
    former: TransparentFormer,
    premise_count: usize,
) -> bool {
    let rule_and_children = match expression {
        Expr::Univ => derivation.rule == "univ-form" && premise_count == 0,
        Expr::Lib(_) => derivation.rule == "library-constant" && premise_count == 0,
        Expr::Var(_) => {
            (derivation.rule.starts_with("ambient-param-")
                || derivation.rule.starts_with("local-var-"))
                && premise_count == 0
        }
        Expr::Lam(_) => derivation.rule == "lam-intro" && premise_count == 1,
        Expr::App(_, _) => match derivation.rule.as_str() {
            "app-beta" => premise_count == 3,
            "univ-app-form" | "app-fun" | "app-el-pi" | "app-stuck" => premise_count == 2,
            _ => false,
        },
        Expr::Pi(_, _) => derivation.rule == "pi-form" && premise_count == 2,
        Expr::Sigma(_, _) => derivation.rule == "sigma-form" && premise_count == 2,
        Expr::Id(_, _, _) => derivation.rule == "id-form" && premise_count == 3,
        Expr::Refl(_) => derivation.rule == "refl-intro" && premise_count == 1,
        Expr::Susp(_) => derivation.rule == "susp-form" && premise_count == 1,
        Expr::Trunc(_) => derivation.rule == "trunc-form" && premise_count == 1,
        Expr::Flat(_) => derivation.rule == "flat-form" && premise_count == 1,
        Expr::Sharp(_) => derivation.rule == "sharp-form" && premise_count == 1,
        Expr::Disc(_) => derivation.rule == "disc-form" && premise_count == 1,
        Expr::Shape(_) => derivation.rule == "shape-form" && premise_count == 1,
        Expr::Next(_) => derivation.rule == "next-form" && premise_count == 1,
        Expr::Eventually(_) => derivation.rule == "eventually-form" && premise_count == 1,
        Expr::PathCon(_) | Expr::Bang(_) | Expr::WhyNot(_) => false,
    };
    let expected_former = match expression {
        Expr::Univ => Some(TransparentFormer::AmbientUniverse),
        Expr::Lib(_) => Some(TransparentFormer::SealedLibraryConstant),
        Expr::Var(_) => Some(TransparentFormer::VariableReference),
        Expr::Lam(_) => Some(TransparentFormer::LambdaIntroduction),
        Expr::App(_, _) => Some(TransparentFormer::Application),
        Expr::Pi(_, _) => Some(TransparentFormer::PiFormation),
        Expr::Sigma(_, _) => Some(TransparentFormer::SigmaFormation),
        Expr::Id(_, _, _) => Some(TransparentFormer::IdentityFormation),
        Expr::Refl(_) => Some(TransparentFormer::ReflexivityIntroduction),
        Expr::Susp(_) => Some(TransparentFormer::SuspensionFormation),
        Expr::Trunc(_) => Some(TransparentFormer::TruncationFormation),
        Expr::Flat(_) => Some(TransparentFormer::FlatFormation),
        Expr::Sharp(_) => Some(TransparentFormer::SharpFormation),
        Expr::Disc(_) => Some(TransparentFormer::DiscreteFormation),
        Expr::Shape(_) => Some(TransparentFormer::ShapeFormation),
        Expr::Next(_) => Some(TransparentFormer::NextFormation),
        Expr::Eventually(_) => Some(TransparentFormer::EventuallyFormation),
        Expr::PathCon(_) | Expr::Bang(_) | Expr::WhyNot(_) => None,
    };
    rule_and_children
        && derivation.children.len() == premise_count
        && expected_former == Some(former)
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
        | ContextualFormationTermProvenanceV4::SealedLibraryConstant { .. } => premises.is_empty(),
    } && premises
        .iter()
        .all(|premise| premise.full_provenance_retained);
    let exact_kernel_rule_replayed =
        exact_kernel_rule_matches(expression, derivation, former, premises.len());
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
                    let function_whnf =
                        whnf(function, scope_len, NORMALIZATION_FUEL).map_err(|error| {
                            TSm1aErrorV4::Formation(format!("function WHNF failed: {error}"))
                        })?;
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

fn dependent_motives_for_exact_context(
    context: &[KernelTy],
) -> Result<Vec<DependentContextMotive>, TSm1aErrorV4> {
    context
        .iter()
        .enumerate()
        .map(|(index, ty)| {
            contextual_motive(ty)
                .map(|motive| DependentContextMotive::Independent { motive })
                .ok_or_else(|| {
                    TSm1aErrorV4::Formation(format!(
                        "context parameter {} has undeclared classifier {ty:?}; an opaque sealed-type reference would be required",
                        index + 1
                    ))
                })
        })
        .collect()
}

fn issue_contextual_type_body_derivation_v4(
    signature: &SealedSignature,
    visible_library: u32,
    context: Vec<KernelTy>,
    expression: Expr,
    declared_role: ClauseRole,
) -> Result<ContextualFormationBodyDerivationV4, TSm1aErrorV4> {
    if context.is_empty() {
        return Err(TSm1aErrorV4::Formation(
            "T-SM1a requires a live contextual source; closed Formation remains under its existing closure rules"
                .to_owned(),
        ));
    }
    let candidate = Telescope::new(vec![ClauseRec::new(declared_role, expression.clone())]);
    let motives = dependent_motives_for_exact_context(&context)?;
    let declaration = issue_dependent_ambient_context_declaration(
        signature,
        &candidate,
        visible_library,
        motives,
    )
    .map_err(|error| TSm1aErrorV4::Formation(format!("dependent declaration: {error}")))?;
    replay_dependent_ambient_context_declaration(signature, declaration.projection())
        .map_err(|error| TSm1aErrorV4::Formation(format!("dependent replay: {error}")))?;
    let dependent = declaration.projection();
    if dependent.signature_digest != signature.digest()
        || dependent.visible_library != visible_library
        || dependent.body_telescope != candidate
        || dependent.declared_role != declared_role
        || dependent.expression != expression
        || dependent.exact_ambient_kernel_types != context
        || !dependent.every_motive_formable_over_predecessors
        || !dependent.no_outcome_filtering_used
    {
        return Err(TSm1aErrorV4::Formation(
            "dependent declaration does not bind the exact contextual judgment".to_owned(),
        ));
    }
    let elaboration = &dependent.typed_body_elaboration;
    let derivation = &dependent.typed_body_derivation;
    if elaboration.kernel_ty != KernelTy::Type
        || elaboration.kernel_role != declared_role
        || derivation.kernel_ty != KernelTy::Type
    {
        return Err(TSm1aErrorV4::Formation(format!(
            "contextual body is not an exact Type-classified judgment: role {:?}, classifier {:?}",
            elaboration.kernel_role, elaboration.kernel_ty
        )));
    }
    let term_evidence = issue_contextual_internal_term(
        &expression,
        derivation,
        &context,
        &[],
        visible_library,
        &dependent.declaration_hash,
    )?;
    let dependent_context_replayed = true;
    let exact_kernel_typing_replayed = term_evidence.expression == expression
        && term_evidence.kernel_type == KernelTyProjectionV2::Type
        && term_evidence.normal_form == elaboration.normal_form
        && term_evidence.derivation_rule == derivation.rule;
    let every_transparent_premise_replayed = term_evidence.internal_term_issued;
    let candidate_fresh_formation_admitted = false;
    let independent_demand_orbit_exported = false;
    let internal_closure_issued = dependent_context_replayed
        && exact_kernel_typing_replayed
        && every_transparent_premise_replayed
        && !candidate_fresh_formation_admitted
        && !independent_demand_orbit_exported;
    if !internal_closure_issued {
        return Err(TSm1aErrorV4::Formation(
            "transparent contextual constructor derivation did not close".to_owned(),
        ));
    }
    let mut projection = ContextualFormationBodyDerivationV4 {
        version: T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION.to_owned(),
        theorem_statement: T_SM1A_THEOREM_STATEMENT.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate: candidate.clone(),
        candidate_hash: candidate_hash(&candidate),
        declared_context: context.iter().map(kernel_ty_projection).collect(),
        dependent_context: dependent.clone(),
        clause_index: 0,
        declared_role,
        expression,
        normal_form: elaboration.normal_form.clone(),
        kernel_type: kernel_ty_projection(&elaboration.kernel_ty),
        inferred_motive: ContextualMotive::Type,
        kernel_coarse_assumptions: elaboration.coarse_assumptions,
        term_evidence,
        dependent_context_replayed,
        exact_kernel_typing_replayed,
        every_transparent_premise_replayed,
        candidate_fresh_formation_admitted,
        independent_demand_orbit_exported,
        internal_closure_issued,
        marginal_kappa: 0,
        marginal_nu: 0,
        anchors_minted: 0,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("contextual-formation-body", &projection);
    Ok(projection)
}

fn reissue_contextual_type_body_derivation_v4(
    signature: &SealedSignature,
    projection: &ContextualFormationBodyDerivationV4,
) -> Result<ContextualFormationBodyDerivationV4, TSm1aErrorV4> {
    let context = projection
        .declared_context
        .iter()
        .map(kernel_ty_from_projection)
        .collect();
    let reissued = issue_contextual_type_body_derivation_v4(
        signature,
        projection.visible_library,
        context,
        projection.expression.clone(),
        projection.declared_role,
    )?;
    if reissued == *projection {
        Ok(reissued)
    } else {
        Err(TSm1aErrorV4::ReplayMismatch)
    }
}

pub fn issue_sealed_contextual_formation_internal_v4(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
) -> Result<SealedContextualFormationInternalTokenV4, TSm1aErrorV4> {
    let family = issue_exact_public_family_v4(signature, visible_library, source)?;
    if family.canonical_kernel_role != ClauseRole::Formation
        || family.source.kernel_role != ClauseRole::Formation
        || family.canonical_kernel_type != KernelTy::Type
        || family.source.kernel_type != KernelTy::Type
    {
        return Err(TSm1aErrorV4::SealedSource(
            "source is not an exact sealed Formation/Type family".to_owned(),
        ));
    }
    let body_derivation = issue_contextual_type_body_derivation_v4(
        signature,
        visible_library,
        family.canonical_context.clone(),
        family.canonical_expression.clone(),
        ClauseRole::Formation,
    )?;
    let exact_sealed_source_replayed = body_derivation.signature_digest == signature.digest()
        && body_derivation.visible_library == visible_library
        && body_derivation.expression == family.canonical_expression
        && body_derivation.declared_context
            == family
                .canonical_context
                .iter()
                .map(kernel_ty_projection)
                .collect::<Vec<_>>()
        && body_derivation.normal_form == family.canonical_expression;
    let sealed_source_is_contextual_formation = !family.canonical_context.is_empty()
        && family.canonical_kernel_role == ClauseRole::Formation
        && family.canonical_kernel_type == KernelTy::Type;
    let zero_credit = body_derivation.marginal_kappa == 0
        && body_derivation.marginal_nu == 0
        && body_derivation.anchors_minted == 0
        && !body_derivation.independent_demand_orbit_exported;
    let internal_closure_issued = exact_sealed_source_replayed
        && sealed_source_is_contextual_formation
        && zero_credit
        && body_derivation.internal_closure_issued
        && !body_derivation.candidate_fresh_formation_admitted;
    if !internal_closure_issued {
        return Err(TSm1aErrorV4::SealedSource(
            "exact sealed contextual Formation relation did not issue".to_owned(),
        ));
    }
    let mut projection = SealedContextualFormationInternalProjectionV4 {
        version: T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION.to_owned(),
        sealed_family: family,
        body_derivation,
        exact_sealed_source_replayed,
        sealed_source_is_contextual_formation,
        zero_credit,
        internal_closure_issued,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("sealed-contextual-formation", &projection);
    Ok(SealedContextualFormationInternalTokenV4 { projection })
}

fn reissue_sealed_contextual_formation_internal_v4(
    signature: &SealedSignature,
    projection: &SealedContextualFormationInternalProjectionV4,
) -> Result<SealedContextualFormationInternalTokenV4, TSm1aErrorV4> {
    replay_exact_public_family_v4(signature, &projection.sealed_family)?;
    reissue_contextual_type_body_derivation_v4(signature, &projection.body_derivation)?;
    let reissued = issue_sealed_contextual_formation_internal_v4(
        signature,
        projection.sealed_family.visible_library,
        &projection.sealed_family.source,
    )?;
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(TSm1aErrorV4::ReplayMismatch)
    }
}

pub fn replay_sealed_contextual_formation_internal_v4(
    signature: &SealedSignature,
    projection: &SealedContextualFormationInternalProjectionV4,
) -> Result<(), TSm1aErrorV4> {
    reissue_sealed_contextual_formation_internal_v4(signature, projection).map(|_| ())
}

fn contains_exact_beta_child(term: &ContextualFormationTermDerivationV4) -> bool {
    (term.derivation_rule == "app-beta" && term.premises.len() == 3)
        || term.premises.iter().any(contains_exact_beta_child)
}

pub fn issue_chronological_image_internal_v4(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
    interface_mode: A3ChronologicalInterfaceMode,
    target_context: Vec<KernelTy>,
) -> Result<ChronologicalImageInternalTokenV4, TSm1aErrorV4> {
    let evidence = issue_exact_public_family_v4(signature, visible_library, source)?;
    let raw_expression = match &interface_mode {
        A3ChronologicalInterfaceMode::DirectType => {
            if evidence.canonical_kernel_type != KernelTy::Type {
                return Err(TSm1aErrorV4::Specialization(
                    "direct chronological image is not Type-classified".to_owned(),
                ));
            }
            evidence.canonical_expression.clone()
        }
        A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { domain } => {
            let KernelTy::Fun(actual_domain, codomain) = &evidence.canonical_kernel_type else {
                return Err(TSm1aErrorV4::Specialization(
                    "pointwise chronological image source is not a function".to_owned(),
                ));
            };
            if actual_domain.as_ref() != domain
                || codomain.as_ref() != &KernelTy::Type
                || target_context.is_empty()
            {
                return Err(TSm1aErrorV4::Specialization(
                    "pointwise chronological image does not have its exact declared Type-valued interface"
                        .to_owned(),
                ));
            }
            Expr::App(
                Box::new(evidence.canonical_expression.clone()),
                Box::new(Expr::Var(1)),
            )
        }
    };
    let (typed, _) = elaborate_single_clause_with_typed_ambient(
        &raw_expression,
        &target_context,
        &[],
        visible_library,
    )
    .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    if typed.kernel_ty != KernelTy::Type {
        return Err(TSm1aErrorV4::Specialization(format!(
            "chronological first image synthesizes {:?}, expected Type",
            typed.kernel_ty
        )));
    }
    let body_derivation = issue_contextual_type_body_derivation_v4(
        signature,
        visible_library,
        target_context.clone(),
        raw_expression.clone(),
        typed.kernel_role,
    )?;
    let raw_to_normal_equality = univalent_equality(
        &raw_expression,
        &body_derivation.normal_form,
        target_context.len() as u32,
        NORMALIZATION_FUEL,
    )
    .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    if !raw_to_normal_equality.equal {
        return Err(TSm1aErrorV4::Specialization(
            "raw chronological image is not equal to its replayed normal form".to_owned(),
        ));
    }
    let exact_beta_child_replayed = !matches!(
        interface_mode,
        A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. }
    ) || contains_exact_beta_child(&body_derivation.term_evidence);
    let exact_sealed_source_replayed = evidence.source == *source
        && evidence.signature_digest == signature.digest()
        && evidence.visible_library == visible_library;
    let raw_registered_expression_retained = body_derivation.expression == raw_expression;
    let internal_closure_issued = exact_sealed_source_replayed
        && raw_registered_expression_retained
        && exact_beta_child_replayed
        && body_derivation.internal_closure_issued;
    if !internal_closure_issued {
        return Err(TSm1aErrorV4::Specialization(
            "chronological image Internal relation did not close".to_owned(),
        ));
    }
    let mut projection = ChronologicalImageInternalProjectionV4 {
        version: T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        source: source.clone(),
        source_evidence: evidence,
        interface_mode,
        target_context: target_context.iter().map(kernel_ty_projection).collect(),
        raw_expression: raw_expression.clone(),
        raw_expression_role: typed.kernel_role,
        normal_form: body_derivation.normal_form.clone(),
        raw_to_normal_equality,
        body_derivation,
        exact_sealed_source_replayed,
        raw_registered_expression_retained,
        exact_beta_child_replayed,
        internal_closure_issued,
        marginal_nu: 0,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("chronological-image-internal", &projection);
    Ok(ChronologicalImageInternalTokenV4 { projection })
}

fn reissue_chronological_image_internal_v4(
    signature: &SealedSignature,
    projection: &ChronologicalImageInternalProjectionV4,
) -> Result<ChronologicalImageInternalTokenV4, TSm1aErrorV4> {
    replay_exact_public_family_v4(signature, &projection.source_evidence)?;
    reissue_contextual_type_body_derivation_v4(signature, &projection.body_derivation)?;
    let target_context = projection
        .target_context
        .iter()
        .map(kernel_ty_from_projection)
        .collect();
    let reissued = issue_chronological_image_internal_v4(
        signature,
        projection.visible_library,
        &projection.source,
        projection.interface_mode.clone(),
        target_context,
    )?;
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(TSm1aErrorV4::ReplayMismatch)
    }
}

pub fn replay_chronological_image_internal_v4(
    signature: &SealedSignature,
    projection: &ChronologicalImageInternalProjectionV4,
) -> Result<(), TSm1aErrorV4> {
    reissue_chronological_image_internal_v4(signature, projection).map(|_| ())
}

fn expected_interface_mode(
    older: &A3TypedClauseSource,
) -> Result<A3ChronologicalInterfaceMode, TSm1aErrorV4> {
    match &older.kernel_type {
        KernelTy::Type => Ok(A3ChronologicalInterfaceMode::DirectType),
        KernelTy::Fun(domain, codomain) if codomain.as_ref() == &KernelTy::Type => {
            Ok(A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction {
                domain: domain.as_ref().clone(),
            })
        }
        other => Err(TSm1aErrorV4::Specialization(format!(
            "older chronological interface is not Type-valued: {other:?}"
        ))),
    }
}

fn target_context_for_chronological_pair(
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
) -> Result<Vec<KernelTy>, TSm1aErrorV4> {
    let target_arity = older
        .canonical_presentation
        .parameters
        .len()
        .max(newest.canonical_presentation.parameters.len());
    let parameters = (0..target_arity)
        .map(|index| {
            older
                .canonical_presentation
                .parameters
                .get(index)
                .or_else(|| newest.canonical_presentation.parameters.get(index))
                .cloned()
                .ok_or_else(|| {
                    TSm1aErrorV4::Specialization(
                        "chronological target parameter inventory has a hole".to_owned(),
                    )
                })
        })
        .collect::<Result<Vec<ParamSort>, _>>()?;
    Ok(kernel_context_from_parameter_sorts(&parameters))
}

pub fn issue_chronological_formation_specialization_v4(
    signature: &SealedSignature,
    visible_library: u32,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
    interface_mode: A3ChronologicalInterfaceMode,
    slot_map: A3ChronologicalInterfaceSlotMap,
) -> Result<ChronologicalFormationSpecializationTokenV4, TSm1aErrorV4> {
    if older.step >= newest.step || !older.exported_public_clause || !newest.exported_public_clause
    {
        return Err(TSm1aErrorV4::Specialization(
            "chronological source orientation/publicity failed".to_owned(),
        ));
    }
    let expected_mode = expected_interface_mode(older)?;
    if interface_mode != expected_mode {
        return Err(TSm1aErrorV4::Specialization(
            "chronological interface mode differs from the exact older classifier".to_owned(),
        ));
    }
    let source = issue_sealed_contextual_formation_internal_v4(signature, visible_library, newest)?;
    let source_context = source
        .projection
        .body_derivation
        .declared_context
        .iter()
        .map(kernel_ty_from_projection)
        .collect::<Vec<_>>();
    let source_arity = source_context.len() as u32;
    replay_chronological_interface_slot_map(&slot_map, source_arity)
        .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    if source_arity == 0 {
        return Err(TSm1aErrorV4::Specialization(
            "contextual Formation source has no chronological interface parameter".to_owned(),
        ));
    }
    let target_context = target_context_for_chronological_pair(older, newest)?;
    if target_context.len() < source_context.len() {
        return Err(TSm1aErrorV4::Specialization(
            "chronological target context is smaller than the source context".to_owned(),
        ));
    }
    let first_image_internal = issue_chronological_image_internal_v4(
        signature,
        visible_library,
        older,
        interface_mode.clone(),
        target_context.clone(),
    )?;
    let first_term = first_image_internal.projection.raw_expression.clone();
    let first_image_evidence = match &interface_mode {
        A3ChronologicalInterfaceMode::DirectType => {
            ChronologicalFirstImageEvidenceV4::DirectSealedTypeFamily {
                evidence: first_image_internal.projection.source_evidence.clone(),
            }
        }
        A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. } => {
            ChronologicalFirstImageEvidenceV4::PointwiseSealedTypeValuedFamily {
                evidence: first_image_internal.projection.source_evidence.clone(),
                argument_parameter: 1,
            }
        }
    };
    let images = source_context
        .iter()
        .enumerate()
        .map(|(index, _)| SubstitutionImage {
            source_parameter: index as u32 + 1,
            term: if index == 0 {
                first_term.clone()
            } else {
                Expr::Var(index as u32 + 1)
            },
        })
        .collect::<Vec<_>>();
    let ordered_identity_tail_exact = images.iter().skip(1).enumerate().all(|(index, image)| {
        let parameter = index as u32 + 2;
        image.source_parameter == parameter && image.term == Expr::Var(parameter)
    });
    let no_permutation_or_instance_override =
        slot_map
            .assignments
            .iter()
            .enumerate()
            .all(|(index, assignment)| {
                assignment.interface_slot == index as u32 + 1
                    && assignment.parameter == index as u32 + 1
            })
            && !slot_map.inferred_from_derivation_success
            && !slot_map.instance_override_permitted;

    let mut image_typings = Vec::with_capacity(images.len());
    for (index, (image, expected_classifier)) in
        images.iter().zip(source_context.iter()).enumerate()
    {
        let (elaboration, derivation) = elaborate_single_clause_with_typed_ambient(
            &image.term,
            &target_context,
            &[],
            visible_library,
        )
        .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
        let exact_classifier_replayed = elaboration.kernel_ty == *expected_classifier;
        if !exact_classifier_replayed {
            return Err(TSm1aErrorV4::Specialization(format!(
                "image {} has classifier {:?}, expected {:?}",
                image.source_parameter, elaboration.kernel_ty, expected_classifier
            )));
        }
        let internal_derivation = if index == 0 {
            first_image_internal.projection.body_derivation.clone()
        } else {
            issue_contextual_type_body_derivation_v4(
                signature,
                visible_library,
                target_context.clone(),
                image.term.clone(),
                elaboration.kernel_role,
            )?
        };
        let genuine_internal_replayed = internal_derivation.expression == image.term
            && internal_derivation.declared_context
                == target_context
                    .iter()
                    .map(kernel_ty_projection)
                    .collect::<Vec<_>>()
            && internal_derivation.kernel_type == kernel_ty_projection(expected_classifier)
            && internal_derivation.internal_closure_issued;
        if !genuine_internal_replayed {
            return Err(TSm1aErrorV4::Specialization(format!(
                "image {} lacks an exact replayed Internal derivation",
                image.source_parameter
            )));
        }
        let mut typing = ExactContextualImageTypingV4 {
            source_parameter: image.source_parameter,
            expected_classifier: kernel_ty_projection(expected_classifier),
            term: image.term.clone(),
            target_context: target_context.iter().map(kernel_ty_projection).collect(),
            elaboration,
            derivation,
            exact_classifier_replayed,
            internal_derivation,
            genuine_internal_replayed,
            derivation_hash: String::new(),
        };
        typing.derivation_hash = tagged_hash("exact-contextual-image", &typing);
        image_typings.push(typing);
    }
    let every_image_exactly_typed = image_typings
        .iter()
        .all(|image| image.exact_classifier_replayed && image.genuine_internal_replayed);
    let substitution_body = source.projection.body_derivation.expression.clone();
    let substitution = issue_structural_substitution(
        structural_context(&source_context),
        structural_context(&target_context),
        images.clone(),
        substitution_body.clone(),
    )
    .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    let substitution_result = substitution.result().clone();
    let normal_substitution = issue_structural_substitution(
        structural_context(&source_context),
        structural_context(&target_context),
        images.clone(),
        source.projection.body_derivation.normal_form.clone(),
    )
    .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    replay_structural_substitution(&normal_substitution)
        .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    let (target_typed, _) = elaborate_single_clause_with_typed_ambient(
        &substitution_result,
        &target_context,
        &[],
        visible_library,
    )
    .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    if target_typed.kernel_ty != KernelTy::Type || target_typed.kernel_role != ClauseRole::Formation
    {
        return Err(TSm1aErrorV4::Specialization(format!(
            "post-substitution judgment is not Formation/Type: {:?}/{:?}",
            target_typed.kernel_role, target_typed.kernel_ty
        )));
    }
    let specialized_internal = issue_contextual_type_body_derivation_v4(
        signature,
        visible_library,
        target_context.clone(),
        substitution_result.clone(),
        ClauseRole::Formation,
    )?;
    let normalization_commutes = univalent_equality(
        normal_substitution.result(),
        &specialized_internal.normal_form,
        target_context.len() as u32,
        NORMALIZATION_FUEL,
    )
    .map_err(|error| TSm1aErrorV4::Specialization(error.to_string()))?;
    if !normalization_commutes.equal {
        return Err(TSm1aErrorV4::Specialization(
            "normalization does not commute with chronological substitution".to_owned(),
        ));
    }
    let post_substitution_internal_replayed = specialized_internal.expression
        == substitution_result
        && specialized_internal.internal_closure_issued
        && specialized_internal.declared_context
            == target_context
                .iter()
                .map(kernel_ty_projection)
                .collect::<Vec<_>>();
    let exact_open_substitution_result_replayed = post_substitution_internal_replayed
        && every_image_exactly_typed
        && ordered_identity_tail_exact
        && no_permutation_or_instance_override
        && substitution.result() == &specialized_internal.expression
        && normalization_commutes.equal;
    if !exact_open_substitution_result_replayed {
        return Err(TSm1aErrorV4::Specialization(
            "exact post-substitution Internal judgment did not replay".to_owned(),
        ));
    }
    let candidate_fresh_formation_admitted = false;
    let mut projection = ChronologicalFormationSpecializationProjectionV4 {
        version: T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        source: source.projection.clone(),
        older_source: older.clone(),
        first_image_evidence,
        first_image_internal: first_image_internal.projection.clone(),
        interface_mode,
        slot_map,
        source_context: source_context.iter().map(kernel_ty_projection).collect(),
        target_context: target_context.iter().map(kernel_ty_projection).collect(),
        images,
        image_typings,
        every_image_exactly_typed,
        ordered_identity_tail_exact,
        no_permutation_or_instance_override,
        substitution_body,
        substitution_result,
        substitution_derivation_hash: substitution.derivation_hash().to_owned(),
        structural_substitution_replayed: true,
        source_normal_form_substitution_result: normal_substitution.result().clone(),
        normalization_commutes,
        specialized_internal,
        post_substitution_internal_replayed,
        exact_open_substitution_result_replayed,
        candidate_fresh_formation_admitted,
        marginal_nu: 0,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("chronological-formation-specialization", &projection);
    Ok(ChronologicalFormationSpecializationTokenV4 { projection })
}

fn reissue_chronological_formation_specialization_v4(
    signature: &SealedSignature,
    projection: &ChronologicalFormationSpecializationProjectionV4,
) -> Result<ChronologicalFormationSpecializationTokenV4, TSm1aErrorV4> {
    reissue_sealed_contextual_formation_internal_v4(signature, &projection.source)?;
    reissue_chronological_image_internal_v4(signature, &projection.first_image_internal)?;
    reissue_contextual_type_body_derivation_v4(signature, &projection.specialized_internal)?;
    for image in &projection.image_typings {
        reissue_contextual_type_body_derivation_v4(signature, &image.internal_derivation)?;
    }
    let reissued = issue_chronological_formation_specialization_v4(
        signature,
        projection.visible_library,
        &projection.older_source,
        &projection.source.sealed_family.source,
        projection.interface_mode.clone(),
        projection.slot_map.clone(),
    )?;
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(TSm1aErrorV4::ReplayMismatch)
    }
}

pub fn replay_chronological_formation_specialization_v4(
    signature: &SealedSignature,
    projection: &ChronologicalFormationSpecializationProjectionV4,
) -> Result<(), TSm1aErrorV4> {
    reissue_chronological_formation_specialization_v4(signature, projection).map(|_| ())
}

#[derive(Clone)]
struct LiveFormationCaseV4 {
    instance_id: String,
    older: A3TypedClauseSource,
    newest: A3TypedClauseSource,
    interface_mode: A3ChronologicalInterfaceMode,
    slot_map: A3ChronologicalInterfaceSlotMap,
}

fn live_formation_cases_v4(
    signature: &SealedSignature,
) -> Result<Vec<LiveFormationCaseV4>, TSm1aErrorV4> {
    let window = generate_a3_window_for_exact_prefix_unbounded(signature, 16)
        .map_err(|error| TSm1aErrorV4::Audit(error.to_string()))?;
    let mut cases = Vec::new();
    for instance in &window.instances {
        let scheme = window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .ok_or_else(|| TSm1aErrorV4::Audit("orphan live A3 instance".to_owned()))?;
        if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
            continue;
        }
        if instance.source_anchor_ids.len() != 2 {
            return Err(TSm1aErrorV4::Audit(format!(
                "chronological instance {} does not have two sources",
                instance.instance_id
            )));
        }
        let older = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[0])
            .ok_or_else(|| TSm1aErrorV4::Audit("older live source absent".to_owned()))?;
        let newest = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[1])
            .ok_or_else(|| TSm1aErrorV4::Audit("newest live source absent".to_owned()))?;
        if newest.kernel_role != ClauseRole::Formation {
            continue;
        }
        let A3DemandOutputType::ChronologicalInteraction {
            older_family,
            older_type,
            newest_family,
            newest_type,
            interface_mode,
            interface_slot_map,
        } = &scheme.required_output
        else {
            return Err(TSm1aErrorV4::Audit(
                "live chronological scheme has a non-chronological output".to_owned(),
            ));
        };
        if older_family != &older.canonical_family_key
            || older_type != &older.kernel_type
            || newest_family != &newest.canonical_family_key
            || newest_type != &newest.kernel_type
            || instance.source_family_keys
                != vec![
                    older.canonical_family_key.clone(),
                    newest.canonical_family_key.clone(),
                ]
        {
            return Err(TSm1aErrorV4::Audit(format!(
                "live chronological source join failed at {}",
                instance.instance_id
            )));
        }
        cases.push(LiveFormationCaseV4 {
            instance_id: instance.instance_id.clone(),
            older: older.clone(),
            newest: newest.clone(),
            interface_mode: interface_mode.clone(),
            slot_map: interface_slot_map.clone(),
        });
    }
    cases.sort_by(|left, right| left.instance_id.cmp(&right.instance_id));
    let ids = cases
        .iter()
        .map(|case| case.instance_id.as_str())
        .collect::<BTreeSet<_>>();
    if cases.len() != T_SM1A_EXPECTED_CASES || ids.len() != cases.len() {
        return Err(TSm1aErrorV4::Audit(format!(
            "live Formation surface is {}/{}, expected {T_SM1A_EXPECTED_CASES} unique cases",
            cases.len(),
            ids.len()
        )));
    }
    Ok(cases)
}

/// Seal the live 54-case surface and return every exact replay projection.
/// This function never reads the archived v3 result; callers may compare only
/// after it returns.
pub fn issue_t_sm1a_corpus_sweep_v4() -> Result<TSm1aCorpusSweepV4, TSm1aErrorV4> {
    let signature = SealedSignature::genesis_del_h15();
    let live = live_formation_cases_v4(&signature)?;
    let live_case_ids = live
        .iter()
        .map(|case| case.instance_id.clone())
        .collect::<Vec<_>>();
    let live_case_ids_unique =
        live_case_ids.iter().collect::<BTreeSet<_>>().len() == live_case_ids.len();
    let live_surface_hash_before_archive_read = tagged_hash(
        "live-54-surface-before-archive-read",
        &live
            .iter()
            .map(|case| {
                (
                    &case.instance_id,
                    case.older.step,
                    case.older.clause_index,
                    case.newest.step,
                    case.newest.clause_index,
                    &case.interface_mode,
                    &case.slot_map,
                )
            })
            .collect::<Vec<_>>(),
    );
    let mut cases = Vec::with_capacity(live.len());
    for case in live {
        let specialization = issue_chronological_formation_specialization_v4(
            &signature,
            15,
            &case.older,
            &case.newest,
            case.interface_mode,
            case.slot_map,
        )
        .map_err(|error| {
            TSm1aErrorV4::Audit(format!(
                "live case {} (older {}:{}, newest {}:{}) failed: {error}",
                case.instance_id,
                case.older.step,
                case.older.clause_index,
                case.newest.step,
                case.newest.clause_index,
            ))
        })?;
        replay_chronological_formation_specialization_v4(&signature, specialization.projection())?;
        let mut row = TSm1aCorpusCaseV4 {
            instance_id: case.instance_id,
            older_step: case.older.step,
            older_clause: case.older.clause_index,
            newest_step: case.newest.step,
            newest_clause: case.newest.clause_index,
            specialization: specialization.projection.clone(),
            replayed: true,
            case_hash: String::new(),
        };
        row.case_hash = tagged_hash("t-sm1a-corpus-case", &row);
        cases.push(row);
    }
    let every_case_replayed = cases.len() == T_SM1A_EXPECTED_CASES
        && cases.iter().all(|case| {
            case.replayed
                && case.specialization.exact_open_substitution_result_replayed
                && case.specialization.post_substitution_internal_replayed
        });
    let mut sweep = TSm1aCorpusSweepV4 {
        version: T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        stage: 16,
        live_case_ids,
        live_case_ids_unique,
        live_surface_hash_before_archive_read,
        cases,
        every_case_replayed,
        sweep_hash: String::new(),
    };
    sweep.sweep_hash = tagged_hash("t-sm1a-corpus-sweep", &sweep);
    Ok(sweep)
}

pub fn replay_t_sm1a_corpus_sweep_v4(sweep: &TSm1aCorpusSweepV4) -> Result<(), TSm1aErrorV4> {
    let expected = issue_t_sm1a_corpus_sweep_v4()?;
    if expected == *sweep {
        Ok(())
    } else {
        Err(TSm1aErrorV4::ReplayMismatch)
    }
}

fn archived_v3_formation_gap_ids() -> Result<Vec<String>, TSm1aErrorV4> {
    let text = std::str::from_utf8(FROZEN_V3_ARTIFACT_BYTES)
        .map_err(|error| TSm1aErrorV4::Audit(error.to_string()))?;
    let replay = replay_chronological_slot_map_v3_json(text);
    if !replay.valid {
        return Err(TSm1aErrorV4::Audit(format!(
            "frozen v3 artifact failed independent replay: {}",
            replay.errors.join("; ")
        )));
    }
    let value: Value =
        serde_json::from_str(text).map_err(|error| TSm1aErrorV4::Audit(error.to_string()))?;
    let windows = value
        .get("windows")
        .and_then(Value::as_array)
        .ok_or_else(|| TSm1aErrorV4::Audit("v3 windows absent".to_owned()))?;
    let mut ids = Vec::new();
    for window in windows {
        let instances = window
            .get("instances")
            .and_then(Value::as_array)
            .ok_or_else(|| TSm1aErrorV4::Audit("v3 instances absent".to_owned()))?;
        for instance in instances {
            let sealed = instance
                .get("independently_sealed_discharge")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let gap = instance
                .get("membership")
                .and_then(|membership| membership.get("gap_id"))
                .and_then(Value::as_str);
            if sealed && gap == Some(CHRONOLOGICAL_FORMATION_GAP_V3) {
                let id = instance
                    .get("instance_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| TSm1aErrorV4::Audit("v3 instance id absent".to_owned()))?;
                ids.push(id.to_owned());
            }
        }
    }
    ids.sort();
    Ok(ids)
}

pub fn issue_t_sm1a_audit_v4() -> Result<TSm1aAuditV4, TSm1aErrorV4> {
    // The complete live proof surface is sealed before the first archive read.
    let sweep = issue_t_sm1a_corpus_sweep_v4()?;
    // Issuance already replays every one of the 54 specialization tokens.
    // A caller can use `replay_t_sm1a_corpus_sweep_v4` to reissue the whole
    // sealed sweep; repeating it here would add no premise to the audit.
    let live_ids = sweep.live_case_ids.clone();
    let archived_ids = archived_v3_formation_gap_ids()?;
    let exact_v3_formation_gap_surface_recovered = live_ids == archived_ids;
    let frozen_v3_artifact_replayed = true;

    let mut cases = Vec::with_capacity(sweep.cases.len());
    for case in &sweep.cases {
        let mut row = TSm1aCaseAuditV4 {
            instance_id: case.instance_id.clone(),
            older_step: case.older_step,
            older_clause: case.older_clause,
            newest_step: case.newest_step,
            newest_clause: case.newest_clause,
            source_derivation_hash: Some(case.specialization.source.derivation_hash.clone()),
            specialization_derivation_hash: Some(case.specialization.derivation_hash.clone()),
            exact_open_substitution_result_replayed: case
                .specialization
                .exact_open_substitution_result_replayed,
            post_substitution_internal_replayed: case
                .specialization
                .post_substitution_internal_replayed,
            replayed: case.replayed,
            gap: None,
            row_hash: String::new(),
        };
        row.row_hash = tagged_hash("t-sm1a-audit-case", &row);
        cases.push(row);
    }
    let derived_count = cases.iter().filter(|case| case.gap.is_none()).count();
    let named_gap_count = cases.len() - derived_count;
    let mut named_gap_counts = BTreeMap::new();
    for case in &cases {
        if let Some(gap) = &case.gap {
            *named_gap_counts.entry(gap.gap_id.clone()).or_insert(0) += 1;
        }
    }
    let source_formation_coordinates = sweep
        .cases
        .iter()
        .map(|case| (case.newest_step, case.newest_clause))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let older_interface_coordinates = sweep
        .cases
        .iter()
        .map(|case| (case.older_step, case.older_clause))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let every_source_context_replayed = sweep.cases.iter().all(|case| {
        case.specialization.source.exact_sealed_source_replayed
            && case
                .specialization
                .source
                .body_derivation
                .dependent_context_replayed
    });
    let every_post_substitution_internal_replayed = cases
        .iter()
        .all(|case| case.post_substitution_internal_replayed);
    let every_exact_open_substitution_result_replayed = cases
        .iter()
        .all(|case| case.exact_open_substitution_result_replayed);
    let candidate_fresh_formation_admitted = sweep.cases.iter().any(|case| {
        case.specialization.candidate_fresh_formation_admitted
            || case
                .specialization
                .source
                .body_derivation
                .candidate_fresh_formation_admitted
            || case
                .specialization
                .specialized_internal
                .candidate_fresh_formation_admitted
    });
    let t_sm1a_passed = sweep.live_case_ids_unique
        && sweep.cases.len() == T_SM1A_EXPECTED_CASES
        && exact_v3_formation_gap_surface_recovered
        && source_formation_coordinates.len() == 6
        && older_interface_coordinates.len() == 9
        && derived_count == T_SM1A_EXPECTED_CASES
        && named_gap_count == 0
        && every_source_context_replayed
        && every_post_substitution_internal_replayed
        && every_exact_open_substitution_result_replayed
        && !candidate_fresh_formation_admitted;
    let mut adjudication_bindings = BTreeMap::new();
    adjudication_bindings.insert(
        "docs/nu_register_adjudication.md".to_owned(),
        bytes_hash(NU_REGISTER_ADJUDICATION_BYTES),
    );
    adjudication_bindings.insert(
        "docs/contextual_internality_adjudication.md".to_owned(),
        bytes_hash(CONTEXTUAL_INTERNALITY_ADJUDICATION_BYTES),
    );
    adjudication_bindings.insert(
        "docs/motive_parametric_coherence_adjudication.md".to_owned(),
        bytes_hash(MOTIVE_COHERENCE_ADJUDICATION_BYTES),
    );
    adjudication_bindings.insert(
        "docs/dependent_context_adjudication.md".to_owned(),
        bytes_hash(DEPENDENT_CONTEXT_ADJUDICATION_BYTES),
    );
    let mut frozen_v3_source_bindings = BTreeMap::new();
    frozen_v3_source_bindings.insert(
        "crates/pen-search/src/contextual_formation_coherence_v3.rs".to_owned(),
        bytes_hash(FROZEN_V3_THEOREM_SOURCE_BYTES),
    );
    frozen_v3_source_bindings.insert(
        "crates/pen-search/src/chronological_slot_map_v3.rs".to_owned(),
        bytes_hash(FROZEN_V3_WRAPPER_SOURCE_BYTES),
    );
    let mut audit = TSm1aAuditV4 {
        version: T_SM1A_CONTEXTUAL_FORMATION_V4_VERSION.to_owned(),
        theorem_statement: T_SM1A_THEOREM_STATEMENT.to_owned(),
        adjudication_bindings,
        frozen_v3_source_bindings,
        frozen_v3_artifact_digest: bytes_hash(FROZEN_V3_ARTIFACT_BYTES),
        frozen_v3_artifact_replayed,
        live_case_ids_fixed_before_v3_comparison: true,
        live_surface_hash_before_v3_comparison: sweep
            .live_surface_hash_before_archive_read
            .clone(),
        live_case_count: sweep.cases.len(),
        live_case_ids_unique: sweep.live_case_ids_unique,
        exact_v3_formation_gap_surface_recovered,
        source_formation_coordinates,
        older_interface_coordinates,
        cases,
        derived_count,
        named_gap_count,
        named_gap_counts,
        every_source_context_replayed,
        every_post_substitution_internal_replayed,
        every_exact_open_substitution_result_replayed,
        candidate_fresh_formation_admitted,
        t_sm1a_passed,
        fixed_f_sm1_positive_gate_unchanged: true,
        full_f_sm1_passed: false,
        bi0_prerequisite_reopened: false,
        conclusion: if t_sm1a_passed {
            "T-SM1a discharges exactly the 54 contextual-Formation gaps with replayable source, premise, substitution, and target Internal derivations. T-SM1b and the fixed 72/72 F-SM1 gate remain outstanding; this theorem authorizes no BI-0, BI-1, or BI-4 action."
        } else {
            "T-SM1a remains false with named theorem gaps. The fixed 72/72 F-SM1 gate is unchanged, and no BI-0, BI-1, or BI-4 action is authorized."
        }
        .to_owned(),
        audit_hash: String::new(),
    };
    audit.audit_hash = tagged_hash("t-sm1a-audit", &audit);
    Ok(audit)
}

pub fn replay_t_sm1a_audit_v4(audit: &TSm1aAuditV4) -> TSm1aReplayV4 {
    let mut errors = Vec::new();
    let mut hash_projection = audit.clone();
    hash_projection.audit_hash.clear();
    if audit.audit_hash != tagged_hash("t-sm1a-audit", &hash_projection) {
        errors.push("audit hash mismatch".to_owned());
    }
    match issue_t_sm1a_audit_v4() {
        Ok(expected) if expected == *audit => {}
        Ok(_) => errors.push("audit differs from independent T-SM1a reissuance".to_owned()),
        Err(error) => errors.push(format!("independent T-SM1a reissuance failed: {error}")),
    }
    TSm1aReplayV4 {
        valid: errors.is_empty(),
        t_sm1a_passed: audit.t_sm1a_passed,
        derived_count: audit.derived_count,
        named_gap_count: audit.named_gap_count,
        errors,
    }
}

pub fn render_t_sm1a_audit_v4(audit: &TSm1aAuditV4) -> String {
    format!(
        "T-SM1a v4: passed={} derived={}/{} gaps={} source-formations={} older-interfaces={} exact-v3-surface={} post-substitution-internal={} full-F-SM1={} BI0={} digest={}\n{}",
        audit.t_sm1a_passed,
        audit.derived_count,
        audit.live_case_count,
        audit.named_gap_count,
        audit.source_formation_coordinates.len(),
        audit.older_interface_coordinates.len(),
        audit.exact_v3_formation_gap_surface_recovered,
        audit.every_post_substitution_internal_replayed,
        audit.full_f_sm1_passed,
        audit.bi0_prerequisite_reopened,
        audit.audit_hash,
        audit.conclusion,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pointwise_case() -> (
        SealedSignature,
        ChronologicalFormationSpecializationProjectionV4,
    ) {
        let signature = SealedSignature::genesis_del_h15();
        let case = live_formation_cases_v4(&signature)
            .expect("live Formation cases")
            .into_iter()
            .find(|case| case.older.step == 14 && case.older.clause_index == 8)
            .expect("pointwise Step-14 clause-8 case");
        let token = issue_chronological_formation_specialization_v4(
            &signature,
            15,
            &case.older,
            &case.newest,
            case.interface_mode,
            case.slot_map,
        )
        .expect("pointwise chronological Formation specialization");
        (signature, token.projection().clone())
    }

    #[test]
    fn exact_54_case_sweep_includes_six_raw_beta_images() {
        let sweep = issue_t_sm1a_corpus_sweep_v4().expect("T-SM1a corpus sweep");
        assert_eq!(sweep.cases.len(), T_SM1A_EXPECTED_CASES);
        assert!(sweep.live_case_ids_unique);
        assert!(sweep.every_case_replayed);
        let beta_cases = sweep
            .cases
            .iter()
            .filter(|case| case.older_step == 14 && case.older_clause == 8)
            .collect::<Vec<_>>();
        assert_eq!(beta_cases.len(), 6);
        assert!(beta_cases.iter().all(|case| {
            let image = &case.specialization.first_image_internal;
            matches!(image.raw_expression, Expr::App(_, _))
                && image.raw_registered_expression_retained
                && image.raw_to_normal_equality.equal
                && image.exact_beta_child_replayed
                && image.internal_closure_issued
        }));
    }

    #[test]
    fn audit_discharge_is_local_and_does_not_weaken_the_full_gate() {
        let audit = issue_t_sm1a_audit_v4().expect("T-SM1a audit");
        assert!(audit.t_sm1a_passed);
        assert_eq!(audit.derived_count, 54);
        assert_eq!(audit.named_gap_count, 0);
        assert_eq!(audit.source_formation_coordinates.len(), 6);
        assert_eq!(audit.older_interface_coordinates.len(), 9);
        assert!(audit.exact_v3_formation_gap_surface_recovered);
        assert!(audit.every_post_substitution_internal_replayed);
        assert!(audit.every_exact_open_substitution_result_replayed);
        assert!(!audit.candidate_fresh_formation_admitted);
        assert!(audit.fixed_f_sm1_positive_gate_unchanged);
        assert!(!audit.full_f_sm1_passed);
        assert!(!audit.bi0_prerequisite_reopened);
    }

    #[test]
    fn context_source_beta_and_slot_map_mutations_all_fail_replay() {
        let (signature, projection) = pointwise_case();
        replay_chronological_formation_specialization_v4(&signature, &projection)
            .expect("unmodified pointwise projection replays");

        let mut context_mutation = projection.clone();
        context_mutation.specialized_internal.declared_context[0] = KernelTyProjectionV2::Neutral;
        context_mutation.specialized_internal.derivation_hash = tagged_hash(
            "contextual-formation-body",
            &context_mutation.specialized_internal,
        );
        context_mutation.derivation_hash =
            tagged_hash("chronological-formation-specialization", &context_mutation);
        assert!(
            replay_chronological_formation_specialization_v4(&signature, &context_mutation)
                .is_err()
        );

        let mut source_mutation = projection.clone();
        source_mutation.older_source.anchor_id.push_str("-forged");
        source_mutation.derivation_hash =
            tagged_hash("chronological-formation-specialization", &source_mutation);
        assert!(
            replay_chronological_formation_specialization_v4(&signature, &source_mutation).is_err()
        );

        let mut beta_mutation = projection.clone();
        beta_mutation.first_image_internal.raw_expression = Expr::Var(1);
        beta_mutation.first_image_internal.derivation_hash = tagged_hash(
            "chronological-image-internal",
            &beta_mutation.first_image_internal,
        );
        beta_mutation.derivation_hash =
            tagged_hash("chronological-formation-specialization", &beta_mutation);
        assert!(
            replay_chronological_formation_specialization_v4(&signature, &beta_mutation).is_err()
        );

        let mut slot_mutation = projection;
        slot_mutation.slot_map.assignments[0].parameter = 2;
        slot_mutation.derivation_hash =
            tagged_hash("chronological-formation-specialization", &slot_mutation);
        assert!(
            replay_chronological_formation_specialization_v4(&signature, &slot_mutation).is_err()
        );
    }
}
