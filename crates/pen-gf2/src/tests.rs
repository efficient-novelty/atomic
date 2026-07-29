use crate::operational::{
    OneNullaryUseDecision, canonical_replay_is_permutation_invariant_for_test,
    decide_one_nullary_use, operational_caps_reject_before_insert_for_test,
    rejects_incomplete_fixed_point_for_test, verify_demand_connected_response,
};
use crate::quotient::exercise_quotient_guards_for_test;
use crate::rewrite::{
    RewriteTamper, rejects_rewrite_tamper_for_test, verify_one_nullary_equation_extension,
};
use crate::{
    VerifiedDemandConnectedResponse, VerifiedDerivedUse, VerifiedEquationExtensionSet,
    VerifiedUnderivedUse, current_gf2_slice_verifier_manifest_v1,
};
use pen_demand::gsc::{
    CLOSED_FORMER_FRAME_SCHEMA_VERSION, CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION, ClosedFormerFrame,
    ClosedInductiveCode, ComputationMode, ConstructorCode, GscOutcome, GscUnknownReason,
    IntroductionAlias, PublicSourceId, TelescopeCode, VerifiedCanonicalDemandFamilyV2,
    VerifiedClosedFormerFrame, VerifiedGscSemanticManifest, VerifiedGscVerifierManifest,
    compile_compute_v1, compile_use_v1, current_gsc_verifier_manifest_v1,
    frozen_gsc_semantic_manifest_v1, verify_closed_former_frame, verify_closed_inductive_code,
    verify_gsc_reference_agreement_v1, verify_gsc_semantic_manifest_v1,
    verify_gsc_verifier_manifest_v1,
};
use pen_kernel::{
    Declaration, Digest, GlobalId, Kernel, KernelLimits, Term, UncheckedSignature,
    VerifiedSignature,
};

struct Fixture {
    kernel: Kernel,
    semantic: VerifiedGscSemanticManifest,
    boundary: VerifiedSignature,
    frame: VerifiedClosedFormerFrame,
    use_family: VerifiedCanonicalDemandFamilyV2,
    compute_family: VerifiedCanonicalDemandFamilyV2,
}

struct RawRun {
    underived: VerifiedUnderivedUse,
    equations: VerifiedEquationExtensionSet,
    derived: VerifiedDerivedUse,
    response: VerifiedDemandConnectedResponse,
}

fn proven<T>(outcome: GscOutcome<T>) -> T {
    match outcome {
        GscOutcome::Proven(value) => value,
        GscOutcome::Unknown(reason) => panic!("expected proof, got {reason:?}"),
    }
}

fn demand_verifier(
    kernel: &Kernel,
    semantic: &VerifiedGscSemanticManifest,
) -> VerifiedGscVerifierManifest {
    let agreement = proven(verify_gsc_reference_agreement_v1(kernel, semantic));
    let candidate = proven(current_gsc_verifier_manifest_v1(
        semantic, kernel, &agreement,
    ));
    proven(verify_gsc_verifier_manifest_v1(
        semantic, kernel, &agreement, &candidate,
    ))
}

fn seeded_id(seed: &[u8], label: &[u8]) -> GlobalId {
    GlobalId(Digest::of_domain_chunks(
        "pen-gf2/synthetic-id/v1",
        &[seed, label],
    ))
}

fn fixture(seed: &[u8], transparent_aliases: bool) -> Fixture {
    fixture_with_higher_order_head(seed, transparent_aliases, false)
}

fn fixture_with_higher_order_head(
    seed: &[u8],
    transparent_aliases: bool,
    add_higher_order_head: bool,
) -> Fixture {
    let kernel = Kernel::new(KernelLimits::default()).expect("valid kernel");
    let semantic = proven(verify_gsc_semantic_manifest_v1(
        &frozen_gsc_semantic_manifest_v1(),
    ));
    let owner = seeded_id(seed, b"owner");
    let introduction = seeded_id(seed, b"introduction");
    let mut declarations = vec![
        Declaration {
            id: owner.clone(),
            ty: Term::Sort { level: 0 },
            body: transparent_aliases.then_some(Term::UnitType),
        },
        Declaration {
            id: introduction.clone(),
            ty: Term::Global { id: owner.clone() },
            body: transparent_aliases.then_some(Term::Unit),
        },
    ];
    if add_higher_order_head {
        declarations.push(Declaration {
            id: seeded_id(seed, b"higher order head"),
            ty: Term::Pi {
                parameter: Box::new(Term::Pi {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::Sort { level: 0 }),
                }),
                body: Box::new(Term::Pi {
                    parameter: Box::new(Term::Pi {
                        parameter: Box::new(Term::UnitType),
                        body: Box::new(Term::UnitType),
                    }),
                    body: Box::new(Term::Pi {
                        parameter: Box::new(Term::UnitType),
                        body: Box::new(Term::Apply {
                            function: Box::new(Term::Var { index: 2 }),
                            argument: Box::new(Term::Var { index: 0 }),
                        }),
                    }),
                }),
            },
            body: None,
        });
    }
    let boundary = kernel
        .verify_signature(&UncheckedSignature { declarations })
        .expect("synthetic boundary");
    let code = proven(verify_closed_inductive_code(
        &semantic,
        &ClosedInductiveCode {
            schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
            universe_level: 0,
            parameters: TelescopeCode::default(),
            indices: TelescopeCode::default(),
            constructors: vec![ConstructorCode {
                arguments: TelescopeCode::default(),
                result_indices: Vec::new(),
                recursive_positions: Vec::new(),
                boundary_ports: Vec::new(),
            }],
            generated_computation_mode: ComputationMode::JudgmentalFreshHead,
        },
    ));
    let constructor = code.constructor_id(0).expect("one constructor").clone();
    let frame = proven(verify_closed_former_frame(
        &semantic,
        &kernel,
        &boundary,
        &code,
        &ClosedFormerFrame {
            schema_version: CLOSED_FORMER_FRAME_SCHEMA_VERSION,
            code_id: code.id().clone(),
            owner,
            introductions: vec![IntroductionAlias {
                constructor: constructor.clone(),
                introduction,
            }],
            source_declarations: vec![seeded_id(seed, b"owner"), seeded_id(seed, b"introduction")],
            principal_sources: vec![
                PublicSourceId(Digest::of_domain_chunks(
                    "pen-gf2/synthetic-source/v1",
                    &[seed, b"owner"],
                )),
                PublicSourceId(Digest::of_domain_chunks(
                    "pen-gf2/synthetic-source/v1",
                    &[seed, b"introduction"],
                )),
            ],
            birth_support: Vec::new(),
        },
    ));
    let use_family = proven(compile_use_v1(&semantic, &kernel, &boundary, &frame));
    let use_port = use_family.ports()[0].key().clone();
    let compute_family = proven(compile_compute_v1(
        &semantic,
        &kernel,
        &boundary,
        &frame,
        &use_family,
        &use_port,
        &constructor,
    ));
    Fixture {
        kernel,
        semantic,
        boundary,
        frame,
        use_family,
        compute_family,
    }
}

fn raw_run(fixture: &Fixture) -> RawRun {
    let underived = match proven(decide_one_nullary_use(
        &fixture.kernel,
        &fixture.boundary,
        &fixture.semantic,
        &fixture.use_family,
    )) {
        OneNullaryUseDecision::Underived(certificate) => certificate,
        OneNullaryUseDecision::Derived(_) => panic!("library unexpectedly filled the use port"),
    };
    let equations = proven(verify_one_nullary_equation_extension(
        &fixture.kernel,
        &fixture.boundary,
        &fixture.semantic,
        &fixture.frame,
        &fixture.use_family,
        &fixture.compute_family,
    ));
    let derived = match proven(decide_one_nullary_use(
        &fixture.kernel,
        equations.extended_signature(),
        &fixture.semantic,
        &fixture.use_family,
    )) {
        OneNullaryUseDecision::Derived(certificate) => certificate,
        OneNullaryUseDecision::Underived(_) => panic!("fresh head did not fill the use port"),
    };
    let response = proven(verify_demand_connected_response(
        &fixture.use_family,
        &fixture.compute_family,
        &equations,
        derived.clone(),
    ));
    RawRun {
        underived,
        equations,
        derived,
        response,
    }
}

#[test]
fn exact_one_nullary_slice_is_typed_and_demand_connected() {
    let fixture = fixture(b"alias fixture", true);
    let run = raw_run(&fixture);

    assert!(run.underived.is_complete_relative_to_grammar());
    assert!(run.underived.canonical_forms().has_single_outer_function());
    assert!(run.underived.canonical_forms().has_stuck_atomic_body());
    assert!(
        run.underived
            .canonical_forms()
            .neutral_heads_are_exhausted()
    );
    assert!(
        run.underived
            .canonical_forms()
            .has_no_higher_order_argument_gap()
    );
    assert!(
        run.underived
            .canonical_forms()
            .introduction_rules_are_separated()
    );
    assert_eq!(
        run.underived.dispositions().len(),
        crate::ORDERED_OPERATIONAL_RULES.len()
    );
    assert!(
        run.underived
            .dispositions()
            .iter()
            .all(|disposition| disposition.is_exhaustively_decided())
    );
    assert_eq!(
        run.underived.grounded_rule_instances(),
        run.underived
            .dispositions()
            .iter()
            .map(|disposition| disposition.grounded_instances())
            .sum::<u32>()
    );
    assert_eq!(run.underived.structural_goal_count(), 2);
    assert!(!run.underived.closure_frontier_digests().is_empty());
    for digest in [
        run.underived.closure_base_digest(),
        run.underived.closure_fixed_point_digest(),
        run.underived.goal_rule_coverage_digest(),
        run.underived.canonical_replay_digest(),
        run.underived.grounding_inventory_digest(),
        run.underived.library_inventory_digest(),
    ] {
        assert_ne!(digest, &Digest::of_bytes(b""));
    }
    let q0 = run
        .underived
        .dispositions()
        .iter()
        .find(|disposition| disposition.rule() == crate::OperationalRule::Q0Conversion)
        .expect("Q0 disposition");
    assert_eq!(q0.grounded_instances(), run.underived.library_nodes());
    for disposition in run.underived.dispositions() {
        assert_eq!(
            disposition.certified_inapplicable_instances(),
            disposition
                .inapplicability_evidence()
                .iter()
                .map(|evidence| evidence.instances())
                .sum::<u32>()
        );
    }
    let application = run
        .underived
        .dispositions()
        .iter()
        .find(|disposition| disposition.rule() == crate::OperationalRule::Application)
        .expect("application disposition");
    assert!(
        application
            .inapplicability_evidence()
            .iter()
            .any(|evidence| {
                evidence.reason()
                    == crate::CertifiedInapplicabilityReason::PrincipalIsNotDependentFunction
            })
    );
    let public_exact = run
        .underived
        .dispositions()
        .iter()
        .find(|disposition| disposition.rule() == crate::OperationalRule::PublicExact)
        .expect("public-exact disposition");
    assert!(
        public_exact.applicable_instances() >= 8,
        "same-typed transparent aliases must remain distinct typed conclusions"
    );
    assert!(
        application
            .inapplicability_evidence()
            .iter()
            .any(|evidence| {
                evidence.reason() == crate::CertifiedInapplicabilityReason::ArgumentTypeMismatch
            })
    );

    let invariants = run.equations.clause().invariants();
    assert!(invariants.is_left_linear());
    assert!(invariants.is_nonrecursive());
    assert!(invariants.has_no_critical_overlaps());
    assert!(invariants.is_terminating_in_admitted_fragment());
    assert!(invariants.is_confluent_in_admitted_fragment());
    assert!(invariants.is_conservative_on_old_terms());
    assert!(invariants.exact_generated_substitution_preserves_equation());
    assert_eq!(run.equations.clauses_len(), 1);
    assert_eq!(
        run.equations
            .clause()
            .substitution()
            .source_context()
            .0
            .len(),
        3
    );
    assert_eq!(
        run.equations
            .clause()
            .substitution()
            .target_context()
            .0
            .len(),
        2
    );
    assert_eq!(
        run.derived.term(),
        run.equations.clause().substitution().filler()
    );
    assert_eq!(
        run.derived.derivation().conclusion().rule(),
        crate::OperationalRule::Application
    );
    assert!(run.response.all_outputs_filled());
    assert!(run.response.all_positive_clauses_connected());
    assert_eq!(run.response.discharges().len(), 2);
    assert!(exercise_quotient_guards_for_test(
        &fixture.semantic,
        &fixture.boundary,
        &run.equations,
        &run.response,
    ));
}

#[test]
fn every_rewrite_tamper_fails_closed() {
    let fixture = fixture(b"tamper fixture", false);
    for tamper in [
        RewriteTamper::AlteredHead,
        RewriteTamper::AlteredConstructor,
        RewriteTamper::ReversedOrientation,
        RewriteTamper::AlteredRightSide,
        RewriteTamper::AlteredType,
        RewriteTamper::BodyfulHead,
        RewriteTamper::DuplicateRule,
        RewriteTamper::MissingRule,
        RewriteTamper::OldHeadRewrite,
    ] {
        assert!(
            rejects_rewrite_tamper_for_test(
                &fixture.kernel,
                &fixture.boundary,
                &fixture.semantic,
                &fixture.frame,
                &fixture.use_family,
                &fixture.compute_family,
                tamper,
            ),
            "tamper was accepted: {tamper:?}"
        );
    }
}

#[test]
fn opaque_identifier_permutation_preserves_disposition_not_binding() {
    let first_fixture = fixture(b"permutation one", true);
    let second_fixture = fixture(b"permutation two", true);
    let first = raw_run(&first_fixture);
    let second = raw_run(&second_fixture);

    let disposition_shape = |certificate: &VerifiedUnderivedUse| {
        certificate
            .dispositions()
            .iter()
            .map(|disposition| {
                (
                    disposition.rule(),
                    disposition.grounded_instances(),
                    disposition.applicable_instances(),
                    disposition.certified_inapplicable_instances(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        disposition_shape(&first.underived),
        disposition_shape(&second.underived)
    );
    assert_eq!(
        first.equations.clauses_len(),
        second.equations.clauses_len()
    );
    assert_eq!(
        first.response.discharges().len(),
        second.response.discharges().len()
    );
    assert_ne!(first.underived.digest(), second.underived.digest());
    assert_ne!(first.equations.digest(), second.equations.digest());
    assert_ne!(first.response.digest(), second.response.digest());
}

#[test]
fn mismatched_family_tuple_and_resource_exhaustion_are_unknown() {
    let first = fixture(b"family one", false);
    let second = fixture(b"family two", false);
    assert!(matches!(
        verify_one_nullary_equation_extension(
            &first.kernel,
            &first.boundary,
            &first.semantic,
            &first.frame,
            &first.use_family,
            &second.compute_family,
        ),
        GscOutcome::Unknown(GscUnknownReason::FamilyMismatch)
            | GscOutcome::Unknown(GscUnknownReason::PortMismatch)
    ));

    let first_equations = proven(verify_one_nullary_equation_extension(
        &first.kernel,
        &first.boundary,
        &first.semantic,
        &first.frame,
        &first.use_family,
        &first.compute_family,
    ));
    let first_derived = match proven(decide_one_nullary_use(
        &first.kernel,
        first_equations.extended_signature(),
        &first.semantic,
        &first.use_family,
    )) {
        OneNullaryUseDecision::Derived(derived) => derived,
        OneNullaryUseDecision::Underived(_) => panic!("fresh head must derive"),
    };
    let second_equations = proven(verify_one_nullary_equation_extension(
        &second.kernel,
        &second.boundary,
        &second.semantic,
        &second.frame,
        &second.use_family,
        &second.compute_family,
    ));
    assert!(matches!(
        verify_demand_connected_response(
            &first.use_family,
            &first.compute_family,
            &second_equations,
            first_derived,
        ),
        GscOutcome::Unknown(GscUnknownReason::PortMismatch)
    ));

    let tiny = Kernel::new(KernelLimits {
        max_operations: 1,
        max_depth: 8,
        normalization_fuel: 8,
    })
    .expect("positive limits");
    assert!(matches!(
        verify_one_nullary_equation_extension(
            &tiny,
            &first.boundary,
            &first.semantic,
            &first.frame,
            &first.use_family,
            &first.compute_family,
        ),
        GscOutcome::Unknown(GscUnknownReason::ResourceExhausted)
    ));
}

#[test]
#[ignore = "requires the exact pinned Agda executable and version"]
fn verifier_manifest_is_bound_and_rejects_unknown_wire_fields() {
    let fixture = fixture(b"verifier manifest", false);
    let demand_verifier = demand_verifier(&fixture.kernel, &fixture.semantic);
    let manifest = proven(current_gf2_slice_verifier_manifest_v1(
        &fixture.semantic,
        &fixture.kernel,
        &demand_verifier,
    ));
    assert_eq!(
        manifest.semantic_manifest_digest,
        *fixture.semantic.digest()
    );
    assert_ne!(manifest.verifier_source_digest, Digest::of_bytes(b""));

    let mut wire = serde_json::to_value(&manifest).expect("serialize manifest");
    wire.as_object_mut()
        .expect("manifest object")
        .insert("unrecognized".to_owned(), serde_json::Value::Bool(true));
    assert!(serde_json::from_value::<crate::Gf2SliceVerifierManifestV1>(wire).is_err());
}

#[test]
fn canonical_replay_is_permutation_invariant_and_caps_fail_before_insert() {
    let fixture = fixture(b"canonical replay", true);
    assert!(canonical_replay_is_permutation_invariant_for_test(
        &fixture.kernel,
        &fixture.boundary,
        &fixture.semantic,
        &fixture.use_family,
    ));
    assert!(operational_caps_reject_before_insert_for_test(
        &fixture.kernel,
        &fixture.boundary,
        &fixture.semantic,
        &fixture.use_family,
    ));
}

#[test]
fn missing_higher_order_introduction_enumeration_is_unknown_not_underived() {
    let fixture = fixture_with_higher_order_head(b"higher order gap", true, true);
    assert!(matches!(
        decide_one_nullary_use(
            &fixture.kernel,
            &fixture.boundary,
            &fixture.semantic,
            &fixture.use_family,
        ),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedCode)
    ));
}

#[test]
fn unfinished_saturation_bound_is_unknown() {
    let fixture = fixture(b"fixed point bound", false);
    assert!(rejects_incomplete_fixed_point_for_test(
        &fixture.kernel,
        &fixture.boundary,
        &fixture.semantic,
        &fixture.use_family,
    ));
}
