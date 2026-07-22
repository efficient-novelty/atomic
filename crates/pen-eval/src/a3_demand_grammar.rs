//! Count-blind generator half of the historical A3 demand grammar.
//!
//! This module deliberately has no dependency on the directive-debt
//! timeline, admissibility focus labels, candidate scores, bars, expected
//! package tables, or a successor winner.  A historical window is rebuilt
//! from the raw sealed telescopes and every source clause is elaborated.
//! Unary, chronological, and higher base rules range over those typed public
//! sources in every complete window.  Separately, the two-entry
//! [`StructuralDebt`] summary selects completion-hole constructors whose
//! actual demand orbits project focus.  Thus structural completion cannot
//! erase the Stage-16 base inventory.
//!
//! The result is the *generator half* of E-5.  It supplies typed schemes,
//! typed instances, the frozen-equality/orbit quotient hook, and the
//! constructor-evidence focus projection.  It intentionally does not decide
//! `D(B)`, execute F1, issue Theorem 12, or authorize a bridge/halt result.

use crate::typed_families::{CanonicalPresentation, clause_presentation};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_type::elaborate::{ClauseFailure, KernelTy, SealedSignature, elaborate_telescope};
use pen_type::equality::{EqualityWitness, KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use pen_type::obligations::{StructuralDebt, summarize_structural_debt};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const A3_HISTORICAL_GRAMMAR_SCHEMA: &str = "a3-historical-demand-grammar-v1";
pub const A3_WINDOW_DEPTH: u16 = 2;
pub const A3_FIRST_STAGE: u32 = 1;
pub const A3_LAST_STAGE: u32 = 16;
pub const CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1: &str = "chronological-interface-slot-map-v1";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(A3_HISTORICAL_GRAMMAR_SCHEMA, domain, value))
        .expect("A3 grammar values serialize");
    format!("blake3:{}", blake3_hex(&bytes))
}

/// The closed structural-obligation constructor inventory.  These are
/// constructors, not accepted package labels: each variant is minted only
/// by replaying its predicate over a raw two-entry structural summary.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum A3DemandConstructor {
    FormerEliminator,
    InitialHit,
    TruncationHit,
    HigherHit,
    SphereLift,
    AxiomaticBundle,
    ModalShell,
    ConnectionShell,
    CurvatureShell,
    OperatorBundle,
    HilbertFunctional,
    TemporalShell,
}

impl A3DemandConstructor {
    pub const ALL: [Self; 12] = [
        Self::FormerEliminator,
        Self::InitialHit,
        Self::TruncationHit,
        Self::HigherHit,
        Self::SphereLift,
        Self::AxiomaticBundle,
        Self::ModalShell,
        Self::ConnectionShell,
        Self::CurvatureShell,
        Self::OperatorBundle,
        Self::HilbertFunctional,
        Self::TemporalShell,
    ];

    /// Stable semantic characteristic key for typed discharge checks.  This
    /// is a constructor projection, not an input to generation.
    pub const fn slug(self) -> &'static str {
        match self {
            Self::FormerEliminator => "former_eliminator",
            Self::InitialHit => "initial_hit",
            Self::TruncationHit => "truncation_hit",
            Self::HigherHit => "higher_hit",
            Self::SphereLift => "sphere_lift",
            Self::AxiomaticBundle => "axiomatic_bundle",
            Self::ModalShell => "modal_shell",
            Self::ConnectionShell => "connection_shell",
            Self::CurvatureShell => "curvature_shell",
            Self::OperatorBundle => "operator_bundle",
            Self::HilbertFunctional => "hilbert_functional",
            Self::TemporalShell => "temporal_shell",
        }
    }
}

/// The closed instance-rule inventory.  `UnaryAction` is the ordinary
/// family action; `ChronologicalComparison` is the genuinely depth-two
/// rule; `HigherOpenBoxReduction` is enabled only by a typed path source.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum A3RuleConstructor {
    UnaryAction,
    ChronologicalComparison,
    HigherOpenBoxReduction,
    StructuralCompletionHole,
}

impl A3RuleConstructor {
    pub const ALL: [Self; 4] = [
        Self::UnaryAction,
        Self::ChronologicalComparison,
        Self::HigherOpenBoxReduction,
        Self::StructuralCompletionHole,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum A3WindowLayer {
    Older,
    Newest,
}

/// Serializable copy of the raw structural summary used by constructor
/// replay.  No score, bar, historical count, or caller label occurs here.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
pub struct A3StructuralSnapshot {
    pub active_entries: u16,
    pub active_exports: u16,
    pub foundation_entries: u16,
    pub constructor_entries: u16,
    pub dependent_entries: u16,
    pub truncated_entries: u16,
    pub max_path_dimension: u16,
    pub modal_entries: u16,
    pub modal_coupled_entries: u16,
    pub differential_entries: u16,
    pub differential_coupled_entries: u16,
    pub curvature_entries: u16,
    pub operator_bundle_entries: u16,
    pub hilbert_shell_entries: u16,
    pub temporal_shell_entries: u16,
    pub has_modal_ops: bool,
    pub has_temporal_ops: bool,
}

impl From<StructuralDebt> for A3StructuralSnapshot {
    fn from(debt: StructuralDebt) -> Self {
        Self {
            active_entries: debt.active_entries,
            active_exports: debt.active_exports,
            foundation_entries: debt.foundation_entries,
            constructor_entries: debt.constructor_entries,
            dependent_entries: debt.dependent_entries,
            truncated_entries: debt.truncated_entries,
            max_path_dimension: debt.max_path_dimension,
            modal_entries: debt.modal_entries,
            modal_coupled_entries: debt.modal_coupled_entries,
            differential_entries: debt.differential_entries,
            differential_coupled_entries: debt.differential_coupled_entries,
            curvature_entries: debt.curvature_entries,
            operator_bundle_entries: debt.operator_bundle_entries,
            hilbert_shell_entries: debt.hilbert_shell_entries,
            temporal_shell_entries: debt.temporal_shell_entries,
            has_modal_ops: debt.has_modal_ops,
            has_temporal_ops: debt.has_temporal_ops,
        }
    }
}

/// Atomic premises used by the twelve structural constructors.  Recording
/// these makes a constructor token replayable without trusting a variant
/// name or a boolean supplied by its caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "premise", content = "value")]
pub enum A3StructuralPremise {
    ActiveEntriesAtLeast(u16),
    ActiveExportsAtLeast(u16),
    ActiveExportsLessThan(u16),
    ConstructorEntriesAtLeast(u16),
    ConstructorEntriesEqual(u16),
    DependentEntriesAtLeast(u16),
    DependentEntriesEqual(u16),
    TruncatedEntriesAtLeast(u16),
    TruncatedEntriesEqual(u16),
    MaxPathDimensionEqual(u16),
    ModalEntriesAtLeast(u16),
    ModalCoupledEntriesAtLeast(u16),
    ModalCoupledEntriesEqual(u16),
    DifferentialEntriesAtLeast(u16),
    DifferentialCoupledEntriesAtLeast(u16),
    CurvatureEntriesAtLeast(u16),
    CurvatureEntriesEqual(u16),
    OperatorBundleEntriesAtLeast(u16),
    OperatorBundleEntriesEqual(u16),
    HilbertShellEntriesAtLeast(u16),
    HilbertShellEntriesEqual(u16),
    TemporalShellEntriesEqual(u16),
    NoModalOperations,
    NoTemporalOperations,
}

impl A3StructuralPremise {
    fn holds(self, debt: A3StructuralSnapshot) -> bool {
        match self {
            Self::ActiveEntriesAtLeast(value) => debt.active_entries >= value,
            Self::ActiveExportsAtLeast(value) => debt.active_exports >= value,
            Self::ActiveExportsLessThan(value) => debt.active_exports < value,
            Self::ConstructorEntriesAtLeast(value) => debt.constructor_entries >= value,
            Self::ConstructorEntriesEqual(value) => debt.constructor_entries == value,
            Self::DependentEntriesAtLeast(value) => debt.dependent_entries >= value,
            Self::DependentEntriesEqual(value) => debt.dependent_entries == value,
            Self::TruncatedEntriesAtLeast(value) => debt.truncated_entries >= value,
            Self::TruncatedEntriesEqual(value) => debt.truncated_entries == value,
            Self::MaxPathDimensionEqual(value) => debt.max_path_dimension == value,
            Self::ModalEntriesAtLeast(value) => debt.modal_entries >= value,
            Self::ModalCoupledEntriesAtLeast(value) => debt.modal_coupled_entries >= value,
            Self::ModalCoupledEntriesEqual(value) => debt.modal_coupled_entries == value,
            Self::DifferentialEntriesAtLeast(value) => debt.differential_entries >= value,
            Self::DifferentialCoupledEntriesAtLeast(value) => {
                debt.differential_coupled_entries >= value
            }
            Self::CurvatureEntriesAtLeast(value) => debt.curvature_entries >= value,
            Self::CurvatureEntriesEqual(value) => debt.curvature_entries == value,
            Self::OperatorBundleEntriesAtLeast(value) => debt.operator_bundle_entries >= value,
            Self::OperatorBundleEntriesEqual(value) => debt.operator_bundle_entries == value,
            Self::HilbertShellEntriesAtLeast(value) => debt.hilbert_shell_entries >= value,
            Self::HilbertShellEntriesEqual(value) => debt.hilbert_shell_entries == value,
            Self::TemporalShellEntriesEqual(value) => debt.temporal_shell_entries == value,
            Self::NoModalOperations => !debt.has_modal_ops,
            Self::NoTemporalOperations => !debt.has_temporal_ops,
        }
    }
}

fn premises_for(constructor: A3DemandConstructor) -> Vec<A3StructuralPremise> {
    use A3StructuralPremise as P;
    match constructor {
        A3DemandConstructor::FormerEliminator => vec![
            P::MaxPathDimensionEqual(0),
            P::NoModalOperations,
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ConstructorEntriesAtLeast(2),
            P::DependentEntriesEqual(0),
        ],
        A3DemandConstructor::InitialHit => vec![
            P::MaxPathDimensionEqual(0),
            P::NoModalOperations,
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ConstructorEntriesAtLeast(1),
            P::DependentEntriesAtLeast(1),
        ],
        A3DemandConstructor::TruncationHit => vec![
            P::MaxPathDimensionEqual(1),
            P::TruncatedEntriesEqual(0),
            P::NoModalOperations,
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
        ],
        A3DemandConstructor::HigherHit => vec![
            P::MaxPathDimensionEqual(1),
            P::TruncatedEntriesAtLeast(1),
            P::NoModalOperations,
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ConstructorEntriesAtLeast(1),
        ],
        A3DemandConstructor::SphereLift => vec![
            P::MaxPathDimensionEqual(2),
            P::TruncatedEntriesAtLeast(1),
            P::NoModalOperations,
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ConstructorEntriesAtLeast(1),
        ],
        A3DemandConstructor::AxiomaticBundle => vec![
            P::MaxPathDimensionEqual(3),
            P::TruncatedEntriesEqual(0),
            P::NoModalOperations,
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ActiveExportsAtLeast(6),
            P::ConstructorEntriesAtLeast(2),
        ],
        A3DemandConstructor::ModalShell => vec![
            P::MaxPathDimensionEqual(3),
            P::TruncatedEntriesEqual(0),
            P::NoModalOperations,
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ActiveExportsAtLeast(4),
            P::ActiveExportsLessThan(6),
            P::ConstructorEntriesEqual(1),
        ],
        A3DemandConstructor::ConnectionShell => vec![
            P::MaxPathDimensionEqual(0),
            P::TruncatedEntriesEqual(0),
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ActiveExportsAtLeast(2),
            P::ConstructorEntriesEqual(0),
            P::ModalEntriesAtLeast(1),
            P::ModalCoupledEntriesEqual(0),
        ],
        A3DemandConstructor::CurvatureShell => vec![
            P::MaxPathDimensionEqual(0),
            P::TruncatedEntriesEqual(0),
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ActiveExportsAtLeast(2),
            P::ConstructorEntriesEqual(0),
            P::ModalEntriesAtLeast(1),
            P::ModalCoupledEntriesAtLeast(1),
            P::DifferentialEntriesAtLeast(1),
            P::CurvatureEntriesEqual(0),
        ],
        A3DemandConstructor::OperatorBundle => vec![
            P::MaxPathDimensionEqual(0),
            P::TruncatedEntriesEqual(0),
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ActiveExportsAtLeast(2),
            P::ConstructorEntriesEqual(0),
            P::DifferentialCoupledEntriesAtLeast(2),
            P::CurvatureEntriesAtLeast(1),
            P::OperatorBundleEntriesEqual(0),
        ],
        A3DemandConstructor::HilbertFunctional => vec![
            P::MaxPathDimensionEqual(0),
            P::TruncatedEntriesEqual(0),
            P::NoTemporalOperations,
            P::ActiveEntriesAtLeast(2),
            P::ActiveExportsAtLeast(2),
            P::ConstructorEntriesEqual(0),
            P::DifferentialCoupledEntriesAtLeast(2),
            P::CurvatureEntriesAtLeast(1),
            P::OperatorBundleEntriesAtLeast(1),
            P::HilbertShellEntriesEqual(0),
        ],
        A3DemandConstructor::TemporalShell => vec![
            P::MaxPathDimensionEqual(0),
            P::TruncatedEntriesEqual(0),
            P::ActiveEntriesAtLeast(2),
            P::ActiveExportsAtLeast(2),
            P::ConstructorEntriesEqual(0),
            P::OperatorBundleEntriesAtLeast(1),
            P::HilbertShellEntriesAtLeast(1),
            P::TemporalShellEntriesEqual(0),
        ],
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3ConstructorEvidence {
    pub constructor: A3DemandConstructor,
    pub structural_snapshot: A3StructuralSnapshot,
    pub premises: Vec<A3StructuralPremise>,
    pub evidence_hash: String,
}

fn constructor_evidence(
    debt: StructuralDebt,
) -> Result<Vec<A3ConstructorEvidence>, A3DemandGrammarError> {
    let snapshot = A3StructuralSnapshot::from(debt);
    A3DemandConstructor::ALL
        .into_iter()
        .filter_map(|constructor| {
            let premises = premises_for(constructor);
            if !premises.iter().all(|premise| premise.holds(snapshot)) {
                return None;
            }
            let evidence_hash =
                tagged_hash("constructor-evidence", &(constructor, snapshot, &premises));
            Some(Ok(A3ConstructorEvidence {
                constructor,
                structural_snapshot: snapshot,
                premises,
                evidence_hash,
            }))
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3TypedClauseSource {
    pub anchor_id: String,
    pub layer: A3WindowLayer,
    pub step: u32,
    pub candidate_hash: String,
    pub clause_index: u16,
    pub kernel_role: ClauseRole,
    pub kernel_type: KernelTy,
    pub normal_form: Expr,
    pub canonical_presentation: CanonicalPresentation,
    pub canonical_family_key: String,
    pub exported_public_clause: bool,
    pub public_eligibility_hash: String,
    pub telescope_elaboration_hash: String,
    pub typing_derivation_hash: String,
}

fn typed_sources_for_step(
    signature: &SealedSignature,
    step: u32,
    layer: A3WindowLayer,
) -> Result<Vec<A3TypedClauseSource>, A3DemandGrammarError> {
    let entry = signature
        .entry(step)
        .ok_or(A3DemandGrammarError::MissingHistoricalStep(step))?;
    let elaboration = elaborate_telescope(signature, &entry.telescope, step.saturating_sub(1))
        .map_err(|failure| A3DemandGrammarError::HistoricalElaboration { step, failure })?;
    let kernel_roles = elaboration
        .clauses
        .iter()
        .map(|clause| clause.kernel_role)
        .collect::<Vec<_>>();

    elaboration
        .clauses
        .iter()
        .map(|clause| {
            let free_scope_len = elaboration.ambient_parameters + u32::from(clause.clause_index);
            let canonical_presentation = clause_presentation(
                &clause.normal_form,
                free_scope_len,
                &kernel_roles[..usize::from(clause.clause_index)],
                elaboration.ambient_parameters,
            );
            let canonical_family_key = tagged_hash(
                "typed-source-family",
                &(
                    signature.digest(),
                    KERNEL_EQUALITY_PROCEDURE,
                    &canonical_presentation.canonical_normal_form,
                    &canonical_presentation.parameters,
                ),
            );
            let typing_derivation_hash = tagged_hash(
                "typed-source",
                &(
                    step,
                    &entry.candidate_hash,
                    clause.clause_index,
                    clause.kernel_role,
                    &clause.kernel_ty,
                    &clause.normal_form,
                    &canonical_family_key,
                    &elaboration.derivation_hash,
                ),
            );
            let anchor_id = tagged_hash(
                "source-anchor",
                &(layer, step, clause.clause_index, &typing_derivation_hash),
            );
            // A clause owned by a sealed telescope is an exported public
            // clause of that entry.  The ownership tuple is hashed so this
            // fact cannot be transferred to a different clause occurrence.
            let public_eligibility_hash = tagged_hash(
                "public-clause-eligibility",
                &(
                    step,
                    &entry.candidate_hash,
                    clause.clause_index,
                    entry.telescope.clauses.len(),
                    &typing_derivation_hash,
                ),
            );
            Ok(A3TypedClauseSource {
                anchor_id,
                layer,
                step,
                candidate_hash: entry.candidate_hash.clone(),
                clause_index: clause.clause_index,
                kernel_role: clause.kernel_role,
                kernel_type: clause.kernel_ty.clone(),
                normal_form: clause.normal_form.clone(),
                canonical_presentation,
                canonical_family_key,
                exported_public_clause: true,
                public_eligibility_hash,
                telescope_elaboration_hash: elaboration.derivation_hash.clone(),
                typing_derivation_hash,
            })
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "interface_mode")]
pub enum A3ChronologicalInterfaceMode {
    DirectType,
    PointwiseTypeValuedFunction { domain: KernelTy },
}

/// One enumerated interface-to-parameter assignment. Both coordinates are
/// one-based. Interface slots follow sealing order and parameters follow
/// their declared dependent-context order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct A3ChronologicalInterfaceSlotAssignment {
    pub interface_slot: u32,
    pub parameter: u32,
}

/// Registration-time declaration carried by every chronological scheme.
///
/// This value is minted before any instance is tested. It contains no
/// success-dependent branch and permits no instance override. Its only
/// lawful value at arity `n` is the enumerated identity `1->1, ..., n->n`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3ChronologicalInterfaceSlotMap {
    pub rule: String,
    pub declared_arity: u32,
    pub assignments: Vec<A3ChronologicalInterfaceSlotAssignment>,
    pub declaration_order_is_sequential: bool,
    pub sealing_order_is_older_before_newest: bool,
    pub inferred_from_derivation_success: bool,
    pub instance_override_permitted: bool,
    pub kappa_charge: u32,
    pub nu_charge: u32,
    pub declaration_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum A3ChronologicalInterfaceSlotMapError {
    #[error("chronological slot-map rule id drifted")]
    RuleDrift,
    #[error("chronological slot-map arity is {found}, expected {expected}")]
    ArityDrift { expected: u32, found: u32 },
    #[error("chronological slot-map is not the enumerated order-preserving identity")]
    NotOrderPreservingIdentity,
    #[error("chronological slot-map declaration-order or sealing-order evidence is absent")]
    OrderEvidenceAbsent,
    #[error("chronological slot-map was inferred or permits an instance override")]
    InferenceOrOverride,
    #[error("chronological slot-map minted a nonzero charge")]
    NonzeroCharge,
    #[error("chronological slot-map declaration hash failed replay")]
    HashMismatch,
}

fn chronological_slot_map_hash(
    rule: &str,
    declared_arity: u32,
    assignments: &[A3ChronologicalInterfaceSlotAssignment],
    declaration_order_is_sequential: bool,
    sealing_order_is_older_before_newest: bool,
    inferred_from_derivation_success: bool,
    instance_override_permitted: bool,
    kappa_charge: u32,
    nu_charge: u32,
) -> String {
    tagged_hash(
        "chronological-interface-slot-map-declaration",
        &(
            rule,
            declared_arity,
            assignments,
            declaration_order_is_sequential,
            sealing_order_is_older_before_newest,
            inferred_from_derivation_success,
            instance_override_permitted,
            kappa_charge,
            nu_charge,
        ),
    )
}

/// Declare the unique count-blind map authorized by
/// `chronological-interface-slot-map-v1`.
pub fn declare_chronological_interface_slot_map(
    declared_arity: u32,
) -> A3ChronologicalInterfaceSlotMap {
    let assignments = (1..=declared_arity)
        .map(|index| A3ChronologicalInterfaceSlotAssignment {
            interface_slot: index,
            parameter: index,
        })
        .collect::<Vec<_>>();
    let rule = CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1.to_owned();
    let declaration_order_is_sequential = true;
    let sealing_order_is_older_before_newest = true;
    let inferred_from_derivation_success = false;
    let instance_override_permitted = false;
    let kappa_charge = 0;
    let nu_charge = 0;
    let declaration_hash = chronological_slot_map_hash(
        &rule,
        declared_arity,
        &assignments,
        declaration_order_is_sequential,
        sealing_order_is_older_before_newest,
        inferred_from_derivation_success,
        instance_override_permitted,
        kappa_charge,
        nu_charge,
    );
    A3ChronologicalInterfaceSlotMap {
        rule,
        declared_arity,
        assignments,
        declaration_order_is_sequential,
        sealing_order_is_older_before_newest,
        inferred_from_derivation_success,
        instance_override_permitted,
        kappa_charge,
        nu_charge,
        declaration_hash,
    }
}

/// Definition replay. No alternative permutation is searched when this
/// check fails.
pub fn replay_chronological_interface_slot_map(
    declaration: &A3ChronologicalInterfaceSlotMap,
    expected_arity: u32,
) -> Result<(), A3ChronologicalInterfaceSlotMapError> {
    if declaration.rule != CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1 {
        return Err(A3ChronologicalInterfaceSlotMapError::RuleDrift);
    }
    if declaration.declared_arity != expected_arity {
        return Err(A3ChronologicalInterfaceSlotMapError::ArityDrift {
            expected: expected_arity,
            found: declaration.declared_arity,
        });
    }
    let expected = (1..=expected_arity)
        .map(|index| A3ChronologicalInterfaceSlotAssignment {
            interface_slot: index,
            parameter: index,
        })
        .collect::<Vec<_>>();
    if declaration.assignments != expected {
        return Err(A3ChronologicalInterfaceSlotMapError::NotOrderPreservingIdentity);
    }
    if !declaration.declaration_order_is_sequential
        || !declaration.sealing_order_is_older_before_newest
    {
        return Err(A3ChronologicalInterfaceSlotMapError::OrderEvidenceAbsent);
    }
    if declaration.inferred_from_derivation_success || declaration.instance_override_permitted {
        return Err(A3ChronologicalInterfaceSlotMapError::InferenceOrOverride);
    }
    if declaration.kappa_charge != 0 || declaration.nu_charge != 0 {
        return Err(A3ChronologicalInterfaceSlotMapError::NonzeroCharge);
    }
    let expected_hash = chronological_slot_map_hash(
        &declaration.rule,
        declaration.declared_arity,
        &declaration.assignments,
        declaration.declaration_order_is_sequential,
        declaration.sealing_order_is_older_before_newest,
        declaration.inferred_from_derivation_success,
        declaration.instance_override_permitted,
        declaration.kappa_charge,
        declaration.nu_charge,
    );
    if declaration.declaration_hash != expected_hash {
        return Err(A3ChronologicalInterfaceSlotMapError::HashMismatch);
    }
    Ok(())
}

fn chronological_interface_mode(kernel_type: &KernelTy) -> Option<A3ChronologicalInterfaceMode> {
    match kernel_type {
        KernelTy::Type => Some(A3ChronologicalInterfaceMode::DirectType),
        KernelTy::Fun(domain, codomain) if matches!(codomain.as_ref(), KernelTy::Type) => {
            Some(A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction {
                domain: domain.as_ref().clone(),
            })
        }
        _ => None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "output")]
pub enum A3DemandOutputType {
    ActionAt {
        source_family: String,
        source_type: KernelTy,
    },
    ChronologicalInteraction {
        older_family: String,
        older_type: KernelTy,
        newest_family: String,
        newest_type: KernelTy,
        interface_mode: A3ChronologicalInterfaceMode,
        interface_slot_map: A3ChronologicalInterfaceSlotMap,
    },
    ContractibleOpenBox {
        dimension: u32,
        boundary_families: Vec<String>,
        boundary_types: Vec<KernelTy>,
        path_witness_families: Vec<String>,
    },
    StructuralCompletion {
        constructor: A3DemandConstructor,
        structural_snapshot: A3StructuralSnapshot,
    },
}

/// Hash projection frozen before the slot-map declaration was added. The
/// declaration is evidence attached to a scheme, not a new semantic demand
/// output, so existing scheme and instance identities must not churn.
#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "output")]
enum A3DemandOutputIdentityKey<'a> {
    ActionAt {
        source_family: &'a String,
        source_type: &'a KernelTy,
    },
    ChronologicalInteraction {
        older_family: &'a String,
        older_type: &'a KernelTy,
        newest_family: &'a String,
        newest_type: &'a KernelTy,
        interface_mode: &'a A3ChronologicalInterfaceMode,
    },
    ContractibleOpenBox {
        dimension: u32,
        boundary_families: &'a Vec<String>,
        boundary_types: &'a Vec<KernelTy>,
        path_witness_families: &'a Vec<String>,
    },
    StructuralCompletion {
        constructor: A3DemandConstructor,
        structural_snapshot: &'a A3StructuralSnapshot,
    },
}

impl A3DemandOutputType {
    fn frozen_identity_key(&self) -> A3DemandOutputIdentityKey<'_> {
        match self {
            Self::ActionAt {
                source_family,
                source_type,
            } => A3DemandOutputIdentityKey::ActionAt {
                source_family,
                source_type,
            },
            Self::ChronologicalInteraction {
                older_family,
                older_type,
                newest_family,
                newest_type,
                interface_mode,
                ..
            } => A3DemandOutputIdentityKey::ChronologicalInteraction {
                older_family,
                older_type,
                newest_family,
                newest_type,
                interface_mode,
            },
            Self::ContractibleOpenBox {
                dimension,
                boundary_families,
                boundary_types,
                path_witness_families,
            } => A3DemandOutputIdentityKey::ContractibleOpenBox {
                dimension: *dimension,
                boundary_families,
                boundary_types,
                path_witness_families,
            },
            Self::StructuralCompletion {
                constructor,
                structural_snapshot,
            } => A3DemandOutputIdentityKey::StructuralCompletion {
                constructor: *constructor,
                structural_snapshot,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "premise", content = "value")]
pub enum A3BaseRulePremise {
    CompleteTwoStepWindow,
    TypedSource(String),
    ExportedPublicClause(String),
    ChronologicalOlderToNewest,
    OlderInterfaceHasKernelTypeType(String),
    OlderInterfaceIsPointwiseTypeValued { anchor: String, domain: KernelTy },
    TypedPathWitness { anchor: String, dimension: u32 },
}

/// Closed evidence for one base A3 rule occurrence.  Unlike structural
/// completion evidence, this token is present at Stage 16 and therefore
/// keeps `C(S15,S14)` available to the downstream D(B15) decision.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3BaseRuleEvidence {
    pub rule_constructor: A3RuleConstructor,
    pub source_anchor_ids: Vec<String>,
    pub premises: Vec<A3BaseRulePremise>,
    pub evidence_hash: String,
}

fn base_rule_evidence(
    rule_constructor: A3RuleConstructor,
    sources: &[&A3TypedClauseSource],
    extra_premises: Vec<A3BaseRulePremise>,
) -> A3BaseRuleEvidence {
    let source_anchor_ids = sources
        .iter()
        .map(|source| source.anchor_id.clone())
        .collect::<Vec<_>>();
    let mut premises = vec![A3BaseRulePremise::CompleteTwoStepWindow];
    for source in sources {
        premises.push(A3BaseRulePremise::TypedSource(
            source.typing_derivation_hash.clone(),
        ));
        premises.push(A3BaseRulePremise::ExportedPublicClause(
            source.public_eligibility_hash.clone(),
        ));
    }
    premises.extend(extra_premises);
    let evidence_hash = tagged_hash(
        "base-rule-evidence",
        &(rule_constructor, &source_anchor_ids, &premises),
    );
    A3BaseRuleEvidence {
        rule_constructor,
        source_anchor_ids,
        premises,
        evidence_hash,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "origin")]
pub enum A3DemandSchemeOrigin {
    BaseRule {
        evidence: A3BaseRuleEvidence,
    },
    StructuralCompletion {
        constructor: A3DemandConstructor,
        constructor_evidence_hash: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(
    rename_all = "snake_case",
    tag = "origin_kind",
    content = "constructor"
)]
enum A3DemandSchemeOriginKey {
    BaseRule,
    StructuralCompletion(A3DemandConstructor),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3TypedDemandScheme {
    pub scheme_id: String,
    pub origin: A3DemandSchemeOrigin,
    pub rule_constructor: A3RuleConstructor,
    pub support_depth: u8,
    pub support_steps: Vec<u32>,
    pub parameter_family_keys: Vec<String>,
    pub parameter_kernel_types: Vec<KernelTy>,
    pub required_output: A3DemandOutputType,
    pub formation_derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3TypedDemandInstance {
    pub instance_id: String,
    pub scheme_id: String,
    pub source_anchor_ids: Vec<String>,
    pub source_family_keys: Vec<String>,
    pub identity_or_uniform_specialization: bool,
    pub independently_exported_demand: bool,
    pub origin_evidence_hash: String,
    pub typing_derivation_hash: String,
}

fn make_scheme_and_instance(
    origin: A3DemandSchemeOrigin,
    rule_constructor: A3RuleConstructor,
    sources: &[&A3TypedClauseSource],
    required_output: A3DemandOutputType,
) -> (A3TypedDemandScheme, A3TypedDemandInstance) {
    let frozen_output_identity = required_output.frozen_identity_key();
    let source_anchor_ids = sources
        .iter()
        .map(|source| source.anchor_id.clone())
        .collect::<Vec<_>>();
    let source_family_keys = sources
        .iter()
        .map(|source| source.canonical_family_key.clone())
        .collect::<Vec<_>>();
    let parameter_kernel_types = sources
        .iter()
        .map(|source| source.kernel_type.clone())
        .collect::<Vec<_>>();
    let support_steps = sources
        .iter()
        .map(|source| source.step)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let support_depth = u8::try_from(support_steps.len()).expect("two-step support fits u8");
    let independently_exported_demand =
        matches!(&origin, A3DemandSchemeOrigin::StructuralCompletion { .. });
    let (origin_key, origin_evidence_hash) = match &origin {
        A3DemandSchemeOrigin::BaseRule { evidence } => (
            A3DemandSchemeOriginKey::BaseRule,
            evidence.evidence_hash.clone(),
        ),
        A3DemandSchemeOrigin::StructuralCompletion {
            constructor,
            constructor_evidence_hash,
        } => (
            A3DemandSchemeOriginKey::StructuralCompletion(*constructor),
            constructor_evidence_hash.clone(),
        ),
    };
    let formation_derivation_hash = tagged_hash(
        "typed-scheme-formation",
        &(
            &origin,
            rule_constructor,
            support_depth,
            &source_family_keys,
            &parameter_kernel_types,
            &frozen_output_identity,
        ),
    );
    // Locations are deliberately absent from the scheme id.  Equal typed
    // family parameters therefore share one natural scheme; their clause
    // occurrences remain separate typed instances below.
    let scheme_id = tagged_hash(
        "natural-demand-scheme",
        &(
            origin_key,
            rule_constructor,
            &source_family_keys,
            &parameter_kernel_types,
            &frozen_output_identity,
        ),
    );
    let typing_derivation_hash = tagged_hash(
        "typed-demand-instance",
        &(
            &scheme_id,
            &source_anchor_ids,
            sources
                .iter()
                .map(|source| source.typing_derivation_hash.as_str())
                .collect::<Vec<_>>(),
            &formation_derivation_hash,
        ),
    );
    let instance_id = tagged_hash(
        "demand-instance-id",
        &(&scheme_id, &source_anchor_ids, &typing_derivation_hash),
    );
    (
        A3TypedDemandScheme {
            scheme_id: scheme_id.clone(),
            origin,
            rule_constructor,
            support_depth,
            support_steps,
            parameter_family_keys: source_family_keys.clone(),
            parameter_kernel_types,
            required_output,
            formation_derivation_hash,
        },
        A3TypedDemandInstance {
            instance_id,
            scheme_id,
            source_anchor_ids,
            source_family_keys,
            identity_or_uniform_specialization: true,
            // Base occurrences are uniform members of one natural scheme;
            // independent export lives at the quotient orbit.  A structural
            // completion hole is itself the one explicitly exported demand.
            independently_exported_demand,
            origin_evidence_hash,
            typing_derivation_hash,
        },
    )
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3SourceEqualityEvidence {
    pub representative_anchor: String,
    pub member_anchor: String,
    pub canonical_family_key: String,
    pub frozen_equality: EqualityWitness,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3InstanceOrbitEquality {
    pub member_instance_id: String,
    pub source_equalities: Vec<A3SourceEqualityEvidence>,
    pub same_typed_scheme_output: bool,
    pub equality_derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3DemandOrbit {
    pub orbit_id: String,
    pub scheme_id: String,
    pub representative_instance_id: String,
    pub member_instance_ids: Vec<String>,
    pub equality_procedure: String,
    pub quotient_equalities: Vec<A3InstanceOrbitEquality>,
    pub uniform_specializations_collapsed: bool,
    pub independently_exported_demand_orbit: bool,
    pub orbit_derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum A3SeedRejectionReason {
    IncompleteTwoStepWindow,
    OlderInterfaceNotTypeValuedOrNotPublic,
    NoTypedPathWitness,
}

/// Accounting for promotion of Agent-D-style finite rule seeds into this
/// module's typed instances.  Counts are computed from the raw typed source
/// inventory; none is an expected historical total.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3RuleSeedDisposition {
    pub rule_constructor: A3RuleConstructor,
    pub raw_seed_count: usize,
    pub promoted_seed_count: usize,
    pub rejected_seed_count: usize,
    pub generated_typed_instance_count: usize,
    pub rejection_reason: Option<A3SeedRejectionReason>,
    pub eligibility_evidence_hash: String,
    pub derivation_hash: String,
}

fn seed_disposition(
    rule_constructor: A3RuleConstructor,
    raw_seed_count: usize,
    promoted_seed_count: usize,
    generated_typed_instance_count: usize,
    rejection_reason: Option<A3SeedRejectionReason>,
    eligibility_evidence_hash: String,
) -> A3RuleSeedDisposition {
    let rejected_seed_count = raw_seed_count.saturating_sub(promoted_seed_count);
    let derivation_hash = tagged_hash(
        "rule-seed-disposition",
        &(
            rule_constructor,
            raw_seed_count,
            promoted_seed_count,
            rejected_seed_count,
            generated_typed_instance_count,
            rejection_reason,
            &eligibility_evidence_hash,
        ),
    );
    A3RuleSeedDisposition {
        rule_constructor,
        raw_seed_count,
        promoted_seed_count,
        rejected_seed_count,
        generated_typed_instance_count,
        rejection_reason,
        eligibility_evidence_hash,
        derivation_hash,
    }
}

fn quotient_orbits(
    schemes: &[A3TypedDemandScheme],
    instances: &[A3TypedDemandInstance],
    sources: &[A3TypedClauseSource],
) -> Result<Vec<A3DemandOrbit>, A3DemandGrammarError> {
    let source_by_id = sources
        .iter()
        .map(|source| (source.anchor_id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let mut instances_by_scheme: BTreeMap<&str, Vec<&A3TypedDemandInstance>> = BTreeMap::new();
    for instance in instances {
        instances_by_scheme
            .entry(&instance.scheme_id)
            .or_default()
            .push(instance);
    }

    schemes
        .iter()
        .map(|scheme| {
            let members = instances_by_scheme
                .get(scheme.scheme_id.as_str())
                .ok_or_else(|| A3DemandGrammarError::OrphanScheme(scheme.scheme_id.clone()))?;
            let representative = members[0];
            let representative_sources = representative
                .source_anchor_ids
                .iter()
                .map(|anchor| {
                    source_by_id
                        .get(anchor.as_str())
                        .copied()
                        .ok_or_else(|| A3DemandGrammarError::MissingSourceAnchor(anchor.clone()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let mut quotient_equalities = Vec::with_capacity(members.len());
            for member in members {
                if member.source_anchor_ids.len() != representative_sources.len() {
                    return Err(A3DemandGrammarError::OrbitArityMismatch {
                        scheme_id: scheme.scheme_id.clone(),
                    });
                }
                let member_sources = member
                    .source_anchor_ids
                    .iter()
                    .map(|anchor| {
                        source_by_id.get(anchor.as_str()).copied().ok_or_else(|| {
                            A3DemandGrammarError::MissingSourceAnchor(anchor.clone())
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let mut source_equalities = Vec::with_capacity(member_sources.len());
                for (left, right) in representative_sources.iter().zip(member_sources) {
                    if left.canonical_family_key != right.canonical_family_key
                        || left.canonical_presentation.parameters
                            != right.canonical_presentation.parameters
                    {
                        return Err(A3DemandGrammarError::OrbitFamilyMismatch {
                            scheme_id: scheme.scheme_id.clone(),
                        });
                    }
                    let scope = u32::try_from(left.canonical_presentation.parameters.len())
                        .expect("parameter arity fits u32");
                    let frozen_equality = univalent_equality(
                        &left.canonical_presentation.canonical_normal_form,
                        &right.canonical_presentation.canonical_normal_form,
                        scope,
                        256,
                    )
                    .map_err(|error| A3DemandGrammarError::OrbitEquality {
                        scheme_id: scheme.scheme_id.clone(),
                        reason: error.to_string(),
                    })?;
                    if !frozen_equality.equal {
                        return Err(A3DemandGrammarError::OrbitFamilyMismatch {
                            scheme_id: scheme.scheme_id.clone(),
                        });
                    }
                    source_equalities.push(A3SourceEqualityEvidence {
                        representative_anchor: left.anchor_id.clone(),
                        member_anchor: right.anchor_id.clone(),
                        canonical_family_key: left.canonical_family_key.clone(),
                        frozen_equality,
                    });
                }
                let equality_derivation_hash = tagged_hash(
                    "instance-orbit-equality",
                    &(
                        &representative.instance_id,
                        &member.instance_id,
                        &source_equalities,
                        &scheme.required_output,
                    ),
                );
                quotient_equalities.push(A3InstanceOrbitEquality {
                    member_instance_id: member.instance_id.clone(),
                    source_equalities,
                    same_typed_scheme_output: true,
                    equality_derivation_hash,
                });
            }
            let member_instance_ids = members
                .iter()
                .map(|member| member.instance_id.clone())
                .collect::<Vec<_>>();
            // Export provenance belongs to the demand orbit only when at
            // least one member is an independently exported demand.  Base
            // rule occurrences are uniform specializations and must not be
            // upgraded merely because the quotient exists.
            let independently_exported_demand_orbit = members
                .iter()
                .any(|member| member.independently_exported_demand);
            let orbit_id = tagged_hash(
                "demand-orbit",
                &(
                    &scheme.scheme_id,
                    KERNEL_EQUALITY_PROCEDURE,
                    &quotient_equalities,
                ),
            );
            let orbit_derivation_hash = tagged_hash(
                "demand-orbit-derivation",
                &(
                    &orbit_id,
                    &member_instance_ids,
                    &quotient_equalities,
                    independently_exported_demand_orbit,
                ),
            );
            Ok(A3DemandOrbit {
                orbit_id,
                scheme_id: scheme.scheme_id.clone(),
                representative_instance_id: representative.instance_id.clone(),
                member_instance_ids,
                equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
                quotient_equalities,
                uniform_specializations_collapsed: true,
                independently_exported_demand_orbit,
                orbit_derivation_hash,
            })
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3StructuralBootstrapWitness {
    pub importing_step: u32,
    pub imported_steps: Vec<u32>,
    pub importing_candidate_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "jurisdiction")]
pub enum A3StructuralJurisdiction {
    PreStructural {
        visible_prefix_steps: u32,
        no_typed_cross_entry_import_yet: bool,
    },
    Structural {
        bootstrap: A3StructuralBootstrapWitness,
    },
}

impl A3StructuralJurisdiction {
    pub fn is_structural(&self) -> bool {
        matches!(self, Self::Structural { .. })
    }
}

fn structural_jurisdiction(
    signature: &SealedSignature,
    visible_prefix_steps: u32,
) -> A3StructuralJurisdiction {
    let bootstrap = signature
        .entries()
        .iter()
        .filter(|entry| entry.step <= visible_prefix_steps)
        .find_map(|entry| {
            let imported_steps = entry
                .direct_imports
                .iter()
                .copied()
                .filter(|import| *import < entry.step)
                .collect::<Vec<_>>();
            if imported_steps.is_empty() {
                None
            } else {
                let derivation_hash = tagged_hash(
                    "structural-bootstrap",
                    &(
                        entry.step,
                        &imported_steps,
                        &entry.candidate_hash,
                        signature.digest(),
                    ),
                );
                Some(A3StructuralBootstrapWitness {
                    importing_step: entry.step,
                    imported_steps,
                    importing_candidate_hash: entry.candidate_hash.clone(),
                    derivation_hash,
                })
            }
        });
    match bootstrap {
        Some(bootstrap) => A3StructuralJurisdiction::Structural { bootstrap },
        None => A3StructuralJurisdiction::PreStructural {
            visible_prefix_steps,
            no_typed_cross_entry_import_yet: true,
        },
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3CompletionOrbitLink {
    pub constructor: A3DemandConstructor,
    pub constructor_evidence_hash: String,
    pub scheme_id: String,
    pub orbit_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3FocusProjection {
    pub jurisdiction: A3StructuralJurisdiction,
    pub constructor_evidence_hashes: Vec<String>,
    pub completion_orbits: Vec<A3CompletionOrbitLink>,
    pub projected_focus: Option<A3DemandConstructor>,
    pub projected_focus_constructors: Vec<A3DemandConstructor>,
    pub demand_precedes_jurisdiction: bool,
    pub derivation_hash: String,
}

fn focus_projection(
    jurisdiction: A3StructuralJurisdiction,
    evidence: &[A3ConstructorEvidence],
    completion_orbits: Vec<A3CompletionOrbitLink>,
) -> Result<A3FocusProjection, A3DemandGrammarError> {
    let constructor_evidence_hashes = evidence
        .iter()
        .map(|item| item.evidence_hash.clone())
        .collect::<Vec<_>>();
    let linked_evidence = completion_orbits
        .iter()
        .map(|link| (link.constructor, link.constructor_evidence_hash.as_str()))
        .collect::<BTreeSet<_>>();
    let expected_evidence = evidence
        .iter()
        .map(|item| (item.constructor, item.evidence_hash.as_str()))
        .collect::<BTreeSet<_>>();
    if linked_evidence != expected_evidence {
        return Err(A3DemandGrammarError::CompletionOrbitMismatch);
    }
    // Once the evidence/orbit bijection is checked, focus is projected from
    // the constructors of the actual completion-hole demand orbits.
    let constructors = completion_orbits
        .iter()
        .map(|link| link.constructor)
        .collect::<BTreeSet<_>>();
    if constructors.len() > 1 {
        return Err(A3DemandGrammarError::AmbiguousFocus(
            constructors.into_iter().collect(),
        ));
    }
    let demanded = constructors.iter().next().copied();
    let projected_focus = jurisdiction.is_structural().then_some(demanded).flatten();
    let projected_focus_constructors = projected_focus.into_iter().collect::<Vec<_>>();
    let demand_precedes_jurisdiction = demanded.is_some() && !jurisdiction.is_structural();
    let derivation_hash = tagged_hash(
        "focus-projection",
        &(
            &jurisdiction,
            &constructor_evidence_hashes,
            &completion_orbits,
            projected_focus,
            &projected_focus_constructors,
            demand_precedes_jurisdiction,
        ),
    );
    Ok(A3FocusProjection {
        jurisdiction,
        constructor_evidence_hashes,
        completion_orbits,
        projected_focus,
        projected_focus_constructors,
        demand_precedes_jurisdiction,
        derivation_hash,
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3HistoricalWindow {
    pub stage: u32,
    pub newest_step: Option<u32>,
    pub older_step: Option<u32>,
    pub orientation: String,
    pub structural_snapshot: A3StructuralSnapshot,
    pub typed_sources: Vec<A3TypedClauseSource>,
    pub constructor_evidence: Vec<A3ConstructorEvidence>,
    pub base_rule_evidence: Vec<A3BaseRuleEvidence>,
    pub seed_dispositions: Vec<A3RuleSeedDisposition>,
    pub schemes: Vec<A3TypedDemandScheme>,
    pub instances: Vec<A3TypedDemandInstance>,
    pub orbits: Vec<A3DemandOrbit>,
    pub focus_projection: A3FocusProjection,
    pub finite_by_construction: bool,
    /// Every occurrence is formed from typed public source clauses.  This is
    /// deliberately weaker than constructing and type-checking the demanded
    /// output term; the latter remains false on the generator boundary.
    pub every_instance_typed: bool,
    pub window_derivation_hash: String,
}

fn generate_window(
    signature: &SealedSignature,
    library: &Library,
    stage: u32,
) -> Result<A3HistoricalWindow, A3DemandGrammarError> {
    let visible_prefix_steps = stage.saturating_sub(1);
    let newest_step = (visible_prefix_steps >= 1).then_some(visible_prefix_steps);
    let older_step = if visible_prefix_steps >= 2 {
        Some(visible_prefix_steps - 1)
    } else {
        None
    };
    let visible_library = library
        [..usize::try_from(visible_prefix_steps).expect("historical stage fits usize")]
        .to_vec();
    let debt = summarize_structural_debt(&visible_library, A3_WINDOW_DEPTH);
    let structural_snapshot = A3StructuralSnapshot::from(debt);
    let constructor_evidence = constructor_evidence(debt)?;

    let mut typed_sources = Vec::new();
    if let Some(step) = older_step {
        typed_sources.extend(typed_sources_for_step(
            signature,
            step,
            A3WindowLayer::Older,
        )?);
    }
    if let Some(step) = newest_step {
        typed_sources.extend(typed_sources_for_step(
            signature,
            step,
            A3WindowLayer::Newest,
        )?);
    }
    let older_sources = typed_sources
        .iter()
        .filter(|source| source.layer == A3WindowLayer::Older)
        .collect::<Vec<_>>();
    let newest_sources = typed_sources
        .iter()
        .filter(|source| source.layer == A3WindowLayer::Newest)
        .collect::<Vec<_>>();
    let path_sources = typed_sources
        .iter()
        .filter(|source| {
            source.exported_public_clause && matches!(source.kernel_type, KernelTy::PathDecl { .. })
        })
        .collect::<Vec<_>>();
    let complete_two_step_window = older_step.is_some() && newest_step.is_some();

    let mut scheme_by_id = BTreeMap::<String, A3TypedDemandScheme>::new();
    let mut instances = Vec::new();
    let mut base_rule_evidence_tokens = Vec::new();

    // Base A3 rules are generated for every complete typed window.  They do
    // not depend on structural completion/focus evidence; in particular,
    // Stage 16 retains its full base inventory for the later D(B15) audit.
    if complete_two_step_window {
        for source in &typed_sources {
            let evidence = base_rule_evidence(A3RuleConstructor::UnaryAction, &[source], vec![]);
            let output = A3DemandOutputType::ActionAt {
                source_family: source.canonical_family_key.clone(),
                source_type: source.kernel_type.clone(),
            };
            let (scheme, instance) = make_scheme_and_instance(
                A3DemandSchemeOrigin::BaseRule {
                    evidence: evidence.clone(),
                },
                A3RuleConstructor::UnaryAction,
                &[source],
                output,
            );
            scheme_by_id
                .entry(scheme.scheme_id.clone())
                .or_insert(scheme);
            instances.push(instance);
            base_rule_evidence_tokens.push(evidence);
        }
        for older in &older_sources {
            for newest in &newest_sources {
                // A chronological specialization consumes an exported
                // interface family from the older layer.  Direct `Type`
                // interfaces and pointwise `Fun(_, Type)` interfaces are
                // both operationally typed; no count decides the split.
                let Some(interface_mode) = chronological_interface_mode(&older.kernel_type) else {
                    continue;
                };
                if !older.exported_public_clause || !newest.exported_public_clause {
                    continue;
                }
                let interface_premise = match &interface_mode {
                    A3ChronologicalInterfaceMode::DirectType => {
                        A3BaseRulePremise::OlderInterfaceHasKernelTypeType(older.anchor_id.clone())
                    }
                    A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { domain } => {
                        A3BaseRulePremise::OlderInterfaceIsPointwiseTypeValued {
                            anchor: older.anchor_id.clone(),
                            domain: domain.clone(),
                        }
                    }
                };
                let evidence = base_rule_evidence(
                    A3RuleConstructor::ChronologicalComparison,
                    &[*older, *newest],
                    vec![
                        A3BaseRulePremise::ChronologicalOlderToNewest,
                        interface_premise,
                    ],
                );
                let output = A3DemandOutputType::ChronologicalInteraction {
                    older_family: older.canonical_family_key.clone(),
                    older_type: older.kernel_type.clone(),
                    newest_family: newest.canonical_family_key.clone(),
                    newest_type: newest.kernel_type.clone(),
                    interface_mode,
                    interface_slot_map: declare_chronological_interface_slot_map(
                        u32::try_from(newest.canonical_presentation.parameters.len())
                            .expect("chronological parameter arity fits u32"),
                    ),
                };
                let (scheme, instance) = make_scheme_and_instance(
                    A3DemandSchemeOrigin::BaseRule {
                        evidence: evidence.clone(),
                    },
                    A3RuleConstructor::ChronologicalComparison,
                    &[*older, *newest],
                    output,
                );
                scheme_by_id
                    .entry(scheme.scheme_id.clone())
                    .or_insert(scheme);
                instances.push(instance);
                base_rule_evidence_tokens.push(evidence);
            }
        }
        if !path_sources.is_empty() {
            let dimension = path_sources
                .iter()
                .filter_map(|source| match source.kernel_type {
                    KernelTy::PathDecl { dimension } => Some(dimension),
                    _ => None,
                })
                .max()
                .expect("non-empty path sources have a dimension");
            let all_sources = typed_sources.iter().collect::<Vec<_>>();
            let evidence = base_rule_evidence(
                A3RuleConstructor::HigherOpenBoxReduction,
                &all_sources,
                path_sources
                    .iter()
                    .filter_map(|source| match source.kernel_type {
                        KernelTy::PathDecl { dimension } => {
                            Some(A3BaseRulePremise::TypedPathWitness {
                                anchor: source.anchor_id.clone(),
                                dimension,
                            })
                        }
                        _ => None,
                    })
                    .collect(),
            );
            let output = A3DemandOutputType::ContractibleOpenBox {
                dimension,
                boundary_families: typed_sources
                    .iter()
                    .map(|source| source.canonical_family_key.clone())
                    .collect(),
                boundary_types: typed_sources
                    .iter()
                    .map(|source| source.kernel_type.clone())
                    .collect(),
                path_witness_families: path_sources
                    .iter()
                    .map(|source| source.canonical_family_key.clone())
                    .collect(),
            };
            let (scheme, instance) = make_scheme_and_instance(
                A3DemandSchemeOrigin::BaseRule {
                    evidence: evidence.clone(),
                },
                A3RuleConstructor::HigherOpenBoxReduction,
                &all_sources,
                output,
            );
            scheme_by_id
                .entry(scheme.scheme_id.clone())
                .or_insert(scheme);
            instances.push(instance);
            base_rule_evidence_tokens.push(evidence);
        }
    }

    // Structural constructors contribute one actual completion-hole demand
    // orbit each.  Focus is projected from these orbits below; it is not a
    // parallel label attached to the otherwise independent base grammar.
    for evidence in &constructor_evidence {
        let all_sources = typed_sources.iter().collect::<Vec<_>>();
        let output = A3DemandOutputType::StructuralCompletion {
            constructor: evidence.constructor,
            structural_snapshot,
        };
        let (scheme, instance) = make_scheme_and_instance(
            A3DemandSchemeOrigin::StructuralCompletion {
                constructor: evidence.constructor,
                constructor_evidence_hash: evidence.evidence_hash.clone(),
            },
            A3RuleConstructor::StructuralCompletionHole,
            &all_sources,
            output,
        );
        scheme_by_id
            .entry(scheme.scheme_id.clone())
            .or_insert(scheme);
        instances.push(instance);
    }
    let schemes = scheme_by_id.into_values().collect::<Vec<_>>();
    let unary_raw_seed_count = if complete_two_step_window {
        typed_sources.len()
    } else {
        0
    };
    let binary_raw_seed_count = if complete_two_step_window {
        older_sources.len().saturating_mul(newest_sources.len())
    } else {
        0
    };
    let higher_raw_seed_count = usize::from(complete_two_step_window);
    let generated_for = |rule| {
        instances
            .iter()
            .filter(|instance| {
                schemes.iter().any(|scheme| {
                    scheme.scheme_id == instance.scheme_id && scheme.rule_constructor == rule
                })
            })
            .count()
    };
    let unary_eligibility_evidence_hash = tagged_hash(
        "unary-seed-eligibility",
        &(
            complete_two_step_window,
            typed_sources
                .iter()
                .map(|source| {
                    (
                        source.anchor_id.as_str(),
                        source.exported_public_clause,
                        source.typing_derivation_hash.as_str(),
                    )
                })
                .collect::<Vec<_>>(),
        ),
    );
    let binary_eligibility_evidence_hash = tagged_hash(
        "binary-seed-eligibility",
        &older_sources
            .iter()
            .flat_map(|older| {
                newest_sources.iter().map(move |newest| {
                    (
                        older.anchor_id.as_str(),
                        newest.anchor_id.as_str(),
                        older.exported_public_clause,
                        newest.exported_public_clause,
                        chronological_interface_mode(&older.kernel_type),
                    )
                })
            })
            .collect::<Vec<_>>(),
    );
    let higher_eligibility_evidence_hash = tagged_hash(
        "higher-seed-eligibility",
        &(
            complete_two_step_window,
            typed_sources
                .iter()
                .map(|source| {
                    (
                        source.anchor_id.as_str(),
                        source.exported_public_clause,
                        &source.kernel_type,
                    )
                })
                .collect::<Vec<_>>(),
            path_sources
                .iter()
                .map(|source| source.anchor_id.as_str())
                .collect::<Vec<_>>(),
        ),
    );
    let completion_eligibility_evidence_hash = tagged_hash(
        "completion-seed-eligibility",
        &constructor_evidence
            .iter()
            .map(|evidence| evidence.evidence_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let seed_dispositions = vec![
        seed_disposition(
            A3RuleConstructor::UnaryAction,
            unary_raw_seed_count,
            unary_raw_seed_count,
            generated_for(A3RuleConstructor::UnaryAction),
            None,
            unary_eligibility_evidence_hash,
        ),
        seed_disposition(
            A3RuleConstructor::ChronologicalComparison,
            binary_raw_seed_count,
            generated_for(A3RuleConstructor::ChronologicalComparison),
            generated_for(A3RuleConstructor::ChronologicalComparison),
            (generated_for(A3RuleConstructor::ChronologicalComparison) < binary_raw_seed_count)
                .then_some(A3SeedRejectionReason::OlderInterfaceNotTypeValuedOrNotPublic),
            binary_eligibility_evidence_hash,
        ),
        seed_disposition(
            A3RuleConstructor::HigherOpenBoxReduction,
            higher_raw_seed_count,
            usize::from(complete_two_step_window && !path_sources.is_empty()),
            generated_for(A3RuleConstructor::HigherOpenBoxReduction),
            (complete_two_step_window && path_sources.is_empty())
                .then_some(A3SeedRejectionReason::NoTypedPathWitness),
            higher_eligibility_evidence_hash,
        ),
        seed_disposition(
            A3RuleConstructor::StructuralCompletionHole,
            constructor_evidence.len(),
            constructor_evidence.len(),
            generated_for(A3RuleConstructor::StructuralCompletionHole),
            None,
            completion_eligibility_evidence_hash,
        ),
    ];
    let orbits = quotient_orbits(&schemes, &instances, &typed_sources)?;
    let completion_orbits = schemes
        .iter()
        .filter_map(|scheme| match &scheme.origin {
            A3DemandSchemeOrigin::StructuralCompletion {
                constructor,
                constructor_evidence_hash,
            } => orbits
                .iter()
                .find(|orbit| orbit.scheme_id == scheme.scheme_id)
                .map(|orbit| A3CompletionOrbitLink {
                    constructor: *constructor,
                    constructor_evidence_hash: constructor_evidence_hash.clone(),
                    scheme_id: scheme.scheme_id.clone(),
                    orbit_id: orbit.orbit_id.clone(),
                }),
            A3DemandSchemeOrigin::BaseRule { .. } => None,
        })
        .collect::<Vec<_>>();
    let focus_projection = focus_projection(
        structural_jurisdiction(signature, visible_prefix_steps),
        &constructor_evidence,
        completion_orbits,
    )?;
    let every_instance_typed = instances.iter().all(|instance| {
        !instance.typing_derivation_hash.is_empty()
            && !instance.source_anchor_ids.is_empty()
            && instance.identity_or_uniform_specialization
            && !instance.origin_evidence_hash.is_empty()
    });
    let window_derivation_hash = tagged_hash(
        "historical-window",
        &(
            stage,
            newest_step,
            older_step,
            structural_snapshot,
            &typed_sources,
            &constructor_evidence,
            &base_rule_evidence_tokens,
            &seed_dispositions,
            &schemes,
            &instances,
            &orbits,
            &focus_projection,
            every_instance_typed,
        ),
    );
    Ok(A3HistoricalWindow {
        stage,
        newest_step,
        older_step,
        orientation: "W=(newest_step,older_step)".to_owned(),
        structural_snapshot,
        typed_sources,
        constructor_evidence,
        base_rule_evidence: base_rule_evidence_tokens,
        seed_dispositions,
        schemes,
        instances,
        orbits,
        focus_projection,
        finite_by_construction: true,
        every_instance_typed,
        window_derivation_hash,
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3GeneratorBoundary {
    pub demand_constructors: Vec<A3DemandConstructor>,
    pub rule_constructors: Vec<A3RuleConstructor>,
    pub all_historical_windows_covered: bool,
    pub raw_typed_window_is_authoritative: bool,
    pub structural_obligation_predicates_are_authoritative: bool,
    pub caller_inventory_is_not_a_generator_input: bool,
    pub score_bar_expected_count_and_next_winner_are_not_inputs: bool,
    pub equality_orbit_quotient_hook_installed: bool,
    pub focus_projection_uses_constructor_evidence: bool,
    pub completion_predicates_replayed_from_raw_snapshot_without_coarse_labels: bool,
    pub demand_output_terms_constructed: bool,
    pub demand_output_terms_kernel_typed: bool,
    pub rule_constructor_inventory_exhaustiveness_proved: bool,
    pub d_membership_decided: bool,
    pub f1_executed: bool,
    pub theorem12_issued: bool,
    pub bridge_authorized: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3HistoricalDemandGrammar {
    pub schema: String,
    pub signature_digest: String,
    pub windows: Vec<A3HistoricalWindow>,
    pub boundary: A3GeneratorBoundary,
    pub derivation_hash: String,
}

/// Deliberately untrusted negative-control input.  Neither field is read by
/// the issuer: accepting the probe at this API boundary demonstrates that
/// an empty or forged caller inventory cannot suppress or manufacture A3
/// constructor evidence, schemes, instances, or focus.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UntrustedCallerDemandInventory {
    pub stage: u32,
    pub claimed_focus_labels: Vec<String>,
    pub claimed_demand_ids: Vec<String>,
}

fn validate_historical_signature(signature: &SealedSignature) -> Result<(), A3DemandGrammarError> {
    let found = signature
        .entries()
        .iter()
        .map(|entry| entry.step)
        .collect::<Vec<_>>();
    let expected = (1..A3_LAST_STAGE).collect::<Vec<_>>();
    if found != expected {
        return Err(A3DemandGrammarError::NonHistoricalSignature { expected, found });
    }
    Ok(())
}

pub fn issue_historical_a3_demand_grammar(
    signature: &SealedSignature,
) -> Result<A3HistoricalDemandGrammar, A3DemandGrammarError> {
    validate_historical_signature(signature)?;
    let mut library = Library::new();
    for step in 1..A3_LAST_STAGE {
        let entry = signature
            .entry(step)
            .ok_or(A3DemandGrammarError::MissingHistoricalStep(step))?;
        library.push(LibraryEntry::from_telescope(&entry.telescope, &library));
    }
    let windows = (A3_FIRST_STAGE..=A3_LAST_STAGE)
        .map(|stage| generate_window(signature, &library, stage))
        .collect::<Result<Vec<_>, _>>()?;
    let boundary = A3GeneratorBoundary {
        demand_constructors: A3DemandConstructor::ALL.to_vec(),
        rule_constructors: A3RuleConstructor::ALL.to_vec(),
        all_historical_windows_covered: windows.len()
            == usize::try_from(A3_LAST_STAGE - A3_FIRST_STAGE + 1)
                .expect("historical span fits usize")
            && windows
                .iter()
                .map(|window| window.stage)
                .eq(A3_FIRST_STAGE..=A3_LAST_STAGE),
        raw_typed_window_is_authoritative: true,
        structural_obligation_predicates_are_authoritative: true,
        caller_inventory_is_not_a_generator_input: true,
        score_bar_expected_count_and_next_winner_are_not_inputs: true,
        equality_orbit_quotient_hook_installed: true,
        focus_projection_uses_constructor_evidence: true,
        completion_predicates_replayed_from_raw_snapshot_without_coarse_labels: true,
        demand_output_terms_constructed: false,
        demand_output_terms_kernel_typed: false,
        rule_constructor_inventory_exhaustiveness_proved: false,
        d_membership_decided: false,
        f1_executed: false,
        theorem12_issued: false,
        bridge_authorized: false,
    };
    let derivation_hash = tagged_hash(
        "historical-grammar",
        &(signature.digest(), &windows, &boundary),
    );
    Ok(A3HistoricalDemandGrammar {
        schema: A3_HISTORICAL_GRAMMAR_SCHEMA.to_owned(),
        signature_digest: signature.digest().to_owned(),
        windows,
        boundary,
        derivation_hash,
    })
}

/// Deterministic integration surface for the complete two-entry historical
/// domain.  Stages 1 and 2 have no complete two-entry support window, so this
/// convenience API returns exactly stages 3 through 16.
pub fn generate_historical_a3_windows() -> Result<Vec<A3HistoricalWindow>, A3DemandGrammarError> {
    let signature = SealedSignature::genesis_del_h15();
    Ok(issue_historical_a3_demand_grammar(&signature)?
        .windows
        .into_iter()
        .filter(|window| window.stage >= 3)
        .collect())
}

/// Generate one A3 window from an arbitrary exact sealed prefix.
///
/// This is the branch-safe integration surface used by the adopted R-T2
/// protocol.  The caller supplies no focus label, candidate score, expected
/// count, or successor answer: the prefix must contain exactly the sealed
/// entries `1..stage`, and the ordinary generator above computes the window
/// from those entries and their raw structural summary.
pub fn generate_a3_window_for_prefix(
    signature: &SealedSignature,
    stage: u32,
) -> Result<A3HistoricalWindow, A3DemandGrammarError> {
    if !(A3_FIRST_STAGE..=A3_LAST_STAGE).contains(&stage) {
        return Err(A3DemandGrammarError::StageOutsideHistoricalDomain(stage));
    }
    generate_a3_window_for_exact_prefix_unbounded(signature, stage)
}

/// Generate one A3 window from an arbitrary exact sealed prefix without the
/// archival `1..=16` convenience-domain cap.
///
/// This is the law-independent continuation surface.  It changes neither the
/// A3 constructors nor their typing rules: it merely permits the same local
/// two-entry computation at a later exact prefix.  Callers remain responsible
/// for imposing a separately recorded resource bound on an open-ended search.
pub fn generate_a3_window_for_exact_prefix_unbounded(
    signature: &SealedSignature,
    stage: u32,
) -> Result<A3HistoricalWindow, A3DemandGrammarError> {
    if stage < A3_FIRST_STAGE {
        return Err(A3DemandGrammarError::StageOutsideHistoricalDomain(stage));
    }
    let visible_prefix_steps = stage.saturating_sub(1);
    let found = signature
        .entries()
        .iter()
        .map(|entry| entry.step)
        .collect::<Vec<_>>();
    let expected = (1..=visible_prefix_steps).collect::<Vec<_>>();
    if found != expected {
        return Err(A3DemandGrammarError::NonExactWindowPrefix {
            stage,
            expected,
            found,
        });
    }
    let mut library = Library::new();
    for step in 1..=visible_prefix_steps {
        let entry = signature
            .entry(step)
            .ok_or(A3DemandGrammarError::MissingHistoricalStep(step))?;
        library.push(LibraryEntry::from_telescope(&entry.telescope, &library));
    }
    generate_window(signature, &library, stage)
}

/// Negative-control entry point.  The untrusted inventory is intentionally
/// not forwarded to the issuer and cannot influence even an output digest.
pub fn issue_historical_a3_demand_grammar_with_untrusted_inventory(
    signature: &SealedSignature,
    _untrusted_inventory: &[UntrustedCallerDemandInventory],
) -> Result<A3HistoricalDemandGrammar, A3DemandGrammarError> {
    issue_historical_a3_demand_grammar(signature)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct A3CallerInventoryNoninterference {
    pub baseline_derivation_hash: String,
    pub empty_inventory_derivation_hash: String,
    pub forged_inventory_derivation_hash: String,
    pub empty_inventory_equal: bool,
    pub forged_inventory_equal: bool,
    pub derivation_hash: String,
}

/// Public negative-control equality: both the empty inventory and an
/// arbitrary forged inventory must return the byte-for-byte same grammar as
/// the inventory-free issuer.
pub fn audit_a3_caller_inventory_noninterference(
    signature: &SealedSignature,
    forged_inventory: &[UntrustedCallerDemandInventory],
) -> Result<A3CallerInventoryNoninterference, A3DemandGrammarError> {
    let baseline = issue_historical_a3_demand_grammar(signature)?;
    let empty = issue_historical_a3_demand_grammar_with_untrusted_inventory(signature, &[])?;
    let forged =
        issue_historical_a3_demand_grammar_with_untrusted_inventory(signature, forged_inventory)?;
    let empty_inventory_equal = empty == baseline;
    let forged_inventory_equal = forged == baseline;
    let derivation_hash = tagged_hash(
        "caller-inventory-noninterference",
        &(
            &baseline.derivation_hash,
            &empty.derivation_hash,
            &forged.derivation_hash,
            empty_inventory_equal,
            forged_inventory_equal,
        ),
    );
    Ok(A3CallerInventoryNoninterference {
        baseline_derivation_hash: baseline.derivation_hash,
        empty_inventory_derivation_hash: empty.derivation_hash,
        forged_inventory_derivation_hash: forged.derivation_hash,
        empty_inventory_equal,
        forged_inventory_equal,
        derivation_hash,
    })
}

pub fn replay_historical_a3_demand_grammar(
    signature: &SealedSignature,
    certificate: &A3HistoricalDemandGrammar,
) -> Result<(), A3DemandGrammarError> {
    let replay = issue_historical_a3_demand_grammar(signature)?;
    if &replay != certificate {
        return Err(A3DemandGrammarError::ReplayMismatch);
    }
    Ok(())
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum A3DemandGrammarError {
    #[error("stage {0} is outside the supported A3 historical domain")]
    StageOutsideHistoricalDomain(u32),
    #[error("stage {stage} requires the exact sealed prefix {expected:?}, found {found:?}")]
    NonExactWindowPrefix {
        stage: u32,
        expected: Vec<u32>,
        found: Vec<u32>,
    },
    #[error("signature is not the complete ordered historical step domain")]
    NonHistoricalSignature { expected: Vec<u32>, found: Vec<u32> },
    #[error("historical signature is missing step {0}")]
    MissingHistoricalStep(u32),
    #[error("historical step {step} failed typed elaboration: {failure}")]
    HistoricalElaboration { step: u32, failure: ClauseFailure },
    #[error("constructor {constructor:?} was selected but premise {premise:?} failed replay")]
    ConstructorPremiseMismatch {
        constructor: A3DemandConstructor,
        premise: A3StructuralPremise,
    },
    #[error("constructor evidence projects more than one focus: {0:?}")]
    AmbiguousFocus(Vec<A3DemandConstructor>),
    #[error("structural constructor evidence does not match completion-hole demand orbits")]
    CompletionOrbitMismatch,
    #[error("scheme {0} has no typed instance")]
    OrphanScheme(String),
    #[error("typed source anchor {0} is absent from its window")]
    MissingSourceAnchor(String),
    #[error("scheme {scheme_id} has inconsistent instance arity")]
    OrbitArityMismatch { scheme_id: String },
    #[error("scheme {scheme_id} attempted to quotient unequal typed source families")]
    OrbitFamilyMismatch { scheme_id: String },
    #[error("scheme {scheme_id} frozen equality failed: {reason}")]
    OrbitEquality { scheme_id: String, reason: String },
    #[error("historical A3 demand grammar replay mismatch")]
    ReplayMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::telescope::Telescope;

    fn genesis() -> A3HistoricalDemandGrammar {
        issue_historical_a3_demand_grammar(&SealedSignature::genesis_del_h15())
            .expect("historical A3 grammar")
    }

    fn window(grammar: &A3HistoricalDemandGrammar, stage: u32) -> &A3HistoricalWindow {
        grammar
            .windows
            .iter()
            .find(|window| window.stage == stage)
            .expect("historical stage")
    }

    #[test]
    fn every_chronological_scheme_carries_the_declared_identity_slot_map() {
        let grammar = genesis();
        let mut chronological_schemes = 0_usize;
        let mut observed_arities = BTreeSet::new();
        for window in &grammar.windows {
            for scheme in &window.schemes {
                let A3DemandOutputType::ChronologicalInteraction {
                    interface_slot_map, ..
                } = &scheme.required_output
                else {
                    continue;
                };
                chronological_schemes += 1;
                let representative = window
                    .instances
                    .iter()
                    .find(|instance| instance.scheme_id == scheme.scheme_id)
                    .expect("chronological scheme has an instance");
                let newest_anchor = representative
                    .source_anchor_ids
                    .get(1)
                    .expect("chronological instance has a newest source");
                let newest = window
                    .typed_sources
                    .iter()
                    .find(|source| &source.anchor_id == newest_anchor)
                    .expect("newest family resolves in its exact window");
                let arity = u32::try_from(newest.canonical_presentation.parameters.len())
                    .expect("parameter arity fits u32");
                replay_chronological_interface_slot_map(interface_slot_map, arity)
                    .expect("registration carries the unique declared identity");
                observed_arities.insert(arity);
            }
        }
        assert!(chronological_schemes > 0);
        assert!(observed_arities.contains(&2));
    }

    #[test]
    fn slot_map_replay_rejects_permutations_inference_overrides_and_charge() {
        let canonical = declare_chronological_interface_slot_map(3);
        replay_chronological_interface_slot_map(&canonical, 3).expect("identity replays");

        let mut permutation = canonical.clone();
        permutation.assignments.swap(0, 1);
        assert_eq!(
            replay_chronological_interface_slot_map(&permutation, 3),
            Err(A3ChronologicalInterfaceSlotMapError::NotOrderPreservingIdentity)
        );

        let mut inferred = canonical.clone();
        inferred.inferred_from_derivation_success = true;
        assert_eq!(
            replay_chronological_interface_slot_map(&inferred, 3),
            Err(A3ChronologicalInterfaceSlotMapError::InferenceOrOverride)
        );

        let mut overridden = canonical.clone();
        overridden.instance_override_permitted = true;
        assert_eq!(
            replay_chronological_interface_slot_map(&overridden, 3),
            Err(A3ChronologicalInterfaceSlotMapError::InferenceOrOverride)
        );

        let mut charged = canonical;
        charged.nu_charge = 1;
        assert_eq!(
            replay_chronological_interface_slot_map(&charged, 3),
            Err(A3ChronologicalInterfaceSlotMapError::NonzeroCharge)
        );
    }

    #[test]
    fn generator_covers_all_windows_with_closed_typed_constructor_inventory() {
        let signature = SealedSignature::genesis_del_h15();
        let grammar = issue_historical_a3_demand_grammar(&signature).unwrap();
        replay_historical_a3_demand_grammar(&signature, &grammar).unwrap();
        assert_eq!(grammar.windows.len(), 16);
        assert!(grammar.boundary.all_historical_windows_covered);
        assert_eq!(
            grammar.boundary.demand_constructors,
            A3DemandConstructor::ALL
        );
        assert_eq!(grammar.boundary.rule_constructors, A3RuleConstructor::ALL);
        assert!(grammar.windows.iter().all(|window| {
            window.finite_by_construction
                && window.every_instance_typed
                && window.orbits.iter().all(|orbit| {
                    orbit.quotient_equalities.iter().all(|member| {
                        member.same_typed_scheme_output
                            && member
                                .source_equalities
                                .iter()
                                .all(|source| source.frozen_equality.equal)
                    })
                })
        }));
        assert!(!grammar.boundary.d_membership_decided);
        assert!(
            grammar
                .boundary
                .completion_predicates_replayed_from_raw_snapshot_without_coarse_labels
        );
        assert!(!grammar.boundary.demand_output_terms_constructed);
        assert!(!grammar.boundary.demand_output_terms_kernel_typed);
        assert!(
            !grammar
                .boundary
                .rule_constructor_inventory_exhaustiveness_proved
        );
        assert!(!grammar.boundary.f1_executed);
        assert!(!grammar.boundary.theorem12_issued);
        assert!(!grammar.boundary.bridge_authorized);
        let integration_windows = generate_historical_a3_windows().unwrap();
        assert_eq!(integration_windows.len(), 14);
        assert!(
            integration_windows
                .iter()
                .map(|window| window.stage)
                .eq(3..=16)
        );
    }

    #[test]
    fn constructor_projection_rederives_the_ladder_and_stage_three_wrinkle() {
        let grammar = genesis();
        let expected = [
            None,
            None,
            Some(A3DemandConstructor::FormerEliminator),
            Some(A3DemandConstructor::FormerEliminator),
            Some(A3DemandConstructor::InitialHit),
            Some(A3DemandConstructor::TruncationHit),
            Some(A3DemandConstructor::HigherHit),
            Some(A3DemandConstructor::SphereLift),
            Some(A3DemandConstructor::AxiomaticBundle),
            Some(A3DemandConstructor::ModalShell),
            Some(A3DemandConstructor::ConnectionShell),
            Some(A3DemandConstructor::CurvatureShell),
            Some(A3DemandConstructor::OperatorBundle),
            Some(A3DemandConstructor::HilbertFunctional),
            Some(A3DemandConstructor::TemporalShell),
            None,
        ];
        for (window, expected_constructor) in grammar.windows.iter().zip(expected) {
            assert_eq!(
                window
                    .constructor_evidence
                    .first()
                    .map(|evidence| evidence.constructor),
                expected_constructor,
                "stage {}",
                window.stage
            );
            assert_eq!(
                window.constructor_evidence.len(),
                usize::from(expected_constructor.is_some()),
                "stage {}",
                window.stage
            );
        }

        let three = window(&grammar, 3);
        assert!(!three.instances.is_empty());
        assert_eq!(three.focus_projection.projected_focus, None);
        assert!(three.focus_projection.demand_precedes_jurisdiction);
        assert!(matches!(
            three.focus_projection.jurisdiction,
            A3StructuralJurisdiction::PreStructural { .. }
        ));

        let four = window(&grammar, 4);
        assert_eq!(
            four.focus_projection.projected_focus,
            Some(A3DemandConstructor::FormerEliminator)
        );
        assert!(!four.focus_projection.demand_precedes_jurisdiction);
        assert!(matches!(
            four.focus_projection.jurisdiction,
            A3StructuralJurisdiction::Structural { .. }
        ));
        assert_eq!(four.focus_projection.completion_orbits.len(), 1);
        let completion_scheme = four
            .schemes
            .iter()
            .find(|scheme| {
                matches!(
                    &scheme.origin,
                    A3DemandSchemeOrigin::StructuralCompletion { .. }
                )
            })
            .expect("completion scheme");
        assert!(
            four.instances
                .iter()
                .filter(|instance| instance.scheme_id == completion_scheme.scheme_id)
                .all(|instance| instance.independently_exported_demand)
        );
        assert!(
            four.instances
                .iter()
                .filter(|instance| instance.scheme_id != completion_scheme.scheme_id)
                .all(|instance| !instance.independently_exported_demand)
        );
    }

    #[test]
    fn empty_and_forged_caller_inventories_are_observationally_irrelevant() {
        let signature = SealedSignature::genesis_del_h15();
        let canonical = issue_historical_a3_demand_grammar(&signature).unwrap();
        let empty =
            issue_historical_a3_demand_grammar_with_untrusted_inventory(&signature, &[]).unwrap();
        let forged = issue_historical_a3_demand_grammar_with_untrusted_inventory(
            &signature,
            &(1..=16)
                .map(|stage| UntrustedCallerDemandInventory {
                    stage,
                    claimed_focus_labels: vec!["manufactured_focus".to_owned()],
                    claimed_demand_ids: vec![format!("forged-{stage}")],
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
        assert_eq!(
            empty, canonical,
            "empty caller data cannot suppress demands"
        );
        assert_eq!(forged, canonical, "forged caller data cannot mint demands");
        assert!(!window(&empty, 3).instances.is_empty());
        assert!(
            !window(&empty, 16).instances.is_empty(),
            "caller emptiness must not vacuously erase Stage-16 base A3"
        );
        let audit = audit_a3_caller_inventory_noninterference(&signature, &[]).unwrap();
        assert!(audit.empty_inventory_equal);
        assert!(audit.forged_inventory_equal);
    }

    #[test]
    fn raw_window_mutation_changes_generation_and_forged_labels_cannot_restore_it() {
        let telescopes = (1..=15)
            .map(|step| {
                let telescope = if step == 2 {
                    Telescope::default()
                } else {
                    Telescope::reference(step)
                };
                (step, telescope)
            })
            .collect();
        let mutated = SealedSignature::from_telescopes(telescopes);
        let forged = [UntrustedCallerDemandInventory {
            stage: 3,
            claimed_focus_labels: vec!["former_eliminator".to_owned()],
            claimed_demand_ids: vec!["manufactured-former-demand".to_owned()],
        }];
        let grammar =
            issue_historical_a3_demand_grammar_with_untrusted_inventory(&mutated, &forged).unwrap();
        assert!(window(&grammar, 3).constructor_evidence.is_empty());
        assert!(
            window(&grammar, 3).schemes.iter().all(|scheme| !matches!(
                scheme.origin,
                A3DemandSchemeOrigin::StructuralCompletion { .. }
            )),
            "forged labels cannot restore a structurally absent completion hole"
        );
        assert!(
            !window(&grammar, 3).instances.is_empty(),
            "base A3 remains derived from whatever typed sources survive"
        );
        assert_eq!(window(&grammar, 3).focus_projection.projected_focus, None);
    }

    #[test]
    fn stage_sixteen_base_inventory_is_nonvacuous_and_count_blindly_filtered() {
        let grammar = genesis();
        let sixteen = window(&grammar, 16);
        let disposition = |rule| {
            sixteen
                .seed_dispositions
                .iter()
                .find(|item| item.rule_constructor == rule)
                .expect("rule disposition")
        };
        let unary = disposition(A3RuleConstructor::UnaryAction);
        let binary = disposition(A3RuleConstructor::ChronologicalComparison);
        let higher = disposition(A3RuleConstructor::HigherOpenBoxReduction);
        assert_eq!((unary.raw_seed_count, unary.promoted_seed_count), (17, 17));
        assert_eq!(
            (binary.raw_seed_count, binary.promoted_seed_count),
            (72, 72)
        );
        assert_eq!((higher.raw_seed_count, higher.promoted_seed_count), (1, 0));
        assert_eq!(
            higher.rejection_reason,
            Some(A3SeedRejectionReason::NoTypedPathWitness)
        );
        assert!(sixteen.constructor_evidence.is_empty());
        assert!(sixteen.focus_projection.completion_orbits.is_empty());
        assert_eq!(sixteen.focus_projection.projected_focus, None);
        assert!(
            sixteen
                .instances
                .iter()
                .all(|instance| !instance.independently_exported_demand)
        );
        assert!(
            sixteen
                .orbits
                .iter()
                .all(|orbit| !orbit.independently_exported_demand_orbit),
            "uniform base-rule orbits cannot mint independent export provenance"
        );
        let mut direct = 0;
        let mut pointwise = 0;
        for instance in &sixteen.instances {
            let scheme = sixteen
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .expect("instance scheme");
            if let A3DemandOutputType::ChronologicalInteraction { interface_mode, .. } =
                &scheme.required_output
            {
                match interface_mode {
                    A3ChronologicalInterfaceMode::DirectType => direct += 1,
                    A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. } => {
                        pointwise += 1
                    }
                }
            }
        }
        assert_eq!((direct, pointwise), (64, 8));
        assert_eq!(sixteen.instances.len(), 89);
    }

    #[test]
    fn arbitrary_prefix_window_replays_the_historical_stage_five_surface() {
        let full = SealedSignature::genesis_del_h15();
        let prefix = SealedSignature::from_telescopes(
            (1..=4)
                .map(|step| {
                    (
                        step,
                        full.entry(step)
                            .expect("historical prefix entry")
                            .telescope
                            .clone(),
                    )
                })
                .collect(),
        );
        let branch_window = generate_a3_window_for_prefix(&prefix, 5).unwrap();
        let historical = issue_historical_a3_demand_grammar(&full).unwrap();
        let historical_window = window(&historical, 5);
        // Scheme/source identifiers bind the signature digest, so a genuine
        // prefix has different identifiers from the fifteen-entry signature.
        // The semantic window surface must nevertheless replay exactly.
        assert_eq!(branch_window.stage, historical_window.stage);
        assert_eq!(branch_window.newest_step, historical_window.newest_step);
        assert_eq!(branch_window.older_step, historical_window.older_step);
        assert_eq!(
            branch_window.structural_snapshot,
            historical_window.structural_snapshot
        );
        assert_eq!(
            branch_window
                .typed_sources
                .iter()
                .map(|source| (
                    source.layer,
                    source.step,
                    source.clause_index,
                    source.kernel_role,
                    &source.kernel_type,
                    &source.normal_form,
                    &source.canonical_presentation,
                ))
                .collect::<Vec<_>>(),
            historical_window
                .typed_sources
                .iter()
                .map(|source| (
                    source.layer,
                    source.step,
                    source.clause_index,
                    source.kernel_role,
                    &source.kernel_type,
                    &source.normal_form,
                    &source.canonical_presentation,
                ))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            branch_window
                .constructor_evidence
                .iter()
                .map(|evidence| (evidence.constructor, &evidence.premises))
                .collect::<Vec<_>>(),
            historical_window
                .constructor_evidence
                .iter()
                .map(|evidence| (evidence.constructor, &evidence.premises))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            branch_window
                .seed_dispositions
                .iter()
                .map(|row| (
                    row.rule_constructor,
                    row.raw_seed_count,
                    row.promoted_seed_count,
                    row.generated_typed_instance_count,
                    row.rejection_reason,
                ))
                .collect::<Vec<_>>(),
            historical_window
                .seed_dispositions
                .iter()
                .map(|row| (
                    row.rule_constructor,
                    row.raw_seed_count,
                    row.promoted_seed_count,
                    row.generated_typed_instance_count,
                    row.rejection_reason,
                ))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            branch_window.focus_projection.projected_focus,
            historical_window.focus_projection.projected_focus
        );

        let non_exact = SealedSignature::from_telescopes(vec![
            (1, Telescope::reference(1)),
            (3, Telescope::reference(3)),
        ]);
        assert!(matches!(
            generate_a3_window_for_prefix(&non_exact, 3),
            Err(A3DemandGrammarError::NonExactWindowPrefix { .. })
        ));
    }

    #[test]
    fn natural_scheme_quotient_collapses_repeated_uniform_presentations() {
        let grammar = genesis();
        let three = window(&grammar, 3);
        assert!(three.orbits.len() <= three.instances.len());
        assert!(
            three
                .orbits
                .iter()
                .any(|orbit| orbit.member_instance_ids.len() > 1),
            "the repeated App(Univ,m) presentation must be one scheme orbit"
        );
        let represented = three
            .orbits
            .iter()
            .map(|orbit| orbit.member_instance_ids.len())
            .sum::<usize>();
        assert_eq!(represented, three.instances.len());
    }

    #[test]
    fn higher_rule_requires_a_typed_path_witness() {
        let grammar = genesis();
        assert!(
            window(&grammar, 5)
                .schemes
                .iter()
                .all(|scheme| scheme.rule_constructor != A3RuleConstructor::HigherOpenBoxReduction)
        );
        assert!(
            window(&grammar, 6)
                .schemes
                .iter()
                .any(|scheme| scheme.rule_constructor == A3RuleConstructor::HigherOpenBoxReduction)
        );
    }
}
