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

use pen_kernel::UncheckedSignature;
use pen_kernel::{Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, Term};
use pen_kernel_synthesis::{
    BaseQ0ConversionPolicyV2, DeltaPolicyEntryV2, verify_base_q0_conversion_policy_v2,
};
use pen_production_wire::decode_bundle_v1;
use pen_semantic_audit::model::PublicAvailabilityV1;
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
    diagnose_v3_predecessor_public_delta_policy_binding_v1,
    verify_kernel_cost_lambda_unit_manifest_v2, verify_production_inventory_bridge_v1,
    verify_public_audit_inventory_v1, verify_public_clause_census_v1,
    verify_public_inventory_compatibility_v2, verify_semantic_audit_lambda_unit_manifest_v1,
    verify_semantic_audit_lambda_unit_manifest_v2, verify_semantic_audit_lambda_unit_manifest_v3,
    verify_semantic_seed_base_census_v3,
};
use pen_semantic_audit::{
    proposed_kernel_cost_lambda_unit_manifest_v2, proposed_semantic_audit_lambda_unit_manifest_v1,
    proposed_semantic_audit_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};

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
/// the fixture declarations, in the restricted normalizer's exact rule
/// form: the prefix parameter context `[b : UnitType]` (the scrutinee
/// slot is consumed by the constructor in the spine, not bound), left
/// `uelim b constructor`, right `b`, type `UnitType`. Owner
/// `wire_global(4)` (uelim, bodyless), constructor `wire_global(2)`
/// (the opaque unit-typed constant).
fn fresh_equation_judgment() -> GenericJudgmentV1 {
    GenericJudgmentV1::Equation {
        context: DependentContext(vec![Term::UnitType]),
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
    cost_manifest: pen_semantic_audit::VerifiedCostManifestV2,
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
/// the genuine fresh equation.
///
/// Without the equation, the bodyful unit value is the sole
/// predecessor-public declaration and the successor event seals the
/// three bodyless declarations. With the equation, the act structure
/// follows the restricted normalizer's fresh-program discipline: the
/// predecessor history seals the unit value and then the two opaque
/// constants, and the successor event seals exactly one new
/// declaration — the fresh owner `uelim` — together with its sealed
/// computation equation.
fn chain_v3(with_equation: bool) -> ChainV3 {
    let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
    let declarations = declarations();
    let predecessor_declaration = declarations[0].clone();
    let predecessor_id = predecessor_declaration.id.clone();
    let predecessor_group = GlobalId(Digest::of_bytes(b"native-carrier-chain/predecessor-group"));
    let successor_group = GlobalId(Digest::of_bytes(b"native-carrier-chain/successor-group"));
    let predecessor_event = EventIdV1(Digest::of_bytes(b"native-carrier-chain/predecessor-event"));
    let successor_event = EventIdV1(Digest::of_bytes(b"native-carrier-chain/successor-event"));
    let equation = pen_semantic_audit::EquationIdV1(Digest::of_bytes(
        b"native-carrier-chain/uelim-computation",
    ));

    let (
        predecessor_history,
        predecessor_boundary,
        successor_ids,
        declaration_wires,
        declaration_groups,
        added_equations,
        equations,
        availability,
        dag_edges,
    ) = if with_equation {
        let middle_group = GlobalId(Digest::of_bytes(b"native-carrier-chain/middle-group"));
        let middle_event = EventIdV1(Digest::of_bytes(b"native-carrier-chain/middle-event"));
        let first_boundary = UncheckedSignature {
            declarations: vec![predecessor_declaration.clone()],
        };
        let predecessor_boundary = UncheckedSignature {
            declarations: declarations[..3].to_vec(),
        };
        let predecessor_history = vec![
            UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group.clone()],
                    added_declarations: vec![predecessor_id.clone()],
                    added_equations: Vec::new(),
                    added_forced_projections: Vec::new(),
                    added_demand_contracts: Vec::new(),
                },
                successor_boundary: first_boundary,
            },
            UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: middle_event.clone(),
                    added_groups: vec![middle_group.clone()],
                    added_declarations: vec![
                        declarations[1].id.clone(),
                        declarations[2].id.clone(),
                    ],
                    added_equations: Vec::new(),
                    added_forced_projections: Vec::new(),
                    added_demand_contracts: Vec::new(),
                },
                successor_boundary: predecessor_boundary.clone(),
            },
        ];
        let successor_ids = vec![declarations[3].id.clone()];
        let declaration_wires = vec![
            UncheckedPublicDeclarationV1 {
                declaration: predecessor_id.clone(),
                origin: predecessor_event.clone(),
                group: predecessor_group.clone(),
                source_to_normal: source_declaration(&predecessor_declaration),
            },
            UncheckedPublicDeclarationV1 {
                declaration: declarations[1].id.clone(),
                origin: middle_event.clone(),
                group: middle_group.clone(),
                source_to_normal: source_declaration(&declarations[1]),
            },
            UncheckedPublicDeclarationV1 {
                declaration: declarations[2].id.clone(),
                origin: middle_event.clone(),
                group: middle_group.clone(),
                source_to_normal: source_declaration(&declarations[2]),
            },
            UncheckedPublicDeclarationV1 {
                declaration: declarations[3].id.clone(),
                origin: successor_event.clone(),
                group: successor_group.clone(),
                source_to_normal: source_declaration(&declarations[3]),
            },
        ];
        let declaration_groups = vec![
            UncheckedPublicGroupV1 {
                group: predecessor_group.clone(),
                origin: predecessor_event.clone(),
                declarations: vec![predecessor_id.clone()],
            },
            UncheckedPublicGroupV1 {
                group: middle_group,
                origin: middle_event,
                declarations: vec![declarations[1].id.clone(), declarations[2].id.clone()],
            },
            UncheckedPublicGroupV1 {
                group: successor_group.clone(),
                origin: successor_event.clone(),
                declarations: vec![declarations[3].id.clone()],
            },
        ];
        let equations = vec![UncheckedPublicEquationV1 {
            equation: equation.clone(),
            owner_head: wire_global(4),
            origin: successor_event.clone(),
            source_to_normal: source_judgment(&fresh_equation_judgment()),
            demand_port: None,
        }];
        // The derived dependency dag contains one edge per global the
        // equation judgment mentions plus its owner head: the
        // constructor is now predecessor-public, the owner is a prior
        // export of the same successor event.
        let mut availability = Vec::new();
        let mut dag_edges = Vec::new();
        for (prerequisite, claim) in [
            (
                wire_global(2),
                PublicAvailabilityV1::PredecessorPublicExport {
                    target: wire_global(2),
                },
            ),
            (
                wire_global(4),
                PublicAvailabilityV1::DependencyPriorExport {
                    target: wire_global(4),
                },
            ),
        ] {
            let dependency = PublicDependencyUseV1 {
                dependent: PublicSubjectV1::Equation {
                    equation: equation.clone(),
                },
                prerequisite,
            };
            availability.push(UncheckedPublicAvailabilityClaimV1 {
                dependency: dependency.clone(),
                claimed: claim,
            });
            dag_edges.push(dependency);
        }
        (
            predecessor_history,
            predecessor_boundary,
            successor_ids,
            declaration_wires,
            declaration_groups,
            vec![equation.clone()],
            equations,
            availability,
            dag_edges,
        )
    } else {
        let predecessor_boundary = UncheckedSignature {
            declarations: vec![predecessor_declaration.clone()],
        };
        let successor_ids: Vec<GlobalId> = declarations[1..]
            .iter()
            .map(|declaration| declaration.id.clone())
            .collect();
        let predecessor_history = vec![UncheckedPublicHistoryStepV1 {
            census: UncheckedPublicEventCensusV1 {
                event: predecessor_event.clone(),
                added_groups: vec![predecessor_group.clone()],
                added_declarations: vec![predecessor_id.clone()],
                added_equations: Vec::new(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary: predecessor_boundary.clone(),
        }];
        let declaration_wires = std::iter::once(UncheckedPublicDeclarationV1 {
            declaration: predecessor_id.clone(),
            origin: predecessor_event.clone(),
            group: predecessor_group.clone(),
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
        .collect();
        let declaration_groups = vec![
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
        ];
        (
            predecessor_history,
            predecessor_boundary,
            successor_ids,
            declaration_wires,
            declaration_groups,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    };

    let successor_boundary = UncheckedSignature {
        declarations: declarations.clone(),
    };
    let origin_cutoff = predecessor_history
        .last()
        .map(|step| step.census.event.clone());
    let wire = UncheckedPublicAuditInventoryV1 {
        schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
        predecessor_history,
        predecessor_boundary,
        successor_event: UncheckedPublicEventCensusV1 {
            event: successor_event.clone(),
            added_groups: vec![successor_group.clone()],
            added_declarations: successor_ids,
            added_equations,
            added_forced_projections: Vec::new(),
            added_demand_contracts: Vec::new(),
        },
        successor_boundary,
        declaration_groups,
        declarations: declaration_wires,
        equations,
        forced_projections: Vec::new(),
        predecessor_demand_contracts: Vec::new(),
        public_availability: availability,
        dependency_dag: UncheckedPublicDependencyDagV1 { edges: dag_edges },
        q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
            schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
            origin_cutoff,
            entries: Vec::new(),
        },
    };
    let AuditDecision::Proven(v1_manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
        &proposed_semantic_audit_lambda_unit_manifest_v1(),
    ) else {
        panic!("lambda/unit V1 manifest");
    };
    let AuditDecision::Proven(cost_manifest) =
        verify_kernel_cost_lambda_unit_manifest_v2(&proposed_kernel_cost_lambda_unit_manifest_v2())
    else {
        panic!("lambda/unit cost V2 manifest");
    };
    let inventory = match verify_public_audit_inventory_v1(&v1_manifest, &kernel, &wire) {
        AuditDecision::Proven(inventory) => inventory,
        other => panic!("verified public inventory: {other:?}"),
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
        cost_manifest,
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
fn phase_h_capabilities(chain: &ChainV3) -> pen_semantic_audit::MintedProductionCorrespondencesV1 {
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

/// The Phase J rewrite authority over one chain: complete typed
/// reduction graph, all-pairs overlap census, termination, confluence,
/// substitution stability, and predecessor conservativity.
fn run_phase_j_rewrite_authority(
    chain: &ChainV3,
    carrier: &pen_semantic_audit::VerifiedNativeRankInductiveCarrierV3,
    root_inventory: &pen_semantic_audit::VerifiedCarrierRootInventoryV3,
    census: &pen_semantic_audit::VerifiedSynthesisBackedTypedOccurrenceCensusV3,
    subject_bundle: &pen_semantic_audit::VerifiedCarrierSubjectBundleV3,
    typed_inventory: Option<&pen_semantic_audit::VerifiedTypedRewriteInventoryV1>,
) -> pen_semantic_audit::VerifiedRewriteAuthorityV3 {
    let authority = pen_semantic_audit::diagnose_rewrite_authority_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        &chain.compatibility,
        carrier,
        root_inventory,
        census,
        subject_bundle,
        typed_inventory,
    )
    .expect("rewrite authority");
    assert_eq!(authority.carrier_digest(), carrier.digest());
    assert_eq!(authority.census_digest(), census.digest());
    assert!(!authority.nodes().is_empty());
    // Every node has a unique normal form: the confluence witness.
    assert_eq!(authority.normal_forms().len(), authority.nodes().len());
    for node in authority.nodes() {
        assert!(authority.normal_form_of(node.id()).is_some());
    }
    authority
}

/// The Phase J family quotient over the exact native/rewrite chain. The
/// constructor derives the authorized-Q0 seed list internally and checks its
/// complete image against the rewrite graph before quotienting.
fn run_phase_j_family_quotient(
    chain: &ChainV3,
    carrier: &pen_semantic_audit::VerifiedNativeRankInductiveCarrierV3,
    rewrite: &pen_semantic_audit::VerifiedRewriteAuthorityV3,
    typed_inventory: Option<&pen_semantic_audit::VerifiedTypedRewriteInventoryV1>,
    fresh_program: Option<&pen_semantic_audit::VerifiedFreshConstructorComputationV1>,
) -> pen_semantic_audit::VerifiedFamilyQuotientV3 {
    let quotient = pen_semantic_audit::diagnose_family_quotient_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        carrier,
        rewrite,
        typed_inventory,
        fresh_program,
    )
    .expect("authorized-Q0 family quotient");
    assert_eq!(quotient.native_carrier_digest(), carrier.digest());
    assert_eq!(quotient.rewrite_authority_digest(), rewrite.digest());
    assert_eq!(quotient.family_images().len(), carrier.families().len());
    assert_eq!(quotient.normalized_family_count(), carrier.families().len());
    assert!(!quotient.classes().is_empty());
    println!(
        "family quotient normalized {} classes {}",
        quotient.normalized_family_count(),
        quotient.classes().len()
    );

    // The constructor is a computation, not a one-shot assertion: reminting
    // the exact chain must reproduce the same canonical authority.
    let remint = pen_semantic_audit::diagnose_family_quotient_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        carrier,
        rewrite,
        typed_inventory,
        fresh_program,
    )
    .expect("deterministic family-quotient remint");
    assert_eq!(remint.digest(), quotient.digest());
    assert_eq!(remint.quotient_digest(), quotient.quotient_digest());
    assert_eq!(remint.classes(), quotient.classes());
    quotient
}

/// The Phase J structural weakening/restriction theorem. The predecessor
/// carrier and quotient are reconstructed internally; callers provide no
/// family sets, maps, or conservativity assertion.
fn run_phase_j_family_weakening(
    chain: &ChainV3,
    rewrite: &pen_semantic_audit::VerifiedRewriteAuthorityV3,
    quotient: &pen_semantic_audit::VerifiedFamilyQuotientV3,
) -> pen_semantic_audit::VerifiedFamilyWeakeningV3 {
    use std::collections::{BTreeMap, BTreeSet};

    let weakening = pen_semantic_audit::diagnose_family_weakening_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        rewrite,
        quotient,
    )
    .expect("derived family weakening and restriction");
    assert_eq!(
        weakening.semantic_manifest_digest(),
        chain.manifest_v3.candidate_digest()
    );
    assert_eq!(weakening.inventory_digest(), chain.inventory.digest());
    assert_eq!(
        weakening.exact_extension_digest(),
        chain.inventory.exact_extension().digest()
    );
    assert_eq!(
        weakening.predecessor_boundary_digest(),
        chain.inventory.predecessor_boundary().digest()
    );
    assert_eq!(
        weakening.successor_boundary_digest(),
        chain.inventory.successor_boundary().digest()
    );
    assert_eq!(
        weakening.new_event(),
        chain.inventory.exact_extension().event()
    );
    assert_eq!(weakening.rewrite_authority_digest(), rewrite.digest());
    assert_eq!(
        weakening.predecessor_reconstruction_digest(),
        rewrite.predecessor_reconstruction_digest()
    );
    assert_eq!(weakening.successor_quotient_digest(), quotient.digest());

    let raw_map = weakening
        .raw_weakening()
        .iter()
        .map(|entry| (entry.source().clone(), entry.target().clone()))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(raw_map.len(), weakening.raw_weakening().len());
    assert_eq!(
        raw_map.values().cloned().collect::<BTreeSet<_>>().len(),
        raw_map.len()
    );

    let class_map = weakening
        .weakening()
        .iter()
        .map(|entry| (entry.source().clone(), entry.target().clone()))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(class_map.len(), weakening.predecessor_classes().len());
    let targets = class_map.values().cloned().collect::<BTreeSet<_>>();
    assert_eq!(targets.len(), class_map.len());
    assert_eq!(
        weakening.image().iter().cloned().collect::<BTreeSet<_>>(),
        targets
    );
    let restriction = weakening
        .restriction_on_image()
        .iter()
        .map(|entry| (entry.source().clone(), entry.target().clone()))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(restriction.len(), class_map.len());
    for (source, target) in &class_map {
        assert_eq!(restriction.get(target), Some(source));
    }

    let successor_classes = quotient
        .classes()
        .iter()
        .map(|class| (class.id().clone(), class))
        .collect::<BTreeMap<_, _>>();
    for predecessor_class in weakening.predecessor_classes() {
        let target = class_map
            .get(predecessor_class.id())
            .expect("every predecessor class has an image");
        let successor_class = successor_classes
            .get(target)
            .expect("every image is a successor class");
        for member in predecessor_class.members() {
            let transported = raw_map
                .get(member)
                .expect("every predecessor member has raw weakening");
            assert!(successor_class.members().contains(transported));
        }
    }

    let remint = pen_semantic_audit::diagnose_family_weakening_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        rewrite,
        quotient,
    )
    .expect("deterministic family-weakening remint");
    assert_eq!(remint, weakening);
    assert_eq!(remint.digest(), weakening.digest());
    assert_eq!(
        remint.predecessor_quotient_digest(),
        weakening.predecessor_quotient_digest()
    );
    weakening
}

/// The exact `MarginalFamilySet` stage. Marginality is computed as the
/// successor-class complement of the weakening image before member support
/// is checked; the capability issues no cost, demand, or SR2 authority.
fn run_phase_j_marginal_family_set(
    chain: &ChainV3,
    quotient: &pen_semantic_audit::VerifiedFamilyQuotientV3,
    weakening: &pen_semantic_audit::VerifiedFamilyWeakeningV3,
) -> pen_semantic_audit::VerifiedMarginalFamilySetV3 {
    use std::collections::BTreeSet;

    let marginals = pen_semantic_audit::diagnose_marginal_family_set_v3(
        &chain.manifest_v3,
        quotient,
        weakening,
    )
    .expect("exact marginal-family complement");
    assert_eq!(
        marginals.semantic_manifest_digest(),
        chain.manifest_v3.candidate_digest()
    );
    assert_eq!(marginals.inventory_digest(), chain.inventory.digest());
    assert_eq!(
        marginals.exact_extension_digest(),
        chain.inventory.exact_extension().digest()
    );
    assert_eq!(
        marginals.predecessor_boundary_digest(),
        chain.inventory.predecessor_boundary().digest()
    );
    assert_eq!(
        marginals.successor_boundary_digest(),
        chain.inventory.successor_boundary().digest()
    );
    assert_eq!(
        marginals.new_event(),
        chain.inventory.exact_extension().event()
    );
    assert_eq!(
        marginals.rewrite_authority_digest(),
        weakening.rewrite_authority_digest()
    );
    assert_eq!(
        marginals.predecessor_reconstruction_digest(),
        weakening.predecessor_reconstruction_digest()
    );
    assert_eq!(marginals.successor_quotient_digest(), quotient.digest());
    assert_eq!(marginals.weakening_digest(), weakening.digest());

    let marginal_ids = marginals
        .marginal_ids()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(marginal_ids.len(), marginals.marginal_ids().len());
    assert_eq!(
        marginal_ids,
        marginals
            .marginals()
            .iter()
            .map(|class| class.id().clone())
            .collect::<BTreeSet<_>>()
    );
    let image = weakening.image().iter().cloned().collect::<BTreeSet<_>>();
    assert!(image.is_disjoint(&marginal_ids));
    assert_eq!(
        image.union(&marginal_ids).cloned().collect::<BTreeSet<_>>(),
        quotient
            .classes()
            .iter()
            .map(|class| class.id().clone())
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        marginals.image_class_count() + marginals.marginal_count(),
        marginals.successor_class_count()
    );
    assert_eq!(
        marginals.image_raw_member_count() + marginals.marginal_raw_member_count(),
        marginals.successor_raw_family_count()
    );

    let remint = pen_semantic_audit::diagnose_marginal_family_set_v3(
        &chain.manifest_v3,
        quotient,
        weakening,
    )
    .expect("deterministic marginal-family remint");
    assert_eq!(remint, marginals);
    assert_eq!(remint.digest(), marginals.digest());
    assert_eq!(
        remint.semantic_set_digest(),
        marginals.semantic_set_digest()
    );
    marginals
}

/// The inventory-relative empty base of `DemandOrbitCensus`. The current
/// chains contain no predecessor demand contracts, so their exact demand
/// carrier has the uniquely empty quotient. This capability does not mint
/// realizations, outputs, SR2, or a live debt verdict.
fn run_phase_j_empty_demand_orbit_census(
    chain: &ChainV3,
    marginals: &pen_semantic_audit::VerifiedMarginalFamilySetV3,
) -> pen_semantic_audit::VerifiedDemandOrbitCensusV3 {
    let census = pen_semantic_audit::diagnose_demand_orbit_census_v3(
        &chain.manifest_v3,
        &chain.inventory,
        marginals,
    )
    .expect("complete empty demand-orbit census");
    assert_eq!(
        census.semantic_manifest_digest(),
        chain.manifest_v3.candidate_digest()
    );
    assert_eq!(census.inventory_digest(), chain.inventory.digest());
    assert_eq!(
        census.inventory_coverage_digest(),
        chain.inventory.coverage().digest()
    );
    assert_eq!(
        census.predecessor_history_digest(),
        chain.inventory.predecessor_history_digest()
    );
    assert_eq!(
        census.exact_extension_digest(),
        chain.inventory.exact_extension().digest()
    );
    assert_eq!(
        census.predecessor_boundary_digest(),
        chain.inventory.predecessor_boundary().digest()
    );
    assert_eq!(
        census.successor_boundary_digest(),
        chain.inventory.successor_boundary().digest()
    );
    assert_eq!(
        census.new_event(),
        chain.inventory.exact_extension().event()
    );
    assert_eq!(census.marginal_family_set_digest(), marginals.digest());
    assert_eq!(
        census.marginal_semantic_set_digest(),
        marginals.semantic_set_digest()
    );
    assert_eq!(census.predecessor_demand_contract_count(), 0);
    assert_eq!(census.orbit_count(), 0);
    assert!(census.is_empty());
    assert_eq!(
        census.digest(),
        &Digest::of_canonical(
            "pen-semantic-audit/verified-demand-orbit-census/v3",
            &census,
        )
    );

    let remint = pen_semantic_audit::diagnose_demand_orbit_census_v3(
        &chain.manifest_v3,
        &chain.inventory,
        marginals,
    )
    .expect("deterministic empty demand-orbit remint");
    assert_eq!(remint, census);
    assert_eq!(remint.digest(), census.digest());
    census
}

/// The inventory-relative empty base of `DemandRealizationCensus`. It
/// independently compares the verified inventory's equation ports with the
/// complete V3 seed metadata ledger before certifying the empty relation.
fn run_phase_j_empty_demand_realization_census(
    chain: &ChainV3,
    marginals: &pen_semantic_audit::VerifiedMarginalFamilySetV3,
    orbits: &pen_semantic_audit::VerifiedDemandOrbitCensusV3,
) -> pen_semantic_audit::VerifiedDemandRealizationCensusV3 {
    let census = pen_semantic_audit::diagnose_demand_realization_census_v3(
        &chain.manifest_v3,
        &chain.inventory,
        &chain.seed_census,
        marginals,
        orbits,
    )
    .expect("complete empty demand-realization census");
    assert_eq!(
        census.semantic_manifest_digest(),
        chain.manifest_v3.candidate_digest()
    );
    assert_eq!(census.inventory_digest(), chain.inventory.digest());
    assert_eq!(
        census.inventory_coverage_digest(),
        chain.inventory.coverage().digest()
    );
    assert_eq!(
        census.predecessor_history_digest(),
        chain.inventory.predecessor_history_digest()
    );
    assert_eq!(
        census.exact_extension_digest(),
        chain.inventory.exact_extension().digest()
    );
    assert_eq!(
        census.predecessor_boundary_digest(),
        chain.inventory.predecessor_boundary().digest()
    );
    assert_eq!(
        census.successor_boundary_digest(),
        chain.inventory.successor_boundary().digest()
    );
    assert_eq!(
        census.new_event(),
        chain.inventory.exact_extension().event()
    );
    assert_eq!(
        census.semantic_seed_base_census_digest(),
        chain.seed_census.digest()
    );
    assert_eq!(
        census.public_clause_census_digest(),
        chain.seed_census.public_clause_census_digest()
    );
    assert_eq!(census.marginal_family_set_digest(), marginals.digest());
    assert_eq!(
        census.marginal_semantic_set_digest(),
        marginals.semantic_set_digest()
    );
    assert_eq!(census.demand_orbit_census_digest(), orbits.digest());
    assert_eq!(
        census.predecessor_demand_census_digest(),
        orbits.predecessor_demand_census_digest()
    );
    assert_eq!(
        census.empty_orbit_partition_digest(),
        orbits.empty_orbit_partition_digest()
    );
    assert_eq!(census.realization_obligation_count(), 0);
    assert_eq!(census.realization_count(), 0);
    assert!(census.is_empty());
    assert_eq!(
        census.digest(),
        &Digest::of_canonical(
            "pen-semantic-audit/verified-demand-realization-census/v3",
            &census,
        )
    );

    let remint = pen_semantic_audit::diagnose_demand_realization_census_v3(
        &chain.manifest_v3,
        &chain.inventory,
        &chain.seed_census,
        marginals,
        orbits,
    )
    .expect("deterministic empty demand-realization remint");
    assert_eq!(remint, census);
    assert_eq!(remint.digest(), census.digest());
    census
}

/// Exhaustive pre-SR2 clause support over every raw member of every exact
/// marginal class. This stage derives support only; it chooses no principal
/// source, assigns no provenance tag, and issues no `nu`.
fn run_phase_j_sr2_dependency_support_census(
    chain: &ChainV3,
    quotient: &pen_semantic_audit::VerifiedFamilyQuotientV3,
    marginals: &pen_semantic_audit::VerifiedMarginalFamilySetV3,
    orbits: &pen_semantic_audit::VerifiedDemandOrbitCensusV3,
    realizations: &pen_semantic_audit::VerifiedDemandRealizationCensusV3,
) -> pen_semantic_audit::VerifiedSr2DependencySupportCensusV3 {
    use std::collections::{BTreeMap, BTreeSet};

    let census = pen_semantic_audit::diagnose_sr2_dependency_support_v3(
        &chain.manifest_v3,
        &chain.inventory,
        &chain.public_clauses,
        &chain.seed_census,
        quotient,
        marginals,
        orbits,
        realizations,
    )
    .expect("exhaustive V3 SR2 dependency-support census");
    assert_eq!(
        census.semantic_manifest_digest(),
        chain.manifest_v3.candidate_digest()
    );
    assert_eq!(census.inventory_digest(), chain.inventory.digest());
    assert_eq!(
        census.inventory_coverage_digest(),
        chain.inventory.coverage().digest()
    );
    assert_eq!(
        census.predecessor_history_digest(),
        chain.inventory.predecessor_history_digest()
    );
    assert_eq!(
        census.exact_extension_digest(),
        chain.inventory.exact_extension().digest()
    );
    assert_eq!(
        census.public_clause_census_digest(),
        chain.public_clauses.digest()
    );
    assert_eq!(
        census.semantic_seed_base_census_digest(),
        chain.seed_census.digest()
    );
    assert_eq!(census.successor_quotient_digest(), quotient.digest());
    assert_eq!(census.marginal_family_set_digest(), marginals.digest());
    assert_eq!(census.demand_orbit_census_digest(), orbits.digest());
    assert_eq!(
        census.demand_realization_census_digest(),
        realizations.digest()
    );
    assert_eq!(census.marginal_family_count(), marginals.marginal_count());
    assert_eq!(
        census.marginal_raw_member_count(),
        marginals.marginal_raw_member_count()
    );
    assert_eq!(census.preexisting_output_count(), 0);
    assert_eq!(census.family_support().len(), marginals.marginal_count());

    let marginal_index = marginals
        .marginals()
        .iter()
        .map(|class| (class.id().clone(), class))
        .collect::<BTreeMap<_, _>>();
    let clause_ids = chain
        .public_clauses
        .clauses()
        .iter()
        .map(|clause| clause.id().clone())
        .collect::<BTreeSet<_>>();
    for support in census.family_support() {
        let marginal = marginal_index
            .get(support.family())
            .expect("support family belongs to exact marginal set");
        assert_eq!(support.role(), marginal.role());
        assert_eq!(support.raw_member_count(), marginal.members().len());
        assert!(
            support
                .all_clauses()
                .iter()
                .all(|clause| clause_ids.contains(clause))
        );
        assert!(
            support
                .candidate_local_clauses()
                .iter()
                .all(|clause| support.all_clauses().contains(clause))
        );
    }
    assert_eq!(
        census.digest(),
        &Digest::of_canonical(
            "pen-semantic-audit/verified-sr2-dependency-support-census/v3",
            &census,
        )
    );

    let remint = pen_semantic_audit::diagnose_sr2_dependency_support_v3(
        &chain.manifest_v3,
        &chain.inventory,
        &chain.public_clauses,
        &chain.seed_census,
        quotient,
        marginals,
        orbits,
        realizations,
    )
    .expect("deterministic SR2 dependency-support remint");
    assert_eq!(remint, census);
    assert_eq!(remint.digest(), census.digest());
    census
}

/// The deliberately restricted cost-V2 theorem over an exact V3 chain. The
/// constructor derives every raw cost clause, dependency edge, demand-port
/// binding, and leave-one-out negative witness internally from verifier-minted
/// authorities. It charges bodyless successor declarations and separately
/// sealed successor equations; it is not a general Q0-minimal-basis theorem.
fn run_phase_j_restricted_kernel_cost_basis(
    chain: &ChainV3,
    carrier: &pen_semantic_audit::VerifiedNativeRankInductiveCarrierV3,
    rewrite: &pen_semantic_audit::VerifiedRewriteAuthorityV3,
) -> pen_semantic_audit::VerifiedRestrictedKernelCostBasisV3 {
    use std::collections::BTreeSet;

    let basis = pen_semantic_audit::diagnose_restricted_kernel_cost_basis_v3(
        &chain.cost_manifest,
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.compatibility,
        &chain.inventory,
        &chain.public_clauses,
        carrier,
        rewrite,
    )
    .expect("restricted V3 kernel-cost basis");
    assert_eq!(
        basis.cost_manifest_digest(),
        chain.cost_manifest.candidate_digest()
    );
    assert_eq!(
        basis.semantic_manifest_digest(),
        chain.manifest_v3.candidate_digest()
    );
    assert_eq!(basis.inventory_digest(), chain.inventory.digest());
    assert_eq!(
        basis.public_clause_census_digest(),
        chain.public_clauses.digest()
    );
    assert_eq!(basis.carrier_digest(), carrier.digest());
    assert_eq!(basis.rewrite_authority_digest(), rewrite.digest());
    assert_eq!(basis.kernel_cost(), basis.clause_count() as u16);
    assert_eq!(basis.basis_classes().len(), basis.clause_count());

    let mut expected_members = chain
        .inventory
        .exact_extension()
        .new_declarations()
        .iter()
        .map(|head| {
            chain
                .public_clauses
                .clause_for_declaration(head)
                .expect("successor declaration has a canonical clause")
                .clone()
        })
        .collect::<BTreeSet<_>>();
    expected_members.extend(chain.inventory.equations().iter().map(|equation| {
        chain
            .public_clauses
            .clause_for_equation(equation.equation())
            .expect("successor equation has a canonical clause")
            .clone()
    }));
    let actual_members = basis
        .basis_classes()
        .iter()
        .map(|class| {
            assert_eq!(class.members().len(), 1);
            class.members()[0].clone()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_members.len(), basis.basis_classes().len());
    assert_eq!(actual_members, expected_members);
    assert_eq!(
        basis.digest(),
        &Digest::of_canonical(
            "pen-semantic-audit/verified-restricted-kernel-cost-basis/v3",
            &basis,
        )
    );

    let remint = pen_semantic_audit::diagnose_restricted_kernel_cost_basis_v3(
        &chain.cost_manifest,
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.compatibility,
        &chain.inventory,
        &chain.public_clauses,
        carrier,
        rewrite,
    )
    .expect("deterministic restricted cost-basis remint");
    assert_eq!(remint, basis);
    assert_eq!(remint.digest(), basis.digest());
    basis
}

/// Prove that the current exact marginal set has no typed, role-preserving
/// injection into the frozen SR2 tag codomain. This support-independent
/// pigeonhole theorem is diagnostic only: failed positive SR2 makes `nu`
/// undefined, not zero.
fn run_phase_j_sr2_noninjectivity(
    chain: &ChainV3,
    marginals: &pen_semantic_audit::VerifiedMarginalFamilySetV3,
    orbits: &pen_semantic_audit::VerifiedDemandOrbitCensusV3,
    realizations: &pen_semantic_audit::VerifiedDemandRealizationCensusV3,
    restricted_cost: &pen_semantic_audit::VerifiedRestrictedKernelCostBasisV3,
) -> pen_semantic_audit::VerifiedSr2NonInjectivityV3 {
    let obstruction = pen_semantic_audit::diagnose_sr2_noninjectivity_v3(
        &chain.manifest_v3,
        &chain.inventory,
        &chain.public_clauses,
        marginals,
        orbits,
        realizations,
        restricted_cost,
    )
    .expect("exact SR2 noninjectivity theorem");
    assert_eq!(
        obstruction.semantic_manifest_digest(),
        chain.manifest_v3.candidate_digest()
    );
    assert_eq!(obstruction.inventory_digest(), chain.inventory.digest());
    assert_eq!(
        obstruction.public_clause_census_digest(),
        chain.public_clauses.digest()
    );
    assert_eq!(obstruction.marginal_family_set_digest(), marginals.digest());
    assert_eq!(obstruction.demand_orbit_census_digest(), orbits.digest());
    assert_eq!(
        obstruction.demand_realization_census_digest(),
        realizations.digest()
    );
    assert_eq!(
        obstruction.restricted_cost_basis_digest(),
        restricted_cost.digest()
    );
    assert_eq!(
        obstruction.marginal_family_count(),
        marginals.marginal_count()
    );
    assert_eq!(
        obstruction.restricted_cost_basis_count(),
        restricted_cost.basis_classes().len()
    );
    assert_eq!(obstruction.recognized_role_count(), 4);
    assert_eq!(obstruction.preexisting_demand_output_count(), 0);
    assert_eq!(
        obstruction.role_fiber_codomain_capacity(),
        restricted_cost.basis_classes().len()
    );
    assert_eq!(
        obstruction.full_codomain_capacity(),
        restricted_cost.basis_classes().len() * obstruction.recognized_role_count()
    );
    match obstruction.witness() {
        pen_semantic_audit::Sr2PigeonholeWitnessV3::RoleFiber {
            domain_count,
            codomain_capacity,
            ..
        }
        | pen_semantic_audit::Sr2PigeonholeWitnessV3::TotalCapacity {
            domain_count,
            codomain_capacity,
        } => assert!(domain_count > codomain_capacity),
    }
    assert_eq!(
        obstruction.digest(),
        &Digest::of_canonical(
            "pen-semantic-audit/verified-sr2-noninjectivity/v3",
            &obstruction,
        )
    );

    let remint = pen_semantic_audit::diagnose_sr2_noninjectivity_v3(
        &chain.manifest_v3,
        &chain.inventory,
        &chain.public_clauses,
        marginals,
        orbits,
        realizations,
        restricted_cost,
    )
    .expect("deterministic SR2 noninjectivity remint");
    assert_eq!(remint, obstruction);
    assert_eq!(remint.digest(), obstruction.digest());
    obstruction
}

/// Full Phase I mint over the genuine equationless chain and the
/// fresh-equation chain, with an adversarial cross-chain binding
/// rejection. Requires the pinned local Agda runtime.
#[test]
#[ignore]
fn phase_i_carrier_census_mints_over_genuine_chains() {
    let chain = chain_v3(false);
    let minted = phase_h_capabilities(&chain);
    let (carrier, root_inventory, subject_bundle, census) = run_phase_i_pipeline(&chain, &minted);

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
    assert!(
        root_inventory
            .roots()
            .iter()
            .all(|root| matches!(root.kind(), pen_semantic_audit::CarrierRootKindV3::Subject))
    );

    // The subject bundle is a genuine canonical bundle distinct from the
    // committed correspondence fixture.
    let committed = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    assert_ne!(
        subject_bundle.bundle().canonical_bytes(),
        committed.as_slice()
    );

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
    let (equation_carrier, equation_roots, equation_bundle, equation_census) =
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
    assert!(
        equation_bundle
            .wire_inexpressible()
            .iter()
            .all(|(_, reason)| matches!(
                reason,
                pen_semantic_audit::WireInexpressibleReasonV3::ContextGrowingEquationAction
            ))
    );
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

    // ---- Phase J: the rewrite authority over both chains. ----

    // Equationless chain: no sealed equations, so no typed rewrite
    // inventory; edges come from delta on the bodyful unit value (and
    // any beta), and the predecessor restriction is conservative.
    let authority = run_phase_j_rewrite_authority(
        &chain,
        &carrier,
        &root_inventory,
        &census,
        &subject_bundle,
        None,
    );
    assert!(!authority.edges().is_empty());
    assert!(authority.edges().iter().all(|edge| matches!(
        edge.rule(),
        pen_semantic_audit::RewriteRuleV3::OrdinaryBeta
            | pen_semantic_audit::RewriteRuleV3::PublicDelta { .. }
    )));
    assert!(authority.predecessor_node_count() > 0);
    let quotient = run_phase_j_family_quotient(&chain, &carrier, &authority, None, None);
    assert_eq!(quotient.seed_images().len(), 4);
    assert_eq!(quotient.fresh_program_digest(), None);
    assert_eq!(quotient.classes().len(), 10);
    let weakening = run_phase_j_family_weakening(&chain, &authority, &quotient);
    assert_eq!(weakening.predecessor_seed_count(), 1);
    assert_eq!(weakening.predecessor_raw_family_count(), 1);
    assert_eq!(weakening.predecessor_classes().len(), 1);
    assert_eq!(weakening.predecessor_q2_disposition_count(), 5);
    assert_eq!(weakening.raw_weakening().len(), 1);
    assert_eq!(weakening.weakening().len(), 1);
    assert_eq!(weakening.restriction_on_image().len(), 1);
    assert_eq!(weakening.image().len(), 1);
    // Pre-mint arithmetic only; the capability below supplies authority.
    assert_eq!(quotient.classes().len() - weakening.image().len(), 9);
    let marginal_set = run_phase_j_marginal_family_set(&chain, &quotient, &weakening);
    assert_eq!(marginal_set.successor_class_count(), 10);
    assert_eq!(marginal_set.image_class_count(), 1);
    assert_eq!(marginal_set.marginal_count(), 9);
    assert_eq!(marginal_set.successor_raw_family_count(), 24);
    assert_eq!(marginal_set.image_raw_member_count(), 1);
    assert_eq!(marginal_set.marginal_raw_member_count(), 23);
    let demand_orbit_census = run_phase_j_empty_demand_orbit_census(&chain, &marginal_set);
    assert!(demand_orbit_census.is_empty());
    let demand_realization_census =
        run_phase_j_empty_demand_realization_census(&chain, &marginal_set, &demand_orbit_census);
    assert!(demand_realization_census.is_empty());
    let sr2_dependency_support = run_phase_j_sr2_dependency_support_census(
        &chain,
        &quotient,
        &marginal_set,
        &demand_orbit_census,
        &demand_realization_census,
    );
    assert_eq!(sr2_dependency_support.marginal_family_count(), 9);
    assert_eq!(sr2_dependency_support.marginal_raw_member_count(), 23);
    let cost_basis = run_phase_j_restricted_kernel_cost_basis(&chain, &carrier, &authority);
    assert_eq!(cost_basis.kernel_cost(), 3);
    assert_eq!(cost_basis.clause_count(), 3);
    let sr2_noninjectivity = run_phase_j_sr2_noninjectivity(
        &chain,
        &marginal_set,
        &demand_orbit_census,
        &demand_realization_census,
        &cost_basis,
    );
    assert!(matches!(
        sr2_noninjectivity.witness(),
        pen_semantic_audit::Sr2PigeonholeWitnessV3::RoleFiber {
            role: pen_semantic_audit::LocalRoleV1::SupportAction,
            domain_count: 6,
            codomain_capacity: 3,
        }
    ));

    // Fresh-equation chain: the typed rewrite inventory binds the
    // sealed `uelim b constructor = b` equation through the restricted
    // normalizer; the authority gains fresh edges, and delta/fresh
    // interactions appear in the overlap census with joins.
    let fresh_program = equation_fresh_program(&equation_chain);
    let typed_inventory =
        equation_typed_rewrite_inventory_with_program(&equation_chain, &fresh_program);
    let equation_authority = run_phase_j_rewrite_authority(
        &equation_chain,
        &equation_carrier,
        &equation_roots,
        &equation_census,
        &equation_bundle,
        Some(&typed_inventory),
    );
    assert!(equation_authority.edges().iter().any(|edge| matches!(
        edge.rule(),
        pen_semantic_audit::RewriteRuleV3::FreshEquation { .. }
    )));
    assert!(equation_authority.edges().iter().any(|edge| matches!(
        edge.rule(),
        pen_semantic_audit::RewriteRuleV3::PublicDelta { .. }
    )));
    assert!(!equation_authority.overlap_pairs().is_empty());
    let equation_quotient = run_phase_j_family_quotient(
        &equation_chain,
        &equation_carrier,
        &equation_authority,
        Some(&typed_inventory),
        Some(&fresh_program),
    );
    assert_eq!(
        equation_quotient.fresh_program_digest(),
        Some(fresh_program.program_digest())
    );
    assert_eq!(
        equation_quotient.family_images().len(),
        equation_carrier.families().len()
    );
    assert_eq!(equation_quotient.classes().len(), 13);
    let equation_weakening =
        run_phase_j_family_weakening(&equation_chain, &equation_authority, &equation_quotient);
    assert_eq!(equation_weakening.predecessor_seed_count(), 3);
    assert_eq!(equation_weakening.predecessor_raw_family_count(), 3);
    assert_eq!(equation_weakening.predecessor_classes().len(), 3);
    assert_eq!(equation_weakening.predecessor_q2_disposition_count(), 45);
    assert_eq!(equation_weakening.raw_weakening().len(), 3);
    assert_eq!(equation_weakening.weakening().len(), 3);
    assert_eq!(equation_weakening.restriction_on_image().len(), 3);
    assert_eq!(equation_weakening.image().len(), 3);
    // Pre-mint arithmetic only; the capability below supplies authority.
    assert_eq!(
        equation_quotient.classes().len() - equation_weakening.image().len(),
        10
    );
    let equation_marginal_set =
        run_phase_j_marginal_family_set(&equation_chain, &equation_quotient, &equation_weakening);
    assert_eq!(equation_marginal_set.successor_class_count(), 13);
    assert_eq!(equation_marginal_set.image_class_count(), 3);
    assert_eq!(equation_marginal_set.marginal_count(), 10);
    assert_eq!(equation_marginal_set.successor_raw_family_count(), 29);
    assert_eq!(equation_marginal_set.image_raw_member_count(), 3);
    assert_eq!(equation_marginal_set.marginal_raw_member_count(), 26);
    let equation_demand_orbit_census =
        run_phase_j_empty_demand_orbit_census(&equation_chain, &equation_marginal_set);
    assert!(equation_demand_orbit_census.is_empty());
    let equation_demand_realization_census = run_phase_j_empty_demand_realization_census(
        &equation_chain,
        &equation_marginal_set,
        &equation_demand_orbit_census,
    );
    assert!(equation_demand_realization_census.is_empty());
    let equation_sr2_dependency_support = run_phase_j_sr2_dependency_support_census(
        &equation_chain,
        &equation_quotient,
        &equation_marginal_set,
        &equation_demand_orbit_census,
        &equation_demand_realization_census,
    );
    assert_eq!(equation_sr2_dependency_support.marginal_family_count(), 10);
    assert_eq!(
        equation_sr2_dependency_support.marginal_raw_member_count(),
        26
    );
    let equation_cost_basis = run_phase_j_restricted_kernel_cost_basis(
        &equation_chain,
        &equation_carrier,
        &equation_authority,
    );
    assert_eq!(equation_cost_basis.kernel_cost(), 2);
    assert_eq!(equation_cost_basis.clause_count(), 2);
    let equation_sr2_noninjectivity = run_phase_j_sr2_noninjectivity(
        &equation_chain,
        &equation_marginal_set,
        &equation_demand_orbit_census,
        &equation_demand_realization_census,
        &equation_cost_basis,
    );
    assert_eq!(equation_sr2_noninjectivity.marginal_family_count(), 10);
    assert_eq!(equation_sr2_noninjectivity.full_codomain_capacity(), 8);

    // Adversarial: the equationless chain's inventory has no equations,
    // so presenting the equation chain's typed inventory must fail
    // closed; and the equation chain without its typed inventory must
    // fail closed as missing.
    assert!(matches!(
        pen_semantic_audit::diagnose_rewrite_authority_v3(
            &chain.v1_manifest,
            &chain.v2_manifest,
            &chain.manifest_v3,
            &chain.kernel,
            &chain.inventory,
            &chain.compatibility,
            &carrier,
            &root_inventory,
            &census,
            &subject_bundle,
            Some(&typed_inventory),
        ),
        Err(pen_semantic_audit::RewriteAuthorityFailureV3::FreshInventoryBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_rewrite_authority_v3(
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.kernel,
            &equation_chain.inventory,
            &equation_chain.compatibility,
            &equation_carrier,
            &equation_roots,
            &equation_census,
            &equation_bundle,
            None,
        ),
        Err(pen_semantic_audit::RewriteAuthorityFailureV3::MissingTypedRewriteInventory)
    ));

    // Family-quotient adversaries: missing or wrong fresh authority,
    // unexpected equation authority on the equationless chain, and a
    // cross-chain rewrite theorem all fail before any quotient is minted.
    assert!(matches!(
        pen_semantic_audit::diagnose_family_quotient_v3(
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.kernel,
            &equation_chain.inventory,
            &equation_carrier,
            &equation_authority,
            Some(&typed_inventory),
            None,
        ),
        Err(pen_semantic_audit::FamilyQuotientFailureV3::MissingFreshProgram)
    ));
    let wrong_fresh_program = other_equation_fresh_program(&equation_chain);
    assert_ne!(
        wrong_fresh_program.program_digest(),
        fresh_program.program_digest()
    );
    assert!(matches!(
        pen_semantic_audit::diagnose_family_quotient_v3(
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.kernel,
            &equation_chain.inventory,
            &equation_carrier,
            &equation_authority,
            Some(&typed_inventory),
            Some(&wrong_fresh_program),
        ),
        Err(pen_semantic_audit::FamilyQuotientFailureV3::FreshProgramBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_family_quotient_v3(
            &chain.v1_manifest,
            &chain.v2_manifest,
            &chain.manifest_v3,
            &chain.kernel,
            &chain.inventory,
            &carrier,
            &authority,
            Some(&typed_inventory),
            Some(&fresh_program),
        ),
        Err(pen_semantic_audit::FamilyQuotientFailureV3::UnexpectedFreshProgram)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_family_quotient_v3(
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.kernel,
            &equation_chain.inventory,
            &equation_carrier,
            &authority,
            Some(&typed_inventory),
            Some(&fresh_program),
        ),
        Err(pen_semantic_audit::FamilyQuotientFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_family_quotient_v3(
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.kernel,
            &equation_chain.inventory,
            &equation_carrier,
            &equation_authority,
            Some(&typed_inventory),
            None,
        ),
        AuditDecision::Unknown(pen_semantic_audit::AuditUnknownReason::MissingFamilyQuotientV3)
    ));

    // Weakening adversaries: the two fixtures share one successor
    // signature, so these failures pin the exact inventory/history/rewrite/
    // quotient chain rather than merely noticing a signature mismatch.
    assert!(matches!(
        pen_semantic_audit::diagnose_family_weakening_v3(
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.kernel,
            &equation_chain.inventory,
            &equation_authority,
            &quotient,
        ),
        Err(pen_semantic_audit::FamilyWeakeningFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_family_weakening_v3(
            &chain.v1_manifest,
            &chain.v2_manifest,
            &chain.manifest_v3,
            &chain.kernel,
            &chain.inventory,
            &authority,
            &equation_quotient,
        ),
        Err(pen_semantic_audit::FamilyWeakeningFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_family_weakening_v3(
            &chain.v1_manifest,
            &chain.v2_manifest,
            &chain.manifest_v3,
            &chain.kernel,
            &chain.inventory,
            &equation_authority,
            &quotient,
        ),
        Err(pen_semantic_audit::FamilyWeakeningFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_family_weakening_v3(
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.kernel,
            &equation_chain.inventory,
            &equation_authority,
            &quotient,
        ),
        AuditDecision::Unknown(
            pen_semantic_audit::AuditUnknownReason::MissingWeakeningImageConservativity
        )
    ));

    // Marginal-set adversaries bind the exact quotient/weakening pair in
    // both directions. The two chains share one successor signature, so a
    // digest-only signature check would not reject these substitutions.
    assert!(matches!(
        pen_semantic_audit::diagnose_marginal_family_set_v3(
            &equation_chain.manifest_v3,
            &equation_quotient,
            &weakening,
        ),
        Err(pen_semantic_audit::MarginalFamilySetFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_marginal_family_set_v3(
            &chain.manifest_v3,
            &quotient,
            &equation_weakening,
        ),
        Err(pen_semantic_audit::MarginalFamilySetFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_marginal_family_set_v3(
            &equation_chain.manifest_v3,
            &equation_quotient,
            &weakening,
        ),
        AuditDecision::Unknown(
            pen_semantic_audit::AuditUnknownReason::MissingWeakeningMarginalAuthority
        )
    ));

    // Demand-orbit adversaries: emptiness is derived from the exact verified
    // inventory and bound to the exact marginal stage. Cross-chain
    // substitution fails even though the successor signatures coincide.
    assert!(matches!(
        pen_semantic_audit::diagnose_demand_orbit_census_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &marginal_set,
        ),
        Err(pen_semantic_audit::DemandOrbitCensusFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_demand_orbit_census_v3(
            &chain.manifest_v3,
            &chain.inventory,
            &equation_marginal_set,
        ),
        Err(pen_semantic_audit::DemandOrbitCensusFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_demand_orbit_census_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &marginal_set,
        ),
        AuditDecision::Unknown(pen_semantic_audit::AuditUnknownReason::MissingDemandOrbitCensusV2)
    ));

    // Realization-census adversaries independently bind both the complete
    // seed port ledger and the exact orbit/marginal chain.
    assert!(matches!(
        pen_semantic_audit::diagnose_demand_realization_census_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &equation_chain.seed_census,
            &equation_marginal_set,
            &demand_orbit_census,
        ),
        Err(pen_semantic_audit::DemandRealizationCensusFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_demand_realization_census_v3(
            &chain.manifest_v3,
            &chain.inventory,
            &equation_chain.seed_census,
            &marginal_set,
            &demand_orbit_census,
        ),
        Err(pen_semantic_audit::DemandRealizationCensusFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_demand_realization_census_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &equation_chain.seed_census,
            &equation_marginal_set,
            &demand_orbit_census,
        ),
        AuditDecision::Unknown(
            pen_semantic_audit::AuditUnknownReason::MissingDemandRealizationCensusV2
        )
    ));

    // Restricted cost-basis adversaries bind the canonical public-clause
    // identities and both V3 authorities, not merely the shared successor
    // signature. No detachable caller-built cost-V2 certificate is accepted.
    assert!(matches!(
        pen_semantic_audit::diagnose_restricted_kernel_cost_basis_v3(
            &equation_chain.cost_manifest,
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.compatibility,
            &equation_chain.inventory,
            &equation_chain.public_clauses,
            &carrier,
            &equation_authority,
        ),
        Err(pen_semantic_audit::RestrictedKernelCostBasisFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_restricted_kernel_cost_basis_v3(
            &chain.cost_manifest,
            &chain.v1_manifest,
            &chain.v2_manifest,
            &chain.manifest_v3,
            &chain.compatibility,
            &chain.inventory,
            &chain.public_clauses,
            &carrier,
            &equation_authority,
        ),
        Err(pen_semantic_audit::RestrictedKernelCostBasisFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_restricted_kernel_cost_basis_v3(
            &equation_chain.cost_manifest,
            &equation_chain.v1_manifest,
            &equation_chain.v2_manifest,
            &equation_chain.manifest_v3,
            &equation_chain.compatibility,
            &equation_chain.inventory,
            &chain.public_clauses,
            &equation_carrier,
            &equation_authority,
        ),
        AuditDecision::Unknown(pen_semantic_audit::AuditUnknownReason::ManifestMismatch)
    ));

    // The pre-SR2 support census binds the exact quotient/marginal/demand
    // chain and independently replays every raw member's seed lineage.
    assert!(matches!(
        pen_semantic_audit::diagnose_sr2_dependency_support_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &equation_chain.public_clauses,
            &equation_chain.seed_census,
            &equation_quotient,
            &equation_marginal_set,
            &equation_demand_orbit_census,
            &demand_realization_census,
        ),
        Err(pen_semantic_audit::Sr2DependencySupportFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::diagnose_sr2_dependency_support_v3(
            &chain.manifest_v3,
            &chain.inventory,
            &chain.public_clauses,
            &equation_chain.seed_census,
            &quotient,
            &marginal_set,
            &demand_orbit_census,
            &demand_realization_census,
        ),
        Err(pen_semantic_audit::Sr2DependencySupportFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_sr2_dependency_support_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &equation_chain.public_clauses,
            &equation_chain.seed_census,
            &equation_quotient,
            &equation_marginal_set,
            &equation_demand_orbit_census,
            &demand_realization_census,
        ),
        AuditDecision::Unknown(
            pen_semantic_audit::AuditUnknownReason::MissingSr2ProvenanceAssignment
        )
    ));

    // The negative SR2 theorem is likewise non-detachable: a cost or demand
    // capability from the other chain cannot establish a pigeonhole result.
    assert!(matches!(
        pen_semantic_audit::diagnose_sr2_noninjectivity_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &equation_chain.public_clauses,
            &equation_marginal_set,
            &equation_demand_orbit_census,
            &equation_demand_realization_census,
            &cost_basis,
        ),
        Err(pen_semantic_audit::Sr2NonInjectivityFailureV3::ChainBindingMismatch)
    ));
    assert!(matches!(
        pen_semantic_audit::verify_sr2_noninjectivity_v3(
            &equation_chain.manifest_v3,
            &equation_chain.inventory,
            &equation_chain.public_clauses,
            &equation_marginal_set,
            &equation_demand_orbit_census,
            &demand_realization_census,
            &equation_cost_basis,
        ),
        AuditDecision::Unknown(pen_semantic_audit::AuditUnknownReason::ManifestMismatch)
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
    assert!(carrier.raw_families().iter().all(|family| family.rank <= 2));
    assert!(!carrier.tuple_dispositions().is_empty());
}

/// Build the typed rewrite inventory for the fresh-equation chain: the
/// restricted normalizer's fresh program over the predecessor boundary
/// (the fresh owner is the sole successor-new declaration), compiled
/// against the sealed successor equation one-to-one.
fn equation_typed_rewrite_inventory(
    chain: &ChainV3,
) -> pen_semantic_audit::VerifiedTypedRewriteInventoryV1 {
    let program = equation_fresh_program(chain);
    equation_typed_rewrite_inventory_with_program(chain, &program)
}

fn equation_fresh_program(
    chain: &ChainV3,
) -> pen_semantic_audit::VerifiedFreshConstructorComputationV1 {
    equation_fresh_program_for_declaration(chain, declarations()[3].clone())
}

fn other_equation_fresh_program(
    chain: &ChainV3,
) -> pen_semantic_audit::VerifiedFreshConstructorComputationV1 {
    let mut declaration = declarations()[3].clone();
    declaration.id = wire_global(9);
    equation_fresh_program_for_declaration(chain, declaration)
}

fn equation_fresh_program_for_declaration(
    chain: &ChainV3,
    fresh_declaration: Declaration,
) -> pen_semantic_audit::VerifiedFreshConstructorComputationV1 {
    let fresh_request = pen_semantic_audit::FreshConstructorComputationRequestV1 {
        fresh_declaration,
        clauses: vec![pen_semantic_audit::FreshConstructorClauseV1 {
            constructor: wire_global(2),
            scrutinee_parameter_ordinal: 1,
        }],
    };
    match pen_semantic_audit::verify_fresh_constructor_computation_v1(
        &chain.v1_manifest,
        &chain.kernel,
        chain.inventory.predecessor_boundary(),
        &fresh_request,
    ) {
        AuditDecision::Proven(program) => program,
        other => panic!("fresh constructor computation program: {other:?}"),
    }
}

fn equation_typed_rewrite_inventory_with_program(
    chain: &ChainV3,
    fresh_program: &pen_semantic_audit::VerifiedFreshConstructorComputationV1,
) -> pen_semantic_audit::VerifiedTypedRewriteInventoryV1 {
    match pen_semantic_audit::compile_typed_rewrite_inventory_lambda_unit_v1(
        &chain.v1_manifest,
        &chain.inventory,
        fresh_program,
    ) {
        AuditDecision::Proven(inventory) => inventory,
        other => panic!("typed rewrite inventory: {other:?}"),
    }
}

/// Pure-Rust probe: both chain shapes must produce verified inventories,
/// and the fresh-equation chain must compile its typed rewrite inventory
/// through the restricted normalizer.
#[test]
fn both_chain_shapes_produce_verified_inventories() {
    let plain = chain_v3(false);
    assert!(plain.inventory.equations().is_empty());
    let with_equation = chain_v3(true);
    assert_eq!(with_equation.inventory.equations().len(), 1);
    let typed = equation_typed_rewrite_inventory(&with_equation);
    assert_eq!(typed.entries().len(), 1);
}

/// Focused Phase J probe over the equationless chain only (pinned Agda
/// required for the Phase H capabilities feeding the carrier).
#[test]
#[ignore]
fn phase_j_probe_equationless() {
    let chain = chain_v3(false);
    let minted = phase_h_capabilities(&chain);
    let (carrier, root_inventory, subject_bundle, census) = run_phase_i_pipeline(&chain, &minted);
    println!(
        "carrier families {} census occurrences {} roots {}",
        carrier.families().len(),
        census.batch().occurrences().len(),
        root_inventory.roots().len()
    );
    match pen_semantic_audit::diagnose_rewrite_authority_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        &chain.compatibility,
        &carrier,
        &root_inventory,
        &census,
        &subject_bundle,
        None,
    ) {
        Ok(authority) => println!(
            "authority nodes {} edges {} overlaps {} stability {}",
            authority.nodes().len(),
            authority.edges().len(),
            authority.overlap_pairs().len(),
            authority.substitution_stability().len()
        ),
        Err(failure) => panic!("authority failed: {failure}"),
    }
}

/// Focused Phase J probe over the fresh-equation chain.
#[test]
#[ignore]
fn phase_j_probe_equation() {
    let chain = chain_v3(true);
    let minted = phase_h_capabilities(&chain);
    let (carrier, root_inventory, subject_bundle, census) = run_phase_i_pipeline(&chain, &minted);
    let typed_inventory = equation_typed_rewrite_inventory(&chain);
    println!(
        "carrier families {} census occurrences {} roots {} witnesses {}",
        carrier.families().len(),
        census.batch().occurrences().len(),
        root_inventory.roots().len(),
        0
    );
    match pen_semantic_audit::diagnose_rewrite_authority_v3(
        &chain.v1_manifest,
        &chain.v2_manifest,
        &chain.manifest_v3,
        &chain.kernel,
        &chain.inventory,
        &chain.compatibility,
        &carrier,
        &root_inventory,
        &census,
        &subject_bundle,
        Some(&typed_inventory),
    ) {
        Ok(authority) => println!(
            "authority nodes {} edges {} overlaps {} stability {}",
            authority.nodes().len(),
            authority.edges().len(),
            authority.overlap_pairs().len(),
            authority.substitution_stability().len()
        ),
        Err(failure) => panic!("authority failed: {failure}"),
    }
}
