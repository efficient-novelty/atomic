//! Versioned R-T2 successor-scheme confluence attempt.
//!
//! The runner reconstructs the Stage-5 A3 window independently over each of
//! the four R-T1 semantic branches.  Constructor-predicate equality and full
//! scheme-family equivalence are deliberately separate judgments: sharing an
//! `InitialHit` label is not confluence.  Candidate/hash ordering is used only
//! to serialize finite set evidence, never to select a branch.

use crate::naturality_orbit_transport::{
    FrozenEqualityRecord, NaturalityOrbitTransportCertificate,
    issue_naturality_orbit_transport_certificate, replay_naturality_orbit_transport_certificate,
};
use crate::t_bf_tie_protocol_rerun::{
    TbfTieProtocolRerunCertificate, issue_t_bf_tie_protocol_rerun_certificate,
    replay_t_bf_tie_protocol_rerun_certificate,
};
use crate::t_bf1_prefix::{issue_t_bf1_prefix_certificate, replay_t_bf1_prefix_certificate};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandOutputType, A3DemandSchemeOrigin, A3HistoricalWindow, A3TypedClauseSource,
    A3TypedDemandScheme, generate_a3_window_for_prefix,
};
use pen_eval::future_hole_hypothesis::{
    FutureHoleOpenJudgment, issue_structural_future_holes_for_window,
};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use pen_type::equality::{KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const R_T2_FUTURE_HOLE_CONFLUENCE_SCHEMA: &str = "r-t2-future-hole-confluence-v1";
pub const R_T2_FUTURE_HOLE_CONFLUENCE_DATE: &str = "2026-07-21";

const FUTURE_HOLE_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/future_hole_definition_adjudication.md");
const TIE_PROTOCOL_BYTES: &[u8] = include_bytes!("../../../docs/tie_resolution_protocol.md");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const FUTURE_HOLE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/future_hole_hypothesis.rs");
const TRANSPORT_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const TIE_RERUN_SOURCE_BYTES: &[u8] = include_bytes!("t_bf_tie_protocol_rerun.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("r_t2_future_hole_confluence.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchemeFamilyPresentation {
    pub source_anchor_audit_only: String,
    pub canonical_normal_form: Expr,
    pub parameter_sorts_json: String,
    pub kernel_type_json: String,
    pub presentation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage5SchemeFormation {
    pub local_scheme_id: String,
    pub local_instance_id: String,
    pub rule_constructor: String,
    pub origin_kind: String,
    pub support_depth: u8,
    pub source_families: Vec<SchemeFamilyPresentation>,
    pub output_shape_json: String,
    pub formation_derivation_hash: String,
    pub semantic_formation_class_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage5ConstructorPredicateEvidence {
    pub constructor: String,
    pub structural_snapshot_json: String,
    pub premises_json: String,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchStructuralFutureHole {
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub stage: u32,
    pub visible_library_at_registration: u32,
    pub structural_constructor: String,
    pub body: Expr,
    pub declared_motives_json: String,
    pub source_kernel_type_json: String,
    pub output_kernel_type_json: String,
    pub output_type_preserved: bool,
    pub motives_declared_before_verdict_or_filler: bool,
    pub motive_inferred_repaired_or_outcome_selected: bool,
    pub open_judgment_kernel_typed: bool,
    pub every_motive_bn_formable: bool,
    pub every_term_node_in_adopted_closure: bool,
    pub hypothetical_derivation_replayable: bool,
    pub d_membership_issued: bool,
    pub zero_kappa_nu_anchor_orbit_and_credit: bool,
    pub named_gap: Option<String>,
    pub generic_substitution_theorem_hash: String,
    pub local_derivation_hash: String,
    pub branch_independent_semantic_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchStage5Formation {
    pub r_t1_class_id: String,
    pub stage4_candidate_hash: String,
    pub stage4_telescope: Telescope,
    pub prefix_signature_digest: String,
    pub prefix_steps_exactly_one_through_four: bool,
    pub stage5_window_derivation_hash: String,
    pub stage5_newest_step: u32,
    pub stage5_older_step: u32,
    pub constructor_predicate: Stage5ConstructorPredicateEvidence,
    pub structural_future_hole: BranchStructuralFutureHole,
    pub every_instance_typed: bool,
    pub every_scheme_has_formation_evidence: bool,
    pub schemes: Vec<Stage5SchemeFormation>,
    pub scheme_formation_class_ids: Vec<String>,
    pub formation_set_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyQuotientComparison {
    pub left_source_anchor_audit_only: String,
    pub right_source_anchor_audit_only: String,
    pub parameter_sorts_equal: bool,
    pub kernel_types_equal: bool,
    pub frozen_equality: FrozenEqualityRecord,
    pub equivalent: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchemeFormationComparison {
    pub left_scheme_id: String,
    pub right_scheme_id: String,
    pub rule_constructor_equal: bool,
    pub origin_kind_equal: bool,
    pub support_depth_equal: bool,
    pub source_arity_equal: bool,
    pub source_family_comparisons: Vec<FamilyQuotientComparison>,
    pub output_shape_equal: bool,
    pub equivalent_under_certified_family_quotient: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PairwiseStage5SchemeSetComparison {
    pub left_candidate_hash: String,
    pub right_candidate_hash: String,
    pub left_scheme_count: usize,
    pub right_scheme_count: usize,
    pub comparisons: Vec<SchemeFormationComparison>,
    pub perfect_matching: Vec<(String, String)>,
    pub unmatched_left_formation_classes: Vec<String>,
    pub unmatched_right_formation_classes: Vec<String>,
    pub full_scheme_sets_equivalent: bool,
    pub comparison_well_formed: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rt2FutureHoleConfluenceOutcome {
    RefutedByInequivalentStage5SchemeSets,
    Stage5SchemeSetsEquivalentFutureIsomorphismPending,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2FutureHoleConfluenceCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Rt2SourceBinding>,
    pub future_hole_adoption_replayed: bool,
    pub tie_protocol_adoption_replayed: bool,
    pub equality_procedure: String,
    pub naturality_orbit_transport_certificate_digest: String,
    pub naturality_orbit_transport_replay_valid: bool,
    pub predecessor_tie_rerun_certificate_digest: String,
    pub predecessor_tie_rerun_replay_valid: bool,
    pub predecessor_was_exact_r_t2_typed_output_blocker: bool,
    pub t_bf1_prefix_certificate_digest: String,
    pub t_bf1_prefix_replay_valid: bool,
    pub exact_four_way_stage4_minimizer_join: bool,
    pub same_stage4_live_demand_proved: bool,
    pub stage4_required_packages: Vec<String>,
    pub every_stage4_minimizer_strict_total_discharger: bool,
    pub r_t1_semantic_class_count: usize,
    pub branch_count: usize,
    pub branches: Vec<BranchStage5Formation>,
    pub every_branch_is_independent_exact_prefix_generation: bool,
    pub constructor_predicates_all_equal: bool,
    pub constructor_predicate_equality_used_as_confluence: bool,
    pub structural_future_holes_all_issued_and_replayable: bool,
    pub structural_future_hole_semantic_keys_all_equal: bool,
    pub pairwise_scheme_set_comparisons: Vec<PairwiseStage5SchemeSetComparison>,
    pub every_pairwise_comparison_well_formed: bool,
    pub all_stage5_scheme_sets_equivalent: bool,
    pub immediate_inequivalent_successor_counterexample: bool,
    pub deterministic_future_isomorphism_replayed: bool,
    pub r_t2_confluence_proved: bool,
    pub r_t2_confluence_refuted: bool,
    pub r_t3_user_adjudication_opened: bool,
    pub selected_candidate_hash: Option<String>,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub desired_history_count_score_or_bar_used_as_premise: bool,
    pub theorem_t_bf1_proved: bool,
    pub theorem_t_bf3_proved: bool,
    pub bar_free_adoption_authorized: bool,
    pub bridge_authorized: bool,
    pub halt_claim_issued: bool,
    pub outcome: Rt2FutureHoleConfluenceOutcome,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2FutureHoleConfluenceReplay {
    pub valid: bool,
    pub outcome: Option<Rt2FutureHoleConfluenceOutcome>,
    pub branch_count: usize,
    pub structural_future_holes_all_issued_and_replayable: bool,
    pub constructor_predicates_all_equal: bool,
    pub pairwise_comparison_count: usize,
    pub every_pairwise_comparison_well_formed: bool,
    pub all_stage5_scheme_sets_equivalent: bool,
    pub r_t2_confluence_proved: bool,
    pub r_t2_confluence_refuted: bool,
    pub r_t3_user_adjudication_opened: bool,
    pub selected_candidate_hash: Option<String>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Rt2FutureHoleConfluenceError {
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
    let bytes = serde_json::to_vec(&(R_T2_FUTURE_HOLE_CONFLUENCE_SCHEMA, domain, value))
        .expect("R-T2 confluence evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<Rt2SourceBinding> {
    [
        (
            "docs/future_hole_definition_adjudication.md",
            "adopted_future_hole_hypothesis_definition",
            FUTURE_HOLE_ADJUDICATION_BYTES,
        ),
        (
            "docs/tie_resolution_protocol.md",
            "adopted_r_t1_r_t2_r_t3_ladder",
            TIE_PROTOCOL_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "count_blind_arbitrary_prefix_stage_window_generator",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/future_hole_hypothesis.rs",
            "adopted_branch_local_structural_future_hole_issuer",
            FUTURE_HOLE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "certified_r_t1_classes_and_family_quotient_machinery",
            TRANSPORT_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf_tie_protocol_rerun.rs",
            "live_predecessor_r_t2_entry_certificate",
            TIE_RERUN_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/r_t2_future_hole_confluence.rs",
            "count_blind_full_stage5_scheme_set_comparison",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Rt2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn replay_adoptions() -> Result<(), Rt2FutureHoleConfluenceError> {
    let future = std::str::from_utf8(FUTURE_HOLE_ADJUDICATION_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceError::Prerequisite(error.to_string()))?;
    for clause in [
        "future-hole-hypothesis-definition-v1",
        "Stage successors (R-T2's instrument)",
        "holes and fillings mint nothing",
        "I adopt `future-hole-hypothesis-definition-v1`",
    ] {
        if !future.contains(clause) {
            return Err(Rt2FutureHoleConfluenceError::Prerequisite(format!(
                "future-hole adjudication omits {clause:?}"
            )));
        }
    }
    let protocol = std::str::from_utf8(TIE_PROTOCOL_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceError::Prerequisite(error.to_string()))?;
    for clause in [
        "R-T1",
        "R-T2",
        "Confluence second",
        "R-T3",
        "Enumeration order is not an admissible option",
    ] {
        if !protocol.contains(clause) {
            return Err(Rt2FutureHoleConfluenceError::Prerequisite(format!(
                "tie protocol omits {clause:?}"
            )));
        }
    }
    Ok(())
}

fn prefix_signature(stage4: &Telescope) -> SealedSignature {
    let mut telescopes = (1..=3)
        .map(|step| (step, Telescope::reference(step)))
        .collect::<Vec<_>>();
    telescopes.push((4, stage4.clone()));
    SealedSignature::from_telescopes(telescopes)
}

fn output_shape_json(output: &A3DemandOutputType) -> Result<String, Rt2FutureHoleConfluenceError> {
    let value = match output {
        A3DemandOutputType::ActionAt { source_type, .. } => {
            serde_json::json!({"output": "action_at", "source_type": source_type})
        }
        A3DemandOutputType::ChronologicalInteraction {
            older_type,
            newest_type,
            interface_mode,
            ..
        } => serde_json::json!({
            "output": "chronological_interaction",
            "older_type": older_type,
            "newest_type": newest_type,
            "interface_mode": interface_mode,
        }),
        A3DemandOutputType::ContractibleOpenBox {
            dimension,
            boundary_types,
            path_witness_families,
            ..
        } => serde_json::json!({
            "output": "contractible_open_box",
            "dimension": dimension,
            "boundary_types": boundary_types,
            "path_witness_count": path_witness_families.len(),
        }),
        A3DemandOutputType::StructuralCompletion {
            constructor,
            structural_snapshot,
        } => serde_json::json!({
            "output": "structural_completion",
            "constructor": constructor,
            "structural_snapshot": structural_snapshot,
        }),
    };
    serde_json::to_string(&value)
        .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))
}

fn origin_kind(origin: &A3DemandSchemeOrigin) -> String {
    match origin {
        A3DemandSchemeOrigin::BaseRule { .. } => "base_rule".to_owned(),
        A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } => {
            format!("structural_completion::{}", constructor.slug())
        }
    }
}

fn family_presentation(
    source: &A3TypedClauseSource,
) -> Result<SchemeFamilyPresentation, Rt2FutureHoleConfluenceError> {
    let parameter_sorts_json = serde_json::to_string(&source.canonical_presentation.parameters)
        .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?;
    let kernel_type_json = serde_json::to_string(&source.kernel_type)
        .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?;
    let presentation_hash = tagged_hash(
        "scheme-source-family-presentation",
        &(
            &source.canonical_presentation.canonical_normal_form,
            &parameter_sorts_json,
            &kernel_type_json,
        ),
    );
    Ok(SchemeFamilyPresentation {
        source_anchor_audit_only: source.anchor_id.clone(),
        canonical_normal_form: source.canonical_presentation.canonical_normal_form.clone(),
        parameter_sorts_json,
        kernel_type_json,
        presentation_hash,
    })
}

fn scheme_formation(
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
) -> Result<Stage5SchemeFormation, Rt2FutureHoleConfluenceError> {
    let instances = window
        .instances
        .iter()
        .filter(|instance| instance.scheme_id == scheme.scheme_id)
        .collect::<Vec<_>>();
    if instances.len() != 1 {
        return Err(Rt2FutureHoleConfluenceError::Invariant(format!(
            "Stage-5 branch scheme {} has {} instances instead of one",
            scheme.scheme_id,
            instances.len()
        )));
    }
    let instance = instances[0];
    let source_families = instance
        .source_anchor_ids
        .iter()
        .map(|anchor| {
            let source = window
                .typed_sources
                .iter()
                .find(|source| source.anchor_id == *anchor)
                .ok_or_else(|| {
                    Rt2FutureHoleConfluenceError::Invariant(format!(
                        "scheme {} source anchor {anchor} is absent",
                        scheme.scheme_id
                    ))
                })?;
            family_presentation(source)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let rule_constructor = serde_json::to_string(&scheme.rule_constructor)
        .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?;
    let origin_kind = origin_kind(&scheme.origin);
    let output_shape_json = output_shape_json(&scheme.required_output)?;
    let semantic_formation_class_id = tagged_hash(
        "stage5-scheme-formation-class",
        &(
            &rule_constructor,
            &origin_kind,
            scheme.support_depth,
            &source_families
                .iter()
                .map(|family| {
                    (
                        &family.canonical_normal_form,
                        &family.parameter_sorts_json,
                        &family.kernel_type_json,
                    )
                })
                .collect::<Vec<_>>(),
            &output_shape_json,
        ),
    );
    Ok(Stage5SchemeFormation {
        local_scheme_id: scheme.scheme_id.clone(),
        local_instance_id: instance.instance_id.clone(),
        rule_constructor,
        origin_kind,
        support_depth: scheme.support_depth,
        source_families,
        output_shape_json,
        formation_derivation_hash: scheme.formation_derivation_hash.clone(),
        semantic_formation_class_id,
    })
}

fn constructor_predicate(
    window: &A3HistoricalWindow,
) -> Result<Stage5ConstructorPredicateEvidence, Rt2FutureHoleConfluenceError> {
    if window.constructor_evidence.len() != 1 {
        return Err(Rt2FutureHoleConfluenceError::Invariant(format!(
            "Stage-5 branch has {} constructor predicates instead of one",
            window.constructor_evidence.len()
        )));
    }
    let evidence = &window.constructor_evidence[0];
    Ok(Stage5ConstructorPredicateEvidence {
        constructor: serde_json::to_string(&evidence.constructor)
            .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?,
        structural_snapshot_json: serde_json::to_string(&evidence.structural_snapshot)
            .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?,
        premises_json: serde_json::to_string(&evidence.premises)
            .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?,
        evidence_hash: evidence.evidence_hash.clone(),
    })
}

fn structural_future_hole(
    window: &A3HistoricalWindow,
    row: &FutureHoleOpenJudgment,
) -> Result<BranchStructuralFutureHole, Rt2FutureHoleConfluenceError> {
    let structural_constructor = row.structural_constructor.clone().ok_or_else(|| {
        Rt2FutureHoleConfluenceError::Invariant(
            "branch-local structural future hole omitted its constructor".to_owned(),
        )
    })?;
    let structural_scheme = window
        .schemes
        .iter()
        .find(|scheme| scheme.scheme_id == row.a3_scheme_id)
        .ok_or_else(|| {
            Rt2FutureHoleConfluenceError::Invariant(format!(
                "future hole references absent scheme {}",
                row.a3_scheme_id
            ))
        })?;
    let A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } = &structural_scheme.origin
    else {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "future-hole scheme is not structural completion".to_owned(),
        ));
    };
    let structural_instance = window
        .instances
        .iter()
        .find(|instance| instance.instance_id == row.a3_instance_id)
        .ok_or_else(|| {
            Rt2FutureHoleConfluenceError::Invariant(format!(
                "future hole references absent instance {}",
                row.a3_instance_id
            ))
        })?;
    let declared_motives_json = serde_json::to_string(&row.declared_motives)
        .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?;
    let zero_kappa_nu_anchor_orbit_and_credit = row.charge.marginal_kappa == 0
        && row.charge.marginal_nu == 0
        && row.charge.anchors_minted == 0
        && row.charge.demand_orbits_minted == 0
        && !row.charge.credit_minted
        && !row.charge.filler_minted_credit;
    if row.stage != 5
        || row.visible_library_at_registration != 4
        || structural_constructor != "initial_hit"
        || constructor.slug() != structural_constructor
        || structural_instance.scheme_id != row.a3_scheme_id
        || row.source_step.is_some()
        || row.source_clause.is_some()
        || row.body != Expr::Var(1)
        || declared_motives_json != "[\"type\"]"
        || row.source_kernel_type_json != "\"Type\""
        || row.output_kernel_type_json != "\"Type\""
        || !row.output_type_preserved
        || !row.motives_declared_before_verdict_or_filler
        || row.motive_inferred_repaired_or_outcome_selected
        || !row.open_judgment_kernel_typed
        || !row.every_motive_bn_formable
        || !row.every_term_node_in_adopted_closure
        || !row.hypothetical_derivation_replayable
        || !row.d_membership_issued
        || !zero_kappa_nu_anchor_orbit_and_credit
        || row.named_gap.is_some()
    {
        return Err(Rt2FutureHoleConfluenceError::Invariant(format!(
            "Stage-5 structural future-hole invariant failed for scheme {}",
            row.a3_scheme_id
        )));
    }
    let branch_independent_semantic_key = tagged_hash(
        "branch-independent-structural-future-hole",
        &(
            &structural_constructor,
            &row.body,
            &declared_motives_json,
            &row.source_kernel_type_json,
            &row.output_kernel_type_json,
            row.output_type_preserved,
            row.open_judgment_kernel_typed,
            row.every_motive_bn_formable,
            row.every_term_node_in_adopted_closure,
            row.hypothetical_derivation_replayable,
            row.d_membership_issued,
            zero_kappa_nu_anchor_orbit_and_credit,
        ),
    );
    Ok(BranchStructuralFutureHole {
        a3_instance_id: row.a3_instance_id.clone(),
        a3_scheme_id: row.a3_scheme_id.clone(),
        stage: row.stage,
        visible_library_at_registration: row.visible_library_at_registration,
        structural_constructor,
        body: row.body.clone(),
        declared_motives_json,
        source_kernel_type_json: row.source_kernel_type_json.clone(),
        output_kernel_type_json: row.output_kernel_type_json.clone(),
        output_type_preserved: row.output_type_preserved,
        motives_declared_before_verdict_or_filler: row.motives_declared_before_verdict_or_filler,
        motive_inferred_repaired_or_outcome_selected: row
            .motive_inferred_repaired_or_outcome_selected,
        open_judgment_kernel_typed: row.open_judgment_kernel_typed,
        every_motive_bn_formable: row.every_motive_bn_formable,
        every_term_node_in_adopted_closure: row.every_term_node_in_adopted_closure,
        hypothetical_derivation_replayable: row.hypothetical_derivation_replayable,
        d_membership_issued: row.d_membership_issued,
        zero_kappa_nu_anchor_orbit_and_credit,
        named_gap: row.named_gap.clone(),
        generic_substitution_theorem_hash: row.generic_substitution_theorem_hash.clone(),
        local_derivation_hash: row.derivation_hash.clone(),
        branch_independent_semantic_key,
    })
}

fn branch_formation(
    r_t1_class_id: &str,
    stage4_candidate_hash: &str,
    stage4_telescope: &Telescope,
) -> Result<
    (SealedSignature, A3HistoricalWindow, BranchStage5Formation),
    Rt2FutureHoleConfluenceError,
> {
    if candidate_hash(stage4_telescope) != stage4_candidate_hash {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "R-T1 branch telescope does not hash to its certified candidate".to_owned(),
        ));
    }
    let signature = prefix_signature(stage4_telescope);
    let window = generate_a3_window_for_prefix(&signature, 5)
        .map_err(|error| Rt2FutureHoleConfluenceError::Prerequisite(error.to_string()))?;
    if window.stage != 5 || window.newest_step != Some(4) || window.older_step != Some(3) {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "branch generator did not return the exact Stage-5 (Step-4, Step-3) window".to_owned(),
        ));
    }
    let mut future_holes = issue_structural_future_holes_for_window(&signature, &window)
        .map_err(|error| Rt2FutureHoleConfluenceError::Prerequisite(error.to_string()))?;
    if future_holes.len() != 1 {
        return Err(Rt2FutureHoleConfluenceError::Invariant(format!(
            "Stage-5 branch issued {} structural future holes instead of one",
            future_holes.len()
        )));
    }
    let structural_future_hole = structural_future_hole(
        &window,
        &future_holes
            .pop()
            .expect("the exact singleton future-hole length was checked"),
    )?;
    let mut schemes = window
        .schemes
        .iter()
        .map(|scheme| scheme_formation(&window, scheme))
        .collect::<Result<Vec<_>, _>>()?;
    schemes.sort_by(|left, right| {
        left.semantic_formation_class_id
            .cmp(&right.semantic_formation_class_id)
            .then(left.local_scheme_id.cmp(&right.local_scheme_id))
    });
    let every_scheme_has_formation_evidence = schemes
        .iter()
        .all(|scheme| !scheme.formation_derivation_hash.is_empty());
    let scheme_formation_class_ids = schemes
        .iter()
        .map(|scheme| scheme.semantic_formation_class_id.clone())
        .collect::<Vec<_>>();
    let formation_set_hash =
        tagged_hash("branch-stage5-formation-set", &scheme_formation_class_ids);
    let branch = BranchStage5Formation {
        r_t1_class_id: r_t1_class_id.to_owned(),
        stage4_candidate_hash: stage4_candidate_hash.to_owned(),
        stage4_telescope: stage4_telescope.clone(),
        prefix_signature_digest: signature.digest().to_owned(),
        prefix_steps_exactly_one_through_four: signature
            .entries()
            .iter()
            .map(|entry| entry.step)
            .eq(1..=4),
        stage5_window_derivation_hash: window.window_derivation_hash.clone(),
        stage5_newest_step: 4,
        stage5_older_step: 3,
        constructor_predicate: constructor_predicate(&window)?,
        structural_future_hole,
        every_instance_typed: window.every_instance_typed,
        every_scheme_has_formation_evidence,
        schemes,
        scheme_formation_class_ids,
        formation_set_hash,
    };
    Ok((signature, window, branch))
}

fn compare_family(
    left: &SchemeFamilyPresentation,
    right: &SchemeFamilyPresentation,
) -> Result<FamilyQuotientComparison, Rt2FutureHoleConfluenceError> {
    let parameter_sorts_equal = left.parameter_sorts_json == right.parameter_sorts_json;
    let kernel_types_equal = left.kernel_type_json == right.kernel_type_json;
    let scope_len = left
        .canonical_normal_form
        .var_refs()
        .into_iter()
        .chain(right.canonical_normal_form.var_refs())
        .max()
        .unwrap_or(0);
    let frozen_equality = FrozenEqualityRecord::from(
        univalent_equality(
            &left.canonical_normal_form,
            &right.canonical_normal_form,
            scope_len,
            128,
        )
        .map_err(|error| Rt2FutureHoleConfluenceError::Invariant(error.to_string()))?,
    );
    let equivalent = parameter_sorts_equal && kernel_types_equal && frozen_equality.equal;
    Ok(FamilyQuotientComparison {
        left_source_anchor_audit_only: left.source_anchor_audit_only.clone(),
        right_source_anchor_audit_only: right.source_anchor_audit_only.clone(),
        parameter_sorts_equal,
        kernel_types_equal,
        frozen_equality,
        equivalent,
    })
}

fn compare_scheme(
    left: &Stage5SchemeFormation,
    right: &Stage5SchemeFormation,
) -> Result<SchemeFormationComparison, Rt2FutureHoleConfluenceError> {
    let rule_constructor_equal = left.rule_constructor == right.rule_constructor;
    let origin_kind_equal = left.origin_kind == right.origin_kind;
    let support_depth_equal = left.support_depth == right.support_depth;
    let source_arity_equal = left.source_families.len() == right.source_families.len();
    let source_family_comparisons = if source_arity_equal {
        left.source_families
            .iter()
            .zip(&right.source_families)
            .map(|(left, right)| compare_family(left, right))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    let output_shape_equal = left.output_shape_json == right.output_shape_json;
    let equivalent_under_certified_family_quotient = rule_constructor_equal
        && origin_kind_equal
        && support_depth_equal
        && source_arity_equal
        && source_family_comparisons
            .iter()
            .all(|comparison| comparison.equivalent)
        && output_shape_equal;
    let mut comparison = SchemeFormationComparison {
        left_scheme_id: left.local_scheme_id.clone(),
        right_scheme_id: right.local_scheme_id.clone(),
        rule_constructor_equal,
        origin_kind_equal,
        support_depth_equal,
        source_arity_equal,
        source_family_comparisons,
        output_shape_equal,
        equivalent_under_certified_family_quotient,
        derivation_hash: String::new(),
    };
    comparison.derivation_hash = tagged_hash("scheme-formation-comparison", &comparison);
    Ok(comparison)
}

fn augment_matching(
    left: usize,
    edges: &[Vec<usize>],
    seen_right: &mut [bool],
    right_to_left: &mut [Option<usize>],
) -> bool {
    for &right in &edges[left] {
        if seen_right[right] {
            continue;
        }
        seen_right[right] = true;
        if right_to_left[right].is_none()
            || augment_matching(
                right_to_left[right].expect("checked present"),
                edges,
                seen_right,
                right_to_left,
            )
        {
            right_to_left[right] = Some(left);
            return true;
        }
    }
    false
}

fn compare_scheme_sets(
    left: &BranchStage5Formation,
    right: &BranchStage5Formation,
) -> Result<PairwiseStage5SchemeSetComparison, Rt2FutureHoleConfluenceError> {
    let mut comparisons = Vec::new();
    let mut edges = vec![Vec::new(); left.schemes.len()];
    for (left_index, left_scheme) in left.schemes.iter().enumerate() {
        for (right_index, right_scheme) in right.schemes.iter().enumerate() {
            let comparison = compare_scheme(left_scheme, right_scheme)?;
            if comparison.equivalent_under_certified_family_quotient {
                edges[left_index].push(right_index);
            }
            comparisons.push(comparison);
        }
    }
    let mut right_to_left = vec![None; right.schemes.len()];
    let mut matched = 0usize;
    for left_index in 0..left.schemes.len() {
        let mut seen_right = vec![false; right.schemes.len()];
        if augment_matching(left_index, &edges, &mut seen_right, &mut right_to_left) {
            matched += 1;
        }
    }
    let mut perfect_matching = right_to_left
        .iter()
        .enumerate()
        .filter_map(|(right_index, left_index)| {
            left_index.map(|left_index| {
                (
                    left.schemes[left_index].local_scheme_id.clone(),
                    right.schemes[right_index].local_scheme_id.clone(),
                )
            })
        })
        .collect::<Vec<_>>();
    perfect_matching.sort();
    let full_scheme_sets_equivalent = left.schemes.len() == right.schemes.len()
        && matched == left.schemes.len()
        && matched == right.schemes.len();
    let left_classes = left
        .scheme_formation_class_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let right_classes = right
        .scheme_formation_class_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let unmatched_left_formation_classes = left_classes
        .difference(&right_classes)
        .cloned()
        .collect::<Vec<_>>();
    let unmatched_right_formation_classes = right_classes
        .difference(&left_classes)
        .cloned()
        .collect::<Vec<_>>();
    let comparison_well_formed = left.every_instance_typed
        && right.every_instance_typed
        && left.every_scheme_has_formation_evidence
        && right.every_scheme_has_formation_evidence
        && left
            .structural_future_hole
            .hypothetical_derivation_replayable
        && right
            .structural_future_hole
            .hypothetical_derivation_replayable
        && comparisons
            .iter()
            .all(|comparison| !comparison.derivation_hash.is_empty());
    let mut result = PairwiseStage5SchemeSetComparison {
        left_candidate_hash: left.stage4_candidate_hash.clone(),
        right_candidate_hash: right.stage4_candidate_hash.clone(),
        left_scheme_count: left.schemes.len(),
        right_scheme_count: right.schemes.len(),
        comparisons,
        perfect_matching,
        unmatched_left_formation_classes,
        unmatched_right_formation_classes,
        full_scheme_sets_equivalent,
        comparison_well_formed,
        hash_or_enumeration_order_used_as_selector: false,
        derivation_hash: String::new(),
    };
    result.derivation_hash = tagged_hash("pairwise-stage5-scheme-set-comparison", &result);
    Ok(result)
}

fn build_branch_formations(
    transport: &NaturalityOrbitTransportCertificate,
) -> Result<
    Vec<(SealedSignature, A3HistoricalWindow, BranchStage5Formation)>,
    Rt2FutureHoleConfluenceError,
> {
    let class_by_candidate = transport
        .stage4
        .orbit_classes
        .iter()
        .flat_map(|class| {
            class
                .member_candidate_hashes
                .iter()
                .map(move |candidate| (candidate.clone(), class.class_id.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    let mut branches = transport
        .stage4
        .packages
        .iter()
        .map(|package| {
            let class_id = class_by_candidate
                .get(&package.candidate_hash)
                .ok_or_else(|| {
                    Rt2FutureHoleConfluenceError::Invariant(format!(
                        "R-T1 package {} has no semantic class",
                        package.candidate_hash
                    ))
                })?;
            branch_formation(class_id, &package.candidate_hash, &package.telescope)
        })
        .collect::<Result<Vec<_>, _>>()?;
    branches.sort_by(|left, right| {
        left.2
            .stage4_candidate_hash
            .cmp(&right.2.stage4_candidate_hash)
    });
    if branches.len() != 4
        || branches
            .iter()
            .any(|(_, _, branch)| !branch.prefix_steps_exactly_one_through_four)
    {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "R-T2 did not reconstruct four exact Step-1-through-Step-4 branch prefixes".to_owned(),
        ));
    }
    Ok(branches)
}

fn all_pairwise_scheme_set_comparisons(
    branches: &[BranchStage5Formation],
) -> Result<Vec<PairwiseStage5SchemeSetComparison>, Rt2FutureHoleConfluenceError> {
    let mut comparisons = Vec::new();
    for left in 0..branches.len() {
        for right in (left + 1)..branches.len() {
            comparisons.push(compare_scheme_sets(&branches[left], &branches[right])?);
        }
    }
    Ok(comparisons)
}

fn certificate_digest(certificate: &Rt2FutureHoleConfluenceCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

pub fn issue_r_t2_future_hole_confluence_certificate()
-> Result<Rt2FutureHoleConfluenceCertificate, Rt2FutureHoleConfluenceError> {
    replay_adoptions()?;

    let transport = issue_naturality_orbit_transport_certificate()
        .map_err(|error| Rt2FutureHoleConfluenceError::Prerequisite(error.to_string()))?;
    let transport_replay = replay_naturality_orbit_transport_certificate(&transport);
    let predecessor: TbfTieProtocolRerunCertificate =
        issue_t_bf_tie_protocol_rerun_certificate()
            .map_err(|error| Rt2FutureHoleConfluenceError::Prerequisite(error.to_string()))?;
    let predecessor_replay = replay_t_bf_tie_protocol_rerun_certificate(&predecessor);
    let t_bf1 = issue_t_bf1_prefix_certificate()
        .map_err(|error| Rt2FutureHoleConfluenceError::Prerequisite(error.to_string()))?;
    let t_bf1_replay = replay_t_bf1_prefix_certificate(&t_bf1);
    if !transport_replay.valid || !predecessor_replay.valid || !t_bf1_replay.valid {
        return Err(Rt2FutureHoleConfluenceError::Prerequisite(format!(
            "live replay failed: transport={:?}; predecessor={:?}; T-BF1={:?}",
            transport_replay.errors, predecessor_replay.errors, t_bf1_replay.errors
        )));
    }

    let predecessor_was_exact_r_t2_typed_output_blocker = predecessor
        .stage4
        .exact_four_way_minimizer_hash_set_agreement
        && predecessor.stage4.r_t1_semantic_class_count == 4
        && predecessor.stage4.r_t2_confluence_required
        && predecessor.r_t2.rung_entered
        && predecessor.r_t2.blocked_before_semantic_branch_comparison
        && !predecessor.r_t2.confluence_proved
        && !predecessor.r_t2.confluence_refuted
        && !predecessor.r_t3_user_adjudication_opened
        && predecessor.stage4.selected_candidate_hash.is_none();
    if !predecessor_was_exact_r_t2_typed_output_blocker {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "predecessor is not the exact fail-closed R-T2 typed-output stop".to_owned(),
        ));
    }

    let t_bf1_stage4 = t_bf1
        .stages
        .iter()
        .find(|stage| stage.stage == 4)
        .ok_or_else(|| Rt2FutureHoleConfluenceError::Invariant("T-BF1 omits Stage 4".to_owned()))?;
    let t_bf1_minimizers = t_bf1_stage4
        .minimizer_hashes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let transport_minimizers = transport
        .stage4
        .packages
        .iter()
        .map(|package| package.candidate_hash.clone())
        .collect::<BTreeSet<_>>();
    let predecessor_minimizers = predecessor
        .stage4
        .r_t1_package_hashes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let exact_four_way_stage4_minimizer_join = t_bf1_minimizers.len() == 4
        && t_bf1_minimizers == transport_minimizers
        && t_bf1_minimizers == predecessor_minimizers;
    if !exact_four_way_stage4_minimizer_join {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "T-BF1, predecessor, and R-T1 do not join to the same four minimizers".to_owned(),
        ));
    }
    let same_stage4_live_demand_proved = t_bf1_stage4.required_packages
        == vec!["former_eliminator".to_owned()]
        && t_bf1_stage4.engine_focus_family == "Some(FormerEliminator)";
    let every_stage4_minimizer_strict_total_discharger = t_bf1_minimizers.iter().all(|hash| {
        t_bf1_stage4
            .candidates_seen_by_parsimony
            .iter()
            .find(|candidate| candidate.candidate_hash == *hash)
            .is_some_and(|candidate| candidate.strict_total_discharger)
    });
    if !same_stage4_live_demand_proved || !every_stage4_minimizer_strict_total_discharger {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "Stage-4 minimizers do not share the certified live demand/discharger premise"
                .to_owned(),
        ));
    }
    if transport.stage4.semantic_class_count != 4
        || transport.stage4.orbit_classes.len() != 4
        || transport
            .stage4
            .orbit_classes
            .iter()
            .any(|class| class.member_candidate_hashes.len() != 1)
        || transport.stage4.r_t1_dissolves_tie
        || !transport.stage4.r_t2_required
    {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "transport predecessor does not expose four singleton R-T1 classes".to_owned(),
        ));
    }

    let branch_records = build_branch_formations(&transport)?;
    let branches = branch_records
        .into_iter()
        .map(|(_, _, branch)| branch)
        .collect::<Vec<_>>();
    let distinct_prefix_digests = branches
        .iter()
        .map(|branch| branch.prefix_signature_digest.clone())
        .collect::<BTreeSet<_>>();
    let distinct_window_derivations = branches
        .iter()
        .map(|branch| branch.stage5_window_derivation_hash.clone())
        .collect::<BTreeSet<_>>();
    let every_branch_is_independent_exact_prefix_generation = branches.len() == 4
        && distinct_prefix_digests.len() == 4
        && distinct_window_derivations.len() == 4
        && branches.iter().all(|branch| {
            branch.prefix_steps_exactly_one_through_four
                && branch.stage5_newest_step == 4
                && branch.stage5_older_step == 3
                && branch.every_instance_typed
                && branch.every_scheme_has_formation_evidence
                && !branch.schemes.is_empty()
        });
    if !every_branch_is_independent_exact_prefix_generation {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "Stage-5 scheme sets were not independently generated over four exact prefixes"
                .to_owned(),
        ));
    }

    let first_constructor = branches.first().map(|branch| {
        (
            &branch.constructor_predicate.constructor,
            &branch.constructor_predicate.structural_snapshot_json,
            &branch.constructor_predicate.premises_json,
        )
    });
    let constructor_predicates_all_equal = first_constructor.is_some()
        && branches.iter().all(|branch| {
            first_constructor
                == Some((
                    &branch.constructor_predicate.constructor,
                    &branch.constructor_predicate.structural_snapshot_json,
                    &branch.constructor_predicate.premises_json,
                ))
        });
    let structural_future_holes_all_issued_and_replayable = branches.iter().all(|branch| {
        let hole = &branch.structural_future_hole;
        hole.structural_constructor == "initial_hit"
            && hole.body == Expr::Var(1)
            && hole.open_judgment_kernel_typed
            && hole.every_motive_bn_formable
            && hole.every_term_node_in_adopted_closure
            && hole.hypothetical_derivation_replayable
            && hole.d_membership_issued
            && hole.zero_kappa_nu_anchor_orbit_and_credit
            && hole.named_gap.is_none()
    });
    let structural_future_hole_semantic_keys = branches
        .iter()
        .map(|branch| {
            branch
                .structural_future_hole
                .branch_independent_semantic_key
                .clone()
        })
        .collect::<BTreeSet<_>>();
    let structural_future_hole_semantic_keys_all_equal =
        structural_future_hole_semantic_keys.len() == 1;
    if !constructor_predicates_all_equal
        || !structural_future_holes_all_issued_and_replayable
        || !structural_future_hole_semantic_keys_all_equal
    {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "branch-local future hypotheses do not share the adopted structural predicate"
                .to_owned(),
        ));
    }

    let pairwise_scheme_set_comparisons = all_pairwise_scheme_set_comparisons(&branches)?;
    let every_pairwise_comparison_well_formed = pairwise_scheme_set_comparisons.len() == 6
        && pairwise_scheme_set_comparisons
            .iter()
            .all(|comparison| comparison.comparison_well_formed);
    if !every_pairwise_comparison_well_formed {
        return Err(Rt2FutureHoleConfluenceError::Invariant(
            "the six full scheme-set comparisons are not all well formed".to_owned(),
        ));
    }
    let all_stage5_scheme_sets_equivalent = pairwise_scheme_set_comparisons
        .iter()
        .all(|comparison| comparison.full_scheme_sets_equivalent);
    let immediate_inequivalent_successor_counterexample =
        structural_future_holes_all_issued_and_replayable
            && every_pairwise_comparison_well_formed
            && !all_stage5_scheme_sets_equivalent;
    let deterministic_future_isomorphism_replayed = false;
    let r_t2_confluence_proved =
        all_stage5_scheme_sets_equivalent && deterministic_future_isomorphism_replayed;
    let r_t2_confluence_refuted = immediate_inequivalent_successor_counterexample;
    let r_t3_user_adjudication_opened = r_t2_confluence_refuted;
    let outcome = if r_t2_confluence_refuted {
        Rt2FutureHoleConfluenceOutcome::RefutedByInequivalentStage5SchemeSets
    } else {
        Rt2FutureHoleConfluenceOutcome::Stage5SchemeSetsEquivalentFutureIsomorphismPending
    };
    let (permitted_conclusion, required_successor_action) = match outcome {
        Rt2FutureHoleConfluenceOutcome::RefutedByInequivalentStage5SchemeSets => (
            "R-T2 is refuted by a well-formed immediate successor: although all four branches issue the same zero-charge InitialHit open judgment, at least one pair of independently generated full Stage-5 A3 scheme sets has no perfect matching under the frozen certified family quotient. Constructor-predicate equality alone was not used as confluence. R-T3 is therefore open, but this artifact selects no branch and proves neither T-BF theorem.".to_owned(),
            "Proceed only to the adopted R-T3 user adjudication over the four surviving semantic options. Do not use hash/enumeration order, the desired history, a score, or the bar to select an option; do not run the bridge or issue a final halt certificate from this artifact.".to_owned(),
        ),
        Rt2FutureHoleConfluenceOutcome::Stage5SchemeSetsEquivalentFutureIsomorphismPending => (
            "The immediate full Stage-5 scheme sets are equivalent under the frozen family quotient, but R-T2 confluence is not yet proved: deterministic future-isomorphism and continuation preservation have not been replayed. R-T3 remains closed.".to_owned(),
            "Construct and replay the deterministic future-isomorphism/continuation implication for the equivalent immediate successor sets. Keep R-T3 closed unless a later well-formed comparison refutes confluence.".to_owned(),
        ),
    };

    let mut certificate = Rt2FutureHoleConfluenceCertificate {
        schema: R_T2_FUTURE_HOLE_CONFLUENCE_SCHEMA.to_owned(),
        date: R_T2_FUTURE_HOLE_CONFLUENCE_DATE.to_owned(),
        source_bindings: source_bindings(),
        future_hole_adoption_replayed: true,
        tie_protocol_adoption_replayed: true,
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        naturality_orbit_transport_certificate_digest: transport.result_digest,
        naturality_orbit_transport_replay_valid: true,
        predecessor_tie_rerun_certificate_digest: predecessor.result_digest,
        predecessor_tie_rerun_replay_valid: true,
        predecessor_was_exact_r_t2_typed_output_blocker,
        t_bf1_prefix_certificate_digest: t_bf1.result_digest,
        t_bf1_prefix_replay_valid: true,
        exact_four_way_stage4_minimizer_join,
        same_stage4_live_demand_proved,
        stage4_required_packages: t_bf1_stage4.required_packages.clone(),
        every_stage4_minimizer_strict_total_discharger,
        r_t1_semantic_class_count: transport.stage4.semantic_class_count,
        branch_count: branches.len(),
        branches,
        every_branch_is_independent_exact_prefix_generation,
        constructor_predicates_all_equal,
        constructor_predicate_equality_used_as_confluence: false,
        structural_future_holes_all_issued_and_replayable,
        structural_future_hole_semantic_keys_all_equal,
        pairwise_scheme_set_comparisons,
        every_pairwise_comparison_well_formed,
        all_stage5_scheme_sets_equivalent,
        immediate_inequivalent_successor_counterexample,
        deterministic_future_isomorphism_replayed,
        r_t2_confluence_proved,
        r_t2_confluence_refuted,
        r_t3_user_adjudication_opened,
        selected_candidate_hash: None,
        hash_or_enumeration_order_used_as_selector: false,
        desired_history_count_score_or_bar_used_as_premise: false,
        theorem_t_bf1_proved: false,
        theorem_t_bf3_proved: false,
        bar_free_adoption_authorized: false,
        bridge_authorized: false,
        halt_claim_issued: false,
        outcome,
        permitted_conclusion,
        required_successor_action,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: String) -> Rt2FutureHoleConfluenceReplay {
    Rt2FutureHoleConfluenceReplay {
        valid: false,
        outcome: None,
        branch_count: 0,
        structural_future_holes_all_issued_and_replayable: false,
        constructor_predicates_all_equal: false,
        pairwise_comparison_count: 0,
        every_pairwise_comparison_well_formed: false,
        all_stage5_scheme_sets_equivalent: false,
        r_t2_confluence_proved: false,
        r_t2_confluence_refuted: false,
        r_t3_user_adjudication_opened: false,
        selected_candidate_hash: None,
        errors: vec![error],
    }
}

pub fn replay_r_t2_future_hole_confluence_certificate(
    certificate: &Rt2FutureHoleConfluenceCertificate,
) -> Rt2FutureHoleConfluenceReplay {
    let expected = match issue_r_t2_future_hole_confluence_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate != &expected {
        errors.push("certificate differs from independent live R-T2 rerun".to_owned());
    }
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    Rt2FutureHoleConfluenceReplay {
        valid: errors.is_empty(),
        outcome: Some(certificate.outcome.clone()),
        branch_count: certificate.branch_count,
        structural_future_holes_all_issued_and_replayable: certificate
            .structural_future_holes_all_issued_and_replayable,
        constructor_predicates_all_equal: certificate.constructor_predicates_all_equal,
        pairwise_comparison_count: certificate.pairwise_scheme_set_comparisons.len(),
        every_pairwise_comparison_well_formed: certificate.every_pairwise_comparison_well_formed,
        all_stage5_scheme_sets_equivalent: certificate.all_stage5_scheme_sets_equivalent,
        r_t2_confluence_proved: certificate.r_t2_confluence_proved,
        r_t2_confluence_refuted: certificate.r_t2_confluence_refuted,
        r_t3_user_adjudication_opened: certificate.r_t3_user_adjudication_opened,
        selected_candidate_hash: certificate.selected_candidate_hash.clone(),
        errors,
    }
}

pub fn emit_r_t2_future_hole_confluence_create_new(
    path: &Path,
) -> Result<Rt2FutureHoleConfluenceReplay, Rt2FutureHoleConfluenceError> {
    let certificate = issue_r_t2_future_hole_confluence_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Rt2FutureHoleConfluenceError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Rt2FutureHoleConfluenceError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Rt2FutureHoleConfluenceError::Io(error.to_string()))?;
    let replay = replay_r_t2_future_hole_confluence_certificate(&certificate);
    if !replay.valid {
        return Err(Rt2FutureHoleConfluenceError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> Rt2FutureHoleConfluenceCertificate {
        static CERTIFICATE: OnceLock<Rt2FutureHoleConfluenceCertificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_r_t2_future_hole_confluence_certificate().unwrap())
            .clone()
    }

    #[test]
    fn full_stage5_scheme_sets_supply_the_only_r_t2_verdict() {
        let certificate = certificate();
        assert_eq!(certificate.branch_count, 4);
        assert_eq!(certificate.r_t1_semantic_class_count, 4);
        assert!(certificate.exact_four_way_stage4_minimizer_join);
        assert!(certificate.same_stage4_live_demand_proved);
        assert!(certificate.every_stage4_minimizer_strict_total_discharger);
        assert!(certificate.every_branch_is_independent_exact_prefix_generation);
        assert!(certificate.constructor_predicates_all_equal);
        assert!(!certificate.constructor_predicate_equality_used_as_confluence);
        assert!(certificate.structural_future_holes_all_issued_and_replayable);
        assert!(certificate.structural_future_hole_semantic_keys_all_equal);
        assert!(
            certificate
                .branches
                .iter()
                .all(|branch| branch.schemes.len() == 5)
        );
        assert_eq!(certificate.pairwise_scheme_set_comparisons.len(), 6);
        assert!(certificate.every_pairwise_comparison_well_formed);
        assert!(!certificate.hash_or_enumeration_order_used_as_selector);
        assert!(certificate.selected_candidate_hash.is_none());
        assert!(!certificate.theorem_t_bf1_proved);
        assert!(!certificate.theorem_t_bf3_proved);
        assert!(!certificate.bar_free_adoption_authorized);
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.halt_claim_issued);
        let replay = replay_r_t2_future_hole_confluence_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
    }

    #[test]
    fn a_well_formed_inequivalence_and_only_it_opens_r_t3() {
        let certificate = certificate();
        if certificate.all_stage5_scheme_sets_equivalent {
            assert_eq!(
                certificate.outcome,
                Rt2FutureHoleConfluenceOutcome::Stage5SchemeSetsEquivalentFutureIsomorphismPending
            );
            assert!(!certificate.r_t2_confluence_proved);
            assert!(!certificate.r_t2_confluence_refuted);
            assert!(!certificate.r_t3_user_adjudication_opened);
        } else {
            assert_eq!(
                certificate.outcome,
                Rt2FutureHoleConfluenceOutcome::RefutedByInequivalentStage5SchemeSets
            );
            assert!(certificate.immediate_inequivalent_successor_counterexample);
            assert!(!certificate.r_t2_confluence_proved);
            assert!(certificate.r_t2_confluence_refuted);
            assert!(certificate.r_t3_user_adjudication_opened);
        }
    }

    #[test]
    fn replay_rejects_a_redigested_forged_confluence_or_selection() {
        let mut forged = certificate();
        forged.r_t2_confluence_proved = true;
        forged.r_t2_confluence_refuted = false;
        forged.r_t3_user_adjudication_opened = false;
        forged.selected_candidate_hash = forged
            .branches
            .first()
            .map(|branch| branch.stage4_candidate_hash.clone());
        forged.theorem_t_bf1_proved = true;
        forged.theorem_t_bf3_proved = true;
        forged.bar_free_adoption_authorized = true;
        forged.bridge_authorized = true;
        forged.halt_claim_issued = true;
        forged.result_digest = certificate_digest(&forged);
        assert!(!replay_r_t2_future_hole_confluence_certificate(&forged).valid);
    }
}
