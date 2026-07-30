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

fn beta_redex() -> WireTermV1 {
    WireTermV1::Apply {
        function: Box::new(WireTermV1::Lambda {
            parameter: Box::new(WireTermV1::UnitType),
            body: Box::new(WireTermV1::Variable { index: 0 }),
        }),
        argument: Box::new(WireTermV1::Unit),
    }
}

fn unit_pi() -> WireTermV1 {
    WireTermV1::Pi {
        parameter: Box::new(WireTermV1::UnitType),
        body: Box::new(WireTermV1::UnitType),
    }
}

fn identity_conversion(
    conversion_id: WireIdV1,
    term: WireTermV1,
    census: Vec<NoRedexEntryWireV1>,
) -> ConversionCertificateWireV1 {
    ConversionCertificateWireV1 {
        conversion_id,
        context: ProductionContextWireV1::default(),
        left: term.clone(),
        right: term.clone(),
        endpoint_judgment: EndpointJudgmentWireV1::TypeFormation,
        common_normal_form: term.clone(),
        left_trace: BaseQ0ReductionTraceWireV1 {
            start: term.clone(),
            steps: Vec::new(),
            end: term.clone(),
        },
        right_trace: BaseQ0ReductionTraceWireV1 {
            start: term.clone(),
            steps: Vec::new(),
            end: term,
        },
        no_redex_census: NoRedexCensusWireV1 { entries: census },
    }
}

/// The semantic exercises: identity conversions mediating the
/// application elimination, and one genuine beta conversion whose
/// target the Agda semantic replay recomputes by instantiation.
fn semantic_conversions() -> Vec<ConversionCertificateWireV1> {
    use ConversionPathComponentWireV1 as Path;
    use NoRedexDispositionWireV1 as Disposition;
    vec![
        identity_conversion(
            id(12),
            unit_pi(),
            vec![
                NoRedexEntryWireV1 {
                    path: Vec::new(),
                    term: unit_pi(),
                    disposition: Disposition::Pi,
                },
                NoRedexEntryWireV1 {
                    path: vec![Path::PiParameter],
                    term: WireTermV1::UnitType,
                    disposition: Disposition::UnitType,
                },
                NoRedexEntryWireV1 {
                    path: vec![Path::PiBody],
                    term: WireTermV1::UnitType,
                    disposition: Disposition::UnitType,
                },
            ],
        ),
        identity_conversion(
            id(13),
            WireTermV1::UnitType,
            vec![NoRedexEntryWireV1 {
                path: Vec::new(),
                term: WireTermV1::UnitType,
                disposition: Disposition::UnitType,
            }],
        ),
        ConversionCertificateWireV1 {
            conversion_id: id(14),
            context: ProductionContextWireV1::default(),
            left: beta_redex(),
            right: WireTermV1::Unit,
            endpoint_judgment: EndpointJudgmentWireV1::HasType {
                expected_type: WireTermV1::UnitType,
            },
            common_normal_form: WireTermV1::Unit,
            left_trace: BaseQ0ReductionTraceWireV1 {
                start: beta_redex(),
                steps: vec![BaseQ0ReductionStepWireV1::Beta {
                    source: beta_redex(),
                    target: WireTermV1::Unit,
                }],
                end: WireTermV1::Unit,
            },
            right_trace: BaseQ0ReductionTraceWireV1 {
                start: WireTermV1::Unit,
                steps: Vec::new(),
                end: WireTermV1::Unit,
            },
            no_redex_census: NoRedexCensusWireV1 {
                entries: vec![NoRedexEntryWireV1 {
                    path: Vec::new(),
                    term: WireTermV1::Unit,
                    disposition: Disposition::Unit,
                }],
            },
        },
    ]
}

/// Rewrite every `Unit` endpoint of a one-step conversion to
/// `UnitType`, coherently, so the mutant stays structurally valid while
/// the step no longer replays semantically.
fn replace_unit_with_unit_type(conversion: &mut ConversionCertificateWireV1) {
    conversion.right = WireTermV1::UnitType;
    conversion.common_normal_form = WireTermV1::UnitType;
    match &mut conversion.left_trace.steps[0] {
        BaseQ0ReductionStepWireV1::Beta { target, .. }
        | BaseQ0ReductionStepWireV1::TransparentDelta { target, .. } => {
            *target = WireTermV1::UnitType;
        }
        _ => panic!("unexpected step shape"),
    }
    conversion.left_trace.end = WireTermV1::UnitType;
    conversion.right_trace.start = WireTermV1::UnitType;
    conversion.right_trace.end = WireTermV1::UnitType;
    conversion.no_redex_census.entries = vec![NoRedexEntryWireV1 {
        path: Vec::new(),
        term: WireTermV1::UnitType,
        disposition: NoRedexDispositionWireV1::UnitType,
    }];
}

fn canonical_bundle() -> ProductionRefinementBundleV1 {
    let empty = ProductionContextWireV1::default();
    let one_local = ProductionContextWireV1 {
        entries_oldest_first: vec![WireTermV1::UnitType],
    };
    // Discriminating context: a variable entry and an under-binder Pi
    // entry make the transcript's oldest-first ordinal formula, entry
    // selection, shift iteration count, and shift cutoff byte-visible.
    let discriminating = ProductionContextWireV1 {
        entries_oldest_first: vec![
            WireTermV1::UnitType,
            WireTermV1::Variable { index: 0 },
            WireTermV1::Pi {
                parameter: Box::new(WireTermV1::Variable { index: 1 }),
                body: Box::new(WireTermV1::Variable { index: 0 }),
            },
        ],
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
        contexts: vec![empty.clone(), one_local.clone(), discriminating],
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
        }]
        .into_iter()
        .chain(semantic_conversions())
        .collect(),
        conversion_typing_supplements: Vec::new(),
        synthesis_codes: vec![
            SynthesisCertificateWireV1 {
                synthesis_id: id(6),
                context: one_local.clone(),
                subject: WireTermV1::Variable { index: 0 },
                inferred_type: WireTermV1::UnitType,
                code: SynthesisCodeWireV1::VariableLookup {
                    index: 0,
                    context_ordinal: 0,
                    shift_distance: 1,
                },
            },
            SynthesisCertificateWireV1 {
                synthesis_id: id(15),
                context: ProductionContextWireV1::default(),
                subject: beta_redex(),
                inferred_type: WireTermV1::UnitType,
                code: SynthesisCodeWireV1::ApplicationElimination {
                    function: Box::new(SynthesisCodeWireV1::LambdaIntroduction {
                        parameter_type: Box::new(SynthesisCodeWireV1::UnitType),
                        body: Box::new(SynthesisCodeWireV1::VariableLookup {
                            index: 0,
                            context_ordinal: 0,
                            shift_distance: 1,
                        }),
                    }),
                    argument: Box::new(SynthesisCodeWireV1::Unit),
                    function_conversion_id: id(12),
                    argument_conversion_id: id(13),
                    dependent_result_type: WireTermV1::UnitType,
                },
            },
        ],
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

/// The committed safe Agda vector modules, resolved relative to this crate.
const AGDA_VECTOR_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/BundleDecodeTestV1.agda"
);
const AGDA_TRANSCRIPT_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/ContextTranscriptTestV1.agda"
);

fn committed_agda_literal(path: &str, header: &str) -> Vec<u8> {
    let source = std::fs::read_to_string(path)
        .expect("committed Agda vector module must be readable");
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

fn committed_agda_vector() -> Vec<u8> {
    committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =")
}

/// The pinned length-preserving mutations mirrored by the Agda module.
/// Offsets and values must stay identical to `BundleDecodeTestV1.agda`.
const PINNED_MUTATIONS: [(&str, usize, u8); 6] = [
    ("magic-flip", 0, 81),
    ("manifest-frozen", 105, 1),
    ("public-universe-level", 226, 2),
    ("unknown-term-tag", 478, 255),
    ("synthesis-variable-scope", 1089, 1),
    ("q0-swap-first", 1256, 1),
];

#[test]
fn canonical_vector_matches_committed_agda_literal() {
    let bytes = encode_bundle_v1(&canonical_bundle()).expect("fixture encodes");
    assert_eq!(bytes.len(), 1727, "pinned vector length");
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
    assert_eq!((q0[1256], q0[1257]), (0, 1));
    q0[1256] = 1;
    q0[1257] = 0;
    assert!(decode_bundle_v1(&q0).is_err());

    // Truncation by one byte, mirrored by `drop-last` in Agda.
    let mut truncated = bytes;
    truncated.pop();
    assert!(decode_bundle_v1(&truncated).is_err());
}

fn shift_term(cutoff: u32, term: &WireTermV1) -> WireTermV1 {
    match term {
        WireTermV1::Sort { level } => WireTermV1::Sort { level: *level },
        WireTermV1::Variable { index } => WireTermV1::Variable {
            index: if *index < cutoff { *index } else { index + 1 },
        },
        WireTermV1::GlobalSlot { slot } => WireTermV1::GlobalSlot { slot: *slot },
        WireTermV1::Pi { parameter, body } => WireTermV1::Pi {
            parameter: Box::new(shift_term(cutoff, parameter)),
            body: Box::new(shift_term(cutoff + 1, body)),
        },
        WireTermV1::Lambda { parameter, body } => WireTermV1::Lambda {
            parameter: Box::new(shift_term(cutoff, parameter)),
            body: Box::new(shift_term(cutoff + 1, body)),
        },
        WireTermV1::Apply { function, argument } => WireTermV1::Apply {
            function: Box::new(shift_term(cutoff, function)),
            argument: Box::new(shift_term(cutoff, argument)),
        },
        WireTermV1::UnitType => WireTermV1::UnitType,
        WireTermV1::Unit => WireTermV1::Unit,
    }
}

fn encode_term(term: &WireTermV1, out: &mut Vec<u8>) {
    match term {
        WireTermV1::Sort { level } => {
            out.push(0);
            out.extend_from_slice(&level.to_le_bytes());
        }
        WireTermV1::Variable { index } => {
            out.push(1);
            out.extend_from_slice(&index.to_le_bytes());
        }
        WireTermV1::GlobalSlot { slot } => {
            out.push(2);
            out.extend_from_slice(&slot.to_le_bytes());
        }
        WireTermV1::Pi { parameter, body } => {
            out.push(3);
            encode_term(parameter, out);
            encode_term(body, out);
        }
        WireTermV1::Lambda { parameter, body } => {
            out.push(4);
            encode_term(parameter, out);
            encode_term(body, out);
        }
        WireTermV1::Apply { function, argument } => {
            out.push(5);
            encode_term(function, out);
            encode_term(argument, out);
        }
        WireTermV1::UnitType => out.push(6),
        WireTermV1::Unit => out.push(7),
    }
}

/// The Rust view of the context/global lookup transcript: strict-prior
/// stored declarations for globals, and shift-computed in-context types
/// for every variable of every context, with the oldest-first ordinal
/// and shift-distance metadata. `ContextTranscriptTestV1.agda` proves by
/// refl that the intrinsic Agda view renders identical bytes.
fn context_global_transcript(bundle: &ProductionRefinementBundleV1) -> Vec<u8> {
    let mut out = Vec::new();
    let globals = &bundle.global_slot_table.entries;
    out.extend_from_slice(&(globals.len() as u64).to_le_bytes());
    for entry in globals {
        out.extend_from_slice(&entry.slot.to_le_bytes());
        encode_term(&entry.declaration_type, &mut out);
        match &entry.declaration_body {
            None => out.push(0),
            Some(body) => {
                out.push(1);
                encode_term(body, &mut out);
            }
        }
    }
    out.extend_from_slice(&(bundle.contexts.len() as u64).to_le_bytes());
    for context in &bundle.contexts {
        let entries = &context.entries_oldest_first;
        let count = entries.len() as u32;
        out.extend_from_slice(&(entries.len() as u64).to_le_bytes());
        for index in 0..count {
            let oldest = count - 1 - index;
            let shift = index + 1;
            out.extend_from_slice(&index.to_le_bytes());
            out.extend_from_slice(&oldest.to_le_bytes());
            out.extend_from_slice(&shift.to_le_bytes());
            let mut looked_up = entries[oldest as usize].clone();
            for _ in 0..shift {
                looked_up = shift_term(0, &looked_up);
            }
            encode_term(&looked_up, &mut out);
        }
    }
    out
}

#[test]
fn context_transcript_matches_committed_agda_literal() {
    let transcript = context_global_transcript(&canonical_bundle());
    assert_eq!(
        transcript,
        committed_agda_literal(AGDA_TRANSCRIPT_MODULE, "context-transcript-v1 ="),
        "the committed Agda transcript literal must equal the Rust rendering"
    );
}

const AGDA_SEMANTIC_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/SemanticReplayTestV1.agda"
);

/// The Phase E semantic mutants are STRUCTURALLY valid in Rust — the
/// validator inside `encode_bundle_v1` accepts every one — while the
/// committed Agda module proves each fails the semantic replay. This
/// pins both halves of that claim from the repository alone.
#[test]
fn semantic_mutants_are_structurally_valid_and_pinned() {
    let mut delta_wrong_body = canonical_bundle();
    replace_unit_with_unit_type(&mut delta_wrong_body.conversions[0]);
    let delta_bytes = encode_bundle_v1(&delta_wrong_body)
        .expect("delta mutant must stay structurally valid");
    assert_eq!(
        delta_bytes,
        committed_agda_literal(AGDA_SEMANTIC_MODULE, "mutant-delta-wrong-body-v1 ="),
    );

    let mut beta_wrong_result = canonical_bundle();
    replace_unit_with_unit_type(&mut beta_wrong_result.conversions[3]);
    let beta_bytes = encode_bundle_v1(&beta_wrong_result)
        .expect("beta mutant must stay structurally valid");
    assert_eq!(
        beta_bytes,
        committed_agda_literal(AGDA_SEMANTIC_MODULE, "mutant-beta-wrong-result-v1 ="),
    );

    let mut lookup_wrong_type = canonical_bundle();
    lookup_wrong_type.synthesis_codes[0].inferred_type = WireTermV1::Sort { level: 0 };
    let lookup_bytes = encode_bundle_v1(&lookup_wrong_type)
        .expect("lookup mutant must stay structurally valid");
    assert_eq!(
        lookup_bytes,
        committed_agda_literal(AGDA_SEMANTIC_MODULE, "mutant-lookup-wrong-type-v1 ="),
    );
}
