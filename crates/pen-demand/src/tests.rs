use crate::{
    FamilyId, FiniteDemandDomain, FiniteRule, InstanceId, LibrarySeeds, OpaqueWindow,
    RegisteredFamily, RegisteredInstance, RelativeCensus, RelativeCensusOutcome,
    UncheckedRelativeCensusCertificate, UnknownReason, compute_relative_census,
    verify_relative_census_certificate,
};
use pen_kernel::{
    Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, OpenJudgment, Term,
    UncheckedSignature, VerifiedSignature,
};

struct Fixture {
    kernel: Kernel,
    signature: VerifiedSignature,
    window: OpaqueWindow,
    domain: FiniteDemandDomain,
    library: LibrarySeeds,
    seed: InstanceId,
    left: InstanceId,
    right: InstanceId,
    join: InstanceId,
    isolated: InstanceId,
}

fn global(label: &[u8]) -> GlobalId {
    GlobalId(Digest::of_bytes(label))
}

fn type_motive(term: Term) -> OpenJudgment {
    OpenJudgment::TypeFormation {
        context: DependentContext::default(),
        term,
    }
}

fn global_type(id: &GlobalId) -> Term {
    Term::Global { id: id.clone() }
}

fn pi(body: Term) -> Term {
    Term::Pi {
        parameter: Box::new(Term::UnitType),
        body: Box::new(body),
    }
}

fn sigma(body: Term) -> Term {
    Term::Sigma {
        parameter: Box::new(Term::UnitType),
        body: Box::new(body),
    }
}

fn rule(mut premises: Vec<InstanceId>, conclusion: InstanceId) -> FiniteRule {
    premises.sort();
    FiniteRule {
        premises,
        conclusion,
    }
}

fn fixture() -> Fixture {
    let kernel = Kernel::new(KernelLimits::default()).expect("positive limits");
    let first_anchor = global(b"opaque-anchor-a");
    let second_anchor = global(b"opaque-anchor-b");
    let signature = kernel
        .verify_signature(&UncheckedSignature {
            declarations: vec![
                Declaration {
                    id: first_anchor.clone(),
                    ty: Term::Sort { level: 0 },
                    body: None,
                },
                Declaration {
                    id: second_anchor.clone(),
                    ty: Term::Sort { level: 0 },
                    body: None,
                },
            ],
        })
        .expect("closed signature");
    let window = OpaqueWindow {
        entries: [first_anchor.clone(), second_anchor],
    };

    let family = RegisteredFamily::canonical(type_motive(global_type(&first_anchor)));
    let mut instances = vec![
        RegisteredInstance::canonical(family.id.clone(), type_motive(global_type(&first_anchor))),
        RegisteredInstance::canonical(
            family.id.clone(),
            type_motive(pi(global_type(&first_anchor))),
        ),
        RegisteredInstance::canonical(
            family.id.clone(),
            type_motive(sigma(global_type(&first_anchor))),
        ),
        RegisteredInstance::canonical(
            family.id.clone(),
            type_motive(pi(pi(global_type(&first_anchor)))),
        ),
        RegisteredInstance::canonical(
            family.id.clone(),
            type_motive(sigma(sigma(global_type(&first_anchor)))),
        ),
    ];
    let seed = instances[0].id.clone();
    let left = instances[1].id.clone();
    let right = instances[2].id.clone();
    let join = instances[3].id.clone();
    let isolated = instances[4].id.clone();
    instances.sort_by(|a, b| a.id.cmp(&b.id));

    let mut rules = vec![
        rule(vec![seed.clone()], left.clone()),
        rule(vec![seed.clone()], right.clone()),
        rule(vec![left.clone(), right.clone()], join.clone()),
        rule(vec![isolated.clone()], isolated.clone()),
    ];
    rules.sort();

    Fixture {
        kernel,
        signature,
        window,
        domain: FiniteDemandDomain {
            families: vec![family],
            instances,
            rules,
        },
        library: LibrarySeeds {
            instance_ids: vec![seed.clone()],
        },
        seed,
        left,
        right,
        join,
        isolated,
    }
}

fn complete(fixture: &Fixture, operations: u64) -> RelativeCensus {
    match compute_relative_census(
        &fixture.kernel,
        &fixture.signature,
        &fixture.domain,
        &fixture.window,
        &fixture.library,
        operations,
    ) {
        RelativeCensusOutcome::CompleteRelative(census) => census,
        RelativeCensusOutcome::Unknown(reason) => {
            panic!("expected complete relative census, got {reason:?}")
        }
    }
}

fn claim(fixture: &Fixture, census: &RelativeCensus) -> UncheckedRelativeCensusCertificate {
    match UncheckedRelativeCensusCertificate::claim(
        &fixture.kernel,
        &fixture.signature,
        &fixture.domain,
        &fixture.window,
        &fixture.library,
        census,
    ) {
        RelativeCensusOutcome::CompleteRelative(certificate) => certificate,
        RelativeCensusOutcome::Unknown(reason) => {
            panic!("expected bounded certificate claim, got {reason:?}")
        }
    }
}

#[test]
fn families_and_instances_remain_distinct_registered_objects() {
    let fixture = fixture();
    assert_eq!(fixture.domain.families.len(), 1);
    assert_eq!(fixture.domain.instances.len(), 5);
    assert!(
        fixture
            .domain
            .instances
            .iter()
            .all(|instance| instance.family_id == fixture.domain.families[0].id)
    );
    assert_ne!(
        fixture.domain.families[0].id.0,
        fixture.domain.instances[0].id.0
    );
}

#[test]
fn invalid_references_duplicates_and_order_fail_closed() {
    let mut unknown_family = fixture();
    unknown_family.domain.instances[0].family_id = FamilyId(Digest::of_bytes(b"absent-family"));
    assert_eq!(
        compute_relative_census(
            &unknown_family.kernel,
            &unknown_family.signature,
            &unknown_family.domain,
            &unknown_family.window,
            &unknown_family.library,
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    );

    let mut duplicate = fixture();
    duplicate
        .domain
        .instances
        .push(duplicate.domain.instances[0].clone());
    duplicate.domain.instances.sort_by(|a, b| a.id.cmp(&b.id));
    assert_eq!(
        compute_relative_census(
            &duplicate.kernel,
            &duplicate.signature,
            &duplicate.domain,
            &duplicate.window,
            &duplicate.library,
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    );

    let mut out_of_order = fixture();
    out_of_order.domain.instances.reverse();
    assert_eq!(
        compute_relative_census(
            &out_of_order.kernel,
            &out_of_order.signature,
            &out_of_order.domain,
            &out_of_order.window,
            &out_of_order.library,
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    );

    let mut unknown_rule_reference = fixture();
    unknown_rule_reference.domain.rules.push(rule(
        vec![InstanceId(Digest::of_bytes(b"absent-instance"))],
        unknown_rule_reference.seed.clone(),
    ));
    unknown_rule_reference.domain.rules.sort();
    assert_eq!(
        compute_relative_census(
            &unknown_rule_reference.kernel,
            &unknown_rule_reference.signature,
            &unknown_rule_reference.domain,
            &unknown_rule_reference.window,
            &unknown_rule_reference.library,
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    );
}

#[test]
fn identifiers_and_type_formation_motives_are_rechecked() {
    let mut forged_id = fixture();
    forged_id.domain.families[0].id = FamilyId(Digest::of_bytes(b"forged-family-id"));
    assert_eq!(
        compute_relative_census(
            &forged_id.kernel,
            &forged_id.signature,
            &forged_id.domain,
            &forged_id.window,
            &forged_id.library,
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    );

    let base = fixture();
    let invalid_family = RegisteredFamily::canonical(OpenJudgment::HasType {
        context: DependentContext::default(),
        term: Term::Unit,
        ty: Term::UnitType,
    });
    let invalid_domain = FiniteDemandDomain {
        families: vec![invalid_family],
        instances: Vec::new(),
        rules: Vec::new(),
    };
    assert_eq!(
        compute_relative_census(
            &base.kernel,
            &base.signature,
            &invalid_domain,
            &base.window,
            &LibrarySeeds::default(),
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    );
}

#[test]
fn fixed_point_layers_handle_a_diamond_cycle_and_unreachable_node() {
    let case = fixture();
    let census = complete(&case, u64::MAX);

    let mut first_delta = vec![case.left.clone(), case.right.clone()];
    first_delta.sort();
    assert_eq!(census.layers[0], vec![case.seed.clone()]);
    assert_eq!(census.layers[1], first_delta);
    assert_eq!(census.layers[2], vec![case.join.clone()]);
    assert!(census.reached.contains(&case.join));
    assert!(!census.reached.contains(&case.isolated));
    assert_eq!(census.unreached, vec![case.isolated.clone()]);

    let mut seeded_cycle = fixture();
    seeded_cycle.library.instance_ids =
        vec![seeded_cycle.seed.clone(), seeded_cycle.isolated.clone()];
    seeded_cycle.library.instance_ids.sort();
    let seeded = complete(&seeded_cycle, u64::MAX);
    assert!(seeded.reached.contains(&seeded_cycle.isolated));
    assert!(seeded.unreached.is_empty());
}

#[test]
fn support_is_derived_from_motives_and_the_fixed_window() {
    let fixture = fixture();
    let census = complete(&fixture, u64::MAX);
    let anchor = &fixture.window.entries[0];
    assert_eq!(census.active.len(), fixture.domain.instances.len());
    assert!(census.support.iter().all(|support| {
        support.globals.contains(anchor) && support.window_hits == [true, false]
    }));
}

#[test]
fn replay_recomputes_exact_sets_layers_and_bindings() {
    let fixture = fixture();
    let census = complete(&fixture, u64::MAX);
    let mut certificate = claim(&fixture, &census);
    assert!(matches!(
        verify_relative_census_certificate(
            &fixture.kernel,
            &fixture.signature,
            &fixture.domain,
            &fixture.window,
            &fixture.library,
            u64::MAX,
            &certificate,
        ),
        RelativeCensusOutcome::CompleteRelative(_)
    ));

    certificate.layers[1].pop();
    assert!(matches!(
        verify_relative_census_certificate(
            &fixture.kernel,
            &fixture.signature,
            &fixture.domain,
            &fixture.window,
            &fixture.library,
            u64::MAX,
            &certificate,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    ));
}

#[test]
fn unknown_json_fields_are_rejected_at_wire_boundaries() {
    let fixture = fixture();
    let census = complete(&fixture, u64::MAX);
    let certificate = claim(&fixture, &census);
    let mut certificate_json = serde_json::to_value(certificate).expect("serialize certificate");
    certificate_json
        .as_object_mut()
        .expect("certificate object")
        .insert("trusted".to_owned(), serde_json::Value::Bool(true));
    assert!(
        serde_json::from_value::<UncheckedRelativeCensusCertificate>(certificate_json).is_err()
    );

    let mut domain_json = serde_json::to_value(&fixture.domain).expect("serialize domain");
    domain_json
        .as_object_mut()
        .expect("domain object")
        .insert("authority".to_owned(), serde_json::Value::Bool(true));
    assert!(serde_json::from_value::<FiniteDemandDomain>(domain_json).is_err());
}

#[test]
fn operation_exhaustion_is_unknown_not_a_smaller_complete_result() {
    let fixture = fixture();
    assert_eq!(
        compute_relative_census(
            &fixture.kernel,
            &fixture.signature,
            &fixture.domain,
            &fixture.window,
            &fixture.library,
            0,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::ResourceExhausted)
    );
    assert!(matches!(
        compute_relative_census(
            &fixture.kernel,
            &fixture.signature,
            &fixture.domain,
            &fixture.window,
            &fixture.library,
            u64::MAX,
        ),
        RelativeCensusOutcome::CompleteRelative(_)
    ));
}

#[test]
fn deep_and_wide_domains_exhaust_before_recursive_hashing() {
    let fixture = fixture();
    let limited = Kernel::new(KernelLimits {
        max_operations: 32,
        max_depth: 8,
        normalization_fuel: 32,
    })
    .expect("bounded kernel");

    let mut deep_term = Term::UnitType;
    for _ in 0..10 {
        deep_term = pi(deep_term);
    }
    let deep = FiniteDemandDomain {
        families: vec![RegisteredFamily {
            id: FamilyId(Digest::of_bytes(b"unhashed-deep-family")),
            motive: type_motive(deep_term),
        }],
        instances: Vec::new(),
        rules: Vec::new(),
    };
    assert_eq!(
        compute_relative_census(
            &limited,
            &fixture.signature,
            &deep,
            &fixture.window,
            &LibrarySeeds::default(),
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::ResourceExhausted)
    );

    let family = RegisteredFamily {
        id: FamilyId(Digest::of_bytes(b"unhashed-wide-family")),
        motive: type_motive(Term::UnitType),
    };
    let wide = FiniteDemandDomain {
        families: vec![family; 33],
        instances: Vec::new(),
        rules: Vec::new(),
    };
    assert_eq!(
        compute_relative_census(
            &limited,
            &fixture.signature,
            &wide,
            &fixture.window,
            &LibrarySeeds::default(),
            u64::MAX,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::ResourceExhausted)
    );
}

#[test]
fn replay_rejects_stale_kernel_and_normalizer_scope() {
    let fixture = fixture();
    let census = complete(&fixture, u64::MAX);
    let mut stale_kernel = claim(&fixture, &census);
    stale_kernel.kernel_digest = Digest::of_bytes(b"stale-kernel");
    assert!(matches!(
        verify_relative_census_certificate(
            &fixture.kernel,
            &fixture.signature,
            &fixture.domain,
            &fixture.window,
            &fixture.library,
            u64::MAX,
            &stale_kernel,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    ));

    let mut stale_normalizer = claim(&fixture, &census);
    stale_normalizer.normalizer_digest = Digest::of_bytes(b"stale-normalizer");
    assert!(matches!(
        verify_relative_census_certificate(
            &fixture.kernel,
            &fixture.signature,
            &fixture.domain,
            &fixture.window,
            &fixture.library,
            u64::MAX,
            &stale_normalizer,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::Unsupported)
    ));
}

#[test]
fn oversized_mismatched_certificate_payload_exhausts_without_cloning() {
    let fixture = fixture();
    let census = complete(&fixture, u64::MAX);
    let mut certificate = claim(&fixture, &census);
    certificate
        .active
        .extend(std::iter::repeat_n(fixture.seed.clone(), 256));
    let limited = Kernel::new(KernelLimits {
        max_operations: 128,
        max_depth: 32,
        normalization_fuel: 128,
    })
    .expect("bounded kernel");
    assert!(matches!(
        verify_relative_census_certificate(
            &limited,
            &fixture.signature,
            &fixture.domain,
            &fixture.window,
            &fixture.library,
            u64::MAX,
            &certificate,
        ),
        RelativeCensusOutcome::Unknown(UnknownReason::ResourceExhausted)
    ));
}

#[test]
fn window_width_is_encoded_by_the_array_type() {
    fn requires_fixed_pair(_: [GlobalId; 2]) {}

    let fixture = fixture();
    requires_fixed_pair(fixture.window.entries.clone());
    let [first, second] = fixture.window.entries;
    assert_ne!(first, second);
}
