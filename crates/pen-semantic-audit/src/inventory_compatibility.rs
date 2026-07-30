//! Narrow transport of a replayed V1 public inventory into the V2
//! lambda/unit development.
//!
//! This module does not cast or rebind the inventory.  It mints an opaque
//! compatibility capability only for the exact verified lambda/unit profile
//! pair and only after checking every manifest field used by public-inventory
//! normalization or fragment classification.  In particular, none of the V1
//! substitution-carrier census is transported.

use crate::fragment::{
    LambdaUnitSyntaxViolation, lambda_unit_judgment_syntax_violation,
    lambda_unit_term_syntax_violation,
};
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, Q0RuleV1, Q3RuleV1,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2,
    VerifiedSemanticAuditManifestV1, VerifiedSemanticAuditManifestV2,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::sync::Arc;

/// Closed inventory of the facts transported by the compatibility theorem.
///
/// The list deliberately contains no substitution-carrier, construction-
/// substitution, generic-substitution, occurrence, edge-matching, overlap, or
/// conservativity item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreservedPublicInventoryInvariantV2 {
    ExactLambdaUnitProfilePair,
    LambdaUnitTermGrammar,
    ProjectionFreeFragmentBoundary,
    UniverseLevelInventory,
    DependentContextGrammar,
    KernelTypingProtocol,
    DeBruijnRepresentation,
    MaximumContextEntries,
    Q0RuleInventory,
    Q0EtaRegistryEmptiness,
    KernelNormalizerProtocol,
    EmptyOriginCutoffQ3Rule,
    PublicHistoryReplay,
    ExactKernelBoundaries,
}

impl CanonicalEncode for PreservedPublicInventoryInvariantV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::ExactLambdaUnitProfilePair => 0,
            Self::LambdaUnitTermGrammar => 1,
            Self::ProjectionFreeFragmentBoundary => 2,
            Self::UniverseLevelInventory => 3,
            Self::DependentContextGrammar => 4,
            Self::KernelTypingProtocol => 5,
            Self::DeBruijnRepresentation => 6,
            Self::MaximumContextEntries => 7,
            Self::Q0RuleInventory => 8,
            Self::Q0EtaRegistryEmptiness => 9,
            Self::KernelNormalizerProtocol => 10,
            Self::EmptyOriginCutoffQ3Rule => 11,
            Self::PublicHistoryReplay => 12,
            Self::ExactKernelBoundaries => 13,
        });
    }
}

const PRESERVED_INVARIANTS: [PreservedPublicInventoryInvariantV2; 14] = [
    PreservedPublicInventoryInvariantV2::ExactLambdaUnitProfilePair,
    PreservedPublicInventoryInvariantV2::LambdaUnitTermGrammar,
    PreservedPublicInventoryInvariantV2::ProjectionFreeFragmentBoundary,
    PreservedPublicInventoryInvariantV2::UniverseLevelInventory,
    PreservedPublicInventoryInvariantV2::DependentContextGrammar,
    PreservedPublicInventoryInvariantV2::KernelTypingProtocol,
    PreservedPublicInventoryInvariantV2::DeBruijnRepresentation,
    PreservedPublicInventoryInvariantV2::MaximumContextEntries,
    PreservedPublicInventoryInvariantV2::Q0RuleInventory,
    PreservedPublicInventoryInvariantV2::Q0EtaRegistryEmptiness,
    PreservedPublicInventoryInvariantV2::KernelNormalizerProtocol,
    PreservedPublicInventoryInvariantV2::EmptyOriginCutoffQ3Rule,
    PreservedPublicInventoryInvariantV2::PublicHistoryReplay,
    PreservedPublicInventoryInvariantV2::ExactKernelBoundaries,
];

/// Opaque, verifier-minted compatibility proof.
///
/// This capability has private fields and intentionally implements neither
/// `Deserialize` nor `Serialize`.  It transports only the closed invariant set
/// returned by [`Self::preserved_invariants`].
#[derive(Clone, Debug)]
pub struct VerifiedPublicInventoryCompatibilityV2 {
    v1_manifest_digest: Digest,
    v2_manifest_digest: Digest,
    inventory_digest: Digest,
    coverage_digest: Digest,
    predecessor_history_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    exact_extension_digest: Digest,
    normalizer_protocol_digest: Digest,
    q3_registry_digest: Digest,
    preserved_invariants: Arc<[PreservedPublicInventoryInvariantV2]>,
    invariant_digest: Digest,
    digest: Digest,
}

impl VerifiedPublicInventoryCompatibilityV2 {
    pub fn v1_manifest_digest(&self) -> &Digest {
        &self.v1_manifest_digest
    }

    pub fn v2_manifest_digest(&self) -> &Digest {
        &self.v2_manifest_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn predecessor_history_digest(&self) -> &Digest {
        &self.predecessor_history_digest
    }

    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn exact_extension_digest(&self) -> &Digest {
        &self.exact_extension_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn q3_registry_digest(&self) -> &Digest {
        &self.q3_registry_digest
    }

    pub fn preserved_invariants(&self) -> &[PreservedPublicInventoryInvariantV2] {
        &self.preserved_invariants
    }

    pub fn invariant_digest(&self) -> &Digest {
        &self.invariant_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

struct InvariantMaterial<'a> {
    v1_profile_id: &'a str,
    v2_profile_id: &'a str,
    universe_levels: &'a [u16],
    maximum_context_entries: u16,
    q0_rules: &'a [Q0RuleV1],
    q0_eta_registry_empty: bool,
    q3_rule: Q3RuleV1,
    preserved_invariants: &'a [PreservedPublicInventoryInvariantV2],
}

impl CanonicalEncode for InvariantMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(self.v1_profile_id);
        encoder.text(self.v2_profile_id);
        encoder.u64(self.universe_levels.len() as u64);
        for level in self.universe_levels {
            encoder.u16(*level);
        }
        encoder.u16(self.maximum_context_entries);
        encoder.sequence(self.q0_rules);
        encoder.tag(u8::from(self.q0_eta_registry_empty));
        self.q3_rule.encode_canonical(encoder);
        encoder.sequence(self.preserved_invariants);
    }
}

struct CompatibilityMaterial<'a> {
    v1_manifest: &'a Digest,
    v2_manifest: &'a Digest,
    inventory: &'a Digest,
    coverage: &'a Digest,
    predecessor_history: &'a Digest,
    predecessor_boundary: &'a Digest,
    successor_boundary: &'a Digest,
    exact_extension: &'a Digest,
    normalizer_protocol: &'a Digest,
    q3_registry: &'a Digest,
    invariants: &'a Digest,
}

impl CanonicalEncode for CompatibilityMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.v1_manifest.encode_canonical(encoder);
        self.v2_manifest.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        self.coverage.encode_canonical(encoder);
        self.predecessor_history.encode_canonical(encoder);
        self.predecessor_boundary.encode_canonical(encoder);
        self.successor_boundary.encode_canonical(encoder);
        self.exact_extension.encode_canonical(encoder);
        self.normalizer_protocol.encode_canonical(encoder);
        self.q3_registry.encode_canonical(encoder);
        self.invariants.encode_canonical(encoder);
    }
}

/// Verify the narrow V1-inventory/V2-profile compatibility theorem.
///
/// The exact profile identifiers are significant.  In particular,
/// `normalizer.rs` activates recursive lambda/unit surface classification for
/// the V1 identifier, so equality of only the universe and Q0 vectors would
/// not establish fragment identity.
pub fn verify_public_inventory_compatibility_v2(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    inventory: &VerifiedPublicAuditInventoryV1,
) -> AuditDecision<VerifiedPublicInventoryCompatibilityV2> {
    let v1 = v1_manifest.manifest();
    let v2 = v2_manifest.manifest();

    if v1.profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
        || v2.profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2
        || inventory.manifest_digest() != v1_manifest.candidate_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::MissingVerifiedPublicInventory);
    }

    // These are exactly the manifest fields read by public-inventory
    // normalization/fragment checking, plus the Q3 rule governing the
    // inventory's positively verified empty origin-cutoff registry.
    //
    // Deliberate exclusions:
    // - v1.substitution_carrier_rules;
    // - v2.substitution_census_scope;
    // - v2.construction_substitution_rules;
    // - v2.generic_substitution_theorems.
    //
    // The V1 global substitution census is therefore not transported.
    if v1.universe_levels != v2.universe_levels
        || v1.maximum_context_entries != v2.maximum_context_entries
        || v1.q0_rules != v2.q0_rules
        || v1.q0_eta_registry_empty != v2.q0_eta_registry_empty
        || v1.q3_rule != v2.q3_rule
    {
        return AuditDecision::Unknown(AuditUnknownReason::MissingVerifiedPublicInventory);
    }

    if !inventory.forced_projections().is_empty() {
        return AuditDecision::OutsideFragment(OutsideFragmentReason::DescriptorProjection);
    }
    if let Some(violation) = public_inventory_syntax_violation(inventory, &v1.universe_levels) {
        return AuditDecision::OutsideFragment(violation.outside_reason());
    }

    let coverage = inventory.coverage();
    if coverage.group_count() != inventory.declaration_groups().len()
        || coverage.declaration_count() != inventory.declarations().len()
        || coverage.equation_count() != inventory.equations().len()
        || coverage.forced_projection_count() != inventory.forced_projections().len()
        || coverage.predecessor_demand_contract_count()
            != inventory.predecessor_demand_contracts().len()
        || coverage.dependency_count() != inventory.dependency_dag().edges().len()
        || inventory.exact_extension().predecessor_boundary_digest()
            != inventory.predecessor_boundary().digest()
        || inventory.exact_extension().successor_boundary_digest()
            != inventory.successor_boundary().digest()
        || !inventory.q3_registry().is_empty()
        || inventory.q3_registry().predecessor_history_digest()
            != inventory.predecessor_history_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::MissingVerifiedPublicInventory);
    }

    let invariant_digest = Digest::of_canonical(
        "pen-semantic-audit/public-inventory-preserved-invariants/v2",
        &InvariantMaterial {
            v1_profile_id: &v1.profile_id,
            v2_profile_id: &v2.profile_id,
            universe_levels: &v1.universe_levels,
            maximum_context_entries: v1.maximum_context_entries,
            q0_rules: &v1.q0_rules,
            q0_eta_registry_empty: v1.q0_eta_registry_empty,
            q3_rule: v1.q3_rule,
            preserved_invariants: &PRESERVED_INVARIANTS,
        },
    );

    let v1_manifest_digest = v1_manifest.candidate_digest().clone();
    let v2_manifest_digest = v2_manifest.candidate_digest().clone();
    let inventory_digest = inventory.digest().clone();
    let coverage_digest = coverage.digest().clone();
    let predecessor_history_digest = inventory.predecessor_history_digest().clone();
    let predecessor_boundary_digest = inventory.predecessor_boundary().digest().clone();
    let successor_boundary_digest = inventory.successor_boundary().digest().clone();
    let exact_extension_digest = inventory.exact_extension().digest().clone();
    let normalizer_protocol_digest = inventory.normalizer_protocol_digest().clone();
    let q3_registry_digest = inventory.q3_registry().digest().clone();
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-public-inventory-compatibility/v2",
        &CompatibilityMaterial {
            v1_manifest: &v1_manifest_digest,
            v2_manifest: &v2_manifest_digest,
            inventory: &inventory_digest,
            coverage: &coverage_digest,
            predecessor_history: &predecessor_history_digest,
            predecessor_boundary: &predecessor_boundary_digest,
            successor_boundary: &successor_boundary_digest,
            exact_extension: &exact_extension_digest,
            normalizer_protocol: &normalizer_protocol_digest,
            q3_registry: &q3_registry_digest,
            invariants: &invariant_digest,
        },
    );

    AuditDecision::Proven(VerifiedPublicInventoryCompatibilityV2 {
        v1_manifest_digest,
        v2_manifest_digest,
        inventory_digest,
        coverage_digest,
        predecessor_history_digest,
        predecessor_boundary_digest,
        successor_boundary_digest,
        exact_extension_digest,
        normalizer_protocol_digest,
        q3_registry_digest,
        preserved_invariants: Arc::from(PRESERVED_INVARIANTS),
        invariant_digest,
        digest,
    })
}

fn public_inventory_syntax_violation(
    inventory: &VerifiedPublicAuditInventoryV1,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    let mut violation = None;
    for declaration in inventory.declarations() {
        for candidate in [declaration.source(), declaration.normalized()] {
            violation = violation.max(lambda_unit_term_syntax_violation(
                &candidate.ty,
                universe_levels,
            ));
            if let Some(body) = &candidate.body {
                violation = violation.max(lambda_unit_term_syntax_violation(body, universe_levels));
            }
        }
    }
    for equation in inventory.equations() {
        violation = violation.max(lambda_unit_judgment_syntax_violation(
            equation.source(),
            universe_levels,
        ));
        violation = violation.max(lambda_unit_judgment_syntax_violation(
            equation.normalized(),
            universe_levels,
        ));
    }
    for demand in inventory.predecessor_demand_contracts() {
        violation = violation.max(lambda_unit_judgment_syntax_violation(
            demand.source_requirement(),
            universe_levels,
        ));
        violation = violation.max(lambda_unit_judgment_syntax_violation(
            demand.normalized_requirement(),
            universe_levels,
        ));
    }
    violation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        ORIGIN_CUTOFF_Q3_SCHEMA_VERSION, PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
        UncheckedOriginCutoffQ3RegistryV1, UncheckedPublicAuditInventoryV1,
        UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEventCensusV1,
        UncheckedPublicGroupV1, UncheckedSourceNormalizedDeclarationV1,
        verify_public_audit_inventory_v1,
    };
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_lambda_unit_manifest_v2, proposed_semantic_audit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v2, verify_semantic_audit_manifest_v1,
    };
    use crate::model::EventIdV1;
    use pen_kernel::{Declaration, GlobalId, Kernel, KernelLimits, Term, UncheckedSignature};

    fn require_proven<T>(decision: AuditDecision<T>) -> T {
        match decision {
            AuditDecision::Proven(value) => value,
            AuditDecision::OutsideFragment(reason) => {
                panic!("unexpected outside-fragment result: {reason:?}")
            }
            AuditDecision::Unknown(reason) => panic!("unexpected unknown result: {reason:?}"),
        }
    }

    fn verified_lambda_manifests() -> (
        VerifiedSemanticAuditManifestV1,
        VerifiedSemanticAuditManifestV2,
    ) {
        (
            require_proven(verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )),
            require_proven(verify_semantic_audit_lambda_unit_manifest_v2(
                &proposed_semantic_audit_lambda_unit_manifest_v2(),
            )),
        )
    }

    fn minimal_inventory(
        manifest: &VerifiedSemanticAuditManifestV1,
    ) -> VerifiedPublicAuditInventoryV1 {
        let declaration_id = GlobalId(Digest::of_bytes(b"compat/declaration"));
        minimal_inventory_with_declaration(
            manifest,
            Declaration {
                id: declaration_id,
                ty: Term::Sort { level: 0 },
                body: None,
            },
        )
    }

    fn minimal_inventory_with_declaration(
        manifest: &VerifiedSemanticAuditManifestV1,
        declaration: Declaration,
    ) -> VerifiedPublicAuditInventoryV1 {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let declaration_id = declaration.id.clone();
        let group_id = GlobalId(Digest::of_bytes(b"compat/group"));
        let event_id = EventIdV1(Digest::of_bytes(b"compat/event"));
        let source_identity = Digest::of_canonical(
            "pen-semantic-audit/inventory-source-declaration/v1",
            &declaration,
        );
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: Vec::new(),
            predecessor_boundary: UncheckedSignature::default(),
            successor_event: UncheckedPublicEventCensusV1 {
                event: event_id.clone(),
                added_groups: vec![group_id.clone()],
                added_declarations: vec![declaration_id.clone()],
                added_equations: Vec::new(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary: UncheckedSignature {
                declarations: vec![declaration.clone()],
            },
            declaration_groups: vec![UncheckedPublicGroupV1 {
                group: group_id.clone(),
                origin: event_id.clone(),
                declarations: vec![declaration_id.clone()],
            }],
            declarations: vec![UncheckedPublicDeclarationV1 {
                declaration: declaration_id,
                origin: event_id,
                group: group_id,
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
        require_proven(verify_public_audit_inventory_v1(manifest, &kernel, &wire))
    }

    #[test]
    fn exact_profiles_mint_narrow_digest_bound_compatibility() {
        let (v1, v2) = verified_lambda_manifests();
        let inventory = minimal_inventory(&v1);

        let compatibility = require_proven(verify_public_inventory_compatibility_v2(
            &v1, &v2, &inventory,
        ));

        assert_eq!(compatibility.v1_manifest_digest(), v1.candidate_digest());
        assert_eq!(compatibility.v2_manifest_digest(), v2.candidate_digest());
        assert_eq!(compatibility.inventory_digest(), inventory.digest());
        assert_eq!(
            compatibility.coverage_digest(),
            inventory.coverage().digest()
        );
        assert_eq!(
            compatibility.predecessor_history_digest(),
            inventory.predecessor_history_digest()
        );
        assert_eq!(
            compatibility.predecessor_boundary_digest(),
            inventory.predecessor_boundary().digest()
        );
        assert_eq!(
            compatibility.successor_boundary_digest(),
            inventory.successor_boundary().digest()
        );
        assert_eq!(
            compatibility.normalizer_protocol_digest(),
            inventory.normalizer_protocol_digest()
        );
        assert_eq!(compatibility.preserved_invariants(), &PRESERVED_INVARIANTS);
    }

    #[test]
    fn generic_v1_profile_cannot_use_the_lambda_unit_compatibility_theorem() {
        let core_v1 = require_proven(verify_semantic_audit_manifest_v1(
            &proposed_semantic_audit_manifest_v1(),
        ));
        let (_, v2) = verified_lambda_manifests();
        let inventory = minimal_inventory(&core_v1);

        assert!(matches!(
            verify_public_inventory_compatibility_v2(&core_v1, &v2, &inventory),
            AuditDecision::Unknown(AuditUnknownReason::MissingVerifiedPublicInventory)
        ));
    }

    #[test]
    fn inventory_manifest_binding_is_not_replaced_by_profile_compatibility() {
        let core_v1 = require_proven(verify_semantic_audit_manifest_v1(
            &proposed_semantic_audit_manifest_v1(),
        ));
        let core_inventory = minimal_inventory(&core_v1);
        let (lambda_v1, lambda_v2) = verified_lambda_manifests();

        assert!(matches!(
            verify_public_inventory_compatibility_v2(&lambda_v1, &lambda_v2, &core_inventory),
            AuditDecision::Unknown(AuditUnknownReason::MissingVerifiedPublicInventory)
        ));
    }

    #[test]
    fn replayed_inventory_outside_lambda_unit_surface_is_not_transported() {
        let (v1, v2) = verified_lambda_manifests();
        let inventory = minimal_inventory_with_declaration(
            &v1,
            Declaration {
                id: GlobalId(Digest::of_bytes(b"compat/sigma")),
                ty: Term::Sort { level: 0 },
                body: Some(Term::Sigma {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::UnitType),
                }),
            },
        );

        assert!(matches!(
            verify_public_inventory_compatibility_v2(&v1, &v2, &inventory),
            AuditDecision::OutsideFragment(OutsideFragmentReason::UnsupportedTerm)
        ));
    }

    #[test]
    fn compatibility_digest_is_deterministic() {
        let (v1, v2) = verified_lambda_manifests();
        let inventory = minimal_inventory(&v1);
        let first = require_proven(verify_public_inventory_compatibility_v2(
            &v1, &v2, &inventory,
        ));
        let second = require_proven(verify_public_inventory_compatibility_v2(
            &v1, &v2, &inventory,
        ));

        assert_eq!(first.invariant_digest(), second.invariant_digest());
        assert_eq!(first.digest(), second.digest());
    }
}
