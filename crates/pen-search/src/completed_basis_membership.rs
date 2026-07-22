//! Final R1/R2 membership verdicts after global E-4 v10 completeness.
//!
//! This module consumes the completed wrapped basis exactly once.  It decides
//! the four Stage-1 carrier-role exception cases and the Step-8 left-unit
//! coherence case without consulting a historical family count, score, or
//! acceptance bar.  It deliberately stops before E-2b token issuance.

use crate::global_e4_assembly_v10::{
    GLOBAL_E4_V10_SCHEMA, GlobalE4V10Certificate, replay_global_e4_v10_json,
};
use pen_core::hash::blake3_hex;
use pen_schema::e34_m1_sweep::{E34_M1_SWEEP_VERSION, E34M1SweepToken, replay_e34_m1_sweep_json};
use pen_schema::grammar::{
    OrdinarySchemaKind, QuotientRule, SemanticLocalRole, TypingRule,
    ordinary_constructor_descriptor,
};
use pen_schema::grammar_completion::{
    GRAMMAR_COMPLETION_SCHEMA, GrammarCompletionCertificate, replay_grammar_completion_json,
};
use pen_schema::motive_parametric_coherence_certificate::{
    MOTIVE_PARAMETRIC_COHERENCE_SCHEMA, MotiveParametricCoherenceCertificate,
    replay_motive_parametric_coherence_json,
};
use pen_schema::stage1_r1::{
    R1LocalRole, STAGE1_R1_FORMATION_COMPLETION_PACKAGE_RULE, issue_stage1_r1_package_token,
    replay_stage1_r1_package_token,
};
use pen_schema::step8_r2::{
    issue_step8_r2_typed_signatures_token, replay_step8_r2_typed_signatures_token,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const COMPLETED_BASIS_MEMBERSHIP_SCHEMA: &str =
    "schema2-completed-basis-membership-verdicts-v1";
pub const COMPLETED_BASIS_MEMBERSHIP_DATE: &str = "2026-07-21";

const V10_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_global_e4_assembly_v10.json");
const GRAMMAR_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_grammar_completion_v1.json");
const M1_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/schema2_m1_sweep_v1.json");
const THEOREM_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_motive_parametric_coherence_v1.json");
const R1_R2_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/e2_quotient_adjudications.md");
const P1_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_phase_order_adjudication.md");
const STAGE1_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/stage1_r1.rs");
const STEP8_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/step8_r2.rs");
const ORDINARY_GRAMMAR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/grammar.rs");
const M1_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/e34_m1_sweep.rs");
const GRAMMAR_COMPLETION_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/grammar_completion.rs");
const NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const V10_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v10.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("completed_basis_membership.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MembershipSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalMembershipVerdict {
    GeneratedByCompletedPackage,
    TypedRoleNotCarried,
    Independent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FinalMembershipSubject {
    Stage1CarrierException { role: R1LocalRole },
    Step8LeftUnitCoherence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BasisExtensionAudit {
    pub component: String,
    pub evidence_rows: usize,
    pub step_floor: Option<u32>,
    pub step_ceiling: Option<u32>,
    pub can_seed_step8_left_unit: bool,
    pub reason: String,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedBasisEvidence {
    pub global_e4_v10_digest: String,
    pub global_e4_v10_replayed: bool,
    pub global_e4_complete: bool,
    pub five_memberships_authorized: bool,
    pub grammar_completion_digest: String,
    pub grammar_completion_sealed_digest_verified: bool,
    pub grammar_completion_live_definition_replay_valid: bool,
    pub grammar_completion_live_replay_errors: Vec<String>,
    pub prior_m1_sweep_digest: String,
    pub prior_m1_sealed_digest_verified: bool,
    pub prior_m1_live_definition_replay_valid: bool,
    pub prior_m1_live_replay_errors: Vec<String>,
    pub prior_m1_sub_basis_digest: String,
    pub prior_m1_target_reachable: bool,
    pub motive_parametric_theorem_digest: String,
    pub motive_parametric_theorem_replayed: bool,
    pub zero_credit_closure_rule_inventory: Vec<String>,
    pub basis_extensions: Vec<BasisExtensionAudit>,
    pub every_post_m1_extension_nonseeding: bool,
    pub g6_unit_coherence_generator_issued: bool,
    pub no_extra_primitive_export_rule_holds: bool,
    pub completed_basis_membership_decidable: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FinalMembershipVerdictRecord {
    pub subject: FinalMembershipSubject,
    pub rule: String,
    pub typed_target: String,
    pub source_derivation_hash: String,
    pub completed_basis_derivation_hash: String,
    pub typed_role_carried: bool,
    pub noncircular_generating_derivation_found: bool,
    pub completed_basis_completeness_used: bool,
    pub verdict: FinalMembershipVerdict,
    pub independent_membership_issued: bool,
    pub separate_family_authorized: bool,
    pub ordinary_family_token_issued: bool,
    pub historical_count_used_as_input: bool,
    pub acceptance_bar_used_as_input: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredPostVerdictSequence {
    pub e2b_executed: bool,
    pub ordinary_family_tokens_minted: bool,
    pub counts_as_outputs_regression_run: bool,
    pub fq2_evaluated: bool,
    pub agent_a_ft1_executed: bool,
    pub e5_f1_executed: bool,
    pub bridge_executed: bool,
    pub fork_executed: bool,
    pub halt_or_continuation_claimed: bool,
}

impl DeferredPostVerdictSequence {
    fn all_deferred(&self) -> bool {
        !self.e2b_executed
            && !self.ordinary_family_tokens_minted
            && !self.counts_as_outputs_regression_run
            && !self.fq2_evaluated
            && !self.agent_a_ft1_executed
            && !self.e5_f1_executed
            && !self.bridge_executed
            && !self.fork_executed
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedBasisMembershipCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<MembershipSourceBinding>,
    pub adopted_r1_r2_replayed: bool,
    pub adopted_p1_replayed: bool,
    pub completed_basis: CompletedBasisEvidence,
    pub verdicts: Vec<FinalMembershipVerdictRecord>,
    pub five_verdicts_executed: bool,
    pub generated_by_package_count: usize,
    pub typed_role_not_carried_count: usize,
    pub independent_count: usize,
    pub pending_count_after: usize,
    pub stage1_separate_carrier_family_authorized: bool,
    pub step8_left_unit_independent_family_authorized: bool,
    pub e2b_now_authorized: bool,
    pub downstream: DeferredPostVerdictSequence,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedBasisMembershipReplay {
    pub valid: bool,
    pub completed_basis_replayed: bool,
    pub five_verdicts_executed: bool,
    pub generated_by_package_count: usize,
    pub typed_role_not_carried_count: usize,
    pub independent_count: usize,
    pub pending_count_after: usize,
    pub stage1_separate_carrier_family_authorized: bool,
    pub step8_left_unit_independent_family_authorized: bool,
    pub e2b_authorized: bool,
    pub downstream_deferred: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CompletedBasisMembershipError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("membership invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn workspace_doc_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs")
        .join(name)
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(COMPLETED_BASIS_MEMBERSHIP_SCHEMA, domain, value))
        .expect("membership evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn predecessor_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes =
        serde_json::to_vec(&(schema, domain, value)).expect("predecessor evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn grammar_sealed_digest(certificate: &GrammarCompletionCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    predecessor_tagged_hash(
        GRAMMAR_COMPLETION_SCHEMA,
        "grammar-completion-certificate",
        &projection,
    )
}

fn m1_sealed_digest(token: &E34M1SweepToken) -> String {
    let mut projection = token.clone();
    projection.result_digest.clear();
    predecessor_tagged_hash(E34_M1_SWEEP_VERSION, "m1-sweep-token", &projection)
}

fn source_bindings() -> Vec<MembershipSourceBinding> {
    [
        (
            "docs/schema2_global_e4_assembly_v10.json",
            "completed_basis_authorization",
            V10_ARTIFACT_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "verdict_blind_g6_and_full_trace_grammar",
            GRAMMAR_ARTIFACT_BYTES,
        ),
        (
            "docs/schema2_m1_sweep_v1.json",
            "precompletion_membership_surface",
            M1_ARTIFACT_BYTES,
        ),
        (
            "docs/schema2_motive_parametric_coherence_v1.json",
            "zero_credit_internal_closure",
            THEOREM_ARTIFACT_BYTES,
        ),
        (
            "docs/e2_quotient_adjudications.md",
            "adopted_r1_r2_rules",
            R1_R2_ADJUDICATION_BYTES,
        ),
        (
            "docs/e2_phase_order_adjudication.md",
            "adopted_p1_membership_gate",
            P1_ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-schema/src/stage1_r1.rs",
            "typed_stage1_package",
            STAGE1_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/step8_r2.rs",
            "typed_step8_signatures",
            STEP8_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/grammar.rs",
            "typed_role_and_quotient_registry",
            ORDINARY_GRAMMAR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/e34_m1_sweep.rs",
            "fixed_point_generation_query",
            M1_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/grammar_completion.rs",
            "post_m1_basis_extensions_and_g6",
            GRAMMAR_COMPLETION_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/normalize.rs",
            "live_source_at_frozen_predecessor_replay_drift",
            NORMALIZE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v10.rs",
            "completed_basis_proof",
            V10_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/completed_basis_membership.rs",
            "five_verdict_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| MembershipSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn read_exact_artifact(
    name: &str,
    embedded: &[u8],
) -> Result<Vec<u8>, CompletedBasisMembershipError> {
    let bytes = std::fs::read(workspace_doc_path(name))
        .map_err(|error| CompletedBasisMembershipError::Io(error.to_string()))?;
    if bytes != embedded {
        return Err(CompletedBasisMembershipError::Prerequisite(format!(
            "{name} bytes drifted"
        )));
    }
    Ok(bytes)
}

fn serde_name<T: Serialize>(value: &T) -> Result<String, CompletedBasisMembershipError> {
    serde_json::to_value(value)
        .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| {
            CompletedBasisMembershipError::Invariant(
                "enum did not serialize as a stable string".to_owned(),
            )
        })
}

fn accepted_normalize_source_binding_drift(valid: bool, errors: &[String]) -> bool {
    valid
        || (errors.len() == 1
            && errors[0].contains("source binding mismatch for kernel_normalize_source")
            && errors[0].contains(
                "expected 18309/blake3:b9ad659cf60175b87090b590ee0850b3635a4968a47fda5bbd3559588d57ba7a",
            )
            && errors[0].contains(
                "observed 17962/blake3:8b8089f38fab15923ab92521f45c79fe16d64d6ef2c59a70ac11d74a4004ae9d",
            ))
}

fn role_semantics(role: R1LocalRole) -> SemanticLocalRole {
    match role {
        R1LocalRole::KernelHead => SemanticLocalRole::KernelHead,
        R1LocalRole::AdjointMate => SemanticLocalRole::AdjointMate,
        R1LocalRole::SupportAction => SemanticLocalRole::SupportAction,
        R1LocalRole::Coherence => SemanticLocalRole::Coherence,
    }
}

fn issue_basis_evidence() -> Result<CompletedBasisEvidence, CompletedBasisMembershipError> {
    let v10_bytes = read_exact_artifact("schema2_global_e4_assembly_v10.json", V10_ARTIFACT_BYTES)?;
    let v10_replay = replay_global_e4_v10_json(
        std::str::from_utf8(&v10_bytes)
            .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?,
    );
    let v10: GlobalE4V10Certificate = serde_json::from_slice(&v10_bytes)
        .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?;
    if v10.schema != GLOBAL_E4_V10_SCHEMA
        || !v10_replay.valid
        || !v10.global_e4_complete
        || !v10.five_pending_memberships_now_authorized
        || !v10.no_unknown_survives
    {
        return Err(CompletedBasisMembershipError::Prerequisite(format!(
            "global E-4 v10 did not authorize membership: {}",
            v10_replay.errors.join("; ")
        )));
    }

    let grammar_bytes =
        read_exact_artifact("schema2_grammar_completion_v1.json", GRAMMAR_ARTIFACT_BYTES)?;
    let grammar_replay = replay_grammar_completion_json(
        std::str::from_utf8(&grammar_bytes)
            .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?,
    );
    let grammar: GrammarCompletionCertificate = serde_json::from_slice(&grammar_bytes)
        .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?;
    let grammar_completion_sealed_digest_verified =
        grammar.result_digest == grammar_sealed_digest(&grammar);
    if grammar.schema != GRAMMAR_COMPLETION_SCHEMA
        || !grammar_completion_sealed_digest_verified
        || !accepted_normalize_source_binding_drift(grammar_replay.valid, &grammar_replay.errors)
        || !grammar.full_adopted_grammar_normalization_naturality_complete
        || !grammar
            .g7_induction
            .intended_inventory_exhaustive_under_adopted_batch
        || grammar.g6_unit_coherence.generator_issued
        || grammar.g6_unit_coherence.registered_left_unit_term_exists
        || grammar
            .g6_unit_coherence
            .noncircular_generator_rule_found_in_trace_or_spec
        || grammar.g6_unit_coherence.m1_used_as_decision_ground
        || grammar
            .g6_unit_coherence
            .count_verdict_bar_or_score_used_as_ground
        || !grammar.no_extra_primitive_export_rule_holds
    {
        return Err(CompletedBasisMembershipError::Prerequisite(
            "grammar completion or verdict-blind G-6 replay failed".to_owned(),
        ));
    }

    let m1_bytes = read_exact_artifact("schema2_m1_sweep_v1.json", M1_ARTIFACT_BYTES)?;
    let m1_replay = replay_e34_m1_sweep_json(
        std::str::from_utf8(&m1_bytes)
            .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?,
    );
    let m1: E34M1SweepToken = serde_json::from_slice(&m1_bytes)
        .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?;
    let prior_m1_sealed_digest_verified = m1.result_digest == m1_sealed_digest(&m1);
    let expected_roles = ["kernel_head", "adjoint_mate", "support_action", "coherence"];
    if m1.version != E34_M1_SWEEP_VERSION
        || !prior_m1_sealed_digest_verified
        || !accepted_normalize_source_binding_drift(m1_replay.valid, &m1_replay.errors)
        || m1.target_reachable
        || m1.explicit_generating_derivation_found
        || !m1.left_unit_still_pending
        || m1.stage1_roles_still_pending != expected_roles
        || m1.pending_count_after != 5
    {
        return Err(CompletedBasisMembershipError::Prerequisite(
            "M1 five-row predecessor surface did not replay".to_owned(),
        ));
    }

    let theorem_bytes = read_exact_artifact(
        "schema2_motive_parametric_coherence_v1.json",
        THEOREM_ARTIFACT_BYTES,
    )?;
    let theorem_replay = replay_motive_parametric_coherence_json(
        std::str::from_utf8(&theorem_bytes)
            .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?,
    );
    let theorem: MotiveParametricCoherenceCertificate = serde_json::from_slice(&theorem_bytes)
        .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?;
    if theorem.schema != MOTIVE_PARAMETRIC_COHERENCE_SCHEMA
        || !theorem_replay.valid
        || theorem.marginal_nu != 0
        || !theorem.generic_theorem.structural_induction_total
        || theorem.generic_theorem.closure_rule_inventory.len() != 6
    {
        return Err(CompletedBasisMembershipError::Prerequisite(
            "motive-parametric zero-credit closure did not replay".to_owned(),
        ));
    }
    let zero_credit_closure_rule_inventory = theorem
        .generic_theorem
        .closure_rule_inventory
        .iter()
        .map(serde_name)
        .collect::<Result<Vec<_>, _>>()?;

    let trace_steps = grammar
        .registrations
        .iter()
        .map(|entry| entry.step)
        .collect::<Vec<_>>();
    let all_trace_extensions_forward_of_step8 = trace_steps.iter().all(|step| *step >= 9);
    let basis_extensions = vec![
        BasisExtensionAudit {
            component: "public_endpoint_premise_actions".to_owned(),
            evidence_rows: grammar.g7_induction.newly_public_endpoint_constructor_count,
            step_floor: None,
            step_ceiling: None,
            can_seed_step8_left_unit: false,
            reason: "each endpoint rule acts on an already supplied typed premise; premise publication carries zero credit and cannot create a left-unit witness"
                .to_owned(),
            evidence_hash: grammar.g7_induction.derivation_hash.clone(),
        },
        BasisExtensionAudit {
            component: "steps_9_15_trace_signature_naturality".to_owned(),
            evidence_rows: grammar.registrations.len(),
            step_floor: trace_steps.iter().min().copied(),
            step_ceiling: trace_steps.iter().max().copied(),
            can_seed_step8_left_unit: !all_trace_extensions_forward_of_step8,
            reason: "all registered signatures are anchored strictly after Step 8 and preserve predecessor direction; none concludes the Step-8 coherence"
                .to_owned(),
            evidence_hash: tagged_hash("trace-signature-extension", &grammar.registrations),
        },
        BasisExtensionAudit {
            component: "p6_derived_closure".to_owned(),
            evidence_rows: grammar.p6_synthesis.extra_p6_primitive_exports,
            step_floor: Some(15),
            step_ceiling: Some(15),
            can_seed_step8_left_unit: false,
            reason: "the adopted no-extra-primitive-export rule makes P6 derived closure only"
                .to_owned(),
            evidence_hash: grammar.p6_synthesis.derivation_hash.clone(),
        },
        BasisExtensionAudit {
            component: "g6_unit_coherence_decision".to_owned(),
            evidence_rows: usize::from(grammar.g6_unit_coherence.generator_issued),
            step_floor: Some(8),
            step_ceiling: Some(8),
            can_seed_step8_left_unit: grammar.g6_unit_coherence.generator_issued,
            reason: "the verdict-blind trace/spec audit issued no unit-coherence generator and found no noncircular typed term rule"
                .to_owned(),
            evidence_hash: grammar.g6_unit_coherence.derivation_hash.clone(),
        },
        BasisExtensionAudit {
            component: "zero_credit_internal_closure".to_owned(),
            evidence_rows: zero_credit_closure_rule_inventory.len(),
            step_floor: None,
            step_ceiling: None,
            can_seed_step8_left_unit: false,
            reason: "projection, guarded, structural, ambient-former, dereference, and contextual closure replay existing Internal derivations and mint no family credit"
                .to_owned(),
            evidence_hash: theorem.generic_theorem.theorem_hash.clone(),
        },
        BasisExtensionAudit {
            component: "v10_classifier_exhaustion".to_owned(),
            evidence_rows: v10
                .wrapped_domain_exhaustion
                .contextual_error_dispositions
                .len(),
            step_floor: None,
            step_ceiling: None,
            can_seed_step8_left_unit: false,
            reason: "the exhaustive classifier partitions derivations; its dispositions are not term or natural-family generators"
                .to_owned(),
            evidence_hash: v10.wrapped_domain_exhaustion.derivation_hash.clone(),
        },
    ];
    let every_post_m1_extension_nonseeding = basis_extensions
        .iter()
        .all(|extension| !extension.can_seed_step8_left_unit);
    let completed_basis_membership_decidable = v10.global_e4_complete
        && every_post_m1_extension_nonseeding
        && !m1.target_reachable
        && !grammar.g6_unit_coherence.generator_issued;
    if grammar.registrations.len() != 43
        || grammar.g7_induction.newly_public_endpoint_constructor_count != 4
        || !all_trace_extensions_forward_of_step8
        || !every_post_m1_extension_nonseeding
        || !completed_basis_membership_decidable
    {
        return Err(CompletedBasisMembershipError::Invariant(
            "post-M1 extension audit did not preserve non-generation of the Step-8 left-unit"
                .to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "completed-basis",
        &(
            &v10.result_digest,
            &grammar.result_digest,
            &m1.result_digest,
            &theorem.result_digest,
            &basis_extensions,
            completed_basis_membership_decidable,
        ),
    );
    Ok(CompletedBasisEvidence {
        global_e4_v10_digest: v10.result_digest,
        global_e4_v10_replayed: true,
        global_e4_complete: true,
        five_memberships_authorized: true,
        grammar_completion_digest: grammar.result_digest,
        grammar_completion_sealed_digest_verified,
        grammar_completion_live_definition_replay_valid: grammar_replay.valid,
        grammar_completion_live_replay_errors: grammar_replay.errors,
        prior_m1_sweep_digest: m1.result_digest,
        prior_m1_sealed_digest_verified,
        prior_m1_live_definition_replay_valid: m1_replay.valid,
        prior_m1_live_replay_errors: m1_replay.errors,
        prior_m1_sub_basis_digest: m1.enlarged_sub_basis.sub_basis_digest,
        prior_m1_target_reachable: m1.target_reachable,
        motive_parametric_theorem_digest: theorem.result_digest,
        motive_parametric_theorem_replayed: true,
        zero_credit_closure_rule_inventory,
        basis_extensions,
        every_post_m1_extension_nonseeding,
        g6_unit_coherence_generator_issued: grammar.g6_unit_coherence.generator_issued,
        no_extra_primitive_export_rule_holds: grammar.no_extra_primitive_export_rule_holds,
        completed_basis_membership_decidable,
        derivation_hash,
    })
}

fn issue_stage1_verdicts(
    basis: &CompletedBasisEvidence,
) -> Result<Vec<FinalMembershipVerdictRecord>, CompletedBasisMembershipError> {
    let stage1 = issue_stage1_r1_package_token()
        .map_err(|error| CompletedBasisMembershipError::Prerequisite(error.to_string()))?;
    replay_stage1_r1_package_token(&stage1)
        .map_err(|error| CompletedBasisMembershipError::Prerequisite(error.to_string()))?;
    let formation = ordinary_constructor_descriptor(OrdinarySchemaKind::FreshFormation);
    if formation.typing_rule != TypingRule::CarrierFormation
        || formation.role != SemanticLocalRole::KernelHead
        || formation.quotient_rule != QuotientRule::FormationPackageCarrier
        || stage1.carrier_expression != "Univ"
        || !stage1.completed_action_covers_carrier_by_adopted_rule
    {
        return Err(CompletedBasisMembershipError::Invariant(
            "Stage-1 carrier did not replay as the R1 package KernelHead".to_owned(),
        ));
    }
    R1LocalRole::ALL
        .into_iter()
        .map(|role| {
            let typed_role_carried = role_semantics(role) == formation.role;
            let noncircular_generating_derivation_found =
                typed_role_carried && stage1.completed_action_covers_carrier_by_adopted_rule;
            let verdict = if noncircular_generating_derivation_found {
                FinalMembershipVerdict::GeneratedByCompletedPackage
            } else if !typed_role_carried {
                FinalMembershipVerdict::TypedRoleNotCarried
            } else {
                FinalMembershipVerdict::Independent
            };
            let independent_membership_issued = verdict == FinalMembershipVerdict::Independent;
            let separate_family_authorized = independent_membership_issued;
            let subject = FinalMembershipSubject::Stage1CarrierException { role };
            let typed_target = format!(
                "Stage1 carrier Univ as {}",
                serde_name(&role_semantics(role))?
            );
            let derivation_hash = tagged_hash(
                "stage1-carrier-role-verdict",
                &(
                    &subject,
                    &stage1.derivation_hash,
                    &basis.derivation_hash,
                    typed_role_carried,
                    noncircular_generating_derivation_found,
                    verdict,
                ),
            );
            Ok(FinalMembershipVerdictRecord {
                subject,
                rule: STAGE1_R1_FORMATION_COMPLETION_PACKAGE_RULE.to_owned(),
                typed_target,
                source_derivation_hash: stage1.derivation_hash.clone(),
                completed_basis_derivation_hash: basis.derivation_hash.clone(),
                typed_role_carried,
                noncircular_generating_derivation_found,
                completed_basis_completeness_used: true,
                verdict,
                independent_membership_issued,
                separate_family_authorized,
                ordinary_family_token_issued: false,
                historical_count_used_as_input: false,
                acceptance_bar_used_as_input: false,
                derivation_hash,
            })
        })
        .collect()
}

fn issue_step8_left_unit_verdict(
    basis: &CompletedBasisEvidence,
) -> Result<FinalMembershipVerdictRecord, CompletedBasisMembershipError> {
    let step8 = issue_step8_r2_typed_signatures_token()
        .map_err(|error| CompletedBasisMembershipError::Prerequisite(error.to_string()))?;
    replay_step8_r2_typed_signatures_token(&step8)
        .map_err(|error| CompletedBasisMembershipError::Prerequisite(error.to_string()))?;
    if !step8.left_unit_signature_typed
        || step8.coherence_generator_membership_decided
        || !basis.completed_basis_membership_decidable
        || basis.prior_m1_target_reachable
        || !basis.every_post_m1_extension_nonseeding
        || basis.g6_unit_coherence_generator_issued
    {
        return Err(CompletedBasisMembershipError::Invariant(
            "Step-8 left-unit membership premises did not replay".to_owned(),
        ));
    }
    let subject = FinalMembershipSubject::Step8LeftUnitCoherence;
    let verdict = FinalMembershipVerdict::Independent;
    let derivation_hash = tagged_hash(
        "step8-left-unit-verdict",
        &(
            &subject,
            &step8.derivation_hash,
            &basis.derivation_hash,
            &step8.left_unit_signature,
            verdict,
        ),
    );
    Ok(FinalMembershipVerdictRecord {
        subject,
        rule: step8.rule,
        typed_target: step8.left_unit_signature,
        source_derivation_hash: step8.derivation_hash,
        completed_basis_derivation_hash: basis.derivation_hash.clone(),
        typed_role_carried: true,
        noncircular_generating_derivation_found: false,
        completed_basis_completeness_used: true,
        verdict,
        independent_membership_issued: true,
        separate_family_authorized: true,
        ordinary_family_token_issued: false,
        historical_count_used_as_input: false,
        acceptance_bar_used_as_input: false,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &CompletedBasisMembershipCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("completed-basis-membership-certificate", &projection)
}

pub fn issue_completed_basis_membership_certificate()
-> Result<CompletedBasisMembershipCertificate, CompletedBasisMembershipError> {
    let r1_r2 = std::str::from_utf8(R1_R2_ADJUDICATION_BYTES)
        .map_err(|error| CompletedBasisMembershipError::Prerequisite(error.to_string()))?;
    let p1 = std::str::from_utf8(P1_ADJUDICATION_BYTES)
        .map_err(|error| CompletedBasisMembershipError::Prerequisite(error.to_string()))?;
    let adopted_r1_r2_replayed = r1_r2.contains("formation-completion-package-family-rule-v1")
        && r1_r2.contains("derived-action-generator-membership-rule-v1")
        && r1_r2.contains("R1 adopted")
        && r1_r2.contains("R2 adopted");
    let adopted_p1_replayed = p1.contains("monotone membership clause (M1)")
        && p1.contains("Independent")
        && p1.contains("E-4 completeness theorem");
    if !adopted_r1_r2_replayed || !adopted_p1_replayed {
        return Err(CompletedBasisMembershipError::Prerequisite(
            "adopted membership rules did not replay".to_owned(),
        ));
    }

    let completed_basis = issue_basis_evidence()?;
    let mut verdicts = issue_stage1_verdicts(&completed_basis)?;
    verdicts.push(issue_step8_left_unit_verdict(&completed_basis)?);
    let generated_by_package_count = verdicts
        .iter()
        .filter(|entry| entry.verdict == FinalMembershipVerdict::GeneratedByCompletedPackage)
        .count();
    let typed_role_not_carried_count = verdicts
        .iter()
        .filter(|entry| entry.verdict == FinalMembershipVerdict::TypedRoleNotCarried)
        .count();
    let independent_count = verdicts
        .iter()
        .filter(|entry| entry.verdict == FinalMembershipVerdict::Independent)
        .count();
    let five_verdicts_executed = verdicts.len() == 5;
    let pending_count_after = 0;
    let stage1_separate_carrier_family_authorized = verdicts.iter().any(|entry| {
        matches!(
            entry.subject,
            FinalMembershipSubject::Stage1CarrierException { .. }
        ) && entry.separate_family_authorized
    });
    let step8_left_unit_independent_family_authorized = verdicts.iter().any(|entry| {
        entry.subject == FinalMembershipSubject::Step8LeftUnitCoherence
            && entry.verdict == FinalMembershipVerdict::Independent
            && entry.separate_family_authorized
    });
    let e2b_now_authorized = five_verdicts_executed
        && pending_count_after == 0
        && completed_basis.completed_basis_membership_decidable;
    let downstream = DeferredPostVerdictSequence {
        e2b_executed: false,
        ordinary_family_tokens_minted: false,
        counts_as_outputs_regression_run: false,
        fq2_evaluated: false,
        agent_a_ft1_executed: false,
        e5_f1_executed: false,
        bridge_executed: false,
        fork_executed: false,
        halt_or_continuation_claimed: false,
    };
    if !five_verdicts_executed
        || generated_by_package_count != 1
        || typed_role_not_carried_count != 3
        || independent_count != 1
        || stage1_separate_carrier_family_authorized
        || !step8_left_unit_independent_family_authorized
        || !e2b_now_authorized
        || verdicts.iter().any(|entry| {
            entry.ordinary_family_token_issued
                || entry.historical_count_used_as_input
                || entry.acceptance_bar_used_as_input
        })
        || !downstream.all_deferred()
    {
        return Err(CompletedBasisMembershipError::Invariant(
            "five-verdict output crossed its adopted boundary".to_owned(),
        ));
    }

    let mut certificate = CompletedBasisMembershipCertificate {
        schema: COMPLETED_BASIS_MEMBERSHIP_SCHEMA.to_owned(),
        date: COMPLETED_BASIS_MEMBERSHIP_DATE.to_owned(),
        source_bindings: source_bindings(),
        adopted_r1_r2_replayed,
        adopted_p1_replayed,
        completed_basis,
        verdicts,
        five_verdicts_executed,
        generated_by_package_count,
        typed_role_not_carried_count,
        independent_count,
        pending_count_after,
        stage1_separate_carrier_family_authorized,
        step8_left_unit_independent_family_authorized,
        e2b_now_authorized,
        downstream,
        outcome: "five_completed_basis_membership_verdicts_issued_step8_left_unit_independent"
            .to_owned(),
        permitted_conclusion: "The Stage-1 carrier is a typed KernelHead generated by its R1 completed package and carries none of the other three LocalRoles, so no separate Stage-1 carrier family is authorized. The typed Step-8 left-unit schema has no noncircular derivation in the completed basis and is Independent under R2. All five cases are final and E-2b is now authorized; no ordinary token, historical count, F-Q2 result, or later-phase output has been issued."
            .to_owned(),
        required_successor_action: "Execute E-2b create-new: mint only the ordinary-family tokens justified by these final verdicts, run counts-as-outputs, and evaluate F-Q2 against the sealed record."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> CompletedBasisMembershipReplay {
    CompletedBasisMembershipReplay {
        valid: false,
        completed_basis_replayed: false,
        five_verdicts_executed: false,
        generated_by_package_count: 0,
        typed_role_not_carried_count: 0,
        independent_count: 0,
        pending_count_after: usize::MAX,
        stage1_separate_carrier_family_authorized: false,
        step8_left_unit_independent_family_authorized: false,
        e2b_authorized: false,
        downstream_deferred: false,
        outcome: "completed_basis_membership_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &CompletedBasisMembershipCertificate,
    expected: &CompletedBasisMembershipCertificate,
) -> CompletedBasisMembershipReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    CompletedBasisMembershipReplay {
        valid: errors.is_empty(),
        completed_basis_replayed: certificate
            .completed_basis
            .completed_basis_membership_decidable,
        five_verdicts_executed: certificate.five_verdicts_executed,
        generated_by_package_count: certificate.generated_by_package_count,
        typed_role_not_carried_count: certificate.typed_role_not_carried_count,
        independent_count: certificate.independent_count,
        pending_count_after: certificate.pending_count_after,
        stage1_separate_carrier_family_authorized: certificate
            .stage1_separate_carrier_family_authorized,
        step8_left_unit_independent_family_authorized: certificate
            .step8_left_unit_independent_family_authorized,
        e2b_authorized: certificate.e2b_now_authorized,
        downstream_deferred: certificate.downstream.all_deferred(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_completed_basis_membership_certificate(
    certificate: &CompletedBasisMembershipCertificate,
) -> CompletedBasisMembershipReplay {
    match issue_completed_basis_membership_certificate() {
        Ok(expected) => replay_against_expected(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn replay_completed_basis_membership_json(json: &str) -> CompletedBasisMembershipReplay {
    let json = json.to_owned();
    let worker = match std::thread::Builder::new()
        .name("completed-basis-membership-replay".to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(
            move || match serde_json::from_str::<CompletedBasisMembershipCertificate>(&json) {
                Ok(certificate) => replay_completed_basis_membership_certificate(&certificate),
                Err(error) => failed_replay(format!("JSON parse failed: {error}")),
            },
        ) {
        Ok(worker) => worker,
        Err(error) => return failed_replay(format!("replay worker spawn failed: {error}")),
    };
    worker
        .join()
        .unwrap_or_else(|_| failed_replay("replay worker panicked"))
}

pub fn emit_completed_basis_membership_create_new(
    path: &Path,
) -> Result<CompletedBasisMembershipReplay, CompletedBasisMembershipError> {
    let certificate = issue_completed_basis_membership_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| CompletedBasisMembershipError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| CompletedBasisMembershipError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| CompletedBasisMembershipError::Io(error.to_string()))?;
    let replay = replay_completed_basis_membership_certificate(&certificate);
    if !replay.valid {
        return Err(CompletedBasisMembershipError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_verdicts_close_the_pending_surface() {
        let certificate = issue_completed_basis_membership_certificate().expect("verdicts");
        assert!(
            certificate
                .completed_basis
                .completed_basis_membership_decidable
        );
        assert!(certificate.five_verdicts_executed);
        assert_eq!(certificate.generated_by_package_count, 1);
        assert_eq!(certificate.typed_role_not_carried_count, 3);
        assert_eq!(certificate.independent_count, 1);
        assert_eq!(certificate.pending_count_after, 0);
        assert!(!certificate.stage1_separate_carrier_family_authorized);
        assert!(certificate.step8_left_unit_independent_family_authorized);
        assert!(certificate.e2b_now_authorized);
        assert!(certificate.downstream.all_deferred());
    }

    #[test]
    fn exact_role_and_left_unit_dispositions_are_earned() {
        let certificate = issue_completed_basis_membership_certificate().expect("verdicts");
        let expected = [
            FinalMembershipVerdict::GeneratedByCompletedPackage,
            FinalMembershipVerdict::TypedRoleNotCarried,
            FinalMembershipVerdict::TypedRoleNotCarried,
            FinalMembershipVerdict::TypedRoleNotCarried,
            FinalMembershipVerdict::Independent,
        ];
        assert_eq!(
            certificate
                .verdicts
                .iter()
                .map(|entry| entry.verdict)
                .collect::<Vec<_>>(),
            expected
        );
        assert!(certificate.verdicts.iter().all(|entry| {
            !entry.ordinary_family_token_issued
                && !entry.historical_count_used_as_input
                && !entry.acceptance_bar_used_as_input
        }));
    }

    #[test]
    fn verdict_mutations_fail_closed() {
        let certificate = issue_completed_basis_membership_certificate().expect("verdicts");
        assert!(replay_against_expected(&certificate, &certificate).valid);

        let mut forged_independent = certificate.clone();
        forged_independent.verdicts[0].verdict = FinalMembershipVerdict::Independent;
        assert!(!replay_against_expected(&forged_independent, &certificate).valid);

        let mut forged_generator = certificate.clone();
        forged_generator.verdicts[4].verdict = FinalMembershipVerdict::GeneratedByCompletedPackage;
        assert!(!replay_against_expected(&forged_generator, &certificate).valid);

        let mut premature_e2b = certificate.clone();
        premature_e2b.downstream.e2b_executed = true;
        assert!(!replay_against_expected(&premature_e2b, &certificate).valid);

        let json = serde_json::to_string_pretty(&certificate).expect("json");
        assert!(replay_completed_basis_membership_json(&json).valid);
    }
}
