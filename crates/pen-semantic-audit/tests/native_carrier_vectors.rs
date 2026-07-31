//! Phase I vectors: the native rank-0/1/2 V3 carrier, the carrier-derived
//! root inventory, the carrier-projected canonical subject bundle, and the
//! carrier-derived typed-occurrence census.
//!
//! The chain construction mirrors `canonical_bundle_vectors.rs` (the same
//! four fixture declarations under the same predecessor story) and can be
//! extended with one genuine fresh computation equation
//! `uelim b constructor = b`, which exercises equation seeds, generic
//! equation actions, reduct roots, and the wire fresh-rule registry.
//!
//! The census itself requires the Phase H capabilities (minted under the
//! pinned Agda runtime), so the full mint is an ignored external-gate
//! test; the carrier enumeration inputs and subject-bundle projection are
//! additionally pinned by the in-crate unit tests.

use pen_kernel::{Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, Term};
use pen_kernel_synthesis::{
    BaseQ0ConversionPolicyV2, DeltaPolicyEntryV2, verify_base_q0_conversion_policy_v2,
};
use pen_semantic_audit::{
    AuditDecision, EventIdV1, GenericJudgmentV1, ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
    PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION, ProductionBundlePayloadV1, PublicDependencyUseV1,
    PublicSubjectV1, SourceNormalizedJudgmentV1, UncheckedOriginCutoffQ3RegistryV1,
    UncheckedPublicAuditInventoryV1, UncheckedPublicAvailabilityClaimV1,
    UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1,
    UncheckedPublicEventCensusV1, UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
    UncheckedSourceNormalizedDeclarationV1, build_canonical_production_bundle_v1,
    diagnose_global_slot_table_v1, diagnose_predecessor_public_delta_policy_binding_v1,
    diagnose_production_synthesis_protocol_identity_v2,
    diagnose_v3_predecessor_public_delta_policy_binding_v1, verify_production_inventory_bridge_v1,
    verify_public_audit_inventory_v1, verify_public_clause_census_v1,
    verify_public_inventory_compatibility_v2, verify_semantic_audit_lambda_unit_manifest_v1,
    verify_semantic_audit_lambda_unit_manifest_v2, verify_semantic_audit_lambda_unit_manifest_v3,
    verify_semantic_seed_base_census_v3,
};
use pen_semantic_audit::model::PublicAvailabilityV1;
use pen_semantic_audit::{
    proposed_semantic_audit_lambda_unit_manifest_v1,
    proposed_semantic_audit_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use pen_kernel::UncheckedSignature;
use pen_production_wire::decode_bundle_v1;

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
            ty: Term::UnitType,
            body: Some(Term::Unit),
        },
        Declaration {
            id: wire_global(2),
            ty: Term::UnitType,
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

/// The genuine fresh computation equation `uelim b constructor = b` over
/// the fixture declarations: owner `wire_global(4)` (uelim, bodyless),
/// constructor `wire_global(2)` (the opaque unit-typed constant), in the
/// two-entry parameter context whose oldest entry is the never-referenced
/// scrutinee variable, exactly matching the wire fresh-pattern
/// discipline.
fn fresh_equation_judgment() -> GenericJudgmentV1 {
    GenericJudgmentV1::Equation {
        context: DependentContext(vec![Term::UnitType, Term::UnitType]),
        left: Term::Apply {
            function: Box::new(Term::Apply {
                function: Box::new(Term::Global { id: wire_global(4) }),
                argument: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(Term::Global { id: wire_global(2) }),
        },
        right: Term::Var { index: 0 },
        ty: Term::UnitType,
    }
}

struct ChainV3 {
    kernel: Kernel,
    v1_manifest: pen_semantic_audit::VerifiedSemanticAuditManifestV1,
    v2_manifest: pen_semantic_audit::VerifiedSemanticAuditManifestV2,
    manifest_v3: pen_semantic_audit::VerifiedSemanticAuditManifestV3,
    inventory: pen_semantic_audit::VerifiedPublicAuditInventoryV1,
    compatibility: pen_semantic_audit::VerifiedPublicInventoryCompatibilityV2,
    public_clauses: pen_semantic_audit::VerifiedPublicClauseCensusV1,
    seed_census: pen_semantic_audit::VerifiedSemanticSeedBaseCensusV3,
    signature: pen_kernel::VerifiedSignature,
    slots: pen_semantic_audit::VerifiedGlobalSlotTableV1,
    bridge: pen_semantic_audit::VerifiedProductionInventoryBridgeV1,
    delta_policy: pen_semantic_audit::VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    synthesis: pen_semantic_audit::VerifiedProductionSynthesisProtocolIdentityV2,
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

fn source_judgment(judgment: &GenericJudgmentV1) -> SourceNormalizedJudgmentV1 {
    SourceNormalizedJudgmentV1 {
        source_identity: Digest::of_canonical(
            "pen-semantic-audit/inventory-source-judgment/v1",
            judgment,
        ),
        source: judgment.clone(),
        claimed_normalized: judgment.clone(),
    }
}

/// The verified chain for the fixture content, optionally extended with
/// the genuine fresh equation. The bodyful unit value is the sole
/// predecessor-public declaration; the successor event seals the three
/// bodyless declarations (and, when requested, the equation).
fn chain_v3(with_equation: bool) -> ChainV3 {
    let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
    let declarations = declarations();
    let predecessor_declaration = declarations[0].clone();
    let predecessor_id = predecessor_declaration.id.clone();
    let successor_ids: Vec<GlobalId> = declarations[1..]
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect();
    let predecessor_group = GlobalId(Digest::of_bytes(b"native-carrier-chain/predecessor-group"));
    let successor_group = GlobalId(Digest::of_bytes(b"native-carrier-chain/successor-group"));
    let predecessor_event = EventIdV1(Digest::of_bytes(b"native-carrier-chain/predecessor-event"));
    let successor_event = EventIdV1(Digest::of_bytes(b"native-carrier-chain/successor-event"));
    let equation = pen_semantic_audit::EquationIdV1(Digest::of_bytes(
        b"native-carrier-chain/uelim-computation",
    ));
    let predecessor_boundary = UncheckedSignature {
        declarations: vec![predecessor_declaration.clone()],
    };
    let successor_boundary = UncheckedSignature {
        declarations: declarations.clone(),
    };
    let mut added_equations = Vec::new();
    let mut equations = Vec::new();
    let mut availability = Vec::new();
    let mut dag_edges = Vec::new();
    if with_equation {
        added_equations.push(equation.clone());
        equations.push(UncheckedPublicEquationV1 {
            equation: equation.clone(),
            owner_head: wire_global(4),
            origin: successor_event.clone(),
            source_to_normal: source_judgment(&fresh_equation_judgment()),
            demand_port: None,
        });
        // The derived dependency dag contains one edge per global the
        // equation judgment mentions plus its owner head; both are
        // successor declarations, so both claims are prior exports.
        for prerequisite in [wire_global(2), wire_global(4)] {
            let dependency = PublicDependencyUseV1 {
                dependent: PublicSubjectV1::Equation {
                    equation: equation.clone(),
                },
                prerequisite: prerequisite.clone(),
            };
            availability.push(UncheckedPublicAvailabilityClaimV1 {
                dependency: dependency.clone(),
                claimed: PublicAvailabilityV1::DependencyPriorExport {
                    target: prerequisite,
                },
            });
            dag_edges.push(dependency);
        }
    }
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
            added_equations,
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
            source_to_normal: source_declaration(&predecessor_declaration),
        })
        .chain(
            declarations[1..]
                .iter()
                .map(|declaration| UncheckedPublicDeclarationV1 {
                    declaration: declaration.id.clone(),
                    origin: successor_event.clone(),
                    group: successor_group.clone(),
                    source_to_normal: source_declaration(declaration),
                }),
        )
        .collect(),
        equations,
        forced_projections: Vec::new(),
        predecessor_demand_contracts: Vec::new(),
        public_availability: availability,
        dependency_dag: UncheckedPublicDependencyDagV1 { edges: dag_edges },
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
    let AuditDecision::Proven(public_clauses) = verify_public_clause_census_v1(&inventory) else {
        panic!("public clause census");
    };
    let AuditDecision::Proven(manifest_v3) = verify_semantic_audit_lambda_unit_manifest_v3(
        &proposed_semantic_audit_lambda_unit_manifest_v3(),
    ) else {
        panic!("lambda/unit V3 manifest");
    };
    let AuditDecision::Proven(seed_census) = verify_semantic_seed_base_census_v3(
        &manifest_v3,
        &kernel,
        &compatibility,
        &inventory,
        &public_clauses,
    ) else {
        panic!("V3 seed census");
    };
    let signature = inventory.successor_boundary().clone();
    let slots = diagnose_global_slot_table_v1(&kernel, &signature).expect("slot table");
    let policy_wire = BaseQ0ConversionPolicyV2 {
        allowed_transparent_deltas: vec![DeltaPolicyEntryV2 {
            global_slot: 0,
            id: predecessor_id,
        }],
    };
    let policy =
        verify_base_q0_conversion_policy_v2(&signature, &policy_wire).expect("verified policy");
    let binding = diagnose_predecessor_public_delta_policy_binding_v1(
        &inventory,
        &compatibility,
        &signature,
        &slots,
        &policy,
    )
    .expect("predecessor-public policy binding");
    let delta_policy = diagnose_v3_predecessor_public_delta_policy_binding_v1(
        &v2_manifest,
        &manifest_v3,
        &inventory,
        &compatibility,
        &binding,
    )
    .expect("V3 delta policy transport");
    let AuditDecision::Proven(bridge) = verify_production_inventory_bridge_v1(&manifest_v3) else {
        panic!("production inventory bridge");
    };
    let synthesis =
        diagnose_production_synthesis_protocol_identity_v2().expect("synthesis identity");
    ChainV3 {
        kernel,
        v1_manifest,
        v2_manifest,
        manifest_v3,
        inventory,
        compatibility,
        public_clauses,
        seed_census,
        signature,
        slots,
        bridge,
        delta_policy,
        synthesis,
    }
}

/// Mint the Phase H capabilities over the committed fixture bundle so the
/// carrier can consume the production refinement and typing metatheory.
/// Requires the pinned local Agda runtime.
fn phase_h_capabilities(
    chain: &ChainV3,
) -> pen_semantic_audit::MintedProductionCorrespondencesV1 {
    let committed = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    let decoded = decode_bundle_v1(&committed).expect("committed vector decodes");
    let payload = ProductionBundlePayloadV1 {
        contexts: decoded.contexts.clone(),
        conversions: decoded.conversions.clone(),
        conversion_typing_supplements: decoded.conversion_typing_supplements.clone(),
        synthesis_codes: decoded.synthesis_codes.clone(),
        fresh_rule_schemas: decoded.fresh_rule_schemas.clone(),
        family_payloads: decoded.family_payloads.clone(),
    };
    let bundle = build_canonical_production_bundle_v1(
        &chain.manifest_v3,
        &chain.signature,
        &chain.slots,
        &chain.bridge,
        &chain.delta_policy,
        &chain.synthesis,
        payload,
    )
    .expect("genuine canonical fixture bundle");
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
    pen_semantic_audit::mint_production_correspondences_v1(
        &pen_semantic_audit::ProductionCorrespondenceFactoryInputV1 {
            manifest: &chain.manifest_v3,
            kernel: &chain.kernel,
            signature: &chain.signature,
            abstract_foundation: &abstract_foundation,
            production_agda_foundation: &production_agda_foundation,
            delta_policy_binding: &chain.delta_policy,
            bundle: &bundle,
            acceptance: &acceptance,
            replay: &replay,
            agreement: &agreement,
        },
    )
    .expect("Phase H factory mint over the fixture bundle")
}

/// The complete Phase I pipeline over one chain: carrier, root inventory,
/// subject bundle, and census, with structural assertions.
fn run_phase_i_pipeline(
    chain: &ChainV3,
    minted: &pen_semantic_audit::MintedProductionCorrespondencesV1,
) -> (
    pen_semantic_audit::VerifiedNativeRankInductiveCarrierV3,
    pen_semantic_audit::VerifiedCarrierRootInventoryV3,
    pen_semantic_audit::VerifiedCarrierSubjectBundleV3,
    pen_semantic_audit::VerifiedSynthesisBackedTypedOccurrenceCensusV3,
) {
    let carrier = pen_semantic_audit::diagnose_native_rank_inductive_carrier_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.compatibility,
        &chain.inventory,
        &chain.public_clauses,
        &chain.seed_census,
        minted.production_refinement(),
        minted.typing_metatheory(),
    )
    .expect("native rank-inductive carrier");
    let root_inventory = pen_semantic_audit::diagnose_carrier_root_inventory_v3(
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        &carrier,
    )
    .expect("carrier root inventory");
    let subject_bundle = pen_semantic_audit::diagnose_carrier_subject_bundle_v3(
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        &chain.slots,
        &chain.bridge,
        &chain.delta_policy,
        &chain.synthesis,
        &carrier,
    )
    .expect("carrier subject bundle");
    let AuditDecision::Proven(census) =
        pen_semantic_audit::verify_carrier_derived_typed_occurrence_census_v3(
            &chain.manifest_v3,
            &chain.kernel,
            &chain.signature,
            minted.typing_metatheory(),
            minted.production_refinement(),
            &carrier,
            &root_inventory,
            &subject_bundle,
        )
    else {
        panic!("carrier-derived typed-occurrence census");
    };
    assert_eq!(census.carrier_digest(), carrier.digest());
    assert_eq!(census.root_inventory_digest(), root_inventory.digest());
    assert_eq!(census.subject_bundle_digest(), subject_bundle.digest());
    assert_eq!(
        census.satisfied_prerequisites(),
        &pen_semantic_audit::SYNTHESIS_BACKED_OCCURRENCE_CENSUS_PREREQUISITES_V3
    );
    assert_eq!(census.batch().roots().len(), root_inventory.roots().len());
    (carrier, root_inventory, subject_bundle, census)
}

/// Full Phase I mint over the genuine equationless chain and the
/// fresh-equation chain, with an adversarial cross-chain binding
/// rejection. Requires the pinned local Agda runtime.
#[test]
#[ignore]
fn phase_i_carrier_census_mints_over_genuine_chains() {
    let chain = chain_v3(false);
    let minted = phase_h_capabilities(&chain);
    let (carrier, root_inventory, subject_bundle, _census) =
        run_phase_i_pipeline(&chain, &minted);

    // The equationless carrier: four seed families plus every admissible
    // application through rank two (uelim partial applications at rank
    // one and their saturations at rank two, under each registered
    // context witness).
    assert_eq!(carrier.families().len(), 24);
    assert_eq!(
        carrier
            .families()
            .iter()
            .filter(|family| family.rank == 0)
            .count(),
        4
    );
    assert!(carrier.families().iter().all(|family| family.rank <= 2));
    // Distinct context witnesses can generate families with identical
    // judgments; the root inventory deduplicates them.
    assert!(root_inventory.roots().len() <= carrier.families().len());
    assert!(root_inventory.roots().len() >= 10);
    assert!(root_inventory.roots().iter().all(|root| matches!(
        root.kind(),
        pen_semantic_audit::CarrierRootKindV3::Subject
    )));

    // The subject bundle is a genuine canonical bundle distinct from the
    // committed correspondence fixture.
    let committed = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    assert_ne!(subject_bundle.bundle().canonical_bytes(), committed.as_slice());

    // The full cross-language gate generalizes to the carrier-projected
    // subject bundle: independent replay, pinned-Agda acceptance, exact
    // transcript agreement, and the Phase H factory all accept it.
    let subject_replay =
        pen_semantic_audit::verify_rust_production_replay_v1(subject_bundle.bundle())
            .expect("subject bundle replay capability");
    let subject_acceptance = pen_semantic_audit::verify_agda_production_acceptance_v1(
        subject_bundle.bundle(),
        &subject_replay,
    )
    .expect("subject bundle safe-Agda acceptance");
    let subject_agreement = pen_semantic_audit::verify_production_transcript_agreement_v1(
        subject_bundle.bundle(),
        &subject_acceptance,
        &subject_replay,
    )
    .expect("subject bundle transcript agreement");
    let AuditDecision::Proven(abstract_foundation) =
        pen_semantic_audit::verify_pinned_lambda_unit_typing_foundation_v1()
    else {
        panic!("pinned abstract typing foundation");
    };
    let production_agda_foundation =
        pen_semantic_audit::diagnose_pinned_production_refinement_agda_foundation_v1()
            .expect("pinned production Agda foundation");
    pen_semantic_audit::mint_production_correspondences_v1(
        &pen_semantic_audit::ProductionCorrespondenceFactoryInputV1 {
            manifest: &chain.manifest_v3,
            kernel: &chain.kernel,
            signature: &chain.signature,
            abstract_foundation: &abstract_foundation,
            production_agda_foundation: &production_agda_foundation,
            delta_policy_binding: &chain.delta_policy,
            bundle: subject_bundle.bundle(),
            acceptance: &subject_acceptance,
            replay: &subject_replay,
            agreement: &subject_agreement,
        },
    )
    .expect("Phase H factory mint over the carrier subject bundle");

    // The fresh-equation chain: equation seeds, generic equation actions,
    // and reduct-bearing equation roots, with the wire fresh-rule
    // registry populated.
    let equation_chain = chain_v3(true);
    let equation_minted = phase_h_capabilities(&equation_chain);
    let (equation_carrier, equation_roots, equation_bundle, _equation_census) =
        run_phase_i_pipeline(&equation_chain, &equation_minted);
    assert!(equation_carrier.families().len() > carrier.families().len());
    assert!(equation_carrier.families().iter().any(|family| matches!(
        family.constructor,
        pen_semantic_audit::FamilyConstructorV1::PublicEquationSeed { .. }
    )));
    assert!(equation_carrier.families().iter().any(|family| matches!(
        family.constructor,
        pen_semantic_audit::FamilyConstructorV1::GenericEquationAction { .. }
    )));
    assert!(equation_roots.roots().iter().any(|root| matches!(
        root.kind(),
        pen_semantic_audit::CarrierRootKindV3::EquationWithReducts
    )));
    let decoded_subject = decode_bundle_v1(equation_bundle.bundle().canonical_bytes())
        .expect("equation subject bundle decodes");
    assert_eq!(decoded_subject.fresh_rule_schemas.len(), 1);
    assert!(
        decoded_subject
            .family_payloads
            .iter()
            .any(|payload| matches!(
                payload,
                pen_production_wire::FamilyPayloadWireV1::Seed {
                    source: pen_production_wire::SeedSourceWireV1::PublicEquation { .. },
                    ..
                }
            ))
    );
    // Under the native seed derivation every generic equation action is
    // judged in a grown amalgamated context, which the wire's action
    // payload cannot carry: the actions are registered as
    // wire-inexpressible, never silently dropped, and their equation
    // roots (with reducts) stay inside the root inventory and census.
    assert!(!equation_bundle.wire_inexpressible().is_empty());
    assert!(equation_bundle.wire_inexpressible().iter().all(
        |(_, reason)| matches!(
            reason,
            pen_semantic_audit::WireInexpressibleReasonV3::ContextGrowingEquationAction
        )
    ));
    assert!(
        decoded_subject
            .family_payloads
            .iter()
            .all(|payload| !matches!(
                payload,
                pen_production_wire::FamilyPayloadWireV1::GenericEquationAction { .. }
            ))
    );

    // Adversarial cross-chain binding: a root inventory and subject
    // bundle from the equation carrier cannot mint a census against the
    // equationless carrier, and vice versa.
    assert!(matches!(
        pen_semantic_audit::verify_carrier_derived_typed_occurrence_census_v3(
            &chain.manifest_v3,
            &chain.kernel,
            &chain.signature,
            equation_minted.typing_metatheory(),
            equation_minted.production_refinement(),
            &carrier,
            &equation_roots,
            &subject_bundle,
        ),
        AuditDecision::Unknown(_)
    ));
}

/// The natively derived seed wires exhaust the registered rank-0/1/2
/// enumeration (pure Rust; no pinned gate): four seed families close to
/// exactly twenty-four raw families, including the certified
/// `ArgumentTypeMismatch` negative for the dependent type-family head —
/// the classification the fixture content originally exposed as an
/// engine gap.
#[test]
fn native_seed_wires_enumerate_the_rank_two_carrier() {
    use pen_semantic_audit::model::{
        HeadPresentationV1, LocalRoleV1, PublicHeadSeedV1, SemanticSchemaSeedV1,
    };
    let chain = chain_v3(false);
    let mut seeds = Vec::new();
    for declaration in chain.inventory.declarations() {
        let clause_id = chain
            .public_clauses
            .clause_for_declaration(declaration.declaration())
            .expect("clause for declaration");
        let clause = chain
            .public_clauses
            .clauses()
            .iter()
            .find(|candidate| candidate.id() == clause_id)
            .expect("clause");
        let source_judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Global {
                id: declaration.declaration().clone(),
            },
            ty: declaration.source().ty.clone(),
        };
        let presentation = if declaration.source().body.is_some() {
            HeadPresentationV1::TransparentDefinition
        } else {
            HeadPresentationV1::Opaque
        };
        seeds.push(SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
            declaration: declaration.declaration().clone(),
            origin_event: declaration.origin().clone(),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: declaration.source_identity().clone(),
                source: source_judgment.clone(),
                claimed_normalized: source_judgment,
            },
            presentation,
            claimed_role: LocalRoleV1::KernelHead,
            public_support: clause.public_support().clone(),
            source_clause: Some(clause.id().clone()),
        }));
    }
    for seed in &seeds {
        assert!(matches!(
            pen_semantic_audit::verify_pre_q0_semantic_seed_v1(
                &chain.kernel,
                &chain.signature,
                &chain.v1_manifest,
                seed,
            ),
            AuditDecision::Proven(_)
        ));
    }
    let AuditDecision::Proven(carrier) = pen_semantic_audit::enumerate_pre_q0_raw_families_v1(
        &chain.kernel,
        &chain.signature,
        &chain.v1_manifest,
        &seeds,
        &[],
    ) else {
        panic!("natively derived seeds must exhaust the rank-two carrier");
    };
    assert_eq!(carrier.verified_seeds().len(), 4);
    assert_eq!(carrier.raw_families().len(), 24);
    assert!(
        carrier
            .raw_families()
            .iter()
            .all(|family| family.rank <= 2)
    );
    assert!(!carrier.tuple_dispositions().is_empty());
}
