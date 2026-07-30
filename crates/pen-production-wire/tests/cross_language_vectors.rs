//! Rust-side pinning of the committed cross-language byte vectors.
//!
//! `LawV2/Wire/BundleDecodeTestV1.agda` embeds one canonical bundle byte
//! vector and proves by refl that safe Agda accepts it and rejects seven
//! pinned length-preserving mutations. This test makes those claims
//! reproducible from the repository alone: it rebuilds the same fixture,
//! re-derives the exact bytes, checks them against the committed Agda
//! literal, and re-asserts every pinned Rust verdict.
//!
//! This is a development regression test, not correspondence authority.

use pen_production_wire::*;

fn id(byte: u8) -> WireIdV1 {
    WireIdV1([byte; 32])
}

fn canonical_bundle() -> ProductionRefinementBundleV1 {
    let empty = ProductionContextWireV1::default();
    let one_local = ProductionContextWireV1 {
        entries_oldest_first: vec![WireTermV1::UnitType],
    };
    let common = WireTermV1::Unit;
    ProductionRefinementBundleV1 {
        header: WireHeaderV1::canonical(),
        manifest_surface: V3CorrespondenceManifestWireV1 {
            semantic_schema_version: V3_SEMANTIC_SCHEMA_VERSION,
            profile_id: V3_SEMANTIC_PROFILE_ID.to_vec(),
            semantic_manifest_digest: id(20),
            authority: ManifestAuthorityWireV1::GenericPrototypeOnly,
            frozen: false,
            live_profile_a_access: false,
            production_inventory_bridge_digest: id(21),
            public_universe_levels: PUBLIC_UNIVERSE_LEVELS_V1.to_vec(),
            checker_universe_levels: CHECKER_UNIVERSE_LEVELS_V1.to_vec(),
            formation_witness_levels: FORMATION_WITNESS_LEVELS_V1.to_vec(),
            maximum_context_entries: 32,
            synthesis_rule_inventory: EXACT_SYNTHESIS_INVENTORY_V1.to_vec(),
            predecessor_delta_policy_binding_digest: id(22),
            synthesis_protocol_id: SYNTHESIS_PROTOCOL_ID_V2.to_vec(),
            synthesis_schema_version: SYNTHESIS_SCHEMA_VERSION_V2,
        },
        signature: ProductionSignatureWireV1 {
            signature_digest: id(23),
            kernel_protocol_digest: id(24),
            global_slot_table_digest: id(25),
            allowed_transparent_deltas: vec![DeltaPolicyEntryWireV1 {
                global_slot: 0,
                global_id_bytes: id(1),
            }],
        },
        global_slot_table: GlobalSlotTableWireV1 {
            entries: vec![
                GlobalSlotEntryWireV1 {
                    slot: 0,
                    global_id_bytes: id(1),
                    declaration_type: WireTermV1::UnitType,
                    declaration_body: Some(WireTermV1::Unit),
                },
                GlobalSlotEntryWireV1 {
                    slot: 1,
                    global_id_bytes: id(2),
                    declaration_type: WireTermV1::GlobalSlot { slot: 0 },
                    declaration_body: None,
                },
            ],
        },
        contexts: vec![empty.clone(), one_local.clone()],
        conversions: vec![ConversionCertificateWireV1 {
            conversion_id: id(5),
            context: empty.clone(),
            left: WireTermV1::GlobalSlot { slot: 0 },
            right: common.clone(),
            endpoint_judgment: EndpointJudgmentWireV1::HasType {
                expected_type: WireTermV1::UnitType,
            },
            common_normal_form: common.clone(),
            left_trace: BaseQ0ReductionTraceWireV1 {
                start: WireTermV1::GlobalSlot { slot: 0 },
                steps: vec![BaseQ0ReductionStepWireV1::TransparentDelta {
                    source: WireTermV1::GlobalSlot { slot: 0 },
                    target: common.clone(),
                    global_slot: 0,
                }],
                end: common.clone(),
            },
            right_trace: BaseQ0ReductionTraceWireV1 {
                start: common.clone(),
                steps: Vec::new(),
                end: common.clone(),
            },
            no_redex_census: NoRedexCensusWireV1 {
                entries: vec![NoRedexEntryWireV1 {
                    path: Vec::new(),
                    term: common,
                    disposition: NoRedexDispositionWireV1::Unit,
                }],
            },
        }],
        conversion_typing_supplements: Vec::new(),
        synthesis_codes: vec![SynthesisCertificateWireV1 {
            synthesis_id: id(6),
            context: one_local.clone(),
            subject: WireTermV1::Variable { index: 0 },
            inferred_type: WireTermV1::UnitType,
            code: SynthesisCodeWireV1::VariableLookup {
                index: 0,
                context_ordinal: 0,
                shift_distance: 1,
            },
        }],
        q0_inventory: Q0InventoryWireV1 {
            ordered_rules: EXACT_Q0_INVENTORY_V1.to_vec(),
        },
        fresh_rule_schemas: vec![FreshRuleSchemaWireV1 {
            equation_id: id(7),
            owner_slot: 0,
            constructor_slot: 1,
            parameter_context: one_local,
            left: WireTermV1::Apply {
                function: Box::new(WireTermV1::GlobalSlot { slot: 0 }),
                argument: Box::new(WireTermV1::GlobalSlot { slot: 1 }),
            },
            right: WireTermV1::Variable { index: 0 },
            ty: WireTermV1::UnitType,
            scrutinee_ordinal: 0,
            arity: 1,
        }],
        family_inventory: FamilyInventoryWireV1 {
            ordered_codes: EXACT_FAMILY_INVENTORY_V1.to_vec(),
        },
        family_payloads: vec![
            FamilyPayloadWireV1::Seed {
                family_id: id(8),
                source: SeedSourceWireV1::PublicHead { owner_slot: 0 },
                judgment: FamilyJudgmentWireV1 {
                    context: empty.clone(),
                    subject: WireTermV1::Unit,
                    ty: WireTermV1::UnitType,
                },
            },
            FamilyPayloadWireV1::Seed {
                family_id: id(9),
                source: SeedSourceWireV1::PublicEquation { equation_id: id(7) },
                judgment: FamilyJudgmentWireV1 {
                    context: empty.clone(),
                    subject: WireTermV1::Unit,
                    ty: WireTermV1::UnitType,
                },
            },
            FamilyPayloadWireV1::GenericPublicApplication {
                family_id: id(10),
                function_family_id: id(8),
                argument_family_id: id(9),
                judgment: FamilyJudgmentWireV1 {
                    context: empty.clone(),
                    subject: WireTermV1::Unit,
                    ty: WireTermV1::UnitType,
                },
            },
            FamilyPayloadWireV1::GenericEquationAction {
                family_id: id(11),
                equation_id: id(7),
                source_family_id: id(8),
                judgment: FamilyJudgmentWireV1 {
                    context: empty,
                    subject: WireTermV1::Unit,
                    ty: WireTermV1::UnitType,
                },
            },
        ],
    }
}

/// The committed safe Agda vector module, resolved relative to this crate.
const AGDA_VECTOR_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/BundleDecodeTestV1.agda"
);

fn committed_agda_vector() -> Vec<u8> {
    let source = std::fs::read_to_string(AGDA_VECTOR_MODULE)
        .expect("committed Agda vector module must be readable");
    let mut bytes = Vec::new();
    let mut in_literal = false;
    for line in source.lines() {
        if line.trim_end() == "canonical-vector-v1 =" {
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
    assert!(in_literal, "canonical-vector-v1 literal not found");
    bytes
}

/// The pinned length-preserving mutations mirrored by the Agda module.
/// Offsets and values must stay identical to `BundleDecodeTestV1.agda`.
const PINNED_MUTATIONS: [(&str, usize, u8); 6] = [
    ("magic-flip", 0, 81),
    ("manifest-frozen", 105, 1),
    ("public-universe-level", 226, 2),
    ("unknown-term-tag", 478, 255),
    ("synthesis-variable-scope", 752, 1),
    ("q0-swap-first", 787, 1),
];

#[test]
fn canonical_vector_matches_committed_agda_literal() {
    let bytes = encode_bundle_v1(&canonical_bundle()).expect("fixture encodes");
    assert_eq!(bytes.len(), 1258, "pinned vector length");
    assert_eq!(
        bytes,
        committed_agda_vector(),
        "the committed Agda byte literal must equal the canonical encoding"
    );
    assert_eq!(decode_bundle_v1(&bytes).as_ref(), Ok(&canonical_bundle()));
}

#[test]
fn pinned_mutations_are_rejected_by_rust() {
    let bytes = encode_bundle_v1(&canonical_bundle()).expect("fixture encodes");
    for (name, offset, value) in PINNED_MUTATIONS {
        let mut mutated = bytes.clone();
        assert_ne!(
            mutated[offset], value,
            "{name}: mutation must change the byte"
        );
        mutated[offset] = value;
        assert!(
            decode_bundle_v1(&mutated).is_err(),
            "{name}: Rust must reject the pinned mutation"
        );
    }

    // The Q0 swap is a two-byte mutation: rules 0 and 1 exchanged.
    let mut q0 = bytes.clone();
    assert_eq!((q0[787], q0[788]), (0, 1));
    q0[787] = 1;
    q0[788] = 0;
    assert!(decode_bundle_v1(&q0).is_err());

    // Truncation by one byte, mirrored by `drop-last` in Agda.
    let mut truncated = bytes;
    truncated.pop();
    assert!(decode_bundle_v1(&truncated).is_err());
}
