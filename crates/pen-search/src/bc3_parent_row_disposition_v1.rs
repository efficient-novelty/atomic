//! BC-3 aggregate parent-row disposition.
//!
//! This certificate audits both the frozen Schema-3/4/5 artifacts and the
//! source-first dynamic-programming quotient that produced Schema 3.  It does
//! not infer semantic nonexistence merely from the absence of candidates
//! in the JSON projection.  It reconstructs the generator's expression-key
//! observation map on a bounded sub-surface and exhibits two distinct typed
//! canonical presentations that the same `ExprJoinKey` identifies.  This
//! refutes the proposed *complete-presentation* factorization route; it does
//! not by itself refute every possible E8 congruence theorem.
//!
//! The lawful BC-3 result is consequently fail closed: every registered parent row
//! receive explicit, row-local named-impossibility dispositions scoped to
//! refinement from the current registered inputs.  These are named proof
//! obstructions, not proofs that a typed refinement cannot exist.  No row is
//! promoted and no desired bridge outcome, score, bar, digest, aggregate
//! multiplicity, or enumeration order selects a disposition.

use crate::candidate_join::{CANDIDATE_JOIN_V4_SCHEMA_VERSION, CandidateJoinV4Certificate};
use crate::candidate_join_v5::CandidateJoinV5Certificate;
use crate::enumerate::{EnumerationContext, LateFamilySurface, enumerate_exprs};
use crate::m3_e7_e8_bridge_v1::{
    M3_E7_E8_BRIDGE_V1_SCHEMA, M3_E7_GAP, M3_E8_GAP, M3_PARENT_ROW_GAP, M3ClaimStatus,
    M3E7E8BridgeV1Certificate, M3Register, M3RunStatus,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_eval::typed_families::{
    CanonicalPresentation, ClauseClosureDisposition, PredecessorClosure, clause_presentation,
    closure_clause_disposition, predecessor_closure,
};
use pen_type::elaborate::{
    ElabError, SealedSignature, elaborate_single_clause, required_clause_ambient,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const BC3_PARENT_ROW_DISPOSITION_V1_SCHEMA: &str =
    "bridge-completion-bc3-parent-row-disposition-v1";
pub const BC3_PARENT_ROW_DISPOSITION_V1_DATE: &str = "2026-07-23";
pub const BC3_CERTIFICATE_NAME: &str = "bc3_parent_row_disposition_v1.json";
pub const BC3_REPORT_NAME: &str = "BC3_PARENT_ROW_DISPOSITION_RESULT.md";
pub const BC3_UNIVERSAL_FAILURE_ID: &str =
    "BC3_SOURCE_DP_EXPR_JOIN_KEY_DOES_NOT_DETERMINE_COMPLETE_TYPED_PRESENTATION";
pub const BC3_ROW_IMPOSSIBILITY_PREFIX: &str =
    "BC3_PARENT_ROW_REFINEMENT_REQUIRES_A_STRONGER_TYPED_DP_QUOTIENT";
pub const FROZEN_M3_RESULT_DIGEST: &str =
    "blake3:a61fe8093ef9ce61a05aebfe859b0723a88a766a712ea6a2312e0f22ca386ba8";
pub const FROZEN_SCHEMA3_INTERNAL_DIGEST: &str =
    "blake3:e4cb89fe83476fb8a0eee43bcc976053bcbbff0bd6db35607d7e8154a3ef858b";
pub const FROZEN_SCHEMA4_INTERNAL_DIGEST: &str =
    "blake3:6a6c3ccfe4731080bfaa944b98f50afe2bde1eb29fdd679b8f62e3a42f72d1e2";
pub const FROZEN_SCHEMA5_INTERNAL_DIGEST: &str =
    "blake3:ee5a0883ef0a81450035d472c855a81d259d6058498983eddc05707b8c3b637b";

const BRIDGE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/bridge_completion_plan.md");
const M3_CERTIFICATE_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e7_e8_bridge_v1.json");
const M3_REPORT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_E7_E8_BRIDGE_V1_RESULT.md");
const SCHEMA3_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v3.json");
const SCHEMA4_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v4.json");
const SCHEMA5_BYTES: &[u8] =
    include_bytes!("../../../docs/ip1_candidate_verdict_join_v5_element_overlay.json");
const SCHEMA3_SOURCE_BYTES: &[u8] = include_bytes!("ip1_candidate_join.rs");
const SCHEMA4_SOURCE_BYTES: &[u8] = include_bytes!("candidate_join.rs");
const SCHEMA5_SOURCE_BYTES: &[u8] = include_bytes!("candidate_join_v5.rs");
const M3_SOURCE_BYTES: &[u8] = include_bytes!("m3_e7_e8_bridge_v1.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("bc3_parent_row_disposition_v1.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc3Register {
    SemanticRegisterAuthority,
    StructuralTestimony,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3RegisteredNumber {
    pub decimal: String,
    pub register: Bc3Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Bc3RegisteredNumber,
    pub blake3: String,
    pub register: Bc3Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3M3BaselineAudit {
    pub frozen_result_digest: String,
    pub internal_digest_recomputed: bool,
    pub proved_condition_count: Bc3RegisteredNumber,
    pub open_condition_count: Bc3RegisteredNumber,
    pub exact_condition_partition_reproduced: bool,
    pub exact_enacted_e5_bi2_projection_reproduced: bool,
    pub named_gap_count: Bc3RegisteredNumber,
    pub exact_named_gap_vector_reproduced: bool,
    pub promotion_count: Bc3RegisteredNumber,
    pub no_promotions_reproduced: bool,
    pub stopped_status_reproduced: bool,
    pub m4_unauthorized_reproduced: bool,
    pub baseline_authenticated: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3GeneratorCounterexample {
    pub position: Bc3RegisteredNumber,
    pub bounded_subsurface_max_nodes: Bc3RegisteredNumber,
    pub left_expression: String,
    pub right_expression: String,
    pub expressions_distinct: bool,
    pub reconstructed_expr_join_keys_equal: bool,
    pub expr_join_key_digest: String,
    pub left_typed_presentation_digest: String,
    pub right_typed_presentation_digest: String,
    pub typed_presentations_distinct: bool,
    pub both_members_of_registered_bounded_expression_surface: bool,
    pub enumeration_order_used_as_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3UniversalAttempt {
    pub attempted: bool,
    pub proposed_statement: String,
    pub source_first_generator_audited: bool,
    pub artifact_erasure_used_as_sole_obstruction: bool,
    pub existing_expr_join_key_determines_complete_typed_presentation: bool,
    pub counterexample_refutes_every_e8_congruence: bool,
    pub universal_refinement_theorem_proved: bool,
    pub universal_failure_id: String,
    pub exact_failure_point: String,
    pub counterexample: Bc3GeneratorCounterexample,
    pub lawful_successor_routes: Vec<String>,
    pub richer_replay_implemented_inside_bc3: bool,
    pub semantics_beyond_adopted_ledger_invented: bool,
    pub failed_attempt_published: bool,
    pub derivation_hash: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc3DispositionKind {
    Refined,
    CoveredByUniversalTheorem,
    NamedImpossibility,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3ParentRowDisposition {
    pub stratum_index: Bc3RegisteredNumber,
    pub row_index: Bc3RegisteredNumber,
    pub aggregate_row_id: String,
    pub kappa: Bc3RegisteredNumber,
    pub schema3_row_digest: String,
    pub schema4_row_derivation_hash: String,
    pub schema5_row_derivation_hash: String,
    pub class: String,
    pub clause_local_extraction_proxy: String,
    pub direct_support: Bc3RegisteredNumber,
    pub amplification_route: String,
    pub aggregate_multiplicity: Bc3RegisteredNumber,
    pub candidate_field_present: bool,
    pub telescope_field_present: bool,
    pub disposition: Bc3DispositionKind,
    pub named_impossibility_id: Option<String>,
    pub named_impossibility_scoped_to_current_inputs: bool,
    pub semantic_nonexistence_proved: bool,
    pub exact_obstruction: String,
    pub aggregate_multiplicity_used_as_selector: bool,
    pub score_bar_hash_or_order_used_as_selector: bool,
    pub row_promoted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3CoverageAudit {
    pub schema3_strata: Bc3RegisteredNumber,
    pub expected_stratum_row_counts: Vec<Bc3RegisteredNumber>,
    pub total_parent_rows: Bc3RegisteredNumber,
    pub refined_rows: Bc3RegisteredNumber,
    pub universal_rows: Bc3RegisteredNumber,
    pub named_impossibility_rows: Bc3RegisteredNumber,
    pub silent_residue_rows: Bc3RegisteredNumber,
    pub every_row_has_unique_disposition: bool,
    pub schema4_exact_row_digest_join: bool,
    pub schema5_exact_row_digest_join: bool,
    pub all_parent_rows_remain_unpromoted: bool,
    pub every_named_impossibility_is_current_input_scoped: bool,
    pub any_semantic_nonexistence_claimed: bool,
    pub count_blind: bool,
    pub register_discipline_valid: bool,
    pub coverage_complete: bool,
    pub derivation_hash: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc3RunStatus {
    StoppedNamedImpossibilities,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3ParentRowDispositionV1Certificate {
    pub schema: String,
    pub date: String,
    pub syntax_identifier_policy: String,
    pub syntax_identifier_register: Bc3Register,
    pub source_bindings: Vec<Bc3SourceBinding>,
    pub m3_baseline: Bc3M3BaselineAudit,
    pub universal_attempt: Bc3UniversalAttempt,
    pub row_dispositions: Vec<Bc3ParentRowDisposition>,
    pub coverage: Bc3CoverageAudit,
    pub failed_universal_attempt_used_to_disposition_rows: bool,
    pub row_dispositions_derived_from_each_rows_registered_payload: bool,
    pub parent_refinement_condition_promoted: bool,
    pub missing_or_duplicate_rejection_condition_promoted: bool,
    pub m3_v2_authorized_by_bc3: bool,
    pub m4_authorized_by_bc3: bool,
    pub status: Bc3RunStatus,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc3Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: Bc3RunStatus,
    pub coverage_complete: bool,
    pub m3_v2_authorized_by_bc3: bool,
    pub m4_authorized_by_bc3: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bc3Error {
    #[error("BC-3 input failure: {0}")]
    Input(String),
    #[error("BC-3 invariant failure: {0}")]
    Invariant(String),
    #[error("BC-3 JSON failure: {0}")]
    Json(String),
    #[error("BC-3 create-new I/O failure: {0}")]
    Io(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Schema3RowProjection {
    kappa: u16,
    class: String,
    clause_local_extraction_proxy: String,
    direct_support: u8,
    amplification_route: String,
    count: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct AuditAmbientFacts {
    ambient: u32,
    used_fields: Vec<u16>,
    marginal_by_assignment: Vec<bool>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct AuditExprJoinKey {
    formation: bool,
    shortfall: u32,
    per_ambient: Vec<AuditAmbientFacts>,
    invalid_named: bool,
    unclassified: bool,
    basic_formation: bool,
    top_path: bool,
    top_modal: bool,
    temporal_like: bool,
    top_suspension: bool,
    has_lib: bool,
    former_root: bool,
    support_mask: u8,
    modal_kind: u8,
    path_basis: u8,
    synthesis_sites: u8,
    app14: bool,
    app15: bool,
    potential_typed_app14: bool,
    potential_typed_app15: bool,
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BC3_PARENT_ROW_DISPOSITION_V1_SCHEMA, domain, value))
        .expect("BC-3 evidence serializes");
    bytes_hash(&bytes)
}

fn metadata_number(value: impl ToString) -> Bc3RegisteredNumber {
    Bc3RegisteredNumber {
        decimal: value.to_string(),
        register: Bc3Register::ArtifactMetadata,
    }
}

fn syntax_number(value: impl ToString) -> Bc3RegisteredNumber {
    Bc3RegisteredNumber {
        decimal: value.to_string(),
        register: Bc3Register::SyntaxIdentifier,
    }
}

fn structural_number(value: impl ToString) -> Bc3RegisteredNumber {
    Bc3RegisteredNumber {
        decimal: value.to_string(),
        register: Bc3Register::StructuralTestimony,
    }
}

fn source_bindings() -> Vec<Bc3SourceBinding> {
    [
        (
            "docs/bridge_completion_plan.md",
            "frozen BC-1/BC-2/BC-3 theorem program and falsifiers",
            BRIDGE_PLAN_BYTES,
        ),
        (
            "docs/schema2_e7_e8_bridge_v1.json",
            "sealed M-3 v1 fail-closed baseline",
            M3_CERTIFICATE_BYTES,
        ),
        (
            "docs/SCHEMA2_E7_E8_BRIDGE_V1_RESULT.md",
            "sealed M-3 v1 human-readable result",
            M3_REPORT_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v3.json",
            "frozen Schema-3 aggregate parent rows",
            SCHEMA3_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v4.json",
            "frozen Schema-4 row-digest bridge",
            SCHEMA4_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v5_element_overlay.json",
            "frozen Schema-5 conservative row overlay",
            SCHEMA5_BYTES,
        ),
        (
            "crates/pen-search/src/ip1_candidate_join.rs",
            "source-first Schema-3 DP generator audited for complete typed-presentation retention",
            SCHEMA3_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/candidate_join.rs",
            "Schema-4 aggregate-row machinery",
            SCHEMA4_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/candidate_join_v5.rs",
            "Schema-5 aggregate-row machinery",
            SCHEMA5_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/m3_e7_e8_bridge_v1.rs",
            "M-3 v1 baseline issuer",
            M3_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/bc3_parent_row_disposition_v1.rs",
            "BC-3 issuer, replay, source-first audit, and mutation gates",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Bc3SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata_number(bytes.len()),
        blake3: bytes_hash(bytes),
        register: Bc3Register::ArtifactMetadata,
    })
    .collect()
}

fn m3_certificate_digest(certificate: &M3E7E8BridgeV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(M3_E7_E8_BRIDGE_V1_SCHEMA, "certificate", &projection))
        .expect("M-3 baseline serializes");
    bytes_hash(&bytes)
}

fn audit_m3_baseline(
    certificate: &M3E7E8BridgeV1Certificate,
) -> Result<Bc3M3BaselineAudit, Bc3Error> {
    let internal_digest_recomputed = m3_certificate_digest(certificate)
        == certificate.result_digest
        && certificate.result_digest == FROZEN_M3_RESULT_DIGEST;
    let proved_ordinals = certificate
        .e8
        .bridge_conditions
        .iter()
        .filter(|condition| condition.proved)
        .map(|condition| condition.ordinal.value)
        .collect::<Vec<_>>();
    let open_ordinals = certificate
        .e8
        .bridge_conditions
        .iter()
        .filter(|condition| !condition.proved)
        .map(|condition| condition.ordinal.value)
        .collect::<Vec<_>>();
    let exact_condition_partition_reproduced = proved_ordinals == [1, 2, 7, 9]
        && open_ordinals == [3, 4, 5, 6, 8]
        && certificate.e8.proved_bridge_condition_count.value == proved_ordinals.len()
        && certificate.e8.bridge_condition_count.value
            == proved_ordinals.len() + open_ordinals.len();
    let e5 = &certificate.enacted_regression.e5_projection;
    let bi2 = &certificate.enacted_regression.bi2_projection;
    let exact_enacted_e5_bi2_projection_reproduced = e5 == bi2
        && e5.exact_a3_inventory.value == 89
        && e5.exact_a3_inventory.register == M3Register::SemanticDemandInstanceAuthority
        && e5.unary_families.value == 17
        && e5.unary_families.register == M3Register::SemanticFamilyAuthority
        && e5.direct_chronological_instances.value == 64
        && e5.direct_chronological_instances.register == M3Register::StructuralTestimony
        && e5.pointwise_chronological_families.value == 8
        && e5.pointwise_chronological_families.register == M3Register::SemanticFamilyAuthority
        && e5.higher_families.value == 0
        && e5.final_window_structural_instances.value == 0
        && e5.historical_structural_registrations.value == 13
        && e5.historical_structural_realizations.value == 13
        && e5.membership_pair_count.value == 89
        && e5.exact_d_partition
        && e5.semantic_o16_empty
        && e5.f1_executed
        && e5.f1_excluded
        && e5.theorem12_full_instance_granularity
        && e5.focus_projection_reproduces_ladder
        && e5.stage_three_wrinkle_reproduced
        && e5.e5_complete
        && certificate.enacted_regression.projections_exactly_equal;
    let gap_ids = certificate
        .open_gaps
        .iter()
        .map(|gap| gap.id.as_str())
        .collect::<Vec<_>>();
    let exact_named_gap_vector_reproduced = gap_ids == [M3_E7_GAP, M3_E8_GAP, M3_PARENT_ROW_GAP]
        && certificate.open_gap_count.value == 3
        && certificate
            .open_gaps
            .iter()
            .all(|gap| gap.keeps_rows_unpromoted)
        && certificate.e7.named_gap == M3_E7_GAP
        && certificate.e7.c2_closed == false
        && certificate
            .e8
            .bridge_conditions
            .iter()
            .filter(|condition| !condition.proved)
            .all(|condition| {
                condition
                    .named_gap
                    .as_deref()
                    .is_some_and(|gap| gap_ids.contains(&gap))
            });
    let no_promotions_reproduced =
        certificate.zero_promotions_made && certificate.promoted_row_count.value == 0;
    let stopped_status_reproduced = certificate.m3_status == M3RunStatus::StoppedNamedGaps
        && !certificate.bridge_claim_issued
        && certificate.e8.bridge_conditions.iter().all(|condition| {
            condition.proved
                || condition
                    .named_gap
                    .as_ref()
                    .is_some_and(|gap| !gap.is_empty())
        })
        && certificate
            .open_gaps
            .iter()
            .all(|gap| gap.keeps_rows_unpromoted)
        && certificate
            .cone
            .claims
            .iter()
            .all(|claim| claim.status == M3ClaimStatus::Certified);
    let m4_unauthorized_reproduced = !certificate.m4_authorized;
    let baseline_authenticated = internal_digest_recomputed
        && exact_condition_partition_reproduced
        && exact_enacted_e5_bi2_projection_reproduced
        && exact_named_gap_vector_reproduced
        && no_promotions_reproduced
        && stopped_status_reproduced
        && m4_unauthorized_reproduced;
    if !baseline_authenticated {
        return Err(Bc3Error::Invariant(
            "F-BC1: frozen M-3 v1 baseline did not reproduce exactly".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "m3-baseline-audit",
        &(
            certificate.result_digest.as_str(),
            &proved_ordinals,
            &open_ordinals,
            e5,
            &gap_ids,
            internal_digest_recomputed,
            exact_condition_partition_reproduced,
            exact_enacted_e5_bi2_projection_reproduced,
            exact_named_gap_vector_reproduced,
            no_promotions_reproduced,
            stopped_status_reproduced,
            m4_unauthorized_reproduced,
        ),
    );
    Ok(Bc3M3BaselineAudit {
        frozen_result_digest: certificate.result_digest.clone(),
        internal_digest_recomputed,
        proved_condition_count: metadata_number(proved_ordinals.len()),
        open_condition_count: metadata_number(open_ordinals.len()),
        exact_condition_partition_reproduced,
        exact_enacted_e5_bi2_projection_reproduced,
        named_gap_count: metadata_number(gap_ids.len()),
        exact_named_gap_vector_reproduced,
        promotion_count: metadata_number(certificate.promoted_row_count.value),
        no_promotions_reproduced,
        stopped_status_reproduced,
        m4_unauthorized_reproduced,
        baseline_authenticated,
        derivation_hash,
    })
}

fn generator_context(position: u32, max_expr_nodes: u8) -> EnumerationContext {
    EnumerationContext {
        library_size: 15,
        scope_size: 2 + position,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes,
        require_former_eliminator_clauses: false,
        require_initial_hit_clauses: false,
        require_truncation_hit_clauses: false,
        require_higher_hit_clauses: false,
        require_sphere_lift_clauses: false,
        require_axiomatic_bundle_clauses: false,
        require_modal_shell_clauses: false,
        require_connection_shell_clauses: false,
        require_curvature_shell_clauses: false,
        require_operator_bundle_clauses: false,
        require_hilbert_functional_clauses: false,
        require_temporal_shell_clauses: false,
        historical_anchor_ref: None,
        late_family_surface: LateFamilySurface::None,
    }
}

fn basic_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Var(_))
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn former_root(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Lam(_) | Expr::App(_, _)
    )
}

fn modal_kind(expr: &Expr) -> u8 {
    match expr {
        Expr::Flat(_) => 1,
        Expr::Sharp(_) => 2,
        Expr::Disc(_) => 4,
        Expr::Shape(_) => 8,
        _ => 0,
    }
}

fn polymorphic_temporal_site(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::App(function, _) if matches!(function.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))))
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Next(inner2) | Expr::Bang(inner2) if matches!(inner2.as_ref(), Expr::Var(_))))
                    && matches!(codomain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                    && matches!(codomain.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
}

fn spatial_temporal_site(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(_))
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner) | Expr::Eventually(inner)
                                | Expr::Bang(inner) | Expr::WhyNot(inner)
                                if matches!(inner.as_ref(), Expr::Var(_))
                        )
            )
    )
}

fn synthesis_site_count(expr: &Expr) -> u8 {
    u8::from(polymorphic_temporal_site(expr)) + u8::from(spatial_temporal_site(expr))
}

fn lib_mask(expr: &Expr) -> u8 {
    let refs = expr.lib_refs();
    u8::from(refs.contains(&14)) | (u8::from(refs.contains(&15)) << 1)
}

fn contains_direct_application(expr: &Expr, dominant: u32) -> bool {
    match expr {
        Expr::App(function, argument) => {
            matches!(function.as_ref(), Expr::Lib(step) if *step == dominant)
                || contains_direct_application(function, dominant)
                || contains_direct_application(argument, dominant)
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            contains_direct_application(domain, dominant)
                || contains_direct_application(codomain, dominant)
        }
        Expr::Id(ty, left, right) => {
            contains_direct_application(ty, dominant)
                || contains_direct_application(left, dominant)
                || contains_direct_application(right, dominant)
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => contains_direct_application(inner, dominant),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn reconstruct_expr_join_key(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    position: u32,
    expr: &Expr,
) -> Option<(AuditExprJoinKey, CanonicalPresentation)> {
    // The counterexample search is deliberately restricted to expressions
    // without library support, so the two private P5-domain flags in the
    // original generator are definitionally false.
    if expr.has_lib_pointer() {
        return None;
    }
    let shortfall = required_clause_ambient(expr, position);
    if shortfall > 2 {
        return None;
    }
    let formation_probe =
        elaborate_single_clause(expr, 2, &vec![ClauseRole::Formation; position as usize], 15);
    let (formation, invalid_named, unclassified) = match formation_probe {
        Ok(elaborated) => (
            elaborated.kernel_role == ClauseRole::Formation,
            false,
            false,
        ),
        Err(ElabError::BareUnivArgument) => (false, true, false),
        Err(_) => (false, false, true),
    };
    if invalid_named || unclassified {
        return None;
    }
    let mut per_ambient = Vec::new();
    let mut terminal_presentation = None;
    for ambient in shortfall..=2 {
        let elaborated = elaborate_single_clause(
            expr,
            ambient,
            &vec![ClauseRole::Formation; position as usize],
            15,
        )
        .ok()?;
        let free_scope = ambient + position;
        let probe = clause_presentation(
            &elaborated.normal_form,
            free_scope,
            &vec![ClauseRole::Formation; position as usize],
            ambient,
        );
        let used_fields = probe
            .renaming
            .forward
            .iter()
            .filter(|(level, _)| *level > ambient)
            .map(|(level, _)| (*level - ambient - 1) as u16)
            .collect::<Vec<_>>();
        let mut marginal_by_assignment = Vec::new();
        for assignment in 0..(1usize << used_fields.len()) {
            let mut roles = vec![ClauseRole::Introduction; position as usize];
            for (bit, field) in used_fields.iter().enumerate() {
                if assignment & (1 << bit) != 0 {
                    roles[usize::from(*field)] = ClauseRole::Formation;
                }
            }
            let presentation =
                clause_presentation(&elaborated.normal_form, free_scope, &roles, ambient);
            marginal_by_assignment.push(
                closure_clause_disposition(signature, closure, &presentation)
                    == ClauseClosureDisposition::Marginal,
            );
        }
        per_ambient.push(AuditAmbientFacts {
            ambient,
            used_fields,
            marginal_by_assignment,
        });
        if ambient == 2 {
            terminal_presentation = Some(probe);
        }
    }
    let support_mask = lib_mask(expr);
    let app14 = contains_direct_application(expr, 14);
    let app15 = contains_direct_application(expr, 15);
    let key = AuditExprJoinKey {
        formation,
        shortfall,
        per_ambient,
        invalid_named,
        unclassified,
        basic_formation: basic_formation(expr),
        top_path: matches!(expr, Expr::PathCon(_)),
        top_modal: expr.is_modal(),
        temporal_like: expr.is_temporal_like(),
        top_suspension: matches!(expr, Expr::Susp(_)),
        has_lib: support_mask != 0,
        former_root: former_root(expr),
        support_mask,
        modal_kind: modal_kind(expr),
        path_basis: match expr {
            Expr::PathCon(dimension) => {
                u8::try_from(1 + dimension.saturating_mul(*dimension)).unwrap_or(u8::MAX)
            }
            _ => 0,
        },
        synthesis_sites: synthesis_site_count(expr),
        app14,
        app15,
        potential_typed_app14: false,
        potential_typed_app15: false,
    };
    Some((key, terminal_presentation?))
}

fn find_generator_counterexample() -> Result<Bc3GeneratorCounterexample, Bc3Error> {
    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature)
        .map_err(|error| Bc3Error::Input(format!("predecessor closure failed: {error}")))?;
    // A small, deterministic sub-surface is enough to refute the proposed
    // complete-presentation factorization statement over the registered
    // bounded expression surface.
    let position = 0_u32;
    let max_nodes = 4_u8;
    let expressions = enumerate_exprs(generator_context(position, max_nodes));
    let mut members_by_key: BTreeMap<AuditExprJoinKey, Vec<(String, Expr, CanonicalPresentation)>> =
        BTreeMap::new();
    for expr in expressions {
        let Some((key, presentation)) =
            reconstruct_expr_join_key(&signature, &closure, position, &expr)
        else {
            continue;
        };
        members_by_key
            .entry(key)
            .or_default()
            .push((format!("{expr:?}"), expr, presentation));
    }
    // Witness choice is syntax-lexicographic after the complete bounded scan;
    // it is independent of enumerator order and is not a bridge selector.
    for (key, mut members) in members_by_key {
        members.sort_by(|left, right| left.0.cmp(&right.0));
        for left_index in 0..members.len() {
            for right_index in left_index + 1..members.len() {
                let (left_expression, left_expr, left_presentation) = &members[left_index];
                let (right_expression, right_expr, right_presentation) = &members[right_index];
                if left_expr == right_expr || left_presentation == right_presentation {
                    continue;
                }
                let key_digest = tagged_hash("source-expr-join-key", &key);
                let left_typed_presentation_digest =
                    tagged_hash("typed-presentation", left_presentation);
                let right_typed_presentation_digest =
                    tagged_hash("typed-presentation", right_presentation);
                let derivation_hash = tagged_hash(
                    "generator-counterexample",
                    &(
                        position,
                        max_nodes,
                        left_expression,
                        right_expression,
                        &key_digest,
                        &left_typed_presentation_digest,
                        &right_typed_presentation_digest,
                    ),
                );
                return Ok(Bc3GeneratorCounterexample {
                    position: syntax_number(position),
                    bounded_subsurface_max_nodes: metadata_number(max_nodes),
                    left_expression: left_expression.clone(),
                    right_expression: right_expression.clone(),
                    expressions_distinct: true,
                    reconstructed_expr_join_keys_equal: true,
                    expr_join_key_digest: key_digest,
                    left_typed_presentation_digest,
                    right_typed_presentation_digest,
                    typed_presentations_distinct: true,
                    both_members_of_registered_bounded_expression_surface: max_nodes <= 6,
                    enumeration_order_used_as_selector: false,
                    derivation_hash,
                });
            }
        }
    }
    Err(Bc3Error::Invariant(
        "source-first universal audit found no complete-presentation counterexample on the registered bounded sub-surface; BC-3 may not publish failure of that route"
            .to_owned(),
    ))
}

fn universal_attempt() -> Result<Bc3UniversalAttempt, Bc3Error> {
    let counterexample = find_generator_counterexample()?;
    let attempted = true;
    let proposed_statement = "The source generator's ExprJoinKey completely determines the typed canonical presentation of every expression on the registered bounded expression surface, and therefore supplies a sufficient universal factorization route for refining every Schema-3 parent row without enumerating its candidate fiber.".to_owned();
    let source_first_generator_audited = true;
    let artifact_erasure_used_as_sole_obstruction = false;
    let existing_expr_join_key_determines_complete_typed_presentation = false;
    let counterexample_refutes_every_e8_congruence = false;
    let universal_refinement_theorem_proved = false;
    let universal_failure_id = BC3_UNIVERSAL_FAILURE_ID.to_owned();
    let exact_failure_point = format!(
        "At the registered source position, `{}` and `{}` reconstruct to the same ExprJoinKey observation, but their typed canonical-presentation digests differ. This refutes the proposed complete-presentation factorization route. It does not establish that the pair has different E8 outcomes, does not refute every weaker E8 congruence, and does not disposition any parent row by itself.",
        counterexample.left_expression, counterexample.right_expression,
    );
    let lawful_successor_routes = vec![
        "Version and adjudicate a stronger source DP key carrying the complete typed canonical presentation, natural-family/orbit extraction, marginality, and act-local provenance; then prove its exact E8 factorization property before quotienting.".to_owned(),
        "Materialize and type-check every member of every parent-row candidate fiber. BC-3 does not pretend to have performed that construction.".to_owned(),
    ];
    let richer_replay_implemented_inside_bc3 = false;
    let semantics_beyond_adopted_ledger_invented = false;
    let failed_attempt_published = true;
    let derivation_hash = tagged_hash(
        "universal-attempt",
        &(
            attempted,
            &proposed_statement,
            source_first_generator_audited,
            artifact_erasure_used_as_sole_obstruction,
            existing_expr_join_key_determines_complete_typed_presentation,
            counterexample_refutes_every_e8_congruence,
            universal_refinement_theorem_proved,
            &universal_failure_id,
            &exact_failure_point,
            &counterexample,
            &lawful_successor_routes,
            richer_replay_implemented_inside_bc3,
            semantics_beyond_adopted_ledger_invented,
            failed_attempt_published,
        ),
    );
    Ok(Bc3UniversalAttempt {
        attempted,
        proposed_statement,
        source_first_generator_audited,
        artifact_erasure_used_as_sole_obstruction,
        existing_expr_join_key_determines_complete_typed_presentation,
        counterexample_refutes_every_e8_congruence,
        universal_refinement_theorem_proved,
        universal_failure_id,
        exact_failure_point,
        counterexample,
        lawful_successor_routes,
        richer_replay_implemented_inside_bc3,
        semantics_beyond_adopted_ledger_invented,
        failed_attempt_published,
        derivation_hash,
    })
}

fn schema3_row_digest(
    stratum_index: usize,
    row_index: usize,
    row: &Schema3RowProjection,
) -> String {
    let bytes = serde_json::to_vec(&(
        CANDIDATE_JOIN_V4_SCHEMA_VERSION,
        "schema3-projected-row",
        &(stratum_index, row_index, row),
    ))
    .expect("Schema-3 projection serializes");
    bytes_hash(&bytes)
}

fn parse_schema3_rows() -> Result<Vec<Vec<(Schema3RowProjection, bool, bool)>>, Bc3Error> {
    let archive: Value =
        serde_json::from_slice(SCHEMA3_BYTES).map_err(|error| Bc3Error::Json(error.to_string()))?;
    if archive.get("digest").and_then(Value::as_str) != Some(FROZEN_SCHEMA3_INTERNAL_DIGEST) {
        return Err(Bc3Error::Invariant(
            "frozen Schema-3 internal digest changed".to_owned(),
        ));
    }
    let strata = archive
        .get("strata")
        .and_then(Value::as_array)
        .ok_or_else(|| Bc3Error::Input("Schema-3 archive lacks strata".to_owned()))?;
    let expected_counts = [63_usize, 72, 78];
    if strata.len() != expected_counts.len() {
        return Err(Bc3Error::Invariant(format!(
            "Schema-3 expected three strata, found {}",
            strata.len()
        )));
    }
    let mut out = Vec::new();
    for (stratum_index, (stratum, expected_count)) in strata.iter().zip(expected_counts).enumerate()
    {
        let rows = stratum
            .get("rows")
            .and_then(Value::as_array)
            .ok_or_else(|| Bc3Error::Input("Schema-3 stratum lacks rows".to_owned()))?;
        if rows.len() != expected_count {
            return Err(Bc3Error::Invariant(format!(
                "Schema-3 stratum {stratum_index} row count changed"
            )));
        }
        let mut projected = Vec::new();
        for row in rows {
            let projection = Schema3RowProjection {
                kappa: u16::try_from(
                    row.get("kappa")
                        .and_then(Value::as_u64)
                        .ok_or_else(|| Bc3Error::Input("row lacks kappa".to_owned()))?,
                )
                .map_err(|error| Bc3Error::Input(error.to_string()))?,
                class: row
                    .get("class")
                    .and_then(Value::as_str)
                    .ok_or_else(|| Bc3Error::Input("row lacks class".to_owned()))?
                    .to_owned(),
                clause_local_extraction_proxy: row
                    .get("clause_local_extraction_proxy")
                    .and_then(Value::as_str)
                    .ok_or_else(|| Bc3Error::Input("row lacks extraction proxy".to_owned()))?
                    .to_owned(),
                direct_support: u8::try_from(
                    row.get("direct_support")
                        .and_then(Value::as_u64)
                        .ok_or_else(|| Bc3Error::Input("row lacks direct support".to_owned()))?,
                )
                .map_err(|error| Bc3Error::Input(error.to_string()))?,
                amplification_route: row
                    .get("amplification_route")
                    .and_then(Value::as_str)
                    .ok_or_else(|| Bc3Error::Input("row lacks amplification route".to_owned()))?
                    .to_owned(),
                count: row
                    .get("count")
                    .and_then(Value::as_str)
                    .ok_or_else(|| Bc3Error::Input("row lacks aggregate multiplicity".to_owned()))?
                    .to_owned(),
            };
            let multiplicity = projection
                .count
                .parse::<u128>()
                .map_err(|error| Bc3Error::Input(error.to_string()))?;
            if multiplicity <= 1 {
                return Err(Bc3Error::Invariant(
                    "Schema-3 row is no longer an aggregate bucket".to_owned(),
                ));
            }
            projected.push((
                projection,
                row.get("candidate").is_some(),
                row.get("telescope").is_some(),
            ));
        }
        out.push(projected);
    }
    Ok(out)
}

fn build_dispositions(
    schema4: &CandidateJoinV4Certificate,
    schema5: &CandidateJoinV5Certificate,
) -> Result<(Vec<Bc3ParentRowDisposition>, Bc3CoverageAudit), Bc3Error> {
    let schema3 = parse_schema3_rows()?;
    if schema4.strata.len() != schema3.len() || schema5.strata.len() != schema3.len() {
        return Err(Bc3Error::Invariant(
            "Schema-3/4/5 stratum counts differ".to_owned(),
        ));
    }
    let mut dispositions = Vec::new();
    let mut schema4_exact_row_digest_join = true;
    let mut schema5_exact_row_digest_join = true;
    for (stratum_index, source_rows) in schema3.iter().enumerate() {
        let schema4_stratum = &schema4.strata[stratum_index];
        let schema5_stratum = &schema5.strata[stratum_index];
        if schema4_stratum.rows.len() != source_rows.len()
            || schema5_stratum.rows.len() != source_rows.len()
        {
            return Err(Bc3Error::Invariant(format!(
                "Schema-3/4/5 row counts differ in stratum {stratum_index}"
            )));
        }
        for (row_index, (source, candidate_present, telescope_present)) in
            source_rows.iter().enumerate()
        {
            let schema4_row = &schema4_stratum.rows[row_index];
            let schema5_row = &schema5_stratum.rows[row_index];
            let expected_row_digest = schema3_row_digest(stratum_index, row_index, source);
            schema4_exact_row_digest_join &= schema4_row.schema3_row_digest == expected_row_digest
                && usize::from(schema4_row.stratum_index) == stratum_index
                && usize::from(schema4_row.row_index) == row_index
                && schema4_row.kappa == source.kappa
                && schema4_row.class == source.class
                && schema4_row.clause_local_extraction_proxy
                    == source.clause_local_extraction_proxy
                && schema4_row.direct_support == source.direct_support
                && schema4_row.amplification_route == source.amplification_route
                && schema4_row.count == source.count;
            schema5_exact_row_digest_join &= schema5_row.schema3_row_digest == expected_row_digest
                && schema5_row.schema4_row_derivation_hash == schema4_row.derivation_hash
                && usize::from(schema5_row.stratum_index) == stratum_index
                && usize::from(schema5_row.row_index) == row_index;
            if *candidate_present || *telescope_present {
                return Err(Bc3Error::Invariant(format!(
                    "Schema-3 row {stratum_index}/{row_index} unexpectedly contains candidate content"
                )));
            }
            let disposition = Bc3DispositionKind::NamedImpossibility;
            let aggregate_row_id =
                format!("schema3:S{stratum_index}/R{row_index}:{expected_row_digest}");
            let named_impossibility_id = Some(format!(
                "{BC3_ROW_IMPOSSIBILITY_PREFIX}:S{stratum_index}:R{row_index}"
            ));
            let named_impossibility_scoped_to_current_inputs = true;
            let semantic_nonexistence_proved = false;
            let exact_obstruction = format!(
                "{aggregate_row_id} has neither a candidate payload nor a telescope payload in the frozen Schema-3 row; Schema 4 and Schema 5 add only exact comparator digests and obligation statuses. No row-local candidate fiber was materialized and no successful universal factorization theorem is registered. Consequently a typed child-refinement proof is not constructible from this row's current registered inputs. This is a named proof obstruction, not a proof that a typed refinement cannot exist."
            );
            let aggregate_multiplicity_used_as_selector = false;
            let score_bar_hash_or_order_used_as_selector = false;
            let row_promoted = false;
            let derivation_hash = tagged_hash(
                "parent-row-disposition",
                &(
                    stratum_index,
                    row_index,
                    aggregate_row_id.as_str(),
                    source,
                    schema4_row.derivation_hash.as_str(),
                    schema5_row.derivation_hash.as_str(),
                    *candidate_present,
                    *telescope_present,
                    &disposition,
                    &named_impossibility_id,
                    named_impossibility_scoped_to_current_inputs,
                    semantic_nonexistence_proved,
                    &exact_obstruction,
                    aggregate_multiplicity_used_as_selector,
                    score_bar_hash_or_order_used_as_selector,
                    row_promoted,
                ),
            );
            dispositions.push(Bc3ParentRowDisposition {
                stratum_index: syntax_number(stratum_index),
                row_index: syntax_number(row_index),
                aggregate_row_id,
                kappa: structural_number(source.kappa),
                schema3_row_digest: expected_row_digest,
                schema4_row_derivation_hash: schema4_row.derivation_hash.clone(),
                schema5_row_derivation_hash: schema5_row.derivation_hash.clone(),
                class: source.class.clone(),
                clause_local_extraction_proxy: source.clause_local_extraction_proxy.clone(),
                direct_support: structural_number(source.direct_support),
                amplification_route: source.amplification_route.clone(),
                aggregate_multiplicity: structural_number(&source.count),
                candidate_field_present: *candidate_present,
                telescope_field_present: *telescope_present,
                disposition,
                named_impossibility_id,
                named_impossibility_scoped_to_current_inputs,
                semantic_nonexistence_proved,
                exact_obstruction,
                aggregate_multiplicity_used_as_selector,
                score_bar_hash_or_order_used_as_selector,
                row_promoted,
                derivation_hash,
            });
        }
    }
    if !schema4_exact_row_digest_join || !schema5_exact_row_digest_join {
        return Err(Bc3Error::Invariant(
            "Schema-3/4/5 exact row-digest join failed".to_owned(),
        ));
    }
    let refined = dispositions
        .iter()
        .filter(|row| row.disposition == Bc3DispositionKind::Refined)
        .count();
    let universal = dispositions
        .iter()
        .filter(|row| row.disposition == Bc3DispositionKind::CoveredByUniversalTheorem)
        .count();
    let named = dispositions
        .iter()
        .filter(|row| row.disposition == Bc3DispositionKind::NamedImpossibility)
        .count();
    let silent = dispositions
        .iter()
        .filter(|row| {
            row.disposition == Bc3DispositionKind::NamedImpossibility
                && row.named_impossibility_id.is_none()
        })
        .count();
    let every_row_has_unique_disposition = dispositions.len() == 213
        && refined + universal + named == dispositions.len()
        && dispositions.iter().all(|row| match row.disposition {
            Bc3DispositionKind::NamedImpossibility => {
                row.named_impossibility_id.is_some()
                    && row.named_impossibility_scoped_to_current_inputs
                    && !row.semantic_nonexistence_proved
                    && !row.exact_obstruction.is_empty()
            }
            Bc3DispositionKind::Refined | Bc3DispositionKind::CoveredByUniversalTheorem => {
                row.named_impossibility_id.is_none()
            }
        });
    let all_parent_rows_remain_unpromoted = dispositions.iter().all(|row| !row.row_promoted)
        && schema4.completeness.no_row_promoted_across_open_gap
        && schema5.completeness.no_row_promoted;
    let every_named_impossibility_is_current_input_scoped = dispositions.iter().all(|row| {
        row.disposition != Bc3DispositionKind::NamedImpossibility
            || row.named_impossibility_scoped_to_current_inputs
    });
    let any_semantic_nonexistence_claimed = dispositions
        .iter()
        .any(|row| row.semantic_nonexistence_proved);
    let count_blind = dispositions
        .iter()
        .all(|row| !row.aggregate_multiplicity_used_as_selector)
        && dispositions
            .iter()
            .all(|row| !row.score_bar_hash_or_order_used_as_selector);
    let register_discipline_valid = dispositions.iter().all(|row| {
        row.stratum_index.register == Bc3Register::SyntaxIdentifier
            && row.row_index.register == Bc3Register::SyntaxIdentifier
            && row.kappa.register == Bc3Register::StructuralTestimony
            && row.direct_support.register == Bc3Register::StructuralTestimony
            && row.aggregate_multiplicity.register == Bc3Register::StructuralTestimony
    });
    let coverage_complete = every_row_has_unique_disposition
        && silent == 0
        && schema4_exact_row_digest_join
        && schema5_exact_row_digest_join
        && all_parent_rows_remain_unpromoted
        && every_named_impossibility_is_current_input_scoped
        && !any_semantic_nonexistence_claimed
        && count_blind
        && register_discipline_valid;
    if !coverage_complete {
        return Err(Bc3Error::Invariant(
            "F-BC2/F-BC3: BC-3 row coverage or register discipline failed".to_owned(),
        ));
    }
    let expected_stratum_row_counts = vec![
        metadata_number(63),
        metadata_number(72),
        metadata_number(78),
    ];
    let derivation_hash = tagged_hash(
        "coverage-audit",
        &(
            &expected_stratum_row_counts,
            dispositions.len(),
            refined,
            universal,
            named,
            silent,
            every_row_has_unique_disposition,
            schema4_exact_row_digest_join,
            schema5_exact_row_digest_join,
            all_parent_rows_remain_unpromoted,
            every_named_impossibility_is_current_input_scoped,
            any_semantic_nonexistence_claimed,
            count_blind,
            register_discipline_valid,
            coverage_complete,
            dispositions
                .iter()
                .map(|row| row.derivation_hash.as_str())
                .collect::<Vec<_>>(),
        ),
    );
    let coverage = Bc3CoverageAudit {
        schema3_strata: metadata_number(3),
        expected_stratum_row_counts,
        total_parent_rows: metadata_number(dispositions.len()),
        refined_rows: metadata_number(refined),
        universal_rows: metadata_number(universal),
        named_impossibility_rows: metadata_number(named),
        silent_residue_rows: metadata_number(silent),
        every_row_has_unique_disposition,
        schema4_exact_row_digest_join,
        schema5_exact_row_digest_join,
        all_parent_rows_remain_unpromoted,
        every_named_impossibility_is_current_input_scoped,
        any_semantic_nonexistence_claimed,
        count_blind,
        register_discipline_valid,
        coverage_complete,
        derivation_hash,
    };
    Ok((dispositions, coverage))
}

fn certificate_digest(certificate: &Bc3ParentRowDispositionV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn build_certificate() -> Result<Bc3ParentRowDispositionV1Certificate, Bc3Error> {
    let m3: M3E7E8BridgeV1Certificate = serde_json::from_slice(M3_CERTIFICATE_BYTES)
        .map_err(|error| Bc3Error::Json(error.to_string()))?;
    let schema4: CandidateJoinV4Certificate =
        serde_json::from_slice(SCHEMA4_BYTES).map_err(|error| Bc3Error::Json(error.to_string()))?;
    let schema5: CandidateJoinV5Certificate =
        serde_json::from_slice(SCHEMA5_BYTES).map_err(|error| Bc3Error::Json(error.to_string()))?;
    if schema4.digest != FROZEN_SCHEMA4_INTERNAL_DIGEST
        || schema5.digest != FROZEN_SCHEMA5_INTERNAL_DIGEST
    {
        return Err(Bc3Error::Invariant(
            "frozen Schema-4 or Schema-5 internal digest changed".to_owned(),
        ));
    }
    let m3_baseline = audit_m3_baseline(&m3)?;
    let universal_attempt = universal_attempt()?;
    let (row_dispositions, coverage) = build_dispositions(&schema4, &schema5)?;
    let parent_refinement_condition_promoted = false;
    let missing_or_duplicate_rejection_condition_promoted = false;
    let m3_v2_authorized_by_bc3 = false;
    let m4_authorized_by_bc3 = false;
    let status = Bc3RunStatus::StoppedNamedImpossibilities;
    let mut certificate = Bc3ParentRowDispositionV1Certificate {
        schema: BC3_PARENT_ROW_DISPOSITION_V1_SCHEMA.to_owned(),
        date: BC3_PARENT_ROW_DISPOSITION_V1_DATE.to_owned(),
        syntax_identifier_policy: "Numeric substrings in dates, schema names, theorem IDs, condition labels, stratum labels, row labels, and expression-constructor indices are identifiers rather than quantitative evidence. Every published quantity is wrapped with its register.".to_owned(),
        syntax_identifier_register: Bc3Register::SyntaxIdentifier,
        source_bindings: source_bindings(),
        m3_baseline,
        universal_attempt,
        row_dispositions,
        coverage,
        failed_universal_attempt_used_to_disposition_rows: false,
        row_dispositions_derived_from_each_rows_registered_payload: true,
        parent_refinement_condition_promoted,
        missing_or_duplicate_rejection_condition_promoted,
        m3_v2_authorized_by_bc3,
        m4_authorized_by_bc3,
        status,
        mutation_falsifiers: vec![
            "F-BC1: change any frozen M-3 baseline condition, enacted projection, named gap, status, or digest and replay must fail".to_owned(),
            "F-BC2: delete, duplicate, reorder, or leave undisposed any parent row and replay must fail".to_owned(),
            "F-BC3: retag any published quantity or use structural multiplicity in a law-level derivation and replay must fail".to_owned(),
            "F-BC4: promote any row or bridge condition while a named impossibility remains and replay must fail".to_owned(),
            "F-BC5: erase or forge the source-first complete-presentation counterexample, use it as a parent-row disposition, or claim the failed universal route and replay must fail".to_owned(),
            "F-BC6: introduce any item excluded by the frozen scope rule and replay must fail".to_owned(),
            "F-BC7: alter the frozen wrapped-surface scope or issue a cone/branch claim from BC-3 and replay must fail".to_owned(),
            "add any unknown JSON field and replay must fail".to_owned(),
        ],
        permitted_conclusion: "The frozen M-3 baseline is authenticated and every Schema-3 aggregate parent row has an explicit disposition. The attempted complete-presentation factorization route fails at a source-level counterexample. Independently, each row lacks candidate and telescope payloads, no row-local fiber was materialized, and no successful universal refinement theorem is registered. Therefore every row is a named impossibility of refinement from the current registered inputs, not a proof of semantic nonexistence. No row is promoted, and BC-3 does not authorize either successor gate.".to_owned(),
        required_successor_action: "Version and adjudicate a stronger source-first typed DP quotient that retains canonical presentations, natural-family and demand-orbit extraction, marginality, and act-local provenance, then prove its exact E8 factorization property and rerun BC-3 create-new. Alternatively supply an explicit bounded exhaustive candidate-fiber construction. The current bridge remains fail closed.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<Bc3ParentRowDispositionV1Certificate, String>> =
    OnceLock::new();

fn expected_certificate() -> Result<&'static Bc3ParentRowDispositionV1Certificate, Bc3Error> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Bc3Error::Input(error.clone())),
    }
}

pub fn issue_bc3_parent_row_disposition_v1()
-> Result<Bc3ParentRowDispositionV1Certificate, Bc3Error> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> Bc3Replay {
    Bc3Replay {
        valid: false,
        errors: vec![error.into()],
        status: Bc3RunStatus::StoppedNamedImpossibilities,
        coverage_complete: false,
        m3_v2_authorized_by_bc3: false,
        m4_authorized_by_bc3: false,
    }
}

pub fn replay_bc3_parent_row_disposition_v1(
    claimed: &Bc3ParentRowDispositionV1Certificate,
) -> Bc3Replay {
    let expected = match expected_certificate() {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("BC-3 certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("BC-3 source bindings drifted".to_owned());
    }
    if claimed != expected {
        errors.push("BC-3 certificate differs from deterministic reissuance".to_owned());
    }
    Bc3Replay {
        valid: errors.is_empty(),
        errors,
        status: claimed.status,
        coverage_complete: claimed.coverage.coverage_complete,
        m3_v2_authorized_by_bc3: claimed.m3_v2_authorized_by_bc3,
        m4_authorized_by_bc3: claimed.m4_authorized_by_bc3,
    }
}

pub fn replay_bc3_parent_row_disposition_v1_json(json: &str) -> Bc3Replay {
    let raw = match serde_json::from_str::<Value>(json) {
        Ok(raw) => raw,
        Err(error) => return invalid_replay(format!("invalid BC-3 JSON: {error}")),
    };
    let certificate = match serde_json::from_str::<Bc3ParentRowDispositionV1Certificate>(json) {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(format!("invalid BC-3 JSON: {error}")),
    };
    let projected = match serde_json::to_value(&certificate) {
        Ok(projected) => projected,
        Err(error) => return invalid_replay(format!("invalid BC-3 projection: {error}")),
    };
    if raw != projected {
        return invalid_replay("unknown or ignored field changes BC-3 JSON projection");
    }
    replay_bc3_parent_row_disposition_v1(&certificate)
}

fn register_label(register: Bc3Register) -> &'static str {
    match register {
        Bc3Register::SemanticRegisterAuthority => "semantic_register_authority",
        Bc3Register::StructuralTestimony => "structural_testimony",
        Bc3Register::ArtifactMetadata => "artifact_metadata",
        Bc3Register::SyntaxIdentifier => "syntax_identifier",
    }
}

fn render_number(number: &Bc3RegisteredNumber) -> String {
    format!(
        "{} [register: `{}`]",
        number.decimal,
        register_label(number.register)
    )
}

pub fn render_bc3_parent_row_disposition_v1(
    certificate: &Bc3ParentRowDispositionV1Certificate,
) -> String {
    let mut out = String::new();
    out.push_str("# BC-3 aggregate parent-row disposition result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Status:** `stopped_named_impossibilities`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("BC-3 authenticates the exact M-3 v1 baseline, but it does not close the aggregate-row theorem. A sufficient complete-presentation factorization route was attempted against the source generator itself and failed before any row was promoted. The counterexample does not claim to refute every weaker E8 congruence.\n\n");
    out.push_str("## Frozen baseline\n\n");
    out.push_str(&format!(
        "- M-3 v1 digest: `{}`.\n- Internal digest, the exact proved/open condition partition, enacted E-5/BI-2 projection, named-gap vector, registered promotion count, stopped status, and the closed M-4 gate all reproduce: **{}**.\n\n",
        certificate.m3_baseline.frozen_result_digest,
        certificate.m3_baseline.baseline_authenticated,
    ));
    out.push_str("## Published universal attempt\n\n");
    out.push_str(&format!(
        "Proposed theorem: {}  \nResult: **not proved**. Failure: `{}`.\n\n{}\n\n",
        certificate.universal_attempt.proposed_statement,
        certificate.universal_attempt.universal_failure_id,
        certificate.universal_attempt.exact_failure_point,
    ));
    let witness = &certificate.universal_attempt.counterexample;
    out.push_str(&format!(
        "Counterexample on position {} and bounded source sub-surface {}:\n\n- left: `{}`\n- right: `{}`\n- common reconstructed key: `{}`\n- left typed presentation: `{}`\n- right typed presentation: `{}`\n\n",
        render_number(&witness.position),
        render_number(&witness.bounded_subsurface_max_nodes),
        witness.left_expression,
        witness.right_expression,
        witness.expr_join_key_digest,
        witness.left_typed_presentation_digest,
        witness.right_typed_presentation_digest,
    ));
    out.push_str("The existing DP key therefore does not determine the complete typed canonical presentation. This defeats the attempted sufficient factorization route; it does not prove that the witness expressions have different E8 outcomes and is not used to disposition any parent row. A stronger typed key is a new versioned construction; enumerating every candidate fiber was not performed.\n\n");
    out.push_str("## Coverage\n\n");
    out.push_str(&format!(
        "- parent rows: {}\n- refined: {}\n- covered by a successful universal theorem: {}\n- named impossibility of current-input refinement: {}\n- silent residue: {}\n- exact Schema-4 join: `{}`\n- exact Schema-5 join: `{}`\n- current-input scope explicit: `{}`\n- semantic nonexistence claimed: `{}`\n- count-blind: `{}`\n- all rows unpromoted: `{}`\n\n",
        render_number(&certificate.coverage.total_parent_rows),
        render_number(&certificate.coverage.refined_rows),
        render_number(&certificate.coverage.universal_rows),
        render_number(&certificate.coverage.named_impossibility_rows),
        render_number(&certificate.coverage.silent_residue_rows),
        certificate.coverage.schema4_exact_row_digest_join,
        certificate.coverage.schema5_exact_row_digest_join,
        certificate
            .coverage
            .every_named_impossibility_is_current_input_scoped,
        certificate.coverage.any_semantic_nonexistence_claimed,
        certificate.coverage.count_blind,
        certificate.coverage.all_parent_rows_remain_unpromoted,
    ));
    out.push_str("## Explicit row dispositions\n\n");
    out.push_str("| stratum | row | κ | class | extraction proxy | support | multiplicity | disposition | current-input obstruction |\n|---:|---:|---:|---|---|---:|---:|---|---|\n");
    for row in &certificate.row_dispositions {
        out.push_str(&format!(
            "| {} | {} | {} | `{}` | `{}` | {} | {} | `named_impossibility_of_current_input_refinement` | `{}` |\n",
            render_number(&row.stratum_index),
            render_number(&row.row_index),
            render_number(&row.kappa),
            row.class,
            row.clause_local_extraction_proxy,
            render_number(&row.direct_support),
            render_number(&row.aggregate_multiplicity),
            row.named_impossibility_id.as_deref().unwrap_or("missing"),
        ));
    }
    out.push_str("\n## Gate\n\n");
    out.push_str("The registered open bridge-condition identifiers remain unpromoted. BC-3 authorizes neither successor gate. This is the plan's explicit named-impossibility branch, scoped to proof construction from the current registered inputs; it is not silent residue and not a semantic nonexistence theorem.\n\n");
    out.push_str("## Required successor\n\n");
    out.push_str(&certificate.required_successor_action);
    out.push('\n');
    out
}

pub fn emit_bc3_parent_row_disposition_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Bc3ParentRowDispositionV1Certificate, Bc3Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Bc3Error::Io(
            "create-new refused because a BC-3 output already exists".to_owned(),
        ));
    }
    let certificate = issue_bc3_parent_row_disposition_v1()?;
    let replay = replay_bc3_parent_row_disposition_v1(&certificate);
    if !replay.valid {
        return Err(Bc3Error::Invariant(format!(
            "new BC-3 certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Bc3Error::Json(error.to_string()))?;
    let report = render_bc3_parent_row_disposition_v1(&certificate);
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(certificate_path)
        .map_err(|error| Bc3Error::Io(error.to_string()))?;
    if let Err(error) = certificate_file
        .write_all(json.as_bytes())
        .and_then(|_| certificate_file.write_all(b"\n"))
    {
        let _ = remove_file(certificate_path);
        return Err(Bc3Error::Io(error.to_string()));
    }
    let mut report_file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
    {
        Ok(file) => file,
        Err(error) => {
            let _ = remove_file(certificate_path);
            return Err(Bc3Error::Io(error.to_string()));
        }
    };
    if let Err(error) = report_file.write_all(report.as_bytes()) {
        let _ = remove_file(report_path);
        let _ = remove_file(certificate_path);
        return Err(Bc3Error::Io(error.to_string()));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reject(mut mutation: Bc3ParentRowDispositionV1Certificate) {
        mutation.result_digest = certificate_digest(&mutation);
        assert!(!replay_bc3_parent_row_disposition_v1(&mutation).valid);
    }

    #[test]
    fn bc3_replays_with_exact_named_impossibility_coverage() {
        let certificate = issue_bc3_parent_row_disposition_v1().expect("BC-3 certificate");
        let replay = replay_bc3_parent_row_disposition_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(certificate.m3_baseline.baseline_authenticated);
        assert!(certificate.universal_attempt.attempted);
        assert!(certificate.universal_attempt.source_first_generator_audited);
        assert!(
            !certificate
                .universal_attempt
                .artifact_erasure_used_as_sole_obstruction
        );
        assert!(
            !certificate
                .universal_attempt
                .existing_expr_join_key_determines_complete_typed_presentation
        );
        assert!(
            !certificate
                .universal_attempt
                .counterexample_refutes_every_e8_congruence
        );
        assert!(
            !certificate
                .universal_attempt
                .universal_refinement_theorem_proved
        );
        assert_eq!(certificate.row_dispositions.len(), 213);
        assert!(certificate.row_dispositions.iter().all(|row| {
            row.disposition == Bc3DispositionKind::NamedImpossibility
                && row.named_impossibility_id.is_some()
                && row.named_impossibility_scoped_to_current_inputs
                && !row.semantic_nonexistence_proved
                && !row.row_promoted
        }));
        assert_eq!(certificate.coverage.named_impossibility_rows.decimal, "213");
        assert_eq!(certificate.coverage.silent_residue_rows.decimal, "0");
        assert!(certificate.coverage.coverage_complete);
        assert!(
            certificate
                .coverage
                .every_named_impossibility_is_current_input_scoped
        );
        assert!(!certificate.coverage.any_semantic_nonexistence_claimed);
        assert!(!certificate.failed_universal_attempt_used_to_disposition_rows);
        assert!(certificate.row_dispositions_derived_from_each_rows_registered_payload);
        assert!(!certificate.m3_v2_authorized_by_bc3);
        assert!(!certificate.m4_authorized_by_bc3);
    }

    #[test]
    fn every_row_disposition_is_mutation_bound() {
        let certificate = issue_bc3_parent_row_disposition_v1().expect("BC-3 certificate");
        for row_index in 0..certificate.row_dispositions.len() {
            let mut disposition = certificate.clone();
            disposition.row_dispositions[row_index].disposition =
                Bc3DispositionKind::CoveredByUniversalTheorem;
            reject(disposition);

            let mut gap = certificate.clone();
            gap.row_dispositions[row_index].named_impossibility_id = None;
            reject(gap);

            let mut semantic_overclaim = certificate.clone();
            semantic_overclaim.row_dispositions[row_index].semantic_nonexistence_proved = true;
            reject(semantic_overclaim);

            let mut digest = certificate.clone();
            digest.row_dispositions[row_index]
                .derivation_hash
                .push_str(":mutated");
            reject(digest);

            let mut row_id = certificate.clone();
            row_id.row_dispositions[row_index]
                .aggregate_row_id
                .push_str(":mutated");
            reject(row_id);
        }

        let mut reordered = certificate.clone();
        reordered.row_dispositions.swap(0, 1);
        reject(reordered);

        let mut duplicated = certificate.clone();
        duplicated.row_dispositions[1] = duplicated.row_dispositions[0].clone();
        reject(duplicated);
    }

    #[test]
    fn baseline_universal_coverage_register_and_gate_mutations_fail() {
        let certificate = issue_bc3_parent_row_disposition_v1().expect("BC-3 certificate");
        let mut baseline = certificate.clone();
        baseline.m3_baseline.baseline_authenticated = false;
        reject(baseline);

        let mut universal = certificate.clone();
        universal
            .universal_attempt
            .universal_refinement_theorem_proved = true;
        reject(universal);

        let mut overbroad_counterexample = certificate.clone();
        overbroad_counterexample
            .universal_attempt
            .counterexample_refutes_every_e8_congruence = true;
        reject(overbroad_counterexample);

        let mut universal_used_for_rows = certificate.clone();
        universal_used_for_rows.failed_universal_attempt_used_to_disposition_rows = true;
        reject(universal_used_for_rows);

        let mut counterexample = certificate.clone();
        counterexample
            .universal_attempt
            .counterexample
            .typed_presentations_distinct = false;
        reject(counterexample);

        let mut coverage = certificate.clone();
        coverage.coverage.silent_residue_rows = metadata_number(1);
        reject(coverage);

        let mut register = certificate.clone();
        register.row_dispositions[0].aggregate_multiplicity.register =
            Bc3Register::SemanticRegisterAuthority;
        reject(register);

        let mut selector = certificate.clone();
        selector.row_dispositions[0].aggregate_multiplicity_used_as_selector = true;
        reject(selector);

        let mut promotion = certificate.clone();
        promotion.row_dispositions[0].row_promoted = true;
        reject(promotion);

        let mut gate = certificate.clone();
        gate.m3_v2_authorized_by_bc3 = true;
        reject(gate);
    }

    #[test]
    fn unknown_json_fields_and_outer_digest_mutations_fail() {
        let certificate = issue_bc3_parent_row_disposition_v1().expect("BC-3 certificate");
        let mut value = serde_json::to_value(&certificate).expect("BC-3 JSON");
        value
            .as_object_mut()
            .expect("certificate object")
            .insert("unexpected_claim".to_owned(), Value::Bool(true));
        assert!(
            !replay_bc3_parent_row_disposition_v1_json(
                &serde_json::to_string(&value).expect("mutated JSON")
            )
            .valid
        );

        let mut digest = certificate;
        digest.result_digest.push_str(":mutated");
        assert!(!replay_bc3_parent_row_disposition_v1(&digest).valid);
    }
}
