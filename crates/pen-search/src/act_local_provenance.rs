//! Candidate-local ordinary-charge provenance (`T-BI-NU1`).
//!
//! The issuer in this module has exactly two semantic inputs: an exact sealed
//! prefix and the candidate act over that prefix.  It does not read a Genesis
//! winner, a historical total, a bar, an enacted future, or a full-history
//! archive.  Historical artifacts may compare the completed output in a
//! separate module, but cannot occur in a token derivation.
//!
//! A structural-novelty scalar is never split into anonymous ordinals here.
//! Each unit is reconstructed as an explicitly indexed schema role (a clause
//! role, a path-axis pair, a reference pair, or transport of an already
//! certified source family).  The role is attached to a kernel-extracted,
//! normalized natural family.  A fresh role receives an injective local
//! `(clause, LocalRole)` anchor.  A uniform instance, repeated family, or
//! occupied local role must instead answer a distinct output position of an
//! independently exported A3 structural-completion orbit which existed over
//! the prefix before the candidate was inspected.

use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3HistoricalWindow, A3RuleConstructor,
    generate_a3_window_for_exact_prefix_unbounded,
};
use pen_eval::nu::structural_nu;
use pen_eval::semantic_provenance::{CreditMechanism, LocalRole};
use pen_eval::typed_families::{
    CandidateExtractionOutcome, ExtractedFamily, InstanceKind, extract_candidate_families,
    predecessor_closure,
};
use pen_type::elaborate::{KernelTy, SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::equality::{KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const ACT_LOCAL_PROVENANCE_SCHEMA: &str = "act-local-provenance-v1";
pub const ACT_LOCAL_PROVENANCE_DATE: &str = "2026-07-22";
pub const T_BI_NU1_THEOREM_ID: &str =
    "T-BI-NU1-candidate-local-structural-nu-family-orbit-decomposition";
pub const ACT_LOCAL_ORDINARY_TOKEN_VERSION: &str = "act-local-ordinary-charge-token-v1";
pub const R2_GENERATED_ACTION_THEOREM_ID: &str = "derived-action-generator-membership-rule-v1";

const R2_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_quotient_adjudications.md");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_PROVENANCE_SCHEMA, domain, value))
        .expect("act-local provenance evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspensionSchemaRole {
    Formation,
    NorthPoint,
    SouthPoint,
    Meridian,
    Eliminator,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "role")]
pub enum ActLocalSchemaRole {
    FoundationCompletion {
        clause: u16,
    },
    FoundationFormation {
        clause: u16,
    },
    FormerIntroductionHead {
        clause: u16,
    },
    FormerAdjoint {
        clause: u16,
    },
    FormerEliminator {
        clause: u16,
    },
    HitPrePathDeclaration {
        clause: u16,
    },
    HitFormationPackage {
        clause: u16,
        package_role: u8,
    },
    HitParametricFormationAction {
        clause: u16,
    },
    HitPathBeta {
        clause: u16,
    },
    HitKanCoherence {
        clause: u16,
        left_axis: u32,
        right_axis: u32,
    },
    HitPostPathFace {
        clause: u16,
    },
    HitCanonicalOperationAction {
        clause: u16,
        operation_slot: u16,
    },
    HitUnformedLocalFace {
        clause: u16,
    },
    HitUnformedPrefixAction {
        clause: u16,
        target_step: u32,
    },
    SuspensionOperation {
        clause: u16,
        operation: SuspensionSchemaRole,
    },
    MapSingleHead {
        clause: u16,
    },
    MapSingleAction {
        clause: u16,
    },
    MapPrecomposition {
        clause: u16,
    },
    MapPostcomposition {
        clause: u16,
    },
    MapReferenceCoherence {
        clause: u16,
        left_step: u32,
        right_step: u32,
    },
    ModalLocalDeclaration {
        clause: u16,
    },
    ModalUniformLegacyAction {
        clause: u16,
        target_step: u32,
    },
    ModalPairwiseCoherence {
        clause: u16,
        left_kind: String,
        right_kind: String,
    },
    AxiomaticIntroductionHead {
        clause: u16,
    },
    AxiomaticInheritedFamily {
        clause: u16,
        source_step: u32,
        source_family_id: String,
        source_credit_id: String,
    },
    AxiomaticLocalAndBridgeFace {
        clause: u16,
    },
    AxiomaticSupportBridge {
        clause: u16,
        left_step: u32,
        right_step: u32,
    },
    SynthesisLocalDeclaration {
        clause: u16,
    },
    SynthesisUniformTemporalAction {
        clause: u16,
        target_step: u32,
    },
    SynthesisDistributiveTransport {
        clause: u16,
        source_step: u32,
        source_family_id: String,
        source_credit_id: String,
    },
    SynthesisInfinitesimalShift {
        clause: u16,
        source_step: u32,
        source_path_clause: u16,
        left_axis: u32,
        right_axis: u32,
    },
    GenericIntroductionHead {
        clause: u16,
    },
    GenericAdjoint {
        clause: u16,
    },
    GenericEliminator {
        clause: u16,
    },
}

impl ActLocalSchemaRole {
    fn owner_clause(&self) -> u16 {
        match self {
            Self::FoundationCompletion { clause }
            | Self::FoundationFormation { clause }
            | Self::FormerIntroductionHead { clause }
            | Self::FormerAdjoint { clause }
            | Self::FormerEliminator { clause }
            | Self::HitPrePathDeclaration { clause }
            | Self::HitFormationPackage { clause, .. }
            | Self::HitParametricFormationAction { clause }
            | Self::HitPathBeta { clause }
            | Self::HitKanCoherence { clause, .. }
            | Self::HitPostPathFace { clause }
            | Self::HitCanonicalOperationAction { clause, .. }
            | Self::HitUnformedLocalFace { clause }
            | Self::HitUnformedPrefixAction { clause, .. }
            | Self::SuspensionOperation { clause, .. }
            | Self::MapSingleHead { clause }
            | Self::MapSingleAction { clause }
            | Self::MapPrecomposition { clause }
            | Self::MapPostcomposition { clause }
            | Self::MapReferenceCoherence { clause, .. }
            | Self::ModalLocalDeclaration { clause }
            | Self::ModalUniformLegacyAction { clause, .. }
            | Self::ModalPairwiseCoherence { clause, .. }
            | Self::AxiomaticIntroductionHead { clause }
            | Self::AxiomaticInheritedFamily { clause, .. }
            | Self::AxiomaticLocalAndBridgeFace { clause }
            | Self::AxiomaticSupportBridge { clause, .. }
            | Self::SynthesisLocalDeclaration { clause }
            | Self::SynthesisUniformTemporalAction { clause, .. }
            | Self::SynthesisDistributiveTransport { clause, .. }
            | Self::SynthesisInfinitesimalShift { clause, .. }
            | Self::GenericIntroductionHead { clause }
            | Self::GenericAdjoint { clause }
            | Self::GenericEliminator { clause } => *clause,
        }
    }

    fn is_uniform_instance(&self) -> bool {
        matches!(
            self,
            Self::HitUnformedPrefixAction { .. }
                | Self::ModalUniformLegacyAction { .. }
                | Self::SynthesisUniformTemporalAction { .. }
        )
    }

    /// The natural-family shape deliberately erases only a uniform target.
    /// All other coordinates distinguish actual schema roles (axes, source
    /// families, or ordered interfaces), not presentation occurrences.
    fn family_shape(&self) -> serde_json::Value {
        match self {
            Self::HitUnformedPrefixAction { clause, .. } => serde_json::json!({
                "role": "hit_unformed_prefix_action",
                "clause": clause,
                "uniform_target_erased": true,
            }),
            Self::ModalUniformLegacyAction { clause, .. } => serde_json::json!({
                "role": "modal_uniform_legacy_action",
                "clause": clause,
                "uniform_target_erased": true,
            }),
            Self::SynthesisUniformTemporalAction { clause, .. } => serde_json::json!({
                "role": "synthesis_uniform_temporal_action",
                "clause": clause,
                "uniform_target_erased": true,
            }),
            Self::AxiomaticInheritedFamily {
                clause,
                source_step,
                source_family_id,
                ..
            } => serde_json::json!({
                "role": "axiomatic_inherited_family",
                "clause": clause,
                "source_step": source_step,
                "source_family_id": source_family_id,
                "exported_source_credit_erased_from_natural_family": true,
            }),
            Self::SynthesisDistributiveTransport {
                clause,
                source_step,
                source_family_id,
                ..
            } => serde_json::json!({
                "role": "synthesis_distributive_transport",
                "clause": clause,
                "source_step": source_step,
                "source_family_id": source_family_id,
                "exported_source_credit_erased_from_natural_family": true,
            }),
            _ => serde_json::to_value(self).expect("schema role serializes"),
        }
    }
}

#[derive(Clone, Debug)]
struct FormulaCell {
    mechanism: CreditMechanism,
    local_role: LocalRole,
    role: ActLocalSchemaRole,
    generated_instance_removed_by_r2: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalGeneratorFamilyEvidence {
    pub owner_clause: u16,
    pub extractor_family_id: String,
    pub generator_univalent_key: String,
    pub canonical_normal_form: Expr,
    pub parameter_sorts: Vec<String>,
    pub kernel_type_json: String,
    pub candidate_extraction_hash: String,
    pub normalization_and_typing_replayed: bool,
    pub naturality_square_hash: String,
    pub naturality_square_equal: bool,
    pub clause_is_generator_or_family_instance: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalMarginalityEvidence {
    pub equality_procedure: String,
    pub predecessor_token_count: usize,
    pub same_shape_comparison_count: usize,
    pub univalent_preimage_count: usize,
    pub no_predecessor_or_earlier_act_preimage: bool,
    pub full_sweep_hash: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActLocalCreditKind {
    MarginalNaturalFamily,
    LiveDemandFamilyOutput,
    IndependentlyExportedUniformInstance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "anchor")]
pub enum ActLocalProvenanceAnchor {
    ChargedLocalRole {
        clause: u16,
        role: LocalRole,
        injection_hash: String,
    },
    LiveDemandOutput {
        orbit_id: String,
        scheme_id: String,
        orbit_derivation_hash: String,
        output_position_id: String,
        output_type_hash: String,
        demand_window_hash: String,
        demand_existed_before_candidate: bool,
        independently_exported_demand_orbit: bool,
        uniform_specializations_collapsed_at_orbit: bool,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalOrdinaryFamilyToken {
    pub token_version: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub mechanism: CreditMechanism,
    pub schema_role: ActLocalSchemaRole,
    pub family_shape_key: String,
    pub semantic_family_id: String,
    pub specialization_instance_id: Option<String>,
    pub generator: ActLocalGeneratorFamilyEvidence,
    pub marginality: ActLocalMarginalityEvidence,
    pub credit_kind: ActLocalCreditKind,
    pub anchor: ActLocalProvenanceAnchor,
    pub typed_normalized_natural: bool,
    pub family_or_instance_decided: bool,
    pub uniform_specialization_multiplied_without_exported_demand: bool,
    pub candidate_and_prefix_are_only_semantic_inputs: bool,
    pub enacted_future_used: bool,
    pub archive_join_used: bool,
    pub historical_total_used_as_input: bool,
    pub bar_or_verdict_used_as_input: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalGeneratedInstanceRemoval {
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub generated_schema_role: ActLocalSchemaRole,
    pub generated_family_id: String,
    pub parent_family_id: String,
    pub path_clause_index: u16,
    pub parent_operation_clause_index: u16,
    pub path_dimension: u32,
    pub candidate_elaboration_hash: String,
    pub path_clause_is_typed_path: bool,
    pub parent_operation_is_typed_introduction: bool,
    pub adopted_rule_source_hash: String,
    pub count_bar_archive_or_future_used: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalNuProvenanceCertificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub predecessor_act_local_digest: String,
    pub candidate_elaboration_hash: String,
    pub candidate_family_extraction_hash: String,
    pub a3_demand_window_hash: String,
    pub independent_structural_demand_orbit_ids: Vec<String>,
    pub class: TelescopeClass,
    pub kappa: u32,
    pub structural_formula_total: u32,
    pub formula_cell_count_before_quotient: u32,
    pub generated_instance_adjustment: i32,
    pub exact_certified_nu: u32,
    pub ordinary_family_tokens: Vec<ActLocalOrdinaryFamilyToken>,
    pub generated_instance_family_ids_removed_by_quotient: Vec<String>,
    pub generated_instance_removals: Vec<ActLocalGeneratedInstanceRemoval>,
    pub every_formula_cell_has_explicit_semantic_coordinates: bool,
    pub every_counted_unit_typed_normalized_natural: bool,
    pub local_role_injection_holds: bool,
    pub demand_output_injection_holds: bool,
    pub every_uniform_or_repeated_unit_has_independent_demand_output: bool,
    pub transportable_on_same_act_and_prefix: bool,
    pub archive_future_total_bar_or_verdict_read_by_issuer: bool,
    pub authoritative: bool,
    pub derivation_hash: String,
}

impl ActLocalNuProvenanceCertificate {
    pub fn authoritative_token_hashes(&self) -> Vec<String> {
        self.ordinary_family_tokens
            .iter()
            .map(|token| token.derivation_hash.clone())
            .collect()
    }

    pub fn counted_family_ids(&self) -> Vec<String> {
        self.ordinary_family_tokens
            .iter()
            .map(|token| token.semantic_family_id.clone())
            .collect()
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ActLocalProvenanceError {
    #[error("invalid act-local input: {0}")]
    Input(String),
    #[error("kernel/family extraction failed: {0}")]
    Family(String),
    #[error("formula decomposition failed: {0}")]
    Decomposition(String),
    #[error("provenance injection failed: {0}")]
    Injection(String),
}

fn role_mechanism(role: &ActLocalSchemaRole) -> (CreditMechanism, LocalRole) {
    use ActLocalSchemaRole as R;
    match role {
        R::FormerAdjoint { .. }
        | R::FormerEliminator { .. }
        | R::MapPrecomposition { .. }
        | R::MapPostcomposition { .. }
        | R::MapSingleAction { .. } => (CreditMechanism::AdjointCompletion, LocalRole::AdjointMate),
        R::HitKanCoherence { .. } => (CreditMechanism::DimensionSquared, LocalRole::Coherence),
        R::MapReferenceCoherence { .. } => {
            (CreditMechanism::ReferenceSquared, LocalRole::Coherence)
        }
        R::ModalUniformLegacyAction { .. }
        | R::SynthesisUniformTemporalAction { .. }
        | R::HitUnformedPrefixAction { .. } => (
            CreditMechanism::P6UniformSpecialization,
            LocalRole::SupportAction,
        ),
        R::ModalPairwiseCoherence { .. } => (
            CreditMechanism::ModalPairwiseCoherence,
            LocalRole::Coherence,
        ),
        R::AxiomaticInheritedFamily { .. } => (
            CreditMechanism::P5InheritedSurface,
            LocalRole::SupportAction,
        ),
        R::AxiomaticLocalAndBridgeFace { .. } | R::AxiomaticSupportBridge { .. } => {
            (CreditMechanism::P5LocalAndBridge, LocalRole::SupportAction)
        }
        R::SynthesisDistributiveTransport { .. } => (
            CreditMechanism::DistributiveInheritance,
            LocalRole::Coherence,
        ),
        R::SynthesisInfinitesimalShift { .. } => (
            CreditMechanism::CombinatorialSynthesis,
            LocalRole::SupportAction,
        ),
        R::HitCanonicalOperationAction { .. }
        | R::HitParametricFormationAction { .. }
        | R::HitPostPathFace { .. } => (
            CreditMechanism::P6UniformSpecialization,
            LocalRole::SupportAction,
        ),
        R::HitFormationPackage {
            package_role: 1, ..
        } => (CreditMechanism::AdjointCompletion, LocalRole::AdjointMate),
        R::HitFormationPackage {
            package_role: 2, ..
        } => (CreditMechanism::IntrinsicKernel, LocalRole::SupportAction),
        R::SuspensionOperation {
            operation: SuspensionSchemaRole::Eliminator,
            ..
        } => (CreditMechanism::AdjointCompletion, LocalRole::AdjointMate),
        R::SuspensionOperation {
            operation: SuspensionSchemaRole::Meridian,
            ..
        } => (CreditMechanism::DimensionSquared, LocalRole::Coherence),
        R::SuspensionOperation {
            operation: SuspensionSchemaRole::SouthPoint,
            ..
        } => (CreditMechanism::IntrinsicKernel, LocalRole::SupportAction),
        _ => (CreditMechanism::IntrinsicKernel, LocalRole::KernelHead),
    }
}

fn cell(role: ActLocalSchemaRole, generated_instance_removed_by_r2: bool) -> FormulaCell {
    let (mechanism, local_role) = role_mechanism(&role);
    FormulaCell {
        mechanism,
        local_role,
        role,
        generated_instance_removed_by_r2,
    }
}

fn as_u16(index: usize) -> Result<u16, ActLocalProvenanceError> {
    u16::try_from(index)
        .map_err(|_| ActLocalProvenanceError::Decomposition("clause index exceeds u16".to_owned()))
}

fn is_type_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ)
        || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ))
        || matches!(expr, Expr::Trunc(_))
}

fn is_intro(expr: &Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) | Expr::Lam(_) | Expr::Var(_) => true,
        Expr::App(function, _) => !matches!(function.as_ref(), Expr::Lam(_)),
        _ => false,
    }
}

fn is_elim(expr: &Expr) -> bool {
    matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)))
}

fn has_direct_lib(expr: &Expr) -> bool {
    match expr {
        Expr::Lib(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            has_direct_lib(left) || has_direct_lib(right)
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => has_direct_lib(inner),
        Expr::Id(ty, left, right) => {
            has_direct_lib(ty) || has_direct_lib(left) || has_direct_lib(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::PathCon(_) => false,
    }
}

fn has_operator_ref(expr: &Expr) -> bool {
    match expr {
        Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Bang(_)
        | Expr::WhyNot(_) => true,
        Expr::Pi(left, right) | Expr::Sigma(left, right) | Expr::App(left, right) => {
            has_operator_ref(left) || has_operator_ref(right)
        }
        Expr::Lam(inner) | Expr::Refl(inner) | Expr::Susp(inner) | Expr::Trunc(inner) => {
            has_operator_ref(inner)
        }
        Expr::Id(ty, left, right) => {
            has_operator_ref(ty) || has_operator_ref(left) || has_operator_ref(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn pi_is_preservation(domain: &Expr, codomain: &Expr) -> bool {
    matches!((domain, codomain), (Expr::Var(left), Expr::Var(right)) if left == right)
}

fn is_axiomatic_intro(expr: &Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) | Expr::Lam(_) | Expr::Var(_) => true,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_) | Expr::Lib(_)) => false,
        Expr::App(_, _) => true,
        Expr::Pi(domain, codomain) => {
            !has_direct_lib(domain)
                && !has_direct_lib(codomain)
                && !has_operator_ref(domain)
                && !has_operator_ref(codomain)
                && !pi_is_preservation(domain, codomain)
        }
        _ => false,
    }
}

fn is_parametric_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(_)))
}

fn is_modal_wrapping_temporal(expr: &Expr) -> bool {
    match expr {
        Expr::Flat(inner) | Expr::Sharp(inner) | Expr::Disc(inner) | Expr::Shape(inner) => {
            matches!(
                inner.as_ref(),
                Expr::Next(_) | Expr::Eventually(_) | Expr::Bang(_) | Expr::WhyNot(_)
            )
        }
        _ => false,
    }
}

fn is_temporal_wrapping_modal(expr: &Expr) -> bool {
    match expr {
        Expr::Next(inner) | Expr::Eventually(inner) | Expr::Bang(inner) | Expr::WhyNot(inner) => {
            matches!(
                inner.as_ref(),
                Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_)
            )
        }
        _ => false,
    }
}

fn is_distributive_law(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(domain, codomain)
        if (is_modal_wrapping_temporal(domain) && is_temporal_wrapping_modal(codomain))
            || (is_temporal_wrapping_modal(domain) && is_modal_wrapping_temporal(codomain)))
}

fn is_polymorphic_temporal_elim(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::App(function, _) if matches!(function.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))))
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Next(inner2) | Expr::Bang(inner2) if matches!(inner2.as_ref(), Expr::Var(_))))
                    && matches!(codomain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                    && matches!(codomain.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
}

fn is_spatial_temporal_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(_))
                        && matches!(argument.as_ref(), Expr::Next(inner) | Expr::Eventually(inner) | Expr::Bang(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))
            )
    )
}

fn clause_for_ref(telescope: &Telescope, step: u32) -> Result<u16, ActLocalProvenanceError> {
    telescope
        .clauses
        .iter()
        .position(|clause| clause.expr.lib_refs().contains(&step))
        .ok_or_else(|| {
            ActLocalProvenanceError::Decomposition(format!(
                "candidate reference {step} has no owning clause"
            ))
        })
        .and_then(as_u16)
}

fn modal_kinds(telescope: &Telescope) -> Vec<(String, u16)> {
    let mut kinds = Vec::new();
    for (name, index) in [
        (
            "flat",
            telescope
                .clauses
                .iter()
                .position(|clause| matches!(clause.expr, Expr::Flat(_))),
        ),
        (
            "sharp",
            telescope
                .clauses
                .iter()
                .position(|clause| matches!(clause.expr, Expr::Sharp(_))),
        ),
        (
            "disc",
            telescope
                .clauses
                .iter()
                .position(|clause| matches!(clause.expr, Expr::Disc(_))),
        ),
        (
            "shape",
            telescope
                .clauses
                .iter()
                .position(|clause| matches!(clause.expr, Expr::Shape(_))),
        ),
    ] {
        if let Some(index) = index.and_then(|index| u16::try_from(index).ok()) {
            kinds.push((name.to_owned(), index));
        }
    }
    kinds
}

fn dominant_reference(
    refs: impl IntoIterator<Item = u32>,
    history: &[(u32, u32)],
    predicate: impl Fn(u32) -> bool,
) -> Option<u32> {
    refs.into_iter()
        .filter(|step| predicate(*step))
        .filter_map(|step| {
            history
                .iter()
                .find(|(candidate, _)| *candidate == step)
                .map(|(_, total)| (step, *total))
        })
        // Sealing order is the only tie rule.  The total is computed from the
        // prefix, not read from testimony.
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(&left.0)))
        .map(|(step, _)| step)
}

fn formula_cells(
    candidate: &Telescope,
    library: &Library,
    history: &[(u32, u32)],
    prefix_packages: &[ActLocalNuProvenanceCertificate],
    prefix_telescopes: &[(u32, Telescope)],
) -> Result<Vec<FormulaCell>, ActLocalProvenanceError> {
    let formula = structural_nu(candidate, library, history);
    if formula.total == 0 {
        return Ok(Vec::new());
    }
    let class = candidate.classify(library);
    let mut cells = Vec::new();
    match class {
        TelescopeClass::Foundation => {
            if let Some(universe) = candidate
                .clauses
                .iter()
                .position(|clause| matches!(clause.expr, Expr::Univ))
            {
                for index in 0..candidate.clauses.len() {
                    if index != universe {
                        cells.push(cell(
                            ActLocalSchemaRole::FoundationCompletion {
                                clause: as_u16(index)?,
                            },
                            false,
                        ));
                    }
                }
            } else {
                for (index, clause) in candidate.clauses.iter().enumerate() {
                    if is_type_formation(&clause.expr) {
                        cells.push(cell(
                            ActLocalSchemaRole::FoundationFormation {
                                clause: as_u16(index)?,
                            },
                            false,
                        ));
                    }
                }
            }
        }
        TelescopeClass::Former => {
            for (index, clause) in candidate.clauses.iter().enumerate() {
                let clause_index = as_u16(index)?;
                if is_intro(&clause.expr) {
                    cells.push(cell(
                        ActLocalSchemaRole::FormerIntroductionHead {
                            clause: clause_index,
                        },
                        false,
                    ));
                    cells.push(cell(
                        ActLocalSchemaRole::FormerAdjoint {
                            clause: clause_index,
                        },
                        false,
                    ));
                }
                if is_elim(&clause.expr) {
                    cells.push(cell(
                        ActLocalSchemaRole::FormerEliminator {
                            clause: clause_index,
                        },
                        false,
                    ));
                }
            }
        }
        TelescopeClass::Hit => {
            let path_indices = candidate
                .clauses
                .iter()
                .enumerate()
                .filter_map(|(index, clause)| match clause.expr {
                    Expr::PathCon(dimension) => Some((index, dimension)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let (first_path, _) = path_indices.first().copied().ok_or_else(|| {
                ActLocalProvenanceError::Decomposition("HIT has no path clause".to_owned())
            })?;
            let formation = candidate
                .clauses
                .iter()
                .position(|clause| is_type_formation(&clause.expr));
            if let Some(formation_index) = formation {
                for index in 0..first_path {
                    cells.push(cell(
                        ActLocalSchemaRole::HitPrePathDeclaration {
                            clause: as_u16(index)?,
                        },
                        false,
                    ));
                }
                let formation_clause = as_u16(formation_index)?;
                for package_role in 0..3_u8 {
                    cells.push(cell(
                        ActLocalSchemaRole::HitFormationPackage {
                            clause: formation_clause,
                            package_role,
                        },
                        false,
                    ));
                }
                if candidate
                    .clauses
                    .iter()
                    .any(|clause| is_parametric_formation(&clause.expr))
                {
                    cells.push(cell(
                        ActLocalSchemaRole::HitParametricFormationAction {
                            clause: formation_clause,
                        },
                        false,
                    ));
                }
            }
            for (index, dimension) in &path_indices {
                let clause = as_u16(*index)?;
                cells.push(cell(ActLocalSchemaRole::HitPathBeta { clause }, false));
                for left_axis in 0..*dimension {
                    for right_axis in 0..*dimension {
                        cells.push(cell(
                            ActLocalSchemaRole::HitKanCoherence {
                                clause,
                                left_axis,
                                right_axis,
                            },
                            false,
                        ));
                    }
                }
            }
            if formation.is_some() {
                let post = (first_path + 1..candidate.clauses.len()).collect::<Vec<_>>();
                for index in &post {
                    cells.push(cell(
                        ActLocalSchemaRole::HitPostPathFace {
                            clause: as_u16(*index)?,
                        },
                        false,
                    ));
                }
                for operation_slot in 0..post.len().div_ceil(2) {
                    let index = post[2 * operation_slot];
                    cells.push(cell(
                        ActLocalSchemaRole::HitCanonicalOperationAction {
                            clause: as_u16(index)?,
                            operation_slot: as_u16(operation_slot)?,
                        },
                        true,
                    ));
                }
            } else {
                let path_owner = as_u16(first_path)?;
                for index in 0..candidate.clauses.len() {
                    cells.push(cell(
                        ActLocalSchemaRole::HitUnformedLocalFace {
                            clause: as_u16(index)?,
                        },
                        false,
                    ));
                }
                for target_step in 1..=library.len() as u32 {
                    cells.push(cell(
                        ActLocalSchemaRole::HitUnformedPrefixAction {
                            clause: path_owner,
                            target_step,
                        },
                        false,
                    ));
                }
            }
        }
        TelescopeClass::Suspension => {
            let owner = 0;
            for operation in [
                SuspensionSchemaRole::Formation,
                SuspensionSchemaRole::NorthPoint,
                SuspensionSchemaRole::SouthPoint,
                SuspensionSchemaRole::Meridian,
                SuspensionSchemaRole::Eliminator,
            ] {
                cells.push(cell(
                    ActLocalSchemaRole::SuspensionOperation {
                        clause: owner,
                        operation,
                    },
                    false,
                ));
            }
        }
        TelescopeClass::Map => {
            if candidate.kappa() == 1 {
                cells.push(cell(ActLocalSchemaRole::MapSingleHead { clause: 0 }, false));
                cells.push(cell(
                    ActLocalSchemaRole::MapSingleAction { clause: 0 },
                    false,
                ));
            } else {
                for index in 0..candidate.clauses.len() {
                    let clause = as_u16(index)?;
                    cells.push(cell(
                        ActLocalSchemaRole::MapPrecomposition { clause },
                        false,
                    ));
                    cells.push(cell(
                        ActLocalSchemaRole::MapPostcomposition { clause },
                        false,
                    ));
                }
                let refs = candidate.lib_refs().into_iter().collect::<Vec<_>>();
                for left_step in &refs {
                    for right_step in &refs {
                        cells.push(cell(
                            ActLocalSchemaRole::MapReferenceCoherence {
                                clause: clause_for_ref(candidate, *left_step)?,
                                left_step: *left_step,
                                right_step: *right_step,
                            },
                            false,
                        ));
                    }
                }
            }
        }
        TelescopeClass::Modal => {
            for index in 0..candidate.clauses.len() {
                cells.push(cell(
                    ActLocalSchemaRole::ModalLocalDeclaration {
                        clause: as_u16(index)?,
                    },
                    false,
                ));
            }
            let kinds = modal_kinds(candidate);
            let owner = kinds.first().map(|(_, clause)| *clause).ok_or_else(|| {
                ActLocalProvenanceError::Decomposition("modal act has no modal owner".to_owned())
            })?;
            for target_step in 1..=library.len() as u32 {
                cells.push(cell(
                    ActLocalSchemaRole::ModalUniformLegacyAction {
                        clause: owner,
                        target_step,
                    },
                    false,
                ));
            }
            for left in 0..kinds.len() {
                for right in left + 1..kinds.len() {
                    cells.push(cell(
                        ActLocalSchemaRole::ModalPairwiseCoherence {
                            clause: kinds[left].1,
                            left_kind: kinds[left].0.clone(),
                            right_kind: kinds[right].0.clone(),
                        },
                        false,
                    ));
                }
            }
        }
        TelescopeClass::Axiomatic => {
            for (index, clause) in candidate.clauses.iter().enumerate() {
                if is_axiomatic_intro(&clause.expr) {
                    cells.push(cell(
                        ActLocalSchemaRole::AxiomaticIntroductionHead {
                            clause: as_u16(index)?,
                        },
                        false,
                    ));
                }
            }
            let refs = candidate.lib_refs().into_iter().collect::<Vec<_>>();
            if let Some(source_step) = dominant_reference(refs.iter().copied(), history, |_| true) {
                let source = prefix_packages
                    .iter()
                    .find(|package| package.stage == source_step)
                    .ok_or_else(|| {
                        ActLocalProvenanceError::Decomposition(format!(
                            "dominant prefix step {source_step} lacks act-local provenance"
                        ))
                    })?;
                let clause = clause_for_ref(candidate, source_step)?;
                for source_token in &source.ordinary_family_tokens {
                    cells.push(cell(
                        ActLocalSchemaRole::AxiomaticInheritedFamily {
                            clause,
                            source_step,
                            source_family_id: source_token.semantic_family_id.clone(),
                            source_credit_id: source_token.derivation_hash.clone(),
                        },
                        false,
                    ));
                }
            }
            for index in 0..candidate.clauses.len() {
                cells.push(cell(
                    ActLocalSchemaRole::AxiomaticLocalAndBridgeFace {
                        clause: as_u16(index)?,
                    },
                    false,
                ));
            }
            for pair in refs.windows(2) {
                cells.push(cell(
                    ActLocalSchemaRole::AxiomaticSupportBridge {
                        clause: clause_for_ref(candidate, pair[1])?,
                        left_step: pair[0],
                        right_step: pair[1],
                    },
                    false,
                ));
            }
        }
        TelescopeClass::Synthesis => {
            for index in 0..candidate.clauses.len() {
                cells.push(cell(
                    ActLocalSchemaRole::SynthesisLocalDeclaration {
                        clause: as_u16(index)?,
                    },
                    false,
                ));
            }
            for (index, clause) in candidate.clauses.iter().enumerate() {
                if is_polymorphic_temporal_elim(&clause.expr) {
                    for target_step in 1..=library.len() as u32 {
                        cells.push(cell(
                            ActLocalSchemaRole::SynthesisUniformTemporalAction {
                                clause: as_u16(index)?,
                                target_step,
                            },
                            false,
                        ));
                    }
                }
            }
            let modal_source = dominant_reference(candidate.lib_refs(), history, |step| {
                library
                    .get(step.saturating_sub(1) as usize)
                    .is_some_and(|entry| entry.capabilities.has_modal_ops)
            });
            if let Some(source_step) = modal_source {
                let source = prefix_packages
                    .iter()
                    .find(|package| package.stage == source_step)
                    .ok_or_else(|| {
                        ActLocalProvenanceError::Decomposition(format!(
                            "modal prefix step {source_step} lacks act-local provenance"
                        ))
                    })?;
                for (index, clause) in candidate.clauses.iter().enumerate() {
                    if is_distributive_law(&clause.expr) {
                        for source_token in &source.ordinary_family_tokens {
                            cells.push(cell(
                                ActLocalSchemaRole::SynthesisDistributiveTransport {
                                    clause: as_u16(index)?,
                                    source_step,
                                    source_family_id: source_token.semantic_family_id.clone(),
                                    source_credit_id: source_token.derivation_hash.clone(),
                                },
                                false,
                            ));
                        }
                    }
                }
            }
            if let Some(owner) = candidate
                .clauses
                .iter()
                .position(|clause| is_spatial_temporal_clause(&clause.expr))
            {
                let owner = as_u16(owner)?;
                for (source_step, source_telescope) in prefix_telescopes {
                    for (path_index, path_clause) in source_telescope.clauses.iter().enumerate() {
                        let Expr::PathCon(dimension) = path_clause.expr else {
                            continue;
                        };
                        for left_axis in 0..dimension {
                            for right_axis in 0..dimension {
                                cells.push(cell(
                                    ActLocalSchemaRole::SynthesisInfinitesimalShift {
                                        clause: owner,
                                        source_step: *source_step,
                                        source_path_clause: as_u16(path_index)?,
                                        left_axis,
                                        right_axis,
                                    },
                                    false,
                                ));
                            }
                        }
                    }
                }
            }
        }
        TelescopeClass::Unknown => {
            for (index, clause) in candidate.clauses.iter().enumerate() {
                let clause_index = as_u16(index)?;
                if is_intro(&clause.expr) {
                    cells.push(cell(
                        ActLocalSchemaRole::GenericIntroductionHead {
                            clause: clause_index,
                        },
                        false,
                    ));
                    cells.push(cell(
                        ActLocalSchemaRole::GenericAdjoint {
                            clause: clause_index,
                        },
                        false,
                    ));
                }
                if is_elim(&clause.expr) {
                    cells.push(cell(
                        ActLocalSchemaRole::GenericEliminator {
                            clause: clause_index,
                        },
                        false,
                    ));
                }
            }
        }
    }
    if cells.len() as u32 != formula.total {
        return Err(ActLocalProvenanceError::Decomposition(format!(
            "explicit semantic cells {} do not reconstruct structural formula {} for class {class:?}",
            cells.len(),
            formula.total
        )));
    }
    Ok(cells)
}

fn structural_demand_orbits<'a>(
    window: &'a A3HistoricalWindow,
) -> Result<Vec<(&'a str, &'a str, &'a str)>, ActLocalProvenanceError> {
    let schemes = window
        .schemes
        .iter()
        .map(|scheme| (scheme.scheme_id.as_str(), scheme))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    for orbit in &window.orbits {
        if !orbit.independently_exported_demand_orbit {
            continue;
        }
        let scheme = schemes.get(orbit.scheme_id.as_str()).ok_or_else(|| {
            ActLocalProvenanceError::Injection(format!("A3 orbit {} has no scheme", orbit.orbit_id))
        })?;
        if !matches!(
            scheme.origin,
            A3DemandSchemeOrigin::StructuralCompletion { .. }
        ) || scheme.rule_constructor != A3RuleConstructor::StructuralCompletionHole
        {
            return Err(ActLocalProvenanceError::Injection(format!(
                "independently exported orbit {} is not structural completion",
                orbit.orbit_id
            )));
        }
        rows.push((
            orbit.orbit_id.as_str(),
            orbit.scheme_id.as_str(),
            orbit.orbit_derivation_hash.as_str(),
        ));
    }
    Ok(rows)
}

fn generator_for_clause<'a>(
    families: &'a [ExtractedFamily],
    clause: u16,
) -> Option<&'a ExtractedFamily> {
    families.iter().find(|family| {
        family
            .instances
            .iter()
            .any(|instance| instance.clause_index == clause)
    })
}

fn generator_evidence(
    family: &ExtractedFamily,
    clause: u16,
    extraction_hash: &str,
) -> ActLocalGeneratorFamilyEvidence {
    let parameter_sorts = family
        .presentation
        .parameters
        .iter()
        .map(|sort| format!("{sort:?}"))
        .collect::<Vec<_>>();
    let kernel_type_json =
        serde_json::to_string(&family.generator_kernel_ty).expect("kernel type serializes");
    let generator_univalent_key = tagged_hash(
        "generator-univalent-key",
        &(
            &family.presentation.canonical_normal_form,
            &parameter_sorts,
            &kernel_type_json,
        ),
    );
    let naturality_square_hash = tagged_hash("generator-naturality-square", &family.naturality);
    let clause_is_generator_or_family_instance = family.instances.iter().any(|record| {
        record.clause_index == clause
            && matches!(
                record.kind,
                InstanceKind::Generator
                    | InstanceKind::RenamingInstance
                    | InstanceKind::Specialization { .. }
            )
    });
    let mut evidence = ActLocalGeneratorFamilyEvidence {
        owner_clause: clause,
        extractor_family_id: family.id.as_str().to_owned(),
        generator_univalent_key,
        canonical_normal_form: family.presentation.canonical_normal_form.clone(),
        parameter_sorts,
        kernel_type_json,
        candidate_extraction_hash: extraction_hash.to_owned(),
        normalization_and_typing_replayed: true,
        naturality_square_hash,
        naturality_square_equal: family.naturality.square.equal,
        clause_is_generator_or_family_instance,
        derivation_hash: String::new(),
    };
    evidence.derivation_hash = tagged_hash("generator-family-evidence", &evidence);
    evidence
}

fn marginality_sweep(
    family_shape_key: &str,
    generator: &ActLocalGeneratorFamilyEvidence,
    prior: &[ActLocalOrdinaryFamilyToken],
) -> Result<ActLocalMarginalityEvidence, ActLocalProvenanceError> {
    let scope = generator.parameter_sorts.len() as u32;
    let mut comparisons = Vec::with_capacity(prior.len());
    let mut same_shape_comparison_count = 0;
    let mut preimage_count = 0;
    for token in prior {
        let same_shape = token.family_shape_key == family_shape_key
            && token.generator.parameter_sorts == generator.parameter_sorts
            && token.generator.kernel_type_json == generator.kernel_type_json;
        let equality = if same_shape {
            same_shape_comparison_count += 1;
            let witness = univalent_equality(
                &token.generator.canonical_normal_form,
                &generator.canonical_normal_form,
                scope,
                256,
            )
            .map_err(|error| ActLocalProvenanceError::Family(error.to_string()))?;
            preimage_count += usize::from(witness.equal);
            Some((
                witness.equal,
                tagged_hash("marginality-univalent-equality", &witness),
            ))
        } else {
            None
        };
        comparisons.push((
            token.derivation_hash.as_str(),
            token.semantic_family_id.as_str(),
            same_shape,
            equality,
        ));
    }
    Ok(ActLocalMarginalityEvidence {
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        predecessor_token_count: prior.len(),
        same_shape_comparison_count,
        univalent_preimage_count: preimage_count,
        no_predecessor_or_earlier_act_preimage: preimage_count == 0,
        full_sweep_hash: tagged_hash("full-predecessor-family-sweep", &comparisons),
    })
}

fn r2_adoption_hash() -> Result<String, ActLocalProvenanceError> {
    let text = std::str::from_utf8(R2_ADJUDICATION_BYTES)
        .map_err(|error| ActLocalProvenanceError::Input(error.to_string()))?;
    if !text.contains(R2_GENERATED_ACTION_THEOREM_ID)
        || !text.contains("**R2 adopted**")
        || !text.contains("position, label, or archived cardinality")
    {
        return Err(ActLocalProvenanceError::Input(
            "adopted count-blind R2 source did not replay".to_owned(),
        ));
    }
    Ok(bytes_hash(R2_ADJUDICATION_BYTES))
}

fn certificate_digest(certificate: &ActLocalNuProvenanceCertificate) -> String {
    let mut projection = certificate.clone();
    projection.derivation_hash.clear();
    tagged_hash("act-local-nu-provenance-certificate", &projection)
}

fn issue_one(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    library: &Library,
    history: &[(u32, u32)],
    prefix_packages: &[ActLocalNuProvenanceCertificate],
) -> Result<ActLocalNuProvenanceCertificate, ActLocalProvenanceError> {
    let elaboration = elaborate_telescope(prefix, candidate, stage.saturating_sub(1))
        .map_err(|error| ActLocalProvenanceError::Family(error.to_string()))?;
    let closure = predecessor_closure(prefix)
        .map_err(|error| ActLocalProvenanceError::Family(error.to_string()))?;
    let extraction =
        extract_candidate_families(prefix, &closure, candidate, stage.saturating_sub(1));
    let CandidateExtractionOutcome::Extracted(extraction) = extraction else {
        return Err(ActLocalProvenanceError::Family(
            "candidate family extraction returned KernelInvalid after elaboration".to_owned(),
        ));
    };
    let window = generate_a3_window_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| ActLocalProvenanceError::Family(error.to_string()))?;
    let independent_orbits = structural_demand_orbits(&window)?;
    if independent_orbits.len() > 1 {
        return Err(ActLocalProvenanceError::Injection(format!(
            "Stage {stage} has {} independently exported structural demand orbits; T-BI-NU1 does not improvise a demand selector",
            independent_orbits.len()
        )));
    }
    let prefix_telescopes = prefix
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.telescope.clone()))
        .collect::<Vec<_>>();
    let cells = formula_cells(
        candidate,
        library,
        history,
        prefix_packages,
        &prefix_telescopes,
    )?;
    let formula = structural_nu(candidate, library, history);
    let candidate_digest = candidate_hash(candidate);
    let predecessor_act_local_digest = tagged_hash(
        "predecessor-act-local-packages",
        &prefix_packages
            .iter()
            .map(|package| package.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let mut all_prior_tokens = prefix_packages
        .iter()
        .flat_map(|package| package.ordinary_family_tokens.iter().cloned())
        .collect::<Vec<_>>();
    let mut local_slots = BTreeSet::<(u16, LocalRole)>::new();
    let mut output_positions = BTreeSet::<String>::new();
    let mut tokens = Vec::new();
    let mut removals = Vec::new();
    let r2_hash = r2_adoption_hash()?;

    for formula_cell in &cells {
        let owner_clause = formula_cell.role.owner_clause();
        let family = generator_for_clause(&extraction.families, owner_clause).ok_or_else(|| {
            ActLocalProvenanceError::Family(format!(
                "formula role {:?} has no extracted owner family",
                formula_cell.role
            ))
        })?;
        let generator = generator_evidence(family, owner_clause, &extraction.derivation_hash);
        if !generator.naturality_square_equal
            || !generator.normalization_and_typing_replayed
            || !generator.clause_is_generator_or_family_instance
        {
            return Err(ActLocalProvenanceError::Family(format!(
                "formula role {:?} does not have typed/normalized/natural generator evidence",
                formula_cell.role
            )));
        }
        let family_shape = formula_cell.role.family_shape();
        let family_shape_key = tagged_hash("natural-family-shape", &family_shape);
        let semantic_family_id = tagged_hash(
            "act-local-semantic-family-id",
            &(&family_shape_key, &generator.generator_univalent_key),
        );
        if formula_cell.generated_instance_removed_by_r2 {
            let ActLocalSchemaRole::HitCanonicalOperationAction { clause, .. } = &formula_cell.role
            else {
                return Err(ActLocalProvenanceError::Decomposition(
                    "non-HIT cell marked as R2 generated".to_owned(),
                ));
            };
            let parent = tokens
                .iter()
                .rev()
                .find(|token: &&ActLocalOrdinaryFamilyToken| {
                    matches!(
                        token.schema_role,
                        ActLocalSchemaRole::HitPostPathFace { clause: parent_clause }
                            if parent_clause == *clause
                    )
                })
                .ok_or_else(|| {
                    ActLocalProvenanceError::Decomposition(format!(
                        "R2 generated action at clause {clause} has no counted parent face"
                    ))
                })?;
            let typed_clause = elaboration
                .clauses
                .get(usize::from(*clause))
                .ok_or_else(|| {
                    ActLocalProvenanceError::Family("R2 operation clause is absent".to_owned())
                })?;
            let path_clause = elaboration
                .clauses
                .iter()
                .find(|clause| matches!(clause.kernel_ty, KernelTy::PathDecl { .. }))
                .ok_or_else(|| {
                    ActLocalProvenanceError::Family(
                        "R2 generated action has no typed path parent".to_owned(),
                    )
                })?;
            let KernelTy::PathDecl { dimension } = path_clause.kernel_ty else {
                unreachable!("selected path clause is PathDecl")
            };
            if typed_clause.kernel_role != pen_core::clause::ClauseRole::Introduction {
                return Err(ActLocalProvenanceError::Family(format!(
                    "R2 canonical operation clause {clause} is not a typed introduction"
                )));
            }
            let mut removal = ActLocalGeneratedInstanceRemoval {
                theorem_id: R2_GENERATED_ACTION_THEOREM_ID.to_owned(),
                stage,
                candidate_hash: candidate_digest.clone(),
                generated_schema_role: formula_cell.role.clone(),
                generated_family_id: semantic_family_id,
                parent_family_id: parent.semantic_family_id.clone(),
                path_clause_index: path_clause.clause_index,
                parent_operation_clause_index: *clause,
                path_dimension: dimension,
                candidate_elaboration_hash: elaboration.derivation_hash.clone(),
                path_clause_is_typed_path: true,
                parent_operation_is_typed_introduction: true,
                adopted_rule_source_hash: r2_hash.clone(),
                count_bar_archive_or_future_used: false,
                derivation_hash: String::new(),
            };
            removal.derivation_hash = tagged_hash("r2-generated-instance-removal", &removal);
            removals.push(removal);
            continue;
        }

        let marginality = marginality_sweep(&family_shape_key, &generator, &all_prior_tokens)?;
        let uniform = formula_cell.role.is_uniform_instance();
        let can_use_local = !uniform
            && marginality.no_predecessor_or_earlier_act_preimage
            && local_slots.insert((owner_clause, formula_cell.local_role));
        let (credit_kind, specialization_instance_id, anchor) = if can_use_local {
            let injection_hash = tagged_hash(
                "local-role-injection",
                &(
                    stage,
                    &candidate_digest,
                    owner_clause,
                    formula_cell.local_role,
                    &semantic_family_id,
                ),
            );
            (
                ActLocalCreditKind::MarginalNaturalFamily,
                None,
                ActLocalProvenanceAnchor::ChargedLocalRole {
                    clause: owner_clause,
                    role: formula_cell.local_role,
                    injection_hash,
                },
            )
        } else {
            let [(orbit_id, scheme_id, orbit_derivation_hash)] = independent_orbits.as_slice()
            else {
                return Err(ActLocalProvenanceError::Injection(format!(
                    "Stage {stage} role {:?} is uniform, repeated, or has occupied local role, but no unique pre-existing independent structural demand orbit can fund it",
                    formula_cell.role
                )));
            };
            let instance_discriminant =
                tagged_hash("schema-role-instance-discriminant", &formula_cell.role);
            let output_type_hash = tagged_hash(
                "demand-output-normal-type",
                &(
                    &generator.kernel_type_json,
                    &generator.parameter_sorts,
                    &semantic_family_id,
                ),
            );
            let output_position_id = tagged_hash(
                "structural-demand-output-position",
                &(
                    orbit_id,
                    scheme_id,
                    &semantic_family_id,
                    &instance_discriminant,
                    &output_type_hash,
                ),
            );
            if !output_positions.insert(output_position_id.clone()) {
                return Err(ActLocalProvenanceError::Injection(format!(
                    "Stage {stage} reused demand output position for {:?}",
                    formula_cell.role
                )));
            }
            (
                if uniform {
                    ActLocalCreditKind::IndependentlyExportedUniformInstance
                } else {
                    ActLocalCreditKind::LiveDemandFamilyOutput
                },
                Some(instance_discriminant),
                ActLocalProvenanceAnchor::LiveDemandOutput {
                    orbit_id: (*orbit_id).to_owned(),
                    scheme_id: (*scheme_id).to_owned(),
                    orbit_derivation_hash: (*orbit_derivation_hash).to_owned(),
                    output_position_id,
                    output_type_hash,
                    demand_window_hash: window.window_derivation_hash.clone(),
                    demand_existed_before_candidate: true,
                    independently_exported_demand_orbit: true,
                    uniform_specializations_collapsed_at_orbit: true,
                },
            )
        };
        let mut token = ActLocalOrdinaryFamilyToken {
            token_version: ACT_LOCAL_ORDINARY_TOKEN_VERSION.to_owned(),
            theorem_id: T_BI_NU1_THEOREM_ID.to_owned(),
            stage,
            candidate_hash: candidate_digest.clone(),
            predecessor_signature_digest: prefix.digest().to_owned(),
            mechanism: formula_cell.mechanism,
            schema_role: formula_cell.role.clone(),
            family_shape_key,
            semantic_family_id,
            specialization_instance_id,
            generator,
            marginality,
            credit_kind,
            anchor,
            typed_normalized_natural: true,
            family_or_instance_decided: true,
            uniform_specialization_multiplied_without_exported_demand: false,
            candidate_and_prefix_are_only_semantic_inputs: true,
            enacted_future_used: false,
            archive_join_used: false,
            historical_total_used_as_input: false,
            bar_or_verdict_used_as_input: false,
            derivation_hash: String::new(),
        };
        token.derivation_hash = tagged_hash("ordinary-family-token", &token);
        all_prior_tokens.push(token.clone());
        tokens.push(token);
    }

    let generated_instance_adjustment = -(removals.len() as i32);
    let exact_certified_nu = tokens.len() as u32;
    let local_count = tokens
        .iter()
        .filter(|token| {
            matches!(
                token.anchor,
                ActLocalProvenanceAnchor::ChargedLocalRole { .. }
            )
        })
        .count();
    let demand_count = tokens.len() - local_count;
    let local_role_injection_holds = local_slots.len() == local_count;
    let demand_output_injection_holds = output_positions.len() == demand_count;
    let every_uniform_or_repeated_unit_has_independent_demand_output = tokens.iter().all(|token| {
        if token.schema_role.is_uniform_instance()
            || !token.marginality.no_predecessor_or_earlier_act_preimage
        {
            matches!(
                token.anchor,
                ActLocalProvenanceAnchor::LiveDemandOutput { .. }
            )
        } else {
            true
        }
    });
    let authoritative = cells.len() == tokens.len() + removals.len()
        && exact_certified_nu as i64
            == i64::from(formula.total) + i64::from(generated_instance_adjustment)
        && tokens.iter().all(|token| {
            token.typed_normalized_natural
                && token.family_or_instance_decided
                && !token.uniform_specialization_multiplied_without_exported_demand
                && token.candidate_and_prefix_are_only_semantic_inputs
                && !token.enacted_future_used
                && !token.archive_join_used
                && !token.historical_total_used_as_input
                && !token.bar_or_verdict_used_as_input
        })
        && local_role_injection_holds
        && demand_output_injection_holds
        && every_uniform_or_repeated_unit_has_independent_demand_output;
    let mut certificate = ActLocalNuProvenanceCertificate {
        schema: ACT_LOCAL_PROVENANCE_SCHEMA.to_owned(),
        date: ACT_LOCAL_PROVENANCE_DATE.to_owned(),
        theorem_id: T_BI_NU1_THEOREM_ID.to_owned(),
        stage,
        candidate_hash: candidate_digest,
        predecessor_signature_digest: prefix.digest().to_owned(),
        predecessor_act_local_digest,
        candidate_elaboration_hash: elaboration.derivation_hash,
        candidate_family_extraction_hash: extraction.derivation_hash,
        a3_demand_window_hash: window.window_derivation_hash.clone(),
        independent_structural_demand_orbit_ids: independent_orbits
            .iter()
            .map(|(orbit, _, _)| (*orbit).to_owned())
            .collect(),
        class: candidate.classify(library),
        kappa: candidate.kappa() as u32,
        structural_formula_total: formula.total,
        formula_cell_count_before_quotient: cells.len() as u32,
        generated_instance_adjustment,
        exact_certified_nu,
        ordinary_family_tokens: tokens,
        generated_instance_family_ids_removed_by_quotient: removals
            .iter()
            .map(|removal| removal.generated_family_id.clone())
            .collect(),
        generated_instance_removals: removals,
        every_formula_cell_has_explicit_semantic_coordinates: true,
        every_counted_unit_typed_normalized_natural: true,
        local_role_injection_holds,
        demand_output_injection_holds,
        every_uniform_or_repeated_unit_has_independent_demand_output,
        transportable_on_same_act_and_prefix: true,
        archive_future_total_bar_or_verdict_read_by_issuer: false,
        authoritative,
        derivation_hash: String::new(),
    };
    certificate.derivation_hash = certificate_digest(&certificate);
    if !certificate.authoritative {
        return Err(ActLocalProvenanceError::Injection(format!(
            "Stage {stage} act-local provenance did not close its authority conditions"
        )));
    }
    Ok(certificate)
}

fn validate_prefix(
    prefix: &SealedSignature,
    stage: u32,
) -> Result<Vec<(u32, Telescope)>, ActLocalProvenanceError> {
    if stage == 0 {
        return Err(ActLocalProvenanceError::Input(
            "stage must be positive".to_owned(),
        ));
    }
    let entries = prefix
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.telescope.clone()))
        .collect::<Vec<_>>();
    if entries.len() != stage.saturating_sub(1) as usize
        || !entries.iter().map(|(step, _)| *step).eq(1..stage)
    {
        return Err(ActLocalProvenanceError::Input(format!(
            "Stage {stage} prefix is not exactly contiguous Stages 1..{}",
            stage.saturating_sub(1)
        )));
    }
    Ok(entries)
}

/// Issue exact ordinary-charge provenance from the act and its sealed prefix.
/// The prefix packages are recomputed recursively from their own earlier
/// prefixes; no caller-supplied score history is accepted.
pub fn issue_act_local_provenance(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
) -> Result<ActLocalNuProvenanceCertificate, ActLocalProvenanceError> {
    let mut entries = validate_prefix(prefix, stage)?;
    entries.push((stage, candidate.clone()));
    let packages = issue_act_local_sequence(&entries)?;
    packages.last().cloned().ok_or_else(|| {
        ActLocalProvenanceError::Input("act-local sequence was unexpectedly empty".to_owned())
    })
}

/// Issue a contiguous sequence.  This is useful for a branch runner: each
/// result depends only on the prefix of `entries` ending at that act.
pub fn issue_act_local_sequence(
    entries: &[(u32, Telescope)],
) -> Result<Vec<ActLocalNuProvenanceCertificate>, ActLocalProvenanceError> {
    if entries
        .iter()
        .map(|(stage, _)| *stage)
        .ne(1..=entries.len() as u32)
    {
        return Err(ActLocalProvenanceError::Input(
            "act-local sequence must be contiguous from Stage 1".to_owned(),
        ));
    }
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut library: Library = Vec::new();
    let mut history = Vec::<(u32, u32)>::new();
    let mut packages = Vec::new();
    for (stage, candidate) in entries {
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let package = issue_one(&prefix, *stage, candidate, &library, &history, &packages)?;
        history.push((*stage, package.exact_certified_nu));
        library.push(LibraryEntry::from_telescope(candidate, &library));
        accepted.push((*stage, candidate.clone()));
        packages.push(package);
    }
    Ok(packages)
}

/// Replay by independent reissuance from exactly the same act and prefix.
pub fn replay_act_local_provenance(
    prefix: &SealedSignature,
    candidate: &Telescope,
    claimed: &ActLocalNuProvenanceCertificate,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != certificate_digest(claimed) {
        errors.push("act-local certificate digest mismatch".to_owned());
    }
    if claimed.schema != ACT_LOCAL_PROVENANCE_SCHEMA
        || claimed.theorem_id != T_BI_NU1_THEOREM_ID
        || claimed.stage != prefix.entries().len() as u32 + 1
        || claimed.candidate_hash != candidate_hash(candidate)
        || claimed.predecessor_signature_digest != prefix.digest()
        || claimed.kappa != candidate.kappa() as u32
    {
        errors.push("act-local certificate subject/prefix/stage binding mismatch".to_owned());
    }
    let token_hashes = claimed
        .ordinary_family_tokens
        .iter()
        .map(|token| token.derivation_hash.as_str())
        .collect::<BTreeSet<_>>();
    let token_coordinates = claimed
        .ordinary_family_tokens
        .iter()
        .map(|token| {
            (
                token.semantic_family_id.as_str(),
                token.specialization_instance_id.as_deref(),
                tagged_hash("replay-schema-role-coordinate", &token.schema_role),
            )
        })
        .collect::<BTreeSet<_>>();
    let local_anchors = claimed
        .ordinary_family_tokens
        .iter()
        .filter_map(|token| match token.anchor {
            ActLocalProvenanceAnchor::ChargedLocalRole { clause, role, .. } => Some((clause, role)),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let output_positions = claimed
        .ordinary_family_tokens
        .iter()
        .filter_map(|token| match &token.anchor {
            ActLocalProvenanceAnchor::LiveDemandOutput {
                output_position_id, ..
            } => Some(output_position_id.as_str()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let removed_family_ids = claimed
        .generated_instance_removals
        .iter()
        .map(|removal| removal.generated_family_id.as_str())
        .collect::<BTreeSet<_>>();
    let projected_removed_family_ids = claimed
        .generated_instance_removals
        .iter()
        .map(|removal| removal.generated_family_id.clone())
        .collect::<Vec<_>>();
    let local_count = claimed
        .ordinary_family_tokens
        .iter()
        .filter(|token| {
            matches!(
                token.anchor,
                ActLocalProvenanceAnchor::ChargedLocalRole { .. }
            )
        })
        .count();
    let output_count = claimed.ordinary_family_tokens.len() - local_count;
    if claimed.exact_certified_nu as usize != claimed.ordinary_family_tokens.len()
        || claimed.formula_cell_count_before_quotient as usize
            != claimed.ordinary_family_tokens.len() + claimed.generated_instance_removals.len()
        || i64::from(claimed.exact_certified_nu)
            != i64::from(claimed.structural_formula_total)
                + i64::from(claimed.generated_instance_adjustment)
        || claimed.generated_instance_adjustment
            != -(claimed.generated_instance_removals.len() as i32)
        || token_hashes.len() != claimed.ordinary_family_tokens.len()
        || token_coordinates.len() != claimed.ordinary_family_tokens.len()
        || local_anchors.len() != local_count
        || output_positions.len() != output_count
        || removed_family_ids.len() != claimed.generated_instance_removals.len()
        || claimed.generated_instance_family_ids_removed_by_quotient != projected_removed_family_ids
    {
        errors.push("act-local count, coordinate, or anchor injection mismatch".to_owned());
    }
    if !claimed.authoritative
        || claimed.archive_future_total_bar_or_verdict_read_by_issuer
        || claimed.ordinary_family_tokens.iter().any(|token| {
            token.stage != claimed.stage
                || token.candidate_hash != claimed.candidate_hash
                || token.predecessor_signature_digest != claimed.predecessor_signature_digest
                || !token.typed_normalized_natural
                || !token.family_or_instance_decided
                || !token.candidate_and_prefix_are_only_semantic_inputs
                || token.enacted_future_used
                || token.archive_join_used
                || token.historical_total_used_as_input
                || token.bar_or_verdict_used_as_input
                || token.uniform_specialization_multiplied_without_exported_demand
        })
        || claimed.generated_instance_removals.iter().any(|removal| {
            removal.stage != claimed.stage
                || removal.candidate_hash != claimed.candidate_hash
                || removal.count_bar_archive_or_future_used
        })
    {
        errors.push("act-local authority or forbidden-input flag mismatch".to_owned());
    }
    match issue_act_local_provenance(prefix, claimed.stage, candidate) {
        Ok(expected) if &expected == claimed => {}
        Ok(_) => {
            errors.push("act-local certificate differs from independent reissuance".to_owned())
        }
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enacted_entries(last: u32) -> Vec<(u32, Telescope)> {
        (1..=last)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect()
    }

    #[test]
    fn enacted_sequence_issues_without_archive_inputs() {
        let packages = issue_act_local_sequence(&enacted_entries(15)).expect("T-BI-NU1");
        assert_eq!(packages.len(), 15);
        assert!(packages.iter().all(|package| package.authoritative));
        assert!(
            packages
                .iter()
                .all(|package| !package.archive_future_total_bar_or_verdict_read_by_issuer)
        );
        assert_eq!(
            packages
                .iter()
                .map(|package| package.exact_certified_nu)
                .collect::<Vec<_>>(),
            vec![1, 1, 2, 5, 7, 8, 10, 17, 17, 19, 26, 34, 46, 62, 103]
        );
    }

    #[test]
    fn stage_eight_generated_action_is_removed_with_a_parent() {
        let packages = issue_act_local_sequence(&enacted_entries(8)).expect("T-BI-NU1");
        let step8 = &packages[7];
        assert_eq!(step8.structural_formula_total, 18);
        assert_eq!(step8.generated_instance_adjustment, -1);
        assert_eq!(step8.exact_certified_nu, 17);
        assert_eq!(step8.generated_instance_removals.len(), 1);
        assert!(
            !step8.generated_instance_removals[0]
                .parent_family_id
                .is_empty()
        );
    }

    #[test]
    fn mutation_and_prefix_swap_are_rejected() {
        let entries = enacted_entries(10);
        let prefix = SealedSignature::from_telescopes(entries[..9].to_vec());
        let claimed = issue_act_local_provenance(&prefix, 10, &entries[9].1).expect("issue");
        assert!(replay_act_local_provenance(&prefix, &entries[9].1, &claimed).is_empty());

        let mut mutated = claimed.clone();
        mutated.ordinary_family_tokens[0].archive_join_used = true;
        assert!(!replay_act_local_provenance(&prefix, &entries[9].1, &mutated).is_empty());

        let wrong_prefix = SealedSignature::from_telescopes(entries[..8].to_vec());
        assert!(!replay_act_local_provenance(&wrong_prefix, &entries[9].1, &claimed).is_empty());
    }
}
