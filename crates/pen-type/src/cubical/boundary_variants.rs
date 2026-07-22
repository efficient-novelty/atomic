//! Additive structural boundary theories for the historical attachment audit.
//! This module deliberately does not alter the incumbent cubical syntax or
//! its V1 token hashes. Its V2 tokens retain their proposal-era version for
//! archival replay; adopted evidence is issued only by
//! [`super::typed_boundary`], which binds the exact adopted axiom and charging
//! policy strings.
//!
//! V2 treats a coherent boundary diagram as an explicit declared premise.  The
//! small diagram language checks all codimension-one faces and their pairwise
//! overlaps.  It does **not** elaborate the annotated face terms or type
//! boundary-aware transports; those remain separate obligations.  The
//! structural fragment is sufficient to compare the historical boundary
//! shapes and to retain a general-position constructor for audit tests.

use super::PATHCON_ATTACHMENT_AXIOM_VERSION;
use crate::elaborate::SealedSignature;
use crate::tdc1::FormedPathTyping;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const BOUNDARY_VARIANTS_FRAGMENT_VERSION: &str = "tdc1-boundary-variants-v1";
pub const DECLARED_BOUNDARY_RULE_VERSION: &str =
    "tdc1-pathcon-attachment-explicit-declared-boundary-diagram-proposal-v2";
pub const TRACE_DERIVED_BOUNDARY_CONSTRAINT_VERSION: &str =
    "tdc1-pathcon-attachment-trace-derived-underdetermined-constraints-v3";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryTheory {
    V1ImplicitConstant,
    V2DeclaredBoundary,
    V3TraceDerived,
}

impl BoundaryTheory {
    pub const fn version(self) -> &'static str {
        match self {
            Self::V1ImplicitConstant => PATHCON_ATTACHMENT_AXIOM_VERSION,
            Self::V2DeclaredBoundary => DECLARED_BOUNDARY_RULE_VERSION,
            Self::V3TraceDerived => TRACE_DERIVED_BOUNDARY_CONSTRAINT_VERSION,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BoundaryFaceKey {
    pub axis: u32,
    pub endpoint: bool,
}

impl BoundaryFaceKey {
    pub const fn new(axis: u32, endpoint: bool) -> Self {
        Self { axis, endpoint }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BoundaryTermOrigin {
    HistoricalPointClause { clause: u16 },
    SemanticPointParameter { parameter: String },
    DeclaredFaceFamily { family: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BoundaryPointAnnotation {
    pub name: String,
    pub owner: Expr,
    pub semantic_context: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BoundaryFaceAnnotation {
    pub name: String,
    pub owner: Expr,
    pub remaining_axes: Vec<u32>,
    pub restrictions: Vec<(BoundaryFaceKey, String)>,
    pub origin: BoundaryTermOrigin,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BoundaryFaceDeclaration {
    pub fixed: BoundaryFaceKey,
    pub term: BoundaryFaceAnnotation,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeclaredBoundaryDiagram {
    pub owner: Expr,
    pub dimension: u32,
    pub parameters: Vec<BoundaryPointAnnotation>,
    pub faces: Vec<BoundaryFaceDeclaration>,
    /// Boundary data is an input to the attachment rule, not a list of
    /// independently charged computation exports.
    pub boundary_is_declared_input_not_credit: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BoundaryVariantError {
    #[error("a boundary map requires positive dimension")]
    ZeroDimension,
    #[error("boundary face axis {axis} is outside dimension {dimension}")]
    FaceAxisOutOfRange { axis: u32, dimension: u32 },
    #[error("boundary map does not contain exactly the two faces of every axis")]
    IncompleteFaceInventory,
    #[error("boundary point annotations must have unique names and the attachment owner")]
    InvalidPointParameters,
    #[error("boundary face annotation does not carry the attachment owner")]
    FaceOwnerMismatch,
    #[error("boundary face term has an invalid remaining-axis context")]
    InvalidRemainingAxes,
    #[error("boundary face term has an incomplete restriction inventory")]
    IncompleteRestrictionInventory,
    #[error("boundary face refers to a point annotation absent from the declared context")]
    MissingPointParameter,
    #[error("declared boundary faces disagree on a pairwise overlap")]
    IncoherentOverlap,
    #[error("formed path and declared boundary have different owners or dimensions")]
    AttachmentShapeMismatch,
    #[error("V1 accepts only an implicit constant boundary")]
    V1RequiresConstantBoundary,
    #[error("the sealed PathCon trace does not determine a V3 boundary rule")]
    TraceBoundaryUnderdetermined,
    #[error("formed-path source replay failed: {0}")]
    FormedSourceReplay(String),
    #[error("boundary token replay mismatch")]
    BoundaryReplayMismatch,
    #[error("attachment token replay mismatch")]
    AttachmentReplayMismatch,
    #[error("basis presentation is inconsistent")]
    BasisPresentationMismatch,
    #[error("the two declared boundary diagrams do not witness trace-projection noninjectivity")]
    NotTraceProjectionNonInjectivity,
}

fn tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(BOUNDARY_VARIANTS_FRAGMENT_VERSION, domain, payload))
        .expect("boundary-variant proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn expected_face_keys(dimension: u32) -> BTreeSet<BoundaryFaceKey> {
    (0..dimension)
        .flat_map(|axis| {
            [
                BoundaryFaceKey::new(axis, false),
                BoundaryFaceKey::new(axis, true),
            ]
        })
        .collect()
}

fn expected_restriction_keys(dimension: u32, fixed_axis: u32) -> BTreeSet<BoundaryFaceKey> {
    (0..dimension)
        .filter(|axis| *axis != fixed_axis)
        .flat_map(|axis| {
            [
                BoundaryFaceKey::new(axis, false),
                BoundaryFaceKey::new(axis, true),
            ]
        })
        .collect()
}

fn canonical_overlap_name(left: BoundaryFaceKey, right: BoundaryFaceKey, prefix: &str) -> String {
    let (first, second) = if left < right {
        (left, right)
    } else {
        (right, left)
    };
    format!(
        "{prefix}:overlap:{}:{}:{}:{}",
        first.axis,
        u8::from(first.endpoint),
        second.axis,
        u8::from(second.endpoint)
    )
}

pub fn constant_boundary(
    owner: Expr,
    dimension: u32,
    base_name: impl Into<String>,
    point_clause: u16,
) -> Result<DeclaredBoundaryDiagram, BoundaryVariantError> {
    if dimension == 0 {
        return Err(BoundaryVariantError::ZeroDimension);
    }
    let base_name = base_name.into();
    let faces = expected_face_keys(dimension)
        .into_iter()
        .map(|fixed| {
            let remaining_axes = (0..dimension)
                .filter(|axis| *axis != fixed.axis)
                .collect::<Vec<_>>();
            let restrictions = expected_restriction_keys(dimension, fixed.axis)
                .into_iter()
                .map(|key| (key, base_name.clone()))
                .collect::<Vec<_>>();
            BoundaryFaceDeclaration {
                fixed,
                term: BoundaryFaceAnnotation {
                    name: base_name.clone(),
                    owner: owner.clone(),
                    remaining_axes,
                    restrictions,
                    origin: BoundaryTermOrigin::HistoricalPointClause {
                        clause: point_clause,
                    },
                },
            }
        })
        .collect();
    Ok(DeclaredBoundaryDiagram {
        owner,
        dimension,
        parameters: Vec::new(),
        faces,
        boundary_is_declared_input_not_credit: true,
    })
}

pub fn interval_endpoint_boundary(
    owner: Expr,
    left: impl Into<String>,
    right: impl Into<String>,
    semantic_context: impl Into<String>,
) -> DeclaredBoundaryDiagram {
    let left = left.into();
    let right = right.into();
    let semantic_context = semantic_context.into();
    let parameters = [&left, &right]
        .into_iter()
        .map(|name| BoundaryPointAnnotation {
            name: name.clone(),
            owner: owner.clone(),
            semantic_context: semantic_context.clone(),
        })
        .collect::<Vec<_>>();
    let faces = [(false, left), (true, right)]
        .into_iter()
        .map(|(endpoint, name)| BoundaryFaceDeclaration {
            fixed: BoundaryFaceKey::new(0, endpoint),
            term: BoundaryFaceAnnotation {
                name: name.clone(),
                owner: owner.clone(),
                remaining_axes: Vec::new(),
                restrictions: Vec::new(),
                origin: BoundaryTermOrigin::SemanticPointParameter { parameter: name },
            },
        })
        .collect();
    DeclaredBoundaryDiagram {
        owner,
        dimension: 1,
        parameters,
        faces,
        boundary_is_declared_input_not_credit: true,
    }
}

/// A coherent general-position boundary diagram with distinct face families.
/// It is used to demonstrate that a dimension-only trace projection cannot
/// recover a unique boundary map.
pub fn generic_declared_boundary(
    owner: Expr,
    dimension: u32,
    prefix: impl AsRef<str>,
) -> Result<DeclaredBoundaryDiagram, BoundaryVariantError> {
    if dimension == 0 {
        return Err(BoundaryVariantError::ZeroDimension);
    }
    let prefix = prefix.as_ref();
    let faces = expected_face_keys(dimension)
        .into_iter()
        .map(|fixed| {
            let name = format!("{prefix}:face:{}:{}", fixed.axis, u8::from(fixed.endpoint));
            let remaining_axes = (0..dimension)
                .filter(|axis| *axis != fixed.axis)
                .collect::<Vec<_>>();
            let restrictions = expected_restriction_keys(dimension, fixed.axis)
                .into_iter()
                .map(|other| (other, canonical_overlap_name(fixed, other, prefix)))
                .collect::<Vec<_>>();
            BoundaryFaceDeclaration {
                fixed,
                term: BoundaryFaceAnnotation {
                    name: name.clone(),
                    owner: owner.clone(),
                    remaining_axes,
                    restrictions,
                    origin: BoundaryTermOrigin::DeclaredFaceFamily { family: name },
                },
            }
        })
        .collect();
    Ok(DeclaredBoundaryDiagram {
        owner,
        dimension,
        parameters: Vec::new(),
        faces,
        boundary_is_declared_input_not_credit: true,
    })
}

fn boundary_is_constant(map: &DeclaredBoundaryDiagram) -> bool {
    let Some(first) = map.faces.first() else {
        return false;
    };
    map.faces.iter().all(|face| {
        face.term.name == first.term.name
            && face
                .term
                .restrictions
                .iter()
                .all(|(_, value)| value == &first.term.name)
    })
}

fn validate_boundary(map: &DeclaredBoundaryDiagram) -> Result<(u32, bool), BoundaryVariantError> {
    if map.dimension == 0 {
        return Err(BoundaryVariantError::ZeroDimension);
    }
    if !map.boundary_is_declared_input_not_credit {
        return Err(BoundaryVariantError::BasisPresentationMismatch);
    }
    let parameter_names = map
        .parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect::<BTreeSet<_>>();
    if parameter_names.len() != map.parameters.len()
        || map.parameters.iter().any(|parameter| {
            parameter.owner != map.owner
                || parameter.name.is_empty()
                || parameter.semantic_context.is_empty()
        })
    {
        return Err(BoundaryVariantError::InvalidPointParameters);
    }

    let expected_faces = expected_face_keys(map.dimension);
    let actual_faces = map
        .faces
        .iter()
        .map(|face| face.fixed)
        .collect::<BTreeSet<_>>();
    if actual_faces != expected_faces || actual_faces.len() != map.faces.len() {
        return Err(BoundaryVariantError::IncompleteFaceInventory);
    }

    let face_by_key = map
        .faces
        .iter()
        .map(|face| (face.fixed, face))
        .collect::<BTreeMap<_, _>>();
    for face in &map.faces {
        if face.fixed.axis >= map.dimension {
            return Err(BoundaryVariantError::FaceAxisOutOfRange {
                axis: face.fixed.axis,
                dimension: map.dimension,
            });
        }
        if face.term.owner != map.owner || face.term.name.is_empty() {
            return Err(BoundaryVariantError::FaceOwnerMismatch);
        }
        let expected_axes = (0..map.dimension)
            .filter(|axis| *axis != face.fixed.axis)
            .collect::<Vec<_>>();
        if face.term.remaining_axes != expected_axes {
            return Err(BoundaryVariantError::InvalidRemainingAxes);
        }
        let expected_restrictions = expected_restriction_keys(map.dimension, face.fixed.axis);
        let actual_restrictions = face
            .term
            .restrictions
            .iter()
            .map(|(key, _)| *key)
            .collect::<BTreeSet<_>>();
        if actual_restrictions != expected_restrictions
            || actual_restrictions.len() != face.term.restrictions.len()
            || face
                .term
                .restrictions
                .iter()
                .any(|(_, value)| value.is_empty())
        {
            return Err(BoundaryVariantError::IncompleteRestrictionInventory);
        }
        if let BoundaryTermOrigin::SemanticPointParameter { parameter } = &face.term.origin
            && (!parameter_names.contains(parameter) || parameter != &face.term.name)
        {
            return Err(BoundaryVariantError::MissingPointParameter);
        }
    }

    let faces = map.faces.iter().collect::<Vec<_>>();
    let mut overlaps = 0u32;
    for (index, left) in faces.iter().enumerate() {
        for right in faces.iter().skip(index + 1) {
            if left.fixed.axis == right.fixed.axis {
                continue;
            }
            let left_restriction = left
                .term
                .restrictions
                .iter()
                .find_map(|(key, value)| (*key == right.fixed).then_some(value));
            let right_restriction = right
                .term
                .restrictions
                .iter()
                .find_map(|(key, value)| (*key == left.fixed).then_some(value));
            if left_restriction != right_restriction {
                return Err(BoundaryVariantError::IncoherentOverlap);
            }
            overlaps = overlaps
                .checked_add(1)
                .ok_or(BoundaryVariantError::BasisPresentationMismatch)?;
        }
    }

    // Keep the map lookup live in the proof path: every expected face must
    // resolve to the exact declaration already checked above.
    if expected_faces
        .iter()
        .any(|key| !face_by_key.contains_key(key))
    {
        return Err(BoundaryVariantError::IncompleteFaceInventory);
    }
    Ok((overlaps, boundary_is_constant(map)))
}

/// Opaque structural-check token.  It is serializable for projection, but
/// deliberately not deserializable or publicly constructible.  It proves the
/// declared owner/face inventory and overlap equations, not term typing.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CheckedBoundaryDiagramToken {
    map: DeclaredBoundaryDiagram,
    map_digest: String,
    face_count: u32,
    overlap_count: u32,
    constant_boundary: bool,
    derivation_hash: String,
}

impl CheckedBoundaryDiagramToken {
    pub fn map_digest(&self) -> &str {
        &self.map_digest
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub const fn dimension(&self) -> u32 {
        self.map.dimension
    }

    pub fn owner(&self) -> &Expr {
        &self.map.owner
    }

    pub const fn face_count(&self) -> u32 {
        self.face_count
    }

    pub const fn overlap_count(&self) -> u32 {
        self.overlap_count
    }

    pub const fn is_constant(&self) -> bool {
        self.constant_boundary
    }
}

pub fn check_declared_boundary_diagram(
    map: DeclaredBoundaryDiagram,
) -> Result<CheckedBoundaryDiagramToken, BoundaryVariantError> {
    let (overlap_count, constant_boundary) = validate_boundary(&map)?;
    let face_count = u32::try_from(map.faces.len())
        .map_err(|_| BoundaryVariantError::BasisPresentationMismatch)?;
    let map_digest = tagged_digest("declared-boundary-diagram", &map);
    let derivation_hash = tagged_digest(
        "checked-boundary-diagram",
        &(
            &map_digest,
            &map.owner,
            map.dimension,
            face_count,
            overlap_count,
            constant_boundary,
            map.boundary_is_declared_input_not_credit,
        ),
    );
    Ok(CheckedBoundaryDiagramToken {
        map,
        map_digest,
        face_count,
        overlap_count,
        constant_boundary,
        derivation_hash,
    })
}

pub fn replay_declared_boundary_diagram(
    token: &CheckedBoundaryDiagramToken,
) -> Result<(), BoundaryVariantError> {
    let replay = check_declared_boundary_diagram(token.map.clone())?;
    if &replay == token {
        Ok(())
    } else {
        Err(BoundaryVariantError::BoundaryReplayMismatch)
    }
}

fn trace_projection_digest(typing: &FormedPathTyping) -> String {
    tagged_digest(
        "formed-path-trace-projection",
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
        ),
    )
}

/// Rule-relative structural attachment token.  For V2 this binds a checked
/// diagram to an already formed owner/path declaration; it is not a token for
/// a typed boundary term or boundary-aware eliminator.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BoundaryAttachmentToken {
    theory: BoundaryTheory,
    theory_version: String,
    trace_projection_digest: String,
    boundary_map_digest: String,
    boundary_derivation_hash: String,
    dimension: u32,
    constant_boundary: bool,
    derivation_hash: String,
}

impl BoundaryAttachmentToken {
    pub const fn theory(&self) -> BoundaryTheory {
        self.theory
    }

    pub fn theory_version(&self) -> &str {
        &self.theory_version
    }

    pub fn trace_projection_digest(&self) -> &str {
        &self.trace_projection_digest
    }

    pub fn boundary_map_digest(&self) -> &str {
        &self.boundary_map_digest
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub const fn dimension(&self) -> u32 {
        self.dimension
    }

    pub const fn is_constant(&self) -> bool {
        self.constant_boundary
    }
}

pub fn issue_boundary_attachment(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
    theory: BoundaryTheory,
    boundary: &CheckedBoundaryDiagramToken,
) -> Result<BoundaryAttachmentToken, BoundaryVariantError> {
    super::validate_formed_source(signature, telescope, visible_library, typing)
        .map_err(|error| BoundaryVariantError::FormedSourceReplay(error.to_string()))?;
    replay_declared_boundary_diagram(boundary)?;
    if boundary.owner() != &typing.formation_normal_form || boundary.dimension() != typing.dimension
    {
        return Err(BoundaryVariantError::AttachmentShapeMismatch);
    }
    match theory {
        BoundaryTheory::V1ImplicitConstant if !boundary.is_constant() => {
            return Err(BoundaryVariantError::V1RequiresConstantBoundary);
        }
        BoundaryTheory::V3TraceDerived => {
            return Err(BoundaryVariantError::TraceBoundaryUnderdetermined);
        }
        BoundaryTheory::V1ImplicitConstant | BoundaryTheory::V2DeclaredBoundary => {}
    }
    let trace_projection_digest = trace_projection_digest(typing);
    let theory_version = theory.version().to_owned();
    let derivation_hash = tagged_digest(
        "boundary-attachment",
        &(
            theory,
            &theory_version,
            &trace_projection_digest,
            boundary.map_digest(),
            boundary.derivation_hash(),
            boundary.dimension(),
            boundary.is_constant(),
        ),
    );
    Ok(BoundaryAttachmentToken {
        theory,
        theory_version,
        trace_projection_digest,
        boundary_map_digest: boundary.map_digest().to_owned(),
        boundary_derivation_hash: boundary.derivation_hash().to_owned(),
        dimension: boundary.dimension(),
        constant_boundary: boundary.is_constant(),
        derivation_hash,
    })
}

pub fn replay_boundary_attachment(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
    boundary: &CheckedBoundaryDiagramToken,
    token: &BoundaryAttachmentToken,
) -> Result<(), BoundaryVariantError> {
    let replay = issue_boundary_attachment(
        signature,
        telescope,
        visible_library,
        typing,
        token.theory,
        boundary,
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(BoundaryVariantError::AttachmentReplayMismatch)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BoundaryBasisKey {
    Beta,
    PrincipalTransport { principal: u32 },
    TransportNaturality { principal: u32, probe: u32 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryBasisFormula {
    OnePlusDimensionSquared,
}

/// Raw finite key presentation bound to an attachment token.  V1 can compare
/// this presentation with incumbent typed terms.  V2 has no term realizer in
/// this module, so its token proves cardinality only.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BoundaryBasisPresentationToken {
    attachment_derivation_hash: String,
    formula: BoundaryBasisFormula,
    keys: Vec<BoundaryBasisKey>,
    beta_count: u32,
    principal_transport_count: u32,
    ordered_naturality_count: u64,
    total_count: u64,
    derivation_hash: String,
}

impl BoundaryBasisPresentationToken {
    pub const fn formula(&self) -> BoundaryBasisFormula {
        self.formula
    }

    pub fn keys(&self) -> &[BoundaryBasisKey] {
        &self.keys
    }

    pub const fn beta_count(&self) -> u32 {
        self.beta_count
    }

    pub const fn principal_transport_count(&self) -> u32 {
        self.principal_transport_count
    }

    pub const fn ordered_naturality_count(&self) -> u64 {
        self.ordered_naturality_count
    }

    pub const fn total_count(&self) -> u64 {
        self.total_count
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub const fn boundary_basis_cardinality(dimension: u32) -> u64 {
    let dimension = dimension as u64;
    1 + dimension * dimension
}

pub fn present_boundary_basis(
    attachment: &BoundaryAttachmentToken,
) -> Result<BoundaryBasisPresentationToken, BoundaryVariantError> {
    let dimension = attachment.dimension;
    if dimension == 0 || attachment.theory == BoundaryTheory::V3TraceDerived {
        return Err(BoundaryVariantError::BasisPresentationMismatch);
    }
    let mut keys = Vec::new();
    keys.push(BoundaryBasisKey::Beta);
    for principal in 0..dimension {
        keys.push(BoundaryBasisKey::PrincipalTransport { principal });
        for probe in 0..dimension {
            if principal != probe {
                keys.push(BoundaryBasisKey::TransportNaturality { principal, probe });
            }
        }
    }
    let unique = keys.iter().cloned().collect::<BTreeSet<_>>();
    let ordered_naturality_count = u64::from(dimension)
        .checked_mul(u64::from(dimension - 1))
        .ok_or(BoundaryVariantError::BasisPresentationMismatch)?;
    let total_count = boundary_basis_cardinality(dimension);
    if unique.len() != keys.len()
        || u64::try_from(keys.len()).ok() != Some(total_count)
        || total_count != 1 + u64::from(dimension) + ordered_naturality_count
    {
        return Err(BoundaryVariantError::BasisPresentationMismatch);
    }
    let formula = BoundaryBasisFormula::OnePlusDimensionSquared;
    let derivation_hash = tagged_digest(
        "boundary-basis-presentation",
        &(
            attachment.derivation_hash(),
            formula,
            &keys,
            1u32,
            dimension,
            ordered_naturality_count,
            total_count,
        ),
    );
    Ok(BoundaryBasisPresentationToken {
        attachment_derivation_hash: attachment.derivation_hash().to_owned(),
        formula,
        keys,
        beta_count: 1,
        principal_transport_count: dimension,
        ordered_naturality_count,
        total_count,
        derivation_hash,
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TraceBoundaryNonInjectivityWitness {
    trace_projection_digest: String,
    left_boundary_digest: String,
    right_boundary_digest: String,
    same_trace_projection: bool,
    distinct_declared_boundary_diagrams: bool,
    derivation_hash: String,
}

impl TraceBoundaryNonInjectivityWitness {
    pub fn trace_projection_digest(&self) -> &str {
        &self.trace_projection_digest
    }

    pub fn left_boundary_digest(&self) -> &str {
        &self.left_boundary_digest
    }

    pub fn right_boundary_digest(&self) -> &str {
        &self.right_boundary_digest
    }

    pub const fn same_trace_projection(&self) -> bool {
        self.same_trace_projection
    }

    pub const fn distinct_declared_boundary_diagrams(&self) -> bool {
        self.distinct_declared_boundary_diagrams
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn witness_trace_boundary_noninjectivity(
    typing: &FormedPathTyping,
    left: &CheckedBoundaryDiagramToken,
    right: &CheckedBoundaryDiagramToken,
) -> Result<TraceBoundaryNonInjectivityWitness, BoundaryVariantError> {
    replay_declared_boundary_diagram(left)?;
    replay_declared_boundary_diagram(right)?;
    if left.owner() != &typing.formation_normal_form
        || right.owner() != &typing.formation_normal_form
        || left.dimension() != typing.dimension
        || right.dimension() != typing.dimension
        || left.map_digest() == right.map_digest()
    {
        return Err(BoundaryVariantError::NotTraceProjectionNonInjectivity);
    }
    let trace_projection_digest = trace_projection_digest(typing);
    let derivation_hash = tagged_digest(
        "trace-boundary-noninjectivity",
        &(
            &trace_projection_digest,
            left.map_digest(),
            right.map_digest(),
            true,
            true,
        ),
    );
    Ok(TraceBoundaryNonInjectivityWitness {
        trace_projection_digest,
        left_boundary_digest: left.map_digest().to_owned(),
        right_boundary_digest: right.map_digest().to_owned(),
        same_trace_projection: true,
        distinct_declared_boundary_diagrams: true,
        derivation_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tdc1::elaborate_formed_path;

    fn historical_source(step: u32) -> (SealedSignature, Telescope, FormedPathTyping) {
        let signature = SealedSignature::from_telescopes(
            (1..step)
                .map(|index| (index, Telescope::reference(index)))
                .collect(),
        );
        let telescope = Telescope::reference(step);
        let (typing, _) = elaborate_formed_path(&signature, &telescope, step - 1)
            .expect("historical source elaborates");
        (signature, telescope, typing)
    }

    #[test]
    fn constant_and_endpoint_boundaries_are_checked_and_replayed() {
        let (_, _, s1) = historical_source(5);
        let constant = check_declared_boundary_diagram(
            constant_boundary(s1.formation_normal_form.clone(), 1, "base", 1)
                .expect("constant boundary"),
        )
        .expect("checked constant boundary diagram");
        replay_declared_boundary_diagram(&constant).expect("constant replay");
        assert!(constant.is_constant());

        let (_, _, trunc) = historical_source(6);
        let endpoints = check_declared_boundary_diagram(interval_endpoint_boundary(
            trunc.formation_normal_form.clone(),
            "x",
            "y",
            "A:Type; x,y:Trunc(A)",
        ))
        .expect("checked endpoint diagram");
        replay_declared_boundary_diagram(&endpoints).expect("endpoint replay");
        assert!(!endpoints.is_constant());
    }

    #[test]
    fn v1_rejects_squash_shape_while_v2_accepts_it() {
        let (signature, telescope, typing) = historical_source(6);
        let boundary = check_declared_boundary_diagram(interval_endpoint_boundary(
            typing.formation_normal_form.clone(),
            "x",
            "y",
            "A:Type; x,y:Trunc(A)",
        ))
        .expect("checked endpoint diagram");
        assert_eq!(
            issue_boundary_attachment(
                &signature,
                &telescope,
                5,
                &typing,
                BoundaryTheory::V1ImplicitConstant,
                &boundary,
            ),
            Err(BoundaryVariantError::V1RequiresConstantBoundary)
        );
        let attachment = issue_boundary_attachment(
            &signature,
            &telescope,
            5,
            &typing,
            BoundaryTheory::V2DeclaredBoundary,
            &boundary,
        )
        .expect("V2 accepts endpoint boundary");
        replay_boundary_attachment(&signature, &telescope, 5, &typing, &boundary, &attachment)
            .expect("attachment replay");
        assert_eq!(
            present_boundary_basis(&attachment).unwrap().total_count(),
            2
        );
    }

    #[test]
    fn declared_general_position_checks_pairwise_overlaps() {
        let (_, _, typing) = historical_source(8);
        let map = generic_declared_boundary(
            typing.formation_normal_form.clone(),
            typing.dimension,
            "generic",
        )
        .expect("generic map");
        let token =
            check_declared_boundary_diagram(map.clone()).expect("coherent general boundary");
        assert!(!token.is_constant());

        let mut broken = map;
        let (_, restriction) = broken.faces[0]
            .term
            .restrictions
            .iter_mut()
            .next()
            .expect("higher boundary has overlaps");
        restriction.push_str(":broken");
        assert_eq!(
            check_declared_boundary_diagram(broken),
            Err(BoundaryVariantError::IncoherentOverlap)
        );
    }

    #[test]
    fn basis_partition_and_trace_noninjectivity_are_machine_checked() {
        for step in [5, 6, 7, 8] {
            let (signature, telescope, typing) = historical_source(step);
            let constant = check_declared_boundary_diagram(
                constant_boundary(
                    typing.formation_normal_form.clone(),
                    typing.dimension,
                    "base",
                    1,
                )
                .expect("constant map"),
            )
            .expect("checked constant diagram");
            let general = check_declared_boundary_diagram(
                generic_declared_boundary(
                    typing.formation_normal_form.clone(),
                    typing.dimension,
                    "other",
                )
                .expect("general map"),
            )
            .expect("checked general diagram");
            let witness = witness_trace_boundary_noninjectivity(&typing, &constant, &general)
                .expect("same trace has two boundaries");
            assert!(witness.same_trace_projection());
            assert!(witness.distinct_declared_boundary_diagrams());

            let attachment = issue_boundary_attachment(
                &signature,
                &telescope,
                step - 1,
                &typing,
                BoundaryTheory::V2DeclaredBoundary,
                &constant,
            )
            .expect("declared attachment");
            let basis = present_boundary_basis(&attachment).expect("basis presentation");
            assert_eq!(
                basis.total_count(),
                boundary_basis_cardinality(typing.dimension)
            );
            assert_eq!(
                basis.total_count(),
                1 + u64::from(typing.dimension)
                    + u64::from(typing.dimension) * u64::from(typing.dimension - 1)
            );
        }
    }

    #[test]
    fn v3_refuses_to_invent_a_rule_from_dimension_only_trace() {
        let (signature, telescope, typing) = historical_source(5);
        let boundary = check_declared_boundary_diagram(
            constant_boundary(typing.formation_normal_form.clone(), 1, "base", 1)
                .expect("constant boundary"),
        )
        .expect("checked boundary diagram");
        assert_eq!(
            issue_boundary_attachment(
                &signature,
                &telescope,
                4,
                &typing,
                BoundaryTheory::V3TraceDerived,
                &boundary,
            ),
            Err(BoundaryVariantError::TraceBoundaryUnderdetermined)
        );
    }
}
