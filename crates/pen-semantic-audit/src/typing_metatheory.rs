//! Safe-Agda foundation and fail-closed production frontier for the
//! lambda/unit V3 typing-and-synthesis metatheory.
//!
//! The fixed package proves a conversion-free declarative calculus with
//! abstract variables, relational synthesis soundness/completeness, dependent
//! simultaneous substitution, binder lifting, and typed beta/closed-delta/
//! fresh-schema stability. It does **not** itself prove that those codes
//! coincide with the production oldest-first finite de-Bruijn contexts,
//! kernel conversion/normalization, synthesis derivation codes, or exact
//! Q0/family inventories. Consequently this module's public verifiers mint
//! only [`VerifiedLambdaUnitTypingFoundationV1`]; the combined
//! [`VerifiedLambdaUnitTypingMetatheoryV1`] is minted exclusively through
//! the crate-private Phase H continuation constructor below, which
//! structurally requires all four production correspondence capabilities —
//! themselves constructible only by the single private correspondence
//! factory — so the factory remains the sole effective mint path.

use crate::agda_gate::{
    AgdaReferenceFailureV1, FixedAgdaSourceV1, VerifiedFixedAgdaPackageV1,
    diagnose_pinned_fixed_agda_package_v1,
};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3,
    VerifiedSemanticAuditManifestV3,
};
use crate::production_refinement_theorem::{
    VerifiedFiniteContextCorrespondenceV1, VerifiedKernelBaseConversionCorrespondenceV1,
    VerifiedSynthesisCodeCorrespondenceV1, VerifiedV3InventoryCorrespondenceV1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel};

const AGDA_ENTRY_RELATIVE_PATH: &str = "LawV2/LambdaUnit/LambdaUnitTypingMetatheoryV3.agda";

// This order is the pinned checker's exact depth-first reporting order with
// interfaces disabled. Every imported source appears once.
const AGDA_SOURCES: &[FixedAgdaSourceV1] = &[
    FixedAgdaSourceV1 {
        relative_path: AGDA_ENTRY_RELATIVE_PATH,
        module_name: "LawV2.LambdaUnit.LambdaUnitTypingMetatheoryV3",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/LambdaUnitTypingMetatheoryV3.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/TypingSyntax.agda",
        module_name: "LawV2.LambdaUnit.TypingSyntax",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/TypingSyntax.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/Substitution.agda",
        module_name: "LawV2.LambdaUnit.Substitution",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/Substitution.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/SubstitutionReduction.agda",
        module_name: "LawV2.LambdaUnit.SubstitutionReduction",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/SubstitutionReduction.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/TypingJudgment.agda",
        module_name: "LawV2.LambdaUnit.TypingJudgment",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/TypingJudgment.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/Synthesis.agda",
        module_name: "LawV2.LambdaUnit.Synthesis",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/Synthesis.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/SubstitutionTypingV3.agda",
        module_name: "LawV2.LambdaUnit.SubstitutionTypingV3",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/SubstitutionTypingV3.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/ReductionTyping.agda",
        module_name: "LawV2.LambdaUnit.ReductionTyping",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/ReductionTyping.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/FamilyNaturality.agda",
        module_name: "LawV2.LambdaUnit.FamilyNaturality",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/FamilyNaturality.agda"),
    },
];

const ABSTRACT_TYPING_RULE_INVENTORY: &[&str] = &[
    "sort",
    "unit-type",
    "unit",
    "variable",
    "closed-global",
    "pi",
    "lambda",
    "application",
];

const ABSTRACT_SUBSTITUTION_THEOREM_INVENTORY: &[&str] = &[
    "raw-identity",
    "raw-composition",
    "raw-associativity",
    "context-renaming",
    "weakening",
    "binder-lifting",
    "typed-identity",
    "typed-composition",
    "dependent-substitution",
];

const ABSTRACT_REDUCTION_THEOREM_INVENTORY: &[&str] = &[
    "typed-beta",
    "typed-closed-delta",
    "unit-formation-and-introduction-stability",
    "typed-fresh-equation-schema",
    "typed-step-substitution-stability",
];

/// Opaque evidence for exactly the theorem surface described by this module.
///
/// This is intentionally weaker than production typing authority. Fields are
/// private and the type has no `Deserialize` implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedLambdaUnitTypingFoundationV1 {
    abstract_typing_rule_inventory_digest: Digest,
    abstract_substitution_theorem_inventory_digest: Digest,
    abstract_reduction_theorem_inventory_digest: Digest,
    agda_source_tree_digest: Digest,
    agda_checker_transcript_digest: Digest,
    pinned_agda_package_digest: Digest,
    digest: Digest,
}

impl VerifiedLambdaUnitTypingFoundationV1 {
    pub fn abstract_typing_rule_inventory_digest(&self) -> &Digest {
        &self.abstract_typing_rule_inventory_digest
    }

    pub fn abstract_substitution_theorem_inventory_digest(&self) -> &Digest {
        &self.abstract_substitution_theorem_inventory_digest
    }

    pub fn abstract_reduction_theorem_inventory_digest(&self) -> &Digest {
        &self.abstract_reduction_theorem_inventory_digest
    }

    pub fn agda_source_tree_digest(&self) -> &Digest {
        &self.agda_source_tree_digest
    }

    pub fn agda_checker_transcript_digest(&self) -> &Digest {
        &self.agda_checker_transcript_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedLambdaUnitTypingFoundationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.abstract_typing_rule_inventory_digest
            .encode_canonical(encoder);
        self.abstract_substitution_theorem_inventory_digest
            .encode_canonical(encoder);
        self.abstract_reduction_theorem_inventory_digest
            .encode_canonical(encoder);
        self.agda_source_tree_digest.encode_canonical(encoder);
        self.agda_checker_transcript_digest
            .encode_canonical(encoder);
        self.pinned_agda_package_digest.encode_canonical(encoder);
    }
}

/// Requested combined production capability.
///
/// There is deliberately no constructor and no `Deserialize` implementation.
/// Its complete field surface records every identity that a future
/// correspondence theorem must bind before this type may be minted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedLambdaUnitTypingMetatheoryV1 {
    semantic_manifest_digest: Digest,
    kernel_protocol_digest: Digest,
    synthesis_protocol_digest: Digest,
    typing_rule_inventory_digest: Digest,
    substitution_rule_inventory_digest: Digest,
    q0_schema_inventory_digest: Digest,
    family_constructor_inventory_digest: Digest,
    synthesis_soundness_digest: Digest,
    synthesis_completeness_digest: Digest,
    substitution_typing_digest: Digest,
    reduction_stability_digest: Digest,
    agda_source_tree_digest: Digest,
    agda_checker_transcript_digest: Digest,
    pinned_agda_package_digest: Digest,
    digest: Digest,
}

impl VerifiedLambdaUnitTypingMetatheoryV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn synthesis_protocol_digest(&self) -> &Digest {
        &self.synthesis_protocol_digest
    }

    pub fn typing_rule_inventory_digest(&self) -> &Digest {
        &self.typing_rule_inventory_digest
    }

    pub fn substitution_rule_inventory_digest(&self) -> &Digest {
        &self.substitution_rule_inventory_digest
    }

    pub fn q0_schema_inventory_digest(&self) -> &Digest {
        &self.q0_schema_inventory_digest
    }

    pub fn family_constructor_inventory_digest(&self) -> &Digest {
        &self.family_constructor_inventory_digest
    }

    pub fn synthesis_soundness_digest(&self) -> &Digest {
        &self.synthesis_soundness_digest
    }

    pub fn synthesis_completeness_digest(&self) -> &Digest {
        &self.synthesis_completeness_digest
    }

    pub fn substitution_typing_digest(&self) -> &Digest {
        &self.substitution_typing_digest
    }

    pub fn reduction_stability_digest(&self) -> &Digest {
        &self.reduction_stability_digest
    }

    pub fn agda_source_tree_digest(&self) -> &Digest {
        &self.agda_source_tree_digest
    }

    pub fn agda_checker_transcript_digest(&self) -> &Digest {
        &self.agda_checker_transcript_digest
    }

    pub fn pinned_agda_package_digest(&self) -> &Digest {
        &self.pinned_agda_package_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedLambdaUnitTypingMetatheoryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.synthesis_protocol_digest.encode_canonical(encoder);
        self.typing_rule_inventory_digest.encode_canonical(encoder);
        self.substitution_rule_inventory_digest
            .encode_canonical(encoder);
        self.q0_schema_inventory_digest.encode_canonical(encoder);
        self.family_constructor_inventory_digest
            .encode_canonical(encoder);
        self.synthesis_soundness_digest.encode_canonical(encoder);
        self.synthesis_completeness_digest.encode_canonical(encoder);
        self.substitution_typing_digest.encode_canonical(encoder);
        self.reduction_stability_digest.encode_canonical(encoder);
        self.agda_source_tree_digest.encode_canonical(encoder);
        self.agda_checker_transcript_digest
            .encode_canonical(encoder);
        self.pinned_agda_package_digest.encode_canonical(encoder);
    }
}

/// Exact production correspondence obligations not discharged by the abstract
/// safe-Agda package.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypingProductionCorrespondenceObligationV1 {
    OldestFirstFiniteDeBruijnContexts,
    KernelConversionAndNormalization,
    RustSynthesisTermAndDerivationCodes,
    TypedQ0AndFamilyInventories,
}

pub const TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1:
    &[TypingProductionCorrespondenceObligationV1] = &[
    TypingProductionCorrespondenceObligationV1::OldestFirstFiniteDeBruijnContexts,
    TypingProductionCorrespondenceObligationV1::KernelConversionAndNormalization,
    TypingProductionCorrespondenceObligationV1::RustSynthesisTermAndDerivationCodes,
    TypingProductionCorrespondenceObligationV1::TypedQ0AndFamilyInventories,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LambdaUnitTypingMetatheoryFailureV1 {
    ManifestMismatch,
    AgdaGate(AgdaReferenceFailureV1),
    MissingProductionCorrespondence {
        obligations: &'static [TypingProductionCorrespondenceObligationV1],
    },
}

impl std::fmt::Display for LambdaUnitTypingMetatheoryFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManifestMismatch => formatter
                .write_str("semantic manifest is not the exact unfrozen lambda/unit V3 successor"),
            Self::AgdaGate(failure) => write!(formatter, "pinned Agda gate failed: {failure}"),
            Self::MissingProductionCorrespondence { .. } => formatter.write_str(
                "the abstract typing proofs are not yet related to oldest-first finite \
                 de-Bruijn contexts, kernel conversion/normalization, Rust synthesis codes, \
                 and the exact typed Q0/family inventories",
            ),
        }
    }
}

/// Check the exact fixed abstract theorem package and mint only its weaker
/// foundation capability.
pub fn verify_pinned_lambda_unit_typing_foundation_v1()
-> AuditDecision<VerifiedLambdaUnitTypingFoundationV1> {
    match diagnose_pinned_fixed_agda_package_v1(AGDA_SOURCES, AGDA_ENTRY_RELATIVE_PATH) {
        Ok(package) => AuditDecision::Proven(foundation_from_package(package)),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::UnsupportedVerifier),
    }
}

/// Diagnose the full production capability frontier.
///
/// The synthesis protocol digest is obtained directly from the isolated
/// successor crate. Merely computing and binding all production inventory
/// digests is not a correspondence proof, so the function still fails closed
/// after the exact Agda package succeeds.
pub fn diagnose_lambda_unit_typing_metatheory_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
) -> Result<VerifiedLambdaUnitTypingMetatheoryV1, LambdaUnitTypingMetatheoryFailureV1> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3
        || manifest.manifest().frozen
        || manifest.manifest().live_profile_a_access
    {
        return Err(LambdaUnitTypingMetatheoryFailureV1::ManifestMismatch);
    }

    let package = diagnose_pinned_fixed_agda_package_v1(AGDA_SOURCES, AGDA_ENTRY_RELATIVE_PATH)
        .map_err(LambdaUnitTypingMetatheoryFailureV1::AgdaGate)?;

    // Bind every requested production identity before reporting the missing
    // correspondence. None of these values is converted into capability
    // authority.
    let _exact_production_bindings = (
        manifest.candidate_digest(),
        kernel.kernel_protocol_digest(),
        pen_kernel_synthesis::synthesis_protocol_digest_v1(),
        fixed_inventory_digest(
            "pen-semantic-audit/lambda-unit-typing-rule-inventory/v1",
            ABSTRACT_TYPING_RULE_INVENTORY,
        ),
        fixed_inventory_digest(
            "pen-semantic-audit/lambda-unit-substitution-rule-inventory/v1",
            ABSTRACT_SUBSTITUTION_THEOREM_INVENTORY,
        ),
        Digest::of_canonical(
            "pen-semantic-audit/lambda-unit-q0-schema-inventory/v3",
            &CanonicalSlice(&manifest.manifest().q0_rules),
        ),
        Digest::of_canonical(
            "pen-semantic-audit/lambda-unit-family-constructor-inventory/v3",
            &CanonicalSlice(&manifest.manifest().ordered_derivation_rules),
        ),
        package.source_tree_digest(),
        package.checker_stdout_digest(),
        package.digest(),
    );

    Err(
        LambdaUnitTypingMetatheoryFailureV1::MissingProductionCorrespondence {
            obligations: TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1,
        },
    )
}

pub fn verify_lambda_unit_typing_metatheory_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
) -> AuditDecision<VerifiedLambdaUnitTypingMetatheoryV1> {
    match diagnose_lambda_unit_typing_metatheory_v1(manifest, kernel) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(LambdaUnitTypingMetatheoryFailureV1::ManifestMismatch) => {
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        }
        Err(LambdaUnitTypingMetatheoryFailureV1::AgdaGate(_)) => {
            AuditDecision::Unknown(AuditUnknownReason::UnsupportedVerifier)
        }
        Err(LambdaUnitTypingMetatheoryFailureV1::MissingProductionCorrespondence { .. }) => {
            AuditDecision::Unknown(AuditUnknownReason::MissingLambdaUnitTypingMetatheory)
        }
    }
}

/// Phase H continuation constructor: mint the combined typing
/// metatheory from the four production correspondence capabilities.
///
/// This is deliberately `pub(crate)` and consumes the four
/// correspondence capabilities, which have no constructor outside the
/// single private correspondence factory
/// (`production_refinement_theorem::correspondence_factory`). A caller
/// therefore cannot reach this function without the factory having
/// succeeded over one canonical bundle; the factory is the only caller.
/// Each of the four typing-production obligations in
/// [`TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1`] is discharged by the
/// corresponding capability:
///
/// - `OldestFirstFiniteDeBruijnContexts` by the finite-context
///   correspondence;
/// - `KernelConversionAndNormalization` by the base-conversion
///   correspondence;
/// - `RustSynthesisTermAndDerivationCodes` by the synthesis-code
///   correspondence; and
/// - `TypedQ0AndFamilyInventories` by the V3 inventory correspondence.
///
/// The four theorem digests bind the factory's evidence-core digest and
/// the discharging capability digests; they are recorded only after the
/// factory's actual byte comparisons succeeded. The synthesis protocol
/// digest is the protocol V2 implementation digest — the protocol the
/// correspondence evidence actually replays — not the historical V1
/// digest the pre-H diagnostic bound and discarded.
#[allow(clippy::too_many_arguments)]
pub(crate) fn mint_lambda_unit_typing_metatheory_from_production_correspondences_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    foundation: &VerifiedLambdaUnitTypingFoundationV1,
    context: &VerifiedFiniteContextCorrespondenceV1,
    conversion: &VerifiedKernelBaseConversionCorrespondenceV1,
    synthesis: &VerifiedSynthesisCodeCorrespondenceV1,
    inventory: &VerifiedV3InventoryCorrespondenceV1,
    synthesis_protocol_digest: &Digest,
    correspondence_evidence_digest: &Digest,
) -> VerifiedLambdaUnitTypingMetatheoryV1 {
    let theorem_digest = |domain: &str, discharging: &[&Digest]| {
        let mut encoder = CanonicalEncoder::new();
        encoder.u16(1);
        correspondence_evidence_digest.encode_canonical(&mut encoder);
        encoder.u64(discharging.len() as u64);
        for digest in discharging {
            digest.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes(domain, encoder.as_bytes())
    };
    let mut verified = VerifiedLambdaUnitTypingMetatheoryV1 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        synthesis_protocol_digest: synthesis_protocol_digest.clone(),
        typing_rule_inventory_digest: fixed_inventory_digest(
            "pen-semantic-audit/lambda-unit-typing-rule-inventory/v1",
            ABSTRACT_TYPING_RULE_INVENTORY,
        ),
        substitution_rule_inventory_digest: fixed_inventory_digest(
            "pen-semantic-audit/lambda-unit-substitution-rule-inventory/v1",
            ABSTRACT_SUBSTITUTION_THEOREM_INVENTORY,
        ),
        q0_schema_inventory_digest: Digest::of_canonical(
            "pen-semantic-audit/lambda-unit-q0-schema-inventory/v3",
            &CanonicalSlice(&manifest.manifest().q0_rules),
        ),
        family_constructor_inventory_digest: Digest::of_canonical(
            "pen-semantic-audit/lambda-unit-family-constructor-inventory/v3",
            &CanonicalSlice(&manifest.manifest().ordered_derivation_rules),
        ),
        synthesis_soundness_digest: theorem_digest(
            "pen-semantic-audit/lambda-unit-metatheory/synthesis-soundness/v1",
            &[synthesis.digest(), context.digest()],
        ),
        synthesis_completeness_digest: theorem_digest(
            "pen-semantic-audit/lambda-unit-metatheory/synthesis-completeness/v1",
            &[synthesis.digest()],
        ),
        substitution_typing_digest: theorem_digest(
            "pen-semantic-audit/lambda-unit-metatheory/substitution-typing/v1",
            &[context.digest()],
        ),
        reduction_stability_digest: theorem_digest(
            "pen-semantic-audit/lambda-unit-metatheory/reduction-stability/v1",
            &[conversion.digest(), inventory.digest()],
        ),
        agda_source_tree_digest: foundation.agda_source_tree_digest.clone(),
        agda_checker_transcript_digest: foundation.agda_checker_transcript_digest.clone(),
        pinned_agda_package_digest: foundation.pinned_agda_package_digest.clone(),
        digest: Digest::of_bytes(b"pending lambda/unit typing metatheory"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-lambda-unit-typing-metatheory/v1",
        &verified,
    );
    verified
}

fn foundation_from_package(
    package: VerifiedFixedAgdaPackageV1,
) -> VerifiedLambdaUnitTypingFoundationV1 {
    let abstract_typing_rule_inventory_digest = fixed_inventory_digest(
        "pen-semantic-audit/abstract-typing-rule-inventory/v1",
        ABSTRACT_TYPING_RULE_INVENTORY,
    );
    let abstract_substitution_theorem_inventory_digest = fixed_inventory_digest(
        "pen-semantic-audit/abstract-substitution-theorem-inventory/v1",
        ABSTRACT_SUBSTITUTION_THEOREM_INVENTORY,
    );
    let abstract_reduction_theorem_inventory_digest = fixed_inventory_digest(
        "pen-semantic-audit/abstract-reduction-theorem-inventory/v1",
        ABSTRACT_REDUCTION_THEOREM_INVENTORY,
    );
    let mut verified = VerifiedLambdaUnitTypingFoundationV1 {
        abstract_typing_rule_inventory_digest,
        abstract_substitution_theorem_inventory_digest,
        abstract_reduction_theorem_inventory_digest,
        agda_source_tree_digest: package.source_tree_digest().clone(),
        agda_checker_transcript_digest: package.checker_stdout_digest().clone(),
        pinned_agda_package_digest: package.digest().clone(),
        digest: Digest::of_bytes(b"pending lambda/unit typing foundation"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-lambda-unit-typing-foundation/v1",
        &verified,
    );
    verified
}

fn fixed_inventory_digest(domain: &str, inventory: &[&str]) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(1);
    encoder.u64(inventory.len() as u64);
    for item in inventory {
        encoder.text(item);
    }
    Digest::of_domain_bytes(domain, encoder.as_bytes())
}

struct CanonicalSlice<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSlice<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AGDA_SOURCES, LambdaUnitTypingMetatheoryFailureV1,
        TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1, TypingProductionCorrespondenceObligationV1,
        diagnose_lambda_unit_typing_metatheory_v1, verify_pinned_lambda_unit_typing_foundation_v1,
    };
    use crate::manifest::{
        AuditDecision, proposed_semantic_audit_lambda_unit_manifest_v3,
        verify_semantic_audit_lambda_unit_manifest_v3,
    };
    use pen_kernel::{Kernel, KernelLimits};

    #[test]
    fn fixed_sources_are_safe_and_contain_no_axiom_escape_hatches() {
        for source in AGDA_SOURCES {
            let text = std::str::from_utf8(source.bytes).expect("fixed source must be UTF-8");
            assert!(text.contains("{-# OPTIONS --safe --without-K #-}"));
            for forbidden in [
                "postulate",
                "{-# TERMINATING #-}",
                "{-# NO_POSITIVITY_CHECK #-}",
            ] {
                assert!(
                    !text.contains(forbidden),
                    "{} contains forbidden text {forbidden}",
                    source.relative_path
                );
            }
        }
    }

    #[test]
    fn production_frontier_names_every_unproved_correspondence() {
        assert_eq!(
            TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1,
            &[
                TypingProductionCorrespondenceObligationV1::OldestFirstFiniteDeBruijnContexts,
                TypingProductionCorrespondenceObligationV1::KernelConversionAndNormalization,
                TypingProductionCorrespondenceObligationV1::RustSynthesisTermAndDerivationCodes,
                TypingProductionCorrespondenceObligationV1::TypedQ0AndFamilyInventories,
            ]
        );
    }

    #[test]
    #[ignore = "runs the pinned external Agda checker"]
    fn pinned_typing_foundation_checks_live() {
        assert!(matches!(
            verify_pinned_lambda_unit_typing_foundation_v1(),
            AuditDecision::Proven(_)
        ));
    }

    #[test]
    #[ignore = "runs the pinned external Agda checker"]
    fn combined_capability_remains_fail_closed_after_foundation_checks() {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v3(
            &proposed_semantic_audit_lambda_unit_manifest_v3(),
        ) else {
            panic!("exact V3 manifest should verify");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("safe kernel");
        let Err(LambdaUnitTypingMetatheoryFailureV1::MissingProductionCorrespondence {
            obligations,
        }) = diagnose_lambda_unit_typing_metatheory_v1(&manifest, &kernel)
        else {
            panic!("combined capability must stop at production correspondence");
        };
        assert_eq!(obligations, TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1);
    }
}
