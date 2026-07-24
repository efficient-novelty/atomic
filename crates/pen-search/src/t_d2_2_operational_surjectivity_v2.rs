//! T-D2-2 v2: fail-closed dependency audit for operational surjectivity.
//!
//! T-D2-2 may load the bridge grammar only after T-D2-1 publicly supplies
//! all of the following:
//!
//! * a replayable context-indexed membership derivation whose recursive
//!   premises retain their binder contexts;
//! * the complete adopted natural-family quotient witness (dependent
//!   parameter telescope, checked renaming, all-substitution naturality,
//!   univalent congruence, and instance quotient); and
//! * a full-content projection of the sealed family and membership
//!   regressions.
//!
//! T-D2-1 v2 does not yet expose those theorem objects.  Consequently this
//! issuer stops before loading BC-1, constructing a generator relation, or
//! issuing either direction of surjectivity.  In particular, it contains no
//! target-marker surrogate for a generator derivation.

use crate::t_d2_1_operational_domain_v2::{
    Td21OperationalDomainV2Certificate, Td21V2ConstructiveRule, Td21V2ContextualMembershipDecision,
    Td21V2OperationalNode, Td21V2ParameterTelescope, Td21V2PublicFormer, Td21V2SyntaxNumber,
    derive_t_d2_1_contextual_membership_v2, issue_t_d2_1_operational_domain_v2,
    replay_t_d2_1_contextual_membership_v2, replay_t_d2_1_operational_domain_v2,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, read, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const T_D2_2_OPERATIONAL_SURJECTIVITY_V2_SCHEMA: &str =
    "t-d2-2-operational-domain-surjectivity-v2";
pub const T_D2_2_OPERATIONAL_SURJECTIVITY_V2_DATE: &str = "2026-07-24";
pub const T_D2_2_OPERATIONAL_SURJECTIVITY_V2_CERTIFICATE_NAME: &str =
    "t_d2_2_operational_surjectivity_v2.json";
pub const T_D2_2_OPERATIONAL_SURJECTIVITY_V2_REPORT_NAME: &str =
    "T_D2_2_OPERATIONAL_SURJECTIVITY_V2_RESULT.md";

pub const TD22_V2_CONTEXTUAL_DERIVATION_API_GAP: &str =
    "TD22_V2_CONTEXT_INDEXED_MEMBERSHIP_DERIVATION_UNAVAILABLE";
pub const TD22_V2_NATURAL_FAMILY_QUOTIENT_API_GAP: &str =
    "TD22_V2_NATURAL_FAMILY_QUOTIENT_WITNESS_UNAVAILABLE";
pub const TD22_V2_FULL_CONTENT_REGRESSION_GAP: &str =
    "TD22_V2_FULL_CONTENT_REGRESSION_PROJECTION_UNAVAILABLE";

const PARENT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/depth_two_domain_adjudication.md");
const OPERATIONAL_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_operational_domain_adjudication.md");
const BRIDGE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/bridge_completion_plan.md");
const TD21_SOURCE_BYTES: &[u8] = include_bytes!("t_d2_1_operational_domain_v2.rs");
const TYPED_FAMILIES_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const SCHEMA_CONTEXT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/context.rs");
const AMBIENT_FORMER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/ambient_former_internality.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_d2_2_operational_surjectivity_v2.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td22V2Register {
    Law,
    IndependentDomainAuthority,
    MissingTheorem,
    ArtifactMetadata,
    Gate,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td22V2RunStatus {
    StoppedOnTd21V2Prerequisite,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td22V2TheoremDisposition {
    NotAttemptedDependencyUnsatisfied,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22V2TaggedNumber {
    pub decimal: String,
    pub register: Td22V2Register,
    pub meaning: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22V2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Td22V2TaggedNumber,
    pub blake3: String,
    pub register: Td22V2Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22V2RequiredContractCheck {
    pub contract_id: String,
    pub required_public_object: String,
    pub available: bool,
    pub exact_obstruction: String,
    pub no_proxy_accepted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22V2Td21Binding {
    pub typed_result_digest: String,
    pub public_replay_valid: bool,
    pub public_replay_errors: Vec<String>,
    pub t_d2_1_reported_prerequisite_satisfied: bool,
    pub symbolic_membership_elimination_reported_total: bool,
    pub symbolic_induction_reported_total: bool,
    pub explicit_representative_vector_extensionally_complete: bool,
    pub family_regression_reported_passed: bool,
    pub membership_row_regression_reported_passed: bool,
    pub contextual_binder_probe: Td21V2ContextualMembershipDecision,
    pub closed_bound_variable_probe: Td21V2ContextualMembershipDecision,
    pub contextual_binder_probe_publicly_replayed: bool,
    pub binder_child_context_preserved: bool,
    pub closed_bound_variable_rejected: bool,
    pub quotient_probe_established: bool,
    pub required_contract_checks: Vec<Td22V2RequiredContractCheck>,
    pub every_required_public_contract_available: bool,
    pub complete_typed_dependency_contract: bool,
    pub exact_dependency_statement: String,
    pub binding_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22V2NamedGap {
    pub id: String,
    pub phase: String,
    pub exact_obstruction: String,
    pub minimal_witness_shape: String,
    pub keeps_grammar_unloaded: bool,
    pub keeps_surjectivity_unissued: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22V2ZeroCharge {
    pub kappa_minted: Td22V2TaggedNumber,
    pub nu_minted: Td22V2TaggedNumber,
    pub anchors_minted: Td22V2TaggedNumber,
    pub demand_orbits_minted: Td22V2TaggedNumber,
    pub zero_charge_holds: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22V2Gate {
    pub bridge_grammar_loaded: bool,
    pub operational_scope_bindings_built: bool,
    pub generator_derivation_relation_built: bool,
    pub reach_subset_domain_attempted: bool,
    pub domain_subset_reach_attempted: bool,
    pub theorem_verdict_issued: bool,
    pub surjectivity_proved: bool,
    pub surjectivity_refuted: bool,
    pub marker_surface_size: Td22V2TaggedNumber,
    pub t_d2_3_key_schema_changed: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m3_successor_issued: bool,
    pub m4_authorized: bool,
    pub exact_gate_statement: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22OperationalSurjectivityV2Certificate {
    pub schema: String,
    pub date: String,
    pub status: Td22V2RunStatus,
    pub theorem_disposition: Td22V2TheoremDisposition,
    pub numeric_register_policy: String,
    pub source_bindings: Vec<Td22V2SourceBinding>,
    pub td21_binding: Td22V2Td21Binding,
    pub named_gaps: Vec<Td22V2NamedGap>,
    pub desired_outcome_read: bool,
    pub generator_inventory_used_to_shape_domain: bool,
    pub regression_inventory_used_to_shape_domain: bool,
    pub selector_value_or_bar_read: bool,
    pub zero_charge: Td22V2ZeroCharge,
    pub gate: Td22V2Gate,
    pub exact_result: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub mutation_falsifiers: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Td22V2Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: Option<Td22V2RunStatus>,
    pub theorem_disposition: Option<Td22V2TheoremDisposition>,
    pub td21_dependency_satisfied: bool,
    pub grammar_loaded: bool,
    pub theorem_verdict_issued: bool,
    pub surjectivity_proved: bool,
    pub refutation_issued: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Td22V2Error {
    #[error("T-D2-2 v2 input failure: {0}")]
    Input(String),
    #[error("T-D2-2 v2 invariant failure: {0}")]
    Invariant(String),
    #[error("T-D2-2 v2 JSON failure: {0}")]
    Json(String),
    #[error("T-D2-2 v2 create-new I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_D2_2_OPERATIONAL_SURJECTIVITY_V2_SCHEMA, domain, value))
        .expect("T-D2-2 v2 evidence serializes");
    bytes_hash(&bytes)
}

fn tagged_number(
    value: impl ToString,
    register: Td22V2Register,
    meaning: &str,
) -> Td22V2TaggedNumber {
    Td22V2TaggedNumber {
        decimal: value.to_string(),
        register,
        meaning: meaning.to_owned(),
    }
}

fn source_bindings() -> Vec<Td22V2SourceBinding> {
    [
        (
            "docs/depth_two_domain_adjudication.md",
            "independent-domain and noncircular-surjectivity law",
            PARENT_ADJUDICATION_BYTES,
            Td22V2Register::Law,
        ),
        (
            "docs/schema2_operational_domain_adjudication.md",
            "adopted operational bounds and fitting ban",
            OPERATIONAL_ADJUDICATION_BYTES,
            Td22V2Register::Law,
        ),
        (
            "docs/bridge_completion_plan.md",
            "frozen bridge construction and gate discipline",
            BRIDGE_PLAN_BYTES,
            Td22V2Register::Law,
        ),
        (
            "crates/pen-search/src/t_d2_1_operational_domain_v2.rs",
            "current public T-D2-1 dependency contract",
            TD21_SOURCE_BYTES,
            Td22V2Register::IndependentDomainAuthority,
        ),
        (
            "crates/pen-eval/src/typed_families.rs",
            "sample canonical-presentation and naturality machinery; not a universal theorem",
            TYPED_FAMILIES_SOURCE_BYTES,
            Td22V2Register::MissingTheorem,
        ),
        (
            "crates/pen-schema/src/context.rs",
            "restricted E1 context surface with its typed-elaboration gap",
            SCHEMA_CONTEXT_SOURCE_BYTES,
            Td22V2Register::MissingTheorem,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "closed-candidate transparent-former scope; not a generic contextual bridge",
            AMBIENT_FORMER_SOURCE_BYTES,
            Td22V2Register::MissingTheorem,
        ),
        (
            "crates/pen-search/src/t_d2_2_operational_surjectivity_v2.rs",
            "this fail-closed dependency audit",
            THIS_SOURCE_BYTES,
            Td22V2Register::ArtifactMetadata,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes, register)| Td22V2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: tagged_number(
            bytes.len(),
            Td22V2Register::ArtifactMetadata,
            "exact source byte length",
        ),
        blake3: bytes_hash(bytes),
        register,
    })
    .collect()
}

fn contract_check(
    contract_id: &str,
    required_public_object: &str,
    available: bool,
    exact_obstruction: &str,
) -> Td22V2RequiredContractCheck {
    let mut check = Td22V2RequiredContractCheck {
        contract_id: contract_id.to_owned(),
        required_public_object: required_public_object.to_owned(),
        available,
        exact_obstruction: exact_obstruction.to_owned(),
        no_proxy_accepted: true,
        derivation_hash: String::new(),
    };
    let mut projection = check.clone();
    projection.derivation_hash.clear();
    check.derivation_hash = tagged_hash("required-contract-check", &projection);
    check
}

fn syntax_number(value: u32, bound_name: &str) -> Td21V2SyntaxNumber {
    Td21V2SyntaxNumber {
        value,
        bound_name: bound_name.to_owned(),
    }
}

fn minimal_lambda_bound_probe() -> Td21V2OperationalNode {
    Td21V2OperationalNode::FormerApplication {
        former: Td21V2PublicFormer::LambdaIntroduction,
        arguments: vec![Td21V2OperationalNode::BoundVariable {
            binder_level: syntax_number(1, "sealed_maximum_binder_nesting"),
        }],
        dimension: None,
    }
}

fn closed_bound_probe() -> Td21V2OperationalNode {
    Td21V2OperationalNode::BoundVariable {
        binder_level: syntax_number(1, "sealed_maximum_binder_nesting"),
    }
}

fn probe_contextual_contract() -> Result<
    (
        Td21V2ContextualMembershipDecision,
        Td21V2ContextualMembershipDecision,
        bool,
        bool,
        bool,
    ),
    Td22V2Error,
> {
    let context = Td21V2ParameterTelescope {
        hypotheses: Vec::new(),
    };
    let lambda = minimal_lambda_bound_probe();
    let contextual_binder_probe = derive_t_d2_1_contextual_membership_v2(&context, &lambda)
        .map_err(|error| Td22V2Error::Input(error.to_string()))?;
    let contextual_binder_probe_publicly_replayed =
        replay_t_d2_1_contextual_membership_v2(&context, &lambda, &contextual_binder_probe)
            .map_err(|error| Td22V2Error::Input(error.to_string()))?;
    let binder_child_context_preserved =
        contextual_binder_probe
            .derivation
            .as_ref()
            .is_some_and(|root| {
                root.rule == Td21V2ConstructiveRule::FormerApplication
                    && root.exact_node == lambda
                    && root.every_child_context_replayed
                    && root.children.len() == 1
                    && root.children[0].rule == Td21V2ConstructiveRule::BoundProjection
                    && root.children[0].active_binder_classifiers.len() == 1
                    && root.children[0].exact_node == closed_bound_probe()
            });
    let closed_node = closed_bound_probe();
    let closed_bound_variable_probe =
        derive_t_d2_1_contextual_membership_v2(&context, &closed_node)
            .map_err(|error| Td22V2Error::Input(error.to_string()))?;
    let closed_bound_variable_rejected = !closed_bound_variable_probe.typed_judgment_accepted
        && !closed_bound_variable_probe.domain_member
        && closed_bound_variable_probe.derivation.is_none();
    Ok((
        contextual_binder_probe,
        closed_bound_variable_probe,
        contextual_binder_probe_publicly_replayed,
        binder_child_context_preserved,
        closed_bound_variable_rejected,
    ))
}

fn required_contract_checks(
    contextual_derivation_available: bool,
    contextual_probe_passed: bool,
    quotient_established: bool,
    full_content_regressions_proved: bool,
) -> Vec<Td22V2RequiredContractCheck> {
    vec![
        contract_check(
            TD22_V2_CONTEXTUAL_DERIVATION_API_GAP,
            "A public, replayable T-D2-1 derivation of Gamma; Delta |- node member, exposing every recursive premise with its own context and reconstructing the typed conclusion.",
            contextual_derivation_available,
            if contextual_derivation_available {
                "No obstruction: the public contextual derivation/replay and universal grammar induction both replay."
            } else if contextual_probe_passed {
                "The public LambdaIntroduction(BoundVariable(1)) probe is sound: its child retains one active binder and the same BoundVariable(1) is rejected closed. That local witness does not establish the universal contextual grammar/eliminator and constructor induction, which repaired T-D2-1 keeps false."
            } else {
                "The public contextual derivation/replay did not preserve the binder child while rejecting the same bound variable at closed context."
            },
        ),
        contract_check(
            TD22_V2_NATURAL_FAMILY_QUOTIENT_API_GAP,
            "A public theorem object carrying the dependent parameter telescope, checked canonical renaming, naturality for every well-typed substitution, univalent congruence, and the natural-family/instance quotient witness.",
            quotient_established,
            "The certificate currently reports quotient booleans and hashes. Existing typed-family machinery checks only a small sample of renamings; it does not export the universal dependent-family quotient theorem required by the adopted domain.",
        ),
        contract_check(
            TD22_V2_FULL_CONTENT_REGRESSION_GAP,
            "A public full-content projection from every sealed proved family and A3 membership row into the operational family judgment, including its role-specific typing and quotient witnesses.",
            full_content_regressions_proved,
            "Older proxy artifacts reported inventory/count assertions of 61 and 89, but repaired T-D2-1 recognizes zero exact judgments and does not promote those counts to an exact-content regression theorem. Counts and IDs are testimony, never a replacement for typed content.",
        ),
    ]
}

fn binding_digest(binding: &Td22V2Td21Binding) -> String {
    let mut projection = binding.clone();
    projection.binding_digest.clear();
    tagged_hash("td21-binding", &projection)
}

fn build_td21_binding(
    certificate: &Td21OperationalDomainV2Certificate,
) -> Result<Td22V2Td21Binding, Td22V2Error> {
    let replay = replay_t_d2_1_operational_domain_v2(certificate);
    let (
        contextual_binder_probe,
        closed_bound_variable_probe,
        contextual_binder_probe_publicly_replayed,
        binder_child_context_preserved,
        closed_bound_variable_rejected,
    ) = probe_contextual_contract()?;
    let contextual_probe_passed = contextual_binder_probe.typed_judgment_accepted
        && contextual_binder_probe.derivation.is_some()
        && contextual_binder_probe_publicly_replayed
        && binder_child_context_preserved
        && closed_bound_variable_rejected;
    let contextual_derivation_available = contextual_probe_passed
        && certificate
            .symbolic_carrier_grammar
            .membership_elimination_total
        && certificate.symbolic_carrier_induction.induction_total
        && certificate
            .context_finiteness
            .finite_inductive_grammar_proved;
    let quotient_probe_established = contextual_binder_probe.adopted_quotient_established
        && contextual_binder_probe.domain_member
        && contextual_binder_probe
            .derivation
            .as_ref()
            .is_some_and(|derivation| {
                derivation.quotient_evidence.adopted_quotient_established
                    && derivation
                        .quotient_evidence
                        .canonical_weakening_maps_checked
                    && derivation.quotient_evidence.canonical_renaming_maps_checked
                    && derivation.quotient_evidence.naturality_derivation_replayed
                    && derivation
                        .quotient_evidence
                        .univalent_equality_witnesses_replayed
            });
    let full_content_regressions_proved =
        certificate.family_regression_passed && certificate.membership_row_regression_passed;
    let checks = required_contract_checks(
        contextual_derivation_available,
        contextual_probe_passed,
        quotient_probe_established,
        full_content_regressions_proved,
    );
    let every_required_public_contract_available = checks
        .iter()
        .all(|check| check.available && check.no_proxy_accepted);
    let complete_typed_dependency_contract = replay.valid
        && certificate.t_d2_2_prerequisite_satisfied
        && every_required_public_contract_available;
    let exact_dependency_statement = if complete_typed_dependency_contract {
        "Every public typed dependency required by T-D2-2 replayed.".to_owned()
    } else {
        "T-D2-1's current public symbolic grammar proposal and archived proxy regressions do not supply the universal context-indexed induction, quotient witness, or exact-content regression theorem T-D2-2 needs. Finiteness itself remains unproved. The prerequisite is therefore false at the consumer boundary regardless of any older summary boolean."
            .to_owned()
    };
    let mut binding = Td22V2Td21Binding {
        typed_result_digest: certificate.result_digest.clone(),
        public_replay_valid: replay.valid,
        public_replay_errors: replay.errors,
        t_d2_1_reported_prerequisite_satisfied: certificate.t_d2_2_prerequisite_satisfied,
        symbolic_membership_elimination_reported_total: certificate
            .symbolic_carrier_grammar
            .membership_elimination_total,
        symbolic_induction_reported_total: certificate.symbolic_carrier_induction.induction_total,
        explicit_representative_vector_extensionally_complete: certificate
            .quotient_representatives_extensionally_complete,
        family_regression_reported_passed: certificate.family_regression_passed,
        membership_row_regression_reported_passed: certificate.membership_row_regression_passed,
        contextual_binder_probe,
        closed_bound_variable_probe,
        contextual_binder_probe_publicly_replayed,
        binder_child_context_preserved,
        closed_bound_variable_rejected,
        quotient_probe_established,
        required_contract_checks: checks,
        every_required_public_contract_available,
        complete_typed_dependency_contract,
        exact_dependency_statement,
        binding_digest: String::new(),
    };
    binding.binding_digest = binding_digest(&binding);
    Ok(binding)
}

fn named_gap(
    id: &str,
    phase: &str,
    exact_obstruction: &str,
    minimal_witness_shape: &str,
) -> Td22V2NamedGap {
    let mut gap = Td22V2NamedGap {
        id: id.to_owned(),
        phase: phase.to_owned(),
        exact_obstruction: exact_obstruction.to_owned(),
        minimal_witness_shape: minimal_witness_shape.to_owned(),
        keeps_grammar_unloaded: true,
        keeps_surjectivity_unissued: true,
        derivation_hash: String::new(),
    };
    let mut projection = gap.clone();
    projection.derivation_hash.clear();
    gap.derivation_hash = tagged_hash("named-gap", &projection);
    gap
}

fn named_gaps(checks: &[Td22V2RequiredContractCheck]) -> Vec<Td22V2NamedGap> {
    checks
        .iter()
        .filter(|check| !check.available)
        .map(|check| match check.contract_id.as_str() {
            TD22_V2_CONTEXTUAL_DERIVATION_API_GAP => named_gap(
                &check.contract_id,
                "T-D2-1 contextual membership dependency",
                &check.exact_obstruction,
                "LambdaIntroduction(BoundVariable(1)): the body is valid under one active binder and invalid as a closed member.",
            ),
            TD22_V2_NATURAL_FAMILY_QUOTIENT_API_GAP => named_gap(
                &check.contract_id,
                "T-D2-1 natural-family quotient dependency",
                &check.exact_obstruction,
                "A two-parameter family under an arbitrary well-typed dependent substitution outside the identity/single-transposition sample.",
            ),
            _ => named_gap(
                &check.contract_id,
                "T-D2-1 full-content regression dependency",
                &check.exact_obstruction,
                "A recognized regression row whose public projection supplies only recognition metadata rather than the complete typed family judgment.",
            ),
        })
        .collect()
}

fn zero_charge() -> Td22V2ZeroCharge {
    let mut zero = Td22V2ZeroCharge {
        kappa_minted: tagged_number(0, Td22V2Register::Gate, "kappa minted"),
        nu_minted: tagged_number(0, Td22V2Register::Gate, "nu minted"),
        anchors_minted: tagged_number(0, Td22V2Register::Gate, "anchors minted"),
        demand_orbits_minted: tagged_number(0, Td22V2Register::Gate, "demand orbits minted"),
        zero_charge_holds: true,
        derivation_hash: String::new(),
    };
    let mut projection = zero.clone();
    projection.derivation_hash.clear();
    zero.derivation_hash = tagged_hash("zero-charge", &projection);
    zero
}

fn gate() -> Td22V2Gate {
    let exact_gate_statement = "The T-D2-1 consumer contract is incomplete. BC-1 is not issued or inspected, no operational scope binding or generator derivation relation is constructed, neither inclusion is attempted, no theorem verdict exists, M-3 v1 remains authoritative, and M-4 remains unauthorized."
        .to_owned();
    let mut gate = Td22V2Gate {
        bridge_grammar_loaded: false,
        operational_scope_bindings_built: false,
        generator_derivation_relation_built: false,
        reach_subset_domain_attempted: false,
        domain_subset_reach_attempted: false,
        theorem_verdict_issued: false,
        surjectivity_proved: false,
        surjectivity_refuted: false,
        marker_surface_size: tagged_number(
            0,
            Td22V2Register::Gate,
            "target-marker surface; target annotations are forbidden and none are built",
        ),
        t_d2_3_key_schema_changed: false,
        m3_v1_remains_authoritative: true,
        m3_successor_issued: false,
        m4_authorized: false,
        exact_gate_statement,
        derivation_hash: String::new(),
    };
    let mut projection = gate.clone();
    projection.derivation_hash.clear();
    gate.derivation_hash = tagged_hash("gate", &projection);
    gate
}

fn mutation_falsifiers() -> Vec<String> {
    vec![
        "claim_a_context_stripped_recursive_premise_then_replay_must_fail".to_owned(),
        "replace_a_universal_naturality_witness_with_a_sample_renaming_then_replay_must_fail"
            .to_owned(),
        "replace_full_typed_regression_content_with_counts_or_identifiers_then_replay_must_fail"
            .to_owned(),
        "load_or_inspect_BC1_before_every_dependency_contract_is_public_then_replay_must_fail"
            .to_owned(),
        "construct_any_target_marker_or_generator_relation_on_this_stop_then_replay_must_fail"
            .to_owned(),
        "echo_any_claimed_proof_or_refutation_from_an_invalid_replay_then_replay_must_fail"
            .to_owned(),
        "mint_any_kappa_nu_anchor_or_demand_orbit_then_replay_must_fail".to_owned(),
        "move_M3_or_M4_or_change_T_D2_3_then_replay_must_fail".to_owned(),
        "read_any_desired_outcome_inventory_value_bar_or_hash_as_selector_then_replay_must_fail"
            .to_owned(),
        "add_any_unknown_JSON_field_then_replay_must_fail".to_owned(),
    ]
}

fn certificate_digest(certificate: &Td22OperationalSurjectivityV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn build_certificate() -> Result<Td22OperationalSurjectivityV2Certificate, Td22V2Error> {
    let td21 = issue_t_d2_1_operational_domain_v2()
        .map_err(|error| Td22V2Error::Input(error.to_string()))?;
    let td21_binding = build_td21_binding(&td21)?;
    if td21_binding.complete_typed_dependency_contract {
        return Err(Td22V2Error::Invariant(
            "the strict dependency-stop issuer unexpectedly found every repaired public contract; version the positive generator-derivation theorem before loading BC-1"
                .to_owned(),
        ));
    }
    let gaps = named_gaps(&td21_binding.required_contract_checks);
    let unavailable_contract_count = td21_binding
        .required_contract_checks
        .iter()
        .filter(|check| !check.available)
        .count();
    if gaps.len() != unavailable_contract_count || gaps.is_empty() {
        return Err(Td22V2Error::Invariant(
            "dependency gap projection lost or invented a required public contract".to_owned(),
        ));
    }
    let mut certificate = Td22OperationalSurjectivityV2Certificate {
        schema: T_D2_2_OPERATIONAL_SURJECTIVITY_V2_SCHEMA.to_owned(),
        date: T_D2_2_OPERATIONAL_SURJECTIVITY_V2_DATE.to_owned(),
        status: Td22V2RunStatus::StoppedOnTd21V2Prerequisite,
        theorem_disposition: Td22V2TheoremDisposition::NotAttemptedDependencyUnsatisfied,
        numeric_register_policy: "Every public quantitative datum is a tagged decimal. Digits in dates, versions, theorem labels, gap IDs, and digests are syntax identifiers."
            .to_owned(),
        source_bindings: source_bindings(),
        td21_binding,
        named_gaps: gaps,
        desired_outcome_read: false,
        generator_inventory_used_to_shape_domain: false,
        regression_inventory_used_to_shape_domain: false,
        selector_value_or_bar_read: false,
        zero_charge: zero_charge(),
        gate: gate(),
        exact_result: "Lawful dependency stop. T-D2-1's current public certificate supplies a bounded symbolic grammar proposal and a sound local binder probe, but it does not prove the universal context-indexed induction, finite quotient, natural-family quotient theorem, or full-content regression projection required by a noncircular T-D2-2 proof. The older count proxies remain insufficient testimony. The bridge grammar was never loaded; no target markers, generator derivations, inclusion attempts, proof, or refutation were produced."
            .to_owned(),
        permitted_conclusion: "T-D2-2 remains unattempted. This certificate proves only that its public dependency contract is incomplete; it is neither evidence for nor evidence against surjectivity."
            .to_owned(),
        required_successor_action: "Repair T-D2-1 with versioned public theorem objects for context-indexed membership, the complete dependent natural-family quotient, and full-content regression projection. Rerun this dependency audit create-new; only then may a separately versioned T-D2-2 generator-derivation theorem load BC-1."
            .to_owned(),
        mutation_falsifiers: mutation_falsifiers(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<Td22OperationalSurjectivityV2Certificate, String>> =
    OnceLock::new();

fn expected_certificate() -> Result<&'static Td22OperationalSurjectivityV2Certificate, Td22V2Error>
{
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Td22V2Error::Input(error.clone())),
    }
}

pub fn issue_t_d2_2_operational_surjectivity_v2()
-> Result<Td22OperationalSurjectivityV2Certificate, Td22V2Error> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> Td22V2Replay {
    Td22V2Replay {
        valid: false,
        errors: vec![error.into()],
        status: None,
        theorem_disposition: None,
        td21_dependency_satisfied: false,
        grammar_loaded: false,
        theorem_verdict_issued: false,
        surjectivity_proved: false,
        refutation_issued: false,
        m4_authorized: false,
    }
}

pub fn replay_t_d2_2_operational_surjectivity_v2(
    claimed: &Td22OperationalSurjectivityV2Certificate,
) -> Td22V2Replay {
    let expected = match expected_certificate() {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-D2-2 v2 result digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("T-D2-2 v2 source bindings drifted".to_owned());
    }
    if claimed != expected {
        errors.push("T-D2-2 v2 certificate differs from deterministic reissuance".to_owned());
    }
    if claimed.td21_binding.complete_typed_dependency_contract
        || claimed.gate.bridge_grammar_loaded
        || claimed.gate.operational_scope_bindings_built
        || claimed.gate.generator_derivation_relation_built
        || claimed.gate.reach_subset_domain_attempted
        || claimed.gate.domain_subset_reach_attempted
        || claimed.gate.theorem_verdict_issued
        || claimed.gate.surjectivity_proved
        || claimed.gate.surjectivity_refuted
        || claimed.gate.marker_surface_size.decimal != "0"
    {
        errors.push("T-D2-2 v2 dependency-stop or no-marker invariant failed".to_owned());
    }
    if claimed.desired_outcome_read
        || claimed.generator_inventory_used_to_shape_domain
        || claimed.regression_inventory_used_to_shape_domain
        || claimed.selector_value_or_bar_read
    {
        errors.push("T-D2-2 v2 independence/fitting firewall failed".to_owned());
    }
    if !claimed.zero_charge.zero_charge_holds
        || claimed.zero_charge.kappa_minted.decimal != "0"
        || claimed.zero_charge.nu_minted.decimal != "0"
        || claimed.zero_charge.anchors_minted.decimal != "0"
        || claimed.zero_charge.demand_orbits_minted.decimal != "0"
    {
        errors.push("T-D2-2 v2 zero-charge invariant failed".to_owned());
    }
    if claimed.gate.t_d2_3_key_schema_changed
        || !claimed.gate.m3_v1_remains_authoritative
        || claimed.gate.m3_successor_issued
        || claimed.gate.m4_authorized
    {
        errors.push("T-D2-2 v2 moved a forbidden downstream gate".to_owned());
    }
    let unavailable_contract_count = claimed
        .td21_binding
        .required_contract_checks
        .iter()
        .filter(|check| !check.available)
        .count();
    if claimed.named_gaps.len() != unavailable_contract_count
        || claimed.named_gaps.iter().any(|gap| {
            !gap.keeps_grammar_unloaded
                || !gap.keeps_surjectivity_unissued
                || gap.derivation_hash.is_empty()
        })
    {
        errors.push("T-D2-2 v2 named dependency gaps are incomplete".to_owned());
    }
    if !errors.is_empty() {
        return invalid_replay(errors.join("; "));
    }
    Td22V2Replay {
        valid: true,
        errors,
        status: Some(Td22V2RunStatus::StoppedOnTd21V2Prerequisite),
        theorem_disposition: Some(Td22V2TheoremDisposition::NotAttemptedDependencyUnsatisfied),
        td21_dependency_satisfied: false,
        grammar_loaded: false,
        theorem_verdict_issued: false,
        surjectivity_proved: false,
        refutation_issued: false,
        m4_authorized: false,
    }
}

pub fn replay_t_d2_2_operational_surjectivity_v2_json(json: &str) -> Td22V2Replay {
    match serde_json::from_str::<Td22OperationalSurjectivityV2Certificate>(json) {
        Ok(certificate) => replay_t_d2_2_operational_surjectivity_v2(&certificate),
        Err(error) => invalid_replay(format!("invalid T-D2-2 v2 JSON: {error}")),
    }
}

pub fn render_t_d2_2_operational_surjectivity_v2(
    certificate: &Td22OperationalSurjectivityV2Certificate,
) -> String {
    let mut out = String::new();
    out.push_str("# T-D2-2 operational surjectivity v2 dependency result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Status:** `stopped_on_td21_v2_prerequisite`. **Theorem disposition:** `not_attempted_dependency_unsatisfied`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("## Result\n\n");
    out.push_str(&certificate.exact_result);
    out.push_str("\n\n## Exact dependency gaps\n\n");
    for gap in &certificate.named_gaps {
        out.push_str(&format!(
            "- `{}`: {} Minimal witness shape: `{}`\n",
            gap.id, gap.exact_obstruction, gap.minimal_witness_shape
        ));
    }
    out.push_str("\n## Gate and charge\n\n");
    out.push_str(&certificate.gate.exact_gate_statement);
    out.push_str("\n\nMarker surface: **0**. Kappa, nu, anchors, and demand orbits minted: **0**. M-4 authorized: **false**.\n\n");
    out.push_str("## Permitted conclusion\n\n");
    out.push_str(&certificate.permitted_conclusion);
    out.push_str("\n\n## Required successor\n\n");
    out.push_str(&certificate.required_successor_action);
    out.push('\n');
    out
}

fn cleanup_created(path: &Path, created: bool) {
    if created {
        let _ = remove_file(path);
    }
}

pub fn emit_t_d2_2_operational_surjectivity_v2_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td22OperationalSurjectivityV2Certificate, Td22V2Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Td22V2Error::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_t_d2_2_operational_surjectivity_v2()?;
    let replay = replay_t_d2_2_operational_surjectivity_v2(&certificate);
    if !replay.valid {
        return Err(Td22V2Error::Invariant(format!(
            "new T-D2-2 v2 certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Td22V2Error::Json(error.to_string()))?;
    json.push(b'\n');
    let report = render_t_d2_2_operational_surjectivity_v2(&certificate);
    let mut certificate_created = false;
    let mut report_created = false;
    let result = (|| -> Result<(), Td22V2Error> {
        let mut certificate_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(certificate_path)
            .map_err(|error| Td22V2Error::Io(error.to_string()))?;
        certificate_created = true;
        certificate_file
            .write_all(&json)
            .and_then(|_| certificate_file.sync_all())
            .map_err(|error| Td22V2Error::Io(error.to_string()))?;
        drop(certificate_file);
        let mut report_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(report_path)
            .map_err(|error| Td22V2Error::Io(error.to_string()))?;
        report_created = true;
        report_file
            .write_all(report.as_bytes())
            .and_then(|_| report_file.sync_all())
            .map_err(|error| Td22V2Error::Io(error.to_string()))?;
        drop(report_file);
        let emitted_json =
            read_to_string(certificate_path).map_err(|error| Td22V2Error::Io(error.to_string()))?;
        let emitted_report =
            read(report_path).map_err(|error| Td22V2Error::Io(error.to_string()))?;
        if emitted_json.as_bytes() != json.as_slice()
            || emitted_report.as_slice() != report.as_bytes()
        {
            return Err(Td22V2Error::Invariant(
                "create-new bytes differ from deterministic payloads".to_owned(),
            ));
        }
        let emitted_replay = replay_t_d2_2_operational_surjectivity_v2_json(&emitted_json);
        if !emitted_replay.valid {
            return Err(Td22V2Error::Invariant(format!(
                "emitted T-D2-2 v2 certificate did not replay: {}",
                emitted_replay.errors.join("; ")
            )));
        }
        Ok(())
    })();
    if let Err(error) = result {
        cleanup_created(certificate_path, certificate_created);
        cleanup_created(report_path, report_created);
        return Err(error);
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reseal(certificate: &mut Td22OperationalSurjectivityV2Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    #[test]
    fn dependency_stop_never_loads_grammar_or_issues_a_verdict() {
        let certificate = issue_t_d2_2_operational_surjectivity_v2().expect("T-D2-2 v2 issues");
        let replay = replay_t_d2_2_operational_surjectivity_v2(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(certificate.td21_binding.public_replay_valid);
        assert!(!certificate.td21_binding.complete_typed_dependency_contract);
        assert!(!certificate.gate.bridge_grammar_loaded);
        assert!(!certificate.gate.generator_derivation_relation_built);
        assert!(!certificate.gate.reach_subset_domain_attempted);
        assert!(!certificate.gate.domain_subset_reach_attempted);
        assert!(!certificate.gate.theorem_verdict_issued);
        assert!(!certificate.gate.surjectivity_proved);
        assert!(!certificate.gate.surjectivity_refuted);
        assert_eq!(certificate.gate.marker_surface_size.decimal, "0");
        assert!(certificate.gate.m3_v1_remains_authoritative);
        assert!(!certificate.gate.m4_authorized);
    }

    #[test]
    fn exact_public_contract_gaps_are_named_without_proxies() {
        let certificate = issue_t_d2_2_operational_surjectivity_v2().expect("T-D2-2 v2 issues");
        assert_eq!(
            certificate
                .named_gaps
                .iter()
                .map(|gap| gap.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                TD22_V2_CONTEXTUAL_DERIVATION_API_GAP,
                TD22_V2_NATURAL_FAMILY_QUOTIENT_API_GAP,
                TD22_V2_FULL_CONTENT_REGRESSION_GAP,
            ]
        );
        let contextual = certificate
            .td21_binding
            .required_contract_checks
            .iter()
            .find(|check| check.contract_id == TD22_V2_CONTEXTUAL_DERIVATION_API_GAP)
            .expect("contextual contract");
        assert!(!contextual.available);
        assert!(contextual.no_proxy_accepted);
        assert!(certificate.td21_binding.binder_child_context_preserved);
        assert!(certificate.td21_binding.closed_bound_variable_rejected);
        assert!(
            certificate
                .td21_binding
                .required_contract_checks
                .iter()
                .all(|check| !check.available && check.no_proxy_accepted)
        );
        assert!(
            certificate
                .named_gaps
                .iter()
                .all(|gap| gap.keeps_grammar_unloaded && gap.keeps_surjectivity_unissued)
        );
    }

    #[test]
    fn resealed_nested_mutations_fail_and_replay_does_not_echo_claimed_verdicts() {
        let certificate = issue_t_d2_2_operational_surjectivity_v2().expect("T-D2-2 v2 issues");

        let mut contract = certificate.clone();
        contract.td21_binding.required_contract_checks[0].available = true;
        let mut contract_projection = contract.td21_binding.required_contract_checks[0].clone();
        contract_projection.derivation_hash.clear();
        contract.td21_binding.required_contract_checks[0].derivation_hash =
            tagged_hash("required-contract-check", &contract_projection);
        contract.td21_binding.binding_digest = binding_digest(&contract.td21_binding);
        reseal(&mut contract);
        let replay = replay_t_d2_2_operational_surjectivity_v2(&contract);
        assert!(!replay.valid);
        assert_eq!(replay.status, None);
        assert_eq!(replay.theorem_disposition, None);
        assert!(!replay.theorem_verdict_issued);
        assert!(!replay.surjectivity_proved);
        assert!(!replay.refutation_issued);

        let mut gap = certificate.clone();
        gap.named_gaps[0].keeps_grammar_unloaded = false;
        let mut gap_projection = gap.named_gaps[0].clone();
        gap_projection.derivation_hash.clear();
        gap.named_gaps[0].derivation_hash = tagged_hash("named-gap", &gap_projection);
        reseal(&mut gap);
        assert!(!replay_t_d2_2_operational_surjectivity_v2(&gap).valid);

        let mut grammar = certificate.clone();
        grammar.gate.bridge_grammar_loaded = true;
        let mut gate_projection = grammar.gate.clone();
        gate_projection.derivation_hash.clear();
        grammar.gate.derivation_hash = tagged_hash("gate", &gate_projection);
        reseal(&mut grammar);
        let replay = replay_t_d2_2_operational_surjectivity_v2(&grammar);
        assert!(!replay.valid);
        assert!(!replay.grammar_loaded);
        assert!(!replay.theorem_verdict_issued);

        let mut marker = certificate.clone();
        marker.gate.marker_surface_size.decimal = "1".to_owned();
        let mut gate_projection = marker.gate.clone();
        gate_projection.derivation_hash.clear();
        marker.gate.derivation_hash = tagged_hash("gate", &gate_projection);
        reseal(&mut marker);
        assert!(!replay_t_d2_2_operational_surjectivity_v2(&marker).valid);

        let mut charge = certificate.clone();
        charge.zero_charge.nu_minted.decimal = "1".to_owned();
        let mut zero_projection = charge.zero_charge.clone();
        zero_projection.derivation_hash.clear();
        charge.zero_charge.derivation_hash = tagged_hash("zero-charge", &zero_projection);
        reseal(&mut charge);
        assert!(!replay_t_d2_2_operational_surjectivity_v2(&charge).valid);

        let mut downstream = certificate;
        downstream.gate.m4_authorized = true;
        let mut gate_projection = downstream.gate.clone();
        gate_projection.derivation_hash.clear();
        downstream.gate.derivation_hash = tagged_hash("gate", &gate_projection);
        reseal(&mut downstream);
        assert!(!replay_t_d2_2_operational_surjectivity_v2(&downstream).valid);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_t_d2_2_operational_surjectivity_v2().expect("T-D2-2 v2 issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("certificate object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        let replay = replay_t_d2_2_operational_surjectivity_v2_json(
            &serde_json::to_string(&value).expect("json"),
        );
        assert!(!replay.valid);
        assert_eq!(replay.status, None);
        assert!(!replay.theorem_verdict_issued);
    }
}
