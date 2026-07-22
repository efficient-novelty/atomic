//! Combined T-BF1/T-BF3 replay at the adopted Stage-4 tie ladder.
//!
//! The two live theorem attempts are replayed rather than weakened.  Their
//! four-way Stage-4 stop is joined extensionally to the independently issued
//! R-T1 quotient certificate.  Candidate hashes are used only to prove that
//! all three artifacts discuss the same finite cone; neither hash order nor
//! any other presentation order is a selector.

use crate::naturality_orbit_transport::{
    A3_STRUCTURAL_COMPLETION_OUTPUT_GAP, A3PendingOutput, NaturalityOrbitTransportCertificate,
    issue_naturality_orbit_transport_certificate, replay_naturality_orbit_transport_certificate,
};
use crate::t_bf1_prefix::{
    ParsimonyPair, PrefixDecision, Tbf1Outcome, issue_t_bf1_prefix_certificate,
    replay_t_bf1_prefix_certificate,
};
use crate::t_bf3_enactment_equivalence::{
    Tbf3Outcome, Tbf3StageDisposition, issue_t_bf3_certificate, replay_t_bf3_certificate,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BF_TIE_PROTOCOL_RERUN_SCHEMA: &str = "t-bf1-t-bf3-tie-protocol-rerun-v3";
pub const T_BF_TIE_PROTOCOL_RERUN_DATE: &str = "2026-07-21";

const PROTOCOL_BYTES: &[u8] = include_bytes!("../../../docs/tie_resolution_protocol.md");
const T_BF1_SOURCE_BYTES: &[u8] = include_bytes!("t_bf1_prefix.rs");
const T_BF3_SOURCE_BYTES: &[u8] = include_bytes!("t_bf3_enactment_equivalence.rs");
const TRANSPORT_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_bf_tie_protocol_rerun.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TieRerunSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4TieEvidenceJoin {
    pub t_bf1_stop_stage: u32,
    pub t_bf3_stop_stage: u32,
    pub minimum_kappa: u16,
    pub minimum_certified_nu: u32,
    pub t_bf1_minimizer_hashes: Vec<String>,
    pub t_bf3_minimizer_hashes: Vec<String>,
    pub r_t1_package_hashes: Vec<String>,
    pub exact_four_way_minimizer_hash_set_agreement: bool,
    pub hashes_used_only_for_extensional_evidence_join: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub selected_candidate_hash: Option<String>,
    pub r_t1_semantic_class_count: usize,
    pub r_t1_class_members: Vec<Vec<String>>,
    pub r_t1_pairwise_comparison_count: usize,
    pub every_distinct_presentation_pair_semantically_inequivalent: bool,
    pub r_t1_completed: bool,
    pub r_t1_dissolved_tie: bool,
    pub r_t2_confluence_required: bool,
    pub r_t2_confluence_executed: bool,
    pub r_t3_user_adjudication_opened: bool,
    pub derivation_hash: String,
}

/// The first count-blind R-T2 obligation, recorded before any branch is
/// selected.  The historical Stage-5 completion occurrence is a premise
/// locator, not a proof that the four alternative Stage-4 branches share a
/// typed successor.  That comparison can begin only after the located output
/// hole has an intrinsically typed term.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2ConfluenceAttempt {
    pub rung_entered: bool,
    pub semantic_branch_count: usize,
    pub semantic_branch_class_ids: Vec<String>,
    pub historical_successor_stage: u32,
    pub historical_successor_rule: String,
    pub historical_successor_pending_output: A3PendingOutput,
    pub exact_instance_scheme_gap_evidence_present: bool,
    pub intrinsically_typed_successor_output_term_exists: bool,
    pub branchwise_typed_successor_obligations_constructed: bool,
    pub coarse_focus_label: String,
    pub coarse_focus_equality_used_as_semantic_confluence: bool,
    pub blocked_before_semantic_branch_comparison: bool,
    pub same_successor_obligation_proved: bool,
    pub equivalent_sealed_futures_proved: bool,
    pub confluence_proved: bool,
    pub confluence_refuted: bool,
    pub r_t3_opening_precondition_satisfied: bool,
    pub exact_blocker: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TieProtocolRerunOutcome {
    Rt2AttemptBlockedTypedSuccessorUndefined,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TbfTieProtocolRerunCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<TieRerunSourceBinding>,
    pub adopted_protocol_replayed: bool,
    pub t_bf1_live_certificate_digest: String,
    pub t_bf3_live_certificate_digest: String,
    pub naturality_orbit_transport_live_certificate_digest: String,
    pub t_bf1_live_replay_valid: bool,
    pub t_bf3_live_replay_valid: bool,
    pub naturality_orbit_transport_live_replay_valid: bool,
    pub stage4: Stage4TieEvidenceJoin,
    pub r_t2: Rt2ConfluenceAttempt,
    pub outcome: TieProtocolRerunOutcome,
    pub theorem_t_bf1_proved: bool,
    pub theorem_t_bf3_proved: bool,
    pub r_t3_user_adjudication_opened: bool,
    pub proposal_adoption_authorized: bool,
    pub bridge_authorized: bool,
    pub halt_claim_issued: bool,
    pub desired_history_or_downstream_score_used_as_premise: bool,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TbfTieProtocolRerunReplay {
    pub valid: bool,
    pub outcome: Option<TieProtocolRerunOutcome>,
    pub exact_four_way_hash_set_agreement: bool,
    pub r_t1_semantic_class_count: usize,
    pub r_t2_confluence_required: bool,
    pub r_t2_confluence_attempted: bool,
    pub r_t2_blocked_before_semantic_branch_comparison: bool,
    pub r_t2_confluence_proved: bool,
    pub r_t2_confluence_refuted: bool,
    pub theorem_t_bf1_proved: bool,
    pub theorem_t_bf3_proved: bool,
    pub r_t3_user_adjudication_opened: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TbfTieProtocolRerunError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted artifact did not replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BF_TIE_PROTOCOL_RERUN_SCHEMA, domain, value))
        .expect("tie-protocol evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<TieRerunSourceBinding> {
    [
        (
            "docs/tie_resolution_protocol.md",
            "adopted_quotient_confluence_adjudication_ladder",
            PROTOCOL_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf1_prefix.rs",
            "live_t_bf1_prefix_issuer_and_replay",
            T_BF1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf3_enactment_equivalence.rs",
            "live_t_bf3_enactment_issuer_and_replay",
            T_BF3_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "independent_r_t1_semantic_quotient",
            TRANSPORT_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf_tie_protocol_rerun.rs",
            "combined_fail_closed_evidence_join_and_replay",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| TieRerunSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn replay_protocol() -> Result<(), TbfTieProtocolRerunError> {
    let protocol = std::str::from_utf8(PROTOCOL_BYTES)
        .map_err(|error| TbfTieProtocolRerunError::Prerequisite(error.to_string()))?;
    for clause in [
        "R-T1",
        "Quotient first",
        "R-T2",
        "Confluence second",
        "R-T3",
        "Adjudication last",
        "Enumeration order is not an admissible option",
        "I adopt the tie-resolution protocol",
    ] {
        if !protocol.contains(clause) {
            return Err(TbfTieProtocolRerunError::Prerequisite(format!(
                "adopted tie protocol no longer contains {clause:?}"
            )));
        }
    }
    Ok(())
}

fn unique_hash_set(
    label: &str,
    values: &[String],
) -> Result<BTreeSet<String>, TbfTieProtocolRerunError> {
    let set = values.iter().cloned().collect::<BTreeSet<_>>();
    if set.len() != values.len() {
        return Err(TbfTieProtocolRerunError::Invariant(format!(
            "{label} contains duplicate candidate hashes"
        )));
    }
    Ok(set)
}

fn sorted_hashes(set: &BTreeSet<String>) -> Vec<String> {
    set.iter().cloned().collect()
}

fn certificate_digest(certificate: &TbfTieProtocolRerunCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

pub fn issue_t_bf_tie_protocol_rerun_certificate()
-> Result<TbfTieProtocolRerunCertificate, TbfTieProtocolRerunError> {
    replay_protocol()?;

    let t_bf1 = issue_t_bf1_prefix_certificate()
        .map_err(|error| TbfTieProtocolRerunError::Prerequisite(error.to_string()))?;
    let t_bf3 = issue_t_bf3_certificate()
        .map_err(|error| TbfTieProtocolRerunError::Prerequisite(error.to_string()))?;
    let transport: NaturalityOrbitTransportCertificate =
        issue_naturality_orbit_transport_certificate()
            .map_err(|error| TbfTieProtocolRerunError::Prerequisite(error.to_string()))?;

    let t_bf1_replay = replay_t_bf1_prefix_certificate(&t_bf1);
    let t_bf3_replay = replay_t_bf3_certificate(&t_bf3);
    let transport_replay = replay_naturality_orbit_transport_certificate(&transport);
    if !t_bf1_replay.valid || !t_bf3_replay.valid || !transport_replay.valid {
        return Err(TbfTieProtocolRerunError::Prerequisite(format!(
            "live predecessor replay failed: T-BF1={:?}; T-BF3={:?}; transport={:?}",
            t_bf1_replay.errors, t_bf3_replay.errors, transport_replay.errors
        )));
    }

    if !matches!(
        t_bf1.outcome,
        Tbf1Outcome::StoppedFBf1ParsimonyTie { stage: 4, count: 4 }
    ) || t_bf1.theorem_t_bf1_proved
    {
        return Err(TbfTieProtocolRerunError::Invariant(
            "live T-BF1 is not the exact unproved four-way Stage-4 stop".to_owned(),
        ));
    }
    if !matches!(
        t_bf3.outcome,
        Tbf3Outcome::FailedEqualPairParsimonyTie { stage: 4, count: 4 }
    ) || t_bf3.theorem_issued
        || t_bf3.enactment_equivalence_proved
    {
        return Err(TbfTieProtocolRerunError::Invariant(
            "live T-BF3 is not the exact unproved four-way Stage-4 stop".to_owned(),
        ));
    }

    let t_bf1_stage4 = t_bf1
        .stages
        .iter()
        .find(|stage| stage.stage == 4)
        .ok_or_else(|| TbfTieProtocolRerunError::Invariant("T-BF1 omits Stage 4".to_owned()))?;
    let t_bf3_stage4 = t_bf3
        .stage_records
        .iter()
        .find(|stage| stage.stage == 4)
        .ok_or_else(|| TbfTieProtocolRerunError::Invariant("T-BF3 omits Stage 4".to_owned()))?;

    if t_bf1_stage4.decision != PrefixDecision::StoppedFBf1EqualMinimumPair
        || t_bf1_stage4.selected_hash.is_some()
        || t_bf3_stage4.disposition != Tbf3StageDisposition::HaltedEqualPairParsimonyTie
        || t_bf3_stage4.selected_hash.is_some()
    {
        return Err(TbfTieProtocolRerunError::Invariant(
            "a live theorem attempt selected a Stage-4 presentation before R-T1".to_owned(),
        ));
    }
    let minimum = t_bf1_stage4.minimum_pair.clone().ok_or_else(|| {
        TbfTieProtocolRerunError::Invariant("T-BF1 Stage 4 has no minimum pair".to_owned())
    })?;
    if minimum
        != (ParsimonyPair {
            kappa: 3,
            certified_nu: 5,
        })
        || t_bf3_stage4.minimum_kappa != Some(minimum.kappa)
        || t_bf3_stage4.minimum_certified_nu_at_kappa != Some(minimum.certified_nu)
    {
        return Err(TbfTieProtocolRerunError::Invariant(
            "the live Stage-4 minimum pair does not agree at (kappa, nu) = (3, 5)".to_owned(),
        ));
    }

    let t_bf1_hash_set = unique_hash_set("T-BF1 minimizers", &t_bf1_stage4.minimizer_hashes)?;
    let t_bf3_hash_set = unique_hash_set("T-BF3 minimizers", &t_bf3_stage4.minimizer_hashes)?;
    let transport_hashes = transport
        .stage4
        .packages
        .iter()
        .map(|package| package.candidate_hash.clone())
        .collect::<Vec<_>>();
    let transport_hash_set = unique_hash_set("R-T1 packages", &transport_hashes)?;
    let exact_hash_agreement = t_bf1_hash_set.len() == 4
        && t_bf1_hash_set == t_bf3_hash_set
        && t_bf1_hash_set == transport_hash_set;
    if !exact_hash_agreement {
        return Err(TbfTieProtocolRerunError::Invariant(
            "T-BF1, T-BF3, and R-T1 do not name the same exact four-way minimizer set".to_owned(),
        ));
    }

    let mut class_members = transport
        .stage4
        .orbit_classes
        .iter()
        .map(|class| {
            let mut members = class.member_candidate_hashes.clone();
            members.sort();
            members
        })
        .collect::<Vec<_>>();
    class_members.sort();
    let class_member_set = class_members
        .iter()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let every_pair_inequivalent = transport.stage4.pairwise_comparisons.len() == 6
        && transport
            .stage4
            .pairwise_comparisons
            .iter()
            .all(|comparison| !comparison.packages_equal);
    if transport.stage4.minimizer_count != 4
        || transport.stage4.semantic_class_count != 4
        || class_members.len() != 4
        || class_members.iter().any(|members| members.len() != 1)
        || class_member_set != transport_hash_set
        || !every_pair_inequivalent
        || transport.stage4.r_t1_dissolves_tie
        || !transport.stage4.r_t2_required
        || !transport.stage4.no_selector_or_presentation_order_used
    {
        return Err(TbfTieProtocolRerunError::Invariant(
            "R-T1 did not certify four distinct Stage-4 semantic classes and advance to R-T2"
                .to_owned(),
        ));
    }

    let mut stage4 = Stage4TieEvidenceJoin {
        t_bf1_stop_stage: 4,
        t_bf3_stop_stage: 4,
        minimum_kappa: minimum.kappa,
        minimum_certified_nu: minimum.certified_nu,
        t_bf1_minimizer_hashes: sorted_hashes(&t_bf1_hash_set),
        t_bf3_minimizer_hashes: sorted_hashes(&t_bf3_hash_set),
        r_t1_package_hashes: sorted_hashes(&transport_hash_set),
        exact_four_way_minimizer_hash_set_agreement: true,
        hashes_used_only_for_extensional_evidence_join: true,
        hash_or_enumeration_order_used_as_selector: false,
        selected_candidate_hash: None,
        r_t1_semantic_class_count: transport.stage4.semantic_class_count,
        r_t1_class_members: class_members,
        r_t1_pairwise_comparison_count: transport.stage4.pairwise_comparisons.len(),
        every_distinct_presentation_pair_semantically_inequivalent: every_pair_inequivalent,
        r_t1_completed: true,
        r_t1_dissolved_tie: false,
        r_t2_confluence_required: true,
        r_t2_confluence_executed: false,
        r_t3_user_adjudication_opened: false,
        derivation_hash: String::new(),
    };
    stage4.derivation_hash = tagged_hash("stage4-evidence-join", &stage4);

    // A Stage-4 minimizer is followed by the Stage-5 structural demand.  The
    // historical A3 inventory contains exactly one occurrence of that
    // successor premise.  It is deliberately used only to locate the typed
    // confluence obligation; its coarse focus label cannot establish that
    // the four semantic branches have equal successors.
    let mut stage5_successor_pending = transport
        .a3
        .structural_completion_pending_outputs
        .iter()
        .filter(|pending| pending.stage == 5)
        .cloned()
        .collect::<Vec<_>>();
    if stage5_successor_pending.len() != 1 {
        return Err(TbfTieProtocolRerunError::Invariant(format!(
            "R-T2 expected one historical Stage-5 structural-completion successor premise, found {}",
            stage5_successor_pending.len()
        )));
    }
    let historical_successor_pending_output = stage5_successor_pending
        .pop()
        .expect("the exact singleton length was checked");
    let exact_instance_scheme_gap_evidence_present = historical_successor_pending_output
        .a3_instance_id
        .as_deref()
        .is_some_and(|value| !value.is_empty())
        && historical_successor_pending_output
            .a3_scheme_id
            .as_deref()
            .is_some_and(|value| !value.is_empty())
        && historical_successor_pending_output.gap_id == A3_STRUCTURAL_COMPLETION_OUTPUT_GAP
        && !historical_successor_pending_output.exact_reason.is_empty()
        && !historical_successor_pending_output
            .derivation_hash
            .is_empty();
    if historical_successor_pending_output.rule != "structural_completion::InitialHit"
        || historical_successor_pending_output.clause_index.is_some()
        || historical_successor_pending_output
            .source_registration_name
            .is_some()
        || historical_successor_pending_output
            .source_naturality_hash
            .is_some()
        || !exact_instance_scheme_gap_evidence_present
    {
        return Err(TbfTieProtocolRerunError::Invariant(
            "the historical Stage-5 successor is not the exact pending InitialHit typed-hole occurrence"
                .to_owned(),
        ));
    }
    let mut semantic_branch_class_ids = transport
        .stage4
        .orbit_classes
        .iter()
        .map(|class| class.class_id.clone())
        .collect::<Vec<_>>();
    semantic_branch_class_ids.sort();
    if semantic_branch_class_ids.len() != 4
        || semantic_branch_class_ids.iter().any(|id| id.is_empty())
        || semantic_branch_class_ids
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            != 4
    {
        return Err(TbfTieProtocolRerunError::Invariant(
            "R-T2 did not receive the four distinct R-T1 semantic branch classes".to_owned(),
        ));
    }
    let mut r_t2 = Rt2ConfluenceAttempt {
        rung_entered: true,
        semantic_branch_count: semantic_branch_class_ids.len(),
        semantic_branch_class_ids,
        historical_successor_stage: historical_successor_pending_output.stage,
        historical_successor_rule: historical_successor_pending_output.rule.clone(),
        historical_successor_pending_output,
        exact_instance_scheme_gap_evidence_present,
        intrinsically_typed_successor_output_term_exists: false,
        branchwise_typed_successor_obligations_constructed: false,
        coarse_focus_label: "initial_hit".to_owned(),
        coarse_focus_equality_used_as_semantic_confluence: false,
        blocked_before_semantic_branch_comparison: true,
        same_successor_obligation_proved: false,
        equivalent_sealed_futures_proved: false,
        confluence_proved: false,
        confluence_refuted: false,
        r_t3_opening_precondition_satisfied: false,
        exact_blocker: "The historical Stage-5 InitialHit occurrence supplies an exact A3 instance id and natural-scheme id, but its structural-completion output remains A3_STRUCTURAL_COMPLETION_TYPED_HOLE_UNDEFINED. Without that intrinsically typed term, the occurrence cannot be instantiated on each of the four Stage-4 semantic branches, so neither successor-orbit equality nor sealed-future equivalence is a well-formed comparison. The common coarse focus label is not semantic confluence.".to_owned(),
        derivation_hash: String::new(),
    };
    r_t2.derivation_hash = tagged_hash("r-t2-first-confluence-obligation", &r_t2);

    // "Executed" here means the adopted rung was actually entered and its
    // first obligation attempted.  It does not mean confluence was proved or
    // refuted; those statuses are separately serialized above.
    stage4.r_t2_confluence_executed = true;
    stage4.derivation_hash.clear();
    stage4.derivation_hash = tagged_hash("stage4-evidence-join", &stage4);

    let mut certificate = TbfTieProtocolRerunCertificate {
        schema: T_BF_TIE_PROTOCOL_RERUN_SCHEMA.to_owned(),
        date: T_BF_TIE_PROTOCOL_RERUN_DATE.to_owned(),
        source_bindings: source_bindings(),
        adopted_protocol_replayed: true,
        t_bf1_live_certificate_digest: t_bf1.result_digest,
        t_bf3_live_certificate_digest: t_bf3.result_digest,
        naturality_orbit_transport_live_certificate_digest: transport.result_digest,
        t_bf1_live_replay_valid: true,
        t_bf3_live_replay_valid: true,
        naturality_orbit_transport_live_replay_valid: true,
        stage4,
        r_t2,
        outcome: TieProtocolRerunOutcome::Rt2AttemptBlockedTypedSuccessorUndefined,
        theorem_t_bf1_proved: false,
        theorem_t_bf3_proved: false,
        r_t3_user_adjudication_opened: false,
        proposal_adoption_authorized: false,
        bridge_authorized: false,
        halt_claim_issued: false,
        desired_history_or_downstream_score_used_as_premise: false,
        permitted_conclusion: "The live T-BF1 and T-BF3 attempts stop on the same four Stage-4 minimizers. R-T1 leaves four distinct semantic packages. R-T2 has now been entered, and its first count-blind obligation is the Stage-5 InitialHit successor orbit. The historical occurrence has exact instance, scheme, and gap evidence, but no intrinsically typed structural-completion output term; branch comparison is therefore blocked before confluence can be proved or refuted. No presentation is selected, R-T3 remains closed, and neither theorem is proved.".to_owned(),
        required_successor_action: "Define and kernel-type the Stage-5 InitialHit structural-completion output without importing the later historical answer. Instantiate that output independently over all four R-T1 semantic classes, then compare successor obligations and sealed futures count-blindly at R-T2. Keep R-T3 closed unless a well-formed confluence comparison refutes confluence; do not adopt the bar-free law, run the bridge, or issue a halt certificate from this artifact.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: String) -> TbfTieProtocolRerunReplay {
    TbfTieProtocolRerunReplay {
        valid: false,
        outcome: None,
        exact_four_way_hash_set_agreement: false,
        r_t1_semantic_class_count: 0,
        r_t2_confluence_required: false,
        r_t2_confluence_attempted: false,
        r_t2_blocked_before_semantic_branch_comparison: false,
        r_t2_confluence_proved: false,
        r_t2_confluence_refuted: false,
        theorem_t_bf1_proved: false,
        theorem_t_bf3_proved: false,
        r_t3_user_adjudication_opened: false,
        errors: vec![error],
    }
}

pub fn replay_t_bf_tie_protocol_rerun_certificate(
    certificate: &TbfTieProtocolRerunCertificate,
) -> TbfTieProtocolRerunReplay {
    let expected = match issue_t_bf_tie_protocol_rerun_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate != &expected {
        errors.push("certificate differs from independent live tie-protocol rerun".to_owned());
    }
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    TbfTieProtocolRerunReplay {
        valid: errors.is_empty(),
        outcome: Some(certificate.outcome.clone()),
        exact_four_way_hash_set_agreement: certificate
            .stage4
            .exact_four_way_minimizer_hash_set_agreement,
        r_t1_semantic_class_count: certificate.stage4.r_t1_semantic_class_count,
        r_t2_confluence_required: certificate.stage4.r_t2_confluence_required,
        r_t2_confluence_attempted: certificate.r_t2.rung_entered,
        r_t2_blocked_before_semantic_branch_comparison: certificate
            .r_t2
            .blocked_before_semantic_branch_comparison,
        r_t2_confluence_proved: certificate.r_t2.confluence_proved,
        r_t2_confluence_refuted: certificate.r_t2.confluence_refuted,
        theorem_t_bf1_proved: certificate.theorem_t_bf1_proved,
        theorem_t_bf3_proved: certificate.theorem_t_bf3_proved,
        r_t3_user_adjudication_opened: certificate.r_t3_user_adjudication_opened,
        errors,
    }
}

pub fn emit_t_bf_tie_protocol_rerun_create_new(
    path: &Path,
) -> Result<TbfTieProtocolRerunReplay, TbfTieProtocolRerunError> {
    let certificate = issue_t_bf_tie_protocol_rerun_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| TbfTieProtocolRerunError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| TbfTieProtocolRerunError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| TbfTieProtocolRerunError::Io(error.to_string()))?;
    let replay = replay_t_bf_tie_protocol_rerun_certificate(&certificate);
    if !replay.valid {
        return Err(TbfTieProtocolRerunError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> TbfTieProtocolRerunCertificate {
        static CERTIFICATE: OnceLock<TbfTieProtocolRerunCertificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_t_bf_tie_protocol_rerun_certificate().unwrap())
            .clone()
    }

    #[test]
    fn exact_live_tie_enters_r_t2_and_records_the_typed_successor_blocker() {
        let certificate = certificate();
        assert_eq!(
            certificate.outcome,
            TieProtocolRerunOutcome::Rt2AttemptBlockedTypedSuccessorUndefined
        );
        assert!(
            certificate
                .stage4
                .exact_four_way_minimizer_hash_set_agreement
        );
        assert_eq!(certificate.stage4.r_t1_semantic_class_count, 4);
        assert_eq!(certificate.stage4.r_t1_class_members.len(), 4);
        assert!(
            certificate
                .stage4
                .r_t1_class_members
                .iter()
                .all(|members| members.len() == 1)
        );
        assert!(certificate.stage4.r_t2_confluence_required);
        assert!(certificate.stage4.r_t2_confluence_executed);
        assert!(!certificate.stage4.r_t3_user_adjudication_opened);
        assert!(certificate.stage4.selected_candidate_hash.is_none());
        assert!(
            !certificate
                .stage4
                .hash_or_enumeration_order_used_as_selector
        );
        assert!(!certificate.theorem_t_bf1_proved);
        assert!(!certificate.theorem_t_bf3_proved);
        assert!(!certificate.proposal_adoption_authorized);
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.halt_claim_issued);
        assert!(certificate.r_t2.rung_entered);
        assert_eq!(certificate.r_t2.semantic_branch_count, 4);
        assert_eq!(certificate.r_t2.semantic_branch_class_ids.len(), 4);
        assert_eq!(certificate.r_t2.historical_successor_stage, 5);
        assert_eq!(
            certificate.r_t2.historical_successor_rule,
            "structural_completion::InitialHit"
        );
        assert!(
            certificate
                .r_t2
                .historical_successor_pending_output
                .a3_instance_id
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        );
        assert!(
            certificate
                .r_t2
                .historical_successor_pending_output
                .a3_scheme_id
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        );
        assert_eq!(
            certificate.r_t2.historical_successor_pending_output.gap_id,
            A3_STRUCTURAL_COMPLETION_OUTPUT_GAP
        );
        assert!(certificate.r_t2.exact_instance_scheme_gap_evidence_present);
        assert!(!certificate.r_t2.exact_blocker.is_empty());
        assert!(certificate.r_t2.blocked_before_semantic_branch_comparison);
        assert!(
            !certificate
                .r_t2
                .coarse_focus_equality_used_as_semantic_confluence
        );
        assert!(!certificate.r_t2.confluence_proved);
        assert!(!certificate.r_t2.confluence_refuted);
        assert!(!certificate.r_t2.r_t3_opening_precondition_satisfied);
        let replay = replay_t_bf_tie_protocol_rerun_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(replay.r_t2_confluence_attempted);
        assert!(replay.r_t2_blocked_before_semantic_branch_comparison);
        assert!(!replay.r_t2_confluence_proved);
        assert!(!replay.r_t2_confluence_refuted);
    }

    #[test]
    fn replay_rejects_a_redigested_forged_promotion() {
        let mut forged = certificate();
        forged.theorem_t_bf1_proved = true;
        forged.theorem_t_bf3_proved = true;
        forged.r_t2.intrinsically_typed_successor_output_term_exists = true;
        forged
            .r_t2
            .branchwise_typed_successor_obligations_constructed = true;
        forged.r_t2.blocked_before_semantic_branch_comparison = false;
        forged.r_t2.same_successor_obligation_proved = true;
        forged.r_t2.equivalent_sealed_futures_proved = true;
        forged.r_t2.confluence_proved = true;
        forged.r_t2.r_t3_opening_precondition_satisfied = true;
        forged.r_t2.derivation_hash.clear();
        forged.r_t2.derivation_hash = tagged_hash("r-t2-first-confluence-obligation", &forged.r_t2);
        forged.r_t3_user_adjudication_opened = true;
        forged.stage4.r_t3_user_adjudication_opened = true;
        forged.stage4.derivation_hash.clear();
        forged.stage4.derivation_hash = tagged_hash("stage4-evidence-join", &forged.stage4);
        forged.proposal_adoption_authorized = true;
        forged.bridge_authorized = true;
        forged.halt_claim_issued = true;
        forged.result_digest = certificate_digest(&forged);
        assert!(!replay_t_bf_tie_protocol_rerun_certificate(&forged).valid);
    }
}
