//! Fail-closed typed/provenance layer for the adopted declared-boundary rule.
//!
//! This module is additive.  It does not reinterpret the incumbent V1
//! cubical tokens and it does not promote the structural V2 audit in
//! [`super::boundary_variants`] into a general term elaborator.  Instead it
//! checks the four *registered* historical diagrams in the small closed term
//! language actually used by the adopted axiom: a boundary term is either a
//! reference to a sealed clause of the owning package or a variable in an
//! explicitly typed parameter context.
//!
//! Under the archival V2 issuer, Trunc types and S1/S2/S3 return the original
//! replay-bound C7 obstruction.  The additive adopted V3 issuer applies only
//! the closed three-entry historical element overlay after deriving a
//! name-blind operational-role witness; it can then type those three base
//! bindings without rewriting the shallow record.  HIST-CERT can consume
//! separately source-bound prefix issuers for exact B4/B6/B7 signatures.
//!
//! This module proves neither semantic basis independence/exhaustiveness nor
//! any HIST replay result.  Its C6 bridge only joins already replayed typed
//! boundaries and path realizers for the three registered constant diagrams.

use super::boundary_variants::{
    BoundaryBasisFormula, BoundaryBasisKey, BoundaryFaceKey, BoundaryTermOrigin, BoundaryTheory,
    BoundaryVariantError, DeclaredBoundaryDiagram, boundary_basis_cardinality,
    check_declared_boundary_diagram, constant_boundary, interval_endpoint_boundary,
    issue_boundary_attachment, present_boundary_basis, replay_boundary_attachment,
};
use super::{
    TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION, audit_endpoint_path_computation,
    endpoint_source_digest, issue_endpoint_schema_premise_context, realize_endpoint_path_basis,
    realize_path_basis, replay_endpoint_path_realization, replay_path_realization,
};
use crate::elaborate::{
    KernelTy, SealedSignature, TelescopeElaboration, candidate_hash, elaborate_telescope,
};
use crate::substitution::{
    ParameterSort, SORT_PRESERVATION_SCOPE, SortPreservingSubstitutionToken,
    SortedParameterContext, SubstitutionImage, issue_sort_preserving_substitution,
    replay_sort_preserving_substitution,
};
use crate::tdc1::{FormedPathTyping, PathSchemaKey, elaborate_formed_path};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

/// Implementation version for this additive, registered-diagram checker.
pub const TYPED_BOUNDARY_FRAGMENT_VERSION: &str = "tdc1-typed-declared-boundary-v1";

/// The exact axiom adopted in `docs/boundary_adjudication_proposal.md`.
pub const ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION: &str =
    "pathcon-attachment-declared-bound-boundary-theory-axiom-v2";

/// The adopted successor axiom.  Existing V2 issuers deliberately continue
/// to use [`ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION`], so their evidence and
/// replay hashes are archival.  V3 evidence is available only through the
/// explicitly suffixed issuers in this module.
pub const ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION: &str =
    "pathcon-attachment-declared-bound-boundary-theory-axiom-v3-element-overlay";

/// The exact element overlay carried by the adopted V3 axiom.
pub const HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION: &str =
    "historical-element-declaration-overlay-v1";

/// Additive implementation envelope for V3 tokens.  It is separate from the
/// V1 typed-boundary fragment so V2 token digests cannot drift.
pub const TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION: &str =
    "tdc1-typed-declared-boundary-element-overlay-v1";

/// Narrow bridge joining the V3 prefix-bound boundary witness to the already
/// typed path-basis realizers.  This version makes no independence,
/// exhaustiveness, generic-C6, or semantic-classification claim.
pub const HISTORICAL_PREFIX_V3_C6_BRIDGE_VERSION: &str =
    "tdc1-historical-prefix-v3-typed-boundary-path-basis-bridge-v1";

pub const C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP: &str =
    "C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER";

/// Successor evidence domain that retires the named Trunc computation gap
/// without changing the archival schema-5 bridge API that records it.
pub const TRUNC_ENDPOINT_V3_C6_BUNDLE_VERSION: &str = "tdc1-trunc-endpoint-v3-c6-bundle-v1";

/// Uniform successor handoff over B4/B5/B6/B7.  Constant packages wrap the
/// archival C6 bridge; Trunc wraps the new endpoint-dependent bundle.
pub const HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION: &str =
    "tdc1-historical-prefix-v3-c6-typed-handoff-v1";

pub const C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION_GAP: &str =
    "C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION";

/// The exact charging convention adopted with the boundary axiom.
pub const BOUNDARY_CHARGE_POLICY_VERSION: &str = "boundary-charge-zero-reference-only-v1";

/// Adopted wrapper joining the finite path-basis formula to the zero-charge
/// reference-only provenance token. The older structural V2 audit token keeps
/// its proposal-era version for archival compatibility and is not this proof.
pub const ADOPTED_BASIS_CHARGE_BINDING_VERSION: &str =
    "adopted-declared-boundary-basis-charge-binding-v1";

/// Stable name for the registered interpretation of the four historical
/// constructors.  This is deliberately distinct from the axiom version.
pub const REGISTERED_HISTORICAL_DIAGRAMS_VERSION: &str =
    "pathcon-attachment-registered-historical-diagrams-v2";

/// Error text required when purported boundary data is not reference-only.
pub const REFERENCE_ONLY_HOISTING_RULE: &str = "a boundary component not expressible as a sealed package clause or declared context variable must be hoisted to its own charged telescope clause";

/// Named obstruction emitted instead of forging the missing historical
/// point-clause typing judgement.
pub const HISTORICAL_POINT_TYPING_OBSTRUCTION_ID: &str =
    "C-7-historical-point-clause-not-typed-as-owner-element-v1";

const TRUNC_PARAMETER_CONTEXT_LABEL: &str = "A:Type; x,y:Trunc(A)";
const HISTORICAL_FORMATION_CLAUSE: u16 = 0;
const HISTORICAL_POINT_CLAUSE: u16 = 1;
const HISTORICAL_PATH_CLAUSE: u16 = 2;

fn tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(
        TYPED_BOUNDARY_FRAGMENT_VERSION,
        ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION,
        BOUNDARY_CHARGE_POLICY_VERSION,
        domain,
        payload,
    ))
    .expect("typed-boundary proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn element_overlay_tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(
        TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
        ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION,
        HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION,
        BOUNDARY_CHARGE_POLICY_VERSION,
        domain,
        payload,
    ))
    .expect("element-overlay proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn trunc_endpoint_tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(
        TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION,
        TRUNC_ENDPOINT_V3_C6_BUNDLE_VERSION,
        ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION,
        HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION,
        BOUNDARY_CHARGE_POLICY_VERSION,
        domain,
        payload,
    ))
    .expect("Trunc endpoint proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

/// The four diagrams registered verbatim by the adopted V2 interpretation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegisteredBoundaryKind {
    S1,
    Trunc,
    S2,
    S3,
}

impl RegisteredBoundaryKind {
    pub const ALL: [Self; 4] = [Self::S1, Self::Trunc, Self::S2, Self::S3];

    pub const fn step(self) -> u32 {
        match self {
            Self::S1 => 5,
            Self::Trunc => 6,
            Self::S2 => 7,
            Self::S3 => 8,
        }
    }

    pub const fn dimension(self) -> u32 {
        match self {
            Self::S1 | Self::Trunc => 1,
            Self::S2 => 2,
            Self::S3 => 3,
        }
    }

    pub const fn needs_historical_base_binding(self) -> bool {
        matches!(self, Self::S1 | Self::S2 | Self::S3)
    }

    pub const fn constructor_name(self) -> &'static str {
        match self {
            Self::S1 => "loop",
            Self::Trunc => "squash",
            Self::S2 => "surface2",
            Self::S3 => "surface3",
        }
    }
}

/// One entry in the closed adopted element overlay.  Fields are private so
/// callers cannot extend the registry by constructing a look-alike entry.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HistoricalElementOverlayEntry {
    kind: RegisteredBoundaryKind,
    step: u32,
    formation_clause: u16,
    element_clause: u16,
}

impl HistoricalElementOverlayEntry {
    pub const fn kind(self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn step(self) -> u32 {
        self.step
    }

    pub const fn formation_clause(self) -> u16 {
        self.formation_clause
    }

    pub const fn element_clause(self) -> u16 {
        self.element_clause
    }
}

const REGISTERED_HISTORICAL_ELEMENT_OVERLAY_ENTRIES: [HistoricalElementOverlayEntry; 3] = [
    HistoricalElementOverlayEntry {
        kind: RegisteredBoundaryKind::S1,
        step: 5,
        formation_clause: HISTORICAL_FORMATION_CLAUSE,
        element_clause: HISTORICAL_POINT_CLAUSE,
    },
    HistoricalElementOverlayEntry {
        kind: RegisteredBoundaryKind::S2,
        step: 7,
        formation_clause: HISTORICAL_FORMATION_CLAUSE,
        element_clause: HISTORICAL_POINT_CLAUSE,
    },
    HistoricalElementOverlayEntry {
        kind: RegisteredBoundaryKind::S3,
        step: 8,
        formation_clause: HISTORICAL_FORMATION_CLAUSE,
        element_clause: HISTORICAL_POINT_CLAUSE,
    },
];

/// Return the closed V1 overlay registry: exactly Steps 5, 7, and 8, clause
/// 1.  Trunc is intentionally absent.
pub const fn registered_historical_element_overlay_entries() -> [HistoricalElementOverlayEntry; 3] {
    REGISTERED_HISTORICAL_ELEMENT_OVERLAY_ENTRIES
}

fn registered_element_overlay_entry(
    kind: RegisteredBoundaryKind,
) -> Result<HistoricalElementOverlayEntry, TypedBoundaryError> {
    REGISTERED_HISTORICAL_ELEMENT_OVERLAY_ENTRIES
        .iter()
        .copied()
        .find(|entry| entry.kind == kind)
        .ok_or(TypedBoundaryError::HistoricalElementOverlayNotRegistered { kind })
}

/// Sorts needed by the registered parameter contexts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BoundaryParameterType {
    Type,
    Element { owner: Expr },
}

/// One binding in an explicitly typed boundary parameter context.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedParameterBinding {
    index: u16,
    name: String,
    ty: BoundaryParameterType,
}

impl TypedParameterBinding {
    pub const fn index(&self) -> u16 {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &BoundaryParameterType {
        &self.ty
    }
}

/// An ordinary typed context, kept separate from cubical interval variables.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedParameterContext {
    owner: Expr,
    bindings: Vec<TypedParameterBinding>,
    context_digest: String,
}

impl TypedParameterContext {
    pub fn owner(&self) -> &Expr {
        &self.owner
    }

    pub fn bindings(&self) -> &[TypedParameterBinding] {
        &self.bindings
    }

    pub fn context_digest(&self) -> &str {
        &self.context_digest
    }

    fn lookup(&self, name: &str) -> Option<&TypedParameterBinding> {
        self.bindings.iter().find(|binding| binding.name == name)
    }
}

/// A resolved reference to a clause of the exact sealed owning package.
/// Fields are private so callers cannot manufacture a reference without
/// passing the issuer's signature/package checks.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedPackageClauseRef {
    signature_digest: String,
    step: u32,
    candidate_hash: String,
    clause: u16,
    term: Expr,
}

impl SealedPackageClauseRef {
    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }

    pub const fn step(&self) -> u32 {
        self.step
    }

    pub fn candidate_hash(&self) -> &str {
        &self.candidate_hash
    }

    pub const fn clause(&self) -> u16 {
        self.clause
    }

    pub fn term(&self) -> &Expr {
        &self.term
    }
}

/// A resolved lookup in the declared typed parameter context.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextVariableRef {
    context_digest: String,
    index: u16,
    name: String,
    ty: BoundaryParameterType,
}

impl ContextVariableRef {
    pub fn context_digest(&self) -> &str {
        &self.context_digest
    }

    pub const fn index(&self) -> u16 {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &BoundaryParameterType {
        &self.ty
    }
}

/// The entire adopted reference-only term language.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ResolvedBoundaryRef {
    SealedPackageClause(SealedPackageClauseRef),
    ContextVariable(ContextVariableRef),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResolvedBoundaryRestriction {
    face: BoundaryFaceKey,
    term: ResolvedBoundaryRef,
}

impl ResolvedBoundaryRestriction {
    pub const fn face(&self) -> BoundaryFaceKey {
        self.face
    }

    pub fn term(&self) -> &ResolvedBoundaryRef {
        &self.term
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResolvedBoundaryFace {
    fixed: BoundaryFaceKey,
    term: ResolvedBoundaryRef,
    remaining_axes: Vec<u32>,
    restrictions: Vec<ResolvedBoundaryRestriction>,
}

impl ResolvedBoundaryFace {
    pub const fn fixed(&self) -> BoundaryFaceKey {
        self.fixed
    }

    pub fn term(&self) -> &ResolvedBoundaryRef {
        &self.term
    }

    pub fn remaining_axes(&self) -> &[u32] {
        &self.remaining_axes
    }

    pub fn restrictions(&self) -> &[ResolvedBoundaryRestriction] {
        &self.restrictions
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceResolutionFailure {
    ArbitraryDeclaredFaceFamily,
    UnboundContextVariable,
    MissingSealedPackageClause,
    UnboundRestrictionTerm,
    AmbiguousBoundaryName,
}

/// Replay-bound account of the kernel judgement missing at C-7.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalPointTypingObstruction {
    obstruction_id: String,
    axiom_version: String,
    signature_digest: String,
    kind: RegisteredBoundaryKind,
    step: u32,
    candidate_hash: String,
    formation_clause: u16,
    point_clause: u16,
    path_clause: u16,
    point_term: Expr,
    owner: Expr,
    expected_type: KernelTy,
    actual_type: KernelTy,
    elaboration_derivation_hash: String,
    derivation_hash: String,
}

impl HistoricalPointTypingObstruction {
    pub fn obstruction_id(&self) -> &str {
        &self.obstruction_id
    }

    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn step(&self) -> u32 {
        self.step
    }

    pub fn expected_type(&self) -> &KernelTy {
        &self.expected_type
    }

    pub fn actual_type(&self) -> &KernelTy {
        &self.actual_type
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Machine-readable F-O1 causes.  None of these variants refers to a
/// constructor label or boundary-term name: eligibility is positional and
/// operational only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalRoleFailure {
    EntryDoesNotMatchSealedPackage,
    RequiredClauseAbsent,
    ElementDoesNotImmediatelyFollowFormation,
    FormationRoleMismatch,
    ElementRoleMismatch,
    PathRoleMismatch,
    PathConstructorMismatch,
    RegisteredFaceInventoryMismatch,
    RegisteredBoundaryDoesNotBindElement,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum TypedBoundaryError {
    #[error("the registered boundary checker requires the exact sealed Genesis signature")]
    SignatureNotSealedGenesis,
    #[error("registered historical package step {step} is absent")]
    MissingRegisteredStep { step: u32 },
    #[error("registered historical package step {step} differs from its sealed reference")]
    RegisteredSourceMismatch { step: u32 },
    #[error("V3 historical predecessor signature is not the exact sealed B_{predecessor}")]
    HistoricalPrefixMismatch { predecessor: u32 },
    #[error("V3 historical package step {step} is not its exact sealed telescope")]
    HistoricalCurrentTelescopeMismatch { step: u32 },
    #[error("V4 prefix-general S3 boundary requires an exact contiguous seven-entry prefix")]
    PrefixGeneralS3PrefixShapeMismatch,
    #[error("V4 prefix-general predecessor step {step} did not elaborate: {error}")]
    PrefixGeneralPredecessorElaboration { step: u32, error: String },
    #[error("registered historical source failed formed-path elaboration: {error}")]
    FormedPathElaboration { error: String },
    #[error("registered source has the wrong formation/path clauses or dimension")]
    RegisteredPathShapeMismatch,
    #[error("legacy structural boundary validation failed: {error}")]
    StructuralBoundary { error: String },
    #[error("the submitted diagram is not the exact registered {kind:?} interpretation")]
    RegisteredDiagramMismatch { kind: RegisteredBoundaryKind },
    #[error(
        "boundary term `{term}` at face {face:?} is not reference-only ({reason:?}); policy `{charging_policy}` requires: {hoisting_rule}"
    )]
    HoistingRequired {
        face: BoundaryFaceKey,
        term: String,
        reason: ReferenceResolutionFailure,
        charging_policy: String,
        hoisting_rule: String,
    },
    #[error("historical base binding is not applicable to the Trunc endpoint diagram")]
    HistoricalBaseBindingNotApplicable,
    #[error("the adopted element overlay has no entry for {kind:?}")]
    HistoricalElementOverlayNotRegistered { kind: RegisteredBoundaryKind },
    #[error("F-O1 operational-role witness failed for {kind:?}: {reason:?}")]
    OperationalRoleWitnessFailed {
        kind: RegisteredBoundaryKind,
        reason: OperationalRoleFailure,
    },
    #[error("F-O1 operational-role witness replay mismatch")]
    OperationalRoleReplayMismatch,
    #[error("historical element-overlay token replay mismatch")]
    HistoricalElementOverlayReplayMismatch,
    #[error("F-O2 element-overlay reading does not type the registered base binding")]
    HistoricalElementOverlayTypeMismatch,
    #[error("historical point typing is obstructed: {obstruction:?}")]
    HistoricalPointTypingObstructed {
        obstruction: Box<HistoricalPointTypingObstruction>,
    },
    #[error("boundary term at face {face:?} does not inhabit the formed owner")]
    BoundaryTermTypeMismatch { face: BoundaryFaceKey },
    #[error("typed boundary faces disagree on an overlap")]
    TypedOverlapMismatch,
    #[error("reference-only charge token replay mismatch")]
    ReferenceOnlyReplayMismatch,
    #[error("adopted boundary basis/charge token replay mismatch")]
    AdoptedBasisChargeReplayMismatch,
    #[error("historical base-binding token replay mismatch")]
    HistoricalBaseBindingReplayMismatch,
    #[error("V3 historical base-binding token replay mismatch")]
    HistoricalBaseBindingV3ReplayMismatch,
    #[error("typed-boundary token replay mismatch")]
    TypedBoundaryReplayMismatch,
    #[error("V3 typed-boundary token replay mismatch")]
    TypedBoundaryV3ReplayMismatch,
    #[error(
        "C-6 endpoint-dependent boundary gap `{gap_id}`: Trunc is outside the historical constant-boundary bridge"
    )]
    C6EndpointDependentBoundaryGap { gap_id: String },
    #[error("C-6 historical path-basis realization failed: {error}")]
    C6PathBasisRealization { error: String },
    #[error("C-6 historical path-basis count is not exactly 1 + d^2")]
    C6PathBasisCountMismatch,
    #[error("C-6 boundary-basis presentation and typed path keys are not in exact bijection")]
    C6BoundaryBasisBijectionMismatch,
    #[error("C-6 historical prefix boundary/basis bridge replay mismatch")]
    C6HistoricalPrefixBridgeReplayMismatch,
    #[error("registered Trunc endpoint face/context inventory mismatch")]
    TruncEndpointInventoryMismatch,
    #[error("restricted C-1 Trunc parameter instantiation failed: {error}")]
    TruncRestrictedInstantiation { error: String },
    #[error("Trunc endpoint-dependent C6 bundle replay mismatch")]
    TruncEndpointBundleReplayMismatch,
    #[error("successor historical typed-bundle handoff replay mismatch")]
    HistoricalTypedHandoffReplayMismatch,
}

impl From<BoundaryVariantError> for TypedBoundaryError {
    fn from(error: BoundaryVariantError) -> Self {
        Self::StructuralBoundary {
            error: error.to_string(),
        }
    }
}

struct RegisteredSource {
    kind: RegisteredBoundaryKind,
    candidate_hash: String,
    telescope: Telescope,
    typing: FormedPathTyping,
    elaboration: TelescopeElaboration,
}

fn registered_source(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<RegisteredSource, TypedBoundaryError> {
    let sealed = SealedSignature::genesis_del_h15();
    if signature.digest() != sealed.digest() {
        return Err(TypedBoundaryError::SignatureNotSealedGenesis);
    }

    let step = kind.step();
    let entry = signature
        .entry(step)
        .ok_or(TypedBoundaryError::MissingRegisteredStep { step })?;
    let telescope = Telescope::reference(step);
    if entry.telescope != telescope {
        return Err(TypedBoundaryError::RegisteredSourceMismatch { step });
    }
    let (typing, elaboration) =
        elaborate_formed_path(signature, &telescope, step - 1).map_err(|error| {
            TypedBoundaryError::FormedPathElaboration {
                error: error.to_string(),
            }
        })?;
    if typing.formation_clause != HISTORICAL_FORMATION_CLAUSE
        || typing.path_clause != HISTORICAL_PATH_CLAUSE
        || typing.dimension != kind.dimension()
    {
        return Err(TypedBoundaryError::RegisteredPathShapeMismatch);
    }
    Ok(RegisteredSource {
        kind,
        candidate_hash: entry.candidate_hash.clone(),
        telescope,
        typing,
        elaboration,
    })
}

fn registered_source_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
) -> Result<RegisteredSource, TypedBoundaryError> {
    // This source path covers all four registered V2 diagrams. Overlay and
    // base-binding issuers perform the separate closed-entry check, leaving
    // Trunc available only as the no-overlay typed-context control.
    let step = kind.step();
    let expected_prefix = SealedSignature::from_telescopes(
        (1..step)
            .map(|step| (step, Telescope::reference(step)))
            .collect(),
    );
    if predecessor_signature != &expected_prefix {
        return Err(TypedBoundaryError::HistoricalPrefixMismatch {
            predecessor: step - 1,
        });
    }
    let sealed_current = Telescope::reference(step);
    if current_telescope != &sealed_current {
        return Err(TypedBoundaryError::HistoricalCurrentTelescopeMismatch { step });
    }

    let sealed = SealedSignature::genesis_del_h15();
    let candidate_hash = sealed
        .entry(step)
        .ok_or(TypedBoundaryError::MissingRegisteredStep { step })?
        .candidate_hash
        .clone();
    let (typing, elaboration) =
        elaborate_formed_path(predecessor_signature, current_telescope, step - 1).map_err(
            |error| TypedBoundaryError::FormedPathElaboration {
                error: error.to_string(),
            },
        )?;
    if typing.formation_clause != HISTORICAL_FORMATION_CLAUSE
        || typing.path_clause != HISTORICAL_PATH_CLAUSE
        || typing.dimension != kind.dimension()
    {
        return Err(TypedBoundaryError::RegisteredPathShapeMismatch);
    }
    Ok(RegisteredSource {
        kind,
        candidate_hash,
        telescope: sealed_current,
        typing,
        elaboration,
    })
}

/// Prefix-parametric source reconstruction for the registered S3 boundary.
///
/// Unlike [`registered_source_for_historical_prefix`], this theorem does not
/// compare the predecessor against the historical B7 telescope sequence.  It
/// re-elaborates each of the seven supplied predecessor acts against only its
/// own earlier prefix, then elaborates the registered five-clause S3 package
/// over that exact prefix.  The local S3 interpretation is therefore licensed
/// by typing and source shape, not by enacted-prefix identity.
fn registered_s3_source_for_prefix_general_v4(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
) -> Result<RegisteredSource, TypedBoundaryError> {
    const STEP: u32 = 8;
    if predecessor_signature.entries().len() != (STEP - 1) as usize
        || predecessor_signature
            .entries()
            .iter()
            .map(|entry| entry.step)
            .ne(1..STEP)
    {
        return Err(TypedBoundaryError::PrefixGeneralS3PrefixShapeMismatch);
    }

    let mut checked_prefix = Vec::<(u32, Telescope)>::new();
    for entry in predecessor_signature.entries() {
        let earlier = SealedSignature::from_telescopes(checked_prefix.clone());
        elaborate_telescope(&earlier, &entry.telescope, entry.step.saturating_sub(1)).map_err(
            |error| TypedBoundaryError::PrefixGeneralPredecessorElaboration {
                step: entry.step,
                error: error.to_string(),
            },
        )?;
        checked_prefix.push((entry.step, entry.telescope.clone()));
    }
    if SealedSignature::from_telescopes(checked_prefix).digest() != predecessor_signature.digest() {
        return Err(TypedBoundaryError::PrefixGeneralS3PrefixShapeMismatch);
    }

    let sealed_current = Telescope::reference(STEP);
    if current_telescope != &sealed_current {
        return Err(TypedBoundaryError::HistoricalCurrentTelescopeMismatch { step: STEP });
    }
    let (typing, elaboration) =
        elaborate_formed_path(predecessor_signature, current_telescope, STEP - 1).map_err(
            |error| TypedBoundaryError::FormedPathElaboration {
                error: error.to_string(),
            },
        )?;
    let kind = RegisteredBoundaryKind::S3;
    if typing.formation_clause != HISTORICAL_FORMATION_CLAUSE
        || typing.path_clause != HISTORICAL_PATH_CLAUSE
        || typing.dimension != kind.dimension()
    {
        return Err(TypedBoundaryError::RegisteredPathShapeMismatch);
    }
    Ok(RegisteredSource {
        kind,
        candidate_hash: candidate_hash(current_telescope),
        telescope: current_telescope.clone(),
        typing,
        elaboration,
    })
}

fn parameter_context_for_source(source: &RegisteredSource) -> TypedParameterContext {
    let owner = source.typing.formation_normal_form.clone();
    let bindings = match source.kind {
        RegisteredBoundaryKind::Trunc => vec![
            TypedParameterBinding {
                index: 0,
                name: "A".to_owned(),
                ty: BoundaryParameterType::Type,
            },
            TypedParameterBinding {
                index: 1,
                name: "x".to_owned(),
                ty: BoundaryParameterType::Element {
                    owner: owner.clone(),
                },
            },
            TypedParameterBinding {
                index: 2,
                name: "y".to_owned(),
                ty: BoundaryParameterType::Element {
                    owner: owner.clone(),
                },
            },
        ],
        RegisteredBoundaryKind::S1 | RegisteredBoundaryKind::S2 | RegisteredBoundaryKind::S3 => {
            Vec::new()
        }
    };
    let context_digest = tagged_digest("typed-parameter-context", &(&owner, &bindings));
    TypedParameterContext {
        owner,
        bindings,
        context_digest,
    }
}

fn canonical_diagram_for_source(
    source: &RegisteredSource,
) -> Result<DeclaredBoundaryDiagram, TypedBoundaryError> {
    let owner = source.typing.formation_normal_form.clone();
    match source.kind {
        RegisteredBoundaryKind::Trunc => Ok(interval_endpoint_boundary(
            owner,
            "x",
            "y",
            TRUNC_PARAMETER_CONTEXT_LABEL,
        )),
        RegisteredBoundaryKind::S1 | RegisteredBoundaryKind::S2 | RegisteredBoundaryKind::S3 => {
            constant_boundary(
                owner,
                source.kind.dimension(),
                "base",
                HISTORICAL_POINT_CLAUSE,
            )
            .map_err(Into::into)
        }
    }
}

/// Return the exact registered raw diagram.  It is still raw: callers must
/// obtain the reference-only and typed tokens separately.
pub fn registered_boundary_diagram(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<DeclaredBoundaryDiagram, TypedBoundaryError> {
    canonical_diagram_for_source(&registered_source(signature, kind)?)
}

pub fn registered_boundary_diagram_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
) -> Result<DeclaredBoundaryDiagram, TypedBoundaryError> {
    canonical_diagram_for_source(&registered_source_for_historical_prefix(
        predecessor_signature,
        kind,
        current_telescope,
    )?)
}

/// Build the registered S3 diagram from an arbitrary typed seven-act prefix.
/// No historical prefix or branch identity is accepted as an input.
pub fn registered_s3_boundary_diagram_for_prefix_general_v4(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
) -> Result<DeclaredBoundaryDiagram, TypedBoundaryError> {
    canonical_diagram_for_source(&registered_s3_source_for_prefix_general_v4(
        predecessor_signature,
        current_telescope,
    )?)
}

/// Return the typed ordinary parameter context registered for a diagram.
pub fn registered_parameter_context(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<TypedParameterContext, TypedBoundaryError> {
    Ok(parameter_context_for_source(&registered_source(
        signature, kind,
    )?))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct OperationalFaceBinding {
    fixed: BoundaryFaceKey,
    element_clause: u16,
    remaining_axes: Vec<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct OperationalRoleProjection {
    formation_role: ClauseRole,
    element_role: ClauseRole,
    path_role: ClauseRole,
    path_dimension: u32,
    face_bindings: Vec<OperationalFaceBinding>,
}

fn operational_role_projection(
    source: &RegisteredSource,
    entry: HistoricalElementOverlayEntry,
    map: &DeclaredBoundaryDiagram,
) -> Result<OperationalRoleProjection, TypedBoundaryError> {
    let fail = |reason| TypedBoundaryError::OperationalRoleWitnessFailed {
        kind: source.kind,
        reason,
    };
    if entry.kind != source.kind
        || entry.step != source.kind.step()
        || entry.formation_clause != HISTORICAL_FORMATION_CLAUSE
        || entry.element_clause != HISTORICAL_POINT_CLAUSE
    {
        return Err(fail(OperationalRoleFailure::EntryDoesNotMatchSealedPackage));
    }
    if entry.element_clause != entry.formation_clause + 1 {
        return Err(fail(
            OperationalRoleFailure::ElementDoesNotImmediatelyFollowFormation,
        ));
    }
    let Some(formation) = source
        .telescope
        .clauses
        .get(usize::from(entry.formation_clause))
    else {
        return Err(fail(OperationalRoleFailure::RequiredClauseAbsent));
    };
    let Some(element) = source
        .telescope
        .clauses
        .get(usize::from(entry.element_clause))
    else {
        return Err(fail(OperationalRoleFailure::RequiredClauseAbsent));
    };
    let Some(path) = source
        .telescope
        .clauses
        .get(usize::from(HISTORICAL_PATH_CLAUSE))
    else {
        return Err(fail(OperationalRoleFailure::RequiredClauseAbsent));
    };
    if formation.role != ClauseRole::Formation {
        return Err(fail(OperationalRoleFailure::FormationRoleMismatch));
    }
    if element.role != ClauseRole::Introduction {
        return Err(fail(OperationalRoleFailure::ElementRoleMismatch));
    }
    if path.role != ClauseRole::PathAttach {
        return Err(fail(OperationalRoleFailure::PathRoleMismatch));
    }
    if path.expr != Expr::PathCon(source.kind.dimension()) {
        return Err(fail(OperationalRoleFailure::PathConstructorMismatch));
    }

    let expected_faces = (0..source.kind.dimension())
        .flat_map(|axis| {
            [
                BoundaryFaceKey::new(axis, false),
                BoundaryFaceKey::new(axis, true),
            ]
        })
        .collect::<BTreeSet<_>>();
    let actual_faces = map
        .faces
        .iter()
        .map(|face| face.fixed)
        .collect::<BTreeSet<_>>();
    if map.dimension != source.kind.dimension()
        || map.faces.len() != expected_faces.len()
        || actual_faces != expected_faces
    {
        return Err(fail(
            OperationalRoleFailure::RegisteredFaceInventoryMismatch,
        ));
    }

    let mut face_bindings = Vec::with_capacity(map.faces.len());
    for face in &map.faces {
        let BoundaryTermOrigin::HistoricalPointClause { clause } = face.term.origin else {
            return Err(fail(
                OperationalRoleFailure::RegisteredBoundaryDoesNotBindElement,
            ));
        };
        if clause != entry.element_clause {
            return Err(fail(
                OperationalRoleFailure::RegisteredBoundaryDoesNotBindElement,
            ));
        }
        face_bindings.push(OperationalFaceBinding {
            fixed: face.fixed,
            element_clause: clause,
            remaining_axes: face.term.remaining_axes.clone(),
        });
    }
    face_bindings.sort_by_key(|binding| binding.fixed);
    Ok(OperationalRoleProjection {
        formation_role: formation.role,
        element_role: element.role,
        path_role: path.role,
        path_dimension: source.kind.dimension(),
        face_bindings,
    })
}

/// Opaque F-O1 witness that an overlay entry is justified by the sealed
/// telescope's operational structure and the registered boundary's clause
/// references.  No term label, constructor name, or annotation name occurs
/// in the witness projection.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationalRoleWitness {
    fragment_version: String,
    axiom_version: String,
    overlay_version: String,
    signature_digest: String,
    candidate_hash: String,
    kind: RegisteredBoundaryKind,
    step: u32,
    formation_clause: u16,
    element_clause: u16,
    path_clause: u16,
    sealed_telescope_digest: String,
    projection: OperationalRoleProjection,
    projection_digest: String,
    derivation_hash: String,
}

impl OperationalRoleWitness {
    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn step(&self) -> u32 {
        self.step
    }

    pub const fn formation_clause(&self) -> u16 {
        self.formation_clause
    }

    pub const fn element_clause(&self) -> u16 {
        self.element_clause
    }

    pub const fn path_clause(&self) -> u16 {
        self.path_clause
    }

    pub fn overlay_version(&self) -> &str {
        &self.overlay_version
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}

pub fn issue_operational_role_witness_v3(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<OperationalRoleWitness, TypedBoundaryError> {
    let source = registered_source(signature, kind)?;
    issue_operational_role_witness_v3_from_source(signature.digest(), &source)
}

fn issue_operational_role_witness_v3_from_source(
    signature_digest: &str,
    source: &RegisteredSource,
) -> Result<OperationalRoleWitness, TypedBoundaryError> {
    let kind = source.kind;
    let entry = registered_element_overlay_entry(kind)?;
    let map = canonical_diagram_for_source(&source)?;
    let projection = operational_role_projection(source, entry, &map)?;
    let fragment_version = TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION.to_owned();
    let axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned();
    let overlay_version = HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION.to_owned();
    let signature_digest = signature_digest.to_owned();
    let candidate_hash = source.candidate_hash.clone();
    let sealed_telescope_digest =
        element_overlay_tagged_digest("sealed-telescope", &source.telescope);
    let projection_digest =
        element_overlay_tagged_digest("operational-role-projection", &projection);
    let derivation_hash = element_overlay_tagged_digest(
        "operational-role-witness",
        &(
            &fragment_version,
            &axiom_version,
            &overlay_version,
            &signature_digest,
            &candidate_hash,
            kind,
            entry.step,
            entry.formation_clause,
            entry.element_clause,
            HISTORICAL_PATH_CLAUSE,
            &sealed_telescope_digest,
            &projection,
            &projection_digest,
        ),
    );
    Ok(OperationalRoleWitness {
        fragment_version,
        axiom_version,
        overlay_version,
        signature_digest,
        candidate_hash,
        kind,
        step: entry.step,
        formation_clause: entry.formation_clause,
        element_clause: entry.element_clause,
        path_clause: HISTORICAL_PATH_CLAUSE,
        sealed_telescope_digest,
        projection,
        projection_digest,
        derivation_hash,
    })
}

/// Prefix-bound F-O1 issuer for HIST-CERT.  `predecessor_signature` must be
/// exactly B_(step-1), and `current_telescope` must be the exact sealed Step
/// 5/7/8 package selected by `kind`.
pub fn issue_operational_role_witness_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
) -> Result<OperationalRoleWitness, TypedBoundaryError> {
    let source =
        registered_source_for_historical_prefix(predecessor_signature, kind, current_telescope)?;
    issue_operational_role_witness_v3_from_source(predecessor_signature.digest(), &source)
}

pub fn replay_operational_role_witness_v3(
    signature: &SealedSignature,
    token: &OperationalRoleWitness,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_operational_role_witness_v3(signature, token.kind)?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::OperationalRoleReplayMismatch)
    }
}

pub fn replay_operational_role_witness_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &OperationalRoleWitness,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_operational_role_witness_v3_for_historical_prefix(
        predecessor_signature,
        token.kind,
        current_telescope,
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::OperationalRoleReplayMismatch)
    }
}

/// Opaque V3 interpretation token reading one registered clause as an
/// element of its immediately preceding formation clause.  The recorded
/// shallow type is retained beside the overlay reading; no sealed data is
/// rewritten.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalElementOverlayToken {
    fragment_version: String,
    axiom_version: String,
    overlay_version: String,
    signature_digest: String,
    candidate_hash: String,
    kind: RegisteredBoundaryKind,
    step: u32,
    formation_clause: u16,
    element_clause: u16,
    formation_term: Expr,
    element_term: Expr,
    owner: Expr,
    recorded_type: KernelTy,
    overlay_type: KernelTy,
    elaboration_derivation_hash: String,
    operational_role_derivation_hash: String,
    derivation_hash: String,
}

impl HistoricalElementOverlayToken {
    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn step(&self) -> u32 {
        self.step
    }

    pub const fn formation_clause(&self) -> u16 {
        self.formation_clause
    }

    pub const fn element_clause(&self) -> u16 {
        self.element_clause
    }

    pub fn axiom_version(&self) -> &str {
        &self.axiom_version
    }

    pub fn overlay_version(&self) -> &str {
        &self.overlay_version
    }

    pub fn owner(&self) -> &Expr {
        &self.owner
    }

    pub fn recorded_type(&self) -> &KernelTy {
        &self.recorded_type
    }

    pub fn overlay_type(&self) -> &KernelTy {
        &self.overlay_type
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_historical_element_overlay_token(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<HistoricalElementOverlayToken, TypedBoundaryError> {
    let source = registered_source(signature, kind)?;
    issue_historical_element_overlay_token_from_source(signature.digest(), &source)
}

fn issue_historical_element_overlay_token_from_source(
    signature_digest: &str,
    source: &RegisteredSource,
) -> Result<HistoricalElementOverlayToken, TypedBoundaryError> {
    let kind = source.kind;
    let entry = registered_element_overlay_entry(kind)?;
    let role = issue_operational_role_witness_v3_from_source(signature_digest, source)?;
    let formation = source
        .telescope
        .clauses
        .get(usize::from(entry.formation_clause))
        .ok_or(TypedBoundaryError::OperationalRoleWitnessFailed {
            kind,
            reason: OperationalRoleFailure::RequiredClauseAbsent,
        })?;
    let element = source
        .telescope
        .clauses
        .get(usize::from(entry.element_clause))
        .ok_or(TypedBoundaryError::OperationalRoleWitnessFailed {
            kind,
            reason: OperationalRoleFailure::RequiredClauseAbsent,
        })?;
    let element_elaboration = source
        .elaboration
        .clauses
        .get(usize::from(entry.element_clause))
        .ok_or(TypedBoundaryError::OperationalRoleWitnessFailed {
            kind,
            reason: OperationalRoleFailure::RequiredClauseAbsent,
        })?;
    let recorded_type = element_elaboration.kernel_ty.clone();
    if recorded_type != KernelTy::Type {
        return Err(TypedBoundaryError::HistoricalElementOverlayTypeMismatch);
    }
    let owner = source.typing.formation_normal_form.clone();
    let overlay_type = KernelTy::El(owner.clone());
    let fragment_version = TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION.to_owned();
    let axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned();
    let overlay_version = HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION.to_owned();
    let signature_digest = signature_digest.to_owned();
    let candidate_hash = source.candidate_hash.clone();
    let formation_term = formation.expr.clone();
    let element_term = element.expr.clone();
    let elaboration_derivation_hash = source.elaboration.derivation_hash.clone();
    let operational_role_derivation_hash = role.derivation_hash;
    let derivation_hash = element_overlay_tagged_digest(
        "historical-element-overlay-token",
        &(
            &fragment_version,
            &axiom_version,
            &overlay_version,
            &signature_digest,
            &candidate_hash,
            kind,
            entry.step,
            entry.formation_clause,
            entry.element_clause,
            &formation_term,
            &element_term,
            &owner,
            &recorded_type,
            &overlay_type,
            &elaboration_derivation_hash,
            &operational_role_derivation_hash,
        ),
    );
    Ok(HistoricalElementOverlayToken {
        fragment_version,
        axiom_version,
        overlay_version,
        signature_digest,
        candidate_hash,
        kind,
        step: entry.step,
        formation_clause: entry.formation_clause,
        element_clause: entry.element_clause,
        formation_term,
        element_term,
        owner,
        recorded_type,
        overlay_type,
        elaboration_derivation_hash,
        operational_role_derivation_hash,
        derivation_hash,
    })
}

pub fn issue_historical_element_overlay_token_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
) -> Result<HistoricalElementOverlayToken, TypedBoundaryError> {
    let source =
        registered_source_for_historical_prefix(predecessor_signature, kind, current_telescope)?;
    issue_historical_element_overlay_token_from_source(predecessor_signature.digest(), &source)
}

pub fn replay_historical_element_overlay_token(
    signature: &SealedSignature,
    token: &HistoricalElementOverlayToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_element_overlay_token(signature, token.kind)?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalElementOverlayReplayMismatch)
    }
}

pub fn replay_historical_element_overlay_token_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &HistoricalElementOverlayToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_element_overlay_token_for_historical_prefix(
        predecessor_signature,
        token.kind,
        current_telescope,
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalElementOverlayReplayMismatch)
    }
}

fn hoisting_required(
    face: BoundaryFaceKey,
    term: impl Into<String>,
    reason: ReferenceResolutionFailure,
) -> TypedBoundaryError {
    TypedBoundaryError::HoistingRequired {
        face,
        term: term.into(),
        reason,
        charging_policy: BOUNDARY_CHARGE_POLICY_VERSION.to_owned(),
        hoisting_rule: REFERENCE_ONLY_HOISTING_RULE.to_owned(),
    }
}

fn resolve_face_origin(
    signature: &SealedSignature,
    source: &RegisteredSource,
    context: &TypedParameterContext,
    face: BoundaryFaceKey,
    name: &str,
    origin: &BoundaryTermOrigin,
) -> Result<ResolvedBoundaryRef, TypedBoundaryError> {
    match origin {
        BoundaryTermOrigin::HistoricalPointClause { clause } => {
            let Some(clause_rec) = source.telescope.clauses.get(usize::from(*clause)) else {
                return Err(hoisting_required(
                    face,
                    name,
                    ReferenceResolutionFailure::MissingSealedPackageClause,
                ));
            };
            Ok(ResolvedBoundaryRef::SealedPackageClause(
                SealedPackageClauseRef {
                    signature_digest: signature.digest().to_owned(),
                    step: source.kind.step(),
                    candidate_hash: source.candidate_hash.clone(),
                    clause: *clause,
                    term: clause_rec.expr.clone(),
                },
            ))
        }
        BoundaryTermOrigin::SemanticPointParameter { parameter } => {
            let Some(binding) = context.lookup(parameter) else {
                return Err(hoisting_required(
                    face,
                    parameter,
                    ReferenceResolutionFailure::UnboundContextVariable,
                ));
            };
            Ok(ResolvedBoundaryRef::ContextVariable(ContextVariableRef {
                context_digest: context.context_digest.clone(),
                index: binding.index,
                name: binding.name.clone(),
                ty: binding.ty.clone(),
            }))
        }
        BoundaryTermOrigin::DeclaredFaceFamily { family } => Err(hoisting_required(
            face,
            family,
            ReferenceResolutionFailure::ArbitraryDeclaredFaceFamily,
        )),
    }
}

/// Opaque replayable proof that every registered boundary occurrence resolves
/// to an already charged sealed clause or a typed context variable.  The
/// legacy caller boolean on `DeclaredBoundaryDiagram` is not used as charging
/// evidence; the resolved references below are the evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReferenceOnlyChargeToken {
    fragment_version: String,
    axiom_version: String,
    charging_policy: String,
    hoisting_rule: String,
    registered_diagrams_version: String,
    signature_digest: String,
    candidate_hash: String,
    kind: RegisteredBoundaryKind,
    map: DeclaredBoundaryDiagram,
    structural_map_digest: String,
    parameter_context: TypedParameterContext,
    resolved_faces: Vec<ResolvedBoundaryFace>,
    boundary_credit: u8,
    derivation_hash: String,
}

impl ReferenceOnlyChargeToken {
    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub fn charging_policy(&self) -> &str {
        &self.charging_policy
    }

    pub fn hoisting_rule(&self) -> &str {
        &self.hoisting_rule
    }

    pub const fn boundary_credit(&self) -> u8 {
        self.boundary_credit
    }

    pub fn parameter_context(&self) -> &TypedParameterContext {
        &self.parameter_context
    }

    pub fn resolved_faces(&self) -> &[ResolvedBoundaryFace] {
        &self.resolved_faces
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Derive reference-only provenance and zero boundary charge from actual
/// lookups.  This certificate is available for all four registered diagrams,
/// independently of the C-7 point-typing obstruction.
pub fn issue_reference_only_charge_token(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    map: DeclaredBoundaryDiagram,
) -> Result<ReferenceOnlyChargeToken, TypedBoundaryError> {
    let source = registered_source(signature, kind)?;
    issue_reference_only_charge_token_from_source(signature, &source, map, TypedBoundaryAxiom::V2)
}

fn issue_reference_only_charge_token_from_source(
    signature: &SealedSignature,
    source: &RegisteredSource,
    map: DeclaredBoundaryDiagram,
    axiom: TypedBoundaryAxiom,
) -> Result<ReferenceOnlyChargeToken, TypedBoundaryError> {
    let kind = source.kind;
    let context = parameter_context_for_source(&source);

    // Resolve before comparing with the registry so arbitrary/unbound family
    // errors name the mandatory hoisting rule rather than degrading to a
    // generic diagram-mismatch error.
    let mut source_by_face = BTreeMap::new();
    let mut source_by_name: BTreeMap<String, ResolvedBoundaryRef> = BTreeMap::new();
    for declaration in &map.faces {
        let resolved = resolve_face_origin(
            signature,
            source,
            &context,
            declaration.fixed,
            &declaration.term.name,
            &declaration.term.origin,
        )?;
        if let Some(previous) = source_by_name.get(&declaration.term.name)
            && previous != &resolved
        {
            return Err(hoisting_required(
                declaration.fixed,
                &declaration.term.name,
                ReferenceResolutionFailure::AmbiguousBoundaryName,
            ));
        }
        source_by_name.insert(declaration.term.name.clone(), resolved.clone());
        source_by_face.insert(declaration.fixed, resolved);
    }

    let mut resolved_faces = Vec::with_capacity(map.faces.len());
    for declaration in &map.faces {
        let term = source_by_face
            .get(&declaration.fixed)
            .expect("each face was resolved above")
            .clone();
        let mut restrictions = Vec::with_capacity(declaration.term.restrictions.len());
        for (restricted_face, label) in &declaration.term.restrictions {
            let Some(resolved) = source_by_name.get(label) else {
                return Err(hoisting_required(
                    declaration.fixed,
                    label,
                    ReferenceResolutionFailure::UnboundRestrictionTerm,
                ));
            };
            restrictions.push(ResolvedBoundaryRestriction {
                face: *restricted_face,
                term: resolved.clone(),
            });
        }
        resolved_faces.push(ResolvedBoundaryFace {
            fixed: declaration.fixed,
            term,
            remaining_axes: declaration.term.remaining_axes.clone(),
            restrictions,
        });
    }

    let structural = check_declared_boundary_diagram(map.clone())?;
    let canonical = canonical_diagram_for_source(source)?;
    if map != canonical {
        return Err(TypedBoundaryError::RegisteredDiagramMismatch { kind });
    }

    let fragment_version = match axiom {
        TypedBoundaryAxiom::V2 => TYPED_BOUNDARY_FRAGMENT_VERSION,
        TypedBoundaryAxiom::V3ElementOverlay => TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
    }
    .to_owned();
    let axiom_version = match axiom {
        TypedBoundaryAxiom::V2 => ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION,
        TypedBoundaryAxiom::V3ElementOverlay => ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION,
    }
    .to_owned();
    let charging_policy = BOUNDARY_CHARGE_POLICY_VERSION.to_owned();
    let hoisting_rule = REFERENCE_ONLY_HOISTING_RULE.to_owned();
    let registered_diagrams_version = REGISTERED_HISTORICAL_DIAGRAMS_VERSION.to_owned();
    let signature_digest = signature.digest().to_owned();
    let candidate_hash = source.candidate_hash.clone();
    let structural_map_digest = structural.map_digest().to_owned();
    let boundary_credit = 0;
    let derivation_payload = &(
        &fragment_version,
        &axiom_version,
        &charging_policy,
        &hoisting_rule,
        &registered_diagrams_version,
        &signature_digest,
        &candidate_hash,
        kind,
        &structural_map_digest,
        &context,
        &resolved_faces,
        boundary_credit,
    );
    let derivation_hash = match axiom {
        TypedBoundaryAxiom::V2 => tagged_digest("reference-only-charge-token", derivation_payload),
        TypedBoundaryAxiom::V3ElementOverlay => element_overlay_tagged_digest(
            "reference-only-charge-token-v3-historical-prefix",
            derivation_payload,
        ),
    };
    Ok(ReferenceOnlyChargeToken {
        fragment_version,
        axiom_version,
        charging_policy,
        hoisting_rule,
        registered_diagrams_version,
        signature_digest,
        candidate_hash,
        kind,
        map,
        structural_map_digest,
        parameter_context: context,
        resolved_faces,
        boundary_credit,
        derivation_hash,
    })
}

pub fn issue_reference_only_charge_token_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
    map: DeclaredBoundaryDiagram,
) -> Result<ReferenceOnlyChargeToken, TypedBoundaryError> {
    let source =
        registered_source_for_historical_prefix(predecessor_signature, kind, current_telescope)?;
    issue_reference_only_charge_token_from_source(
        predecessor_signature,
        &source,
        map,
        TypedBoundaryAxiom::V3ElementOverlay,
    )
}

pub fn replay_reference_only_charge_token(
    signature: &SealedSignature,
    token: &ReferenceOnlyChargeToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_reference_only_charge_token(signature, token.kind, token.map.clone())?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::ReferenceOnlyReplayMismatch)
    }
}

pub fn replay_reference_only_charge_token_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &ReferenceOnlyChargeToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_reference_only_charge_token_v3_for_historical_prefix(
        predecessor_signature,
        token.kind,
        current_telescope,
        token.map.clone(),
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::ReferenceOnlyReplayMismatch)
    }
}

/// Opaque proof that the registered presentation uses the finite formula
/// `1 + d^2` and that the only additional boundary term is `c(b) = 0`, with
/// both conclusions bound to the exact adopted axiom and policy versions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdoptedBoundaryBasisChargeToken {
    fragment_version: String,
    axiom_version: String,
    charging_policy: String,
    signature_digest: String,
    candidate_hash: String,
    kind: RegisteredBoundaryKind,
    map: DeclaredBoundaryDiagram,
    dimension: u32,
    formula: BoundaryBasisFormula,
    beta_count: u32,
    principal_transport_count: u32,
    ordered_naturality_count: u64,
    basis_count: u64,
    boundary_credit: u8,
    total_with_boundary: u64,
    reference_only_derivation_hash: String,
    derivation_hash: String,
}

impl AdoptedBoundaryBasisChargeToken {
    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn dimension(&self) -> u32 {
        self.dimension
    }

    pub const fn formula(&self) -> BoundaryBasisFormula {
        self.formula
    }

    pub const fn boundary_credit(&self) -> u8 {
        self.boundary_credit
    }

    pub const fn basis_count(&self) -> u64 {
        self.basis_count
    }

    pub const fn total_with_boundary(&self) -> u64 {
        self.total_with_boundary
    }

    pub fn charging_policy(&self) -> &str {
        &self.charging_policy
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_adopted_boundary_basis_charge_token(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    map: DeclaredBoundaryDiagram,
) -> Result<AdoptedBoundaryBasisChargeToken, TypedBoundaryError> {
    let reference_only = issue_reference_only_charge_token(signature, kind, map.clone())?;
    let source = registered_source(signature, kind)?;
    let dimension = kind.dimension();
    let beta_count = 1;
    let principal_transport_count = dimension;
    let ordered_naturality_count = u64::from(dimension) * u64::from(dimension.saturating_sub(1));
    let basis_count = boundary_basis_cardinality(dimension);
    if basis_count
        != u64::from(beta_count) + u64::from(principal_transport_count) + ordered_naturality_count
    {
        return Err(TypedBoundaryError::AdoptedBasisChargeReplayMismatch);
    }
    let boundary_credit = reference_only.boundary_credit();
    let total_with_boundary = basis_count + u64::from(boundary_credit);
    let fragment_version = ADOPTED_BASIS_CHARGE_BINDING_VERSION.to_owned();
    let axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION.to_owned();
    let charging_policy = BOUNDARY_CHARGE_POLICY_VERSION.to_owned();
    let signature_digest = signature.digest().to_owned();
    let candidate_hash = source.candidate_hash;
    let formula = BoundaryBasisFormula::OnePlusDimensionSquared;
    let reference_only_derivation_hash = reference_only.derivation_hash().to_owned();
    let derivation_hash = tagged_digest(
        "adopted-boundary-basis-charge-token",
        &(
            &fragment_version,
            &axiom_version,
            &charging_policy,
            &signature_digest,
            &candidate_hash,
            kind,
            &map,
            dimension,
            formula,
            beta_count,
            principal_transport_count,
            ordered_naturality_count,
            basis_count,
            boundary_credit,
            total_with_boundary,
            &reference_only_derivation_hash,
        ),
    );
    Ok(AdoptedBoundaryBasisChargeToken {
        fragment_version,
        axiom_version,
        charging_policy,
        signature_digest,
        candidate_hash,
        kind,
        map,
        dimension,
        formula,
        beta_count,
        principal_transport_count,
        ordered_naturality_count,
        basis_count,
        boundary_credit,
        total_with_boundary,
        reference_only_derivation_hash,
        derivation_hash,
    })
}

pub fn replay_adopted_boundary_basis_charge_token(
    signature: &SealedSignature,
    token: &AdoptedBoundaryBasisChargeToken,
) -> Result<(), TypedBoundaryError> {
    let replay =
        issue_adopted_boundary_basis_charge_token(signature, token.kind, token.map.clone())?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::AdoptedBasisChargeReplayMismatch)
    }
}

/// Opaque typed witness connecting the cubical base to the owner's historical
/// point clause.  The current kernel cannot issue one for S1/S2/S3; keeping
/// the future-success representation here makes that absence explicit and
/// replayable rather than silently changing the required judgement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalBaseBindingToken {
    fragment_version: String,
    axiom_version: String,
    signature_digest: String,
    candidate_hash: String,
    kind: RegisteredBoundaryKind,
    formation_clause: u16,
    point_clause: u16,
    path_clause: u16,
    owner: Expr,
    point_term: Expr,
    point_type: KernelTy,
    elaboration_derivation_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    element_overlay_derivation_hash: Option<String>,
    derivation_hash: String,
}

impl HistoricalBaseBindingToken {
    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn point_clause(&self) -> u16 {
        self.point_clause
    }

    pub fn owner(&self) -> &Expr {
        &self.owner
    }

    pub fn point_term(&self) -> &Expr {
        &self.point_term
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn axiom_version(&self) -> &str {
        &self.axiom_version
    }

    pub fn element_overlay_derivation_hash(&self) -> Option<&str> {
        self.element_overlay_derivation_hash.as_deref()
    }

    pub fn point_type(&self) -> &KernelTy {
        &self.point_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum HistoricalBaseBindingAudit {
    Bound(HistoricalBaseBindingToken),
    Obstructed(HistoricalPointTypingObstruction),
}

/// Audit, but do not assume, the historical point typing judgement.
pub fn audit_historical_base_binding(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<HistoricalBaseBindingAudit, TypedBoundaryError> {
    if !kind.needs_historical_base_binding() {
        return Err(TypedBoundaryError::HistoricalBaseBindingNotApplicable);
    }
    let source = registered_source(signature, kind)?;
    let point_clause = source
        .telescope
        .clauses
        .get(usize::from(HISTORICAL_POINT_CLAUSE))
        .ok_or(TypedBoundaryError::RegisteredSourceMismatch { step: kind.step() })?;
    let point_elaboration = source
        .elaboration
        .clauses
        .get(usize::from(HISTORICAL_POINT_CLAUSE))
        .ok_or(TypedBoundaryError::RegisteredSourceMismatch { step: kind.step() })?;
    let owner = source.typing.formation_normal_form.clone();
    let expected_type = KernelTy::El(owner.clone());
    let actual_type = point_elaboration.kernel_ty.clone();
    if actual_type != expected_type {
        let mut obstruction = HistoricalPointTypingObstruction {
            obstruction_id: HISTORICAL_POINT_TYPING_OBSTRUCTION_ID.to_owned(),
            axiom_version: ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION.to_owned(),
            signature_digest: signature.digest().to_owned(),
            kind,
            step: kind.step(),
            candidate_hash: source.candidate_hash,
            formation_clause: source.typing.formation_clause,
            point_clause: HISTORICAL_POINT_CLAUSE,
            path_clause: source.typing.path_clause,
            point_term: point_clause.expr.clone(),
            owner,
            expected_type,
            actual_type,
            elaboration_derivation_hash: source.elaboration.derivation_hash,
            derivation_hash: String::new(),
        };
        obstruction.derivation_hash = tagged_digest(
            "historical-point-typing-obstruction",
            &(
                &obstruction.obstruction_id,
                &obstruction.axiom_version,
                &obstruction.signature_digest,
                obstruction.kind,
                obstruction.step,
                &obstruction.candidate_hash,
                obstruction.formation_clause,
                obstruction.point_clause,
                obstruction.path_clause,
                &obstruction.point_term,
                &obstruction.owner,
                &obstruction.expected_type,
                &obstruction.actual_type,
                &obstruction.elaboration_derivation_hash,
            ),
        );
        return Ok(HistoricalBaseBindingAudit::Obstructed(obstruction));
    }

    let fragment_version = TYPED_BOUNDARY_FRAGMENT_VERSION.to_owned();
    let axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION.to_owned();
    let signature_digest = signature.digest().to_owned();
    let candidate_hash = source.candidate_hash;
    let point_term = point_clause.expr.clone();
    let elaboration_derivation_hash = source.elaboration.derivation_hash;
    let derivation_hash = tagged_digest(
        "historical-base-binding-token",
        &(
            &fragment_version,
            &axiom_version,
            &signature_digest,
            &candidate_hash,
            kind,
            HISTORICAL_FORMATION_CLAUSE,
            HISTORICAL_POINT_CLAUSE,
            HISTORICAL_PATH_CLAUSE,
            &owner,
            &point_term,
            &actual_type,
            &elaboration_derivation_hash,
        ),
    );
    Ok(HistoricalBaseBindingAudit::Bound(
        HistoricalBaseBindingToken {
            fragment_version,
            axiom_version,
            signature_digest,
            candidate_hash,
            kind,
            formation_clause: HISTORICAL_FORMATION_CLAUSE,
            point_clause: HISTORICAL_POINT_CLAUSE,
            path_clause: HISTORICAL_PATH_CLAUSE,
            owner,
            point_term,
            point_type: actual_type,
            elaboration_derivation_hash,
            element_overlay_derivation_hash: None,
            derivation_hash,
        },
    ))
}

pub fn issue_historical_base_binding_token(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<HistoricalBaseBindingToken, TypedBoundaryError> {
    match audit_historical_base_binding(signature, kind)? {
        HistoricalBaseBindingAudit::Bound(token) => Ok(token),
        HistoricalBaseBindingAudit::Obstructed(obstruction) => {
            Err(TypedBoundaryError::HistoricalPointTypingObstructed {
                obstruction: Box::new(obstruction),
            })
        }
    }
}

pub fn replay_historical_base_binding_token(
    signature: &SealedSignature,
    token: &HistoricalBaseBindingToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_base_binding_token(signature, token.kind)?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalBaseBindingReplayMismatch)
    }
}

fn construct_historical_base_binding_token_v3(
    signature_digest: &str,
    source: RegisteredSource,
    overlay: &HistoricalElementOverlayToken,
) -> Result<HistoricalBaseBindingToken, TypedBoundaryError> {
    let point_clause = source
        .telescope
        .clauses
        .get(usize::from(HISTORICAL_POINT_CLAUSE))
        .ok_or(TypedBoundaryError::HistoricalElementOverlayTypeMismatch)?;
    let owner = source.typing.formation_normal_form;
    let expected_type = KernelTy::El(owner.clone());
    if overlay.kind != source.kind
        || overlay.step != source.kind.step()
        || overlay.formation_clause != HISTORICAL_FORMATION_CLAUSE
        || overlay.element_clause != HISTORICAL_POINT_CLAUSE
        || overlay.signature_digest != signature_digest
        || overlay.candidate_hash != source.candidate_hash
        || overlay.element_term != point_clause.expr
        || overlay.owner != owner
        || overlay.overlay_type != expected_type
        || overlay.recorded_type != KernelTy::Type
    {
        return Err(TypedBoundaryError::HistoricalElementOverlayTypeMismatch);
    }

    let fragment_version = TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION.to_owned();
    let axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned();
    let signature_digest = signature_digest.to_owned();
    let candidate_hash = source.candidate_hash;
    let kind = source.kind;
    let point_term = point_clause.expr.clone();
    let point_type = expected_type;
    let elaboration_derivation_hash = source.elaboration.derivation_hash;
    let element_overlay_derivation_hash = Some(overlay.derivation_hash.clone());
    let derivation_hash = element_overlay_tagged_digest(
        "historical-base-binding-token-v3",
        &(
            &fragment_version,
            &axiom_version,
            HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION,
            &signature_digest,
            &candidate_hash,
            kind,
            HISTORICAL_FORMATION_CLAUSE,
            HISTORICAL_POINT_CLAUSE,
            HISTORICAL_PATH_CLAUSE,
            &owner,
            &point_term,
            &point_type,
            &elaboration_derivation_hash,
            &element_overlay_derivation_hash,
        ),
    );
    Ok(HistoricalBaseBindingToken {
        fragment_version,
        axiom_version,
        signature_digest,
        candidate_hash,
        kind,
        formation_clause: HISTORICAL_FORMATION_CLAUSE,
        point_clause: HISTORICAL_POINT_CLAUSE,
        path_clause: HISTORICAL_PATH_CLAUSE,
        owner,
        point_term,
        point_type,
        elaboration_derivation_hash,
        element_overlay_derivation_hash,
        derivation_hash,
    })
}

/// Issue the actual historical base binding under the adopted V3 overlay.
/// The overlay is obtained internally and replayed from the sealed source;
/// callers cannot supply a claimed sort reading.
pub fn issue_historical_base_binding_token_v3(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
) -> Result<HistoricalBaseBindingToken, TypedBoundaryError> {
    let overlay = issue_historical_element_overlay_token(signature, kind)?;
    let source = registered_source(signature, kind)?;
    construct_historical_base_binding_token_v3(signature.digest(), source, &overlay)
}

pub fn issue_historical_base_binding_token_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
) -> Result<HistoricalBaseBindingToken, TypedBoundaryError> {
    let overlay = issue_historical_element_overlay_token_for_historical_prefix(
        predecessor_signature,
        kind,
        current_telescope,
    )?;
    let source =
        registered_source_for_historical_prefix(predecessor_signature, kind, current_telescope)?;
    construct_historical_base_binding_token_v3(predecessor_signature.digest(), source, &overlay)
}

pub fn replay_historical_base_binding_token_v3(
    signature: &SealedSignature,
    token: &HistoricalBaseBindingToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_base_binding_token_v3(signature, token.kind)?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalBaseBindingV3ReplayMismatch)
    }
}

pub fn replay_historical_base_binding_token_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &HistoricalBaseBindingToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_base_binding_token_v3_for_historical_prefix(
        predecessor_signature,
        token.kind,
        current_telescope,
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalBaseBindingV3ReplayMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BoundaryTypingWitness {
    ContextLookup {
        context_digest: String,
        binding_index: u16,
    },
    HistoricalBaseBinding {
        binding_derivation_hash: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedBoundaryTerm {
    source: ResolvedBoundaryRef,
    ty: BoundaryParameterType,
    normal_form_digest: String,
    witness: BoundaryTypingWitness,
}

impl TypedBoundaryTerm {
    pub fn source(&self) -> &ResolvedBoundaryRef {
        &self.source
    }

    pub fn ty(&self) -> &BoundaryParameterType {
        &self.ty
    }

    pub fn normal_form_digest(&self) -> &str {
        &self.normal_form_digest
    }

    pub fn witness(&self) -> &BoundaryTypingWitness {
        &self.witness
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedBoundaryRestriction {
    face: BoundaryFaceKey,
    term: TypedBoundaryTerm,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedBoundaryFace {
    fixed: BoundaryFaceKey,
    term: TypedBoundaryTerm,
    remaining_axes: Vec<u32>,
    restrictions: Vec<TypedBoundaryRestriction>,
}

impl TypedBoundaryFace {
    pub const fn fixed(&self) -> BoundaryFaceKey {
        self.fixed
    }

    pub fn term(&self) -> &TypedBoundaryTerm {
        &self.term
    }

    pub fn restrictions(&self) -> &[TypedBoundaryRestriction] {
        &self.restrictions
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedOverlapWitness {
    left: BoundaryFaceKey,
    right: BoundaryFaceKey,
    common_normal_form_digest: String,
}

/// Opaque term-level certificate for one exact registered diagram.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedDeclaredBoundaryToken {
    fragment_version: String,
    axiom_version: String,
    registered_diagrams_version: String,
    signature_digest: String,
    candidate_hash: String,
    kind: RegisteredBoundaryKind,
    owner: Expr,
    dimension: u32,
    map: DeclaredBoundaryDiagram,
    parameter_context: TypedParameterContext,
    reference_only_derivation_hash: String,
    base_binding_derivation_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    element_overlay_derivation_hash: Option<String>,
    faces: Vec<TypedBoundaryFace>,
    overlaps: Vec<TypedOverlapWitness>,
    derivation_hash: String,
}

impl TypedDeclaredBoundaryToken {
    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub fn owner(&self) -> &Expr {
        &self.owner
    }

    pub const fn dimension(&self) -> u32 {
        self.dimension
    }

    pub fn parameter_context(&self) -> &TypedParameterContext {
        &self.parameter_context
    }

    pub fn faces(&self) -> &[TypedBoundaryFace] {
        &self.faces
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn axiom_version(&self) -> &str {
        &self.axiom_version
    }

    pub fn element_overlay_derivation_hash(&self) -> Option<&str> {
        self.element_overlay_derivation_hash.as_deref()
    }

    pub fn base_binding_derivation_hash(&self) -> Option<&str> {
        self.base_binding_derivation_hash.as_deref()
    }

    pub fn reference_only_derivation_hash(&self) -> &str {
        &self.reference_only_derivation_hash
    }
}

#[derive(Clone, Copy)]
enum TypedBoundaryAxiom {
    V2,
    V3ElementOverlay,
}

fn type_resolved_term(
    face: BoundaryFaceKey,
    source: &ResolvedBoundaryRef,
    owner: &Expr,
    base_binding: Option<&HistoricalBaseBindingToken>,
) -> Result<TypedBoundaryTerm, TypedBoundaryError> {
    let expected = BoundaryParameterType::Element {
        owner: owner.clone(),
    };
    let witness = match source {
        ResolvedBoundaryRef::ContextVariable(reference) => {
            if reference.ty != expected {
                return Err(TypedBoundaryError::BoundaryTermTypeMismatch { face });
            }
            BoundaryTypingWitness::ContextLookup {
                context_digest: reference.context_digest.clone(),
                binding_index: reference.index,
            }
        }
        ResolvedBoundaryRef::SealedPackageClause(reference) => {
            let Some(binding) = base_binding else {
                return Err(TypedBoundaryError::BoundaryTermTypeMismatch { face });
            };
            if binding.point_clause != reference.clause
                || binding.point_term != reference.term
                || binding.owner != *owner
                || binding.point_type != KernelTy::El(owner.clone())
            {
                return Err(TypedBoundaryError::BoundaryTermTypeMismatch { face });
            }
            BoundaryTypingWitness::HistoricalBaseBinding {
                binding_derivation_hash: binding.derivation_hash.clone(),
            }
        }
    };
    let normal_form_digest = tagged_digest("typed-boundary-term-normal-form", source);
    Ok(TypedBoundaryTerm {
        source: source.clone(),
        ty: expected,
        normal_form_digest,
        witness,
    })
}

/// Type-check an exact registered diagram in the closed adopted term language.
/// At present this succeeds for Trunc and returns the named C-7 obstruction
/// for S1/S2/S3.
pub fn issue_typed_declared_boundary_token(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    map: DeclaredBoundaryDiagram,
) -> Result<TypedDeclaredBoundaryToken, TypedBoundaryError> {
    issue_typed_declared_boundary_token_under(signature, kind, map, TypedBoundaryAxiom::V2)
}

fn issue_typed_declared_boundary_token_under(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    map: DeclaredBoundaryDiagram,
    axiom: TypedBoundaryAxiom,
) -> Result<TypedDeclaredBoundaryToken, TypedBoundaryError> {
    let reference_only = issue_reference_only_charge_token(signature, kind, map.clone())?;
    let source = registered_source(signature, kind)?;
    let base_binding = match (axiom, kind.needs_historical_base_binding()) {
        (TypedBoundaryAxiom::V2, true) => {
            Some(issue_historical_base_binding_token(signature, kind)?)
        }
        (TypedBoundaryAxiom::V3ElementOverlay, true) => {
            Some(issue_historical_base_binding_token_v3(signature, kind)?)
        }
        (_, false) => None,
    };
    assemble_typed_declared_boundary_token(
        signature.digest(),
        source,
        map,
        reference_only,
        base_binding,
        axiom,
    )
}

fn assemble_typed_declared_boundary_token(
    signature_digest: &str,
    source: RegisteredSource,
    map: DeclaredBoundaryDiagram,
    reference_only: ReferenceOnlyChargeToken,
    base_binding: Option<HistoricalBaseBindingToken>,
    axiom: TypedBoundaryAxiom,
) -> Result<TypedDeclaredBoundaryToken, TypedBoundaryError> {
    let kind = source.kind;
    let owner = source.typing.formation_normal_form;
    let mut faces = Vec::with_capacity(reference_only.resolved_faces.len());
    for resolved in &reference_only.resolved_faces {
        let term = type_resolved_term(
            resolved.fixed,
            &resolved.term,
            &owner,
            base_binding.as_ref(),
        )?;
        let mut restrictions = Vec::with_capacity(resolved.restrictions.len());
        for restriction in &resolved.restrictions {
            restrictions.push(TypedBoundaryRestriction {
                face: restriction.face,
                term: type_resolved_term(
                    restriction.face,
                    &restriction.term,
                    &owner,
                    base_binding.as_ref(),
                )?,
            });
        }
        faces.push(TypedBoundaryFace {
            fixed: resolved.fixed,
            term,
            remaining_axes: resolved.remaining_axes.clone(),
            restrictions,
        });
    }

    let face_by_key = faces
        .iter()
        .map(|face| (face.fixed, face))
        .collect::<BTreeMap<_, _>>();
    let mut overlaps = Vec::new();
    for (index, left) in faces.iter().enumerate() {
        for right in faces.iter().skip(index + 1) {
            if left.fixed.axis == right.fixed.axis {
                continue;
            }
            let left_term = left
                .restrictions
                .iter()
                .find(|restriction| restriction.face == right.fixed)
                .map(|restriction| &restriction.term);
            let right_term = right
                .restrictions
                .iter()
                .find(|restriction| restriction.face == left.fixed)
                .map(|restriction| &restriction.term);
            let (Some(left_term), Some(right_term)) = (left_term, right_term) else {
                return Err(TypedBoundaryError::TypedOverlapMismatch);
            };
            if left_term.source != right_term.source
                || left_term.ty != right_term.ty
                || left_term.normal_form_digest != right_term.normal_form_digest
            {
                return Err(TypedBoundaryError::TypedOverlapMismatch);
            }
            overlaps.push(TypedOverlapWitness {
                left: left.fixed,
                right: right.fixed,
                common_normal_form_digest: left_term.normal_form_digest.clone(),
            });
        }
    }
    if face_by_key.len() != faces.len() {
        return Err(TypedBoundaryError::TypedOverlapMismatch);
    }

    let fragment_version = match axiom {
        TypedBoundaryAxiom::V2 => TYPED_BOUNDARY_FRAGMENT_VERSION,
        TypedBoundaryAxiom::V3ElementOverlay => TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
    }
    .to_owned();
    let axiom_version = match axiom {
        TypedBoundaryAxiom::V2 => ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION,
        TypedBoundaryAxiom::V3ElementOverlay => ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION,
    }
    .to_owned();
    let registered_diagrams_version = REGISTERED_HISTORICAL_DIAGRAMS_VERSION.to_owned();
    let signature_digest = signature_digest.to_owned();
    let candidate_hash = source.candidate_hash;
    let dimension = kind.dimension();
    let parameter_context = reference_only.parameter_context.clone();
    let reference_only_derivation_hash = reference_only.derivation_hash.clone();
    let base_binding_derivation_hash = base_binding
        .as_ref()
        .map(|binding| binding.derivation_hash.clone());
    let element_overlay_derivation_hash = base_binding
        .as_ref()
        .and_then(|binding| binding.element_overlay_derivation_hash.clone());
    let derivation_hash = match axiom {
        TypedBoundaryAxiom::V2 => tagged_digest(
            "typed-declared-boundary-token",
            &(
                &fragment_version,
                &axiom_version,
                &registered_diagrams_version,
                &signature_digest,
                &candidate_hash,
                kind,
                &owner,
                dimension,
                &parameter_context,
                &reference_only_derivation_hash,
                &base_binding_derivation_hash,
                &faces,
                &overlaps,
            ),
        ),
        TypedBoundaryAxiom::V3ElementOverlay => element_overlay_tagged_digest(
            "typed-declared-boundary-token-v3",
            &(
                &fragment_version,
                &axiom_version,
                HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION,
                &registered_diagrams_version,
                &signature_digest,
                &candidate_hash,
                kind,
                &owner,
                dimension,
                &parameter_context,
                &reference_only_derivation_hash,
                &base_binding_derivation_hash,
                &element_overlay_derivation_hash,
                &faces,
                &overlaps,
            ),
        ),
    };
    Ok(TypedDeclaredBoundaryToken {
        fragment_version,
        axiom_version,
        registered_diagrams_version,
        signature_digest,
        candidate_hash,
        kind,
        owner,
        dimension,
        map,
        parameter_context,
        reference_only_derivation_hash,
        base_binding_derivation_hash,
        element_overlay_derivation_hash,
        faces,
        overlaps,
        derivation_hash,
    })
}

/// Type-check an exact registered diagram under the adopted V3 element
/// overlay.  S1/S2/S3 consume a replayed overlay/base-binding token; Trunc
/// consumes no overlay entry and remains the V2 control case.
pub fn issue_typed_declared_boundary_token_v3(
    signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    map: DeclaredBoundaryDiagram,
) -> Result<TypedDeclaredBoundaryToken, TypedBoundaryError> {
    issue_typed_declared_boundary_token_under(
        signature,
        kind,
        map,
        TypedBoundaryAxiom::V3ElementOverlay,
    )
}

pub fn issue_typed_declared_boundary_token_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
    map: DeclaredBoundaryDiagram,
) -> Result<TypedDeclaredBoundaryToken, TypedBoundaryError> {
    let source =
        registered_source_for_historical_prefix(predecessor_signature, kind, current_telescope)?;
    let reference_only = issue_reference_only_charge_token_from_source(
        predecessor_signature,
        &source,
        map.clone(),
        TypedBoundaryAxiom::V3ElementOverlay,
    )?;
    let base_binding = if kind.needs_historical_base_binding() {
        let overlay = issue_historical_element_overlay_token_from_source(
            predecessor_signature.digest(),
            &source,
        )?;
        Some(construct_historical_base_binding_token_v3(
            predecessor_signature.digest(),
            registered_source_for_historical_prefix(
                predecessor_signature,
                kind,
                current_telescope,
            )?,
            &overlay,
        )?)
    } else {
        None
    };
    assemble_typed_declared_boundary_token(
        predecessor_signature.digest(),
        source,
        map,
        reference_only,
        base_binding,
        TypedBoundaryAxiom::V3ElementOverlay,
    )
}

/// Prefix-general successor of the historical-prefix V3 API for the S3
/// package.  It uses the same adopted element-overlay axiom and token
/// representation, but its source theorem re-elaborates the caller's exact
/// seven-act prefix rather than comparing it with sealed historical B7.
///
/// On historical B7 this returns exactly the legacy token; that equality is a
/// regression theorem, not an input to issuance on any other prefix.
pub fn issue_typed_declared_s3_boundary_token_v4_for_prefix_general(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    map: DeclaredBoundaryDiagram,
) -> Result<TypedDeclaredBoundaryToken, TypedBoundaryError> {
    let source =
        registered_s3_source_for_prefix_general_v4(predecessor_signature, current_telescope)?;
    let reference_only = issue_reference_only_charge_token_from_source(
        predecessor_signature,
        &source,
        map.clone(),
        TypedBoundaryAxiom::V3ElementOverlay,
    )?;
    let overlay = issue_historical_element_overlay_token_from_source(
        predecessor_signature.digest(),
        &source,
    )?;
    let base_binding = Some(construct_historical_base_binding_token_v3(
        predecessor_signature.digest(),
        registered_s3_source_for_prefix_general_v4(predecessor_signature, current_telescope)?,
        &overlay,
    )?);
    assemble_typed_declared_boundary_token(
        predecessor_signature.digest(),
        source,
        map,
        reference_only,
        base_binding,
        TypedBoundaryAxiom::V3ElementOverlay,
    )
}

pub fn replay_typed_declared_boundary_token(
    signature: &SealedSignature,
    token: &TypedDeclaredBoundaryToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_typed_declared_boundary_token(signature, token.kind, token.map.clone())?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::TypedBoundaryReplayMismatch)
    }
}

pub fn replay_typed_declared_boundary_token_v3(
    signature: &SealedSignature,
    token: &TypedDeclaredBoundaryToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_typed_declared_boundary_token_v3(signature, token.kind, token.map.clone())?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::TypedBoundaryV3ReplayMismatch)
    }
}

pub fn replay_typed_declared_boundary_token_v3_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &TypedDeclaredBoundaryToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_typed_declared_boundary_token_v3_for_historical_prefix(
        predecessor_signature,
        token.kind,
        current_telescope,
        token.map.clone(),
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::TypedBoundaryV3ReplayMismatch)
    }
}

pub fn replay_typed_declared_s3_boundary_token_v4_for_prefix_general(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &TypedDeclaredBoundaryToken,
) -> Result<(), TypedBoundaryError> {
    if token.kind != RegisteredBoundaryKind::S3 {
        return Err(TypedBoundaryError::RegisteredDiagramMismatch {
            kind: RegisteredBoundaryKind::S3,
        });
    }
    let replay = issue_typed_declared_s3_boundary_token_v4_for_prefix_general(
        predecessor_signature,
        current_telescope,
        token.map.clone(),
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::TypedBoundaryV3ReplayMismatch)
    }
}

/// Opaque, source-bound C6 bridge for the three constant-boundary historical
/// packages only.  It records successful replay of the V3 typed declared
/// boundary and every existing path-basis realizer.  It intentionally does
/// not assert that the basis is independent or exhaustive.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BoundaryBasisKeyCorrespondence {
    boundary_key: BoundaryBasisKey,
    path_key: PathSchemaKey,
}

impl BoundaryBasisKeyCorrespondence {
    pub fn boundary_key(&self) -> &BoundaryBasisKey {
        &self.boundary_key
    }

    pub fn path_key(&self) -> &PathSchemaKey {
        &self.path_key
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalPrefixV3C6BridgeToken {
    bridge_version: String,
    axiom_version: String,
    overlay_version: String,
    signature_digest: String,
    candidate_hash: String,
    kind: RegisteredBoundaryKind,
    step: u32,
    dimension: u32,
    telescope_digest: String,
    typing_derivation_hash: String,
    typed_boundary_derivation_hash: String,
    boundary_attachment_derivation_hash: String,
    boundary_basis_presentation_derivation_hash: String,
    key_correspondence: Vec<BoundaryBasisKeyCorrespondence>,
    key_bijection_digest: String,
    path_basis_derivation_hash: String,
    path_realization_derivation_hashes: Vec<String>,
    expected_basis_count: u64,
    realized_basis_count: u64,
    derivation_hash: String,
}

impl HistoricalPrefixV3C6BridgeToken {
    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn step(&self) -> u32 {
        self.step
    }

    pub const fn dimension(&self) -> u32 {
        self.dimension
    }

    pub const fn expected_basis_count(&self) -> u64 {
        self.expected_basis_count
    }

    pub const fn realized_basis_count(&self) -> u64 {
        self.realized_basis_count
    }

    pub fn typed_boundary_derivation_hash(&self) -> &str {
        &self.typed_boundary_derivation_hash
    }

    pub fn path_basis_derivation_hash(&self) -> &str {
        &self.path_basis_derivation_hash
    }

    pub fn boundary_basis_presentation_derivation_hash(&self) -> &str {
        &self.boundary_basis_presentation_derivation_hash
    }

    pub fn key_correspondence(&self) -> &[BoundaryBasisKeyCorrespondence] {
        &self.key_correspondence
    }

    pub fn key_bijection_digest(&self) -> &str {
        &self.key_bijection_digest
    }

    pub fn path_realization_derivation_hashes(&self) -> &[String] {
        &self.path_realization_derivation_hashes
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_historical_prefix_v3_c6_bridge_token(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
) -> Result<HistoricalPrefixV3C6BridgeToken, TypedBoundaryError> {
    // Even the named Trunc obstruction is source-bound evidence: reject a
    // wrong predecessor or current package before reporting the honest C6
    // limitation for the exact registered Step-6 input.
    let source =
        registered_source_for_historical_prefix(predecessor_signature, kind, current_telescope)?;
    if kind == RegisteredBoundaryKind::Trunc {
        return Err(TypedBoundaryError::C6EndpointDependentBoundaryGap {
            gap_id: C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP.to_owned(),
        });
    }
    let entry = registered_element_overlay_entry(kind)?;
    let map = canonical_diagram_for_source(&source)?;
    let typed = issue_typed_declared_boundary_token_v3_for_historical_prefix(
        predecessor_signature,
        kind,
        current_telescope,
        map.clone(),
    )?;
    replay_typed_declared_boundary_token_v3_for_historical_prefix(
        predecessor_signature,
        current_telescope,
        &typed,
    )?;

    let typing = source.typing.clone();
    let checked_boundary = check_declared_boundary_diagram(map)?;
    let attachment = issue_boundary_attachment(
        predecessor_signature,
        current_telescope,
        entry.step - 1,
        &typing,
        BoundaryTheory::V2DeclaredBoundary,
        &checked_boundary,
    )?;
    replay_boundary_attachment(
        predecessor_signature,
        current_telescope,
        entry.step - 1,
        &typing,
        &checked_boundary,
        &attachment,
    )?;
    let presentation = present_boundary_basis(&attachment)?;
    let basis = realize_path_basis(
        predecessor_signature,
        current_telescope,
        entry.step - 1,
        &typing,
    )
    .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
        error: error.to_string(),
    })?;
    for token in basis.tokens() {
        replay_path_realization(
            predecessor_signature,
            current_telescope,
            entry.step - 1,
            &typing,
            token,
        )
        .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
            error: error.to_string(),
        })?;
    }
    let expected_basis_count = 1 + u64::from(kind.dimension()).pow(2);
    let realized_basis_count = basis.tokens().len() as u64;
    if realized_basis_count != expected_basis_count
        || presentation.formula() != BoundaryBasisFormula::OnePlusDimensionSquared
        || presentation.total_count() != expected_basis_count
    {
        return Err(TypedBoundaryError::C6PathBasisCountMismatch);
    }

    let mut key_correspondence = Vec::with_capacity(presentation.keys().len());
    for boundary_key in presentation.keys() {
        let path_key = match boundary_key {
            BoundaryBasisKey::Beta => PathSchemaKey::Beta,
            BoundaryBasisKey::PrincipalTransport { principal } => PathSchemaKey::Kan {
                principal: *principal,
                probe: *principal,
            },
            BoundaryBasisKey::TransportNaturality { principal, probe } if principal != probe => {
                PathSchemaKey::Kan {
                    principal: *principal,
                    probe: *probe,
                }
            }
            BoundaryBasisKey::TransportNaturality { .. } => {
                return Err(TypedBoundaryError::C6BoundaryBasisBijectionMismatch);
            }
        };
        key_correspondence.push(BoundaryBasisKeyCorrespondence {
            boundary_key: boundary_key.clone(),
            path_key,
        });
    }
    let mapped_keys = key_correspondence
        .iter()
        .map(|entry| entry.path_key.clone())
        .collect::<BTreeSet<_>>();
    let realized_keys = basis
        .tokens()
        .iter()
        .map(|token| token.key().clone())
        .collect::<BTreeSet<_>>();
    if key_correspondence.len() != presentation.keys().len()
        || mapped_keys.len() != key_correspondence.len()
        || realized_keys.len() != basis.tokens().len()
        || mapped_keys != realized_keys
    {
        return Err(TypedBoundaryError::C6BoundaryBasisBijectionMismatch);
    }
    let key_bijection_digest =
        element_overlay_tagged_digest("c6-boundary-path-key-bijection", &key_correspondence);

    let bridge_version = HISTORICAL_PREFIX_V3_C6_BRIDGE_VERSION.to_owned();
    let axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned();
    let overlay_version = HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION.to_owned();
    let signature_digest = predecessor_signature.digest().to_owned();
    let candidate_hash = source.candidate_hash;
    let dimension = kind.dimension();
    let telescope_digest =
        element_overlay_tagged_digest("c6-bridge-sealed-telescope", current_telescope);
    let typing_derivation_hash = typing.elaboration_derivation_hash;
    let typed_boundary_derivation_hash = typed.derivation_hash;
    let boundary_attachment_derivation_hash = attachment.derivation_hash().to_owned();
    let boundary_basis_presentation_derivation_hash = presentation.derivation_hash().to_owned();
    let path_basis_derivation_hash = basis.derivation_hash().to_owned();
    let path_realization_derivation_hashes = basis
        .tokens()
        .iter()
        .map(|token| token.derivation_hash().to_owned())
        .collect::<Vec<_>>();
    let derivation_hash = element_overlay_tagged_digest(
        "historical-prefix-v3-c6-bridge-token",
        &(
            (
                &bridge_version,
                &axiom_version,
                &overlay_version,
                &signature_digest,
                &candidate_hash,
                kind,
                entry.step,
                dimension,
            ),
            (
                &telescope_digest,
                &typing_derivation_hash,
                &typed_boundary_derivation_hash,
                &boundary_attachment_derivation_hash,
                &boundary_basis_presentation_derivation_hash,
                &key_correspondence,
                &key_bijection_digest,
            ),
            (
                &path_basis_derivation_hash,
                &path_realization_derivation_hashes,
                expected_basis_count,
                realized_basis_count,
            ),
        ),
    );
    Ok(HistoricalPrefixV3C6BridgeToken {
        bridge_version,
        axiom_version,
        overlay_version,
        signature_digest,
        candidate_hash,
        kind,
        step: entry.step,
        dimension,
        telescope_digest,
        typing_derivation_hash,
        typed_boundary_derivation_hash,
        boundary_attachment_derivation_hash,
        boundary_basis_presentation_derivation_hash,
        key_correspondence,
        key_bijection_digest,
        path_basis_derivation_hash,
        path_realization_derivation_hashes,
        expected_basis_count,
        realized_basis_count,
        derivation_hash,
    })
}

pub fn replay_historical_prefix_v3_c6_bridge_token(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &HistoricalPrefixV3C6BridgeToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_prefix_v3_c6_bridge_token(
        predecessor_signature,
        token.kind,
        current_telescope,
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::C6HistoricalPrefixBridgeReplayMismatch)
    }
}

/// The complete inventory of sort-identical variable-image instantiations of
/// the endpoints in the fixed context `A : Type; x,y : Trunc(A)`.  The owner
/// variable has only one sort-identical target (`A`); each endpoint may target
/// either `x` or `y`, so these four cases are exhaustive.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TruncRestrictedInstanceKind {
    Identity,
    Swap,
    CollapseToX,
    CollapseToY,
}

impl TruncRestrictedInstanceKind {
    pub const ALL: [Self; 4] = [
        Self::Identity,
        Self::Swap,
        Self::CollapseToX,
        Self::CollapseToY,
    ];

    const fn endpoint_target_variables(self) -> (u32, u32) {
        match self {
            Self::Identity => (2, 3),
            Self::Swap => (3, 2),
            Self::CollapseToX => (2, 2),
            Self::CollapseToY => (3, 3),
        }
    }
}

/// Replayable evidence for one restricted variable-image instantiation.  The
/// same canonical C-1 image inventory is applied separately to the owner,
/// the zero endpoint, and the one endpoint.  The resulting endpoint pair is
/// then checked by the typed constructor/PathP-method/eliminator/coe audit;
/// this deliberately includes the two degenerate endpoint pairs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TruncRestrictedVariableInstanceToken {
    kind: TruncRestrictedInstanceKind,
    images: Vec<SubstitutionImage>,
    owner_substitution: SortPreservingSubstitutionToken,
    zero_endpoint_substitution: SortPreservingSubstitutionToken,
    one_endpoint_substitution: SortPreservingSubstitutionToken,
    instantiated_owner: Expr,
    zero_target_variable: u32,
    one_target_variable: u32,
    zero_boundary_parameter: u16,
    one_boundary_parameter: u16,
    substitution_derivation_hash: String,
    premise_source_digest: String,
    premise_context_derivation_hash: String,
    computation_audit_derivation_hash: String,
    derivation_hash: String,
}

impl TruncRestrictedVariableInstanceToken {
    pub const fn kind(&self) -> TruncRestrictedInstanceKind {
        self.kind
    }

    pub fn images(&self) -> &[SubstitutionImage] {
        &self.images
    }

    pub fn instantiated_owner(&self) -> &Expr {
        &self.instantiated_owner
    }

    pub const fn zero_target_variable(&self) -> u32 {
        self.zero_target_variable
    }

    pub const fn one_target_variable(&self) -> u32 {
        self.one_target_variable
    }

    pub const fn zero_boundary_parameter(&self) -> u16 {
        self.zero_boundary_parameter
    }

    pub const fn one_boundary_parameter(&self) -> u16 {
        self.one_boundary_parameter
    }

    pub const fn has_degenerate_endpoints(&self) -> bool {
        self.zero_boundary_parameter == self.one_boundary_parameter
    }

    pub fn owner_substitution_derivation_hash(&self) -> &str {
        self.owner_substitution.derivation_hash()
    }

    pub fn zero_endpoint_substitution_derivation_hash(&self) -> &str {
        self.zero_endpoint_substitution.derivation_hash()
    }

    pub fn one_endpoint_substitution_derivation_hash(&self) -> &str {
        self.one_endpoint_substitution.derivation_hash()
    }

    pub fn substitution_derivation_hash(&self) -> &str {
        &self.substitution_derivation_hash
    }

    pub fn premise_source_digest(&self) -> &str {
        &self.premise_source_digest
    }

    pub fn premise_context_derivation_hash(&self) -> &str {
        &self.premise_context_derivation_hash
    }

    pub fn computation_audit_derivation_hash(&self) -> &str {
        &self.computation_audit_derivation_hash
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

fn trunc_parameter_context() -> SortedParameterContext {
    SortedParameterContext::new(vec![
        ParameterSort::Type,
        ParameterSort::Opaque,
        ParameterSort::Opaque,
    ])
}

fn trunc_restricted_images(kind: TruncRestrictedInstanceKind) -> Vec<SubstitutionImage> {
    let (zero_target, one_target) = kind.endpoint_target_variables();
    vec![
        SubstitutionImage {
            source_parameter: 1,
            term: Expr::Var(1),
        },
        SubstitutionImage {
            source_parameter: 2,
            term: Expr::Var(zero_target),
        },
        SubstitutionImage {
            source_parameter: 3,
            term: Expr::Var(one_target),
        },
    ]
}

fn endpoint_target(expression: &Expr) -> Result<(u32, u16), TypedBoundaryError> {
    match expression {
        Expr::Var(2) => Ok((2, 1)),
        Expr::Var(3) => Ok((3, 2)),
        _ => Err(TypedBoundaryError::TruncEndpointInventoryMismatch),
    }
}

/// Issue one Trunc instantiation from an explicit C-1 image inventory.  The
/// fixed source and target context makes this API fail closed: wrong-sort and
/// non-variable images are rejected by C-1 before cubical evidence is issued.
pub fn issue_trunc_restricted_variable_instance(
    typing: &FormedPathTyping,
    images: Vec<SubstitutionImage>,
) -> Result<TruncRestrictedVariableInstanceToken, TypedBoundaryError> {
    let expected_owner = Expr::Trunc(Box::new(Expr::Var(1)));
    if typing.dimension != 1 || typing.formation_normal_form != expected_owner {
        return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
    }

    let context = trunc_parameter_context();
    let issue = |body| {
        issue_sort_preserving_substitution(context.clone(), context.clone(), images.clone(), body)
            .map_err(|error| TypedBoundaryError::TruncRestrictedInstantiation {
                error: error.to_string(),
            })
    };
    let owner_substitution = issue(typing.formation_normal_form.clone())?;
    let zero_endpoint_substitution = issue(Expr::Var(2))?;
    let one_endpoint_substitution = issue(Expr::Var(3))?;
    for token in [
        &owner_substitution,
        &zero_endpoint_substitution,
        &one_endpoint_substitution,
    ] {
        replay_sort_preserving_substitution(token).map_err(|error| {
            TypedBoundaryError::TruncRestrictedInstantiation {
                error: error.to_string(),
            }
        })?;
    }

    let canonical_images = owner_substitution.structural().images().to_vec();
    if zero_endpoint_substitution.structural().images() != canonical_images
        || one_endpoint_substitution.structural().images() != canonical_images
        || owner_substitution.result() != &expected_owner
    {
        return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
    }
    let (zero_target_variable, zero_boundary_parameter) =
        endpoint_target(zero_endpoint_substitution.result())?;
    let (one_target_variable, one_boundary_parameter) =
        endpoint_target(one_endpoint_substitution.result())?;
    let kind = match (zero_target_variable, one_target_variable) {
        (2, 3) => TruncRestrictedInstanceKind::Identity,
        (3, 2) => TruncRestrictedInstanceKind::Swap,
        (2, 2) => TruncRestrictedInstanceKind::CollapseToX,
        (3, 3) => TruncRestrictedInstanceKind::CollapseToY,
        _ => return Err(TypedBoundaryError::TruncEndpointInventoryMismatch),
    };

    let instantiated_owner = owner_substitution.result().clone();
    let substitution_derivation_hash = trunc_endpoint_tagged_digest(
        "trunc-restricted-variable-image-substitutions",
        &(
            &canonical_images,
            &owner_substitution,
            &zero_endpoint_substitution,
            &one_endpoint_substitution,
        ),
    );
    let premise_source_digest = trunc_endpoint_tagged_digest(
        "trunc-restricted-variable-instance-premise-source",
        &(
            &typing.subject_hash,
            &typing.signature_digest,
            typing.visible_library,
            typing.kappa,
            typing.formation_clause,
            &typing.formation_normal_form,
            typing.path_clause,
            typing.dimension,
            &typing.elaboration_derivation_hash,
            kind,
            &canonical_images,
            &substitution_derivation_hash,
            zero_boundary_parameter,
            one_boundary_parameter,
        ),
    );
    let premise_context = issue_endpoint_schema_premise_context(
        typing,
        &premise_source_digest,
        zero_boundary_parameter,
        one_boundary_parameter,
    )
    .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
        error: error.to_string(),
    })?;
    let computation_audit = audit_endpoint_path_computation(
        typing,
        &premise_context,
        zero_boundary_parameter,
        one_boundary_parameter,
    )
    .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
        error: error.to_string(),
    })?;
    let premise_context_derivation_hash = premise_context.derivation_hash().to_owned();
    let computation_audit_derivation_hash = computation_audit.derivation_hash().to_owned();
    let derivation_hash = trunc_endpoint_tagged_digest(
        "trunc-restricted-variable-instance",
        &(
            kind,
            &canonical_images,
            &owner_substitution,
            &zero_endpoint_substitution,
            &one_endpoint_substitution,
            &instantiated_owner,
            zero_target_variable,
            one_target_variable,
            zero_boundary_parameter,
            one_boundary_parameter,
            &substitution_derivation_hash,
            &premise_source_digest,
            &premise_context_derivation_hash,
            &computation_audit_derivation_hash,
        ),
    );
    Ok(TruncRestrictedVariableInstanceToken {
        kind,
        images: canonical_images,
        owner_substitution,
        zero_endpoint_substitution,
        one_endpoint_substitution,
        instantiated_owner,
        zero_target_variable,
        one_target_variable,
        zero_boundary_parameter,
        one_boundary_parameter,
        substitution_derivation_hash,
        premise_source_digest,
        premise_context_derivation_hash,
        computation_audit_derivation_hash,
        derivation_hash,
    })
}

pub fn replay_trunc_restricted_variable_instance(
    typing: &FormedPathTyping,
    token: &TruncRestrictedVariableInstanceToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_trunc_restricted_variable_instance(typing, token.images.clone())?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalTypedHandoffReplayMismatch)
    }
}

/// Replayable evidence that all four and only the restricted Trunc endpoint
/// variable-image instances were computation-audited.  Arbitrary typed images
/// remain an explicitly named gap.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TruncRestrictedInstantiationToken {
    theorem_scope: String,
    arbitrary_typed_instance_gap: String,
    arbitrary_typed_images_used: bool,
    all_sort_identical_variable_images_covered: bool,
    variable_instances: Vec<TruncRestrictedVariableInstanceToken>,
    derivation_hash: String,
}

impl TruncRestrictedInstantiationToken {
    fn instance(&self, kind: TruncRestrictedInstanceKind) -> &TruncRestrictedVariableInstanceToken {
        self.variable_instances
            .iter()
            .find(|instance| instance.kind == kind)
            .expect("the issuer fixes the complete four-instance inventory")
    }

    pub fn theorem_scope(&self) -> &str {
        &self.theorem_scope
    }

    pub fn arbitrary_typed_instance_gap(&self) -> &str {
        &self.arbitrary_typed_instance_gap
    }

    pub const fn arbitrary_typed_images_used(&self) -> bool {
        self.arbitrary_typed_images_used
    }

    pub const fn all_sort_identical_variable_images_covered(&self) -> bool {
        self.all_sort_identical_variable_images_covered
    }

    pub fn variable_instances(&self) -> &[TruncRestrictedVariableInstanceToken] {
        &self.variable_instances
    }

    // These compatibility accessors preserve the public projection consumed
    // by the v1 sidecar.  The first three still denote the identity instance;
    // the swap projection now binds the same-image three-body derivation.
    pub fn owner_substitution_derivation_hash(&self) -> &str {
        self.instance(TruncRestrictedInstanceKind::Identity)
            .owner_substitution_derivation_hash()
    }

    pub fn zero_endpoint_substitution_derivation_hash(&self) -> &str {
        self.instance(TruncRestrictedInstanceKind::Identity)
            .zero_endpoint_substitution_derivation_hash()
    }

    pub fn one_endpoint_substitution_derivation_hash(&self) -> &str {
        self.instance(TruncRestrictedInstanceKind::Identity)
            .one_endpoint_substitution_derivation_hash()
    }

    pub fn endpoint_swap_substitution_derivation_hash(&self) -> &str {
        self.instance(TruncRestrictedInstanceKind::Swap)
            .substitution_derivation_hash()
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_trunc_restricted_instantiation(
    typing: &FormedPathTyping,
) -> Result<TruncRestrictedInstantiationToken, TypedBoundaryError> {
    let mut variable_instances = Vec::with_capacity(TruncRestrictedInstanceKind::ALL.len());
    for kind in TruncRestrictedInstanceKind::ALL {
        let instance =
            issue_trunc_restricted_variable_instance(typing, trunc_restricted_images(kind))?;
        if instance.kind() != kind {
            return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
        }
        replay_trunc_restricted_variable_instance(typing, &instance)?;
        variable_instances.push(instance);
    }
    let observed = variable_instances
        .iter()
        .map(|instance| {
            (
                instance.kind(),
                instance.zero_boundary_parameter(),
                instance.one_boundary_parameter(),
            )
        })
        .collect::<Vec<_>>();
    if observed
        != vec![
            (TruncRestrictedInstanceKind::Identity, 1, 2),
            (TruncRestrictedInstanceKind::Swap, 2, 1),
            (TruncRestrictedInstanceKind::CollapseToX, 1, 1),
            (TruncRestrictedInstanceKind::CollapseToY, 2, 2),
        ]
    {
        return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
    }

    let theorem_scope = SORT_PRESERVATION_SCOPE.to_owned();
    let arbitrary_typed_instance_gap = C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION_GAP.to_owned();
    let arbitrary_typed_images_used = false;
    let all_sort_identical_variable_images_covered = true;
    let derivation_hash = trunc_endpoint_tagged_digest(
        "trunc-restricted-parameter-instantiation",
        &(
            &theorem_scope,
            &arbitrary_typed_instance_gap,
            arbitrary_typed_images_used,
            all_sort_identical_variable_images_covered,
            &variable_instances,
        ),
    );
    Ok(TruncRestrictedInstantiationToken {
        theorem_scope,
        arbitrary_typed_instance_gap,
        arbitrary_typed_images_used,
        all_sort_identical_variable_images_covered,
        variable_instances,
        derivation_hash,
    })
}

pub fn replay_trunc_restricted_instantiation(
    typing: &FormedPathTyping,
    token: &TruncRestrictedInstantiationToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_trunc_restricted_instantiation(typing)?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalTypedHandoffReplayMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TruncEndpointFaceBinding {
    face: BoundaryFaceKey,
    parameter_index: u16,
    parameter_name: String,
    parameter_context_digest: String,
    boundary_term_normal_form_digest: String,
    ty: BoundaryParameterType,
    typing_witness: BoundaryTypingWitness,
}

impl TruncEndpointFaceBinding {
    pub const fn face(&self) -> BoundaryFaceKey {
        self.face
    }

    pub const fn parameter_index(&self) -> u16 {
        self.parameter_index
    }

    pub fn parameter_name(&self) -> &str {
        &self.parameter_name
    }

    pub fn parameter_context_digest(&self) -> &str {
        &self.parameter_context_digest
    }

    pub fn boundary_term_normal_form_digest(&self) -> &str {
        &self.boundary_term_normal_form_digest
    }

    pub fn ty(&self) -> &BoundaryParameterType {
        &self.ty
    }
}

fn trunc_endpoint_face_bindings(
    typed: &TypedDeclaredBoundaryToken,
) -> Result<Vec<TruncEndpointFaceBinding>, TypedBoundaryError> {
    if typed.kind != RegisteredBoundaryKind::Trunc
        || typed.dimension != 1
        || typed.faces.len() != 2
        || typed.parameter_context.bindings.len() != 3
    {
        return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
    }
    let mut bindings = Vec::with_capacity(2);
    for face in &typed.faces {
        if face.fixed.axis != 0 || !face.restrictions.is_empty() {
            return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
        }
        let expected_index = if face.fixed.endpoint { 2 } else { 1 };
        let expected_name = if face.fixed.endpoint { "y" } else { "x" };
        let ResolvedBoundaryRef::ContextVariable(reference) = &face.term.source else {
            return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
        };
        let BoundaryTypingWitness::ContextLookup {
            context_digest,
            binding_index,
        } = &face.term.witness
        else {
            return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
        };
        if reference.index != expected_index
            || reference.name != expected_name
            || reference.context_digest != typed.parameter_context.context_digest
            || reference.ty
                != (BoundaryParameterType::Element {
                    owner: typed.owner.clone(),
                })
            || binding_index != &expected_index
            || context_digest != &reference.context_digest
        {
            return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
        }
        bindings.push(TruncEndpointFaceBinding {
            face: face.fixed,
            parameter_index: reference.index,
            parameter_name: reference.name.clone(),
            parameter_context_digest: reference.context_digest.clone(),
            boundary_term_normal_form_digest: face.term.normal_form_digest.clone(),
            ty: reference.ty.clone(),
            typing_witness: face.term.witness.clone(),
        });
    }
    bindings.sort_by_key(|binding| binding.face);
    if bindings[0].face != BoundaryFaceKey::new(0, false)
        || bindings[1].face != BoundaryFaceKey::new(0, true)
    {
        return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
    }
    Ok(bindings)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TruncEndpointSourceScope {
    FullH15,
    HistoricalPrefixB5,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TruncEndpointRealizationAudit {
    key: PathSchemaKey,
    term_hash: String,
    normal_form_hash: String,
    type_hash: String,
    derivation_hash: String,
}

impl TruncEndpointRealizationAudit {
    pub fn key(&self) -> &PathSchemaKey {
        &self.key
    }

    pub fn term_hash(&self) -> &str {
        &self.term_hash
    }

    pub fn normal_form_hash(&self) -> &str {
        &self.normal_form_hash
    }

    pub fn type_hash(&self) -> &str {
        &self.type_hash
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Opaque registered-Trunc computation certificate.  It is issued only after
/// replaying the exact v3 typed boundary, its two context-variable faces,
/// the restricted C-1 instantiation, PathP computation, and both basis keys.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TruncEndpointV3C6BundleToken {
    bundle_version: String,
    cubical_fragment_version: String,
    axiom_version: String,
    source_scope: TruncEndpointSourceScope,
    signature_digest: String,
    candidate_hash: String,
    step: u32,
    dimension: u32,
    telescope_digest: String,
    typing_derivation_hash: String,
    typed_boundary_derivation_hash: String,
    parameter_context_digest: String,
    endpoint_faces: Vec<TruncEndpointFaceBinding>,
    boundary_binding_digest: String,
    restricted_instantiation: TruncRestrictedInstantiationToken,
    endpoint_premise_source_digest: String,
    endpoint_premise_context_derivation_hash: String,
    computation_audit_derivation_hash: String,
    boundary_attachment_derivation_hash: String,
    boundary_basis_presentation_derivation_hash: String,
    method_shape_digest: String,
    key_correspondence: Vec<BoundaryBasisKeyCorrespondence>,
    key_bijection_digest: String,
    path_basis_derivation_hash: String,
    path_realizations: Vec<TruncEndpointRealizationAudit>,
    expected_basis_count: u64,
    realized_basis_count: u64,
    obstruction_retired: String,
    general_c6_proved: bool,
    derivation_hash: String,
}

impl TruncEndpointV3C6BundleToken {
    pub fn bundle_version(&self) -> &str {
        &self.bundle_version
    }

    pub const fn source_scope(&self) -> TruncEndpointSourceScope {
        self.source_scope
    }

    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }

    pub fn candidate_hash(&self) -> &str {
        &self.candidate_hash
    }

    pub const fn step(&self) -> u32 {
        self.step
    }

    pub const fn dimension(&self) -> u32 {
        self.dimension
    }

    pub fn telescope_digest(&self) -> &str {
        &self.telescope_digest
    }

    pub fn typing_derivation_hash(&self) -> &str {
        &self.typing_derivation_hash
    }

    pub fn typed_boundary_derivation_hash(&self) -> &str {
        &self.typed_boundary_derivation_hash
    }

    pub fn parameter_context_digest(&self) -> &str {
        &self.parameter_context_digest
    }

    pub fn boundary_binding_digest(&self) -> &str {
        &self.boundary_binding_digest
    }

    pub fn endpoint_faces(&self) -> &[TruncEndpointFaceBinding] {
        &self.endpoint_faces
    }

    pub fn restricted_instantiation(&self) -> &TruncRestrictedInstantiationToken {
        &self.restricted_instantiation
    }

    pub fn endpoint_premise_source_digest(&self) -> &str {
        &self.endpoint_premise_source_digest
    }

    pub fn endpoint_premise_context_derivation_hash(&self) -> &str {
        &self.endpoint_premise_context_derivation_hash
    }

    pub fn computation_audit_derivation_hash(&self) -> &str {
        &self.computation_audit_derivation_hash
    }

    pub fn boundary_attachment_derivation_hash(&self) -> &str {
        &self.boundary_attachment_derivation_hash
    }

    pub fn boundary_basis_presentation_derivation_hash(&self) -> &str {
        &self.boundary_basis_presentation_derivation_hash
    }

    pub fn method_shape_digest(&self) -> &str {
        &self.method_shape_digest
    }

    pub fn key_correspondence(&self) -> &[BoundaryBasisKeyCorrespondence] {
        &self.key_correspondence
    }

    pub fn key_bijection_digest(&self) -> &str {
        &self.key_bijection_digest
    }

    pub fn path_basis_derivation_hash(&self) -> &str {
        &self.path_basis_derivation_hash
    }

    pub fn path_realizations(&self) -> &[TruncEndpointRealizationAudit] {
        &self.path_realizations
    }

    pub const fn expected_basis_count(&self) -> u64 {
        self.expected_basis_count
    }

    pub const fn realized_basis_count(&self) -> u64 {
        self.realized_basis_count
    }

    pub fn obstruction_retired(&self) -> &str {
        &self.obstruction_retired
    }

    pub const fn general_c6_proved(&self) -> bool {
        self.general_c6_proved
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

fn assemble_trunc_endpoint_v3_c6_bundle(
    signature: &SealedSignature,
    source: RegisteredSource,
    typed: TypedDeclaredBoundaryToken,
    source_scope: TruncEndpointSourceScope,
) -> Result<TruncEndpointV3C6BundleToken, TypedBoundaryError> {
    if source.kind != RegisteredBoundaryKind::Trunc || source.typing.dimension != 1 {
        return Err(TypedBoundaryError::TruncEndpointInventoryMismatch);
    }
    let endpoint_faces = trunc_endpoint_face_bindings(&typed)?;
    let zero_parameter = endpoint_faces[0].parameter_index;
    let one_parameter = endpoint_faces[1].parameter_index;
    let boundary_binding_digest = trunc_endpoint_tagged_digest(
        "registered-trunc-declared-endpoint-binding",
        &(
            typed.derivation_hash(),
            typed.parameter_context.context_digest(),
            &endpoint_faces,
        ),
    );
    let restricted_instantiation = issue_trunc_restricted_instantiation(&source.typing)?;
    replay_trunc_restricted_instantiation(&source.typing, &restricted_instantiation)?;
    let endpoint_premise_source_digest = endpoint_source_digest(
        &source.typing,
        &boundary_binding_digest,
        restricted_instantiation.derivation_hash(),
        zero_parameter,
        one_parameter,
    );
    let endpoint_premise_context = issue_endpoint_schema_premise_context(
        &source.typing,
        &endpoint_premise_source_digest,
        zero_parameter,
        one_parameter,
    )
    .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
        error: error.to_string(),
    })?;
    let endpoint_premise_context_derivation_hash =
        endpoint_premise_context.derivation_hash().to_owned();
    let computation_audit = audit_endpoint_path_computation(
        &source.typing,
        &endpoint_premise_context,
        zero_parameter,
        one_parameter,
    )
    .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
        error: error.to_string(),
    })?;

    let map = canonical_diagram_for_source(&source)?;
    let checked_boundary = check_declared_boundary_diagram(map)?;
    let attachment = issue_boundary_attachment(
        signature,
        &source.telescope,
        source.kind.step() - 1,
        &source.typing,
        BoundaryTheory::V2DeclaredBoundary,
        &checked_boundary,
    )?;
    replay_boundary_attachment(
        signature,
        &source.telescope,
        source.kind.step() - 1,
        &source.typing,
        &checked_boundary,
        &attachment,
    )?;
    let presentation = present_boundary_basis(&attachment)?;
    let basis = realize_endpoint_path_basis(
        signature,
        &source.telescope,
        source.kind.step() - 1,
        &source.typing,
        &boundary_binding_digest,
        restricted_instantiation.derivation_hash(),
        zero_parameter,
        one_parameter,
        &endpoint_premise_context,
    )
    .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
        error: error.to_string(),
    })?;
    for token in basis.tokens() {
        replay_endpoint_path_realization(
            signature,
            &source.telescope,
            source.kind.step() - 1,
            &source.typing,
            &boundary_binding_digest,
            restricted_instantiation.derivation_hash(),
            zero_parameter,
            one_parameter,
            &endpoint_premise_context,
            token,
        )
        .map_err(|error| TypedBoundaryError::C6PathBasisRealization {
            error: error.to_string(),
        })?;
    }
    if presentation.formula() != BoundaryBasisFormula::OnePlusDimensionSquared
        || presentation.total_count() != 2
        || basis.tokens().len() != 2
    {
        return Err(TypedBoundaryError::C6PathBasisCountMismatch);
    }
    let key_correspondence = vec![
        BoundaryBasisKeyCorrespondence {
            boundary_key: BoundaryBasisKey::Beta,
            path_key: PathSchemaKey::Beta,
        },
        BoundaryBasisKeyCorrespondence {
            boundary_key: BoundaryBasisKey::PrincipalTransport { principal: 0 },
            path_key: PathSchemaKey::Kan {
                principal: 0,
                probe: 0,
            },
        },
    ];
    let mapped_boundary_keys = key_correspondence
        .iter()
        .map(|entry| entry.boundary_key.clone())
        .collect::<Vec<_>>();
    let realized_path_keys = basis
        .tokens()
        .iter()
        .map(|token| token.key().clone())
        .collect::<Vec<_>>();
    let mapped_path_keys = key_correspondence
        .iter()
        .map(|entry| entry.path_key.clone())
        .collect::<Vec<_>>();
    if presentation.keys() != mapped_boundary_keys.as_slice()
        || realized_path_keys != mapped_path_keys
    {
        return Err(TypedBoundaryError::C6BoundaryBasisBijectionMismatch);
    }
    let key_bijection_digest = trunc_endpoint_tagged_digest(
        "trunc-endpoint-boundary-path-key-bijection",
        &key_correspondence,
    );
    let path_realizations = basis
        .tokens()
        .iter()
        .map(|token| TruncEndpointRealizationAudit {
            key: token.key().clone(),
            term_hash: token.term_hash().to_owned(),
            normal_form_hash: token.normal_form_hash().to_owned(),
            type_hash: token.type_hash().to_owned(),
            derivation_hash: token.derivation_hash().to_owned(),
        })
        .collect::<Vec<_>>();

    let bundle_version = TRUNC_ENDPOINT_V3_C6_BUNDLE_VERSION.to_owned();
    let cubical_fragment_version = TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION.to_owned();
    let axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned();
    let signature_digest = signature.digest().to_owned();
    let candidate_hash = source.candidate_hash;
    let step = source.kind.step();
    let dimension = source.kind.dimension();
    let telescope_digest =
        trunc_endpoint_tagged_digest("trunc-sealed-telescope", &source.telescope);
    let typing_derivation_hash = source.typing.elaboration_derivation_hash;
    let typed_boundary_derivation_hash = typed.derivation_hash;
    let parameter_context_digest = typed.parameter_context.context_digest;
    let computation_audit_derivation_hash = computation_audit.derivation_hash().to_owned();
    let boundary_attachment_derivation_hash = attachment.derivation_hash().to_owned();
    let boundary_basis_presentation_derivation_hash = presentation.derivation_hash().to_owned();
    let method_shape_digest = basis.method_shape_digest().to_owned();
    let path_basis_derivation_hash = basis.derivation_hash().to_owned();
    let expected_basis_count = 2;
    let realized_basis_count = path_realizations.len() as u64;
    let obstruction_retired = C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP.to_owned();
    let general_c6_proved = false;
    let derivation_hash = trunc_endpoint_tagged_digest(
        "trunc-endpoint-v3-c6-bundle",
        &(
            (
                &bundle_version,
                &cubical_fragment_version,
                &axiom_version,
                source_scope,
                &signature_digest,
                &candidate_hash,
                step,
                dimension,
            ),
            (
                &telescope_digest,
                &typing_derivation_hash,
                &typed_boundary_derivation_hash,
                &parameter_context_digest,
                &endpoint_faces,
                &boundary_binding_digest,
                &restricted_instantiation,
                &endpoint_premise_source_digest,
                &endpoint_premise_context_derivation_hash,
                &computation_audit_derivation_hash,
            ),
            (
                &boundary_attachment_derivation_hash,
                &boundary_basis_presentation_derivation_hash,
                &method_shape_digest,
                &key_correspondence,
                &key_bijection_digest,
                &path_basis_derivation_hash,
                &path_realizations,
                expected_basis_count,
                realized_basis_count,
            ),
            (&obstruction_retired, general_c6_proved),
        ),
    );
    Ok(TruncEndpointV3C6BundleToken {
        bundle_version,
        cubical_fragment_version,
        axiom_version,
        source_scope,
        signature_digest,
        candidate_hash,
        step,
        dimension,
        telescope_digest,
        typing_derivation_hash,
        typed_boundary_derivation_hash,
        parameter_context_digest,
        endpoint_faces,
        boundary_binding_digest,
        restricted_instantiation,
        endpoint_premise_source_digest,
        endpoint_premise_context_derivation_hash,
        computation_audit_derivation_hash,
        boundary_attachment_derivation_hash,
        boundary_basis_presentation_derivation_hash,
        method_shape_digest,
        key_correspondence,
        key_bijection_digest,
        path_basis_derivation_hash,
        path_realizations,
        expected_basis_count,
        realized_basis_count,
        obstruction_retired,
        general_c6_proved,
        derivation_hash,
    })
}

pub fn issue_trunc_endpoint_v3_c6_bundle_token(
    signature: &SealedSignature,
) -> Result<TruncEndpointV3C6BundleToken, TypedBoundaryError> {
    let source = registered_source(signature, RegisteredBoundaryKind::Trunc)?;
    let map = canonical_diagram_for_source(&source)?;
    let typed =
        issue_typed_declared_boundary_token_v3(signature, RegisteredBoundaryKind::Trunc, map)?;
    replay_typed_declared_boundary_token_v3(signature, &typed)?;
    assemble_trunc_endpoint_v3_c6_bundle(
        signature,
        source,
        typed,
        TruncEndpointSourceScope::FullH15,
    )
}

pub fn replay_trunc_endpoint_v3_c6_bundle_token(
    signature: &SealedSignature,
    token: &TruncEndpointV3C6BundleToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_trunc_endpoint_v3_c6_bundle_token(signature)?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::TruncEndpointBundleReplayMismatch)
    }
}

pub fn issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
) -> Result<TruncEndpointV3C6BundleToken, TypedBoundaryError> {
    let source = registered_source_for_historical_prefix(
        predecessor_signature,
        RegisteredBoundaryKind::Trunc,
        current_telescope,
    )?;
    let map = canonical_diagram_for_source(&source)?;
    let typed = issue_typed_declared_boundary_token_v3_for_historical_prefix(
        predecessor_signature,
        RegisteredBoundaryKind::Trunc,
        current_telescope,
        map,
    )?;
    replay_typed_declared_boundary_token_v3_for_historical_prefix(
        predecessor_signature,
        current_telescope,
        &typed,
    )?;
    assemble_trunc_endpoint_v3_c6_bundle(
        predecessor_signature,
        source,
        typed,
        TruncEndpointSourceScope::HistoricalPrefixB5,
    )
}

pub fn replay_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &TruncEndpointV3C6BundleToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
        predecessor_signature,
        current_telescope,
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::TruncEndpointBundleReplayMismatch)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalTypedBundleEvidenceKind {
    ArchivalConstantBridge,
    TruncEndpointDependentV1,
}

/// Uniform Agent-A-facing successor wrapper.  The old bridge remains
/// archival and still reports the Trunc gap; this API is the explicit
/// successor that covers all four packages.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalPrefixV3C6TypedBundleToken {
    handoff_version: String,
    evidence_kind: HistoricalTypedBundleEvidenceKind,
    signature_digest: String,
    kind: RegisteredBoundaryKind,
    step: u32,
    dimension: u32,
    typed_boundary_derivation_hash: String,
    key_correspondence: Vec<BoundaryBasisKeyCorrespondence>,
    path_realization_derivation_hashes: Vec<String>,
    expected_basis_count: u64,
    realized_basis_count: u64,
    child_derivation_hash: String,
    derivation_hash: String,
}

impl HistoricalPrefixV3C6TypedBundleToken {
    pub const fn evidence_kind(&self) -> HistoricalTypedBundleEvidenceKind {
        self.evidence_kind
    }

    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }

    pub const fn kind(&self) -> RegisteredBoundaryKind {
        self.kind
    }

    pub const fn step(&self) -> u32 {
        self.step
    }

    pub const fn dimension(&self) -> u32 {
        self.dimension
    }

    pub fn typed_boundary_derivation_hash(&self) -> &str {
        &self.typed_boundary_derivation_hash
    }

    pub fn key_correspondence(&self) -> &[BoundaryBasisKeyCorrespondence] {
        &self.key_correspondence
    }

    pub fn path_realization_derivation_hashes(&self) -> &[String] {
        &self.path_realization_derivation_hashes
    }

    pub const fn expected_basis_count(&self) -> u64 {
        self.expected_basis_count
    }

    pub const fn realized_basis_count(&self) -> u64 {
        self.realized_basis_count
    }

    pub fn child_derivation_hash(&self) -> &str {
        &self.child_derivation_hash
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_historical_prefix_v3_c6_typed_bundle_token(
    predecessor_signature: &SealedSignature,
    kind: RegisteredBoundaryKind,
    current_telescope: &Telescope,
) -> Result<HistoricalPrefixV3C6TypedBundleToken, TypedBoundaryError> {
    let (
        evidence_kind,
        typed_boundary_derivation_hash,
        key_correspondence,
        path_realization_derivation_hashes,
        expected_basis_count,
        realized_basis_count,
        child_derivation_hash,
    ) = if kind == RegisteredBoundaryKind::Trunc {
        let child = issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
            predecessor_signature,
            current_telescope,
        )?;
        replay_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
            predecessor_signature,
            current_telescope,
            &child,
        )?;
        (
            HistoricalTypedBundleEvidenceKind::TruncEndpointDependentV1,
            child.typed_boundary_derivation_hash.clone(),
            child.key_correspondence.clone(),
            child
                .path_realizations
                .iter()
                .map(|audit| audit.derivation_hash.clone())
                .collect(),
            child.expected_basis_count,
            child.realized_basis_count,
            child.derivation_hash.clone(),
        )
    } else {
        let child = issue_historical_prefix_v3_c6_bridge_token(
            predecessor_signature,
            kind,
            current_telescope,
        )?;
        replay_historical_prefix_v3_c6_bridge_token(
            predecessor_signature,
            current_telescope,
            &child,
        )?;
        (
            HistoricalTypedBundleEvidenceKind::ArchivalConstantBridge,
            child.typed_boundary_derivation_hash.clone(),
            child.key_correspondence.clone(),
            child.path_realization_derivation_hashes.clone(),
            child.expected_basis_count,
            child.realized_basis_count,
            child.derivation_hash.clone(),
        )
    };
    let handoff_version = HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION.to_owned();
    let signature_digest = predecessor_signature.digest().to_owned();
    let step = kind.step();
    let dimension = kind.dimension();
    let derivation_hash = trunc_endpoint_tagged_digest(
        "historical-prefix-v3-c6-typed-bundle-handoff",
        &(
            &handoff_version,
            evidence_kind,
            &signature_digest,
            kind,
            step,
            dimension,
            &typed_boundary_derivation_hash,
            &key_correspondence,
            &path_realization_derivation_hashes,
            expected_basis_count,
            realized_basis_count,
            &child_derivation_hash,
        ),
    );
    Ok(HistoricalPrefixV3C6TypedBundleToken {
        handoff_version,
        evidence_kind,
        signature_digest,
        kind,
        step,
        dimension,
        typed_boundary_derivation_hash,
        key_correspondence,
        path_realization_derivation_hashes,
        expected_basis_count,
        realized_basis_count,
        child_derivation_hash,
        derivation_hash,
    })
}

pub fn replay_historical_prefix_v3_c6_typed_bundle_token(
    predecessor_signature: &SealedSignature,
    current_telescope: &Telescope,
    token: &HistoricalPrefixV3C6TypedBundleToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_prefix_v3_c6_typed_bundle_token(
        predecessor_signature,
        token.kind,
        current_telescope,
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalTypedHandoffReplayMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalV3C6TypedHandoffToken {
    handoff_version: String,
    bundles: Vec<HistoricalPrefixV3C6TypedBundleToken>,
    registered_counts: Vec<u64>,
    all_replayed: bool,
    derivation_hash: String,
}

impl HistoricalV3C6TypedHandoffToken {
    pub fn bundles(&self) -> &[HistoricalPrefixV3C6TypedBundleToken] {
        &self.bundles
    }

    pub fn registered_counts(&self) -> &[u64] {
        &self.registered_counts
    }

    pub const fn all_replayed(&self) -> bool {
        self.all_replayed
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_historical_v3_c6_typed_handoff_token()
-> Result<HistoricalV3C6TypedHandoffToken, TypedBoundaryError> {
    let mut bundles = Vec::with_capacity(4);
    for kind in RegisteredBoundaryKind::ALL {
        let prefix = SealedSignature::from_telescopes(
            (1..kind.step())
                .map(|step| (step, Telescope::reference(step)))
                .collect(),
        );
        let current = Telescope::reference(kind.step());
        let bundle = issue_historical_prefix_v3_c6_typed_bundle_token(&prefix, kind, &current)?;
        replay_historical_prefix_v3_c6_typed_bundle_token(&prefix, &current, &bundle)?;
        bundles.push(bundle);
    }
    let registered_counts = bundles
        .iter()
        .map(|bundle| bundle.realized_basis_count)
        .collect::<Vec<_>>();
    if registered_counts != [2, 2, 5, 10] {
        return Err(TypedBoundaryError::C6PathBasisCountMismatch);
    }
    let handoff_version = HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION.to_owned();
    let all_replayed = true;
    let derivation_hash = trunc_endpoint_tagged_digest(
        "historical-v3-c6-all-four-typed-handoff",
        &(&handoff_version, &bundles, &registered_counts, all_replayed),
    );
    Ok(HistoricalV3C6TypedHandoffToken {
        handoff_version,
        bundles,
        registered_counts,
        all_replayed,
        derivation_hash,
    })
}

pub fn replay_historical_v3_c6_typed_handoff_token(
    token: &HistoricalV3C6TypedHandoffToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_historical_v3_c6_typed_handoff_token()?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::HistoricalTypedHandoffReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::super::boundary_variants::generic_declared_boundary;
    use super::*;
    use pen_core::clause::ClauseRec;

    fn signature() -> SealedSignature {
        SealedSignature::genesis_del_h15()
    }

    fn predecessor_prefix(step: u32) -> SealedSignature {
        SealedSignature::from_telescopes(
            (1..step)
                .map(|prior| (prior, Telescope::reference(prior)))
                .collect(),
        )
    }

    fn alternate_stage4_prefix() -> SealedSignature {
        let alternate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                ))),
            ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(
                    Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(3)))),
                    Box::new(Expr::Var(2)),
                ),
            ),
            ClauseRec::new(
                ClauseRole::Elimination,
                Expr::App(
                    Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Var(2)),
                ),
            ),
        ]);
        SealedSignature::from_telescopes(
            (1..=7)
                .map(|step| {
                    (
                        step,
                        if step == 4 {
                            alternate.clone()
                        } else {
                            Telescope::reference(step)
                        },
                    )
                })
                .collect(),
        )
    }

    #[test]
    fn adopted_versions_are_pinned_verbatim() {
        assert_eq!(
            ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION,
            "pathcon-attachment-declared-bound-boundary-theory-axiom-v2"
        );
        assert_eq!(
            BOUNDARY_CHARGE_POLICY_VERSION,
            "boundary-charge-zero-reference-only-v1"
        );
        assert_eq!(
            ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION,
            "pathcon-attachment-declared-bound-boundary-theory-axiom-v3-element-overlay"
        );
        assert_eq!(
            HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION,
            "historical-element-declaration-overlay-v1"
        );
        assert!(REFERENCE_ONLY_HOISTING_RULE.contains("must be hoisted"));
    }

    #[test]
    fn v3_overlay_registry_is_exactly_steps_5_7_8_clause_1() {
        let entries = registered_historical_element_overlay_entries();
        assert_eq!(
            entries.map(|entry| (
                entry.kind(),
                entry.step(),
                entry.formation_clause(),
                entry.element_clause()
            )),
            [
                (RegisteredBoundaryKind::S1, 5, 0, 1),
                (RegisteredBoundaryKind::S2, 7, 0, 1),
                (RegisteredBoundaryKind::S3, 8, 0, 1),
            ]
        );
        assert_eq!(
            issue_historical_element_overlay_token(&signature(), RegisteredBoundaryKind::Trunc),
            Err(TypedBoundaryError::HistoricalElementOverlayNotRegistered {
                kind: RegisteredBoundaryKind::Trunc,
            })
        );
    }

    #[test]
    fn all_registered_diagrams_have_replayable_reference_only_zero_charge() {
        let signature = signature();
        for kind in RegisteredBoundaryKind::ALL {
            let map = registered_boundary_diagram(&signature, kind).expect("registered map");
            let token = issue_reference_only_charge_token(&signature, kind, map.clone())
                .expect("registered references resolve");
            assert_eq!(token.kind(), kind);
            assert_eq!(token.charging_policy(), BOUNDARY_CHARGE_POLICY_VERSION);
            assert_eq!(token.hoisting_rule(), REFERENCE_ONLY_HOISTING_RULE);
            assert_eq!(token.boundary_credit(), 0);
            assert_eq!(
                token.resolved_faces().len(),
                (2 * kind.dimension()) as usize
            );
            replay_reference_only_charge_token(&signature, &token).expect("reference replay");

            let basis = issue_adopted_boundary_basis_charge_token(&signature, kind, map)
                .expect("adopted basis formula is joined to zero charge");
            assert_eq!(basis.kind(), kind);
            assert_eq!(basis.dimension(), kind.dimension());
            assert_eq!(
                basis.formula(),
                BoundaryBasisFormula::OnePlusDimensionSquared
            );
            assert_eq!(basis.boundary_credit(), 0);
            assert_eq!(
                basis.basis_count(),
                1 + u64::from(kind.dimension()) * u64::from(kind.dimension())
            );
            assert_eq!(basis.total_with_boundary(), basis.basis_count());
            assert_eq!(basis.charging_policy(), BOUNDARY_CHARGE_POLICY_VERSION);
            replay_adopted_boundary_basis_charge_token(&signature, &basis)
                .expect("basis/charge replay");
        }
    }

    #[test]
    fn archival_v2_derivation_hashes_are_pinned_across_the_v3_addition() {
        let signature = signature();
        for kind in RegisteredBoundaryKind::ALL {
            let map = registered_boundary_diagram(&signature, kind).expect("registered map");
            let reference = issue_reference_only_charge_token(&signature, kind, map.clone())
                .expect("V2 reference token");
            let basis = issue_adopted_boundary_basis_charge_token(&signature, kind, map)
                .expect("V2 basis token");
            let (expected_reference, expected_basis) = match kind {
                RegisteredBoundaryKind::S1 => (
                    "blake3:14553e160a3daf841bf85cdab3f491e600286dcd944223152cff156f029c5baf",
                    "blake3:3deede85f8bf3a530989490a76da765341ee298b178dec6971a9822f09aa480d",
                ),
                RegisteredBoundaryKind::Trunc => (
                    "blake3:2abdb6bb7a834c039721bd3a13c57d23929446eced113fec34bc8c1a1fe14acc",
                    "blake3:2264fa85d67d406f660cf0a241b9bb1a1869cc8fe90a1fc72c40bbb49fe243e6",
                ),
                RegisteredBoundaryKind::S2 => (
                    "blake3:ae6a58ad107157914a3b110c45e1131634d5f5fc57288a8850ac1425d7b9bbaa",
                    "blake3:22bc0bda5518e2f8a6adae520970c7e3d43976dd98e4cf3420eef803b3fbbc3e",
                ),
                RegisteredBoundaryKind::S3 => (
                    "blake3:7347676469ba1d3831913e1fc754b1d88621eb7740cadcccfe7733b180b2570f",
                    "blake3:ff323cdaf4eb58f34ae96aeeb0f931d320f4939d39bc32ffb30c5ac6d9776f42",
                ),
            };
            assert_eq!(reference.derivation_hash(), expected_reference);
            assert_eq!(basis.derivation_hash(), expected_basis);
        }
        let trunc = issue_typed_declared_boundary_token(
            &signature,
            RegisteredBoundaryKind::Trunc,
            registered_boundary_diagram(&signature, RegisteredBoundaryKind::Trunc)
                .expect("Trunc map"),
        )
        .expect("V2 Trunc token");
        assert_eq!(
            trunc.derivation_hash(),
            "blake3:f34fd0d2660bb44b2f699cae6e0babb2f7bf9c9cd496c6abe616d99934d0b055"
        );
    }

    #[test]
    fn v3_operational_role_is_positional_name_blind_and_fails_closed() {
        let signature = signature();
        let source =
            registered_source(&signature, RegisteredBoundaryKind::S2).expect("sealed S2 source");
        let entry = registered_element_overlay_entry(RegisteredBoundaryKind::S2)
            .expect("registered overlay entry");
        let map = canonical_diagram_for_source(&source).expect("registered map");
        let expected = operational_role_projection(&source, entry, &map)
            .expect("operational role from positions");

        let mut relabelled = map.clone();
        for (index, face) in relabelled.faces.iter_mut().enumerate() {
            face.term.name = format!("irrelevant-{index}");
            for (_, restriction_name) in &mut face.term.restrictions {
                *restriction_name = format!("also-irrelevant-{index}");
            }
        }
        assert_eq!(
            operational_role_projection(&source, entry, &relabelled),
            Ok(expected)
        );

        let mut wrong_binding = map;
        wrong_binding.faces[0].term.origin =
            BoundaryTermOrigin::HistoricalPointClause { clause: 0 };
        assert_eq!(
            operational_role_projection(&source, entry, &wrong_binding),
            Err(TypedBoundaryError::OperationalRoleWitnessFailed {
                kind: RegisteredBoundaryKind::S2,
                reason: OperationalRoleFailure::RegisteredBoundaryDoesNotBindElement,
            })
        );

        let mut wrong_role =
            registered_source(&signature, RegisteredBoundaryKind::S2).expect("sealed S2 source");
        wrong_role.telescope.clauses[1].role = ClauseRole::Formation;
        assert_eq!(
            operational_role_projection(
                &wrong_role,
                entry,
                &canonical_diagram_for_source(&wrong_role).expect("shape remains constructible"),
            ),
            Err(TypedBoundaryError::OperationalRoleWitnessFailed {
                kind: RegisteredBoundaryKind::S2,
                reason: OperationalRoleFailure::ElementRoleMismatch,
            })
        );
    }

    #[test]
    fn trunc_registered_context_and_terms_are_typed_and_replayed() {
        let signature = signature();
        let map = registered_boundary_diagram(&signature, RegisteredBoundaryKind::Trunc)
            .expect("Trunc map");
        let token =
            issue_typed_declared_boundary_token(&signature, RegisteredBoundaryKind::Trunc, map)
                .expect("Trunc endpoints are typed context variables");
        assert_eq!(token.dimension(), 1);
        assert_eq!(token.parameter_context().bindings().len(), 3);
        assert_eq!(token.parameter_context().bindings()[0].name(), "A");
        assert_eq!(
            token.parameter_context().bindings()[0].ty(),
            &BoundaryParameterType::Type
        );
        assert_eq!(token.faces().len(), 2);
        assert!(token.faces().iter().all(|face| matches!(
            face.term().source(),
            ResolvedBoundaryRef::ContextVariable(_)
        )));
        replay_typed_declared_boundary_token(&signature, &token).expect("typed replay");
    }

    #[test]
    fn v3_overlay_constructs_real_base_bindings_and_typed_boundaries() {
        let signature = signature();
        let signature_before = signature.clone();
        let full_signature_digest_before = signature.digest().to_owned();
        for kind in [
            RegisteredBoundaryKind::S1,
            RegisteredBoundaryKind::S2,
            RegisteredBoundaryKind::S3,
        ] {
            let telescope_before = Telescope::reference(kind.step());
            let kappa_before = telescope_before.kappa();
            let candidate_hash_before = signature
                .entry(kind.step())
                .expect("registered step")
                .candidate_hash
                .clone();
            let source = registered_source(&signature, kind).expect("registered source");
            assert_eq!(
                source.elaboration.clauses[1].kernel_ty,
                KernelTy::Type,
                "the overlay must not rewrite the sealed shallow sort"
            );

            let role = issue_operational_role_witness_v3(&signature, kind)
                .expect("operational role witness");
            assert_eq!(role.step(), kind.step());
            assert_eq!(role.formation_clause(), 0);
            assert_eq!(role.element_clause(), 1);
            assert_eq!(role.path_clause(), 2);
            replay_operational_role_witness_v3(&signature, &role).expect("role replay");

            let overlay = issue_historical_element_overlay_token(&signature, kind)
                .expect("targeted element overlay");
            assert_eq!(overlay.recorded_type(), &KernelTy::Type);
            assert_eq!(
                overlay.overlay_type(),
                &KernelTy::El(overlay.owner().clone())
            );
            replay_historical_element_overlay_token(&signature, &overlay).expect("overlay replay");

            let base =
                issue_historical_base_binding_token_v3(&signature, kind).expect("V3 base binding");
            assert_eq!(
                base.axiom_version(),
                ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION
            );
            assert_eq!(base.point_type(), &KernelTy::El(base.owner().clone()));
            assert_eq!(
                base.element_overlay_derivation_hash(),
                Some(overlay.derivation_hash())
            );
            replay_historical_base_binding_token_v3(&signature, &base).expect("base replay");

            let map = registered_boundary_diagram(&signature, kind).expect("registered map");
            let typed = issue_typed_declared_boundary_token_v3(&signature, kind, map)
                .expect("V3 typed boundary");
            assert_eq!(
                typed.axiom_version(),
                ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION
            );
            assert_eq!(
                typed.element_overlay_derivation_hash(),
                Some(overlay.derivation_hash())
            );
            assert_eq!(
                typed.base_binding_derivation_hash(),
                Some(base.derivation_hash())
            );
            replay_typed_declared_boundary_token_v3(&signature, &typed).expect("typed V3 replay");

            assert_eq!(Telescope::reference(kind.step()), telescope_before);
            assert_eq!(Telescope::reference(kind.step()).kappa(), kappa_before);
            assert_eq!(
                signature
                    .entry(kind.step())
                    .expect("registered step")
                    .candidate_hash,
                candidate_hash_before
            );
            assert!(matches!(
                audit_historical_base_binding(&signature, kind),
                Ok(HistoricalBaseBindingAudit::Obstructed(_))
            ));
        }
        assert_eq!(signature, signature_before);
        assert_eq!(signature.digest(), full_signature_digest_before);

        let trunc_map = registered_boundary_diagram(&signature, RegisteredBoundaryKind::Trunc)
            .expect("Trunc map");
        let v2_trunc = issue_typed_declared_boundary_token(
            &signature,
            RegisteredBoundaryKind::Trunc,
            trunc_map.clone(),
        )
        .expect("archival V2 Trunc typing");
        let v3_trunc = issue_typed_declared_boundary_token_v3(
            &signature,
            RegisteredBoundaryKind::Trunc,
            trunc_map,
        )
        .expect("V3 conservatively replays Trunc without an overlay entry");
        assert_eq!(v3_trunc.element_overlay_derivation_hash(), None);
        assert_eq!(v3_trunc.owner, v2_trunc.owner);
        assert_eq!(v3_trunc.parameter_context, v2_trunc.parameter_context);
        assert_eq!(v3_trunc.faces, v2_trunc.faces);
        assert_eq!(v3_trunc.overlaps, v2_trunc.overlaps);
        replay_typed_declared_boundary_token(&signature, &v2_trunc)
            .expect("V2 token still replays after the additive V3 path");
        replay_typed_declared_boundary_token_v3(&signature, &v3_trunc)
            .expect("V3 Trunc token replay");
    }

    #[test]
    fn historical_constant_diagrams_emit_the_machine_readable_c7_obstruction() {
        let signature = signature();
        for kind in [
            RegisteredBoundaryKind::S1,
            RegisteredBoundaryKind::S2,
            RegisteredBoundaryKind::S3,
        ] {
            let obstruction =
                match audit_historical_base_binding(&signature, kind).expect("base audit runs") {
                    HistoricalBaseBindingAudit::Bound(_) => {
                        panic!("current shallow kernel must not forge a base binding")
                    }
                    HistoricalBaseBindingAudit::Obstructed(obstruction) => obstruction,
                };
            assert_eq!(
                obstruction.obstruction_id(),
                HISTORICAL_POINT_TYPING_OBSTRUCTION_ID
            );
            assert_eq!(obstruction.kind(), kind);
            assert_eq!(obstruction.step(), kind.step());
            assert_eq!(
                obstruction.expected_type(),
                &KernelTy::El(obstruction.owner.clone())
            );
            assert_ne!(obstruction.actual_type(), obstruction.expected_type());

            let map = registered_boundary_diagram(&signature, kind).expect("registered map");
            let error = issue_typed_declared_boundary_token(&signature, kind, map)
                .expect_err("typed token must stay blocked");
            assert!(matches!(
                error,
                TypedBoundaryError::HistoricalPointTypingObstructed { .. }
            ));
        }
    }

    #[test]
    fn v3_historical_prefix_issuers_validate_exact_prefix_and_current_package() {
        for kind in [
            RegisteredBoundaryKind::S1,
            RegisteredBoundaryKind::S2,
            RegisteredBoundaryKind::S3,
        ] {
            let prefix = predecessor_prefix(kind.step());
            let current = Telescope::reference(kind.step());
            let prefix_before = prefix.clone();
            let prefix_digest_before = prefix.digest().to_owned();
            let current_before = current.clone();
            let kappa_before = current.kappa();
            let role =
                issue_operational_role_witness_v3_for_historical_prefix(&prefix, kind, &current)
                    .expect("prefix role");
            replay_operational_role_witness_v3_for_historical_prefix(&prefix, &current, &role)
                .expect("prefix role replay");
            let overlay = issue_historical_element_overlay_token_for_historical_prefix(
                &prefix, kind, &current,
            )
            .expect("prefix overlay");
            replay_historical_element_overlay_token_for_historical_prefix(
                &prefix, &current, &overlay,
            )
            .expect("prefix overlay replay");
            let base = issue_historical_base_binding_token_v3_for_historical_prefix(
                &prefix, kind, &current,
            )
            .expect("prefix base binding");
            replay_historical_base_binding_token_v3_for_historical_prefix(&prefix, &current, &base)
                .expect("prefix base replay");
            let map = registered_boundary_diagram_for_historical_prefix(&prefix, kind, &current)
                .expect("prefix registered map");
            let typed = issue_typed_declared_boundary_token_v3_for_historical_prefix(
                &prefix, kind, &current, map,
            )
            .expect("prefix typed boundary");
            replay_typed_declared_boundary_token_v3_for_historical_prefix(
                &prefix, &current, &typed,
            )
            .expect("prefix typed replay");

            let bridge = issue_historical_prefix_v3_c6_bridge_token(&prefix, kind, &current)
                .expect("narrow C6 bridge");
            assert_eq!(bridge.step(), kind.step());
            assert_eq!(bridge.dimension(), kind.dimension());
            assert_eq!(
                bridge.realized_basis_count(),
                1 + u64::from(kind.dimension()).pow(2)
            );
            assert_eq!(bridge.realized_basis_count(), bridge.expected_basis_count());
            assert_eq!(
                bridge.path_realization_derivation_hashes().len() as u64,
                bridge.realized_basis_count()
            );
            assert_eq!(
                bridge.key_correspondence().len() as u64,
                bridge.realized_basis_count()
            );
            for correspondence in bridge.key_correspondence() {
                match (correspondence.boundary_key(), correspondence.path_key()) {
                    (BoundaryBasisKey::Beta, PathSchemaKey::Beta) => {}
                    (
                        BoundaryBasisKey::PrincipalTransport { principal },
                        PathSchemaKey::Kan {
                            principal: mapped_principal,
                            probe,
                        },
                    ) => {
                        assert_eq!(principal, mapped_principal);
                        assert_eq!(principal, probe);
                    }
                    (
                        BoundaryBasisKey::TransportNaturality { principal, probe },
                        PathSchemaKey::Kan {
                            principal: mapped_principal,
                            probe: mapped_probe,
                        },
                    ) => {
                        assert_ne!(principal, probe);
                        assert_eq!(principal, mapped_principal);
                        assert_eq!(probe, mapped_probe);
                    }
                    other => panic!("incorrect boundary/path key correspondence: {other:?}"),
                }
            }
            replay_historical_prefix_v3_c6_bridge_token(&prefix, &current, &bridge)
                .expect("C6 bridge replay");
            assert_eq!(prefix, prefix_before);
            assert_eq!(prefix.digest(), prefix_digest_before);
            assert_eq!(current, current_before);
            assert_eq!(current.kappa(), kappa_before);
        }

        let s1_prefix = predecessor_prefix(5);
        let s1 = Telescope::reference(5);
        assert_eq!(
            issue_operational_role_witness_v3_for_historical_prefix(
                &s1_prefix,
                RegisteredBoundaryKind::S2,
                &s1,
            ),
            Err(TypedBoundaryError::HistoricalPrefixMismatch { predecessor: 6 })
        );
        let mut wrong_current = s1.clone();
        wrong_current.clauses[1].role = ClauseRole::Formation;
        assert_eq!(
            issue_historical_base_binding_token_v3_for_historical_prefix(
                &s1_prefix,
                RegisteredBoundaryKind::S1,
                &wrong_current,
            ),
            Err(TypedBoundaryError::HistoricalCurrentTelescopeMismatch { step: 5 })
        );
        let trunc_prefix = predecessor_prefix(6);
        let trunc = Telescope::reference(6);
        let trunc_map = registered_boundary_diagram_for_historical_prefix(
            &trunc_prefix,
            RegisteredBoundaryKind::Trunc,
            &trunc,
        )
        .expect("prefix Trunc map");
        let trunc_typed = issue_typed_declared_boundary_token_v3_for_historical_prefix(
            &trunc_prefix,
            RegisteredBoundaryKind::Trunc,
            &trunc,
            trunc_map,
        )
        .expect("prefix Trunc typed context control");
        assert_eq!(trunc_typed.element_overlay_derivation_hash(), None);
        assert_eq!(trunc_typed.base_binding_derivation_hash(), None);
        assert_eq!(trunc_typed.parameter_context().bindings().len(), 3);
        replay_typed_declared_boundary_token_v3_for_historical_prefix(
            &trunc_prefix,
            &trunc,
            &trunc_typed,
        )
        .expect("prefix Trunc typed replay");
        assert_eq!(
            issue_historical_element_overlay_token_for_historical_prefix(
                &trunc_prefix,
                RegisteredBoundaryKind::Trunc,
                &trunc,
            ),
            Err(TypedBoundaryError::HistoricalElementOverlayNotRegistered {
                kind: RegisteredBoundaryKind::Trunc,
            })
        );
        assert_eq!(
            issue_historical_prefix_v3_c6_bridge_token(
                &trunc_prefix,
                RegisteredBoundaryKind::Trunc,
                &trunc,
            ),
            Err(TypedBoundaryError::C6EndpointDependentBoundaryGap {
                gap_id: C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP.to_owned(),
            })
        );

        let wrong_trunc_prefix = predecessor_prefix(5);
        assert_eq!(
            issue_historical_prefix_v3_c6_bridge_token(
                &wrong_trunc_prefix,
                RegisteredBoundaryKind::Trunc,
                &trunc,
            ),
            Err(TypedBoundaryError::HistoricalPrefixMismatch { predecessor: 5 })
        );
        let mut wrong_trunc_current = trunc.clone();
        wrong_trunc_current.clauses[1].role = ClauseRole::Formation;
        assert_eq!(
            issue_historical_prefix_v3_c6_bridge_token(
                &trunc_prefix,
                RegisteredBoundaryKind::Trunc,
                &wrong_trunc_current,
            ),
            Err(TypedBoundaryError::HistoricalCurrentTelescopeMismatch { step: 6 })
        );
    }

    #[test]
    fn prefix_general_s3_boundary_accepts_a_typed_fork_without_historical_b7() {
        let prefix = alternate_stage4_prefix();
        let current = Telescope::reference(8);
        assert_ne!(prefix, predecessor_prefix(8));
        let diagram =
            registered_s3_boundary_diagram_for_prefix_general_v4(&prefix, &current).unwrap();
        let token = issue_typed_declared_s3_boundary_token_v4_for_prefix_general(
            &prefix, &current, diagram,
        )
        .unwrap();
        replay_typed_declared_s3_boundary_token_v4_for_prefix_general(&prefix, &current, &token)
            .unwrap();
        assert_eq!(token.kind(), RegisteredBoundaryKind::S3);
        assert_eq!(token.signature_digest, prefix.digest());
        assert_eq!(token.candidate_hash, candidate_hash(&current));
        assert_eq!(token.faces().len(), 6);
    }

    #[test]
    fn prefix_general_s3_boundary_is_exactly_legacy_on_historical_b7() {
        let prefix = predecessor_prefix(8);
        let current = Telescope::reference(8);
        let legacy_diagram = registered_boundary_diagram_for_historical_prefix(
            &prefix,
            RegisteredBoundaryKind::S3,
            &current,
        )
        .unwrap();
        let legacy = issue_typed_declared_boundary_token_v3_for_historical_prefix(
            &prefix,
            RegisteredBoundaryKind::S3,
            &current,
            legacy_diagram,
        )
        .unwrap();
        let general_diagram =
            registered_s3_boundary_diagram_for_prefix_general_v4(&prefix, &current).unwrap();
        let general = issue_typed_declared_s3_boundary_token_v4_for_prefix_general(
            &prefix,
            &current,
            general_diagram,
        )
        .unwrap();
        assert_eq!(general, legacy);
    }

    #[test]
    fn arbitrary_face_families_name_the_hoisting_rule_and_policy() {
        let signature = signature();
        let context = registered_parameter_context(&signature, RegisteredBoundaryKind::S2)
            .expect("registered context");
        let map = generic_declared_boundary(context.owner().clone(), 2, "invented")
            .expect("structural generic boundary");
        let error = issue_reference_only_charge_token(&signature, RegisteredBoundaryKind::S2, map)
            .expect_err("arbitrary face families require hoisting");
        match error {
            TypedBoundaryError::HoistingRequired {
                reason,
                charging_policy,
                hoisting_rule,
                ..
            } => {
                assert_eq!(
                    reason,
                    ReferenceResolutionFailure::ArbitraryDeclaredFaceFamily
                );
                assert_eq!(charging_policy, BOUNDARY_CHARGE_POLICY_VERSION);
                assert_eq!(hoisting_rule, REFERENCE_ONLY_HOISTING_RULE);
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn unbound_context_parameters_fail_with_the_same_hoisting_policy() {
        let signature = signature();
        let mut map = registered_boundary_diagram(&signature, RegisteredBoundaryKind::Trunc)
            .expect("Trunc map");
        map.faces[0].term.origin = BoundaryTermOrigin::SemanticPointParameter {
            parameter: "z".to_owned(),
        };
        let error =
            issue_reference_only_charge_token(&signature, RegisteredBoundaryKind::Trunc, map)
                .expect_err("unbound parameter must fail");
        assert!(matches!(
            &error,
            TypedBoundaryError::HoistingRequired {
                reason: ReferenceResolutionFailure::UnboundContextVariable,
                ..
            }
        ));
        assert!(error.to_string().contains(BOUNDARY_CHARGE_POLICY_VERSION));
        assert!(error.to_string().contains("must be hoisted"));
    }

    #[test]
    fn registered_diagram_mutation_fails_closed() {
        let signature = signature();
        let mut map =
            registered_boundary_diagram(&signature, RegisteredBoundaryKind::S1).expect("S1 map");
        map.faces[0].term.name = "other-base".to_owned();
        assert_eq!(
            issue_reference_only_charge_token(&signature, RegisteredBoundaryKind::S1, map),
            Err(TypedBoundaryError::RegisteredDiagramMismatch {
                kind: RegisteredBoundaryKind::S1,
            })
        );
    }

    #[test]
    fn private_token_field_mutations_fail_replay() {
        let signature = signature();
        let map = registered_boundary_diagram(&signature, RegisteredBoundaryKind::Trunc)
            .expect("Trunc map");
        let reference = issue_reference_only_charge_token(
            &signature,
            RegisteredBoundaryKind::Trunc,
            map.clone(),
        )
        .expect("reference token");
        let mut bad_reference = reference.clone();
        bad_reference.boundary_credit = 1;
        assert_eq!(
            replay_reference_only_charge_token(&signature, &bad_reference),
            Err(TypedBoundaryError::ReferenceOnlyReplayMismatch)
        );

        let typed =
            issue_typed_declared_boundary_token(&signature, RegisteredBoundaryKind::Trunc, map)
                .expect("typed token");
        let mut bad_typed = typed.clone();
        bad_typed.derivation_hash.push_str(":mutated");
        assert_eq!(
            replay_typed_declared_boundary_token(&signature, &bad_typed),
            Err(TypedBoundaryError::TypedBoundaryReplayMismatch)
        );

        let basis = issue_adopted_boundary_basis_charge_token(
            &signature,
            RegisteredBoundaryKind::Trunc,
            registered_boundary_diagram(&signature, RegisteredBoundaryKind::Trunc)
                .expect("Trunc map"),
        )
        .expect("basis/charge token");
        for field in 0..16 {
            let mut mutated = basis.clone();
            match field {
                0 => mutated.fragment_version.push_str(":mutated"),
                1 => mutated.axiom_version.push_str(":mutated"),
                2 => mutated.charging_policy.push_str(":mutated"),
                3 => mutated.signature_digest.push_str(":mutated"),
                4 => mutated.candidate_hash.push_str(":mutated"),
                5 => mutated.kind = RegisteredBoundaryKind::S1,
                6 => mutated.map.boundary_is_declared_input_not_credit = false,
                7 => mutated.dimension += 1,
                8 => mutated.beta_count += 1,
                9 => mutated.principal_transport_count += 1,
                10 => mutated.ordered_naturality_count += 1,
                11 => mutated.basis_count += 1,
                12 => mutated.boundary_credit += 1,
                13 => mutated.total_with_boundary += 1,
                14 => mutated.reference_only_derivation_hash.push_str(":mutated"),
                15 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_adopted_boundary_basis_charge_token(&signature, &mutated).is_err(),
                "field mutation {field} must fail replay"
            );
        }
    }

    #[test]
    fn every_v3_overlay_token_field_mutation_fails_replay() {
        let signature = signature();
        let role = issue_operational_role_witness_v3(&signature, RegisteredBoundaryKind::S1)
            .expect("role token");
        for field in 0..14 {
            let mut mutated = role.clone();
            match field {
                0 => mutated.fragment_version.push_str(":mutated"),
                1 => mutated.axiom_version.push_str(":mutated"),
                2 => mutated.overlay_version.push_str(":mutated"),
                3 => mutated.signature_digest.push_str(":mutated"),
                4 => mutated.candidate_hash.push_str(":mutated"),
                5 => mutated.kind = RegisteredBoundaryKind::S2,
                6 => mutated.step += 1,
                7 => mutated.formation_clause += 1,
                8 => mutated.element_clause += 1,
                9 => mutated.path_clause += 1,
                10 => mutated.sealed_telescope_digest.push_str(":mutated"),
                11 => mutated.projection.path_dimension += 1,
                12 => mutated.projection_digest.push_str(":mutated"),
                13 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_operational_role_witness_v3(&signature, &mutated).is_err(),
                "operational-role field {field} must fail replay"
            );
        }

        let overlay =
            issue_historical_element_overlay_token(&signature, RegisteredBoundaryKind::S1)
                .expect("overlay token");
        for field in 0..17 {
            let mut mutated = overlay.clone();
            match field {
                0 => mutated.fragment_version.push_str(":mutated"),
                1 => mutated.axiom_version.push_str(":mutated"),
                2 => mutated.overlay_version.push_str(":mutated"),
                3 => mutated.signature_digest.push_str(":mutated"),
                4 => mutated.candidate_hash.push_str(":mutated"),
                5 => mutated.kind = RegisteredBoundaryKind::S2,
                6 => mutated.step += 1,
                7 => mutated.formation_clause += 1,
                8 => mutated.element_clause += 1,
                9 => mutated.formation_term = Expr::Univ,
                10 => mutated.element_term = Expr::Univ,
                11 => mutated.owner = Expr::Univ,
                12 => mutated.recorded_type = KernelTy::Neutral,
                13 => mutated.overlay_type = KernelTy::Type,
                14 => mutated.elaboration_derivation_hash.push_str(":mutated"),
                15 => mutated
                    .operational_role_derivation_hash
                    .push_str(":mutated"),
                16 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_historical_element_overlay_token(&signature, &mutated).is_err(),
                "element-overlay field {field} must fail replay"
            );
        }

        let source =
            registered_source(&signature, RegisteredBoundaryKind::S1).expect("registered source");
        let mut forged_overlay = overlay;
        forged_overlay.owner = Expr::Univ;
        assert_eq!(
            construct_historical_base_binding_token_v3(signature.digest(), source, &forged_overlay,),
            Err(TypedBoundaryError::HistoricalElementOverlayTypeMismatch)
        );
    }

    #[test]
    fn every_v3_base_typed_and_c6_bridge_field_mutation_fails_replay() {
        let signature = signature();
        let base = issue_historical_base_binding_token_v3(&signature, RegisteredBoundaryKind::S1)
            .expect("V3 base token");
        for field in 0..14 {
            let mut mutated = base.clone();
            match field {
                0 => mutated.fragment_version.push_str(":mutated"),
                1 => mutated.axiom_version.push_str(":mutated"),
                2 => mutated.signature_digest.push_str(":mutated"),
                3 => mutated.candidate_hash.push_str(":mutated"),
                4 => mutated.kind = RegisteredBoundaryKind::S2,
                5 => mutated.formation_clause += 1,
                6 => mutated.point_clause += 1,
                7 => mutated.path_clause += 1,
                8 => mutated.owner = Expr::Univ,
                9 => mutated.point_term = Expr::Univ,
                10 => mutated.point_type = KernelTy::Type,
                11 => mutated.elaboration_derivation_hash.push_str(":mutated"),
                12 => mutated
                    .element_overlay_derivation_hash
                    .as_mut()
                    .expect("V3 overlay hash")
                    .push_str(":mutated"),
                13 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_historical_base_binding_token_v3(&signature, &mutated).is_err(),
                "V3 base field {field} must fail replay"
            );
        }

        let map =
            registered_boundary_diagram(&signature, RegisteredBoundaryKind::S2).expect("S2 map");
        let typed =
            issue_typed_declared_boundary_token_v3(&signature, RegisteredBoundaryKind::S2, map)
                .expect("V3 typed token");
        for field in 0..16 {
            let mut mutated = typed.clone();
            match field {
                0 => mutated.fragment_version.push_str(":mutated"),
                1 => mutated.axiom_version.push_str(":mutated"),
                2 => mutated.registered_diagrams_version.push_str(":mutated"),
                3 => mutated.signature_digest.push_str(":mutated"),
                4 => mutated.candidate_hash.push_str(":mutated"),
                5 => mutated.kind = RegisteredBoundaryKind::S1,
                6 => mutated.owner = Expr::Univ,
                7 => mutated.dimension += 1,
                8 => mutated.map.faces[0].term.name.push_str(":mutated"),
                9 => mutated
                    .parameter_context
                    .context_digest
                    .push_str(":mutated"),
                10 => mutated.reference_only_derivation_hash.push_str(":mutated"),
                11 => mutated
                    .base_binding_derivation_hash
                    .as_mut()
                    .expect("base hash")
                    .push_str(":mutated"),
                12 => mutated
                    .element_overlay_derivation_hash
                    .as_mut()
                    .expect("overlay hash")
                    .push_str(":mutated"),
                13 => mutated.faces[0]
                    .term
                    .normal_form_digest
                    .push_str(":mutated"),
                14 => mutated.overlaps[0]
                    .common_normal_form_digest
                    .push_str(":mutated"),
                15 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_typed_declared_boundary_token_v3(&signature, &mutated).is_err(),
                "V3 typed field {field} must fail replay"
            );
        }

        let prefix = predecessor_prefix(7);
        let current = Telescope::reference(7);
        let bridge = issue_historical_prefix_v3_c6_bridge_token(
            &prefix,
            RegisteredBoundaryKind::S2,
            &current,
        )
        .expect("C6 bridge");
        for field in 0..20 {
            let mut mutated = bridge.clone();
            match field {
                0 => mutated.bridge_version.push_str(":mutated"),
                1 => mutated.axiom_version.push_str(":mutated"),
                2 => mutated.overlay_version.push_str(":mutated"),
                3 => mutated.signature_digest.push_str(":mutated"),
                4 => mutated.candidate_hash.push_str(":mutated"),
                5 => mutated.kind = RegisteredBoundaryKind::S1,
                6 => mutated.step += 1,
                7 => mutated.dimension += 1,
                8 => mutated.telescope_digest.push_str(":mutated"),
                9 => mutated.typing_derivation_hash.push_str(":mutated"),
                10 => mutated.typed_boundary_derivation_hash.push_str(":mutated"),
                11 => mutated
                    .boundary_attachment_derivation_hash
                    .push_str(":mutated"),
                12 => mutated
                    .boundary_basis_presentation_derivation_hash
                    .push_str(":mutated"),
                13 => {
                    mutated.key_correspondence[0].path_key = PathSchemaKey::Kan {
                        principal: 9,
                        probe: 9,
                    }
                }
                14 => mutated.key_bijection_digest.push_str(":mutated"),
                15 => mutated.path_basis_derivation_hash.push_str(":mutated"),
                16 => mutated.path_realization_derivation_hashes[0].push_str(":mutated"),
                17 => mutated.expected_basis_count += 1,
                18 => mutated.realized_basis_count += 1,
                19 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_historical_prefix_v3_c6_bridge_token(&prefix, &current, &mutated).is_err(),
                "C6 bridge field {field} must fail replay"
            );
        }
    }

    #[test]
    fn trunc_endpoint_bundle_replays_in_full_h15_and_exact_b5() {
        let full_signature = signature();
        let full = issue_trunc_endpoint_v3_c6_bundle_token(&full_signature)
            .expect("full-H15 Trunc endpoint bundle");
        assert_eq!(full.source_scope(), TruncEndpointSourceScope::FullH15);
        assert_eq!(
            full.signature_digest(),
            "blake3:d51ffb32c2e6e18f45ee15a8c1af3edbf40759fda1696032ae6e97439c48f10a"
        );
        assert_eq!(full.expected_basis_count(), 2);
        assert_eq!(full.realized_basis_count(), 2);
        assert_eq!(full.endpoint_faces().len(), 2);
        assert_eq!(
            full.endpoint_faces()[0].face(),
            BoundaryFaceKey::new(0, false)
        );
        assert_eq!(full.endpoint_faces()[0].parameter_index(), 1);
        assert_eq!(full.endpoint_faces()[0].parameter_name(), "x");
        assert_eq!(
            full.endpoint_faces()[1].face(),
            BoundaryFaceKey::new(0, true)
        );
        assert_eq!(full.endpoint_faces()[1].parameter_index(), 2);
        assert_eq!(full.endpoint_faces()[1].parameter_name(), "y");
        assert_eq!(full.path_realizations().len(), 2);
        assert_eq!(full.path_realizations()[0].key(), &PathSchemaKey::Beta);
        assert_eq!(
            full.path_realizations()[1].key(),
            &PathSchemaKey::Kan {
                principal: 0,
                probe: 0,
            }
        );
        assert_eq!(
            full.obstruction_retired(),
            C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP
        );
        assert!(!full.general_c6_proved());
        assert_eq!(
            full.restricted_instantiation().theorem_scope(),
            SORT_PRESERVATION_SCOPE
        );
        assert_eq!(
            full.restricted_instantiation()
                .arbitrary_typed_instance_gap(),
            C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION_GAP
        );
        assert!(
            !full
                .restricted_instantiation()
                .arbitrary_typed_images_used()
        );
        assert!(!full.endpoint_premise_source_digest().is_empty());
        assert!(!full.endpoint_premise_context_derivation_hash().is_empty());
        replay_trunc_endpoint_v3_c6_bundle_token(&full_signature, &full)
            .expect("full-H15 bundle replay");

        let prefix = predecessor_prefix(6);
        let current = Telescope::reference(6);
        assert_eq!(
            prefix.digest(),
            "blake3:0ee6911b2820caeb0e745b4e44fa0875b8bfbca8f66b6a4921694ebde1cf44fd"
        );
        let historical =
            issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(&prefix, &current)
                .expect("exact B5 Trunc endpoint bundle");
        assert_eq!(
            historical.source_scope(),
            TruncEndpointSourceScope::HistoricalPrefixB5
        );
        assert_eq!(historical.signature_digest(), prefix.digest());
        assert_eq!(historical.expected_basis_count(), 2);
        assert_eq!(historical.realized_basis_count(), 2);
        assert_ne!(
            historical.endpoint_premise_source_digest(),
            full.endpoint_premise_source_digest(),
            "the formal premise context is bound to the H15/B5 source"
        );
        assert_ne!(
            historical.endpoint_premise_context_derivation_hash(),
            full.endpoint_premise_context_derivation_hash()
        );
        replay_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
            &prefix,
            &current,
            &historical,
        )
        .expect("B5 bundle replay");
        assert_eq!(
            replay_trunc_endpoint_v3_c6_bundle_token(&full_signature, &historical),
            Err(TypedBoundaryError::TruncEndpointBundleReplayMismatch),
            "the B5 source-bound token must not replay as a full-H15 token"
        );
        assert_eq!(
            replay_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
                &prefix, &current, &full,
            ),
            Err(TypedBoundaryError::TruncEndpointBundleReplayMismatch),
            "the full-H15 source-bound token must not replay as a B5 token"
        );

        // Schema 5 is an archival snapshot: its old API continues to record
        // the then-current gap.  The successor API is what retires it.
        assert_eq!(
            issue_historical_prefix_v3_c6_bridge_token(
                &prefix,
                RegisteredBoundaryKind::Trunc,
                &current,
            ),
            Err(TypedBoundaryError::C6EndpointDependentBoundaryGap {
                gap_id: C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP.to_owned(),
            })
        );
        let successor = issue_historical_prefix_v3_c6_typed_bundle_token(
            &prefix,
            RegisteredBoundaryKind::Trunc,
            &current,
        )
        .expect("successor Trunc handoff");
        assert_eq!(
            successor.evidence_kind(),
            HistoricalTypedBundleEvidenceKind::TruncEndpointDependentV1
        );
        assert_eq!(successor.realized_basis_count(), 2);
        replay_historical_prefix_v3_c6_typed_bundle_token(&prefix, &current, &successor)
            .expect("successor handoff replay");
    }

    #[test]
    fn trunc_successor_handoff_is_exactly_two_two_five_ten_and_preserves_constant_children() {
        let handoff = issue_historical_v3_c6_typed_handoff_token().expect("four-package handoff");
        assert_eq!(handoff.registered_counts(), [2, 2, 5, 10]);
        assert!(handoff.all_replayed());
        assert_eq!(
            handoff
                .bundles()
                .iter()
                .map(HistoricalPrefixV3C6TypedBundleToken::kind)
                .collect::<Vec<_>>(),
            RegisteredBoundaryKind::ALL
        );
        assert_eq!(
            handoff
                .bundles()
                .iter()
                .map(HistoricalPrefixV3C6TypedBundleToken::signature_digest)
                .collect::<Vec<_>>(),
            vec![
                "blake3:11c74244bfb002c53bc4f3b9a3c7013e7e38460ad03ce63bf9bc41e535fc8458",
                "blake3:0ee6911b2820caeb0e745b4e44fa0875b8bfbca8f66b6a4921694ebde1cf44fd",
                "blake3:e8cba04def50e09c83b1e28a252bb6f2e327e6468027a3d9353e20d9ad19e5ed",
                "blake3:301916fbd578c9e0d6375724b44201f0198d2d4bbd9493614009a603325a3adb",
            ]
        );
        assert_eq!(
            [
                handoff.bundles()[0].child_derivation_hash(),
                handoff.bundles()[2].child_derivation_hash(),
                handoff.bundles()[3].child_derivation_hash(),
            ],
            [
                "blake3:29f76eaf1700ad76a423be6df0336f29f078fb3ff2768b542969fbcbfdf76db0",
                "blake3:69fcbffdbaf7ea2a38c404422ccfcc72d457cefe75c3f55d993903289c4ac127",
                "blake3:875427e0bb735f45bffc27e02eb8b31452c932b816072294a11d4716394899f8",
            ]
        );
        replay_historical_v3_c6_typed_handoff_token(&handoff).expect("handoff replay");
    }

    #[test]
    fn trunc_endpoint_issuers_fail_before_evidence_on_wrong_prefix_or_current() {
        let exact_prefix = predecessor_prefix(6);
        let current = Telescope::reference(6);
        assert_eq!(
            issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
                &predecessor_prefix(5),
                &current,
            ),
            Err(TypedBoundaryError::HistoricalPrefixMismatch { predecessor: 5 })
        );
        let mut wrong_current = current.clone();
        wrong_current.clauses[1].role = ClauseRole::Formation;
        assert_eq!(
            issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
                &exact_prefix,
                &wrong_current,
            ),
            Err(TypedBoundaryError::HistoricalCurrentTelescopeMismatch { step: 6 })
        );
    }

    #[test]
    fn trunc_endpoint_bundle_and_handoff_mutations_fail_definition_replay() {
        let prefix = predecessor_prefix(6);
        let current = Telescope::reference(6);
        let bundle =
            issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(&prefix, &current)
                .expect("B5 bundle");
        for field in 0..33 {
            let mut mutated = bundle.clone();
            match field {
                0 => mutated.bundle_version.push_str(":mutated"),
                1 => mutated.cubical_fragment_version.push_str(":mutated"),
                2 => mutated.axiom_version.push_str(":mutated"),
                3 => mutated.source_scope = TruncEndpointSourceScope::FullH15,
                4 => mutated.signature_digest.push_str(":mutated"),
                5 => mutated.candidate_hash.push_str(":mutated"),
                6 => mutated.step += 1,
                7 => mutated.dimension += 1,
                8 => mutated.telescope_digest.push_str(":mutated"),
                9 => mutated.typing_derivation_hash.push_str(":mutated"),
                10 => mutated.typed_boundary_derivation_hash.push_str(":mutated"),
                11 => mutated.parameter_context_digest.push_str(":mutated"),
                12 => mutated.endpoint_faces[0].parameter_index += 1,
                13 => mutated.endpoint_faces[0]
                    .boundary_term_normal_form_digest
                    .push_str(":mutated"),
                14 => mutated.boundary_binding_digest.push_str(":mutated"),
                15 => mutated
                    .restricted_instantiation
                    .derivation_hash
                    .push_str(":mutated"),
                16 => mutated.endpoint_premise_source_digest.push_str(":mutated"),
                17 => mutated
                    .endpoint_premise_context_derivation_hash
                    .push_str(":mutated"),
                18 => mutated
                    .computation_audit_derivation_hash
                    .push_str(":mutated"),
                19 => mutated
                    .boundary_attachment_derivation_hash
                    .push_str(":mutated"),
                20 => mutated
                    .boundary_basis_presentation_derivation_hash
                    .push_str(":mutated"),
                21 => mutated.method_shape_digest.push_str(":mutated"),
                22 => {
                    mutated.key_correspondence[0].path_key = PathSchemaKey::Kan {
                        principal: 9,
                        probe: 9,
                    }
                }
                23 => mutated.key_bijection_digest.push_str(":mutated"),
                24 => mutated.path_basis_derivation_hash.push_str(":mutated"),
                25 => mutated.path_realizations[0].term_hash.push_str(":mutated"),
                26 => mutated.path_realizations[0]
                    .normal_form_hash
                    .push_str(":mutated"),
                27 => mutated.path_realizations[1].type_hash.push_str(":mutated"),
                28 => mutated.path_realizations[1]
                    .derivation_hash
                    .push_str(":mutated"),
                29 => mutated.expected_basis_count += 1,
                30 => mutated.realized_basis_count += 1,
                31 => mutated.general_c6_proved = true,
                32 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(
                    &prefix, &current, &mutated,
                )
                .is_err(),
                "Trunc endpoint bundle field {field} must fail replay"
            );
        }

        let handoff = issue_historical_prefix_v3_c6_typed_bundle_token(
            &prefix,
            RegisteredBoundaryKind::Trunc,
            &current,
        )
        .expect("successor handoff");
        for field in 0..14 {
            let mut mutated = handoff.clone();
            match field {
                0 => mutated.handoff_version.push_str(":mutated"),
                1 => {
                    mutated.evidence_kind =
                        HistoricalTypedBundleEvidenceKind::ArchivalConstantBridge
                }
                2 => mutated.signature_digest.push_str(":mutated"),
                3 => mutated.kind = RegisteredBoundaryKind::S1,
                4 => mutated.step += 1,
                5 => mutated.dimension += 1,
                6 => mutated.typed_boundary_derivation_hash.push_str(":mutated"),
                7 => {
                    mutated.key_correspondence.remove(0);
                }
                8 => mutated.path_realization_derivation_hashes[0].push_str(":mutated"),
                9 => mutated.expected_basis_count += 1,
                10 => mutated.realized_basis_count += 1,
                11 => mutated.child_derivation_hash.push_str(":mutated"),
                12 => mutated.derivation_hash.push_str(":mutated"),
                13 => mutated.key_correspondence.reverse(),
                _ => unreachable!(),
            }
            assert!(
                replay_historical_prefix_v3_c6_typed_bundle_token(&prefix, &current, &mutated,)
                    .is_err(),
                "successor handoff field {field} must fail replay"
            );
        }
    }

    #[test]
    fn trunc_restricted_instantiation_names_arbitrary_typed_image_gap() {
        let context = SortedParameterContext::new(vec![
            ParameterSort::Type,
            ParameterSort::Opaque,
            ParameterSort::Opaque,
        ]);
        let error = issue_sort_preserving_substitution(
            context.clone(),
            context,
            vec![
                SubstitutionImage {
                    source_parameter: 1,
                    term: Expr::Trunc(Box::new(Expr::Var(1))),
                },
                SubstitutionImage {
                    source_parameter: 2,
                    term: Expr::Var(2),
                },
                SubstitutionImage {
                    source_parameter: 3,
                    term: Expr::Var(3),
                },
            ],
            Expr::Var(1),
        )
        .expect_err("arbitrary owner image needs the inherited typed-instance theorem");
        assert_eq!(
            error,
            crate::substitution::SubstitutionError::NonVariableImageNeedsTypedJudgement {
                source_parameter: 1,
            }
        );
        assert_eq!(
            C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION_GAP,
            "C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION"
        );
    }

    #[test]
    fn trunc_restricted_instantiation_exhausts_and_replays_all_four_variable_images() {
        let source = registered_source(&signature(), RegisteredBoundaryKind::Trunc)
            .expect("registered Trunc source");
        let token = issue_trunc_restricted_instantiation(&source.typing)
            .expect("complete restricted instantiation audit");
        assert!(token.all_sort_identical_variable_images_covered());
        assert_eq!(token.variable_instances().len(), 4);
        assert_eq!(
            token
                .variable_instances()
                .iter()
                .map(|instance| (
                    instance.kind(),
                    instance.zero_boundary_parameter(),
                    instance.one_boundary_parameter(),
                    instance.has_degenerate_endpoints(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (TruncRestrictedInstanceKind::Identity, 1, 2, false),
                (TruncRestrictedInstanceKind::Swap, 2, 1, false),
                (TruncRestrictedInstanceKind::CollapseToX, 1, 1, true),
                (TruncRestrictedInstanceKind::CollapseToY, 2, 2, true),
            ]
        );

        for instance in token.variable_instances() {
            assert_eq!(
                instance.images(),
                instance.owner_substitution.structural().images()
            );
            assert_eq!(
                instance.images(),
                instance.zero_endpoint_substitution.structural().images()
            );
            assert_eq!(
                instance.images(),
                instance.one_endpoint_substitution.structural().images()
            );
            assert_eq!(
                instance.instantiated_owner(),
                &Expr::Trunc(Box::new(Expr::Var(1)))
            );
            assert!(!instance.premise_source_digest().is_empty());
            assert!(!instance.premise_context_derivation_hash().is_empty());
            assert!(!instance.computation_audit_derivation_hash().is_empty());
            replay_trunc_restricted_variable_instance(&source.typing, instance)
                .expect("individual restricted instance replay");
        }
        replay_trunc_restricted_instantiation(&source.typing, &token)
            .expect("complete restricted inventory replay");

        // The identity projections consumed by the v1 sidecar remain the
        // exact incumbent C-1 tokens even though the complete token grows.
        assert_eq!(
            token.owner_substitution_derivation_hash(),
            "blake3:c5c65290f568a2a5cca313f995e5a77d9631de4419a981fb8902fe7ac568ee64"
        );
        assert_eq!(
            token.zero_endpoint_substitution_derivation_hash(),
            "blake3:83fc10a0e12708151eb153b3120ebfd85bcfa2354b194edac11cfbcff3536788"
        );
        assert_eq!(
            token.one_endpoint_substitution_derivation_hash(),
            "blake3:a04013d52bf305065a7918bbd0193f5aedff421984d2677f3f4e24da964fd6a6"
        );

        let mut mutated = token.variable_instances()[2].clone();
        mutated.derivation_hash.push_str(":mutated");
        assert!(
            replay_trunc_restricted_variable_instance(&source.typing, &mutated).is_err(),
            "an individual instance must replay by full definition equality"
        );
        let mut mutated_premise_source = token.variable_instances()[2].clone();
        mutated_premise_source
            .premise_source_digest
            .push_str(":mutated");
        assert!(
            replay_trunc_restricted_variable_instance(&source.typing, &mutated_premise_source,)
                .is_err(),
            "the instantiated formal-premise source must be replay-bound"
        );
        let mut mutated_premise_context = token.variable_instances()[2].clone();
        mutated_premise_context
            .premise_context_derivation_hash
            .push_str(":mutated");
        assert!(
            replay_trunc_restricted_variable_instance(&source.typing, &mutated_premise_context,)
                .is_err(),
            "the instantiated formal-premise ledger must be replay-bound"
        );
    }

    #[test]
    fn trunc_restricted_instance_rejects_wrong_sort_and_non_variable_images() {
        let source = registered_source(&signature(), RegisteredBoundaryKind::Trunc)
            .expect("registered Trunc source");
        let wrong_sort = vec![
            SubstitutionImage {
                source_parameter: 1,
                term: Expr::Var(2),
            },
            SubstitutionImage {
                source_parameter: 2,
                term: Expr::Var(2),
            },
            SubstitutionImage {
                source_parameter: 3,
                term: Expr::Var(3),
            },
        ];
        assert!(matches!(
            issue_trunc_restricted_variable_instance(&source.typing, wrong_sort),
            Err(TypedBoundaryError::TruncRestrictedInstantiation { error })
                if error.contains("source parameter 1 has sort Type")
                    && error.contains("target parameter 2 has sort Opaque")
        ));

        let non_variable = vec![
            SubstitutionImage {
                source_parameter: 1,
                term: Expr::Var(1),
            },
            SubstitutionImage {
                source_parameter: 2,
                term: Expr::Trunc(Box::new(Expr::Var(1))),
            },
            SubstitutionImage {
                source_parameter: 3,
                term: Expr::Var(3),
            },
        ];
        assert!(matches!(
            issue_trunc_restricted_variable_instance(&source.typing, non_variable),
            Err(TypedBoundaryError::TruncRestrictedInstantiation { error })
                if error.contains("requires a variable image for source parameter 2")
                    && error.contains("needs a typed instance judgement")
        ));
    }

    #[test]
    fn non_genesis_signature_cannot_replay_registered_tokens() {
        let signature = signature();
        let map = registered_boundary_diagram(&signature, RegisteredBoundaryKind::Trunc)
            .expect("Trunc map");
        let token =
            issue_reference_only_charge_token(&signature, RegisteredBoundaryKind::Trunc, map)
                .expect("reference token");
        let partial = SealedSignature::from_telescopes(vec![
            (5, Telescope::reference(5)),
            (6, Telescope::reference(6)),
        ]);
        assert_eq!(
            replay_reference_only_charge_token(&partial, &token),
            Err(TypedBoundaryError::SignatureNotSealedGenesis)
        );
    }
}
