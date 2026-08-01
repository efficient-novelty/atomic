//! Exact pre-SR2 dependency-support census for the V3 semantic audit.
//!
//! This stage reconstructs the complete clause support of every raw member
//! of every exact marginal family class.  It deliberately stops before
//! choosing a principal source or consulting a kernel-cost basis.  The exact
//! empty demand-orbit and realization authorities prove that the current
//! support surface contains no pre-existing demand-output alternative.
//!
//! The capability mints no provenance tag, SR2 injection, `nu`, cost, live
//! value, or Selective-Law authority.

use crate::demand_orbit_census_v3::VerifiedDemandOrbitCensusV3;
use crate::demand_realization_census_v3::VerifiedDemandRealizationCensusV3;
use crate::family_quotient_v3::{
    FamilyClassIdV3, NormalizedFamilyIdV3, VerifiedFamilyClassV3, VerifiedFamilyQuotientV3,
    derive_v3_classes, derive_v3_family_ids,
};
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::marginal_family_set_v3::VerifiedMarginalFamilySetV3;
use crate::model::{
    ClauseIdV1, EventIdV1, FamilyConstructorV1, LocalRoleV1, PublicSupportV1, RawFamilyIdV1,
    RawFamilyV1, SeedIdV1, SemanticSchemaSeedV1,
};
use crate::semantic_authority::{
    PublicClauseSubjectV1, VerifiedPublicClauseCensusV1, VerifiedPublicClauseIdentityV1,
};
use crate::semantic_authority_v3::{
    PublicSemanticSeedSubjectV3, VerifiedPreQ0SemanticSeedV3, VerifiedSemanticSeedBaseCensusV3,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const SR2_DEPENDENCY_SUPPORT_SCHEMA_VERSION_V3: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sr2DependencySupportFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    NonEmptyDemandSurface,
    RawIdentityMismatch,
    QuotientCoverageMismatch,
    MarginalCoverageMismatch,
    SeedLineageMismatch,
    ClauseCoverageMismatch,
    RecursiveSupportMismatch,
}

impl std::fmt::Display for Sr2DependencySupportFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifest is not the exact registered lambda/unit V3 proposal",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the inventory, clause, seed, quotient, marginal, orbit, and realization authorities are not one exact V3 chain",
            ),
            Self::NonEmptyDemandSurface => formatter.write_str(
                "this pre-SR2 support census accepts only the exact empty demand-orbit and realization base",
            ),
            Self::RawIdentityMismatch => formatter.write_str(
                "the authorized carrier cannot be reconstructed on the V3 raw-family identity surface",
            ),
            Self::QuotientCoverageMismatch => formatter.write_str(
                "the V3 quotient classes do not partition the authorized raw-family carrier exactly",
            ),
            Self::MarginalCoverageMismatch => formatter.write_str(
                "the supplied marginal classes are not the exact post-weakening marginal set",
            ),
            Self::SeedLineageMismatch => formatter.write_str(
                "an authorized carrier seed does not preserve the exact V3 seed and public-clause lineage",
            ),
            Self::ClauseCoverageMismatch => formatter.write_str(
                "recursive family support refers outside the complete public-clause census",
            ),
            Self::RecursiveSupportMismatch => formatter.write_str(
                "a raw-family constructor, role, support, or class member has incomplete recursive support",
            ),
        }
    }
}

impl std::error::Error for Sr2DependencySupportFailureV3 {}

impl Sr2DependencySupportFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        AuditDecision::Unknown(AuditUnknownReason::MissingSr2ProvenanceAssignment)
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteSr2DependencySupportCoverage;
}

/// Exhaustive, class-level clause support derived before any principal-source
/// or cost-basis selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSr2FamilyDependencySupportV3 {
    family: FamilyClassIdV3,
    role: LocalRoleV1,
    raw_member_count: u64,
    all_clauses: Arc<[ClauseIdV1]>,
    candidate_local_clauses: Arc<[ClauseIdV1]>,
    raw_member_support_digest: Digest,
    digest: Digest,
}

impl VerifiedSr2FamilyDependencySupportV3 {
    pub fn family(&self) -> &FamilyClassIdV3 {
        &self.family
    }

    pub fn role(&self) -> LocalRoleV1 {
        self.role
    }

    pub fn raw_member_count(&self) -> usize {
        self.raw_member_count as usize
    }

    pub fn all_clauses(&self) -> &[ClauseIdV1] {
        &self.all_clauses
    }

    pub fn candidate_local_clauses(&self) -> &[ClauseIdV1] {
        &self.candidate_local_clauses
    }

    pub fn raw_member_support_digest(&self) -> &Digest {
        &self.raw_member_support_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSr2FamilyDependencySupportV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.family.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
        encoder.u64(self.raw_member_count);
        encoder.sequence(&self.all_clauses);
        encoder.sequence(&self.candidate_local_clauses);
        self.raw_member_support_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RawMemberDependencySupportV3 {
    family: FamilyClassIdV3,
    member: NormalizedFamilyIdV3,
    all_clauses: Arc<[ClauseIdV1]>,
    candidate_local_clauses: Arc<[ClauseIdV1]>,
}

impl CanonicalEncode for RawMemberDependencySupportV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.family.encode_canonical(encoder);
        self.member.encode_canonical(encoder);
        encoder.sequence(&self.all_clauses);
        encoder.sequence(&self.candidate_local_clauses);
    }
}

/// Opaque proof of exhaustive recursive clause support for the complete V3
/// marginal-family set.  This is a pre-SR2 structural capability only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSr2DependencySupportCensusV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    predecessor_history_digest: Digest,
    exact_extension_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    new_event: EventIdV1,
    public_clause_census_digest: Digest,
    semantic_seed_base_census_digest: Digest,
    successor_quotient_digest: Digest,
    marginal_family_set_digest: Digest,
    marginal_semantic_set_digest: Digest,
    demand_orbit_census_digest: Digest,
    demand_realization_census_digest: Digest,
    authorized_carrier_digest: Digest,
    raw_identity_digest: Digest,
    quotient_partition_digest: Digest,
    raw_member_support_census_digest: Digest,
    family_support_census_digest: Digest,
    empty_preexisting_output_digest: Digest,
    marginal_family_count: u64,
    marginal_raw_member_count: u64,
    preexisting_output_count: u64,
    family_support: Arc<[VerifiedSr2FamilyDependencySupportV3]>,
    raw_member_support: Arc<[RawMemberDependencySupportV3]>,
    complete: private::CompleteSr2DependencySupportCoverage,
    digest: Digest,
}

impl VerifiedSr2DependencySupportCensusV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn inventory_coverage_digest(&self) -> &Digest {
        &self.inventory_coverage_digest
    }

    pub fn predecessor_history_digest(&self) -> &Digest {
        &self.predecessor_history_digest
    }

    pub fn exact_extension_digest(&self) -> &Digest {
        &self.exact_extension_digest
    }

    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn new_event(&self) -> &EventIdV1 {
        &self.new_event
    }

    pub fn public_clause_census_digest(&self) -> &Digest {
        &self.public_clause_census_digest
    }

    pub fn semantic_seed_base_census_digest(&self) -> &Digest {
        &self.semantic_seed_base_census_digest
    }

    pub fn successor_quotient_digest(&self) -> &Digest {
        &self.successor_quotient_digest
    }

    pub fn marginal_family_set_digest(&self) -> &Digest {
        &self.marginal_family_set_digest
    }

    pub fn marginal_semantic_set_digest(&self) -> &Digest {
        &self.marginal_semantic_set_digest
    }

    pub fn demand_orbit_census_digest(&self) -> &Digest {
        &self.demand_orbit_census_digest
    }

    pub fn demand_realization_census_digest(&self) -> &Digest {
        &self.demand_realization_census_digest
    }

    pub fn authorized_carrier_digest(&self) -> &Digest {
        &self.authorized_carrier_digest
    }

    pub fn raw_identity_digest(&self) -> &Digest {
        &self.raw_identity_digest
    }

    pub fn raw_member_support_census_digest(&self) -> &Digest {
        &self.raw_member_support_census_digest
    }

    pub fn family_support_census_digest(&self) -> &Digest {
        &self.family_support_census_digest
    }

    /// Structural census cardinality only. This is not an issued `nu`.
    pub fn marginal_family_count(&self) -> usize {
        self.marginal_family_count as usize
    }

    pub fn marginal_raw_member_count(&self) -> usize {
        self.marginal_raw_member_count as usize
    }

    pub fn preexisting_output_count(&self) -> usize {
        self.preexisting_output_count as usize
    }

    pub fn family_support(&self) -> &[VerifiedSr2FamilyDependencySupportV3] {
        &self.family_support
    }

    pub fn support_for_family(
        &self,
        family: &FamilyClassIdV3,
    ) -> Option<&VerifiedSr2FamilyDependencySupportV3> {
        self.family_support
            .binary_search_by(|support| support.family.cmp(family))
            .ok()
            .map(|index| &self.family_support[index])
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSr2DependencySupportCensusV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.inventory_coverage_digest.encode_canonical(encoder);
        self.predecessor_history_digest.encode_canonical(encoder);
        self.exact_extension_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.new_event.encode_canonical(encoder);
        self.public_clause_census_digest.encode_canonical(encoder);
        self.semantic_seed_base_census_digest
            .encode_canonical(encoder);
        self.successor_quotient_digest.encode_canonical(encoder);
        self.marginal_family_set_digest.encode_canonical(encoder);
        self.marginal_semantic_set_digest.encode_canonical(encoder);
        self.demand_orbit_census_digest.encode_canonical(encoder);
        self.demand_realization_census_digest
            .encode_canonical(encoder);
        self.authorized_carrier_digest.encode_canonical(encoder);
        self.raw_identity_digest.encode_canonical(encoder);
        self.quotient_partition_digest.encode_canonical(encoder);
        self.raw_member_support_census_digest
            .encode_canonical(encoder);
        self.family_support_census_digest.encode_canonical(encoder);
        self.empty_preexisting_output_digest
            .encode_canonical(encoder);
        encoder.u64(self.marginal_family_count);
        encoder.u64(self.marginal_raw_member_count);
        encoder.u64(self.preexisting_output_count);
        encoder.sequence(&self.family_support);
        encoder.sequence(&self.raw_member_support);
        encoder.tag(1);
    }
}

/// Derive the complete pre-SR2 dependency-support surface.  Callers supply no
/// family list, clause support, demand-output set, cost basis, source choice,
/// or completeness assertion.
#[allow(clippy::too_many_arguments)]
pub fn diagnose_sr2_dependency_support_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    seed_base: &VerifiedSemanticSeedBaseCensusV3,
    successor_quotient: &VerifiedFamilyQuotientV3,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
) -> Result<VerifiedSr2DependencySupportCensusV3, Sr2DependencySupportFailureV3> {
    if v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3() {
        return Err(Sr2DependencySupportFailureV3::ExactManifestIdentityMismatch);
    }
    verify_chain_bindings(
        v3_manifest,
        inventory,
        public_clauses,
        seed_base,
        successor_quotient,
        marginals,
        orbits,
        realizations,
    )?;
    verify_empty_demand_surface(inventory, seed_base, orbits, realizations)?;

    let clause_index = unique_clause_index(public_clauses.clauses())?;
    let clause_origins = clause_index
        .iter()
        .map(|(id, clause)| (id.clone(), clause.origin_event().clone()))
        .collect::<BTreeMap<_, _>>();

    let carrier = successor_quotient.authorized_carrier_proof();
    verify_seed_lineage(successor_quotient, seed_base, &clause_index)?;

    let raw_ids = derive_v3_family_ids(v3_manifest, carrier, successor_quotient.seed_images())
        .map_err(|_| Sr2DependencySupportFailureV3::RawIdentityMismatch)?;
    let expected_classes = derive_v3_classes(
        v3_manifest,
        successor_quotient.legacy_quotient_proof(),
        &raw_ids,
    )
    .map_err(|_| Sr2DependencySupportFailureV3::QuotientCoverageMismatch)?;
    if expected_classes != successor_quotient.classes() {
        return Err(Sr2DependencySupportFailureV3::QuotientCoverageMismatch);
    }

    let raw_index = unique_raw_index(carrier.raw_families())?;
    let seed_index = unique_seed_index(carrier.verified_seeds())?;
    let inverse_raw_ids = invert_raw_ids(&raw_ids)?;
    let class_index = verify_exact_quotient_partition(successor_quotient.classes(), &raw_ids)?;
    let marginal_classes = verify_exact_marginal_coverage(marginals, &class_index)?;

    let mut memo = BTreeMap::new();
    let mut family_support = Vec::with_capacity(marginal_classes.len());
    let mut raw_member_support = Vec::new();
    for class in marginal_classes {
        let mut class_witnesses = Vec::with_capacity(class.members().len());
        for member in class.members() {
            let legacy_id = inverse_raw_ids
                .get(member)
                .ok_or(Sr2DependencySupportFailureV3::RawIdentityMismatch)?;
            let raw = raw_index
                .get(legacy_id)
                .copied()
                .ok_or(Sr2DependencySupportFailureV3::RawIdentityMismatch)?;
            if raw.role != class.role() {
                return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
            }
            let derived = derive_recursive_support(
                legacy_id,
                &raw_index,
                &seed_index,
                &mut memo,
                &mut BTreeSet::new(),
            )?;
            if derived
                .clauses
                .iter()
                .any(|clause| !clause_index.contains_key(clause))
            {
                return Err(Sr2DependencySupportFailureV3::ClauseCoverageMismatch);
            }
            let candidate_local = candidate_local_clauses(
                &derived.clauses,
                &clause_origins,
                inventory.exact_extension().event(),
            )?;
            let witness = RawMemberDependencySupportV3 {
                family: class.id().clone(),
                member: member.clone(),
                all_clauses: Arc::from(derived.clauses.into_iter().collect::<Vec<_>>()),
                candidate_local_clauses: Arc::from(candidate_local.into_iter().collect::<Vec<_>>()),
            };
            class_witnesses.push(witness.clone());
            raw_member_support.push(witness);
        }
        class_witnesses.sort_by(|left, right| left.member.cmp(&right.member));
        let (all_clauses, candidate_local) = aggregate_class_support(class, &class_witnesses)?;
        if class
            .source_clause()
            .is_some_and(|source| !all_clauses.contains(source))
        {
            return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
        }
        let raw_member_support_digest = Digest::of_canonical(
            "pen-semantic-audit/sr2-raw-member-dependency-support/v3",
            &CanonicalSequence(&class_witnesses),
        );
        let mut support = VerifiedSr2FamilyDependencySupportV3 {
            family: class.id().clone(),
            role: class.role(),
            raw_member_count: class_witnesses.len() as u64,
            all_clauses: Arc::from(all_clauses.into_iter().collect::<Vec<_>>()),
            candidate_local_clauses: Arc::from(candidate_local.into_iter().collect::<Vec<_>>()),
            raw_member_support_digest,
            digest: Digest::of_bytes(b"pending verified SR2 family dependency support v3"),
        };
        support.digest = Digest::of_canonical(
            "pen-semantic-audit/verified-sr2-family-dependency-support/v3",
            &support,
        );
        family_support.push(support);
    }
    family_support.sort_by(|left, right| left.family.cmp(&right.family));
    raw_member_support
        .sort_by(|left, right| (&left.family, &left.member).cmp(&(&right.family, &right.member)));

    if family_support.len() != marginals.marginal_count()
        || raw_member_support.len() != marginals.marginal_raw_member_count()
    {
        return Err(Sr2DependencySupportFailureV3::MarginalCoverageMismatch);
    }

    let raw_identity_entries = raw_ids
        .iter()
        .map(|(legacy, semantic)| RawIdentityEntryV3 {
            legacy: legacy.clone(),
            semantic: semantic.clone(),
        })
        .collect::<Vec<_>>();
    let raw_identity_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-authorized-raw-identity/v3",
        &CanonicalSequence(&raw_identity_entries),
    );
    let quotient_partition_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-exact-quotient-partition/v3",
        &CanonicalSequence(successor_quotient.classes()),
    );
    let raw_member_support_census_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-raw-member-support-census/v3",
        &CanonicalSequence(&raw_member_support),
    );
    let family_support_census_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-family-support-census/v3",
        &CanonicalSequence(&family_support),
    );
    let empty_preexisting_output_digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-empty-preexisting-output-support/v3",
        &EmptyPreexistingOutputMaterial {
            demand_orbit_census: orbits.digest(),
            demand_realization_census: realizations.digest(),
            predecessor_demand_census: orbits.predecessor_demand_census_digest(),
            output_count: 0,
        },
    );

    let mut verified = VerifiedSr2DependencySupportCensusV3 {
        schema_version: SR2_DEPENDENCY_SUPPORT_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        inventory_digest: inventory.digest().clone(),
        inventory_coverage_digest: inventory.coverage().digest().clone(),
        predecessor_history_digest: inventory.predecessor_history_digest().clone(),
        exact_extension_digest: inventory.exact_extension().digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        new_event: inventory.exact_extension().event().clone(),
        public_clause_census_digest: public_clauses.digest().clone(),
        semantic_seed_base_census_digest: seed_base.digest().clone(),
        successor_quotient_digest: successor_quotient.digest().clone(),
        marginal_family_set_digest: marginals.digest().clone(),
        marginal_semantic_set_digest: marginals.semantic_set_digest().clone(),
        demand_orbit_census_digest: orbits.digest().clone(),
        demand_realization_census_digest: realizations.digest().clone(),
        authorized_carrier_digest: successor_quotient.authorized_carrier_digest().clone(),
        raw_identity_digest,
        quotient_partition_digest,
        raw_member_support_census_digest,
        family_support_census_digest,
        empty_preexisting_output_digest,
        marginal_family_count: family_support.len() as u64,
        marginal_raw_member_count: raw_member_support.len() as u64,
        preexisting_output_count: 0,
        family_support: Arc::from(family_support.into_boxed_slice()),
        raw_member_support: Arc::from(raw_member_support.into_boxed_slice()),
        complete: private::CompleteSr2DependencySupportCoverage,
        digest: Digest::of_bytes(b"pending verified SR2 dependency support census v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-sr2-dependency-support-census/v3",
        &verified,
    );
    Ok(verified)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_sr2_dependency_support_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    seed_base: &VerifiedSemanticSeedBaseCensusV3,
    successor_quotient: &VerifiedFamilyQuotientV3,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
) -> AuditDecision<VerifiedSr2DependencySupportCensusV3> {
    match diagnose_sr2_dependency_support_v3(
        v3_manifest,
        inventory,
        public_clauses,
        seed_base,
        successor_quotient,
        marginals,
        orbits,
        realizations,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_chain_bindings(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    seed_base: &VerifiedSemanticSeedBaseCensusV3,
    successor_quotient: &VerifiedFamilyQuotientV3,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
) -> Result<(), Sr2DependencySupportFailureV3> {
    let manifest_digest = v3_manifest.candidate_digest();
    if public_clauses.inventory_digest() != inventory.digest()
        || public_clauses.inventory_coverage_digest() != inventory.coverage().digest()
        || seed_base.semantic_manifest_digest() != manifest_digest
        || seed_base.inventory_digest() != inventory.digest()
        || seed_base.public_clause_census_digest() != public_clauses.digest()
        || successor_quotient.semantic_manifest_digest() != manifest_digest
        || successor_quotient.inventory_digest() != inventory.digest()
        || successor_quotient.signature_digest() != inventory.successor_boundary().digest()
        || marginals.semantic_manifest_digest() != manifest_digest
        || marginals.inventory_digest() != inventory.digest()
        || marginals.exact_extension_digest() != inventory.exact_extension().digest()
        || marginals.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || marginals.successor_boundary_digest() != inventory.successor_boundary().digest()
        || marginals.new_event() != inventory.exact_extension().event()
        || marginals.successor_quotient_digest() != successor_quotient.digest()
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
        || realizations.semantic_seed_base_census_digest() != seed_base.digest()
        || realizations.public_clause_census_digest() != public_clauses.digest()
        || realizations.marginal_family_set_digest() != marginals.digest()
        || realizations.marginal_semantic_set_digest() != marginals.semantic_set_digest()
        || realizations.demand_orbit_census_digest() != orbits.digest()
        || realizations.predecessor_demand_census_digest()
            != orbits.predecessor_demand_census_digest()
        || realizations.empty_orbit_partition_digest() != orbits.empty_orbit_partition_digest()
    {
        return Err(Sr2DependencySupportFailureV3::ChainBindingMismatch);
    }
    Ok(())
}

fn verify_empty_demand_surface(
    inventory: &VerifiedPublicAuditInventoryV1,
    seed_base: &VerifiedSemanticSeedBaseCensusV3,
    orbits: &VerifiedDemandOrbitCensusV3,
    realizations: &VerifiedDemandRealizationCensusV3,
) -> Result<(), Sr2DependencySupportFailureV3> {
    if !inventory.predecessor_demand_contracts().is_empty()
        || inventory
            .equations()
            .iter()
            .any(|equation| equation.demand_port().is_some())
        || !seed_base.equation_port_metadata().is_empty()
        || !orbits.is_empty()
        || orbits.predecessor_demand_contract_count() != 0
        || orbits.orbit_count() != 0
        || !realizations.is_empty()
        || realizations.realization_obligation_count() != 0
        || realizations.realization_count() != 0
    {
        return Err(Sr2DependencySupportFailureV3::NonEmptyDemandSurface);
    }
    Ok(())
}

fn unique_clause_index(
    clauses: &[VerifiedPublicClauseIdentityV1],
) -> Result<BTreeMap<ClauseIdV1, &VerifiedPublicClauseIdentityV1>, Sr2DependencySupportFailureV3> {
    let mut index = BTreeMap::new();
    for clause in clauses {
        if index.insert(clause.id().clone(), clause).is_some() {
            return Err(Sr2DependencySupportFailureV3::ClauseCoverageMismatch);
        }
    }
    Ok(index)
}

fn verify_seed_lineage(
    quotient: &VerifiedFamilyQuotientV3,
    seed_base: &VerifiedSemanticSeedBaseCensusV3,
    clause_index: &BTreeMap<ClauseIdV1, &VerifiedPublicClauseIdentityV1>,
) -> Result<(), Sr2DependencySupportFailureV3> {
    let mut semantic_seeds = BTreeMap::new();
    for seed in seed_base.seeds() {
        if semantic_seeds.insert(seed.id().clone(), seed).is_some() {
            return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
        }
    }
    let carrier = quotient.authorized_carrier_proof();
    let mut authorized_seeds = BTreeMap::new();
    for seed in carrier.verified_seeds() {
        if authorized_seeds.insert(seed.id.clone(), seed).is_some() {
            return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
        }
    }
    let mut image_by_normalized = BTreeMap::new();
    let mut image_sources = BTreeSet::new();
    let mut image_semantic = BTreeSet::new();
    for image in quotient.seed_images() {
        if !image_sources.insert(image.source().clone())
            || !image_semantic.insert(image.semantic().clone())
            || image_by_normalized
                .insert(image.normalized().clone(), image.semantic().clone())
                .is_some()
        {
            return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
        }
    }
    if image_by_normalized.len() != authorized_seeds.len()
        || image_semantic != semantic_seeds.keys().cloned().collect()
    {
        return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
    }
    for (id, legacy) in authorized_seeds {
        let semantic_id = image_by_normalized
            .get(&id)
            .ok_or(Sr2DependencySupportFailureV3::SeedLineageMismatch)?;
        let semantic = semantic_seeds
            .get(semantic_id)
            .copied()
            .ok_or(Sr2DependencySupportFailureV3::SeedLineageMismatch)?;
        validate_seed_lineage(legacy, semantic, clause_index)?;
    }
    Ok(())
}

fn validate_seed_lineage(
    legacy: &crate::carrier::VerifiedSemanticSeedV1,
    semantic: &VerifiedPreQ0SemanticSeedV3,
    clause_index: &BTreeMap<ClauseIdV1, &VerifiedPublicClauseIdentityV1>,
) -> Result<(), Sr2DependencySupportFailureV3> {
    let clause = clause_index
        .get(semantic.source_clause())
        .copied()
        .ok_or(Sr2DependencySupportFailureV3::SeedLineageMismatch)?;
    let common_matches = legacy.derived_role == semantic.local_role()
        && semantic.source_identity() == clause.source_identity()
        && semantic.origin_event() == clause.origin_event()
        && semantic.structural_support().events() == &clause.public_support().events
        && semantic.structural_support().declarations() == &clause.public_support().declarations
        && clause.public_support().demand_outputs.is_empty();
    let specific_matches = match (&legacy.seed, semantic.subject(), clause.subject()) {
        (
            SemanticSchemaSeedV1::PublicHead(seed),
            PublicSemanticSeedSubjectV3::Declaration {
                declaration,
                public_group,
            },
            PublicClauseSubjectV1::Declaration {
                declaration: clause_declaration,
            },
        ) => {
            seed.source_clause.as_ref() == Some(semantic.source_clause())
                && seed.declaration == *declaration
                && seed.declaration == *clause_declaration
                && seed.origin_event == *semantic.origin_event()
                && seed.judgment.source_identity == *semantic.source_identity()
                && seed.judgment.source == *semantic.source_judgment()
                && seed.claimed_role == semantic.local_role()
                && seed.public_support.events == *semantic.structural_support().events()
                && seed.public_support.declarations == *semantic.structural_support().declarations()
                && seed.public_support.demand_outputs.is_empty()
                && clause.public_group() == public_group
        }
        (
            SemanticSchemaSeedV1::PublicEquation(seed),
            PublicSemanticSeedSubjectV3::Equation {
                equation,
                owner_head,
                public_group,
            },
            PublicClauseSubjectV1::Equation {
                equation: clause_equation,
                owner_head: clause_owner,
            },
        ) => {
            seed.source_clause.as_ref() == Some(semantic.source_clause())
                && seed.equation == *equation
                && seed.equation == *clause_equation
                && seed.owner_head == *owner_head
                && seed.owner_head == *clause_owner
                && seed.origin_event == *semantic.origin_event()
                && seed.judgment.source_identity == *semantic.source_identity()
                && seed.judgment.source == *semantic.source_judgment()
                && seed.claimed_role == semantic.local_role()
                && seed.public_support.events == *semantic.structural_support().events()
                && seed.public_support.declarations == *semantic.structural_support().declarations()
                && seed.public_support.demand_outputs.is_empty()
                && seed.demand_anchor.is_none()
                && clause.public_group() == public_group
        }
        _ => false,
    };
    if !common_matches || !specific_matches {
        return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
    }
    Ok(())
}

fn unique_raw_index(
    raw: &[RawFamilyV1],
) -> Result<BTreeMap<RawFamilyIdV1, &RawFamilyV1>, Sr2DependencySupportFailureV3> {
    let mut index = BTreeMap::new();
    for family in raw {
        if index.insert(family.id.clone(), family).is_some() {
            return Err(Sr2DependencySupportFailureV3::RawIdentityMismatch);
        }
    }
    Ok(index)
}

fn unique_seed_index(
    seeds: &[crate::carrier::VerifiedSemanticSeedV1],
) -> Result<
    BTreeMap<SeedIdV1, &crate::carrier::VerifiedSemanticSeedV1>,
    Sr2DependencySupportFailureV3,
> {
    let mut index = BTreeMap::new();
    for seed in seeds {
        if index.insert(seed.id.clone(), seed).is_some() {
            return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
        }
    }
    Ok(index)
}

fn invert_raw_ids(
    raw_ids: &BTreeMap<RawFamilyIdV1, NormalizedFamilyIdV3>,
) -> Result<BTreeMap<NormalizedFamilyIdV3, RawFamilyIdV1>, Sr2DependencySupportFailureV3> {
    let mut inverse = BTreeMap::new();
    for (legacy, semantic) in raw_ids {
        if inverse.insert(semantic.clone(), legacy.clone()).is_some() {
            return Err(Sr2DependencySupportFailureV3::RawIdentityMismatch);
        }
    }
    Ok(inverse)
}

fn verify_exact_quotient_partition<'a>(
    classes: &'a [VerifiedFamilyClassV3],
    raw_ids: &BTreeMap<RawFamilyIdV1, NormalizedFamilyIdV3>,
) -> Result<BTreeMap<FamilyClassIdV3, &'a VerifiedFamilyClassV3>, Sr2DependencySupportFailureV3> {
    let all_raw = raw_ids.values().cloned().collect::<BTreeSet<_>>();
    let mut class_index = BTreeMap::new();
    let mut covered = BTreeSet::new();
    for class in classes {
        if class.members().is_empty()
            || !class.members().contains(class.representative())
            || class_index.insert(class.id().clone(), class).is_some()
        {
            return Err(Sr2DependencySupportFailureV3::QuotientCoverageMismatch);
        }
        for member in class.members() {
            if !all_raw.contains(member) || !covered.insert(member.clone()) {
                return Err(Sr2DependencySupportFailureV3::QuotientCoverageMismatch);
            }
        }
    }
    if covered != all_raw {
        return Err(Sr2DependencySupportFailureV3::QuotientCoverageMismatch);
    }
    Ok(class_index)
}

fn verify_exact_marginal_coverage<'a>(
    marginals: &'a VerifiedMarginalFamilySetV3,
    class_index: &BTreeMap<FamilyClassIdV3, &'a VerifiedFamilyClassV3>,
) -> Result<Vec<&'a VerifiedFamilyClassV3>, Sr2DependencySupportFailureV3> {
    let marginal_ids = marginals
        .marginal_ids()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if marginal_ids.len() != marginals.marginal_ids().len()
        || marginal_ids.len() != marginals.marginals().len()
        || marginal_ids.len() != marginals.marginal_count()
    {
        return Err(Sr2DependencySupportFailureV3::MarginalCoverageMismatch);
    }
    let mut classes = Vec::with_capacity(marginal_ids.len());
    for marginal in marginals.marginals() {
        if !marginal_ids.contains(marginal.id())
            || class_index.get(marginal.id()).copied() != Some(marginal)
        {
            return Err(Sr2DependencySupportFailureV3::MarginalCoverageMismatch);
        }
        classes.push(marginal);
    }
    classes.sort_by(|left, right| left.id().cmp(right.id()));
    Ok(classes)
}

#[derive(Clone)]
struct DerivedRecursiveSupport {
    clauses: BTreeSet<ClauseIdV1>,
    structural: PublicSupportV1,
}

fn derive_recursive_support(
    id: &RawFamilyIdV1,
    raw_index: &BTreeMap<RawFamilyIdV1, &RawFamilyV1>,
    seed_index: &BTreeMap<SeedIdV1, &crate::carrier::VerifiedSemanticSeedV1>,
    memo: &mut BTreeMap<RawFamilyIdV1, DerivedRecursiveSupport>,
    visiting: &mut BTreeSet<RawFamilyIdV1>,
) -> Result<DerivedRecursiveSupport, Sr2DependencySupportFailureV3> {
    if let Some(support) = memo.get(id) {
        return Ok(support.clone());
    }
    let raw = raw_index
        .get(id)
        .copied()
        .ok_or(Sr2DependencySupportFailureV3::RawIdentityMismatch)?;
    if raw.rank > 2 || !visiting.insert(id.clone()) {
        return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
    }
    let result = (|| {
        let (expected_role, expected_source, support) = match &raw.constructor {
            FamilyConstructorV1::PublicHeadSeed { seed }
            | FamilyConstructorV1::PublicEquationSeed { seed } => {
                if raw.rank != 0 {
                    return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
                }
                let verified = seed_index
                    .get(seed)
                    .copied()
                    .ok_or(Sr2DependencySupportFailureV3::SeedLineageMismatch)?;
                let (constructor_matches, source_clause, structural) = match &verified.seed {
                    SemanticSchemaSeedV1::PublicHead(seed_wire) => (
                        matches!(&raw.constructor, FamilyConstructorV1::PublicHeadSeed { .. }),
                        seed_wire.source_clause.clone(),
                        seed_wire.public_support.clone(),
                    ),
                    SemanticSchemaSeedV1::PublicEquation(seed_wire) => {
                        if seed_wire.demand_anchor.is_some() {
                            return Err(Sr2DependencySupportFailureV3::NonEmptyDemandSurface);
                        }
                        (
                            matches!(
                                &raw.constructor,
                                FamilyConstructorV1::PublicEquationSeed { .. }
                            ),
                            seed_wire.source_clause.clone(),
                            seed_wire.public_support.clone(),
                        )
                    }
                    SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
                        return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
                    }
                };
                let Some(source_clause) = source_clause else {
                    return Err(Sr2DependencySupportFailureV3::SeedLineageMismatch);
                };
                if !constructor_matches || !structural.demand_outputs.is_empty() {
                    return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
                }
                (
                    verified.derived_role,
                    Some(source_clause.clone()),
                    DerivedRecursiveSupport {
                        clauses: BTreeSet::from([source_clause]),
                        structural,
                    },
                )
            }
            FamilyConstructorV1::GenericPublicApplication {
                function, argument, ..
            } => {
                let function_raw = child_raw(raw, function, raw_index)?;
                let argument_raw = child_raw(raw, argument, raw_index)?;
                let function_support =
                    derive_recursive_support(function, raw_index, seed_index, memo, visiting)?;
                let argument_support =
                    derive_recursive_support(argument, raw_index, seed_index, memo, visiting)?;
                if raw.rank != function_raw.rank.max(argument_raw.rank).saturating_add(1) {
                    return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
                }
                (
                    LocalRoleV1::SupportAction,
                    common_option(&function_raw.source_clause, &argument_raw.source_clause),
                    union_recursive_support(function_support, argument_support),
                )
            }
            FamilyConstructorV1::GenericEquationAction {
                equation, context, ..
            } => {
                let equation_raw = child_raw(raw, equation, raw_index)?;
                let context_raw = child_raw(raw, context, raw_index)?;
                let equation_support =
                    derive_recursive_support(equation, raw_index, seed_index, memo, visiting)?;
                let context_support =
                    derive_recursive_support(context, raw_index, seed_index, memo, visiting)?;
                if raw.rank != equation_raw.rank.max(context_raw.rank).saturating_add(1) {
                    return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
                }
                (
                    LocalRoleV1::Coherence,
                    equation_raw.source_clause.clone(),
                    union_recursive_support(equation_support, context_support),
                )
            }
        };
        if raw.role != expected_role
            || raw.source_clause != expected_source
            || raw.demand_anchor.is_some()
            || !raw.public_support.demand_outputs.is_empty()
            || raw.public_support != support.structural
            || raw
                .source_clause
                .as_ref()
                .is_some_and(|clause| !support.clauses.contains(clause))
        {
            return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
        }
        Ok(support)
    })();
    visiting.remove(id);
    let support = result?;
    memo.insert(id.clone(), support.clone());
    Ok(support)
}

fn child_raw<'a>(
    parent: &RawFamilyV1,
    child: &RawFamilyIdV1,
    raw_index: &BTreeMap<RawFamilyIdV1, &'a RawFamilyV1>,
) -> Result<&'a RawFamilyV1, Sr2DependencySupportFailureV3> {
    let child = raw_index
        .get(child)
        .copied()
        .ok_or(Sr2DependencySupportFailureV3::RawIdentityMismatch)?;
    if child.rank >= parent.rank {
        return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
    }
    Ok(child)
}

fn common_option<T: Clone + Eq>(left: &Option<T>, right: &Option<T>) -> Option<T> {
    if left == right { left.clone() } else { None }
}

fn union_recursive_support(
    mut left: DerivedRecursiveSupport,
    right: DerivedRecursiveSupport,
) -> DerivedRecursiveSupport {
    left.clauses.extend(right.clauses);
    left.structural = left.structural.union(&right.structural);
    left
}

fn candidate_local_clauses(
    clauses: &BTreeSet<ClauseIdV1>,
    clause_origins: &BTreeMap<ClauseIdV1, EventIdV1>,
    new_event: &EventIdV1,
) -> Result<BTreeSet<ClauseIdV1>, Sr2DependencySupportFailureV3> {
    let mut candidate_local = BTreeSet::new();
    for clause in clauses {
        let origin = clause_origins
            .get(clause)
            .ok_or(Sr2DependencySupportFailureV3::ClauseCoverageMismatch)?;
        if origin == new_event {
            candidate_local.insert(clause.clone());
        }
    }
    Ok(candidate_local)
}

fn aggregate_class_support(
    class: &VerifiedFamilyClassV3,
    witnesses: &[RawMemberDependencySupportV3],
) -> Result<(BTreeSet<ClauseIdV1>, BTreeSet<ClauseIdV1>), Sr2DependencySupportFailureV3> {
    if witnesses.len() != class.members().len() {
        return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
    }
    let mut seen = BTreeSet::new();
    let mut all = BTreeSet::new();
    let mut candidate_local = BTreeSet::new();
    for witness in witnesses {
        if witness.family != *class.id()
            || !class.members().contains(&witness.member)
            || !seen.insert(witness.member.clone())
        {
            return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
        }
        all.extend(witness.all_clauses.iter().cloned());
        candidate_local.extend(witness.candidate_local_clauses.iter().cloned());
    }
    if seen.len() != class.members().len() {
        return Err(Sr2DependencySupportFailureV3::RecursiveSupportMismatch);
    }
    Ok((all, candidate_local))
}

#[derive(Clone)]
struct RawIdentityEntryV3 {
    legacy: RawFamilyIdV1,
    semantic: NormalizedFamilyIdV3,
}

impl CanonicalEncode for RawIdentityEntryV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.legacy.encode_canonical(encoder);
        self.semantic.encode_canonical(encoder);
    }
}

struct EmptyPreexistingOutputMaterial<'a> {
    demand_orbit_census: &'a Digest,
    demand_realization_census: &'a Digest,
    predecessor_demand_census: &'a Digest,
    output_count: u64,
}

impl CanonicalEncode for EmptyPreexistingOutputMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.demand_orbit_census.encode_canonical(encoder);
        self.demand_realization_census.encode_canonical(encoder);
        self.predecessor_demand_census.encode_canonical(encoder);
        encoder.u64(self.output_count);
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

    fn clause(label: &[u8]) -> ClauseIdV1 {
        ClauseIdV1(Digest::of_domain_bytes("sr2-support-test/clause", label))
    }

    fn event(label: &[u8]) -> EventIdV1 {
        EventIdV1(Digest::of_domain_bytes("sr2-support-test/event", label))
    }

    #[test]
    fn candidate_local_filter_uses_exact_clause_origins() {
        let old = clause(b"old");
        let new_left = clause(b"new-left");
        let new_right = clause(b"new-right");
        let old_event = event(b"old");
        let new_event = event(b"new");
        let clauses = BTreeSet::from([old.clone(), new_left.clone(), new_right.clone()]);
        let origins = BTreeMap::from([
            (old, old_event),
            (new_left.clone(), new_event.clone()),
            (new_right.clone(), new_event.clone()),
        ]);
        assert_eq!(
            candidate_local_clauses(&clauses, &origins, &new_event),
            Ok(BTreeSet::from([new_left, new_right]))
        );
    }

    #[test]
    fn recursive_union_keeps_nonrepresentative_member_support() {
        let representative = BTreeSet::from([clause(b"representative")]);
        let other = BTreeSet::from([clause(b"other")]);
        let mut union = representative.clone();
        union.extend(other.iter().cloned());
        assert_eq!(union.len(), 2);
        assert!(representative.is_subset(&union));
        assert_ne!(representative, union);
        assert!(other.is_subset(&union));
    }

    #[test]
    fn every_failure_maps_to_the_registered_sr2_stage_reason() {
        for failure in [
            Sr2DependencySupportFailureV3::ExactManifestIdentityMismatch,
            Sr2DependencySupportFailureV3::ChainBindingMismatch,
            Sr2DependencySupportFailureV3::NonEmptyDemandSurface,
            Sr2DependencySupportFailureV3::RawIdentityMismatch,
            Sr2DependencySupportFailureV3::QuotientCoverageMismatch,
            Sr2DependencySupportFailureV3::MarginalCoverageMismatch,
            Sr2DependencySupportFailureV3::SeedLineageMismatch,
            Sr2DependencySupportFailureV3::ClauseCoverageMismatch,
            Sr2DependencySupportFailureV3::RecursiveSupportMismatch,
        ] {
            assert!(matches!(
                failure.into_decision::<()>(),
                AuditDecision::Unknown(AuditUnknownReason::MissingSr2ProvenanceAssignment)
            ));
        }
    }
}
