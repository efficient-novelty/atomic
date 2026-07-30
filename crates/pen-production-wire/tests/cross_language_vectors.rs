//! Rust-side pinning of the committed cross-language byte vectors.
//!
//! `LawV2/Wire/BundleDecodeTestV1.agda` embeds one canonical bundle byte
//! vector and proves by refl that safe Agda accepts it and rejects
//! pinned mutations. `SemanticReplayTestV1.agda` and
//! `TypingReplayTestV1.agda` embed structurally valid mutants that the
//! semantic replay and the typing replay reject. This test makes those
//! claims reproducible from the repository alone: it rebuilds the same
//! fixture, re-derives the exact bytes, checks them against the
//! committed Agda literals, and re-asserts every pinned Rust verdict.
//!
//! The fixture is deliberately well typed under the kernel discipline
//! (every declared type forms a type, every stored body checks, every
//! context entry is a type) and deliberately discriminating: it
//! contains congruence conversions with binder-local supplements, a
//! `TypeFormation` supplement with a recovered formation level, an
//! application certificate whose raw instantiation differs from its
//! kernel-normalized dependent result, a well-typed fresh computation
//! rule `uelim b unit = b` under a bodyless owner, and a six-payload
//! family chain whose equation action records a genuine rewrite of
//! the saturated application's subject.
//!
//! This is a development regression test, not correspondence authority.

use pen_production_wire::*;

fn id(byte: u8) -> WireIdV1 {
    WireIdV1([byte; 32])
}

// The six capability-derived digest fields of the genuine canonical
// bundle. Source of truth: the ignored `regenerate_canonical_digest_fields`
// test in `pen-semantic-audit/tests/canonical_bundle_vectors.rs`, which
// constructs the complete verified-input chain (kernel signature, slot
// table, exact V3 manifest, inventory bridge, predecessor-public delta
// policy, synthesis identity) and prints these arrays; the
// `canonical_vector_is_builder_derived` test in the same file requires
// the committed canonical vector to equal the builder output over that
// chain byte-for-byte, so a drift in any of these constants fails there.
const SEMANTIC_MANIFEST_DIGEST: [u8; 32] = [
    58, 26, 157, 119, 153, 25, 35, 203, 195, 71, 124, 197, 21, 119, 46, 37, 13, 108, 130, 113, 60,
    114, 125, 80, 249, 20, 34, 5, 252, 178, 148, 117,
];
const PRODUCTION_INVENTORY_BRIDGE_DIGEST: [u8; 32] = [
    218, 203, 112, 45, 148, 239, 116, 185, 67, 221, 90, 121, 81, 68, 174, 9, 52, 188, 88, 164,
    164, 239, 205, 160, 74, 88, 161, 232, 147, 21, 73, 21,
];
const PREDECESSOR_DELTA_POLICY_BINDING_DIGEST: [u8; 32] = [
    253, 200, 242, 108, 153, 215, 143, 23, 61, 238, 152, 220, 186, 19, 115, 67, 18, 114, 137, 30,
    241, 43, 193, 231, 66, 251, 175, 118, 129, 160, 131, 166,
];
const SIGNATURE_DIGEST: [u8; 32] = [
    197, 237, 173, 116, 246, 112, 104, 99, 198, 17, 110, 169, 180, 227, 156, 141, 204, 141, 61,
    178, 117, 33, 165, 51, 164, 112, 236, 162, 89, 87, 10, 112,
];
const KERNEL_PROTOCOL_DIGEST: [u8; 32] = [
    186, 126, 100, 158, 249, 65, 126, 240, 91, 247, 249, 159, 145, 184, 158, 13, 240, 58, 49, 117,
    226, 63, 24, 100, 197, 40, 154, 124, 70, 57, 210, 197,
];
const GLOBAL_SLOT_TABLE_DIGEST: [u8; 32] = [
    173, 244, 151, 202, 88, 73, 235, 182, 128, 4, 72, 215, 45, 97, 213, 44, 204, 179, 83, 202,
    107, 251, 220, 116, 45, 243, 186, 85, 181, 191, 110, 40,
];

fn unit_type() -> WireTermV1 {
    WireTermV1::UnitType
}

fn unit() -> WireTermV1 {
    WireTermV1::Unit
}

fn sort(level: u16) -> WireTermV1 {
    WireTermV1::Sort { level }
}

fn var(index: u32) -> WireTermV1 {
    WireTermV1::Variable { index }
}

fn global(slot: u32) -> WireTermV1 {
    WireTermV1::GlobalSlot { slot }
}

fn pi(parameter: WireTermV1, body: WireTermV1) -> WireTermV1 {
    WireTermV1::Pi {
        parameter: Box::new(parameter),
        body: Box::new(body),
    }
}

fn lambda(parameter: WireTermV1, body: WireTermV1) -> WireTermV1 {
    WireTermV1::Lambda {
        parameter: Box::new(parameter),
        body: Box::new(body),
    }
}

fn apply(function: WireTermV1, argument: WireTermV1) -> WireTermV1 {
    WireTermV1::Apply {
        function: Box::new(function),
        argument: Box::new(argument),
    }
}

fn beta_redex() -> WireTermV1 {
    apply(lambda(unit_type(), var(0)), unit())
}

fn unit_pi() -> WireTermV1 {
    pi(unit_type(), unit_type())
}

/// `(F : UnitType -> Sort 0) -> F Unit`: the declared type of global
/// slot 2. Its neutral dependent body makes the raw instantiation of an
/// application differ from the kernel-normalized dependent result.
fn family_application_pi() -> WireTermV1 {
    pi(pi(unit_type(), sort(0)), apply(var(0), unit()))
}

fn unit_family_pi() -> WireTermV1 {
    pi(unit_type(), sort(0))
}

/// `(b : UnitType) -> (u : UnitType) -> UnitType`: the declared type of
/// the bodyless fresh head at slot 3. The scrutinee is the final
/// telescope parameter, matching the kernel normalizer's
/// `expected_scrutinee = arity - 1` discipline.
fn uelim_pi() -> WireTermV1 {
    pi(unit_type(), pi(unit_type(), unit_type()))
}

fn constant_unit_family() -> WireTermV1 {
    lambda(unit_type(), unit_type())
}

fn empty_context() -> ProductionContextWireV1 {
    ProductionContextWireV1::default()
}

fn one_local() -> ProductionContextWireV1 {
    ProductionContextWireV1 {
        entries_oldest_first: vec![unit_type()],
    }
}

/// `[scrutinee : UnitType, b : UnitType]` oldest-first: the fresh
/// rule's parameter context. Under the wire pattern convention the
/// scrutinee variable is the oldest entry (it never occurs on the left,
/// where the constructor sits in its spine position) and the branch
/// parameter `b` is `Variable 0`.
fn two_locals() -> ProductionContextWireV1 {
    ProductionContextWireV1 {
        entries_oldest_first: vec![unit_type(), unit_type()],
    }
}

/// The genuine well-typed fresh computation rule `uelim b unit = b`:
/// owner slot 3 (`uelim`, bodyless), constructor slot 1 (the opaque
/// unit-typed constant), left `uelim b constructor`, right `b`, type
/// `UnitType`. Both sides check against the type under the parameter
/// context, the pattern is left-linear with the constructor in the
/// scrutinee position, and the right side never mentions the owner.
fn fresh_unit_eliminator_rule() -> FreshRuleSchemaWireV1 {
    FreshRuleSchemaWireV1 {
        equation_id: id(7),
        owner_slot: 3,
        constructor_slot: 1,
        parameter_context: two_locals(),
        left: apply(apply(global(3), var(0)), global(1)),
        right: var(0),
        ty: unit_type(),
        scrutinee_ordinal: 1,
        arity: 2,
    }
}

/// The six family payloads form one genuine derivation chain, all in
/// the empty context: two seeds (the bodyful unit constant and the
/// fresh equation's owner head), a partial application, the
/// constructor seed, the saturated application — whose subject is
/// exactly the fresh rule's left-hand side instantiated with
/// `b := global 0` — and the equation action rewriting that subject to
/// `global 0` with its type preserved.
fn family_payloads() -> Vec<FamilyPayloadWireV1> {
    vec![
        FamilyPayloadWireV1::Seed {
            family_id: id(8),
            source: SeedSourceWireV1::PublicHead { owner_slot: 0 },
            judgment: FamilyJudgmentWireV1 {
                context: empty_context(),
                subject: global(0),
                ty: unit_type(),
            },
        },
        FamilyPayloadWireV1::Seed {
            family_id: id(9),
            source: SeedSourceWireV1::PublicEquation { equation_id: id(7) },
            judgment: FamilyJudgmentWireV1 {
                context: empty_context(),
                subject: global(3),
                ty: uelim_pi(),
            },
        },
        FamilyPayloadWireV1::GenericPublicApplication {
            family_id: id(10),
            function_family_id: id(9),
            argument_family_id: id(8),
            judgment: FamilyJudgmentWireV1 {
                context: empty_context(),
                subject: apply(global(3), global(0)),
                ty: unit_pi(),
            },
        },
        FamilyPayloadWireV1::Seed {
            family_id: id(35),
            source: SeedSourceWireV1::PublicHead { owner_slot: 1 },
            judgment: FamilyJudgmentWireV1 {
                context: empty_context(),
                subject: global(1),
                ty: unit_type(),
            },
        },
        FamilyPayloadWireV1::GenericPublicApplication {
            family_id: id(36),
            function_family_id: id(10),
            argument_family_id: id(35),
            judgment: FamilyJudgmentWireV1 {
                context: empty_context(),
                subject: apply(apply(global(3), global(0)), global(1)),
                ty: unit_type(),
            },
        },
        FamilyPayloadWireV1::GenericEquationAction {
            family_id: id(37),
            equation_id: id(7),
            source_family_id: id(36),
            judgment: FamilyJudgmentWireV1 {
                context: empty_context(),
                subject: global(0),
                ty: unit_type(),
            },
        },
    ]
}

/// Recompute the exact no-redex census of a normal form relative to the
/// enabled delta slot 0, mirroring `expected_no_redex_census`.
fn census_of(term: &WireTermV1) -> Vec<NoRedexEntryWireV1> {
    fn walk(
        term: &WireTermV1,
        path: &mut Vec<ConversionPathComponentWireV1>,
        entries: &mut Vec<NoRedexEntryWireV1>,
    ) {
        use ConversionPathComponentWireV1 as Path;
        use NoRedexDispositionWireV1 as Disposition;
        let disposition = match term {
            WireTermV1::Sort { .. } => Disposition::Sort,
            WireTermV1::Variable { .. } => Disposition::Variable,
            WireTermV1::GlobalSlot { .. } => Disposition::GlobalNotEnabledByPolicy,
            WireTermV1::Pi { .. } => Disposition::Pi,
            WireTermV1::Lambda { .. } => Disposition::Lambda,
            WireTermV1::Apply { .. } => Disposition::NeutralApplication,
            WireTermV1::UnitType => Disposition::UnitType,
            WireTermV1::Unit => Disposition::Unit,
        };
        entries.push(NoRedexEntryWireV1 {
            path: path.clone(),
            term: term.clone(),
            disposition,
        });
        match term {
            WireTermV1::Pi { parameter, body } => {
                path.push(Path::PiParameter);
                walk(parameter, path, entries);
                path.pop();
                path.push(Path::PiBody);
                walk(body, path, entries);
                path.pop();
            }
            WireTermV1::Lambda { parameter, body } => {
                path.push(Path::LambdaParameter);
                walk(parameter, path, entries);
                path.pop();
                path.push(Path::LambdaBody);
                walk(body, path, entries);
                path.pop();
            }
            WireTermV1::Apply { function, argument } => {
                path.push(Path::ApplyFunction);
                walk(function, path, entries);
                path.pop();
                path.push(Path::ApplyArgument);
                walk(argument, path, entries);
                path.pop();
            }
            _ => {}
        }
    }
    let mut entries = Vec::new();
    walk(term, &mut Vec::new(), &mut entries);
    entries
}

fn identity_conversion(
    conversion_id: WireIdV1,
    context: ProductionContextWireV1,
    term: WireTermV1,
    endpoint_judgment: EndpointJudgmentWireV1,
) -> ConversionCertificateWireV1 {
    ConversionCertificateWireV1 {
        conversion_id,
        context,
        left: term.clone(),
        right: term.clone(),
        endpoint_judgment,
        common_normal_form: term.clone(),
        left_trace: BaseQ0ReductionTraceWireV1 {
            start: term.clone(),
            steps: Vec::new(),
            end: term.clone(),
        },
        right_trace: BaseQ0ReductionTraceWireV1 {
            start: term.clone(),
            steps: Vec::new(),
            end: term.clone(),
        },
        no_redex_census: NoRedexCensusWireV1 {
            entries: census_of(&term),
        },
    }
}

/// A one-step conversion whose right side is already the common form.
fn one_step_conversion(
    conversion_id: WireIdV1,
    context: ProductionContextWireV1,
    left: WireTermV1,
    step: BaseQ0ReductionStepWireV1,
    common: WireTermV1,
    endpoint_judgment: EndpointJudgmentWireV1,
) -> ConversionCertificateWireV1 {
    ConversionCertificateWireV1 {
        conversion_id,
        context,
        left: left.clone(),
        right: common.clone(),
        endpoint_judgment,
        common_normal_form: common.clone(),
        left_trace: BaseQ0ReductionTraceWireV1 {
            start: left,
            steps: vec![step],
            end: common.clone(),
        },
        right_trace: BaseQ0ReductionTraceWireV1 {
            start: common.clone(),
            steps: Vec::new(),
            end: common.clone(),
        },
        no_redex_census: NoRedexCensusWireV1 {
            entries: census_of(&common),
        },
    }
}

fn variable_lookup_code(index: u32, context_len: u32) -> SynthesisCodeWireV1 {
    SynthesisCodeWireV1::VariableLookup {
        index,
        context_ordinal: context_len - 1 - index,
        shift_distance: index + 1,
    }
}

/// `(\y : UnitType. y) x` under `[UnitType]`: the lambda-body congruence
/// premise certified by supplement A.
fn inner_beta_redex() -> WireTermV1 {
    apply(lambda(unit_type(), var(0)), var(0))
}

/// `(\y : UnitType. UnitType) x` under `[UnitType]`: the pi-body
/// congruence premise certified by supplement B.
fn inner_type_redex() -> WireTermV1 {
    apply(constant_unit_family(), var(0))
}

fn conversions() -> Vec<ConversionCertificateWireV1> {
    vec![
        // id 5: transparent delta of the bodyful slot 0.
        one_step_conversion(
            id(5),
            empty_context(),
            global(0),
            BaseQ0ReductionStepWireV1::TransparentDelta {
                source: global(0),
                target: unit(),
                global_slot: 0,
            },
            unit(),
            EndpointJudgmentWireV1::HasType {
                expected_type: unit_type(),
            },
        ),
        // id 12/13: identity mediators for certificate 15 (empty context).
        identity_conversion(
            id(12),
            empty_context(),
            unit_pi(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
        identity_conversion(
            id(13),
            empty_context(),
            unit_type(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
        // id 14: a genuine top-level beta conversion.
        one_step_conversion(
            id(14),
            empty_context(),
            beta_redex(),
            BaseQ0ReductionStepWireV1::Beta {
                source: beta_redex(),
                target: unit(),
            },
            unit(),
            EndpointJudgmentWireV1::HasType {
                expected_type: unit_type(),
            },
        ),
        // id 16/17: identity mediators under `[UnitType]` for the
        // supplement certificates.
        identity_conversion(
            id(16),
            one_local(),
            unit_pi(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
        identity_conversion(
            id(17),
            one_local(),
            unit_type(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
        // id 18: a lambda-body congruence with a binder-local beta
        // premise, certified by supplement A.
        one_step_conversion(
            id(18),
            empty_context(),
            lambda(unit_type(), inner_beta_redex()),
            BaseQ0ReductionStepWireV1::LambdaBodyCongruence {
                source: lambda(unit_type(), inner_beta_redex()),
                target: lambda(unit_type(), var(0)),
                premise: Box::new(BaseQ0ReductionStepWireV1::Beta {
                    source: inner_beta_redex(),
                    target: var(0),
                }),
            },
            lambda(unit_type(), var(0)),
            EndpointJudgmentWireV1::HasType {
                expected_type: unit_pi(),
            },
        ),
        // id 19: a pi-body congruence with a binder-local type-level
        // beta premise, certified by supplement B (TypeFormation with a
        // recovered formation level).
        one_step_conversion(
            id(19),
            empty_context(),
            pi(unit_type(), inner_type_redex()),
            BaseQ0ReductionStepWireV1::PiBodyCongruence {
                source: pi(unit_type(), inner_type_redex()),
                target: unit_pi(),
                premise: Box::new(BaseQ0ReductionStepWireV1::Beta {
                    source: inner_type_redex(),
                    target: unit_type(),
                }),
            },
            unit_pi(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
        // id 26: identity mediator on the unit family pi under
        // `[UnitType]`, for supplement B's application certificate.
        identity_conversion(
            id(26),
            one_local(),
            unit_family_pi(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
        // id 27/28: identity mediators in the empty context for the
        // dependent-normalization certificate 29.
        identity_conversion(
            id(27),
            empty_context(),
            family_application_pi(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
        identity_conversion(
            id(28),
            empty_context(),
            unit_family_pi(),
            EndpointJudgmentWireV1::TypeFormation,
        ),
    ]
}

fn synthesis_codes() -> Vec<SynthesisCertificateWireV1> {
    vec![
        SynthesisCertificateWireV1 {
            synthesis_id: id(6),
            context: one_local(),
            subject: var(0),
            inferred_type: unit_type(),
            code: variable_lookup_code(0, 1),
        },
        SynthesisCertificateWireV1 {
            synthesis_id: id(15),
            context: empty_context(),
            subject: beta_redex(),
            inferred_type: unit_type(),
            code: SynthesisCodeWireV1::ApplicationElimination {
                function: Box::new(SynthesisCodeWireV1::LambdaIntroduction {
                    parameter_type: Box::new(SynthesisCodeWireV1::UnitType),
                    body: Box::new(variable_lookup_code(0, 1)),
                }),
                argument: Box::new(SynthesisCodeWireV1::Unit),
                function_conversion_id: id(12),
                argument_conversion_id: id(13),
                dependent_result_type: unit_type(),
            },
        },
        // id 29: `global 2` applied to the constant unit family. The raw
        // instantiation is `(\y:UnitType. UnitType) Unit`, a beta redex;
        // the recorded dependent result is its kernel normal form
        // `UnitType`, pinning the bounded-normalization reconciliation.
        SynthesisCertificateWireV1 {
            synthesis_id: id(29),
            context: empty_context(),
            subject: apply(global(2), constant_unit_family()),
            inferred_type: unit_type(),
            code: SynthesisCodeWireV1::ApplicationElimination {
                function: Box::new(SynthesisCodeWireV1::GlobalLookup { global_slot: 2 }),
                argument: Box::new(SynthesisCodeWireV1::LambdaIntroduction {
                    parameter_type: Box::new(SynthesisCodeWireV1::UnitType),
                    body: Box::new(SynthesisCodeWireV1::UnitType),
                }),
                function_conversion_id: id(27),
                argument_conversion_id: id(28),
                dependent_result_type: unit_type(),
            },
        },
    ]
}

/// Supplement A: the binder-local beta premise of conversion 18, under
/// the derived context `[UnitType]`, with a `HasType UnitType` endpoint.
fn supplement_a() -> ConversionTypingSupplementWireV1 {
    ConversionTypingSupplementWireV1 {
        conversion_id: id(18),
        step_path: vec![0, 0, 0],
        local_context: one_local(),
        local_endpoint_judgment: EndpointJudgmentWireV1::HasType {
            expected_type: unit_type(),
        },
        source_typing_code: SynthesisCertificateWireV1 {
            synthesis_id: id(30),
            context: one_local(),
            subject: inner_beta_redex(),
            inferred_type: unit_type(),
            code: SynthesisCodeWireV1::ApplicationElimination {
                function: Box::new(SynthesisCodeWireV1::LambdaIntroduction {
                    parameter_type: Box::new(SynthesisCodeWireV1::UnitType),
                    body: Box::new(variable_lookup_code(0, 2)),
                }),
                argument: Box::new(variable_lookup_code(0, 1)),
                function_conversion_id: id(16),
                argument_conversion_id: id(17),
                dependent_result_type: unit_type(),
            },
        },
        target_typing_code: SynthesisCertificateWireV1 {
            synthesis_id: id(31),
            context: one_local(),
            subject: var(0),
            inferred_type: unit_type(),
            code: variable_lookup_code(0, 1),
        },
        formation_level: None,
    }
}

/// Supplement B: the binder-local type-level premise of conversion 19,
/// with a `TypeFormation` endpoint and recovered formation level 0.
fn supplement_b() -> ConversionTypingSupplementWireV1 {
    ConversionTypingSupplementWireV1 {
        conversion_id: id(19),
        step_path: vec![0, 0, 0],
        local_context: one_local(),
        local_endpoint_judgment: EndpointJudgmentWireV1::TypeFormation,
        source_typing_code: SynthesisCertificateWireV1 {
            synthesis_id: id(32),
            context: one_local(),
            subject: inner_type_redex(),
            inferred_type: sort(0),
            code: SynthesisCodeWireV1::ApplicationElimination {
                function: Box::new(SynthesisCodeWireV1::LambdaIntroduction {
                    parameter_type: Box::new(SynthesisCodeWireV1::UnitType),
                    body: Box::new(SynthesisCodeWireV1::UnitType),
                }),
                argument: Box::new(variable_lookup_code(0, 1)),
                function_conversion_id: id(26),
                argument_conversion_id: id(17),
                dependent_result_type: sort(0),
            },
        },
        target_typing_code: SynthesisCertificateWireV1 {
            synthesis_id: id(33),
            context: one_local(),
            subject: unit_type(),
            inferred_type: sort(0),
            code: SynthesisCodeWireV1::UnitType,
        },
        formation_level: Some(0),
    }
}

fn canonical_bundle() -> ProductionRefinementBundleV1 {
    // Discriminating and well-typed context: a universe entry, a
    // variable entry (a type because its own type is that universe),
    // and an under-binder pi entry, so the transcript's oldest-first
    // ordinal formula, entry selection, shift iteration count, and
    // under-binder shift cutoff are byte-visible while every entry
    // still forms a type under the kernel discipline.
    let discriminating = ProductionContextWireV1 {
        entries_oldest_first: vec![sort(0), var(0), pi(var(1), var(2))],
    };
    ProductionRefinementBundleV1 {
        header: WireHeaderV1::canonical(),
        manifest_surface: V3CorrespondenceManifestWireV1 {
            semantic_schema_version: V3_SEMANTIC_SCHEMA_VERSION,
            profile_id: V3_SEMANTIC_PROFILE_ID.to_vec(),
            semantic_manifest_digest: WireIdV1(SEMANTIC_MANIFEST_DIGEST),
            authority: ManifestAuthorityWireV1::GenericPrototypeOnly,
            frozen: false,
            live_profile_a_access: false,
            production_inventory_bridge_digest: WireIdV1(PRODUCTION_INVENTORY_BRIDGE_DIGEST),
            public_universe_levels: PUBLIC_UNIVERSE_LEVELS_V1.to_vec(),
            checker_universe_levels: CHECKER_UNIVERSE_LEVELS_V1.to_vec(),
            formation_witness_levels: FORMATION_WITNESS_LEVELS_V1.to_vec(),
            maximum_context_entries: 32,
            synthesis_rule_inventory: EXACT_SYNTHESIS_INVENTORY_V1.to_vec(),
            predecessor_delta_policy_binding_digest: WireIdV1(
                PREDECESSOR_DELTA_POLICY_BINDING_DIGEST,
            ),
            synthesis_protocol_id: SYNTHESIS_PROTOCOL_ID_V2.to_vec(),
            synthesis_schema_version: SYNTHESIS_SCHEMA_VERSION_V2,
        },
        signature: ProductionSignatureWireV1 {
            signature_digest: WireIdV1(SIGNATURE_DIGEST),
            kernel_protocol_digest: WireIdV1(KERNEL_PROTOCOL_DIGEST),
            global_slot_table_digest: WireIdV1(GLOBAL_SLOT_TABLE_DIGEST),
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
                    declaration_type: unit_type(),
                    declaration_body: Some(unit()),
                },
                GlobalSlotEntryWireV1 {
                    slot: 1,
                    global_id_bytes: id(2),
                    declaration_type: unit_type(),
                    declaration_body: None,
                },
                GlobalSlotEntryWireV1 {
                    slot: 2,
                    global_id_bytes: id(3),
                    declaration_type: family_application_pi(),
                    declaration_body: None,
                },
                GlobalSlotEntryWireV1 {
                    slot: 3,
                    global_id_bytes: id(4),
                    declaration_type: uelim_pi(),
                    declaration_body: None,
                },
            ],
        },
        contexts: vec![empty_context(), one_local(), discriminating],
        conversions: conversions(),
        conversion_typing_supplements: vec![supplement_a(), supplement_b()],
        synthesis_codes: synthesis_codes(),
        q0_inventory: Q0InventoryWireV1 {
            ordered_rules: EXACT_Q0_INVENTORY_V1.to_vec(),
        },
        fresh_rule_schemas: vec![fresh_unit_eliminator_rule()],
        family_inventory: FamilyInventoryWireV1 {
            ordered_codes: EXACT_FAMILY_INVENTORY_V1.to_vec(),
        },
        family_payloads: family_payloads(),
    }
}

// --- Semantic-layer mutants -------------------------------------------------

/// Rewrite every `Unit` endpoint of a one-step conversion to
/// `UnitType`, coherently, so the mutant stays structurally valid while
/// the step no longer replays semantically.
fn replace_unit_with_unit_type(conversion: &mut ConversionCertificateWireV1) {
    conversion.right = unit_type();
    conversion.common_normal_form = unit_type();
    match &mut conversion.left_trace.steps[0] {
        BaseQ0ReductionStepWireV1::Beta { target, .. }
        | BaseQ0ReductionStepWireV1::TransparentDelta { target, .. } => {
            *target = unit_type();
        }
        _ => panic!("unexpected step shape"),
    }
    conversion.left_trace.end = unit_type();
    conversion.right_trace.start = unit_type();
    conversion.right_trace.end = unit_type();
    conversion.no_redex_census.entries = census_of(&unit_type());
}

fn mutant_delta_wrong_body() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    replace_unit_with_unit_type(&mut bundle.conversions[0]);
    bundle
}

fn mutant_beta_wrong_result() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    replace_unit_with_unit_type(&mut bundle.conversions[3]);
    bundle
}

fn mutant_lookup_wrong_type() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.synthesis_codes[0].inferred_type = sort(0);
    bundle
}

/// The dependent result of certificate 29 recorded as the RAW
/// instantiation instead of its kernel normal form: structurally valid
/// (the wire only requires `dependent_result_type == inferred_type`),
/// rejected by the reconciled bounded-normalization rule.
fn mutant_result_not_normalized() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    let raw = apply(constant_unit_family(), unit());
    bundle.synthesis_codes[2].inferred_type = raw.clone();
    match &mut bundle.synthesis_codes[2].code {
        SynthesisCodeWireV1::ApplicationElimination {
            dependent_result_type,
            ..
        } => *dependent_result_type = raw,
        _ => panic!("unexpected code shape"),
    }
    bundle
}

// --- Typing-layer mutants ---------------------------------------------------

/// Slot 1 declared with the type `global 0`, whose own type is
/// `UnitType`, not a universe: exactly the ill-typed declaration shape
/// the kernel signature verification rejects.
fn mutant_signature_ill_typed() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.global_slot_table.entries[1].declaration_type = global(0);
    bundle
}

/// A standalone context whose second entry is a term of `UnitType`
/// rather than a type: rejected by kernel context verification.
fn mutant_context_ill_typed() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.contexts[2] = ProductionContextWireV1 {
        entries_oldest_first: vec![unit_type(), var(0), pi(var(1), var(2))],
    };
    bundle
}

/// Conversion 14's endpoint claims the wrong expected type: the
/// intermediates check against `UnitType`, not against the unit pi.
fn mutant_endpoint_wrong_expected() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.conversions[3].endpoint_judgment = EndpointJudgmentWireV1::HasType {
        expected_type: unit_pi(),
    };
    bundle
}

/// Supplement A's step path addresses the outer congruence step rather
/// than its premise, so the path fails to resolve and the premise
/// coverage of conversion 18 is broken.
fn mutant_supplement_step_path() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.conversion_typing_supplements[0].step_path = vec![0, 0];
    bundle
}

/// Supplement B recovers the wrong formation level: its certificates
/// record `Sort 0`, not `Sort 1`.
fn mutant_supplement_formation_level() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.conversion_typing_supplements[1].formation_level = Some(1);
    bundle
}

/// All supplements dropped: every congruence premise loses its
/// exactly-one coverage.
fn mutant_supplement_missing() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.conversion_typing_supplements = Vec::new();
    bundle
}

/// Supplement A claims a coherent but wrong binder-local context (the
/// unit pi instead of `UnitType`): the derived-context comparison
/// rejects it.
fn mutant_supplement_derived_context() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    let wrong = ProductionContextWireV1 {
        entries_oldest_first: vec![unit_pi()],
    };
    bundle.conversion_typing_supplements[0].local_context = wrong.clone();
    bundle.conversion_typing_supplements[0]
        .source_typing_code
        .context = wrong.clone();
    bundle.conversion_typing_supplements[0]
        .target_typing_code
        .context = wrong;
    bundle
}

// --- Inventory-layer mutants ------------------------------------------------
//
// Every inventory mutant is structurally valid, invisible to the
// semantic replay (which covers only conversions and synthesis), and
// invisible to the typing replay (which covers the signature, standalone
// contexts, conversions, synthesis, and supplements). Only the Phase F
// inventory layer inspects sections 9 and 11 semantically.

/// The good rule with its type changed to `Sort 0`: the pattern is
/// intact but neither side checks against the recorded type.
fn mutant_fresh_ill_typed() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    bundle.fresh_rule_schemas[0].ty = sort(0);
    bundle
}

/// The old fixture shape: owner slot 0 is bodyful (and delta-enabled),
/// which the kernel normalizer forbids for a fresh head. The pattern
/// stays exact, so every earlier layer accepts it.
fn mutant_fresh_owner_bodyful() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    let rule = &mut bundle.fresh_rule_schemas[0];
    rule.owner_slot = 0;
    rule.left = apply(apply(global(0), var(0)), global(1));
    bundle
}

/// Two rules for the same `(owner, constructor)` pair under distinct
/// equation identities: structural dedup is by equation id only, but
/// the inventory layer requires pairwise-disjoint constructor patterns.
fn mutant_fresh_duplicate_pair() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    let mut duplicate = fresh_unit_eliminator_rule();
    duplicate.equation_id = id(34);
    bundle.fresh_rule_schemas.push(duplicate);
    bundle
}

/// The old degenerate seed shape: subject `Unit` instead of the public
/// head `global 0`. It still types, but the seed judgment no longer
/// binds the declared head.
fn mutant_seed_subject_mismatch() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    match &mut bundle.family_payloads[0] {
        FamilyPayloadWireV1::Seed { judgment, .. } => judgment.subject = unit(),
        _ => panic!("unexpected payload shape"),
    }
    bundle
}

/// A seed type that is merely CONVERTIBLE to the declared type: the
/// beta redex `(\y:UnitType. UnitType) Unit` normalizes to `UnitType`,
/// so the typed replay accepts it, but the inventory layer demands the
/// exact declared type.
fn mutant_seed_type_convertible() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    match &mut bundle.family_payloads[0] {
        FamilyPayloadWireV1::Seed { judgment, .. } => {
            judgment.ty = apply(constant_unit_family(), unit());
        }
        _ => panic!("unexpected payload shape"),
    }
    bundle
}

/// An application family whose subject applies the wrong argument: it
/// still types (`global 1` is also unit-typed), but the subject is not
/// the application of its components' subjects.
fn mutant_application_subject_mismatch() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    match &mut bundle.family_payloads[2] {
        FamilyPayloadWireV1::GenericPublicApplication { judgment, .. } => {
            judgment.subject = apply(global(3), global(1));
        }
        _ => panic!("unexpected payload shape"),
    }
    bundle
}

/// An application family judged in a different context than its
/// components: everything is closed so the typed replay accepts it,
/// but the shared-context discipline (the wire image of the dropped
/// `context_witness`) is broken.
fn mutant_application_context_mismatch() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    match &mut bundle.family_payloads[2] {
        FamilyPayloadWireV1::GenericPublicApplication { judgment, .. } => {
            judgment.context = one_local();
        }
        _ => panic!("unexpected payload shape"),
    }
    bundle
}

/// An equation action that changes the judged type: subject and type
/// are replaced by a coherent but different judgment
/// (`UnitType : Sort 0`), breaking type preservation against the
/// source family.
fn mutant_action_type_changed() -> ProductionRefinementBundleV1 {
    let mut bundle = canonical_bundle();
    match &mut bundle.family_payloads[5] {
        FamilyPayloadWireV1::GenericEquationAction { judgment, .. } => {
            judgment.subject = unit_type();
            judgment.ty = sort(0);
        }
        _ => panic!("unexpected payload shape"),
    }
    bundle
}

// --- Committed Agda literals ------------------------------------------------

const AGDA_VECTOR_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/BundleDecodeTestV1.agda"
);
const AGDA_TRANSCRIPT_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/ContextTranscriptTestV1.agda"
);
const AGDA_SEMANTIC_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/SemanticReplayTestV1.agda"
);
const AGDA_TYPING_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/TypingReplayTestV1.agda"
);
const AGDA_INVENTORY_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../pen-semantic-audit/agda/LawV2/Wire/InventoryReplayTestV1.agda"
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

fn committed_agda_vector() -> Vec<u8> {
    committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =")
}

// --- Envelope offset derivation ---------------------------------------------

/// Payload byte offsets of the eleven sections, derived by walking the
/// canonical envelope, so the pinned decode-layer mutation offsets stay
/// correct when the fixture changes.
fn section_payload_offsets(bytes: &[u8]) -> Vec<(u8, usize, usize)> {
    let mut sections = Vec::new();
    let mut cursor = 16 + 2 + 2;
    while cursor < bytes.len() {
        let tag = bytes[cursor];
        let mut length_bytes = [0u8; 8];
        length_bytes.copy_from_slice(&bytes[cursor + 1..cursor + 9]);
        let length = u64::from_le_bytes(length_bytes) as usize;
        sections.push((tag, cursor + 9, length));
        cursor += 9 + length;
    }
    sections
}

fn payload_offset(bytes: &[u8], tag: u8) -> usize {
    section_payload_offsets(bytes)
        .into_iter()
        .find(|(section, _, _)| *section == tag)
        .map(|(_, offset, _)| offset)
        .expect("section present")
}

/// The pinned decode-layer mutations, derived from the canonical bytes:
/// (name, offset, replacement value, decode-level rejection).
fn pinned_mutations(bytes: &[u8]) -> Vec<(&'static str, usize, u8, bool)> {
    // Manifest layout prefix is fixed: magic 16, version 2, count 2,
    // section header 9, schema 2, profile length 8, profile 33,
    // digest 32 -> the reserved byte at 104 and frozen flag at 105.
    let manifest = payload_offset(bytes, 1);
    let frozen = manifest + 2 + 8 + 33 + 32 + 1;
    // Public universe levels follow frozen, live, two 32-byte digests,
    // the synthesis protocol string (8 + 35), the synthesis schema u16,
    // and the list count u64; the second u16 entry's low byte then sits
    // one past the first entry.
    let public_level_one = frozen + 1 + 1 + 32 + 32 + 8 + 35 + 2 + 8 + 2;
    // Global slot table: count u64, then slot u32 + id 32 -> the first
    // declaration-type tag.
    let first_declaration_tag = payload_offset(bytes, 3) + 8 + 4 + 32;
    // Synthesis section: count u64, certificate id 32, context count
    // u64 + one UnitType entry, subject tag -> the variable index low
    // byte follows the subject tag.
    let synthesis_variable_index = payload_offset(bytes, 7) + 8 + 32 + 8 + 1 + 1;
    // Q0 inventory: count u64, then the seven rule tags.
    let q0_first_rule = payload_offset(bytes, 8) + 8;
    vec![
        ("magic-flip", 0, 81, true),
        ("manifest-frozen", frozen, 1, false),
        ("public-universe-level", public_level_one, 2, false),
        ("unknown-term-tag", first_declaration_tag, 255, true),
        ("synthesis-variable-scope", synthesis_variable_index, 1, false),
        ("q0-swap-first", q0_first_rule, 1, false),
    ]
}

#[test]
fn canonical_vector_matches_committed_agda_literal() {
    let bytes = encode_bundle_v1(&canonical_bundle()).expect("fixture encodes");
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
    for (name, offset, value, _parse_level) in pinned_mutations(&bytes) {
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
    let q0_first = payload_offset(&bytes, 8) + 8;
    let mut q0 = bytes.clone();
    assert_eq!((q0[q0_first], q0[q0_first + 1]), (0, 1));
    q0[q0_first] = 1;
    q0[q0_first + 1] = 0;
    assert!(decode_bundle_v1(&q0).is_err());

    // Truncation by one byte, mirrored by `drop-last` in Agda.
    let mut truncated = bytes;
    truncated.pop();
    assert!(decode_bundle_v1(&truncated).is_err());
}

// --- Transcript rendering ---------------------------------------------------

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

// --- Structurally-valid mutant pinning --------------------------------------

/// The semantic mutants are STRUCTURALLY valid in Rust — the validator
/// inside `encode_bundle_v1` accepts every one — while the committed
/// Agda module proves each fails the semantic replay.
#[test]
fn semantic_mutants_are_structurally_valid_and_pinned() {
    for (bundle, header) in [
        (mutant_delta_wrong_body(), "mutant-delta-wrong-body-v1 ="),
        (mutant_beta_wrong_result(), "mutant-beta-wrong-result-v1 ="),
        (mutant_lookup_wrong_type(), "mutant-lookup-wrong-type-v1 ="),
        (
            mutant_result_not_normalized(),
            "mutant-result-not-normalized-v1 =",
        ),
    ] {
        let bytes = encode_bundle_v1(&bundle).expect("semantic mutant must stay structurally valid");
        assert_eq!(
            bytes,
            committed_agda_literal(AGDA_SEMANTIC_MODULE, header),
            "committed semantic mutant literal must match: {header}"
        );
    }
}

/// The typing mutants are structurally valid AND pass the semantic
/// replay layer; the committed Agda typing module proves each fails the
/// typing replay.
#[test]
fn typing_mutants_are_structurally_valid_and_pinned() {
    for (bundle, header) in [
        (
            mutant_signature_ill_typed(),
            "mutant-signature-ill-typed-v1 =",
        ),
        (mutant_context_ill_typed(), "mutant-context-ill-typed-v1 ="),
        (
            mutant_endpoint_wrong_expected(),
            "mutant-endpoint-wrong-expected-v1 =",
        ),
        (
            mutant_supplement_step_path(),
            "mutant-supplement-step-path-v1 =",
        ),
        (
            mutant_supplement_formation_level(),
            "mutant-supplement-formation-level-v1 =",
        ),
        (
            mutant_supplement_missing(),
            "mutant-supplement-missing-v1 =",
        ),
        (
            mutant_supplement_derived_context(),
            "mutant-supplement-derived-context-v1 =",
        ),
    ] {
        let bytes = encode_bundle_v1(&bundle).expect("typing mutant must stay structurally valid");
        assert_eq!(
            bytes,
            committed_agda_literal(AGDA_TYPING_MODULE, header),
            "committed typing mutant literal must match: {header}"
        );
    }
}

/// The inventory mutants are structurally valid AND pass both the
/// semantic replay and the typing replay; the committed Agda inventory
/// module proves each fails the Phase F inventory layer.
#[test]
fn inventory_mutants_are_structurally_valid_and_pinned() {
    for (bundle, header) in [
        (mutant_fresh_ill_typed(), "mutant-fresh-ill-typed-v1 ="),
        (mutant_fresh_owner_bodyful(), "mutant-fresh-owner-bodyful-v1 ="),
        (
            mutant_fresh_duplicate_pair(),
            "mutant-fresh-duplicate-pair-v1 =",
        ),
        (
            mutant_seed_subject_mismatch(),
            "mutant-seed-subject-mismatch-v1 =",
        ),
        (
            mutant_seed_type_convertible(),
            "mutant-seed-type-convertible-v1 =",
        ),
        (
            mutant_application_subject_mismatch(),
            "mutant-application-subject-mismatch-v1 =",
        ),
        (
            mutant_application_context_mismatch(),
            "mutant-application-context-mismatch-v1 =",
        ),
        (mutant_action_type_changed(), "mutant-action-type-changed-v1 ="),
    ] {
        let bytes =
            encode_bundle_v1(&bundle).expect("inventory mutant must stay structurally valid");
        assert_eq!(
            bytes,
            committed_agda_literal(AGDA_INVENTORY_MODULE, header),
            "committed inventory mutant literal must match: {header}"
        );
    }
}

// --- Literal regeneration helper --------------------------------------------

fn agda_literal(bytes: &[u8]) -> String {
    let mut out = String::new();
    for chunk in bytes.chunks(12) {
        out.push_str("  ");
        for byte in chunk {
            out.push_str(&format!("{byte} \u{2237} "));
        }
        out.push('\n');
    }
    out.push_str("  []\n");
    out
}

/// Prints every committed Agda literal for this fixture. Run manually
/// after a fixture change:
/// `cargo test --manifest-path crates/pen-production-wire/Cargo.toml \
///    regenerate_agda_literals -- --ignored --nocapture`
#[test]
#[ignore]
fn regenerate_agda_literals() {
    let canonical = encode_bundle_v1(&canonical_bundle()).expect("fixture encodes");
    println!("-- canonical-vector-v1 ({} bytes)", canonical.len());
    println!("canonical-vector-v1 =");
    print!("{}", agda_literal(&canonical));
    let transcript = context_global_transcript(&canonical_bundle());
    println!("-- context-transcript-v1 ({} bytes)", transcript.len());
    println!("context-transcript-v1 =");
    print!("{}", agda_literal(&transcript));
    for (bundle, name) in [
        (mutant_delta_wrong_body(), "mutant-delta-wrong-body-v1"),
        (mutant_beta_wrong_result(), "mutant-beta-wrong-result-v1"),
        (mutant_lookup_wrong_type(), "mutant-lookup-wrong-type-v1"),
        (
            mutant_result_not_normalized(),
            "mutant-result-not-normalized-v1",
        ),
        (mutant_signature_ill_typed(), "mutant-signature-ill-typed-v1"),
        (mutant_context_ill_typed(), "mutant-context-ill-typed-v1"),
        (
            mutant_endpoint_wrong_expected(),
            "mutant-endpoint-wrong-expected-v1",
        ),
        (
            mutant_supplement_step_path(),
            "mutant-supplement-step-path-v1",
        ),
        (
            mutant_supplement_formation_level(),
            "mutant-supplement-formation-level-v1",
        ),
        (mutant_supplement_missing(), "mutant-supplement-missing-v1"),
        (
            mutant_supplement_derived_context(),
            "mutant-supplement-derived-context-v1",
        ),
        (mutant_fresh_ill_typed(), "mutant-fresh-ill-typed-v1"),
        (mutant_fresh_owner_bodyful(), "mutant-fresh-owner-bodyful-v1"),
        (
            mutant_fresh_duplicate_pair(),
            "mutant-fresh-duplicate-pair-v1",
        ),
        (
            mutant_seed_subject_mismatch(),
            "mutant-seed-subject-mismatch-v1",
        ),
        (
            mutant_seed_type_convertible(),
            "mutant-seed-type-convertible-v1",
        ),
        (
            mutant_application_subject_mismatch(),
            "mutant-application-subject-mismatch-v1",
        ),
        (
            mutant_application_context_mismatch(),
            "mutant-application-context-mismatch-v1",
        ),
        (mutant_action_type_changed(), "mutant-action-type-changed-v1"),
    ] {
        let bytes = encode_bundle_v1(&bundle).expect("mutant encodes");
        println!("-- {name} ({} bytes)", bytes.len());
        println!("{name} =");
        print!("{}", agda_literal(&bytes));
    }
    println!("-- pinned decode-layer mutation offsets");
    for (name, offset, value, parse_level) in pinned_mutations(&canonical) {
        println!("--   {name}: set-at {offset} {value} (parse-level {parse_level})");
    }
    let q0_first = payload_offset(&canonical, 8) + 8;
    println!("--   q0-swap: set-at {} 1 then set-at {} 0", q0_first, q0_first + 1);
}
