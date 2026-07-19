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
use crate::tdc1::{FormedPathTyping, PathSchemaKey, elaborate_formed_path};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const CUBICAL_FRAGMENT_VERSION: &str = "tdc1-cubical-fragment-v3";
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
}

impl PointExpr {
    fn owner(&self) -> &Expr {
        match self {
            Self::Base { owner }
            | Self::Neutral { owner, .. }
            | Self::PathConstructor { owner, .. } => owner,
        }
    }

    fn substitute(&self, variable: u16, replacement: Dim) -> Self {
        match self {
            Self::Base { .. } | Self::Neutral { .. } => self.clone(),
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
        }
    }

    fn depends_on(&self, variable: u16) -> bool {
        matches!(
            self,
            Self::PathConstructor { coordinates, .. }
                if coordinates.contains(&Dim::Var(variable))
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Motive {
    pub owner: Expr,
    pub name: String,
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
    /// Motive-typed `PathCon` elimination.  On a constructor scrutinee this
    /// normalizes to `method`.
    PathElim {
        motive: Motive,
        base: Box<CubicalTerm>,
        method: Box<CubicalTerm>,
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

fn check_point_scope(context: &CubicalContext, point: &PointExpr) -> Result<(), CubicalTypeError> {
    if let PointExpr::PathConstructor {
        dimension,
        coordinates,
        ..
    } = point
    {
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
    Ok(())
}

fn check_type_scope(context: &CubicalContext, ty: &CubicalType) -> Result<(), CubicalTypeError> {
    match ty {
        CubicalType::Owner { .. } => Ok(()),
        CubicalType::MotiveFiber { motive, point } => {
            check_point_scope(context, point)?;
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
            check_type_scope(&context.extend_interval(), body)
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
        PointExpr::Base { .. } | PointExpr::Neutral { .. } => point.clone(),
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

fn lower_term_after_removal(term: &CubicalTerm, removed: u16) -> CubicalTerm {
    match term {
        CubicalTerm::Point { point } => CubicalTerm::Point {
            point: lower_point_after_removal(point, removed),
        },
        CubicalTerm::MotiveBase { .. } => term.clone(),
        CubicalTerm::PathMethod { motive, path, base } => CubicalTerm::PathMethod {
            motive: motive.clone(),
            path: lower_point_after_removal(path, removed),
            base: Box::new(lower_term_after_removal(base, removed)),
        },
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
        CubicalTerm::PathMethod { motive, path, base } => {
            let base = substitute_term(base, variable, replacement);
            match path.substitute(variable, replacement) {
                PointExpr::Base { .. } => base,
                path @ (PointExpr::Neutral { .. } | PointExpr::PathConstructor { .. }) => {
                    CubicalTerm::PathMethod {
                        motive: motive.clone(),
                        path,
                        base: Box::new(base),
                    }
                }
            }
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

pub fn infer_term(
    context: &CubicalContext,
    term: &CubicalTerm,
) -> Result<CubicalType, CubicalTypeError> {
    match term {
        CubicalTerm::Point { point } => {
            check_point_scope(context, point)?;
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
        CubicalTerm::PathMethod { motive, path, base } => {
            let PointExpr::PathConstructor { owner, .. } = path else {
                return Err(CubicalTypeError::MethodNotAtConstructor);
            };
            if owner != &motive.owner {
                return Err(CubicalTypeError::MotiveOwnerMismatch);
            }
            check_point_scope(context, path)?;
            let expected_base = CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: PointExpr::Base {
                    owner: motive.owner.clone(),
                },
            };
            expect_type(infer_term(context, base)?, &expected_base)?;
            Ok(CubicalType::MotiveFiber {
                motive: motive.clone(),
                point: path.clone(),
            })
        }
        CubicalTerm::PathElim {
            motive,
            base,
            method,
            scrutinee,
        } => {
            let scrutinee_ty = infer_term(context, scrutinee)?;
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
            expect_type(infer_term(context, base)?, &expected_base)?;
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
            expect_type(infer_term(context, method)?, &expected_method)?;
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
        CubicalTerm::DimLambda { binder, body } => {
            if *binder != context.interval_count {
                return Err(CubicalTypeError::IntervalOutOfScope {
                    variable: *binder,
                    interval_count: context.interval_count,
                });
            }
            let body_ty = infer_term(&context.extend_interval(), body)?;
            Ok(CubicalType::DimensionFunction {
                binder: *binder,
                body: Box::new(body_ty),
            })
        }
        CubicalTerm::DimApp { function, argument } => {
            check_dim_scope(context, *argument)?;
            let CubicalType::DimensionFunction { binder, body } = infer_term(context, function)?
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
            check_type_scope(&context.extend_interval(), family)?;
            check_dim_scope(context, *from)?;
            check_dim_scope(context, *to)?;
            let source = instantiate_type(family, *binder, *from);
            let target = instantiate_type(family, *binder, *to);
            expect_type(infer_term(context, term)?, &source)?;
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
            check_type_scope(&extended, family)?;
            if family.depends_on(*binder) {
                return Err(CubicalTypeError::DependentHcomUnsupported);
            }
            check_dim_scope(context, *from)?;
            check_dim_scope(context, *to)?;
            let source = instantiate_type(family, *binder, *from);
            let target = instantiate_type(family, *binder, *to);
            expect_type(infer_term(context, cap)?, &source)?;
            for tube in tubes {
                check_face_scope(context.interval_count, &tube.face)?;
                let body_ty = infer_term(&extended, &tube.body)?;
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
        CubicalTerm::Point { .. } | CubicalTerm::MotiveBase { .. } => term.clone(),
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
}
