//! Finite-basis/index-set checkers for the combinatorial sides of L1 and L2.
//!
//! # Scope
//!
//! This module explicitly materializes finite index sets, checks their stated
//! partitions, and compares their cardinalities with `1 + d^2` and
//! `2 * kappa + r^2`. It is **not** a Cubical proof that those index sets are
//! equivalent to semantic schema classes modulo definitional equality.
//!
//! In particular, this code does not prove availability, independence,
//! exhaustiveness, union/transport staging, the L1 transpose obstruction,
//! fibration semantics, nontrivial monodromy, absence of definitional
//! inverses, or L2 face/transport disjointness. L2 callers must explicitly
//! assert every trusted semantic premise. Those booleans are assertions at a
//! trust boundary, not proof objects. Thus a successful result checks only the
//! conditional finite combinatorics; it does not by itself prove that the
//! Genesis Sequence stops after Step 15.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use thiserror::Error;

/// This checker materializes every key and is intentionally bounded.
pub const MAX_EXPLICIT_SCHEMA_COUNT: usize = 1_000_000;

/// A key in the single-constructor L1 finite basis.
///
/// Principal and probe indices are zero based. A diagonal `Kan` key denotes a
/// monodromy; an off-diagonal key denotes a directed variation.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "schema", rename_all = "snake_case")]
pub enum L1SchemaKey {
    Beta,
    Kan { principal: u32, probe: u32 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct L1Counts {
    pub beta: usize,
    pub diagonal_monodromies: usize,
    pub off_diagonal_variations: usize,
    pub total: usize,
}

/// An opaque, generated presentation of one L1 finite index set.
///
/// Private fields prevent callers from constructing a value that bypasses the
/// check. Serialization is for audit output only; certificates are not
/// deserializable back into trusted values.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct L1Certificate {
    dimension: u32,
    schemas: BTreeSet<L1SchemaKey>,
    beta: BTreeSet<L1SchemaKey>,
    diagonal_monodromies: BTreeSet<L1SchemaKey>,
    off_diagonal_variations: BTreeSet<L1SchemaKey>,
    counts: L1Counts,
}

impl L1Certificate {
    pub fn dimension(&self) -> u32 {
        self.dimension
    }

    pub fn schemas(&self) -> &BTreeSet<L1SchemaKey> {
        &self.schemas
    }

    pub fn beta(&self) -> &BTreeSet<L1SchemaKey> {
        &self.beta
    }

    pub fn diagonal_monodromies(&self) -> &BTreeSet<L1SchemaKey> {
        &self.diagonal_monodromies
    }

    pub fn off_diagonal_variations(&self) -> &BTreeSet<L1SchemaKey> {
        &self.off_diagonal_variations
    }

    pub fn counts(&self) -> L1Counts {
        self.counts
    }

    /// Recheck positivity, the formula, every key category, and the exact
    /// beta/diagonal/off-diagonal partition.
    pub fn invariants_hold(&self) -> bool {
        if self.dimension == 0 {
            return false;
        }

        let mut expected_beta = BTreeSet::new();
        expected_beta.insert(L1SchemaKey::Beta);
        let mut expected_diagonal = BTreeSet::new();
        let mut expected_off_diagonal = BTreeSet::new();
        for principal in 0..self.dimension {
            for probe in 0..self.dimension {
                let key = L1SchemaKey::Kan { principal, probe };
                if principal == probe {
                    expected_diagonal.insert(key);
                } else {
                    expected_off_diagonal.insert(key);
                }
            }
        }

        let dimension = self.dimension as usize;
        let expected_counts = L1Counts {
            beta: 1,
            diagonal_monodromies: dimension,
            off_diagonal_variations: dimension * (dimension - 1),
            total: 1 + dimension * dimension,
        };
        self.beta == expected_beta
            && self.diagonal_monodromies == expected_diagonal
            && self.off_diagonal_variations == expected_off_diagonal
            && self.counts == expected_counts
            && exact_partition(
                &self.schemas,
                [
                    &self.beta,
                    &self.diagonal_monodromies,
                    &self.off_diagonal_variations,
                ],
            )
    }

    pub fn partition_is_exact(&self) -> bool {
        self.invariants_hold()
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum L1CheckError {
    #[error("the single L1 path-constructor dimension must be positive")]
    ZeroDimension,
    #[error(
        "explicit L1 finite-basis enumeration would contain {count} keys, above the limit of {limit}"
    )]
    EnumerationLimitExceeded { count: u128, limit: usize },
    #[error("the generated L1 finite-basis certificate failed its invariant check")]
    CertificateInvariantViolation,
}

/// Materialize beta plus every ordered `(principal, probe)` key for one
/// positive-dimensional path constructor.
pub fn check_l1_finite_basis(dimension: u32) -> Result<L1Certificate, L1CheckError> {
    if dimension == 0 {
        return Err(L1CheckError::ZeroDimension);
    }
    let dimension_wide = u128::from(dimension);
    let expected_total = 1 + dimension_wide * dimension_wide;
    ensure_enumeration_bound(expected_total)
        .map_err(|(count, limit)| L1CheckError::EnumerationLimitExceeded { count, limit })?;

    let mut schemas = BTreeSet::new();
    let mut beta = BTreeSet::new();
    let mut diagonal_monodromies = BTreeSet::new();
    let mut off_diagonal_variations = BTreeSet::new();

    schemas.insert(L1SchemaKey::Beta);
    beta.insert(L1SchemaKey::Beta);
    for principal in 0..dimension {
        for probe in 0..dimension {
            let key = L1SchemaKey::Kan { principal, probe };
            schemas.insert(key.clone());
            if principal == probe {
                diagonal_monodromies.insert(key);
            } else {
                off_diagonal_variations.insert(key);
            }
        }
    }

    let certificate = L1Certificate {
        dimension,
        counts: L1Counts {
            beta: beta.len(),
            diagonal_monodromies: diagonal_monodromies.len(),
            off_diagonal_variations: off_diagonal_variations.len(),
            total: schemas.len(),
        },
        schemas,
        beta,
        diagonal_monodromies,
        off_diagonal_variations,
    };
    if certificate.counts.total as u128 != expected_total || !certificate.invariants_hold() {
        return Err(L1CheckError::CertificateInvariantViolation);
    }
    Ok(certificate)
}

/// Stable identifier for a sealed L2 reference stratum.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ReferenceId(pub String);

impl ReferenceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<&str> for ReferenceId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for ReferenceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for ReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReferenceEdge {
    pub from: ReferenceId,
    pub to: ReferenceId,
}

impl ReferenceEdge {
    pub fn new(from: impl Into<ReferenceId>, to: impl Into<ReferenceId>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum L2FaceKind {
    Forward,
    Lifting,
}

/// A key in the L2 finite basis.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "schema", rename_all = "snake_case")]
pub enum L2SchemaKey {
    Face {
        clause: usize,
        kind: L2FaceKind,
    },
    Transport {
        source: ReferenceId,
        target: ReferenceId,
    },
}

/// Trusted semantic assertions that the finite index checker cannot prove.
///
/// Every field must be explicitly true. These booleans document a semantic
/// trust boundary; they are not evidence or proof terms.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct L2SemanticAssertions {
    pub charged_fibration: bool,
    pub definitionally_nontrivial: bool,
    pub no_definitional_inverse: bool,
    pub face_transport_disjoint: bool,
    pub basis_available: bool,
    pub basis_independent: bool,
    pub basis_exhaustive_by_staging: bool,
}

impl L2SemanticAssertions {
    /// Assert every premise. This remains a trusted declaration, not a proof.
    pub const fn all_asserted() -> Self {
        Self {
            charged_fibration: true,
            definitionally_nontrivial: true,
            no_definitional_inverse: true,
            face_transport_disjoint: true,
            basis_available: true,
            basis_independent: true,
            basis_exhaustive_by_staging: true,
        }
    }

    pub const fn asserts(self, assertion: L2SemanticAssertion) -> bool {
        match assertion {
            L2SemanticAssertion::ChargedFibration => self.charged_fibration,
            L2SemanticAssertion::DefinitionallyNontrivial => self.definitionally_nontrivial,
            L2SemanticAssertion::NoDefinitionalInverse => self.no_definitional_inverse,
            L2SemanticAssertion::FaceTransportDisjoint => self.face_transport_disjoint,
            L2SemanticAssertion::BasisAvailable => self.basis_available,
            L2SemanticAssertion::BasisIndependent => self.basis_independent,
            L2SemanticAssertion::BasisExhaustiveByStaging => self.basis_exhaustive_by_staging,
        }
    }

    fn all_are_asserted(self) -> bool {
        L2_SEMANTIC_ASSERTIONS
            .into_iter()
            .all(|assertion| self.asserts(assertion))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum L2SemanticAssertion {
    ChargedFibration,
    DefinitionallyNontrivial,
    NoDefinitionalInverse,
    FaceTransportDisjoint,
    BasisAvailable,
    BasisIndependent,
    BasisExhaustiveByStaging,
}

const L2_SEMANTIC_ASSERTIONS: [L2SemanticAssertion; 7] = [
    L2SemanticAssertion::ChargedFibration,
    L2SemanticAssertion::DefinitionallyNontrivial,
    L2SemanticAssertion::NoDefinitionalInverse,
    L2SemanticAssertion::FaceTransportDisjoint,
    L2SemanticAssertion::BasisAvailable,
    L2SemanticAssertion::BasisIndependent,
    L2SemanticAssertion::BasisExhaustiveByStaging,
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct L2Spec {
    pub kappa: usize,
    pub references: Vec<ReferenceId>,
    pub edges: Vec<ReferenceEdge>,
    pub assertions: Option<L2SemanticAssertions>,
}

impl L2Spec {
    pub fn new(
        kappa: usize,
        references: Vec<ReferenceId>,
        edges: Vec<ReferenceEdge>,
        assertions: Option<L2SemanticAssertions>,
    ) -> Self {
        Self {
            kappa,
            references,
            edges,
            assertions,
        }
    }

    pub fn validate(self) -> Result<ValidatedL2Spec, L2CheckError> {
        ValidatedL2Spec::try_from(self)
    }
}

/// An opaque all-pairs reachability and SCC result for a submitted graph.
/// Reachability includes each vertex itself via the empty path.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReferenceGraphCertificate {
    references: BTreeSet<ReferenceId>,
    edges: BTreeSet<ReferenceEdge>,
    reachability: BTreeMap<ReferenceId, BTreeSet<ReferenceId>>,
    strongly_connected_components: Vec<BTreeSet<ReferenceId>>,
}

impl ReferenceGraphCertificate {
    pub fn references(&self) -> &BTreeSet<ReferenceId> {
        &self.references
    }

    pub fn edges(&self) -> &BTreeSet<ReferenceEdge> {
        &self.edges
    }

    pub fn reachability(&self) -> &BTreeMap<ReferenceId, BTreeSet<ReferenceId>> {
        &self.reachability
    }

    pub fn strongly_connected_components(&self) -> &[BTreeSet<ReferenceId>] {
        &self.strongly_connected_components
    }

    pub fn can_reach(&self, source: &ReferenceId, target: &ReferenceId) -> bool {
        self.reachability
            .get(source)
            .is_some_and(|targets| targets.contains(target))
    }

    pub fn is_strongly_connected(&self) -> bool {
        !self.references.is_empty()
            && self.strongly_connected_components.len() == 1
            && self.strongly_connected_components[0] == self.references
            && self.invariants_hold()
    }

    fn invariants_hold(&self) -> bool {
        if self.references.is_empty()
            || self.edges.iter().any(|edge| {
                !self.references.contains(&edge.from) || !self.references.contains(&edge.to)
            })
        {
            return false;
        }
        let (reachability, components) = graph_analysis(&self.references, &self.edges);
        self.reachability == reachability && self.strongly_connected_components == components
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum L2CheckError {
    #[error("L2 requires explicit trusted semantic assertions")]
    MissingSemanticAssertions,
    #[error("L2 trusted semantic assertion {assertion:?} was not asserted")]
    UnassertedSemanticPremise { assertion: L2SemanticAssertion },
    #[error("L2 requires kappa >= 2, got {kappa}")]
    KappaTooSmall { kappa: usize },
    #[error("L2 requires at least one sealed reference")]
    NoReferences,
    #[error("L2 reference {reference} occurs more than once")]
    DuplicateReference { reference: ReferenceId },
    #[error("L2 graph edge names unknown reference {reference}")]
    UnknownEdgeReference { reference: ReferenceId },
    #[error("L2 reference graph is not strongly connected: {from} cannot reach {to}")]
    NotStronglyConnected { from: ReferenceId, to: ReferenceId },
    #[error(
        "explicit L2 finite-basis enumeration would contain {count} keys, above the limit of {limit}"
    )]
    EnumerationLimitExceeded { count: u128, limit: usize },
    #[error("a generated finite-basis certificate failed its invariant check")]
    CertificateInvariantViolation,
}

/// Analyze a submitted directed graph by DFS from every vertex and partition
/// it into SCCs by mutual reachability. Disconnected results are permitted.
pub fn analyze_reference_graph(
    references: &[ReferenceId],
    edges: &[ReferenceEdge],
) -> Result<ReferenceGraphCertificate, L2CheckError> {
    if references.is_empty() {
        return Err(L2CheckError::NoReferences);
    }
    let mut reference_set = BTreeSet::new();
    for reference in references {
        if !reference_set.insert(reference.clone()) {
            return Err(L2CheckError::DuplicateReference {
                reference: reference.clone(),
            });
        }
    }
    let edge_set: BTreeSet<_> = edges.iter().cloned().collect();
    for edge in &edge_set {
        for endpoint in [&edge.from, &edge.to] {
            if !reference_set.contains(endpoint) {
                return Err(L2CheckError::UnknownEdgeReference {
                    reference: endpoint.clone(),
                });
            }
        }
    }
    let (reachability, strongly_connected_components) = graph_analysis(&reference_set, &edge_set);
    let certificate = ReferenceGraphCertificate {
        references: reference_set,
        edges: edge_set,
        reachability,
        strongly_connected_components,
    };
    if !certificate.invariants_hold() {
        return Err(L2CheckError::CertificateInvariantViolation);
    }
    Ok(certificate)
}

/// Analyze a submitted graph and reject it unless every ordered pair is
/// reachable.
pub fn validate_reference_graph(
    references: &[ReferenceId],
    edges: &[ReferenceEdge],
) -> Result<ReferenceGraphCertificate, L2CheckError> {
    let certificate = analyze_reference_graph(references, edges)?;
    if !certificate.is_strongly_connected() {
        for source in certificate.references() {
            for target in certificate.references() {
                if !certificate.can_reach(source, target) {
                    return Err(L2CheckError::NotStronglyConnected {
                        from: source.clone(),
                        to: target.clone(),
                    });
                }
            }
        }
        return Err(L2CheckError::CertificateInvariantViolation);
    }
    Ok(certificate)
}

fn graph_analysis(
    references: &BTreeSet<ReferenceId>,
    edges: &BTreeSet<ReferenceEdge>,
) -> (
    BTreeMap<ReferenceId, BTreeSet<ReferenceId>>,
    Vec<BTreeSet<ReferenceId>>,
) {
    let mut adjacency: BTreeMap<ReferenceId, BTreeSet<ReferenceId>> = references
        .iter()
        .cloned()
        .map(|reference| (reference, BTreeSet::new()))
        .collect();
    for edge in edges {
        if let Some(neighbors) = adjacency.get_mut(&edge.from) {
            neighbors.insert(edge.to.clone());
        }
    }
    let reachability: BTreeMap<_, _> = references
        .iter()
        .cloned()
        .map(|source| {
            let reachable = reachable_from(&source, &adjacency);
            (source, reachable)
        })
        .collect();

    let mut remaining = references.clone();
    let mut components = Vec::new();
    while let Some(seed) = remaining.iter().next().cloned() {
        let component: BTreeSet<_> = remaining
            .iter()
            .filter(|candidate| {
                reachability[&seed].contains(*candidate) && reachability[*candidate].contains(&seed)
            })
            .cloned()
            .collect();
        for member in &component {
            remaining.remove(member);
        }
        components.push(component);
    }
    (reachability, components)
}

fn reachable_from(
    source: &ReferenceId,
    adjacency: &BTreeMap<ReferenceId, BTreeSet<ReferenceId>>,
) -> BTreeSet<ReferenceId> {
    let mut reachable = BTreeSet::new();
    let mut stack = vec![source.clone()];
    while let Some(current) = stack.pop() {
        if !reachable.insert(current.clone()) {
            continue;
        }
        if let Some(neighbors) = adjacency.get(&current) {
            stack.extend(neighbors.iter().cloned());
        }
    }
    reachable
}

/// A spec whose finite preconditions and trusted assertions have been checked.
/// Private fields prevent ordinary Rust construction from bypassing validation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ValidatedL2Spec {
    kappa: usize,
    references: Vec<ReferenceId>,
    assertions: L2SemanticAssertions,
    graph: ReferenceGraphCertificate,
}

impl ValidatedL2Spec {
    pub fn kappa(&self) -> usize {
        self.kappa
    }

    pub fn references(&self) -> &[ReferenceId] {
        &self.references
    }

    pub fn assertions(&self) -> L2SemanticAssertions {
        self.assertions
    }

    pub fn graph(&self) -> &ReferenceGraphCertificate {
        &self.graph
    }
}

impl TryFrom<L2Spec> for ValidatedL2Spec {
    type Error = L2CheckError;

    fn try_from(spec: L2Spec) -> Result<Self, Self::Error> {
        let assertions = spec
            .assertions
            .ok_or(L2CheckError::MissingSemanticAssertions)?;
        for assertion in L2_SEMANTIC_ASSERTIONS {
            if !assertions.asserts(assertion) {
                return Err(L2CheckError::UnassertedSemanticPremise { assertion });
            }
        }
        if spec.kappa < 2 {
            return Err(L2CheckError::KappaTooSmall { kappa: spec.kappa });
        }
        let reference_count = spec.references.len() as u128;
        let expected_total = 2 * spec.kappa as u128 + reference_count * reference_count;
        ensure_enumeration_bound(expected_total)
            .map_err(|(count, limit)| L2CheckError::EnumerationLimitExceeded { count, limit })?;
        let graph = validate_reference_graph(&spec.references, &spec.edges)?;
        let references = graph.references().iter().cloned().collect();
        Ok(Self {
            kappa: spec.kappa,
            references,
            assertions,
            graph,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct L2Counts {
    pub clauses: usize,
    pub references: usize,
    pub forward_faces: usize,
    pub lifting_faces: usize,
    pub faces: usize,
    pub diagonal_monodromies: usize,
    pub off_diagonal_couplings: usize,
    pub transports: usize,
    pub total: usize,
}

/// An opaque, generated presentation of one conditional L2 finite index set.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct L2Certificate {
    kappa: usize,
    references: Vec<ReferenceId>,
    schemas: BTreeSet<L2SchemaKey>,
    faces: BTreeSet<L2SchemaKey>,
    diagonal_monodromies: BTreeSet<L2SchemaKey>,
    off_diagonal_couplings: BTreeSet<L2SchemaKey>,
    counts: L2Counts,
    assertions: L2SemanticAssertions,
    graph: ReferenceGraphCertificate,
}

impl L2Certificate {
    pub fn kappa(&self) -> usize {
        self.kappa
    }

    pub fn references(&self) -> &[ReferenceId] {
        &self.references
    }

    pub fn schemas(&self) -> &BTreeSet<L2SchemaKey> {
        &self.schemas
    }

    pub fn faces(&self) -> &BTreeSet<L2SchemaKey> {
        &self.faces
    }

    pub fn diagonal_monodromies(&self) -> &BTreeSet<L2SchemaKey> {
        &self.diagonal_monodromies
    }

    pub fn off_diagonal_couplings(&self) -> &BTreeSet<L2SchemaKey> {
        &self.off_diagonal_couplings
    }

    pub fn counts(&self) -> L2Counts {
        self.counts
    }

    pub fn assertions(&self) -> L2SemanticAssertions {
        self.assertions
    }

    pub fn graph(&self) -> &ReferenceGraphCertificate {
        &self.graph
    }

    /// Recheck structural preconditions, trusted assertions, formulas, every
    /// key category, graph consistency, and the exact three-way partition.
    pub fn invariants_hold(&self) -> bool {
        if self.kappa < 2
            || self.references.is_empty()
            || self.references.windows(2).any(|pair| pair[0] >= pair[1])
            || !self.assertions.all_are_asserted()
            || !self.graph.is_strongly_connected()
            || self.graph.references().iter().ne(self.references.iter())
        {
            return false;
        }

        let mut expected_faces = BTreeSet::new();
        for clause in 0..self.kappa {
            for kind in [L2FaceKind::Forward, L2FaceKind::Lifting] {
                expected_faces.insert(L2SchemaKey::Face { clause, kind });
            }
        }
        let mut expected_diagonal = BTreeSet::new();
        let mut expected_off_diagonal = BTreeSet::new();
        for source in &self.references {
            for target in &self.references {
                let key = L2SchemaKey::Transport {
                    source: source.clone(),
                    target: target.clone(),
                };
                if source == target {
                    expected_diagonal.insert(key);
                } else {
                    expected_off_diagonal.insert(key);
                }
            }
        }

        let reference_count = self.references.len();
        let expected_counts = L2Counts {
            clauses: self.kappa,
            references: reference_count,
            forward_faces: self.kappa,
            lifting_faces: self.kappa,
            faces: 2 * self.kappa,
            diagonal_monodromies: reference_count,
            off_diagonal_couplings: reference_count * (reference_count - 1),
            transports: reference_count * reference_count,
            total: 2 * self.kappa + reference_count * reference_count,
        };
        self.faces == expected_faces
            && self.diagonal_monodromies == expected_diagonal
            && self.off_diagonal_couplings == expected_off_diagonal
            && self.counts == expected_counts
            && exact_partition(
                &self.schemas,
                [
                    &self.faces,
                    &self.diagonal_monodromies,
                    &self.off_diagonal_couplings,
                ],
            )
    }

    pub fn partition_is_exact(&self) -> bool {
        self.invariants_hold()
    }
}

/// Enumerate the two clause-face keys and the full ordered reference-pair
/// index set from an already validated spec.
pub fn enumerate_l2_finite_basis(spec: &ValidatedL2Spec) -> Result<L2Certificate, L2CheckError> {
    let mut schemas = BTreeSet::new();
    let mut faces = BTreeSet::new();
    let mut diagonal_monodromies = BTreeSet::new();
    let mut off_diagonal_couplings = BTreeSet::new();

    for clause in 0..spec.kappa {
        for kind in [L2FaceKind::Forward, L2FaceKind::Lifting] {
            let key = L2SchemaKey::Face { clause, kind };
            schemas.insert(key.clone());
            faces.insert(key);
        }
    }
    for source in &spec.references {
        for target in &spec.references {
            let key = L2SchemaKey::Transport {
                source: source.clone(),
                target: target.clone(),
            };
            schemas.insert(key.clone());
            if source == target {
                diagonal_monodromies.insert(key);
            } else {
                off_diagonal_couplings.insert(key);
            }
        }
    }

    let reference_count = spec.references.len();
    let certificate = L2Certificate {
        kappa: spec.kappa,
        references: spec.references.clone(),
        counts: L2Counts {
            clauses: spec.kappa,
            references: reference_count,
            forward_faces: spec.kappa,
            lifting_faces: spec.kappa,
            faces: faces.len(),
            diagonal_monodromies: diagonal_monodromies.len(),
            off_diagonal_couplings: off_diagonal_couplings.len(),
            transports: diagonal_monodromies.len() + off_diagonal_couplings.len(),
            total: schemas.len(),
        },
        schemas,
        faces,
        diagonal_monodromies,
        off_diagonal_couplings,
        assertions: spec.assertions,
        graph: spec.graph.clone(),
    };
    if !certificate.invariants_hold() {
        return Err(L2CheckError::CertificateInvariantViolation);
    }
    Ok(certificate)
}

/// Validate every finite precondition and trusted assertion, then check the L2
/// finite basis.
pub fn check_l2_finite_basis(spec: &L2Spec) -> Result<L2Certificate, L2CheckError> {
    let validated = spec.clone().validate()?;
    enumerate_l2_finite_basis(&validated)
}

fn ensure_enumeration_bound(count: u128) -> Result<(), (u128, usize)> {
    if count > MAX_EXPLICIT_SCHEMA_COUNT as u128 {
        Err((count, MAX_EXPLICIT_SCHEMA_COUNT))
    } else {
        Ok(())
    }
}

fn exact_partition<T, const N: usize>(universe: &BTreeSet<T>, parts: [&BTreeSet<T>; N]) -> bool
where
    T: Clone + Ord,
{
    let expected_len: usize = parts.iter().map(|part| part.len()).sum();
    let union: BTreeSet<_> = parts
        .into_iter()
        .flat_map(|part| part.iter().cloned())
        .collect();
    expected_len == union.len() && &union == universe
}

#[cfg(test)]
mod tests {
    use super::{
        L1CheckError, L1SchemaKey, L2CheckError, L2FaceKind, L2SchemaKey, L2SemanticAssertion,
        L2SemanticAssertions, L2Spec, ReferenceEdge, ReferenceId, analyze_reference_graph,
        check_l1_finite_basis, check_l2_finite_basis, enumerate_l2_finite_basis,
    };
    use crate::nu::{compute_nu_c, compute_nu_h};
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::telescope::{Telescope, TelescopeClass};
    use std::collections::HashSet;

    fn reference(value: &str) -> ReferenceId {
        ReferenceId::from(value)
    }

    fn cycle_edges(references: &[ReferenceId]) -> Vec<ReferenceEdge> {
        (0..references.len())
            .map(|index| ReferenceEdge {
                from: references[index].clone(),
                to: references[(index + 1) % references.len()].clone(),
            })
            .collect()
    }

    fn asserted_spec(kappa: usize, names: &[&str]) -> L2Spec {
        let references: Vec<_> = names.iter().copied().map(reference).collect();
        let edges = cycle_edges(&references);
        L2Spec::new(
            kappa,
            references,
            edges,
            Some(L2SemanticAssertions::all_asserted()),
        )
    }

    #[test]
    fn l1_circle_index_anchor_is_beta_plus_one_diagonal() {
        let certificate = check_l1_finite_basis(1).expect("dimension one is positive");
        assert_eq!(certificate.dimension(), 1);
        assert_eq!(certificate.counts().beta, 1);
        assert_eq!(certificate.counts().diagonal_monodromies, 1);
        assert_eq!(certificate.counts().off_diagonal_variations, 0);
        assert_eq!(certificate.counts().total, 2);
        assert!(certificate.invariants_hold());
        assert!(certificate.beta().contains(&L1SchemaKey::Beta));
        assert!(
            certificate
                .diagonal_monodromies()
                .contains(&L1SchemaKey::Kan {
                    principal: 0,
                    probe: 0,
                })
        );
    }

    #[test]
    fn l1_rejects_zero_dimension() {
        assert_eq!(check_l1_finite_basis(0), Err(L1CheckError::ZeroDimension));
    }

    #[test]
    fn l1_bounded_single_dimension_range_matches_one_plus_square() {
        for dimension in 1_u32..=20 {
            let certificate = check_l1_finite_basis(dimension).expect("small positive dimension");
            assert_eq!(
                certificate.counts().total,
                1 + (dimension * dimension) as usize
            );
            assert_eq!(
                certificate.counts().diagonal_monodromies,
                dimension as usize
            );
            assert!(certificate.partition_is_exact());
        }
    }

    #[test]
    fn l1_single_constructor_index_count_matches_evaluator() {
        let telescope = Telescope::new(vec![ClauseRec::new(
            ClauseRole::PathAttach,
            Expr::PathCon(3),
        )]);
        let dimension = telescope.path_dimensions()[0];
        let certificate = check_l1_finite_basis(dimension).expect("positive path dimension");
        assert_eq!(certificate.counts().total as u32, compute_nu_h(&telescope));
        assert_eq!(certificate.counts().total, 10);
    }

    #[test]
    fn schema_keys_are_hashable_and_json_round_trip() {
        let l1_key = L1SchemaKey::Kan {
            principal: 1,
            probe: 0,
        };
        let l2_key = L2SchemaKey::Face {
            clause: 3,
            kind: L2FaceKind::Lifting,
        };
        let mut l1_keys = HashSet::new();
        assert!(l1_keys.insert(l1_key.clone()));
        let mut l2_keys = HashSet::new();
        assert!(l2_keys.insert(l2_key.clone()));
        let l1_json = serde_json::to_string(&l1_key).expect("serialize L1 key");
        let l2_json = serde_json::to_string(&l2_key).expect("serialize L2 key");
        assert_eq!(
            serde_json::from_str::<L1SchemaKey>(&l1_json).expect("deserialize L1 key"),
            l1_key
        );
        assert_eq!(
            serde_json::from_str::<L2SchemaKey>(&l2_json).expect("deserialize L2 key"),
            l2_key
        );
    }

    #[test]
    fn graph_analysis_reports_reachability_and_sccs() {
        let references = vec![reference("A"), reference("B"), reference("C")];
        let edges = vec![
            ReferenceEdge::new("A", "B"),
            ReferenceEdge::new("B", "A"),
            ReferenceEdge::new("B", "C"),
        ];
        let graph = analyze_reference_graph(&references, &edges).expect("well-formed graph");
        assert!(graph.can_reach(&reference("A"), &reference("C")));
        assert!(!graph.can_reach(&reference("C"), &reference("A")));
        assert_eq!(graph.strongly_connected_components().len(), 2);
        assert!(!graph.is_strongly_connected());
    }

    #[test]
    fn l2_rejects_absent_or_false_trusted_assertions() {
        let mut missing = asserted_spec(2, &["B", "F"]);
        missing.assertions = None;
        assert_eq!(
            check_l2_finite_basis(&missing),
            Err(L2CheckError::MissingSemanticAssertions)
        );

        for assertion in [
            L2SemanticAssertion::ChargedFibration,
            L2SemanticAssertion::DefinitionallyNontrivial,
            L2SemanticAssertion::NoDefinitionalInverse,
            L2SemanticAssertion::FaceTransportDisjoint,
            L2SemanticAssertion::BasisAvailable,
            L2SemanticAssertion::BasisIndependent,
            L2SemanticAssertion::BasisExhaustiveByStaging,
        ] {
            let mut assertions = L2SemanticAssertions::all_asserted();
            match assertion {
                L2SemanticAssertion::ChargedFibration => assertions.charged_fibration = false,
                L2SemanticAssertion::DefinitionallyNontrivial => {
                    assertions.definitionally_nontrivial = false;
                }
                L2SemanticAssertion::NoDefinitionalInverse => {
                    assertions.no_definitional_inverse = false;
                }
                L2SemanticAssertion::FaceTransportDisjoint => {
                    assertions.face_transport_disjoint = false;
                }
                L2SemanticAssertion::BasisAvailable => assertions.basis_available = false,
                L2SemanticAssertion::BasisIndependent => assertions.basis_independent = false,
                L2SemanticAssertion::BasisExhaustiveByStaging => {
                    assertions.basis_exhaustive_by_staging = false;
                }
            }
            let mut spec = asserted_spec(2, &["B", "F"]);
            spec.assertions = Some(assertions);
            assert_eq!(
                check_l2_finite_basis(&spec),
                Err(L2CheckError::UnassertedSemanticPremise { assertion })
            );
        }
    }

    #[test]
    fn l2_rejects_invalid_finite_structure() {
        let too_small = asserted_spec(1, &["B", "F"]);
        assert_eq!(
            check_l2_finite_basis(&too_small),
            Err(L2CheckError::KappaTooSmall { kappa: 1 })
        );
        let duplicate = L2Spec::new(
            2,
            vec![reference("B"), reference("B")],
            vec![],
            Some(L2SemanticAssertions::all_asserted()),
        );
        assert_eq!(
            check_l2_finite_basis(&duplicate),
            Err(L2CheckError::DuplicateReference {
                reference: reference("B")
            })
        );
        let disconnected = L2Spec::new(
            2,
            vec![reference("B"), reference("F")],
            vec![ReferenceEdge::new("B", "F")],
            Some(L2SemanticAssertions::all_asserted()),
        );
        assert_eq!(
            check_l2_finite_basis(&disconnected),
            Err(L2CheckError::NotStronglyConnected {
                from: reference("F"),
                to: reference("B")
            })
        );
        let unknown = L2Spec::new(
            2,
            vec![reference("B"), reference("F")],
            vec![ReferenceEdge::new("B", "X")],
            Some(L2SemanticAssertions::all_asserted()),
        );
        assert_eq!(
            check_l2_finite_basis(&unknown),
            Err(L2CheckError::UnknownEdgeReference {
                reference: reference("X")
            })
        );
    }

    #[test]
    fn mobius_conditional_index_anchor_has_four_faces_and_four_transports() {
        let certificate = check_l2_finite_basis(&asserted_spec(2, &["B", "F"]))
            .expect("submitted Mobius-style index data satisfy finite checks");
        assert_eq!(certificate.counts().forward_faces, 2);
        assert_eq!(certificate.counts().lifting_faces, 2);
        assert_eq!(certificate.counts().faces, 4);
        assert_eq!(certificate.counts().diagonal_monodromies, 2);
        assert_eq!(certificate.counts().off_diagonal_couplings, 2);
        assert_eq!(certificate.counts().transports, 4);
        assert_eq!(certificate.counts().total, 8);
        assert!(certificate.invariants_hold());
    }

    #[test]
    fn hopf_conditional_formula_parity_matches_map_evaluator() {
        let hopf = Telescope::reference(9);
        let names: Vec<_> = hopf.lib_refs().iter().map(u32::to_string).collect();
        let name_refs: Vec<_> = names.iter().map(String::as_str).collect();

        // Only kappa and the reference cardinality come from the AST. The
        // directed cycle is deliberately submitted/fabricated test data; it is
        // not a graph derived from the Hopf AST. Likewise all semantic premises
        // below are trusted assertions. This test checks formula parity only.
        let certificate = check_l2_finite_basis(&asserted_spec(hopf.kappa(), &name_refs))
            .expect("submitted conditional finite-basis inputs validate");
        assert_eq!(certificate.counts().clauses, 4);
        assert_eq!(certificate.counts().references, 3);
        assert_eq!(certificate.counts().total, 17);
        let empty_library = Vec::new();
        assert_eq!(
            certificate.counts().total as u32,
            compute_nu_c(TelescopeClass::Map, &hopf, &empty_library, &[])
        );
    }

    #[test]
    fn l2_bounded_generic_index_ranges_match_two_kappa_plus_r_squared() {
        for kappa in 2..=7 {
            for reference_count in 1..=6 {
                let names: Vec<_> = (0..reference_count)
                    .map(|index| format!("R{index}"))
                    .collect();
                let name_refs: Vec<_> = names.iter().map(String::as_str).collect();
                let validated = asserted_spec(kappa, &name_refs)
                    .validate()
                    .expect("bounded submitted spec");
                let certificate =
                    enumerate_l2_finite_basis(&validated).expect("bounded explicit enumeration");
                assert_eq!(
                    certificate.counts().total,
                    2 * kappa + reference_count * reference_count
                );
                assert_eq!(certificate.counts().diagonal_monodromies, reference_count);
                assert_eq!(
                    certificate.counts().off_diagonal_couplings,
                    reference_count * (reference_count - 1)
                );
                assert!(certificate.graph().is_strongly_connected());
                assert!(certificate.partition_is_exact());
            }
        }
    }
}
