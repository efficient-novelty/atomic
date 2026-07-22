//! A small, closed cubical fragment for the TDC formed-`PathCon` audit.
//!
//! This is deliberately not an extension of the global shallow `Expr` AST.
//! A cubical judgement is always rooted in a kernel-checked
//! [`FormedPathTyping`].  Interval variables, face formulae, terms and opaque
//! realization tokens therefore cannot make an untyped telescope look typed.
//!
//! The face lattice is constructive: formulae are canonical finite DNFs of
//! endpoint assignments.  No complement or excluded-middle operation is
//! exposed.  `coe`, `hcom`, and the `PathCon` eliminator are checked by this
//! module before normalization or token issuance.

use crate::elaborate::SealedSignature;
use crate::equality::univalent_equality;
use crate::tdc1::{FormedPathTyping, PathSchemaKey, elaborate_formed_path};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub mod boundary_variants;
pub mod typed_boundary;

pub const CUBICAL_FRAGMENT_VERSION: &str = "tdc1-cubical-fragment-v3";
pub const HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION: &str = "hist-cert-path-family-fragment-v1";
/// Additive d=1 fragment for a constructor whose two faces are distinct
/// ordinary-context points.  It has a separate digest domain so incumbent
/// constant-boundary realizations remain archival.
pub const TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION: &str =
    "tdc1-trunc-endpoint-dependent-pathcon-realizer-v1";
/// Explicit theory-relative rule used by this experiment.  Kernel v1 and the
/// shallow two-clause `d=4` surface do not derive an owner point `a : A`.
/// The experiment conditionally fixes an implicit base `a`, a free
/// `p : I^d -> A`, and definitional constant boundary `p|partial I^d = a`.
pub const PATHCON_ATTACHMENT_AXIOM_VERSION: &str =
    "tdc1-pathcon-attachment-implicit-base-constant-boundary-theory-axiom-v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", content = "index", rename_all = "snake_case")]
pub enum Dim {
    Zero,
    One,
    Var(u16),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Endpoint {
    pub variable: u16,
    pub value: bool,
}

impl Endpoint {
    pub const fn zero(variable: u16) -> Self {
        Self {
            variable,
            value: false,
        }
    }

    pub const fn one(variable: u16) -> Self {
        Self {
            variable,
            value: true,
        }
    }
}

type Face = Vec<Endpoint>;

/// Canonical constructive cofibration: a disjunction of conjunctions of
/// endpoint equations.  The empty outer vector is false; a singleton empty
/// conjunction is true.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Cofibration {
    clauses: Vec<Face>,
}

impl Cofibration {
    pub fn false_formula() -> Self {
        Self { clauses: vec![] }
    }

    pub fn true_formula() -> Self {
        Self {
            clauses: vec![vec![]],
        }
    }

    pub fn endpoint(endpoint: Endpoint) -> Self {
        Self {
            clauses: vec![vec![endpoint]],
        }
    }

    pub fn conjunction(endpoints: impl IntoIterator<Item = Endpoint>) -> Self {
        match canonical_face(endpoints) {
            Some(face) => Self::from_canonical_faces(vec![face]),
            None => Self::false_formula(),
        }
    }

    pub fn disjunction(
        clauses: impl IntoIterator<Item = impl IntoIterator<Item = Endpoint>>,
    ) -> Self {
        let faces = clauses
            .into_iter()
            .filter_map(canonical_face)
            .collect::<Vec<_>>();
        Self::from_canonical_faces(faces)
    }

    pub fn is_false(&self) -> bool {
        self.clauses.is_empty()
    }

    pub fn is_true(&self) -> bool {
        self.clauses == [Vec::<Endpoint>::new()]
    }

    pub fn clauses(&self) -> &[Vec<Endpoint>] {
        &self.clauses
    }

    pub fn join(&self, other: &Self) -> Self {
        Self::from_canonical_faces(self.clauses.iter().chain(&other.clauses).cloned().collect())
    }

    pub fn meet(&self, other: &Self) -> Self {
        let mut faces = Vec::new();
        for left in &self.clauses {
            for right in &other.clauses {
                if let Some(face) = canonical_face(left.iter().chain(right).copied()) {
                    faces.push(face);
                }
            }
        }
        Self::from_canonical_faces(faces)
    }

    /// Dimension substitution in a face formula.  Endpoint replacement
    /// decides the affected equation constructively; variable replacement
    /// renames it and canonicalization handles collisions and contradictions.
    pub fn substitute(&self, variable: u16, replacement: Dim) -> Self {
        let mut faces = Vec::new();
        for clause in &self.clauses {
            let mut substituted = Vec::new();
            let mut satisfied = true;
            for endpoint in clause {
                if endpoint.variable != variable {
                    substituted.push(*endpoint);
                    continue;
                }
                match replacement {
                    Dim::Zero if !endpoint.value => {}
                    Dim::One if endpoint.value => {}
                    Dim::Zero | Dim::One => {
                        satisfied = false;
                        break;
                    }
                    Dim::Var(target) => substituted.push(Endpoint {
                        variable: target,
                        value: endpoint.value,
                    }),
                }
            }
            if satisfied {
                if let Some(face) = canonical_face(substituted) {
                    faces.push(face);
                }
            }
        }
        Self::from_canonical_faces(faces)
    }

    /// Constructively checks `self => other` between canonical finite DNFs.
    pub fn implies(&self, other: &Self) -> bool {
        self.clauses.iter().all(|source| {
            other
                .clauses
                .iter()
                .any(|target| face_is_subset(target, source))
        })
    }

    fn variables(&self) -> impl Iterator<Item = u16> + '_ {
        self.clauses
            .iter()
            .flatten()
            .map(|endpoint| endpoint.variable)
    }

    fn from_canonical_faces(mut clauses: Vec<Face>) -> Self {
        clauses.sort_by(|left, right| left.len().cmp(&right.len()).then(left.cmp(right)));
        clauses.dedup();

        // If an earlier (shorter) conjunction is a subset of this one, the
        // longer disjunct is absorbed: a || (a && b) = a.
        let mut irredundant: Vec<Face> = Vec::new();
        for face in clauses {
            if !irredundant.iter().any(|kept| face_is_subset(kept, &face)) {
                irredundant.push(face);
            }
        }
        Self {
            clauses: irredundant,
        }
    }
}

fn canonical_face(endpoints: impl IntoIterator<Item = Endpoint>) -> Option<Face> {
    let mut assignments = BTreeMap::new();
    for endpoint in endpoints {
        match assignments.insert(endpoint.variable, endpoint.value) {
            Some(previous) if previous != endpoint.value => return None,
            _ => {}
        }
    }
    Some(
        assignments
            .into_iter()
            .map(|(variable, value)| Endpoint { variable, value })
            .collect(),
    )
}

fn face_is_subset(subset: &[Endpoint], superset: &[Endpoint]) -> bool {
    subset
        .iter()
        .all(|endpoint| superset.binary_search(endpoint).is_ok())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CubicalContext {
    interval_count: u16,
    assumption: Cofibration,
}

impl CubicalContext {
    pub fn total(interval_count: u16) -> Self {
        Self {
            interval_count,
            assumption: Cofibration::true_formula(),
        }
    }

    pub fn new(interval_count: u16, assumption: Cofibration) -> Result<Self, CubicalTypeError> {
        check_face_scope(interval_count, &assumption)?;
        Ok(Self {
            interval_count,
            assumption,
        })
    }

    pub fn interval_count(&self) -> u16 {
        self.interval_count
    }

    pub fn assumption(&self) -> &Cofibration {
        &self.assumption
    }

    pub fn under(&self, face: &Cofibration) -> Result<Self, CubicalTypeError> {
        check_face_scope(self.interval_count, face)?;
        Self::new(self.interval_count, self.assumption.meet(face))
    }

    pub fn substitute(&self, variable: u16, replacement: Dim) -> Result<Self, CubicalTypeError> {
        check_dim_scope(self, replacement)?;
        Self::new(
            self.interval_count,
            self.assumption.substitute(variable, replacement),
        )
    }

    fn extend_interval(&self) -> Self {
        Self {
            interval_count: self.interval_count + 1,
            assumption: self.assumption.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PointExpr {
    Base {
        owner: Expr,
    },
    /// A point supplied by the endpoint-only formal premise context.
    /// Historical Trunc uses parameters 1 and 2 for its declared `x` and
    /// `y` endpoints.  Ordinary context-free inference rejects this syntax;
    /// the registered issuer in `typed_boundary` binds the indices to exact
    /// replayed context lookups before they can contribute to evidence.
    BoundaryParameter {
        owner: Expr,
        parameter: u16,
    },
    /// A typed neutral point `name : owner`.  Neutrals make the eliminator
    /// meaningful beyond constructor redexes: elimination is typed in the
    /// motive fiber at the neutral, but cannot compute until the scrutinee is
    /// exposed as `Base` or `PathConstructor`.
    Neutral {
        owner: Expr,
        name: String,
    },
    PathConstructor {
        owner: Expr,
        dimension: u32,
        coordinates: Vec<Dim>,
    },
    /// The d=1 endpoint-dependent constructor.  This is deliberately a new
    /// variant rather than a field added to `PathConstructor`: the latter's
    /// serialized form and all archival constant-boundary token hashes must
    /// remain byte-identical.
    EndpointPathConstructor {
        owner: Expr,
        coordinate: Dim,
        zero: Box<PointExpr>,
        one: Box<PointExpr>,
    },
}

impl PointExpr {
    fn owner(&self) -> &Expr {
        match self {
            Self::Base { owner }
            | Self::BoundaryParameter { owner, .. }
            | Self::Neutral { owner, .. }
            | Self::PathConstructor { owner, .. }
            | Self::EndpointPathConstructor { owner, .. } => owner,
        }
    }

    fn substitute(&self, variable: u16, replacement: Dim) -> Self {
        match self {
            Self::Base { .. } | Self::BoundaryParameter { .. } | Self::Neutral { .. } => {
                self.clone()
            }
            Self::PathConstructor {
                owner,
                dimension,
                coordinates,
            } => {
                let coordinates = coordinates
                    .iter()
                    .map(|coordinate| substitute_dimension(*coordinate, variable, replacement))
                    .collect::<Vec<_>>();
                // `PathCon(d)` has constant cube boundary by formation: every
                // endpoint restriction is definitionally the owner basepoint.
                if coordinates
                    .iter()
                    .any(|coordinate| matches!(coordinate, Dim::Zero | Dim::One))
                {
                    Self::Base {
                        owner: owner.clone(),
                    }
                } else {
                    Self::PathConstructor {
                        owner: owner.clone(),
                        dimension: *dimension,
                        coordinates,
                    }
                }
            }
            Self::EndpointPathConstructor {
                owner,
                coordinate,
                zero,
                one,
            } => {
                let zero = zero.substitute(variable, replacement);
                let one = one.substitute(variable, replacement);
                match substitute_dimension(*coordinate, variable, replacement) {
                    Dim::Zero => zero,
                    Dim::One => one,
                    coordinate @ Dim::Var(_) => Self::EndpointPathConstructor {
                        owner: owner.clone(),
                        coordinate,
                        zero: Box::new(zero),
                        one: Box::new(one),
                    },
                }
            }
        }
    }

    fn depends_on(&self, variable: u16) -> bool {
        match self {
            Self::PathConstructor { coordinates, .. } => coordinates.contains(&Dim::Var(variable)),
            Self::EndpointPathConstructor {
                coordinate,
                zero,
                one,
                ..
            } => {
                *coordinate == Dim::Var(variable)
                    || zero.depends_on(variable)
                    || one.depends_on(variable)
            }
            Self::Base { .. } | Self::BoundaryParameter { .. } | Self::Neutral { .. } => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Motive {
    pub owner: Expr,
    pub name: String,
}

/// Opaque reference to a formal endpoint-evaluation hypothesis in an
/// [`EndpointSchemaPremiseContext`].  This is a sequent variable, not a term
/// constructor asserting that an arbitrary motive fiber is inhabited.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EndpointEvaluationHypothesisRef {
    premise_context_derivation_hash: String,
    premise_derivation_hash: String,
    motive: Motive,
    point: PointExpr,
}

/// Opaque reference to the formal PathP method hypothesis `q`.  Its two
/// boundary references are part of the reference itself, so dimension
/// substitution computes `q(0)`/`q(1)` to the ledger's `e0`/`e1` entries.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EndpointMethodHypothesisRef {
    premise_context_derivation_hash: String,
    premise_derivation_hash: String,
    motive: Motive,
    path: PointExpr,
    zero: EndpointEvaluationHypothesisRef,
    one: EndpointEvaluationHypothesisRef,
}

/// Endpoint-only formal sequent context
/// `P; e0 : P(x); e1 : P(y); q : PathP(P ∘ squash) e0 e1`.
///
/// Formation of this ledger declares hypotheses; it does not derive their
/// inhabitants and is therefore not an additional axiom.  Its fields are
/// private and every accepted reference is checked against its exact,
/// source-bound reconstruction.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct EndpointSchemaPremiseContext {
    source_digest: String,
    motive: Motive,
    zero_parameter: u16,
    one_parameter: u16,
    zero_evaluation: EndpointEvaluationHypothesisRef,
    one_evaluation: EndpointEvaluationHypothesisRef,
    method_path: PointExpr,
    method_premise_derivation_hash: String,
    derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CubicalType {
    Owner { owner: Expr },
    MotiveFiber { motive: Motive, point: PointExpr },
    DimensionFunction { binder: u16, body: Box<CubicalType> },
}

impl CubicalType {
    pub fn substitute(&self, variable: u16, replacement: Dim) -> Self {
        match self {
            Self::Owner { .. } => self.clone(),
            Self::MotiveFiber { motive, point } => Self::MotiveFiber {
                motive: motive.clone(),
                point: point.substitute(variable, replacement),
            },
            Self::DimensionFunction { binder, body } if *binder == variable => self.clone(),
            Self::DimensionFunction { binder, body } => Self::DimensionFunction {
                binder: *binder,
                body: Box::new(body.substitute(variable, replacement)),
            },
        }
    }

    pub fn depends_on(&self, variable: u16) -> bool {
        match self {
            Self::Owner { .. } => false,
            Self::MotiveFiber { point, .. } => point.depends_on(variable),
            Self::DimensionFunction { binder, body } => {
                *binder != variable && body.depends_on(variable)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Tube {
    pub face: Cofibration,
    pub body: Box<CubicalTerm>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CubicalTerm {
    Point {
        point: PointExpr,
    },
    MotiveBase {
        motive: Motive,
    },
    /// Reference to an endpoint evaluation declared in the endpoint-only
    /// formal premise context.  Ordinary context-free inference rejects it.
    EndpointEvaluationHypothesis {
        reference: EndpointEvaluationHypothesisRef,
    },
    /// Primitive method constructor
    /// `q(i_1,...,i_d) : P(p(i_1,...,i_d))`.  Its `base` is an actual term,
    /// and dimension substitution definitionally sends every constructor
    /// face of this term to that base.  There is no caller-supplied boundary
    /// assertion.
    PathMethod {
        motive: Motive,
        path: PointExpr,
        base: Box<CubicalTerm>,
    },
    /// Reference to the PathP-shaped method `q` declared in the endpoint
    /// formal premise context.  Dimension substitution at 0/1 computes to
    /// its recorded `e0`/`e1` references.
    EndpointMethodHypothesis {
        reference: EndpointMethodHypothesisRef,
    },
    /// Motive-typed `PathCon` elimination.  On a constructor scrutinee this
    /// normalizes to `method`.
    PathElim {
        motive: Motive,
        base: Box<CubicalTerm>,
        method: Box<CubicalTerm>,
        scrutinee: Box<CubicalTerm>,
    },
    /// Eliminator for the endpoint-dependent constructor.  Its method is a
    /// reference into the endpoint premise ledger, so a boundary-parameter
    /// scrutinee computes to `e0`/`e1`, a constructor computes to `q`, and
    /// any other typed neutral remains stuck while retaining both inputs.
    EndpointPathElim {
        motive: Motive,
        method: Box<CubicalTerm>,
        scrutinee: Box<CubicalTerm>,
    },
    /// Canonical stuck form of the endpoint eliminator at a point that is
    /// neither a declared endpoint nor the constructor.  It retains the
    /// exact formal method reference and scrutinee, so substitution cannot
    /// erase or manufacture its premise provenance.
    EndpointElimNeutral {
        motive: Motive,
        /// The original opaque `q` premise identity, not its current
        /// dimension-wise evaluation.  Keeping the reference separate from
        /// term substitution prevents a face restriction from turning the
        /// stuck eliminator into an arbitrary `e0 : P(x)`/`e1 : P(y)` term
        /// that could then be misused at an unrelated neutral point.
        method: EndpointMethodHypothesisRef,
        scrutinee: Box<CubicalTerm>,
    },
    /// Dimension abstraction over a fresh interval variable.  The binder is
    /// checked to be exactly the next context index.
    DimLambda {
        binder: u16,
        body: Box<CubicalTerm>,
    },
    DimApp {
        function: Box<CubicalTerm>,
        argument: Dim,
    },
    Coe {
        /// A type expression in the extended context `Gamma,i`.  `binder`
        /// must be the fresh next interval index; the input is checked at
        /// `family[i:=from]` and the result at `family[i:=to]`.
        family: CubicalType,
        binder: u16,
        from: Dim,
        to: Dim,
        term: Box<CubicalTerm>,
    },
    Hcom {
        family: CubicalType,
        binder: u16,
        from: Dim,
        to: Dim,
        cap: Box<CubicalTerm>,
        tubes: Vec<Tube>,
    },
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CubicalTypeError {
    #[error("interval variable {variable} is out of scope for {interval_count} variables")]
    IntervalOutOfScope { variable: u16, interval_count: u16 },
    #[error("PathCon coordinate arity is {found}, expected {expected}")]
    PathCoordinateArity { expected: u32, found: usize },
    #[error("term has type {found:?}, expected {expected:?}")]
    TypeMismatch {
        expected: CubicalType,
        found: CubicalType,
    },
    #[error("motive owner does not match the point owner")]
    MotiveOwnerMismatch,
    #[error("PathCon method must be indexed by a constructor point")]
    MethodNotAtConstructor,
    #[error("PathCon method boundary does not normalize to the eliminator base")]
    MethodBoundaryMismatch,
    #[error("endpoint-dependent PathCon is restricted to dimension one")]
    EndpointPathDimensionMismatch,
    #[error("endpoint-dependent PathCon endpoints do not inhabit its owner")]
    EndpointOwnerMismatch,
    #[error("endpoint-dependent PathCon method must be indexed by an endpoint constructor")]
    EndpointMethodNotAtConstructor,
    #[error("endpoint-dependent PathCon method images do not match the declared endpoints")]
    EndpointMethodBoundaryMismatch,
    #[error("endpoint-dependent PathCon requires two distinct nonzero boundary parameters")]
    EndpointBoundaryParameterMismatch,
    #[error("endpoint hypothesis reference requires its formal premise context")]
    EndpointPremiseContextRequired,
    #[error("endpoint formal premise context or source does not match the term")]
    EndpointPremiseContextMismatch,
    #[error("endpoint contraction has incoherent endpoint-evaluation references")]
    EndpointPremiseContractionMismatch,
    #[error("stuck endpoint eliminator scrutinee is a reducible endpoint or constructor")]
    EndpointNeutralScrutineeReducible,
    #[error("hcom tube does not agree with its cap on the starting face")]
    CapFaceMismatch,
    #[error("hcom tube bodies disagree on a nonempty overlap")]
    TubeOverlapMismatch,
    #[error("coe/hcom binder {found} is not the fresh interval index {expected}")]
    CompositionBinderNotFresh { expected: u16, found: u16 },
    #[error("dimension application requires a dimension-function term")]
    ExpectedDimensionFunction,
    #[error("hcom is homogeneous; dependent composition must use coe")]
    DependentHcomUnsupported,
    #[error("typed nested hcom staging did not normalize to the direct union")]
    InvalidStagedUnion,
    #[error("realization context must be total and have exactly {expected} interval variables")]
    NonStandardRealizationContext { expected: u32 },
    #[error("normalized typed term is not a TDC path-schema realizer")]
    NotPathSchemaRealizer,
    #[error("derived path-schema direction is outside dimension {dimension}")]
    SchemaDirectionOutOfRange { dimension: u32 },
    #[error("historical path-family projection supports dimensions 1 through 3, found {found}")]
    HistoricalFamilyDimensionOutsideFragment { found: u32 },
    #[error("cubical realization source does not match the formed-path typing")]
    SourceMismatch,
    #[error("formed-path source replay failed: {reason}")]
    FormedSourceReplayFailed { reason: String },
}

fn check_dim_scope(context: &CubicalContext, dimension: Dim) -> Result<(), CubicalTypeError> {
    if let Dim::Var(variable) = dimension {
        if variable >= context.interval_count {
            return Err(CubicalTypeError::IntervalOutOfScope {
                variable,
                interval_count: context.interval_count,
            });
        }
    }
    Ok(())
}

fn check_face_scope(interval_count: u16, face: &Cofibration) -> Result<(), CubicalTypeError> {
    for variable in face.variables() {
        if variable >= interval_count {
            return Err(CubicalTypeError::IntervalOutOfScope {
                variable,
                interval_count,
            });
        }
    }
    Ok(())
}

fn check_point_scope_with_optional_endpoint_premises(
    context: &CubicalContext,
    endpoint_premises: Option<&EndpointSchemaPremiseContext>,
    point: &PointExpr,
) -> Result<(), CubicalTypeError> {
    match point {
        PointExpr::Base { .. } | PointExpr::Neutral { .. } => {}
        PointExpr::BoundaryParameter { .. } => {
            let premises =
                endpoint_premises.ok_or(CubicalTypeError::EndpointPremiseContextRequired)?;
            premises.accepts_endpoint_point(point)?;
        }
        PointExpr::PathConstructor {
            dimension,
            coordinates,
            ..
        } => {
            if coordinates.len() != *dimension as usize {
                return Err(CubicalTypeError::PathCoordinateArity {
                    expected: *dimension,
                    found: coordinates.len(),
                });
            }
            for coordinate in coordinates {
                check_dim_scope(context, *coordinate)?;
            }
        }
        PointExpr::EndpointPathConstructor {
            owner,
            coordinate,
            zero,
            one,
        } => {
            let premises =
                endpoint_premises.ok_or(CubicalTypeError::EndpointPremiseContextRequired)?;
            premises.accepts_endpoint_point(point)?;
            check_dim_scope(context, *coordinate)?;
            check_point_scope_with_optional_endpoint_premises(context, endpoint_premises, zero)?;
            check_point_scope_with_optional_endpoint_premises(context, endpoint_premises, one)?;
            if zero.owner() != owner || one.owner() != owner {
                return Err(CubicalTypeError::EndpointOwnerMismatch);
            }
        }
    }
    Ok(())
}

fn check_type_scope_with_optional_endpoint_premises(
    context: &CubicalContext,
    endpoint_premises: Option<&EndpointSchemaPremiseContext>,
    ty: &CubicalType,
) -> Result<(), CubicalTypeError> {
    match ty {
        CubicalType::Owner { .. } => Ok(()),
        CubicalType::MotiveFiber { motive, point } => {
            check_point_scope_with_optional_endpoint_premises(context, endpoint_premises, point)?;
            if point.owner() != &motive.owner {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            Ok(())
        }
        CubicalType::DimensionFunction { binder, body } => {
            if *binder != context.interval_count {
                return Err(CubicalTypeError::IntervalOutOfScope {
                    variable: *binder,
                    interval_count: context.interval_count,
                });
            }
            check_type_scope_with_optional_endpoint_premises(
                &context.extend_interval(),
                endpoint_premises,
                body,
            )
        }
    }
}

fn expect_type(found: CubicalType, expected: &CubicalType) -> Result<(), CubicalTypeError> {
    if &found == expected {
        Ok(())
    } else {
        Err(CubicalTypeError::TypeMismatch {
            expected: expected.clone(),
            found,
        })
    }
}

fn substitute_dimension(dimension: Dim, variable: u16, replacement: Dim) -> Dim {
    match dimension {
        Dim::Var(found) if found == variable => replacement,
        _ => dimension,
    }
}

fn lower_dimension_after_removal(dimension: Dim, removed: u16) -> Dim {
    match dimension {
        Dim::Var(variable) if variable > removed => Dim::Var(variable - 1),
        _ => dimension,
    }
}

fn lower_point_after_removal(point: &PointExpr, removed: u16) -> PointExpr {
    match point {
        PointExpr::Base { .. }
        | PointExpr::BoundaryParameter { .. }
        | PointExpr::Neutral { .. } => point.clone(),
        PointExpr::PathConstructor {
            owner,
            dimension,
            coordinates,
        } => PointExpr::PathConstructor {
            owner: owner.clone(),
            dimension: *dimension,
            coordinates: coordinates
                .iter()
                .map(|coordinate| lower_dimension_after_removal(*coordinate, removed))
                .collect(),
        },
        PointExpr::EndpointPathConstructor {
            owner,
            coordinate,
            zero,
            one,
        } => PointExpr::EndpointPathConstructor {
            owner: owner.clone(),
            coordinate: lower_dimension_after_removal(*coordinate, removed),
            zero: Box::new(lower_point_after_removal(zero, removed)),
            one: Box::new(lower_point_after_removal(one, removed)),
        },
    }
}

fn lower_type_after_removal(ty: &CubicalType, removed: u16) -> CubicalType {
    match ty {
        CubicalType::Owner { .. } => ty.clone(),
        CubicalType::MotiveFiber { motive, point } => CubicalType::MotiveFiber {
            motive: motive.clone(),
            point: lower_point_after_removal(point, removed),
        },
        CubicalType::DimensionFunction { binder, body } => CubicalType::DimensionFunction {
            binder: if *binder > removed {
                *binder - 1
            } else {
                *binder
            },
            body: Box::new(lower_type_after_removal(body, removed)),
        },
    }
}

fn lower_face_after_removal(face: &Cofibration, removed: u16) -> Cofibration {
    Cofibration::disjunction(face.clauses.iter().map(|clause| {
        clause.iter().map(|endpoint| Endpoint {
            variable: if endpoint.variable > removed {
                endpoint.variable - 1
            } else {
                endpoint.variable
            },
            value: endpoint.value,
        })
    }))
}

fn lower_endpoint_evaluation_reference(
    reference: &EndpointEvaluationHypothesisRef,
    removed: u16,
) -> EndpointEvaluationHypothesisRef {
    EndpointEvaluationHypothesisRef {
        premise_context_derivation_hash: reference.premise_context_derivation_hash.clone(),
        premise_derivation_hash: reference.premise_derivation_hash.clone(),
        motive: reference.motive.clone(),
        point: lower_point_after_removal(&reference.point, removed),
    }
}

fn lower_endpoint_method_reference(
    reference: &EndpointMethodHypothesisRef,
    removed: u16,
) -> EndpointMethodHypothesisRef {
    EndpointMethodHypothesisRef {
        premise_context_derivation_hash: reference.premise_context_derivation_hash.clone(),
        premise_derivation_hash: reference.premise_derivation_hash.clone(),
        motive: reference.motive.clone(),
        path: lower_point_after_removal(&reference.path, removed),
        zero: lower_endpoint_evaluation_reference(&reference.zero, removed),
        one: lower_endpoint_evaluation_reference(&reference.one, removed),
    }
}

fn lower_term_after_removal(term: &CubicalTerm, removed: u16) -> CubicalTerm {
    match term {
        CubicalTerm::Point { point } => CubicalTerm::Point {
            point: lower_point_after_removal(point, removed),
        },
        CubicalTerm::MotiveBase { .. } => term.clone(),
        CubicalTerm::EndpointEvaluationHypothesis { reference } => {
            CubicalTerm::EndpointEvaluationHypothesis {
                reference: lower_endpoint_evaluation_reference(reference, removed),
            }
        }
        CubicalTerm::PathMethod { motive, path, base } => CubicalTerm::PathMethod {
            motive: motive.clone(),
            path: lower_point_after_removal(path, removed),
            base: Box::new(lower_term_after_removal(base, removed)),
        },
        CubicalTerm::EndpointMethodHypothesis { reference } => {
            CubicalTerm::EndpointMethodHypothesis {
                reference: lower_endpoint_method_reference(reference, removed),
            }
        }
        CubicalTerm::PathElim {
            motive,
            base,
            method,
            scrutinee,
        } => CubicalTerm::PathElim {
            motive: motive.clone(),
            base: Box::new(lower_term_after_removal(base, removed)),
            method: Box::new(lower_term_after_removal(method, removed)),
            scrutinee: Box::new(lower_term_after_removal(scrutinee, removed)),
        },
        CubicalTerm::EndpointPathElim {
            motive,
            method,
            scrutinee,
        } => CubicalTerm::EndpointPathElim {
            motive: motive.clone(),
            method: Box::new(lower_term_after_removal(method, removed)),
            scrutinee: Box::new(lower_term_after_removal(scrutinee, removed)),
        },
        CubicalTerm::EndpointElimNeutral {
            motive,
            method,
            scrutinee,
        } => CubicalTerm::EndpointElimNeutral {
            motive: motive.clone(),
            // `method` is an opaque premise identity, not a term in the
            // ambient dimension context.  Binder removal must not rewrite
            // it into a different (and unregistered) premise.
            method: method.clone(),
            scrutinee: Box::new(lower_term_after_removal(scrutinee, removed)),
        },
        CubicalTerm::DimLambda { binder, body } => CubicalTerm::DimLambda {
            binder: if *binder > removed {
                *binder - 1
            } else {
                *binder
            },
            body: Box::new(lower_term_after_removal(body, removed)),
        },
        CubicalTerm::DimApp { function, argument } => CubicalTerm::DimApp {
            function: Box::new(lower_term_after_removal(function, removed)),
            argument: lower_dimension_after_removal(*argument, removed),
        },
        CubicalTerm::Coe {
            family,
            binder,
            from,
            to,
            term,
        } => CubicalTerm::Coe {
            family: lower_type_after_removal(family, removed),
            binder: if *binder > removed {
                *binder - 1
            } else {
                *binder
            },
            from: lower_dimension_after_removal(*from, removed),
            to: lower_dimension_after_removal(*to, removed),
            term: Box::new(lower_term_after_removal(term, removed)),
        },
        CubicalTerm::Hcom {
            family,
            binder,
            from,
            to,
            cap,
            tubes,
        } => CubicalTerm::Hcom {
            family: lower_type_after_removal(family, removed),
            binder: if *binder > removed {
                *binder - 1
            } else {
                *binder
            },
            from: lower_dimension_after_removal(*from, removed),
            to: lower_dimension_after_removal(*to, removed),
            cap: Box::new(lower_term_after_removal(cap, removed)),
            tubes: tubes
                .iter()
                .map(|tube| Tube {
                    face: lower_face_after_removal(&tube.face, removed),
                    body: Box::new(lower_term_after_removal(&tube.body, removed)),
                })
                .collect(),
        },
    }
}

/// Instantiate a fresh dimension binder and remove it from the ambient
/// context.  Variables and nested fresh binders above the removed level are
/// lowered together, so beta reduction cannot leave a stale composition
/// binder behind.
fn instantiate_term(term: &CubicalTerm, binder: u16, argument: Dim) -> CubicalTerm {
    lower_term_after_removal(&substitute_term(term, binder, argument), binder)
}

fn instantiate_type(ty: &CubicalType, binder: u16, argument: Dim) -> CubicalType {
    lower_type_after_removal(&ty.substitute(binder, argument), binder)
}

fn reduce_endpoint_path_elim(
    motive: &Motive,
    method: CubicalTerm,
    scrutinee: CubicalTerm,
) -> CubicalTerm {
    if let (CubicalTerm::EndpointMethodHypothesis { reference }, CubicalTerm::Point { point }) =
        (&method, &scrutinee)
        && let PointExpr::EndpointPathConstructor {
            zero: zero_point,
            one: one_point,
            ..
        } = &reference.path
    {
        if point == zero_point.as_ref() {
            return CubicalTerm::EndpointEvaluationHypothesis {
                reference: reference.zero.clone(),
            };
        }
        if point == one_point.as_ref() {
            return CubicalTerm::EndpointEvaluationHypothesis {
                reference: reference.one.clone(),
            };
        }
        if point == &reference.path {
            return method;
        }
        return CubicalTerm::EndpointElimNeutral {
            motive: motive.clone(),
            method: reference.clone(),
            scrutinee: Box::new(scrutinee),
        };
    }
    CubicalTerm::EndpointPathElim {
        motive: motive.clone(),
        method: Box::new(method),
        scrutinee: Box::new(scrutinee),
    }
}

fn substitute_endpoint_evaluation_reference(
    reference: &EndpointEvaluationHypothesisRef,
    variable: u16,
    replacement: Dim,
) -> EndpointEvaluationHypothesisRef {
    EndpointEvaluationHypothesisRef {
        premise_context_derivation_hash: reference.premise_context_derivation_hash.clone(),
        premise_derivation_hash: reference.premise_derivation_hash.clone(),
        motive: reference.motive.clone(),
        point: reference.point.substitute(variable, replacement),
    }
}

fn substitute_endpoint_method_reference(
    reference: &EndpointMethodHypothesisRef,
    variable: u16,
    replacement: Dim,
) -> CubicalTerm {
    let zero = substitute_endpoint_evaluation_reference(&reference.zero, variable, replacement);
    let one = substitute_endpoint_evaluation_reference(&reference.one, variable, replacement);
    let PointExpr::EndpointPathConstructor { coordinate, .. } = &reference.path else {
        return CubicalTerm::EndpointMethodHypothesis {
            reference: EndpointMethodHypothesisRef {
                premise_context_derivation_hash: reference.premise_context_derivation_hash.clone(),
                premise_derivation_hash: reference.premise_derivation_hash.clone(),
                motive: reference.motive.clone(),
                path: reference.path.substitute(variable, replacement),
                zero,
                one,
            },
        };
    };
    match substitute_dimension(*coordinate, variable, replacement) {
        Dim::Zero => CubicalTerm::EndpointEvaluationHypothesis { reference: zero },
        Dim::One => CubicalTerm::EndpointEvaluationHypothesis { reference: one },
        Dim::Var(_) => CubicalTerm::EndpointMethodHypothesis {
            reference: EndpointMethodHypothesisRef {
                premise_context_derivation_hash: reference.premise_context_derivation_hash.clone(),
                premise_derivation_hash: reference.premise_derivation_hash.clone(),
                motive: reference.motive.clone(),
                path: reference.path.substitute(variable, replacement),
                zero,
                one,
            },
        },
    }
}

/// Capture-free substitution of a dimension expression through cubical
/// syntax.  `PathMethod` and `PathConstructor` compute definitionally on a
/// constant boundary: restricting any constructor coordinate to an endpoint
/// returns the checked base term/basepoint.
pub fn substitute_term(term: &CubicalTerm, variable: u16, replacement: Dim) -> CubicalTerm {
    match term {
        CubicalTerm::Point { point } => CubicalTerm::Point {
            point: point.substitute(variable, replacement),
        },
        CubicalTerm::MotiveBase { .. } => term.clone(),
        CubicalTerm::EndpointEvaluationHypothesis { reference } => {
            CubicalTerm::EndpointEvaluationHypothesis {
                reference: substitute_endpoint_evaluation_reference(
                    reference,
                    variable,
                    replacement,
                ),
            }
        }
        CubicalTerm::PathMethod { motive, path, base } => {
            let base = substitute_term(base, variable, replacement);
            match path.substitute(variable, replacement) {
                PointExpr::Base { .. } => base,
                path => CubicalTerm::PathMethod {
                    motive: motive.clone(),
                    path,
                    base: Box::new(base),
                },
            }
        }
        CubicalTerm::EndpointMethodHypothesis { reference } => {
            substitute_endpoint_method_reference(reference, variable, replacement)
        }
        CubicalTerm::PathElim {
            motive,
            base,
            method,
            scrutinee,
        } => {
            let base = substitute_term(base, variable, replacement);
            let method = substitute_term(method, variable, replacement);
            let scrutinee = substitute_term(scrutinee, variable, replacement);
            match scrutinee {
                CubicalTerm::Point {
                    point: PointExpr::Base { .. },
                } => base,
                CubicalTerm::Point {
                    point: PointExpr::PathConstructor { .. },
                } => method,
                scrutinee => CubicalTerm::PathElim {
                    motive: motive.clone(),
                    base: Box::new(base),
                    method: Box::new(method),
                    scrutinee: Box::new(scrutinee),
                },
            }
        }
        CubicalTerm::EndpointPathElim {
            motive,
            method,
            scrutinee,
        } => {
            let reduced = reduce_endpoint_path_elim(
                motive,
                method.as_ref().clone(),
                scrutinee.as_ref().clone(),
            );
            if !matches!(reduced, CubicalTerm::EndpointPathElim { .. }) {
                substitute_term(&reduced, variable, replacement)
            } else {
                reduce_endpoint_path_elim(
                    motive,
                    substitute_term(method, variable, replacement),
                    substitute_term(scrutinee, variable, replacement),
                )
            }
        }
        CubicalTerm::EndpointElimNeutral {
            motive,
            method,
            scrutinee,
        } => CubicalTerm::EndpointElimNeutral {
            motive: motive.clone(),
            // Preserve the source-bound `q` identity.  Only the ambient
            // scrutinee participates in dimension substitution.
            method: method.clone(),
            scrutinee: Box::new(substitute_term(scrutinee, variable, replacement)),
        },
        CubicalTerm::DimLambda { binder, .. } if *binder == variable => term.clone(),
        CubicalTerm::DimLambda { binder, body } => CubicalTerm::DimLambda {
            binder: *binder,
            body: Box::new(substitute_term(body, variable, replacement)),
        },
        CubicalTerm::DimApp { function, argument } => CubicalTerm::DimApp {
            function: Box::new(substitute_term(function, variable, replacement)),
            argument: substitute_dimension(*argument, variable, replacement),
        },
        CubicalTerm::Coe {
            family,
            binder,
            from,
            to,
            term,
        } => CubicalTerm::Coe {
            family: if *binder == variable {
                family.clone()
            } else {
                family.substitute(variable, replacement)
            },
            binder: *binder,
            from: substitute_dimension(*from, variable, replacement),
            to: substitute_dimension(*to, variable, replacement),
            term: Box::new(substitute_term(term, variable, replacement)),
        },
        CubicalTerm::Hcom {
            family,
            binder,
            from,
            to,
            cap,
            tubes,
        } => CubicalTerm::Hcom {
            family: if *binder == variable {
                family.clone()
            } else {
                family.substitute(variable, replacement)
            },
            binder: *binder,
            from: substitute_dimension(*from, variable, replacement),
            to: substitute_dimension(*to, variable, replacement),
            cap: Box::new(substitute_term(cap, variable, replacement)),
            tubes: tubes
                .iter()
                .map(|tube| Tube {
                    face: tube.face.substitute(variable, replacement),
                    body: if *binder == variable {
                        tube.body.clone()
                    } else {
                        Box::new(substitute_term(&tube.body, variable, replacement))
                    },
                })
                .collect(),
        },
    }
}

/// Check a substitution of an ambient interval variable and its typing
/// preservation.  Composition and lambda binders are always the fresh next
/// index, while `replacement` must already be scoped in `context`; therefore
/// a well-scoped ambient substitution cannot capture one of those binders.
pub fn substitute_typed_term(
    context: &CubicalContext,
    term: &CubicalTerm,
    variable: u16,
    replacement: Dim,
) -> Result<(CubicalType, CubicalTerm), CubicalTypeError> {
    if variable >= context.interval_count {
        return Err(CubicalTypeError::IntervalOutOfScope {
            variable,
            interval_count: context.interval_count,
        });
    }
    check_dim_scope(context, replacement)?;
    let source_type = infer_term(context, term)?;
    let expected_type = source_type.substitute(variable, replacement);
    let substituted = substitute_term(term, variable, replacement);
    expect_type(infer_term(context, &substituted)?, &expected_type)?;
    Ok((expected_type, substituted))
}

fn substitute_endpoint_typed_term(
    context: &CubicalContext,
    premises: &EndpointSchemaPremiseContext,
    term: &CubicalTerm,
    variable: u16,
    replacement: Dim,
) -> Result<(CubicalType, CubicalTerm), CubicalTypeError> {
    if variable >= context.interval_count {
        return Err(CubicalTypeError::IntervalOutOfScope {
            variable,
            interval_count: context.interval_count,
        });
    }
    premises.validate()?;
    check_dim_scope(context, replacement)?;
    let source_type = infer_term_with_endpoint_premises(context, premises, term)?;
    let expected_type = source_type.substitute(variable, replacement);
    let substituted = substitute_term(term, variable, replacement);
    expect_type(
        infer_term_with_endpoint_premises(context, premises, &substituted)?,
        &expected_type,
    )?;
    Ok((expected_type, substituted))
}

fn substitute_term_by_face(term: &CubicalTerm, face: &[Endpoint]) -> CubicalTerm {
    face.iter().fold(term.clone(), |term, endpoint| {
        substitute_term(
            &term,
            endpoint.variable,
            if endpoint.value { Dim::One } else { Dim::Zero },
        )
    })
}

fn substitute_type_by_face(ty: &CubicalType, face: &[Endpoint]) -> CubicalType {
    face.iter().fold(ty.clone(), |ty, endpoint| {
        ty.substitute(
            endpoint.variable,
            if endpoint.value { Dim::One } else { Dim::Zero },
        )
    })
}

fn terms_equal_under_face(
    context: &CubicalContext,
    left: &CubicalTerm,
    right: &CubicalTerm,
    face: &Cofibration,
) -> bool {
    let active = context.assumption.meet(face);
    active.clauses.iter().all(|branch| {
        let left = substitute_term_by_face(left, branch);
        let right = substitute_term_by_face(right, branch);
        let branch_context = CubicalContext::total(context.interval_count);
        normalize_term(&branch_context, &left) == normalize_term(&branch_context, &right)
    })
}

fn types_equal_under_face(
    context: &CubicalContext,
    left: &CubicalType,
    right: &CubicalType,
    face: &Cofibration,
) -> bool {
    context.assumption.meet(face).clauses.iter().all(|branch| {
        substitute_type_by_face(left, branch) == substitute_type_by_face(right, branch)
    })
}

impl EndpointSchemaPremiseContext {
    pub(crate) fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub(crate) fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    fn zero_term(&self) -> CubicalTerm {
        CubicalTerm::EndpointEvaluationHypothesis {
            reference: self.zero_evaluation.clone(),
        }
    }

    fn one_term(&self) -> CubicalTerm {
        CubicalTerm::EndpointEvaluationHypothesis {
            reference: self.one_evaluation.clone(),
        }
    }

    fn method_reference(&self) -> EndpointMethodHypothesisRef {
        EndpointMethodHypothesisRef {
            premise_context_derivation_hash: self.derivation_hash.clone(),
            premise_derivation_hash: self.method_premise_derivation_hash.clone(),
            motive: self.motive.clone(),
            path: self.method_path.clone(),
            zero: self.zero_evaluation.clone(),
            one: self.one_evaluation.clone(),
        }
    }

    fn method_term(&self) -> CubicalTerm {
        CubicalTerm::EndpointMethodHypothesis {
            reference: self.method_reference(),
        }
    }

    fn validate(&self) -> Result<(), CubicalTypeError> {
        if self.zero_parameter == self.one_parameter && self.zero_evaluation != self.one_evaluation
        {
            return Err(CubicalTypeError::EndpointPremiseContractionMismatch);
        }
        let replay = form_endpoint_schema_premise_context(
            self.source_digest.clone(),
            self.motive.clone(),
            self.zero_parameter,
            self.one_parameter,
        )?;
        if &replay != self {
            return Err(CubicalTypeError::EndpointPremiseContextMismatch);
        }
        Ok(())
    }

    fn accepts_evaluation(
        &self,
        reference: &EndpointEvaluationHypothesisRef,
    ) -> Result<(), CubicalTypeError> {
        self.validate()?;
        if reference == &self.zero_evaluation || reference == &self.one_evaluation {
            Ok(())
        } else {
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        }
    }

    fn accepts_method(
        &self,
        reference: &EndpointMethodHypothesisRef,
    ) -> Result<(), CubicalTypeError> {
        self.validate()?;
        if reference == &self.method_reference() {
            Ok(())
        } else {
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        }
    }

    /// Validate the endpoint-only point syntax against this exact ledger.
    /// Boundary parameters must be one of its declared evaluations, while a
    /// path may vary only in its scoped interval coordinate: its owner and
    /// ordered endpoints remain the source-bound `squash(x,y)` family.
    fn accepts_endpoint_point(&self, point: &PointExpr) -> Result<(), CubicalTypeError> {
        self.validate()?;
        match point {
            PointExpr::BoundaryParameter { .. }
                if point == &self.zero_evaluation.point || point == &self.one_evaluation.point =>
            {
                Ok(())
            }
            PointExpr::EndpointPathConstructor {
                owner, zero, one, ..
            } => {
                let PointExpr::EndpointPathConstructor {
                    owner: expected_owner,
                    zero: expected_zero,
                    one: expected_one,
                    ..
                } = &self.method_path
                else {
                    return Err(CubicalTypeError::EndpointPremiseContextMismatch);
                };
                if owner == expected_owner && zero == expected_zero && one == expected_one {
                    Ok(())
                } else {
                    Err(CubicalTypeError::EndpointPremiseContextMismatch)
                }
            }
            _ => Err(CubicalTypeError::EndpointPremiseContextMismatch),
        }
    }
}

pub fn infer_term(
    context: &CubicalContext,
    term: &CubicalTerm,
) -> Result<CubicalType, CubicalTypeError> {
    infer_term_with_optional_endpoint_premises(context, None, term)
}

fn infer_term_with_endpoint_premises(
    context: &CubicalContext,
    premises: &EndpointSchemaPremiseContext,
    term: &CubicalTerm,
) -> Result<CubicalType, CubicalTypeError> {
    infer_term_with_optional_endpoint_premises(context, Some(premises), term)
}

fn infer_term_with_optional_endpoint_premises(
    context: &CubicalContext,
    endpoint_premises: Option<&EndpointSchemaPremiseContext>,
    term: &CubicalTerm,
) -> Result<CubicalType, CubicalTypeError> {
    match term {
        CubicalTerm::Point { point } => {
            check_point_scope_with_optional_endpoint_premises(context, endpoint_premises, point)?;
            Ok(CubicalType::Owner {
                owner: point.owner().clone(),
            })
        }
        CubicalTerm::MotiveBase { motive } => Ok(CubicalType::MotiveFiber {
            motive: motive.clone(),
            point: PointExpr::Base {
                owner: motive.owner.clone(),
            },
        }),
        CubicalTerm::EndpointEvaluationHypothesis { reference } => {
            let premises =
                endpoint_premises.ok_or(CubicalTypeError::EndpointPremiseContextRequired)?;
            premises.accepts_evaluation(reference)?;
            check_point_scope_with_optional_endpoint_premises(
                context,
                endpoint_premises,
                &reference.point,
            )?;
            if reference.point.owner() != &reference.motive.owner {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            Ok(CubicalType::MotiveFiber {
                motive: reference.motive.clone(),
                point: reference.point.clone(),
            })
        }
        CubicalTerm::PathMethod { motive, path, base } => {
            let PointExpr::PathConstructor { owner, .. } = path else {
                return Err(CubicalTypeError::MethodNotAtConstructor);
            };
            if owner != &motive.owner {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            check_point_scope_with_optional_endpoint_premises(context, endpoint_premises, path)?;
            let expected_base = CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: PointExpr::Base {
                    owner: motive.owner.clone(),
                },
            };
            expect_type(
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, base)?,
                &expected_base,
            )?;
            Ok(CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: path.clone(),
            })
        }
        CubicalTerm::EndpointMethodHypothesis { reference } => {
            let premises =
                endpoint_premises.ok_or(CubicalTypeError::EndpointPremiseContextRequired)?;
            premises.accepts_method(reference)?;
            let PointExpr::EndpointPathConstructor {
                owner,
                zero: zero_point,
                one: one_point,
                ..
            } = &reference.path
            else {
                return Err(CubicalTypeError::EndpointMethodNotAtConstructor);
            };
            if owner != &reference.motive.owner {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            check_point_scope_with_optional_endpoint_premises(
                context,
                endpoint_premises,
                &reference.path,
            )?;
            let expected_zero = CubicalType::MotiveFiber {
                motive: reference.motive.clone(),
                point: zero_point.as_ref().clone(),
            };
            let expected_one = CubicalType::MotiveFiber {
                motive: reference.motive.clone(),
                point: one_point.as_ref().clone(),
            };
            expect_type(
                infer_term_with_optional_endpoint_premises(
                    context,
                    endpoint_premises,
                    &CubicalTerm::EndpointEvaluationHypothesis {
                        reference: reference.zero.clone(),
                    },
                )?,
                &expected_zero,
            )?;
            expect_type(
                infer_term_with_optional_endpoint_premises(
                    context,
                    endpoint_premises,
                    &CubicalTerm::EndpointEvaluationHypothesis {
                        reference: reference.one.clone(),
                    },
                )?,
                &expected_one,
            )?;
            Ok(CubicalType::MotiveFiber {
                motive: reference.motive.clone(),
                point: reference.path.clone(),
            })
        }
        CubicalTerm::PathElim {
            motive,
            base,
            method,
            scrutinee,
        } => {
            let scrutinee_ty =
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, scrutinee)?;
            expect_type(
                scrutinee_ty,
                &CubicalType::Owner {
                    owner: motive.owner.clone(),
                },
            )?;
            let CubicalTerm::Point {
                point: scrutinee_point,
            } = scrutinee.as_ref()
            else {
                return Err(CubicalTypeError::MethodNotAtConstructor);
            };
            let expected_base = CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: PointExpr::Base {
                    owner: motive.owner.clone(),
                },
            };
            expect_type(
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, base)?,
                &expected_base,
            )?;
            let CubicalTerm::PathMethod {
                motive: method_motive,
                path: method_path,
                base: method_base,
            } = method.as_ref()
            else {
                return Err(CubicalTypeError::MethodNotAtConstructor);
            };
            if method_motive != motive {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            let PointExpr::PathConstructor { .. } = method_path else {
                return Err(CubicalTypeError::MethodNotAtConstructor);
            };
            let expected_method = CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: method_path.clone(),
            };
            expect_type(
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, method)?,
                &expected_method,
            )?;
            if normalize_term(context, method_base) != normalize_term(context, base) {
                return Err(CubicalTypeError::MethodBoundaryMismatch);
            }
            if matches!(scrutinee_point, PointExpr::PathConstructor { .. })
                && scrutinee_point != method_path
            {
                return Err(CubicalTypeError::TypeMismatch {
                    expected: CubicalType::MotiveFiber {
                        motive: motive.clone(),
                        point: scrutinee_point.clone(),
                    },
                    found: expected_method,
                });
            }
            Ok(CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: scrutinee_point.clone(),
            })
        }
        CubicalTerm::EndpointElimNeutral {
            motive,
            method,
            scrutinee,
        } => {
            let premises =
                endpoint_premises.ok_or(CubicalTypeError::EndpointPremiseContextRequired)?;
            premises.accepts_method(method)?;
            if &method.motive != motive {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            let scrutinee_ty =
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, scrutinee)?;
            expect_type(
                scrutinee_ty,
                &CubicalType::Owner {
                    owner: motive.owner.clone(),
                },
            )?;
            let CubicalTerm::Point { point } = scrutinee.as_ref() else {
                return Err(CubicalTypeError::EndpointMethodNotAtConstructor);
            };
            check_point_scope_with_optional_endpoint_premises(context, endpoint_premises, point)?;
            if point.owner() != &motive.owner {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            if point == &method.zero.point || point == &method.one.point || point == &method.path {
                return Err(CubicalTypeError::EndpointNeutralScrutineeReducible);
            }
            Ok(CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: point.clone(),
            })
        }
        CubicalTerm::EndpointPathElim {
            motive,
            method,
            scrutinee,
        } => {
            let scrutinee_ty =
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, scrutinee)?;
            expect_type(
                scrutinee_ty,
                &CubicalType::Owner {
                    owner: motive.owner.clone(),
                },
            )?;
            let CubicalTerm::Point {
                point: scrutinee_point,
            } = scrutinee.as_ref()
            else {
                return Err(CubicalTypeError::EndpointMethodNotAtConstructor);
            };
            let CubicalTerm::EndpointMethodHypothesis { reference } = method.as_ref() else {
                return Err(CubicalTypeError::EndpointMethodNotAtConstructor);
            };
            if &reference.motive != motive {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            let PointExpr::EndpointPathConstructor { .. } = &reference.path else {
                return Err(CubicalTypeError::EndpointMethodNotAtConstructor);
            };
            let expected_method = CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: reference.path.clone(),
            };
            expect_type(
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, method)?,
                &expected_method,
            )?;
            if matches!(scrutinee_point, PointExpr::EndpointPathConstructor { .. })
                && scrutinee_point != &reference.path
            {
                return Err(CubicalTypeError::TypeMismatch {
                    expected: CubicalType::MotiveFiber {
                        motive: motive.clone(),
                        point: scrutinee_point.clone(),
                    },
                    found: expected_method,
                });
            }
            Ok(CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: scrutinee_point.clone(),
            })
        }
        CubicalTerm::DimLambda { binder, body } => {
            if *binder != context.interval_count {
                return Err(CubicalTypeError::IntervalOutOfScope {
                    variable: *binder,
                    interval_count: context.interval_count,
                });
            }
            let body_ty = infer_term_with_optional_endpoint_premises(
                &context.extend_interval(),
                endpoint_premises,
                body,
            )?;
            Ok(CubicalType::DimensionFunction {
                binder: *binder,
                body: Box::new(body_ty),
            })
        }
        CubicalTerm::DimApp { function, argument } => {
            check_dim_scope(context, *argument)?;
            let CubicalType::DimensionFunction { binder, body } =
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, function)?
            else {
                return Err(CubicalTypeError::ExpectedDimensionFunction);
            };
            Ok(instantiate_type(&body, binder, *argument))
        }
        CubicalTerm::Coe {
            family,
            binder,
            from,
            to,
            term,
        } => {
            if *binder != context.interval_count {
                return Err(CubicalTypeError::CompositionBinderNotFresh {
                    expected: context.interval_count,
                    found: *binder,
                });
            }
            check_type_scope_with_optional_endpoint_premises(
                &context.extend_interval(),
                endpoint_premises,
                family,
            )?;
            check_dim_scope(context, *from)?;
            check_dim_scope(context, *to)?;
            let source = instantiate_type(family, *binder, *from);
            let target = instantiate_type(family, *binder, *to);
            expect_type(
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, term)?,
                &source,
            )?;
            Ok(target)
        }
        CubicalTerm::Hcom {
            family,
            binder,
            from,
            to,
            cap,
            tubes,
        } => {
            if *binder != context.interval_count {
                return Err(CubicalTypeError::CompositionBinderNotFresh {
                    expected: context.interval_count,
                    found: *binder,
                });
            }
            let extended = context.extend_interval();
            check_type_scope_with_optional_endpoint_premises(&extended, endpoint_premises, family)?;
            if family.depends_on(*binder) {
                return Err(CubicalTypeError::DependentHcomUnsupported);
            }
            check_dim_scope(context, *from)?;
            check_dim_scope(context, *to)?;
            let source = instantiate_type(family, *binder, *from);
            let target = instantiate_type(family, *binder, *to);
            expect_type(
                infer_term_with_optional_endpoint_premises(context, endpoint_premises, cap)?,
                &source,
            )?;
            for tube in tubes {
                check_face_scope(context.interval_count, &tube.face)?;
                let body_ty = infer_term_with_optional_endpoint_premises(
                    &extended,
                    endpoint_premises,
                    &tube.body,
                )?;
                if !types_equal_under_face(&extended, &body_ty, family, &tube.face) {
                    return Err(CubicalTypeError::TypeMismatch {
                        expected: family.clone(),
                        found: body_ty,
                    });
                }
                let body_at_start = instantiate_term(&tube.body, *binder, *from);
                if !terms_equal_under_face(context, &body_at_start, cap, &tube.face) {
                    return Err(CubicalTypeError::CapFaceMismatch);
                }
            }
            for left in 0..tubes.len() {
                for right in left + 1..tubes.len() {
                    let overlap = tubes[left].face.meet(&tubes[right].face);
                    if !terms_equal_under_face(
                        &extended,
                        &tubes[left].body,
                        &tubes[right].body,
                        &overlap,
                    ) {
                        return Err(CubicalTypeError::TubeOverlapMismatch);
                    }
                }
            }
            Ok(target)
        }
    }
}

fn canonicalize_tubes(mut tubes: Vec<Tube>) -> Vec<Tube> {
    tubes.sort_by(|left, right| {
        left.face.cmp(&right.face).then_with(|| {
            serde_json::to_vec(left)
                .expect("cubical tube serializes")
                .cmp(&serde_json::to_vec(right).expect("cubical tube serializes"))
        })
    });
    tubes.dedup();
    tubes
}

/// Normalize only after typing when the result is used as evidence.  This
/// function itself is public to make the oriented computation rules easy to
/// inspect; [`normalize_typed_term`] is the evidence-producing entry point.
pub fn normalize_term(context: &CubicalContext, term: &CubicalTerm) -> CubicalTerm {
    match term {
        CubicalTerm::Point { .. }
        | CubicalTerm::MotiveBase { .. }
        | CubicalTerm::EndpointEvaluationHypothesis { .. }
        | CubicalTerm::EndpointMethodHypothesis { .. } => term.clone(),
        CubicalTerm::EndpointElimNeutral {
            motive,
            method,
            scrutinee,
        } => CubicalTerm::EndpointElimNeutral {
            motive: motive.clone(),
            method: method.clone(),
            scrutinee: Box::new(normalize_term(context, scrutinee)),
        },
        CubicalTerm::PathMethod { motive, path, base } => CubicalTerm::PathMethod {
            motive: motive.clone(),
            path: path.clone(),
            base: Box::new(normalize_term(context, base)),
        },
        CubicalTerm::PathElim {
            motive,
            base,
            method,
            scrutinee,
        } => {
            let base = normalize_term(context, base);
            let method = normalize_term(context, method);
            let scrutinee = normalize_term(context, scrutinee);
            match scrutinee {
                CubicalTerm::Point {
                    point: PointExpr::Base { .. },
                } => base,
                CubicalTerm::Point {
                    point: PointExpr::PathConstructor { .. },
                } => {
                    // Constructor beta:
                    // elim(P,b,q,p(i_1,...,i_d)) --> q(i_1,...,i_d).
                    method
                }
                scrutinee => CubicalTerm::PathElim {
                    motive: motive.clone(),
                    base: Box::new(base),
                    method: Box::new(method),
                    scrutinee: Box::new(scrutinee),
                },
            }
        }
        CubicalTerm::EndpointPathElim {
            motive,
            method,
            scrutinee,
        } => reduce_endpoint_path_elim(
            motive,
            normalize_term(context, method),
            normalize_term(context, scrutinee),
        ),
        CubicalTerm::DimLambda { binder, body } => CubicalTerm::DimLambda {
            binder: *binder,
            body: Box::new(normalize_term(&context.extend_interval(), body)),
        },
        CubicalTerm::DimApp { function, argument } => {
            let function = normalize_term(context, function);
            if let CubicalTerm::DimLambda { binder, body } = function {
                normalize_term(context, &instantiate_term(&body, binder, *argument))
            } else {
                CubicalTerm::DimApp {
                    function: Box::new(function),
                    argument: *argument,
                }
            }
        }
        CubicalTerm::Coe {
            family,
            binder,
            from,
            to,
            term,
        } => {
            let term = normalize_term(context, term);
            let constant_family = !family.depends_on(*binder);
            if from == to || constant_family {
                term
            } else {
                CubicalTerm::Coe {
                    family: family.clone(),
                    binder: *binder,
                    from: *from,
                    to: *to,
                    term: Box::new(term),
                }
            }
        }
        CubicalTerm::Hcom {
            family,
            binder,
            from,
            to,
            cap,
            tubes,
        } => {
            if let CubicalTerm::Hcom {
                family: inner_family,
                binder: inner_binder,
                from: inner_from,
                to: inner_to,
                cap: inner_cap,
                tubes: inner_tubes,
            } = cap.as_ref()
            {
                if inner_family == family
                    && inner_binder == binder
                    && inner_from == from
                    && inner_to == to
                {
                    let mut union_tubes = inner_tubes.clone();
                    union_tubes.extend(tubes.iter().cloned());
                    return normalize_term(
                        context,
                        &CubicalTerm::Hcom {
                            family: family.clone(),
                            binder: *binder,
                            from: *from,
                            to: *to,
                            cap: inner_cap.clone(),
                            tubes: canonicalize_tubes(union_tubes),
                        },
                    );
                }
            }
            let cap = normalize_term(context, cap);
            if from == to {
                return cap;
            }
            let tubes = canonicalize_tubes(
                tubes
                    .iter()
                    .map(|tube| Tube {
                        face: tube.face.clone(),
                        body: Box::new(normalize_term(&context.extend_interval(), &tube.body)),
                    })
                    .collect(),
            );
            if let Some(active) = tubes
                .iter()
                .find(|tube| context.assumption.implies(&tube.face))
            {
                let body = instantiate_term(&active.body, *binder, *to);
                return normalize_term(context, &body);
            }
            CubicalTerm::Hcom {
                family: family.clone(),
                binder: *binder,
                from: *from,
                to: *to,
                cap: Box::new(cap),
                tubes,
            }
        }
    }
}

pub fn normalize_typed_term(
    context: &CubicalContext,
    term: &CubicalTerm,
) -> Result<(CubicalType, CubicalTerm), CubicalTypeError> {
    let ty = infer_term(context, term)?;
    let normal = normalize_term(context, term);
    expect_type(infer_term(context, &normal)?, &ty)?;
    Ok((ty, normal))
}

fn normalize_endpoint_typed_term(
    context: &CubicalContext,
    premises: &EndpointSchemaPremiseContext,
    term: &CubicalTerm,
) -> Result<(CubicalType, CubicalTerm), CubicalTypeError> {
    premises.validate()?;
    let ty = infer_term_with_endpoint_premises(context, premises, term)?;
    let normal = normalize_term(context, term);
    expect_type(
        infer_term_with_endpoint_premises(context, premises, &normal)?,
        &ty,
    )?;
    Ok((ty, normal))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StagedUnionReplay {
    pub direct_union: Cofibration,
    pub staged_union: Cofibration,
    pub stages_checked: usize,
    pub nested_term_digest: String,
    pub flattened_term_digest: String,
    pub flatten_law_checked: bool,
    pub normal_form_digest: String,
}

/// Check an `hcom` tube system directly and as genuinely nested singleton
/// compositions.  Every nested prefix is independently typed.  The
/// homogeneous union-flatten rule lives in [`normalize_term`], so the final
/// nested normal form is compared against the separately typed direct union.
pub fn replay_hcom_staged_union(
    context: &CubicalContext,
    family: CubicalType,
    binder: u16,
    from: Dim,
    to: Dim,
    cap: CubicalTerm,
    tubes: Vec<Tube>,
) -> Result<StagedUnionReplay, CubicalTypeError> {
    let tubes = canonicalize_tubes(tubes);
    let direct_term = CubicalTerm::Hcom {
        family: family.clone(),
        binder,
        from,
        to,
        cap: Box::new(cap.clone()),
        tubes: tubes.clone(),
    };
    let (_, direct_nf) = normalize_typed_term(context, &direct_term)?;
    let direct_union = tubes
        .iter()
        .fold(Cofibration::false_formula(), |union, tube| {
            union.join(&tube.face)
        });

    let mut staged_term = cap.clone();
    let mut staged_union = Cofibration::false_formula();
    let mut stages_checked = 0;
    for tube in &tubes {
        staged_union = staged_union.join(&tube.face);
        staged_term = CubicalTerm::Hcom {
            family: family.clone(),
            binder,
            from,
            to,
            cap: Box::new(staged_term),
            tubes: vec![tube.clone()],
        };
        infer_term(context, &staged_term)?;
        stages_checked += 1;
    }
    let nested_term_digest = tagged_digest("nested-singleton-hcom", &staged_term);
    let flattened_term_digest = tagged_digest("flattened-hcom-union", &direct_term);
    let (_, staged_nf) = normalize_typed_term(context, &staged_term)?;
    if direct_union != staged_union || direct_nf != staged_nf {
        return Err(CubicalTypeError::InvalidStagedUnion);
    }
    Ok(StagedUnionReplay {
        direct_union,
        staged_union,
        stages_checked,
        nested_term_digest,
        flattened_term_digest,
        flatten_law_checked: true,
        normal_form_digest: tagged_digest("staged-hcom", &direct_nf),
    })
}

fn tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(
        CUBICAL_FRAGMENT_VERSION,
        PATHCON_ATTACHMENT_AXIOM_VERSION,
        domain,
        payload,
    ))
    .expect("cubical proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn endpoint_tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(
        CUBICAL_FRAGMENT_VERSION,
        TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION,
        PATHCON_ATTACHMENT_AXIOM_VERSION,
        domain,
        payload,
    ))
    .expect("endpoint-dependent cubical proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_digest(typing: &FormedPathTyping) -> String {
    tagged_digest(
        "formed-path-source",
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
            PATHCON_ATTACHMENT_AXIOM_VERSION,
        ),
    )
}

fn standard_path(typing: &FormedPathTyping) -> PointExpr {
    PointExpr::PathConstructor {
        owner: typing.formation_normal_form.clone(),
        dimension: typing.dimension,
        coordinates: (0..typing.dimension)
            .map(|index| Dim::Var(index as u16))
            .collect(),
    }
}

fn path_with_probe_binder(typing: &FormedPathTyping, probe: u32, binder: u16) -> PointExpr {
    let mut coordinates = (0..typing.dimension)
        .map(|index| Dim::Var(index as u16))
        .collect::<Vec<_>>();
    coordinates[probe as usize] = Dim::Var(binder);
    PointExpr::PathConstructor {
        owner: typing.formation_normal_form.clone(),
        dimension: typing.dimension,
        coordinates,
    }
}

fn path_with_principal_and_probe_binders(
    typing: &FormedPathTyping,
    principal: u32,
    principal_binder: u16,
    probe: u32,
    probe_binder: u16,
) -> PointExpr {
    let mut coordinates = (0..typing.dimension)
        .map(|index| Dim::Var(index as u16))
        .collect::<Vec<_>>();
    coordinates[principal as usize] = Dim::Var(principal_binder);
    coordinates[probe as usize] = Dim::Var(probe_binder);
    PointExpr::PathConstructor {
        owner: typing.formation_normal_form.clone(),
        dimension: typing.dimension,
        coordinates,
    }
}

fn standard_motive(typing: &FormedPathTyping) -> Motive {
    Motive {
        owner: typing.formation_normal_form.clone(),
        name: format!(
            "P:{}",
            tagged_digest("motive", &(&typing.subject_hash, typing.dimension))
        ),
    }
}

fn endpoint_boundary_point(typing: &FormedPathTyping, parameter: u16) -> PointExpr {
    PointExpr::BoundaryParameter {
        owner: typing.formation_normal_form.clone(),
        parameter,
    }
}

fn endpoint_path(
    typing: &FormedPathTyping,
    coordinate: Dim,
    zero_parameter: u16,
    one_parameter: u16,
) -> PointExpr {
    PointExpr::EndpointPathConstructor {
        owner: typing.formation_normal_form.clone(),
        coordinate,
        zero: Box::new(endpoint_boundary_point(typing, zero_parameter)),
        one: Box::new(endpoint_boundary_point(typing, one_parameter)),
    }
}

fn form_endpoint_schema_premise_context(
    source_digest: String,
    motive: Motive,
    zero_parameter: u16,
    one_parameter: u16,
) -> Result<EndpointSchemaPremiseContext, CubicalTypeError> {
    if source_digest.is_empty() || zero_parameter == 0 || one_parameter == 0 {
        return Err(CubicalTypeError::EndpointPremiseContextMismatch);
    }
    let zero_point = PointExpr::BoundaryParameter {
        owner: motive.owner.clone(),
        parameter: zero_parameter,
    };
    let one_point = PointExpr::BoundaryParameter {
        owner: motive.owner.clone(),
        parameter: one_parameter,
    };
    let method_path = PointExpr::EndpointPathConstructor {
        owner: motive.owner.clone(),
        coordinate: Dim::Var(0),
        zero: Box::new(zero_point.clone()),
        one: Box::new(one_point.clone()),
    };
    let zero_premise_derivation_hash = endpoint_tagged_digest(
        "endpoint-evaluation-formal-premise",
        &(&source_digest, &motive, &zero_point),
    );
    let one_premise_derivation_hash = endpoint_tagged_digest(
        "endpoint-evaluation-formal-premise",
        &(&source_digest, &motive, &one_point),
    );
    let method_premise_derivation_hash = endpoint_tagged_digest(
        "endpoint-pathp-method-formal-premise",
        &(
            &source_digest,
            &motive,
            &method_path,
            &zero_premise_derivation_hash,
            &one_premise_derivation_hash,
        ),
    );
    let derivation_hash = endpoint_tagged_digest(
        "endpoint-formal-schema-premise-context",
        &(
            &source_digest,
            &motive,
            zero_parameter,
            one_parameter,
            &zero_point,
            &one_point,
            &method_path,
            &zero_premise_derivation_hash,
            &one_premise_derivation_hash,
            &method_premise_derivation_hash,
        ),
    );
    let zero_evaluation = EndpointEvaluationHypothesisRef {
        premise_context_derivation_hash: derivation_hash.clone(),
        premise_derivation_hash: zero_premise_derivation_hash,
        motive: motive.clone(),
        point: zero_point,
    };
    let one_evaluation = EndpointEvaluationHypothesisRef {
        premise_context_derivation_hash: derivation_hash.clone(),
        premise_derivation_hash: one_premise_derivation_hash,
        motive: motive.clone(),
        point: one_point,
    };
    if zero_parameter == one_parameter && zero_evaluation != one_evaluation {
        return Err(CubicalTypeError::EndpointPremiseContractionMismatch);
    }
    Ok(EndpointSchemaPremiseContext {
        source_digest,
        motive,
        zero_parameter,
        one_parameter,
        zero_evaluation,
        one_evaluation,
        method_path,
        method_premise_derivation_hash,
        derivation_hash,
    })
}

pub(crate) fn issue_endpoint_schema_premise_context(
    typing: &FormedPathTyping,
    premise_source_digest: &str,
    zero_parameter: u16,
    one_parameter: u16,
) -> Result<EndpointSchemaPremiseContext, CubicalTypeError> {
    if typing.dimension != 1 {
        return Err(CubicalTypeError::EndpointPathDimensionMismatch);
    }
    form_endpoint_schema_premise_context(
        premise_source_digest.to_owned(),
        standard_motive(typing),
        zero_parameter,
        one_parameter,
    )
}

fn replay_endpoint_schema_premise_context(
    typing: &FormedPathTyping,
    premise_source_digest: &str,
    zero_parameter: u16,
    one_parameter: u16,
    premises: &EndpointSchemaPremiseContext,
) -> Result<(), CubicalTypeError> {
    let replay = issue_endpoint_schema_premise_context(
        typing,
        premise_source_digest,
        zero_parameter,
        one_parameter,
    )?;
    if &replay == premises {
        Ok(())
    } else {
        Err(CubicalTypeError::EndpointPremiseContextMismatch)
    }
}

fn endpoint_method(premises: &EndpointSchemaPremiseContext) -> CubicalTerm {
    premises.method_term()
}

fn endpoint_beta_realizer_term(
    typing: &FormedPathTyping,
    premises: &EndpointSchemaPremiseContext,
    zero_parameter: u16,
    one_parameter: u16,
) -> CubicalTerm {
    let motive = standard_motive(typing);
    let path = endpoint_path(typing, Dim::Var(0), zero_parameter, one_parameter);
    CubicalTerm::EndpointPathElim {
        motive,
        method: Box::new(endpoint_method(premises)),
        scrutinee: Box::new(CubicalTerm::Point { point: path }),
    }
}

fn endpoint_transport_realizer_term(
    typing: &FormedPathTyping,
    premises: &EndpointSchemaPremiseContext,
    zero_parameter: u16,
    one_parameter: u16,
) -> CubicalTerm {
    let motive = standard_motive(typing);
    let binder = 1;
    CubicalTerm::Coe {
        family: CubicalType::MotiveFiber {
            motive: motive.clone(),
            point: endpoint_path(typing, Dim::Var(binder), zero_parameter, one_parameter),
        },
        binder,
        from: Dim::Zero,
        to: Dim::One,
        term: Box::new(premises.zero_term()),
    }
}

fn derive_endpoint_path_schema_key(
    typing: &FormedPathTyping,
    premises: &EndpointSchemaPremiseContext,
    zero_parameter: u16,
    one_parameter: u16,
    normal: &CubicalTerm,
) -> Result<PathSchemaKey, CubicalTypeError> {
    if normal == &endpoint_method(premises) {
        return Ok(PathSchemaKey::Beta);
    }
    if normal == &endpoint_transport_realizer_term(typing, premises, zero_parameter, one_parameter)
    {
        return Ok(PathSchemaKey::Kan {
            principal: 0,
            probe: 0,
        });
    }
    Err(CubicalTypeError::NotPathSchemaRealizer)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct EndpointPathRealizationToken {
    fragment_version: String,
    source_digest: String,
    subject_hash: String,
    signature_digest: String,
    boundary_binding_digest: String,
    parameter_instantiation_digest: String,
    zero_parameter: u16,
    one_parameter: u16,
    premise_source_digest: String,
    premise_context_derivation_hash: String,
    context: CubicalContext,
    term: CubicalTerm,
    normal_form: CubicalTerm,
    term_hash: String,
    normal_form_hash: String,
    type_hash: String,
    key: PathSchemaKey,
    derivation_hash: String,
}

impl EndpointPathRealizationToken {
    pub(crate) fn key(&self) -> &PathSchemaKey {
        &self.key
    }

    pub(crate) fn term_hash(&self) -> &str {
        &self.term_hash
    }

    pub(crate) fn normal_form_hash(&self) -> &str {
        &self.normal_form_hash
    }

    pub(crate) fn type_hash(&self) -> &str {
        &self.type_hash
    }

    pub(crate) fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub(crate) fn endpoint_source_digest(
    typing: &FormedPathTyping,
    boundary_binding_digest: &str,
    parameter_instantiation_digest: &str,
    zero_parameter: u16,
    one_parameter: u16,
) -> String {
    endpoint_tagged_digest(
        "formed-endpoint-path-source",
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
            boundary_binding_digest,
            parameter_instantiation_digest,
            zero_parameter,
            one_parameter,
        ),
    )
}

fn issue_endpoint_path_realization(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
    boundary_binding_digest: &str,
    parameter_instantiation_digest: &str,
    zero_parameter: u16,
    one_parameter: u16,
    premises: &EndpointSchemaPremiseContext,
    term: CubicalTerm,
) -> Result<EndpointPathRealizationToken, CubicalTypeError> {
    validate_formed_source(signature, telescope, visible_library, typing)?;
    if typing.dimension != 1 {
        return Err(CubicalTypeError::EndpointPathDimensionMismatch);
    }
    if zero_parameter == 0 || one_parameter == 0 || zero_parameter == one_parameter {
        return Err(CubicalTypeError::EndpointBoundaryParameterMismatch);
    }
    let source_digest = endpoint_source_digest(
        typing,
        boundary_binding_digest,
        parameter_instantiation_digest,
        zero_parameter,
        one_parameter,
    );
    replay_endpoint_schema_premise_context(
        typing,
        &source_digest,
        zero_parameter,
        one_parameter,
        premises,
    )?;
    let context = CubicalContext::total(1);
    let (inferred_type, normal_form) = normalize_endpoint_typed_term(&context, premises, &term)?;
    let key = derive_endpoint_path_schema_key(
        typing,
        premises,
        zero_parameter,
        one_parameter,
        &normal_form,
    )?;
    let premise_source_digest = premises.source_digest().to_owned();
    let premise_context_derivation_hash = premises.derivation_hash().to_owned();
    let term_hash = endpoint_tagged_digest("endpoint-realizer-term", &(&source_digest, &term));
    let normal_form_hash = endpoint_tagged_digest(
        "endpoint-realizer-normal-form",
        &(&source_digest, &normal_form),
    );
    let type_hash = endpoint_tagged_digest("endpoint-realizer-type", &inferred_type);
    let derivation_hash = endpoint_tagged_digest(
        "endpoint-realization-derivation",
        &(
            &source_digest,
            &typing.subject_hash,
            &typing.signature_digest,
            &boundary_binding_digest,
            &parameter_instantiation_digest,
            zero_parameter,
            one_parameter,
            &premise_source_digest,
            &premise_context_derivation_hash,
            &term_hash,
            &normal_form_hash,
            &type_hash,
            &key,
        ),
    );
    Ok(EndpointPathRealizationToken {
        fragment_version: TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION.to_owned(),
        source_digest,
        subject_hash: typing.subject_hash.clone(),
        signature_digest: typing.signature_digest.clone(),
        boundary_binding_digest: boundary_binding_digest.to_owned(),
        parameter_instantiation_digest: parameter_instantiation_digest.to_owned(),
        zero_parameter,
        one_parameter,
        premise_source_digest,
        premise_context_derivation_hash,
        context,
        term,
        normal_form,
        term_hash,
        normal_form_hash,
        type_hash,
        key,
        derivation_hash,
    })
}

pub(crate) fn replay_endpoint_path_realization(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
    boundary_binding_digest: &str,
    parameter_instantiation_digest: &str,
    zero_parameter: u16,
    one_parameter: u16,
    premises: &EndpointSchemaPremiseContext,
    token: &EndpointPathRealizationToken,
) -> Result<(), CubicalTypeError> {
    if token.boundary_binding_digest != boundary_binding_digest
        || token.parameter_instantiation_digest != parameter_instantiation_digest
        || token.zero_parameter != zero_parameter
        || token.one_parameter != one_parameter
        || token.premise_source_digest != premises.source_digest()
        || token.premise_context_derivation_hash != premises.derivation_hash()
    {
        return Err(CubicalTypeError::SourceMismatch);
    }
    let replay = issue_endpoint_path_realization(
        signature,
        telescope,
        visible_library,
        typing,
        boundary_binding_digest,
        parameter_instantiation_digest,
        zero_parameter,
        one_parameter,
        premises,
        token.term.clone(),
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(CubicalTypeError::SourceMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct EndpointPathBasisRealization {
    dimension: u32,
    zero_parameter: u16,
    one_parameter: u16,
    boundary_binding_digest: String,
    parameter_instantiation_digest: String,
    premise_source_digest: String,
    premise_context_derivation_hash: String,
    method_shape_digest: String,
    tokens: Vec<EndpointPathRealizationToken>,
    derivation_hash: String,
}

impl EndpointPathBasisRealization {
    pub(crate) fn tokens(&self) -> &[EndpointPathRealizationToken] {
        &self.tokens
    }

    pub(crate) fn method_shape_digest(&self) -> &str {
        &self.method_shape_digest
    }

    pub(crate) fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct EndpointPathComputationAudit {
    premise_source_digest: String,
    premise_context_derivation_hash: String,
    zero_method_restriction_hash: String,
    one_method_restriction_hash: String,
    zero_scrutinee_computation_hash: String,
    one_scrutinee_computation_hash: String,
    constructor_beta_computation_hash: String,
    neutral_type_hash: String,
    neutral_stuck_normal_form_hash: String,
    transport_target_type_hash: String,
    derivation_hash: String,
}

impl EndpointPathComputationAudit {
    pub(crate) fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub(crate) fn audit_endpoint_path_computation(
    typing: &FormedPathTyping,
    premises: &EndpointSchemaPremiseContext,
    zero_parameter: u16,
    one_parameter: u16,
) -> Result<EndpointPathComputationAudit, CubicalTypeError> {
    if typing.dimension != 1 {
        return Err(CubicalTypeError::EndpointPathDimensionMismatch);
    }
    replay_endpoint_schema_premise_context(
        typing,
        premises.source_digest(),
        zero_parameter,
        one_parameter,
        premises,
    )?;
    let context = CubicalContext::total(1);
    let motive = standard_motive(typing);
    let zero_point = endpoint_boundary_point(typing, zero_parameter);
    let one_point = endpoint_boundary_point(typing, one_parameter);
    let zero_method = premises.zero_term();
    let one_method = premises.one_term();
    let method = endpoint_method(premises);
    let method_reference = premises.method_reference();
    let zero_restriction =
        substitute_endpoint_typed_term(&context, premises, &method, 0, Dim::Zero)?.1;
    let one_restriction =
        substitute_endpoint_typed_term(&context, premises, &method, 0, Dim::One)?.1;
    if zero_restriction != zero_method || one_restriction != one_method {
        return Err(CubicalTypeError::EndpointMethodBoundaryMismatch);
    }

    let elim = |point| CubicalTerm::EndpointPathElim {
        motive: motive.clone(),
        method: Box::new(method.clone()),
        scrutinee: Box::new(CubicalTerm::Point { point }),
    };
    let zero_normal =
        normalize_endpoint_typed_term(&context, premises, &elim(zero_point.clone()))?.1;
    let one_normal = normalize_endpoint_typed_term(&context, premises, &elim(one_point.clone()))?.1;
    let beta_normal = normalize_endpoint_typed_term(
        &context,
        premises,
        &elim(endpoint_path(
            typing,
            Dim::Var(0),
            zero_parameter,
            one_parameter,
        )),
    )?
    .1;
    if zero_normal != zero_method || one_normal != one_method || beta_normal != method {
        return Err(CubicalTypeError::EndpointMethodBoundaryMismatch);
    }

    let neutral = PointExpr::Neutral {
        owner: typing.formation_normal_form.clone(),
        name: "registered-trunc-neutral-probe".to_owned(),
    };
    let neutral_scrutinee = CubicalTerm::Point {
        point: neutral.clone(),
    };
    let (neutral_type, neutral_normal) =
        normalize_endpoint_typed_term(&context, premises, &elim(neutral.clone()))?;
    if neutral_type
        != (CubicalType::MotiveFiber {
            motive: motive.clone(),
            point: neutral.clone(),
        })
        || !matches!(
            &neutral_normal,
            CubicalTerm::EndpointElimNeutral {
                method: retained_method,
                scrutinee: retained_scrutinee,
                ..
            } if retained_method == &method_reference
                && retained_scrutinee.as_ref() == &neutral_scrutinee
        )
    {
        return Err(CubicalTypeError::EndpointMethodBoundaryMismatch);
    }
    let (transport_target, transport_normal) = normalize_endpoint_typed_term(
        &context,
        premises,
        &endpoint_transport_realizer_term(typing, premises, zero_parameter, one_parameter),
    )?;
    if transport_target
        != (CubicalType::MotiveFiber {
            motive,
            point: one_point,
        })
        || !matches!(transport_normal, CubicalTerm::Coe { .. })
    {
        return Err(CubicalTypeError::EndpointMethodBoundaryMismatch);
    }

    let zero_method_restriction_hash =
        endpoint_tagged_digest("zero-method-restriction", &zero_restriction);
    let one_method_restriction_hash =
        endpoint_tagged_digest("one-method-restriction", &one_restriction);
    let zero_scrutinee_computation_hash =
        endpoint_tagged_digest("zero-scrutinee-computation", &zero_normal);
    let one_scrutinee_computation_hash =
        endpoint_tagged_digest("one-scrutinee-computation", &one_normal);
    let constructor_beta_computation_hash =
        endpoint_tagged_digest("constructor-beta-computation", &beta_normal);
    let neutral_type_hash = endpoint_tagged_digest("neutral-motive-type", &neutral_type);
    let neutral_stuck_normal_form_hash =
        endpoint_tagged_digest("neutral-stuck-normal-form", &neutral_normal);
    let transport_target_type_hash =
        endpoint_tagged_digest("endpoint-transport-target-type", &transport_target);
    let premise_source_digest = premises.source_digest().to_owned();
    let premise_context_derivation_hash = premises.derivation_hash().to_owned();
    let derivation_hash = endpoint_tagged_digest(
        "endpoint-computation-audit",
        &(
            &premise_source_digest,
            &premise_context_derivation_hash,
            &zero_method_restriction_hash,
            &one_method_restriction_hash,
            &zero_scrutinee_computation_hash,
            &one_scrutinee_computation_hash,
            &constructor_beta_computation_hash,
            &neutral_type_hash,
            &neutral_stuck_normal_form_hash,
            &transport_target_type_hash,
        ),
    );
    Ok(EndpointPathComputationAudit {
        premise_source_digest,
        premise_context_derivation_hash,
        zero_method_restriction_hash,
        one_method_restriction_hash,
        zero_scrutinee_computation_hash,
        one_scrutinee_computation_hash,
        constructor_beta_computation_hash,
        neutral_type_hash,
        neutral_stuck_normal_form_hash,
        transport_target_type_hash,
        derivation_hash,
    })
}

pub(crate) fn realize_endpoint_path_basis(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
    boundary_binding_digest: &str,
    parameter_instantiation_digest: &str,
    zero_parameter: u16,
    one_parameter: u16,
    premises: &EndpointSchemaPremiseContext,
) -> Result<EndpointPathBasisRealization, CubicalTypeError> {
    validate_formed_source(signature, telescope, visible_library, typing)?;
    let expected_premise_source = endpoint_source_digest(
        typing,
        boundary_binding_digest,
        parameter_instantiation_digest,
        zero_parameter,
        one_parameter,
    );
    replay_endpoint_schema_premise_context(
        typing,
        &expected_premise_source,
        zero_parameter,
        one_parameter,
        premises,
    )?;
    let terms = [
        endpoint_beta_realizer_term(typing, premises, zero_parameter, one_parameter),
        endpoint_transport_realizer_term(typing, premises, zero_parameter, one_parameter),
    ];
    let tokens = terms
        .into_iter()
        .map(|term| {
            issue_endpoint_path_realization(
                signature,
                telescope,
                visible_library,
                typing,
                boundary_binding_digest,
                parameter_instantiation_digest,
                zero_parameter,
                one_parameter,
                premises,
                term,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let keys = tokens
        .iter()
        .map(|token| token.key.clone())
        .collect::<BTreeSet<_>>();
    let expected_keys = BTreeSet::from([
        PathSchemaKey::Beta,
        PathSchemaKey::Kan {
            principal: 0,
            probe: 0,
        },
    ]);
    if tokens.len() != 2 || keys != expected_keys {
        return Err(CubicalTypeError::NotPathSchemaRealizer);
    }
    for token in &tokens {
        replay_endpoint_path_realization(
            signature,
            telescope,
            visible_library,
            typing,
            boundary_binding_digest,
            parameter_instantiation_digest,
            zero_parameter,
            one_parameter,
            premises,
            token,
        )?;
    }
    let method_shape_digest =
        endpoint_tagged_digest("endpoint-pathp-method-shape", &endpoint_method(premises));
    let premise_source_digest = premises.source_digest().to_owned();
    let premise_context_derivation_hash = premises.derivation_hash().to_owned();
    let derivation_hash = endpoint_tagged_digest(
        "endpoint-basis-realization",
        &(
            endpoint_source_digest(
                typing,
                boundary_binding_digest,
                parameter_instantiation_digest,
                zero_parameter,
                one_parameter,
            ),
            &premise_source_digest,
            &premise_context_derivation_hash,
            &method_shape_digest,
            tokens
                .iter()
                .map(EndpointPathRealizationToken::derivation_hash)
                .collect::<Vec<_>>(),
        ),
    );
    Ok(EndpointPathBasisRealization {
        dimension: 1,
        zero_parameter,
        one_parameter,
        boundary_binding_digest: boundary_binding_digest.to_owned(),
        parameter_instantiation_digest: parameter_instantiation_digest.to_owned(),
        premise_source_digest,
        premise_context_derivation_hash,
        method_shape_digest,
        tokens,
        derivation_hash,
    })
}

#[cfg(test)]
fn standard_cap(typing: &FormedPathTyping) -> CubicalTerm {
    CubicalTerm::Point {
        point: standard_path(typing),
    }
}

pub fn beta_realizer_term(typing: &FormedPathTyping) -> CubicalTerm {
    let motive = standard_motive(typing);
    let base = CubicalTerm::MotiveBase {
        motive: motive.clone(),
    };
    let path = standard_path(typing);
    let method = CubicalTerm::PathMethod {
        motive: motive.clone(),
        path: path.clone(),
        base: Box::new(base.clone()),
    };
    CubicalTerm::PathElim {
        motive,
        base: Box::new(base),
        method: Box::new(method),
        scrutinee: Box::new(CubicalTerm::Point { point: path }),
    }
}

pub fn diagonal_realizer_term(typing: &FormedPathTyping, principal: u32) -> CubicalTerm {
    let motive = standard_motive(typing);
    let binder = typing.dimension as u16;
    CubicalTerm::Coe {
        family: CubicalType::MotiveFiber {
            motive: motive.clone(),
            point: path_with_probe_binder(typing, principal, binder),
        },
        binder,
        from: Dim::Zero,
        to: Dim::One,
        term: Box::new(CubicalTerm::MotiveBase { motive }),
    }
}

pub fn off_diagonal_realizer_term(
    typing: &FormedPathTyping,
    principal: u32,
    probe: u32,
) -> CubicalTerm {
    let binder = typing.dimension as u16;
    let coe_binder = binder + 1;
    let motive = standard_motive(typing);
    CubicalTerm::DimLambda {
        binder,
        body: Box::new(CubicalTerm::Coe {
            family: CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: path_with_principal_and_probe_binders(
                    typing, principal, coe_binder, probe, binder,
                ),
            },
            binder: coe_binder,
            from: Dim::Zero,
            to: Dim::One,
            term: Box::new(CubicalTerm::MotiveBase { motive }),
        }),
    }
}

fn off_diagonal_indices(typing: &FormedPathTyping, normal: &CubicalTerm) -> Option<(u32, u32)> {
    let CubicalTerm::DimLambda { binder, body } = normal else {
        return None;
    };
    if *binder != typing.dimension as u16 {
        return None;
    }
    let CubicalTerm::Coe {
        family: CubicalType::MotiveFiber { motive, point },
        binder: coe_binder,
        from: Dim::Zero,
        to: Dim::One,
        term,
    } = body.as_ref()
    else {
        return None;
    };
    if *coe_binder != *binder + 1
        || motive != &standard_motive(typing)
        || !matches!(term.as_ref(), CubicalTerm::MotiveBase { motive: base } if base == motive)
    {
        return None;
    }
    let PointExpr::PathConstructor {
        owner,
        dimension,
        coordinates,
    } = point
    else {
        return None;
    };
    if owner != &typing.formation_normal_form || *dimension != typing.dimension {
        return None;
    }
    let probes = coordinates
        .iter()
        .enumerate()
        .filter_map(|(index, coordinate)| {
            (*coordinate == Dim::Var(*binder)).then_some(index as u32)
        })
        .collect::<Vec<_>>();
    let [probe] = probes.as_slice() else {
        return None;
    };
    let principals = coordinates
        .iter()
        .enumerate()
        .filter_map(|(index, coordinate)| {
            (*coordinate == Dim::Var(*coe_binder)).then_some(index as u32)
        })
        .collect::<Vec<_>>();
    let [principal] = principals.as_slice() else {
        return None;
    };
    if *principal == *probe
        || coordinates.iter().enumerate().any(|(index, coordinate)| {
            index as u32 != *probe
                && index as u32 != *principal
                && *coordinate != Dim::Var(index as u16)
        })
    {
        return None;
    }

    let context = CubicalContext::total(typing.dimension as u16);
    let endpoint = |argument| {
        normalize_typed_term(
            &context,
            &CubicalTerm::DimApp {
                function: Box::new(normal.clone()),
                argument,
            },
        )
        .ok()
        .map(|(_, normal)| normal)
    };
    let zero = endpoint(Dim::Zero);
    let one = endpoint(Dim::One);
    if zero != one
        || !matches!(zero, Some(CubicalTerm::MotiveBase { motive: base }) if base == *motive)
    {
        return None;
    }
    Some((*principal, *probe))
}

fn diagonal_index(typing: &FormedPathTyping, normal: &CubicalTerm) -> Option<u32> {
    let CubicalTerm::Coe {
        family: CubicalType::MotiveFiber { motive, point },
        binder,
        from: Dim::Zero,
        to: Dim::One,
        term,
    } = normal
    else {
        return None;
    };
    if *binder != typing.dimension as u16
        || motive != &standard_motive(typing)
        || !matches!(term.as_ref(), CubicalTerm::MotiveBase { motive: base } if base == motive)
    {
        return None;
    }
    let PointExpr::PathConstructor {
        owner,
        dimension,
        coordinates,
    } = point
    else {
        return None;
    };
    if owner != &typing.formation_normal_form || *dimension != typing.dimension {
        return None;
    }
    let principal = coordinates
        .iter()
        .enumerate()
        .filter_map(|(index, coordinate)| {
            (*coordinate == Dim::Var(*binder)).then_some(index as u32)
        })
        .collect::<Vec<_>>();
    let [principal] = principal.as_slice() else {
        return None;
    };
    if coordinates.iter().enumerate().any(|(index, coordinate)| {
        index as u32 != *principal && *coordinate != Dim::Var(index as u16)
    }) {
        return None;
    }
    Some(*principal)
}

fn is_standard_path(point: &PointExpr, typing: &FormedPathTyping) -> bool {
    point == &standard_path(typing)
}

/// Classification is deliberately downstream of type checking and
/// normalization.  No term constructor contains a `PathSchemaKey`.
fn derive_path_schema_key(
    typing: &FormedPathTyping,
    normal: &CubicalTerm,
) -> Result<PathSchemaKey, CubicalTypeError> {
    let key = match normal {
        CubicalTerm::PathMethod { motive, path, base }
            if motive == &standard_motive(typing)
                && is_standard_path(path, typing)
                && matches!(
                    base.as_ref(),
                    CubicalTerm::MotiveBase { motive: base_motive }
                        if base_motive == motive
                ) =>
        {
            PathSchemaKey::Beta
        }
        _ if diagonal_index(typing, normal).is_some() => {
            let principal = diagonal_index(typing, normal).expect("guard established diagonal");
            PathSchemaKey::Kan {
                principal,
                probe: principal,
            }
        }
        _ if off_diagonal_indices(typing, normal).is_some() => {
            let (principal, probe) = off_diagonal_indices(typing, normal)
                .expect("guard established off-diagonal indices");
            PathSchemaKey::Kan { principal, probe }
        }
        _ => return Err(CubicalTypeError::NotPathSchemaRealizer),
    };
    match key {
        PathSchemaKey::Beta => Ok(key),
        PathSchemaKey::Kan { principal, probe }
            if principal < typing.dimension && probe < typing.dimension =>
        {
            Ok(key)
        }
        PathSchemaKey::Kan { .. } => Err(CubicalTypeError::SchemaDirectionOutOfRange {
            dimension: typing.dimension,
        }),
    }
}

/// Opaque outside `pen-type`: all fields are private and there is no
/// deserializer.  Consumers can only obtain a token by replaying typing,
/// normalization and classification through [`issue_path_realization`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PathRealizationToken {
    source_digest: String,
    subject_hash: String,
    signature_digest: String,
    context: CubicalContext,
    term: CubicalTerm,
    normal_form: CubicalTerm,
    term_hash: String,
    normal_form_hash: String,
    key: PathSchemaKey,
    derivation_hash: String,
}

impl PathRealizationToken {
    pub fn key(&self) -> &PathSchemaKey {
        &self.key
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn subject_hash(&self) -> &str {
        &self.subject_hash
    }

    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }

    pub fn term_hash(&self) -> &str {
        &self.term_hash
    }

    pub fn normal_form_hash(&self) -> &str {
        &self.normal_form_hash
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_path_realization(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
    context: &CubicalContext,
    term: CubicalTerm,
) -> Result<PathRealizationToken, CubicalTypeError> {
    validate_formed_source(signature, telescope, visible_library, typing)?;
    if context.interval_count != typing.dimension as u16 || !context.assumption.is_true() {
        return Err(CubicalTypeError::NonStandardRealizationContext {
            expected: typing.dimension,
        });
    }
    let (_, normal_form) = normalize_typed_term(context, &term)?;
    let key = derive_path_schema_key(typing, &normal_form)?;
    let source_digest = source_digest(typing);
    let term_hash = tagged_digest("realizer-term", &(&source_digest, context, &term));
    let normal_form_hash = tagged_digest("realizer-normal-form", &(&source_digest, &normal_form));
    let derivation_hash = tagged_digest(
        "realization-derivation",
        &(
            &source_digest,
            &typing.subject_hash,
            &typing.signature_digest,
            &term_hash,
            &normal_form_hash,
            &key,
        ),
    );
    Ok(PathRealizationToken {
        source_digest,
        subject_hash: typing.subject_hash.clone(),
        signature_digest: typing.signature_digest.clone(),
        context: context.clone(),
        term,
        normal_form,
        term_hash,
        normal_form_hash,
        key,
        derivation_hash,
    })
}

pub fn replay_path_realization(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
    token: &PathRealizationToken,
) -> Result<(), CubicalTypeError> {
    validate_formed_source(signature, telescope, visible_library, typing)?;
    if token.source_digest != source_digest(typing)
        || token.subject_hash != typing.subject_hash
        || token.signature_digest != typing.signature_digest
    {
        return Err(CubicalTypeError::SourceMismatch);
    }
    let replay = issue_path_realization(
        signature,
        telescope,
        visible_library,
        typing,
        &token.context,
        token.term.clone(),
    )?;
    if &replay == token {
        Ok(())
    } else {
        Err(CubicalTypeError::SourceMismatch)
    }
}

/// Support-canonical projection of a replayable path token.  The owner
/// support is the fresh declaration identified by the telescope subject,
/// not merely its shallow formation expression.  This prevents two distinct
/// declarations with the same `App(Univ, Var(_))` presentation from being
/// collapsed before an equality witness exists.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalPathFamilyNormalForm {
    owner_support_hash: String,
    owner_normal_form: Expr,
    path_dimension: u32,
    key: PathSchemaKey,
    judgement: crate::tdc1::PathSchemaJudgement,
    type_hash: String,
}

impl HistoricalPathFamilyNormalForm {
    pub fn owner_support_hash(&self) -> &str {
        &self.owner_support_hash
    }

    pub fn owner_normal_form(&self) -> &Expr {
        &self.owner_normal_form
    }

    pub fn path_dimension(&self) -> u32 {
        self.path_dimension
    }

    pub fn key(&self) -> &PathSchemaKey {
        &self.key
    }

    pub fn judgement(&self) -> crate::tdc1::PathSchemaJudgement {
        self.judgement
    }

    pub fn type_hash(&self) -> &str {
        &self.type_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistoricalPathFamilyProjection {
    normal_form: HistoricalPathFamilyNormalForm,
    family_hash: String,
    projection_derivation_hash: String,
    typed_derivation_hash: String,
}

impl HistoricalPathFamilyProjection {
    pub fn normal_form(&self) -> &HistoricalPathFamilyNormalForm {
        &self.normal_form
    }

    pub fn family_hash(&self) -> &str {
        &self.family_hash
    }

    pub fn projection_derivation_hash(&self) -> &str {
        &self.projection_derivation_hash
    }

    pub fn typed_derivation_hash(&self) -> &str {
        &self.typed_derivation_hash
    }
}

pub fn project_historical_path_family(
    token: &PathRealizationToken,
) -> Result<HistoricalPathFamilyProjection, CubicalTypeError> {
    let path_dimension = token.path_dimension();
    if !(1..=3).contains(&path_dimension) {
        return Err(CubicalTypeError::HistoricalFamilyDimensionOutsideFragment {
            found: path_dimension,
        });
    }
    let inferred_type = infer_term(&token.context, &token.normal_form)?;
    let type_hash = tagged_digest("historical-path-family-type", &inferred_type);
    let judgement = match token.key {
        PathSchemaKey::Beta => crate::tdc1::PathSchemaJudgement::ConstructorComputation,
        PathSchemaKey::Kan { principal, probe } if principal == probe => {
            crate::tdc1::PathSchemaJudgement::FiberAutomorphism
        }
        PathSchemaKey::Kan { .. } => crate::tdc1::PathSchemaJudgement::AutomorphismNaturality,
    };
    let owner_support_hash = tagged_digest(
        "historical-fresh-owner-support",
        &(HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION, &token.subject_hash),
    );
    let normal_form = HistoricalPathFamilyNormalForm {
        owner_support_hash,
        owner_normal_form: cubical_type_owner(&inferred_type).clone(),
        path_dimension,
        key: token.key.clone(),
        judgement,
        type_hash,
    };
    let family_hash = tagged_digest(
        "historical-path-natural-family",
        &(HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION, &normal_form),
    );
    let projection_derivation_hash = tagged_digest(
        "historical-path-family-projection",
        &(
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
            &token.derivation_hash,
            &family_hash,
        ),
    );
    let typed_derivation_hash = tagged_digest(
        "historical-path-family-typed-derivation",
        &(
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
            &token.derivation_hash,
            &normal_form.type_hash,
            &inferred_type,
            &projection_derivation_hash,
        ),
    );
    Ok(HistoricalPathFamilyProjection {
        normal_form,
        family_hash,
        projection_derivation_hash,
        typed_derivation_hash,
    })
}

impl PathRealizationToken {
    fn path_dimension(&self) -> u32 {
        match &self.normal_form {
            CubicalTerm::PathMethod { path, .. } => point_dimension(path),
            CubicalTerm::Coe {
                family: CubicalType::MotiveFiber { point, .. },
                ..
            } => point_dimension(point),
            CubicalTerm::DimLambda { body, .. } => match body.as_ref() {
                CubicalTerm::Coe {
                    family: CubicalType::MotiveFiber { point, .. },
                    ..
                } => point_dimension(point),
                _ => 0,
            },
            _ => 0,
        }
    }
}

fn cubical_type_owner(ty: &CubicalType) -> &Expr {
    match ty {
        CubicalType::Owner { owner } => owner,
        CubicalType::MotiveFiber { motive, .. } => &motive.owner,
        CubicalType::DimensionFunction { body, .. } => cubical_type_owner(body),
    }
}

fn point_dimension(point: &PointExpr) -> u32 {
    match point {
        PointExpr::PathConstructor { dimension, .. } => *dimension,
        PointExpr::EndpointPathConstructor { .. } => 1,
        PointExpr::Base { .. }
        | PointExpr::BoundaryParameter { .. }
        | PointExpr::Neutral { .. } => 0,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum HistoricalPathEqualityDecision {
    Equal {
        left_family_hash: String,
        right_family_hash: String,
        equality_witness_hash: String,
        derivation_hash: String,
    },
    DistinctWithinFragment {
        left_family_hash: String,
        right_family_hash: String,
        rigid_separator: String,
        derivation_hash: String,
    },
    Undefined {
        left_family_hash: String,
        right_family_hash: String,
        obligation: String,
        derivation_hash: String,
    },
}

impl HistoricalPathEqualityDecision {
    pub fn is_equal(&self) -> bool {
        matches!(self, Self::Equal { .. })
    }

    pub fn is_distinct(&self) -> bool {
        matches!(self, Self::DistinctWithinFragment { .. })
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined { .. })
    }

    pub fn derivation_hash(&self) -> &str {
        match self {
            Self::Equal {
                derivation_hash, ..
            }
            | Self::DistinctWithinFragment {
                derivation_hash, ..
            }
            | Self::Undefined {
                derivation_hash, ..
            } => derivation_hash,
        }
    }

    pub fn reason(&self) -> &str {
        match self {
            Self::Equal { .. } => "equal in the frozen historical path-family fragment",
            Self::DistinctWithinFragment {
                rigid_separator, ..
            } => rigid_separator,
            Self::Undefined { obligation, .. } => obligation,
        }
    }
}

/// Compare two already-issued path-family projections in one predecessor
/// signature.  The procedure is deliberately three-valued.  A distinct
/// fresh owner is a rigid separator only when the sealed prefix contains no
/// universe-identity clause that would require a transport/univalence audit.
pub fn decide_historical_path_family_equality(
    signature: &SealedSignature,
    left: &PathRealizationToken,
    right: &PathRealizationToken,
) -> HistoricalPathEqualityDecision {
    let fallback_left_hash = tagged_digest(
        "historical-path-unprojected-token",
        &(
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
            &left.derivation_hash,
        ),
    );
    let fallback_right_hash = tagged_digest(
        "historical-path-unprojected-token",
        &(
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
            &right.derivation_hash,
        ),
    );
    let left_projection = match project_historical_path_family(left) {
        Ok(projection) => projection,
        Err(error) => {
            return historical_path_equality_undefined(
                fallback_left_hash,
                fallback_right_hash,
                format!("left family is outside the historical projection fragment: {error}"),
            );
        }
    };
    let right_projection = match project_historical_path_family(right) {
        Ok(projection) => projection,
        Err(error) => {
            return historical_path_equality_undefined(
                fallback_left_hash,
                fallback_right_hash,
                format!("right family is outside the historical projection fragment: {error}"),
            );
        }
    };
    let left_hash = left_projection.family_hash().to_owned();
    let right_hash = right_projection.family_hash().to_owned();
    let undefined = |obligation: String| {
        let derivation_hash = tagged_digest(
            "historical-path-equality-undefined",
            &(
                HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
                &left_hash,
                &right_hash,
                &obligation,
            ),
        );
        HistoricalPathEqualityDecision::Undefined {
            left_family_hash: left_hash.clone(),
            right_family_hash: right_hash.clone(),
            obligation,
            derivation_hash,
        }
    };
    if left.signature_digest != signature.digest() || right.signature_digest != signature.digest() {
        return undefined(
            "both family tokens must be reissued and replayed in the audited predecessor signature"
                .to_owned(),
        );
    }
    let left_normal = left_projection.normal_form();
    let right_normal = right_projection.normal_form();
    let separator = if left_normal.owner_support_hash() != right_normal.owner_support_hash() {
        if signature_has_identity_clause(signature) {
            return undefined(
                "the prefix contains identity syntax; fresh-owner separation requires a typed univalent transport audit"
                    .to_owned(),
            );
        }
        Some("distinct fresh declaration support under the equality-free sealed prefix")
    } else if left_normal.path_dimension() != right_normal.path_dimension() {
        Some("path dimension")
    } else if left_normal.key() != right_normal.key() {
        Some("ordered path-family key")
    } else if left_normal.type_hash() != right_normal.type_hash() {
        Some("inferred family type")
    } else {
        None
    };
    if let Some(separator) = separator {
        let rigid_separator = separator.to_owned();
        let derivation_hash = tagged_digest(
            "historical-path-equality-separation",
            &(
                HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
                &left_hash,
                &right_hash,
                &rigid_separator,
                signature.digest(),
            ),
        );
        return HistoricalPathEqualityDecision::DistinctWithinFragment {
            left_family_hash: left_hash,
            right_family_hash: right_hash,
            rigid_separator,
            derivation_hash,
        };
    }

    let scope_len = left_normal
        .owner_normal_form()
        .var_refs()
        .into_iter()
        .max()
        .unwrap_or(0);
    let equality = match univalent_equality(
        left_normal.owner_normal_form(),
        right_normal.owner_normal_form(),
        scope_len,
        64,
    ) {
        Ok(equality) => equality,
        Err(error) => {
            return undefined(format!(
                "frozen univalent equality did not decide the owner comparison: {error}"
            ));
        }
    };
    if !equality.equal {
        let rigid_separator =
            "owner normal forms are unequal under frozen univalent equality".to_owned();
        let derivation_hash = tagged_digest(
            "historical-path-equality-separation",
            &(
                HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
                &left_hash,
                &right_hash,
                &rigid_separator,
                &equality,
            ),
        );
        return HistoricalPathEqualityDecision::DistinctWithinFragment {
            left_family_hash: left_hash,
            right_family_hash: right_hash,
            rigid_separator,
            derivation_hash,
        };
    }
    let equality_witness_hash = tagged_digest(
        "historical-path-owner-equality",
        &(
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
            &left_hash,
            &right_hash,
            &equality,
        ),
    );
    let derivation_hash = tagged_digest(
        "historical-path-equality",
        &(
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
            left_projection.typed_derivation_hash(),
            right_projection.typed_derivation_hash(),
            &equality_witness_hash,
        ),
    );
    HistoricalPathEqualityDecision::Equal {
        left_family_hash: left_hash,
        right_family_hash: right_hash,
        equality_witness_hash,
        derivation_hash,
    }
}

fn historical_path_equality_undefined(
    left_family_hash: String,
    right_family_hash: String,
    obligation: String,
) -> HistoricalPathEqualityDecision {
    let derivation_hash = tagged_digest(
        "historical-path-equality-undefined",
        &(
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION,
            &left_family_hash,
            &right_family_hash,
            &obligation,
        ),
    );
    HistoricalPathEqualityDecision::Undefined {
        left_family_hash,
        right_family_hash,
        obligation,
        derivation_hash,
    }
}

fn signature_has_identity_clause(signature: &SealedSignature) -> bool {
    signature.entries().iter().any(|entry| {
        entry
            .telescope
            .clauses
            .iter()
            .any(|clause| expr_has_identity(&clause.expr))
    })
}

fn expr_has_identity(expr: &Expr) -> bool {
    match expr {
        Expr::Id(_, _, _) => true,
        Expr::Pi(left, right) | Expr::Sigma(left, right) | Expr::App(left, right) => {
            expr_has_identity(left) || expr_has_identity(right)
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => expr_has_identity(body),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PathBasisRealization {
    dimension: u32,
    tokens: Vec<PathRealizationToken>,
    derivation_hash: String,
}

impl PathBasisRealization {
    pub fn dimension(&self) -> u32 {
        self.dimension
    }

    pub fn tokens(&self) -> &[PathRealizationToken] {
        &self.tokens
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn realize_path_basis(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
) -> Result<PathBasisRealization, CubicalTypeError> {
    validate_formed_source(signature, telescope, visible_library, typing)?;
    let context = CubicalContext::total(typing.dimension as u16);
    let mut tokens = Vec::with_capacity(1 + (typing.dimension * typing.dimension) as usize);
    tokens.push(issue_path_realization(
        signature,
        telescope,
        visible_library,
        typing,
        &context,
        beta_realizer_term(typing),
    )?);
    for principal in 0..typing.dimension {
        for probe in 0..typing.dimension {
            let term = if principal == probe {
                diagonal_realizer_term(typing, principal)
            } else {
                off_diagonal_realizer_term(typing, principal, probe)
            };
            tokens.push(issue_path_realization(
                signature,
                telescope,
                visible_library,
                typing,
                &context,
                term,
            )?);
        }
    }
    let keys = tokens
        .iter()
        .map(|token| token.key.clone())
        .collect::<BTreeSet<_>>();
    if keys.len() != tokens.len() || tokens.len() != 1 + (typing.dimension.pow(2) as usize) {
        return Err(CubicalTypeError::NotPathSchemaRealizer);
    }
    for token in &tokens {
        replay_path_realization(signature, telescope, visible_library, typing, token)?;
    }
    let derivation_hash = tagged_digest(
        "basis-realization",
        &(
            source_digest(typing),
            tokens
                .iter()
                .map(PathRealizationToken::derivation_hash)
                .collect::<Vec<_>>(),
        ),
    );
    Ok(PathBasisRealization {
        dimension: typing.dimension,
        tokens,
        derivation_hash,
    })
}

fn validate_formed_source(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    typing: &FormedPathTyping,
) -> Result<(), CubicalTypeError> {
    let (replayed, _) =
        elaborate_formed_path(signature, telescope, visible_library).map_err(|error| {
            CubicalTypeError::FormedSourceReplayFailed {
                reason: error.to_string(),
            }
        })?;
    if &replayed == typing && typing.signature_digest == signature.digest() {
        Ok(())
    } else {
        Err(CubicalTypeError::SourceMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::telescope::Telescope;

    fn source(dimension: u32) -> (SealedSignature, Telescope, FormedPathTyping) {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(15))),
            ),
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(dimension)),
        ]);
        let typing = elaborate_formed_path(&signature, &candidate, 15)
            .expect("formed path")
            .0;
        (signature, candidate, typing)
    }

    fn point(owner: Expr) -> CubicalTerm {
        CubicalTerm::Point {
            point: PointExpr::Base { owner },
        }
    }

    #[test]
    fn constructive_face_dnf_is_canonical() {
        let i0 = Endpoint::zero(0);
        let i1 = Endpoint::one(0);
        let j0 = Endpoint::zero(1);
        assert!(Cofibration::conjunction([i0, i1]).is_false());
        let absorbed = Cofibration::disjunction([vec![i0, j0], vec![i0], vec![i0]]);
        assert_eq!(absorbed, Cofibration::endpoint(i0));
        assert!(
            Cofibration::endpoint(i0)
                .meet(&Cofibration::endpoint(i1))
                .is_false()
        );
        assert!(Cofibration::conjunction([i0, j0]).implies(&Cofibration::endpoint(i0)));
    }

    #[test]
    fn context_rejects_out_of_scope_faces() {
        assert_eq!(
            CubicalContext::new(1, Cofibration::endpoint(Endpoint::zero(1))),
            Err(CubicalTypeError::IntervalOutOfScope {
                variable: 1,
                interval_count: 1
            })
        );
    }

    #[test]
    fn coe_and_hcom_computation_rules_are_oriented_and_typed() {
        let (signature, telescope, typing) = source(1);
        let context = CubicalContext::total(1);
        let cap = standard_cap(&typing);
        let family = CubicalType::Owner {
            owner: typing.formation_normal_form.clone(),
        };
        let coe = CubicalTerm::Coe {
            family: family.clone(),
            binder: 1,
            from: Dim::Zero,
            to: Dim::One,
            term: Box::new(cap.clone()),
        };
        // Constant-family regularity collapses; a vacuous Owner coe cannot
        // mint a diagonal key.
        assert_eq!(normalize_typed_term(&context, &coe).unwrap().1, cap);
        assert_eq!(
            issue_path_realization(&signature, &telescope, 15, &typing, &context, coe),
            Err(CubicalTypeError::NotPathSchemaRealizer)
        );

        let hcom_cap = substitute_term(&cap, 0, Dim::Zero);
        let hcom = CubicalTerm::Hcom {
            family: family.clone(),
            binder: 1,
            from: Dim::Zero,
            to: Dim::One,
            cap: Box::new(hcom_cap.clone()),
            tubes: vec![Tube {
                face: Cofibration::true_formula(),
                body: Box::new(hcom_cap.clone()),
            }],
        };
        assert_eq!(normalize_typed_term(&context, &hcom).unwrap().1, hcom_cap);

        // There is no independent `cap_boundary` witness.  The actual tube
        // body is restricted at the start and rejected when it disagrees.
        let forged_body = CubicalTerm::Hcom {
            family: family.clone(),
            binder: 2,
            from: Dim::Zero,
            to: Dim::One,
            cap: Box::new(hcom_cap.clone()),
            tubes: vec![],
        };
        let bad = CubicalTerm::Hcom {
            family,
            binder: 1,
            from: Dim::Zero,
            to: Dim::One,
            cap: Box::new(hcom_cap),
            tubes: vec![Tube {
                face: Cofibration::true_formula(),
                body: Box::new(forged_body),
            }],
        };
        assert_eq!(
            infer_term(&context, &bad),
            Err(CubicalTypeError::CapFaceMismatch)
        );
    }

    #[test]
    fn hcom_rejects_overlap_disagreement_and_replays_staged_union() {
        let owner = Expr::Univ;
        let context = CubicalContext::total(3);
        let cap = point(owner.clone());
        let family = CubicalType::Owner {
            owner: owner.clone(),
        };
        let binder_sensitive_body = CubicalTerm::Point {
            point: PointExpr::PathConstructor {
                owner: owner.clone(),
                dimension: 1,
                coordinates: vec![Dim::Var(3)],
            },
        };
        let face_i = Cofibration::true_formula();
        let face_j = Cofibration::endpoint(Endpoint::zero(2));
        let tubes = vec![
            Tube {
                face: face_i,
                body: Box::new(binder_sensitive_body.clone()),
            },
            Tube {
                face: face_j,
                body: Box::new(binder_sensitive_body),
            },
        ];
        let replay = replay_hcom_staged_union(
            &context,
            family.clone(),
            3,
            Dim::Zero,
            Dim::One,
            cap.clone(),
            tubes.clone(),
        )
        .expect("staged replay");
        assert_eq!(replay.direct_union, replay.staged_union);
        assert_eq!(replay.stages_checked, 2);
        assert!(replay.flatten_law_checked);
        assert_ne!(replay.nested_term_digest, replay.flattened_term_digest);

        let mut bad = tubes;
        // This body restricts to the cap at the outer start (`i0=0`), so
        // both individual tubes pass cap agreement.  On their overlap it is
        // still a nontrivial composition and must be rejected.
        bad[1].body = Box::new(CubicalTerm::Hcom {
            family: family.clone(),
            binder: 4,
            from: Dim::Zero,
            to: Dim::Var(3),
            cap: Box::new(cap.clone()),
            tubes: vec![],
        });
        let term = CubicalTerm::Hcom {
            family,
            binder: 3,
            from: Dim::Zero,
            to: Dim::One,
            cap: Box::new(cap),
            tubes: bad,
        };
        assert_eq!(
            infer_term(&context, &term),
            Err(CubicalTypeError::TubeOverlapMismatch)
        );
    }

    #[test]
    fn motive_typed_eliminator_beta_reduces_to_method() {
        let (_, _, typing) = source(2);
        let context = CubicalContext::total(2);
        let term = beta_realizer_term(&typing);
        let (_, normal) = normalize_typed_term(&context, &term).expect("typed eliminator");
        assert!(matches!(normal, CubicalTerm::PathMethod { .. }));
        assert_eq!(
            derive_path_schema_key(&typing, &normal),
            Ok(PathSchemaKey::Beta)
        );
        assert!(matches!(
            substitute_term(&normal, 0, Dim::Zero),
            CubicalTerm::MotiveBase { .. }
        ));
    }

    #[test]
    fn motive_typed_eliminator_at_a_neutral_has_type_p_x_and_stays_stuck() {
        let (_, _, typing) = source(2);
        let context = CubicalContext::total(2);
        let motive = standard_motive(&typing);
        let base = CubicalTerm::MotiveBase {
            motive: motive.clone(),
        };
        let method = CubicalTerm::PathMethod {
            motive: motive.clone(),
            path: standard_path(&typing),
            base: Box::new(base.clone()),
        };
        let neutral = PointExpr::Neutral {
            owner: typing.formation_normal_form.clone(),
            name: "x".to_owned(),
        };
        let term = CubicalTerm::PathElim {
            motive: motive.clone(),
            base: Box::new(base),
            method: Box::new(method),
            scrutinee: Box::new(CubicalTerm::Point {
                point: neutral.clone(),
            }),
        };
        let (ty, normal) = normalize_typed_term(&context, &term).expect("neutral eliminator");
        assert_eq!(
            ty,
            CubicalType::MotiveFiber {
                motive,
                point: neutral,
            }
        );
        assert!(matches!(
            normal,
            CubicalTerm::PathElim {
                scrutinee,
                ..
            } if matches!(
                scrutinee.as_ref(),
                CubicalTerm::Point {
                    point: PointExpr::Neutral { name, .. }
                } if name == "x"
            )
        ));
    }

    #[test]
    fn endpoint_constructor_and_pathp_method_compute_at_declared_faces() {
        let (_, _, typing) = source(1);
        let context = CubicalContext::total(1);
        let premises = issue_endpoint_schema_premise_context(
            &typing,
            "endpoint-premise:test-constructor",
            1,
            2,
        )
        .expect("endpoint formal premise context");
        let path = endpoint_path(&typing, Dim::Var(0), 1, 2);
        let method = endpoint_method(&premises);
        assert_eq!(
            infer_term(&context, &method),
            Err(CubicalTypeError::EndpointPremiseContextRequired),
            "ordinary context-free inference must not manufacture q"
        );
        assert!(matches!(
            infer_term_with_endpoint_premises(&context, &premises, &method),
            Ok(CubicalType::MotiveFiber {
                point: PointExpr::EndpointPathConstructor { .. },
                ..
            })
        ));
        assert_eq!(
            substitute_term(&CubicalTerm::Point { point: path }, 0, Dim::Zero),
            CubicalTerm::Point {
                point: endpoint_boundary_point(&typing, 1)
            }
        );
        assert_eq!(substitute_term(&method, 0, Dim::Zero), premises.zero_term());
        assert_eq!(substitute_term(&method, 0, Dim::One), premises.one_term());

        let owner = typing.formation_normal_form.clone();
        let degenerate = PointExpr::EndpointPathConstructor {
            owner: owner.clone(),
            coordinate: Dim::Var(0),
            zero: Box::new(PointExpr::Base {
                owner: owner.clone(),
            }),
            one: Box::new(PointExpr::Base { owner }),
        };
        assert!(matches!(
            degenerate.substitute(0, Dim::Zero),
            PointExpr::Base { .. }
        ));
        assert!(matches!(
            degenerate.substitute(0, Dim::One),
            PointExpr::Base { .. }
        ));
    }

    #[test]
    fn endpoint_formal_premise_context_rejects_missing_mutated_and_cross_source_refs() {
        let (_, _, typing) = source(1);
        let context = CubicalContext::total(1);
        let left =
            issue_endpoint_schema_premise_context(&typing, "endpoint-premise:source-left", 1, 2)
                .expect("left formal premise context");
        let right =
            issue_endpoint_schema_premise_context(&typing, "endpoint-premise:source-right", 1, 2)
                .expect("right formal premise context");
        let left_evaluation = left.zero_term();
        let left_method = left.method_term();
        assert_eq!(
            infer_term(&context, &left_evaluation),
            Err(CubicalTypeError::EndpointPremiseContextRequired)
        );
        assert_eq!(
            infer_term(&context, &left_method),
            Err(CubicalTypeError::EndpointPremiseContextRequired)
        );
        assert_eq!(
            infer_term_with_endpoint_premises(&context, &right, &left_evaluation),
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        );
        assert_eq!(
            infer_term_with_endpoint_premises(&context, &right, &left_method),
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        );
        assert_eq!(
            replay_endpoint_schema_premise_context(
                &typing,
                "endpoint-premise:source-left",
                1,
                2,
                &right,
            ),
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        );

        let mut mutated = left.clone();
        mutated.method_premise_derivation_hash.push_str(":mutated");
        assert_eq!(
            infer_term_with_endpoint_premises(&context, &mutated, &mutated.method_term()),
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        );

        let mut incoherent =
            issue_endpoint_schema_premise_context(&typing, "endpoint-premise:contraction", 1, 1)
                .expect("coherent contracted premise context");
        incoherent
            .one_evaluation
            .premise_derivation_hash
            .push_str(":incoherent");
        assert_eq!(
            infer_term_with_endpoint_premises(&context, &incoherent, &incoherent.method_term(),),
            Err(CubicalTypeError::EndpointPremiseContractionMismatch)
        );
    }

    #[test]
    fn endpoint_point_syntax_cannot_forge_transport_to_a_neutral() {
        let (_, _, typing) = source(1);
        let context = CubicalContext::total(0);
        let owner = typing.formation_normal_form.clone();
        let motive = standard_motive(&typing);
        let neutral = PointExpr::Neutral {
            owner: owner.clone(),
            name: "forged-transport-target".to_owned(),
        };
        let context_free_attack = CubicalTerm::Coe {
            family: CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: PointExpr::EndpointPathConstructor {
                    owner: owner.clone(),
                    coordinate: Dim::Var(0),
                    zero: Box::new(PointExpr::Base {
                        owner: owner.clone(),
                    }),
                    one: Box::new(neutral.clone()),
                },
            },
            binder: 0,
            from: Dim::Zero,
            to: Dim::One,
            term: Box::new(CubicalTerm::MotiveBase {
                motive: motive.clone(),
            }),
        };
        assert_eq!(
            infer_term(&context, &context_free_attack),
            Err(CubicalTypeError::EndpointPremiseContextRequired)
        );

        let premises = issue_endpoint_schema_premise_context(
            &typing,
            "endpoint-premise:forged-transport",
            1,
            2,
        )
        .expect("endpoint formal premise context");
        assert_eq!(
            infer_term_with_endpoint_premises(&context, &premises, &context_free_attack),
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        );

        let ledger_attack = CubicalTerm::Coe {
            family: CubicalType::MotiveFiber {
                motive,
                point: PointExpr::EndpointPathConstructor {
                    owner: owner.clone(),
                    coordinate: Dim::Var(0),
                    zero: Box::new(endpoint_boundary_point(&typing, 1)),
                    one: Box::new(neutral),
                },
            },
            binder: 0,
            from: Dim::Zero,
            to: Dim::One,
            term: Box::new(premises.zero_term()),
        };
        assert_eq!(
            infer_term_with_endpoint_premises(&context, &premises, &ledger_attack),
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        );

        assert_eq!(
            infer_term(
                &CubicalContext::total(1),
                &CubicalTerm::Point {
                    point: endpoint_path(&typing, Dim::Var(0), 1, 2),
                },
            ),
            Err(CubicalTypeError::EndpointPremiseContextRequired)
        );
    }

    #[test]
    fn endpoint_eliminator_computes_on_faces_and_constructor_but_sticks_on_neutral() {
        let (_, _, typing) = source(1);
        let context = CubicalContext::total(1);
        let motive = standard_motive(&typing);
        let premises = issue_endpoint_schema_premise_context(
            &typing,
            "endpoint-premise:test-eliminator",
            1,
            2,
        )
        .expect("endpoint formal premise context");
        let method = endpoint_method(&premises);
        let method_reference = premises.method_reference();
        let constructor = endpoint_path(&typing, Dim::Var(0), 1, 2);
        let beta = CubicalTerm::EndpointPathElim {
            motive: motive.clone(),
            method: Box::new(method.clone()),
            scrutinee: Box::new(CubicalTerm::Point { point: constructor }),
        };
        assert_eq!(
            normalize_typed_term(&context, &beta),
            Err(CubicalTypeError::EndpointPremiseContextRequired)
        );
        let (_, beta_normal) = normalize_endpoint_typed_term(&context, &premises, &beta)
            .expect("endpoint beta under formal premises");
        assert_eq!(beta_normal, method);

        let zero = CubicalTerm::EndpointPathElim {
            motive: motive.clone(),
            method: Box::new(endpoint_method(&premises)),
            scrutinee: Box::new(CubicalTerm::Point {
                point: endpoint_boundary_point(&typing, 1),
            }),
        };
        assert_eq!(
            normalize_endpoint_typed_term(&context, &premises, &zero)
                .expect("zero endpoint")
                .1,
            premises.zero_term()
        );

        let neutral = PointExpr::Neutral {
            owner: typing.formation_normal_form.clone(),
            name: "z".to_owned(),
        };
        let stuck = CubicalTerm::EndpointPathElim {
            motive: motive.clone(),
            method: Box::new(endpoint_method(&premises)),
            scrutinee: Box::new(CubicalTerm::Point {
                point: neutral.clone(),
            }),
        };
        let (stuck_ty, stuck_normal) = normalize_endpoint_typed_term(&context, &premises, &stuck)
            .expect("typed neutral endpoint eliminator");
        assert_eq!(
            stuck_ty,
            CubicalType::MotiveFiber {
                motive,
                point: neutral.clone(),
            }
        );
        assert!(matches!(
            &stuck_normal,
            CubicalTerm::EndpointElimNeutral {
                method: retained_method,
                scrutinee: retained_scrutinee,
                ..
            } if retained_method == &method_reference
                && retained_scrutinee.as_ref()
                    == &CubicalTerm::Point {
                        point: neutral.clone()
                    }
        ));

        let (_, beta_zero) =
            substitute_endpoint_typed_term(&context, &premises, &beta, 0, Dim::Zero)
                .expect("endpoint beta substitution preserves typing");
        assert_eq!(beta_zero, premises.zero_term());
        let (_, beta_one) = substitute_endpoint_typed_term(&context, &premises, &beta, 0, Dim::One)
            .expect("endpoint beta substitution at One preserves typing");
        assert_eq!(beta_one, premises.one_term());
        let (_, neutral_zero) =
            substitute_endpoint_typed_term(&context, &premises, &stuck, 0, Dim::Zero)
                .expect("neutral endpoint elimination substitution preserves typing");
        assert!(matches!(
            neutral_zero,
            CubicalTerm::EndpointElimNeutral { method, scrutinee, .. }
                if method == method_reference
                    && scrutinee.as_ref()
                        == &CubicalTerm::Point {
                            point: neutral.clone()
                        }
        ));

        let mut evaluation_derived_method = premises.method_reference();
        evaluation_derived_method.premise_derivation_hash =
            premises.zero_evaluation.premise_derivation_hash.clone();
        let forged_neutral = CubicalTerm::EndpointElimNeutral {
            motive: standard_motive(&typing),
            method: evaluation_derived_method,
            scrutinee: Box::new(CubicalTerm::Point { point: neutral }),
        };
        assert_eq!(
            infer_term_with_endpoint_premises(&context, &premises, &forged_neutral),
            Err(CubicalTypeError::EndpointPremiseContextMismatch)
        );

        for reducible_point in [
            endpoint_boundary_point(&typing, 1),
            endpoint_boundary_point(&typing, 2),
            premises.method_path.clone(),
        ] {
            let reducible_neutral = CubicalTerm::EndpointElimNeutral {
                motive: standard_motive(&typing),
                method: premises.method_reference(),
                scrutinee: Box::new(CubicalTerm::Point {
                    point: reducible_point,
                }),
            };
            assert_eq!(
                infer_term_with_endpoint_premises(&context, &premises, &reducible_neutral),
                Err(CubicalTypeError::EndpointNeutralScrutineeReducible)
            );
        }
    }

    #[test]
    fn endpoint_computation_audit_accepts_degenerate_instantiations() {
        let (_, _, typing) = source(1);
        let x_premises =
            issue_endpoint_schema_premise_context(&typing, "endpoint-premise:collapse-x", 1, 1)
                .expect("x/x formal premise context");
        let y_premises =
            issue_endpoint_schema_premise_context(&typing, "endpoint-premise:collapse-y", 2, 2)
                .expect("y/y formal premise context");
        assert_eq!(x_premises.zero_term(), x_premises.one_term());
        assert_eq!(y_premises.zero_term(), y_premises.one_term());
        let collapse_to_x = audit_endpoint_path_computation(&typing, &x_premises, 1, 1)
            .expect("x/x is a typed restricted instantiation");
        let collapse_to_y = audit_endpoint_path_computation(&typing, &y_premises, 2, 2)
            .expect("y/y is a typed restricted instantiation");
        assert_ne!(
            collapse_to_x.derivation_hash(),
            collapse_to_y.derivation_hash()
        );
    }

    #[test]
    fn endpoint_boundary_aware_coe_and_exact_two_key_basis_replay() {
        let (signature, telescope, typing) = source(1);
        let context = CubicalContext::total(1);
        let binding = "typed-boundary:test";
        let instantiation = "restricted-instantiation:test";
        let premise_source = endpoint_source_digest(&typing, binding, instantiation, 1, 2);
        let premises = issue_endpoint_schema_premise_context(&typing, &premise_source, 1, 2)
            .expect("source-bound endpoint premise context");
        let transport = endpoint_transport_realizer_term(&typing, &premises, 1, 2);
        assert!(matches!(
            &transport,
            CubicalTerm::Coe { term, .. } if term.as_ref() == &premises.zero_term()
        ));
        assert_eq!(
            normalize_typed_term(&context, &transport),
            Err(CubicalTypeError::EndpointPremiseContextRequired)
        );
        let (transport_ty, transport_normal) =
            normalize_endpoint_typed_term(&context, &premises, &transport)
                .expect("endpoint-aware coe");
        assert_eq!(
            transport_ty,
            CubicalType::MotiveFiber {
                motive: standard_motive(&typing),
                point: endpoint_boundary_point(&typing, 2),
            }
        );
        assert!(matches!(transport_normal, CubicalTerm::Coe { .. }));

        let basis = realize_endpoint_path_basis(
            &signature,
            &telescope,
            15,
            &typing,
            binding,
            instantiation,
            1,
            2,
            &premises,
        )
        .expect("endpoint basis");
        assert_eq!(basis.tokens().len(), 2);
        assert_eq!(basis.tokens()[0].key(), &PathSchemaKey::Beta);
        assert_eq!(
            basis.tokens()[1].key(),
            &PathSchemaKey::Kan {
                principal: 0,
                probe: 0,
            }
        );
        for token in basis.tokens() {
            replay_endpoint_path_realization(
                &signature,
                &telescope,
                15,
                &typing,
                binding,
                instantiation,
                1,
                2,
                &premises,
                token,
            )
            .expect("endpoint realization replay");
        }
    }

    #[test]
    fn every_endpoint_realization_token_field_mutation_fails_replay() {
        let (signature, telescope, typing) = source(1);
        let binding = "typed-boundary:mutation-test";
        let instantiation = "restricted-instantiation:mutation-test";
        let premise_source = endpoint_source_digest(&typing, binding, instantiation, 1, 2);
        let premises = issue_endpoint_schema_premise_context(&typing, &premise_source, 1, 2)
            .expect("source-bound endpoint premise context");
        let basis = realize_endpoint_path_basis(
            &signature,
            &telescope,
            15,
            &typing,
            binding,
            instantiation,
            1,
            2,
            &premises,
        )
        .expect("endpoint basis");
        let token = basis.tokens()[0].clone();
        for field in 0..18 {
            let mut mutated = token.clone();
            match field {
                0 => mutated.fragment_version.push_str(":mutated"),
                1 => mutated.source_digest.push_str(":mutated"),
                2 => mutated.subject_hash.push_str(":mutated"),
                3 => mutated.signature_digest.push_str(":mutated"),
                4 => mutated.boundary_binding_digest.push_str(":mutated"),
                5 => mutated.parameter_instantiation_digest.push_str(":mutated"),
                6 => mutated.zero_parameter += 1,
                7 => mutated.one_parameter += 1,
                8 => mutated.premise_source_digest.push_str(":mutated"),
                9 => mutated.premise_context_derivation_hash.push_str(":mutated"),
                10 => mutated.context.interval_count += 1,
                11 => {
                    mutated.term = CubicalTerm::MotiveBase {
                        motive: standard_motive(&typing),
                    }
                }
                12 => mutated.normal_form = mutated.term.clone(),
                13 => mutated.term_hash.push_str(":mutated"),
                14 => mutated.normal_form_hash.push_str(":mutated"),
                15 => mutated.type_hash.push_str(":mutated"),
                16 => {
                    mutated.key = PathSchemaKey::Kan {
                        principal: 9,
                        probe: 9,
                    }
                }
                17 => mutated.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert!(
                replay_endpoint_path_realization(
                    &signature,
                    &telescope,
                    15,
                    &typing,
                    binding,
                    instantiation,
                    1,
                    2,
                    &premises,
                    &mutated,
                )
                .is_err(),
                "endpoint realization field {field} must fail replay"
            );
        }
    }

    #[test]
    fn diagonal_uses_a_genuinely_dependent_motive_family() {
        let (signature, telescope, typing) = source(2);
        let context = CubicalContext::total(2);
        let term = diagonal_realizer_term(&typing, 0);
        let (ty, normal) = normalize_typed_term(&context, &term).expect("dependent coe types");
        assert!(matches!(normal, CubicalTerm::Coe { .. }));
        assert!(matches!(
            ty,
            CubicalType::MotiveFiber {
                point: PointExpr::Base { .. },
                ..
            }
        ));
        let token = issue_path_realization(&signature, &telescope, 15, &typing, &context, term)
            .expect("dependent diagonal token");
        assert_eq!(
            token.key(),
            &PathSchemaKey::Kan {
                principal: 0,
                probe: 0
            }
        );
    }

    #[test]
    fn off_diagonal_tokens_require_the_exact_dependent_lambda_shape() {
        let (signature, telescope, typing) = source(2);
        let context = CubicalContext::total(2);
        let term = off_diagonal_realizer_term(&typing, 0, 1);
        let (_, normal) = normalize_typed_term(&context, &term).expect("off-diagonal term");
        assert!(matches!(
            normal,
            CubicalTerm::DimLambda {
                binder: 2,
                body,
            } if matches!(
                body.as_ref(),
                CubicalTerm::Coe {
                    binder: 3,
                    family,
                    ..
                } if family.depends_on(3)
            )
        ));
        let endpoint_normal = |argument| {
            normalize_typed_term(
                &context,
                &CubicalTerm::DimApp {
                    function: Box::new(term.clone()),
                    argument,
                },
            )
            .expect("typed endpoint")
            .1
        };
        let zero = endpoint_normal(Dim::Zero);
        let one = endpoint_normal(Dim::One);
        assert_eq!(zero, one);
        assert!(matches!(zero, CubicalTerm::MotiveBase { .. }));

        let token =
            issue_path_realization(&signature, &telescope, 15, &typing, &context, term.clone())
                .expect("off-diagonal token");
        assert_eq!(
            token.key(),
            &PathSchemaKey::Kan {
                principal: 0,
                probe: 1,
            }
        );
        let transpose = issue_path_realization(
            &signature,
            &telescope,
            15,
            &typing,
            &context,
            off_diagonal_realizer_term(&typing, 1, 0),
        )
        .expect("transposed token");
        assert_ne!(token.normal_form_hash(), transpose.normal_form_hash());
        assert_ne!(token.derivation_hash(), transpose.derivation_hash());

        let mut wrong_binder = term.clone();
        if let CubicalTerm::DimLambda { binder, .. } = &mut wrong_binder {
            *binder = 3;
        }
        assert!(
            issue_path_realization(&signature, &telescope, 15, &typing, &context, wrong_binder,)
                .is_err()
        );

        let mut wrong_coordinate = term.clone();
        if let CubicalTerm::DimLambda { body, .. } = &mut wrong_coordinate
            && let CubicalTerm::Coe {
                family: CubicalType::MotiveFiber { point, .. },
                ..
            } = body.as_mut()
            && let PointExpr::PathConstructor { coordinates, .. } = point
        {
            coordinates[0] = Dim::Zero;
        }
        assert_eq!(
            issue_path_realization(
                &signature,
                &telescope,
                15,
                &typing,
                &context,
                wrong_coordinate,
            ),
            Err(CubicalTypeError::NotPathSchemaRealizer)
        );

        let mut wrong_endpoint = term;
        if let CubicalTerm::DimLambda { body, .. } = &mut wrong_endpoint
            && let CubicalTerm::Coe { to, .. } = body.as_mut()
        {
            *to = Dim::Zero;
        }
        assert_eq!(
            issue_path_realization(
                &signature,
                &telescope,
                15,
                &typing,
                &context,
                wrong_endpoint,
            ),
            Err(CubicalTypeError::NotPathSchemaRealizer)
        );
    }

    #[test]
    fn endpoint_mutation_and_degenerate_point_hcom_cannot_mint_keys() {
        let (signature, telescope, typing) = source(2);
        let context = CubicalContext::total(2);
        let motive = standard_motive(&typing);
        let base = CubicalTerm::MotiveBase {
            motive: motive.clone(),
        };
        let mutation = CubicalTerm::DimLambda {
            binder: 2,
            body: Box::new(CubicalTerm::Hcom {
                family: CubicalType::MotiveFiber {
                    motive,
                    point: PointExpr::Base {
                        owner: typing.formation_normal_form.clone(),
                    },
                },
                binder: 3,
                from: Dim::Zero,
                to: Dim::Var(2),
                cap: Box::new(base),
                tubes: vec![],
            }),
        };
        let endpoint = |argument| {
            normalize_typed_term(
                &context,
                &CubicalTerm::DimApp {
                    function: Box::new(mutation.clone()),
                    argument,
                },
            )
            .expect("mutation endpoint types")
            .1
        };
        assert!(matches!(
            endpoint(Dim::Zero),
            CubicalTerm::MotiveBase { .. }
        ));
        assert!(matches!(endpoint(Dim::One), CubicalTerm::Hcom { .. }));
        assert_eq!(
            issue_path_realization(&signature, &telescope, 15, &typing, &context, mutation,),
            Err(CubicalTypeError::NotPathSchemaRealizer)
        );

        let point_hcom = CubicalTerm::Hcom {
            family: CubicalType::Owner {
                owner: typing.formation_normal_form.clone(),
            },
            binder: 2,
            from: Dim::Zero,
            to: Dim::One,
            cap: Box::new(standard_cap(&typing)),
            tubes: vec![],
        };
        assert!(infer_term(&context, &point_hcom).is_ok());
        assert_eq!(
            issue_path_realization(&signature, &telescope, 15, &typing, &context, point_hcom,),
            Err(CubicalTypeError::NotPathSchemaRealizer)
        );
    }

    #[test]
    fn fresh_binder_substitution_preserves_typing_and_outer_face_reduces() {
        let owner = Expr::Univ;
        let context = CubicalContext::total(1);
        let cap = point(owner.clone());
        let face = Cofibration::endpoint(Endpoint::zero(0));
        let term = CubicalTerm::Hcom {
            family: CubicalType::Owner {
                owner: owner.clone(),
            },
            binder: 1,
            from: Dim::Zero,
            to: Dim::One,
            cap: Box::new(cap.clone()),
            tubes: vec![Tube {
                face: face.clone(),
                body: Box::new(CubicalTerm::Point {
                    point: PointExpr::PathConstructor {
                        owner,
                        dimension: 1,
                        coordinates: vec![Dim::Var(1)],
                    },
                }),
            }],
        };
        let original_type = infer_term(&context, &term).expect("fresh-binder hcom types");
        let (substituted_type, substituted) = substitute_typed_term(&context, &term, 0, Dim::Zero)
            .expect("ambient substitution preserves typing");
        assert_eq!(substituted_type, original_type.substitute(0, Dim::Zero));
        assert!(matches!(
            substituted,
            CubicalTerm::Hcom {
                binder: 1,
                tubes,
                ..
            } if matches!(
                tubes[0].body.as_ref(),
                CubicalTerm::Point {
                    point: PointExpr::PathConstructor { coordinates, .. }
                } if coordinates == &vec![Dim::Var(1)]
            )
        ));
        let under_face = context.under(&face).expect("outer face context");
        assert_eq!(
            normalize_typed_term(&under_face, &term)
                .expect("outer face reduction")
                .1,
            cap
        );

        let mut stale_binder = term;
        if let CubicalTerm::Hcom { binder, .. } = &mut stale_binder {
            *binder = 0;
        }
        assert_eq!(
            infer_term(&context, &stale_binder),
            Err(CubicalTypeError::CompositionBinderNotFresh {
                expected: 1,
                found: 0,
            })
        );
    }

    #[test]
    fn basis_tokens_are_source_bound_and_exact_through_d4() {
        for dimension in 1..=4 {
            let (signature, telescope, typing) = source(dimension);
            let basis = realize_path_basis(&signature, &telescope, 15, &typing)
                .expect("typed cubical basis");
            assert_eq!(basis.tokens().len(), (1 + dimension * dimension) as usize);
            assert!(basis.tokens().iter().all(|token| {
                replay_path_realization(&signature, &telescope, 15, &typing, token).is_ok()
            }));
        }

        let (signature1, telescope1, typing1) = source(1);
        let token = realize_path_basis(&signature1, &telescope1, 15, &typing1)
            .unwrap()
            .tokens()[0]
            .clone();
        let mut hash_forgery = token.clone();
        hash_forgery.term_hash = "blake3:00".to_owned();
        assert_eq!(
            replay_path_realization(&signature1, &telescope1, 15, &typing1, &hash_forgery),
            Err(CubicalTypeError::SourceMismatch)
        );
        let (signature2, telescope2, typing2) = source(2);
        assert_eq!(
            replay_path_realization(&signature2, &telescope2, 15, &typing2, &token),
            Err(CubicalTypeError::SourceMismatch)
        );
    }

    #[test]
    fn historical_path_projection_separates_fresh_owners_and_replays_equality() {
        let signature = SealedSignature::from_telescopes(
            (1..=7)
                .map(|step| (step, Telescope::reference(step)))
                .collect(),
        );
        let first = Telescope::reference(5);
        let second = Telescope::reference(7);
        let first_typing = elaborate_formed_path(&signature, &first, 4)
            .expect("first historical path")
            .0;
        let second_typing = elaborate_formed_path(&signature, &second, 6)
            .expect("second historical path")
            .0;
        let first_basis =
            realize_path_basis(&signature, &first, 4, &first_typing).expect("first basis");
        let second_basis =
            realize_path_basis(&signature, &second, 6, &second_typing).expect("second basis");
        let first_beta = &first_basis.tokens()[0];
        let second_beta = &second_basis.tokens()[0];

        let reflexive = decide_historical_path_family_equality(&signature, first_beta, first_beta);
        assert!(reflexive.is_equal());
        let separated = decide_historical_path_family_equality(&signature, first_beta, second_beta);
        assert!(separated.is_distinct());
        assert_eq!(
            separated.reason(),
            "distinct fresh declaration support under the equality-free sealed prefix"
        );
        assert_ne!(
            project_historical_path_family(first_beta)
                .expect("first historical projection")
                .family_hash(),
            project_historical_path_family(second_beta)
                .expect("second historical projection")
                .family_hash()
        );

        let first_kan = &first_basis.tokens()[1];
        let different_key =
            decide_historical_path_family_equality(&signature, first_beta, first_kan);
        assert!(different_key.is_distinct());
        assert_eq!(different_key.reason(), "ordered path-family key");
    }
}
