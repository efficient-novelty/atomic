//! Complete empty-base demand-orbit census for the V3 semantic audit.
//!
//! The verified public inventory is the authoritative finite carrier of
//! predecessor demand contracts. Its empty carrier has a uniquely empty
//! quotient, so that base case can be certified without the still-missing
//! typed-reindexing quotient theorem. Any nonempty carrier fails closed.
//!
//! This module never promotes equation-port metadata to semantic authority
//! and mints no demand output, realization, cost, SR2, `nu`, or Selective-Law
//! capability.

use crate::inventory::{VerifiedDemandContractV1, VerifiedPublicAuditInventoryV1};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::marginal_family_set_v3::VerifiedMarginalFamilySetV3;
use crate::model::EventIdV1;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};

pub const DEMAND_ORBIT_CENSUS_SCHEMA_VERSION_V3: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DemandOrbitCensusFailureV3 {
    ExactManifestIdentityMismatch,
    InventoryCoverageMismatch,
    ChainBindingMismatch,
    NonEmptyPredecessorDemandContracts,
    NonEmptyBoundEquationPorts,
}

impl std::fmt::Display for DemandOrbitCensusFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifest is not the exact registered lambda/unit V3 proposal",
            ),
            Self::InventoryCoverageMismatch => formatter.write_str(
                "the verified predecessor-demand coverage does not equal the inventoried carrier",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the verified inventory and marginal-family set are not one exact V3 chain",
            ),
            Self::NonEmptyPredecessorDemandContracts => formatter.write_str(
                "a nonempty demand carrier requires the missing typed-reindexing orbit quotient",
            ),
            Self::NonEmptyBoundEquationPorts => formatter
                .write_str("bound equation-port metadata cannot mint a semantic demand orbit"),
        }
    }
}

impl std::error::Error for DemandOrbitCensusFailureV3 {}

impl DemandOrbitCensusFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        AuditDecision::Unknown(AuditUnknownReason::MissingDemandOrbitCensusV2)
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteEmptyDemandOrbitCoverage;
}

/// Opaque proof that the exact predecessor demand carrier, and therefore its
/// orbit quotient, is empty for one verified V3 chain.
///
/// This is inventory-relative structural authority only. It does not prove
/// that a marginal realizes an output, that SR2 succeeds, or that any live
/// Profile-A debt set is empty.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedDemandOrbitCensusV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    predecessor_history_digest: Digest,
    exact_extension_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    new_event: EventIdV1,
    marginal_family_set_digest: Digest,
    marginal_semantic_set_digest: Digest,
    predecessor_demand_census_digest: Digest,
    empty_orbit_partition_digest: Digest,
    predecessor_demand_contract_count: u64,
    orbit_count: u64,
    complete: private::CompleteEmptyDemandOrbitCoverage,
    digest: Digest,
}

impl VerifiedDemandOrbitCensusV3 {
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

    pub fn marginal_family_set_digest(&self) -> &Digest {
        &self.marginal_family_set_digest
    }

    pub fn marginal_semantic_set_digest(&self) -> &Digest {
        &self.marginal_semantic_set_digest
    }

    pub fn predecessor_demand_census_digest(&self) -> &Digest {
        &self.predecessor_demand_census_digest
    }

    pub fn empty_orbit_partition_digest(&self) -> &Digest {
        &self.empty_orbit_partition_digest
    }

    pub fn predecessor_demand_contract_count(&self) -> usize {
        self.predecessor_demand_contract_count as usize
    }

    pub fn orbit_count(&self) -> usize {
        self.orbit_count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.predecessor_demand_contract_count == 0 && self.orbit_count == 0
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedDemandOrbitCensusV3 {
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
        self.marginal_family_set_digest.encode_canonical(encoder);
        self.marginal_semantic_set_digest.encode_canonical(encoder);
        self.predecessor_demand_census_digest
            .encode_canonical(encoder);
        self.empty_orbit_partition_digest.encode_canonical(encoder);
        encoder.u64(self.predecessor_demand_contract_count);
        encoder.u64(self.orbit_count);
        encoder.tag(1);
    }
}

/// Certify the complete empty orbit quotient for one exact post-marginal V3
/// chain. No caller supplies contracts, orbit members, counts, or a
/// completeness flag.
pub fn diagnose_demand_orbit_census_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    marginals: &VerifiedMarginalFamilySetV3,
) -> Result<VerifiedDemandOrbitCensusV3, DemandOrbitCensusFailureV3> {
    if v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3() {
        return Err(DemandOrbitCensusFailureV3::ExactManifestIdentityMismatch);
    }

    if marginals.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || marginals.inventory_digest() != inventory.digest()
        || marginals.exact_extension_digest() != inventory.exact_extension().digest()
        || marginals.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || marginals.successor_boundary_digest() != inventory.successor_boundary().digest()
        || marginals.new_event() != inventory.exact_extension().event()
    {
        return Err(DemandOrbitCensusFailureV3::ChainBindingMismatch);
    }

    let demand_contract_count = inventory.predecessor_demand_contracts().len();
    let equation_port_count = inventory
        .equations()
        .iter()
        .filter(|equation| equation.demand_port().is_some())
        .count();
    verify_empty_surface_counts(
        inventory.coverage().predecessor_demand_contract_count(),
        demand_contract_count,
        equation_port_count,
    )?;

    let predecessor_demand_census_digest = Digest::of_canonical(
        "pen-semantic-audit/predecessor-demand-contract-census/v3",
        &PredecessorDemandCensusMaterial {
            inventory: inventory.digest(),
            inventory_coverage: inventory.coverage().digest(),
            predecessor_history: inventory.predecessor_history_digest(),
            contracts: inventory.predecessor_demand_contracts(),
        },
    );
    let empty_orbit_partition_digest = Digest::of_canonical(
        "pen-semantic-audit/empty-demand-orbit-partition/v3",
        &EmptyOrbitPartitionMaterial {
            predecessor_demand_census: &predecessor_demand_census_digest,
            demand_contract_count: 0,
            orbit_count: 0,
        },
    );

    let mut verified = VerifiedDemandOrbitCensusV3 {
        schema_version: DEMAND_ORBIT_CENSUS_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        inventory_digest: inventory.digest().clone(),
        inventory_coverage_digest: inventory.coverage().digest().clone(),
        predecessor_history_digest: inventory.predecessor_history_digest().clone(),
        exact_extension_digest: inventory.exact_extension().digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        new_event: inventory.exact_extension().event().clone(),
        marginal_family_set_digest: marginals.digest().clone(),
        marginal_semantic_set_digest: marginals.semantic_set_digest().clone(),
        predecessor_demand_census_digest,
        empty_orbit_partition_digest,
        predecessor_demand_contract_count: 0,
        orbit_count: 0,
        complete: private::CompleteEmptyDemandOrbitCoverage,
        digest: Digest::of_bytes(b"pending verified demand orbit census v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-demand-orbit-census/v3",
        &verified,
    );
    Ok(verified)
}

pub fn verify_demand_orbit_census_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    marginals: &VerifiedMarginalFamilySetV3,
) -> AuditDecision<VerifiedDemandOrbitCensusV3> {
    match diagnose_demand_orbit_census_v3(v3_manifest, inventory, marginals) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn verify_empty_surface_counts(
    covered_demand_contract_count: usize,
    demand_contract_count: usize,
    equation_port_count: usize,
) -> Result<(), DemandOrbitCensusFailureV3> {
    if covered_demand_contract_count != demand_contract_count {
        return Err(DemandOrbitCensusFailureV3::InventoryCoverageMismatch);
    }
    if demand_contract_count != 0 {
        return Err(DemandOrbitCensusFailureV3::NonEmptyPredecessorDemandContracts);
    }
    if equation_port_count != 0 {
        return Err(DemandOrbitCensusFailureV3::NonEmptyBoundEquationPorts);
    }
    Ok(())
}

struct PredecessorDemandCensusMaterial<'a> {
    inventory: &'a Digest,
    inventory_coverage: &'a Digest,
    predecessor_history: &'a Digest,
    contracts: &'a [VerifiedDemandContractV1],
}

impl CanonicalEncode for PredecessorDemandCensusMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.inventory_coverage.encode_canonical(encoder);
        self.predecessor_history.encode_canonical(encoder);
        encoder.sequence(self.contracts);
    }
}

struct EmptyOrbitPartitionMaterial<'a> {
    predecessor_demand_census: &'a Digest,
    demand_contract_count: u64,
    orbit_count: u64,
}

impl CanonicalEncode for EmptyOrbitPartitionMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.predecessor_demand_census.encode_canonical(encoder);
        encoder.u64(self.demand_contract_count);
        encoder.u64(self.orbit_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_complete_empty_surface_is_accepted() {
        assert_eq!(verify_empty_surface_counts(0, 0, 0), Ok(()));
        assert_eq!(
            verify_empty_surface_counts(0, 1, 0),
            Err(DemandOrbitCensusFailureV3::InventoryCoverageMismatch)
        );
        assert_eq!(
            verify_empty_surface_counts(1, 1, 0),
            Err(DemandOrbitCensusFailureV3::NonEmptyPredecessorDemandContracts)
        );
        assert_eq!(
            verify_empty_surface_counts(0, 0, 1),
            Err(DemandOrbitCensusFailureV3::NonEmptyBoundEquationPorts)
        );
    }

    #[test]
    fn every_failure_maps_to_the_registered_stage_reason() {
        for failure in [
            DemandOrbitCensusFailureV3::ExactManifestIdentityMismatch,
            DemandOrbitCensusFailureV3::InventoryCoverageMismatch,
            DemandOrbitCensusFailureV3::ChainBindingMismatch,
            DemandOrbitCensusFailureV3::NonEmptyPredecessorDemandContracts,
            DemandOrbitCensusFailureV3::NonEmptyBoundEquationPorts,
        ] {
            assert!(matches!(
                failure.into_decision::<()>(),
                AuditDecision::Unknown(AuditUnknownReason::MissingDemandOrbitCensusV2)
            ));
        }
    }
}
