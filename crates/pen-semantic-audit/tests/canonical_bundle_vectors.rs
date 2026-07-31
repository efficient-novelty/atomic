//! The canonical bundle is genuinely builder-derived (Phase G).
//!
//! This test constructs the complete verified-input chain — kernel
//! signature, verified global-slot table, exact V3 manifest, production
//! inventory bridge, ledger-relative predecessor-public delta policy
//! (transported into V3), and the synthesis protocol identity — for the
//! cross-language fixture content, calls
//! `build_canonical_production_bundle_v1`, and requires the resulting
//! canonical bytes to equal the committed cross-language literal
//! byte-for-byte. The certificate payload sections are taken from the
//! decoded committed literal itself (they are authority-free caller
//! payload by construction); sections 1–3 and the inventories are
//! derived from the verified capabilities, so this test proves the
//! committed vector's manifest surface, signature binding, and slot
//! table are exactly what the capability chain derives.
//!
//! The predecessor story matches the delta policy: the bodyful unit
//! value (slot 0) is the sole predecessor-public declaration, so the
//! all-and-only-bodyful-predecessor policy is exactly `[slot 0]`; the
//! successor event seals the three bodyless declarations.
//!
//! This is a development regression test, not correspondence authority.

use pen_kernel::{Declaration, Digest, GlobalId, Kernel, KernelLimits, Term};
use pen_kernel_synthesis::{
    BaseQ0ConversionPolicyV2, DeltaPolicyEntryV2, verify_base_q0_conversion_policy_v2,
};
use pen_semantic_audit::{
    AuditDecision, EventIdV1, ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
    PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION, ProductionBundlePayloadV1,
    UncheckedOriginCutoffQ3RegistryV1, UncheckedPublicAuditInventoryV1,
    UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEventCensusV1,
    UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1, UncheckedSourceNormalizedDeclarationV1,
    build_canonical_production_bundle_v1, diagnose_global_slot_table_v1,
    diagnose_predecessor_public_delta_policy_binding_v1,
    diagnose_production_synthesis_protocol_identity_v2,
    diagnose_v3_predecessor_public_delta_policy_binding_v1,
    proposed_semantic_audit_lambda_unit_manifest_v1,
    proposed_semantic_audit_lambda_unit_manifest_v2, replay_production_bundle_v1,
    verify_production_inventory_bridge_v1, verify_public_audit_inventory_v1,
    verify_public_inventory_compatibility_v2, verify_semantic_audit_lambda_unit_manifest_v1,
    verify_semantic_audit_lambda_unit_manifest_v2, verify_semantic_audit_lambda_unit_manifest_v3,
};
use pen_production_wire::decode_bundle_v1;
use pen_semantic_audit::proposed_semantic_audit_lambda_unit_manifest_v3;
use pen_kernel::UncheckedSignature;

const AGDA_VECTOR_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/agda/LawV2/Wire/BundleDecodeTestV1.agda"
);

fn committed_agda_literal(path: &str, header: &str) -> Vec<u8> {
    let source =
        std::fs::read_to_string(path).expect("committed Agda vector module must be readable");
    let mut bytes = Vec::new();
    let mut in_literal = false;
    for line in source.lines() {
        if line.trim_end() == header {
            in_literal = true;
            continue;
        }
        if !in_literal {
            continue;
        }
        if line.trim() == "[]" {
            break;
        }
        for token in line.split('\u{2237}') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            bytes.push(
                token
                    .parse::<u8>()
                    .expect("Agda vector literal must contain only bytes"),
            );
        }
    }
    assert!(in_literal, "Agda byte literal not found: {header}");
    bytes
}

fn wire_global(byte: u8) -> GlobalId {
    let mut hex = String::from("blake3:");
    for _ in 0..32 {
        hex.push_str(&format!("{byte:02x}"));
    }
    GlobalId(Digest::parse(hex).expect("canonical wire global id"))
}

fn unit_type() -> Term {
    Term::UnitType
}

fn family_application_pi() -> Term {
    Term::Pi {
        parameter: Box::new(Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::Sort { level: 0 }),
        }),
        body: Box::new(Term::Apply {
            function: Box::new(Term::Var { index: 0 }),
            argument: Box::new(Term::Unit),
        }),
    }
}

fn uelim_pi() -> Term {
    Term::Pi {
        parameter: Box::new(Term::UnitType),
        body: Box::new(Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        }),
    }
}

fn declarations() -> Vec<Declaration> {
    vec![
        Declaration {
            id: wire_global(1),
            ty: unit_type(),
            body: Some(Term::Unit),
        },
        Declaration {
            id: wire_global(2),
            ty: unit_type(),
            body: None,
        },
        Declaration {
            id: wire_global(3),
            ty: family_application_pi(),
            body: None,
        },
        Declaration {
            id: wire_global(4),
            ty: uelim_pi(),
            body: None,
        },
    ]
}

struct GenuineCapabilities {
    kernel: Kernel,
    manifest_v3: pen_semantic_audit::VerifiedSemanticAuditManifestV3,
    signature: pen_kernel::VerifiedSignature,
    slots: pen_semantic_audit::VerifiedGlobalSlotTableV1,
    bridge: pen_semantic_audit::VerifiedProductionInventoryBridgeV1,
    delta_policy: pen_semantic_audit::VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    synthesis: pen_semantic_audit::VerifiedProductionSynthesisProtocolIdentityV2,
}

/// The genuine verified-input chain for the fixture content: the
/// bodyful unit value is the sole predecessor-public declaration; the
/// successor event seals the three bodyless declarations.
fn genuine_capabilities() -> GenuineCapabilities {
    let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
    let declarations = declarations();
    let predecessor_declaration = declarations[0].clone();
    let predecessor_id = predecessor_declaration.id.clone();
    let successor_ids: Vec<GlobalId> = declarations[1..]
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect();
    let predecessor_group = GlobalId(Digest::of_bytes(b"canonical-bundle/predecessor-group"));
    let successor_group = GlobalId(Digest::of_bytes(b"canonical-bundle/successor-group"));
    let predecessor_event =
        EventIdV1(Digest::of_bytes(b"canonical-bundle/predecessor-event"));
    let successor_event = EventIdV1(Digest::of_bytes(b"canonical-bundle/successor-event"));
    let predecessor_boundary = UncheckedSignature {
        declarations: vec![predecessor_declaration.clone()],
    };
    let successor_boundary = UncheckedSignature {
        declarations: declarations.clone(),
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
            added_declarations: successor_ids.clone(),
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
                declarations: successor_ids.clone(),
            },
        ],
        declarations: std::iter::once(UncheckedPublicDeclarationV1 {
            declaration: predecessor_id.clone(),
            origin: predecessor_event.clone(),
            group: predecessor_group,
            source_to_normal: source(&predecessor_declaration),
        })
        .chain(
            declarations[1..]
                .iter()
                .map(|declaration| UncheckedPublicDeclarationV1 {
                    declaration: declaration.id.clone(),
                    origin: successor_event.clone(),
                    group: successor_group.clone(),
                    source_to_normal: source(declaration),
                }),
        )
        .collect(),
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
        panic!("verified public inventory");
    };
    let AuditDecision::Proven(v2_manifest) = verify_semantic_audit_lambda_unit_manifest_v2(
        &proposed_semantic_audit_lambda_unit_manifest_v2(),
    ) else {
        panic!("lambda/unit V2 manifest");
    };
    let AuditDecision::Proven(compatibility) =
        verify_public_inventory_compatibility_v2(&v1_manifest, &v2_manifest, &inventory)
    else {
        panic!("inventory compatibility");
    };
    let successor = inventory.successor_boundary().clone();
    let slots = diagnose_global_slot_table_v1(&kernel, &successor).expect("slot table");
    let policy_wire = BaseQ0ConversionPolicyV2 {
        allowed_transparent_deltas: vec![DeltaPolicyEntryV2 {
            global_slot: 0,
            id: predecessor_id,
        }],
    };
    let policy =
        verify_base_q0_conversion_policy_v2(&successor, &policy_wire).expect("verified policy");
    let binding = diagnose_predecessor_public_delta_policy_binding_v1(
        &inventory,
        &compatibility,
        &successor,
        &slots,
        &policy,
    )
    .expect("predecessor-public policy binding");
    let AuditDecision::Proven(manifest_v3) = verify_semantic_audit_lambda_unit_manifest_v3(
        &proposed_semantic_audit_lambda_unit_manifest_v3(),
    ) else {
        panic!("lambda/unit V3 manifest");
    };
    let delta_policy = diagnose_v3_predecessor_public_delta_policy_binding_v1(
        &v2_manifest,
        &manifest_v3,
        &inventory,
        &compatibility,
        &binding,
    )
    .expect("V3 delta policy transport");
    let AuditDecision::Proven(bridge) = verify_production_inventory_bridge_v1(&manifest_v3)
    else {
        panic!("production inventory bridge");
    };
    let synthesis =
        diagnose_production_synthesis_protocol_identity_v2().expect("synthesis identity");
    GenuineCapabilities {
        kernel,
        manifest_v3,
        signature: successor,
        slots,
        bridge,
        delta_policy,
        synthesis,
    }
}

/// The committed canonical vector is exactly the builder output over
/// the genuine capability chain, with the certificate payload taken
/// from the committed vector itself.
#[test]
fn canonical_vector_is_builder_derived() {
    let committed = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    let decoded = decode_bundle_v1(&committed).expect("committed vector decodes");
    let capabilities = genuine_capabilities();
    let payload = ProductionBundlePayloadV1 {
        contexts: decoded.contexts.clone(),
        conversions: decoded.conversions.clone(),
        conversion_typing_supplements: decoded.conversion_typing_supplements.clone(),
        synthesis_codes: decoded.synthesis_codes.clone(),
        fresh_rule_schemas: decoded.fresh_rule_schemas.clone(),
        family_payloads: decoded.family_payloads.clone(),
    };
    let bundle = build_canonical_production_bundle_v1(
        &capabilities.manifest_v3,
        &capabilities.signature,
        &capabilities.slots,
        &capabilities.bridge,
        &capabilities.delta_policy,
        &capabilities.synthesis,
        payload,
    )
    .expect("genuine canonical bundle");
    assert_eq!(
        bundle.canonical_bytes(),
        committed.as_slice(),
        "the committed canonical vector must be the builder output"
    );
    let _ = &capabilities.kernel;
    replay_production_bundle_v1(bundle.canonical_bytes()).expect("builder output replays");
}

/// The complete Phase G capability chain over the genuine canonical
/// bundle: the independent Rust replay through the unchanged kernel,
/// the safe-Agda acceptance of the generated input package under the
/// pinned Agda 2.8.0 checker, and the exact common-input and
/// common-transcript byte agreement. Requires the pinned local Agda
/// runtime, so it is ignored by default like every external-gate test.
#[test]
#[ignore]
fn phase_g_capabilities_mint_over_genuine_bundle() {
    let committed = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    let decoded = decode_bundle_v1(&committed).expect("committed vector decodes");
    let capabilities = genuine_capabilities();
    let payload = ProductionBundlePayloadV1 {
        contexts: decoded.contexts.clone(),
        conversions: decoded.conversions.clone(),
        conversion_typing_supplements: decoded.conversion_typing_supplements.clone(),
        synthesis_codes: decoded.synthesis_codes.clone(),
        fresh_rule_schemas: decoded.fresh_rule_schemas.clone(),
        family_payloads: decoded.family_payloads.clone(),
    };
    let bundle = build_canonical_production_bundle_v1(
        &capabilities.manifest_v3,
        &capabilities.signature,
        &capabilities.slots,
        &capabilities.bridge,
        &capabilities.delta_policy,
        &capabilities.synthesis,
        payload,
    )
    .expect("genuine canonical bundle");
    let replay = pen_semantic_audit::verify_rust_production_replay_v1(&bundle)
        .expect("independent Rust replay capability");
    assert_eq!(replay.replayed_bundle_bytes(), bundle.canonical_bytes());
    let acceptance = pen_semantic_audit::verify_agda_production_acceptance_v1(&bundle, &replay)
        .expect("safe-Agda acceptance capability");
    assert_eq!(acceptance.accepted_bundle_bytes(), bundle.canonical_bytes());
    assert_eq!(
        acceptance.accepted_sections().sections(),
        pen_semantic_audit::REQUIRED_ACCEPTED_PRODUCTION_SECTIONS_V1
    );
    let agreement = pen_semantic_audit::verify_production_transcript_agreement_v1(
        &bundle,
        &acceptance,
        &replay,
    )
    .expect("transcript agreement capability");
    assert_eq!(agreement.canonical_bundle_bytes(), bundle.canonical_bytes());
    assert_eq!(agreement.transcript_bytes(), replay.rust_transcript());
}

/// The complete Phase H mint: the single private correspondence factory
/// consumes the four bridge capabilities plus the pinned abstract and
/// production Agda foundations, re-checks the complete identity surface
/// and the actual bytes, and mints the four correspondence capabilities,
/// the combined production refinement, and the combined typing
/// metatheory in one call. Adversarial variants then show a mismatched
/// verified input cannot mint. Requires the pinned local Agda runtime.
#[test]
#[ignore]
fn phase_h_factory_mints_all_six_capabilities_over_genuine_bundle() {
    let committed = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    let decoded = decode_bundle_v1(&committed).expect("committed vector decodes");
    let capabilities = genuine_capabilities();
    let payload = ProductionBundlePayloadV1 {
        contexts: decoded.contexts.clone(),
        conversions: decoded.conversions.clone(),
        conversion_typing_supplements: decoded.conversion_typing_supplements.clone(),
        synthesis_codes: decoded.synthesis_codes.clone(),
        fresh_rule_schemas: decoded.fresh_rule_schemas.clone(),
        family_payloads: decoded.family_payloads.clone(),
    };
    let bundle = build_canonical_production_bundle_v1(
        &capabilities.manifest_v3,
        &capabilities.signature,
        &capabilities.slots,
        &capabilities.bridge,
        &capabilities.delta_policy,
        &capabilities.synthesis,
        payload,
    )
    .expect("genuine canonical bundle");
    let replay = pen_semantic_audit::verify_rust_production_replay_v1(&bundle)
        .expect("independent Rust replay capability");
    let acceptance = pen_semantic_audit::verify_agda_production_acceptance_v1(&bundle, &replay)
        .expect("safe-Agda acceptance capability");
    let agreement = pen_semantic_audit::verify_production_transcript_agreement_v1(
        &bundle,
        &acceptance,
        &replay,
    )
    .expect("transcript agreement capability");
    let AuditDecision::Proven(abstract_foundation) =
        pen_semantic_audit::verify_pinned_lambda_unit_typing_foundation_v1()
    else {
        panic!("pinned abstract typing foundation");
    };
    let production_agda_foundation =
        pen_semantic_audit::diagnose_pinned_production_refinement_agda_foundation_v1()
            .expect("pinned production Agda foundation");

    let input = pen_semantic_audit::ProductionCorrespondenceFactoryInputV1 {
        manifest: &capabilities.manifest_v3,
        kernel: &capabilities.kernel,
        signature: &capabilities.signature,
        abstract_foundation: &abstract_foundation,
        production_agda_foundation: &production_agda_foundation,
        delta_policy_binding: &capabilities.delta_policy,
        bundle: &bundle,
        acceptance: &acceptance,
        replay: &replay,
        agreement: &agreement,
    };
    let minted = pen_semantic_audit::mint_production_correspondences_v1(&input)
        .expect("the single private factory mints over the genuine chain");

    // The six capabilities are mutually bound to the one manifest,
    // signature chain, and evidence core.
    let manifest_digest = capabilities.manifest_v3.candidate_digest();
    assert_eq!(
        minted.production_refinement().semantic_manifest_digest(),
        manifest_digest
    );
    assert_eq!(
        minted.production_refinement().context_correspondence(),
        minted.context_correspondence()
    );
    assert_eq!(
        minted.production_refinement().conversion_correspondence(),
        minted.conversion_correspondence()
    );
    assert_eq!(
        minted.production_refinement().synthesis_correspondence(),
        minted.synthesis_correspondence()
    );
    assert_eq!(
        minted.production_refinement().inventory_correspondence(),
        minted.inventory_correspondence()
    );
    assert_eq!(
        minted
            .context_correspondence()
            .global_slot_table()
            .digest(),
        capabilities.slots.digest()
    );
    assert_eq!(
        minted
            .conversion_correspondence()
            .predecessor_public_delta_policy()
            .digest(),
        capabilities.delta_policy.digest()
    );
    assert_eq!(
        minted
            .inventory_correspondence()
            .rust_inventory_bridge()
            .digest(),
        capabilities.bridge.digest()
    );
    assert_eq!(
        minted.synthesis_correspondence().context_correspondence(),
        minted.context_correspondence()
    );
    assert_eq!(
        minted.typing_metatheory().semantic_manifest_digest(),
        manifest_digest
    );
    assert_eq!(
        minted.typing_metatheory().kernel_protocol_digest(),
        &capabilities.kernel.kernel_protocol_digest()
    );
    assert_eq!(
        minted.typing_metatheory().synthesis_protocol_digest(),
        &pen_kernel_synthesis::synthesis_protocol_digest_v2()
    );

    // The factory is deterministic: a second mint over the same inputs
    // is the identical value.
    let reminted = pen_semantic_audit::mint_production_correspondences_v1(&input)
        .expect("deterministic remint");
    assert_eq!(reminted, minted);

    // Adversarial: a different verified signature (one extra bodyless
    // declaration) cannot mint against the genuine bundle even though
    // every capability is genuine.
    let mut extended = declarations();
    extended.push(Declaration {
        id: wire_global(5),
        ty: unit_type(),
        body: None,
    });
    let other_signature = capabilities
        .kernel
        .verify_signature(&UncheckedSignature {
            declarations: extended,
        })
        .expect("extended signature");
    let mismatched = pen_semantic_audit::ProductionCorrespondenceFactoryInputV1 {
        signature: &other_signature,
        ..input
    };
    assert!(matches!(
        pen_semantic_audit::mint_production_correspondences_v1(&mismatched),
        Err(
            pen_semantic_audit::ProductionCorrespondenceFactoryFailureV1::SignatureBindingMismatch
        )
    ));
}

/// Prints the six capability-derived digest fields as 32-byte arrays
/// for `cross_language_vectors.rs`. Run manually after any capability
/// or fixture change:
/// `cargo test --manifest-path crates/pen-semantic-audit/Cargo.toml \
///    --locked --test canonical_bundle_vectors regenerate -- --ignored --nocapture`
#[test]
#[ignore]
fn regenerate_canonical_digest_fields() {
    let capabilities = genuine_capabilities();
    let hex_bytes = |digest: &Digest| -> Vec<u8> {
        let hex = digest.as_str().strip_prefix("blake3:").expect("prefix");
        hex.as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                u8::from_str_radix(std::str::from_utf8(pair).expect("utf8"), 16).expect("hex")
            })
            .collect()
    };
    let print_field = |name: &str, digest: &Digest| {
        println!("{name}: {:?}", hex_bytes(digest));
    };
    print_field(
        "semantic_manifest_digest",
        capabilities.manifest_v3.candidate_digest(),
    );
    print_field(
        "production_inventory_bridge_digest",
        capabilities.bridge.digest(),
    );
    print_field(
        "predecessor_delta_policy_binding_digest",
        capabilities.delta_policy.digest(),
    );
    print_field("signature_digest", capabilities.signature.digest());
    print_field(
        "kernel_protocol_digest",
        capabilities.slots.kernel_protocol_digest(),
    );
    print_field("global_slot_table_digest", capabilities.slots.digest());
}
