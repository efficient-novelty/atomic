//! Verifier-derived semantic authority for public clauses and seeds.
//!
//! A verified public inventory proves syntax, origin, ownership, and
//! dependency coverage, but the V1 carrier still accepts caller-provided
//! clause/support/anchor metadata.  This module begins the V2 authority chain
//! by deriving clause identities and their public support from the inventory
//! itself.  Demand realization remains a separate capability: a port key is
//! deliberately absent from every proof minted here.

use crate::fragment::lambda_unit_judgment_syntax_violation;
use crate::inventory::{
    DemandContractIdV1, DemandPortKeyV1, PublicSubjectV1, VerifiedPublicAuditInventoryV1,
    VerifiedPublicDeclarationV1, VerifiedPublicEquationV1,
};
use crate::inventory_compatibility::VerifiedPublicInventoryCompatibilityV2;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2,
    VerifiedSemanticAuditManifestV2,
};
use crate::model::{
    ClauseIdV1, DemandOrbitIdV1, DemandOutputIdV1, EquationIdV1, EventIdV1, GenericJudgmentV1,
    LocalRoleV1, PublicSupportV1, SeedIdV1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, DependentContext, Digest, GlobalId, Kernel,
    KernelError, OpenJudgment, ResourceKind, Term,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

/// Exact kind and inventory identity of one public clause.
///
/// This enum is descriptive data.  It carries no authority by itself;
/// authority belongs to [`VerifiedPublicClauseIdentityV1`], whose fields are
/// private and which has no `Deserialize` implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicClauseSubjectV1 {
    Declaration {
        declaration: GlobalId,
    },
    Equation {
        equation: EquationIdV1,
        owner_head: GlobalId,
    },
}

impl CanonicalEncode for PublicClauseSubjectV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Declaration { declaration } => {
                encoder.tag(0);
                declaration.encode_canonical(encoder);
            }
            Self::Equation {
                equation,
                owner_head,
            } => {
                encoder.tag(1);
                equation.encode_canonical(encoder);
                owner_head.encode_canonical(encoder);
            }
        }
    }
}

/// Canonical identity and support of one inventory-covered public clause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPublicClauseIdentityV1 {
    id: ClauseIdV1,
    subject: PublicClauseSubjectV1,
    source_identity: Digest,
    origin_event: EventIdV1,
    public_group: GlobalId,
    normalized_subject_digest: Digest,
    dependency_support: Arc<[GlobalId]>,
    public_support: PublicSupportV1,
    digest: Digest,
}

impl VerifiedPublicClauseIdentityV1 {
    pub fn id(&self) -> &ClauseIdV1 {
        &self.id
    }

    pub fn subject(&self) -> &PublicClauseSubjectV1 {
        &self.subject
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn origin_event(&self) -> &EventIdV1 {
        &self.origin_event
    }

    pub fn public_group(&self) -> &GlobalId {
        &self.public_group
    }

    pub fn normalized_subject_digest(&self) -> &Digest {
        &self.normalized_subject_digest
    }

    pub fn dependency_support(&self) -> &[GlobalId] {
        &self.dependency_support
    }

    /// Support reconstructed from the verified dependency DAG and exact
    /// declaration origins.  Demand outputs are intentionally empty here:
    /// they require a separate realization theorem.
    pub fn public_support(&self) -> &PublicSupportV1 {
        &self.public_support
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedPublicClauseIdentityV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.origin_event.encode_canonical(encoder);
        self.public_group.encode_canonical(encoder);
        self.normalized_subject_digest.encode_canonical(encoder);
        encoder.sequence(&self.dependency_support);
        self.public_support.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Complete declaration/equation clause census for one verified inventory.
///
/// The capability is verifier-minted, has private fields, and has no
/// `Deserialize` implementation.  Clause identity is prior to and independent
/// of all kernel-cost classifications.
#[derive(Clone, Debug)]
pub struct VerifiedPublicClauseCensusV1 {
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    clauses: Arc<[VerifiedPublicClauseIdentityV1]>,
    declaration_to_clause: BTreeMap<GlobalId, ClauseIdV1>,
    equation_to_clause: BTreeMap<EquationIdV1, ClauseIdV1>,
    coverage_digest: Digest,
    digest: Digest,
}

impl VerifiedPublicClauseCensusV1 {
    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn inventory_coverage_digest(&self) -> &Digest {
        &self.inventory_coverage_digest
    }

    pub fn clauses(&self) -> &[VerifiedPublicClauseIdentityV1] {
        &self.clauses
    }

    pub fn clause_for_declaration(&self, declaration: &GlobalId) -> Option<&ClauseIdV1> {
        self.declaration_to_clause.get(declaration)
    }

    pub fn clause_for_equation(&self, equation: &EquationIdV1) -> Option<&ClauseIdV1> {
        self.equation_to_clause.get(equation)
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Derive every public clause identity from the exact verified inventory.
///
/// No clause ID, support set, role, demand anchor, or completeness flag is
/// supplied by the caller.
pub fn verify_public_clause_census_v1(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> AuditDecision<VerifiedPublicClauseCensusV1> {
    let mut clauses =
        Vec::with_capacity(inventory.declarations().len() + inventory.equations().len());
    let mut declaration_to_clause = BTreeMap::new();
    let mut equation_to_clause = BTreeMap::new();
    let mut clause_ids = BTreeSet::new();

    for declaration in inventory.declarations() {
        let dependency_support = dependencies_for(
            inventory,
            &PublicSubjectV1::Declaration {
                declaration: declaration.declaration().clone(),
            },
        );
        let public_support = match derive_public_support(
            inventory,
            declaration.origin(),
            declaration.declaration(),
            &dependency_support,
        ) {
            Ok(support) => support,
            Err(reason) => return AuditDecision::Unknown(reason),
        };
        let clause = declaration_clause(inventory, declaration, dependency_support, public_support);
        if !clause_ids.insert(clause.id.clone())
            || declaration_to_clause
                .insert(declaration.declaration().clone(), clause.id.clone())
                .is_some()
        {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        clauses.push(clause);
    }

    for equation in inventory.equations() {
        let Some(owner) = inventory.public_declaration(equation.owner_head()) else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport);
        };
        let dependency_support = dependencies_for(
            inventory,
            &PublicSubjectV1::Equation {
                equation: equation.equation().clone(),
            },
        );
        let public_support = match derive_public_support(
            inventory,
            equation.origin(),
            equation.owner_head(),
            &dependency_support,
        ) {
            Ok(support) => support,
            Err(reason) => return AuditDecision::Unknown(reason),
        };
        let clause = equation_clause(
            inventory,
            equation,
            owner.group(),
            dependency_support,
            public_support,
        );
        if !clause_ids.insert(clause.id.clone())
            || equation_to_clause
                .insert(equation.equation().clone(), clause.id.clone())
                .is_some()
        {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        clauses.push(clause);
    }

    clauses.sort_by(|left, right| left.id.cmp(&right.id));
    if declaration_to_clause.len() != inventory.declarations().len()
        || equation_to_clause.len() != inventory.equations().len()
        || clauses.len()
            != inventory
                .declarations()
                .len()
                .saturating_add(inventory.equations().len())
    {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }

    let declaration_entries = declaration_to_clause
        .iter()
        .map(|(subject, clause)| DeclarationClauseMapEntry { subject, clause })
        .collect::<Vec<_>>();
    let equation_entries = equation_to_clause
        .iter()
        .map(|(subject, clause)| EquationClauseMapEntry { subject, clause })
        .collect::<Vec<_>>();
    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/public-clause-census-coverage/v1",
        &PublicClauseCoverageMaterial {
            inventory: inventory.digest(),
            inventory_coverage: inventory.coverage().digest(),
            clauses: &clauses,
            declaration_entries: &declaration_entries,
            equation_entries: &equation_entries,
        },
    );
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-public-clause-census/v1",
        &PublicClauseCensusDigestMaterial {
            inventory: inventory.digest(),
            inventory_coverage: inventory.coverage().digest(),
            coverage: &coverage_digest,
            clauses: &clauses,
        },
    );

    AuditDecision::Proven(VerifiedPublicClauseCensusV1 {
        inventory_digest: inventory.digest().clone(),
        inventory_coverage_digest: inventory.coverage().digest().clone(),
        clauses: Arc::from(clauses),
        declaration_to_clause,
        equation_to_clause,
        coverage_digest,
        digest,
    })
}

/// Independently diagnosed obligations that prevent a nonempty demand-anchor
/// census from being minted.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DemandAnchorBlockerV1 {
    /// The exact demand judgments have not yet been quotiented by verified
    /// typed reindexing into complete orbit classes.
    MissingDemandOrbitAuthority,
    /// Port association has not been upgraded to a kernel-replayed typed
    /// specialization from a semantic family.
    MissingTypedRealizationAuthority,
}

/// One verifier-derived demand orbit.
///
/// The current prototype mints no nonempty value of this type until the
/// typed-reindexing quotient theorem exists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedDemandOrbitV1 {
    id: DemandOrbitIdV1,
    representative_contract: DemandContractIdV1,
    representative_judgment_digest: Digest,
    membership_digest: Digest,
    digest: Digest,
}

impl VerifiedDemandOrbitV1 {
    pub fn id(&self) -> &DemandOrbitIdV1 {
        &self.id
    }

    pub fn representative_contract(&self) -> &DemandContractIdV1 {
        &self.representative_contract
    }

    pub fn representative_judgment_digest(&self) -> &Digest {
        &self.representative_judgment_digest
    }

    pub fn membership_digest(&self) -> &Digest {
        &self.membership_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedDemandOrbitV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.representative_contract.encode_canonical(encoder);
        self.representative_judgment_digest
            .encode_canonical(encoder);
        self.membership_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// One exact predecessor demand port assigned to a verified orbit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedDemandPortV1 {
    contract: DemandContractIdV1,
    port: DemandPortKeyV1,
    origin_event: EventIdV1,
    orbit: DemandOrbitIdV1,
    normalized_requirement_digest: Digest,
    digest: Digest,
}

impl VerifiedDemandPortV1 {
    pub fn contract(&self) -> &DemandContractIdV1 {
        &self.contract
    }

    pub fn port(&self) -> &DemandPortKeyV1 {
        &self.port
    }

    pub fn origin_event(&self) -> &EventIdV1 {
        &self.origin_event
    }

    pub fn orbit(&self) -> &DemandOrbitIdV1 {
        &self.orbit
    }

    pub fn normalized_requirement_digest(&self) -> &Digest {
        &self.normalized_requirement_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedDemandPortV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.contract.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
        self.origin_event.encode_canonical(encoder);
        self.orbit.encode_canonical(encoder);
        self.normalized_requirement_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Kernel-replayed typed specialization from one semantic family to one exact
/// demand output.
///
/// A matching `DemandPortKeyV1` cannot construct this type.  Its fields are
/// private, and the current prototype deliberately has no nonempty minting
/// path until the family/reindexing theorem stack is present.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedDemandRealizationV1 {
    equation: EquationIdV1,
    port: DemandPortKeyV1,
    orbit: DemandOrbitIdV1,
    family_judgment_digest: Digest,
    typed_specialization_digest: Digest,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedDemandRealizationV1 {
    pub fn equation(&self) -> &EquationIdV1 {
        &self.equation
    }

    pub fn port(&self) -> &DemandPortKeyV1 {
        &self.port
    }

    pub fn orbit(&self) -> &DemandOrbitIdV1 {
        &self.orbit
    }

    pub fn family_judgment_digest(&self) -> &Digest {
        &self.family_judgment_digest
    }

    pub fn typed_specialization_digest(&self) -> &Digest {
        &self.typed_specialization_digest
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedDemandRealizationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
        self.orbit.encode_canonical(encoder);
        self.family_judgment_digest.encode_canonical(encoder);
        self.typed_specialization_digest.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Complete demand-anchor authority for one inventory.
///
/// The empty base is positively verifiable.  A nonempty inventory cannot mint
/// this capability until both orbit membership and typed family realization
/// are independently certified.
#[derive(Clone, Debug)]
pub struct VerifiedDemandAnchorCensusV1 {
    inventory_digest: Digest,
    predecessor_history_digest: Digest,
    predecessor_demand_digest: Digest,
    orbit_classes: Arc<[VerifiedDemandOrbitV1]>,
    ports: Arc<[VerifiedDemandPortV1]>,
    realization_certificates: Arc<[VerifiedDemandRealizationV1]>,
    digest: Digest,
}

impl VerifiedDemandAnchorCensusV1 {
    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn predecessor_history_digest(&self) -> &Digest {
        &self.predecessor_history_digest
    }

    pub fn predecessor_demand_digest(&self) -> &Digest {
        &self.predecessor_demand_digest
    }

    pub fn orbit_classes(&self) -> &[VerifiedDemandOrbitV1] {
        &self.orbit_classes
    }

    pub fn ports(&self) -> &[VerifiedDemandPortV1] {
        &self.ports
    }

    pub fn realization_certificates(&self) -> &[VerifiedDemandRealizationV1] {
        &self.realization_certificates
    }

    pub fn is_empty(&self) -> bool {
        self.orbit_classes.is_empty()
            && self.ports.is_empty()
            && self.realization_certificates.is_empty()
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Report the exact missing theorem classes without attempting to choose a
/// substitute for either one.
pub fn demand_anchor_blockers_v1(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> BTreeSet<DemandAnchorBlockerV1> {
    let mut blockers = BTreeSet::new();
    if !inventory.predecessor_demand_contracts().is_empty() {
        blockers.insert(DemandAnchorBlockerV1::MissingDemandOrbitAuthority);
    }
    if inventory
        .equations()
        .iter()
        .any(|equation| equation.demand_port().is_some())
    {
        blockers.insert(DemandAnchorBlockerV1::MissingTypedRealizationAuthority);
    }
    blockers
}

/// Verify the complete empty demand-anchor base, or fail closed at the exact
/// nonempty theorem boundary.
///
/// This function never promotes a port-key association to an anchor.  The
/// positive empty result is useful for equation/demand-free predecessors and
/// as the base case of the eventual inductive historical construction.
pub fn verify_demand_anchor_census_v1(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> AuditDecision<VerifiedDemandAnchorCensusV1> {
    let blockers = demand_anchor_blockers_v1(inventory);
    if blockers.contains(&DemandAnchorBlockerV1::MissingDemandOrbitAuthority) {
        return AuditDecision::Unknown(AuditUnknownReason::MissingDemandOrbitAuthority);
    }
    if blockers.contains(&DemandAnchorBlockerV1::MissingTypedRealizationAuthority) {
        return AuditDecision::Unknown(AuditUnknownReason::MissingDemandRealizationAuthority);
    }

    let demand_entries = inventory
        .predecessor_demand_contracts()
        .iter()
        .map(|demand| DemandContractDigestEntry {
            contract: demand.contract(),
            origin: demand.origin(),
            port: demand.port(),
            source_identity: demand.source_identity(),
            source_requirement: demand.source_requirement(),
            normalized_requirement: demand.normalized_requirement(),
        })
        .collect::<Vec<_>>();
    let equation_port_entries = inventory
        .equations()
        .iter()
        .filter_map(|equation| {
            equation.demand_port().map(|port| EquationPortDigestEntry {
                equation: equation.equation(),
                port,
            })
        })
        .collect::<Vec<_>>();
    // The blocker checks above make these positive coverage assertions, not
    // assumptions hidden in a serialized `complete` bit.
    if !demand_entries.is_empty() || !equation_port_entries.is_empty() {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }

    let predecessor_demand_digest = Digest::of_canonical(
        "pen-semantic-audit/predecessor-demand-contract-census/v1",
        &DemandContractCensusMaterial {
            inventory: inventory.digest(),
            inventory_coverage: inventory.coverage().digest(),
            predecessor_history: inventory.predecessor_history_digest(),
            entries: &demand_entries,
        },
    );
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-empty-demand-anchor-census/v1",
        &EmptyDemandAnchorCensusMaterial {
            inventory: inventory.digest(),
            predecessor_history: inventory.predecessor_history_digest(),
            predecessor_demand: &predecessor_demand_digest,
            equation_ports: &equation_port_entries,
        },
    );

    AuditDecision::Proven(VerifiedDemandAnchorCensusV1 {
        inventory_digest: inventory.digest().clone(),
        predecessor_history_digest: inventory.predecessor_history_digest().clone(),
        predecessor_demand_digest,
        orbit_classes: Arc::from(Vec::new()),
        ports: Arc::from(Vec::new()),
        realization_certificates: Arc::from(Vec::new()),
        digest,
    })
}

/// Exact inventory subject of one verifier-derived V2 semantic seed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicSemanticSeedSubjectV2 {
    Declaration {
        declaration: GlobalId,
        public_group: GlobalId,
    },
    Equation {
        equation: EquationIdV1,
        owner_head: GlobalId,
        public_group: GlobalId,
    },
}

impl CanonicalEncode for PublicSemanticSeedSubjectV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Declaration {
                declaration,
                public_group,
            } => {
                encoder.tag(0);
                declaration.encode_canonical(encoder);
                public_group.encode_canonical(encoder);
            }
            Self::Equation {
                equation,
                owner_head,
                public_group,
            } => {
                encoder.tag(1);
                equation.encode_canonical(encoder);
                owner_head.encode_canonical(encoder);
                public_group.encode_canonical(encoder);
            }
        }
    }
}

/// One fully reconstructed, kernel-checked rank-zero V2 seed.
///
/// Every semantic metadata field is private and verifier-derived.  This type
/// deliberately has no `Deserialize` implementation and does not retain a
/// caller-provided V1 seed wire.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPreQ0SemanticSeedV2 {
    id: SeedIdV1,
    subject: PublicSemanticSeedSubjectV2,
    source_identity: Digest,
    origin_event: EventIdV1,
    source_judgment: GenericJudgmentV1,
    source_clause: ClauseIdV1,
    public_support: PublicSupportV1,
    demand_anchor: Option<(DemandOrbitIdV1, DemandOutputIdV1)>,
    local_role: LocalRoleV1,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedPreQ0SemanticSeedV2 {
    pub fn id(&self) -> &SeedIdV1 {
        &self.id
    }

    pub fn subject(&self) -> &PublicSemanticSeedSubjectV2 {
        &self.subject
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn origin_event(&self) -> &EventIdV1 {
        &self.origin_event
    }

    pub fn source_judgment(&self) -> &GenericJudgmentV1 {
        &self.source_judgment
    }

    pub fn source_clause(&self) -> &ClauseIdV1 {
        &self.source_clause
    }

    pub fn public_support(&self) -> &PublicSupportV1 {
        &self.public_support
    }

    pub fn demand_anchor(&self) -> Option<&(DemandOrbitIdV1, DemandOutputIdV1)> {
        self.demand_anchor.as_ref()
    }

    pub fn local_role(&self) -> LocalRoleV1 {
        self.local_role
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedPreQ0SemanticSeedV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.origin_event.encode_canonical(encoder);
        self.source_judgment.encode_canonical(encoder);
        self.source_clause.encode_canonical(encoder);
        self.public_support.encode_canonical(encoder);
        match &self.demand_anchor {
            Some((orbit, output)) => {
                encoder.tag(1);
                orbit.encode_canonical(encoder);
                output.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
        self.local_role.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteSeedCoverage;
}

/// Complete V2 rank-zero seed authority for one compatible public inventory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSemanticSeedCensusV2 {
    semantic_manifest_digest: Digest,
    inventory_compatibility_digest: Digest,
    inventory_digest: Digest,
    public_clause_census_digest: Digest,
    demand_anchor_census_digest: Digest,
    seeds: Arc<[VerifiedPreQ0SemanticSeedV2]>,
    coverage_digest: Digest,
    complete: private::CompleteSeedCoverage,
    digest: Digest,
}

impl VerifiedSemanticSeedCensusV2 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn inventory_compatibility_digest(&self) -> &Digest {
        &self.inventory_compatibility_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn public_clause_census_digest(&self) -> &Digest {
        &self.public_clause_census_digest
    }

    pub fn demand_anchor_census_digest(&self) -> &Digest {
        &self.demand_anchor_census_digest
    }

    pub fn seeds(&self) -> &[VerifiedPreQ0SemanticSeedV2] {
        &self.seeds
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSemanticSeedCensusV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.inventory_compatibility_digest
            .encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.public_clause_census_digest.encode_canonical(encoder);
        self.demand_anchor_census_digest.encode_canonical(encoder);
        encoder.sequence(&self.seeds);
        self.coverage_digest.encode_canonical(encoder);
        encoder.tag(1);
        self.digest.encode_canonical(encoder);
    }
}

/// Reconstruct every public rank-zero seed from compatible V2-bound
/// inventory authority.
///
/// Nonempty demand ports require records in the independently verified demand
/// anchor census.  Since the current demand verifier mints only the positive
/// empty base, a nonempty inventory stops before this function can mint the
/// seed census.
pub fn verify_semantic_seed_census_v2(
    manifest: &VerifiedSemanticAuditManifestV2,
    kernel: &Kernel,
    inventory_compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    demand_anchors: &VerifiedDemandAnchorCensusV1,
) -> AuditDecision<VerifiedSemanticSeedCensusV2> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2
        || inventory_compatibility.v2_manifest_digest() != manifest.candidate_digest()
        || inventory_compatibility.inventory_digest() != inventory.digest()
        || inventory_compatibility.coverage_digest() != inventory.coverage().digest()
        || inventory_compatibility.predecessor_history_digest()
            != inventory.predecessor_history_digest()
        || inventory_compatibility.predecessor_boundary_digest()
            != inventory.predecessor_boundary().digest()
        || inventory_compatibility.successor_boundary_digest()
            != inventory.successor_boundary().digest()
        || inventory_compatibility.exact_extension_digest() != inventory.exact_extension().digest()
        || inventory_compatibility.normalizer_protocol_digest()
            != inventory.normalizer_protocol_digest()
        || inventory_compatibility.normalizer_protocol_digest()
            != &kernel.normalizer_protocol_digest()
        || public_clauses.inventory_digest() != inventory.digest()
        || public_clauses.inventory_coverage_digest() != inventory.coverage().digest()
        || demand_anchors.inventory_digest() != inventory.digest()
        || demand_anchors.predecessor_history_digest() != inventory.predecessor_history_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let expected_seed_count = inventory
        .declarations()
        .len()
        .saturating_add(inventory.equations().len());
    if expected_seed_count > usize::from(manifest.manifest().maximum_seeds)
        || public_clauses.clauses().len() != expected_seed_count
    {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }

    let mut seeds = Vec::with_capacity(expected_seed_count);
    let mut covered_clauses = BTreeSet::new();
    let mut covered_realizations = BTreeSet::new();

    for declaration in inventory.declarations() {
        let Some(clause_id) = public_clauses.clause_for_declaration(declaration.declaration())
        else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        let Some(clause) = public_clauses
            .clauses()
            .iter()
            .find(|clause| clause.id() == clause_id)
        else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        if clause.subject()
            != &(PublicClauseSubjectV1::Declaration {
                declaration: declaration.declaration().clone(),
            })
            || !covered_clauses.insert(clause_id.clone())
        {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        let source_judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Global {
                id: declaration.declaration().clone(),
            },
            ty: declaration.source().ty.clone(),
        };
        let seed = match verified_seed_record(
            manifest,
            kernel,
            inventory,
            PublicSemanticSeedSubjectV2::Declaration {
                declaration: declaration.declaration().clone(),
                public_group: declaration.group().clone(),
            },
            declaration.source_identity(),
            declaration.origin(),
            source_judgment,
            clause,
            None,
            LocalRoleV1::KernelHead,
        ) {
            Ok(seed) => seed,
            Err(failure) => return failure.into_decision(),
        };
        seeds.push(seed);
    }

    for equation in inventory.equations() {
        let Some(clause_id) = public_clauses.clause_for_equation(equation.equation()) else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        let Some(clause) = public_clauses
            .clauses()
            .iter()
            .find(|clause| clause.id() == clause_id)
        else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        if clause.subject()
            != &(PublicClauseSubjectV1::Equation {
                equation: equation.equation().clone(),
                owner_head: equation.owner_head().clone(),
            })
            || !covered_clauses.insert(clause_id.clone())
        {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        let demand_anchor = match equation.demand_port() {
            Some(port) => {
                let matches = demand_anchors
                    .realization_certificates()
                    .iter()
                    .filter(|realization| {
                        realization.equation() == equation.equation() && realization.port() == port
                    })
                    .collect::<Vec<_>>();
                let [realization] = matches.as_slice() else {
                    return AuditDecision::Unknown(
                        AuditUnknownReason::MissingDemandRealizationAuthority,
                    );
                };
                if !covered_realizations.insert(equation.equation().clone())
                    || !demand_anchors
                        .orbit_classes()
                        .iter()
                        .any(|orbit| orbit.id() == realization.orbit())
                    || !demand_anchors.ports().iter().any(|verified_port| {
                        verified_port.port() == port && verified_port.orbit() == realization.orbit()
                    })
                {
                    return AuditDecision::Unknown(AuditUnknownReason::MissingDemandOrbitAuthority);
                }
                Some((realization.orbit().clone(), port.output.clone()))
            }
            None => None,
        };
        let seed = match verified_seed_record(
            manifest,
            kernel,
            inventory,
            PublicSemanticSeedSubjectV2::Equation {
                equation: equation.equation().clone(),
                owner_head: equation.owner_head().clone(),
                public_group: clause.public_group().clone(),
            },
            equation.source_identity(),
            equation.origin(),
            equation.source().clone(),
            clause,
            demand_anchor,
            LocalRoleV1::Coherence,
        ) {
            Ok(seed) => seed,
            Err(failure) => return failure.into_decision(),
        };
        seeds.push(seed);
    }

    if covered_clauses.len() != public_clauses.clauses().len()
        || covered_realizations.len() != demand_anchors.realization_certificates().len()
    {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }
    seeds.sort_by(|left, right| left.id.cmp(&right.id));
    if seeds.windows(2).any(|pair| pair[0].id == pair[1].id) {
        return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
    }

    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/semantic-seed-census-coverage/v2",
        &SemanticSeedCoverageMaterialV2 {
            manifest: manifest.candidate_digest(),
            inventory_compatibility: inventory_compatibility.digest(),
            inventory: inventory.digest(),
            clauses: public_clauses.digest(),
            demand_anchors: demand_anchors.digest(),
            seeds: &seeds,
        },
    );
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-semantic-seed-census/v2",
        &SemanticSeedCensusMaterialV2 {
            manifest: manifest.candidate_digest(),
            inventory_compatibility: inventory_compatibility.digest(),
            inventory: inventory.digest(),
            clauses: public_clauses.digest(),
            demand_anchors: demand_anchors.digest(),
            coverage: &coverage_digest,
            seeds: &seeds,
        },
    );

    AuditDecision::Proven(VerifiedSemanticSeedCensusV2 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        inventory_compatibility_digest: inventory_compatibility.digest().clone(),
        inventory_digest: inventory.digest().clone(),
        public_clause_census_digest: public_clauses.digest().clone(),
        demand_anchor_census_digest: demand_anchors.digest().clone(),
        seeds: Arc::from(seeds),
        coverage_digest,
        complete: private::CompleteSeedCoverage,
        digest,
    })
}

#[derive(Clone, Debug)]
enum SemanticSeedFailureV2 {
    Unknown(AuditUnknownReason),
    Outside(crate::manifest::OutsideFragmentReason),
}

impl SemanticSeedFailureV2 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::Unknown(reason) => AuditDecision::Unknown(reason),
            Self::Outside(reason) => AuditDecision::OutsideFragment(reason),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn verified_seed_record(
    manifest: &VerifiedSemanticAuditManifestV2,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    subject: PublicSemanticSeedSubjectV2,
    source_identity: &Digest,
    origin_event: &EventIdV1,
    source_judgment: GenericJudgmentV1,
    clause: &VerifiedPublicClauseIdentityV1,
    demand_anchor: Option<(DemandOrbitIdV1, DemandOutputIdV1)>,
    local_role: LocalRoleV1,
) -> Result<VerifiedPreQ0SemanticSeedV2, SemanticSeedFailureV2> {
    if source_judgment.context().0.len() > usize::from(manifest.manifest().maximum_context_entries)
    {
        return Err(SemanticSeedFailureV2::Unknown(
            AuditUnknownReason::ResourceExhausted,
        ));
    }
    if let Some(violation) = lambda_unit_judgment_syntax_violation(
        &source_judgment,
        &manifest.manifest().universe_levels,
    ) {
        return Err(SemanticSeedFailureV2::Outside(violation.outside_reason()));
    }
    let kernel_replay_digest = replay_seed_judgment(kernel, inventory, &source_judgment)
        .map_err(SemanticSeedFailureV2::Unknown)?;
    let mut public_support = clause.public_support().clone();
    if let Some((_, output)) = &demand_anchor {
        public_support.demand_outputs.insert(output.clone());
    }
    let id = SeedIdV1(Digest::of_canonical(
        "pen-semantic-audit/pre-q0-semantic-seed-id/v2",
        &SemanticSeedIdentityMaterialV2 {
            inventory: inventory.digest(),
            subject: &subject,
            source_identity,
            origin_event,
            source_judgment: &source_judgment,
            source_clause: clause.id(),
            public_support: &public_support,
            demand_anchor: demand_anchor.as_ref(),
            local_role,
            kernel_replay: &kernel_replay_digest,
        },
    ));
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-pre-q0-semantic-seed/v2",
        &SemanticSeedDigestMaterialV2 {
            id: &id,
            subject: &subject,
            source_identity,
            origin_event,
            source_judgment: &source_judgment,
            source_clause: clause.id(),
            public_support: &public_support,
            demand_anchor: demand_anchor.as_ref(),
            local_role,
            kernel_replay: &kernel_replay_digest,
        },
    );
    Ok(VerifiedPreQ0SemanticSeedV2 {
        id,
        subject,
        source_identity: source_identity.clone(),
        origin_event: origin_event.clone(),
        source_judgment,
        source_clause: clause.id().clone(),
        public_support,
        demand_anchor,
        local_role,
        kernel_replay_digest,
        digest,
    })
}

fn replay_seed_judgment(
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    judgment: &GenericJudgmentV1,
) -> Result<Digest, AuditUnknownReason> {
    let signature = inventory.successor_boundary();
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            let input = OpenJudgment::HasType {
                context: context.clone(),
                term: term.clone(),
                ty: ty.clone(),
            };
            let output = kernel
                .verify_open_judgment(signature, &input)
                .map_err(seed_kernel_unknown)?;
            Ok(Digest::of_canonical(
                "pen-semantic-audit/semantic-seed-kernel-replay/v2",
                &KernelReplayMaterialV2 {
                    input: &input,
                    output: &output,
                },
            ))
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            let left_input = OpenJudgment::HasType {
                context: context.clone(),
                term: left.clone(),
                ty: ty.clone(),
            };
            let right_input = OpenJudgment::HasType {
                context: context.clone(),
                term: right.clone(),
                ty: ty.clone(),
            };
            let outputs = kernel
                .verify_open_judgments(signature, &[&left_input, &right_input])
                .map_err(seed_kernel_unknown)?;
            if outputs.len() != 2 {
                return Err(AuditUnknownReason::KernelCouldNotCertify);
            }
            Ok(Digest::of_canonical(
                "pen-semantic-audit/semantic-equation-seed-kernel-replay/v2",
                &EquationKernelReplayMaterialV2 {
                    left_input: &left_input,
                    left_output: &outputs[0],
                    right_input: &right_input,
                    right_output: &outputs[1],
                },
            ))
        }
    }
}

fn seed_kernel_unknown(error: KernelError) -> AuditUnknownReason {
    match error {
        KernelError::ResourceExhausted(
            ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
        ) => AuditUnknownReason::ResourceExhausted,
        _ => AuditUnknownReason::KernelCouldNotCertify,
    }
}

fn dependencies_for(
    inventory: &VerifiedPublicAuditInventoryV1,
    subject: &PublicSubjectV1,
) -> Vec<GlobalId> {
    inventory
        .dependency_dag()
        .edges()
        .iter()
        .filter(|dependency| &dependency.dependent == subject)
        .map(|dependency| dependency.prerequisite.clone())
        .collect()
}

fn derive_public_support(
    inventory: &VerifiedPublicAuditInventoryV1,
    origin: &EventIdV1,
    owner: &GlobalId,
    dependency_support: &[GlobalId],
) -> Result<PublicSupportV1, AuditUnknownReason> {
    let mut declarations = dependency_support.iter().cloned().collect::<BTreeSet<_>>();
    declarations.insert(owner.clone());

    let mut events = BTreeSet::from([origin.clone()]);
    for declaration in &declarations {
        let declaration_origin = inventory
            .declaration_origins()
            .get(declaration)
            .ok_or(AuditUnknownReason::IncompleteSupport)?;
        events.insert(declaration_origin.clone());
    }

    Ok(PublicSupportV1 {
        events,
        declarations,
        // Port-key association is not a realization theorem.
        demand_outputs: BTreeSet::new(),
    })
}

fn declaration_clause(
    inventory: &VerifiedPublicAuditInventoryV1,
    declaration: &VerifiedPublicDeclarationV1,
    dependency_support: Vec<GlobalId>,
    public_support: PublicSupportV1,
) -> VerifiedPublicClauseIdentityV1 {
    let material = DeclarationClauseIdentityMaterial {
        inventory: inventory.digest(),
        inventory_coverage: inventory.coverage().digest(),
        declaration: declaration.declaration(),
        source_identity: declaration.source_identity(),
        origin: declaration.origin(),
        normalized: declaration.normalized(),
        group: declaration.group(),
        owner: declaration.declaration(),
        dependency_support: &dependency_support,
    };
    let id = ClauseIdV1(Digest::of_canonical(
        "pen-semantic-audit/public-declaration-clause-id/v1",
        &material,
    ));
    let normalized_subject_digest = Digest::of_canonical(
        "pen-semantic-audit/public-clause-normalized-declaration/v1",
        declaration.normalized(),
    );
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-public-declaration-clause/v1",
        &VerifiedClauseDigestMaterial {
            id: &id,
            identity: &material,
            public_support: &public_support,
        },
    );
    VerifiedPublicClauseIdentityV1 {
        id,
        subject: PublicClauseSubjectV1::Declaration {
            declaration: declaration.declaration().clone(),
        },
        source_identity: declaration.source_identity().clone(),
        origin_event: declaration.origin().clone(),
        public_group: declaration.group().clone(),
        normalized_subject_digest,
        dependency_support: Arc::from(dependency_support),
        public_support,
        digest,
    }
}

fn equation_clause(
    inventory: &VerifiedPublicAuditInventoryV1,
    equation: &VerifiedPublicEquationV1,
    group: &GlobalId,
    dependency_support: Vec<GlobalId>,
    public_support: PublicSupportV1,
) -> VerifiedPublicClauseIdentityV1 {
    let material = EquationClauseIdentityMaterial {
        inventory: inventory.digest(),
        inventory_coverage: inventory.coverage().digest(),
        equation: equation.equation(),
        source_identity: equation.source_identity(),
        origin: equation.origin(),
        normalized: equation.normalized(),
        group,
        owner: equation.owner_head(),
        dependency_support: &dependency_support,
    };
    let id = ClauseIdV1(Digest::of_canonical(
        "pen-semantic-audit/public-equation-clause-id/v1",
        &material,
    ));
    let normalized_subject_digest = Digest::of_canonical(
        "pen-semantic-audit/public-clause-normalized-equation/v1",
        equation.normalized(),
    );
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-public-equation-clause/v1",
        &VerifiedClauseDigestMaterial {
            id: &id,
            identity: &material,
            public_support: &public_support,
        },
    );
    VerifiedPublicClauseIdentityV1 {
        id,
        subject: PublicClauseSubjectV1::Equation {
            equation: equation.equation().clone(),
            owner_head: equation.owner_head().clone(),
        },
        source_identity: equation.source_identity().clone(),
        origin_event: equation.origin().clone(),
        public_group: group.clone(),
        normalized_subject_digest,
        dependency_support: Arc::from(dependency_support),
        public_support,
        digest,
    }
}

struct DeclarationClauseIdentityMaterial<'a> {
    inventory: &'a Digest,
    inventory_coverage: &'a Digest,
    declaration: &'a GlobalId,
    source_identity: &'a Digest,
    origin: &'a EventIdV1,
    normalized: &'a Declaration,
    group: &'a GlobalId,
    owner: &'a GlobalId,
    dependency_support: &'a [GlobalId],
}

impl CanonicalEncode for DeclarationClauseIdentityMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.inventory_coverage.encode_canonical(encoder);
        self.declaration.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.normalized.encode_canonical(encoder);
        self.group.encode_canonical(encoder);
        self.owner.encode_canonical(encoder);
        encoder.sequence(self.dependency_support);
    }
}

struct EquationClauseIdentityMaterial<'a> {
    inventory: &'a Digest,
    inventory_coverage: &'a Digest,
    equation: &'a EquationIdV1,
    source_identity: &'a Digest,
    origin: &'a EventIdV1,
    normalized: &'a crate::model::GenericJudgmentV1,
    group: &'a GlobalId,
    owner: &'a GlobalId,
    dependency_support: &'a [GlobalId],
}

impl CanonicalEncode for EquationClauseIdentityMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.inventory_coverage.encode_canonical(encoder);
        self.equation.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.normalized.encode_canonical(encoder);
        self.group.encode_canonical(encoder);
        self.owner.encode_canonical(encoder);
        encoder.sequence(self.dependency_support);
    }
}

struct VerifiedClauseDigestMaterial<'a, T> {
    id: &'a ClauseIdV1,
    identity: &'a T,
    public_support: &'a PublicSupportV1,
}

impl<T: CanonicalEncode> CanonicalEncode for VerifiedClauseDigestMaterial<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.identity.encode_canonical(encoder);
        self.public_support.encode_canonical(encoder);
    }
}

struct DeclarationClauseMapEntry<'a> {
    subject: &'a GlobalId,
    clause: &'a ClauseIdV1,
}

impl CanonicalEncode for DeclarationClauseMapEntry<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.subject.encode_canonical(encoder);
        self.clause.encode_canonical(encoder);
    }
}

struct EquationClauseMapEntry<'a> {
    subject: &'a EquationIdV1,
    clause: &'a ClauseIdV1,
}

impl CanonicalEncode for EquationClauseMapEntry<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.subject.encode_canonical(encoder);
        self.clause.encode_canonical(encoder);
    }
}

struct PublicClauseCoverageMaterial<'a> {
    inventory: &'a Digest,
    inventory_coverage: &'a Digest,
    clauses: &'a [VerifiedPublicClauseIdentityV1],
    declaration_entries: &'a [DeclarationClauseMapEntry<'a>],
    equation_entries: &'a [EquationClauseMapEntry<'a>],
}

impl CanonicalEncode for PublicClauseCoverageMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.inventory_coverage.encode_canonical(encoder);
        encoder.sequence(self.clauses);
        encoder.sequence(self.declaration_entries);
        encoder.sequence(self.equation_entries);
    }
}

struct PublicClauseCensusDigestMaterial<'a> {
    inventory: &'a Digest,
    inventory_coverage: &'a Digest,
    coverage: &'a Digest,
    clauses: &'a [VerifiedPublicClauseIdentityV1],
}

impl CanonicalEncode for PublicClauseCensusDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.inventory_coverage.encode_canonical(encoder);
        self.coverage.encode_canonical(encoder);
        encoder.sequence(self.clauses);
    }
}

struct DemandContractDigestEntry<'a> {
    contract: &'a DemandContractIdV1,
    origin: &'a EventIdV1,
    port: &'a DemandPortKeyV1,
    source_identity: &'a Digest,
    source_requirement: &'a GenericJudgmentV1,
    normalized_requirement: &'a GenericJudgmentV1,
}

impl CanonicalEncode for DemandContractDigestEntry<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.contract.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.source_requirement.encode_canonical(encoder);
        self.normalized_requirement.encode_canonical(encoder);
    }
}

struct EquationPortDigestEntry<'a> {
    equation: &'a EquationIdV1,
    port: &'a DemandPortKeyV1,
}

impl CanonicalEncode for EquationPortDigestEntry<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
    }
}

struct DemandContractCensusMaterial<'a> {
    inventory: &'a Digest,
    inventory_coverage: &'a Digest,
    predecessor_history: &'a Digest,
    entries: &'a [DemandContractDigestEntry<'a>],
}

impl CanonicalEncode for DemandContractCensusMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.inventory_coverage.encode_canonical(encoder);
        self.predecessor_history.encode_canonical(encoder);
        encoder.sequence(self.entries);
    }
}

struct EmptyDemandAnchorCensusMaterial<'a> {
    inventory: &'a Digest,
    predecessor_history: &'a Digest,
    predecessor_demand: &'a Digest,
    equation_ports: &'a [EquationPortDigestEntry<'a>],
}

impl CanonicalEncode for EmptyDemandAnchorCensusMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.predecessor_history.encode_canonical(encoder);
        self.predecessor_demand.encode_canonical(encoder);
        encoder.sequence(self.equation_ports);
    }
}

struct SemanticSeedIdentityMaterialV2<'a> {
    inventory: &'a Digest,
    subject: &'a PublicSemanticSeedSubjectV2,
    source_identity: &'a Digest,
    origin_event: &'a EventIdV1,
    source_judgment: &'a GenericJudgmentV1,
    source_clause: &'a ClauseIdV1,
    public_support: &'a PublicSupportV1,
    demand_anchor: Option<&'a (DemandOrbitIdV1, DemandOutputIdV1)>,
    local_role: LocalRoleV1,
    kernel_replay: &'a Digest,
}

impl CanonicalEncode for SemanticSeedIdentityMaterialV2<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        encode_semantic_seed_material(
            encoder,
            self.subject,
            self.source_identity,
            self.origin_event,
            self.source_judgment,
            self.source_clause,
            self.public_support,
            self.demand_anchor,
            self.local_role,
            self.kernel_replay,
        );
    }
}

struct SemanticSeedDigestMaterialV2<'a> {
    id: &'a SeedIdV1,
    subject: &'a PublicSemanticSeedSubjectV2,
    source_identity: &'a Digest,
    origin_event: &'a EventIdV1,
    source_judgment: &'a GenericJudgmentV1,
    source_clause: &'a ClauseIdV1,
    public_support: &'a PublicSupportV1,
    demand_anchor: Option<&'a (DemandOrbitIdV1, DemandOutputIdV1)>,
    local_role: LocalRoleV1,
    kernel_replay: &'a Digest,
}

impl CanonicalEncode for SemanticSeedDigestMaterialV2<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        encode_semantic_seed_material(
            encoder,
            self.subject,
            self.source_identity,
            self.origin_event,
            self.source_judgment,
            self.source_clause,
            self.public_support,
            self.demand_anchor,
            self.local_role,
            self.kernel_replay,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn encode_semantic_seed_material(
    encoder: &mut CanonicalEncoder,
    subject: &PublicSemanticSeedSubjectV2,
    source_identity: &Digest,
    origin_event: &EventIdV1,
    source_judgment: &GenericJudgmentV1,
    source_clause: &ClauseIdV1,
    public_support: &PublicSupportV1,
    demand_anchor: Option<&(DemandOrbitIdV1, DemandOutputIdV1)>,
    local_role: LocalRoleV1,
    kernel_replay: &Digest,
) {
    subject.encode_canonical(encoder);
    source_identity.encode_canonical(encoder);
    origin_event.encode_canonical(encoder);
    source_judgment.encode_canonical(encoder);
    source_clause.encode_canonical(encoder);
    public_support.encode_canonical(encoder);
    match demand_anchor {
        Some((orbit, output)) => {
            encoder.tag(1);
            orbit.encode_canonical(encoder);
            output.encode_canonical(encoder);
        }
        None => encoder.tag(0),
    }
    local_role.encode_canonical(encoder);
    kernel_replay.encode_canonical(encoder);
}

struct KernelReplayMaterialV2<'a> {
    input: &'a OpenJudgment,
    output: &'a OpenJudgment,
}

impl CanonicalEncode for KernelReplayMaterialV2<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.input.encode_canonical(encoder);
        self.output.encode_canonical(encoder);
    }
}

struct EquationKernelReplayMaterialV2<'a> {
    left_input: &'a OpenJudgment,
    left_output: &'a OpenJudgment,
    right_input: &'a OpenJudgment,
    right_output: &'a OpenJudgment,
}

impl CanonicalEncode for EquationKernelReplayMaterialV2<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.left_input.encode_canonical(encoder);
        self.left_output.encode_canonical(encoder);
        self.right_input.encode_canonical(encoder);
        self.right_output.encode_canonical(encoder);
    }
}

struct SemanticSeedCoverageMaterialV2<'a> {
    manifest: &'a Digest,
    inventory_compatibility: &'a Digest,
    inventory: &'a Digest,
    clauses: &'a Digest,
    demand_anchors: &'a Digest,
    seeds: &'a [VerifiedPreQ0SemanticSeedV2],
}

impl CanonicalEncode for SemanticSeedCoverageMaterialV2<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.inventory_compatibility.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        self.clauses.encode_canonical(encoder);
        self.demand_anchors.encode_canonical(encoder);
        encoder.sequence(self.seeds);
        encoder.u64(self.seeds.len() as u64);
    }
}

struct SemanticSeedCensusMaterialV2<'a> {
    manifest: &'a Digest,
    inventory_compatibility: &'a Digest,
    inventory: &'a Digest,
    clauses: &'a Digest,
    demand_anchors: &'a Digest,
    coverage: &'a Digest,
    seeds: &'a [VerifiedPreQ0SemanticSeedV2],
}

impl CanonicalEncode for SemanticSeedCensusMaterialV2<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.inventory_compatibility.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        self.clauses.encode_canonical(encoder);
        self.demand_anchors.encode_canonical(encoder);
        self.coverage.encode_canonical(encoder);
        encoder.sequence(self.seeds);
        encoder.tag(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        DemandFamilyIdV1, ORIGIN_CUTOFF_Q3_SCHEMA_VERSION, PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
        PublicDependencyUseV1, UncheckedOriginCutoffQ3RegistryV1,
        UncheckedPredecessorDemandContractV1, UncheckedPublicAuditInventoryV1,
        UncheckedPublicAvailabilityClaimV1, UncheckedPublicDeclarationV1,
        UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1, UncheckedPublicEventCensusV1,
        UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::inventory_compatibility::verify_public_inventory_compatibility_v2;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_lambda_unit_manifest_v2,
        verify_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v2,
    };
    use crate::model::{DemandOutputIdV1, PublicAvailabilityV1, SourceNormalizedJudgmentV1};
    use pen_kernel::{
        Declaration, DependentContext, Kernel, KernelLimits, Term, UncheckedSignature,
    };

    fn verified_inventory() -> (VerifiedPublicAuditInventoryV1, GlobalId, EventIdV1) {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("lambda V1 manifest");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let head = GlobalId(Digest::of_bytes(b"clause-census/head"));
        let group = GlobalId(Digest::of_bytes(b"clause-census/group"));
        let event = EventIdV1(Digest::of_bytes(b"clause-census/event"));
        let declaration = Declaration {
            id: head.clone(),
            ty: pen_kernel::Term::UnitType,
            body: None,
        };
        let source_identity = Digest::of_canonical(
            "pen-semantic-audit/inventory-source-declaration/v1",
            &declaration,
        );
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: Vec::new(),
            predecessor_boundary: UncheckedSignature::default(),
            successor_event: UncheckedPublicEventCensusV1 {
                event: event.clone(),
                added_groups: vec![group.clone()],
                added_declarations: vec![head.clone()],
                added_equations: Vec::new(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary: UncheckedSignature {
                declarations: vec![declaration.clone()],
            },
            declaration_groups: vec![UncheckedPublicGroupV1 {
                group,
                origin: event.clone(),
                declarations: vec![head.clone()],
            }],
            declarations: vec![UncheckedPublicDeclarationV1 {
                declaration: head.clone(),
                origin: event.clone(),
                group: GlobalId(Digest::of_bytes(b"clause-census/group")),
                source_to_normal: UncheckedSourceNormalizedDeclarationV1 {
                    source_identity,
                    source: declaration.clone(),
                    claimed_normalized: declaration,
                },
            }],
            equations: Vec::new(),
            forced_projections: Vec::new(),
            predecessor_demand_contracts: Vec::new(),
            public_availability: Vec::new(),
            dependency_dag: UncheckedPublicDependencyDagV1 { edges: Vec::new() },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: None,
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&manifest, &kernel, &wire)
        else {
            panic!("inventory");
        };
        (inventory, head, event)
    }

    fn source_declaration(declaration: &Declaration) -> UncheckedSourceNormalizedDeclarationV1 {
        UncheckedSourceNormalizedDeclarationV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-declaration/v1",
                declaration,
            ),
            source: declaration.clone(),
            claimed_normalized: declaration.clone(),
        }
    }

    fn source_judgment(judgment: &GenericJudgmentV1) -> SourceNormalizedJudgmentV1 {
        SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-judgment/v1",
                judgment,
            ),
            source: judgment.clone(),
            claimed_normalized: judgment.clone(),
        }
    }

    fn verified_inventory_with_unrealized_demand() -> VerifiedPublicAuditInventoryV1 {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("lambda V1 manifest");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let predecessor_head = GlobalId(Digest::of_bytes(b"demand-census/predecessor-head"));
        let successor_head = GlobalId(Digest::of_bytes(b"demand-census/successor-head"));
        let predecessor_group = GlobalId(Digest::of_bytes(b"demand-census/predecessor-group"));
        let successor_group = GlobalId(Digest::of_bytes(b"demand-census/successor-group"));
        let predecessor_event = EventIdV1(Digest::of_bytes(b"demand-census/predecessor-event"));
        let successor_event = EventIdV1(Digest::of_bytes(b"demand-census/successor-event"));
        let contract = DemandContractIdV1(Digest::of_bytes(b"demand-census/contract"));
        let equation = EquationIdV1(Digest::of_bytes(b"demand-census/equation"));
        let port = DemandPortKeyV1 {
            family: DemandFamilyIdV1(Digest::of_bytes(b"demand-census/family")),
            output: DemandOutputIdV1(Digest::of_bytes(b"demand-census/output")),
        };
        let predecessor_declaration = Declaration {
            id: predecessor_head.clone(),
            ty: Term::UnitType,
            body: None,
        };
        let successor_declaration = Declaration {
            id: successor_head.clone(),
            ty: Term::UnitType,
            body: None,
        };
        let demand_judgment = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Unit,
            right: Term::Unit,
            ty: Term::UnitType,
        };
        let predecessor_boundary = UncheckedSignature {
            declarations: vec![predecessor_declaration.clone()],
        };
        let successor_boundary = UncheckedSignature {
            declarations: vec![
                predecessor_declaration.clone(),
                successor_declaration.clone(),
            ],
        };
        let dependency = PublicDependencyUseV1 {
            dependent: PublicSubjectV1::Equation {
                equation: equation.clone(),
            },
            prerequisite: successor_head.clone(),
        };
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: vec![UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group.clone()],
                    added_declarations: vec![predecessor_head.clone()],
                    added_equations: Vec::new(),
                    added_forced_projections: Vec::new(),
                    added_demand_contracts: vec![contract.clone()],
                },
                successor_boundary: predecessor_boundary.clone(),
            }],
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: successor_event.clone(),
                added_groups: vec![successor_group.clone()],
                added_declarations: vec![successor_head.clone()],
                added_equations: vec![equation.clone()],
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups: vec![
                UncheckedPublicGroupV1 {
                    group: predecessor_group.clone(),
                    origin: predecessor_event.clone(),
                    declarations: vec![predecessor_head.clone()],
                },
                UncheckedPublicGroupV1 {
                    group: successor_group.clone(),
                    origin: successor_event.clone(),
                    declarations: vec![successor_head.clone()],
                },
            ],
            declarations: vec![
                UncheckedPublicDeclarationV1 {
                    declaration: predecessor_head,
                    origin: predecessor_event.clone(),
                    group: predecessor_group,
                    source_to_normal: source_declaration(&predecessor_declaration),
                },
                UncheckedPublicDeclarationV1 {
                    declaration: successor_head.clone(),
                    origin: successor_event.clone(),
                    group: successor_group,
                    source_to_normal: source_declaration(&successor_declaration),
                },
            ],
            equations: vec![UncheckedPublicEquationV1 {
                equation,
                owner_head: successor_head.clone(),
                origin: successor_event,
                source_to_normal: source_judgment(&demand_judgment),
                demand_port: Some(port.clone()),
            }],
            forced_projections: Vec::new(),
            predecessor_demand_contracts: vec![UncheckedPredecessorDemandContractV1 {
                contract,
                origin: predecessor_event.clone(),
                port,
                required_judgment: source_judgment(&demand_judgment),
            }],
            public_availability: vec![UncheckedPublicAvailabilityClaimV1 {
                dependency: dependency.clone(),
                claimed: PublicAvailabilityV1::DependencyPriorExport {
                    target: successor_head,
                },
            }],
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: vec![dependency],
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: Some(predecessor_event),
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&manifest, &kernel, &wire)
        else {
            panic!("demand inventory");
        };
        inventory
    }

    #[test]
    fn clause_census_is_complete_canonical_and_support_derived() {
        let (inventory, head, event) = verified_inventory();
        let AuditDecision::Proven(first) = verify_public_clause_census_v1(&inventory) else {
            panic!("first census");
        };
        let AuditDecision::Proven(second) = verify_public_clause_census_v1(&inventory) else {
            panic!("second census");
        };

        assert_eq!(first.digest(), second.digest());
        assert_eq!(first.inventory_digest(), inventory.digest());
        assert_eq!(first.clauses().len(), 1);
        let clause = &first.clauses()[0];
        assert_eq!(first.clause_for_declaration(&head), Some(clause.id()));
        assert_eq!(clause.public_support().events, BTreeSet::from([event]));
        assert_eq!(clause.public_support().declarations, BTreeSet::from([head]));
        assert!(clause.public_support().demand_outputs.is_empty());
        assert!(
            first
                .clause_for_equation(&EquationIdV1(Digest::of_bytes(b"absent")))
                .is_none()
        );
    }

    #[test]
    fn empty_demand_anchor_base_is_positive_and_contains_no_port_upgrade() {
        let (inventory, _, _) = verified_inventory();
        assert!(demand_anchor_blockers_v1(&inventory).is_empty());

        let AuditDecision::Proven(census) = verify_demand_anchor_census_v1(&inventory) else {
            panic!("empty demand census");
        };
        assert_eq!(census.inventory_digest(), inventory.digest());
        assert_eq!(
            census.predecessor_history_digest(),
            inventory.predecessor_history_digest()
        );
        assert!(census.is_empty());
        assert!(census.orbit_classes().is_empty());
        assert!(census.ports().is_empty());
        assert!(census.realization_certificates().is_empty());
    }

    #[test]
    fn port_key_does_not_mint_orbit_or_typed_realization() {
        let inventory = verified_inventory_with_unrealized_demand();
        assert_eq!(
            demand_anchor_blockers_v1(&inventory),
            BTreeSet::from([
                DemandAnchorBlockerV1::MissingDemandOrbitAuthority,
                DemandAnchorBlockerV1::MissingTypedRealizationAuthority,
            ])
        );
        assert!(matches!(
            verify_demand_anchor_census_v1(&inventory),
            AuditDecision::Unknown(AuditUnknownReason::MissingDemandOrbitAuthority)
        ));
    }

    #[test]
    fn empty_demand_inventory_mints_complete_v2_seed_census() {
        let (inventory, head, event) = verified_inventory();
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let AuditDecision::Proven(v1_manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("V1 manifest");
        };
        let AuditDecision::Proven(v2_manifest) = verify_semantic_audit_lambda_unit_manifest_v2(
            &proposed_semantic_audit_lambda_unit_manifest_v2(),
        ) else {
            panic!("V2 manifest");
        };
        let AuditDecision::Proven(compatibility) =
            verify_public_inventory_compatibility_v2(&v1_manifest, &v2_manifest, &inventory)
        else {
            panic!("inventory compatibility");
        };
        let AuditDecision::Proven(clauses) = verify_public_clause_census_v1(&inventory) else {
            panic!("clauses");
        };
        let AuditDecision::Proven(demands) = verify_demand_anchor_census_v1(&inventory) else {
            panic!("empty demands");
        };

        let AuditDecision::Proven(census) = verify_semantic_seed_census_v2(
            &v2_manifest,
            &kernel,
            &compatibility,
            &inventory,
            &clauses,
            &demands,
        ) else {
            panic!("seed census");
        };
        assert_eq!(census.seeds().len(), 1);
        assert_eq!(census.inventory_digest(), inventory.digest());
        assert_eq!(
            census.inventory_compatibility_digest(),
            compatibility.digest()
        );
        let seed = &census.seeds()[0];
        assert_eq!(seed.origin_event(), &event);
        assert_eq!(seed.local_role(), LocalRoleV1::KernelHead);
        assert_eq!(
            seed.subject(),
            &PublicSemanticSeedSubjectV2::Declaration {
                declaration: head.clone(),
                public_group: GlobalId(Digest::of_bytes(b"clause-census/group")),
            }
        );
        assert_eq!(
            seed.source_clause(),
            clauses.clause_for_declaration(&head).unwrap()
        );
        assert_eq!(seed.public_support().declarations, BTreeSet::from([head]));
        assert!(seed.demand_anchor().is_none());
    }
}
