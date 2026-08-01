//! Target-neutral SR2 non-injectivity theorem for the exact empty-demand V3 chain.
//!
//! This module proves only a negative cardinality fact: there is no typed,
//! role-preserving injection from the exact marginal-family set into the SR2
//! tag codomain
//! `(restricted kernel-cost basis x recognized local roles) + pre-existing
//! demand outputs`.  "Role-preserving" is the frozen SR2 invariant: a clause
//! tag for a family must carry that family's unique verified `LocalRoleV1`.
//! This is not a claim about an unconstrained set injection. The verifier
//! prefers a role-fiber pigeonhole witness and otherwise uses the capacity of
//! the full codomain.
//!
//! The capability is diagnostic theorem authority.  It does not construct a
//! positive provenance assignment and mints no `nu`, `gamma`, debt, halt,
//! Selective-Law, or live Profile-A authority.

use crate::demand_orbit_census_v3::VerifiedDemandOrbitCensusV3;
use crate::demand_realization_census_v3::VerifiedDemandRealizationCensusV3;
use crate::family_quotient_v3::FamilyClassIdV3;
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::marginal_family_set_v3::VerifiedMarginalFamilySetV3;
use crate::model::{ClauseIdV1, EventIdV1, LocalRoleV1};
use crate::restricted_kernel_cost_basis_v3::VerifiedRestrictedKernelCostBasisV3;
use crate::semantic_authority::VerifiedPublicClauseCensusV1;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const SR2_NONINJECTIVITY_SCHEMA_VERSION_V3: u16 = 1;

/// The complete role vocabulary admitted by the V3 SR2 clause-role tag.
///
/// This is protocol material, not a fixture-derived cardinality.  Capacity is
/// always computed from this sequence and never from a hard-coded count.
pub const SR2_LOCAL_ROLE_UNIVERSE_V3: &[LocalRoleV1] = &[
    LocalRoleV1::KernelHead,
    LocalRoleV1::AdjointMate,
    LocalRoleV1::SupportAction,
    LocalRoleV1::Coherence,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sr2NonInjectivityFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    NonEmptyDemandSurface,
    MarginalCoverageMismatch,
    RestrictedCostBasisMismatch,
    CapacityOverflow,
    NoPigeonholeWitness,
}

impl std::fmt::Display for Sr2NonInjectivityFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifest is not the exact registered lambda/unit V3 proposal",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the manifest, inventory, clause, marginal, demand, and restricted-cost capabilities are not one exact V3 chain",
            ),
            Self::NonEmptyDemandSurface => formatter.write_str(
                "this theorem requires the complete empty demand-orbit and realization base",
            ),
            Self::MarginalCoverageMismatch => formatter.write_str(
                "the exact marginal capability does not expose one unique class in each derived role fiber",
            ),
            Self::RestrictedCostBasisMismatch => formatter.write_str(
                "the restricted cost capability does not expose a unique canonical singleton basis",
            ),
            Self::CapacityOverflow => formatter.write_str(
                "the exact SR2 domain or codomain capacity cannot be represented by the theorem schema",
            ),
            Self::NoPigeonholeWitness => formatter.write_str(
                "neither a role fiber nor the full SR2 codomain is over capacity",
            ),
        }
    }
}

impl std::error::Error for Sr2NonInjectivityFailureV3 {}

impl Sr2NonInjectivityFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::ExactManifestIdentityMismatch | Self::ChainBindingMismatch => {
                AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
            }
            Self::MarginalCoverageMismatch | Self::RestrictedCostBasisMismatch => {
                AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
            }
            Self::CapacityOverflow => AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted),
            Self::NonEmptyDemandSurface | Self::NoPigeonholeWitness => {
                AuditDecision::Unknown(AuditUnknownReason::MissingSr2ProvenanceAssignment)
            }
        }
    }
}

/// Canonical partition cell of the exact marginal set by recognized local
/// role.  Empty fibers are retained so the proof binds the complete role
/// universe rather than only roles observed in one target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSr2RoleFiberV3 {
    role: LocalRoleV1,
    families: Arc<[FamilyClassIdV3]>,
}

impl VerifiedSr2RoleFiberV3 {
    pub fn role(&self) -> LocalRoleV1 {
        self.role
    }

    pub fn families(&self) -> &[FamilyClassIdV3] {
        &self.families
    }

    pub fn family_count(&self) -> usize {
        self.families.len()
    }
}

impl CanonicalEncode for VerifiedSr2RoleFiberV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.role.encode_canonical(encoder);
        encoder.sequence(&self.families);
    }
}

/// Generic finite pigeonhole witness for the frozen typed SR2 contract. In
/// the role-fiber case, role preservation means every family in the fiber can
/// use only cost tags carrying that same verified role (plus any pre-existing
/// demand-output tags). In the total case, the whole domain is larger than
/// the complete disjoint-union codomain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sr2PigeonholeWitnessV3 {
    RoleFiber {
        role: LocalRoleV1,
        domain_count: u64,
        codomain_capacity: u64,
    },
    TotalCapacity {
        domain_count: u64,
        codomain_capacity: u64,
    },
}

impl CanonicalEncode for Sr2PigeonholeWitnessV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::RoleFiber {
                role,
                domain_count,
                codomain_capacity,
            } => {
                encoder.tag(0);
                role.encode_canonical(encoder);
                encoder.u64(*domain_count);
                encoder.u64(*codomain_capacity);
            }
            Self::TotalCapacity {
                domain_count,
                codomain_capacity,
            } => {
                encoder.tag(1);
                encoder.u64(*domain_count);
                encoder.u64(*codomain_capacity);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sr2RestrictedCostBasisCellV3 {
    class: Digest,
    singleton_member: ClauseIdV1,
}

impl CanonicalEncode for Sr2RestrictedCostBasisCellV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.class.encode_canonical(encoder);
        self.singleton_member.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sr2ClauseRoleTagCellV3 {
    clause: ClauseIdV1,
    role: LocalRoleV1,
}

impl CanonicalEncode for Sr2ClauseRoleTagCellV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.clause.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteSr2NonInjectivityProof;
}

/// Opaque proof that no typed, role-preserving SR2 injection exists for one
/// exact V3 chain under the supplied restricted cost theorem and complete
/// empty demand base.
///
/// There is no deserialization path and no constructor from caller-provided
/// counts, role fibers, basis cells, demand outputs, or witness inequalities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSr2NonInjectivityV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    inventory_manifest_digest: Digest,
    inventory_normalizer_protocol_digest: Digest,
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    predecessor_history_digest: Digest,
    exact_extension_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    new_event: EventIdV1,
    public_clause_census_digest: Digest,
    public_clause_coverage_digest: Digest,
    marginal_family_set_digest: Digest,
    marginal_semantic_set_digest: Digest,
    marginal_rewrite_authority_digest: Digest,
    predecessor_reconstruction_digest: Digest,
    successor_quotient_digest: Digest,
    weakening_digest: Digest,
    demand_orbit_census_digest: Digest,
    predecessor_demand_census_digest: Digest,
    empty_orbit_partition_digest: Digest,
    demand_realization_census_digest: Digest,
    semantic_seed_base_census_digest: Digest,
    realization_obligation_census_digest: Digest,
    empty_realization_relation_digest: Digest,
    restricted_cost_basis_digest: Digest,
    cost_manifest_digest: Digest,
    cost_carrier_digest: Digest,
    cost_rewrite_authority_digest: Digest,
    cost_v2_certificate_digest: Digest,
    recognized_roles: Arc<[LocalRoleV1]>,
    marginal_families: Arc<[FamilyClassIdV3]>,
    role_fibers: Arc<[VerifiedSr2RoleFiberV3]>,
    restricted_cost_basis: Arc<[Sr2RestrictedCostBasisCellV3]>,
    clause_role_tags: Arc<[Sr2ClauseRoleTagCellV3]>,
    marginal_family_count: u64,
    restricted_cost_basis_count: u64,
    recognized_role_count: u64,
    preexisting_demand_output_count: u64,
    role_fiber_codomain_capacity: u64,
    clause_role_tag_capacity: u64,
    full_codomain_capacity: u64,
    role_fiber_census_digest: Digest,
    restricted_cost_tag_census_digest: Digest,
    full_codomain_material_digest: Digest,
    witness: Sr2PigeonholeWitnessV3,
    witness_digest: Digest,
    complete: private::CompleteSr2NonInjectivityProof,
    digest: Digest,
}

impl VerifiedSr2NonInjectivityV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn public_clause_census_digest(&self) -> &Digest {
        &self.public_clause_census_digest
    }

    pub fn marginal_family_set_digest(&self) -> &Digest {
        &self.marginal_family_set_digest
    }

    pub fn demand_orbit_census_digest(&self) -> &Digest {
        &self.demand_orbit_census_digest
    }

    pub fn demand_realization_census_digest(&self) -> &Digest {
        &self.demand_realization_census_digest
    }

    pub fn restricted_cost_basis_digest(&self) -> &Digest {
        &self.restricted_cost_basis_digest
    }

    pub fn marginal_families(&self) -> &[FamilyClassIdV3] {
        &self.marginal_families
    }

    pub fn role_fibers(&self) -> &[VerifiedSr2RoleFiberV3] {
        &self.role_fibers
    }

    /// Structural domain cardinality used by the negative theorem.  This is
    /// not an issued value register and is not `nu`.
    pub fn marginal_family_count(&self) -> usize {
        self.marginal_family_count as usize
    }

    pub fn restricted_cost_basis_count(&self) -> usize {
        self.restricted_cost_basis_count as usize
    }

    pub fn recognized_role_count(&self) -> usize {
        self.recognized_role_count as usize
    }

    pub fn preexisting_demand_output_count(&self) -> usize {
        self.preexisting_demand_output_count as usize
    }

    pub fn role_fiber_codomain_capacity(&self) -> usize {
        self.role_fiber_codomain_capacity as usize
    }

    pub fn full_codomain_capacity(&self) -> usize {
        self.full_codomain_capacity as usize
    }

    pub fn witness(&self) -> &Sr2PigeonholeWitnessV3 {
        &self.witness
    }

    pub fn witness_digest(&self) -> &Digest {
        &self.witness_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSr2NonInjectivityV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.inventory_manifest_digest.encode_canonical(encoder);
        self.inventory_normalizer_protocol_digest
            .encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.inventory_coverage_digest.encode_canonical(encoder);
        self.predecessor_history_digest.encode_canonical(encoder);
        self.exact_extension_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.new_event.encode_canonical(encoder);
        self.public_clause_census_digest.encode_canonical(encoder);
        self.public_clause_coverage_digest.encode_canonical(encoder);
        self.marginal_family_set_digest.encode_canonical(encoder);
        self.marginal_semantic_set_digest.encode_canonical(encoder);
        self.marginal_rewrite_authority_digest
            .encode_canonical(encoder);
        self.predecessor_reconstruction_digest
            .encode_canonical(encoder);
        self.successor_quotient_digest.encode_canonical(encoder);
        self.weakening_digest.encode_canonical(encoder);
        self.demand_orbit_census_digest.encode_canonical(encoder);
        self.predecessor_demand_census_digest
            .encode_canonical(encoder);
        self.empty_orbit_partition_digest.encode_canonical(encoder);
        self.demand_realization_census_digest
            .encode_canonical(encoder);
        self.semantic_seed_base_census_digest
            .encode_canonical(encoder);
        self.realization_obligation_census_digest
            .encode_canonical(encoder);
        self.empty_realization_relation_digest
            .encode_canonical(encoder);
        self.restricted_cost_basis_digest.encode_canonical(encoder);
        self.cost_manifest_digest.encode_canonical(encoder);
        self.cost_carrier_digest.encode_canonical(encoder);
        self.cost_rewrite_authority_digest.encode_canonical(encoder);
        self.cost_v2_certificate_digest.encode_canonical(encoder);
        encoder.sequence(&self.recognized_roles);
        encoder.sequence(&self.marginal_families);
        encoder.sequence(&self.role_fibers);
        encoder.sequence(&self.restricted_cost_basis);
        encoder.sequence(&self.clause_role_tags);
        encoder.u64(self.marginal_family_count);
        encoder.u64(self.restricted_cost_basis_count);
        encoder.u64(self.recognized_role_count);
        encoder.u64(self.preexisting_demand_output_count);
        encoder.u64(self.role_fiber_codomain_capacity);
        encoder.u64(self.clause_role_tag_capacity);
        encoder.u64(self.full_codomain_capacity);
        self.role_fiber_census_digest.encode_canonical(encoder);
        self.restricted_cost_tag_census_digest
            .encode_canonical(encoder);
        self.full_codomain_material_digest.encode_canonical(encoder);
        self.witness.encode_canonical(encoder);
        self.witness_digest.encode_canonical(encoder);
        encoder.tag(1);
        // The final digest is intentionally excluded from its own encoding.
    }
}

/// Derive a finite pigeonhole proof from exact verifier-minted capabilities.
/// No count, role fiber, basis class, demand output, or witness is supplied by
/// the caller.
#[allow(clippy::too_many_arguments)]
pub fn diagnose_sr2_noninjectivity_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
    restricted_cost: &VerifiedRestrictedKernelCostBasisV3,
) -> Result<VerifiedSr2NonInjectivityV3, Sr2NonInjectivityFailureV3> {
    if v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3() {
        return Err(Sr2NonInjectivityFailureV3::ExactManifestIdentityMismatch);
    }
    verify_exact_chain(
        v3_manifest,
        inventory,
        public_clauses,
        marginals,
        orbits,
        realizations,
        restricted_cost,
    )?;
    verify_complete_empty_demand_surface(inventory, orbits, realizations)?;

    let marginal_families = derive_role_fibers(marginals)?;
    let (restricted_cost_basis, clause_role_tags) =
        derive_restricted_cost_tags(public_clauses, restricted_cost)?;

    let marginal_family_count = checked_len(marginal_families.0.len())?;
    let restricted_cost_basis_count = checked_len(restricted_cost_basis.len())?;
    let recognized_role_count = checked_len(SR2_LOCAL_ROLE_UNIVERSE_V3.len())?;

    // Complete empty orbit authority implies that the second summand of the
    // SR2 tag codomain has cardinality zero.  Realization emptiness is checked
    // independently above and bound into the transcript below.
    let preexisting_demand_output_count = 0_u64;
    let role_fiber_codomain_capacity = restricted_cost_basis_count
        .checked_add(preexisting_demand_output_count)
        .ok_or(Sr2NonInjectivityFailureV3::CapacityOverflow)?;
    let clause_role_tag_capacity = restricted_cost_basis_count
        .checked_mul(recognized_role_count)
        .ok_or(Sr2NonInjectivityFailureV3::CapacityOverflow)?;
    if checked_len(clause_role_tags.len())? != clause_role_tag_capacity {
        return Err(Sr2NonInjectivityFailureV3::RestrictedCostBasisMismatch);
    }
    let full_codomain_capacity = clause_role_tag_capacity
        .checked_add(preexisting_demand_output_count)
        .ok_or(Sr2NonInjectivityFailureV3::CapacityOverflow)?;

    let role_cardinalities = marginal_families
        .1
        .iter()
        .map(|fiber| Ok((fiber.role(), checked_len(fiber.family_count())?)))
        .collect::<Result<Vec<_>, Sr2NonInjectivityFailureV3>>()?;
    let witness = select_pigeonhole_witness(
        &role_cardinalities,
        marginal_family_count,
        role_fiber_codomain_capacity,
        full_codomain_capacity,
    )
    .ok_or(Sr2NonInjectivityFailureV3::NoPigeonholeWitness)?;

    let role_fiber_census_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-role-fiber-census/v3",
        &CanonicalSequence(&marginal_families.1),
    );
    let restricted_cost_tag_census_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-restricted-cost-tag-census/v3",
        &RestrictedCostTagMaterial {
            basis: &restricted_cost_basis,
            roles: SR2_LOCAL_ROLE_UNIVERSE_V3,
            tags: &clause_role_tags,
        },
    );
    let full_codomain_material_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-full-codomain-material/v3",
        &FullCodomainMaterial {
            clause_role_tag_census: &restricted_cost_tag_census_digest,
            clause_role_tag_capacity,
            empty_orbit_partition: orbits.empty_orbit_partition_digest(),
            empty_realization_relation: realizations.empty_realization_relation_digest(),
            preexisting_demand_output_count,
            full_codomain_capacity,
        },
    );
    let witness_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-noninjectivity-pigeonhole-witness/v3",
        &PigeonholeWitnessMaterial {
            marginal_semantic_set: marginals.semantic_set_digest(),
            role_fiber_census: &role_fiber_census_digest,
            full_codomain: &full_codomain_material_digest,
            witness: &witness,
        },
    );

    let mut verified = VerifiedSr2NonInjectivityV3 {
        schema_version: SR2_NONINJECTIVITY_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        inventory_manifest_digest: inventory.manifest_digest().clone(),
        inventory_normalizer_protocol_digest: inventory.normalizer_protocol_digest().clone(),
        inventory_digest: inventory.digest().clone(),
        inventory_coverage_digest: inventory.coverage().digest().clone(),
        predecessor_history_digest: inventory.predecessor_history_digest().clone(),
        exact_extension_digest: inventory.exact_extension().digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        new_event: inventory.exact_extension().event().clone(),
        public_clause_census_digest: public_clauses.digest().clone(),
        public_clause_coverage_digest: public_clauses.coverage_digest().clone(),
        marginal_family_set_digest: marginals.digest().clone(),
        marginal_semantic_set_digest: marginals.semantic_set_digest().clone(),
        marginal_rewrite_authority_digest: marginals.rewrite_authority_digest().clone(),
        predecessor_reconstruction_digest: marginals.predecessor_reconstruction_digest().clone(),
        successor_quotient_digest: marginals.successor_quotient_digest().clone(),
        weakening_digest: marginals.weakening_digest().clone(),
        demand_orbit_census_digest: orbits.digest().clone(),
        predecessor_demand_census_digest: orbits.predecessor_demand_census_digest().clone(),
        empty_orbit_partition_digest: orbits.empty_orbit_partition_digest().clone(),
        demand_realization_census_digest: realizations.digest().clone(),
        semantic_seed_base_census_digest: realizations.semantic_seed_base_census_digest().clone(),
        realization_obligation_census_digest: realizations
            .realization_obligation_census_digest()
            .clone(),
        empty_realization_relation_digest: realizations.empty_realization_relation_digest().clone(),
        restricted_cost_basis_digest: restricted_cost.digest().clone(),
        cost_manifest_digest: restricted_cost.cost_manifest_digest().clone(),
        cost_carrier_digest: restricted_cost.carrier_digest().clone(),
        cost_rewrite_authority_digest: restricted_cost.rewrite_authority_digest().clone(),
        cost_v2_certificate_digest: restricted_cost.v2_certificate_digest().clone(),
        recognized_roles: Arc::from(SR2_LOCAL_ROLE_UNIVERSE_V3),
        marginal_families: Arc::from(marginal_families.0.into_boxed_slice()),
        role_fibers: Arc::from(marginal_families.1.into_boxed_slice()),
        restricted_cost_basis: Arc::from(restricted_cost_basis.into_boxed_slice()),
        clause_role_tags: Arc::from(clause_role_tags.into_boxed_slice()),
        marginal_family_count,
        restricted_cost_basis_count,
        recognized_role_count,
        preexisting_demand_output_count,
        role_fiber_codomain_capacity,
        clause_role_tag_capacity,
        full_codomain_capacity,
        role_fiber_census_digest,
        restricted_cost_tag_census_digest,
        full_codomain_material_digest,
        witness,
        witness_digest,
        complete: private::CompleteSr2NonInjectivityProof,
        digest: Digest::of_bytes(b"pending verified SR2 noninjectivity v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-sr2-noninjectivity/v3",
        &verified,
    );
    Ok(verified)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_sr2_noninjectivity_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
    restricted_cost: &VerifiedRestrictedKernelCostBasisV3,
) -> AuditDecision<VerifiedSr2NonInjectivityV3> {
    match diagnose_sr2_noninjectivity_v3(
        v3_manifest,
        inventory,
        public_clauses,
        marginals,
        orbits,
        realizations,
        restricted_cost,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_exact_chain(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
    restricted_cost: &VerifiedRestrictedKernelCostBasisV3,
) -> Result<(), Sr2NonInjectivityFailureV3> {
    let manifest_digest = v3_manifest.candidate_digest();
    if public_clauses.inventory_digest() != inventory.digest()
        || public_clauses.inventory_coverage_digest() != inventory.coverage().digest()
        || marginals.semantic_manifest_digest() != manifest_digest
        || marginals.inventory_digest() != inventory.digest()
        || marginals.exact_extension_digest() != inventory.exact_extension().digest()
        || marginals.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || marginals.successor_boundary_digest() != inventory.successor_boundary().digest()
        || marginals.new_event() != inventory.exact_extension().event()
        || orbits.semantic_manifest_digest() != manifest_digest
        || orbits.inventory_digest() != inventory.digest()
        || orbits.inventory_coverage_digest() != inventory.coverage().digest()
        || orbits.predecessor_history_digest() != inventory.predecessor_history_digest()
        || orbits.exact_extension_digest() != inventory.exact_extension().digest()
        || orbits.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || orbits.successor_boundary_digest() != inventory.successor_boundary().digest()
        || orbits.new_event() != inventory.exact_extension().event()
        || orbits.marginal_family_set_digest() != marginals.digest()
        || orbits.marginal_semantic_set_digest() != marginals.semantic_set_digest()
        || realizations.semantic_manifest_digest() != manifest_digest
        || realizations.inventory_digest() != inventory.digest()
        || realizations.inventory_coverage_digest() != inventory.coverage().digest()
        || realizations.predecessor_history_digest() != inventory.predecessor_history_digest()
        || realizations.exact_extension_digest() != inventory.exact_extension().digest()
        || realizations.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || realizations.successor_boundary_digest() != inventory.successor_boundary().digest()
        || realizations.new_event() != inventory.exact_extension().event()
        || realizations.public_clause_census_digest() != public_clauses.digest()
        || realizations.marginal_family_set_digest() != marginals.digest()
        || realizations.marginal_semantic_set_digest() != marginals.semantic_set_digest()
        || realizations.demand_orbit_census_digest() != orbits.digest()
        || realizations.predecessor_demand_census_digest()
            != orbits.predecessor_demand_census_digest()
        || realizations.empty_orbit_partition_digest() != orbits.empty_orbit_partition_digest()
        || restricted_cost.semantic_manifest_digest() != manifest_digest
        || restricted_cost.inventory_digest() != inventory.digest()
        || restricted_cost.public_clause_census_digest() != public_clauses.digest()
        || restricted_cost.rewrite_authority_digest() != marginals.rewrite_authority_digest()
    {
        return Err(Sr2NonInjectivityFailureV3::ChainBindingMismatch);
    }
    Ok(())
}

fn verify_complete_empty_demand_surface(
    inventory: &VerifiedPublicAuditInventoryV1,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
) -> Result<(), Sr2NonInjectivityFailureV3> {
    if !inventory.predecessor_demand_contracts().is_empty()
        || inventory.coverage().predecessor_demand_contract_count() != 0
        || inventory
            .equations()
            .iter()
            .any(|equation| equation.demand_port().is_some())
        || !orbits.is_empty()
        || orbits.predecessor_demand_contract_count() != 0
        || orbits.orbit_count() != 0
        || !realizations.is_empty()
        || realizations.realization_obligation_count() != 0
        || realizations.realization_count() != 0
    {
        return Err(Sr2NonInjectivityFailureV3::NonEmptyDemandSurface);
    }
    Ok(())
}

fn derive_role_fibers(
    marginals: &VerifiedMarginalFamilySetV3,
) -> Result<(Vec<FamilyClassIdV3>, Vec<VerifiedSr2RoleFiberV3>), Sr2NonInjectivityFailureV3> {
    let declared_ids = marginals
        .marginal_ids()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if declared_ids.len() != marginals.marginal_ids().len()
        || declared_ids.len() != marginals.marginals().len()
        || declared_ids.len() != marginals.marginal_count()
    {
        return Err(Sr2NonInjectivityFailureV3::MarginalCoverageMismatch);
    }

    let role_universe = SR2_LOCAL_ROLE_UNIVERSE_V3
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if role_universe.len() != SR2_LOCAL_ROLE_UNIVERSE_V3.len() {
        return Err(Sr2NonInjectivityFailureV3::MarginalCoverageMismatch);
    }
    let mut by_role = SR2_LOCAL_ROLE_UNIVERSE_V3
        .iter()
        .copied()
        .map(|role| (role, Vec::new()))
        .collect::<BTreeMap<_, _>>();
    let mut observed_ids = BTreeSet::new();
    for family in marginals.marginals() {
        if !declared_ids.contains(family.id())
            || !observed_ids.insert(family.id().clone())
            || !role_universe.contains(&family.role())
        {
            return Err(Sr2NonInjectivityFailureV3::MarginalCoverageMismatch);
        }
        by_role
            .get_mut(&family.role())
            .ok_or(Sr2NonInjectivityFailureV3::MarginalCoverageMismatch)?
            .push(family.id().clone());
    }
    if observed_ids != declared_ids {
        return Err(Sr2NonInjectivityFailureV3::MarginalCoverageMismatch);
    }

    let mut role_fibers = Vec::with_capacity(SR2_LOCAL_ROLE_UNIVERSE_V3.len());
    let mut covered_count = 0_usize;
    for role in SR2_LOCAL_ROLE_UNIVERSE_V3 {
        let mut families = by_role
            .remove(role)
            .ok_or(Sr2NonInjectivityFailureV3::MarginalCoverageMismatch)?;
        families.sort();
        covered_count = covered_count
            .checked_add(families.len())
            .ok_or(Sr2NonInjectivityFailureV3::CapacityOverflow)?;
        role_fibers.push(VerifiedSr2RoleFiberV3 {
            role: *role,
            families: Arc::from(families.into_boxed_slice()),
        });
    }
    if covered_count != declared_ids.len() || !by_role.is_empty() {
        return Err(Sr2NonInjectivityFailureV3::MarginalCoverageMismatch);
    }
    Ok((declared_ids.into_iter().collect(), role_fibers))
}

fn derive_restricted_cost_tags(
    public_clauses: &VerifiedPublicClauseCensusV1,
    restricted_cost: &VerifiedRestrictedKernelCostBasisV3,
) -> Result<
    (
        Vec<Sr2RestrictedCostBasisCellV3>,
        Vec<Sr2ClauseRoleTagCellV3>,
    ),
    Sr2NonInjectivityFailureV3,
> {
    let public_clause_ids = public_clauses
        .clauses()
        .iter()
        .map(|clause| clause.id().clone())
        .collect::<BTreeSet<_>>();
    if public_clause_ids.len() != public_clauses.clauses().len()
        || restricted_cost.kernel_cost() as usize != restricted_cost.basis_classes().len()
        || restricted_cost.clause_count() != restricted_cost.basis_classes().len()
    {
        return Err(Sr2NonInjectivityFailureV3::RestrictedCostBasisMismatch);
    }

    let mut class_ids = BTreeSet::new();
    let mut singleton_members = BTreeSet::new();
    let mut basis = Vec::with_capacity(restricted_cost.basis_classes().len());
    for class in restricted_cost.basis_classes() {
        let [member] = class.members() else {
            return Err(Sr2NonInjectivityFailureV3::RestrictedCostBasisMismatch);
        };
        if !public_clause_ids.contains(member)
            || !class_ids.insert(class.id().clone())
            || !singleton_members.insert(member.clone())
        {
            return Err(Sr2NonInjectivityFailureV3::RestrictedCostBasisMismatch);
        }
        basis.push(Sr2RestrictedCostBasisCellV3 {
            class: class.id().clone(),
            singleton_member: member.clone(),
        });
    }
    basis.sort_by(|left, right| left.class.cmp(&right.class));

    let mut tags = Vec::new();
    for cell in &basis {
        for role in SR2_LOCAL_ROLE_UNIVERSE_V3 {
            tags.push(Sr2ClauseRoleTagCellV3 {
                clause: cell.singleton_member.clone(),
                role: *role,
            });
        }
    }
    Ok((basis, tags))
}

fn checked_len(value: usize) -> Result<u64, Sr2NonInjectivityFailureV3> {
    u64::try_from(value).map_err(|_| Sr2NonInjectivityFailureV3::CapacityOverflow)
}

/// Select the strongest deterministic finite pigeonhole witness: the largest
/// overfull role fiber (lowest role on a tie), then the full codomain.
fn select_pigeonhole_witness(
    role_fibers: &[(LocalRoleV1, u64)],
    total_domain_count: u64,
    role_fiber_codomain_capacity: u64,
    full_codomain_capacity: u64,
) -> Option<Sr2PigeonholeWitnessV3> {
    let mut best_role_fiber: Option<(LocalRoleV1, u64)> = None;
    for &(role, domain_count) in role_fibers {
        if domain_count <= role_fiber_codomain_capacity {
            continue;
        }
        let replace = best_role_fiber.is_none_or(|(best_role, best_count)| {
            domain_count > best_count || (domain_count == best_count && role < best_role)
        });
        if replace {
            best_role_fiber = Some((role, domain_count));
        }
    }
    if let Some((role, domain_count)) = best_role_fiber {
        return Some(Sr2PigeonholeWitnessV3::RoleFiber {
            role,
            domain_count,
            codomain_capacity: role_fiber_codomain_capacity,
        });
    }
    (total_domain_count > full_codomain_capacity).then_some(Sr2PigeonholeWitnessV3::TotalCapacity {
        domain_count: total_domain_count,
        codomain_capacity: full_codomain_capacity,
    })
}

struct RestrictedCostTagMaterial<'a> {
    basis: &'a [Sr2RestrictedCostBasisCellV3],
    roles: &'a [LocalRoleV1],
    tags: &'a [Sr2ClauseRoleTagCellV3],
}

impl CanonicalEncode for RestrictedCostTagMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.basis);
        encoder.sequence(self.roles);
        encoder.sequence(self.tags);
    }
}

struct FullCodomainMaterial<'a> {
    clause_role_tag_census: &'a Digest,
    clause_role_tag_capacity: u64,
    empty_orbit_partition: &'a Digest,
    empty_realization_relation: &'a Digest,
    preexisting_demand_output_count: u64,
    full_codomain_capacity: u64,
}

impl CanonicalEncode for FullCodomainMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.clause_role_tag_census.encode_canonical(encoder);
        encoder.u64(self.clause_role_tag_capacity);
        self.empty_orbit_partition.encode_canonical(encoder);
        self.empty_realization_relation.encode_canonical(encoder);
        encoder.u64(self.preexisting_demand_output_count);
        encoder.u64(self.full_codomain_capacity);
    }
}

struct PigeonholeWitnessMaterial<'a> {
    marginal_semantic_set: &'a Digest,
    role_fiber_census: &'a Digest,
    full_codomain: &'a Digest,
    witness: &'a Sr2PigeonholeWitnessV3,
}

impl CanonicalEncode for PigeonholeWitnessMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.marginal_semantic_set.encode_canonical(encoder);
        self.role_fiber_census.encode_canonical(encoder);
        self.full_codomain.encode_canonical(encoder);
        self.witness.encode_canonical(encoder);
    }
}

struct CanonicalSequence<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSequence<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_fiber_witness_is_preferred_and_deterministic() {
        let fibers = [
            (LocalRoleV1::SupportAction, 6),
            (LocalRoleV1::KernelHead, 6),
            (LocalRoleV1::AdjointMate, 1),
            (LocalRoleV1::Coherence, 0),
        ];
        assert_eq!(
            select_pigeonhole_witness(&fibers, 13, 3, 12),
            Some(Sr2PigeonholeWitnessV3::RoleFiber {
                role: LocalRoleV1::KernelHead,
                domain_count: 6,
                codomain_capacity: 3,
            })
        );
    }

    #[test]
    fn total_capacity_is_used_only_without_an_overfull_role_fiber() {
        let fibers = [
            (LocalRoleV1::KernelHead, 2),
            (LocalRoleV1::AdjointMate, 2),
            (LocalRoleV1::SupportAction, 3),
            (LocalRoleV1::Coherence, 3),
        ];
        assert_eq!(
            select_pigeonhole_witness(&fibers, 10, 3, 8),
            Some(Sr2PigeonholeWitnessV3::TotalCapacity {
                domain_count: 10,
                codomain_capacity: 8,
            })
        );
        assert_eq!(select_pigeonhole_witness(&fibers, 8, 3, 8), None);
    }
}
