//! Independently verified historical rewrite authority for the V2 profile.
//!
//! The empty predecessor is a positive base theorem.  A predecessor with any
//! public equation requires a previously issued theorem for that exact
//! boundary; stored equation syntax alone is never promoted to historical
//! orientation or admissibility authority.

use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::inventory_compatibility::VerifiedPublicInventoryCompatibilityV2;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, Q0RuleV1, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2,
    VerifiedSemanticAuditManifestV2,
};
use crate::model::EquationIdV1;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::sync::Arc;

/// Q0 rules available independently of any sealed public rewrite equation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoricalBaseQ0RuleV1 {
    DeBruijn,
    SequentialSubstitution,
    OrdinaryBeta,
    ProvenancePreservingPublicDelta,
    Unit,
    TelescopeFlattening,
}

impl CanonicalEncode for HistoricalBaseQ0RuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::DeBruijn => 0,
            Self::SequentialSubstitution => 1,
            Self::OrdinaryBeta => 2,
            Self::ProvenancePreservingPublicDelta => 3,
            Self::Unit => 4,
            Self::TelescopeFlattening => 5,
        });
    }
}

const EMPTY_HISTORY_BASE_RULES: [HistoricalBaseQ0RuleV1; 6] = [
    HistoricalBaseQ0RuleV1::DeBruijn,
    HistoricalBaseQ0RuleV1::SequentialSubstitution,
    HistoricalBaseQ0RuleV1::OrdinaryBeta,
    HistoricalBaseQ0RuleV1::ProvenancePreservingPublicDelta,
    HistoricalBaseQ0RuleV1::Unit,
    HistoricalBaseQ0RuleV1::TelescopeFlattening,
];

/// Positive proof that an exact predecessor has zero public rewrite
/// equations and only the exact base Q0 inventory.
#[derive(Clone, Debug)]
pub struct VerifiedEmptyHistoricalRewriteSystemV1 {
    semantic_manifest_digest: Digest,
    inventory_compatibility_digest: Digest,
    history_digest: Digest,
    boundary_digest: Digest,
    inventory_digest: Digest,
    complete_equation_census_digest: Digest,
    base_q0_rule_inventory: Arc<[HistoricalBaseQ0RuleV1]>,
    base_q0_rule_inventory_digest: Digest,
    rewrite_system_digest: Digest,
    digest: Digest,
}

impl VerifiedEmptyHistoricalRewriteSystemV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn inventory_compatibility_digest(&self) -> &Digest {
        &self.inventory_compatibility_digest
    }

    pub fn history_digest(&self) -> &Digest {
        &self.history_digest
    }

    pub fn boundary_digest(&self) -> &Digest {
        &self.boundary_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn complete_equation_census_digest(&self) -> &Digest {
        &self.complete_equation_census_digest
    }

    pub fn base_q0_rule_inventory(&self) -> &[HistoricalBaseQ0RuleV1] {
        &self.base_q0_rule_inventory
    }

    pub fn base_q0_rule_inventory_digest(&self) -> &Digest {
        &self.base_q0_rule_inventory_digest
    }

    pub fn rewrite_system_digest(&self) -> &Digest {
        &self.rewrite_system_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Historical rewrite authority for one exact predecessor boundary.
///
/// The current implementation mints only the verified empty base.  Its
/// private fields and lack of `Deserialize` prevent a caller from presenting
/// stored historical syntax as a theorem.
#[derive(Clone, Debug)]
pub struct VerifiedHistoricalRewriteSystemV1 {
    history_digest: Digest,
    boundary_digest: Digest,
    inventory_digest: Digest,
    rewrite_inventory_digest: Digest,
    rewrite_system_digest: Digest,
    empty_base: VerifiedEmptyHistoricalRewriteSystemV1,
    digest: Digest,
}

impl VerifiedHistoricalRewriteSystemV1 {
    pub fn history_digest(&self) -> &Digest {
        &self.history_digest
    }

    pub fn boundary_digest(&self) -> &Digest {
        &self.boundary_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn rewrite_inventory_digest(&self) -> &Digest {
        &self.rewrite_inventory_digest
    }

    pub fn rewrite_system_digest(&self) -> &Digest {
        &self.rewrite_system_digest
    }

    pub fn empty_base(&self) -> &VerifiedEmptyHistoricalRewriteSystemV1 {
        &self.empty_base
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Verify the empty historical base or stop at the inductive theorem
/// boundary for a nonempty predecessor.
pub fn verify_historical_rewrite_system_v1(
    manifest: &VerifiedSemanticAuditManifestV2,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
) -> AuditDecision<VerifiedHistoricalRewriteSystemV1> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2
        || compatibility.v2_manifest_digest() != manifest.candidate_digest()
        || compatibility.inventory_digest() != inventory.digest()
        || compatibility.predecessor_history_digest() != inventory.predecessor_history_digest()
        || compatibility.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let mut predecessor_equations = inventory
        .equations()
        .iter()
        .filter(|equation| equation.is_predecessor_public())
        .map(|equation| equation.equation().clone())
        .collect::<Vec<_>>();
    predecessor_equations.sort();
    if !predecessor_equations.is_empty() {
        return AuditDecision::Unknown(AuditUnknownReason::MissingHistoricalRewriteAuthority);
    }

    let required_manifest_rules = [
        Q0RuleV1::DeBruijn,
        Q0RuleV1::SequentialSubstitution,
        Q0RuleV1::Beta,
        Q0RuleV1::ProvenancePreservingDelta,
        Q0RuleV1::Unit,
        Q0RuleV1::TelescopeFlattening,
    ];
    if !required_manifest_rules
        .iter()
        .all(|rule| manifest.manifest().q0_rules.contains(rule))
        || manifest
            .manifest()
            .q0_rules
            .contains(&Q0RuleV1::DescriptorForcedProjection)
        || !inventory.forced_projections().is_empty()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let complete_equation_census_digest = Digest::of_canonical(
        "pen-semantic-audit/complete-empty-historical-equation-census/v1",
        &HistoricalEquationCensusMaterial {
            history: inventory.predecessor_history_digest(),
            boundary: inventory.predecessor_boundary().digest(),
            inventory: inventory.digest(),
            equations: &predecessor_equations,
        },
    );
    let base_q0_rule_inventory_digest = Digest::of_canonical(
        "pen-semantic-audit/historical-base-q0-rule-inventory/v1",
        &CanonicalRules(&EMPTY_HISTORY_BASE_RULES),
    );
    let rewrite_system_digest = Digest::of_canonical(
        "pen-semantic-audit/empty-historical-rewrite-system/v1",
        &EmptyHistoricalRewriteMaterial {
            manifest: manifest.candidate_digest(),
            compatibility: compatibility.digest(),
            history: inventory.predecessor_history_digest(),
            boundary: inventory.predecessor_boundary().digest(),
            equation_census: &complete_equation_census_digest,
            base_rules: &base_q0_rule_inventory_digest,
        },
    );
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-empty-historical-rewrite-system/v1",
        &HistoricalCapabilityMaterial {
            manifest: manifest.candidate_digest(),
            compatibility: compatibility.digest(),
            inventory: inventory.digest(),
            history: inventory.predecessor_history_digest(),
            boundary: inventory.predecessor_boundary().digest(),
            rewrite_inventory: &complete_equation_census_digest,
            rewrite_system: &rewrite_system_digest,
        },
    );
    let empty_base = VerifiedEmptyHistoricalRewriteSystemV1 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        inventory_compatibility_digest: compatibility.digest().clone(),
        history_digest: inventory.predecessor_history_digest().clone(),
        boundary_digest: inventory.predecessor_boundary().digest().clone(),
        inventory_digest: inventory.digest().clone(),
        complete_equation_census_digest: complete_equation_census_digest.clone(),
        base_q0_rule_inventory: Arc::from(EMPTY_HISTORY_BASE_RULES),
        base_q0_rule_inventory_digest,
        rewrite_system_digest: rewrite_system_digest.clone(),
        digest: digest.clone(),
    };
    let outer_digest = Digest::of_canonical(
        "pen-semantic-audit/verified-historical-rewrite-system/v1",
        &HistoricalCapabilityMaterial {
            manifest: manifest.candidate_digest(),
            compatibility: compatibility.digest(),
            inventory: inventory.digest(),
            history: inventory.predecessor_history_digest(),
            boundary: inventory.predecessor_boundary().digest(),
            rewrite_inventory: &complete_equation_census_digest,
            rewrite_system: &rewrite_system_digest,
        },
    );
    AuditDecision::Proven(VerifiedHistoricalRewriteSystemV1 {
        history_digest: inventory.predecessor_history_digest().clone(),
        boundary_digest: inventory.predecessor_boundary().digest().clone(),
        inventory_digest: inventory.digest().clone(),
        rewrite_inventory_digest: complete_equation_census_digest,
        rewrite_system_digest,
        empty_base,
        digest: outer_digest,
    })
}

struct CanonicalRules<'a>(&'a [HistoricalBaseQ0RuleV1]);

impl CanonicalEncode for CanonicalRules<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct HistoricalEquationCensusMaterial<'a> {
    history: &'a Digest,
    boundary: &'a Digest,
    inventory: &'a Digest,
    equations: &'a [EquationIdV1],
}

impl CanonicalEncode for HistoricalEquationCensusMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.history.encode_canonical(encoder);
        self.boundary.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        encoder.sequence(self.equations);
        encoder.u64(self.equations.len() as u64);
    }
}

struct EmptyHistoricalRewriteMaterial<'a> {
    manifest: &'a Digest,
    compatibility: &'a Digest,
    history: &'a Digest,
    boundary: &'a Digest,
    equation_census: &'a Digest,
    base_rules: &'a Digest,
}

impl CanonicalEncode for EmptyHistoricalRewriteMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.compatibility.encode_canonical(encoder);
        self.history.encode_canonical(encoder);
        self.boundary.encode_canonical(encoder);
        self.equation_census.encode_canonical(encoder);
        self.base_rules.encode_canonical(encoder);
    }
}

struct HistoricalCapabilityMaterial<'a> {
    manifest: &'a Digest,
    compatibility: &'a Digest,
    inventory: &'a Digest,
    history: &'a Digest,
    boundary: &'a Digest,
    rewrite_inventory: &'a Digest,
    rewrite_system: &'a Digest,
}

impl CanonicalEncode for HistoricalCapabilityMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.compatibility.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        self.history.encode_canonical(encoder);
        self.boundary.encode_canonical(encoder);
        self.rewrite_inventory.encode_canonical(encoder);
        self.rewrite_system.encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        ORIGIN_CUTOFF_Q3_SCHEMA_VERSION, PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
        PublicDependencyUseV1, PublicSubjectV1, UncheckedOriginCutoffQ3RegistryV1,
        UncheckedPublicAuditInventoryV1, UncheckedPublicAvailabilityClaimV1,
        UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1,
        UncheckedPublicEventCensusV1, UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::inventory_compatibility::verify_public_inventory_compatibility_v2;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_lambda_unit_manifest_v2,
        verify_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v2,
    };
    use crate::model::{EventIdV1, GenericJudgmentV1, PublicAvailabilityV1};
    use pen_kernel::{
        Declaration, DependentContext, GlobalId, Kernel, KernelLimits, Term, UncheckedSignature,
    };

    struct Fixture {
        manifest: VerifiedSemanticAuditManifestV2,
        compatibility: VerifiedPublicInventoryCompatibilityV2,
        inventory: VerifiedPublicAuditInventoryV1,
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

    fn source_equation(judgment: &GenericJudgmentV1) -> crate::model::SourceNormalizedJudgmentV1 {
        crate::model::SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-judgment/v1",
                judgment,
            ),
            source: judgment.clone(),
            claimed_normalized: judgment.clone(),
        }
    }

    fn fixture(with_predecessor_equation: bool) -> Fixture {
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
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let predecessor_head = GlobalId(Digest::of_bytes(b"history/predecessor-head"));
        let successor_head = GlobalId(Digest::of_bytes(b"history/successor-head"));
        let predecessor_group = GlobalId(Digest::of_bytes(b"history/predecessor-group"));
        let successor_group = GlobalId(Digest::of_bytes(b"history/successor-group"));
        let predecessor_event = EventIdV1(Digest::of_bytes(b"history/predecessor-event"));
        let successor_event = EventIdV1(Digest::of_bytes(b"history/successor-event"));
        let predecessor_equation = EquationIdV1(Digest::of_bytes(b"history/equation"));
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
        let equation_judgment = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Unit,
            right: Term::Unit,
            ty: Term::UnitType,
        };

        let predecessor_boundary = if with_predecessor_equation {
            UncheckedSignature {
                declarations: vec![predecessor_declaration.clone()],
            }
        } else {
            UncheckedSignature::default()
        };
        let successor_boundary = if with_predecessor_equation {
            UncheckedSignature {
                declarations: vec![
                    predecessor_declaration.clone(),
                    successor_declaration.clone(),
                ],
            }
        } else {
            UncheckedSignature {
                declarations: vec![successor_declaration.clone()],
            }
        };
        let dependency = PublicDependencyUseV1 {
            dependent: PublicSubjectV1::Equation {
                equation: predecessor_equation.clone(),
            },
            prerequisite: predecessor_head.clone(),
        };
        let predecessor_history = if with_predecessor_equation {
            vec![UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group.clone()],
                    added_declarations: vec![predecessor_head.clone()],
                    added_equations: vec![predecessor_equation.clone()],
                    added_forced_projections: Vec::new(),
                    added_demand_contracts: Vec::new(),
                },
                successor_boundary: predecessor_boundary.clone(),
            }]
        } else {
            Vec::new()
        };
        let mut declaration_groups = Vec::new();
        let mut declarations = Vec::new();
        if with_predecessor_equation {
            declaration_groups.push(UncheckedPublicGroupV1 {
                group: predecessor_group.clone(),
                origin: predecessor_event.clone(),
                declarations: vec![predecessor_head.clone()],
            });
            declarations.push(UncheckedPublicDeclarationV1 {
                declaration: predecessor_head.clone(),
                origin: predecessor_event.clone(),
                group: predecessor_group,
                source_to_normal: source_declaration(&predecessor_declaration),
            });
        }
        declaration_groups.push(UncheckedPublicGroupV1 {
            group: successor_group.clone(),
            origin: successor_event.clone(),
            declarations: vec![successor_head.clone()],
        });
        declarations.push(UncheckedPublicDeclarationV1 {
            declaration: successor_head.clone(),
            origin: successor_event.clone(),
            group: successor_group.clone(),
            source_to_normal: source_declaration(&successor_declaration),
        });
        let equations = if with_predecessor_equation {
            vec![UncheckedPublicEquationV1 {
                equation: predecessor_equation,
                owner_head: predecessor_head.clone(),
                origin: predecessor_event.clone(),
                source_to_normal: source_equation(&equation_judgment),
                demand_port: None,
            }]
        } else {
            Vec::new()
        };
        let dependencies = if with_predecessor_equation {
            vec![dependency.clone()]
        } else {
            Vec::new()
        };
        let availability = if with_predecessor_equation {
            vec![UncheckedPublicAvailabilityClaimV1 {
                dependency,
                claimed: PublicAvailabilityV1::PredecessorPublicExport {
                    target: predecessor_head,
                },
            }]
        } else {
            Vec::new()
        };
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history,
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: successor_event,
                added_groups: vec![successor_group],
                added_declarations: vec![successor_head],
                added_equations: Vec::new(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups,
            declarations,
            equations,
            forced_projections: Vec::new(),
            predecessor_demand_contracts: Vec::new(),
            public_availability: availability,
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: dependencies,
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: with_predecessor_equation.then_some(predecessor_event),
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&v1_manifest, &kernel, &wire)
        else {
            panic!("inventory");
        };
        let AuditDecision::Proven(compatibility) =
            verify_public_inventory_compatibility_v2(&v1_manifest, &v2_manifest, &inventory)
        else {
            panic!("compatibility");
        };
        Fixture {
            manifest: v2_manifest,
            compatibility,
            inventory,
        }
    }

    #[test]
    fn equation_free_predecessor_mints_positive_empty_history() {
        let fixture = fixture(false);
        let AuditDecision::Proven(history) = verify_historical_rewrite_system_v1(
            &fixture.manifest,
            &fixture.compatibility,
            &fixture.inventory,
        ) else {
            panic!("empty history");
        };
        assert_eq!(
            history.history_digest(),
            fixture.inventory.predecessor_history_digest()
        );
        assert_eq!(history.empty_base().base_q0_rule_inventory().len(), 6);
        assert_eq!(
            history.rewrite_system_digest(),
            history.empty_base().rewrite_system_digest()
        );
    }

    #[test]
    fn nonempty_predecessor_requires_previously_issued_rewrite_theorem() {
        let fixture = fixture(true);
        assert!(matches!(
            verify_historical_rewrite_system_v1(
                &fixture.manifest,
                &fixture.compatibility,
                &fixture.inventory,
            ),
            AuditDecision::Unknown(AuditUnknownReason::MissingHistoricalRewriteAuthority)
        ));
    }
}
