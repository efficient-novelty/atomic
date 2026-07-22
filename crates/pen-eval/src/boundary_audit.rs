//! Historical boundary-variant audit.  The implementation is feature-gated
//! so the incumbent cubical V1 path remains the default build.
//!
//! This module produces decision evidence only.  It distinguishes an
//! incumbent typed-term replay from structural checking of a supplied
//! boundary diagram and from recovery of that diagram from the sealed trace.
//! It never promotes a raw finite key presentation into a typed semantic
//! classifier or novelty certificate.

use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::cubical::boundary_variants::{
    BOUNDARY_VARIANTS_FRAGMENT_VERSION, BoundaryBasisFormula, BoundaryBasisKey, BoundaryTheory,
    BoundaryVariantError, DECLARED_BOUNDARY_RULE_VERSION,
    TRACE_DERIVED_BOUNDARY_CONSTRAINT_VERSION, check_declared_boundary_diagram, constant_boundary,
    generic_declared_boundary, interval_endpoint_boundary, issue_boundary_attachment,
    present_boundary_basis, replay_boundary_attachment, replay_declared_boundary_diagram,
    witness_trace_boundary_noninjectivity,
};
use pen_type::cubical::{
    CUBICAL_FRAGMENT_VERSION, PATHCON_ATTACHMENT_AXIOM_VERSION, realize_path_basis,
    replay_path_realization,
};
use pen_type::elaborate::SealedSignature;
use pen_type::tdc1::{PathSchemaKey, elaborate_formed_path};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const BOUNDARY_AUDIT_SCHEMA: &str = "boundary-audit-v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VariantId {
    V1ImplicitConstant,
    V2DeclaredBoundary,
    V3TraceDerived,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntendedBoundaryShape {
    ConstantAllFaces,
    EndpointDependentInterval,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalFitVerdict {
    IncumbentTypedPresentationShapeFit,
    FailsHistoricalBoundary,
    StructuralBoundaryAcceptedTypingOpen,
    TraceUnderdetermined,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BasisAlignmentVerdict {
    TypedPresentationMatchesRecorded,
    SurrogateCountMatchesButHistoricalBoundaryFails,
    RawKeyCountMatchesTypingOpen,
    TraceUnderdetermined,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FormulaEvidenceStatus {
    IncumbentTypedPresentation,
    RawCombinatorialProposal,
    TraceUnderdetermined,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FormulaExpression {
    OnePlusDimensionSquared,
    /// Mutation-only alternative.  The canonical builder never emits it.
    OnePlusTwiceDimension,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivabilityStatus {
    NotForcedOrForbiddenByFrozenKernel,
    NotDerivableFromDimensionOnlyTrace,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SuppliedBoundaryDiagramAudit {
    pub step: u32,
    pub label: String,
    pub dimension: u32,
    pub owner_normal_form: Expr,
    pub subject_hash: String,
    pub predecessor_signature_digest: String,
    pub intended_shape: IntendedBoundaryShape,
    pub intended_boundary_description: String,
    pub semantic_context: String,
    pub semantic_source: String,
    pub boundary_diagram_digest: String,
    pub structural_coherence_derivation_hash: String,
    pub face_count: u32,
    pub overlap_count: u32,
    pub constant_boundary: bool,
    pub boundary_is_declared_input_not_credit: bool,
    pub diagram_recoverable_from_sealed_trace: bool,
    pub noninjectivity_trace_projection_digest: String,
    pub alternate_boundary_diagram_digest: String,
    pub trace_projection_noninjectivity_derivation_hash: String,
    pub trace_projection_noninjectivity_checked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoricalVariantAudit {
    pub step: u32,
    pub label: String,
    pub variant: VariantId,
    pub dimension: u32,
    pub fit: HistoricalFitVerdict,
    pub fit_reason: String,
    pub supplied_boundary_diagram_structurally_accepted: bool,
    pub historical_constructor_typed_under_variant: bool,
    pub supplied_diagram_trace_recoverable: bool,
    pub intended_boundary_attempt_error: Option<String>,
    pub attachment_rule_version: String,
    pub attachment_token_derivation_hash: Option<String>,
    pub boundary_diagram_digest_used: Option<String>,
    pub presented_basis_count: Option<u64>,
    pub recorded_basis_count: u64,
    pub basis_alignment: BasisAlignmentVerdict,
    pub basis_formula: Option<FormulaExpression>,
    pub beta_count: Option<u32>,
    pub principal_transport_count: Option<u32>,
    pub ordered_naturality_count: Option<u64>,
    pub basis_presentation_derivation_hash: Option<String>,
    pub variant_specific_term_tokens_replay: bool,
    pub term_realization_scope: String,
    pub semantic_exhaustiveness_proved: bool,
    pub result_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FormulaAudit {
    pub variant: VariantId,
    pub formula: Option<FormulaExpression>,
    pub evidence_status: FormulaEvidenceStatus,
    pub derivation: String,
    pub boundary_premise: String,
    pub boundary_components_separately_charged: bool,
    pub formula_if_boundary_components_are_charged: String,
    pub finite_key_partition_machine_checked: bool,
    pub semantic_classifier_or_exhaustiveness_proved: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DerivabilityAudit {
    pub variant: VariantId,
    pub status: DerivabilityStatus,
    pub kernel_evidence: String,
    pub sidecar_evidence: String,
    pub forced_by_constitutive_law: bool,
    pub forced_by_selective_law: bool,
    pub forbidden_by_frozen_kernel: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VariantSummary {
    pub variant: VariantId,
    pub historical_fit: String,
    pub basis_formula: Option<FormulaExpression>,
    pub derivability: DerivabilityStatus,
    pub uniform_rule_structurally_accepts_all_supplied_diagrams: Option<bool>,
    pub uniform_rule_fully_types_all_historical_constructors: Option<bool>,
    pub consequences: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MaterialFindings {
    pub v1_fails_historical_fit_at_trunc: bool,
    pub v1_constant_shape_rows_have_historical_point_binding: bool,
    pub historical_constraint_differs_from_v1_at_trunc: bool,
    pub tdc_v3_conditionality_is_material: bool,
    pub hist_cert_per_step_conditionality_flags_are_load_bearing: bool,
    pub v2_structurally_accepts_all_supplied_boundary_diagrams: bool,
    pub v2_fully_types_all_historical_constructors: bool,
    pub v2_diagrams_recoverable_from_trace: bool,
    pub v3_rule_recoverable_from_trace: bool,
    pub finite_cardinality_alignment_discriminates_v1_from_v2: bool,
    pub some_single_variant_reproduces_all_recorded_raw_presentations: bool,
    pub some_single_variant_fully_types_all_historical_constructors: bool,
    pub no_boundary_variant_adjudicated: bool,
    pub no_semantic_novelty_or_later_candidate_score_computed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdjudicationOption {
    pub option: String,
    pub recorded_consequences: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundaryAuditPayload {
    pub feature_gate: String,
    pub boundary_fragment_version: String,
    pub incumbent_cubical_fragment_version: String,
    pub incumbent_v1_rule_version: String,
    pub declared_v2_rule_version: String,
    pub trace_v3_constraint_version: String,
    pub incumbent_v1_preservation_scope: String,
    pub boundary_diagrams: Vec<SuppliedBoundaryDiagramAudit>,
    pub historical_rows: Vec<HistoricalVariantAudit>,
    pub formulas: Vec<FormulaAudit>,
    pub derivability: Vec<DerivabilityAudit>,
    pub comparison: Vec<VariantSummary>,
    pub findings: MaterialFindings,
    pub adjudication_required: bool,
    pub adjudication_options: Vec<AdjudicationOption>,
    pub remaining_obligations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MutationFalsifiers {
    pub flipped_fit_verdict_rejected: bool,
    pub mutated_formula_rejected: bool,
    pub mutated_boundary_diagram_rejected: bool,
    pub false_v3_trace_recovery_rejected: bool,
    pub mutated_token_hash_rejected: bool,
    pub deleted_historical_row_rejected: bool,
    pub rebased_outer_digest_rejected: bool,
    pub all_passed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundaryAuditResult {
    pub schema: String,
    pub payload: BoundaryAuditPayload,
    pub mutation_falsifiers: MutationFalsifiers,
    pub result_digest: String,
}

#[derive(Debug, Error)]
pub enum BoundaryAuditError {
    #[error("unsupported historical step {0}")]
    UnsupportedStep(u32),
    #[error("historical formed-path elaboration failed at step {step}: {reason}")]
    Elaboration { step: u32, reason: String },
    #[error("boundary variant failed at step {step}: {reason}")]
    Boundary { step: u32, reason: String },
    #[error("incumbent V1 basis replay failed at step {step}: {reason}")]
    IncumbentBasis { step: u32, reason: String },
    #[error("boundary audit invariant failed: {0}")]
    Invariant(String),
    #[error("boundary audit replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, payload: &T) -> String {
    let bytes = serde_json::to_vec(&(BOUNDARY_AUDIT_SCHEMA, domain, payload))
        .expect("boundary-audit data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn historical_metadata(step: u32) -> Result<(&'static str, u64), BoundaryAuditError> {
    match step {
        5 => Ok(("S1", 2)),
        6 => Ok(("Trunc", 2)),
        7 => Ok(("S2", 5)),
        8 => Ok(("S3", 10)),
        _ => Err(BoundaryAuditError::UnsupportedStep(step)),
    }
}

fn predecessor_signature(step: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        (1..step)
            .map(|index| (index, Telescope::reference(index)))
            .collect(),
    )
}

fn as_boundary_error(step: u32, error: BoundaryVariantError) -> BoundaryAuditError {
    BoundaryAuditError::Boundary {
        step,
        reason: error.to_string(),
    }
}

struct HistoricalCore {
    boundary_diagram: SuppliedBoundaryDiagramAudit,
    rows: Vec<HistoricalVariantAudit>,
}

fn build_historical_core(step: u32) -> Result<HistoricalCore, BoundaryAuditError> {
    let (label, recorded_basis_count) = historical_metadata(step)?;
    let signature = predecessor_signature(step);
    let telescope = Telescope::reference(step);
    let (typing, _) = elaborate_formed_path(&signature, &telescope, step - 1).map_err(|error| {
        BoundaryAuditError::Elaboration {
            step,
            reason: error.to_string(),
        }
    })?;
    if !(1..=3).contains(&typing.dimension) {
        return Err(BoundaryAuditError::Invariant(format!(
            "historical boundary audit is restricted to dimensions one through three; step {step} has {}",
            typing.dimension
        )));
    }

    let (intended_shape, intended_description, semantic_context, semantic_source, intended_map) =
        if step == 6 {
            (
                IntendedBoundaryShape::EndpointDependentInterval,
                "squash(x,y)(0)=x and squash(x,y)(1)=y".to_owned(),
                "A:Type; x,y:Trunc(A)".to_owned(),
                "HIST-CERT v1 concrete Trunc mismatch plus the historical squash interpretation; the parameters are a supplied semantic annotation absent from PathCon bytes"
                    .to_owned(),
                interval_endpoint_boundary(
                    typing.formation_normal_form.clone(),
                    "x",
                    "y",
                    "A:Type; x,y:Trunc(A)",
                ),
            )
        } else {
            let description = match step {
                5 => "loop(0)=base and loop(1)=base",
                7 => "all four codimension-one surface faces are base",
                8 => "all six codimension-one cell faces are base",
                _ => return Err(BoundaryAuditError::UnsupportedStep(step)),
            };
            (
                IntendedBoundaryShape::ConstantAllFaces,
                description.to_owned(),
                format!("base:{label}"),
                "historical semantic label/conventional sphere-HIT reading; the boundary equations are not serialized in PathCon bytes"
                    .to_owned(),
                constant_boundary(
                    typing.formation_normal_form.clone(),
                    typing.dimension,
                    "base",
                    1,
                )
                .map_err(|error| as_boundary_error(step, error))?,
            )
        };
    let intended = check_declared_boundary_diagram(intended_map)
        .map_err(|error| as_boundary_error(step, error))?;
    replay_declared_boundary_diagram(&intended).map_err(|error| as_boundary_error(step, error))?;

    let alternate = if intended.is_constant() {
        check_declared_boundary_diagram(
            generic_declared_boundary(
                typing.formation_normal_form.clone(),
                typing.dimension,
                format!("alternate:{label}"),
            )
            .map_err(|error| as_boundary_error(step, error))?,
        )
        .map_err(|error| as_boundary_error(step, error))?
    } else {
        check_declared_boundary_diagram(
            constant_boundary(
                typing.formation_normal_form.clone(),
                typing.dimension,
                "base",
                1,
            )
            .map_err(|error| as_boundary_error(step, error))?,
        )
        .map_err(|error| as_boundary_error(step, error))?
    };
    let noninjectivity = witness_trace_boundary_noninjectivity(&typing, &intended, &alternate)
        .map_err(|error| as_boundary_error(step, error))?;
    if !noninjectivity.same_trace_projection()
        || !noninjectivity.distinct_declared_boundary_diagrams()
    {
        return Err(BoundaryAuditError::Invariant(
            "trace-boundary noninjectivity witness is incomplete".to_owned(),
        ));
    }

    let boundary_diagram = SuppliedBoundaryDiagramAudit {
        step,
        label: label.to_owned(),
        dimension: typing.dimension,
        owner_normal_form: typing.formation_normal_form.clone(),
        subject_hash: typing.subject_hash.clone(),
        predecessor_signature_digest: signature.digest().to_owned(),
        intended_shape,
        intended_boundary_description: intended_description,
        semantic_context,
        semantic_source,
        boundary_diagram_digest: intended.map_digest().to_owned(),
        structural_coherence_derivation_hash: intended.derivation_hash().to_owned(),
        face_count: intended.face_count(),
        overlap_count: intended.overlap_count(),
        constant_boundary: intended.is_constant(),
        boundary_is_declared_input_not_credit: true,
        diagram_recoverable_from_sealed_trace: false,
        noninjectivity_trace_projection_digest: noninjectivity.trace_projection_digest().to_owned(),
        alternate_boundary_diagram_digest: noninjectivity.right_boundary_digest().to_owned(),
        trace_projection_noninjectivity_derivation_hash: noninjectivity
            .derivation_hash()
            .to_owned(),
        trace_projection_noninjectivity_checked: true,
    };

    // V1: first attempt the intended historical diagram itself.  The Trunc
    // row must be rejected with the specific constant-boundary error before
    // a separately labelled constant surrogate is used to compare raw counts.
    let intended_v1_attempt = issue_boundary_attachment(
        &signature,
        &telescope,
        step - 1,
        &typing,
        BoundaryTheory::V1ImplicitConstant,
        &intended,
    );
    let (v1_boundary, v1_attachment, intended_boundary_attempt_error) =
        match (intended.is_constant(), intended_v1_attempt) {
            (true, Ok(token)) => (intended.clone(), token, None),
            (false, Err(BoundaryVariantError::V1RequiresConstantBoundary)) => {
                let token = issue_boundary_attachment(
                    &signature,
                    &telescope,
                    step - 1,
                    &typing,
                    BoundaryTheory::V1ImplicitConstant,
                    &alternate,
                )
                .map_err(|error| as_boundary_error(step, error))?;
                (
                    alternate.clone(),
                    token,
                    Some(BoundaryVariantError::V1RequiresConstantBoundary.to_string()),
                )
            }
            (true, Err(error)) => return Err(as_boundary_error(step, error)),
            (false, Ok(_)) => {
                return Err(BoundaryAuditError::Invariant(
                    "V1 accepted the intended nonconstant Trunc boundary diagram".to_owned(),
                ));
            }
            (false, Err(error)) => {
                return Err(BoundaryAuditError::Invariant(format!(
                    "V1 rejected the intended Trunc boundary for the wrong reason: {error}"
                )));
            }
        };
    replay_boundary_attachment(
        &signature,
        &telescope,
        step - 1,
        &typing,
        &v1_boundary,
        &v1_attachment,
    )
    .map_err(|error| as_boundary_error(step, error))?;
    let v1_presentation =
        present_boundary_basis(&v1_attachment).map_err(|error| as_boundary_error(step, error))?;
    if v1_presentation.formula() != BoundaryBasisFormula::OnePlusDimensionSquared {
        return Err(BoundaryAuditError::Invariant(
            "V1 basis formula projection drifted".to_owned(),
        ));
    }
    let incumbent =
        realize_path_basis(&signature, &telescope, step - 1, &typing).map_err(|error| {
            BoundaryAuditError::IncumbentBasis {
                step,
                reason: error.to_string(),
            }
        })?;
    for token in incumbent.tokens() {
        replay_path_realization(&signature, &telescope, step - 1, &typing, token).map_err(
            |error| BoundaryAuditError::IncumbentBasis {
                step,
                reason: error.to_string(),
            },
        )?;
    }
    let incumbent_count = u64::try_from(incumbent.tokens().len()).map_err(|_| {
        BoundaryAuditError::Invariant("incumbent basis length does not fit u64".to_owned())
    })?;
    let incumbent_keys = incumbent
        .tokens()
        .iter()
        .map(|token| match token.key() {
            PathSchemaKey::Beta => BoundaryBasisKey::Beta,
            PathSchemaKey::Kan { principal, probe } if principal == probe => {
                BoundaryBasisKey::PrincipalTransport {
                    principal: *principal,
                }
            }
            PathSchemaKey::Kan { principal, probe } => BoundaryBasisKey::TransportNaturality {
                principal: *principal,
                probe: *probe,
            },
        })
        .collect::<BTreeSet<_>>();
    let presented_keys = v1_presentation
        .keys()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if incumbent_count != v1_presentation.total_count() || incumbent_keys != presented_keys {
        return Err(BoundaryAuditError::Invariant(
            "incumbent V1 tokens and boundary-neutral V1 presentation are not in exact key bijection"
                .to_owned(),
        ));
    }
    let v1_fit = if intended.is_constant() {
        HistoricalFitVerdict::IncumbentTypedPresentationShapeFit
    } else {
        HistoricalFitVerdict::FailsHistoricalBoundary
    };
    let v1_alignment = if intended.is_constant() {
        BasisAlignmentVerdict::TypedPresentationMatchesRecorded
    } else {
        BasisAlignmentVerdict::SurrogateCountMatchesButHistoricalBoundaryFails
    };
    let v1_fit_reason = if intended.is_constant() {
        "the supplied diagram has constant shape and the incumbent source-bound term tokens replay, but the implicit cubical base is not derivationally identified with the historical point clause"
    } else {
        "the incumbent constant-boundary tokens replay only a generic loop surrogate; they do not type the supplied endpoint-dependent squash constructor"
    };
    let v1_row = HistoricalVariantAudit {
        step,
        label: label.to_owned(),
        variant: VariantId::V1ImplicitConstant,
        dimension: typing.dimension,
        fit: v1_fit,
        fit_reason: v1_fit_reason.to_owned(),
        supplied_boundary_diagram_structurally_accepted: intended.is_constant(),
        historical_constructor_typed_under_variant: false,
        supplied_diagram_trace_recoverable: false,
        intended_boundary_attempt_error,
        attachment_rule_version: PATHCON_ATTACHMENT_AXIOM_VERSION.to_owned(),
        attachment_token_derivation_hash: Some(v1_attachment.derivation_hash().to_owned()),
        boundary_diagram_digest_used: Some(v1_boundary.map_digest().to_owned()),
        presented_basis_count: Some(incumbent_count),
        recorded_basis_count,
        basis_alignment: v1_alignment,
        basis_formula: Some(FormulaExpression::OnePlusDimensionSquared),
        beta_count: Some(v1_presentation.beta_count()),
        principal_transport_count: Some(v1_presentation.principal_transport_count()),
        ordered_naturality_count: Some(v1_presentation.ordered_naturality_count()),
        basis_presentation_derivation_hash: Some(tagged_hash(
            "incumbent-v1-basis",
            &(
                incumbent.derivation_hash(),
                v1_presentation.derivation_hash(),
            ),
        )),
        variant_specific_term_tokens_replay: true,
        term_realization_scope: "incumbent constant-boundary beta/Kan terms; constant-shape rows lack a base-to-historical-point-clause witness, and the Trunc row is a surrogate"
            .to_owned(),
        semantic_exhaustiveness_proved: false,
        result_derivation_hash: String::new(),
    };

    // V2: the intended boundary diagram is an explicit declared premise.  The
    // checker validates its owner labels, face inventory, restriction labels,
    // and pairwise overlap names.  It does not elaborate the annotation
    // strings as terms, so historical term typing remains an open obligation.
    let v2_attachment = issue_boundary_attachment(
        &signature,
        &telescope,
        step - 1,
        &typing,
        BoundaryTheory::V2DeclaredBoundary,
        &intended,
    )
    .map_err(|error| as_boundary_error(step, error))?;
    replay_boundary_attachment(
        &signature,
        &telescope,
        step - 1,
        &typing,
        &intended,
        &v2_attachment,
    )
    .map_err(|error| as_boundary_error(step, error))?;
    let v2_presentation =
        present_boundary_basis(&v2_attachment).map_err(|error| as_boundary_error(step, error))?;
    let v2_row = HistoricalVariantAudit {
        step,
        label: label.to_owned(),
        variant: VariantId::V2DeclaredBoundary,
        dimension: typing.dimension,
        fit: HistoricalFitVerdict::StructuralBoundaryAcceptedTypingOpen,
        fit_reason: "V2 structurally accepts the supplied declared boundary diagram, but the current fragment does not elaborate its face annotations or construct boundary-aware typed transports"
            .to_owned(),
        supplied_boundary_diagram_structurally_accepted: true,
        historical_constructor_typed_under_variant: false,
        supplied_diagram_trace_recoverable: false,
        intended_boundary_attempt_error: None,
        attachment_rule_version: DECLARED_BOUNDARY_RULE_VERSION.to_owned(),
        attachment_token_derivation_hash: Some(v2_attachment.derivation_hash().to_owned()),
        boundary_diagram_digest_used: Some(intended.map_digest().to_owned()),
        presented_basis_count: Some(v2_presentation.total_count()),
        recorded_basis_count,
        basis_alignment: BasisAlignmentVerdict::RawKeyCountMatchesTypingOpen,
        basis_formula: Some(FormulaExpression::OnePlusDimensionSquared),
        beta_count: Some(v2_presentation.beta_count()),
        principal_transport_count: Some(v2_presentation.principal_transport_count()),
        ordered_naturality_count: Some(v2_presentation.ordered_naturality_count()),
        basis_presentation_derivation_hash: Some(v2_presentation.derivation_hash().to_owned()),
        variant_specific_term_tokens_replay: false,
        term_realization_scope: "structural boundary-diagram checking and a boundary-bound raw key presentation only; face-term elaboration, motive methods, eliminator computation, substitution naturality, independence, and exhaustiveness remain unimplemented"
            .to_owned(),
        semantic_exhaustiveness_proved: false,
        result_derivation_hash: String::new(),
    };

    // V3: exercise the fail-closed API.  The checked noninjectivity witness is
    // why no rule, attachment token, or general formula is emitted.
    let v3_rejection = issue_boundary_attachment(
        &signature,
        &telescope,
        step - 1,
        &typing,
        BoundaryTheory::V3TraceDerived,
        &intended,
    );
    if v3_rejection != Err(BoundaryVariantError::TraceBoundaryUnderdetermined) {
        return Err(BoundaryAuditError::Invariant(
            "V3 did not fail closed on a dimension-only trace".to_owned(),
        ));
    }
    let v3_row = HistoricalVariantAudit {
        step,
        label: label.to_owned(),
        variant: VariantId::V3TraceDerived,
        dimension: typing.dimension,
        fit: HistoricalFitVerdict::TraceUnderdetermined,
        fit_reason: "two distinct declared boundary diagrams have the same sealed trace projection; the named semantic interpretation selects one, but the trace cannot"
            .to_owned(),
        supplied_boundary_diagram_structurally_accepted: false,
        historical_constructor_typed_under_variant: false,
        supplied_diagram_trace_recoverable: false,
        intended_boundary_attempt_error: Some(
            BoundaryVariantError::TraceBoundaryUnderdetermined.to_string(),
        ),
        attachment_rule_version: TRACE_DERIVED_BOUNDARY_CONSTRAINT_VERSION.to_owned(),
        attachment_token_derivation_hash: None,
        boundary_diagram_digest_used: None,
        presented_basis_count: None,
        recorded_basis_count,
        basis_alignment: BasisAlignmentVerdict::TraceUnderdetermined,
        basis_formula: None,
        beta_count: None,
        principal_transport_count: None,
        ordered_naturality_count: None,
        basis_presentation_derivation_hash: None,
        variant_specific_term_tokens_replay: false,
        term_realization_scope:
            "no trace-derived rule exists; importing V2's finite presentation would be V2 evidence"
                .to_owned(),
        semantic_exhaustiveness_proved: false,
        result_derivation_hash: String::new(),
    };

    let mut rows = vec![v1_row, v2_row, v3_row];
    for row in &mut rows {
        if row.presented_basis_count.is_some()
            && row.presented_basis_count != Some(row.recorded_basis_count)
        {
            return Err(BoundaryAuditError::Invariant(format!(
                "{} {:?} failed historical basis alignment",
                row.label, row.variant
            )));
        }
        row.result_derivation_hash.clear();
        row.result_derivation_hash = tagged_hash("historical-variant-row", row);
    }
    Ok(HistoricalCore {
        boundary_diagram,
        rows,
    })
}

fn formula_audits() -> Vec<FormulaAudit> {
    vec![
        FormulaAudit {
            variant: VariantId::V1ImplicitConstant,
            formula: Some(FormulaExpression::OnePlusDimensionSquared),
            evidence_status: FormulaEvidenceStatus::IncumbentTypedPresentation,
            derivation: "one beta + d principal automorphisms + d(d-1) ordered automorphism variations = 1+d+d(d-1) = 1+d^2"
                .to_owned(),
            boundary_premise: "implicit base and definitionally constant boundary"
                .to_owned(),
            boundary_components_separately_charged: false,
            formula_if_boundary_components_are_charged:
                "not applicable to V1's implicit constant boundary premise".to_owned(),
            finite_key_partition_machine_checked: true,
            semantic_classifier_or_exhaustiveness_proved: false,
        },
        FormulaAudit {
            variant: VariantId::V2DeclaredBoundary,
            formula: Some(FormulaExpression::OnePlusDimensionSquared),
            evidence_status: FormulaEvidenceStatus::RawCombinatorialProposal,
            derivation: "conditional raw-key proposal: one beta key + d principal-transport keys + d(d-1) ordered naturality keys = 1+d+d(d-1) = 1+d^2; no V2 term realizer establishes that these keys denote well-typed transports"
                .to_owned(),
            boundary_premise: "a structurally coherent declared boundary diagram and the still-unimplemented motive-boundary data would be parameters of the rule rather than independent computation exports"
                .to_owned(),
            boundary_components_separately_charged: false,
            formula_if_boundary_components_are_charged:
                "1+d^2+c(b), where c(b) depends on the boundary presentation and is not determined by dimension"
                    .to_owned(),
            finite_key_partition_machine_checked: true,
            semantic_classifier_or_exhaustiveness_proved: false,
        },
        FormulaAudit {
            variant: VariantId::V3TraceDerived,
            formula: None,
            evidence_status: FormulaEvidenceStatus::TraceUnderdetermined,
            derivation: "the finite annotated rows do not determine a general-dimensional law; adopting 1+d^2 would explicitly inherit V2 rather than derive V3 from the trace"
                .to_owned(),
            boundary_premise: "absent: PathCon stores dimension only, and checked noninjectivity exhibits distinct declared boundary diagrams over the same trace projection"
                .to_owned(),
            boundary_components_separately_charged: false,
            formula_if_boundary_components_are_charged:
                "undefined because neither a uniform rule nor a boundary charging convention is trace-derived"
                    .to_owned(),
            finite_key_partition_machine_checked: false,
            semantic_classifier_or_exhaustiveness_proved: false,
        },
    ]
}

fn derivability_audits() -> Vec<DerivabilityAudit> {
    vec![
        DerivabilityAudit {
            variant: VariantId::V1ImplicitConstant,
            status: DerivabilityStatus::NotForcedOrForbiddenByFrozenKernel,
            kernel_evidence: "the frozen checker/elaborator records only KernelTy::PathDecl{dimension}; it has no boundary judgement"
                .to_owned(),
            sidecar_evidence: "the incumbent constant-boundary rule is explicitly versioned as a conditional theory axiom"
                .to_owned(),
            forced_by_constitutive_law: false,
            forced_by_selective_law: false,
            forbidden_by_frozen_kernel: false,
        },
        DerivabilityAudit {
            variant: VariantId::V2DeclaredBoundary,
            status: DerivabilityStatus::NotForcedOrForbiddenByFrozenKernel,
            kernel_evidence: "explicit boundary maps, parameter contexts, and overlap equations are outside the frozen Expr grammar"
                .to_owned(),
            sidecar_evidence: "the feature-gated rival proposal structurally checks declared contexts, face inventories, and overlap labels; it does not elaborate boundary terms, while its constant diagrams reproduce V1's structural premise"
                .to_owned(),
            forced_by_constitutive_law: false,
            forced_by_selective_law: false,
            forbidden_by_frozen_kernel: false,
        },
        DerivabilityAudit {
            variant: VariantId::V3TraceDerived,
            status: DerivabilityStatus::NotDerivableFromDimensionOnlyTrace,
            kernel_evidence: "the sealed PathCon projection omits boundary data; a checked pair of distinct declared diagrams shares each historical trace projection"
                .to_owned(),
            sidecar_evidence: "only finite semantic annotations are available, so no unique rule or general-dimensional formula is issued"
                .to_owned(),
            forced_by_constitutive_law: false,
            forced_by_selective_law: false,
            forbidden_by_frozen_kernel: false,
        },
    ]
}

fn comparison_summaries() -> Vec<VariantSummary> {
    vec![
        VariantSummary {
            variant: VariantId::V1ImplicitConstant,
            historical_fit: "typed incumbent presentations have the supplied constant shape at S1, S2, and S3, but lack a base-to-historical-point witness; V1 rejects the supplied Trunc squash boundary"
                .to_owned(),
            basis_formula: Some(FormulaExpression::OnePlusDimensionSquared),
            derivability: DerivabilityStatus::NotForcedOrForbiddenByFrozenKernel,
            uniform_rule_structurally_accepts_all_supplied_diagrams: Some(false),
            uniform_rule_fully_types_all_historical_constructors: Some(false),
            consequences: vec![
                "the historical path counts still replay as typed incumbent constant-boundary presentations"
                    .to_owned(),
                "exact S1/S2/S3 historical typing remains open until the implicit base is linked to the historical point clause"
                    .to_owned(),
                "the Trunc replay remains a generic-loop surrogate".to_owned(),
                "TDC and HIST-CERT claims depending on V1 remain materially conditional"
                    .to_owned(),
            ],
        },
        VariantSummary {
            variant: VariantId::V2DeclaredBoundary,
            historical_fit: "one proposal structurally accepts all four supplied boundary diagrams, but term-level typing is open; none of the diagrams is recoverable from sealed PathCon bytes"
                .to_owned(),
            basis_formula: Some(FormulaExpression::OnePlusDimensionSquared),
            derivability: DerivabilityStatus::NotForcedOrForbiddenByFrozenKernel,
            uniform_rule_structurally_accepts_all_supplied_diagrams: Some(true),
            uniform_rule_fully_types_all_historical_constructors: Some(false),
            consequences: vec![
                "constant boundary is a structurally checked special case".to_owned(),
                "the Trunc endpoint parameters and boundary faces must be supplied outside the sealed grammar"
                    .to_owned(),
                "the raw formula proposal retains its value only when boundary data are premises rather than separately charged exports"
                    .to_owned(),
                "face-term elaboration, a boundary-aware eliminator, and natural-family realization remain explicit obligations"
                    .to_owned(),
            ],
        },
        VariantSummary {
            variant: VariantId::V3TraceDerived,
            historical_fit: "underdetermined: semantic annotations name finite intended shapes, but the sealed trace cannot select them"
                .to_owned(),
            basis_formula: None,
            derivability: DerivabilityStatus::NotDerivableFromDimensionOnlyTrace,
            uniform_rule_structurally_accepts_all_supplied_diagrams: None,
            uniform_rule_fully_types_all_historical_constructors: None,
            consequences: vec![
                "no trace-derived attachment token can be issued".to_owned(),
                "no general-dimensional basis formula follows from the finite annotated rows"
                    .to_owned(),
                "using the V2 rule on the annotations would be V2 evidence, not a V3 derivation"
                    .to_owned(),
            ],
        },
    ]
}

fn build_payload() -> Result<BoundaryAuditPayload, BoundaryAuditError> {
    let mut boundary_diagrams = Vec::new();
    let mut historical_rows = Vec::new();
    for step in [5, 6, 7, 8] {
        let core = build_historical_core(step)?;
        boundary_diagrams.push(core.boundary_diagram);
        historical_rows.extend(core.rows);
    }
    let expected_order = [
        VariantId::V1ImplicitConstant,
        VariantId::V2DeclaredBoundary,
        VariantId::V3TraceDerived,
    ];
    for chunk in historical_rows.chunks_exact(3) {
        if chunk
            .iter()
            .map(|row| row.variant)
            .ne(expected_order.into_iter())
        {
            return Err(BoundaryAuditError::Invariant(
                "historical variant row order drifted".to_owned(),
            ));
        }
    }

    let v1_fails_historical_fit_at_trunc = historical_rows.iter().any(|row| {
        row.step == 6
            && row.variant == VariantId::V1ImplicitConstant
            && row.fit == HistoricalFitVerdict::FailsHistoricalBoundary
    });
    let v2_rows = historical_rows
        .iter()
        .filter(|row| row.variant == VariantId::V2DeclaredBoundary)
        .collect::<Vec<_>>();
    let v2_structurally_accepts_all_supplied_boundary_diagrams = v2_rows.len() == 4
        && v2_rows
            .iter()
            .all(|row| row.supplied_boundary_diagram_structurally_accepted);
    let v2_fully_types_all_historical_constructors = v2_rows.len() == 4
        && v2_rows
            .iter()
            .all(|row| row.historical_constructor_typed_under_variant);
    let variants = [
        VariantId::V1ImplicitConstant,
        VariantId::V2DeclaredBoundary,
        VariantId::V3TraceDerived,
    ];
    let some_single_variant_reproduces_all_recorded_raw_presentations =
        variants.iter().copied().any(|variant| {
            let rows = historical_rows
                .iter()
                .filter(|row| row.variant == variant)
                .collect::<Vec<_>>();
            rows.len() == 4
                && rows
                    .iter()
                    .all(|row| row.presented_basis_count == Some(row.recorded_basis_count))
        });
    let some_single_variant_fully_types_all_historical_constructors =
        variants.iter().copied().any(|variant| {
            let rows = historical_rows
                .iter()
                .filter(|row| row.variant == variant)
                .collect::<Vec<_>>();
            rows.len() == 4
                && rows
                    .iter()
                    .all(|row| row.historical_constructor_typed_under_variant)
        });
    let findings = MaterialFindings {
        v1_fails_historical_fit_at_trunc,
        v1_constant_shape_rows_have_historical_point_binding: false,
        historical_constraint_differs_from_v1_at_trunc: v1_fails_historical_fit_at_trunc,
        tdc_v3_conditionality_is_material: v1_fails_historical_fit_at_trunc,
        hist_cert_per_step_conditionality_flags_are_load_bearing: v1_fails_historical_fit_at_trunc,
        v2_structurally_accepts_all_supplied_boundary_diagrams,
        v2_fully_types_all_historical_constructors,
        v2_diagrams_recoverable_from_trace: false,
        v3_rule_recoverable_from_trace: false,
        finite_cardinality_alignment_discriminates_v1_from_v2: false,
        some_single_variant_reproduces_all_recorded_raw_presentations,
        some_single_variant_fully_types_all_historical_constructors,
        no_boundary_variant_adjudicated: true,
        no_semantic_novelty_or_later_candidate_score_computed: true,
    };
    if !findings.v1_fails_historical_fit_at_trunc
        || findings.v1_constant_shape_rows_have_historical_point_binding
        || !findings.v2_structurally_accepts_all_supplied_boundary_diagrams
        || findings.v2_fully_types_all_historical_constructors
        || findings.v2_diagrams_recoverable_from_trace
        || findings.v3_rule_recoverable_from_trace
        || !findings.some_single_variant_reproduces_all_recorded_raw_presentations
        || findings.some_single_variant_fully_types_all_historical_constructors
    {
        return Err(BoundaryAuditError::Invariant(
            "material boundary findings did not replay".to_owned(),
        ));
    }

    Ok(BoundaryAuditPayload {
        feature_gate: "pen-eval/boundary-audit -> pen-type/boundary-variants".to_owned(),
        boundary_fragment_version: BOUNDARY_VARIANTS_FRAGMENT_VERSION.to_owned(),
        incumbent_cubical_fragment_version: CUBICAL_FRAGMENT_VERSION.to_owned(),
        incumbent_v1_rule_version: PATHCON_ATTACHMENT_AXIOM_VERSION.to_owned(),
        declared_v2_rule_version: DECLARED_BOUNDARY_RULE_VERSION.to_owned(),
        trace_v3_constraint_version: TRACE_DERIVED_BOUNDARY_CONSTRAINT_VERSION.to_owned(),
        incumbent_v1_preservation_scope: "the audit invokes the incumbent V1 realizer and replay APIs directly; source and legacy-artifact hash preservation are external regression checks, not a self-attested payload boolean"
            .to_owned(),
        boundary_diagrams,
        historical_rows,
        formulas: formula_audits(),
        derivability: derivability_audits(),
        comparison: comparison_summaries(),
        findings,
        adjudication_required: true,
        adjudication_options: vec![
            AdjudicationOption {
                option: "retain V1 as the attachment rule".to_owned(),
                recorded_consequences: vec![
                    "S1, S2, and S3 retain source-bound typed constant-boundary presentations with matching shapes"
                        .to_owned(),
                    "exact historical typing still requires a base-to-point-clause witness"
                        .to_owned(),
                    "the historical Trunc constructor remains untyped by that rule"
                        .to_owned(),
                    "the existing TDC and HIST-CERT conditionality remains load-bearing"
                        .to_owned(),
                ],
            },
            AdjudicationOption {
                option: "develop V2 from a structural proposal into a typed declared-boundary rule"
                    .to_owned(),
                recorded_consequences: vec![
                    "the current structural checker accepts all four supplied historical diagrams, but this is not yet a typing result"
                        .to_owned(),
                    "constant boundary is a structural special case".to_owned(),
                    "1+d^2 is currently a raw combinatorial proposal".to_owned(),
                    "adoption capable of discharging the exposure requires term elaboration for contexts, faces, and overlaps plus typed coe/hcom and boundary-aware eliminator evidence"
                        .to_owned(),
                ],
            },
            AdjudicationOption {
                option: "require a genuinely trace-derived V3 rule".to_owned(),
                recorded_consequences: vec![
                    "the current dimension-only trace is insufficient".to_owned(),
                    "additional sealed boundary syntax or an explicit per-constructor semantic axiom is required"
                        .to_owned(),
                    "until then no V3 general-dimensional formula exists".to_owned(),
                ],
            },
            AdjudicationOption {
                option: "defer attachment-rule adjudication".to_owned(),
                recorded_consequences: vec![
                    "all affected results remain theory-relative".to_owned(),
                    "the Trunc mismatch and trace underdetermination remain explicit blockers"
                        .to_owned(),
                ],
            },
        ],
        remaining_obligations: vec![
            "for exact V1 historical typing at S1, S2, and S3, bind the implicit cubical base to the historical point clause with replayable typed evidence"
                .to_owned(),
            "before V2 can establish historical fit, elaborate its contexts and face annotations as terms and type their overlap equalities"
                .to_owned(),
            "add typed coe/hcom, boundary-aware motive methods, and eliminator computation for nonconstant boundaries"
                .to_owned(),
            "prove substitution/naturality for the parameterized Trunc squash family rather than one schematic endpoint instance"
                .to_owned(),
            "prove independence and exhaustiveness of the beta/transport/naturality presentation before treating it as semantic novelty"
                .to_owned(),
            "decide whether boundary-map components are premises or separately exported demand orbits; the latter destroys a dimension-only formula"
                .to_owned(),
            "record the user's boundary-rule adjudication before removing any conditionality flag"
                .to_owned(),
        ],
    })
}

fn payload_replays(presented: &BoundaryAuditPayload) -> bool {
    build_payload().is_ok_and(|canonical| &canonical == presented)
}

fn mutation_falsifiers(payload: &BoundaryAuditPayload) -> MutationFalsifiers {
    let flipped_fit_verdict_rejected = (0..payload.historical_rows.len()).all(|row_index| {
        let mut mutation = payload.clone();
        mutation.historical_rows[row_index].fit = match mutation.historical_rows[row_index].fit {
            HistoricalFitVerdict::IncumbentTypedPresentationShapeFit => {
                HistoricalFitVerdict::FailsHistoricalBoundary
            }
            HistoricalFitVerdict::FailsHistoricalBoundary => {
                HistoricalFitVerdict::IncumbentTypedPresentationShapeFit
            }
            HistoricalFitVerdict::StructuralBoundaryAcceptedTypingOpen => {
                HistoricalFitVerdict::TraceUnderdetermined
            }
            HistoricalFitVerdict::TraceUnderdetermined => {
                HistoricalFitVerdict::StructuralBoundaryAcceptedTypingOpen
            }
        };
        !payload_replays(&mutation)
    });

    let mutate_formula = |formula: Option<FormulaExpression>| match formula {
        Some(FormulaExpression::OnePlusDimensionSquared) | None => {
            Some(FormulaExpression::OnePlusTwiceDimension)
        }
        Some(FormulaExpression::OnePlusTwiceDimension) => {
            Some(FormulaExpression::OnePlusDimensionSquared)
        }
    };
    let formula_table_mutations_rejected = (0..payload.formulas.len()).all(|formula_index| {
        let mut mutation = payload.clone();
        mutation.formulas[formula_index].formula =
            mutate_formula(mutation.formulas[formula_index].formula);
        !payload_replays(&mutation)
    });
    let historical_formula_mutations_rejected =
        (0..payload.historical_rows.len()).all(|row_index| {
            let mut mutation = payload.clone();
            mutation.historical_rows[row_index].basis_formula =
                mutate_formula(mutation.historical_rows[row_index].basis_formula);
            !payload_replays(&mutation)
        });
    let comparison_formula_mutations_rejected =
        (0..payload.comparison.len()).all(|summary_index| {
            let mut mutation = payload.clone();
            mutation.comparison[summary_index].basis_formula =
                mutate_formula(mutation.comparison[summary_index].basis_formula);
            !payload_replays(&mutation)
        });
    let mutated_formula_rejected = formula_table_mutations_rejected
        && historical_formula_mutations_rejected
        && comparison_formula_mutations_rejected;

    let mut diagram = payload.clone();
    diagram.boundary_diagrams[0]
        .boundary_diagram_digest
        .push_str(":mutated");
    let mutated_boundary_diagram_rejected = !payload_replays(&diagram);

    let mut v3 = payload.clone();
    v3.findings.v3_rule_recoverable_from_trace = true;
    let false_v3_trace_recovery_rejected = !payload_replays(&v3);

    let mut token = payload.clone();
    token.historical_rows[0]
        .attachment_token_derivation_hash
        .as_mut()
        .expect("V1 row has attachment token")
        .push_str(":mutated");
    let mutated_token_hash_rejected = !payload_replays(&token);

    let mut deleted = payload.clone();
    deleted.historical_rows.remove(0);
    let deleted_historical_row_rejected = !payload_replays(&deleted);

    // Replay rebuilds the payload from sealed sources before comparing the
    // outer digest.  Therefore recomputing an outer digest cannot legitimize
    // the already rejected semantic mutation above.
    let mut fit = payload.clone();
    fit.historical_rows[0].fit = HistoricalFitVerdict::FailsHistoricalBoundary;
    let forged_outer_digest = tagged_hash("attacker-rebased-result", &fit);
    let canonical_outer_digest = tagged_hash("attacker-rebased-result", payload);
    let rebased_outer_digest_rejected =
        forged_outer_digest != canonical_outer_digest && !payload_replays(&fit);

    let all_passed = flipped_fit_verdict_rejected
        && mutated_formula_rejected
        && mutated_boundary_diagram_rejected
        && false_v3_trace_recovery_rejected
        && mutated_token_hash_rejected
        && deleted_historical_row_rejected
        && rebased_outer_digest_rejected;
    MutationFalsifiers {
        flipped_fit_verdict_rejected,
        mutated_formula_rejected,
        mutated_boundary_diagram_rejected,
        false_v3_trace_recovery_rejected,
        mutated_token_hash_rejected,
        deleted_historical_row_rejected,
        rebased_outer_digest_rejected,
        all_passed,
    }
}

fn finalize_result_digest(result: &BoundaryAuditResult) -> String {
    let mut hashable = result.clone();
    hashable.result_digest.clear();
    tagged_hash("boundary-audit-result", &hashable)
}

pub fn build_boundary_audit() -> Result<BoundaryAuditResult, BoundaryAuditError> {
    let payload = build_payload()?;
    let mutation_falsifiers = mutation_falsifiers(&payload);
    if !mutation_falsifiers.all_passed {
        return Err(BoundaryAuditError::Invariant(
            "mutation falsifier battery did not fail closed".to_owned(),
        ));
    }
    let mut result = BoundaryAuditResult {
        schema: BOUNDARY_AUDIT_SCHEMA.to_owned(),
        payload,
        mutation_falsifiers,
        result_digest: String::new(),
    };
    result.result_digest = finalize_result_digest(&result);
    Ok(result)
}

pub fn replay_boundary_audit(presented: &BoundaryAuditResult) -> Result<(), BoundaryAuditError> {
    if &build_boundary_audit()? == presented {
        Ok(())
    } else {
        Err(BoundaryAuditError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_variants_are_audited_over_all_historical_constructors() {
        let result = build_boundary_audit().expect("boundary audit builds");
        assert_eq!(result.payload.boundary_diagrams.len(), 4);
        assert_eq!(result.payload.historical_rows.len(), 12);
        for step in [5, 6, 7, 8] {
            let rows = result
                .payload
                .historical_rows
                .iter()
                .filter(|row| row.step == step)
                .collect::<Vec<_>>();
            assert_eq!(rows.len(), 3);
        }
    }

    #[test]
    fn v1_trunc_rejection_v2_typing_gap_and_v3_ambiguity_are_explicit() {
        let result = build_boundary_audit().expect("boundary audit builds");
        let trunc = result
            .payload
            .historical_rows
            .iter()
            .filter(|row| row.step == 6)
            .collect::<Vec<_>>();
        assert_eq!(trunc[0].fit, HistoricalFitVerdict::FailsHistoricalBoundary);
        assert_eq!(
            trunc[0].intended_boundary_attempt_error.as_deref(),
            Some("V1 accepts only an implicit constant boundary")
        );
        assert_eq!(
            trunc[1].fit,
            HistoricalFitVerdict::StructuralBoundaryAcceptedTypingOpen
        );
        assert!(!trunc[1].historical_constructor_typed_under_variant);
        assert_eq!(trunc[2].fit, HistoricalFitVerdict::TraceUnderdetermined);
        assert!(
            result
                .payload
                .boundary_diagrams
                .iter()
                .all(|diagram| diagram.trace_projection_noninjectivity_checked
                    && !diagram.diagram_recoverable_from_sealed_trace)
        );
        assert!(result.payload.findings.tdc_v3_conditionality_is_material);
        assert!(
            result
                .payload
                .historical_rows
                .iter()
                .all(|row| !row.historical_constructor_typed_under_variant)
        );
        assert!(
            !result
                .payload
                .findings
                .v1_constant_shape_rows_have_historical_point_binding
        );
        assert!(
            result
                .payload
                .findings
                .hist_cert_per_step_conditionality_flags_are_load_bearing
        );
    }

    #[test]
    fn basis_alignment_and_formula_status_are_fail_closed() {
        let result = build_boundary_audit().expect("boundary audit builds");
        let observed = result
            .payload
            .historical_rows
            .iter()
            .filter(|row| row.variant == VariantId::V2DeclaredBoundary)
            .map(|row| row.presented_basis_count)
            .collect::<Vec<_>>();
        assert_eq!(observed, vec![Some(2), Some(2), Some(5), Some(10)]);
        assert_eq!(
            result.payload.formulas[0].formula,
            Some(FormulaExpression::OnePlusDimensionSquared)
        );
        assert_eq!(
            result.payload.formulas[1].formula,
            Some(FormulaExpression::OnePlusDimensionSquared)
        );
        assert_eq!(result.payload.formulas[2].formula, None);
        assert_eq!(
            result.payload.formulas[1].evidence_status,
            FormulaEvidenceStatus::RawCombinatorialProposal
        );
        assert!(
            result
                .payload
                .historical_rows
                .iter()
                .all(|row| !row.semantic_exhaustiveness_proved)
        );
    }

    #[test]
    fn replay_and_every_registered_mutation_fail_closed() {
        let result = build_boundary_audit().expect("boundary audit builds");
        replay_boundary_audit(&result).expect("exact replay");
        assert!(result.mutation_falsifiers.all_passed);

        let mut fit = result.clone();
        fit.payload.historical_rows[0].fit = HistoricalFitVerdict::FailsHistoricalBoundary;
        fit.result_digest = finalize_result_digest(&fit);
        assert!(matches!(
            replay_boundary_audit(&fit),
            Err(BoundaryAuditError::ReplayMismatch)
        ));

        let mut formula = result.clone();
        formula.payload.formulas[0].formula = Some(FormulaExpression::OnePlusTwiceDimension);
        formula.result_digest = finalize_result_digest(&formula);
        assert!(matches!(
            replay_boundary_audit(&formula),
            Err(BoundaryAuditError::ReplayMismatch)
        ));
    }

    #[test]
    fn result_is_decision_neutral_and_has_no_novelty_score() {
        let result = build_boundary_audit().expect("boundary audit builds");
        assert!(result.payload.adjudication_required);
        assert!(result.payload.findings.no_boundary_variant_adjudicated);
        assert!(
            result
                .payload
                .findings
                .no_semantic_novelty_or_later_candidate_score_computed
        );
        assert_eq!(result.payload.comparison.len(), 3);
        assert!(
            result
                .payload
                .findings
                .some_single_variant_reproduces_all_recorded_raw_presentations
        );
        assert!(
            !result
                .payload
                .findings
                .some_single_variant_fully_types_all_historical_constructors
        );
    }
}
