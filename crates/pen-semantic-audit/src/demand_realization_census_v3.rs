//! Complete empty-base demand-realization census for the V3 semantic audit.
//!
//! Realization obligations are reconstructed independently from the verified
//! inventory and from the complete V3 equation-port metadata ledger. Only an
//! exact, empty obligation carrier over the exact empty demand-orbit census
//! can mint this capability. Empty orbits alone are never treated as
//! realization authority.
//!
//! This module mints no demand output, family assignment, cost, SR2, `nu`, or
//! Selective-Law capability.

use crate::demand_orbit_census_v3::VerifiedDemandOrbitCensusV3;
use crate::inventory::{DemandPortKeyV1, VerifiedPublicAuditInventoryV1};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::marginal_family_set_v3::VerifiedMarginalFamilySetV3;
use crate::model::{EquationIdV1, EventIdV1};
use crate::semantic_authority_v3::{
    VerifiedBoundEquationPortMetadataV3, VerifiedSemanticSeedBaseCensusV3,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::collections::BTreeMap;

pub const DEMAND_REALIZATION_CENSUS_SCHEMA_VERSION_V3: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DemandRealizationCensusFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    InventoryCoverageMismatch,
    EquationPortLedgerMismatch,
    NonEmptyDemandSurface,
    NonEmptyRealizationObligations,
    NonEmptyOrbitSurface,
}

impl std::fmt::Display for DemandRealizationCensusFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifest is not the exact registered lambda/unit V3 proposal",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the seed, marginal, orbit, and inventory authorities are not one exact V3 chain",
            ),
            Self::InventoryCoverageMismatch => formatter.write_str(
                "the verified predecessor-demand coverage does not equal the inventoried carrier",
            ),
            Self::EquationPortLedgerMismatch => formatter.write_str(
                "the complete V3 equation-port ledger does not equal the inventoried obligation set",
            ),
            Self::NonEmptyDemandSurface => formatter.write_str(
                "a nonempty predecessor-demand carrier requires nonempty orbit authority first",
            ),
            Self::NonEmptyRealizationObligations => formatter.write_str(
                "a nonempty equation-port obligation set requires kernel-replayed typed realizations",
            ),
            Self::NonEmptyOrbitSurface => formatter.write_str(
                "a nonempty demand-orbit surface requires a nonempty typed realization census",
            ),
        }
    }
}

impl std::error::Error for DemandRealizationCensusFailureV3 {}

impl DemandRealizationCensusFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        AuditDecision::Unknown(AuditUnknownReason::MissingDemandRealizationCensusV2)
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteEmptyDemandRealizationCoverage;
}

/// Opaque proof that the exact realization-obligation carrier and its
/// realization relation are empty for one verified V3 chain.
///
/// This inventory-relative fact does not prove SR2, discharge any marginal,
/// or establish a live debt verdict.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedDemandRealizationCensusV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    predecessor_history_digest: Digest,
    exact_extension_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    new_event: EventIdV1,
    semantic_seed_base_census_digest: Digest,
    public_clause_census_digest: Digest,
    marginal_family_set_digest: Digest,
    marginal_semantic_set_digest: Digest,
    demand_orbit_census_digest: Digest,
    predecessor_demand_census_digest: Digest,
    empty_orbit_partition_digest: Digest,
    realization_obligation_census_digest: Digest,
    empty_realization_relation_digest: Digest,
    realization_obligation_count: u64,
    realization_count: u64,
    complete: private::CompleteEmptyDemandRealizationCoverage,
    digest: Digest,
}

impl VerifiedDemandRealizationCensusV3 {
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

    pub fn semantic_seed_base_census_digest(&self) -> &Digest {
        &self.semantic_seed_base_census_digest
    }

    pub fn public_clause_census_digest(&self) -> &Digest {
        &self.public_clause_census_digest
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

    pub fn predecessor_demand_census_digest(&self) -> &Digest {
        &self.predecessor_demand_census_digest
    }

    pub fn empty_orbit_partition_digest(&self) -> &Digest {
        &self.empty_orbit_partition_digest
    }

    pub fn realization_obligation_census_digest(&self) -> &Digest {
        &self.realization_obligation_census_digest
    }

    pub fn empty_realization_relation_digest(&self) -> &Digest {
        &self.empty_realization_relation_digest
    }

    pub fn realization_obligation_count(&self) -> usize {
        self.realization_obligation_count as usize
    }

    pub fn realization_count(&self) -> usize {
        self.realization_count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.realization_obligation_count == 0 && self.realization_count == 0
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedDemandRealizationCensusV3 {
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
        self.semantic_seed_base_census_digest
            .encode_canonical(encoder);
        self.public_clause_census_digest.encode_canonical(encoder);
        self.marginal_family_set_digest.encode_canonical(encoder);
        self.marginal_semantic_set_digest.encode_canonical(encoder);
        self.demand_orbit_census_digest.encode_canonical(encoder);
        self.predecessor_demand_census_digest
            .encode_canonical(encoder);
        self.empty_orbit_partition_digest.encode_canonical(encoder);
        self.realization_obligation_census_digest
            .encode_canonical(encoder);
        self.empty_realization_relation_digest
            .encode_canonical(encoder);
        encoder.u64(self.realization_obligation_count);
        encoder.u64(self.realization_count);
        encoder.tag(1);
    }
}

/// Certify the complete empty realization relation for one exact post-orbit
/// V3 chain. Realization obligations are derived independently from both the
/// verified inventory and the complete bound-port ledger.
pub fn diagnose_demand_realization_census_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    seed_base: &VerifiedSemanticSeedBaseCensusV3,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
) -> Result<VerifiedDemandRealizationCensusV3, DemandRealizationCensusFailureV3> {
    if v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3() {
        return Err(DemandRealizationCensusFailureV3::ExactManifestIdentityMismatch);
    }
    if seed_base.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || seed_base.inventory_digest() != inventory.digest()
        || marginals.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || marginals.inventory_digest() != inventory.digest()
        || marginals.exact_extension_digest() != inventory.exact_extension().digest()
        || marginals.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || marginals.successor_boundary_digest() != inventory.successor_boundary().digest()
        || marginals.new_event() != inventory.exact_extension().event()
        || orbits.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || orbits.inventory_digest() != inventory.digest()
        || orbits.inventory_coverage_digest() != inventory.coverage().digest()
        || orbits.predecessor_history_digest() != inventory.predecessor_history_digest()
        || orbits.exact_extension_digest() != inventory.exact_extension().digest()
        || orbits.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || orbits.successor_boundary_digest() != inventory.successor_boundary().digest()
        || orbits.new_event() != inventory.exact_extension().event()
        || orbits.marginal_family_set_digest() != marginals.digest()
        || orbits.marginal_semantic_set_digest() != marginals.semantic_set_digest()
    {
        return Err(DemandRealizationCensusFailureV3::ChainBindingMismatch);
    }

    let expected_obligations = inventory_port_obligations(inventory)?;
    let observed_obligations = seed_port_obligations(seed_base.equation_port_metadata())?;
    if expected_obligations != observed_obligations {
        return Err(DemandRealizationCensusFailureV3::EquationPortLedgerMismatch);
    }

    verify_empty_realization_surface(
        inventory.coverage().predecessor_demand_contract_count(),
        inventory.predecessor_demand_contracts().len(),
        expected_obligations.len(),
        orbits.predecessor_demand_contract_count(),
        orbits.orbit_count(),
        orbits.is_empty(),
    )?;

    let expected_entries = expected_obligations
        .into_iter()
        .map(|(equation, port)| EquationPortObligationV3 { equation, port })
        .collect::<Vec<_>>();
    let realization_obligation_census_digest = Digest::of_canonical(
        "pen-semantic-audit/demand-realization-obligation-census/v3",
        &RealizationObligationCensusMaterial {
            inventory: inventory.digest(),
            semantic_seed_base: seed_base.digest(),
            expected: &expected_entries,
            observed: seed_base.equation_port_metadata(),
        },
    );
    let empty_realization_relation_digest = Digest::of_canonical(
        "pen-semantic-audit/empty-demand-realization-relation/v3",
        &EmptyRealizationRelationMaterial {
            marginal_family_set: marginals.digest(),
            demand_orbit_census: orbits.digest(),
            realization_obligations: &realization_obligation_census_digest,
            obligation_count: 0,
            realization_count: 0,
        },
    );

    let mut verified = VerifiedDemandRealizationCensusV3 {
        schema_version: DEMAND_REALIZATION_CENSUS_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        inventory_digest: inventory.digest().clone(),
        inventory_coverage_digest: inventory.coverage().digest().clone(),
        predecessor_history_digest: inventory.predecessor_history_digest().clone(),
        exact_extension_digest: inventory.exact_extension().digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        new_event: inventory.exact_extension().event().clone(),
        semantic_seed_base_census_digest: seed_base.digest().clone(),
        public_clause_census_digest: seed_base.public_clause_census_digest().clone(),
        marginal_family_set_digest: marginals.digest().clone(),
        marginal_semantic_set_digest: marginals.semantic_set_digest().clone(),
        demand_orbit_census_digest: orbits.digest().clone(),
        predecessor_demand_census_digest: orbits.predecessor_demand_census_digest().clone(),
        empty_orbit_partition_digest: orbits.empty_orbit_partition_digest().clone(),
        realization_obligation_census_digest,
        empty_realization_relation_digest,
        realization_obligation_count: 0,
        realization_count: 0,
        complete: private::CompleteEmptyDemandRealizationCoverage,
        digest: Digest::of_bytes(b"pending verified demand realization census v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-demand-realization-census/v3",
        &verified,
    );
    Ok(verified)
}

pub fn verify_demand_realization_census_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    seed_base: &VerifiedSemanticSeedBaseCensusV3,
    marginals: &VerifiedMarginalFamilySetV3,
    orbits: &VerifiedDemandOrbitCensusV3,
) -> AuditDecision<VerifiedDemandRealizationCensusV3> {
    match diagnose_demand_realization_census_v3(
        v3_manifest,
        inventory,
        seed_base,
        marginals,
        orbits,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn inventory_port_obligations(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> Result<BTreeMap<EquationIdV1, DemandPortKeyV1>, DemandRealizationCensusFailureV3> {
    let mut obligations = BTreeMap::new();
    for equation in inventory.equations() {
        if let Some(port) = equation.demand_port()
            && obligations
                .insert(equation.equation().clone(), port.clone())
                .is_some()
        {
            return Err(DemandRealizationCensusFailureV3::EquationPortLedgerMismatch);
        }
    }
    Ok(obligations)
}

fn seed_port_obligations(
    metadata: &[VerifiedBoundEquationPortMetadataV3],
) -> Result<BTreeMap<EquationIdV1, DemandPortKeyV1>, DemandRealizationCensusFailureV3> {
    let mut obligations = BTreeMap::new();
    for entry in metadata {
        if obligations
            .insert(entry.equation().clone(), entry.port().clone())
            .is_some()
        {
            return Err(DemandRealizationCensusFailureV3::EquationPortLedgerMismatch);
        }
    }
    Ok(obligations)
}

fn verify_empty_realization_surface(
    covered_demand_contract_count: usize,
    demand_contract_count: usize,
    realization_obligation_count: usize,
    orbit_demand_contract_count: usize,
    orbit_count: usize,
    orbit_reports_empty: bool,
) -> Result<(), DemandRealizationCensusFailureV3> {
    if covered_demand_contract_count != demand_contract_count {
        return Err(DemandRealizationCensusFailureV3::InventoryCoverageMismatch);
    }
    if demand_contract_count != 0 {
        return Err(DemandRealizationCensusFailureV3::NonEmptyDemandSurface);
    }
    if realization_obligation_count != 0 {
        return Err(DemandRealizationCensusFailureV3::NonEmptyRealizationObligations);
    }
    if orbit_demand_contract_count != 0 || orbit_count != 0 || !orbit_reports_empty {
        return Err(DemandRealizationCensusFailureV3::NonEmptyOrbitSurface);
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EquationPortObligationV3 {
    equation: EquationIdV1,
    port: DemandPortKeyV1,
}

impl CanonicalEncode for EquationPortObligationV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        self.port.encode_canonical(encoder);
    }
}

struct RealizationObligationCensusMaterial<'a> {
    inventory: &'a Digest,
    semantic_seed_base: &'a Digest,
    expected: &'a [EquationPortObligationV3],
    observed: &'a [VerifiedBoundEquationPortMetadataV3],
}

impl CanonicalEncode for RealizationObligationCensusMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.semantic_seed_base.encode_canonical(encoder);
        encoder.sequence(self.expected);
        encoder.sequence(self.observed);
    }
}

struct EmptyRealizationRelationMaterial<'a> {
    marginal_family_set: &'a Digest,
    demand_orbit_census: &'a Digest,
    realization_obligations: &'a Digest,
    obligation_count: u64,
    realization_count: u64,
}

impl CanonicalEncode for EmptyRealizationRelationMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.marginal_family_set.encode_canonical(encoder);
        self.demand_orbit_census.encode_canonical(encoder);
        self.realization_obligations.encode_canonical(encoder);
        encoder.u64(self.obligation_count);
        encoder.u64(self.realization_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realization_emptiness_is_independent_of_orbit_emptiness() {
        assert_eq!(
            verify_empty_realization_surface(0, 0, 0, 0, 0, true),
            Ok(())
        );
        assert_eq!(
            verify_empty_realization_surface(0, 0, 1, 0, 0, true),
            Err(DemandRealizationCensusFailureV3::NonEmptyRealizationObligations)
        );
        assert_eq!(
            verify_empty_realization_surface(0, 0, 0, 0, 0, false),
            Err(DemandRealizationCensusFailureV3::NonEmptyOrbitSurface)
        );
    }

    #[test]
    fn nonempty_demands_and_incomplete_coverage_fail_before_minting() {
        assert_eq!(
            verify_empty_realization_surface(0, 1, 0, 0, 0, true),
            Err(DemandRealizationCensusFailureV3::InventoryCoverageMismatch)
        );
        assert_eq!(
            verify_empty_realization_surface(1, 1, 0, 0, 0, true),
            Err(DemandRealizationCensusFailureV3::NonEmptyDemandSurface)
        );
    }

    #[test]
    fn every_failure_maps_to_the_registered_stage_reason() {
        for failure in [
            DemandRealizationCensusFailureV3::ExactManifestIdentityMismatch,
            DemandRealizationCensusFailureV3::ChainBindingMismatch,
            DemandRealizationCensusFailureV3::InventoryCoverageMismatch,
            DemandRealizationCensusFailureV3::EquationPortLedgerMismatch,
            DemandRealizationCensusFailureV3::NonEmptyDemandSurface,
            DemandRealizationCensusFailureV3::NonEmptyRealizationObligations,
            DemandRealizationCensusFailureV3::NonEmptyOrbitSurface,
        ] {
            assert!(matches!(
                failure.into_decision::<()>(),
                AuditDecision::Unknown(AuditUnknownReason::MissingDemandRealizationCensusV2)
            ));
        }
    }
}
