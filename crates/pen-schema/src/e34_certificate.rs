//! Create-new, fail-closed successor certificate for the P1-authorized
//! E-3/E-4 development pass.
//!
//! Procedural adjudication P1 splits E-2 into a completed typing milestone
//! (E-2a) and a quotient-closure milestone (E-2b) that follows E-4.  This
//! certificate replays the exact E-2 predecessor, the adopted P1 text, the
//! scoped E-3 audit, and the partial E-4 audit.  M1 permits one final
//! `Generated` verdict for the Step-8 cell action.  Five membership cases
//! remain pending and no output reserved to complete E-4 or E-2b is issued.

use crate::e2_certificate::{
    SCHEMA2_E2_CERTIFICATE_SCHEMA, Schema2E2Certificate, replay_schema2_e2_json,
};
use crate::e3_normalization::{
    E3_GENERAL_C6_GAP, E3_GENERAL_UNIVALENT_EQUALITY_GAP, E3_TOTAL_NORMALIZATION_GAP,
    SCHEMA2_E3_FROZEN_FRAGMENT_VERSION, issue_e3_fragment_audit, replay_e3_fragment_audit,
};
use crate::e4_generator_basis::{
    E4_GENERATOR_BASIS_VERSION, E4_GLOBAL_COMPLETENESS_GAP, M1GeneratedSubject, M1PendingSubject,
    Schema2NaturalityClass, issue_e4_development_audit, replay_e4_development_audit,
};
use crate::grammar::DERIVED_ACTION_MEMBERSHIP_RULE;
use crate::stage1_r1::R1LocalRole;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const SCHEMA2_E34_CERTIFICATE_SCHEMA: &str = "schema2-e3-e4-successor-v3";
pub const SCHEMA2_E34_CERTIFICATE_DATE: &str = "2026-07-20";
pub const PROCEDURAL_ADJUDICATION_ID: &str = "procedural-adjudication-p1-e2a-e3-e4-e2b-order";
pub const MONOTONE_MEMBERSHIP_RULE_ID: &str = "procedural-adjudication-p1-monotone-membership-m1";

const PREDECESSOR_BYTES: &[u8] = include_bytes!("../../../docs/schema2_v2.json");
const P1_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_phase_order_adjudication.md");
const E2_QUOTIENT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/e2_quotient_adjudications.md");
const AGENT_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_schema2_plan.md");
const OPEN_PROBLEM_SPEC_BYTES: &[u8] =
    include_bytes!("../../../docs/step_15_completion_open_problem.md");
const E2_CERTIFICATE_SOURCE_BYTES: &[u8] = include_bytes!("e2_certificate.rs");
const E3_SOURCE_BYTES: &[u8] = include_bytes!("e3_normalization.rs");
const E4_SOURCE_BYTES: &[u8] = include_bytes!("e4_generator_basis.rs");
const CONTEXT_SOURCE_BYTES: &[u8] = include_bytes!("context.rs");
const GRAMMAR_SOURCE_BYTES: &[u8] = include_bytes!("grammar.rs");
const ORDINARY_SOURCE_BYTES: &[u8] = include_bytes!("ordinary.rs");
const STAGE1_SOURCE_BYTES: &[u8] = include_bytes!("stage1_r1.rs");
const STEP8_SOURCE_BYTES: &[u8] = include_bytes!("step8_r2.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const CUBICAL_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/cubical.rs");
const TELESCOPE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const TYPED_BOUNDARY_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/cubical/typed_boundary.rs");
const SCHEMA2_AGDA_BYTES: &[u8] = include_bytes!("../../../agda/Schema2.agda");
const SCHEMA2_E2_AGDA_BYTES: &[u8] = include_bytes!("../../../agda/Schema2E2.agda");
const SCHEMA2_E34_AGDA_BYTES: &[u8] = include_bytes!("../../../agda/Schema2E3E4.agda");
const KERNEL_BRIDGE_AGDA_BYTES: &[u8] = include_bytes!("../../../agda/KernelBridge.agda");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Schema2E34CertificateError {
    #[error("source binding mismatch for {name}: expected {expected}, observed {observed}")]
    SourceBinding {
        name: String,
        expected: String,
        observed: String,
    },
    #[error("invalid UTF-8 in {0}")]
    Utf8(String),
    #[error("predecessor replay failed: {0}")]
    Predecessor(String),
    #[error("adoption replay failed: {0}")]
    Adoption(String),
    #[error("plan/spec replay failed: {0}")]
    PlanSpec(String),
    #[error("E-3 audit failed: {0}")]
    E3(String),
    #[error("E-4 audit failed: {0}")]
    E4(String),
    #[error("Agda source boundary failed: {0}")]
    Agda(String),
    #[error("certificate invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E34SourceBinding {
    pub name: String,
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
    pub semantic_projection_replayed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E34PredecessorEvidence {
    pub path: String,
    pub schema: String,
    pub result_digest: String,
    pub exact_source_binding_replayed: bool,
    pub strict_certificate_replay_valid: bool,
    pub predecessor_e2_complete: bool,
    pub predecessor_fq4_triggered: bool,
    pub predecessor_handoff_issued: bool,
    pub predecessor_halt_claimed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct P1AdoptionEvidence {
    pub adjudication_id: String,
    pub membership_rule_id: String,
    pub exact_document_bound: bool,
    pub adoption_block_replayed_verbatim: bool,
    pub adopter: String,
    pub adoption_date: String,
    pub adopted_phase_order: Vec<String>,
    pub counts_pinned_exclusively_to_e2b: bool,
    pub generated_verdicts_monotone_and_final: bool,
    pub independent_verdicts_require_complete_e4: bool,
    pub amendment_is_versioned_successor: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2aEvidence {
    pub declared_complete_by_adopted_p1: bool,
    pub predecessor_stage1_completed_package_issued: bool,
    pub predecessor_step8_registered_signatures_replayed: bool,
    pub predecessor_ordinary_registry_typed: bool,
    pub typing_milestone_complete: bool,
    pub quotient_membership_decided: bool,
    pub count_or_handoff_issued: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E3FragmentEvidence {
    pub version: String,
    pub audit_derivation_hash: String,
    pub audit_issued_and_replayed: bool,
    pub fragment_milestone_complete: bool,
    pub typed_beta_normalization_replayed: bool,
    pub positive_and_negative_frozen_equality_replayed: bool,
    pub weakening_congruence_replayed: bool,
    pub genuine_substitution_congruence_replayed: bool,
    pub provenance_retained: bool,
    pub global_e3_complete: bool,
    pub total_schema2_normalization_proved: bool,
    pub general_univalent_equality_proved: bool,
    pub general_c6_complete: bool,
    pub gaps: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M1GeneratedEvidence {
    pub rule: String,
    pub membership_rule: String,
    pub subject: String,
    pub sub_basis_digest: String,
    pub membership_derivation_hash: String,
    pub explicit_parent_cube_action_generated_judgment_replayed: bool,
    pub canonical_map_cube_candidate_matched: bool,
    pub typed_face_equation_count: u32,
    pub every_face_equation_bound_to_face_map: bool,
    pub frozen_equality_used: bool,
    pub final_by_basis_monotonicity: bool,
    pub full_e4_completeness_used: bool,
    pub independent_verdict: bool,
    pub ordinary_family_credit_issued: bool,
    pub stage_count_issued: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M1PendingEvidence {
    pub subject: String,
    pub obstruction: String,
    pub independent_verdict_withheld: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PerClassE4Evidence {
    pub class: String,
    pub complete: bool,
    pub named_gaps: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E4DevelopmentEvidence {
    pub version: String,
    pub audit_derivation_hash: String,
    pub audit_issued_and_replayed: bool,
    pub current_e1_context_morphism_decomposition_complete: bool,
    pub per_class: Vec<PerClassE4Evidence>,
    pub development_partial: bool,
    pub global_e4_complete: bool,
    pub independent_membership_issuance_enabled: bool,
    pub global_gap: String,
    pub m1_generated: Vec<M1GeneratedEvidence>,
    pub m1_pending: Vec<M1PendingEvidence>,
    pub exact_one_generated_step8_cell_action: bool,
    pub exact_five_pending_membership_cases: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E34ForbiddenOutputs {
    pub independent_verdict_issued: bool,
    pub ordinary_family_token_issued: bool,
    pub historical_stage_count_issued: bool,
    pub revised_count_regression_run: bool,
    pub fq2_evaluated: bool,
    pub agent_a_handoff_issued: bool,
    pub e2b_quotient_closure_executed: bool,
    pub historical_scores_recomputed: bool,
    pub step16_acceptance_computation_run: bool,
    pub closing_inequality_computed: bool,
    pub global_halt_proved: bool,
    pub global_continuation_proved: bool,
}

impl E34ForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.independent_verdict_issued
            && !self.ordinary_family_token_issued
            && !self.historical_stage_count_issued
            && !self.revised_count_regression_run
            && !self.fq2_evaluated
            && !self.agent_a_handoff_issued
            && !self.e2b_quotient_closure_executed
            && !self.historical_scores_recomputed
            && !self.step16_acceptance_computation_run
            && !self.closing_inequality_computed
            && !self.global_halt_proved
            && !self.global_continuation_proved
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct P1FalsifierEvidence {
    pub fp1_triggered: bool,
    pub fp2_evaluated: bool,
    pub fp2_triggered: bool,
    pub fp3_evaluated: bool,
    pub fp3_triggered: bool,
    pub count_blind: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E34PhaseStatus {
    pub phase: String,
    pub status: String,
    pub scoped_token_issued: bool,
    pub global_phase_complete: bool,
    pub blocked_by: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgdaE34Evidence {
    pub source_bound: bool,
    pub safe_without_k_declared: bool,
    pub postulates_absent: bool,
    pub normalization_and_frozen_equality_fragment_declared: bool,
    pub checked_substitution_decomposition_declared: bool,
    pub m1_monotonicity_declared: bool,
    pub arbitrary_view_completeness_inhabitant_issued: bool,
    pub agda_rust_operational_correspondence_proved: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2E34Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E34SourceBinding>,
    pub predecessor: E34PredecessorEvidence,
    pub p1_adoption: P1AdoptionEvidence,
    pub e2a: E2aEvidence,
    pub e3: E3FragmentEvidence,
    pub e4: E4DevelopmentEvidence,
    pub agda: AgdaE34Evidence,
    pub forbidden_outputs: E34ForbiddenOutputs,
    pub falsifiers: P1FalsifierEvidence,
    pub phases: Vec<E34PhaseStatus>,
    pub e2b_executed: bool,
    pub e3_global_complete: bool,
    pub e4_global_complete: bool,
    pub e34_global_complete: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2E34Replay {
    pub valid: bool,
    pub predecessor_replayed: bool,
    pub p1_adopted: bool,
    pub e2a_complete: bool,
    pub e3_fragment_complete: bool,
    pub e3_global_complete: bool,
    pub e4_development_partial: bool,
    pub e4_global_complete: bool,
    pub m1_generated_count: usize,
    pub m1_pending_count: usize,
    pub e2b_executed: bool,
    pub later_phases_unexecuted: bool,
    pub forbidden_outputs_withheld: bool,
    pub global_halt_proved: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Copy)]
struct SourceSpec {
    name: &'static str,
    path: &'static str,
    role: &'static str,
    bytes: &'static [u8],
    expected_length: u64,
    expected_blake3: &'static str,
    semantic_projection_replayed: bool,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SCHEMA2_E34_CERTIFICATE_SCHEMA, domain, value))
        .expect("E-3/E-4 certificate proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn source_manifest() -> Vec<SourceSpec> {
    // These pins were computed after formatting the final E-3/E-4 sources.
    // Any subsequent mutation therefore fails before a theorem projection
    // can be replayed.
    vec![
        source_spec(
            "schema2_v2",
            "docs/schema2_v2.json",
            "exact_e2_predecessor",
            PREDECESSOR_BYTES,
            18_978,
            "blake3:3785da4e01a06f278edbb7671d9021672b016a0659614a8accbb5b7809294718",
            true,
        ),
        source_spec(
            "p1_adjudication",
            "docs/e2_phase_order_adjudication.md",
            "adopted_phase_order_and_m1_authority",
            P1_ADJUDICATION_BYTES,
            3_741,
            "blake3:6f6eb9e071bd05ce172a113da96653ba281d7eab215a23364ee556a0992f246f",
            true,
        ),
        source_spec(
            "e2_quotient_adjudications",
            "docs/e2_quotient_adjudications.md",
            "unchanged_r1_r2_authority",
            E2_QUOTIENT_ADJUDICATION_BYTES,
            6_355,
            "blake3:6078f8e1f7209a7ba1fe082e44ae4dc491ea5e142e806f1c1b518c221d96a45d",
            true,
        ),
        source_spec(
            "agent_e_plan",
            "docs/agent_e_schema2_plan.md",
            "governing_execution_plan",
            AGENT_PLAN_BYTES,
            9_562,
            "blake3:d06d2039f1cd83e2d5509478aede30d2e09e2548f262cea9e0f88abbfe2767db",
            true,
        ),
        source_spec(
            "schema2_open_problem",
            "docs/step_15_completion_open_problem.md",
            "frozen_schema2_specification",
            OPEN_PROBLEM_SPEC_BYTES,
            15_507,
            "blake3:2026c9a017f3723cadcadc70890dcc416ef37ea4f051af06d29ceeffe6c8b544",
            true,
        ),
        source_spec(
            "e2_certificate_source",
            "crates/pen-schema/src/e2_certificate.rs",
            "predecessor_replay_issuer",
            E2_CERTIFICATE_SOURCE_BYTES,
            72_006,
            "blake3:cb0bf146848d5f89c37b00193e15beaa794686a9c2452e9bb9e6ad882e96e94f",
            true,
        ),
        source_spec(
            "e3_normalization_source",
            "crates/pen-schema/src/e3_normalization.rs",
            "e3_fragment_issuer",
            E3_SOURCE_BYTES,
            106_145,
            "blake3:8de05189efeda3ba291bcbf0b46d7c84222a5d7d6590a6bed4d9e3e0a84d72bb",
            true,
        ),
        source_spec(
            "e4_generator_basis_source",
            "crates/pen-schema/src/e4_generator_basis.rs",
            "e4_development_issuer",
            E4_SOURCE_BYTES,
            82_053,
            "blake3:fb19b2ed000cf23dad0b2de4a844cd2244c30613a7885607474ca4c4c7e54877",
            true,
        ),
        source_spec(
            "context_source",
            "crates/pen-schema/src/context.rs",
            "e1_typed_context_dependency",
            CONTEXT_SOURCE_BYTES,
            71_070,
            "blake3:d0de2a49de234034ec60e62e7735658055bc4e9e3abedd5790c7d471de5f1904",
            false,
        ),
        source_spec(
            "grammar_source",
            "crates/pen-schema/src/grammar.rs",
            "e2_typed_schema_dependency",
            GRAMMAR_SOURCE_BYTES,
            69_162,
            "blake3:c86b9620c3d07cdf1512d33db1114eb108d68969c4868acf40cdbc5d67d04c8c",
            false,
        ),
        source_spec(
            "ordinary_source",
            "crates/pen-schema/src/ordinary.rs",
            "e2_ordinary_registry_dependency",
            ORDINARY_SOURCE_BYTES,
            26_148,
            "blake3:54bf0b295a86e349f0bdaf8cd0a27aaa7a5d094e0f6fcbcab7b4aedcd6f8f154",
            false,
        ),
        source_spec(
            "stage1_r1_source",
            "crates/pen-schema/src/stage1_r1.rs",
            "r1_pending_row_dependency",
            STAGE1_SOURCE_BYTES,
            9_627,
            "blake3:677875dd0ecf02a075639f51fabbbac0367cd05d8deeb464601ee2ba97893f86",
            false,
        ),
        source_spec(
            "step8_r2_source",
            "crates/pen-schema/src/step8_r2.rs",
            "m1_step8_subject_dependency",
            STEP8_SOURCE_BYTES,
            11_143,
            "blake3:834be06764a5259269391454feed6ed486685f1c15cb2e9c874866407f277fbe",
            false,
        ),
        source_spec(
            "kernel_equality_source",
            "crates/pen-type/src/equality.rs",
            "kernel_equality_dependency",
            EQUALITY_SOURCE_BYTES,
            4_757,
            "blake3:7c1ed7386c35aa7e5196e044b194f63e61204e431c3837018305977fbcee6c31",
            false,
        ),
        source_spec(
            "kernel_normalize_source",
            "crates/pen-type/src/normalize.rs",
            "kernel_normalization_dependency",
            NORMALIZE_SOURCE_BYTES,
            18_309,
            "blake3:b9ad659cf60175b87090b590ee0850b3635a4968a47fda5bbd3559588d57ba7a",
            false,
        ),
        source_spec(
            "kernel_cubical_source",
            "crates/pen-type/src/cubical.rs",
            "e1_cubical_context_dependency",
            CUBICAL_SOURCE_BYTES,
            170_387,
            "blake3:28b9394dd92513202b9998bd4ea7d41d89699de618ff4a7514d7d903fb95bb46",
            false,
        ),
        source_spec(
            "telescope_source",
            "crates/pen-core/src/telescope.rs",
            "historical_telescope_dependency",
            TELESCOPE_SOURCE_BYTES,
            19_443,
            "blake3:d9831c28c380d1fdc313f757b393e26e81df1505d87ae35291d3eb8f214cf545",
            false,
        ),
        source_spec(
            "elaborator_source",
            "crates/pen-type/src/elaborate.rs",
            "historical_elaboration_dependency",
            ELABORATE_SOURCE_BYTES,
            100_581,
            "blake3:cc1027fa5e53ac1417b2bf01685bbd463cf832fe36c1b37339c360b7fc75c648",
            false,
        ),
        source_spec(
            "typed_boundary_source",
            "crates/pen-type/src/cubical/typed_boundary.rs",
            "step8_boundary_dependency",
            TYPED_BOUNDARY_SOURCE_BYTES,
            199_287,
            "blake3:0d3f746ddb3a3c28441980f3ab9733762a94fe46f380be5d5a174659c07e8274",
            false,
        ),
        source_spec(
            "schema2_agda",
            "agda/Schema2.agda",
            "agda_e1_e2_base",
            SCHEMA2_AGDA_BYTES,
            25_002,
            "blake3:06531792b49eb1bba506247bea96e00c1422ca42dabd8726f206d531833f6a30",
            false,
        ),
        source_spec(
            "schema2_e2_agda",
            "agda/Schema2E2.agda",
            "agda_step8_typing_boundary",
            SCHEMA2_E2_AGDA_BYTES,
            4_824,
            "blake3:125232bed90c447feab3fcb15a83772d567e3b28855c4771c3a017e85d99ba59",
            false,
        ),
        source_spec(
            "schema2_e34_agda",
            "agda/Schema2E3E4.agda",
            "agda_e3_e4_fragment",
            SCHEMA2_E34_AGDA_BYTES,
            34_396,
            "blake3:6204ba46861cd4889c4899c9dfb211e25108bc5496dc37a6602f1a3e5b5e9cff",
            true,
        ),
        source_spec(
            "kernel_bridge_agda",
            "agda/KernelBridge.agda",
            "agda_schema2_transitive_import",
            KERNEL_BRIDGE_AGDA_BYTES,
            11_224,
            "blake3:4c6f0c215837d0b81fbd472d4119357a41c86d20989b5d91575de614c37a2095",
            false,
        ),
    ]
}

const fn source_spec(
    name: &'static str,
    path: &'static str,
    role: &'static str,
    bytes: &'static [u8],
    expected_length: u64,
    expected_blake3: &'static str,
    semantic_projection_replayed: bool,
) -> SourceSpec {
    SourceSpec {
        name,
        path,
        role,
        bytes,
        expected_length,
        expected_blake3,
        semantic_projection_replayed,
    }
}

fn bind_sources() -> Result<Vec<E34SourceBinding>, Schema2E34CertificateError> {
    source_manifest()
        .into_iter()
        .map(|spec| {
            let actual_length = spec.bytes.len() as u64;
            let actual_digest = bytes_hash(spec.bytes);
            if actual_length != spec.expected_length || actual_digest != spec.expected_blake3 {
                return Err(Schema2E34CertificateError::SourceBinding {
                    name: spec.name.to_owned(),
                    expected: format!("{}/{}", spec.expected_length, spec.expected_blake3),
                    observed: format!("{actual_length}/{actual_digest}"),
                });
            }
            Ok(E34SourceBinding {
                name: spec.name.to_owned(),
                path: spec.path.to_owned(),
                role: spec.role.to_owned(),
                byte_length: actual_length,
                blake3: actual_digest,
                semantic_projection_replayed: spec.semantic_projection_replayed,
            })
        })
        .collect()
}

fn utf8<'a>(name: &str, bytes: &'a [u8]) -> Result<&'a str, Schema2E34CertificateError> {
    std::str::from_utf8(bytes).map_err(|_| Schema2E34CertificateError::Utf8(name.to_owned()))
}

fn validate_predecessor()
-> Result<(E34PredecessorEvidence, Schema2E2Certificate), Schema2E34CertificateError> {
    let json = utf8("docs/schema2_v2.json", PREDECESSOR_BYTES)?;
    let replay = replay_schema2_e2_json(json);
    if !replay.valid {
        return Err(Schema2E34CertificateError::Predecessor(
            replay.errors.join("; "),
        ));
    }
    let predecessor: Schema2E2Certificate = serde_json::from_str(json)
        .map_err(|error| Schema2E34CertificateError::Predecessor(error.to_string()))?;
    if predecessor.schema != SCHEMA2_E2_CERTIFICATE_SCHEMA
        || predecessor.e2_complete
        || !predecessor.falsifiers.fq4.triggered
        || predecessor.ordinary_grammar.agent_a_handoff_token_issued
        || predecessor.conclusion_boundary.global_halt_proved
    {
        return Err(Schema2E34CertificateError::Predecessor(
            "predecessor semantic boundary differs from sealed E-2 result".to_owned(),
        ));
    }
    let evidence = E34PredecessorEvidence {
        path: "docs/schema2_v2.json".to_owned(),
        schema: predecessor.schema.clone(),
        result_digest: predecessor.result_digest.clone(),
        exact_source_binding_replayed: true,
        strict_certificate_replay_valid: true,
        predecessor_e2_complete: predecessor.e2_complete,
        predecessor_fq4_triggered: predecessor.falsifiers.fq4.triggered,
        predecessor_handoff_issued: predecessor.ordinary_grammar.agent_a_handoff_token_issued,
        predecessor_halt_claimed: predecessor.conclusion_boundary.global_halt_proved,
    };
    Ok((evidence, predecessor))
}

fn validate_p1_adoption() -> Result<P1AdoptionEvidence, Schema2E34CertificateError> {
    let text = utf8("docs/e2_phase_order_adjudication.md", P1_ADJUDICATION_BYTES)?;
    let normalized = text.replace("\r\n", "\n");
    let adoption = concat!(
        "> I adopt procedural adjudication P1: the E-2a/E-2b phase split with\n",
        "> quotient closure after E-4, and the monotone membership clause M1, as\n",
        "> amendments to the Schema2 execution order. All adopted rules and axioms\n",
        "> are unchanged.\n",
        ">\n",
        "> — Halvor Lande, 20 July, 2026"
    );
    let required = [
        "E-1  →  E-2a (typing)  →  E-3  →  E-4  →  E-2b (quotient closure)  →  E-5 … E-8",
        "Interim \"generated\" verdicts are final by monotonicity.",
        "**\"Independent\" verdicts require the E-4 completeness theorem.**",
        "F-Q2 and all stage",
        "counts remain pinned to E-2b",
        "Amendment goes through a versioned successor; no silent modification.",
    ];
    if !normalized.contains("**Date:** 2026-07-19. **Status:** **ADOPTED** 2026-07-20")
        || !normalized.contains(adoption)
        || required.iter().any(|needle| !normalized.contains(needle))
    {
        return Err(Schema2E34CertificateError::Adoption(
            "exact P1 adoption/order/M1 boundary was not found".to_owned(),
        ));
    }
    Ok(P1AdoptionEvidence {
        adjudication_id: PROCEDURAL_ADJUDICATION_ID.to_owned(),
        membership_rule_id: MONOTONE_MEMBERSHIP_RULE_ID.to_owned(),
        exact_document_bound: true,
        adoption_block_replayed_verbatim: true,
        adopter: "Halvor Lande".to_owned(),
        adoption_date: "2026-07-20".to_owned(),
        adopted_phase_order: vec![
            "E-1".to_owned(),
            "E-2a".to_owned(),
            "E-3".to_owned(),
            "E-4".to_owned(),
            "E-2b".to_owned(),
            "E-5".to_owned(),
            "E-6".to_owned(),
            "E-7".to_owned(),
            "E-8".to_owned(),
        ],
        counts_pinned_exclusively_to_e2b: true,
        generated_verdicts_monotone_and_final: true,
        independent_verdicts_require_complete_e4: true,
        amendment_is_versioned_successor: true,
    })
}

fn validate_plan_and_spec() -> Result<(), Schema2E34CertificateError> {
    let plan = utf8("docs/agent_e_schema2_plan.md", AGENT_PLAN_BYTES)?;
    let spec = utf8(
        "docs/step_15_completion_open_problem.md",
        OPEN_PROBLEM_SPEC_BYTES,
    )?;
    let plan_needles = [
        "**E-3 (normalization and frozen equality).**",
        "**E-4 (naturality generators — the C2 theorem).**",
        "a class that resists stays a **named per-class",
        "gap** — no smoothing (F-E1).",
        "Counts are",
        "graders, exclusively.",
    ];
    let spec_needles = [
        "C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS",
        "### 1.3 Typed normalization",
        "Naturality completeness.",
        "every grammar constructor has a typed realization and normalization rule",
    ];
    if plan_needles.iter().any(|needle| !plan.contains(needle))
        || spec_needles.iter().any(|needle| !spec.contains(needle))
    {
        return Err(Schema2E34CertificateError::PlanSpec(
            "governing E-3/E-4 requirements were not replayed".to_owned(),
        ));
    }
    Ok(())
}

fn validate_agda_boundary() -> Result<AgdaE34Evidence, Schema2E34CertificateError> {
    let source = utf8("agda/Schema2E3E4.agda", SCHEMA2_E34_AGDA_BYTES)?;
    let markers = [
        "normalization-replay-stable",
        "FrozenNormalEqual",
        "substitution-respects-normal-equality",
        "decompose-checked-sub",
        "ParentCubeActionGenerated",
        "step8-cell-action-is-parent-generated",
        "step8-cell-action-M1-generated",
        "m1-generated-in-eventual",
        "generated-verdict-is-final",
        "ArbitraryViewCompleteness",
        "Rust-Agda-Step8-correspondence-gap",
        "open-E3-E4-gaps",
    ];
    if !source.starts_with("{-# OPTIONS --safe --without-K #-}")
        || source
            .lines()
            .any(|line| line.trim_start().starts_with("postulate"))
        || markers.iter().any(|marker| !source.contains(marker))
    {
        return Err(Schema2E34CertificateError::Agda(
            "safe fragment markers or no-postulate boundary failed".to_owned(),
        ));
    }
    Ok(AgdaE34Evidence {
        source_bound: true,
        safe_without_k_declared: true,
        postulates_absent: true,
        normalization_and_frozen_equality_fragment_declared: true,
        checked_substitution_decomposition_declared: true,
        m1_monotonicity_declared: true,
        arbitrary_view_completeness_inhabitant_issued: false,
        agda_rust_operational_correspondence_proved: false,
    })
}

fn e3_evidence() -> Result<E3FragmentEvidence, Schema2E34CertificateError> {
    let audit = issue_e3_fragment_audit()
        .map_err(|error| Schema2E34CertificateError::E3(error.to_string()))?;
    replay_e3_fragment_audit(&audit)
        .map_err(|error| Schema2E34CertificateError::E3(error.to_string()))?;
    let fragment_complete = audit.termination_by_construction()
        && audit.output_type_rechecked_and_preserved()
        && audit.replay_stable()
        && audit.provenance_source_and_normal_form_retained()
        && audit.positive_equality().equal()
        && !audit.negative_equality().equal()
        && audit.weakening_congruence().holds()
        && audit.genuine_substitution_congruence().holds();
    if !fragment_complete
        || !audit.positive_equality().source_provenance_retained()
        || !audit.positive_equality().surviving_provenance_equal()
        || audit.positive_equality().family_reclassification_decided()
        || audit.negative_equality().surviving_provenance_equal()
        || audit.negative_equality().family_reclassification_decided()
        || audit.total_normalization_proved()
        || audit.general_univalent_equality_proved()
        || audit.general_c6_complete()
        || audit.e4_generator_completeness_proved()
        || audit.independent_verdict_issued()
        || audit.ordinary_family_token_issued()
        || audit.stage_count_computed()
        || audit.fq2_evaluated()
        || audit.handoff_issued()
        || !audit
            .gaps()
            .contains(&E3_TOTAL_NORMALIZATION_GAP.to_owned())
        || !audit
            .gaps()
            .contains(&E3_GENERAL_UNIVALENT_EQUALITY_GAP.to_owned())
        || !audit.gaps().contains(&E3_GENERAL_C6_GAP.to_owned())
    {
        return Err(Schema2E34CertificateError::E3(
            "E-3 audit crossed its scoped fragment boundary".to_owned(),
        ));
    }
    Ok(E3FragmentEvidence {
        version: SCHEMA2_E3_FROZEN_FRAGMENT_VERSION.to_owned(),
        audit_derivation_hash: audit.derivation_hash().to_owned(),
        audit_issued_and_replayed: true,
        fragment_milestone_complete: true,
        typed_beta_normalization_replayed: audit.beta_normalization().type_preserved()
            && audit.beta_normalization().normalization_is_stable(),
        positive_and_negative_frozen_equality_replayed: audit.positive_equality().equal()
            && !audit.negative_equality().equal(),
        weakening_congruence_replayed: audit.weakening_congruence().holds(),
        genuine_substitution_congruence_replayed: audit.genuine_substitution_congruence().holds(),
        provenance_retained: audit.provenance_source_and_normal_form_retained(),
        global_e3_complete: false,
        total_schema2_normalization_proved: audit.total_normalization_proved(),
        general_univalent_equality_proved: audit.general_univalent_equality_proved(),
        general_c6_complete: audit.general_c6_complete(),
        gaps: audit.gaps().to_vec(),
    })
}

fn role_name(role: R1LocalRole) -> &'static str {
    match role {
        R1LocalRole::KernelHead => "stage1_carrier_exception.kernel_head",
        R1LocalRole::AdjointMate => "stage1_carrier_exception.adjoint_mate",
        R1LocalRole::SupportAction => "stage1_carrier_exception.support_action",
        R1LocalRole::Coherence => "stage1_carrier_exception.coherence",
    }
}

fn class_name(class: Schema2NaturalityClass) -> &'static str {
    match class {
        Schema2NaturalityClass::Foundation => "foundation",
        Schema2NaturalityClass::Former => "former",
        Schema2NaturalityClass::Map => "map",
        Schema2NaturalityClass::Axiomatic => "axiomatic",
        Schema2NaturalityClass::Modal => "modal",
        Schema2NaturalityClass::HitV2 => "hit_v2",
        Schema2NaturalityClass::Synthesis => "synthesis",
        Schema2NaturalityClass::Unknown => "unknown",
    }
}

fn e4_evidence() -> Result<E4DevelopmentEvidence, Schema2E34CertificateError> {
    let audit = issue_e4_development_audit()
        .map_err(|error| Schema2E34CertificateError::E4(error.to_string()))?;
    replay_e4_development_audit(&audit)
        .map_err(|error| Schema2E34CertificateError::E4(error.to_string()))?;
    let generated = audit
        .m1_generated()
        .iter()
        .map(
            |token| -> Result<M1GeneratedEvidence, Schema2E34CertificateError> {
                let subject = match token.subject() {
                    M1GeneratedSubject::Step8CellAction { .. } => "step8_cell_action",
                };
                let serialized = serde_json::to_value(token)
                    .map_err(|error| Schema2E34CertificateError::E4(error.to_string()))?;
                let rule = serialized
                    .get("rule")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        Schema2E34CertificateError::E4(
                            "M1 token omitted its procedural rule".to_owned(),
                        )
                    })?;
                let membership_rule = serialized
                    .get("membership_rule")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        Schema2E34CertificateError::E4(
                            "M1 token omitted its R2 membership rule".to_owned(),
                        )
                    })?;
                let parent = serialized
                    .pointer("/generating_derivation/parent_cube_action")
                    .ok_or_else(|| {
                        Schema2E34CertificateError::E4(
                            "M1 token omitted ParentCubeActionGenerated evidence".to_owned(),
                        )
                    })?;
                let canonical_map_cube_candidate_matched =
                    parent.get("candidate_row") == parent.get("canonical_generated_row");
                let face_equations = parent
                    .get("face_equations")
                    .and_then(Value::as_array)
                    .ok_or_else(|| {
                        Schema2E34CertificateError::E4(
                            "parent-cube evidence omitted typed face equations".to_owned(),
                        )
                    })?;
                let every_face_equation_bound_to_face_map = face_equations.iter().all(|equation| {
                    equation.get("common_type").is_some()
                        && equation.get("computation_rule").is_some()
                        && equation.get("face_map").is_some()
                        && equation
                            .get("derivation_hash")
                            .and_then(Value::as_str)
                            .is_some()
                });
                let frozen_equality_used = parent
                    .get("frozen_equality_used")
                    .and_then(Value::as_bool)
                    .ok_or_else(|| {
                        Schema2E34CertificateError::E4(
                            "parent-cube evidence omitted frozen-equality boundary".to_owned(),
                        )
                    })?;
                Ok(M1GeneratedEvidence {
                    rule: rule.to_owned(),
                    membership_rule: membership_rule.to_owned(),
                    subject: subject.to_owned(),
                    sub_basis_digest: token.sub_basis_digest().to_owned(),
                    membership_derivation_hash: token.derivation_hash().to_owned(),
                    explicit_parent_cube_action_generated_judgment_replayed: true,
                    canonical_map_cube_candidate_matched,
                    typed_face_equation_count: u32::try_from(face_equations.len()).map_err(
                        |_| {
                            Schema2E34CertificateError::E4(
                                "parent-cube face-equation count overflowed".to_owned(),
                            )
                        },
                    )?,
                    every_face_equation_bound_to_face_map,
                    frozen_equality_used,
                    final_by_basis_monotonicity: token.final_by_basis_monotonicity(),
                    full_e4_completeness_used: token.full_e4_completeness_used(),
                    independent_verdict: false,
                    ordinary_family_credit_issued: false,
                    stage_count_issued: false,
                })
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let pending = audit
        .m1_pending()
        .iter()
        .map(|record| M1PendingEvidence {
            subject: match record.subject() {
                M1PendingSubject::Stage1CarrierException { role } => role_name(role),
                M1PendingSubject::Step8LeftUnitCoherence => "step8_left_unit_coherence",
            }
            .to_owned(),
            obstruction: record.obstruction().to_owned(),
            independent_verdict_withheld: true,
        })
        .collect::<Vec<_>>();
    let expected_pending = [
        "stage1_carrier_exception.kernel_head",
        "stage1_carrier_exception.adjoint_mate",
        "stage1_carrier_exception.support_action",
        "stage1_carrier_exception.coherence",
        "step8_left_unit_coherence",
    ];
    let exact_generated = generated.len() == 1
        && generated[0].rule == MONOTONE_MEMBERSHIP_RULE_ID
        && generated[0].membership_rule == DERIVED_ACTION_MEMBERSHIP_RULE
        && generated[0].subject == "step8_cell_action"
        && generated[0].explicit_parent_cube_action_generated_judgment_replayed
        && generated[0].canonical_map_cube_candidate_matched
        && generated[0].typed_face_equation_count == 6
        && generated[0].every_face_equation_bound_to_face_map
        && !generated[0].frozen_equality_used
        && generated[0].final_by_basis_monotonicity
        && !generated[0].full_e4_completeness_used
        && !generated[0].independent_verdict
        && !generated[0].ordinary_family_credit_issued
        && !generated[0].stage_count_issued;
    let exact_pending = pending.len() == expected_pending.len()
        && pending
            .iter()
            .zip(expected_pending)
            .all(|(observed, expected)| {
                observed.subject == expected && observed.independent_verdict_withheld
            });
    let per_class = audit
        .basis()
        .per_class()
        .iter()
        .map(|entry| PerClassE4Evidence {
            class: class_name(entry.class()).to_owned(),
            complete: entry.complete(),
            named_gaps: entry.named_gaps().to_vec(),
        })
        .collect::<Vec<_>>();
    if !audit
        .basis()
        .current_e1_context_morphism_decomposition_complete()
        || audit.full_e4_completeness()
        || audit.basis().full_schema2_generator_completeness()
        || audit.basis().independent_membership_issuance_enabled()
        || audit.global_gap() != E4_GLOBAL_COMPLETENESS_GAP
        || per_class.len() != Schema2NaturalityClass::ALL.len()
        || per_class.iter().any(|entry| entry.complete)
        || !audit.forbidden_outputs().all_withheld()
        || !exact_generated
        || !exact_pending
    {
        return Err(Schema2E34CertificateError::E4(
            "E-4 audit crossed its partial/M1 boundary".to_owned(),
        ));
    }
    Ok(E4DevelopmentEvidence {
        version: E4_GENERATOR_BASIS_VERSION.to_owned(),
        audit_derivation_hash: audit.derivation_hash().to_owned(),
        audit_issued_and_replayed: true,
        current_e1_context_morphism_decomposition_complete: true,
        per_class,
        development_partial: true,
        global_e4_complete: false,
        independent_membership_issuance_enabled: false,
        global_gap: audit.global_gap().to_owned(),
        m1_generated: generated,
        m1_pending: pending,
        exact_one_generated_step8_cell_action: true,
        exact_five_pending_membership_cases: true,
    })
}

fn phase_ledger() -> Vec<E34PhaseStatus> {
    vec![
        E34PhaseStatus {
            phase: "E-1".to_owned(),
            status: "predecessor_scoped_fragment_complete_global_c1_open".to_owned(),
            scoped_token_issued: false,
            global_phase_complete: false,
            blocked_by: Some("GLOBAL_C1_AND_AGDA_RUST_CORRESPONDENCE_OPEN".to_owned()),
        },
        E34PhaseStatus {
            phase: "E-2a".to_owned(),
            status: "complete_by_adopted_p1_predecessor_typing_milestone".to_owned(),
            scoped_token_issued: true,
            global_phase_complete: true,
            blocked_by: None,
        },
        E34PhaseStatus {
            phase: "E-3".to_owned(),
            status: "fragment_complete_global_e3_false".to_owned(),
            scoped_token_issued: true,
            global_phase_complete: false,
            blocked_by: Some(E3_TOTAL_NORMALIZATION_GAP.to_owned()),
        },
        E34PhaseStatus {
            phase: "E-4".to_owned(),
            status: "development_partial_one_m1_generated_five_memberships_pending".to_owned(),
            scoped_token_issued: true,
            global_phase_complete: false,
            blocked_by: Some(E4_GLOBAL_COMPLETENESS_GAP.to_owned()),
        },
        E34PhaseStatus {
            phase: "E-2b".to_owned(),
            status: "not_executed_requires_complete_e4_membership".to_owned(),
            scoped_token_issued: false,
            global_phase_complete: false,
            blocked_by: Some(E4_GLOBAL_COMPLETENESS_GAP.to_owned()),
        },
        E34PhaseStatus {
            phase: "E-5".to_owned(),
            status: "not_executed_phase_order".to_owned(),
            scoped_token_issued: false,
            global_phase_complete: false,
            blocked_by: Some("E2B_NOT_COMPLETE".to_owned()),
        },
        E34PhaseStatus {
            phase: "E-6".to_owned(),
            status: "not_executed_phase_order".to_owned(),
            scoped_token_issued: false,
            global_phase_complete: false,
            blocked_by: Some("E2B_NOT_COMPLETE".to_owned()),
        },
        E34PhaseStatus {
            phase: "E-7".to_owned(),
            status: "not_executed_phase_order".to_owned(),
            scoped_token_issued: false,
            global_phase_complete: false,
            blocked_by: Some("E2B_NOT_COMPLETE".to_owned()),
        },
        E34PhaseStatus {
            phase: "E-8".to_owned(),
            status: "not_executed_phase_order".to_owned(),
            scoped_token_issued: false,
            global_phase_complete: false,
            blocked_by: Some("E2B_NOT_COMPLETE".to_owned()),
        },
    ]
}

fn certificate_digest(certificate: &Schema2E34Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("successor-certificate", &projection)
}

pub fn build_schema2_e34_certificate() -> Result<Schema2E34Certificate, Schema2E34CertificateError>
{
    let source_bindings = bind_sources()?;
    let (predecessor_evidence, predecessor) = validate_predecessor()?;
    let p1_adoption = validate_p1_adoption()?;
    validate_plan_and_spec()?;
    let agda = validate_agda_boundary()?;
    let e3 = e3_evidence()?;
    let e4 = e4_evidence()?;
    let e2a = E2aEvidence {
        declared_complete_by_adopted_p1: true,
        predecessor_stage1_completed_package_issued: predecessor
            .r1_stage1
            .completed_package_family_issued,
        predecessor_step8_registered_signatures_replayed: predecessor
            .r2_step8
            .all_registered_signatures_replayed,
        predecessor_ordinary_registry_typed: predecessor.ordinary_grammar.finite_registry_replayed
            && predecessor
                .ordinary_grammar
                .every_registered_constructor_depth_at_most_two,
        typing_milestone_complete: true,
        quotient_membership_decided: false,
        count_or_handoff_issued: false,
    };
    if !e2a.predecessor_stage1_completed_package_issued
        || !e2a.predecessor_step8_registered_signatures_replayed
        || !e2a.predecessor_ordinary_registry_typed
    {
        return Err(Schema2E34CertificateError::Invariant(
            "sealed predecessor does not meet P1 E-2a declaration".to_owned(),
        ));
    }
    let forbidden_outputs = E34ForbiddenOutputs {
        independent_verdict_issued: false,
        ordinary_family_token_issued: false,
        historical_stage_count_issued: false,
        revised_count_regression_run: false,
        fq2_evaluated: false,
        agent_a_handoff_issued: false,
        e2b_quotient_closure_executed: false,
        historical_scores_recomputed: false,
        step16_acceptance_computation_run: false,
        closing_inequality_computed: false,
        global_halt_proved: false,
        global_continuation_proved: false,
    };
    if !forbidden_outputs.all_withheld() {
        return Err(Schema2E34CertificateError::Invariant(
            "F-P1 forbidden output was issued".to_owned(),
        ));
    }
    let mut certificate = Schema2E34Certificate {
        schema: SCHEMA2_E34_CERTIFICATE_SCHEMA.to_owned(),
        date: SCHEMA2_E34_CERTIFICATE_DATE.to_owned(),
        source_bindings,
        predecessor: predecessor_evidence,
        p1_adoption,
        e2a,
        e3,
        e4,
        agda,
        forbidden_outputs,
        falsifiers: P1FalsifierEvidence {
            fp1_triggered: false,
            fp2_evaluated: false,
            fp2_triggered: false,
            fp3_evaluated: false,
            fp3_triggered: false,
            count_blind: true,
        },
        phases: phase_ledger(),
        e2b_executed: false,
        e3_global_complete: false,
        e4_global_complete: false,
        e34_global_complete: false,
        outcome: "partial_e3_fragment_and_e4_development_one_m1_generated_five_membership_cases_pending".to_owned(),
        permitted_conclusion: "P1 makes the sealed E-2 typing milestone E-2a complete; the scoped E-3 fragment replays and E-4 proves one final M1-generated Step-8 cell-action verdict, while global E-3/E-4, five membership cases, E-2b, every count/handoff output, E-5 through E-8, and every halt conclusion remain open".to_owned(),
        remaining_obligations: vec![
            "extend typed normalization and frozen univalent equality from the audited expression fragment to every well-typed intended Schema2(W) constructor".to_owned(),
            "prove class-indexed E-4 generator completeness, including normalization naturality and dependent cubical schema actions".to_owned(),
            "after complete E-4, decide the five pending R1/R2 membership cases without revising the final M1-generated Step-8 cell-action verdict".to_owned(),
            "only then execute E-2b, mint justified individual ordinary-family tokens, run the revised Steps 5-8 output regression, evaluate F-Q2, and issue any lawful Agent A handoff".to_owned(),
            "leave E-5 through E-8 and all acceptance or halt computations unexecuted until E-2b completes".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> Schema2E34Replay {
    Schema2E34Replay {
        valid: false,
        predecessor_replayed: false,
        p1_adopted: false,
        e2a_complete: false,
        e3_fragment_complete: false,
        e3_global_complete: false,
        e4_development_partial: false,
        e4_global_complete: false,
        m1_generated_count: 0,
        m1_pending_count: 0,
        e2b_executed: false,
        later_phases_unexecuted: false,
        forbidden_outputs_withheld: false,
        global_halt_proved: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against(
    certificate: &Schema2E34Certificate,
    expected: &Schema2E34Certificate,
) -> Schema2E34Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    let later_phases_unexecuted = certificate.phases.iter().all(|phase| {
        !matches!(phase.phase.as_str(), "E-5" | "E-6" | "E-7" | "E-8")
            || phase.status == "not_executed_phase_order"
    });
    Schema2E34Replay {
        valid: errors.is_empty(),
        predecessor_replayed: certificate.predecessor.strict_certificate_replay_valid,
        p1_adopted: certificate.p1_adoption.adoption_block_replayed_verbatim,
        e2a_complete: certificate.e2a.typing_milestone_complete,
        e3_fragment_complete: certificate.e3.fragment_milestone_complete,
        e3_global_complete: certificate.e3_global_complete,
        e4_development_partial: certificate.e4.development_partial,
        e4_global_complete: certificate.e4_global_complete,
        m1_generated_count: certificate.e4.m1_generated.len(),
        m1_pending_count: certificate.e4.m1_pending.len(),
        e2b_executed: certificate.e2b_executed,
        later_phases_unexecuted,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        global_halt_proved: certificate.forbidden_outputs.global_halt_proved,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_schema2_e34_certificate(certificate: &Schema2E34Certificate) -> Schema2E34Replay {
    match build_schema2_e34_certificate() {
        Ok(expected) => replay_against(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn schema2_e34_json_pretty() -> Result<String, Schema2E34CertificateError> {
    serde_json::to_string_pretty(&build_schema2_e34_certificate()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| Schema2E34CertificateError::Json(error.to_string()))
}

pub fn replay_schema2_e34_json(json: &str) -> Schema2E34Replay {
    let raw: Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => return failed_replay(format!("invalid JSON: {error}")),
    };
    let certificate: Schema2E34Certificate = match serde_json::from_str(json) {
        Ok(certificate) => certificate,
        Err(error) => return failed_replay(format!("certificate shape error: {error}")),
    };
    let typed = serde_json::to_value(&certificate).expect("E-3/E-4 certificate serializes");
    if raw != typed {
        return failed_replay("JSON contains unknown, duplicate, or ignored structure");
    }
    replay_schema2_e34_certificate(&certificate)
}

pub fn emit_schema2_e34_create_new(
    path: &Path,
) -> Result<Schema2E34Replay, Schema2E34CertificateError> {
    let json = schema2_e34_json_pretty()?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Schema2E34CertificateError::Io(error.to_string()))?;
    output
        .write_all(json.as_bytes())
        .and_then(|()| output.flush())
        .map_err(|error| Schema2E34CertificateError::Io(error.to_string()))?;
    let replay = replay_schema2_e34_json(&json);
    if !replay.valid {
        return Err(Schema2E34CertificateError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn redigest(certificate: &mut Schema2E34Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    fn assert_rejected(mut certificate: Schema2E34Certificate) {
        redigest(&mut certificate);
        assert!(!replay_schema2_e34_certificate(&certificate).valid);
    }

    #[test]
    fn exact_source_pin_manifest_matches_embedded_bytes() {
        let mismatches = source_manifest()
            .into_iter()
            .filter_map(|spec| {
                let actual_length = spec.bytes.len() as u64;
                let actual_digest = bytes_hash(spec.bytes);
                (actual_length != spec.expected_length || actual_digest != spec.expected_blake3)
                    .then_some(format!("{}: {actual_length}/{actual_digest}", spec.name))
            })
            .collect::<Vec<_>>();
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }

    #[test]
    fn p1_e3_e4_boundary_replays_exactly() {
        let certificate = build_schema2_e34_certificate().expect("E-3/E-4 certificate");
        let replay = replay_schema2_e34_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(replay.p1_adopted);
        assert!(replay.e2a_complete);
        assert!(replay.e3_fragment_complete);
        assert!(!replay.e3_global_complete);
        assert!(replay.e4_development_partial);
        assert!(!replay.e4_global_complete);
        assert_eq!(replay.m1_generated_count, 1);
        assert_eq!(replay.m1_pending_count, 5);
        assert!(!replay.e2b_executed);
        assert!(replay.later_phases_unexecuted);
        assert!(replay.forbidden_outputs_withheld);
        assert!(!replay.global_halt_proved);
    }

    #[test]
    fn promotion_mutations_fail_even_when_redigested() {
        let certificate = build_schema2_e34_certificate().unwrap();

        let mut invented_independent = certificate.clone();
        invented_independent
            .forbidden_outputs
            .independent_verdict_issued = true;
        assert_rejected(invented_independent);

        let mut global_e3 = certificate.clone();
        global_e3.e3_global_complete = true;
        global_e3.e3.global_e3_complete = true;
        assert_rejected(global_e3);

        let mut global_e4 = certificate.clone();
        global_e4.e4_global_complete = true;
        global_e4.e4.global_e4_complete = true;
        assert_rejected(global_e4);

        let mut e2b = certificate.clone();
        e2b.e2b_executed = true;
        e2b.forbidden_outputs.e2b_quotient_closure_executed = true;
        assert_rejected(e2b);

        let mut count = certificate.clone();
        count.forbidden_outputs.historical_stage_count_issued = true;
        assert_rejected(count);

        let mut fq2 = certificate.clone();
        fq2.forbidden_outputs.fq2_evaluated = true;
        assert_rejected(fq2);

        let mut handoff = certificate.clone();
        handoff.forbidden_outputs.agent_a_handoff_issued = true;
        assert_rejected(handoff);

        let mut halt = certificate;
        halt.forbidden_outputs.global_halt_proved = true;
        assert_rejected(halt);
    }

    #[test]
    fn membership_and_phase_mutations_fail_when_redigested() {
        let certificate = build_schema2_e34_certificate().unwrap();

        let mut lost_generated = certificate.clone();
        lost_generated.e4.m1_generated.clear();
        assert_rejected(lost_generated);

        let mut invented_pending_resolution = certificate.clone();
        invented_pending_resolution.e4.m1_pending.pop();
        assert_rejected(invented_pending_resolution);

        let mut invented_independent_cell = certificate.clone();
        invented_independent_cell.e4.m1_generated[0].independent_verdict = true;
        assert_rejected(invented_independent_cell);

        let mut invented_family_credit = certificate.clone();
        invented_family_credit.e4.m1_generated[0].ordinary_family_credit_issued = true;
        assert_rejected(invented_family_credit);

        let mut broken_parent_judgment = certificate.clone();
        broken_parent_judgment.e4.m1_generated[0].typed_face_equation_count = 5;
        assert_rejected(broken_parent_judgment);

        let mut e5 = certificate.clone();
        let phase = e5
            .phases
            .iter_mut()
            .find(|phase| phase.phase == "E-5")
            .unwrap();
        phase.status = "complete".to_owned();
        phase.global_phase_complete = true;
        assert_rejected(e5);

        let mut altered_predecessor = certificate;
        altered_predecessor.predecessor.result_digest = "blake3:invented".to_owned();
        assert_rejected(altered_predecessor);
    }

    #[test]
    fn strict_json_rejects_unknown_duplicate_and_altered_structure() {
        let json = schema2_e34_json_pretty().unwrap();
        let replay = replay_schema2_e34_json(&json);
        assert!(replay.valid, "{:?}", replay.errors);

        let unknown = json.replacen("{\n", "{\n  \"unknown_field\": true,\n", 1);
        assert!(!replay_schema2_e34_json(&unknown).valid);

        let duplicate = json.replacen(
            "  \"schema\":",
            "  \"schema\": \"duplicate\",\n  \"schema\":",
            1,
        );
        assert!(!replay_schema2_e34_json(&duplicate).valid);

        let altered = json.replace("\"e2b_executed\": false", "\"e2b_executed\": true");
        assert!(!replay_schema2_e34_json(&altered).valid);
    }

    #[test]
    fn create_new_refuses_overwrite() {
        let nonce = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("schema2-e34-{}-{nonce}.json", std::process::id()));
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let replay = emit_schema2_e34_create_new(&path).unwrap();
        assert!(replay.valid);
        assert!(emit_schema2_e34_create_new(&path).is_err());
        std::fs::remove_file(path).unwrap();
    }
}
