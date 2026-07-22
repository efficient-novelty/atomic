//! M1 sweep over the enlarged v4 proven sub-basis.
//!
//! The sweep is intentionally one-sided.  It may issue `Generated` only when
//! the Step-8 left-unit row is reachable by an explicit derivation from the
//! v4 basis.  Failure to find such a derivation is not an `Independent`
//! verdict: the row remains pending until global E-4 completeness.

use crate::context::{BinderId, Declaration, TermExpr, TypeExpr, form_schema_context};
use crate::e4_generator_basis::{
    M1GeneratedSubject, M1PendingSubject, issue_e4_development_audit, replay_e4_development_audit,
};
use crate::e34_class_certificate::{
    SCHEMA2_E34_CLASS_CERTIFICATE_SCHEMA, Schema2E34ClassCertificate, replay_schema2_e34_class_json,
};
use crate::e34_class_induction::{
    CubicalConstructorKind, issue_class_indexed_e3_e4_attempt, replay_class_indexed_e3_e4_attempt,
};
use crate::grammar::{
    DERIVED_ACTION_MEMBERSHIP_RULE, DerivationRef, OrdinarySchemaKind, SchemaTypeExpr,
    issue_step8_registered_signatures, replay_step8_registered_signatures,
};
use crate::stage1_r1::R1LocalRole;
use crate::step8_r2::{
    issue_step8_r2_typed_signatures_token, replay_step8_r2_typed_signatures_token,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const E34_M1_SWEEP_VERSION: &str = "schema2-e34-enlarged-sub-basis-m1-sweep-v1";
pub const E34_M1_SWEEP_DATE: &str = "2026-07-20";
pub const LEFT_UNIT_GENERATION_GAP: &str =
    "E34_M1_LEFT_UNIT_REQUIRES_A_NONCIRCULAR_PATH_COHERENCE_GENERATOR";

const V4_BYTES: &[u8] = include_bytes!("../../../docs/schema2_v4.json");
const P1_BYTES: &[u8] = include_bytes!("../../../docs/e2_phase_order_adjudication.md");
const R2_BYTES: &[u8] = include_bytes!("../../../docs/e2_quotient_adjudications.md");

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SweepSort {
    TypedStep8ParentOperation,
    RegisteredStep8Cell,
    GeneratedStep8CellAction,
    GenericOrdinarySchema,
    GenericPostPathCoherence,
    PublicCubicalTerm,
    Step8LeftUnitCoherence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SweepSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SweepRuleRecord {
    pub rule: String,
    pub evidence_derivation_hashes: Vec<String>,
    pub premises: Vec<SweepSort>,
    pub conclusion: SweepSort,
    pub fired: bool,
    pub concludes_left_unit: bool,
    pub left_unit_premise_required: bool,
    pub noncircular_left_unit_derivation: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnlargedSubBasisEvidence {
    pub v4_predecessor_digest: String,
    pub v4_strict_replay_valid: bool,
    pub ordinary_naturality_count: usize,
    pub ordinary_constructor_inventory: Vec<String>,
    pub every_ordinary_token_replayed: bool,
    pub public_cubical_constructor_count: usize,
    pub public_cubical_constructor_inventory: Vec<String>,
    pub every_public_cubical_action_replayed: bool,
    pub prior_m1_generated_count: usize,
    pub prior_m1_generated_subjects: Vec<String>,
    pub sub_basis_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Step8LeftUnitTargetEvidence {
    pub step8_derivation_hash: String,
    pub registered_signature_derivation_hash: String,
    pub membership_rule: String,
    pub registered_type: String,
    pub operation_signature_typed: bool,
    pub left_unit_signature_typed: bool,
    pub outer_pi_verified: bool,
    pub codomain_path_verified: bool,
    pub registered_left_unit_term_available: bool,
    pub pending_before_sweep: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M1SweepForbiddenOutputs {
    pub independent_verdict_issued: bool,
    pub ordinary_family_token_issued: bool,
    pub stage_count_issued: bool,
    pub e2b_executed: bool,
    pub global_e4_claimed: bool,
    pub global_halt_claimed: bool,
}

impl M1SweepForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.independent_verdict_issued
            && !self.ordinary_family_token_issued
            && !self.stage_count_issued
            && !self.e2b_executed
            && !self.global_e4_claimed
            && !self.global_halt_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E34M1SweepToken {
    pub version: String,
    pub date: String,
    pub source_bindings: Vec<SweepSourceBinding>,
    pub enlarged_sub_basis: EnlargedSubBasisEvidence,
    pub target: Step8LeftUnitTargetEvidence,
    pub initial_reachable_sorts: Vec<SweepSort>,
    pub rules: Vec<SweepRuleRecord>,
    pub fixed_point_reachable_sorts: Vec<SweepSort>,
    pub fixed_point_iterations: usize,
    pub target_reachable: bool,
    pub explicit_generating_derivation_found: bool,
    pub m1_generated_verdict_issued: bool,
    pub independent_verdict_issued: bool,
    pub generated_count_before: usize,
    pub generated_count_after: usize,
    pub pending_count_before: usize,
    pub pending_count_after: usize,
    pub stage1_roles_still_pending: Vec<String>,
    pub left_unit_still_pending: bool,
    pub obstruction: String,
    pub forbidden_outputs: M1SweepForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E34M1SweepReplay {
    pub valid: bool,
    pub v4_replayed: bool,
    pub enlarged_sub_basis_size: usize,
    pub target_reachable: bool,
    pub generated_verdict_issued: bool,
    pub independent_verdict_issued: bool,
    pub generated_count_after: usize,
    pub pending_count_after: usize,
    pub left_unit_still_pending: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum M1SweepError {
    #[error("invalid UTF-8 in {0}")]
    Utf8(String),
    #[error("v4 replay failed: {0}")]
    V4(String),
    #[error("upstream E4 replay failed: {0}")]
    E4(String),
    #[error("Step-8 replay failed: {0}")]
    Step8(String),
    #[error("class induction replay failed: {0}")]
    ClassInduction(String),
    #[error("sweep invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted sweep did not replay: {0}")]
    EmittedReplay(String),
}

fn utf8<'a>(path: &str, bytes: &'a [u8]) -> Result<&'a str, M1SweepError> {
    std::str::from_utf8(bytes).map_err(|_| M1SweepError::Utf8(path.to_owned()))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(E34_M1_SWEEP_VERSION, domain, value))
        .expect("M1 sweep evidence serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn source_bindings() -> Vec<SweepSourceBinding> {
    [
        (
            "docs/schema2_v4.json",
            "strict_enlarged_sub_basis_predecessor",
            V4_BYTES,
        ),
        (
            "docs/e2_phase_order_adjudication.md",
            "adopted_monotone_membership_authority",
            P1_BYTES,
        ),
        (
            "docs/e2_quotient_adjudications.md",
            "adopted_step8_derived_action_rule",
            R2_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| SweepSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn ordinary_name(kind: OrdinarySchemaKind) -> &'static str {
    match kind {
        OrdinarySchemaKind::FreshFormation => "fresh_formation",
        OrdinarySchemaKind::PointOrUnitIntro => "point_or_unit_intro",
        OrdinarySchemaKind::PathConstructorIntro => "path_constructor_intro",
        OrdinarySchemaKind::Recursor => "recursor",
        OrdinarySchemaKind::Inductor => "inductor",
        OrdinarySchemaKind::TruncParametricAction => "trunc_parametric_action",
        OrdinarySchemaKind::PostPathOperation => "post_path_operation",
        OrdinarySchemaKind::PostPathCoherence => "post_path_coherence",
        OrdinarySchemaKind::CellAction => "cell_action",
    }
}

fn cubical_name(kind: CubicalConstructorKind) -> &'static str {
    match kind {
        CubicalConstructorKind::Point => "point",
        CubicalConstructorKind::MotiveBase => "motive_base",
        CubicalConstructorKind::EndpointEvaluationHypothesis => "endpoint_evaluation_hypothesis",
        CubicalConstructorKind::PathMethod => "path_method",
        CubicalConstructorKind::EndpointMethodHypothesis => "endpoint_method_hypothesis",
        CubicalConstructorKind::PathElim => "path_elim",
        CubicalConstructorKind::EndpointPathElim => "endpoint_path_elim",
        CubicalConstructorKind::EndpointElimNeutral => "endpoint_elim_neutral",
        CubicalConstructorKind::DimLambda => "dim_lambda",
        CubicalConstructorKind::DimApp => "dim_app",
        CubicalConstructorKind::Coe => "coe",
        CubicalConstructorKind::Hcom => "hcom",
    }
}

fn role_name(role: R1LocalRole) -> &'static str {
    match role {
        R1LocalRole::KernelHead => "kernel_head",
        R1LocalRole::AdjointMate => "adjoint_mate",
        R1LocalRole::SupportAction => "support_action",
        R1LocalRole::Coherence => "coherence",
    }
}

fn validate_v4() -> Result<Schema2E34ClassCertificate, M1SweepError> {
    let json = utf8("docs/schema2_v4.json", V4_BYTES)?;
    let replay = replay_schema2_e34_class_json(json);
    if !replay.valid {
        return Err(M1SweepError::V4(replay.errors.join("; ")));
    }
    let certificate: Schema2E34ClassCertificate =
        serde_json::from_str(json).map_err(|error| M1SweepError::V4(error.to_string()))?;
    if certificate.schema != SCHEMA2_E34_CLASS_CERTIFICATE_SCHEMA
        || !certificate
            .ordinary
            .every_constructor_normalized_and_natural
        || !certificate.cubical.public_constructor_inventory_exhaustive
        || !certificate.cubical.every_checked_dimension_action_natural
        || certificate.full_e4_complete
        || certificate.forbidden_outputs.independent_verdict_issued
    {
        return Err(M1SweepError::V4(
            "v4 does not have the exact enlarged partial basis boundary".to_owned(),
        ));
    }
    Ok(certificate)
}

fn rule(
    name: String,
    hashes: Vec<String>,
    premises: Vec<SweepSort>,
    conclusion: SweepSort,
) -> SweepRuleRecord {
    SweepRuleRecord {
        rule: name,
        evidence_derivation_hashes: hashes,
        premises,
        conclusion,
        fired: false,
        concludes_left_unit: conclusion == SweepSort::Step8LeftUnitCoherence,
        left_unit_premise_required: false,
        noncircular_left_unit_derivation: false,
    }
}

fn close_rules(initial: &[SweepSort], rules: &mut [SweepRuleRecord]) -> (Vec<SweepSort>, usize) {
    let mut reachable = initial.iter().copied().collect::<BTreeSet<_>>();
    let mut iterations = 0usize;
    loop {
        iterations += 1;
        let mut changed = false;
        for entry in rules.iter_mut() {
            if entry
                .premises
                .iter()
                .all(|premise| reachable.contains(premise))
            {
                entry.fired = true;
                changed |= reachable.insert(entry.conclusion);
            }
        }
        if !changed {
            break;
        }
    }
    (reachable.into_iter().collect(), iterations)
}

fn certificate_digest(token: &E34M1SweepToken) -> String {
    let mut projection = token.clone();
    projection.result_digest.clear();
    tagged_hash("m1-sweep-token", &projection)
}

pub fn issue_e34_m1_sweep() -> Result<E34M1SweepToken, M1SweepError> {
    let v4 = validate_v4()?;
    let class_attempt = issue_class_indexed_e3_e4_attempt()
        .map_err(|error| M1SweepError::ClassInduction(error.to_string()))?;
    replay_class_indexed_e3_e4_attempt(&class_attempt)
        .map_err(|error| M1SweepError::ClassInduction(error.to_string()))?;

    let development =
        issue_e4_development_audit().map_err(|error| M1SweepError::E4(error.to_string()))?;
    replay_e4_development_audit(&development)
        .map_err(|error| M1SweepError::E4(error.to_string()))?;
    let step8 = issue_step8_r2_typed_signatures_token()
        .map_err(|error| M1SweepError::Step8(error.to_string()))?;
    replay_step8_r2_typed_signatures_token(&step8)
        .map_err(|error| M1SweepError::Step8(error.to_string()))?;

    let carrier = TypeExpr::parameter(0);
    let context = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "S3".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(1),
            name: "base".to_owned(),
            ty: carrier.clone(),
        },
    ])
    .map_err(|error| M1SweepError::Step8(error.to_string()))?;
    let bundle = DerivationRef::parse(step8.registered_boundary_bundle_derivation_hash.clone())
        .map_err(|error| M1SweepError::Step8(error.to_string()))?;
    let signatures =
        issue_step8_registered_signatures(context, carrier, TermExpr::variable(1), bundle)
            .map_err(|error| M1SweepError::Step8(error.to_string()))?;
    replay_step8_registered_signatures(&signatures)
        .map_err(|error| M1SweepError::Step8(error.to_string()))?;
    if signatures.derivation_hash() != step8.schema_signature_derivation_hash {
        return Err(M1SweepError::Step8(
            "registered signature derivation mismatch".to_owned(),
        ));
    }
    let (outer_pi_verified, codomain_path_verified) = match signatures.left_unit_type() {
        SchemaTypeExpr::Pi { codomain, .. } => (
            true,
            matches!(codomain.as_ref(), SchemaTypeExpr::Path { .. }),
        ),
        _ => (false, false),
    };

    let pending_before_sweep = development
        .m1_pending()
        .iter()
        .any(|entry| entry.subject() == M1PendingSubject::Step8LeftUnitCoherence);
    let stage1_roles_still_pending = development
        .m1_pending()
        .iter()
        .filter_map(|entry| match entry.subject() {
            M1PendingSubject::Stage1CarrierException { role } => Some(role_name(role).to_owned()),
            M1PendingSubject::Step8LeftUnitCoherence => None,
        })
        .collect::<Vec<_>>();

    let ordinary_inventory = class_attempt
        .ordinary_naturality()
        .iter()
        .map(|token| ordinary_name(token.constructor()).to_owned())
        .collect::<Vec<_>>();
    let public_cubical = class_attempt
        .cubical_inductions()
        .iter()
        .flat_map(|token| token.observed_constructors().iter().copied())
        .collect::<BTreeSet<_>>();
    let prior_subjects = development
        .m1_generated()
        .iter()
        .map(|token| match token.subject() {
            M1GeneratedSubject::Step8CellAction { .. } => "step8_cell_action".to_owned(),
        })
        .collect::<Vec<_>>();
    let ordinary_hashes = class_attempt
        .ordinary_naturality()
        .iter()
        .map(|token| token.derivation_hash().to_owned())
        .collect::<Vec<_>>();
    let cubical_hashes = class_attempt
        .cubical_inductions()
        .iter()
        .map(|token| token.derivation_hash().to_owned())
        .collect::<Vec<_>>();
    let prior_hashes = development
        .m1_generated()
        .iter()
        .map(|token| token.derivation_hash().to_owned())
        .collect::<Vec<_>>();
    let sub_basis_digest = tagged_hash(
        "enlarged-v4-m1-sub-basis",
        &(
            &v4.result_digest,
            &ordinary_hashes,
            &cubical_hashes,
            &prior_hashes,
        ),
    );
    let enlarged_sub_basis = EnlargedSubBasisEvidence {
        v4_predecessor_digest: v4.result_digest,
        v4_strict_replay_valid: true,
        ordinary_naturality_count: ordinary_inventory.len(),
        ordinary_constructor_inventory: ordinary_inventory,
        every_ordinary_token_replayed: class_attempt
            .ordinary_naturality()
            .iter()
            .all(|token| token.holds()),
        public_cubical_constructor_count: public_cubical.len(),
        public_cubical_constructor_inventory: public_cubical
            .iter()
            .copied()
            .map(|kind| cubical_name(kind).to_owned())
            .collect(),
        every_public_cubical_action_replayed: class_attempt
            .cubical_inductions()
            .iter()
            .all(|token| token.all_dimension_substitutions_natural()),
        prior_m1_generated_count: prior_subjects.len(),
        prior_m1_generated_subjects: prior_subjects,
        sub_basis_digest,
    };

    let target = Step8LeftUnitTargetEvidence {
        step8_derivation_hash: step8.derivation_hash.clone(),
        registered_signature_derivation_hash: signatures.derivation_hash().to_owned(),
        membership_rule: DERIVED_ACTION_MEMBERSHIP_RULE.to_owned(),
        registered_type: step8.left_unit_signature.clone(),
        operation_signature_typed: step8.operation_signature_typed,
        left_unit_signature_typed: step8.left_unit_signature_typed,
        outer_pi_verified,
        codomain_path_verified,
        registered_left_unit_term_available: false,
        pending_before_sweep,
    };

    let initial_reachable_sorts = vec![
        SweepSort::TypedStep8ParentOperation,
        SweepSort::RegisteredStep8Cell,
        SweepSort::GeneratedStep8CellAction,
        SweepSort::GenericOrdinarySchema,
        SweepSort::GenericPostPathCoherence,
        SweepSort::PublicCubicalTerm,
    ];
    let mut rules = Vec::new();
    for token in class_attempt.ordinary_naturality() {
        let (premise, conclusion) = match token.constructor() {
            OrdinarySchemaKind::PostPathOperation => (
                SweepSort::TypedStep8ParentOperation,
                SweepSort::TypedStep8ParentOperation,
            ),
            OrdinarySchemaKind::PostPathCoherence => (
                SweepSort::GenericPostPathCoherence,
                SweepSort::GenericPostPathCoherence,
            ),
            OrdinarySchemaKind::CellAction => (
                SweepSort::GeneratedStep8CellAction,
                SweepSort::GeneratedStep8CellAction,
            ),
            _ => (
                SweepSort::GenericOrdinarySchema,
                SweepSort::GenericOrdinarySchema,
            ),
        };
        rules.push(rule(
            format!(
                "ordinary_normalization_naturality:{}",
                ordinary_name(token.constructor())
            ),
            vec![token.derivation_hash().to_owned()],
            vec![premise],
            conclusion,
        ));
    }
    let mut circular_target_transport = rule(
        "ordinary_normalization_naturality:post_path_coherence:step8_specialization".to_owned(),
        vec![
            class_attempt
                .ordinary_naturality()
                .iter()
                .find(|token| token.constructor() == OrdinarySchemaKind::PostPathCoherence)
                .expect("complete v4 inventory")
                .derivation_hash()
                .to_owned(),
        ],
        vec![SweepSort::Step8LeftUnitCoherence],
        SweepSort::Step8LeftUnitCoherence,
    );
    circular_target_transport.left_unit_premise_required = true;
    rules.push(circular_target_transport);

    for kind in public_cubical.iter().copied() {
        let hashes = class_attempt
            .cubical_inductions()
            .iter()
            .filter(|token| token.observed_constructors().contains(&kind))
            .map(|token| token.derivation_hash().to_owned())
            .collect::<Vec<_>>();
        rules.push(rule(
            format!("public_cubical_dimension_action:{}", cubical_name(kind)),
            hashes,
            vec![SweepSort::PublicCubicalTerm],
            SweepSort::PublicCubicalTerm,
        ));
    }
    rules.push(rule(
        "existing_parent_cube_map_generation".to_owned(),
        prior_hashes,
        vec![
            SweepSort::TypedStep8ParentOperation,
            SweepSort::RegisteredStep8Cell,
        ],
        SweepSort::GeneratedStep8CellAction,
    ));

    let (fixed_point_reachable_sorts, fixed_point_iterations) =
        close_rules(&initial_reachable_sorts, &mut rules);
    let target_reachable = fixed_point_reachable_sorts.contains(&SweepSort::Step8LeftUnitCoherence);
    for entry in &mut rules {
        entry.noncircular_left_unit_derivation =
            entry.concludes_left_unit && entry.fired && !entry.left_unit_premise_required;
    }
    let explicit_generating_derivation_found = rules
        .iter()
        .any(|entry| entry.noncircular_left_unit_derivation);
    let m1_generated_verdict_issued = explicit_generating_derivation_found;
    let independent_verdict_issued = false;
    let generated_count_before = development.m1_generated().len();
    let generated_count_after = generated_count_before + usize::from(m1_generated_verdict_issued);
    let pending_count_before = development.m1_pending().len();
    let pending_count_after = pending_count_before - usize::from(m1_generated_verdict_issued);
    let left_unit_still_pending = !m1_generated_verdict_issued;
    let forbidden_outputs = M1SweepForbiddenOutputs {
        independent_verdict_issued,
        ordinary_family_token_issued: false,
        stage_count_issued: false,
        e2b_executed: false,
        global_e4_claimed: false,
        global_halt_claimed: false,
    };

    if enlarged_sub_basis.ordinary_naturality_count != OrdinarySchemaKind::ALL.len()
        || enlarged_sub_basis.ordinary_naturality_count != 9
        || !enlarged_sub_basis.every_ordinary_token_replayed
        || enlarged_sub_basis.public_cubical_constructor_count != 8
        || !enlarged_sub_basis.every_public_cubical_action_replayed
        || enlarged_sub_basis.prior_m1_generated_count != 1
        || !target.operation_signature_typed
        || !target.left_unit_signature_typed
        || !target.outer_pi_verified
        || !target.codomain_path_verified
        || target.registered_left_unit_term_available
        || !target.pending_before_sweep
        || target_reachable
        || explicit_generating_derivation_found
        || m1_generated_verdict_issued
        || independent_verdict_issued
        || generated_count_before != 1
        || generated_count_after != 1
        || pending_count_before != 5
        || pending_count_after != 5
        || stage1_roles_still_pending.len() != 4
        || !left_unit_still_pending
        || rules
            .iter()
            .filter(|entry| entry.concludes_left_unit)
            .any(|entry| !entry.left_unit_premise_required)
        || !forbidden_outputs.all_withheld()
    {
        return Err(M1SweepError::Invariant(
            "enlarged-basis closure crossed or failed the M1 boundary".to_owned(),
        ));
    }

    let mut token = E34M1SweepToken {
        version: E34_M1_SWEEP_VERSION.to_owned(),
        date: E34_M1_SWEEP_DATE.to_owned(),
        source_bindings: source_bindings(),
        enlarged_sub_basis,
        target,
        initial_reachable_sorts,
        rules,
        fixed_point_reachable_sorts,
        fixed_point_iterations,
        target_reachable,
        explicit_generating_derivation_found,
        m1_generated_verdict_issued,
        independent_verdict_issued,
        generated_count_before,
        generated_count_after,
        pending_count_before,
        pending_count_after,
        stage1_roles_still_pending,
        left_unit_still_pending,
        obstruction: LEFT_UNIT_GENERATION_GAP.to_owned(),
        forbidden_outputs,
        outcome: "enlarged_sub_basis_sweep_clean_no_noncircular_step8_left_unit_generation_derivation"
            .to_owned(),
        permitted_conclusion: "The v4 basis transports an already supplied PostPathCoherence and acts on already typed cubical terms, but neither operation introduces the Step-8 left-unit path coherence from mu. The only rule with the target as conclusion requires that same target as a premise, so M1 issues no new Generated verdict. This is not an Independent verdict; the pending set remains five."
            .to_owned(),
        result_digest: String::new(),
    };
    token.result_digest = certificate_digest(&token);
    Ok(token)
}

pub fn replay_e34_m1_sweep(token: &E34M1SweepToken) -> E34M1SweepReplay {
    let expected = match issue_e34_m1_sweep() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if token.result_digest != certificate_digest(token) {
        errors.push("result digest mismatch".to_owned());
    }
    if token != &expected {
        errors.push("sweep differs from definition replay".to_owned());
    }
    E34M1SweepReplay {
        valid: errors.is_empty(),
        v4_replayed: token.enlarged_sub_basis.v4_strict_replay_valid,
        enlarged_sub_basis_size: token.enlarged_sub_basis.ordinary_naturality_count
            + token.enlarged_sub_basis.public_cubical_constructor_count
            + token.enlarged_sub_basis.prior_m1_generated_count,
        target_reachable: token.target_reachable,
        generated_verdict_issued: token.m1_generated_verdict_issued,
        independent_verdict_issued: token.independent_verdict_issued,
        generated_count_after: token.generated_count_after,
        pending_count_after: token.pending_count_after,
        left_unit_still_pending: token.left_unit_still_pending,
        forbidden_outputs_withheld: token.forbidden_outputs.all_withheld(),
        outcome: token.outcome.clone(),
        errors,
    }
}

fn failed_replay(error: impl Into<String>) -> E34M1SweepReplay {
    E34M1SweepReplay {
        valid: false,
        v4_replayed: false,
        enlarged_sub_basis_size: 0,
        target_reachable: false,
        generated_verdict_issued: false,
        independent_verdict_issued: false,
        generated_count_after: 0,
        pending_count_after: 0,
        left_unit_still_pending: false,
        forbidden_outputs_withheld: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn e34_m1_sweep_json_pretty() -> Result<String, M1SweepError> {
    serde_json::to_string_pretty(&issue_e34_m1_sweep()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| M1SweepError::Json(error.to_string()))
}

pub fn replay_e34_m1_sweep_json(json: &str) -> E34M1SweepReplay {
    let raw: Value = match serde_json::from_str(json) {
        Ok(raw) => raw,
        Err(error) => return failed_replay(format!("invalid JSON: {error}")),
    };
    let token: E34M1SweepToken = match serde_json::from_str(json) {
        Ok(token) => token,
        Err(error) => return failed_replay(format!("sweep shape error: {error}")),
    };
    let typed = serde_json::to_value(&token).expect("M1 sweep serializes");
    if raw != typed {
        return failed_replay("JSON contains unknown, duplicate, or ignored structure");
    }
    replay_e34_m1_sweep(&token)
}

pub fn emit_e34_m1_sweep_create_new(path: &Path) -> Result<E34M1SweepReplay, M1SweepError> {
    let json = e34_m1_sweep_json_pretty()?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| M1SweepError::Io(error.to_string()))?;
    output
        .write_all(json.as_bytes())
        .and_then(|()| output.flush())
        .map_err(|error| M1SweepError::Io(error.to_string()))?;
    let replay = replay_e34_m1_sweep_json(&json);
    if !replay.valid {
        return Err(M1SweepError::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn redigest(token: &mut E34M1SweepToken) {
        token.result_digest = certificate_digest(token);
    }

    #[test]
    fn enlarged_sub_basis_sweep_reaches_no_left_unit() {
        let token = issue_e34_m1_sweep().unwrap();
        let replay = replay_e34_m1_sweep(&token);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(replay.enlarged_sub_basis_size, 18);
        assert!(!replay.target_reachable);
        assert!(!replay.generated_verdict_issued);
        assert!(!replay.independent_verdict_issued);
        assert_eq!(replay.generated_count_after, 1);
        assert_eq!(replay.pending_count_after, 5);
        assert!(replay.left_unit_still_pending);
    }

    #[test]
    fn forged_generation_and_pending_shrink_fail_replay() {
        let token = issue_e34_m1_sweep().unwrap();
        let mut generated = token.clone();
        generated.target_reachable = true;
        generated.explicit_generating_derivation_found = true;
        generated.m1_generated_verdict_issued = true;
        generated.generated_count_after = 2;
        generated.pending_count_after = 4;
        generated.left_unit_still_pending = false;
        redigest(&mut generated);
        assert!(!replay_e34_m1_sweep(&generated).valid);

        let mut independent = token;
        independent.independent_verdict_issued = true;
        independent.forbidden_outputs.independent_verdict_issued = true;
        redigest(&mut independent);
        assert!(!replay_e34_m1_sweep(&independent).valid);
    }
}
