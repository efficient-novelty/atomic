//! Restricted, verifier-derived V3 kernel-cost basis.
//!
//! This module does not identify kernel cost with the semantic-family
//! quotient.  It proves a deliberately narrower statement: on an exact V3
//! chain whose successor-new declarations are bodyless and whose separately
//! sealed equations are reflexive source/normal pairs with no demand-port or
//! historical replay surface, the generic cost-V2 checker returns the full
//! set of canonical public-clause identities as singleton first-irreducible
//! basis classes.
//!
//! The returned capability is generic and unfrozen.  It carries no SR2,
//! marginal-family, demand, `nu`, selective-law, or live Profile-A authority.

use crate::cost::{
    ClauseCostDispositionV2, CompleteNegativeEvidenceV2, CostAuditCertificateV2,
    EquationDemandPortBindingV2, KernelCostAuditInputV2, PublicDependencyEdgeV1,
    RawPublicClauseKindV1, RawPublicClauseV1, SourceNormalizedDeclarationV1,
    SourceNormalizedEquationV1, SourceToNormalDerivationV1,
    audit_kernel_cost_lambda_unit_components_v2,
};
use crate::inventory::{
    PublicSubjectV1, VerifiedPublicAuditInventoryV1, VerifiedPublicDeclarationV1,
    VerifiedPublicEquationV1,
};
use crate::inventory_compatibility::VerifiedPublicInventoryCompatibilityV2;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedCostManifestV2,
    VerifiedSemanticAuditManifestV1, VerifiedSemanticAuditManifestV2,
    VerifiedSemanticAuditManifestV3, proposed_kernel_cost_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v1,
    proposed_semantic_audit_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::model::{ClauseIdV1, GenericJudgmentV1};
use crate::native_carrier_v3::VerifiedNativeRankInductiveCarrierV3;
use crate::rewrite_authority_v3::VerifiedRewriteAuthorityV3;
use crate::semantic_authority::{
    PublicClauseSubjectV1, VerifiedPublicClauseCensusV1, VerifiedPublicClauseIdentityV1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Term};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const RESTRICTED_KERNEL_COST_BASIS_SCHEMA_VERSION_V3: u16 = 1;

/// The exact positive fragment in which the restricted theorem is proved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestrictedKernelCostViolationV3 {
    ForcedProjection,
    PredecessorDemandContract,
    PredecessorEquation,
    DemandPort,
    NonReflexiveDeclaration,
    BodyfulNewDeclaration,
    NonReflexiveEquation,
    NonEquationJudgment,
    OwnerNotSuccessorNewBodyless,
    DuplicateFreshRuleKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RestrictedKernelCostBasisFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    RestrictedFragment(RestrictedKernelCostViolationV3),
    PublicClauseCensusMismatch,
    CanonicalClauseCollision,
    V2AuditOutside(OutsideFragmentReason),
    V2AuditUnknown(AuditUnknownReason),
    CertificateMismatch,
    ResourceExhausted,
}

impl std::fmt::Display for RestrictedKernelCostBasisFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied cost and semantic manifests are not the exact registered lambda/unit proposals",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the compatibility, inventory, public-clause census, carrier, and rewrite authority are not one exact chain",
            ),
            Self::RestrictedFragment(violation) => {
                write!(formatter, "outside the restricted V3 kernel-cost theorem: {violation:?}")
            }
            Self::PublicClauseCensusMismatch => formatter.write_str(
                "the public-clause census does not canonically cover the exact successor-new cost surface",
            ),
            Self::CanonicalClauseCollision => formatter.write_str(
                "two distinct successor-new cost clauses acquired the same canonical public-clause identity",
            ),
            Self::V2AuditOutside(reason) => {
                write!(formatter, "the internally derived cost-V2 audit is outside its fragment: {reason:?}")
            }
            Self::V2AuditUnknown(reason) => {
                write!(formatter, "the internally derived cost-V2 audit failed closed: {reason:?}")
            }
            Self::CertificateMismatch => formatter.write_str(
                "the returned cost-V2 certificate is not the exact full singleton first-irreducible basis",
            ),
            Self::ResourceExhausted => {
                formatter.write_str("the restricted V3 kernel-cost census exceeds registered bounds")
            }
        }
    }
}

impl std::error::Error for RestrictedKernelCostBasisFailureV3 {}

impl RestrictedKernelCostBasisFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::ExactManifestIdentityMismatch | Self::ChainBindingMismatch => {
                AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
            }
            Self::RestrictedFragment(
                RestrictedKernelCostViolationV3::NonReflexiveDeclaration
                | RestrictedKernelCostViolationV3::NonReflexiveEquation,
            ) => AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure),
            Self::RestrictedFragment(RestrictedKernelCostViolationV3::DuplicateFreshRuleKey) => {
                AuditDecision::Unknown(AuditUnknownReason::NonUniqueBasis)
            }
            Self::RestrictedFragment(RestrictedKernelCostViolationV3::ForcedProjection) => {
                AuditDecision::OutsideFragment(OutsideFragmentReason::DescriptorProjection)
            }
            Self::RestrictedFragment(_) => {
                AuditDecision::OutsideFragment(OutsideFragmentReason::UnsupportedTerm)
            }
            Self::PublicClauseCensusMismatch | Self::CanonicalClauseCollision => {
                AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
            }
            Self::V2AuditOutside(reason) => AuditDecision::OutsideFragment(reason),
            Self::V2AuditUnknown(reason) => AuditDecision::Unknown(reason),
            Self::CertificateMismatch => {
                AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify)
            }
            Self::ResourceExhausted => {
                AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted)
            }
        }
    }
}

/// One singleton Q2 class in the restricted first-irreducible basis.
///
/// This remains class-valued even though the theorem proves that its member
/// set has cardinality one.  There is no caller-controlled representative
/// field and no deserialization path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedRestrictedFirstIrreducibleBasisClassV3 {
    id: Digest,
    members: Arc<[ClauseIdV1]>,
    disposition_certificate_digest: Digest,
    independence_certificate_digest: Digest,
}

impl VerifiedRestrictedFirstIrreducibleBasisClassV3 {
    pub fn id(&self) -> &Digest {
        &self.id
    }

    pub fn members(&self) -> &[ClauseIdV1] {
        &self.members
    }

    pub fn disposition_certificate_digest(&self) -> &Digest {
        &self.disposition_certificate_digest
    }

    pub fn independence_certificate_digest(&self) -> &Digest {
        &self.independence_certificate_digest
    }
}

impl CanonicalEncode for VerifiedRestrictedFirstIrreducibleBasisClassV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        encoder.sequence(&self.members);
        self.disposition_certificate_digest
            .encode_canonical(encoder);
        self.independence_certificate_digest
            .encode_canonical(encoder);
    }
}

/// Opaque envelope for the restricted V3 singleton-basis theorem.
///
/// Its canonical encoding deliberately excludes the final `digest` field.
/// Every identity used to derive the cost clauses and the complete cost-V2
/// certificate is bound explicitly below.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedRestrictedKernelCostBasisV3 {
    schema_version: u16,
    cost_manifest_digest: Digest,
    semantic_manifest_v1_digest: Digest,
    semantic_manifest_v2_digest: Digest,
    semantic_manifest_v3_digest: Digest,
    inventory_compatibility_digest: Digest,
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    exact_extension_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    public_clause_census_digest: Digest,
    carrier_digest: Digest,
    rewrite_authority_digest: Digest,
    rewrite_predecessor_reconstruction_digest: Digest,
    clause_set_digest: Digest,
    dependency_dag_digest: Digest,
    cost_input_digest: Digest,
    v2_certificate_manifest_digest: Digest,
    v2_certificate_inventory_digest: Digest,
    v2_certificate_exact_api_digest: Digest,
    v2_certificate_digest: Digest,
    kernel_cost: u16,
    clause_count: u16,
    basis_classes: Arc<[VerifiedRestrictedFirstIrreducibleBasisClassV3]>,
    digest: Digest,
}

impl VerifiedRestrictedKernelCostBasisV3 {
    pub fn cost_manifest_digest(&self) -> &Digest {
        &self.cost_manifest_digest
    }

    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_v3_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn public_clause_census_digest(&self) -> &Digest {
        &self.public_clause_census_digest
    }

    pub fn carrier_digest(&self) -> &Digest {
        &self.carrier_digest
    }

    pub fn rewrite_authority_digest(&self) -> &Digest {
        &self.rewrite_authority_digest
    }

    pub fn v2_certificate_digest(&self) -> &Digest {
        &self.v2_certificate_digest
    }

    pub fn kernel_cost(&self) -> u16 {
        self.kernel_cost
    }

    pub fn clause_count(&self) -> usize {
        usize::from(self.clause_count)
    }

    pub fn basis_classes(&self) -> &[VerifiedRestrictedFirstIrreducibleBasisClassV3] {
        &self.basis_classes
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedRestrictedKernelCostBasisV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.cost_manifest_digest.encode_canonical(encoder);
        self.semantic_manifest_v1_digest.encode_canonical(encoder);
        self.semantic_manifest_v2_digest.encode_canonical(encoder);
        self.semantic_manifest_v3_digest.encode_canonical(encoder);
        self.inventory_compatibility_digest
            .encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.inventory_coverage_digest.encode_canonical(encoder);
        self.exact_extension_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.public_clause_census_digest.encode_canonical(encoder);
        self.carrier_digest.encode_canonical(encoder);
        self.rewrite_authority_digest.encode_canonical(encoder);
        self.rewrite_predecessor_reconstruction_digest
            .encode_canonical(encoder);
        self.clause_set_digest.encode_canonical(encoder);
        self.dependency_dag_digest.encode_canonical(encoder);
        self.cost_input_digest.encode_canonical(encoder);
        self.v2_certificate_manifest_digest
            .encode_canonical(encoder);
        self.v2_certificate_inventory_digest
            .encode_canonical(encoder);
        self.v2_certificate_exact_api_digest
            .encode_canonical(encoder);
        self.v2_certificate_digest.encode_canonical(encoder);
        encoder.u16(self.kernel_cost);
        encoder.u16(self.clause_count);
        encoder.sequence(&self.basis_classes);
        // The final digest is intentionally not encoded into itself.
    }
}

/// Derive and verify the restricted V3 kernel-cost singleton basis.
///
/// No raw cost clause, dependency edge, port, reconstruction, quotient,
/// negative-evidence witness, basis representative, or count is accepted
/// from the caller.
#[allow(clippy::too_many_arguments)]
pub fn diagnose_restricted_kernel_cost_basis_v3(
    cost_manifest: &VerifiedCostManifestV2,
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
    rewrite: &VerifiedRewriteAuthorityV3,
) -> Result<VerifiedRestrictedKernelCostBasisV3, RestrictedKernelCostBasisFailureV3> {
    verify_exact_manifests(cost_manifest, v1_manifest, v2_manifest, v3_manifest)?;
    verify_exact_chain(
        v1_manifest,
        v2_manifest,
        v3_manifest,
        compatibility,
        inventory,
        public_clauses,
        carrier,
        rewrite,
    )?;
    verify_restricted_inventory(inventory)?;

    let (input, clause_ids) = derive_cost_input(cost_manifest, inventory, public_clauses)?;
    let clause_count = u16::try_from(clause_ids.len())
        .map_err(|_| RestrictedKernelCostBasisFailureV3::ResourceExhausted)?;
    if clause_count > cost_manifest.manifest().maximum_clauses {
        return Err(RestrictedKernelCostBasisFailureV3::ResourceExhausted);
    }

    let certificate = match audit_kernel_cost_lambda_unit_components_v2(
        cost_manifest,
        v1_manifest,
        inventory,
        &[],
        &input,
    ) {
        AuditDecision::Proven(certificate) => certificate,
        AuditDecision::OutsideFragment(reason) => {
            return Err(RestrictedKernelCostBasisFailureV3::V2AuditOutside(reason));
        }
        AuditDecision::Unknown(reason) => {
            return Err(RestrictedKernelCostBasisFailureV3::V2AuditUnknown(reason));
        }
    };
    let basis_classes =
        validate_exact_singleton_certificate(cost_manifest, inventory, &certificate, &clause_ids)?;

    let clause_set_digest = Digest::of_canonical(
        "pen-semantic-audit/restricted-kernel-cost-clause-set/v3",
        &CanonicalSet(&clause_ids),
    );
    let dependency_dag_digest = Digest::of_canonical(
        "pen-semantic-audit/restricted-kernel-cost-dependency-dag/v3",
        &CanonicalSequence(&input.public_dependency_dag),
    );
    let cost_input_digest = Digest::of_canonical(
        "pen-semantic-audit/restricted-kernel-cost-input/v3",
        &CanonicalCostInput(&input),
    );
    let certificate_digest = certificate.certificate_digest();

    let mut verified = VerifiedRestrictedKernelCostBasisV3 {
        schema_version: RESTRICTED_KERNEL_COST_BASIS_SCHEMA_VERSION_V3,
        cost_manifest_digest: cost_manifest.candidate_digest().clone(),
        semantic_manifest_v1_digest: v1_manifest.candidate_digest().clone(),
        semantic_manifest_v2_digest: v2_manifest.candidate_digest().clone(),
        semantic_manifest_v3_digest: v3_manifest.candidate_digest().clone(),
        inventory_compatibility_digest: compatibility.digest().clone(),
        inventory_digest: inventory.digest().clone(),
        inventory_coverage_digest: inventory.coverage().digest().clone(),
        exact_extension_digest: inventory.exact_extension().digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        public_clause_census_digest: public_clauses.digest().clone(),
        carrier_digest: carrier.digest().clone(),
        rewrite_authority_digest: rewrite.digest().clone(),
        rewrite_predecessor_reconstruction_digest: rewrite
            .predecessor_reconstruction_digest()
            .clone(),
        clause_set_digest,
        dependency_dag_digest,
        cost_input_digest,
        v2_certificate_manifest_digest: certificate.manifest_digest().clone(),
        v2_certificate_inventory_digest: certificate.inventory_digest().clone(),
        v2_certificate_exact_api_digest: certificate.exact_api_digest().clone(),
        v2_certificate_digest: certificate_digest,
        kernel_cost: certificate.kernel_cost(),
        clause_count,
        basis_classes: Arc::from(basis_classes.into_boxed_slice()),
        digest: Digest::of_bytes(b"pending restricted kernel cost basis v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-restricted-kernel-cost-basis/v3",
        &verified,
    );
    Ok(verified)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_restricted_kernel_cost_basis_v3(
    cost_manifest: &VerifiedCostManifestV2,
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
    rewrite: &VerifiedRewriteAuthorityV3,
) -> AuditDecision<VerifiedRestrictedKernelCostBasisV3> {
    match diagnose_restricted_kernel_cost_basis_v3(
        cost_manifest,
        v1_manifest,
        v2_manifest,
        v3_manifest,
        compatibility,
        inventory,
        public_clauses,
        carrier,
        rewrite,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn verify_exact_manifests(
    cost_manifest: &VerifiedCostManifestV2,
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<(), RestrictedKernelCostBasisFailureV3> {
    if cost_manifest.manifest() != &proposed_kernel_cost_lambda_unit_manifest_v2()
        || v1_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v1()
        || v2_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v2()
        || v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3()
    {
        return Err(RestrictedKernelCostBasisFailureV3::ExactManifestIdentityMismatch);
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn verify_exact_chain(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
    rewrite: &VerifiedRewriteAuthorityV3,
) -> Result<(), RestrictedKernelCostBasisFailureV3> {
    if inventory.manifest_digest() != v1_manifest.candidate_digest()
        || compatibility.v1_manifest_digest() != v1_manifest.candidate_digest()
        || compatibility.v2_manifest_digest() != v2_manifest.candidate_digest()
        || compatibility.inventory_digest() != inventory.digest()
        || compatibility.coverage_digest() != inventory.coverage().digest()
        || compatibility.predecessor_history_digest() != inventory.predecessor_history_digest()
        || compatibility.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || compatibility.successor_boundary_digest() != inventory.successor_boundary().digest()
        || compatibility.exact_extension_digest() != inventory.exact_extension().digest()
        || compatibility.normalizer_protocol_digest() != inventory.normalizer_protocol_digest()
        || compatibility.q3_registry_digest() != inventory.q3_registry().digest()
        || public_clauses.inventory_digest() != inventory.digest()
        || public_clauses.inventory_coverage_digest() != inventory.coverage().digest()
        || carrier.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || carrier.inventory_compatibility_digest() != compatibility.digest()
        || carrier.inventory_digest() != inventory.digest()
        || carrier.public_clause_census_digest() != public_clauses.digest()
        || carrier.signature_digest() != inventory.successor_boundary().digest()
        || rewrite.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || rewrite.carrier_digest() != carrier.digest()
        || rewrite.signature_digest() != carrier.signature_digest()
        || rewrite.kernel_protocol_digest() != carrier.kernel_protocol_digest()
    {
        return Err(RestrictedKernelCostBasisFailureV3::ChainBindingMismatch);
    }
    Ok(())
}

fn verify_restricted_inventory(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> Result<(), RestrictedKernelCostBasisFailureV3> {
    if !inventory.forced_projections().is_empty()
        || inventory.coverage().forced_projection_count() != 0
        || !inventory.forced_projection_origins().is_empty()
    {
        return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
            RestrictedKernelCostViolationV3::ForcedProjection,
        ));
    }
    if !inventory.predecessor_demand_contracts().is_empty()
        || inventory.coverage().predecessor_demand_contract_count() != 0
    {
        return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
            RestrictedKernelCostViolationV3::PredecessorDemandContract,
        ));
    }
    if inventory
        .equations()
        .iter()
        .any(VerifiedPublicEquationV1::is_predecessor_public)
    {
        return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
            RestrictedKernelCostViolationV3::PredecessorEquation,
        ));
    }
    if inventory
        .equations()
        .iter()
        .any(|equation| equation.demand_port().is_some())
    {
        return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
            RestrictedKernelCostViolationV3::DemandPort,
        ));
    }

    let new_declarations = inventory
        .exact_extension()
        .new_declarations()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    for head in &new_declarations {
        let declaration = inventory
            .public_declaration(head)
            .ok_or(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch)?;
        if declaration.source() != declaration.normalized() {
            return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::NonReflexiveDeclaration,
            ));
        }
        if declaration.source().body.is_some() || declaration.normalized().body.is_some() {
            return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::BodyfulNewDeclaration,
            ));
        }
    }

    let mut fresh_rule_keys = BTreeSet::new();
    for equation in inventory.equations() {
        if equation.source() != equation.normalized() {
            return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::NonReflexiveEquation,
            ));
        }
        let owner = inventory.public_declaration(equation.owner_head()).ok_or(
            RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::OwnerNotSuccessorNewBodyless,
            ),
        )?;
        if !new_declarations.contains(equation.owner_head())
            || owner.source().body.is_some()
            || owner.normalized().body.is_some()
        {
            return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::OwnerNotSuccessorNewBodyless,
            ));
        }
        register_fresh_rule_key(
            &mut fresh_rule_keys,
            equation.source(),
            equation.owner_head(),
        )?;
    }
    Ok(())
}

fn fresh_rule_key(
    judgment: &GenericJudgmentV1,
    expected_owner: &GlobalId,
) -> Option<(GlobalId, GlobalId, u64)> {
    let GenericJudgmentV1::Equation { left, .. } = judgment else {
        return None;
    };
    let mut head = left;
    let mut reversed_arguments = Vec::new();
    while let Term::Apply { function, argument } = head {
        reversed_arguments.push(argument.as_ref());
        head = function.as_ref();
    }
    let Term::Global { id: owner } = head else {
        return None;
    };
    if owner != expected_owner || reversed_arguments.is_empty() {
        return None;
    }
    let Term::Global { id: constructor } = reversed_arguments[0] else {
        return None;
    };
    Some((
        owner.clone(),
        constructor.clone(),
        u64::try_from(reversed_arguments.len()).ok()?,
    ))
}

fn register_fresh_rule_key(
    keys: &mut BTreeSet<(GlobalId, GlobalId, u64)>,
    judgment: &GenericJudgmentV1,
    expected_owner: &GlobalId,
) -> Result<(), RestrictedKernelCostBasisFailureV3> {
    let key = fresh_rule_key(judgment, expected_owner).ok_or(
        RestrictedKernelCostBasisFailureV3::RestrictedFragment(
            RestrictedKernelCostViolationV3::NonEquationJudgment,
        ),
    )?;
    if !keys.insert(key) {
        return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
            RestrictedKernelCostViolationV3::DuplicateFreshRuleKey,
        ));
    }
    Ok(())
}

fn derive_cost_input(
    cost_manifest: &VerifiedCostManifestV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
) -> Result<(KernelCostAuditInputV2, BTreeSet<ClauseIdV1>), RestrictedKernelCostBasisFailureV3> {
    let mut clauses = BTreeMap::<ClauseIdV1, RawPublicClauseV1>::new();
    let mut declaration_clauses = BTreeMap::<GlobalId, ClauseIdV1>::new();
    let mut equation_clauses = BTreeMap::new();

    for head in inventory.exact_extension().new_declarations() {
        let declaration = inventory
            .public_declaration(head)
            .ok_or(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch)?;
        let identity = census_declaration(public_clauses, declaration)?;
        let id = identity.id().clone();
        let raw = RawPublicClauseV1 {
            id: id.clone(),
            semantic_dependencies: BTreeSet::new(),
            clause: RawPublicClauseKindV1::PublicDeclaration {
                head: head.clone(),
                pair: SourceNormalizedDeclarationV1 {
                    source_identity: declaration.source_identity().clone(),
                    source_context: DependentContext::default(),
                    source_type: declaration.source().ty.clone(),
                    source_body: None,
                    normalized_context: DependentContext::default(),
                    normalized_type: declaration.normalized().ty.clone(),
                    normalized_body: None,
                    source_to_normal_derivation: SourceToNormalDerivationV1::Reflexivity,
                },
                presentation: crate::model::HeadPresentationV1::Opaque,
                public_group: declaration.group().clone(),
                equation_free_descriptors: Vec::new(),
            },
        };
        if declaration_clauses
            .insert(head.clone(), id.clone())
            .is_some()
            || clauses.insert(id, raw).is_some()
        {
            return Err(RestrictedKernelCostBasisFailureV3::CanonicalClauseCollision);
        }
    }

    let mut equation_demand_ports = Vec::with_capacity(inventory.equations().len());
    for equation in inventory.equations() {
        let identity = census_equation(public_clauses, equation)?;
        let id = identity.id().clone();
        let GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } = equation.source()
        else {
            return Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::NonEquationJudgment,
            ));
        };
        let raw = RawPublicClauseV1 {
            id: id.clone(),
            semantic_dependencies: BTreeSet::new(),
            clause: RawPublicClauseKindV1::PublicEquation {
                equation: equation.equation().clone(),
                owner_head: equation.owner_head().clone(),
                demand_output: None,
                pair: SourceNormalizedEquationV1 {
                    source_identity: equation.source_identity().clone(),
                    source_context: context.clone(),
                    source_left: left.clone(),
                    source_right: right.clone(),
                    source_type: ty.clone(),
                    normalized_context: context.clone(),
                    normalized_left: left.clone(),
                    normalized_right: right.clone(),
                    normalized_type: ty.clone(),
                    source_to_normal_derivation: SourceToNormalDerivationV1::Reflexivity,
                },
            },
        };
        if equation_clauses
            .insert(equation.equation().clone(), id.clone())
            .is_some()
            || clauses.insert(id, raw).is_some()
        {
            return Err(RestrictedKernelCostBasisFailureV3::CanonicalClauseCollision);
        }
        equation_demand_ports.push(EquationDemandPortBindingV2 {
            equation: equation.equation().clone(),
            demand_port: None,
        });
    }

    let mut dependency_edges = BTreeSet::new();
    for dependency in inventory.dependency_dag().edges() {
        let dependent = match &dependency.dependent {
            PublicSubjectV1::Declaration { declaration } => declaration_clauses.get(declaration),
            PublicSubjectV1::Equation { equation } => equation_clauses.get(equation),
            PublicSubjectV1::DemandContract { .. } => None,
        };
        let prerequisite = declaration_clauses.get(&dependency.prerequisite);
        if let (Some(dependent), Some(prerequisite)) = (dependent, prerequisite) {
            let edge = PublicDependencyEdgeV1 {
                dependent: dependent.clone(),
                prerequisite: prerequisite.clone(),
            };
            if !dependency_edges.insert(edge.clone()) {
                return Err(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch);
            }
            clauses
                .get_mut(dependent)
                .ok_or(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch)?
                .semantic_dependencies
                .insert(prerequisite.clone());
        }
    }

    let clause_ids = clauses.keys().cloned().collect::<BTreeSet<_>>();
    let negative_evidence = clause_ids
        .iter()
        .cloned()
        .map(|target| {
            let mut against = clause_ids.clone();
            against.remove(&target);
            CompleteNegativeEvidenceV2 {
                target,
                realized: against.clone(),
                against,
                checked_rules: cost_manifest.manifest().free_completion_rules.clone(),
            }
        })
        .collect();
    equation_demand_ports.sort_by(|left, right| left.equation.cmp(&right.equation));

    Ok((
        KernelCostAuditInputV2 {
            clauses: clauses.into_values().collect(),
            public_dependency_dag: dependency_edges.into_iter().collect(),
            equation_demand_ports,
            reconstructions: Vec::new(),
            negative_evidence,
            presentation_equivalences: Vec::new(),
        },
        clause_ids,
    ))
}

fn census_declaration<'a>(
    public_clauses: &'a VerifiedPublicClauseCensusV1,
    declaration: &VerifiedPublicDeclarationV1,
) -> Result<&'a VerifiedPublicClauseIdentityV1, RestrictedKernelCostBasisFailureV3> {
    let id = public_clauses
        .clause_for_declaration(declaration.declaration())
        .ok_or(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch)?;
    let identity = public_clauses
        .clauses()
        .iter()
        .find(|candidate| candidate.id() == id)
        .ok_or(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch)?;
    if identity.subject()
        != &(PublicClauseSubjectV1::Declaration {
            declaration: declaration.declaration().clone(),
        })
        || identity.source_identity() != declaration.source_identity()
    {
        return Err(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch);
    }
    Ok(identity)
}

fn census_equation<'a>(
    public_clauses: &'a VerifiedPublicClauseCensusV1,
    equation: &VerifiedPublicEquationV1,
) -> Result<&'a VerifiedPublicClauseIdentityV1, RestrictedKernelCostBasisFailureV3> {
    let id = public_clauses
        .clause_for_equation(equation.equation())
        .ok_or(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch)?;
    let identity = public_clauses
        .clauses()
        .iter()
        .find(|candidate| candidate.id() == id)
        .ok_or(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch)?;
    if identity.subject()
        != &(PublicClauseSubjectV1::Equation {
            equation: equation.equation().clone(),
            owner_head: equation.owner_head().clone(),
        })
        || identity.source_identity() != equation.source_identity()
    {
        return Err(RestrictedKernelCostBasisFailureV3::PublicClauseCensusMismatch);
    }
    Ok(identity)
}

fn validate_exact_singleton_certificate(
    cost_manifest: &VerifiedCostManifestV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    certificate: &CostAuditCertificateV2,
    all: &BTreeSet<ClauseIdV1>,
) -> Result<Vec<VerifiedRestrictedFirstIrreducibleBasisClassV3>, RestrictedKernelCostBasisFailureV3>
{
    let expected_cost = u16::try_from(all.len())
        .map_err(|_| RestrictedKernelCostBasisFailureV3::ResourceExhausted)?;
    let expected_subsets = 1_u32
        .checked_shl(u32::from(expected_cost))
        .ok_or(RestrictedKernelCostBasisFailureV3::ResourceExhausted)?;
    if certificate.manifest_digest() != cost_manifest.candidate_digest()
        || certificate.inventory_digest() != inventory.digest()
        || certificate.kernel_cost() != expected_cost
        || certificate.tested_basis_subsets() != expected_subsets
        || certificate.basis_representatives() != [all.clone()]
        || certificate.basis_classes().len() != all.len()
        || certificate.dispositions().len() != all.len()
        || certificate.independence().len() != all.len()
        || certificate.dependency_components().len() != all.len()
    {
        return Err(RestrictedKernelCostBasisFailureV3::CertificateMismatch);
    }

    let expected_classes = all
        .iter()
        .cloned()
        .map(|member| BTreeSet::from([member]))
        .collect::<BTreeSet<_>>();
    let actual_classes = certificate
        .basis_classes()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if actual_classes != expected_classes
        || certificate
            .dependency_components()
            .iter()
            .any(|component| component.members.len() != 1)
        || certificate
            .dependency_components()
            .iter()
            .flat_map(|component| component.members.iter().cloned())
            .collect::<BTreeSet<_>>()
            != *all
    {
        return Err(RestrictedKernelCostBasisFailureV3::CertificateMismatch);
    }

    let mut dispositions = BTreeMap::new();
    for disposition in certificate.dispositions() {
        if disposition.closure_round() != 0
            || !disposition.reconstructions().is_empty()
            || disposition.disposition() != &ClauseCostDispositionV2::FirstIrreducible
            || dispositions
                .insert(disposition.clause().clone(), disposition)
                .is_some()
        {
            return Err(RestrictedKernelCostBasisFailureV3::CertificateMismatch);
        }
    }
    if dispositions.keys().cloned().collect::<BTreeSet<_>>() != *all {
        return Err(RestrictedKernelCostBasisFailureV3::CertificateMismatch);
    }

    let mut independence = BTreeMap::new();
    for witness in certificate.independence() {
        let removed = witness.removed();
        let evidence = witness.negative_evidence();
        let mut expected_against = all.clone();
        expected_against.remove(removed);
        if witness.basis_representative() != all
            || evidence.target != *removed
            || evidence.against != expected_against
            || evidence.realized != expected_against
            || evidence.checked_rules != cost_manifest.manifest().free_completion_rules
            || independence.insert(removed.clone(), witness).is_some()
        {
            return Err(RestrictedKernelCostBasisFailureV3::CertificateMismatch);
        }
    }
    if independence.keys().cloned().collect::<BTreeSet<_>>() != *all {
        return Err(RestrictedKernelCostBasisFailureV3::CertificateMismatch);
    }

    let certificate_digest = certificate.certificate_digest();
    all.iter()
        .map(|member| {
            let disposition = dispositions
                .get(member)
                .ok_or(RestrictedKernelCostBasisFailureV3::CertificateMismatch)?;
            let witness = independence
                .get(member)
                .ok_or(RestrictedKernelCostBasisFailureV3::CertificateMismatch)?;
            let member_set = BTreeSet::from([member.clone()]);
            Ok(VerifiedRestrictedFirstIrreducibleBasisClassV3 {
                id: Digest::of_canonical(
                    "pen-semantic-audit/first-irreducible-basis-class/v3",
                    &FirstIrreducibleClassIdentity {
                        certificate: &certificate_digest,
                        members: &member_set,
                    },
                ),
                members: Arc::from(vec![member.clone()].into_boxed_slice()),
                disposition_certificate_digest: Digest::of_canonical(
                    "pen-semantic-audit/first-irreducible-disposition/v3",
                    *disposition,
                ),
                independence_certificate_digest: Digest::of_canonical(
                    "pen-semantic-audit/first-irreducible-independence/v3",
                    *witness,
                ),
            })
        })
        .collect()
}

struct CanonicalSet<'a, T>(&'a BTreeSet<T>);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSet<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.0.len() as u64);
        for value in self.0 {
            value.encode_canonical(encoder);
        }
    }
}

struct CanonicalSequence<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSequence<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct CanonicalCostInput<'a>(&'a KernelCostAuditInputV2);

impl CanonicalEncode for CanonicalCostInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.0.clauses);
        encoder.sequence(&self.0.public_dependency_dag);
        encoder.sequence(&self.0.equation_demand_ports);
        encoder.sequence(&self.0.reconstructions);
        encoder.sequence(&self.0.negative_evidence);
        encoder.sequence(&self.0.presentation_equivalences);
    }
}

struct FirstIrreducibleClassIdentity<'a> {
    certificate: &'a Digest,
    members: &'a BTreeSet<ClauseIdV1>,
}

impl CanonicalEncode for FirstIrreducibleClassIdentity<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.certificate.encode_canonical(encoder);
        CanonicalSet(self.members).encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn global(label: &'static [u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn apply(function: Term, argument: Term) -> Term {
        Term::Apply {
            function: Box::new(function),
            argument: Box::new(argument),
        }
    }

    fn fresh_equation(owner: &GlobalId, constructor: &GlobalId) -> GenericJudgmentV1 {
        GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: apply(
                apply(Term::Global { id: owner.clone() }, Term::Unit),
                Term::Global {
                    id: constructor.clone(),
                },
            ),
            right: Term::Unit,
            ty: Term::UnitType,
        }
    }

    #[test]
    fn fresh_rule_key_uses_exact_owner_final_constructor_and_spine_arity() {
        let owner = global(b"restricted-cost-test/owner");
        let constructor = global(b"restricted-cost-test/constructor");
        let other = global(b"restricted-cost-test/other");
        let judgment = fresh_equation(&owner, &constructor);

        assert_eq!(
            fresh_rule_key(&judgment, &owner),
            Some((owner.clone(), constructor.clone(), 2))
        );
        assert_eq!(fresh_rule_key(&judgment, &other), None);

        let no_constructor = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Global { id: owner.clone() },
            right: Term::Unit,
            ty: Term::UnitType,
        };
        assert_eq!(fresh_rule_key(&no_constructor, &owner), None);

        let non_global_final_argument = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: apply(Term::Global { id: owner.clone() }, Term::Unit),
            right: Term::Unit,
            ty: Term::UnitType,
        };
        assert_eq!(fresh_rule_key(&non_global_final_argument, &owner), None);
    }

    #[test]
    fn fresh_rule_key_registration_rejects_duplicates_and_non_equations() {
        let owner = global(b"restricted-cost-test/duplicate-owner");
        let constructor = global(b"restricted-cost-test/duplicate-constructor");
        let judgment = fresh_equation(&owner, &constructor);
        let mut keys = BTreeSet::new();

        assert_eq!(
            register_fresh_rule_key(&mut keys, &judgment, &owner),
            Ok(())
        );
        assert_eq!(
            register_fresh_rule_key(&mut keys, &judgment, &owner),
            Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::DuplicateFreshRuleKey,
            ))
        );

        let term_judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        };
        assert_eq!(
            register_fresh_rule_key(&mut keys, &term_judgment, &owner),
            Err(RestrictedKernelCostBasisFailureV3::RestrictedFragment(
                RestrictedKernelCostViolationV3::NonEquationJudgment,
            ))
        );
    }
}
