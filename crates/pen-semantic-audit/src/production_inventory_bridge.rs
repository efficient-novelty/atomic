//! Exact production-code inventory bridge for the lambda/unit V3 successor.
//!
//! This lane proves exact ordered coverage and payload-free classification of
//! the existing V3 Q0 and family-constructor inventories. It does not prove
//! Rust/Agda term correspondence, conversion soundness, carrier-derived
//! subject completeness, or any rewrite authority.

use crate::manifest::{
    AuditDecision, AuditUnknownReason, DerivationRuleV1, ManifestAuthority, Q0RuleV1,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3, SEMANTIC_AUDIT_SCHEMA_VERSION_V3,
    VerifiedSemanticAuditManifestV3,
};
use crate::model::FamilyConstructorV1;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};

pub const EXACT_V3_Q0_RULE_INVENTORY_V1: [Q0RuleV1; 7] = [
    Q0RuleV1::DeBruijn,
    Q0RuleV1::SequentialSubstitution,
    Q0RuleV1::Beta,
    Q0RuleV1::ProvenancePreservingDelta,
    Q0RuleV1::Unit,
    Q0RuleV1::TelescopeFlattening,
    Q0RuleV1::FreshNonrecursiveConstructorComputation,
];

pub const EXACT_PRODUCTION_FAMILY_RULE_INVENTORY_V1: [DerivationRuleV1; 3] = [
    DerivationRuleV1::Seed,
    DerivationRuleV1::GenericPublicApplication,
    DerivationRuleV1::GenericEquationAction,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionQ0CategoryV1 {
    Representation,
    BaseSemantic,
    RuntimePublic,
}

impl CanonicalEncode for ProductionQ0CategoryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Representation => 0,
            Self::BaseSemantic => 1,
            Self::RuntimePublic => 2,
        });
    }
}

/// Classify a Q0 tag only when it belongs to the exact projection-free V3
/// inventory. The excluded projection tag has no V3 production category.
pub fn production_q0_category_v1(rule: Q0RuleV1) -> Option<ProductionQ0CategoryV1> {
    match rule {
        Q0RuleV1::DeBruijn | Q0RuleV1::SequentialSubstitution | Q0RuleV1::TelescopeFlattening => {
            Some(ProductionQ0CategoryV1::Representation)
        }
        Q0RuleV1::Beta | Q0RuleV1::ProvenancePreservingDelta | Q0RuleV1::Unit => {
            Some(ProductionQ0CategoryV1::BaseSemantic)
        }
        Q0RuleV1::FreshNonrecursiveConstructorComputation => {
            Some(ProductionQ0CategoryV1::RuntimePublic)
        }
        Q0RuleV1::DescriptorForcedProjection => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionFamilyCodeV1 {
    Seed,
    GenericPublicApplication,
    GenericEquationAction,
}

impl CanonicalEncode for ProductionFamilyCodeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Seed => 0,
            Self::GenericPublicApplication => 1,
            Self::GenericEquationAction => 2,
        });
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FamilyConstructorShapeV1 {
    PublicHeadSeed,
    PublicEquationSeed,
    GenericPublicApplication,
    GenericEquationAction,
}

impl CanonicalEncode for FamilyConstructorShapeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::PublicHeadSeed => 0,
            Self::PublicEquationSeed => 1,
            Self::GenericPublicApplication => 2,
            Self::GenericEquationAction => 3,
        });
    }
}

pub const EXACT_FAMILY_CONSTRUCTOR_SHAPES_V1: [FamilyConstructorShapeV1; 4] = [
    FamilyConstructorShapeV1::PublicHeadSeed,
    FamilyConstructorShapeV1::PublicEquationSeed,
    FamilyConstructorShapeV1::GenericPublicApplication,
    FamilyConstructorShapeV1::GenericEquationAction,
];

pub fn family_constructor_shape_v1(constructor: &FamilyConstructorV1) -> FamilyConstructorShapeV1 {
    match constructor {
        FamilyConstructorV1::PublicHeadSeed { .. } => FamilyConstructorShapeV1::PublicHeadSeed,
        FamilyConstructorV1::PublicEquationSeed { .. } => {
            FamilyConstructorShapeV1::PublicEquationSeed
        }
        FamilyConstructorV1::GenericPublicApplication { .. } => {
            FamilyConstructorShapeV1::GenericPublicApplication
        }
        FamilyConstructorV1::GenericEquationAction { .. } => {
            FamilyConstructorShapeV1::GenericEquationAction
        }
    }
}

pub fn production_family_code_for_shape_v1(
    shape: FamilyConstructorShapeV1,
) -> ProductionFamilyCodeV1 {
    match shape {
        FamilyConstructorShapeV1::PublicHeadSeed | FamilyConstructorShapeV1::PublicEquationSeed => {
            ProductionFamilyCodeV1::Seed
        }
        FamilyConstructorShapeV1::GenericPublicApplication => {
            ProductionFamilyCodeV1::GenericPublicApplication
        }
        FamilyConstructorShapeV1::GenericEquationAction => {
            ProductionFamilyCodeV1::GenericEquationAction
        }
    }
}

pub fn production_family_code_v1(constructor: &FamilyConstructorV1) -> ProductionFamilyCodeV1 {
    production_family_code_for_shape_v1(family_constructor_shape_v1(constructor))
}

pub fn production_family_code_for_derivation_rule_v1(
    rule: DerivationRuleV1,
) -> ProductionFamilyCodeV1 {
    match rule {
        DerivationRuleV1::Seed => ProductionFamilyCodeV1::Seed,
        DerivationRuleV1::GenericPublicApplication => {
            ProductionFamilyCodeV1::GenericPublicApplication
        }
        DerivationRuleV1::GenericEquationAction => ProductionFamilyCodeV1::GenericEquationAction,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedQ0ProductionClassificationV1 {
    rule: Q0RuleV1,
    category: ProductionQ0CategoryV1,
}

impl VerifiedQ0ProductionClassificationV1 {
    pub fn rule(&self) -> Q0RuleV1 {
        self.rule
    }

    pub fn category(&self) -> ProductionQ0CategoryV1 {
        self.category
    }
}

impl CanonicalEncode for VerifiedQ0ProductionClassificationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.rule.encode_canonical(encoder);
        self.category.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedFamilyProductionMappingV1 {
    shape: FamilyConstructorShapeV1,
    production_code: ProductionFamilyCodeV1,
}

impl VerifiedFamilyProductionMappingV1 {
    pub fn shape(&self) -> FamilyConstructorShapeV1 {
        self.shape
    }

    pub fn production_code(&self) -> ProductionFamilyCodeV1 {
        self.production_code
    }
}

impl CanonicalEncode for VerifiedFamilyProductionMappingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.shape.encode_canonical(encoder);
        self.production_code.encode_canonical(encoder);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionInventoryKindV1 {
    Q0Rules,
    FamilyRules,
}

impl CanonicalEncode for ProductionInventoryKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Q0Rules => 0,
            Self::FamilyRules => 1,
        });
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteOrderedCoverage;
}

/// Opaque verifier evidence that the observed and expected ordered sequences
/// have equal counts, equal members, equal order, and equal canonical digests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedNoExtraNoMissingInventoryV1 {
    kind: ProductionInventoryKindV1,
    expected_count: u64,
    observed_count: u64,
    expected_sequence_digest: Digest,
    observed_sequence_digest: Digest,
    coverage_digest: Digest,
    complete: private::CompleteOrderedCoverage,
    digest: Digest,
}

impl VerifiedNoExtraNoMissingInventoryV1 {
    pub fn kind(&self) -> ProductionInventoryKindV1 {
        self.kind
    }

    pub fn expected_count(&self) -> u64 {
        self.expected_count
    }

    pub fn observed_count(&self) -> u64 {
        self.observed_count
    }

    pub fn expected_sequence_digest(&self) -> &Digest {
        &self.expected_sequence_digest
    }

    pub fn observed_sequence_digest(&self) -> &Digest {
        &self.observed_sequence_digest
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedNoExtraNoMissingInventoryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.kind.encode_canonical(encoder);
        encoder.u64(self.expected_count);
        encoder.u64(self.observed_count);
        self.expected_sequence_digest.encode_canonical(encoder);
        self.observed_sequence_digest.encode_canonical(encoder);
        self.coverage_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Opaque exact-inventory bridge. This type has private fields and no
/// deserialization path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedProductionInventoryBridgeV1 {
    semantic_manifest_digest: Digest,
    q0_classifications: Vec<VerifiedQ0ProductionClassificationV1>,
    family_mappings: Vec<VerifiedFamilyProductionMappingV1>,
    q0_no_extra_no_missing: VerifiedNoExtraNoMissingInventoryV1,
    family_no_extra_no_missing: VerifiedNoExtraNoMissingInventoryV1,
    digest: Digest,
}

impl VerifiedProductionInventoryBridgeV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn q0_classifications(&self) -> &[VerifiedQ0ProductionClassificationV1] {
        &self.q0_classifications
    }

    pub fn family_mappings(&self) -> &[VerifiedFamilyProductionMappingV1] {
        &self.family_mappings
    }

    pub fn q0_no_extra_no_missing(&self) -> &VerifiedNoExtraNoMissingInventoryV1 {
        &self.q0_no_extra_no_missing
    }

    pub fn family_no_extra_no_missing(&self) -> &VerifiedNoExtraNoMissingInventoryV1 {
        &self.family_no_extra_no_missing
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedProductionInventoryBridgeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        encoder.sequence(&self.q0_classifications);
        encoder.sequence(&self.family_mappings);
        self.q0_no_extra_no_missing.encode_canonical(encoder);
        self.family_no_extra_no_missing.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionInventoryBridgeFailureV1 {
    ManifestMismatch,
    Q0InventoryNotExact,
    FamilyRuleInventoryNotExact,
    InternalClassificationMismatch,
}

impl std::fmt::Display for ProductionInventoryBridgeFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ManifestMismatch => {
                "semantic manifest is not the exact generic unfrozen lambda/unit V3 successor"
            }
            Self::Q0InventoryNotExact => {
                "V3 Q0 inventory has an extra, missing, duplicate, reordered, or changed rule"
            }
            Self::FamilyRuleInventoryNotExact => {
                "V3 family-rule inventory has an extra, missing, duplicate, reordered, or changed code"
            }
            Self::InternalClassificationMismatch => {
                "an exact V3 inventory code has no unique production classification"
            }
        })
    }
}

impl std::error::Error for ProductionInventoryBridgeFailureV1 {}

pub fn diagnose_production_inventory_bridge_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<VerifiedProductionInventoryBridgeV1, ProductionInventoryBridgeFailureV1> {
    let wire = manifest.manifest();
    if wire.schema_version != SEMANTIC_AUDIT_SCHEMA_VERSION_V3
        || wire.profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3
        || wire.authority != ManifestAuthority::GenericPrototypeOnly
        || wire.frozen
        || wire.live_profile_a_access
    {
        return Err(ProductionInventoryBridgeFailureV1::ManifestMismatch);
    }

    let q0_no_extra_no_missing = verify_exact_ordered_inventory(
        ProductionInventoryKindV1::Q0Rules,
        &EXACT_V3_Q0_RULE_INVENTORY_V1,
        &wire.q0_rules,
    )
    .ok_or(ProductionInventoryBridgeFailureV1::Q0InventoryNotExact)?;
    let family_no_extra_no_missing = verify_exact_ordered_inventory(
        ProductionInventoryKindV1::FamilyRules,
        &EXACT_PRODUCTION_FAMILY_RULE_INVENTORY_V1,
        &wire.ordered_derivation_rules,
    )
    .ok_or(ProductionInventoryBridgeFailureV1::FamilyRuleInventoryNotExact)?;

    let q0_classifications = EXACT_V3_Q0_RULE_INVENTORY_V1
        .into_iter()
        .map(|rule| {
            production_q0_category_v1(rule)
                .map(|category| VerifiedQ0ProductionClassificationV1 { rule, category })
                .ok_or(ProductionInventoryBridgeFailureV1::InternalClassificationMismatch)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let family_mappings = EXACT_FAMILY_CONSTRUCTOR_SHAPES_V1
        .into_iter()
        .map(|shape| VerifiedFamilyProductionMappingV1 {
            shape,
            production_code: production_family_code_for_shape_v1(shape),
        })
        .collect();

    let mut bridge = VerifiedProductionInventoryBridgeV1 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        q0_classifications,
        family_mappings,
        q0_no_extra_no_missing,
        family_no_extra_no_missing,
        digest: Digest::of_bytes(b"production-inventory-bridge-v1-pending"),
    };
    bridge.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-production-inventory-bridge/v1",
        &ProductionInventoryBridgeDigestInput(&bridge),
    );
    Ok(bridge)
}

pub fn verify_production_inventory_bridge_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
) -> AuditDecision<VerifiedProductionInventoryBridgeV1> {
    match diagnose_production_inventory_bridge_v1(manifest) {
        Ok(bridge) => AuditDecision::Proven(bridge),
        Err(ProductionInventoryBridgeFailureV1::ManifestMismatch) => {
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        }
        Err(
            ProductionInventoryBridgeFailureV1::Q0InventoryNotExact
            | ProductionInventoryBridgeFailureV1::FamilyRuleInventoryNotExact
            | ProductionInventoryBridgeFailureV1::InternalClassificationMismatch,
        ) => AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration),
    }
}

fn verify_exact_ordered_inventory<T>(
    kind: ProductionInventoryKindV1,
    expected: &[T],
    observed: &[T],
) -> Option<VerifiedNoExtraNoMissingInventoryV1>
where
    T: CanonicalEncode + PartialEq,
{
    if expected != observed {
        return None;
    }
    let expected_count = u64::try_from(expected.len()).ok()?;
    let observed_count = u64::try_from(observed.len()).ok()?;
    let expected_sequence_digest = inventory_sequence_digest(kind, expected);
    let observed_sequence_digest = inventory_sequence_digest(kind, observed);
    if expected_count != observed_count || expected_sequence_digest != observed_sequence_digest {
        return None;
    }
    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/no-extra-no-missing-coverage/v1",
        &CoverageMaterial {
            kind,
            expected_count,
            observed_count,
            expected_sequence_digest: &expected_sequence_digest,
            observed_sequence_digest: &observed_sequence_digest,
        },
    );
    let mut verified = VerifiedNoExtraNoMissingInventoryV1 {
        kind,
        expected_count,
        observed_count,
        expected_sequence_digest,
        observed_sequence_digest,
        coverage_digest,
        complete: private::CompleteOrderedCoverage,
        digest: Digest::of_bytes(b"no-extra-no-missing-inventory-v1-pending"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-no-extra-no-missing-inventory/v1",
        &NoExtraNoMissingDigestInput(&verified),
    );
    Some(verified)
}

fn inventory_sequence_digest<T>(kind: ProductionInventoryKindV1, values: &[T]) -> Digest
where
    T: CanonicalEncode,
{
    Digest::of_canonical(
        "pen-semantic-audit/production-inventory-sequence/v1",
        &InventorySequence { kind, values },
    )
}

struct InventorySequence<'a, T> {
    kind: ProductionInventoryKindV1,
    values: &'a [T],
}

impl<T> CanonicalEncode for InventorySequence<'_, T>
where
    T: CanonicalEncode,
{
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.kind.encode_canonical(encoder);
        encoder.sequence(self.values);
    }
}

struct CoverageMaterial<'a> {
    kind: ProductionInventoryKindV1,
    expected_count: u64,
    observed_count: u64,
    expected_sequence_digest: &'a Digest,
    observed_sequence_digest: &'a Digest,
}

impl CanonicalEncode for CoverageMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.kind.encode_canonical(encoder);
        encoder.u64(self.expected_count);
        encoder.u64(self.observed_count);
        self.expected_sequence_digest.encode_canonical(encoder);
        self.observed_sequence_digest.encode_canonical(encoder);
    }
}

struct NoExtraNoMissingDigestInput<'a>(&'a VerifiedNoExtraNoMissingInventoryV1);

impl CanonicalEncode for NoExtraNoMissingDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.kind.encode_canonical(encoder);
        encoder.u64(self.0.expected_count);
        encoder.u64(self.0.observed_count);
        self.0.expected_sequence_digest.encode_canonical(encoder);
        self.0.observed_sequence_digest.encode_canonical(encoder);
        self.0.coverage_digest.encode_canonical(encoder);
    }
}

struct ProductionInventoryBridgeDigestInput<'a>(&'a VerifiedProductionInventoryBridgeV1);

impl CanonicalEncode for ProductionInventoryBridgeDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.semantic_manifest_digest.encode_canonical(encoder);
        encoder.sequence(&self.0.q0_classifications);
        encoder.sequence(&self.0.family_mappings);
        self.0.q0_no_extra_no_missing.encode_canonical(encoder);
        self.0.family_no_extra_no_missing.encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v3,
        verify_semantic_audit_lambda_unit_manifest_v3,
    };
    use crate::model::{ContextWitnessIdV1, RawFamilyIdV1, SeedIdV1};

    fn exact_manifest() -> VerifiedSemanticAuditManifestV3 {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v3(
            &proposed_semantic_audit_lambda_unit_manifest_v3(),
        ) else {
            panic!("exact V3 manifest");
        };
        manifest
    }

    #[test]
    fn exact_v3_inventories_mint_only_the_private_bridge() {
        let manifest = exact_manifest();
        let AuditDecision::Proven(bridge) = verify_production_inventory_bridge_v1(&manifest) else {
            panic!("exact production inventory bridge");
        };
        assert_eq!(
            bridge.semantic_manifest_digest(),
            manifest.candidate_digest()
        );
        assert_eq!(
            bridge
                .q0_classifications()
                .iter()
                .map(VerifiedQ0ProductionClassificationV1::rule)
                .collect::<Vec<_>>(),
            EXACT_V3_Q0_RULE_INVENTORY_V1
        );
        assert_eq!(
            bridge
                .q0_classifications()
                .iter()
                .map(VerifiedQ0ProductionClassificationV1::category)
                .collect::<Vec<_>>(),
            [
                ProductionQ0CategoryV1::Representation,
                ProductionQ0CategoryV1::Representation,
                ProductionQ0CategoryV1::BaseSemantic,
                ProductionQ0CategoryV1::BaseSemantic,
                ProductionQ0CategoryV1::BaseSemantic,
                ProductionQ0CategoryV1::Representation,
                ProductionQ0CategoryV1::RuntimePublic,
            ]
        );
        for evidence in [
            bridge.q0_no_extra_no_missing(),
            bridge.family_no_extra_no_missing(),
        ] {
            assert_eq!(evidence.expected_count(), evidence.observed_count());
            assert_eq!(
                evidence.expected_sequence_digest(),
                evidence.observed_sequence_digest()
            );
        }
        assert_ne!(bridge.digest(), &Digest::of_bytes(b""));
    }

    #[test]
    fn exact_ordered_evidence_rejects_every_coverage_defect() {
        let exact = EXACT_V3_Q0_RULE_INVENTORY_V1;
        assert!(
            verify_exact_ordered_inventory(ProductionInventoryKindV1::Q0Rules, &exact, &exact)
                .is_some()
        );
        assert!(
            verify_exact_ordered_inventory(
                ProductionInventoryKindV1::Q0Rules,
                &exact,
                &exact[..exact.len() - 1],
            )
            .is_none()
        );
        let mut extra = exact.to_vec();
        extra.push(Q0RuleV1::DescriptorForcedProjection);
        assert!(
            verify_exact_ordered_inventory(ProductionInventoryKindV1::Q0Rules, &exact, &extra)
                .is_none()
        );
        let mut duplicate = exact;
        duplicate[6] = Q0RuleV1::Unit;
        assert!(
            verify_exact_ordered_inventory(ProductionInventoryKindV1::Q0Rules, &exact, &duplicate,)
                .is_none()
        );
        let mut reordered = exact;
        reordered.swap(0, 1);
        assert!(
            verify_exact_ordered_inventory(ProductionInventoryKindV1::Q0Rules, &exact, &reordered,)
                .is_none()
        );
        assert_eq!(
            production_q0_category_v1(Q0RuleV1::DescriptorForcedProjection),
            None
        );
    }

    #[test]
    fn all_four_family_shapes_map_to_the_exact_three_production_codes() {
        let seed = SeedIdV1(Digest::of_bytes(b"production-inventory/seed"));
        let family = RawFamilyIdV1(Digest::of_bytes(b"production-inventory/family"));
        let context = ContextWitnessIdV1(Digest::of_bytes(b"production-inventory/context"));
        let constructors = [
            (
                FamilyConstructorV1::PublicHeadSeed { seed: seed.clone() },
                FamilyConstructorShapeV1::PublicHeadSeed,
                ProductionFamilyCodeV1::Seed,
            ),
            (
                FamilyConstructorV1::PublicEquationSeed { seed },
                FamilyConstructorShapeV1::PublicEquationSeed,
                ProductionFamilyCodeV1::Seed,
            ),
            (
                FamilyConstructorV1::GenericPublicApplication {
                    function: family.clone(),
                    argument: family.clone(),
                    context_witness: context.clone(),
                },
                FamilyConstructorShapeV1::GenericPublicApplication,
                ProductionFamilyCodeV1::GenericPublicApplication,
            ),
            (
                FamilyConstructorV1::GenericEquationAction {
                    equation: family.clone(),
                    context: family,
                    hole_ordinal: 0,
                    context_witness: context,
                },
                FamilyConstructorShapeV1::GenericEquationAction,
                ProductionFamilyCodeV1::GenericEquationAction,
            ),
        ];
        for (constructor, shape, code) in constructors {
            assert_eq!(family_constructor_shape_v1(&constructor), shape);
            assert_eq!(production_family_code_v1(&constructor), code);
        }
        assert_eq!(
            EXACT_PRODUCTION_FAMILY_RULE_INVENTORY_V1
                .map(production_family_code_for_derivation_rule_v1),
            [
                ProductionFamilyCodeV1::Seed,
                ProductionFamilyCodeV1::GenericPublicApplication,
                ProductionFamilyCodeV1::GenericEquationAction,
            ]
        );
    }
}
