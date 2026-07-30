//! Replay-minted public inventory for the generic semantic-audit prototype.
//!
//! The verifier proves internal closure relative to the supplied event ledger
//! and kernel boundaries.  It deliberately does not assert that the ledger is
//! an issued Profile-A history; binding that external authority belongs in the
//! future profile adapter.

use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedSemanticAuditManifestV1,
};
use crate::model::{
    DemandOutputIdV1, EquationIdV1, EventIdV1, GenericJudgmentV1, PublicAvailabilityV1,
    SourceNormalizedJudgmentV1,
};
use crate::normalizer::verify_source_normalized_judgment_v1;
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, Digest, GlobalId, Kernel, KernelError, Term,
    UncheckedSignature, VerifiedSignature,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION: u16 = 1;
pub const ORIGIN_CUTOFF_Q3_SCHEMA_VERSION: u16 = 1;

macro_rules! digest_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(pub Digest);

        impl CanonicalEncode for $name {
            fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                self.0.encode_canonical(encoder);
            }
        }
    };
}

digest_id!(DemandFamilyIdV1);
digest_id!(DemandContractIdV1);

/// An exact demand output address.  Human-readable semantic labels are not
/// part of this key.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DemandPortKeyV1 {
    pub family: DemandFamilyIdV1,
    pub output: DemandOutputIdV1,
}

impl CanonicalEncode for DemandPortKeyV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.family.encode_canonical(encoder);
        self.output.encode_canonical(encoder);
    }
}

/// Exact additions attributed to one event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicEventCensusV1 {
    pub event: EventIdV1,
    pub added_groups: Vec<GlobalId>,
    pub added_declarations: Vec<GlobalId>,
    pub added_equations: Vec<EquationIdV1>,
    pub added_forced_projections: Vec<GlobalId>,
    pub added_demand_contracts: Vec<DemandContractIdV1>,
}

/// One predecessor event and its cumulative kernel boundary.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicHistoryStepV1 {
    pub census: UncheckedPublicEventCensusV1,
    pub successor_boundary: UncheckedSignature,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedSourceNormalizedDeclarationV1 {
    pub source_identity: Digest,
    pub source: Declaration,
    pub claimed_normalized: Declaration,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicGroupV1 {
    pub group: GlobalId,
    pub origin: EventIdV1,
    pub declarations: Vec<GlobalId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicDeclarationV1 {
    pub declaration: GlobalId,
    pub origin: EventIdV1,
    pub group: GlobalId,
    pub source_to_normal: UncheckedSourceNormalizedDeclarationV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicEquationV1 {
    pub equation: EquationIdV1,
    pub owner_head: GlobalId,
    pub origin: EventIdV1,
    pub source_to_normal: SourceNormalizedJudgmentV1,
    /// A strict-prior exact-key association only. It is not evidence that the
    /// equation realizes or discharges the schematic demand.
    pub demand_port: Option<DemandPortKeyV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPredecessorDemandContractV1 {
    pub contract: DemandContractIdV1,
    pub origin: EventIdV1,
    pub port: DemandPortKeyV1,
    pub required_judgment: SourceNormalizedJudgmentV1,
}

/// Ledger metadata for one forced-projection declaration.
///
/// Replay verifies census coverage, origin, exact declaration typing, owner
/// precedence, and that the declared type actually mentions the record owner.
/// It does not establish a projection reduction rule or record-elimination
/// theorem; those require the separate rewrite-system gate.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedForcedProjectionV1 {
    pub projection: GlobalId,
    pub record_owner: GlobalId,
    pub field_ordinal: u16,
    pub origin: EventIdV1,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PublicSubjectV1 {
    Declaration { declaration: GlobalId },
    Equation { equation: EquationIdV1 },
    DemandContract { contract: DemandContractIdV1 },
}

impl CanonicalEncode for PublicSubjectV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Declaration { declaration } => {
                encoder.tag(0);
                declaration.encode_canonical(encoder);
            }
            Self::Equation { equation } => {
                encoder.tag(1);
                equation.encode_canonical(encoder);
            }
            Self::DemandContract { contract } => {
                encoder.tag(2);
                contract.encode_canonical(encoder);
            }
        }
    }
}

/// A canonical semantic dependency use.  The verifier derives the complete
/// set from source and normalized syntax; serialized order has no meaning.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicDependencyUseV1 {
    pub dependent: PublicSubjectV1,
    pub prerequisite: GlobalId,
}

impl CanonicalEncode for PublicDependencyUseV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.dependent.encode_canonical(encoder);
        self.prerequisite.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicAvailabilityClaimV1 {
    pub dependency: PublicDependencyUseV1,
    pub claimed: PublicAvailabilityV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicDependencyDagV1 {
    pub edges: Vec<PublicDependencyUseV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OriginCutoffQ3EntryV1 {
    pub older_origin: EventIdV1,
    pub newer_origin: EventIdV1,
    pub identification: Digest,
}

impl CanonicalEncode for OriginCutoffQ3EntryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.older_origin.encode_canonical(encoder);
        self.newer_origin.encode_canonical(encoder);
        self.identification.encode_canonical(encoder);
    }
}

/// Q3 is not inferred from an omitted file.  A typed registry is supplied,
/// bound to the predecessor cutoff, and positively checked to contain zero
/// entries. This proves emptiness only of the supplied generic snapshot; an
/// adapter must separately bind that snapshot to an issued registry.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedOriginCutoffQ3RegistryV1 {
    pub schema_version: u16,
    pub origin_cutoff: Option<EventIdV1>,
    pub entries: Vec<OriginCutoffQ3EntryV1>,
}

/// Entire untrusted inventory wire.  No `complete` boolean or caller-provided
/// coverage digest is accepted.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedPublicAuditInventoryV1 {
    pub schema_version: u16,
    pub predecessor_history: Vec<UncheckedPublicHistoryStepV1>,
    pub predecessor_boundary: UncheckedSignature,
    pub successor_event: UncheckedPublicEventCensusV1,
    pub successor_boundary: UncheckedSignature,
    pub declaration_groups: Vec<UncheckedPublicGroupV1>,
    pub declarations: Vec<UncheckedPublicDeclarationV1>,
    pub equations: Vec<UncheckedPublicEquationV1>,
    pub forced_projections: Vec<UncheckedForcedProjectionV1>,
    pub predecessor_demand_contracts: Vec<UncheckedPredecessorDemandContractV1>,
    pub public_availability: Vec<UncheckedPublicAvailabilityClaimV1>,
    pub dependency_dag: UncheckedPublicDependencyDagV1,
    pub q3_registry: UncheckedOriginCutoffQ3RegistryV1,
}

/// Kernel-replayed strict append from predecessor to successor.
#[derive(Clone, Debug)]
pub struct VerifiedExactExtensionV1 {
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    new_declarations: Arc<[GlobalId]>,
    event: EventIdV1,
    digest: Digest,
}

impl VerifiedExactExtensionV1 {
    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn new_declarations(&self) -> &[GlobalId] {
        &self.new_declarations
    }

    pub fn event(&self) -> &EventIdV1 {
        &self.event
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedPublicGroupV1 {
    group: GlobalId,
    origin: EventIdV1,
    declarations: Arc<[GlobalId]>,
}

impl VerifiedPublicGroupV1 {
    pub fn group(&self) -> &GlobalId {
        &self.group
    }

    pub fn origin(&self) -> &EventIdV1 {
        &self.origin
    }

    pub fn declarations(&self) -> &[GlobalId] {
        &self.declarations
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedPublicDeclarationV1 {
    declaration: GlobalId,
    origin: EventIdV1,
    group: GlobalId,
    source_identity: Digest,
    source: Declaration,
    normalized: Declaration,
}

impl VerifiedPublicDeclarationV1 {
    pub fn declaration(&self) -> &GlobalId {
        &self.declaration
    }

    pub fn origin(&self) -> &EventIdV1 {
        &self.origin
    }

    pub fn group(&self) -> &GlobalId {
        &self.group
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn source(&self) -> &Declaration {
        &self.source
    }

    pub fn normalized(&self) -> &Declaration {
        &self.normalized
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedPublicEquationV1 {
    equation: EquationIdV1,
    owner_head: GlobalId,
    origin: EventIdV1,
    source_identity: Digest,
    source: GenericJudgmentV1,
    normalized: GenericJudgmentV1,
    demand_port: Option<DemandPortKeyV1>,
    predecessor_public: bool,
}

impl VerifiedPublicEquationV1 {
    pub fn equation(&self) -> &EquationIdV1 {
        &self.equation
    }

    pub fn owner_head(&self) -> &GlobalId {
        &self.owner_head
    }

    pub fn origin(&self) -> &EventIdV1 {
        &self.origin
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn source(&self) -> &GenericJudgmentV1 {
        &self.source
    }

    pub fn normalized(&self) -> &GenericJudgmentV1 {
        &self.normalized
    }

    /// Returns the verified strict-prior key association. This accessor does
    /// not confer a demand-specialization or discharge theorem.
    pub fn demand_port(&self) -> Option<&DemandPortKeyV1> {
        self.demand_port.as_ref()
    }

    pub fn is_predecessor_public(&self) -> bool {
        self.predecessor_public
    }
}

/// Verified census/type/origin facts for one ledger-listed projection.
///
/// This capability deliberately contains no left-hand side, right-hand side,
/// reduction witness, or rewrite-authority accessor. The field ordinal is
/// bound to the verified census but is not derived from a record-layout
/// theorem in this generic fragment.
#[derive(Clone, Debug)]
pub struct VerifiedForcedProjectionV1 {
    projection: GlobalId,
    record_owner: GlobalId,
    field_ordinal: u16,
    origin: EventIdV1,
    normalized_type: Term,
    descriptor_digest: Digest,
}

impl VerifiedForcedProjectionV1 {
    pub fn projection(&self) -> &GlobalId {
        &self.projection
    }

    pub fn record_owner(&self) -> &GlobalId {
        &self.record_owner
    }

    pub fn field_ordinal(&self) -> u16 {
        self.field_ordinal
    }

    pub fn origin(&self) -> &EventIdV1 {
        &self.origin
    }

    pub fn normalized_type(&self) -> &Term {
        &self.normalized_type
    }

    pub fn descriptor_digest(&self) -> &Digest {
        &self.descriptor_digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedDemandContractV1 {
    contract: DemandContractIdV1,
    origin: EventIdV1,
    port: DemandPortKeyV1,
    source_identity: Digest,
    source_requirement: GenericJudgmentV1,
    normalized_requirement: GenericJudgmentV1,
}

impl VerifiedDemandContractV1 {
    pub fn contract(&self) -> &DemandContractIdV1 {
        &self.contract
    }

    pub fn origin(&self) -> &EventIdV1 {
        &self.origin
    }

    pub fn port(&self) -> &DemandPortKeyV1 {
        &self.port
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn source_requirement(&self) -> &GenericJudgmentV1 {
        &self.source_requirement
    }

    pub fn normalized_requirement(&self) -> &GenericJudgmentV1 {
        &self.normalized_requirement
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedPublicDependencyDagV1 {
    edges: Arc<[PublicDependencyUseV1]>,
    digest: Digest,
}

impl VerifiedPublicDependencyDagV1 {
    pub fn edges(&self) -> &[PublicDependencyUseV1] {
        &self.edges
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedOriginCutoffQ3RegistryV1 {
    origin_cutoff: Option<EventIdV1>,
    predecessor_history_digest: Digest,
    digest: Digest,
}

impl VerifiedOriginCutoffQ3RegistryV1 {
    pub fn origin_cutoff(&self) -> Option<&EventIdV1> {
        self.origin_cutoff.as_ref()
    }

    pub fn predecessor_history_digest(&self) -> &Digest {
        &self.predecessor_history_digest
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedInventoryCoverageV1 {
    group_count: usize,
    declaration_count: usize,
    equation_count: usize,
    forced_projection_count: usize,
    predecessor_demand_contract_count: usize,
    dependency_count: usize,
    digest: Digest,
}

impl VerifiedInventoryCoverageV1 {
    pub fn group_count(&self) -> usize {
        self.group_count
    }

    pub fn declaration_count(&self) -> usize {
        self.declaration_count
    }

    pub fn equation_count(&self) -> usize {
        self.equation_count
    }

    pub fn forced_projection_count(&self) -> usize {
        self.forced_projection_count
    }

    pub fn predecessor_demand_contract_count(&self) -> usize {
        self.predecessor_demand_contract_count
    }

    pub fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Opaque proof capability minted only by [`verify_public_audit_inventory_v1`].
#[derive(Clone, Debug)]
pub struct VerifiedPublicAuditInventoryV1 {
    manifest_digest: Digest,
    normalizer_protocol_digest: Digest,
    predecessor_history_digest: Digest,
    predecessor_boundary: VerifiedSignature,
    successor_boundary: VerifiedSignature,
    exact_extension: VerifiedExactExtensionV1,
    declaration_groups: Arc<[VerifiedPublicGroupV1]>,
    declarations: Arc<[VerifiedPublicDeclarationV1]>,
    equations: Arc<[VerifiedPublicEquationV1]>,
    forced_projections: Arc<[VerifiedForcedProjectionV1]>,
    declaration_origins: BTreeMap<GlobalId, EventIdV1>,
    equation_origins: BTreeMap<EquationIdV1, EventIdV1>,
    forced_projection_origins: BTreeMap<GlobalId, EventIdV1>,
    predecessor_demand_contracts: Arc<[VerifiedDemandContractV1]>,
    public_availability: BTreeMap<PublicDependencyUseV1, PublicAvailabilityV1>,
    dependency_dag: VerifiedPublicDependencyDagV1,
    q3_registry: VerifiedOriginCutoffQ3RegistryV1,
    coverage: VerifiedInventoryCoverageV1,
    digest: Digest,
}

impl VerifiedPublicAuditInventoryV1 {
    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn predecessor_history_digest(&self) -> &Digest {
        &self.predecessor_history_digest
    }

    pub fn predecessor_boundary(&self) -> &VerifiedSignature {
        &self.predecessor_boundary
    }

    pub fn successor_boundary(&self) -> &VerifiedSignature {
        &self.successor_boundary
    }

    pub fn exact_extension(&self) -> &VerifiedExactExtensionV1 {
        &self.exact_extension
    }

    pub fn declaration_groups(&self) -> &[VerifiedPublicGroupV1] {
        &self.declaration_groups
    }

    pub fn declarations(&self) -> &[VerifiedPublicDeclarationV1] {
        &self.declarations
    }

    pub fn equations(&self) -> &[VerifiedPublicEquationV1] {
        &self.equations
    }

    pub fn forced_projections(&self) -> &[VerifiedForcedProjectionV1] {
        &self.forced_projections
    }

    pub fn forced_projection(&self, projection: &GlobalId) -> Option<&VerifiedForcedProjectionV1> {
        self.forced_projections
            .iter()
            .find(|candidate| candidate.projection() == projection)
    }

    pub fn declaration_origins(&self) -> &BTreeMap<GlobalId, EventIdV1> {
        &self.declaration_origins
    }

    pub fn equation_origins(&self) -> &BTreeMap<EquationIdV1, EventIdV1> {
        &self.equation_origins
    }

    pub fn forced_projection_origins(&self) -> &BTreeMap<GlobalId, EventIdV1> {
        &self.forced_projection_origins
    }

    pub fn predecessor_demand_contracts(&self) -> &[VerifiedDemandContractV1] {
        &self.predecessor_demand_contracts
    }

    /// Availability is returned only from the verifier-derived closure.
    pub fn availability_for(
        &self,
        dependency: &PublicDependencyUseV1,
    ) -> Option<&PublicAvailabilityV1> {
        self.public_availability.get(dependency)
    }

    pub fn dependency_dag(&self) -> &VerifiedPublicDependencyDagV1 {
        &self.dependency_dag
    }

    pub fn q3_registry(&self) -> &VerifiedOriginCutoffQ3RegistryV1 {
        &self.q3_registry
    }

    pub fn coverage(&self) -> &VerifiedInventoryCoverageV1 {
        &self.coverage
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn contains_predecessor_declaration(&self, declaration: &GlobalId) -> bool {
        self.predecessor_boundary
            .declarations()
            .iter()
            .any(|candidate| &candidate.id == declaration)
    }

    pub fn contains_successor_declaration(&self, declaration: &GlobalId) -> bool {
        self.declaration_origins.contains_key(declaration)
    }

    pub fn public_declaration(
        &self,
        declaration: &GlobalId,
    ) -> Option<&VerifiedPublicDeclarationV1> {
        self.declarations
            .iter()
            .find(|candidate| candidate.declaration() == declaration)
    }

    pub fn is_successor_new_declaration(&self, declaration: &GlobalId) -> bool {
        self.contains_successor_declaration(declaration)
            && !self.contains_predecessor_declaration(declaration)
    }

    pub fn declaration_availability(
        &self,
        dependent: &GlobalId,
        prerequisite: &GlobalId,
    ) -> Option<&PublicAvailabilityV1> {
        self.availability_for(&PublicDependencyUseV1 {
            dependent: PublicSubjectV1::Declaration {
                declaration: dependent.clone(),
            },
            prerequisite: prerequisite.clone(),
        })
    }

    /// Exact predecessor replay query for cost V2.  The query compares the
    /// kernel-replayed normal judgment, not a serialized availability label.
    /// Its authority is relative to this verified generic ledger snapshot;
    /// it does not establish that the snapshot is an issued Profile-A history.
    pub fn predecessor_public_equation_id(
        &self,
        normalized: &GenericJudgmentV1,
    ) -> Option<&EquationIdV1> {
        self.equations
            .iter()
            .find(|equation| equation.predecessor_public && equation.normalized == *normalized)
            .map(VerifiedPublicEquationV1::equation)
    }

    pub fn is_predecessor_public_equation(&self, normalized: &GenericJudgmentV1) -> bool {
        self.predecessor_public_equation_id(normalized).is_some()
    }

    pub fn public_equation_id(&self, normalized: &GenericJudgmentV1) -> Option<&EquationIdV1> {
        self.equations
            .iter()
            .find(|equation| equation.normalized == *normalized)
            .map(VerifiedPublicEquationV1::equation)
    }

    /// True only when the exact normalized equation is exhaustively covered
    /// by this inventory and all matching entries originate at the successor
    /// event rather than in the predecessor history.
    pub fn is_successor_new_equation(&self, normalized: &GenericJudgmentV1) -> bool {
        let mut matches = self
            .equations
            .iter()
            .filter(|equation| equation.normalized == *normalized);
        matches
            .next()
            .is_some_and(|equation| !equation.predecessor_public)
            && matches.all(|equation| !equation.predecessor_public)
    }
}

#[derive(Clone, Debug)]
struct ReplayedCensusV1 {
    event: EventIdV1,
    groups: BTreeSet<GlobalId>,
    declarations: BTreeSet<GlobalId>,
    equations: BTreeSet<EquationIdV1>,
    forced_projections: BTreeSet<GlobalId>,
    demand_contracts: BTreeSet<DemandContractIdV1>,
    boundary_digest: Digest,
}

impl CanonicalEncode for ReplayedCensusV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.event.encode_canonical(encoder);
        encode_set(encoder, &self.groups);
        encode_set(encoder, &self.declarations);
        encode_set(encoder, &self.equations);
        encode_set(encoder, &self.forced_projections);
        encode_set(encoder, &self.demand_contracts);
        self.boundary_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
enum InventoryFailure {
    Outside(OutsideFragmentReason),
    Unknown(AuditUnknownReason),
}

type InventoryResult<T> = Result<T, InventoryFailure>;

fn unknown(reason: AuditUnknownReason) -> InventoryFailure {
    InventoryFailure::Unknown(reason)
}

fn decision<T>(result: InventoryResult<T>) -> AuditDecision<T> {
    match result {
        Ok(value) => AuditDecision::Proven(value),
        Err(InventoryFailure::Outside(reason)) => AuditDecision::OutsideFragment(reason),
        Err(InventoryFailure::Unknown(reason)) => AuditDecision::Unknown(reason),
    }
}

fn kernel_failure(error: KernelError) -> InventoryFailure {
    match error {
        KernelError::ResourceExhausted(_) => unknown(AuditUnknownReason::ResourceExhausted),
        _ => unknown(AuditUnknownReason::KernelCouldNotCertify),
    }
}

/// Replay one internally closed generic snapshot.
///
/// Success does not authenticate the snapshot as an issued history. Consumers
/// needing that authority must compare the returned history and boundary
/// digests against independently verified adapter inputs.
pub fn verify_public_audit_inventory_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    wire: &UncheckedPublicAuditInventoryV1,
) -> AuditDecision<VerifiedPublicAuditInventoryV1> {
    decision(verify_inventory_inner(manifest, kernel, wire))
}

fn verify_inventory_inner(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    wire: &UncheckedPublicAuditInventoryV1,
) -> InventoryResult<VerifiedPublicAuditInventoryV1> {
    if wire.schema_version != PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION {
        return Err(unknown(AuditUnknownReason::MalformedInput));
    }
    check_resource_bounds(kernel, wire)?;

    let empty = kernel
        .verify_signature(&UncheckedSignature::default())
        .map_err(kernel_failure)?;
    let mut current = empty;
    let mut current_source = UncheckedSignature::default();
    let mut predecessor_steps = Vec::with_capacity(wire.predecessor_history.len());
    let mut event_boundaries = BTreeMap::new();
    let mut event_order = BTreeMap::new();
    let mut seen_events = BTreeSet::new();
    let mut group_origins = BTreeMap::new();
    let mut declaration_origins = BTreeMap::new();
    let mut equation_origins = BTreeMap::new();
    let mut forced_projection_origins = BTreeMap::new();
    let mut demand_origins = BTreeMap::new();

    for (ordinal, step) in wire.predecessor_history.iter().enumerate() {
        if !seen_events.insert(step.census.event.clone()) {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
        let census = replay_census(&step.census, Digest::of_bytes(b"pending"))?;
        let next = verify_exact_boundary_step(
            kernel,
            &current,
            &current_source,
            &step.successor_boundary,
            &census.declarations,
        )?;
        let replayed = ReplayedCensusV1 {
            boundary_digest: next.digest().clone(),
            ..census
        };
        insert_origins(
            &replayed,
            &mut group_origins,
            &mut declaration_origins,
            &mut equation_origins,
            &mut forced_projection_origins,
            &mut demand_origins,
        )?;
        event_order.insert(replayed.event.clone(), ordinal);
        event_boundaries.insert(replayed.event.clone(), next.clone());
        predecessor_steps.push(replayed);
        current = next;
        current_source = step.successor_boundary.clone();
    }

    let claimed_predecessor = kernel
        .verify_signature(&wire.predecessor_boundary)
        .map_err(kernel_failure)?;
    if current_source != wire.predecessor_boundary
        || current.digest() != claimed_predecessor.digest()
        || current.declarations() != claimed_predecessor.declarations()
    {
        return Err(unknown(AuditUnknownReason::ManifestMismatch));
    }
    let predecessor_boundary = claimed_predecessor;
    let predecessor_history_digest = Digest::of_canonical(
        "pen-semantic-audit/predecessor-history/v1",
        &CanonicalSlice(&predecessor_steps),
    );

    if !seen_events.insert(wire.successor_event.event.clone()) {
        return Err(unknown(AuditUnknownReason::ProvenanceCollision));
    }
    let candidate_census = replay_census(
        &wire.successor_event,
        Digest::of_bytes(b"successor-boundary-pending"),
    )?;
    if !candidate_census.demand_contracts.is_empty() {
        return Err(unknown(AuditUnknownReason::IncompleteSupport));
    }
    if candidate_census.groups.is_empty()
        && candidate_census.declarations.is_empty()
        && candidate_census.equations.is_empty()
    {
        return Err(unknown(AuditUnknownReason::MalformedInput));
    }
    let successor_boundary = verify_exact_boundary_step(
        kernel,
        &predecessor_boundary,
        &wire.predecessor_boundary,
        &wire.successor_boundary,
        &candidate_census.declarations,
    )?;
    let candidate_census = ReplayedCensusV1 {
        boundary_digest: successor_boundary.digest().clone(),
        ..candidate_census
    };
    insert_origins(
        &candidate_census,
        &mut group_origins,
        &mut declaration_origins,
        &mut equation_origins,
        &mut forced_projection_origins,
        &mut demand_origins,
    )?;
    let candidate_ordinal = predecessor_steps.len();
    event_order.insert(candidate_census.event.clone(), candidate_ordinal);
    event_boundaries.insert(candidate_census.event.clone(), successor_boundary.clone());

    let predecessor_declarations = predecessor_boundary
        .declarations()
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<BTreeSet<_>>();
    let all_declarations = successor_boundary
        .declarations()
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<BTreeSet<_>>();
    if declaration_origins.keys().cloned().collect::<BTreeSet<_>>() != all_declarations {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }
    let predecessor_census_declarations = predecessor_steps
        .iter()
        .flat_map(|step| step.declarations.iter().cloned())
        .collect::<BTreeSet<_>>();
    if predecessor_census_declarations != predecessor_declarations {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }

    let groups = verify_groups(
        &wire.declaration_groups,
        &group_origins,
        &declaration_origins,
    )?;
    let declarations = verify_declarations(
        &wire.declarations,
        &wire.successor_boundary,
        &successor_boundary,
        &declaration_origins,
        &groups,
    )?;
    let forced_projections = verify_forced_projections(
        &wire.forced_projections,
        &forced_projection_origins,
        &declaration_origins,
        &event_order,
        &event_boundaries,
        &successor_boundary,
        &declarations,
    )?;
    let demands = verify_demands(
        manifest,
        kernel,
        &wire.predecessor_demand_contracts,
        &demand_origins,
        &event_boundaries,
    )?;
    let demand_by_port = demands
        .iter()
        .map(|contract| (contract.port.clone(), contract))
        .collect::<BTreeMap<_, _>>();
    if demand_by_port.len() != demands.len() {
        return Err(unknown(AuditUnknownReason::ProvenanceCollision));
    }
    let equations = verify_equations(
        manifest,
        kernel,
        &wire.equations,
        &equation_origins,
        &event_boundaries,
        &event_order,
        &declaration_origins,
        &demand_by_port,
        candidate_ordinal,
    )?;

    let expected_dependencies = derive_dependency_dag(&declarations, &equations, &demands);
    let claimed_dependencies = unique_set(&wire.dependency_dag.edges)?;
    if claimed_dependencies != expected_dependencies {
        return Err(unknown(AuditUnknownReason::IncompleteSupport));
    }
    let dependency_edges = expected_dependencies.into_iter().collect::<Vec<_>>();
    let dependency_digest = Digest::of_canonical(
        "pen-semantic-audit/public-dependency-dag/v1",
        &CanonicalSlice(&dependency_edges),
    );
    let dependency_dag = VerifiedPublicDependencyDagV1 {
        edges: Arc::from(dependency_edges.clone()),
        digest: dependency_digest,
    };

    let public_availability = verify_availability(
        &wire.public_availability,
        &dependency_edges,
        &predecessor_declarations,
        &AvailabilityReplayV1 {
            all_declarations: &all_declarations,
            event_order: &event_order,
            declaration_origins: &declaration_origins,
            equation_origins: &equation_origins,
            demand_origins: &demand_origins,
            successor: &successor_boundary,
        },
    )?;

    let expected_cutoff = predecessor_steps.last().map(|step| step.event.clone());
    let q3_registry = verify_q3(
        &wire.q3_registry,
        expected_cutoff,
        &predecessor_history_digest,
    )?;
    let normalizer_protocol_digest = kernel.normalizer_protocol_digest();

    let new_declarations = successor_boundary.declarations()
        [predecessor_boundary.declarations().len()..]
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<Vec<_>>();
    let exact_extension_digest = Digest::of_canonical(
        "pen-semantic-audit/exact-extension/v1",
        &ExactExtensionMaterial {
            predecessor: predecessor_boundary.digest(),
            successor: successor_boundary.digest(),
            event: &candidate_census.event,
            declarations: &new_declarations,
            groups: &candidate_census.groups,
            equations: &candidate_census.equations,
            forced_projections: &candidate_census.forced_projections,
        },
    );
    let exact_extension = VerifiedExactExtensionV1 {
        predecessor_boundary_digest: predecessor_boundary.digest().clone(),
        successor_boundary_digest: successor_boundary.digest().clone(),
        new_declarations: Arc::from(new_declarations),
        event: candidate_census.event.clone(),
        digest: exact_extension_digest,
    };

    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/inventory-coverage/v1",
        &CoverageMaterial {
            normalizer_protocol: &normalizer_protocol_digest,
            history: &predecessor_history_digest,
            predecessor: predecessor_boundary.digest(),
            successor: successor_boundary.digest(),
            group_origins: &group_origins,
            declaration_origins: &declaration_origins,
            equation_origins: &equation_origins,
            forced_projection_origins: &forced_projection_origins,
            groups: &groups,
            declarations: &declarations,
            equations: &equations,
            forced_projections: &forced_projections,
            demands: &demands,
            demand_origins: &demand_origins,
            availability: &public_availability,
            dependencies: &dependency_edges,
            q3: q3_registry.digest(),
        },
    );
    let coverage = VerifiedInventoryCoverageV1 {
        group_count: groups.len(),
        declaration_count: declarations.len(),
        equation_count: equations.len(),
        forced_projection_count: forced_projections.len(),
        predecessor_demand_contract_count: demands.len(),
        dependency_count: dependency_edges.len(),
        digest: coverage_digest,
    };
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-public-inventory/v1",
        &InventoryDigestMaterial {
            manifest: manifest.candidate_digest(),
            normalizer_protocol: &normalizer_protocol_digest,
            history: &predecessor_history_digest,
            extension: exact_extension.digest(),
            groups: &groups,
            declarations: &declarations,
            equations: &equations,
            forced_projections: &forced_projections,
            demands: &demands,
            availability: &public_availability,
            dependency_dag: dependency_dag.digest(),
            q3: q3_registry.digest(),
            coverage: coverage.digest(),
        },
    );

    Ok(VerifiedPublicAuditInventoryV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        normalizer_protocol_digest,
        predecessor_history_digest,
        predecessor_boundary,
        successor_boundary,
        exact_extension,
        declaration_groups: Arc::from(groups),
        declarations: Arc::from(declarations),
        equations: Arc::from(equations),
        forced_projections: Arc::from(forced_projections),
        declaration_origins,
        equation_origins,
        forced_projection_origins,
        predecessor_demand_contracts: Arc::from(demands),
        public_availability,
        dependency_dag,
        q3_registry,
        coverage,
        digest,
    })
}

fn check_resource_bounds(
    kernel: &Kernel,
    wire: &UncheckedPublicAuditInventoryV1,
) -> InventoryResult<()> {
    let limit = kernel.limits().max_operations as usize;
    let mut total = wire
        .predecessor_history
        .len()
        .checked_add(wire.declaration_groups.len())
        .and_then(|value| value.checked_add(wire.declarations.len()))
        .and_then(|value| value.checked_add(wire.equations.len()))
        .and_then(|value| value.checked_add(wire.forced_projections.len()))
        .and_then(|value| value.checked_add(wire.predecessor_demand_contracts.len()))
        .and_then(|value| value.checked_add(wire.public_availability.len()))
        .and_then(|value| value.checked_add(wire.dependency_dag.edges.len()))
        .and_then(|value| value.checked_add(wire.q3_registry.entries.len()))
        .ok_or_else(|| unknown(AuditUnknownReason::ResourceExhausted))?;
    for step in &wire.predecessor_history {
        total = total
            .checked_add(step.successor_boundary.declarations.len())
            .and_then(|value| value.checked_add(step.census.added_groups.len()))
            .and_then(|value| value.checked_add(step.census.added_declarations.len()))
            .and_then(|value| value.checked_add(step.census.added_equations.len()))
            .and_then(|value| value.checked_add(step.census.added_forced_projections.len()))
            .and_then(|value| value.checked_add(step.census.added_demand_contracts.len()))
            .ok_or_else(|| unknown(AuditUnknownReason::ResourceExhausted))?;
    }
    total = total
        .checked_add(wire.successor_boundary.declarations.len())
        .and_then(|value| value.checked_add(wire.successor_event.added_groups.len()))
        .and_then(|value| value.checked_add(wire.successor_event.added_declarations.len()))
        .and_then(|value| value.checked_add(wire.successor_event.added_equations.len()))
        .and_then(|value| value.checked_add(wire.successor_event.added_forced_projections.len()))
        .and_then(|value| value.checked_add(wire.successor_event.added_demand_contracts.len()))
        .ok_or_else(|| unknown(AuditUnknownReason::ResourceExhausted))?;
    for group in &wire.declaration_groups {
        total = total
            .checked_add(group.declarations.len())
            .ok_or_else(|| unknown(AuditUnknownReason::ResourceExhausted))?;
    }
    if total > limit {
        return Err(unknown(AuditUnknownReason::ResourceExhausted));
    }
    Ok(())
}

fn replay_census(
    wire: &UncheckedPublicEventCensusV1,
    boundary_digest: Digest,
) -> InventoryResult<ReplayedCensusV1> {
    Ok(ReplayedCensusV1 {
        event: wire.event.clone(),
        groups: unique_set(&wire.added_groups)?,
        declarations: unique_set(&wire.added_declarations)?,
        equations: unique_set(&wire.added_equations)?,
        forced_projections: unique_set(&wire.added_forced_projections)?,
        demand_contracts: unique_set(&wire.added_demand_contracts)?,
        boundary_digest,
    })
}

fn verify_exact_boundary_step(
    kernel: &Kernel,
    predecessor: &VerifiedSignature,
    predecessor_source: &UncheckedSignature,
    successor_wire: &UncheckedSignature,
    claimed_new_declarations: &BTreeSet<GlobalId>,
) -> InventoryResult<VerifiedSignature> {
    let successor = kernel
        .verify_signature(successor_wire)
        .map_err(kernel_failure)?;
    let predecessor_len = predecessor.declarations().len();
    if successor.declarations().len() < predecessor_len
        || successor.declarations()[..predecessor_len] != *predecessor.declarations()
        || successor_wire.declarations.len() < predecessor_len
        || successor_wire.declarations[..predecessor_len] != predecessor_source.declarations
    {
        return Err(unknown(AuditUnknownReason::FailedRetraction));
    }
    let extension_wire = UncheckedSignature {
        declarations: successor_wire.declarations[predecessor_len..].to_vec(),
    };
    let replayed = kernel
        .verify_extension(predecessor, &extension_wire)
        .map_err(kernel_failure)?;
    if replayed.digest() != successor.digest()
        || replayed.declarations() != successor.declarations()
    {
        return Err(unknown(AuditUnknownReason::FailedRetraction));
    }
    let actual_new = successor.declarations()[predecessor_len..]
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<BTreeSet<_>>();
    if &actual_new != claimed_new_declarations {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }
    Ok(successor)
}

fn insert_origins(
    census: &ReplayedCensusV1,
    groups: &mut BTreeMap<GlobalId, EventIdV1>,
    declarations: &mut BTreeMap<GlobalId, EventIdV1>,
    equations: &mut BTreeMap<EquationIdV1, EventIdV1>,
    forced_projections: &mut BTreeMap<GlobalId, EventIdV1>,
    demands: &mut BTreeMap<DemandContractIdV1, EventIdV1>,
) -> InventoryResult<()> {
    for group in &census.groups {
        if groups.insert(group.clone(), census.event.clone()).is_some() {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    for declaration in &census.declarations {
        if declarations
            .insert(declaration.clone(), census.event.clone())
            .is_some()
        {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    for equation in &census.equations {
        if equations
            .insert(equation.clone(), census.event.clone())
            .is_some()
        {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    for projection in &census.forced_projections {
        if forced_projections
            .insert(projection.clone(), census.event.clone())
            .is_some()
        {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    for demand in &census.demand_contracts {
        if demands
            .insert(demand.clone(), census.event.clone())
            .is_some()
        {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    Ok(())
}

fn verify_groups(
    wire: &[UncheckedPublicGroupV1],
    origins: &BTreeMap<GlobalId, EventIdV1>,
    declaration_origins: &BTreeMap<GlobalId, EventIdV1>,
) -> InventoryResult<Vec<VerifiedPublicGroupV1>> {
    let mut by_id = BTreeMap::new();
    let mut covered = BTreeSet::new();
    for group in wire {
        let Some(expected_origin) = origins.get(&group.group) else {
            return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
        };
        if expected_origin != &group.origin || group.declarations.is_empty() {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
        let declarations = unique_set(&group.declarations)?;
        for declaration in &declarations {
            if declaration_origins.get(declaration) != Some(&group.origin)
                || !covered.insert(declaration.clone())
            {
                return Err(unknown(AuditUnknownReason::ProvenanceCollision));
            }
        }
        let verified = VerifiedPublicGroupV1 {
            group: group.group.clone(),
            origin: group.origin.clone(),
            declarations: Arc::from(declarations.into_iter().collect::<Vec<_>>()),
        };
        if by_id.insert(group.group.clone(), verified).is_some() {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    if by_id.keys().cloned().collect::<BTreeSet<_>>() != origins.keys().cloned().collect()
        || covered != declaration_origins.keys().cloned().collect()
    {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }
    Ok(by_id.into_values().collect())
}

fn verify_declarations(
    wire: &[UncheckedPublicDeclarationV1],
    successor_source: &UncheckedSignature,
    successor: &VerifiedSignature,
    origins: &BTreeMap<GlobalId, EventIdV1>,
    groups: &[VerifiedPublicGroupV1],
) -> InventoryResult<Vec<VerifiedPublicDeclarationV1>> {
    let group_for_declaration = groups
        .iter()
        .flat_map(|group| {
            group
                .declarations
                .iter()
                .map(move |declaration| (declaration.clone(), group.group.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    let input = wire
        .iter()
        .map(|declaration| (declaration.declaration.clone(), declaration))
        .collect::<BTreeMap<_, _>>();
    if input.len() != wire.len()
        || input.keys().cloned().collect::<BTreeSet<_>>() != origins.keys().cloned().collect()
    {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }

    let source_by_id = successor_source
        .declarations
        .iter()
        .map(|declaration| (declaration.id.clone(), declaration))
        .collect::<BTreeMap<_, _>>();
    let mut verified = Vec::with_capacity(successor.declarations().len());
    for normalized in successor.declarations() {
        let candidate = input
            .get(&normalized.id)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteEnumeration))?;
        let source = source_by_id
            .get(&normalized.id)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteEnumeration))?;
        let expected_source_identity = Digest::of_canonical(
            "pen-semantic-audit/inventory-source-declaration/v1",
            *source,
        );
        let expected_origin = origins
            .get(&normalized.id)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteEnumeration))?;
        let expected_group = group_for_declaration
            .get(&normalized.id)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteEnumeration))?;
        if &candidate.origin != expected_origin
            || &candidate.group != expected_group
            || candidate.source_to_normal.source != **source
            || candidate.source_to_normal.claimed_normalized != *normalized
            || candidate.source_to_normal.source_identity != expected_source_identity
            || candidate.declaration != normalized.id
        {
            return Err(unknown(AuditUnknownReason::NormalizationFailure));
        }
        verified.push(VerifiedPublicDeclarationV1 {
            declaration: normalized.id.clone(),
            origin: candidate.origin.clone(),
            group: candidate.group.clone(),
            source_identity: expected_source_identity,
            source: (*source).clone(),
            normalized: normalized.clone(),
        });
    }
    Ok(verified)
}

fn verify_forced_projections(
    wire: &[UncheckedForcedProjectionV1],
    origins: &BTreeMap<GlobalId, EventIdV1>,
    declaration_origins: &BTreeMap<GlobalId, EventIdV1>,
    event_order: &BTreeMap<EventIdV1, usize>,
    event_boundaries: &BTreeMap<EventIdV1, VerifiedSignature>,
    successor: &VerifiedSignature,
    declarations: &[VerifiedPublicDeclarationV1],
) -> InventoryResult<Vec<VerifiedForcedProjectionV1>> {
    let input = wire
        .iter()
        .map(|projection| (projection.projection.clone(), projection))
        .collect::<BTreeMap<_, _>>();
    if input.len() != wire.len()
        || input.keys().cloned().collect::<BTreeSet<_>>() != origins.keys().cloned().collect()
    {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }
    let declarations = declarations
        .iter()
        .map(|declaration| (declaration.declaration.clone(), declaration))
        .collect::<BTreeMap<_, _>>();
    let declaration_order = successor
        .declarations()
        .iter()
        .enumerate()
        .map(|(ordinal, declaration)| (declaration.id.clone(), ordinal))
        .collect::<BTreeMap<_, _>>();
    let mut slots = BTreeSet::new();
    let mut verified = Vec::with_capacity(input.len());
    for (projection_id, projection) in input {
        let expected_origin = origins
            .get(&projection_id)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteEnumeration))?;
        let declaration = declarations
            .get(&projection_id)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteEnumeration))?;
        if &projection.origin != expected_origin
            || declaration.origin() != expected_origin
            || !slots.insert((projection.record_owner.clone(), projection.field_ordinal))
        {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
        let boundary = event_boundaries
            .get(expected_origin)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
        if !boundary
            .declarations()
            .iter()
            .any(|candidate| candidate == declaration.normalized())
        {
            return Err(unknown(AuditUnknownReason::NormalizationFailure));
        }
        let owner_position = declaration_position(
            &projection.record_owner,
            event_order,
            declaration_origins,
            &declaration_order,
        )?;
        let projection_position = declaration_position(
            &projection.projection,
            event_order,
            declaration_origins,
            &declaration_order,
        )?;
        if owner_position >= projection_position {
            return Err(unknown(AuditUnknownReason::IncompleteSupport));
        }
        let mut type_dependencies = BTreeSet::new();
        collect_term_globals(&declaration.source().ty, &mut type_dependencies);
        collect_term_globals(&declaration.normalized().ty, &mut type_dependencies);
        if !type_dependencies.contains(&projection.record_owner) {
            return Err(unknown(AuditUnknownReason::IncompleteSupport));
        }
        let descriptor_digest = Digest::of_canonical(
            "pen-semantic-audit/forced-projection-census/v1",
            &ForcedProjectionMaterial {
                projection: &projection.projection,
                record_owner: &projection.record_owner,
                field_ordinal: projection.field_ordinal,
                origin: &projection.origin,
                normalized_type: &declaration.normalized().ty,
            },
        );
        verified.push(VerifiedForcedProjectionV1 {
            projection: projection.projection.clone(),
            record_owner: projection.record_owner.clone(),
            field_ordinal: projection.field_ordinal,
            origin: projection.origin.clone(),
            normalized_type: declaration.normalized().ty.clone(),
            descriptor_digest,
        });
    }
    Ok(verified)
}

fn verify_demands(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    wire: &[UncheckedPredecessorDemandContractV1],
    origins: &BTreeMap<DemandContractIdV1, EventIdV1>,
    event_boundaries: &BTreeMap<EventIdV1, VerifiedSignature>,
) -> InventoryResult<Vec<VerifiedDemandContractV1>> {
    let mut by_id = BTreeMap::new();
    let mut ports = BTreeSet::new();
    for demand in wire {
        if origins.get(&demand.contract) != Some(&demand.origin)
            || !ports.insert(demand.port.clone())
        {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
        let boundary = event_boundaries
            .get(&demand.origin)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
        verify_source_identity(&demand.required_judgment)?;
        let normalized = require_proven(verify_source_normalized_judgment_v1(
            manifest,
            kernel,
            boundary,
            &demand.required_judgment,
            None,
        ))?;
        let verified = VerifiedDemandContractV1 {
            contract: demand.contract.clone(),
            origin: demand.origin.clone(),
            port: demand.port.clone(),
            source_identity: demand.required_judgment.source_identity.clone(),
            source_requirement: demand.required_judgment.source.clone(),
            normalized_requirement: normalized.normalized().clone(),
        };
        if by_id.insert(demand.contract.clone(), verified).is_some() {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    if by_id.keys().cloned().collect::<BTreeSet<_>>() != origins.keys().cloned().collect() {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }
    Ok(by_id.into_values().collect())
}

#[allow(clippy::too_many_arguments)]
fn verify_equations(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    wire: &[UncheckedPublicEquationV1],
    origins: &BTreeMap<EquationIdV1, EventIdV1>,
    event_boundaries: &BTreeMap<EventIdV1, VerifiedSignature>,
    event_order: &BTreeMap<EventIdV1, usize>,
    declaration_origins: &BTreeMap<GlobalId, EventIdV1>,
    demand_by_port: &BTreeMap<DemandPortKeyV1, &VerifiedDemandContractV1>,
    candidate_ordinal: usize,
) -> InventoryResult<Vec<VerifiedPublicEquationV1>> {
    let mut by_id = BTreeMap::new();
    for equation in wire {
        if origins.get(&equation.equation) != Some(&equation.origin) {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
        let boundary = event_boundaries
            .get(&equation.origin)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
        if !boundary
            .declarations()
            .iter()
            .any(|declaration| declaration.id == equation.owner_head)
            || !declaration_origins.contains_key(&equation.owner_head)
        {
            return Err(unknown(AuditUnknownReason::IncompleteSupport));
        }
        if let Some(port) = &equation.demand_port {
            let contract = demand_by_port
                .get(port)
                .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
            let contract_ordinal = event_order
                .get(contract.origin())
                .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
            let equation_ordinal = event_order
                .get(&equation.origin)
                .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
            if contract_ordinal >= equation_ordinal {
                return Err(unknown(AuditUnknownReason::IncompleteSupport));
            }
        }
        verify_source_identity(&equation.source_to_normal)?;
        let normalized = require_proven(verify_source_normalized_judgment_v1(
            manifest,
            kernel,
            boundary,
            &equation.source_to_normal,
            None,
        ))?;
        if !matches!(normalized.normalized(), GenericJudgmentV1::Equation { .. }) {
            return Err(unknown(AuditUnknownReason::MalformedInput));
        }
        let predecessor_public = *event_order
            .get(&equation.origin)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?
            < candidate_ordinal;
        let verified = VerifiedPublicEquationV1 {
            equation: equation.equation.clone(),
            owner_head: equation.owner_head.clone(),
            origin: equation.origin.clone(),
            source_identity: equation.source_to_normal.source_identity.clone(),
            source: equation.source_to_normal.source.clone(),
            normalized: normalized.normalized().clone(),
            demand_port: equation.demand_port.clone(),
            predecessor_public,
        };
        if by_id.insert(equation.equation.clone(), verified).is_some() {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    if by_id.keys().cloned().collect::<BTreeSet<_>>() != origins.keys().cloned().collect() {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }
    Ok(by_id.into_values().collect())
}

fn verify_source_identity(pair: &SourceNormalizedJudgmentV1) -> InventoryResult<()> {
    let expected = Digest::of_canonical(
        "pen-semantic-audit/inventory-source-judgment/v1",
        &pair.source,
    );
    if pair.source_identity != expected {
        return Err(unknown(AuditUnknownReason::NormalizationFailure));
    }
    Ok(())
}

fn require_proven<T>(decision: AuditDecision<T>) -> InventoryResult<T> {
    match decision {
        AuditDecision::Proven(value) => Ok(value),
        AuditDecision::OutsideFragment(reason) => Err(InventoryFailure::Outside(reason)),
        AuditDecision::Unknown(reason) => Err(InventoryFailure::Unknown(reason)),
    }
}

fn derive_dependency_dag(
    declarations: &[VerifiedPublicDeclarationV1],
    equations: &[VerifiedPublicEquationV1],
    demands: &[VerifiedDemandContractV1],
) -> BTreeSet<PublicDependencyUseV1> {
    let mut edges = BTreeSet::new();
    for declaration in declarations {
        let mut dependencies = BTreeSet::new();
        collect_declaration_globals(&declaration.source, &mut dependencies);
        collect_declaration_globals(&declaration.normalized, &mut dependencies);
        for prerequisite in dependencies {
            edges.insert(PublicDependencyUseV1 {
                dependent: PublicSubjectV1::Declaration {
                    declaration: declaration.declaration.clone(),
                },
                prerequisite,
            });
        }
    }
    for equation in equations {
        let mut dependencies = BTreeSet::new();
        collect_judgment_globals(&equation.source, &mut dependencies);
        collect_judgment_globals(&equation.normalized, &mut dependencies);
        dependencies.insert(equation.owner_head.clone());
        for prerequisite in dependencies {
            edges.insert(PublicDependencyUseV1 {
                dependent: PublicSubjectV1::Equation {
                    equation: equation.equation.clone(),
                },
                prerequisite,
            });
        }
    }
    for demand in demands {
        let mut dependencies = BTreeSet::new();
        collect_judgment_globals(&demand.source_requirement, &mut dependencies);
        collect_judgment_globals(&demand.normalized_requirement, &mut dependencies);
        for prerequisite in dependencies {
            edges.insert(PublicDependencyUseV1 {
                dependent: PublicSubjectV1::DemandContract {
                    contract: demand.contract.clone(),
                },
                prerequisite,
            });
        }
    }
    edges
}

struct AvailabilityReplayV1<'a> {
    all_declarations: &'a BTreeSet<GlobalId>,
    event_order: &'a BTreeMap<EventIdV1, usize>,
    declaration_origins: &'a BTreeMap<GlobalId, EventIdV1>,
    equation_origins: &'a BTreeMap<EquationIdV1, EventIdV1>,
    demand_origins: &'a BTreeMap<DemandContractIdV1, EventIdV1>,
    successor: &'a VerifiedSignature,
}

fn verify_availability(
    wire: &[UncheckedPublicAvailabilityClaimV1],
    dependencies: &[PublicDependencyUseV1],
    predecessor_declarations: &BTreeSet<GlobalId>,
    replay: &AvailabilityReplayV1<'_>,
) -> InventoryResult<BTreeMap<PublicDependencyUseV1, PublicAvailabilityV1>> {
    let mut claimed = BTreeMap::new();
    for entry in wire {
        if claimed
            .insert(entry.dependency.clone(), entry.claimed.clone())
            .is_some()
        {
            return Err(unknown(AuditUnknownReason::ProvenanceCollision));
        }
    }
    if claimed.len() != dependencies.len() {
        return Err(unknown(AuditUnknownReason::IncompleteEnumeration));
    }
    let declaration_order = replay
        .successor
        .declarations()
        .iter()
        .enumerate()
        .map(|(ordinal, declaration)| (declaration.id.clone(), ordinal))
        .collect::<BTreeMap<_, _>>();
    let mut verified = BTreeMap::new();
    for dependency in dependencies {
        if !replay.all_declarations.contains(&dependency.prerequisite) {
            return Err(unknown(AuditUnknownReason::IncompleteSupport));
        }
        let prerequisite_position = declaration_position(
            &dependency.prerequisite,
            replay.event_order,
            replay.declaration_origins,
            &declaration_order,
        )?;
        let dependent_position = subject_position(
            &dependency.dependent,
            replay.event_order,
            replay.declaration_origins,
            replay.equation_origins,
            replay.demand_origins,
            &declaration_order,
        )?;
        if prerequisite_position >= dependent_position {
            return Err(unknown(AuditUnknownReason::IncompleteSupport));
        }
        let expected = if predecessor_declarations.contains(&dependency.prerequisite) {
            PublicAvailabilityV1::PredecessorPublicExport {
                target: dependency.prerequisite.clone(),
            }
        } else {
            PublicAvailabilityV1::DependencyPriorExport {
                target: dependency.prerequisite.clone(),
            }
        };
        if claimed.get(dependency) != Some(&expected) {
            return Err(unknown(AuditUnknownReason::IncompleteSupport));
        }
        verified.insert(dependency.clone(), expected);
    }
    Ok(verified)
}

/// `(event, phase, within-boundary)` is a verifier-derived precedence key.
/// Declarations are phase zero; sealed equations and demand contracts are
/// available only after all declarations attributed to that event.
fn declaration_position(
    declaration: &GlobalId,
    event_order: &BTreeMap<EventIdV1, usize>,
    declaration_origins: &BTreeMap<GlobalId, EventIdV1>,
    declaration_order: &BTreeMap<GlobalId, usize>,
) -> InventoryResult<(usize, u8, usize)> {
    let origin = declaration_origins
        .get(declaration)
        .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
    Ok((
        *event_order
            .get(origin)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?,
        0,
        *declaration_order
            .get(declaration)
            .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?,
    ))
}

fn subject_position(
    subject: &PublicSubjectV1,
    event_order: &BTreeMap<EventIdV1, usize>,
    declaration_origins: &BTreeMap<GlobalId, EventIdV1>,
    equation_origins: &BTreeMap<EquationIdV1, EventIdV1>,
    demand_origins: &BTreeMap<DemandContractIdV1, EventIdV1>,
    declaration_order: &BTreeMap<GlobalId, usize>,
) -> InventoryResult<(usize, u8, usize)> {
    match subject {
        PublicSubjectV1::Declaration { declaration } => declaration_position(
            declaration,
            event_order,
            declaration_origins,
            declaration_order,
        ),
        PublicSubjectV1::Equation { equation } => {
            let origin = equation_origins
                .get(equation)
                .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
            Ok((
                *event_order
                    .get(origin)
                    .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?,
                1,
                0,
            ))
        }
        PublicSubjectV1::DemandContract { contract } => {
            let origin = demand_origins
                .get(contract)
                .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
            Ok((
                *event_order
                    .get(origin)
                    .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?,
                1,
                0,
            ))
        }
    }
}

fn verify_q3(
    wire: &UncheckedOriginCutoffQ3RegistryV1,
    expected_cutoff: Option<EventIdV1>,
    history_digest: &Digest,
) -> InventoryResult<VerifiedOriginCutoffQ3RegistryV1> {
    if wire.schema_version != ORIGIN_CUTOFF_Q3_SCHEMA_VERSION
        || wire.origin_cutoff != expected_cutoff
    {
        return Err(unknown(AuditUnknownReason::ManifestMismatch));
    }
    if !wire.entries.is_empty() {
        return Err(InventoryFailure::Outside(
            OutsideFragmentReason::NonEmptyQ3Registry,
        ));
    }
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-empty-origin-cutoff-q3/v1",
        &Q3Material {
            cutoff: &wire.origin_cutoff,
            history: history_digest,
        },
    );
    Ok(VerifiedOriginCutoffQ3RegistryV1 {
        origin_cutoff: wire.origin_cutoff.clone(),
        predecessor_history_digest: history_digest.clone(),
        digest,
    })
}

fn collect_declaration_globals(declaration: &Declaration, globals: &mut BTreeSet<GlobalId>) {
    collect_term_globals(&declaration.ty, globals);
    if let Some(body) = &declaration.body {
        collect_term_globals(body, globals);
    }
}

fn collect_judgment_globals(judgment: &GenericJudgmentV1, globals: &mut BTreeSet<GlobalId>) {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            for entry in &context.0 {
                collect_term_globals(entry, globals);
            }
            collect_term_globals(term, globals);
            collect_term_globals(ty, globals);
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            for entry in &context.0 {
                collect_term_globals(entry, globals);
            }
            collect_term_globals(left, globals);
            collect_term_globals(right, globals);
            collect_term_globals(ty, globals);
        }
    }
}

fn collect_term_globals(term: &Term, globals: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Global { id } => {
            globals.insert(id.clone());
        }
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            collect_term_globals(parameter, globals);
            collect_term_globals(body, globals);
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            collect_term_globals(parameter_type, globals);
            collect_term_globals(body, globals);
        }
        Term::Apply { function, argument } => {
            collect_term_globals(function, globals);
            collect_term_globals(argument, globals);
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            collect_term_globals(sigma_type, globals);
            collect_term_globals(first, globals);
            collect_term_globals(second, globals);
        }
        Term::First { pair } | Term::Second { pair } => collect_term_globals(pair, globals),
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => {}
    }
}

fn unique_set<T: Clone + Ord>(values: &[T]) -> InventoryResult<BTreeSet<T>> {
    let set = values.iter().cloned().collect::<BTreeSet<_>>();
    if set.len() != values.len() {
        return Err(unknown(AuditUnknownReason::ProvenanceCollision));
    }
    Ok(set)
}

fn encode_set<T: CanonicalEncode>(encoder: &mut CanonicalEncoder, values: &BTreeSet<T>) {
    encoder.u64(values.len() as u64);
    for value in values {
        value.encode_canonical(encoder);
    }
}

fn encode_map<K: CanonicalEncode, V: CanonicalEncode>(
    encoder: &mut CanonicalEncoder,
    values: &BTreeMap<K, V>,
) {
    encoder.u64(values.len() as u64);
    for (key, value) in values {
        key.encode_canonical(encoder);
        value.encode_canonical(encoder);
    }
}

struct CanonicalSlice<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSlice<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct ExactExtensionMaterial<'a> {
    predecessor: &'a Digest,
    successor: &'a Digest,
    event: &'a EventIdV1,
    declarations: &'a [GlobalId],
    groups: &'a BTreeSet<GlobalId>,
    equations: &'a BTreeSet<EquationIdV1>,
    forced_projections: &'a BTreeSet<GlobalId>,
}

impl CanonicalEncode for ExactExtensionMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.predecessor.encode_canonical(encoder);
        self.successor.encode_canonical(encoder);
        self.event.encode_canonical(encoder);
        encoder.sequence(self.declarations);
        encode_set(encoder, self.groups);
        encode_set(encoder, self.equations);
        encode_set(encoder, self.forced_projections);
    }
}

struct ForcedProjectionMaterial<'a> {
    projection: &'a GlobalId,
    record_owner: &'a GlobalId,
    field_ordinal: u16,
    origin: &'a EventIdV1,
    normalized_type: &'a Term,
}

impl CanonicalEncode for ForcedProjectionMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.projection.encode_canonical(encoder);
        self.record_owner.encode_canonical(encoder);
        encoder.u16(self.field_ordinal);
        self.origin.encode_canonical(encoder);
        self.normalized_type.encode_canonical(encoder);
    }
}

struct CoverageMaterial<'a> {
    normalizer_protocol: &'a Digest,
    history: &'a Digest,
    predecessor: &'a Digest,
    successor: &'a Digest,
    group_origins: &'a BTreeMap<GlobalId, EventIdV1>,
    declaration_origins: &'a BTreeMap<GlobalId, EventIdV1>,
    equation_origins: &'a BTreeMap<EquationIdV1, EventIdV1>,
    forced_projection_origins: &'a BTreeMap<GlobalId, EventIdV1>,
    demand_origins: &'a BTreeMap<DemandContractIdV1, EventIdV1>,
    groups: &'a [VerifiedPublicGroupV1],
    declarations: &'a [VerifiedPublicDeclarationV1],
    equations: &'a [VerifiedPublicEquationV1],
    forced_projections: &'a [VerifiedForcedProjectionV1],
    demands: &'a [VerifiedDemandContractV1],
    availability: &'a BTreeMap<PublicDependencyUseV1, PublicAvailabilityV1>,
    dependencies: &'a [PublicDependencyUseV1],
    q3: &'a Digest,
}

impl CanonicalEncode for CoverageMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.normalizer_protocol.encode_canonical(encoder);
        self.history.encode_canonical(encoder);
        self.predecessor.encode_canonical(encoder);
        self.successor.encode_canonical(encoder);
        encode_map(encoder, self.group_origins);
        encode_map(encoder, self.declaration_origins);
        encode_map(encoder, self.equation_origins);
        encode_map(encoder, self.forced_projection_origins);
        encode_map(encoder, self.demand_origins);
        encoder.u64(self.groups.len() as u64);
        encoder.sequence(self.groups);
        encoder.u64(self.declarations.len() as u64);
        encoder.sequence(self.declarations);
        encoder.u64(self.equations.len() as u64);
        encoder.sequence(self.equations);
        encoder.u64(self.forced_projections.len() as u64);
        encoder.sequence(self.forced_projections);
        encoder.u64(self.demands.len() as u64);
        encoder.sequence(self.demands);
        encoder.u64(self.availability.len() as u64);
        encode_map(encoder, self.availability);
        encoder.u64(self.dependencies.len() as u64);
        encoder.sequence(self.dependencies);
        self.q3.encode_canonical(encoder);
    }
}

struct Q3Material<'a> {
    cutoff: &'a Option<EventIdV1>,
    history: &'a Digest,
}

impl CanonicalEncode for Q3Material<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.option(self.cutoff);
        self.history.encode_canonical(encoder);
    }
}

impl CanonicalEncode for VerifiedPublicGroupV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.group.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        encoder.sequence(&self.declarations);
    }
}

impl CanonicalEncode for VerifiedPublicDeclarationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.declaration.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.group.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.source.encode_canonical(encoder);
        self.normalized.encode_canonical(encoder);
    }
}

impl CanonicalEncode for VerifiedPublicEquationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        self.owner_head.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.source.encode_canonical(encoder);
        self.normalized.encode_canonical(encoder);
        encoder.option(&self.demand_port);
        encoder.tag(u8::from(self.predecessor_public));
    }
}

impl CanonicalEncode for VerifiedForcedProjectionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.projection.encode_canonical(encoder);
        self.record_owner.encode_canonical(encoder);
        encoder.u16(self.field_ordinal);
        self.origin.encode_canonical(encoder);
        self.normalized_type.encode_canonical(encoder);
        self.descriptor_digest.encode_canonical(encoder);
    }
}

impl CanonicalEncode for VerifiedDemandContractV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.contract.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.source_requirement.encode_canonical(encoder);
        self.normalized_requirement.encode_canonical(encoder);
    }
}

struct InventoryDigestMaterial<'a> {
    manifest: &'a Digest,
    normalizer_protocol: &'a Digest,
    history: &'a Digest,
    extension: &'a Digest,
    groups: &'a [VerifiedPublicGroupV1],
    declarations: &'a [VerifiedPublicDeclarationV1],
    equations: &'a [VerifiedPublicEquationV1],
    forced_projections: &'a [VerifiedForcedProjectionV1],
    demands: &'a [VerifiedDemandContractV1],
    availability: &'a BTreeMap<PublicDependencyUseV1, PublicAvailabilityV1>,
    dependency_dag: &'a Digest,
    q3: &'a Digest,
    coverage: &'a Digest,
}

impl CanonicalEncode for InventoryDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.normalizer_protocol.encode_canonical(encoder);
        self.history.encode_canonical(encoder);
        self.extension.encode_canonical(encoder);
        encoder.sequence(self.groups);
        encoder.sequence(self.declarations);
        encoder.sequence(self.equations);
        encoder.sequence(self.forced_projections);
        encoder.sequence(self.demands);
        encode_map(encoder, self.availability);
        self.dependency_dag.encode_canonical(encoder);
        self.q3.encode_canonical(encoder);
        self.coverage.encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::verify_core_manifests_v1;
    use pen_kernel::{DependentContext, KernelLimits};

    struct Fixture {
        manifest: VerifiedSemanticAuditManifestV1,
        kernel: Kernel,
        wire: UncheckedPublicAuditInventoryV1,
        predecessor_equation: GenericJudgmentV1,
        successor_equation: GenericJudgmentV1,
        predecessor_declaration: GlobalId,
        successor_declaration: GlobalId,
    }

    fn digest(label: &str) -> Digest {
        Digest::of_bytes(label.as_bytes())
    }

    fn global(label: &str) -> GlobalId {
        GlobalId(digest(&format!("global/{label}")))
    }

    fn event(label: &str) -> EventIdV1 {
        EventIdV1(digest(&format!("event/{label}")))
    }

    fn equation(label: &str) -> EquationIdV1 {
        EquationIdV1(digest(&format!("equation/{label}")))
    }

    fn contract(label: &str) -> DemandContractIdV1 {
        DemandContractIdV1(digest(&format!("contract/{label}")))
    }

    fn port(label: &str) -> DemandPortKeyV1 {
        DemandPortKeyV1 {
            family: DemandFamilyIdV1(digest(&format!("family/{label}"))),
            output: DemandOutputIdV1(digest(&format!("output/{label}"))),
        }
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

    fn source_judgment(judgment: GenericJudgmentV1) -> SourceNormalizedJudgmentV1 {
        SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-judgment/v1",
                &judgment,
            ),
            source: judgment.clone(),
            claimed_normalized: judgment,
        }
    }

    fn dependency(dependent: PublicSubjectV1, prerequisite: &GlobalId) -> PublicDependencyUseV1 {
        PublicDependencyUseV1 {
            dependent,
            prerequisite: prerequisite.clone(),
        }
    }

    fn fixture() -> Fixture {
        let AuditDecision::Proven((manifest, _)) = verify_core_manifests_v1() else {
            panic!("proposed manifest verifies");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");

        let type_head = global("A");
        let predecessor_term = global("a");
        let successor_head = global("f");
        let group_type = global("group/A");
        let group_term = global("group/a");
        let group_successor = global("group/f");
        let event_type = event("type");
        let event_term = event("term");
        let event_successor = event("successor");
        let predecessor_equation_id = equation("predecessor");
        let successor_equation_id = equation("successor");
        let demand_id = contract("compute");
        let demand_port = port("compute");

        let declaration_type = Declaration {
            id: type_head.clone(),
            ty: Term::Sort { level: 0 },
            body: None,
        };
        let declaration_term = Declaration {
            id: predecessor_term.clone(),
            ty: Term::Global {
                id: type_head.clone(),
            },
            body: None,
        };
        let declaration_successor = Declaration {
            id: successor_head.clone(),
            ty: Term::Pi {
                parameter: Box::new(Term::Global {
                    id: type_head.clone(),
                }),
                body: Box::new(Term::Global {
                    id: type_head.clone(),
                }),
            },
            body: None,
        };

        let predecessor_equation = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Unit,
            right: Term::Unit,
            ty: Term::UnitType,
        };
        let successor_equation = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Global {
                id: predecessor_term.clone(),
            },
            right: Term::Global {
                id: predecessor_term.clone(),
            },
            ty: Term::Global {
                id: type_head.clone(),
            },
        };
        let demand_requirement = successor_equation.clone();

        let boundary_type = UncheckedSignature {
            declarations: vec![declaration_type.clone()],
        };
        let predecessor_boundary = UncheckedSignature {
            declarations: vec![declaration_type.clone(), declaration_term.clone()],
        };
        let successor_boundary = UncheckedSignature {
            declarations: vec![
                declaration_type.clone(),
                declaration_term.clone(),
                declaration_successor.clone(),
            ],
        };

        let predecessor_subject = PublicSubjectV1::Equation {
            equation: predecessor_equation_id.clone(),
        };
        let successor_subject = PublicSubjectV1::Equation {
            equation: successor_equation_id.clone(),
        };
        let dependencies = vec![
            dependency(
                PublicSubjectV1::Declaration {
                    declaration: predecessor_term.clone(),
                },
                &type_head,
            ),
            dependency(
                PublicSubjectV1::Declaration {
                    declaration: successor_head.clone(),
                },
                &type_head,
            ),
            dependency(predecessor_subject, &predecessor_term),
            dependency(successor_subject.clone(), &type_head),
            dependency(successor_subject, &predecessor_term),
            dependency(
                PublicSubjectV1::DemandContract {
                    contract: demand_id.clone(),
                },
                &type_head,
            ),
            dependency(
                PublicSubjectV1::DemandContract {
                    contract: demand_id.clone(),
                },
                &predecessor_term,
            ),
        ];
        let availability = dependencies
            .iter()
            .map(|dependency| {
                let claimed = if dependency.prerequisite == successor_head {
                    PublicAvailabilityV1::DependencyPriorExport {
                        target: dependency.prerequisite.clone(),
                    }
                } else {
                    PublicAvailabilityV1::PredecessorPublicExport {
                        target: dependency.prerequisite.clone(),
                    }
                };
                UncheckedPublicAvailabilityClaimV1 {
                    dependency: dependency.clone(),
                    claimed,
                }
            })
            .collect();

        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: vec![
                UncheckedPublicHistoryStepV1 {
                    census: UncheckedPublicEventCensusV1 {
                        event: event_type.clone(),
                        added_groups: vec![group_type.clone()],
                        added_declarations: vec![type_head.clone()],
                        added_equations: Vec::new(),
                        added_forced_projections: Vec::new(),
                        added_demand_contracts: Vec::new(),
                    },
                    successor_boundary: boundary_type,
                },
                UncheckedPublicHistoryStepV1 {
                    census: UncheckedPublicEventCensusV1 {
                        event: event_term.clone(),
                        added_groups: vec![group_term.clone()],
                        added_declarations: vec![predecessor_term.clone()],
                        added_equations: vec![predecessor_equation_id.clone()],
                        added_forced_projections: Vec::new(),
                        added_demand_contracts: vec![demand_id.clone()],
                    },
                    successor_boundary: predecessor_boundary.clone(),
                },
            ],
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: event_successor.clone(),
                added_groups: vec![group_successor.clone()],
                added_declarations: vec![successor_head.clone()],
                added_equations: vec![successor_equation_id.clone()],
                added_forced_projections: vec![successor_head.clone()],
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups: vec![
                UncheckedPublicGroupV1 {
                    group: group_type.clone(),
                    origin: event_type.clone(),
                    declarations: vec![type_head.clone()],
                },
                UncheckedPublicGroupV1 {
                    group: group_term.clone(),
                    origin: event_term.clone(),
                    declarations: vec![predecessor_term.clone()],
                },
                UncheckedPublicGroupV1 {
                    group: group_successor.clone(),
                    origin: event_successor.clone(),
                    declarations: vec![successor_head.clone()],
                },
            ],
            declarations: vec![
                UncheckedPublicDeclarationV1 {
                    declaration: type_head.clone(),
                    origin: event_type.clone(),
                    group: group_type,
                    source_to_normal: source_declaration(&declaration_type),
                },
                UncheckedPublicDeclarationV1 {
                    declaration: predecessor_term.clone(),
                    origin: event_term.clone(),
                    group: group_term,
                    source_to_normal: source_declaration(&declaration_term),
                },
                UncheckedPublicDeclarationV1 {
                    declaration: successor_head.clone(),
                    origin: event_successor.clone(),
                    group: group_successor,
                    source_to_normal: source_declaration(&declaration_successor),
                },
            ],
            equations: vec![
                UncheckedPublicEquationV1 {
                    equation: predecessor_equation_id,
                    owner_head: predecessor_term.clone(),
                    origin: event_term.clone(),
                    source_to_normal: source_judgment(predecessor_equation.clone()),
                    demand_port: None,
                },
                UncheckedPublicEquationV1 {
                    equation: successor_equation_id,
                    owner_head: predecessor_term.clone(),
                    origin: event_successor.clone(),
                    source_to_normal: source_judgment(successor_equation.clone()),
                    demand_port: Some(demand_port.clone()),
                },
            ],
            forced_projections: vec![UncheckedForcedProjectionV1 {
                projection: successor_head.clone(),
                record_owner: type_head,
                field_ordinal: 0,
                origin: event_successor,
            }],
            predecessor_demand_contracts: vec![UncheckedPredecessorDemandContractV1 {
                contract: demand_id,
                origin: event_term.clone(),
                port: demand_port,
                required_judgment: source_judgment(demand_requirement),
            }],
            public_availability: availability,
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: dependencies,
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: Some(event_term),
                entries: Vec::new(),
            },
        };
        Fixture {
            manifest,
            kernel,
            wire,
            predecessor_equation,
            successor_equation,
            predecessor_declaration: predecessor_term,
            successor_declaration: successor_head,
        }
    }

    fn verify(fixture: &Fixture) -> AuditDecision<VerifiedPublicAuditInventoryV1> {
        verify_public_audit_inventory_v1(&fixture.manifest, &fixture.kernel, &fixture.wire)
    }

    #[test]
    fn valid_inventory_mints_closed_replay_capability() {
        let fixture = fixture();
        let AuditDecision::Proven(inventory) = verify(&fixture) else {
            panic!("fixture verifies");
        };
        assert_eq!(
            inventory.normalizer_protocol_digest(),
            &fixture.kernel.normalizer_protocol_digest()
        );
        assert_eq!(inventory.coverage().group_count(), 3);
        assert_eq!(inventory.coverage().declaration_count(), 3);
        assert_eq!(inventory.coverage().equation_count(), 2);
        assert_eq!(inventory.coverage().forced_projection_count(), 1);
        assert_eq!(inventory.coverage().predecessor_demand_contract_count(), 1);
        assert_eq!(inventory.coverage().dependency_count(), 7);
        assert!(
            inventory
                .predecessor_public_equation_id(&fixture.predecessor_equation)
                .is_some()
        );
        assert!(
            !inventory.is_predecessor_public_equation(&fixture.successor_equation)
                && inventory.is_successor_new_equation(&fixture.successor_equation)
        );
        assert!(inventory.contains_predecessor_declaration(&fixture.predecessor_declaration));
        assert!(!inventory.contains_predecessor_declaration(&fixture.successor_declaration));
        assert!(inventory.q3_registry().is_empty());
        let projection = inventory
            .forced_projection(&fixture.successor_declaration)
            .expect("ledger projection");
        assert_eq!(projection.field_ordinal(), 0);
        assert_eq!(
            projection.normalized_type(),
            &fixture.wire.declarations[2]
                .source_to_normal
                .claimed_normalized
                .ty
        );
    }

    #[test]
    fn forced_projection_omission_and_duplicate_fail_closed() {
        let mut omitted = fixture();
        omitted.wire.forced_projections.clear();
        assert!(matches!(
            verify(&omitted),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut census_omitted = fixture();
        census_omitted
            .wire
            .successor_event
            .added_forced_projections
            .clear();
        assert!(matches!(
            verify(&census_omitted),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut duplicate = fixture();
        duplicate
            .wire
            .forced_projections
            .push(duplicate.wire.forced_projections[0].clone());
        assert!(matches!(
            verify(&duplicate),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));
    }

    #[test]
    fn forced_projection_bad_origin_and_type_fail_closed() {
        let mut origin = fixture();
        origin.wire.forced_projections[0].origin =
            origin.wire.predecessor_history[0].census.event.clone();
        assert!(matches!(
            verify(&origin),
            AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
        ));

        let mut bad_type = fixture();
        let bad_declaration = Declaration {
            id: bad_type.successor_declaration.clone(),
            ty: Term::UnitType,
            body: None,
        };
        bad_type.wire.successor_boundary.declarations[2] = bad_declaration.clone();
        bad_type.wire.declarations[2].source_to_normal = source_declaration(&bad_declaration);
        assert!(matches!(
            verify(&bad_type),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));
    }

    #[test]
    fn forced_projection_forward_owner_fails_closed() {
        let mut forward = fixture();
        forward.wire.forced_projections[0].record_owner = forward.successor_declaration.clone();
        assert!(matches!(
            verify(&forward),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));
    }

    #[test]
    fn projection_metadata_and_demand_source_are_digest_bound() {
        let original_fixture = fixture();
        let AuditDecision::Proven(original) = verify(&original_fixture) else {
            panic!("fixture verifies");
        };

        let mut changed_projection = fixture();
        changed_projection.wire.forced_projections[0].field_ordinal = 1;
        let AuditDecision::Proven(changed_projection) = verify(&changed_projection) else {
            panic!("ledger ordinal remains census data");
        };
        assert_ne!(
            original.coverage().digest(),
            changed_projection.coverage().digest()
        );
        assert_ne!(original.digest(), changed_projection.digest());

        let mut changed_demand_source = fixture();
        let type_head = global("A");
        let predecessor_term = global("a");
        let beta_term = Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::Global {
                    id: type_head.clone(),
                }),
                body: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(Term::Global {
                id: predecessor_term,
            }),
        };
        let source = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: beta_term.clone(),
            right: beta_term,
            ty: Term::Global { id: type_head },
        };
        let requirement =
            &mut changed_demand_source.wire.predecessor_demand_contracts[0].required_judgment;
        requirement.source_identity =
            Digest::of_canonical("pen-semantic-audit/inventory-source-judgment/v1", &source);
        requirement.source = source;
        let AuditDecision::Proven(changed_demand_source) = verify(&changed_demand_source) else {
            panic!("beta source replays to the same exact normal requirement");
        };
        assert_ne!(
            original.coverage().digest(),
            changed_demand_source.coverage().digest()
        );
        assert_ne!(original.digest(), changed_demand_source.digest());
    }

    #[test]
    fn declaration_omission_and_duplicate_fail_closed() {
        let mut omitted = fixture();
        omitted.wire.declarations.pop();
        assert!(matches!(
            verify(&omitted),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut duplicate = fixture();
        duplicate
            .wire
            .declarations
            .push(duplicate.wire.declarations[0].clone());
        assert!(matches!(
            verify(&duplicate),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));
    }

    #[test]
    fn group_and_equation_omissions_fail_closed() {
        let mut group = fixture();
        group.wire.declaration_groups.pop();
        assert!(matches!(
            verify(&group),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut equation = fixture();
        equation.wire.equations.pop();
        assert!(matches!(
            verify(&equation),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));
    }

    #[test]
    fn duplicate_ledger_subject_fails_closed() {
        let mut fixture = fixture();
        let repeated = fixture.wire.predecessor_history[0]
            .census
            .added_declarations[0]
            .clone();
        fixture.wire.predecessor_history[1]
            .census
            .added_declarations
            .push(repeated);
        assert!(matches!(
            verify(&fixture),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
                | AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
        ));
    }

    #[test]
    fn origin_tamper_fails_closed() {
        let mut fixture = fixture();
        fixture.wire.equations[1].origin = fixture.wire.predecessor_history[0].census.event.clone();
        assert!(matches!(
            verify(&fixture),
            AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
        ));
    }

    #[test]
    fn post_hoc_or_wrong_demand_port_fails_closed() {
        let mut wrong_port = fixture();
        wrong_port.wire.equations[1].demand_port = Some(port("unregistered"));
        assert!(matches!(
            verify(&wrong_port),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));

        let mut duplicate_port = fixture();
        let mut second = duplicate_port.wire.predecessor_demand_contracts[0].clone();
        second.contract = contract("duplicate-port");
        duplicate_port.wire.predecessor_history[1]
            .census
            .added_demand_contracts
            .push(second.contract.clone());
        duplicate_port
            .wire
            .predecessor_demand_contracts
            .push(second);
        assert!(matches!(
            verify(&duplicate_port),
            AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
        ));

        let mut mismatched_requirement = fixture();
        mismatched_requirement.wire.predecessor_demand_contracts[0].required_judgment =
            source_judgment(mismatched_requirement.predecessor_equation.clone());
        mismatched_requirement
            .wire
            .dependency_dag
            .edges
            .retain(|edge| !matches!(&edge.dependent, PublicSubjectV1::DemandContract { .. }));
        mismatched_requirement
            .wire
            .public_availability
            .retain(|claim| {
                !matches!(
                    &claim.dependency.dependent,
                    PublicSubjectV1::DemandContract { .. }
                )
            });
        let AuditDecision::Proven(inventory) = verify(&mismatched_requirement) else {
            panic!("strict-prior exact key does not claim semantic realization");
        };
        assert_eq!(
            inventory
                .equations()
                .iter()
                .find_map(VerifiedPublicEquationV1::demand_port),
            Some(inventory.predecessor_demand_contracts()[0].port())
        );
    }

    #[test]
    fn availability_tamper_fails_closed() {
        let mut fixture = fixture();
        let target = fixture.wire.public_availability[0]
            .dependency
            .prerequisite
            .clone();
        fixture.wire.public_availability[0].claimed =
            PublicAvailabilityV1::DependencyPriorExport { target };
        assert!(matches!(
            verify(&fixture),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));
    }

    #[test]
    fn dependency_omission_fails_closed() {
        let mut fixture = fixture();
        fixture.wire.dependency_dag.edges.pop();
        assert!(matches!(
            verify(&fixture),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));
    }

    #[test]
    fn nonempty_or_wrong_cutoff_q3_fails_closed() {
        let mut nonempty = fixture();
        nonempty
            .wire
            .q3_registry
            .entries
            .push(OriginCutoffQ3EntryV1 {
                older_origin: event("type"),
                newer_origin: event("successor"),
                identification: digest("q3/forbidden"),
            });
        assert!(matches!(
            verify(&nonempty),
            AuditDecision::OutsideFragment(OutsideFragmentReason::NonEmptyQ3Registry)
        ));

        let mut cutoff = fixture();
        cutoff.wire.q3_registry.origin_cutoff = Some(event("type"));
        assert!(matches!(
            verify(&cutoff),
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        ));
    }

    #[test]
    fn history_and_boundary_order_tamper_fails_closed() {
        let mut history = fixture();
        history.wire.predecessor_history.reverse();
        assert!(matches!(verify(&history), AuditDecision::Unknown(_)));

        let mut boundary = fixture();
        boundary.wire.successor_boundary.declarations.swap(0, 2);
        assert!(matches!(verify(&boundary), AuditDecision::Unknown(_)));
    }

    #[test]
    fn forward_reference_and_dependency_cycle_fail_closed() {
        let mut forward = fixture();
        let successor = forward.successor_declaration.clone();
        forward.wire.successor_boundary.declarations[2].ty = Term::Global { id: successor };
        assert!(matches!(
            verify(&forward),
            AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify)
        ));

        let mut cycle = fixture();
        cycle.wire.dependency_dag.edges.push(PublicDependencyUseV1 {
            dependent: PublicSubjectV1::Declaration {
                declaration: cycle.successor_declaration.clone(),
            },
            prerequisite: cycle.successor_declaration.clone(),
        });
        assert!(matches!(
            verify(&cycle),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));
    }

    #[test]
    fn metadata_order_is_canonical_and_digest_stable() {
        let original_fixture = fixture();
        let AuditDecision::Proven(original) = verify(&original_fixture) else {
            panic!("fixture verifies");
        };
        let mut reordered = fixture();
        reordered.wire.declaration_groups.reverse();
        reordered.wire.declarations.reverse();
        reordered.wire.equations.reverse();
        reordered.wire.public_availability.reverse();
        reordered.wire.dependency_dag.edges.reverse();
        let AuditDecision::Proven(reordered) = verify(&reordered) else {
            panic!("order-independent metadata verifies");
        };
        assert_eq!(original.digest(), reordered.digest());
        assert_eq!(
            original.dependency_dag().digest(),
            reordered.dependency_dag().digest()
        );
    }

    #[test]
    fn wire_rejects_unknown_json_fields() {
        let fixture = fixture();
        let mut value = serde_json::to_value(&fixture.wire).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("complete".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<UncheckedPublicAuditInventoryV1>(value).is_err());
    }
}
