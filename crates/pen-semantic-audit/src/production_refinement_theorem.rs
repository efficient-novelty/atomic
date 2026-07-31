//! Fail-closed composition frontier for
//! `law-v2-lambda-unit-production-refinement-v1`.
//!
//! This module binds the exact V3 candidate, verified production signature,
//! declaration-order global slots, the advertised synthesis protocol V2
//! identity, the exact Rust inventory bridge, and two pinned safe-Agda
//! foundation packages. None of those facts proves Rust/Agda correspondence.
//! The four correspondence capabilities therefore have no deserialization
//! path, and nothing in this module mints them: their private fields are
//! reachable only from the child module
//! [`correspondence_factory`], the single private Phase H factory, which
//! mints them exclusively from the four bridge capabilities (canonical
//! bundle, safe-Agda acceptance, independent Rust replay, and exact
//! transcript agreement). The capability-free diagnostics below continue to
//! stop at [`LambdaUnitProductionRefinementFailureV1::MissingProductionCorrespondences`]:
//! their frontier list is call-relative — a caller that presents no bridge
//! capabilities still has every correspondence unresolved.
//!
//! Generic certificate-checker soundness is deliberately separate from
//! carrier-derived finite subject coverage. This module neither defines nor
//! issues native-carrier authority.

/// The single private Phase H correspondence factory. Mounted as a child
/// module so the Rust module system itself guarantees no second
/// construction site for the correspondence capabilities exists.
#[path = "production_correspondence_factory.rs"]
pub mod correspondence_factory;

use crate::agda_gate::{
    AgdaReferenceFailureV1, FixedAgdaSourceV1, VerifiedFixedAgdaPackageV1,
    diagnose_pinned_fixed_agda_package_v1,
};
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::inventory_compatibility::VerifiedPublicInventoryCompatibilityV2;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedSemanticAuditManifestV2,
    VerifiedSemanticAuditManifestV3, proposed_semantic_audit_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v3, verify_semantic_audit_lambda_unit_manifest_v2,
    verify_semantic_audit_lambda_unit_manifest_v3,
};
use crate::production_inventory_bridge::{
    ProductionInventoryBridgeFailureV1, VerifiedProductionInventoryBridgeV1,
    diagnose_production_inventory_bridge_v1,
};
use crate::production_refinement::{
    ProductionRefinementFailureV1, VerifiedGlobalSlotTableV1,
    VerifiedLambdaUnitPublicSortSuccessorsV1, diagnose_global_slot_table_v1,
    verify_lambda_unit_public_sort_successors_v1,
};
use crate::typing_metatheory::VerifiedLambdaUnitTypingFoundationV1;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel, VerifiedSignature};
use pen_kernel_synthesis::{
    CHECKER_OUTPUT_UNIVERSE_LEVELS_V2, DeltaPolicyEntryV2, NESTED_CONGRUENCE_REPLAY_FRONTIER_V2,
    NestedCongruenceReplayFrontierV2, PUBLIC_UNIVERSE_LEVELS_V2, SYNTHESIS_CODE_INVENTORY_V2,
    SYNTHESIS_PROTOCOL_ID_V2, SYNTHESIS_SCHEMA_VERSION_V2, SynthesisCodeTagV2, SynthesisV2Error,
    TRANSPARENT_DELTA_AUTHORITY_FRONTIER_V2, TransparentDeltaAuthorityFrontierV2,
    VerifiedBaseQ0ConversionPolicyV2, synthesis_code_inventory_digest_v2,
    synthesis_protocol_digest_v2, verify_base_q0_conversion_policy_v2,
};
use std::sync::Arc;

pub const LAMBDA_UNIT_PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1: u16 = 1;

const EXACT_SYNTHESIS_CODE_INVENTORY_V2: [SynthesisCodeTagV2; 8] = [
    SynthesisCodeTagV2::Sort,
    SynthesisCodeTagV2::UnitType,
    SynthesisCodeTagV2::Unit,
    SynthesisCodeTagV2::VariableLookup,
    SynthesisCodeTagV2::GlobalLookup,
    SynthesisCodeTagV2::PiFormation,
    SynthesisCodeTagV2::LambdaIntroduction,
    SynthesisCodeTagV2::ApplicationElimination,
];

const PRODUCTION_AGDA_ENTRY_RELATIVE_PATH: &str =
    "LawV2/LambdaUnit/LambdaUnitProductionRefinementV1.agda";

// Exact depth-first checker reporting order, observed with `--ignore-interfaces`.
const PRODUCTION_AGDA_SOURCES: &[FixedAgdaSourceV1] = &[
    FixedAgdaSourceV1 {
        relative_path: PRODUCTION_AGDA_ENTRY_RELATIVE_PATH,
        module_name: "LawV2.LambdaUnit.LambdaUnitProductionRefinementV1",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/LambdaUnitProductionRefinementV1.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/ProductionSyntaxV1.agda",
        module_name: "LawV2.LambdaUnit.ProductionSyntaxV1",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/ProductionSyntaxV1.agda"),
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
        relative_path: "LawV2/LambdaUnit/ProductionDecodingV1.agda",
        module_name: "LawV2.LambdaUnit.ProductionDecodingV1",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/ProductionDecodingV1.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/ProductionConversionTyping.agda",
        module_name: "LawV2.LambdaUnit.ProductionConversionTyping",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/ProductionConversionTyping.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/TypingJudgment.agda",
        module_name: "LawV2.LambdaUnit.TypingJudgment",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/TypingJudgment.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/ReductionTyping.agda",
        module_name: "LawV2.LambdaUnit.ReductionTyping",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/ReductionTyping.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/SubstitutionTypingV3.agda",
        module_name: "LawV2.LambdaUnit.SubstitutionTypingV3",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/SubstitutionTypingV3.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/ProductionSynthesisCodeV2.agda",
        module_name: "LawV2.LambdaUnit.ProductionSynthesisCodeV2",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/ProductionSynthesisCodeV2.agda"),
    },
];

const INVENTORY_AGDA_ENTRY_RELATIVE_PATH: &str =
    "LawV2/LambdaUnit/ProductionInventoryBridgeV1.agda";

// The inventory bridge is a separate safe-Agda entry and is not silently
// treated as an import of the production aggregate.
const INVENTORY_AGDA_SOURCES: &[FixedAgdaSourceV1] = &[
    FixedAgdaSourceV1 {
        relative_path: INVENTORY_AGDA_ENTRY_RELATIVE_PATH,
        module_name: "LawV2.LambdaUnit.ProductionInventoryBridgeV1",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/ProductionInventoryBridgeV1.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/FamilyNaturality.agda",
        module_name: "LawV2.LambdaUnit.FamilyNaturality",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/FamilyNaturality.agda"),
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
];

/// Exact safe-Agda source/checker identity for the currently proved production
/// foundations.
///
/// This capability proves only that the fixed packages type check under the
/// pinned checker and primitive tree. It is weaker than every Rust/Agda
/// correspondence capability below.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedProductionRefinementAgdaFoundationV1 {
    production_source_tree_digest: Digest,
    production_checker_transcript_digest: Digest,
    production_package_digest: Digest,
    inventory_source_tree_digest: Digest,
    inventory_checker_transcript_digest: Digest,
    inventory_package_digest: Digest,
    digest: Digest,
}

impl VerifiedProductionRefinementAgdaFoundationV1 {
    pub fn production_source_tree_digest(&self) -> &Digest {
        &self.production_source_tree_digest
    }

    pub fn production_checker_transcript_digest(&self) -> &Digest {
        &self.production_checker_transcript_digest
    }

    pub fn inventory_source_tree_digest(&self) -> &Digest {
        &self.inventory_source_tree_digest
    }

    pub fn inventory_checker_transcript_digest(&self) -> &Digest {
        &self.inventory_checker_transcript_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedProductionRefinementAgdaFoundationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.production_source_tree_digest.encode_canonical(encoder);
        self.production_checker_transcript_digest
            .encode_canonical(encoder);
        self.production_package_digest.encode_canonical(encoder);
        self.inventory_source_tree_digest.encode_canonical(encoder);
        self.inventory_checker_transcript_digest
            .encode_canonical(encoder);
        self.inventory_package_digest.encode_canonical(encoder);
    }
}

pub fn diagnose_pinned_production_refinement_agda_foundation_v1()
-> Result<VerifiedProductionRefinementAgdaFoundationV1, AgdaReferenceFailureV1> {
    let production = diagnose_pinned_fixed_agda_package_v1(
        PRODUCTION_AGDA_SOURCES,
        PRODUCTION_AGDA_ENTRY_RELATIVE_PATH,
    )?;
    let inventory = diagnose_pinned_fixed_agda_package_v1(
        INVENTORY_AGDA_SOURCES,
        INVENTORY_AGDA_ENTRY_RELATIVE_PATH,
    )?;
    Ok(production_agda_foundation_from_packages(
        production, inventory,
    ))
}

pub fn verify_pinned_production_refinement_agda_foundation_v1()
-> AuditDecision<VerifiedProductionRefinementAgdaFoundationV1> {
    match diagnose_pinned_production_refinement_agda_foundation_v1() {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::UnsupportedVerifier),
    }
}

fn production_agda_foundation_from_packages(
    production: VerifiedFixedAgdaPackageV1,
    inventory: VerifiedFixedAgdaPackageV1,
) -> VerifiedProductionRefinementAgdaFoundationV1 {
    let mut verified = VerifiedProductionRefinementAgdaFoundationV1 {
        production_source_tree_digest: production.source_tree_digest().clone(),
        production_checker_transcript_digest: production.checker_stdout_digest().clone(),
        production_package_digest: production.digest().clone(),
        inventory_source_tree_digest: inventory.source_tree_digest().clone(),
        inventory_checker_transcript_digest: inventory.checker_stdout_digest().clone(),
        inventory_package_digest: inventory.digest().clone(),
        digest: Digest::of_bytes(b"pending production-refinement Agda foundation"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-production-refinement-agda-foundation/v1",
        &verified,
    );
    verified
}

/// Positive identity for the advertised V2 synthesis protocol surface.
///
/// This binds protocol/source and eight-tag inventory digests. It is not a
/// soundness theorem for the checker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedProductionSynthesisProtocolIdentityV2 {
    protocol_id: &'static str,
    schema_version: u16,
    synthesis_code_constructor_count: u64,
    public_universe_levels: Arc<[u16]>,
    checker_output_universe_levels: Arc<[u16]>,
    synthesis_code_inventory_digest: Digest,
    implementation_protocol_digest: Digest,
    digest: Digest,
}

impl VerifiedProductionSynthesisProtocolIdentityV2 {
    pub fn protocol_id(&self) -> &str {
        self.protocol_id
    }

    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn synthesis_code_constructor_count(&self) -> u64 {
        self.synthesis_code_constructor_count
    }

    pub fn public_universe_levels(&self) -> &[u16] {
        &self.public_universe_levels
    }

    pub fn checker_output_universe_levels(&self) -> &[u16] {
        &self.checker_output_universe_levels
    }

    pub fn synthesis_code_inventory_digest(&self) -> &Digest {
        &self.synthesis_code_inventory_digest
    }

    pub fn implementation_protocol_digest(&self) -> &Digest {
        &self.implementation_protocol_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedProductionSynthesisProtocolIdentityV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(self.protocol_id);
        encoder.u16(self.schema_version);
        encoder.u64(self.synthesis_code_constructor_count);
        encode_u16_sequence(encoder, &self.public_universe_levels);
        encode_u16_sequence(encoder, &self.checker_output_universe_levels);
        self.synthesis_code_inventory_digest
            .encode_canonical(encoder);
        self.implementation_protocol_digest
            .encode_canonical(encoder);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionSynthesisProtocolIdentityFailureV1 {
    AdvertisedSurfaceMismatch,
    SynthesisCodeInventoryMismatch,
    UnstableImplementationIdentity,
}

pub fn diagnose_production_synthesis_protocol_identity_v2() -> Result<
    VerifiedProductionSynthesisProtocolIdentityV2,
    ProductionSynthesisProtocolIdentityFailureV1,
> {
    if SYNTHESIS_PROTOCOL_ID_V2 != "pen-kernel-synthesis/lambda-unit/v2"
        || SYNTHESIS_SCHEMA_VERSION_V2 != 2
        || PUBLIC_UNIVERSE_LEVELS_V2 != [0, 1]
        || CHECKER_OUTPUT_UNIVERSE_LEVELS_V2 != [0, 1, 2]
        || SYNTHESIS_CODE_INVENTORY_V2 != EXACT_SYNTHESIS_CODE_INVENTORY_V2
    {
        return Err(ProductionSynthesisProtocolIdentityFailureV1::AdvertisedSurfaceMismatch);
    }

    let synthesis_code_inventory_digest = synthesis_code_inventory_digest_v2();
    if synthesis_code_inventory_digest != exact_synthesis_code_inventory_digest_v2() {
        return Err(ProductionSynthesisProtocolIdentityFailureV1::SynthesisCodeInventoryMismatch);
    }
    let implementation_protocol_digest = synthesis_protocol_digest_v2();
    if implementation_protocol_digest != synthesis_protocol_digest_v2() {
        return Err(ProductionSynthesisProtocolIdentityFailureV1::UnstableImplementationIdentity);
    }

    let mut verified = VerifiedProductionSynthesisProtocolIdentityV2 {
        protocol_id: SYNTHESIS_PROTOCOL_ID_V2,
        schema_version: SYNTHESIS_SCHEMA_VERSION_V2,
        synthesis_code_constructor_count: 8,
        public_universe_levels: Arc::from(PUBLIC_UNIVERSE_LEVELS_V2),
        checker_output_universe_levels: Arc::from(CHECKER_OUTPUT_UNIVERSE_LEVELS_V2),
        synthesis_code_inventory_digest,
        implementation_protocol_digest,
        digest: Digest::of_bytes(b"pending production synthesis V2 identity"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-production-synthesis-protocol-identity/v2",
        &verified,
    );
    Ok(verified)
}

fn exact_synthesis_code_inventory_digest_v2() -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.sequence(&EXACT_SYNTHESIS_CODE_INVENTORY_V2);
    Digest::of_domain_bytes(
        "lambda-unit-synthesis-code-inventory-v2",
        encoder.as_bytes(),
    )
}

/// Exact semantic binding of a V2 transparent-delta policy to all and only
/// bodyful declarations in one verified predecessor-public boundary.
///
/// The predecessor boundary comes exclusively from
/// [`VerifiedPublicAuditInventoryV1`]. Bodyful declarations found merely by
/// scanning arbitrary successor slots do not acquire predecessor-public
/// status. This remains authority relative to the generic verified ledger
/// snapshot; it does not authenticate that snapshot as issued Profile A
/// history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPredecessorPublicDeltaPolicyBindingV1 {
    public_inventory_digest: Digest,
    public_inventory_compatibility_digest: Digest,
    lambda_unit_v2_manifest_digest: Digest,
    predecessor_history_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_signature_digest: Digest,
    global_slot_table_digest: Digest,
    policy_digest: Digest,
    exact_ordered_entries: Arc<[DeltaPolicyEntryV2]>,
    all_and_only_bodyful_predecessor_digest: Digest,
    digest: Digest,
}

impl VerifiedPredecessorPublicDeltaPolicyBindingV1 {
    pub fn public_inventory_digest(&self) -> &Digest {
        &self.public_inventory_digest
    }

    pub fn public_inventory_compatibility_digest(&self) -> &Digest {
        &self.public_inventory_compatibility_digest
    }

    pub fn lambda_unit_v2_manifest_digest(&self) -> &Digest {
        &self.lambda_unit_v2_manifest_digest
    }

    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_signature_digest(&self) -> &Digest {
        &self.successor_signature_digest
    }

    pub fn global_slot_table_digest(&self) -> &Digest {
        &self.global_slot_table_digest
    }

    pub fn policy_digest(&self) -> &Digest {
        &self.policy_digest
    }

    pub fn exact_ordered_entries(&self) -> &[DeltaPolicyEntryV2] {
        &self.exact_ordered_entries
    }

    pub fn all_and_only_bodyful_predecessor_digest(&self) -> &Digest {
        &self.all_and_only_bodyful_predecessor_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedPredecessorPublicDeltaPolicyBindingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.public_inventory_digest.encode_canonical(encoder);
        self.public_inventory_compatibility_digest
            .encode_canonical(encoder);
        self.lambda_unit_v2_manifest_digest
            .encode_canonical(encoder);
        self.predecessor_history_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_signature_digest.encode_canonical(encoder);
        self.global_slot_table_digest.encode_canonical(encoder);
        self.policy_digest.encode_canonical(encoder);
        encoder.sequence(&self.exact_ordered_entries);
        self.all_and_only_bodyful_predecessor_digest
            .encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PredecessorPublicDeltaPolicyBindingFailureV1 {
    InventoryCompatibilityMismatch,
    InventoryBoundaryMismatch,
    GlobalSlotTableMismatch,
    MissingPredecessorGlobal,
    GlobalSlotOverflow,
    GlobalSlotDeclarationMismatch,
    PolicySignatureMismatch,
    PolicyReplay(SynthesisV2Error),
    PolicyReplayMismatch,
    PolicyInventoryMismatch,
}

impl std::fmt::Display for PredecessorPublicDeltaPolicyBindingFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InventoryCompatibilityMismatch => formatter.write_str(
                "the public inventory lacks the exact lambda/unit V1-to-V2 compatibility binding",
            ),
            Self::InventoryBoundaryMismatch => formatter.write_str(
                "the supplied successor signature is not the verified inventory successor boundary",
            ),
            Self::GlobalSlotTableMismatch => formatter.write_str(
                "the global slot table is not bound to the supplied successor signature",
            ),
            Self::MissingPredecessorGlobal => formatter
                .write_str("a predecessor-public bodyful declaration has no successor global slot"),
            Self::GlobalSlotOverflow => formatter
                .write_str("a predecessor-public global slot does not fit the V2 policy wire"),
            Self::GlobalSlotDeclarationMismatch => formatter.write_str(
                "a mapped successor slot does not replay the exact predecessor declaration",
            ),
            Self::PolicySignatureMismatch => formatter
                .write_str("the verified V2 delta policy is not bound to the successor signature"),
            Self::PolicyReplay(error) => {
                write!(formatter, "the V2 delta policy did not replay: {error}")
            }
            Self::PolicyReplayMismatch => {
                formatter.write_str("replaying the V2 delta policy changed its verified identity")
            }
            Self::PolicyInventoryMismatch => formatter.write_str(
                "the V2 delta policy is not all and only bodyful predecessor-public declarations",
            ),
        }
    }
}

impl std::error::Error for PredecessorPublicDeltaPolicyBindingFailureV1 {}

pub fn diagnose_predecessor_public_delta_policy_binding_v1(
    inventory: &VerifiedPublicAuditInventoryV1,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    successor_signature: &VerifiedSignature,
    global_slots: &VerifiedGlobalSlotTableV1,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
) -> Result<
    VerifiedPredecessorPublicDeltaPolicyBindingV1,
    PredecessorPublicDeltaPolicyBindingFailureV1,
> {
    if compatibility.inventory_digest() != inventory.digest()
        || compatibility.predecessor_history_digest() != inventory.predecessor_history_digest()
        || compatibility.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || compatibility.successor_boundary_digest() != inventory.successor_boundary().digest()
        || compatibility.exact_extension_digest() != inventory.exact_extension().digest()
    {
        return Err(PredecessorPublicDeltaPolicyBindingFailureV1::InventoryCompatibilityMismatch);
    }
    if inventory.successor_boundary().digest() != successor_signature.digest()
        || inventory.exact_extension().successor_boundary_digest() != successor_signature.digest()
        || inventory.exact_extension().predecessor_boundary_digest()
            != inventory.predecessor_boundary().digest()
    {
        return Err(PredecessorPublicDeltaPolicyBindingFailureV1::InventoryBoundaryMismatch);
    }
    if global_slots.signature_digest() != successor_signature.digest() {
        return Err(PredecessorPublicDeltaPolicyBindingFailureV1::GlobalSlotTableMismatch);
    }
    if policy.signature_digest() != successor_signature.digest() {
        return Err(PredecessorPublicDeltaPolicyBindingFailureV1::PolicySignatureMismatch);
    }

    let exact_ordered_entries = derive_predecessor_public_delta_entries(
        inventory.predecessor_boundary(),
        successor_signature,
        global_slots,
    )?;
    if policy.wire().allowed_transparent_deltas != exact_ordered_entries {
        return Err(PredecessorPublicDeltaPolicyBindingFailureV1::PolicyInventoryMismatch);
    }
    let replayed = verify_base_q0_conversion_policy_v2(successor_signature, policy.wire())
        .map_err(PredecessorPublicDeltaPolicyBindingFailureV1::PolicyReplay)?;
    if replayed.signature_digest() != policy.signature_digest()
        || replayed.digest() != policy.digest()
        || replayed.wire() != policy.wire()
    {
        return Err(PredecessorPublicDeltaPolicyBindingFailureV1::PolicyReplayMismatch);
    }

    let exact_ordered_entries =
        Arc::<[DeltaPolicyEntryV2]>::from(exact_ordered_entries.into_boxed_slice());
    let all_and_only_bodyful_predecessor_digest = Digest::of_canonical(
        "pen-semantic-audit/all-and-only-bodyful-predecessor-public-delta-policy/v1",
        &PredecessorPublicDeltaCoverageMaterialV1 {
            predecessor: inventory.predecessor_boundary(),
            successor: successor_signature.digest(),
            global_slots: global_slots.digest(),
            policy: policy.digest(),
            entries: &exact_ordered_entries,
        },
    );
    let mut verified = VerifiedPredecessorPublicDeltaPolicyBindingV1 {
        public_inventory_digest: inventory.digest().clone(),
        public_inventory_compatibility_digest: compatibility.digest().clone(),
        lambda_unit_v2_manifest_digest: compatibility.v2_manifest_digest().clone(),
        predecessor_history_digest: inventory.predecessor_history_digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_signature_digest: successor_signature.digest().clone(),
        global_slot_table_digest: global_slots.digest().clone(),
        policy_digest: policy.digest().clone(),
        exact_ordered_entries,
        all_and_only_bodyful_predecessor_digest,
        digest: Digest::of_bytes(b"pending predecessor-public delta policy binding"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-predecessor-public-delta-policy-binding/v1",
        &verified,
    );
    Ok(verified)
}

pub fn verify_predecessor_public_delta_policy_binding_v1(
    inventory: &VerifiedPublicAuditInventoryV1,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    successor_signature: &VerifiedSignature,
    global_slots: &VerifiedGlobalSlotTableV1,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
) -> AuditDecision<VerifiedPredecessorPublicDeltaPolicyBindingV1> {
    match diagnose_predecessor_public_delta_policy_binding_v1(
        inventory,
        compatibility,
        successor_signature,
        global_slots,
        policy,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(PredecessorPublicDeltaPolicyBindingFailureV1::PolicyReplay(
            SynthesisV2Error::ResourceExhausted,
        )) => AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    }
}

/// Exact transport of one ledger-relative predecessor-public delta policy
/// through the narrow V1-to-V2 public-inventory compatibility theorem and
/// into the exact V3 semantic candidate.
///
/// This transports only the policy provenance needed by base conversion. It
/// is not the full V3 Q0/family inventory correspondence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedV3PredecessorPublicDeltaPolicyBindingV1 {
    lambda_unit_v2_manifest_digest: Digest,
    lambda_unit_v3_manifest_digest: Digest,
    public_inventory_digest: Digest,
    public_inventory_compatibility_digest: Digest,
    ledger_binding: VerifiedPredecessorPublicDeltaPolicyBindingV1,
    preserved_public_inventory_surface_digest: Digest,
    digest: Digest,
}

impl VerifiedV3PredecessorPublicDeltaPolicyBindingV1 {
    pub fn lambda_unit_v3_manifest_digest(&self) -> &Digest {
        &self.lambda_unit_v3_manifest_digest
    }

    pub fn ledger_binding(&self) -> &VerifiedPredecessorPublicDeltaPolicyBindingV1 {
        &self.ledger_binding
    }

    pub fn successor_signature_digest(&self) -> &Digest {
        self.ledger_binding.successor_signature_digest()
    }

    pub fn global_slot_table_digest(&self) -> &Digest {
        self.ledger_binding.global_slot_table_digest()
    }

    pub fn policy_digest(&self) -> &Digest {
        self.ledger_binding.policy_digest()
    }

    pub fn preserved_public_inventory_surface_digest(&self) -> &Digest {
        &self.preserved_public_inventory_surface_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedV3PredecessorPublicDeltaPolicyBindingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.lambda_unit_v2_manifest_digest
            .encode_canonical(encoder);
        self.lambda_unit_v3_manifest_digest
            .encode_canonical(encoder);
        self.public_inventory_digest.encode_canonical(encoder);
        self.public_inventory_compatibility_digest
            .encode_canonical(encoder);
        self.ledger_binding.encode_canonical(encoder);
        self.preserved_public_inventory_surface_digest
            .encode_canonical(encoder);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V3PredecessorPublicDeltaPolicyBindingFailureV1 {
    ExactV2ManifestIdentityMismatch,
    ExactV3ManifestIdentityMismatch,
    InventoryCompatibilityMismatch,
    LedgerBindingMismatch,
    PreservedPublicInventorySurfaceMismatch,
}

impl std::fmt::Display for V3PredecessorPublicDeltaPolicyBindingFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ExactV2ManifestIdentityMismatch => {
                "semantic manifest is not the exact lambda/unit V2 candidate"
            }
            Self::ExactV3ManifestIdentityMismatch => {
                "semantic manifest is not the exact lambda/unit V3 candidate"
            }
            Self::InventoryCompatibilityMismatch => {
                "the public inventory compatibility is not bound to the exact V2 candidate and inventory"
            }
            Self::LedgerBindingMismatch => {
                "the ledger-relative delta policy is not bound to the same inventory compatibility and boundaries"
            }
            Self::PreservedPublicInventorySurfaceMismatch => {
                "the exact V2 and V3 candidates do not preserve the public-inventory surface used by the delta policy"
            }
        })
    }
}

impl std::error::Error for V3PredecessorPublicDeltaPolicyBindingFailureV1 {}

pub fn diagnose_v3_predecessor_public_delta_policy_binding_v1(
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    ledger_binding: &VerifiedPredecessorPublicDeltaPolicyBindingV1,
) -> Result<
    VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    V3PredecessorPublicDeltaPolicyBindingFailureV1,
> {
    verify_exact_v2_identity(v2_manifest)?;
    verify_exact_v3_identity(v3_manifest).map_err(|_| {
        V3PredecessorPublicDeltaPolicyBindingFailureV1::ExactV3ManifestIdentityMismatch
    })?;
    if compatibility.v2_manifest_digest() != v2_manifest.candidate_digest()
        || compatibility.inventory_digest() != inventory.digest()
    {
        return Err(V3PredecessorPublicDeltaPolicyBindingFailureV1::InventoryCompatibilityMismatch);
    }
    if ledger_binding.public_inventory_digest() != inventory.digest()
        || ledger_binding.public_inventory_compatibility_digest() != compatibility.digest()
        || ledger_binding.lambda_unit_v2_manifest_digest() != v2_manifest.candidate_digest()
        || ledger_binding.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || ledger_binding.successor_signature_digest() != inventory.successor_boundary().digest()
    {
        return Err(V3PredecessorPublicDeltaPolicyBindingFailureV1::LedgerBindingMismatch);
    }

    let v2 = v2_manifest.manifest();
    let v3 = v3_manifest.manifest();
    if v2.universe_levels != v3.universe_levels
        || v2.maximum_context_entries != v3.maximum_context_entries
        || v2.q0_rules != v3.q0_rules
        || v2.q0_eta_registry_empty != v3.q0_eta_registry_empty
        || v2.q3_rule != v3.q3_rule
    {
        return Err(
            V3PredecessorPublicDeltaPolicyBindingFailureV1::PreservedPublicInventorySurfaceMismatch,
        );
    }
    let preserved_public_inventory_surface_digest = Digest::of_canonical(
        "pen-semantic-audit/v2-to-v3-preserved-public-inventory-surface/v1",
        &V3DeltaPolicyTransportMaterialV1 {
            v2_manifest: v2_manifest.candidate_digest(),
            v3_manifest: v3_manifest.candidate_digest(),
            inventory: inventory.digest(),
            compatibility: compatibility.digest(),
            ledger_binding: ledger_binding.digest(),
            universe_levels: &v3.universe_levels,
            maximum_context_entries: v3.maximum_context_entries,
            q0_rules: &v3.q0_rules,
            q0_eta_registry_empty: v3.q0_eta_registry_empty,
            q3_rule: v3.q3_rule,
        },
    );
    let mut verified = VerifiedV3PredecessorPublicDeltaPolicyBindingV1 {
        lambda_unit_v2_manifest_digest: v2_manifest.candidate_digest().clone(),
        lambda_unit_v3_manifest_digest: v3_manifest.candidate_digest().clone(),
        public_inventory_digest: inventory.digest().clone(),
        public_inventory_compatibility_digest: compatibility.digest().clone(),
        ledger_binding: ledger_binding.clone(),
        preserved_public_inventory_surface_digest,
        digest: Digest::of_bytes(b"pending V3 predecessor-public delta policy binding"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-v3-predecessor-public-delta-policy-binding/v1",
        &verified,
    );
    Ok(verified)
}

pub fn verify_v3_predecessor_public_delta_policy_binding_v1(
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    inventory: &VerifiedPublicAuditInventoryV1,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    ledger_binding: &VerifiedPredecessorPublicDeltaPolicyBindingV1,
) -> AuditDecision<VerifiedV3PredecessorPublicDeltaPolicyBindingV1> {
    match diagnose_v3_predecessor_public_delta_policy_binding_v1(
        v2_manifest,
        v3_manifest,
        inventory,
        compatibility,
        ledger_binding,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch),
    }
}

struct V3DeltaPolicyTransportMaterialV1<'a> {
    v2_manifest: &'a Digest,
    v3_manifest: &'a Digest,
    inventory: &'a Digest,
    compatibility: &'a Digest,
    ledger_binding: &'a Digest,
    universe_levels: &'a [u16],
    maximum_context_entries: u16,
    q0_rules: &'a [crate::manifest::Q0RuleV1],
    q0_eta_registry_empty: bool,
    q3_rule: crate::manifest::Q3RuleV1,
}

impl CanonicalEncode for V3DeltaPolicyTransportMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.v2_manifest.encode_canonical(encoder);
        self.v3_manifest.encode_canonical(encoder);
        self.inventory.encode_canonical(encoder);
        self.compatibility.encode_canonical(encoder);
        self.ledger_binding.encode_canonical(encoder);
        encode_u16_sequence(encoder, self.universe_levels);
        encoder.u16(self.maximum_context_entries);
        encoder.sequence(self.q0_rules);
        encoder.tag(u8::from(self.q0_eta_registry_empty));
        self.q3_rule.encode_canonical(encoder);
    }
}

fn verify_exact_v2_identity(
    manifest: &VerifiedSemanticAuditManifestV2,
) -> Result<(), V3PredecessorPublicDeltaPolicyBindingFailureV1> {
    let exact_wire = proposed_semantic_audit_lambda_unit_manifest_v2();
    let AuditDecision::Proven(exact) = verify_semantic_audit_lambda_unit_manifest_v2(&exact_wire)
    else {
        return Err(
            V3PredecessorPublicDeltaPolicyBindingFailureV1::ExactV2ManifestIdentityMismatch,
        );
    };
    if manifest.manifest() != &exact_wire || manifest.candidate_digest() != exact.candidate_digest()
    {
        return Err(
            V3PredecessorPublicDeltaPolicyBindingFailureV1::ExactV2ManifestIdentityMismatch,
        );
    }
    Ok(())
}

fn derive_predecessor_public_delta_entries(
    predecessor: &VerifiedSignature,
    successor: &VerifiedSignature,
    global_slots: &VerifiedGlobalSlotTableV1,
) -> Result<Vec<DeltaPolicyEntryV2>, PredecessorPublicDeltaPolicyBindingFailureV1> {
    let mut entries = Vec::new();
    for declaration in predecessor
        .declarations()
        .iter()
        .filter(|declaration| declaration.body.is_some())
    {
        let slot = global_slots
            .slot_for_global(&declaration.id)
            .ok_or(PredecessorPublicDeltaPolicyBindingFailureV1::MissingPredecessorGlobal)?;
        let slot = u32::try_from(slot.ordinal())
            .map_err(|_| PredecessorPublicDeltaPolicyBindingFailureV1::GlobalSlotOverflow)?;
        if successor.declarations().get(slot as usize) != Some(declaration)
            || global_slots
                .entry_for_slot(
                    global_slots
                        .slot_for_global(&declaration.id)
                        .expect("slot checked above"),
                )
                .map(|entry| entry.declaration())
                != Some(declaration)
        {
            return Err(
                PredecessorPublicDeltaPolicyBindingFailureV1::GlobalSlotDeclarationMismatch,
            );
        }
        entries.push(DeltaPolicyEntryV2 {
            global_slot: slot,
            id: declaration.id.clone(),
        });
    }
    Ok(entries)
}

struct PredecessorPublicDeltaCoverageMaterialV1<'a> {
    predecessor: &'a VerifiedSignature,
    successor: &'a Digest,
    global_slots: &'a Digest,
    policy: &'a Digest,
    entries: &'a [DeltaPolicyEntryV2],
}

impl CanonicalEncode for PredecessorPublicDeltaCoverageMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.predecessor.digest().encode_canonical(encoder);
        encoder.sequence(self.predecessor.declarations());
        self.successor.encode_canonical(encoder);
        self.global_slots.encode_canonical(encoder);
        self.policy.encode_canonical(encoder);
        encoder.sequence(self.entries);
    }
}

/// Exact unresolved Rust/Agda correspondence theorems.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionRefinementObligationV1 {
    FiniteContextAndGlobalSlotCorrespondence,
    KernelBaseConversionCorrespondence,
    SynthesisCodeCorrespondence,
    V3InventoryCorrespondence,
}

impl CanonicalEncode for ProductionRefinementObligationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::FiniteContextAndGlobalSlotCorrespondence => 0,
            Self::KernelBaseConversionCorrespondence => 1,
            Self::SynthesisCodeCorrespondence => 2,
            Self::V3InventoryCorrespondence => 3,
        });
    }
}

pub const PRODUCTION_REFINEMENT_CORRESPONDENCE_FRONTIER_V1: &[ProductionRefinementObligationV1] = &[
    ProductionRefinementObligationV1::FiniteContextAndGlobalSlotCorrespondence,
    ProductionRefinementObligationV1::KernelBaseConversionCorrespondence,
    ProductionRefinementObligationV1::SynthesisCodeCorrespondence,
    ProductionRefinementObligationV1::V3InventoryCorrespondence,
];

/// Downstream completeness intentionally excluded from the generic refinement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionRefinementExcludedDownstreamObligationV1 {
    CarrierDerivedFiniteTypingSubjectCoverage,
}

impl CanonicalEncode for ProductionRefinementExcludedDownstreamObligationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(0);
    }
}

pub const PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1:
    ProductionRefinementExcludedDownstreamObligationV1 =
    ProductionRefinementExcludedDownstreamObligationV1::CarrierDerivedFiniteTypingSubjectCoverage;

/// Opaque, currently unminted finite-context correspondence capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedFiniteContextCorrespondenceV1 {
    semantic_manifest_digest: Digest,
    global_slot_table: VerifiedGlobalSlotTableV1,
    agda_foundation: VerifiedProductionRefinementAgdaFoundationV1,
    variable_lookup_correspondence_digest: Digest,
    extension_correspondence_digest: Digest,
    shift_substitution_correspondence_digest: Digest,
    structural_round_trip_digest: Digest,
    digest: Digest,
}

impl VerifiedFiniteContextCorrespondenceV1 {
    pub fn global_slot_table(&self) -> &VerifiedGlobalSlotTableV1 {
        &self.global_slot_table
    }

    pub fn agda_foundation(&self) -> &VerifiedProductionRefinementAgdaFoundationV1 {
        &self.agda_foundation
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedFiniteContextCorrespondenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.global_slot_table.digest().encode_canonical(encoder);
        self.agda_foundation.digest().encode_canonical(encoder);
        self.variable_lookup_correspondence_digest
            .encode_canonical(encoder);
        self.extension_correspondence_digest
            .encode_canonical(encoder);
        self.shift_substitution_correspondence_digest
            .encode_canonical(encoder);
        self.structural_round_trip_digest.encode_canonical(encoder);
    }
}

/// Opaque, currently unminted base-conversion correspondence capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedKernelBaseConversionCorrespondenceV1 {
    semantic_manifest_digest: Digest,
    kernel_protocol_digest: Digest,
    synthesis_protocol_identity: VerifiedProductionSynthesisProtocolIdentityV2,
    predecessor_public_delta_policy: VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    typed_step_preservation_digest: Digest,
    typed_trace_preservation_digest: Digest,
    common_normal_form_conversion_digest: Digest,
    substitution_stability_digest: Digest,
    agda_foundation: VerifiedProductionRefinementAgdaFoundationV1,
    digest: Digest,
}

impl VerifiedKernelBaseConversionCorrespondenceV1 {
    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn synthesis_protocol_identity(&self) -> &VerifiedProductionSynthesisProtocolIdentityV2 {
        &self.synthesis_protocol_identity
    }

    pub fn predecessor_public_delta_policy(
        &self,
    ) -> &VerifiedV3PredecessorPublicDeltaPolicyBindingV1 {
        &self.predecessor_public_delta_policy
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedKernelBaseConversionCorrespondenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.synthesis_protocol_identity.encode_canonical(encoder);
        self.predecessor_public_delta_policy
            .encode_canonical(encoder);
        self.typed_step_preservation_digest
            .encode_canonical(encoder);
        self.typed_trace_preservation_digest
            .encode_canonical(encoder);
        self.common_normal_form_conversion_digest
            .encode_canonical(encoder);
        self.substitution_stability_digest.encode_canonical(encoder);
        self.agda_foundation.digest().encode_canonical(encoder);
    }
}

/// Opaque, currently unminted synthesis-code correspondence capability.
///
/// This is the generic claim that every accepted code has abstract meaning.
/// It does not claim that a carrier-derived finite root set is complete.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSynthesisCodeCorrespondenceV1 {
    semantic_manifest_digest: Digest,
    context_correspondence: VerifiedFiniteContextCorrespondenceV1,
    conversion_correspondence: VerifiedKernelBaseConversionCorrespondenceV1,
    synthesis_protocol_identity: VerifiedProductionSynthesisProtocolIdentityV2,
    checker_soundness_digest: Digest,
    production_image_reification_digest: Digest,
    structural_round_trip_digest: Digest,
    exact_eight_rule_coverage_digest: Digest,
    agda_foundation: VerifiedProductionRefinementAgdaFoundationV1,
    digest: Digest,
}

impl VerifiedSynthesisCodeCorrespondenceV1 {
    pub fn context_correspondence(&self) -> &VerifiedFiniteContextCorrespondenceV1 {
        &self.context_correspondence
    }

    pub fn conversion_correspondence(&self) -> &VerifiedKernelBaseConversionCorrespondenceV1 {
        &self.conversion_correspondence
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSynthesisCodeCorrespondenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.context_correspondence.encode_canonical(encoder);
        self.conversion_correspondence.encode_canonical(encoder);
        self.synthesis_protocol_identity.encode_canonical(encoder);
        self.checker_soundness_digest.encode_canonical(encoder);
        self.production_image_reification_digest
            .encode_canonical(encoder);
        self.structural_round_trip_digest.encode_canonical(encoder);
        self.exact_eight_rule_coverage_digest
            .encode_canonical(encoder);
        self.agda_foundation.digest().encode_canonical(encoder);
    }
}

/// Opaque, currently unminted V3 Q0/family inventory correspondence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedV3InventoryCorrespondenceV1 {
    semantic_manifest_digest: Digest,
    rust_inventory_bridge: VerifiedProductionInventoryBridgeV1,
    representation_rule_bridge_digest: Digest,
    base_q0_bridge_digest: Digest,
    fresh_rule_schema_bridge_digest: Digest,
    family_constructor_bridge_digest: Digest,
    no_extra_no_missing_digest: Digest,
    agda_foundation: VerifiedProductionRefinementAgdaFoundationV1,
    digest: Digest,
}

impl VerifiedV3InventoryCorrespondenceV1 {
    pub fn rust_inventory_bridge(&self) -> &VerifiedProductionInventoryBridgeV1 {
        &self.rust_inventory_bridge
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedV3InventoryCorrespondenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.rust_inventory_bridge.encode_canonical(encoder);
        self.representation_rule_bridge_digest
            .encode_canonical(encoder);
        self.base_q0_bridge_digest.encode_canonical(encoder);
        self.fresh_rule_schema_bridge_digest
            .encode_canonical(encoder);
        self.family_constructor_bridge_digest
            .encode_canonical(encoder);
        self.no_extra_no_missing_digest.encode_canonical(encoder);
        self.agda_foundation.digest().encode_canonical(encoder);
    }
}

/// Requested combined production refinement.
///
/// All four proof capabilities are stored as capabilities, not substitutable
/// digests. There is deliberately no constructor or `Deserialize`
/// implementation, and no verifier in this module returns this type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedLambdaUnitProductionRefinementV1 {
    semantic_manifest_digest: Digest,
    abstract_foundation: VerifiedLambdaUnitTypingFoundationV1,
    context_correspondence: VerifiedFiniteContextCorrespondenceV1,
    conversion_correspondence: VerifiedKernelBaseConversionCorrespondenceV1,
    synthesis_correspondence: VerifiedSynthesisCodeCorrespondenceV1,
    inventory_correspondence: VerifiedV3InventoryCorrespondenceV1,
    agda_foundation: VerifiedProductionRefinementAgdaFoundationV1,
    digest: Digest,
}

impl VerifiedLambdaUnitProductionRefinementV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn abstract_foundation(&self) -> &VerifiedLambdaUnitTypingFoundationV1 {
        &self.abstract_foundation
    }

    pub fn context_correspondence(&self) -> &VerifiedFiniteContextCorrespondenceV1 {
        &self.context_correspondence
    }

    pub fn conversion_correspondence(&self) -> &VerifiedKernelBaseConversionCorrespondenceV1 {
        &self.conversion_correspondence
    }

    pub fn synthesis_correspondence(&self) -> &VerifiedSynthesisCodeCorrespondenceV1 {
        &self.synthesis_correspondence
    }

    pub fn inventory_correspondence(&self) -> &VerifiedV3InventoryCorrespondenceV1 {
        &self.inventory_correspondence
    }

    pub fn agda_foundation(&self) -> &VerifiedProductionRefinementAgdaFoundationV1 {
        &self.agda_foundation
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedLambdaUnitProductionRefinementV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.abstract_foundation.encode_canonical(encoder);
        self.context_correspondence.encode_canonical(encoder);
        self.conversion_correspondence.encode_canonical(encoder);
        self.synthesis_correspondence.encode_canonical(encoder);
        self.inventory_correspondence.encode_canonical(encoder);
        self.agda_foundation.encode_canonical(encoder);
    }
}

/// Positive prerequisite identities plus the exact open theorem frontier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionRefinementFrontierV1 {
    semantic_manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    abstract_foundation_digest: Digest,
    global_slot_table_digest: Digest,
    public_sort_successor_digest: Digest,
    rust_inventory_bridge_digest: Digest,
    synthesis_protocol_identity_digest: Digest,
    production_agda_foundation_digest: Digest,
    predecessor_public_delta_policy_binding_digest: Option<Digest>,
    transparent_delta_authority_frontier: Option<TransparentDeltaAuthorityFrontierV2>,
    nested_congruence_replay_frontier: NestedCongruenceReplayFrontierV2,
    unresolved_correspondences: &'static [ProductionRefinementObligationV1],
    excluded_downstream_obligation: ProductionRefinementExcludedDownstreamObligationV1,
    digest: Digest,
}

impl ProductionRefinementFrontierV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn global_slot_table_digest(&self) -> &Digest {
        &self.global_slot_table_digest
    }

    pub fn public_sort_successor_digest(&self) -> &Digest {
        &self.public_sort_successor_digest
    }

    pub fn rust_inventory_bridge_digest(&self) -> &Digest {
        &self.rust_inventory_bridge_digest
    }

    pub fn synthesis_protocol_identity_digest(&self) -> &Digest {
        &self.synthesis_protocol_identity_digest
    }

    pub fn production_agda_foundation_digest(&self) -> &Digest {
        &self.production_agda_foundation_digest
    }

    pub fn predecessor_public_delta_policy_binding_digest(&self) -> Option<&Digest> {
        self.predecessor_public_delta_policy_binding_digest.as_ref()
    }

    pub fn transparent_delta_authority_frontier(
        &self,
    ) -> Option<TransparentDeltaAuthorityFrontierV2> {
        self.transparent_delta_authority_frontier
    }

    pub fn nested_congruence_replay_frontier(&self) -> NestedCongruenceReplayFrontierV2 {
        self.nested_congruence_replay_frontier
    }

    pub fn unresolved_correspondences(&self) -> &[ProductionRefinementObligationV1] {
        self.unresolved_correspondences
    }

    pub fn excluded_downstream_obligation(
        &self,
    ) -> ProductionRefinementExcludedDownstreamObligationV1 {
        self.excluded_downstream_obligation
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for ProductionRefinementFrontierV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(LAMBDA_UNIT_PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.abstract_foundation_digest.encode_canonical(encoder);
        self.global_slot_table_digest.encode_canonical(encoder);
        self.public_sort_successor_digest.encode_canonical(encoder);
        self.rust_inventory_bridge_digest.encode_canonical(encoder);
        self.synthesis_protocol_identity_digest
            .encode_canonical(encoder);
        self.production_agda_foundation_digest
            .encode_canonical(encoder);
        match &self.predecessor_public_delta_policy_binding_digest {
            Some(digest) => {
                encoder.tag(1);
                digest.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
        encoder.tag(match self.transparent_delta_authority_frontier {
            Some(
                TransparentDeltaAuthorityFrontierV2::MissingExactPredecessorPublicPolicyBinding,
            ) => 1,
            None => 0,
        });
        encoder.tag(match self.nested_congruence_replay_frontier {
            NestedCongruenceReplayFrontierV2::MissingBinderLocalKernelReplays => 0,
        });
        encoder.sequence(self.unresolved_correspondences);
        self.excluded_downstream_obligation
            .encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LambdaUnitProductionRefinementFailureV1 {
    ExactV3ManifestIdentityMismatch,
    GlobalSlotTable(ProductionRefinementFailureV1),
    PublicSortSuccessorsOutsideFragment(OutsideFragmentReason),
    PublicSortSuccessorsUnavailable(AuditUnknownReason),
    ProductionInventoryBridge(ProductionInventoryBridgeFailureV1),
    SynthesisProtocolIdentity(ProductionSynthesisProtocolIdentityFailureV1),
    AgdaGate(AgdaReferenceFailureV1),
    DeltaPolicyBindingMismatch,
    MissingProductionCorrespondences {
        frontier: Box<ProductionRefinementFrontierV1>,
    },
}

impl std::fmt::Display for LambdaUnitProductionRefinementFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactV3ManifestIdentityMismatch => formatter.write_str(
                "semantic manifest is not the exact generic unfrozen lambda/unit V3 candidate",
            ),
            Self::GlobalSlotTable(failure) => {
                write!(formatter, "global slot prerequisite failed: {failure}")
            }
            Self::PublicSortSuccessorsOutsideFragment(reason) => write!(
                formatter,
                "public-sort successor prerequisite left the fragment: {reason:?}"
            ),
            Self::PublicSortSuccessorsUnavailable(reason) => write!(
                formatter,
                "public-sort successor prerequisite is unavailable: {reason:?}"
            ),
            Self::ProductionInventoryBridge(failure) => {
                write!(formatter, "production inventory prerequisite failed: {failure}")
            }
            Self::SynthesisProtocolIdentity(failure) => write!(
                formatter,
                "synthesis protocol V2 identity prerequisite failed: {failure:?}"
            ),
            Self::AgdaGate(failure) => {
                write!(formatter, "pinned production Agda foundation failed: {failure}")
            }
            Self::DeltaPolicyBindingMismatch => formatter.write_str(
                "the supplied predecessor-public delta-policy binding does not match the checked signature and slot table",
            ),
            Self::MissingProductionCorrespondences { .. } => formatter.write_str(
                "the exact finite-context, base-conversion, synthesis-code, and V3-inventory Rust/Agda correspondences remain unproved",
            ),
        }
    }
}

impl std::error::Error for LambdaUnitProductionRefinementFailureV1 {}

/// Diagnose the production-refinement theorem with no claimed
/// predecessor-public delta binding.
///
/// Successful prerequisite checks still end in
/// `MissingProductionCorrespondences`. The transparent-delta policy frontier
/// is recorded as a sub-obligation of base-conversion correspondence.
pub fn diagnose_lambda_unit_production_refinement_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    abstract_foundation: &VerifiedLambdaUnitTypingFoundationV1,
) -> Result<VerifiedLambdaUnitProductionRefinementV1, LambdaUnitProductionRefinementFailureV1> {
    diagnose_lambda_unit_production_refinement_inner_v1(
        manifest,
        kernel,
        signature,
        abstract_foundation,
        None,
    )
}

/// Diagnose the same theorem while binding a separately verified exact
/// predecessor-public delta policy. This removes only that conversion
/// sub-frontier; it does not mint any of the four correspondence capabilities.
pub fn diagnose_lambda_unit_production_refinement_with_delta_policy_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    abstract_foundation: &VerifiedLambdaUnitTypingFoundationV1,
    delta_policy_binding: &VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
) -> Result<VerifiedLambdaUnitProductionRefinementV1, LambdaUnitProductionRefinementFailureV1> {
    diagnose_lambda_unit_production_refinement_inner_v1(
        manifest,
        kernel,
        signature,
        abstract_foundation,
        Some(delta_policy_binding),
    )
}

fn diagnose_lambda_unit_production_refinement_inner_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    abstract_foundation: &VerifiedLambdaUnitTypingFoundationV1,
    delta_policy_binding: Option<&VerifiedV3PredecessorPublicDeltaPolicyBindingV1>,
) -> Result<VerifiedLambdaUnitProductionRefinementV1, LambdaUnitProductionRefinementFailureV1> {
    verify_exact_v3_identity(manifest)?;

    let global_slots = diagnose_global_slot_table_v1(kernel, signature)
        .map_err(LambdaUnitProductionRefinementFailureV1::GlobalSlotTable)?;
    if let Some(binding) = delta_policy_binding {
        if binding.lambda_unit_v3_manifest_digest() != manifest.candidate_digest()
            || binding.successor_signature_digest() != signature.digest()
            || binding.global_slot_table_digest() != global_slots.digest()
        {
            return Err(LambdaUnitProductionRefinementFailureV1::DeltaPolicyBindingMismatch);
        }
    }

    let public_sort_successors = match verify_lambda_unit_public_sort_successors_v1(
        manifest,
        kernel,
        signature,
        &global_slots,
    ) {
        AuditDecision::Proven(verified) => verified,
        AuditDecision::OutsideFragment(reason) => {
            return Err(
                LambdaUnitProductionRefinementFailureV1::PublicSortSuccessorsOutsideFragment(
                    reason,
                ),
            );
        }
        AuditDecision::Unknown(reason) => {
            return Err(
                LambdaUnitProductionRefinementFailureV1::PublicSortSuccessorsUnavailable(reason),
            );
        }
    };
    let inventory_bridge = diagnose_production_inventory_bridge_v1(manifest)
        .map_err(LambdaUnitProductionRefinementFailureV1::ProductionInventoryBridge)?;
    let synthesis_protocol = diagnose_production_synthesis_protocol_identity_v2()
        .map_err(LambdaUnitProductionRefinementFailureV1::SynthesisProtocolIdentity)?;
    let production_agda = diagnose_pinned_production_refinement_agda_foundation_v1()
        .map_err(LambdaUnitProductionRefinementFailureV1::AgdaGate)?;

    let frontier = production_refinement_frontier(
        manifest,
        kernel,
        signature,
        abstract_foundation,
        &global_slots,
        &public_sort_successors,
        &inventory_bridge,
        &synthesis_protocol,
        &production_agda,
        delta_policy_binding,
    );
    Err(
        LambdaUnitProductionRefinementFailureV1::MissingProductionCorrespondences {
            frontier: Box::new(frontier),
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn production_refinement_frontier(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    abstract_foundation: &VerifiedLambdaUnitTypingFoundationV1,
    global_slots: &VerifiedGlobalSlotTableV1,
    public_sort_successors: &VerifiedLambdaUnitPublicSortSuccessorsV1,
    inventory_bridge: &VerifiedProductionInventoryBridgeV1,
    synthesis_protocol: &VerifiedProductionSynthesisProtocolIdentityV2,
    production_agda: &VerifiedProductionRefinementAgdaFoundationV1,
    delta_policy_binding: Option<&VerifiedV3PredecessorPublicDeltaPolicyBindingV1>,
) -> ProductionRefinementFrontierV1 {
    let mut frontier = ProductionRefinementFrontierV1 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        abstract_foundation_digest: abstract_foundation.digest().clone(),
        global_slot_table_digest: global_slots.digest().clone(),
        public_sort_successor_digest: public_sort_successors.digest().clone(),
        rust_inventory_bridge_digest: inventory_bridge.digest().clone(),
        synthesis_protocol_identity_digest: synthesis_protocol.digest().clone(),
        production_agda_foundation_digest: production_agda.digest().clone(),
        predecessor_public_delta_policy_binding_digest: delta_policy_binding
            .map(|binding| binding.digest().clone()),
        transparent_delta_authority_frontier: delta_policy_binding
            .is_none()
            .then_some(TRANSPARENT_DELTA_AUTHORITY_FRONTIER_V2),
        nested_congruence_replay_frontier: NESTED_CONGRUENCE_REPLAY_FRONTIER_V2,
        unresolved_correspondences: PRODUCTION_REFINEMENT_CORRESPONDENCE_FRONTIER_V1,
        excluded_downstream_obligation: PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1,
        digest: Digest::of_bytes(b"pending production-refinement frontier"),
    };
    frontier.digest = Digest::of_canonical(
        "pen-semantic-audit/lambda-unit-production-refinement-frontier/v1",
        &frontier,
    );
    frontier
}

fn verify_exact_v3_identity(
    manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<(), LambdaUnitProductionRefinementFailureV1> {
    let exact_wire = proposed_semantic_audit_lambda_unit_manifest_v3();
    let AuditDecision::Proven(exact) = verify_semantic_audit_lambda_unit_manifest_v3(&exact_wire)
    else {
        return Err(LambdaUnitProductionRefinementFailureV1::ExactV3ManifestIdentityMismatch);
    };
    if manifest.manifest() != &exact_wire || manifest.candidate_digest() != exact.candidate_digest()
    {
        return Err(LambdaUnitProductionRefinementFailureV1::ExactV3ManifestIdentityMismatch);
    }
    Ok(())
}

pub fn verify_lambda_unit_production_refinement_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    abstract_foundation: &VerifiedLambdaUnitTypingFoundationV1,
) -> AuditDecision<VerifiedLambdaUnitProductionRefinementV1> {
    match diagnose_lambda_unit_production_refinement_v1(
        manifest,
        kernel,
        signature,
        abstract_foundation,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(LambdaUnitProductionRefinementFailureV1::ExactV3ManifestIdentityMismatch) => {
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        }
        Err(
            LambdaUnitProductionRefinementFailureV1::AgdaGate(_)
            | LambdaUnitProductionRefinementFailureV1::SynthesisProtocolIdentity(_),
        ) => AuditDecision::Unknown(AuditUnknownReason::UnsupportedVerifier),
        Err(LambdaUnitProductionRefinementFailureV1::GlobalSlotTable(
            ProductionRefinementFailureV1::ResourceExhausted
            | ProductionRefinementFailureV1::KernelReplay(
                pen_kernel::KernelError::ResourceExhausted(_),
            ),
        ))
        | Err(LambdaUnitProductionRefinementFailureV1::PublicSortSuccessorsUnavailable(
            AuditUnknownReason::ResourceExhausted,
        )) => AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted),
        Err(
            LambdaUnitProductionRefinementFailureV1::ProductionInventoryBridge(_)
            | LambdaUnitProductionRefinementFailureV1::MissingProductionCorrespondences { .. },
        ) => AuditDecision::Unknown(AuditUnknownReason::MissingLambdaUnitTypingMetatheory),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    }
}

fn encode_u16_sequence(encoder: &mut CanonicalEncoder, values: &[u16]) {
    encoder.u64(values.len() as u64);
    for value in values {
        encoder.u16(*value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        ORIGIN_CUTOFF_Q3_SCHEMA_VERSION, PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
        UncheckedOriginCutoffQ3RegistryV1, UncheckedPublicAuditInventoryV1,
        UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEventCensusV1,
        UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
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
    use crate::model::EventIdV1;
    use crate::typing_metatheory::verify_pinned_lambda_unit_typing_foundation_v1;
    use pen_kernel::{Declaration, GlobalId, KernelLimits, Term, UncheckedSignature};
    use pen_kernel_synthesis::BaseQ0ConversionPolicyV2;

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("kernel")
    }

    fn global(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn exact_manifest() -> VerifiedSemanticAuditManifestV3 {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v3(
            &proposed_semantic_audit_lambda_unit_manifest_v3(),
        ) else {
            panic!("exact V3 manifest");
        };
        manifest
    }

    #[test]
    fn v2_identity_binds_exact_universe_and_eight_tag_surfaces() {
        let verified =
            diagnose_production_synthesis_protocol_identity_v2().expect("exact V2 identity");
        assert_eq!(
            verified.protocol_id(),
            "pen-kernel-synthesis/lambda-unit/v2"
        );
        assert_eq!(verified.schema_version(), 2);
        assert_eq!(verified.synthesis_code_constructor_count(), 8);
        assert_eq!(verified.public_universe_levels(), &[0, 1]);
        assert_eq!(verified.checker_output_universe_levels(), &[0, 1, 2]);
        assert_eq!(
            verified.synthesis_code_inventory_digest(),
            &synthesis_code_inventory_digest_v2()
        );
        assert_eq!(
            verified.implementation_protocol_digest(),
            &synthesis_protocol_digest_v2()
        );
    }

    #[test]
    fn correspondence_frontier_is_exact_and_excludes_carrier_coverage() {
        assert_eq!(
            PRODUCTION_REFINEMENT_CORRESPONDENCE_FRONTIER_V1,
            &[
                ProductionRefinementObligationV1::FiniteContextAndGlobalSlotCorrespondence,
                ProductionRefinementObligationV1::KernelBaseConversionCorrespondence,
                ProductionRefinementObligationV1::SynthesisCodeCorrespondence,
                ProductionRefinementObligationV1::V3InventoryCorrespondence,
            ]
        );
        assert_eq!(
            PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1,
            ProductionRefinementExcludedDownstreamObligationV1::CarrierDerivedFiniteTypingSubjectCoverage
        );
    }

    #[test]
    fn predecessor_policy_derivation_uses_only_bodyful_predecessor_declarations() {
        let kernel = kernel();
        let predecessor_type = Declaration {
            id: global(b"production-policy/type"),
            ty: Term::Sort { level: 0 },
            body: None,
        };
        let predecessor_alias = Declaration {
            id: global(b"production-policy/predecessor-alias"),
            ty: Term::UnitType,
            body: Some(Term::Unit),
        };
        let successor_alias = Declaration {
            id: global(b"production-policy/successor-alias"),
            ty: Term::UnitType,
            body: Some(Term::Unit),
        };
        let predecessor = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![predecessor_type.clone(), predecessor_alias.clone()],
            })
            .expect("predecessor");
        let successor = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![predecessor_type, predecessor_alias.clone(), successor_alias],
            })
            .expect("successor");
        let slots = diagnose_global_slot_table_v1(&kernel, &successor).expect("slots");
        let entries = derive_predecessor_public_delta_entries(&predecessor, &successor, &slots)
            .expect("derived predecessor policy");
        assert_eq!(
            entries,
            vec![DeltaPolicyEntryV2 {
                global_slot: 1,
                id: predecessor_alias.id,
            }]
        );
        let policy = BaseQ0ConversionPolicyV2 {
            allowed_transparent_deltas: entries,
        };
        assert!(verify_base_q0_conversion_policy_v2(&successor, &policy).is_ok());
    }

    #[test]
    fn predecessor_policy_binding_is_ledger_relative_and_exact() {
        let kernel = kernel();
        let predecessor_id = global(b"production-policy-ledger/predecessor");
        let successor_id = global(b"production-policy-ledger/successor");
        let predecessor_group = global(b"production-policy-ledger/predecessor-group");
        let successor_group = global(b"production-policy-ledger/successor-group");
        let predecessor_event = EventIdV1(Digest::of_bytes(
            b"production-policy-ledger/predecessor-event",
        ));
        let successor_event = EventIdV1(Digest::of_bytes(
            b"production-policy-ledger/successor-event",
        ));
        let predecessor_declaration = Declaration {
            id: predecessor_id.clone(),
            ty: Term::UnitType,
            body: Some(Term::Unit),
        };
        let successor_declaration = Declaration {
            id: successor_id.clone(),
            ty: Term::UnitType,
            body: None,
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
        let source = |declaration: &Declaration| UncheckedSourceNormalizedDeclarationV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-declaration/v1",
                declaration,
            ),
            source: declaration.clone(),
            claimed_normalized: declaration.clone(),
        };
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: vec![UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group.clone()],
                    added_declarations: vec![predecessor_id.clone()],
                    added_equations: Vec::new(),
                    added_forced_projections: Vec::new(),
                    added_demand_contracts: Vec::new(),
                },
                successor_boundary: predecessor_boundary.clone(),
            }],
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: successor_event.clone(),
                added_groups: vec![successor_group.clone()],
                added_declarations: vec![successor_id.clone()],
                added_equations: Vec::new(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups: vec![
                UncheckedPublicGroupV1 {
                    group: predecessor_group.clone(),
                    origin: predecessor_event.clone(),
                    declarations: vec![predecessor_id.clone()],
                },
                UncheckedPublicGroupV1 {
                    group: successor_group.clone(),
                    origin: successor_event.clone(),
                    declarations: vec![successor_id.clone()],
                },
            ],
            declarations: vec![
                UncheckedPublicDeclarationV1 {
                    declaration: predecessor_id.clone(),
                    origin: predecessor_event.clone(),
                    group: predecessor_group,
                    source_to_normal: source(&predecessor_declaration),
                },
                UncheckedPublicDeclarationV1 {
                    declaration: successor_id,
                    origin: successor_event,
                    group: successor_group,
                    source_to_normal: source(&successor_declaration),
                },
            ],
            equations: Vec::new(),
            forced_projections: Vec::new(),
            predecessor_demand_contracts: Vec::new(),
            public_availability: Vec::new(),
            dependency_dag: UncheckedPublicDependencyDagV1 { edges: Vec::new() },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: Some(predecessor_event),
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(v1_manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("lambda/unit V1 manifest");
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&v1_manifest, &kernel, &wire)
        else {
            panic!("minimal verified public inventory");
        };
        let AuditDecision::Proven(v2_manifest) = verify_semantic_audit_lambda_unit_manifest_v2(
            &proposed_semantic_audit_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit V2 manifest");
        };
        let AuditDecision::Proven(compatibility) =
            verify_public_inventory_compatibility_v2(&v1_manifest, &v2_manifest, &inventory)
        else {
            panic!("lambda/unit inventory compatibility");
        };
        let successor = inventory.successor_boundary();
        let slots = diagnose_global_slot_table_v1(&kernel, successor).expect("slots");
        let policy_wire = BaseQ0ConversionPolicyV2 {
            allowed_transparent_deltas: vec![DeltaPolicyEntryV2 {
                global_slot: 0,
                id: predecessor_id,
            }],
        };
        let policy = verify_base_q0_conversion_policy_v2(successor, &policy_wire)
            .expect("verified V2 policy");
        let binding = diagnose_predecessor_public_delta_policy_binding_v1(
            &inventory,
            &compatibility,
            successor,
            &slots,
            &policy,
        )
        .expect("exact predecessor-public policy binding");
        assert_eq!(
            binding.exact_ordered_entries(),
            &policy_wire.allowed_transparent_deltas
        );
        assert_eq!(binding.public_inventory_digest(), inventory.digest());
        assert_eq!(binding.global_slot_table_digest(), slots.digest());
        assert_eq!(binding.policy_digest(), policy.digest());
        let v3_manifest = exact_manifest();
        let transported = diagnose_v3_predecessor_public_delta_policy_binding_v1(
            &v2_manifest,
            &v3_manifest,
            &inventory,
            &compatibility,
            &binding,
        )
        .expect("exact V3 policy transport");
        assert_eq!(
            transported.lambda_unit_v3_manifest_digest(),
            v3_manifest.candidate_digest()
        );
        assert_eq!(transported.ledger_binding(), &binding);
    }

    #[test]
    fn fixed_agda_packages_pin_the_observed_exact_checker_order() {
        assert_eq!(
            PRODUCTION_AGDA_SOURCES
                .iter()
                .map(|source| source.module_name)
                .collect::<Vec<_>>(),
            vec![
                "LawV2.LambdaUnit.LambdaUnitProductionRefinementV1",
                "LawV2.LambdaUnit.ProductionSyntaxV1",
                "LawV2.LambdaUnit.TypingSyntax",
                "LawV2.LambdaUnit.Substitution",
                "LawV2.LambdaUnit.SubstitutionReduction",
                "LawV2.LambdaUnit.ProductionDecodingV1",
                "LawV2.LambdaUnit.ProductionConversionTyping",
                "LawV2.LambdaUnit.TypingJudgment",
                "LawV2.LambdaUnit.ReductionTyping",
                "LawV2.LambdaUnit.SubstitutionTypingV3",
                "LawV2.LambdaUnit.ProductionSynthesisCodeV2",
            ]
        );
        assert_eq!(
            INVENTORY_AGDA_SOURCES
                .iter()
                .map(|source| source.module_name)
                .collect::<Vec<_>>(),
            vec![
                "LawV2.LambdaUnit.ProductionInventoryBridgeV1",
                "LawV2.LambdaUnit.FamilyNaturality",
                "LawV2.LambdaUnit.Substitution",
                "LawV2.LambdaUnit.SubstitutionReduction",
            ]
        );
        for source in [PRODUCTION_AGDA_SOURCES[0], INVENTORY_AGDA_SOURCES[0]] {
            let text = std::str::from_utf8(source.bytes).expect("fixed Agda is UTF-8");
            assert!(text.contains("{-# OPTIONS --safe --without-K #-}"));
            assert!(!text.contains("postulate"));
            assert!(!text.contains("{-# TERMINATING #-}"));
        }
    }

    #[test]
    #[ignore = "requires the pinned Agda executable and primitive source tree"]
    fn exact_prerequisites_reach_only_the_correspondence_frontier() {
        let kernel = kernel();
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let AuditDecision::Proven(abstract_foundation) =
            verify_pinned_lambda_unit_typing_foundation_v1()
        else {
            panic!("pinned abstract foundation");
        };
        let failure = diagnose_lambda_unit_production_refinement_v1(
            &exact_manifest(),
            &kernel,
            &signature,
            &abstract_foundation,
        )
        .expect_err("correspondences remain unproved");
        let LambdaUnitProductionRefinementFailureV1::MissingProductionCorrespondences { frontier } =
            failure
        else {
            panic!("exact prerequisites must stop only at correspondence");
        };
        assert_eq!(
            frontier.unresolved_correspondences(),
            PRODUCTION_REFINEMENT_CORRESPONDENCE_FRONTIER_V1
        );
        assert_eq!(
            frontier.transparent_delta_authority_frontier(),
            Some(TransparentDeltaAuthorityFrontierV2::MissingExactPredecessorPublicPolicyBinding)
        );
        assert_eq!(
            frontier.nested_congruence_replay_frontier(),
            NestedCongruenceReplayFrontierV2::MissingBinderLocalKernelReplays
        );
        assert_eq!(
            frontier.excluded_downstream_obligation(),
            PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1
        );
    }
}
