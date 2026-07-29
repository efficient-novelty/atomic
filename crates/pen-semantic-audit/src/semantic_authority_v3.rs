//! Demand-neutral V3 semantic seed authority.
//!
//! This module preserves the V1 public inventory and the narrow V2 inventory
//! compatibility theorem. It changes only the semantic authority layering:
//! event/declaration support participates in seed identity, while an
//! inventoried equation `PortKey` is retained in a separate metadata ledger.
//! No demand orbit, output, or realization can be obtained from this module.

use crate::fragment::lambda_unit_judgment_syntax_violation;
use crate::inventory::{
    DemandPortKeyV1, VerifiedPublicAuditInventoryV1, VerifiedPublicDeclarationV1,
    VerifiedPublicEquationV1,
};
use crate::inventory_compatibility::VerifiedPublicInventoryCompatibilityV2;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3,
    VerifiedSemanticAuditManifestV3,
};
use crate::model::{ClauseIdV1, EquationIdV1, EventIdV1, GenericJudgmentV1, LocalRoleV1};
use crate::semantic_authority::{
    PublicClauseSubjectV1, VerifiedPublicClauseCensusV1, VerifiedPublicClauseIdentityV1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, KernelError,
    OpenJudgment, ResourceKind, Term,
};
use std::collections::BTreeSet;
use std::sync::Arc;

/// V3 semantic seed identity.
///
/// The wrapped digest is computed without an inventory digest, a V1 clause
/// ID, a demand port, a demand orbit, or a demand output. This is necessary
/// because the preserved V1 inventory and clause IDs bind the complete wire,
/// including non-authoritative port metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeedIdV3(Digest);

impl SeedIdV3 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for SeedIdV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// Structural support used by V3 seed and family identity.
///
/// This capability cannot represent demand output support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedStructuralSupportV1 {
    events: BTreeSet<EventIdV1>,
    declarations: BTreeSet<GlobalId>,
}

impl VerifiedStructuralSupportV1 {
    pub fn events(&self) -> &BTreeSet<EventIdV1> {
        &self.events
    }

    pub fn declarations(&self) -> &BTreeSet<GlobalId> {
        &self.declarations
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            events: self.events.union(&other.events).cloned().collect(),
            declarations: self
                .declarations
                .union(&other.declarations)
                .cloned()
                .collect(),
        }
    }

    pub fn is_old_support(&self, old_events: &BTreeSet<EventIdV1>) -> bool {
        self.events.is_subset(old_events)
    }
}

impl CanonicalEncode for VerifiedStructuralSupportV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_set(encoder, &self.events);
        encode_set(encoder, &self.declarations);
    }
}

/// Exact public subject of one V3 rank-zero seed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicSemanticSeedSubjectV3 {
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

impl CanonicalEncode for PublicSemanticSeedSubjectV3 {
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

/// Bound inventory metadata for an equation port.
///
/// This record proves only that the exact key occurred on the verified
/// equation wire. It is deliberately separate from
/// [`VerifiedPreQ0SemanticSeedV3`] and carries no orbit, realization, family
/// class, SR2, or discharge authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedBoundEquationPortMetadataV3 {
    equation: EquationIdV1,
    source_clause: ClauseIdV1,
    port: DemandPortKeyV1,
    digest: Digest,
}

impl VerifiedBoundEquationPortMetadataV3 {
    pub fn equation(&self) -> &EquationIdV1 {
        &self.equation
    }

    pub fn source_clause(&self) -> &ClauseIdV1 {
        &self.source_clause
    }

    /// Returns bound inventory metadata, not demand provenance authority.
    pub fn port(&self) -> &DemandPortKeyV1 {
        &self.port
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedBoundEquationPortMetadataV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        self.source_clause.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// One kernel-replayed, demand-neutral rank-zero V3 seed.
///
/// `source_clause` is verifier-derived lineage. It is intentionally excluded
/// from `id` and `digest`, because the preserved V1 clause ID binds the full
/// inventory wire and is therefore sensitive to bound PortKey metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPreQ0SemanticSeedV3 {
    id: SeedIdV3,
    subject: PublicSemanticSeedSubjectV3,
    source_identity: Digest,
    origin_event: EventIdV1,
    source_judgment: GenericJudgmentV1,
    source_clause: ClauseIdV1,
    structural_support: VerifiedStructuralSupportV1,
    local_role: LocalRoleV1,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedPreQ0SemanticSeedV3 {
    pub fn id(&self) -> &SeedIdV3 {
        &self.id
    }

    pub fn subject(&self) -> &PublicSemanticSeedSubjectV3 {
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

    pub fn structural_support(&self) -> &VerifiedStructuralSupportV1 {
        &self.structural_support
    }

    pub fn local_role(&self) -> LocalRoleV1 {
        self.local_role
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }

    /// Demand-neutral semantic digest. The V1 source-clause ID and inventory
    /// binding are intentionally absent.
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedPreQ0SemanticSeedV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.origin_event.encode_canonical(encoder);
        self.source_judgment.encode_canonical(encoder);
        // This lineage binding is not part of `id` or `digest`.
        self.source_clause.encode_canonical(encoder);
        self.structural_support.encode_canonical(encoder);
        self.local_role.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteSeedBaseCoverage;
}

/// Complete V3 seed-base authority for one compatible V1 inventory.
///
/// The port metadata ledger is bound into this census proof, but it is not
/// present in any seed ID or semantic digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSemanticSeedBaseCensusV3 {
    semantic_manifest_digest: Digest,
    inventory_compatibility_digest: Digest,
    inventory_digest: Digest,
    public_clause_census_digest: Digest,
    seeds: Arc<[VerifiedPreQ0SemanticSeedV3]>,
    equation_port_metadata: Arc<[VerifiedBoundEquationPortMetadataV3]>,
    coverage_digest: Digest,
    complete: private::CompleteSeedBaseCoverage,
    digest: Digest,
}

impl VerifiedSemanticSeedBaseCensusV3 {
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

    pub fn seeds(&self) -> &[VerifiedPreQ0SemanticSeedV3] {
        &self.seeds
    }

    /// Port associations retained as inventory metadata only.
    pub fn equation_port_metadata(&self) -> &[VerifiedBoundEquationPortMetadataV3] {
        &self.equation_port_metadata
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSemanticSeedBaseCensusV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.inventory_compatibility_digest
            .encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.public_clause_census_digest.encode_canonical(encoder);
        encoder.sequence(&self.seeds);
        encoder.sequence(&self.equation_port_metadata);
        self.coverage_digest.encode_canonical(encoder);
        encoder.tag(1);
        self.digest.encode_canonical(encoder);
    }
}

/// Reconstruct every public V3 seed without demand-orbit or realization input.
///
/// The narrow V2 compatibility capability is used only to transport the
/// preserved V1 inventory facts. Missing demand authority is not on this
/// verifier's critical path.
pub fn verify_semantic_seed_base_census_v3(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory_compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
) -> AuditDecision<VerifiedSemanticSeedBaseCensusV3> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3
        || inventory_compatibility.v1_manifest_digest() != inventory.manifest_digest()
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
        || inventory_compatibility.q3_registry_digest() != inventory.q3_registry().digest()
        || public_clauses.inventory_digest() != inventory.digest()
        || public_clauses.inventory_coverage_digest() != inventory.coverage().digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let expected_seed_count = inventory
        .declarations()
        .len()
        .saturating_add(inventory.equations().len());
    if expected_seed_count > usize::from(manifest.manifest().maximum_seeds) {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }
    if public_clauses.clauses().len() != expected_seed_count {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }

    let mut seeds = Vec::with_capacity(expected_seed_count);
    let mut port_metadata = Vec::new();
    let mut covered_clauses = BTreeSet::new();
    let mut covered_ports = BTreeSet::new();

    for declaration in inventory.declarations() {
        let Some(clause) = declaration_clause(public_clauses, declaration) else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        if !covered_clauses.insert(clause.id().clone()) {
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
            PublicSemanticSeedSubjectV3::Declaration {
                declaration: declaration.declaration().clone(),
                public_group: declaration.group().clone(),
            },
            declaration.source_identity(),
            declaration.origin(),
            source_judgment,
            clause,
            LocalRoleV1::KernelHead,
        ) {
            Ok(seed) => seed,
            Err(failure) => return failure.into_decision(),
        };
        seeds.push(seed);
    }

    for equation in inventory.equations() {
        let Some(clause) = equation_clause(public_clauses, equation) else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        if !covered_clauses.insert(clause.id().clone()) {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        let seed = match verified_seed_record(
            manifest,
            kernel,
            inventory,
            PublicSemanticSeedSubjectV3::Equation {
                equation: equation.equation().clone(),
                owner_head: equation.owner_head().clone(),
                public_group: clause.public_group().clone(),
            },
            equation.source_identity(),
            equation.origin(),
            equation.source().clone(),
            clause,
            LocalRoleV1::Coherence,
        ) {
            Ok(seed) => seed,
            Err(failure) => return failure.into_decision(),
        };
        seeds.push(seed);

        if let Some(port) = equation.demand_port() {
            if !covered_ports.insert(equation.equation().clone()) {
                return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
            }
            let digest = Digest::of_canonical(
                "pen-semantic-audit/bound-equation-port-metadata/v3",
                &BoundEquationPortMaterialV3 {
                    inventory: inventory.digest(),
                    equation: equation.equation(),
                    source_clause: clause.id(),
                    port,
                },
            );
            port_metadata.push(VerifiedBoundEquationPortMetadataV3 {
                equation: equation.equation().clone(),
                source_clause: clause.id().clone(),
                port: port.clone(),
                digest,
            });
        }
    }

    if covered_clauses.len() != public_clauses.clauses().len()
        || covered_ports.len()
            != inventory
                .equations()
                .iter()
                .filter(|equation| equation.demand_port().is_some())
                .count()
    {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }

    seeds.sort_by(|left, right| left.id.cmp(&right.id));
    port_metadata.sort_by(|left, right| left.equation.cmp(&right.equation));
    if seeds.windows(2).any(|pair| pair[0].id == pair[1].id) {
        return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
    }

    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/semantic-seed-base-census-coverage/v3",
        &SemanticSeedBaseCoverageMaterialV3 {
            manifest: manifest.candidate_digest(),
            inventory_compatibility: inventory_compatibility.digest(),
            inventory: inventory.digest(),
            clauses: public_clauses.digest(),
            seeds: &seeds,
            port_metadata: &port_metadata,
        },
    );
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-semantic-seed-base-census/v3",
        &SemanticSeedBaseCensusMaterialV3 {
            manifest: manifest.candidate_digest(),
            inventory_compatibility: inventory_compatibility.digest(),
            inventory: inventory.digest(),
            clauses: public_clauses.digest(),
            coverage: &coverage_digest,
            seeds: &seeds,
            port_metadata: &port_metadata,
        },
    );

    AuditDecision::Proven(VerifiedSemanticSeedBaseCensusV3 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        inventory_compatibility_digest: inventory_compatibility.digest().clone(),
        inventory_digest: inventory.digest().clone(),
        public_clause_census_digest: public_clauses.digest().clone(),
        seeds: Arc::from(seeds),
        equation_port_metadata: Arc::from(port_metadata),
        coverage_digest,
        complete: private::CompleteSeedBaseCoverage,
        digest,
    })
}

fn declaration_clause<'a>(
    public_clauses: &'a VerifiedPublicClauseCensusV1,
    declaration: &VerifiedPublicDeclarationV1,
) -> Option<&'a VerifiedPublicClauseIdentityV1> {
    let clause_id = public_clauses.clause_for_declaration(declaration.declaration())?;
    let clause = public_clauses
        .clauses()
        .iter()
        .find(|candidate| candidate.id() == clause_id)?;
    if clause.subject()
        != &(PublicClauseSubjectV1::Declaration {
            declaration: declaration.declaration().clone(),
        })
        || clause.source_identity() != declaration.source_identity()
        || clause.origin_event() != declaration.origin()
        || clause.public_group() != declaration.group()
    {
        return None;
    }
    Some(clause)
}

fn equation_clause<'a>(
    public_clauses: &'a VerifiedPublicClauseCensusV1,
    equation: &VerifiedPublicEquationV1,
) -> Option<&'a VerifiedPublicClauseIdentityV1> {
    let clause_id = public_clauses.clause_for_equation(equation.equation())?;
    let clause = public_clauses
        .clauses()
        .iter()
        .find(|candidate| candidate.id() == clause_id)?;
    if clause.subject()
        != &(PublicClauseSubjectV1::Equation {
            equation: equation.equation().clone(),
            owner_head: equation.owner_head().clone(),
        })
        || clause.source_identity() != equation.source_identity()
        || clause.origin_event() != equation.origin()
    {
        return None;
    }
    Some(clause)
}

#[derive(Clone, Debug)]
enum SemanticSeedFailureV3 {
    Unknown(AuditUnknownReason),
    Outside(crate::manifest::OutsideFragmentReason),
}

impl SemanticSeedFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::Unknown(reason) => AuditDecision::Unknown(reason),
            Self::Outside(reason) => AuditDecision::OutsideFragment(reason),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn verified_seed_record(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    subject: PublicSemanticSeedSubjectV3,
    source_identity: &Digest,
    origin_event: &EventIdV1,
    source_judgment: GenericJudgmentV1,
    clause: &VerifiedPublicClauseIdentityV1,
    local_role: LocalRoleV1,
) -> Result<VerifiedPreQ0SemanticSeedV3, SemanticSeedFailureV3> {
    if source_judgment.context().0.len() > usize::from(manifest.manifest().maximum_context_entries)
    {
        return Err(SemanticSeedFailureV3::Unknown(
            AuditUnknownReason::ResourceExhausted,
        ));
    }
    if let Some(violation) = lambda_unit_judgment_syntax_violation(
        &source_judgment,
        &manifest.manifest().universe_levels,
    ) {
        return Err(SemanticSeedFailureV3::Outside(violation.outside_reason()));
    }
    if !clause.public_support().demand_outputs.is_empty() {
        return Err(SemanticSeedFailureV3::Unknown(
            AuditUnknownReason::DemandAuthorityEnteredStructuralIdentity,
        ));
    }
    let structural_support = VerifiedStructuralSupportV1 {
        events: clause.public_support().events.clone(),
        declarations: clause.public_support().declarations.clone(),
    };
    let kernel_replay_digest = replay_seed_judgment(kernel, inventory, &source_judgment)
        .map_err(SemanticSeedFailureV3::Unknown)?;
    let semantic_material = SemanticSeedMaterialV3 {
        manifest: manifest.candidate_digest(),
        subject: &subject,
        source_identity,
        origin_event,
        source_judgment: &source_judgment,
        structural_support: &structural_support,
        local_role,
        kernel_replay: &kernel_replay_digest,
    };
    let id = SeedIdV3(Digest::of_canonical(
        "pen-semantic-audit/pre-q0-semantic-seed-id/v3",
        &semantic_material,
    ));
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-pre-q0-semantic-seed/v3",
        &SemanticSeedDigestMaterialV3 {
            id: &id,
            semantic_material: &semantic_material,
        },
    );
    Ok(VerifiedPreQ0SemanticSeedV3 {
        id,
        subject,
        source_identity: source_identity.clone(),
        origin_event: origin_event.clone(),
        source_judgment,
        source_clause: clause.id().clone(),
        structural_support,
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
                "pen-semantic-audit/semantic-seed-kernel-replay/v3",
                &KernelReplayMaterialV3 {
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
                "pen-semantic-audit/semantic-equation-seed-kernel-replay/v3",
                &EquationKernelReplayMaterialV3 {
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

fn encode_set<T: CanonicalEncode>(encoder: &mut CanonicalEncoder, values: &BTreeSet<T>) {
    encoder.u64(values.len() as u64);
    for value in values {
        value.encode_canonical(encoder);
    }
}

struct SemanticSeedMaterialV3<'a> {
    manifest: &'a Digest,
    subject: &'a PublicSemanticSeedSubjectV3,
    source_identity: &'a Digest,
    origin_event: &'a EventIdV1,
    source_judgment: &'a GenericJudgmentV1,
    structural_support: &'a VerifiedStructuralSupportV1,
    local_role: LocalRoleV1,
    kernel_replay: &'a Digest,
}

impl CanonicalEncode for SemanticSeedMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.origin_event.encode_canonical(encoder);
        self.source_judgment.encode_canonical(encoder);
        self.structural_support.encode_canonical(encoder);
        self.local_role.encode_canonical(encoder);
        self.kernel_replay.encode_canonical(encoder);
    }
}

struct SemanticSeedDigestMaterialV3<'a> {
    id: &'a SeedIdV3,
    semantic_material: &'a SemanticSeedMaterialV3<'a>,
}

impl CanonicalEncode for SemanticSeedDigestMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.semantic_material.encode_canonical(encoder);
    }
}

struct BoundEquationPortMaterialV3<'a> {
    inventory: &'a Digest,
    equation: &'a EquationIdV1,
    source_clause: &'a ClauseIdV1,
    port: &'a DemandPortKeyV1,
}

impl CanonicalEncode for BoundEquationPortMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.equation.encode_canonical(encoder);
        self.source_clause.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
    }
}

struct KernelReplayMaterialV3<'a> {
    input: &'a OpenJudgment,
    output: &'a OpenJudgment,
}

impl CanonicalEncode for KernelReplayMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.input.encode_canonical(encoder);
        self.output.encode_canonical(encoder);
    }
}

struct EquationKernelReplayMaterialV3<'a> {
    left_input: &'a OpenJudgment,
    left_output: &'a OpenJudgment,
    right_input: &'a OpenJudgment,
    right_output: &'a OpenJudgment,
}

impl CanonicalEncode for EquationKernelReplayMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.left_input.encode_canonical(encoder);
        self.left_output.encode_canonical(encoder);
        self.right_input.encode_canonical(encoder);
        self.right_output.encode_canonical(encoder);
    }
}

struct SemanticSeedBaseCoverageMaterialV3<'a> {
    manifest: &'a Digest,
    inventory_compatibility: &'a Digest,
    inventory: &'a Digest,
    clauses: &'a Digest,
    seeds: &'a [VerifiedPreQ0SemanticSeedV3],
    port_metadata: &'a [VerifiedBoundEquationPortMetadataV3],
}

impl CanonicalEncode for SemanticSeedBaseCoverageMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.inventory_compatibility.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        self.clauses.encode_canonical(encoder);
        encoder.sequence(self.seeds);
        encoder.sequence(self.port_metadata);
        encoder.u64(self.seeds.len() as u64);
        encoder.u64(self.port_metadata.len() as u64);
    }
}

struct SemanticSeedBaseCensusMaterialV3<'a> {
    manifest: &'a Digest,
    inventory_compatibility: &'a Digest,
    inventory: &'a Digest,
    clauses: &'a Digest,
    coverage: &'a Digest,
    seeds: &'a [VerifiedPreQ0SemanticSeedV3],
    port_metadata: &'a [VerifiedBoundEquationPortMetadataV3],
}

impl CanonicalEncode for SemanticSeedBaseCensusMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.inventory_compatibility.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        self.clauses.encode_canonical(encoder);
        self.coverage.encode_canonical(encoder);
        encoder.sequence(self.seeds);
        encoder.sequence(self.port_metadata);
        encoder.tag(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        DemandContractIdV1, DemandFamilyIdV1, ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
        PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION, PublicDependencyUseV1, PublicSubjectV1,
        UncheckedOriginCutoffQ3RegistryV1, UncheckedPredecessorDemandContractV1,
        UncheckedPublicAuditInventoryV1, UncheckedPublicAvailabilityClaimV1,
        UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1,
        UncheckedPublicEventCensusV1, UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::inventory_compatibility::verify_public_inventory_compatibility_v2;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_lambda_unit_manifest_v2,
        proposed_semantic_audit_lambda_unit_manifest_v3,
        verify_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v2,
        verify_semantic_audit_lambda_unit_manifest_v3,
    };
    use crate::model::{DemandOutputIdV1, PublicAvailabilityV1, SourceNormalizedJudgmentV1};
    use crate::semantic_authority::verify_public_clause_census_v1;
    use pen_kernel::{Declaration, KernelLimits, UncheckedSignature};

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

    fn verified_inventory_with_port(port_salt: &[u8]) -> VerifiedPublicAuditInventoryV1 {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("lambda V1 manifest");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let predecessor_head = GlobalId(Digest::of_bytes(b"v3-seed/predecessor-head"));
        let successor_head = GlobalId(Digest::of_bytes(b"v3-seed/successor-head"));
        let predecessor_group = GlobalId(Digest::of_bytes(b"v3-seed/predecessor-group"));
        let successor_group = GlobalId(Digest::of_bytes(b"v3-seed/successor-group"));
        let predecessor_event = EventIdV1(Digest::of_bytes(b"v3-seed/predecessor-event"));
        let successor_event = EventIdV1(Digest::of_bytes(b"v3-seed/successor-event"));
        let contract = DemandContractIdV1(Digest::of_bytes(b"v3-seed/contract"));
        let equation = EquationIdV1(Digest::of_bytes(b"v3-seed/equation"));
        let port = DemandPortKeyV1 {
            family: DemandFamilyIdV1(Digest::of_bytes(port_salt)),
            output: DemandOutputIdV1(Digest::of_bytes(port_salt)),
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

    fn seed_census(inventory: &VerifiedPublicAuditInventoryV1) -> VerifiedSemanticSeedBaseCensusV3 {
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
        let AuditDecision::Proven(v3_manifest) = verify_semantic_audit_lambda_unit_manifest_v3(
            &proposed_semantic_audit_lambda_unit_manifest_v3(),
        ) else {
            panic!("V3 manifest");
        };
        let AuditDecision::Proven(compatibility) =
            verify_public_inventory_compatibility_v2(&v1_manifest, &v2_manifest, inventory)
        else {
            panic!("inventory compatibility");
        };
        let AuditDecision::Proven(clauses) = verify_public_clause_census_v1(inventory) else {
            panic!("clause census");
        };
        let AuditDecision::Proven(census) = verify_semantic_seed_base_census_v3(
            &v3_manifest,
            &kernel,
            &compatibility,
            inventory,
            &clauses,
        ) else {
            panic!("V3 seed census");
        };
        census
    }

    #[test]
    fn nonempty_port_is_metadata_and_does_not_block_v3_seed_census() {
        let inventory = verified_inventory_with_port(b"v3-seed/port-a");
        let census = seed_census(&inventory);

        assert_eq!(census.inventory_digest(), inventory.digest());
        assert_eq!(census.seeds().len(), 3);
        assert_eq!(census.equation_port_metadata().len(), 1);
        assert_eq!(
            census.equation_port_metadata()[0].port(),
            inventory.equations()[0].demand_port().unwrap()
        );
        assert!(
            census
                .seeds()
                .iter()
                .all(|seed| !seed.structural_support().events().is_empty())
        );
    }

    #[test]
    fn port_key_changes_lineage_census_but_not_seed_ids_or_semantic_digests() {
        let first = seed_census(&verified_inventory_with_port(b"v3-seed/port-a"));
        let second = seed_census(&verified_inventory_with_port(b"v3-seed/port-b"));

        assert_ne!(first.inventory_digest(), second.inventory_digest());
        assert_ne!(
            first.public_clause_census_digest(),
            second.public_clause_census_digest()
        );
        assert_ne!(first.digest(), second.digest());
        assert_ne!(
            first.equation_port_metadata()[0].digest(),
            second.equation_port_metadata()[0].digest()
        );
        assert_eq!(first.seeds().len(), second.seeds().len());
        for (left, right) in first.seeds().iter().zip(second.seeds()) {
            assert_eq!(left.subject(), right.subject());
            assert_eq!(left.id(), right.id());
            assert_eq!(left.digest(), right.digest());
        }
        assert!(
            first
                .seeds()
                .iter()
                .zip(second.seeds())
                .any(|(left, right)| left.source_clause() != right.source_clause())
        );
    }
}
