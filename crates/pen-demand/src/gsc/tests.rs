use super::*;
use pen_gf2_agda::AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST;
use pen_kernel::{
    Declaration, Digest, GlobalId, Kernel, KernelLimits, OpenJudgment, Term, UncheckedSignature,
    VerifiedSignature,
};

fn kernel() -> Kernel {
    Kernel::new(KernelLimits::default()).expect("valid limits")
}

fn global(label: &[u8]) -> GlobalId {
    GlobalId(Digest::of_domain_bytes(
        "pen-demand/gsc-test-global/v1",
        label,
    ))
}

fn manifest() -> VerifiedGscSemanticManifest {
    match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
        GscOutcome::Proven(manifest) => manifest,
        GscOutcome::Unknown(reason) => panic!("frozen manifest was not verified: {reason:?}"),
    }
}

fn reference_agreement(
    kernel: &Kernel,
    manifest: &VerifiedGscSemanticManifest,
) -> VerifiedGscReferenceAgreement {
    proven(verify_gsc_reference_agreement_v1(kernel, manifest))
}

fn nullary_constructor() -> ConstructorCode {
    ConstructorCode {
        arguments: TelescopeCode::default(),
        result_indices: Vec::new(),
        recursive_positions: Vec::new(),
        boundary_ports: Vec::new(),
    }
}

fn two_nullary_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![nullary_constructor(), nullary_constructor()],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    }
}

fn nonrecursive_argument_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![ConstructorCode {
            arguments: TelescopeCode(vec![CodeTerm::Sort { level: 0 }]),
            result_indices: Vec::new(),
            recursive_positions: Vec::new(),
            boundary_ports: Vec::new(),
        }],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    }
}

fn recursive_argument_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![ConstructorCode {
            arguments: TelescopeCode(vec![CodeTerm::OwnerApp {
                parameters: Vec::new(),
                indices: Vec::new(),
            }]),
            result_indices: Vec::new(),
            recursive_positions: vec![0],
            boundary_ports: Vec::new(),
        }],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    }
}

fn path_mode_code() -> ClosedInductiveCode {
    ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![nullary_constructor()],
        generated_computation_mode: ComputationMode::PathTerm,
    }
}

fn constructor_type(code: &ClosedInductiveCode, owner: &GlobalId, index: usize) -> Term {
    let mut result = Term::Global { id: owner.clone() };
    for argument in code.constructors[index].arguments.0.iter().rev() {
        let parameter = match argument {
            CodeTerm::Sort { level } => Term::Sort { level: *level },
            CodeTerm::OwnerApp {
                parameters,
                indices,
            } if parameters.is_empty() && indices.is_empty() => Term::Global { id: owner.clone() },
            _ => panic!("test fixture uses only the reviewed argument subset"),
        };
        result = Term::Pi {
            parameter: Box::new(parameter),
            body: Box::new(result),
        };
    }
    result
}

struct Fixture {
    signature: VerifiedSignature,
    code: VerifiedClosedInductiveCode,
    frame: VerifiedClosedFormerFrame,
}

fn fixture(
    kernel: &Kernel,
    manifest: &VerifiedGscSemanticManifest,
    code: ClosedInductiveCode,
    namespace: &[u8],
) -> Fixture {
    let owner = global(&[namespace, b"/owner"].concat());
    let introductions = (0..code.constructors.len())
        .map(|index| global(&[namespace, format!("/intro-{index}").as_bytes()].concat()))
        .collect::<Vec<_>>();
    let mut declarations = vec![Declaration {
        id: owner.clone(),
        ty: Term::Sort { level: 0 },
        body: None,
    }];
    declarations.extend(
        introductions
            .iter()
            .enumerate()
            .map(|(index, introduction)| Declaration {
                id: introduction.clone(),
                ty: constructor_type(&code, &owner, index),
                body: None,
            }),
    );
    let signature = kernel
        .verify_signature(&UncheckedSignature { declarations })
        .expect("synthetic signature verifies");
    let code = match verify_closed_inductive_code(manifest, &code) {
        GscOutcome::Proven(code) => code,
        GscOutcome::Unknown(reason) => panic!("synthetic code was not verified: {reason:?}"),
    };
    let origin = GscOriginEventId(Digest::of_domain_bytes(
        "pen-demand/gsc-test-origin/v1",
        namespace,
    ));
    let mut source_declarations = vec![owner.clone()];
    source_declarations.extend(introductions.iter().cloned());
    let frame = ClosedFormerFrame {
        schema_version: CLOSED_FORMER_FRAME_SCHEMA_VERSION,
        code_id: code.id().clone(),
        owner,
        introductions: code
            .constructor_ids()
            .iter()
            .cloned()
            .zip(introductions)
            .map(|(constructor, introduction)| IntroductionAlias {
                constructor,
                introduction,
            })
            .collect(),
        principal_sources: source_declarations
            .iter()
            .map(|declaration| PublicSourceId::for_declaration(&origin, declaration))
            .collect(),
        source_declarations,
        birth_support: vec![origin],
    };
    let frame = match verify_closed_former_frame(manifest, kernel, &signature, &code, &frame) {
        GscOutcome::Proven(frame) => frame,
        GscOutcome::Unknown(reason) => panic!("synthetic frame was not verified: {reason:?}"),
    };
    Fixture {
        signature,
        code,
        frame,
    }
}

fn fixture_with_body_only_dependency(
    kernel: &Kernel,
    manifest: &VerifiedGscSemanticManifest,
) -> (Fixture, GlobalId) {
    let code_wire = ClosedInductiveCode {
        schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
        universe_level: 0,
        parameters: TelescopeCode::default(),
        indices: TelescopeCode::default(),
        constructors: vec![nullary_constructor()],
        generated_computation_mode: ComputationMode::JudgmentalFreshHead,
    };
    let body_only = global(b"type-support/body-only");
    let owner = global(b"type-support/owner");
    let introduction = global(b"type-support/introduction");
    let signature = kernel
        .verify_signature(&UncheckedSignature {
            declarations: vec![
                Declaration {
                    id: body_only.clone(),
                    ty: Term::UnitType,
                    body: None,
                },
                Declaration {
                    id: owner.clone(),
                    ty: Term::Sort { level: 0 },
                    body: Some(Term::UnitType),
                },
                Declaration {
                    id: introduction.clone(),
                    ty: Term::Global { id: owner.clone() },
                    body: Some(Term::Global {
                        id: body_only.clone(),
                    }),
                },
            ],
        })
        .expect("body-only dependency signature verifies");
    let code = proven(verify_closed_inductive_code(manifest, &code_wire));
    let origin = GscOriginEventId(Digest::of_domain_bytes(
        "pen-demand/gsc-test-origin/v1",
        b"type-support",
    ));
    let source_declarations = vec![owner.clone(), introduction.clone()];
    let frame_wire = ClosedFormerFrame {
        schema_version: CLOSED_FORMER_FRAME_SCHEMA_VERSION,
        code_id: code.id().clone(),
        owner,
        introductions: vec![IntroductionAlias {
            constructor: code.constructor_ids()[0].clone(),
            introduction,
        }],
        principal_sources: source_declarations
            .iter()
            .map(|declaration| PublicSourceId::for_declaration(&origin, declaration))
            .collect(),
        source_declarations,
        birth_support: vec![origin],
    };
    let frame = proven(verify_closed_former_frame(
        manifest,
        kernel,
        &signature,
        &code,
        &frame_wire,
    ));
    (
        Fixture {
            signature,
            code,
            frame,
        },
        body_only,
    )
}

fn proven<T>(outcome: GscOutcome<T>) -> T {
    match outcome {
        GscOutcome::Proven(value) => value,
        GscOutcome::Unknown(reason) => panic!("expected proof, found {reason:?}"),
    }
}

#[test]
fn semantic_manifest_digest_is_frozen_before_execution() {
    let semantic = manifest();
    assert_eq!(
        semantic.digest().as_str(),
        "blake3:d61458ebd47036861e48af9ef458df1b2b3dc890194069958ef2f14d4afdd11e"
    );
}

#[test]
fn semantic_manifest_excludes_verifier_fields_and_rejects_mutation() {
    let semantic = manifest();
    let semantic_wire =
        serde_json::to_value(semantic.manifest()).expect("serialize semantic manifest");
    let object = semantic_wire.as_object().expect("semantic object");
    for verifier_field in [
        "rust_verifier_digest",
        "agda_reference_digest",
        "toolchain_digest",
        "kernel_protocol_digest",
        "normalizer_protocol_digest",
        "reference_vector_agreement",
    ] {
        assert!(!object.contains_key(verifier_field));
    }
    let serialized = serde_json::to_string(semantic.manifest()).expect("serialize");
    for forbidden_label in ["\"h3\"", "\"unit\"", "\"pi\"", "\"sigma\""] {
        assert!(!serialized.contains(forbidden_label));
    }

    let mut changed = frozen_gsc_semantic_manifest_v1();
    changed.semantic_limits.max_constructors -= 1;
    assert!(matches!(
        verify_gsc_semantic_manifest_v1(&changed),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest)
    ));
}

#[test]
#[ignore = "requires the exact pinned Agda 2.8.0 executable on PATH"]
fn live_reference_agreement_mints_pinned_capability_and_verifier_manifest() {
    let semantic = manifest();
    let kernel = kernel();
    let agreement = reference_agreement(&kernel, &semantic);
    let verifier = proven(current_gsc_verifier_manifest_v1(
        &semantic, &kernel, &agreement,
    ));
    assert_eq!(&verifier.semantic_manifest_digest, semantic.digest());
    assert_ne!(verifier.rust_verifier_digest, *semantic.digest());
    assert_ne!(verifier.agda_reference_digest, *semantic.digest());
    assert_ne!(verifier.toolchain_digest, *semantic.digest());
    assert_eq!(
        verifier.kernel_protocol_digest,
        kernel.kernel_protocol_digest()
    );
    assert_eq!(
        verifier.normalizer_protocol_digest,
        kernel.normalizer_protocol_digest()
    );
    assert_eq!(
        verifier.reference_vector_agreement.status,
        GscReferenceVectorAgreement::ProvenRustReplayAndPinnedPrimitiveSafeAgdaProofV2
    );
    assert_eq!(
        verifier.reference_vector_agreement.vector_suite_digest,
        semantic
            .manifest()
            .reference_vector_suite
            .canonical_digest()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .rust_replay_source_digest,
        verifier.rust_verifier_digest
    );
    assert_eq!(
        verifier.reference_vector_agreement.agda_proof_source_digest,
        verifier.agda_reference_digest
    );
    assert_eq!(
        verifier.reference_vector_agreement.gate_protocol_digest,
        gsc_reference_agreement_gate_digest()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .rust_runtime_replay_digest,
        *agreement.rust_replay_digest()
    );
    assert_eq!(
        verifier.reference_vector_agreement.agda_executable_digest,
        *agreement.agda_executable_digest()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .agda_primitive_tree_digest
            .as_str(),
        AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .agda_primitive_tree_digest,
        *agreement.agda_primitive_tree_digest()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .agda_primitive_source_file_count,
        agreement.agda_primitive_source_file_count()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .agda_primitive_source_byte_length,
        agreement.agda_primitive_source_byte_length()
    );
    assert!(
        verifier
            .reference_vector_agreement
            .agda_primitive_source_file_count
            > 0
    );
    assert!(
        verifier
            .reference_vector_agreement
            .agda_primitive_source_byte_length
            > 0
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .agda_data_dir_probe_argument_protocol_digest,
        *agreement.agda_data_dir_probe_argument_protocol_digest()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .agda_data_dir_probe_stdout_digest,
        *agreement.agda_data_dir_probe_stdout_digest()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .agda_data_dir_probe_stderr_digest,
        *agreement.agda_data_dir_probe_stderr_digest()
    );
    assert_eq!(
        verifier
            .reference_vector_agreement
            .verified_capability_digest,
        *agreement.digest()
    );

    let semantic_wire =
        serde_json::to_value(semantic.manifest()).expect("serialize semantic manifest");
    let object = semantic_wire.as_object().expect("semantic object");
    assert!(!object.contains_key("rust_verifier_digest"));
    assert!(!object.contains_key("agda_reference_digest"));
    assert!(!object.contains_key("toolchain_digest"));
    assert!(!object.contains_key("kernel_protocol_digest"));
    assert!(!object.contains_key("normalizer_protocol_digest"));
    assert!(!object.contains_key("reference_vector_agreement"));
    let serialized = serde_json::to_string(semantic.manifest()).expect("serialize");
    for forbidden_label in ["\"h3\"", "\"unit\"", "\"pi\"", "\"sigma\""] {
        assert!(!serialized.contains(forbidden_label));
    }

    let mut changed = frozen_gsc_semantic_manifest_v1();
    changed.semantic_limits.max_constructors -= 1;
    assert!(matches!(
        verify_gsc_semantic_manifest_v1(&changed),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest)
    ));

    assert_eq!(
        semantic.manifest().derivability_grammar.ordered_rules,
        vec![
            OperationalDerivationRule::PublicExact,
            OperationalDerivationRule::Q0Conversion,
            OperationalDerivationRule::ContextWeakening,
            OperationalDerivationRule::CheckedSubstitution,
            OperationalDerivationRule::LambdaIntroduction,
            OperationalDerivationRule::Application,
            OperationalDerivationRule::PairIntroduction,
            OperationalDerivationRule::FirstProjection,
            OperationalDerivationRule::SecondProjection,
            OperationalDerivationRule::EquationReplay,
            OperationalDerivationRule::QuotientTransport,
        ]
    );
    let response = semantic.manifest().response_candidate_grammar;
    assert_eq!(response.max_live_use_families, 256);
    assert_eq!(response.max_compute_families_per_use, 256);
    assert_eq!(response.max_equations_per_candidate, 256);
    assert_eq!(response.max_response_candidates, 256);
    assert_eq!(
        response.generated_equation_coverage,
        GeneratedEquationCoverage::EveryComputeEquationReferencingExactUsePortV1
    );

    let verified = verify_gsc_verifier_manifest_v1(&semantic, &kernel, &agreement, &verifier);
    assert!(matches!(verified, GscOutcome::Proven(_)));
    let mut changed_tree = verifier.clone();
    changed_tree
        .reference_vector_agreement
        .agda_primitive_tree_digest = Digest::of_bytes(b"changed primitive tree");
    assert!(matches!(
        verify_gsc_verifier_manifest_v1(&semantic, &kernel, &agreement, &changed_tree),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier)
    ));
    let mut changed_tree_count = verifier.clone();
    changed_tree_count
        .reference_vector_agreement
        .agda_primitive_source_file_count += 1;
    assert!(matches!(
        verify_gsc_verifier_manifest_v1(&semantic, &kernel, &agreement, &changed_tree_count),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier)
    ));
    let mut changed_probe = verifier.clone();
    changed_probe
        .reference_vector_agreement
        .agda_data_dir_probe_stdout_digest = Digest::of_bytes(b"changed data-dir stdout");
    assert!(matches!(
        verify_gsc_verifier_manifest_v1(&semantic, &kernel, &agreement, &changed_probe),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier)
    ));
    let mut changed_verifier = verifier;
    changed_verifier.normalizer_protocol_digest = Digest::of_bytes(b"changed normalizer");
    assert!(matches!(
        verify_gsc_verifier_manifest_v1(&semantic, &kernel, &agreement, &changed_verifier),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier)
    ));
}

#[test]
fn two_nullary_vector_compiles_every_use_and_compute_port() {
    let kernel = kernel();
    let manifest = manifest();
    let fixture = fixture(&kernel, &manifest, two_nullary_code(), b"two-nullary");
    let use_family = proven(compile_use_v1(
        &manifest,
        &kernel,
        &fixture.signature,
        &fixture.frame,
    ));
    assert_eq!(use_family.rule(), GscRule::Use);
    assert_eq!(use_family.rank(), 1);
    assert_eq!(use_family.parameter_context().0.len(), 3);
    assert_eq!(use_family.ports().len(), 1);
    let use_port = use_family.ports()[0].key().clone();

    for constructor in fixture.code.constructor_ids() {
        let computation = proven(compile_compute_v1(
            &manifest,
            &kernel,
            &fixture.signature,
            &fixture.frame,
            &use_family,
            &use_port,
            constructor,
        ));
        assert_eq!(computation.rule(), GscRule::Compute);
        assert_eq!(computation.rank(), 2);
        assert_eq!(computation.premise_refs().len(), 1);
        assert!(matches!(
            &computation.premise_refs()[0].source,
            PortRef::Generated { port } if port == &use_port
        ));
        let code = computation.output_clauses()[0]
            .equation_code()
            .expect("equation port");
        assert_eq!(code.use_port(), &use_port);
        assert_eq!(code.constructor(), constructor);
    }
}

#[test]
fn rust_reference_vectors_match_frozen_suite() {
    let kernel = kernel();
    let manifest = manifest();

    let two_nullary = fixture(
        &kernel,
        &manifest,
        two_nullary_code(),
        b"reference/two-nullary",
    );
    let two_nullary_use = proven(compile_use_v1(
        &manifest,
        &kernel,
        &two_nullary.signature,
        &two_nullary.frame,
    ));
    for constructor in two_nullary.code.constructor_ids() {
        proven(compile_compute_v1(
            &manifest,
            &kernel,
            &two_nullary.signature,
            &two_nullary.frame,
            &two_nullary_use,
            two_nullary_use.ports()[0].key(),
            constructor,
        ));
    }

    let nonrecursive = fixture(
        &kernel,
        &manifest,
        nonrecursive_argument_code(),
        b"reference/nonrecursive",
    );
    let nonrecursive_use = proven(compile_use_v1(
        &manifest,
        &kernel,
        &nonrecursive.signature,
        &nonrecursive.frame,
    ));
    proven(compile_compute_v1(
        &manifest,
        &kernel,
        &nonrecursive.signature,
        &nonrecursive.frame,
        &nonrecursive_use,
        nonrecursive_use.ports()[0].key(),
        &nonrecursive.code.constructor_ids()[0],
    ));

    let recursive = fixture(
        &kernel,
        &manifest,
        recursive_argument_code(),
        b"reference/recursive",
    );
    let recursive_use = proven(compile_use_v1(
        &manifest,
        &kernel,
        &recursive.signature,
        &recursive.frame,
    ));
    let recursive_compute = proven(compile_compute_v1(
        &manifest,
        &kernel,
        &recursive.signature,
        &recursive.frame,
        &recursive_use,
        recursive_use.ports()[0].key(),
        &recursive.code.constructor_ids()[0],
    ));
    let recursive_term = match &recursive_compute.verification_judgments()[1] {
        OpenJudgment::HasType { term, .. } => term,
        _ => panic!("recursive beta right side must be a typing judgment"),
    };
    assert!(matches!(
        recursive_term,
        Term::Apply {
            argument,
            ..
        } if matches!(
            argument.as_ref(),
            Term::Apply {
                function,
                argument,
            } if function.as_ref() == &Term::Var { index: 1 }
                && argument.as_ref() == &Term::Var { index: 0 }
        )
    ));

    let path = fixture(&kernel, &manifest, path_mode_code(), b"reference/path");
    assert!(matches!(
        compile_use_v1(&manifest, &kernel, &path.signature, &path.frame),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedComputationMode)
    ));

    let actual = GscReferenceVectorSuiteV1 {
        schema_version: 1,
        spec_token: GSC_REFERENCE_VECTOR_SPEC_TOKEN.to_owned(),
        ordered_cases: vec![
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::TwoNullaryConstructors,
                expected_disposition: GscReferenceVectorDisposition::ProvenSupported,
            },
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::OneNonrecursiveArgument,
                expected_disposition: GscReferenceVectorDisposition::ProvenSupported,
            },
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::OneRecursiveArgumentGeneratedCallBeta,
                expected_disposition:
                    GscReferenceVectorDisposition::ProvenGeneratedRecursiveCallBeta,
            },
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::PathComputationMode,
                expected_disposition:
                    GscReferenceVectorDisposition::UnknownUnsupportedComputationMode,
            },
        ],
    };
    assert_eq!(actual, manifest.manifest().reference_vector_suite);

    let agda = include_str!("../../agda/GscInductiveCoreV1.agda");
    assert!(agda.contains(GSC_REFERENCE_VECTOR_SPEC_TOKEN));
    assert!(agda.contains("fourVectorDispositions = refl"));

    let replay = proven(replay_gsc_reference_vectors_v1(&kernel, &manifest));
    assert_eq!(
        replay.ordered_dispositions(),
        actual
            .ordered_cases
            .iter()
            .map(|case| case.expected_disposition)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        replay.vector_suite_digest(),
        &manifest
            .manifest()
            .reference_vector_suite
            .canonical_digest()
    );
}

#[test]
fn nonrecursive_argument_vector_compiles_typed_method_and_beta_context() {
    let kernel = kernel();
    let manifest = manifest();
    let fixture = fixture(
        &kernel,
        &manifest,
        nonrecursive_argument_code(),
        b"nonrecursive",
    );
    let use_family = proven(compile_use_v1(
        &manifest,
        &kernel,
        &fixture.signature,
        &fixture.frame,
    ));
    assert_eq!(use_family.parameter_context().0.len(), 2);
    let computation = proven(compile_compute_v1(
        &manifest,
        &kernel,
        &fixture.signature,
        &fixture.frame,
        &use_family,
        use_family.ports()[0].key(),
        &fixture.code.constructor_ids()[0],
    ));
    assert_eq!(computation.output_clauses()[0].context().0.len(), 4);
    assert_eq!(computation.verification_judgments().len(), 2);
}

#[test]
fn recursive_argument_vector_uses_generated_eliminator_for_its_beta_hypothesis() {
    let kernel = kernel();
    let manifest = manifest();
    let fixture = fixture(&kernel, &manifest, recursive_argument_code(), b"recursive");
    let use_family = proven(compile_use_v1(
        &manifest,
        &kernel,
        &fixture.signature,
        &fixture.frame,
    ));
    let computation = proven(compile_compute_v1(
        &manifest,
        &kernel,
        &fixture.signature,
        &fixture.frame,
        &use_family,
        use_family.ports()[0].key(),
        &fixture.code.constructor_ids()[0],
    ));
    assert_eq!(
        computation.output_clauses()[0].context().0.len(),
        4,
        "parameters, exact use premise, and recursive argument; no free IH binder"
    );
    assert_eq!(
        computation.verification_judgments()[1],
        OpenJudgment::HasType {
            context: computation.output_clauses()[0].context().clone(),
            term: Term::Apply {
                function: Box::new(Term::Apply {
                    function: Box::new(Term::Var { index: 2 }),
                    argument: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Apply {
                    function: Box::new(Term::Var { index: 1 }),
                    argument: Box::new(Term::Var { index: 0 }),
                }),
            },
            ty: Term::Apply {
                function: Box::new(Term::Var { index: 3 }),
                argument: Box::new(Term::Apply {
                    function: Box::new(Term::Global {
                        id: fixture.frame.introductions()[0].introduction.clone(),
                    }),
                    argument: Box::new(Term::Var { index: 0 }),
                }),
            },
        },
        "recursive beta uses the exact generated eliminator premise recursively"
    );
}

#[test]
fn path_mode_vector_is_valid_code_but_compiler_fails_closed() {
    let kernel = kernel();
    let manifest = manifest();
    let fixture = fixture(&kernel, &manifest, path_mode_code(), b"path-mode");
    assert_eq!(
        fixture.code.code().generated_computation_mode,
        ComputationMode::PathTerm
    );
    assert!(matches!(
        compile_use_v1(&manifest, &kernel, &fixture.signature, &fixture.frame),
        GscOutcome::Unknown(GscUnknownReason::UnsupportedComputationMode)
    ));
}

#[test]
fn recursive_mask_and_exact_port_tampering_do_not_cross_the_boundary() {
    let manifest = manifest();
    let mut malformed = recursive_argument_code();
    malformed.constructors[0].recursive_positions.clear();
    assert!(matches!(
        verify_closed_inductive_code(&manifest, &malformed),
        GscOutcome::Unknown(GscUnknownReason::MalformedCode)
    ));

    let kernel = kernel();
    let fixture = fixture(&kernel, &manifest, two_nullary_code(), b"port-tamper");
    let use_family = proven(compile_use_v1(
        &manifest,
        &kernel,
        &fixture.signature,
        &fixture.frame,
    ));
    let genuine = use_family.ports()[0].key();
    let forged = PortKey::new(
        genuine.family_id().clone(),
        OutputPortId(Digest::of_bytes(b"forged-output-port")),
    );
    assert!(matches!(
        compile_compute_v1(
            &manifest,
            &kernel,
            &fixture.signature,
            &fixture.frame,
            &use_family,
            &forged,
            &fixture.code.constructor_ids()[0],
        ),
        GscOutcome::Unknown(GscUnknownReason::PortMismatch)
    ));
}

#[test]
fn owner_occurrences_in_parameter_and_index_telescopes_are_rejected() {
    let manifest = manifest();
    let mut parameter_owner = two_nullary_code();
    parameter_owner.parameters = TelescopeCode(vec![CodeTerm::OwnerApp {
        parameters: vec![CodeTerm::Sort { level: 0 }],
        indices: Vec::new(),
    }]);
    assert!(matches!(
        verify_closed_inductive_code(&manifest, &parameter_owner),
        GscOutcome::Unknown(GscUnknownReason::MalformedCode)
    ));

    let mut index_owner = two_nullary_code();
    index_owner.indices = TelescopeCode(vec![CodeTerm::OwnerApp {
        parameters: Vec::new(),
        indices: vec![CodeTerm::Sort { level: 0 }],
    }]);
    for constructor in &mut index_owner.constructors {
        constructor.result_indices = vec![CodeTerm::Sort { level: 0 }];
    }
    assert!(matches!(
        verify_closed_inductive_code(&manifest, &index_owner),
        GscOutcome::Unknown(GscUnknownReason::MalformedCode)
    ));
}

#[test]
fn deeply_nested_code_fails_with_resource_exhaustion_before_owner_rescan() {
    let manifest = manifest();
    let mut deep = CodeTerm::Sort { level: 0 };
    for _ in 0..128 {
        deep = CodeTerm::Apply {
            function: Box::new(deep),
            argument: Box::new(CodeTerm::Sort { level: 0 }),
        };
    }
    let mut code = two_nullary_code();
    code.parameters = TelescopeCode(vec![deep]);
    assert!(matches!(
        verify_closed_inductive_code(&manifest, &code),
        GscOutcome::Unknown(GscUnknownReason::ResourceExhausted)
    ));
}

#[test]
fn complete_family_identity_binds_support_and_port_roles() {
    let kernel = kernel();
    let manifest = manifest();
    let first_fixture = fixture(&kernel, &manifest, two_nullary_code(), b"identity-a");
    let second_fixture = fixture(&kernel, &manifest, two_nullary_code(), b"identity-b");
    let first = proven(compile_use_v1(
        &manifest,
        &kernel,
        &first_fixture.signature,
        &first_fixture.frame,
    ));
    let second = proven(compile_use_v1(
        &manifest,
        &kernel,
        &second_fixture.signature,
        &second_fixture.frame,
    ));
    assert_ne!(first.id(), second.id());
    assert_ne!(
        first.ports()[0].key().output_port_id(),
        second.ports()[0].key().output_port_id()
    );
}

#[test]
fn fixed_type_support_uses_normalized_types_but_never_declaration_bodies() {
    let kernel = kernel();
    let manifest = manifest();
    let (fixture, body_only) = fixture_with_body_only_dependency(&kernel, &manifest);
    let use_family = proven(compile_use_v1(
        &manifest,
        &kernel,
        &fixture.signature,
        &fixture.frame,
    ));
    assert!(
        !use_family.fixed_type_support().contains(&body_only),
        "a global mentioned only by a declaration body is not fixed type support"
    );
    for source in &fixture.frame.frame().source_declarations {
        assert!(use_family.fixed_type_support().contains(source));
    }
}

#[test]
fn fixed_agda_reference_is_safe_and_postulate_free() {
    let source = include_str!("../../agda/GscInductiveCoreV1.agda");
    assert!(source.contains("{-# OPTIONS --safe --without-K #-}"));
    assert!(source.contains("pathModeFailsClosed"));
    assert!(source.contains("fourVectorDispositions"));
    assert!(source.contains(GSC_REFERENCE_VECTOR_SPEC_TOKEN));
    assert!(!source.contains("postulate"));
}
