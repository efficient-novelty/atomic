//! Branch-parametric E-5-class finale used by the BI program.
//!
//! Unlike the historical E-5 issuer, every prefix, filler, demand window, and
//! D-membership row is reconstructed from an explicit certified branch.  No
//! Genesis reference telescope or enacted winner is available to this API.

use crate::act_local_provenance::{T_BI_NU1_THEOREM_ID, issue_act_local_provenance};
use crate::branch_invariance::{
    BranchContinuation, BranchContinuationOutcome, BranchNuProvenanceDisposition,
    CertifiedStage4BranchCone, replay_branch_continuation,
};
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3DemandOutputType, A3HistoricalWindow, A3RuleConstructor,
    A3TypedClauseSource, A3TypedDemandInstance, A3TypedDemandScheme,
    generate_a3_window_for_exact_prefix_unbounded, replay_chronological_interface_slot_map,
};
use pen_eval::a3_rule_inventory_exhaustiveness::{
    A3WindowRuleInventoryProof, prove_a3_window_inventory_for_exact_prefix_unbounded,
};
use pen_eval::future_hole_hypothesis_v2 as future_v2;
use pen_eval::typed_families::{
    MarginalityDisposition, ParamSort, extract_candidate_families, predecessor_closure,
};
use pen_type::contextual_internality::{ContextualMotive, issue_ambient_context_declaration_token};
use pen_type::elaborate::{
    KernelTy, SealedSignature, candidate_hash, elaborate_single_clause_with_typed_ambient,
    elaborate_telescope,
};
use pen_type::equality::univalent_equality;
use pen_type::motive_parametric_coherence_v2::{
    CLOSURE_RULE_INVENTORY_V2, ClosedInternalEvidenceV2, MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION,
    issue_actual_body_closure_derivation_v2, issue_closed_internal_evidence_v2,
    issue_motive_typed_closed_assignment_v2, replay_closed_internal_evidence_v2,
    replay_motive_typed_closed_assignment_v2, replay_specialized_closure_derivation_v2,
    replay_verified_closure_derivation_v2, specialize_verified_closure_derivation_v2,
};
use pen_type::substitution::{
    ParameterSort, SortedParameterContext, SubstitutionImage, is_well_scoped,
    issue_structural_substitution, replay_structural_substitution,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const BI_BRANCH_FINALE_SCHEMA: &str = "bi-branch-finale-v2";
pub const BI_CHRONOLOGICAL_INTERFACE_SLOT_MAP_UNDECLARED_V1: &str =
    "BI_CHRONOLOGICAL_INTERFACE_SLOT_MAP_UNDECLARED_V1";
pub const BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2: &str =
    "BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2";
pub const BI_CHRONOLOGICAL_ASSIGNMENT_IMAGE_NOT_CLOSED_V2: &str =
    "BI_CHRONOLOGICAL_ASSIGNMENT_IMAGE_NOT_CLOSED_V2";
pub const BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2: &str =
    "BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2";
pub const BI_CHRONOLOGICAL_SPECIALIZATION_UNAVAILABLE_V2: &str =
    "BI_CHRONOLOGICAL_SPECIALIZATION_UNAVAILABLE_V2";
pub const BI_CHRONOLOGICAL_SPECIALIZED_RESULT_MISMATCH_V2: &str =
    "BI_CHRONOLOGICAL_SPECIALIZED_RESULT_MISMATCH_V2";
pub const BI_STRUCTURAL_FILLER_ORDINARY_CHARGE_PROVENANCE_UNCERTIFIED_V1: &str =
    "BI_STRUCTURAL_FILLER_ORDINARY_CHARGE_PROVENANCE_UNCERTIFIED_V1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchFinaleStepInput {
    pub stage: u32,
    pub telescope: Telescope,
    pub candidate_hash: String,
    pub kappa: u32,
    pub certified_nu: u32,
    pub required_packages_before_selection: Vec<String>,
    pub step_evidence_hash: String,
    pub ordinary_family_token_hashes: Vec<String>,
    /// True only when this branch carries exact candidate-local nu and
    /// ordinary-family token provenance. A diagnostic formula or a post-seal
    /// numerical match cannot authorize ordinary filler charge.
    pub ordinary_charge_provenance_authoritative: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchFinaleInput {
    pub branch_root_hash: String,
    pub continuation_digest: String,
    pub lawful_engine_stage_limit: u32,
    pub halt_step: u32,
    pub terminal_required_packages: Vec<String>,
    pub steps: Vec<BranchFinaleStepInput>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchA3WindowAudit {
    pub stage: u32,
    pub exact_prefix_signature_digest: String,
    pub window_derivation_hash: String,
    pub inventory_proof_hash: String,
    pub relative_inventory_exhaustive: bool,
    pub independent_preimage_ids_injective: bool,
    pub future_hole_typing_deferred_to_branch_v2: bool,
    pub instance_count: usize,
    pub required_packages: Vec<String>,
    pub structural_constructors: Vec<String>,
    pub projected_focus: Option<String>,
    pub demand_precedes_jurisdiction: bool,
    pub expected_branch_demand: Vec<String>,
    pub focus_projection_matches_branch_ledger: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchFutureRegistrationAudit {
    pub stage: u32,
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub rule_constructor: String,
    pub natural_family_id: Option<String>,
    pub occurrence_id: Option<String>,
    pub formation_or_gap_hash: String,
    pub deterministic_replay_valid: bool,
    pub external_inventory_join_exact: bool,
    pub dependent_totality_theorem_replayed: bool,
    pub total_specialization_authoritative: bool,
    pub no_outcome_filtering: bool,
    pub no_reflexivity_fallback: bool,
    pub every_hole_live: bool,
    pub local_semantic_scope_satisfied: bool,
    pub marginal_charge_zero: bool,
    pub gap_id: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchStructuralRealizationAudit {
    pub registration_stage: u32,
    pub jurisdiction_stage: u32,
    pub a3_instance_id: String,
    pub constructor: String,
    pub filler_candidate_hash: String,
    pub ordinary_kappa: u32,
    pub certified_nu: u32,
    /// Whether the filler charge came from exact candidate-local ordinary
    /// family provenance rather than a diagnostic formula.
    pub ordinary_charge_provenance_authoritative: bool,
    pub ordinary_charge_joins_branch_step: bool,
    pub deterministic_replay_valid: bool,
    pub provider_relation_satisfied: bool,
    pub constructor_live_before_filler: bool,
    pub constructor_absent_after_filler: bool,
    pub specialization_replayed: bool,
    pub marginal_charge_zero: bool,
    pub ordinary_charge_preserved: bool,
    pub wrong_provider_rejected: bool,
    pub charge_swap_rejected_by_authoritative_join: bool,
    pub realization_or_gap_hash: String,
    pub gap_id: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchDMembershipDisposition {
    Derivable,
    Underdetermined,
    /// The issuer could not lawfully classify this row.  This is not a
    /// semantic `D = Underdetermined` verdict and therefore cannot execute
    /// F1 until the named expressivity gap is discharged.
    IssuerGap,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchDMembershipAudit {
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub rule_constructor: String,
    pub evidence_hash: String,
    pub hypothetical_derivation_replayed: bool,
    pub output_kernel_typed: bool,
    pub internal_closure_preimage: bool,
    pub internality_rule_id: String,
    /// The narrower candidate-credit classifier may conservatively decline a
    /// non-rigid preimage even when the explicit typed-kernel specialization
    /// rule below replays one.  Its answer is retained, not overwritten.
    pub restricted_candidate_classifier_reported_marginal: bool,
    pub exact_substitution_from_sealed_preimage_replayed: bool,
    pub uniform_specialization_not_new_family: bool,
    pub independently_exported_output_orbit: bool,
    pub marginal_charge_zero_or_not_applicable: bool,
    pub source_or_evidence_swap_rejected: bool,
    pub disposition: BranchDMembershipDisposition,
    pub exact_reason: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchF1CounterfactualAudit {
    pub mutated_instance_id: String,
    pub original_membership_hash: String,
    pub counterfactual_membership_hash: String,
    pub counterfactual_membership: Box<BranchDMembershipAudit>,
    pub authoritative_membership_reissue_rejected: bool,
    pub counterfactual_underdetermined_count: usize,
    pub counterfactual_f1_triggered: bool,
    pub counterfactual_semantic_successor_o_empty: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BranchFinaleAudit {
    pub schema: String,
    pub branch_root_hash: String,
    pub continuation_digest: String,
    pub input_binding_hash: String,
    pub continuation_provenance_join_exact: bool,
    pub lawful_engine_stage_limit: u32,
    pub halt_step: u32,
    pub successor_stage: u32,
    pub input_contiguous_and_self_consistent: bool,
    pub windows: Vec<BranchA3WindowAudit>,
    pub every_window_inventory_exhaustive: bool,
    pub inventory_scope: String,
    pub absolute_semantic_exhaustiveness_claimed: bool,
    pub registrations: Vec<BranchFutureRegistrationAudit>,
    pub unary_registration_count: usize,
    pub structural_registration_count: usize,
    pub registration_gap_ids: Vec<String>,
    pub every_registration_replayed_with_dependent_totality: bool,
    pub every_registration_evidence_mutation_rejected: bool,
    pub realizations: Vec<BranchStructuralRealizationAudit>,
    pub structural_realization_count: usize,
    pub realization_gap_ids: Vec<String>,
    pub every_structural_hole_realized: bool,
    pub every_wrong_provider_rejected: bool,
    pub every_charge_swap_rejected: bool,
    pub stage3_wrinkle_reproduced: bool,
    pub focus_projection_reproduces_branch_ladder: bool,
    pub final_a3_inventory_count: usize,
    pub final_unary_count: usize,
    pub final_direct_chronological_count: usize,
    pub final_pointwise_chronological_count: usize,
    pub final_higher_count: usize,
    pub final_structural_count: usize,
    pub membership: Vec<BranchDMembershipAudit>,
    pub exact_d_partition: bool,
    pub every_membership_source_or_evidence_swap_rejected: bool,
    pub derivable_instance_count: usize,
    pub underdetermined_instance_ids: Vec<String>,
    pub issuer_gap_instance_ids: Vec<String>,
    pub semantic_successor_o_empty: bool,
    pub f1_executed: bool,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub f1_counterfactual: Option<BranchF1CounterfactualAudit>,
    pub f1_counterfactual_underdetermination_detected: bool,
    pub theorem12_full_instance_granularity_proved: bool,
    pub theorem12_scope: String,
    pub broader_absolute_theorem12_claimed: bool,
    pub expressivity_gaps: Vec<String>,
    pub e5_class_complete: bool,
    pub derivation_hash: String,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI_BRANCH_FINALE_SCHEMA, domain, value))
        .expect("BI finale evidence serializes");
    format!("blake3:{}", pen_core::hash::blake3_hex(&bytes))
}

/// Stable binding of the complete branch-local finale input.  This binds the
/// exact telescope and continuation evidence for every step; it is not a
/// digest of a score vector alone.
pub fn branch_finale_input_binding_hash(input: &BranchFinaleInput) -> String {
    tagged_hash("branch-finale-input", input)
}

fn bootstrap_step_evidence_hash(
    continuation: &BranchContinuation,
    stage: u32,
    telescope: &Telescope,
) -> String {
    tagged_hash(
        "shared-prefix-step-evidence",
        &(
            continuation.derivation_hash.as_str(),
            stage,
            telescope,
            candidate_hash(telescope),
        ),
    )
}

/// Construct the only certificate-sound input path: the Stage-4 cone and the
/// continuation are replayed first, then each finale row is joined to its
/// exact continuation record.  The shared pre-fork telescopes are reconstructed
/// only for Stages 1--3; no enacted successor is consulted.
pub fn issue_branch_finale_input(
    cone: &CertifiedStage4BranchCone,
    continuation: &BranchContinuation,
) -> Result<BranchFinaleInput, String> {
    let replay_errors = replay_branch_continuation(cone, continuation);
    if !replay_errors.is_empty() {
        return Err(format!(
            "branch continuation did not replay: {}",
            replay_errors.join("; ")
        ));
    }
    let (halt_step, next_stage) = match continuation.outcome {
        BranchContinuationOutcome::DebtFreeHalt {
            halt_stage,
            next_stage,
        } => (halt_stage, next_stage),
        ref outcome => {
            return Err(format!(
                "an E-5 finale requires a debt-free continuation, found {outcome:?}"
            ));
        }
    };
    if next_stage != halt_step.saturating_add(1)
        || continuation.complete_ledger.len() != halt_step as usize
        || !continuation
            .complete_ledger
            .iter()
            .map(|row| row.stage)
            .eq(1..=halt_step)
    {
        return Err("continuation ledger does not exactly cover its declared halt".to_owned());
    }

    let mut telescopes = BTreeMap::<u32, Telescope>::new();
    let common_stem = cone.common_stem.iter().cloned().collect::<BTreeMap<_, _>>();
    if common_stem.len() != 3 || !common_stem.keys().copied().eq(1..=3) {
        return Err("certified cone does not carry the exact three-step shared stem".to_owned());
    }
    for stage in 1..=halt_step {
        let telescope = match stage {
            1..=3 => common_stem
                .get(&stage)
                .expect("validated common stem")
                .clone(),
            4 => continuation.branch.telescope.clone(),
            _ => continuation
                .stages
                .iter()
                .find(|record| record.stage == stage)
                .and_then(|record| record.winner.as_ref())
                .map(|winner| winner.telescope.clone())
                .ok_or_else(|| format!("continuation omits the Stage {stage} winner"))?,
        };
        telescopes.insert(stage, telescope);
    }
    let signature = SealedSignature::from_telescopes(
        telescopes
            .iter()
            .map(|(stage, telescope)| (*stage, telescope.clone()))
            .collect(),
    );
    let mut steps = Vec::new();
    for ledger in &continuation.complete_ledger {
        let telescope = telescopes
            .get(&ledger.stage)
            .expect("complete telescope map")
            .clone();
        if candidate_hash(&telescope) != ledger.candidate_hash
            || telescope.kappa() as u16 != ledger.kappa
        {
            return Err(format!(
                "continuation ledger/telescope join failed at Stage {}",
                ledger.stage
            ));
        }
        let prefix = SealedSignature::from_telescopes(
            (1..ledger.stage)
                .map(|stage| {
                    (
                        stage,
                        telescopes
                            .get(&stage)
                            .expect("earlier exact telescope")
                            .clone(),
                    )
                })
                .collect(),
        );
        let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(&prefix, ledger.stage)
            .map_err(|error| error.to_string())?;
        let (selection_evidence_hash, record_demand) = if ledger.stage <= 3 {
            (
                bootstrap_step_evidence_hash(continuation, ledger.stage, &telescope),
                None,
            )
        } else if ledger.stage == 4 {
            (continuation.branch.seed_derivation_hash.clone(), None)
        } else {
            let record = continuation
                .stages
                .iter()
                .find(|record| record.stage == ledger.stage)
                .ok_or_else(|| format!("continuation omits Stage {}", ledger.stage))?;
            let winner = record
                .winner
                .as_ref()
                .ok_or_else(|| format!("Stage {} has no winner", ledger.stage))?;
            if winner.candidate_hash != ledger.candidate_hash
                || winner.kappa != ledger.kappa
                || winner.semantic_nu != ledger.semantic_nu
            {
                return Err(format!(
                    "winner/ledger join failed at Stage {}",
                    ledger.stage
                ));
            }
            (
                record.derivation_hash.clone(),
                Some(record.demand.a3_required_packages.as_slice()),
            )
        };
        if record_demand.is_some_and(|demand| demand != proof.required_packages_from_raw_debt) {
            return Err(format!(
                "continuation/A3 demand join failed at Stage {}",
                ledger.stage
            ));
        }
        let act_local = issue_act_local_provenance(&prefix, ledger.stage, &telescope)
            .map_err(|error| format!("Stage {} T-BI-NU1 replay: {error}", ledger.stage))?;
        let BranchNuProvenanceDisposition::ExactCertified {
            theorem_id,
            certificate_hash,
            counted_family_ids,
            authoritative_family_token_hashes,
            generated_instance_family_ids_removed_by_quotient,
        } = &ledger.nu_provenance.disposition
        else {
            return Err(format!(
                "Stage {} lacks exact act-local ordinary-charge provenance",
                ledger.stage
            ));
        };
        let ordinary_family_token_hashes = act_local.authoritative_token_hashes();
        let token_hashes_unique = ordinary_family_token_hashes
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            == ordinary_family_token_hashes.len();
        let ordinary_charge_provenance_authoritative = theorem_id == T_BI_NU1_THEOREM_ID
            && certificate_hash == &act_local.derivation_hash
            && counted_family_ids == &act_local.counted_family_ids()
            && authoritative_family_token_hashes == &ordinary_family_token_hashes
            && generated_instance_family_ids_removed_by_quotient
                == &act_local.generated_instance_family_ids_removed_by_quotient
            && act_local.authoritative
            && act_local.candidate_hash == ledger.candidate_hash
            && act_local.predecessor_signature_digest == prefix.digest()
            && act_local.exact_certified_nu == ledger.semantic_nu
            && ordinary_family_token_hashes.len() == ledger.semantic_nu as usize
            && token_hashes_unique;
        if !ordinary_charge_provenance_authoritative {
            return Err(format!(
                "Stage {} act-local ordinary-charge projection failed its exact join",
                ledger.stage
            ));
        }
        let step_evidence_hash = tagged_hash(
            "branch-step-with-act-local-provenance",
            &(
                selection_evidence_hash.as_str(),
                act_local.derivation_hash.as_str(),
            ),
        );
        steps.push(BranchFinaleStepInput {
            stage: ledger.stage,
            telescope,
            candidate_hash: ledger.candidate_hash.clone(),
            kappa: u32::from(ledger.kappa),
            certified_nu: ledger.semantic_nu,
            required_packages_before_selection: proof.required_packages_from_raw_debt,
            step_evidence_hash,
            ordinary_family_token_hashes,
            ordinary_charge_provenance_authoritative,
        });
    }
    let terminal_prefix = SealedSignature::from_telescopes(
        (1..=halt_step)
            .map(|stage| {
                (
                    stage,
                    telescopes
                        .get(&stage)
                        .expect("completed exact telescope")
                        .clone(),
                )
            })
            .collect(),
    );
    if terminal_prefix != signature {
        return Err("internal exact-prefix reconstruction drifted".to_owned());
    }
    let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(&terminal_prefix, next_stage)
        .map_err(|error| error.to_string())?;
    if proof.required_packages_from_raw_debt != continuation.terminal_demand.a3_required_packages {
        return Err("terminal continuation/A3 demand join failed".to_owned());
    }
    let terminal_required_packages = proof.required_packages_from_raw_debt;
    Ok(BranchFinaleInput {
        branch_root_hash: continuation.branch.candidate_hash.clone(),
        continuation_digest: continuation.derivation_hash.clone(),
        lawful_engine_stage_limit: continuation.limits.max_inspected_stage,
        halt_step,
        terminal_required_packages,
        steps,
    })
}

fn exact_prefix(input: &BranchFinaleInput, stage: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        input
            .steps
            .iter()
            .filter(|step| step.stage < stage)
            .map(|step| (step.stage, step.telescope.clone()))
            .collect(),
    )
}

fn step(input: &BranchFinaleInput, stage: u32) -> Result<&BranchFinaleStepInput, String> {
    input
        .steps
        .iter()
        .find(|row| row.stage == stage)
        .ok_or_else(|| format!("branch finale input omits Stage {stage}"))
}

fn scheme_for_instance<'a>(
    window: &'a A3HistoricalWindow,
    instance: &A3TypedDemandInstance,
) -> Result<&'a A3TypedDemandScheme, String> {
    window
        .schemes
        .iter()
        .find(|scheme| scheme.scheme_id == instance.scheme_id)
        .ok_or_else(|| format!("A3 instance {} is orphaned", instance.instance_id))
}

fn rule_name(rule: A3RuleConstructor) -> &'static str {
    match rule {
        A3RuleConstructor::UnaryAction => "unary_action",
        A3RuleConstructor::ChronologicalComparison => "chronological_comparison",
        A3RuleConstructor::HigherOpenBoxReduction => "higher_open_box_reduction",
        A3RuleConstructor::StructuralCompletionHole => "structural_completion_hole",
    }
}

struct BranchChronologicalMembershipProof {
    evidence_hash: String,
    output_kernel_typed: bool,
    internal_closure_preimage: bool,
    internality_rule_id: &'static str,
    restricted_candidate_classifier_reported_marginal: bool,
    exact_substitution_from_sealed_preimage_replayed: bool,
    uniform_specialization_not_new_family: bool,
    independently_exported_output_orbit: bool,
    source_or_evidence_swap_rejected: bool,
    direct: bool,
}

fn source_for_anchor<'a>(
    window: &'a A3HistoricalWindow,
    anchor: &str,
) -> Result<&'a A3TypedClauseSource, String> {
    window
        .typed_sources
        .iter()
        .find(|source| source.anchor_id == anchor)
        .ok_or_else(|| format!("A3 source anchor {anchor} is absent"))
}

fn chronological_sources<'a>(
    window: &'a A3HistoricalWindow,
    instance: &A3TypedDemandInstance,
) -> Result<(&'a A3TypedClauseSource, &'a A3TypedClauseSource), String> {
    if instance.source_anchor_ids.len() != 2 {
        return Err("chronological instance does not have two exact sources".to_owned());
    }
    let older = source_for_anchor(window, &instance.source_anchor_ids[0])?;
    let newest = source_for_anchor(window, &instance.source_anchor_ids[1])?;
    if Some(older.step) != window.older_step
        || Some(newest.step) != window.newest_step
        || !older.exported_public_clause
        || !newest.exported_public_clause
        || older.typing_derivation_hash.is_empty()
        || newest.typing_derivation_hash.is_empty()
    {
        return Err("chronological sources do not join the oriented typed window".to_owned());
    }
    Ok((older, newest))
}

fn chronological_gap(id: &str, phase: &str, reason: impl std::fmt::Display) -> String {
    format!("{id}::{phase}::{reason}")
}

fn motive_for_parameter_sort(sort: &ParamSort) -> ContextualMotive {
    match sort {
        ParamSort::Type => ContextualMotive::Type,
        ParamSort::Opaque => ContextualMotive::Neutral,
    }
}

/// Produce the exact closed `Internal` evidence required for one substitution
/// image.  This helper deliberately has no weakening-to-open fallback: the
/// motive-parametric theorem accepts closed assignments, so an image retaining
/// any target variable is a named theorem-instantiation gap.
fn issue_chronological_closed_image_evidence(
    signature: &SealedSignature,
    visible_library: u32,
    role: pen_core::clause::ClauseRole,
    term: &Expr,
    image_label: &str,
) -> Result<ClosedInternalEvidenceV2, String> {
    if !is_well_scoped(term, 0) {
        return Err(chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_IMAGE_NOT_CLOSED_V2,
            image_label,
            "the exact declared substitution image retains a target-context variable",
        ));
    }
    let candidate = Telescope::new(vec![ClauseRec::new(role, term.clone())]);
    let relation = issue_actual_body_closure_derivation_v2(
        signature,
        &candidate,
        visible_library,
        0,
        None,
        &BTreeMap::new(),
    )
    .map_err(|error| {
        chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
            image_label,
            error,
        )
    })?;
    replay_verified_closure_derivation_v2(signature, relation.projection()).map_err(|error| {
        chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
            image_label,
            error,
        )
    })?;
    let evidence = issue_closed_internal_evidence_v2(signature, &relation).map_err(|error| {
        chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
            image_label,
            error,
        )
    })?;
    replay_closed_internal_evidence_v2(signature, evidence.projection()).map_err(|error| {
        chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
            image_label,
            error,
        )
    })?;
    if evidence.projection().expression != *term
        || evidence.projection().relation.expression != *term
        || evidence.projection().relation.candidate != candidate
    {
        return Err(chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
            image_label,
            "closed evidence does not bind the exact declared image",
        ));
    }
    Ok(evidence.projection().clone())
}

/// Construct one chronological D-membership directly from the exact branch
/// window.  This deliberately consumes neither the frozen J3 projection nor
/// the enacted naturality/D archive.  The declared order-preserving interface
/// map is composed with the exact older-interface substitution, and Internal
/// is issued only when the proof-strength motive-parametric eliminator accepts
/// every exact image as closed Internal evidence and replays to the demanded
/// output. No permutation is inferred or trialed.
fn prove_branch_chronological_membership(
    signature: &SealedSignature,
    visible_library: u32,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
) -> Result<BranchChronologicalMembershipProof, String> {
    let (older, newest) = chronological_sources(window, instance)?;
    let reversed_instance = A3TypedDemandInstance {
        source_anchor_ids: vec![newest.anchor_id.clone(), older.anchor_id.clone()],
        ..instance.clone()
    };
    let source_order_negative_control = chronological_sources(window, &reversed_instance).is_err();
    let (older_family, older_type, newest_family, newest_type, interface_mode, interface_slot_map) =
        match &scheme.required_output {
            A3DemandOutputType::ChronologicalInteraction {
                older_family,
                older_type,
                newest_family,
                newest_type,
                interface_mode,
                interface_slot_map,
            } => (
                older_family,
                older_type,
                newest_family,
                newest_type,
                interface_mode,
                interface_slot_map,
            ),
            _ => return Err("chronological scheme has a non-chronological output".to_owned()),
        };
    if older_family != &older.canonical_family_key
        || older_type != &older.kernel_type
        || newest_family != &newest.canonical_family_key
        || newest_type != &newest.kernel_type
        || instance.source_family_keys
            != vec![
                older.canonical_family_key.clone(),
                newest.canonical_family_key.clone(),
            ]
    {
        return Err("chronological output does not exactly bind its typed sources".to_owned());
    }
    let expected_mode = match &older.kernel_type {
        KernelTy::Type => A3ChronologicalInterfaceMode::DirectType,
        KernelTy::Fun(domain, codomain) if matches!(codomain.as_ref(), KernelTy::Type) => {
            A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction {
                domain: domain.as_ref().clone(),
            }
        }
        _ => return Err("older chronological interface is not Type-valued".to_owned()),
    };
    if interface_mode != &expected_mode {
        return Err("chronological interface-mode evidence drifted".to_owned());
    }
    let source_arity = u32::try_from(newest.canonical_presentation.parameters.len())
        .map_err(|_| "newest canonical arity does not fit u32".to_owned())?;
    let target_arity = u32::try_from(older.canonical_presentation.parameters.len())
        .map_err(|_| "older canonical arity does not fit u32".to_owned())?;
    replay_chronological_interface_slot_map(interface_slot_map, source_arity)
        .map_err(|error| format!("chronological interface slot-map replay: {error}"))?;
    if source_arity == 0 {
        return Err(chronological_gap(
            BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2,
            "source-interface",
            "chronological specialization has no declared parameter for its older interface",
        ));
    }
    if newest.canonical_presentation.parameters.first() != Some(&ParamSort::Type) {
        return Err(chronological_gap(
            BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2,
            "source-interface",
            "chronological substitution image is not assigned to a declared Type parameter",
        ));
    }
    let source_motives = newest
        .canonical_presentation
        .parameters
        .iter()
        .map(motive_for_parameter_sort)
        .collect::<Vec<_>>();
    let source_candidate = Telescope::new(vec![ClauseRec::new(
        newest.kernel_role,
        newest.canonical_presentation.canonical_normal_form.clone(),
    )]);
    let source_declaration = issue_ambient_context_declaration_token(
        signature,
        &source_candidate,
        visible_library,
        source_motives.clone(),
    )
    .map_err(|error| {
        chronological_gap(
            BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2,
            "source-declaration",
            error,
        )
    })?;
    let source_derivation = issue_actual_body_closure_derivation_v2(
        signature,
        &source_candidate,
        visible_library,
        0,
        Some(&source_declaration),
        &BTreeMap::new(),
    )
    .map_err(|error| {
        chronological_gap(
            BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2,
            "source-closure",
            error,
        )
    })?;
    replay_verified_closure_derivation_v2(signature, source_derivation.projection()).map_err(
        |error| {
            chronological_gap(
                BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2,
                "source-replay",
                error,
            )
        },
    )?;
    if source_derivation.projection().candidate != source_candidate
        || source_derivation.projection().expression
            != newest.canonical_presentation.canonical_normal_form
        || source_derivation.projection().ambient_arity != source_arity
        || source_derivation.projection().signature_digest != signature.digest()
        || source_derivation.projection().visible_library != visible_library
    {
        return Err(chronological_gap(
            BI_CHRONOLOGICAL_SOURCE_CLOSURE_UNAVAILABLE_V2,
            "source-exact-join",
            "replayed source closure does not bind the exact newest canonical family",
        ));
    }
    let target_arity = target_arity.max(source_arity);
    let (image, direct) = match interface_mode {
        A3ChronologicalInterfaceMode::DirectType => (
            older.canonical_presentation.canonical_normal_form.clone(),
            true,
        ),
        A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. } => (
            Expr::App(
                Box::new(older.canonical_presentation.canonical_normal_form.clone()),
                Box::new(Expr::Var(1)),
            ),
            false,
        ),
    };
    let to_sort = |sort: &ParamSort| match sort {
        ParamSort::Type => ParameterSort::Type,
        ParamSort::Opaque => ParameterSort::Opaque,
    };
    let to_kernel_ty = |sort: &ParamSort| match sort {
        ParamSort::Type => KernelTy::Type,
        ParamSort::Opaque => KernelTy::Neutral,
    };
    let source_context = SortedParameterContext::new(
        newest
            .canonical_presentation
            .parameters
            .iter()
            .map(to_sort)
            .collect(),
    );
    let target_parameter_sorts = (0..target_arity as usize)
        .map(|index| {
            older
                .canonical_presentation
                .parameters
                .get(index)
                .or_else(|| newest.canonical_presentation.parameters.get(index))
                .cloned()
                .unwrap_or(ParamSort::Type)
        })
        .collect::<Vec<_>>();
    let target_context =
        SortedParameterContext::new(target_parameter_sorts.iter().map(to_sort).collect());
    let source_ambient_types = newest
        .canonical_presentation
        .parameters
        .iter()
        .map(to_kernel_ty)
        .collect::<Vec<_>>();
    let target_ambient_types = target_parameter_sorts
        .iter()
        .map(to_kernel_ty)
        .collect::<Vec<_>>();
    let (generic_typed, generic_typed_derivation) = elaborate_single_clause_with_typed_ambient(
        &newest.canonical_presentation.canonical_normal_form,
        &source_ambient_types,
        &[],
        visible_library,
    )
    .map_err(|error| format!("chronological generic-family typing: {error}"))?;
    let (image_typed, image_typed_derivation) = elaborate_single_clause_with_typed_ambient(
        &image,
        &target_ambient_types,
        &[],
        visible_library,
    )
    .map_err(|error| format!("chronological substitution-image typing: {error}"))?;
    if image_typed.kernel_ty != KernelTy::Type {
        return Err(format!(
            "chronological substitution image has classifier {:?}, expected Type",
            image_typed.kernel_ty
        ));
    }
    let mut images = Vec::with_capacity(source_arity as usize);
    images.push(SubstitutionImage {
        source_parameter: 1,
        term: image,
    });
    images.extend(
        (2..=source_arity).map(|source_parameter| SubstitutionImage {
            source_parameter,
            term: Expr::Var(source_parameter),
        }),
    );
    let substitution = issue_structural_substitution(
        source_context,
        target_context,
        images,
        newest.canonical_presentation.canonical_normal_form.clone(),
    )
    .map_err(|error| format!("chronological structural substitution: {error}"))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| format!("chronological substitution replay: {error}"))?;
    let output = substitution.result().clone();
    let (typed_output, typed_output_derivation) = elaborate_single_clause_with_typed_ambient(
        &output,
        &target_ambient_types,
        &[],
        visible_library,
    )
    .map_err(|error| format!("chronological typed substitution result: {error}"))?;
    if typed_output.kernel_ty != generic_typed.kernel_ty {
        return Err(format!(
            "chronological typed substitution changed the generic classifier: {:?} -> {:?}",
            generic_typed.kernel_ty, typed_output.kernel_ty
        ));
    }
    let output_telescope = Telescope::new(vec![ClauseRec::new(newest.kernel_role, output.clone())]);
    let elaboration = elaborate_telescope(signature, &output_telescope, visible_library)
        .map_err(|error| format!("chronological output elaboration: {error}"))?;
    let output_clause = elaboration
        .clauses
        .first()
        .ok_or_else(|| "chronological output elaboration is empty".to_owned())?;
    let equality = univalent_equality(
        &output_clause.normal_form,
        &output,
        elaboration.ambient_parameters,
        elaboration.fuel.static_bound.max(256),
    )
    .map_err(|error| format!("chronological normalization equality: {error}"))?;
    if !equality.equal {
        return Err("chronological substitution is not normalization-stable".to_owned());
    }
    let closure = predecessor_closure(signature)
        .map_err(|error| format!("chronological predecessor closure: {error}"))?;
    // Closure-family identity intentionally quotients occurrence-specific
    // renaming maps: the family id is determined by canonical normal form and
    // parameter sorts.  Bind both full presentations in the evidence below,
    // but compare the same semantic payload used by that certified quotient.
    let same_closure_family =
        |family: &pen_eval::typed_families::CanonicalPresentation,
         source: &pen_eval::typed_families::CanonicalPresentation| {
            family.canonical_normal_form == source.canonical_normal_form
                && family.parameters == source.parameters
        };
    let sealed_image_source = closure
        .families
        .iter()
        .find(|family| {
            family.step == older.step
                && family.clause_index == older.clause_index
                && same_closure_family(&family.presentation, &older.canonical_presentation)
        })
        .or_else(|| {
            closure
                .families
                .iter()
                .find(|family| {
                    same_closure_family(&family.presentation, &older.canonical_presentation)
                })
        })
        .ok_or_else(|| {
            format!(
                "chronological older source ({},{}) has no exact-family sealed predecessor-closure preimage",
                older.step, older.clause_index
            )
        })?;
    let sealed_preimage = closure
        .families
        .iter()
        .find(|family| {
            family.step == newest.step
                && family.clause_index == newest.clause_index
                && same_closure_family(&family.presentation, &newest.canonical_presentation)
        })
        .or_else(|| {
            closure
                .families
                .iter()
                .find(|family| {
                    same_closure_family(&family.presentation, &newest.canonical_presentation)
                })
        })
        .ok_or_else(|| {
            format!(
                "chronological newest source ({},{}) has no exact-family sealed predecessor-closure preimage",
                newest.step, newest.clause_index
            )
        })?;
    let exact_substitution_from_sealed_preimage_replayed = substitution.source().arity()
        == source_arity
        && substitution.target().arity() == target_arity
        && substitution.result() == &output
        && substitution.images().len() == source_arity as usize
        && substitution.images().iter().all(|assignment| {
            if assignment.source_parameter == 1 {
                assignment.term
                    == match interface_mode {
                        A3ChronologicalInterfaceMode::DirectType => {
                            older.canonical_presentation.canonical_normal_form.clone()
                        }
                        A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. } => {
                            Expr::App(
                                Box::new(
                                    older.canonical_presentation.canonical_normal_form.clone(),
                                ),
                                Box::new(Expr::Var(1)),
                            )
                        }
                    }
            } else {
                assignment.term == Expr::Var(assignment.source_parameter)
                    && interface_slot_map.assignments.iter().any(|declared| {
                        declared.interface_slot == assignment.source_parameter
                            && declared.parameter == assignment.source_parameter
                    })
            }
        });
    if !exact_substitution_from_sealed_preimage_replayed {
        return Err(
            "chronological exact structural-substitution preimage did not replay".to_owned(),
        );
    }
    let extraction =
        extract_candidate_families(signature, &closure, &output_telescope, visible_library);
    let extracted = extraction
        .extraction()
        .ok_or_else(|| "chronological output failed the live typed-family extraction".to_owned())?;
    if extracted.families.len() != 1
        || !extracted.families[0].naturality.square.equal
        || extracted.signature_digest != signature.digest()
    {
        return Err("chronological output natural-family square did not replay".to_owned());
    }
    let extracted_family = &extracted.families[0];
    // `decide_marginality` is a deliberately narrower counter/EGP
    // classifier: after exact equality it only searches rigid-pattern
    // instances.  Chronological D-membership has stronger, explicit evidence
    // above: the named newest sealed family plus the replayed structural
    // substitution.  Preserve the narrow classifier's answer in the audit;
    // do not convert it into an `InternalDerivable` disposition it did not
    // issue.
    let restricted_candidate_classifier_reported_marginal = matches!(
        extracted_family.marginality,
        MarginalityDisposition::MarginalNoClosurePreimage { .. }
    );
    let assignment_evidence = substitution
        .images()
        .iter()
        .enumerate()
        .map(|(index, assignment)| {
            let expected_parameter = index as u32 + 1;
            if assignment.source_parameter != expected_parameter {
                return Err(chronological_gap(
                    BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
                    "assignment-order",
                    format!(
                        "image {} targets source parameter {}",
                        index + 1,
                        assignment.source_parameter
                    ),
                ));
            }
            issue_chronological_closed_image_evidence(
                signature,
                visible_library,
                if index == 0 {
                    older.kernel_role
                } else {
                    pen_core::clause::ClauseRole::Introduction
                },
                &assignment.term,
                &format!("parameter-{expected_parameter}"),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let assignment = issue_motive_typed_closed_assignment_v2(
        signature,
        visible_library,
        source_motives,
        assignment_evidence,
    )
    .map_err(|error| {
        chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
            "assignment-issuance",
            error,
        )
    })?;
    replay_motive_typed_closed_assignment_v2(signature, assignment.projection()).map_err(
        |error| {
            chronological_gap(
                BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
                "assignment-replay",
                error,
            )
        },
    )?;
    if assignment.projection().images.len() != substitution.images().len()
        || !assignment
            .projection()
            .images
            .iter()
            .zip(substitution.images())
            .all(|(closed, declared)| {
                closed.parameter == declared.source_parameter
                    && closed.term == declared.term
                    && closed.evidence_replayed
                    && closed.evidence.expression == declared.term
            })
    {
        return Err(chronological_gap(
            BI_CHRONOLOGICAL_ASSIGNMENT_EVIDENCE_UNAVAILABLE_V2,
            "assignment-exact-join",
            "closed assignment does not reproduce every declared chronological image",
        ));
    }
    let specialized =
        specialize_verified_closure_derivation_v2(signature, &source_derivation, &assignment)
            .map_err(|error| {
                chronological_gap(
                    BI_CHRONOLOGICAL_SPECIALIZATION_UNAVAILABLE_V2,
                    "specialization-issuance",
                    error,
                )
            })?;
    replay_specialized_closure_derivation_v2(signature, specialized.projection()).map_err(
        |error| {
            chronological_gap(
                BI_CHRONOLOGICAL_SPECIALIZATION_UNAVAILABLE_V2,
                "specialization-replay",
                error,
            )
        },
    )?;
    let specialized_result_equal_to_demanded_output =
        specialized.projection().specialized_expression == output
            && specialized.projection().specialized_candidate == output_telescope
            && specialized.projection().specialized_relation.expression == output
            && specialized
                .projection()
                .specialized_relation
                .relation
                .expression
                == output;
    if !specialized_result_equal_to_demanded_output {
        return Err(chronological_gap(
            BI_CHRONOLOGICAL_SPECIALIZED_RESULT_MISMATCH_V2,
            "specialized-result",
            "replayed closed specialization differs from the demanded chronological output",
        ));
    }
    let internal_closure_preimage = true;
    let orbit = window
        .orbits
        .iter()
        .find(|orbit| {
            orbit.scheme_id == scheme.scheme_id
                && orbit.member_instance_ids.contains(&instance.instance_id)
        })
        .ok_or_else(|| "chronological instance has no exact quotient orbit".to_owned())?;
    let evidence_hash = tagged_hash(
        "branch-chronological-d-membership",
        &(
            (
                MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION,
                signature.digest(),
                visible_library,
                window.window_derivation_hash.as_str(),
                scheme.scheme_id.as_str(),
                instance.instance_id.as_str(),
                older.typing_derivation_hash.as_str(),
                newest.typing_derivation_hash.as_str(),
                orbit.orbit_derivation_hash.as_str(),
                interface_slot_map.declaration_hash.as_str(),
                substitution.derivation_hash(),
                source_derivation.derivation_hash(),
                assignment.projection().assignment_hash.as_str(),
                specialized.projection().derivation_hash.as_str(),
            ),
            (
                &source_ambient_types,
                &target_ambient_types,
                &generic_typed,
                &generic_typed_derivation,
                &image_typed,
                &image_typed_derivation,
                &typed_output,
                &typed_output_derivation,
            ),
            (
                sealed_image_source.step,
                sealed_image_source.clause_index,
                sealed_image_source.id.as_str(),
                &sealed_image_source.presentation,
                &older.canonical_presentation,
                sealed_preimage.step,
                sealed_preimage.clause_index,
                sealed_preimage.id.as_str(),
                &sealed_preimage.presentation,
                &newest.canonical_presentation,
                elaboration.derivation_hash.as_str(),
                extracted.derivation_hash.as_str(),
            ),
            (
                &extracted_family.naturality,
                &extracted_family.marginality,
                restricted_candidate_classifier_reported_marginal,
                exact_substitution_from_sealed_preimage_replayed,
                specialized_result_equal_to_demanded_output,
                &equality,
                source_order_negative_control,
            ),
        ),
    );
    Ok(BranchChronologicalMembershipProof {
        evidence_hash,
        output_kernel_typed: equality.equal && elaboration.fuel.within_bound,
        internal_closure_preimage,
        internality_rule_id: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION,
        restricted_candidate_classifier_reported_marginal,
        exact_substitution_from_sealed_preimage_replayed,
        uniform_specialization_not_new_family: instance.identity_or_uniform_specialization
            && orbit.uniform_specializations_collapsed,
        independently_exported_output_orbit: instance.independently_exported_demand
            || orbit.independently_exported_demand_orbit,
        source_or_evidence_swap_rejected: source_order_negative_control,
        direct,
    })
}

#[allow(clippy::too_many_arguments)]
fn finish_membership_audit(
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    evidence_hash: String,
    hypothetical_derivation_replayed: bool,
    output_kernel_typed: bool,
    internal_closure_preimage: bool,
    internality_rule_id: String,
    restricted_candidate_classifier_reported_marginal: bool,
    exact_substitution_from_sealed_preimage_replayed: bool,
    uniform_specialization_not_new_family: bool,
    independently_exported_output_orbit: bool,
    marginal_charge_zero_or_not_applicable: bool,
    source_or_evidence_swap_rejected: bool,
    exact_reason: Option<String>,
    issuer_gap: bool,
) -> BranchDMembershipAudit {
    let derivable = hypothetical_derivation_replayed
        && output_kernel_typed
        && internal_closure_preimage
        && uniform_specialization_not_new_family
        && !independently_exported_output_orbit
        && marginal_charge_zero_or_not_applicable;
    let mut row = BranchDMembershipAudit {
        a3_instance_id: instance.instance_id.clone(),
        a3_scheme_id: scheme.scheme_id.clone(),
        rule_constructor: rule_name(scheme.rule_constructor).to_owned(),
        evidence_hash,
        hypothetical_derivation_replayed,
        output_kernel_typed,
        internal_closure_preimage,
        internality_rule_id,
        restricted_candidate_classifier_reported_marginal,
        exact_substitution_from_sealed_preimage_replayed,
        uniform_specialization_not_new_family,
        independently_exported_output_orbit,
        marginal_charge_zero_or_not_applicable,
        source_or_evidence_swap_rejected,
        disposition: if issuer_gap {
            BranchDMembershipDisposition::IssuerGap
        } else if derivable {
            BranchDMembershipDisposition::Derivable
        } else {
            BranchDMembershipDisposition::Underdetermined
        },
        exact_reason: if derivable && !issuer_gap {
            None
        } else {
            exact_reason
                .or_else(|| Some("one or more replayed D-membership premises failed".to_owned()))
        },
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("d-membership-audit", &row);
    row
}

pub(crate) fn issue_branch_chronological_membership_audit(
    signature: &SealedSignature,
    visible_library: u32,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
) -> Result<(BranchDMembershipAudit, bool), String> {
    let proof = prove_branch_chronological_membership(
        signature,
        visible_library,
        window,
        scheme,
        instance,
    )?;
    let direct = proof.direct;
    let row = finish_membership_audit(
        instance,
        scheme,
        proof.evidence_hash,
        true,
        proof.output_kernel_typed,
        proof.internal_closure_preimage,
        proof.internality_rule_id.to_owned(),
        proof.restricted_candidate_classifier_reported_marginal,
        proof.exact_substitution_from_sealed_preimage_replayed,
        proof.uniform_specialization_not_new_family,
        proof.independently_exported_output_orbit,
        true,
        proof.source_or_evidence_swap_rejected,
        None,
        false,
    );
    Ok((row, direct))
}

fn replay_branch_chronological_membership_audit(
    signature: &SealedSignature,
    visible_library: u32,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    claimed: &BranchDMembershipAudit,
) -> Vec<String> {
    match issue_branch_chronological_membership_audit(
        signature,
        visible_library,
        window,
        scheme,
        instance,
    ) {
        Ok((expected, _)) if expected == *claimed => Vec::new(),
        Ok(_) => vec!["chronological D-membership differs from exact reissuance".to_owned()],
        Err(error) => vec![format!(
            "chronological D-membership reissuance failed: {error}"
        )],
    }
}

fn charge_joins_step(
    charge: &future_v2::FillerOrdinaryChargeProvenanceV2,
    source: &BranchFinaleStepInput,
) -> bool {
    let unique_token_count = source
        .ordinary_family_token_hashes
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    source.ordinary_charge_provenance_authoritative
        && source.ordinary_family_token_hashes.len() == source.certified_nu as usize
        && unique_token_count == source.ordinary_family_token_hashes.len()
        && future_v2::replay_filler_ordinary_charge_provenance_v2(charge)
        && charge.filler_step == source.stage
        && charge.filler_candidate_hash == source.candidate_hash
        && charge.filler_telescope_digest == source.candidate_hash
        && charge.ordinary_kappa == source.kappa
        && charge.certified_nu == source.certified_nu
        && charge.bit_length == source.telescope.bit_cost()
        && charge.upstream_certificate_hash == source.step_evidence_hash
        && charge.ordinary_family_token_hashes == source.ordinary_family_token_hashes
        && source.candidate_hash == candidate_hash(&source.telescope)
        && source.kappa == source.telescope.kappa() as u32
        && !source.step_evidence_hash.is_empty()
}

/// Replay the proof-strength premises adopted by the authoritative E-5 v2
/// unary membership issuer.  This is contextual Internal evidence; it does
/// not ask the narrower candidate-credit classifier for a predecessor-family
/// marginality verdict.
fn unary_exact_source_replays(registration: &future_v2::RegisteredFutureHoleV2) -> bool {
    let evidence = &registration.parametric_internality;
    let expected_rule_inventory = CLOSURE_RULE_INVENTORY_V2
        .iter()
        .map(|rule| format!("{rule:?}"))
        .collect::<Vec<_>>();
    evidence.hypothetical_internality_issued
        && evidence.eliminator_version == MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION
        && evidence.closure_rule_inventory == expected_rule_inventory
        && evidence.explicit_ambient_arity == registration.declared_ambient_arity
        && evidence.every_declared_parameter_live
        && evidence.every_body_former_registered
        && evidence
            .closure_formers
            .iter()
            .all(|former| evidence.registered_former_inventory.contains(former))
        && evidence.historical_prefix_declaration_hash
            == registration.ambient_declaration.declaration_hash
        && evidence.dependent_declaration_hash == registration.ambient_declaration.declaration_hash
        && evidence.dependent_totality_theorem_replayed
        && evidence.total_specialization_authoritative
        && evidence
            .dependent_totality_theorem
            .total_specialization_theorem_issued
        && evidence.dependent_totality_theorem.declaration_hash
            == registration.ambient_declaration.declaration_hash
        && !evidence.typed_body_elaboration_hash.is_empty()
        && !evidence.evidence_hash.is_empty()
        && evidence.marginal_nu == 0
}

fn unary_exact_body_binding_replays(registration: &future_v2::RegisteredFutureHoleV2) -> bool {
    let evidence = &registration.parametric_internality;
    evidence.source_bound_to_exact_body
        && evidence.exact_body_candidate_hash == candidate_hash(&registration.body_telescope)
        && registration.ambient_declaration.body_telescope == registration.body_telescope
        && registration.ambient_declaration.candidate_hash == evidence.exact_body_candidate_hash
        && registration
            .body_telescope
            .clauses
            .first()
            .is_some_and(|clause| registration.ambient_declaration.expression == clause.expr)
        && registration.ambient_declaration.signature_digest
            == registration.ambient_registration_prefix_signature_digest
        && registration.ambient_declaration.visible_library
            == registration.visible_library_at_registration
}

fn failed_registration_audit(
    stage: u32,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    gap_id: String,
    exact_error: &str,
) -> BranchFutureRegistrationAudit {
    let mut audit = BranchFutureRegistrationAudit {
        stage,
        a3_instance_id: instance.instance_id.clone(),
        a3_scheme_id: scheme.scheme_id.clone(),
        rule_constructor: rule_name(scheme.rule_constructor).to_owned(),
        natural_family_id: None,
        occurrence_id: None,
        formation_or_gap_hash: tagged_hash(
            "future-registration-error-gap",
            &(
                stage,
                scheme.scheme_id.as_str(),
                instance.instance_id.as_str(),
                gap_id.as_str(),
                exact_error,
            ),
        ),
        deterministic_replay_valid: false,
        external_inventory_join_exact: false,
        dependent_totality_theorem_replayed: false,
        total_specialization_authoritative: false,
        no_outcome_filtering: false,
        no_reflexivity_fallback: false,
        every_hole_live: false,
        local_semantic_scope_satisfied: false,
        marginal_charge_zero: false,
        gap_id: Some(gap_id),
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("future-registration-audit", &audit);
    audit
}

fn failed_realization_audit(
    registration_stage: u32,
    jurisdiction_stage: u32,
    instance_id: &str,
    constructor: &str,
    filler_candidate_hash: String,
    ordinary_kappa: u32,
    certified_nu: u32,
    ordinary_charge_provenance_authoritative: bool,
    ordinary_charge_joins_branch_step: bool,
    wrong_provider_rejected: bool,
    charge_swap_rejected: bool,
    gap_id: String,
    exact_error: &str,
) -> BranchStructuralRealizationAudit {
    let mut audit = BranchStructuralRealizationAudit {
        registration_stage,
        jurisdiction_stage,
        a3_instance_id: instance_id.to_owned(),
        constructor: constructor.to_owned(),
        filler_candidate_hash,
        ordinary_kappa,
        certified_nu,
        ordinary_charge_provenance_authoritative,
        ordinary_charge_joins_branch_step,
        deterministic_replay_valid: false,
        provider_relation_satisfied: false,
        constructor_live_before_filler: false,
        constructor_absent_after_filler: false,
        specialization_replayed: false,
        marginal_charge_zero: false,
        ordinary_charge_preserved: false,
        wrong_provider_rejected,
        charge_swap_rejected_by_authoritative_join: charge_swap_rejected,
        realization_or_gap_hash: tagged_hash(
            "structural-realization-error-gap",
            &(
                registration_stage,
                jurisdiction_stage,
                instance_id,
                constructor,
                gap_id.as_str(),
                exact_error,
            ),
        ),
        gap_id: Some(gap_id),
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("structural-realization-audit", &audit);
    audit
}

fn validate_input(input: &BranchFinaleInput) -> bool {
    input.halt_step >= 4
        && input.lawful_engine_stage_limit >= input.halt_step
        && input.steps.len() == input.halt_step as usize
        && input
            .steps
            .iter()
            .map(|row| row.stage)
            .eq(1..=input.halt_step)
        && input.steps.iter().all(|row| {
            row.candidate_hash == candidate_hash(&row.telescope)
                && row.kappa == row.telescope.kappa() as u32
                && !row.step_evidence_hash.is_empty()
        })
        && input
            .steps
            .get(3)
            .is_some_and(|row| row.candidate_hash == input.branch_root_hash)
        && !input.continuation_digest.is_empty()
}

fn capacity_gap_audit(
    input: &BranchFinaleInput,
    successor_stage: u32,
    continuation_provenance_join_exact: bool,
    exact_gap: String,
) -> BranchFinaleAudit {
    let mut audit = BranchFinaleAudit {
        schema: BI_BRANCH_FINALE_SCHEMA.to_owned(),
        branch_root_hash: input.branch_root_hash.clone(),
        continuation_digest: input.continuation_digest.clone(),
        input_binding_hash: branch_finale_input_binding_hash(input),
        continuation_provenance_join_exact,
        lawful_engine_stage_limit: input.lawful_engine_stage_limit,
        halt_step: input.halt_step,
        successor_stage,
        input_contiguous_and_self_consistent: true,
        windows: Vec::new(),
        every_window_inventory_exhaustive: false,
        inventory_scope: "relative_to_adopted_full_A3_depth_two_rule_inventory".to_owned(),
        absolute_semantic_exhaustiveness_claimed: false,
        registrations: Vec::new(),
        unary_registration_count: 0,
        structural_registration_count: 0,
        registration_gap_ids: vec![exact_gap.clone()],
        every_registration_replayed_with_dependent_totality: false,
        every_registration_evidence_mutation_rejected: false,
        realizations: Vec::new(),
        structural_realization_count: 0,
        realization_gap_ids: Vec::new(),
        every_structural_hole_realized: false,
        every_wrong_provider_rejected: false,
        every_charge_swap_rejected: false,
        stage3_wrinkle_reproduced: false,
        focus_projection_reproduces_branch_ladder: false,
        final_a3_inventory_count: 0,
        final_unary_count: 0,
        final_direct_chronological_count: 0,
        final_pointwise_chronological_count: 0,
        final_higher_count: 0,
        final_structural_count: 0,
        membership: Vec::new(),
        exact_d_partition: false,
        every_membership_source_or_evidence_swap_rejected: false,
        derivable_instance_count: 0,
        underdetermined_instance_ids: Vec::new(),
        issuer_gap_instance_ids: Vec::new(),
        semantic_successor_o_empty: false,
        f1_executed: false,
        f1_triggered: false,
        f1_excluded: false,
        f1_counterfactual: None,
        f1_counterfactual_underdetermination_detected: false,
        theorem12_full_instance_granularity_proved: false,
        theorem12_scope: "full_instance_granularity_within_adopted_A3_only".to_owned(),
        broader_absolute_theorem12_claimed: false,
        expressivity_gaps: vec![exact_gap],
        e5_class_complete: false,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("branch-finale-audit", &audit);
    audit
}

/// Execute a raw branch-local finale input.  This is useful for deterministic
/// mutation probes, but cannot complete E-5 because it has not replayed the
/// authoritative continuation.  Production issuance uses
/// [`execute_certified_branch_finale`].
pub fn execute_branch_finale(input: &BranchFinaleInput) -> Result<BranchFinaleAudit, String> {
    execute_branch_finale_inner(input, false)
}

fn execute_branch_finale_inner(
    input: &BranchFinaleInput,
    continuation_provenance_join_exact: bool,
) -> Result<BranchFinaleAudit, String> {
    let input_contiguous_and_self_consistent = validate_input(input);
    if !input_contiguous_and_self_consistent {
        return Err(
            "branch finale input is not a contiguous self-consistent certified branch".to_owned(),
        );
    }
    let successor_stage = input
        .halt_step
        .checked_add(1)
        .ok_or_else(|| "branch halt stage overflows u32".to_owned())?;
    if successor_stage > input.lawful_engine_stage_limit {
        return Ok(capacity_gap_audit(
            input,
            successor_stage,
            continuation_provenance_join_exact,
            format!(
                "bi_finale_successor_exceeds_certified_engine_limit_stage_{successor_stage}_max_{}",
                input.lawful_engine_stage_limit
            ),
        ));
    }
    let mut generated = BTreeMap::<u32, A3HistoricalWindow>::new();
    let mut proofs = BTreeMap::<u32, A3WindowRuleInventoryProof>::new();
    let mut windows = Vec::new();
    for stage_number in 1..=successor_stage {
        let prefix = exact_prefix(input, stage_number);
        let window = match generate_a3_window_for_exact_prefix_unbounded(&prefix, stage_number) {
            Ok(window) => window,
            Err(error) => {
                return Ok(capacity_gap_audit(
                    input,
                    successor_stage,
                    continuation_provenance_join_exact,
                    format!("bi_a3_window_expressivity_gap_stage_{stage_number}::{error}"),
                ));
            }
        };
        let proof =
            match prove_a3_window_inventory_for_exact_prefix_unbounded(&prefix, stage_number) {
                Ok(proof) => proof,
                Err(error) => {
                    return Ok(capacity_gap_audit(
                        input,
                        successor_stage,
                        continuation_provenance_join_exact,
                        format!("bi_a3_exhaustiveness_gap_stage_{stage_number}::{error}"),
                    ));
                }
            };
        if proof.exact_prefix_signature_digest != prefix.digest()
            || proof.stage != stage_number
            || proof.required_packages_from_raw_debt
                != window
                    .constructor_evidence
                    .iter()
                    .map(|evidence| evidence.constructor.slug().to_owned())
                    .collect::<Vec<_>>()
        {
            return Err(format!("Stage {stage_number} A3 proof/window join failed"));
        }
        let expected_branch_demand = if stage_number <= input.halt_step {
            step(input, stage_number)?
                .required_packages_before_selection
                .clone()
        } else {
            input.terminal_required_packages.clone()
        };
        let structural_constructors = window
            .constructor_evidence
            .iter()
            .map(|evidence| evidence.constructor.slug().to_owned())
            .collect::<Vec<_>>();
        let projected_focus = window
            .focus_projection
            .projected_focus
            .map(|constructor| constructor.slug().to_owned());
        let jurisdiction_matches = if stage_number == 3 {
            projected_focus.is_none() && window.focus_projection.demand_precedes_jurisdiction
        } else if structural_constructors.is_empty() {
            projected_focus.is_none()
        } else {
            projected_focus.as_ref() == structural_constructors.first()
        };
        let focus_projection_matches_branch_ledger = expected_branch_demand
            == proof.required_packages_from_raw_debt
            && proof.required_packages_from_raw_debt == structural_constructors
            && jurisdiction_matches;
        let mut audit = BranchA3WindowAudit {
            stage: stage_number,
            exact_prefix_signature_digest: prefix.digest().to_owned(),
            window_derivation_hash: window.window_derivation_hash.clone(),
            inventory_proof_hash: proof.derivation_hash.clone(),
            relative_inventory_exhaustive: proof.relative_rule_inventory_exhaustive_for_window,
            independent_preimage_ids_injective: proof.independent_preimage_ids_injective,
            future_hole_typing_deferred_to_branch_v2: proof
                .future_hole_typing_deferred_to_branch_v2,
            instance_count: window.instances.len(),
            required_packages: proof.required_packages_from_raw_debt.clone(),
            structural_constructors,
            projected_focus,
            demand_precedes_jurisdiction: window.focus_projection.demand_precedes_jurisdiction,
            expected_branch_demand,
            focus_projection_matches_branch_ledger,
            derivation_hash: String::new(),
        };
        audit.derivation_hash = tagged_hash("a3-window-audit", &audit);
        generated.insert(stage_number, window);
        proofs.insert(stage_number, proof);
        windows.push(audit);
    }
    let every_window_inventory_exhaustive = windows.iter().all(|row| {
        row.relative_inventory_exhaustive
            && row.independent_preimage_ids_injective
            && row.future_hole_typing_deferred_to_branch_v2
    });

    let mut registrations = Vec::new();
    let mut unary_registered = BTreeMap::<String, future_v2::RegisteredFutureHoleV2>::new();
    let mut structural_registered =
        BTreeMap::<(u32, String), future_v2::RegisteredFutureHoleV2>::new();
    let mut registration_gap_ids = Vec::new();
    let mut registration_mutation_trials = 0_usize;
    let mut registration_mutation_rejections = 0_usize;
    for stage_number in 1..=successor_stage {
        let window = generated.get(&stage_number).expect("generated above");
        let proof = proofs.get(&stage_number).expect("proved above");
        let registration_prefix =
            exact_prefix(input, if stage_number == 3 { 4 } else { stage_number });
        for instance in &window.instances {
            let scheme = scheme_for_instance(window, instance)?;
            let relevant = scheme.rule_constructor == A3RuleConstructor::StructuralCompletionHole
                || (stage_number == successor_stage
                    && scheme.rule_constructor == A3RuleConstructor::UnaryAction);
            if !relevant {
                continue;
            }
            let raw_result = match scheme.rule_constructor {
                A3RuleConstructor::UnaryAction => future_v2::register_unary_action_v2(
                    &registration_prefix,
                    window,
                    scheme,
                    instance,
                ),
                A3RuleConstructor::StructuralCompletionHole => {
                    future_v2::register_structural_future_hole_v2(
                        &registration_prefix,
                        window,
                        scheme,
                        instance,
                    )
                }
                _ => unreachable!("relevance filter"),
            };
            let raw = match raw_result {
                Ok(raw) => raw,
                Err(error) => {
                    let exact_error = error.to_string();
                    let gap_id = format!(
                        "bi_future_registration_api_gap_stage_{stage_number}::{}",
                        instance.instance_id
                    );
                    registration_gap_ids.push(gap_id.clone());
                    registrations.push(failed_registration_audit(
                        stage_number,
                        scheme,
                        instance,
                        gap_id,
                        &exact_error,
                    ));
                    continue;
                }
            };
            let (joined_disposition, joined_registration) = match raw {
                future_v2::FutureHoleRegistrationDispositionV2::Registered(value) => {
                    let joined = match future_v2::attach_external_exhaustiveness_evidence_v2(
                        &value,
                        proof.derivation_hash.clone(),
                    ) {
                        Ok(joined) => joined,
                        Err(error) => {
                            let exact_error = error.to_string();
                            let gap_id = format!(
                                "bi_future_registration_exhaustiveness_join_gap_stage_{stage_number}::{}",
                                instance.instance_id
                            );
                            registration_gap_ids.push(gap_id.clone());
                            registrations.push(failed_registration_audit(
                                stage_number,
                                scheme,
                                instance,
                                gap_id,
                                &exact_error,
                            ));
                            continue;
                        }
                    };
                    (
                        future_v2::FutureHoleRegistrationDispositionV2::Registered(joined.clone()),
                        Some(joined),
                    )
                }
                future_v2::FutureHoleRegistrationDispositionV2::Gap(gap) => {
                    registration_gap_ids.push(gap.id.clone());
                    (
                        future_v2::FutureHoleRegistrationDispositionV2::Gap(gap),
                        None,
                    )
                }
            };
            let replay = future_v2::replay_future_hole_registration_v2(
                &registration_prefix,
                window,
                scheme,
                instance,
                &joined_disposition,
            );
            if let Some(joined) = &joined_registration {
                let mut mutated = joined.clone();
                mutated.external_exhaustiveness_evidence_hash =
                    Some("blake3:forged-exhaustiveness-join".to_owned());
                let mutated_disposition =
                    future_v2::FutureHoleRegistrationDispositionV2::Registered(mutated);
                let mutated_replay = future_v2::replay_future_hole_registration_v2(
                    &registration_prefix,
                    window,
                    scheme,
                    instance,
                    &mutated_disposition,
                );
                registration_mutation_trials += 1;
                if !mutated_replay.valid {
                    registration_mutation_rejections += 1;
                }
            }
            let (
                natural_family_id,
                occurrence_id,
                formation_or_gap_hash,
                external_inventory_join_exact,
                dependent_totality_theorem_replayed,
                total_specialization_authoritative,
                no_outcome_filtering,
                no_reflexivity_fallback,
                every_hole_live,
                local_semantic_scope_satisfied,
                marginal_charge_zero,
                gap_id,
            ) = match &joined_disposition {
                future_v2::FutureHoleRegistrationDispositionV2::Registered(value) => (
                    Some(value.natural_family_id.clone()),
                    Some(value.occurrence_id.clone()),
                    value.formation_hash.clone(),
                    value.external_exhaustiveness_evidence_hash.as_deref()
                        == Some(proof.derivation_hash.as_str()),
                    value
                        .parametric_internality
                        .dependent_totality_theorem_replayed,
                    value
                        .parametric_internality
                        .total_specialization_authoritative,
                    value.ambient_declaration.no_outcome_filtering_used
                        && value
                            .parametric_internality
                            .dependent_totality_theorem
                            .no_assignment_outcome_filtering,
                    value.no_reflexivity_fallback,
                    value.every_hole_live,
                    value.local_semantic_scope_premises_satisfied,
                    value.hole_marginal_charge.replays_as_zero(),
                    None,
                ),
                future_v2::FutureHoleRegistrationDispositionV2::Gap(gap) => (
                    None,
                    None,
                    gap.gap_hash.clone(),
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    Some(gap.id.clone()),
                ),
            };
            let mut audit = BranchFutureRegistrationAudit {
                stage: stage_number,
                a3_instance_id: instance.instance_id.clone(),
                a3_scheme_id: scheme.scheme_id.clone(),
                rule_constructor: rule_name(scheme.rule_constructor).to_owned(),
                natural_family_id,
                occurrence_id,
                formation_or_gap_hash,
                deterministic_replay_valid: replay.valid,
                external_inventory_join_exact,
                dependent_totality_theorem_replayed,
                total_specialization_authoritative,
                no_outcome_filtering,
                no_reflexivity_fallback,
                every_hole_live,
                local_semantic_scope_satisfied,
                marginal_charge_zero,
                gap_id,
                derivation_hash: String::new(),
            };
            audit.derivation_hash = tagged_hash("future-registration-audit", &audit);
            registrations.push(audit);
            if let Some(joined) = joined_registration {
                if scheme.rule_constructor == A3RuleConstructor::UnaryAction {
                    unary_registered.insert(instance.instance_id.clone(), joined);
                } else {
                    structural_registered
                        .insert((stage_number, instance.instance_id.clone()), joined);
                }
            }
        }
    }
    registrations.sort_by(|left, right| {
        (left.stage, left.a3_instance_id.as_str())
            .cmp(&(right.stage, right.a3_instance_id.as_str()))
    });
    registration_gap_ids.sort();
    let every_registration_replayed_with_dependent_totality = registration_gap_ids.is_empty()
        && registrations.iter().all(|row| {
            row.deterministic_replay_valid
                && row.external_inventory_join_exact
                && row.dependent_totality_theorem_replayed
                && row.total_specialization_authoritative
                && row.no_outcome_filtering
                && row.no_reflexivity_fallback
                && row.every_hole_live
                && row.local_semantic_scope_satisfied
                && row.marginal_charge_zero
        });
    let every_registration_evidence_mutation_rejected = registration_mutation_trials > 0
        && registration_mutation_rejections == registration_mutation_trials;

    let mut realizations = Vec::new();
    let mut realization_gap_ids = Vec::new();
    for ((registration_stage, instance_id), registration) in &structural_registered {
        let future_v2::FutureHoleOutputContractV2::StructuralProvides(contract) =
            &registration.output_contract
        else {
            return Err(format!(
                "structural registration {instance_id} has unary contract"
            ));
        };
        let filler_step = contract.jurisdiction.jurisdiction_stage();
        let filler_source = match step(input, filler_step) {
            Ok(source) => source,
            Err(error) => {
                let gap_id = format!(
                    "bi_structural_provider_outside_completed_branch_stage_{filler_step}::{instance_id}"
                );
                realization_gap_ids.push(gap_id.clone());
                realizations.push(failed_realization_audit(
                    *registration_stage,
                    filler_step,
                    instance_id,
                    contract.constructor.slug(),
                    String::new(),
                    0,
                    0,
                    false,
                    false,
                    false,
                    false,
                    gap_id,
                    &error,
                ));
                continue;
            }
        };
        if !filler_source.ordinary_charge_provenance_authoritative
            || filler_source.ordinary_family_token_hashes.len()
                != filler_source.certified_nu as usize
            || filler_source
                .ordinary_family_token_hashes
                .iter()
                .collect::<BTreeSet<_>>()
                .len()
                != filler_source.ordinary_family_token_hashes.len()
        {
            let gap_id = format!(
                "{BI_STRUCTURAL_FILLER_ORDINARY_CHARGE_PROVENANCE_UNCERTIFIED_V1}::registration_stage_{registration_stage}::filler_stage_{filler_step}::{instance_id}"
            );
            let exact_error = format!(
                "branch filler Stage {filler_step} carries diagnostic nu={} and {} ordinary-family tokens, but no exact candidate-local nu/family-token certificate; self-issued charge cannot replace upstream ordinary provenance",
                filler_source.certified_nu,
                filler_source.ordinary_family_token_hashes.len(),
            );
            realization_gap_ids.push(gap_id.clone());
            realizations.push(failed_realization_audit(
                *registration_stage,
                filler_step,
                instance_id,
                contract.constructor.slug(),
                filler_source.candidate_hash.clone(),
                filler_source.kappa,
                filler_source.certified_nu,
                false,
                false,
                false,
                false,
                gap_id,
                &exact_error,
            ));
            continue;
        }
        let charge = future_v2::issue_filler_ordinary_charge_provenance_v2(
            filler_step,
            &filler_source.telescope,
            filler_source.kappa,
            filler_source.certified_nu,
            filler_source.telescope.bit_cost(),
            filler_source.step_evidence_hash.clone(),
            filler_source.ordinary_family_token_hashes.clone(),
        );
        let ordinary_charge_joins_branch_step = charge_joins_step(&charge, filler_source);
        if !ordinary_charge_joins_branch_step {
            let exact_error = format!(
                "Stage {registration_stage} structural charge does not join branch Stage {filler_step}"
            );
            let gap_id =
                format!("bi_structural_charge_provenance_gap_stage_{filler_step}::{instance_id}");
            realization_gap_ids.push(gap_id.clone());
            realizations.push(failed_realization_audit(
                *registration_stage,
                filler_step,
                instance_id,
                contract.constructor.slug(),
                filler_source.candidate_hash.clone(),
                charge.ordinary_kappa,
                charge.certified_nu,
                true,
                false,
                false,
                false,
                gap_id,
                &exact_error,
            ));
            continue;
        }
        let jurisdiction_prefix = exact_prefix(input, filler_step);

        let wrong_source = match filler_step
            .checked_sub(1)
            .ok_or_else(|| "structural jurisdiction has no predecessor".to_owned())
            .and_then(|stage| step(input, stage))
        {
            Ok(source) => source,
            Err(error) => {
                let gap_id = format!(
                    "bi_wrong_provider_negative_control_unformable_stage_{filler_step}::{instance_id}"
                );
                realization_gap_ids.push(gap_id.clone());
                realizations.push(failed_realization_audit(
                    *registration_stage,
                    filler_step,
                    instance_id,
                    contract.constructor.slug(),
                    filler_source.candidate_hash.clone(),
                    charge.ordinary_kappa,
                    charge.certified_nu,
                    true,
                    true,
                    false,
                    false,
                    gap_id,
                    &error,
                ));
                continue;
            }
        };
        let wrong_charge = future_v2::issue_filler_ordinary_charge_provenance_v2(
            filler_step,
            &wrong_source.telescope,
            wrong_source.kappa,
            wrong_source.certified_nu,
            wrong_source.telescope.bit_cost(),
            wrong_source.step_evidence_hash.clone(),
            wrong_source.ordinary_family_token_hashes.clone(),
        );
        let wrong_result = future_v2::realize_structural_future_hole_v2(
            &jurisdiction_prefix,
            registration,
            filler_step,
            &wrong_source.telescope,
            &wrong_charge,
        );
        let wrong_provider_rejected = match wrong_result {
            Ok(wrong) => matches!(
                &wrong,
                future_v2::FutureHoleRealizationDispositionV2::Gap(gap)
                    if gap.id == "a3_v2_wrong_structural_provider" && gap.replays()
            ),
            Err(error) => {
                realization_gap_ids.push(format!(
                    "bi_wrong_provider_negative_control_api_gap_stage_{filler_step}::{instance_id}::{error}"
                ));
                false
            }
        };
        let swapped_charge = future_v2::issue_filler_ordinary_charge_provenance_v2(
            filler_step,
            &filler_source.telescope,
            wrong_source.kappa,
            wrong_source.certified_nu,
            wrong_source.telescope.bit_cost(),
            wrong_source.step_evidence_hash.clone(),
            wrong_source.ordinary_family_token_hashes.clone(),
        );
        let charge_swap_rejected_by_authoritative_join =
            future_v2::replay_filler_ordinary_charge_provenance_v2(&swapped_charge)
                && !charge_joins_step(&swapped_charge, filler_source);

        let disposition = match future_v2::realize_structural_future_hole_v2(
            &jurisdiction_prefix,
            registration,
            filler_step,
            &filler_source.telescope,
            &charge,
        ) {
            Ok(disposition) => disposition,
            Err(error) => {
                let exact_error = error.to_string();
                let gap_id =
                    format!("bi_structural_realization_api_gap_stage_{filler_step}::{instance_id}");
                realization_gap_ids.push(gap_id.clone());
                realizations.push(failed_realization_audit(
                    *registration_stage,
                    filler_step,
                    instance_id,
                    contract.constructor.slug(),
                    filler_source.candidate_hash.clone(),
                    charge.ordinary_kappa,
                    charge.certified_nu,
                    true,
                    true,
                    wrong_provider_rejected,
                    charge_swap_rejected_by_authoritative_join,
                    gap_id,
                    &exact_error,
                ));
                continue;
            }
        };
        let replay = future_v2::replay_structural_realization_v2(
            &jurisdiction_prefix,
            registration,
            filler_step,
            &filler_source.telescope,
            &charge,
            &disposition,
        );
        let (
            provider_relation_satisfied,
            constructor_live_before_filler,
            constructor_absent_after_filler,
            specialization_replayed,
            marginal_charge_zero,
            ordinary_charge_preserved,
            realization_or_gap_hash,
            gap_id,
        ) = match &disposition {
            future_v2::FutureHoleRealizationDispositionV2::Realized(value) => (
                value.provider_relation_satisfied,
                value.constructor_live_before_filler,
                value.constructor_absent_after_filler,
                value.specialized_expression_is_filler_reference
                    && value.clause4_prime_instantiation_replayed,
                value.discharge_marginal_charge.replays_as_zero(),
                value.filler_ordinary_charge == charge,
                value.realization_hash.clone(),
                None,
            ),
            future_v2::FutureHoleRealizationDispositionV2::Gap(gap) => {
                realization_gap_ids.push(gap.id.clone());
                (
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    gap.gap_hash.clone(),
                    Some(gap.id.clone()),
                )
            }
        };
        let mut audit = BranchStructuralRealizationAudit {
            registration_stage: *registration_stage,
            jurisdiction_stage: filler_step,
            a3_instance_id: instance_id.clone(),
            constructor: contract.constructor.slug().to_owned(),
            filler_candidate_hash: filler_source.candidate_hash.clone(),
            ordinary_kappa: charge.ordinary_kappa,
            certified_nu: charge.certified_nu,
            ordinary_charge_provenance_authoritative: true,
            ordinary_charge_joins_branch_step,
            deterministic_replay_valid: replay.valid,
            provider_relation_satisfied,
            constructor_live_before_filler,
            constructor_absent_after_filler,
            specialization_replayed,
            marginal_charge_zero,
            ordinary_charge_preserved,
            wrong_provider_rejected,
            charge_swap_rejected_by_authoritative_join,
            realization_or_gap_hash,
            gap_id,
            derivation_hash: String::new(),
        };
        audit.derivation_hash = tagged_hash("structural-realization-audit", &audit);
        realizations.push(audit);
    }
    realizations.sort_by(|left, right| {
        (left.registration_stage, left.a3_instance_id.as_str())
            .cmp(&(right.registration_stage, right.a3_instance_id.as_str()))
    });
    realization_gap_ids.sort();
    let every_structural_hole_realized = realization_gap_ids.is_empty()
        && realizations.len() == structural_registered.len()
        && realizations.iter().all(|row| {
            row.ordinary_charge_joins_branch_step
                && row.deterministic_replay_valid
                && row.provider_relation_satisfied
                && row.constructor_live_before_filler
                && row.constructor_absent_after_filler
                && row.specialization_replayed
                && row.marginal_charge_zero
                && row.ordinary_charge_preserved
        });
    let every_wrong_provider_rejected =
        !realizations.is_empty() && realizations.iter().all(|row| row.wrong_provider_rejected);
    let every_charge_swap_rejected = !realizations.is_empty()
        && realizations
            .iter()
            .all(|row| row.charge_swap_rejected_by_authoritative_join);

    let stage3 = registrations
        .iter()
        .find(|row| row.stage == 3 && row.rule_constructor == "structural_completion_hole");
    let stage4 = registrations
        .iter()
        .find(|row| row.stage == 4 && row.rule_constructor == "structural_completion_hole");
    let stage3_realization = realizations.iter().find(|row| row.registration_stage == 3);
    let stage4_realization = realizations.iter().find(|row| row.registration_stage == 4);
    let stage3_wrinkle_reproduced = stage3
        .zip(stage4)
        .zip(stage3_realization.zip(stage4_realization))
        .is_some_and(|((left, right), (left_realized, right_realized))| {
            left.natural_family_id == right.natural_family_id
                && left.occurrence_id != right.occurrence_id
                && left_realized.gap_id.is_none()
                && right_realized.gap_id.is_none()
                && left_realized.constructor == "former_eliminator"
                && right_realized.constructor == "former_eliminator"
                && left_realized.jurisdiction_stage == 4
                && right_realized.jurisdiction_stage == 4
                && left_realized.filler_candidate_hash == right_realized.filler_candidate_hash
                && windows
                    .iter()
                    .find(|row| row.stage == 3)
                    .is_some_and(|row| {
                        row.required_packages == ["former_eliminator"]
                            && row.projected_focus.is_none()
                            && row.demand_precedes_jurisdiction
                    })
        });
    let focus_projection_reproduces_branch_ladder = windows
        .iter()
        .all(|row| row.focus_projection_matches_branch_ledger)
        && stage3_wrinkle_reproduced;

    let full_signature = exact_prefix(input, successor_stage);
    let final_window = generated.get(&successor_stage).expect("generated above");
    let mut membership = Vec::new();
    let mut membership_negative_control_trials = 0_usize;
    let mut membership_negative_control_rejections = 0_usize;
    let mut membership_gap_ids = Vec::new();
    let mut final_direct_chronological_count = 0_usize;
    let mut final_pointwise_chronological_count = 0_usize;
    for instance in &final_window.instances {
        let scheme = scheme_for_instance(final_window, instance)?;
        if scheme.rule_constructor == A3RuleConstructor::ChronologicalComparison {
            match issue_branch_chronological_membership_audit(
                &full_signature,
                input.halt_step,
                final_window,
                scheme,
                instance,
            ) {
                Ok((row, direct)) => {
                    if direct {
                        final_direct_chronological_count += 1;
                    } else {
                        final_pointwise_chronological_count += 1;
                    }
                    membership_negative_control_trials += 1;
                    if row.source_or_evidence_swap_rejected {
                        membership_negative_control_rejections += 1;
                    }
                    membership.push(row);
                }
                Err(reason) => {
                    membership_gap_ids.push(format!(
                        "bi_chronological_membership_gap::{}::{reason}",
                        instance.instance_id
                    ));
                    let direct = matches!(
                        scheme.required_output,
                        A3DemandOutputType::ChronologicalInteraction {
                            interface_mode: A3ChronologicalInterfaceMode::DirectType,
                            ..
                        }
                    );
                    if direct {
                        final_direct_chronological_count += 1;
                    } else {
                        final_pointwise_chronological_count += 1;
                    }
                    membership.push(finish_membership_audit(
                        instance,
                        scheme,
                        tagged_hash(
                            "underdetermined-chronological-instance",
                            &(instance.instance_id.as_str(), reason.as_str()),
                        ),
                        false,
                        false,
                        false,
                        "none".to_owned(),
                        false,
                        false,
                        instance.identity_or_uniform_specialization,
                        instance.independently_exported_demand,
                        true,
                        false,
                        Some(reason),
                        true,
                    ));
                }
            }
            continue;
        }
        let (
            evidence_hash,
            hypothetical_derivation_replayed,
            output_kernel_typed,
            internal_closure_preimage,
            internality_rule_id,
            restricted_candidate_classifier_reported_marginal,
            exact_substitution_from_sealed_preimage_replayed,
            uniform_specialization_not_new_family,
            independently_exported_output_orbit,
            marginal_charge_zero_or_not_applicable,
            source_or_evidence_swap_rejected,
            exact_reason,
        ) = match scheme.rule_constructor {
            A3RuleConstructor::UnaryAction => {
                let Some(registration) = unary_registered.get(&instance.instance_id) else {
                    let reason = format!(
                        "final unary instance {} has no replayed branch-local registration",
                        instance.instance_id
                    );
                    membership_gap_ids.push(format!(
                        "bi_unary_membership_registration_gap::{}::{reason}",
                        instance.instance_id
                    ));
                    let mut row = BranchDMembershipAudit {
                        a3_instance_id: instance.instance_id.clone(),
                        a3_scheme_id: scheme.scheme_id.clone(),
                        rule_constructor: rule_name(scheme.rule_constructor).to_owned(),
                        evidence_hash: tagged_hash(
                            "underdetermined-unary-instance",
                            &(instance.instance_id.as_str(), reason.as_str()),
                        ),
                        hypothetical_derivation_replayed: false,
                        output_kernel_typed: false,
                        internal_closure_preimage: false,
                        internality_rule_id: "none".to_owned(),
                        restricted_candidate_classifier_reported_marginal: false,
                        exact_substitution_from_sealed_preimage_replayed: false,
                        uniform_specialization_not_new_family: instance
                            .identity_or_uniform_specialization,
                        independently_exported_output_orbit: instance.independently_exported_demand,
                        marginal_charge_zero_or_not_applicable: false,
                        source_or_evidence_swap_rejected: false,
                        disposition: BranchDMembershipDisposition::Underdetermined,
                        exact_reason: Some(reason),
                        derivation_hash: String::new(),
                    };
                    row.derivation_hash = tagged_hash("d-membership-audit", &row);
                    row.disposition = BranchDMembershipDisposition::IssuerGap;
                    row.derivation_hash.clear();
                    row.derivation_hash = tagged_hash("d-membership-audit", &row);
                    membership.push(row);
                    continue;
                };
                let exact_source_join = instance.source_anchor_ids.len() == 1
                    && source_for_anchor(final_window, &instance.source_anchor_ids[0]).is_ok_and(
                        |source| {
                            source.canonical_family_key == instance.source_family_keys[0]
                                && registration.a3_instance_id == instance.instance_id
                                && registration.a3_scheme_id == scheme.scheme_id
                        },
                    );
                let exact_source_replayed = unary_exact_source_replays(registration);
                let exact_body_binding_replayed = unary_exact_body_binding_replays(registration);
                let total = registration
                    .parametric_internality
                    .dependent_totality_theorem_replayed
                    && registration
                        .parametric_internality
                        .total_specialization_authoritative;
                // Clause 2--3 of the adopted future-hole definition uses the
                // exact contextual source/body/totality theorem.  It does not
                // require the optional legacy closure-probe derivation.
                let contextual_internality_replayed =
                    exact_source_replayed && exact_body_binding_replayed;
                let mut forged_registration = registration.clone();
                forged_registration
                    .formation_hash
                    .push_str("::forged-membership-evidence");
                let forged_disposition =
                    future_v2::FutureHoleRegistrationDispositionV2::Registered(forged_registration);
                let forged_replay = future_v2::replay_future_hole_registration_v2(
                    &full_signature,
                    final_window,
                    scheme,
                    instance,
                    &forged_disposition,
                );
                let source_or_evidence_swap_rejected = exact_source_join && !forged_replay.valid;
                membership_negative_control_trials += 1;
                if source_or_evidence_swap_rejected {
                    membership_negative_control_rejections += 1;
                }
                (
                    registration.formation_hash.clone(),
                    total
                        && exact_source_join
                        && contextual_internality_replayed
                        && registration.local_semantic_scope_premises_satisfied,
                    total
                        && contextual_internality_replayed
                        && registration.explicit_body_elaboration.kernel_ty
                            == registration.expected_kernel_type
                        && registration
                            .ambient_declaration
                            .typed_body_elaboration
                            .kernel_ty
                            == registration.expected_kernel_type,
                    contextual_internality_replayed,
                    "future-hole-hypothesis-definition-v1+dependent-ambient-context-v1+exact-source-body-totality-v2".to_owned(),
                    false,
                    false,
                    instance.identity_or_uniform_specialization,
                    instance.independently_exported_demand,
                    registration.hole_marginal_charge.replays_as_zero(),
                    source_or_evidence_swap_rejected,
                    None,
                )
            }
            A3RuleConstructor::ChronologicalComparison => {
                unreachable!("chronological membership is handled by its exact replay path above")
            }
            A3RuleConstructor::HigherOpenBoxReduction => {
                let reason = "typed higher-open-box output is demanded but no branch-local derivation was issued".to_owned();
                membership_gap_ids.push(format!(
                    "bi_higher_open_box_membership_gap::{}::{reason}",
                    instance.instance_id
                ));
                (
                    tagged_hash("underdetermined-higher-instance", &instance.instance_id),
                    false,
                    false,
                    false,
                    "none".to_owned(),
                    false,
                    false,
                    instance.identity_or_uniform_specialization,
                    instance.independently_exported_demand,
                    true,
                    false,
                    Some(reason),
                )
            }
            A3RuleConstructor::StructuralCompletionHole => {
                let reason =
                    "a structural demand remains live in the proposed halt window".to_owned();
                membership_gap_ids.push(format!(
                    "bi_structural_membership_gap::{}::{reason}",
                    instance.instance_id
                ));
                (
                    tagged_hash("underdetermined-structural-instance", &instance.instance_id),
                    false,
                    false,
                    false,
                    "none".to_owned(),
                    false,
                    false,
                    instance.identity_or_uniform_specialization,
                    instance.independently_exported_demand,
                    true,
                    false,
                    Some(reason),
                )
            }
        };
        membership.push(finish_membership_audit(
            instance,
            scheme,
            evidence_hash,
            hypothetical_derivation_replayed,
            output_kernel_typed,
            internal_closure_preimage,
            internality_rule_id,
            restricted_candidate_classifier_reported_marginal,
            exact_substitution_from_sealed_preimage_replayed,
            uniform_specialization_not_new_family,
            independently_exported_output_orbit,
            marginal_charge_zero_or_not_applicable,
            source_or_evidence_swap_rejected,
            exact_reason,
            matches!(
                scheme.rule_constructor,
                A3RuleConstructor::HigherOpenBoxReduction
            ),
        ));
    }
    membership.sort_by(|left, right| left.a3_instance_id.cmp(&right.a3_instance_id));
    let membership_ids = membership
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let final_ids = final_window
        .instances
        .iter()
        .map(|row| row.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let issuer_gap_instance_ids = membership
        .iter()
        .filter(|row| matches!(row.disposition, BranchDMembershipDisposition::IssuerGap))
        .map(|row| row.a3_instance_id.clone())
        .collect::<Vec<_>>();
    let exact_d_partition = issuer_gap_instance_ids.is_empty()
        && membership.len() == membership_ids.len()
        && membership_ids == final_ids
        && membership
            .iter()
            .all(|row| !row.evidence_hash.is_empty() && !row.derivation_hash.is_empty());
    let every_membership_source_or_evidence_swap_rejected = membership_negative_control_trials > 0
        && membership_negative_control_rejections == membership_negative_control_trials;
    let derivable_instance_count = membership
        .iter()
        .filter(|row| matches!(row.disposition, BranchDMembershipDisposition::Derivable))
        .count();
    let underdetermined_instance_ids = membership
        .iter()
        .filter(|row| {
            matches!(
                row.disposition,
                BranchDMembershipDisposition::Underdetermined
            )
        })
        .map(|row| row.a3_instance_id.clone())
        .collect::<Vec<_>>();
    let final_count = |rule| {
        final_window
            .instances
            .iter()
            .filter(|instance| {
                scheme_for_instance(final_window, instance)
                    .is_ok_and(|scheme| scheme.rule_constructor == rule)
            })
            .count()
    };
    let final_unary_count = final_count(A3RuleConstructor::UnaryAction);
    let final_higher_count = final_count(A3RuleConstructor::HigherOpenBoxReduction);
    let final_structural_count = final_count(A3RuleConstructor::StructuralCompletionHole);
    let semantic_successor_o_empty = every_window_inventory_exhaustive
        && exact_d_partition
        && underdetermined_instance_ids.is_empty()
        && proofs
            .get(&successor_stage)
            .is_some_and(|proof| proof.required_packages_from_raw_debt.is_empty())
        && final_window.constructor_evidence.is_empty()
        && input.terminal_required_packages.is_empty();
    let f1_executed = every_window_inventory_exhaustive
        && every_registration_replayed_with_dependent_totality
        && every_structural_hole_realized
        && focus_projection_reproduces_branch_ladder
        && exact_d_partition;
    let f1_triggered = f1_executed && !underdetermined_instance_ids.is_empty();
    let f1_excluded = f1_executed && underdetermined_instance_ids.is_empty();
    // Counterfactual negative control: remove one *actual* derived instance
    // from D, recompute the partition predicates, and require F1 to trigger
    // while successor-O ceases to be empty.  This exercises the real row and
    // the real O/F1 equations; it is not the tautology "1 > 0".
    let f1_counterfactual = if f1_executed {
        membership
            .iter()
            .find(|row| {
                row.rule_constructor == "chronological_comparison"
                    && matches!(row.disposition, BranchDMembershipDisposition::Derivable)
            })
            .and_then(|original| {
                let instance = final_window
                    .instances
                    .iter()
                    .find(|instance| instance.instance_id == original.a3_instance_id)?;
                let scheme = scheme_for_instance(final_window, instance).ok()?;
                let original_authoritative_replay = replay_branch_chronological_membership_audit(
                    &full_signature,
                    input.halt_step,
                    final_window,
                    scheme,
                    instance,
                    original,
                )
                .is_empty();
                let mut counterfactual_rows = membership.clone();
                let counterfactual_membership = {
                    let mutated = counterfactual_rows
                        .iter_mut()
                        .find(|row| row.a3_instance_id == original.a3_instance_id)
                        .expect("selected actual membership row remains present");
                    mutated.hypothetical_derivation_replayed = false;
                    mutated.internal_closure_preimage = false;
                    mutated.exact_substitution_from_sealed_preimage_replayed = false;
                    mutated.disposition = BranchDMembershipDisposition::Underdetermined;
                    mutated.exact_reason = Some(
                        "F1 negative control deliberately withholds this actual derivation"
                            .to_owned(),
                    );
                    mutated.derivation_hash.clear();
                    mutated.derivation_hash = tagged_hash("d-membership-audit", mutated);
                    mutated.clone()
                };
                let counterfactual_membership_hash =
                    counterfactual_membership.derivation_hash.clone();
                let authoritative_membership_reissue_rejected = original_authoritative_replay
                    && !replay_branch_chronological_membership_audit(
                        &full_signature,
                        input.halt_step,
                        final_window,
                        scheme,
                        instance,
                        &counterfactual_membership,
                    )
                    .is_empty();
                let counterfactual_underdetermined_count = counterfactual_rows
                    .iter()
                    .filter(|row| {
                        matches!(
                            row.disposition,
                            BranchDMembershipDisposition::Underdetermined
                        )
                    })
                    .count();
                let counterfactual_f1_triggered =
                    f1_executed && counterfactual_underdetermined_count > 0;
                let counterfactual_semantic_successor_o_empty = every_window_inventory_exhaustive
                    && exact_d_partition
                    && counterfactual_underdetermined_count == 0
                    && final_window.constructor_evidence.is_empty()
                    && input.terminal_required_packages.is_empty();
                let mut audit = BranchF1CounterfactualAudit {
                    mutated_instance_id: original.a3_instance_id.clone(),
                    original_membership_hash: original.derivation_hash.clone(),
                    counterfactual_membership_hash,
                    counterfactual_membership: Box::new(counterfactual_membership),
                    authoritative_membership_reissue_rejected,
                    counterfactual_underdetermined_count,
                    counterfactual_f1_triggered,
                    counterfactual_semantic_successor_o_empty,
                    derivation_hash: String::new(),
                };
                audit.derivation_hash = tagged_hash("f1-counterfactual-audit", &audit);
                Some(audit)
            })
    } else {
        None
    };
    let f1_counterfactual_underdetermination_detected =
        f1_counterfactual.as_ref().is_some_and(|audit| {
            audit.original_membership_hash != audit.counterfactual_membership_hash
                && audit.counterfactual_membership.derivation_hash
                    == audit.counterfactual_membership_hash
                && audit.counterfactual_membership.a3_instance_id == audit.mutated_instance_id
                && audit.authoritative_membership_reissue_rejected
                && audit.counterfactual_underdetermined_count > 0
                && audit.counterfactual_f1_triggered
                && !audit.counterfactual_semantic_successor_o_empty
        });
    let theorem12_full_instance_granularity_proved = f1_excluded && semantic_successor_o_empty;
    let expressivity_gaps = registration_gap_ids
        .iter()
        .chain(&realization_gap_ids)
        .chain(&membership_gap_ids)
        .cloned()
        .collect::<Vec<_>>();
    let e5_class_complete = theorem12_full_instance_granularity_proved
        && expressivity_gaps.is_empty()
        && continuation_provenance_join_exact
        && every_registration_evidence_mutation_rejected
        && every_membership_source_or_evidence_swap_rejected
        && f1_counterfactual_underdetermination_detected
        && every_wrong_provider_rejected
        && every_charge_swap_rejected;
    let structural_realization_count = realizations
        .iter()
        .filter(|row| row.gap_id.is_none())
        .count();
    let mut audit = BranchFinaleAudit {
        schema: BI_BRANCH_FINALE_SCHEMA.to_owned(),
        branch_root_hash: input.branch_root_hash.clone(),
        continuation_digest: input.continuation_digest.clone(),
        input_binding_hash: branch_finale_input_binding_hash(input),
        continuation_provenance_join_exact,
        lawful_engine_stage_limit: input.lawful_engine_stage_limit,
        halt_step: input.halt_step,
        successor_stage,
        input_contiguous_and_self_consistent,
        windows,
        every_window_inventory_exhaustive,
        inventory_scope: "relative_to_adopted_full_A3_depth_two_rule_inventory".to_owned(),
        absolute_semantic_exhaustiveness_claimed: false,
        registrations,
        unary_registration_count: unary_registered.len(),
        structural_registration_count: structural_registered.len(),
        registration_gap_ids,
        every_registration_replayed_with_dependent_totality,
        every_registration_evidence_mutation_rejected,
        realizations,
        structural_realization_count,
        realization_gap_ids,
        every_structural_hole_realized,
        every_wrong_provider_rejected,
        every_charge_swap_rejected,
        stage3_wrinkle_reproduced,
        focus_projection_reproduces_branch_ladder,
        final_a3_inventory_count: final_window.instances.len(),
        final_unary_count,
        final_direct_chronological_count,
        final_pointwise_chronological_count,
        final_higher_count,
        final_structural_count,
        membership,
        exact_d_partition,
        every_membership_source_or_evidence_swap_rejected,
        derivable_instance_count,
        underdetermined_instance_ids,
        issuer_gap_instance_ids,
        semantic_successor_o_empty,
        f1_executed,
        f1_triggered,
        f1_excluded,
        f1_counterfactual,
        f1_counterfactual_underdetermination_detected,
        theorem12_full_instance_granularity_proved,
        theorem12_scope: "full_instance_granularity_within_adopted_A3_only".to_owned(),
        broader_absolute_theorem12_claimed: false,
        expressivity_gaps,
        e5_class_complete,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("branch-finale-audit", &audit);
    Ok(audit)
}

/// Replay the continuation and execute its exact branch-local finale.
pub fn execute_certified_branch_finale(
    cone: &CertifiedStage4BranchCone,
    continuation: &BranchContinuation,
) -> Result<BranchFinaleAudit, String> {
    let input = issue_branch_finale_input(cone, continuation)?;
    execute_branch_finale_inner(&input, true)
}

/// Reissue a certified finale.  Recomputing only the outer digest cannot make
/// a mutated input or audit pass because the continuation and every semantic
/// row are reconstructed again.
pub fn replay_certified_branch_finale(
    cone: &CertifiedStage4BranchCone,
    continuation: &BranchContinuation,
    claimed: &BranchFinaleAudit,
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut projection = claimed.clone();
    projection.derivation_hash.clear();
    if claimed.schema != BI_BRANCH_FINALE_SCHEMA {
        errors.push("branch finale schema mismatch".to_owned());
    }
    if claimed.derivation_hash != tagged_hash("branch-finale-audit", &projection) {
        errors.push("branch finale derivation hash mismatch".to_owned());
    }
    match execute_certified_branch_finale(cone, continuation) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("branch finale differs from exact reissuance".to_owned()),
        Err(error) => errors.push(format!("branch finale reissuance failed: {error}")),
    }
    errors
}

/// Replay a raw, explicitly bound input.  This proves deterministic semantic
/// reissuance and is used for mutation controls; the result correctly retains
/// `continuation_provenance_join_exact = false` and therefore cannot complete
/// E-5 on its own.
pub fn replay_branch_finale_input(
    input: &BranchFinaleInput,
    claimed: &BranchFinaleAudit,
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut projection = claimed.clone();
    projection.derivation_hash.clear();
    if claimed.derivation_hash != tagged_hash("branch-finale-audit", &projection) {
        errors.push("branch finale derivation hash mismatch".to_owned());
    }
    match execute_branch_finale(input) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("branch finale differs from exact input reissuance".to_owned()),
        Err(error) => errors.push(format!("branch finale input reissuance failed: {error}")),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::branch_invariance::{
        BranchContinuationLimits, execute_branch_continuation, issue_certified_stage4_branch_cone,
    };

    fn raw_fixture() -> (BranchFinaleInput, BranchFinaleAudit) {
        let telescopes = (1..=15)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<BTreeMap<_, _>>();
        let mut steps = Vec::new();
        for stage in 1..=15 {
            let prefix = SealedSignature::from_telescopes(
                (1..stage)
                    .map(|older| (older, telescopes[&older].clone()))
                    .collect(),
            );
            let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(&prefix, stage)
                .expect("exact window proof");
            let telescope = telescopes[&stage].clone();
            steps.push(BranchFinaleStepInput {
                stage,
                candidate_hash: candidate_hash(&telescope),
                kappa: telescope.kappa() as u32,
                certified_nu: stage,
                required_packages_before_selection: proof.required_packages_from_raw_debt,
                step_evidence_hash: tagged_hash("test-step-evidence", &(stage, &telescope)),
                ordinary_family_token_hashes: Vec::new(),
                ordinary_charge_provenance_authoritative: false,
                telescope,
            });
        }
        let terminal_prefix = SealedSignature::from_telescopes(
            (1..=15)
                .map(|stage| (stage, telescopes[&stage].clone()))
                .collect(),
        );
        let terminal = prove_a3_window_inventory_for_exact_prefix_unbounded(&terminal_prefix, 16)
            .expect("successor proof");
        let input = BranchFinaleInput {
            branch_root_hash: candidate_hash(&telescopes[&4]),
            continuation_digest: tagged_hash("test-continuation", &steps),
            lawful_engine_stage_limit: 20,
            halt_step: 15,
            terminal_required_packages: terminal.required_packages_from_raw_debt,
            steps,
        };
        let audit = execute_branch_finale(&input).expect("raw finale");
        (input, audit)
    }

    fn rehash_audit(audit: &mut BranchFinaleAudit) {
        audit.derivation_hash.clear();
        audit.derivation_hash = tagged_hash("branch-finale-audit", audit);
    }

    #[test]
    fn branch_generic_finale_replays_and_records_chronological_closure_gaps() {
        let (input, audit) = raw_fixture();
        assert!(replay_branch_finale_input(&input, &audit).is_empty());
        assert!(!audit.continuation_provenance_join_exact);
        assert!(audit.every_window_inventory_exhaustive);
        assert_eq!(audit.expressivity_gaps.len(), 85);
        let chronological_gaps = audit
            .expressivity_gaps
            .iter()
            .filter(|gap| gap.contains(BI_CHRONOLOGICAL_INTERFACE_SLOT_MAP_UNDECLARED_V1))
            .collect::<Vec<_>>();
        assert!(chronological_gaps.is_empty());
        assert_eq!(
            audit
                .expressivity_gaps
                .iter()
                .filter(|gap| gap.contains("BI_CHRONOLOGICAL_"))
                .count(),
            72
        );
        assert_eq!(
            audit
                .expressivity_gaps
                .iter()
                .filter(|gap| gap
                    .contains(BI_STRUCTURAL_FILLER_ORDINARY_CHARGE_PROVENANCE_UNCERTIFIED_V1))
                .count(),
            13
        );
        let derived_chronological = audit
            .membership
            .iter()
            .filter(|row| {
                row.rule_constructor == "chronological_comparison"
                    && matches!(row.disposition, BranchDMembershipDisposition::Derivable)
            })
            .collect::<Vec<_>>();
        assert!(derived_chronological.is_empty());
        assert!(!audit.exact_d_partition);
        assert!(audit.every_registration_evidence_mutation_rejected);
        assert_eq!(audit.structural_realization_count, 0);
        assert_eq!(audit.realization_gap_ids.len(), 13);
        assert!(audit.realizations.iter().all(|row| {
            !row.ordinary_charge_provenance_authoritative
                && !row.ordinary_charge_joins_branch_step
                && row.gap_id.as_deref().is_some_and(|gap| {
                    gap.contains(BI_STRUCTURAL_FILLER_ORDINARY_CHARGE_PROVENANCE_UNCERTIFIED_V1)
                })
        }));
        assert!(!audit.every_structural_hole_realized);
        assert!(!audit.every_wrong_provider_rejected);
        assert!(!audit.every_charge_swap_rejected);
        assert!(!audit.stage3_wrinkle_reproduced);
        assert!(!audit.focus_projection_reproduces_branch_ladder);
        assert!(audit.every_membership_source_or_evidence_swap_rejected);
        assert_eq!(audit.issuer_gap_instance_ids.len(), 72);
        assert!(audit.f1_counterfactual.is_none());
        assert!(!audit.f1_counterfactual_underdetermination_detected);
        assert!(
            !audit.e5_class_complete,
            "raw input cannot complete certified E-5"
        );
    }

    #[test]
    #[ignore = "F-BI requires a sealed passing BI-0 artifact before any branch-final certificate run"]
    fn real_cone_finale_completes_with_act_local_charge_provenance_after_bi0() {
        let cone = issue_certified_stage4_branch_cone().expect("certified cone");
        let limits = BranchContinuationLimits {
            max_inspected_stage: 20,
            max_enumerated_candidates_per_stage: 1_000_000,
        };
        let continuation =
            execute_branch_continuation(&cone, &cone.branches[0].candidate_hash, &limits)
                .expect("branch continuation");
        let audit =
            execute_certified_branch_finale(&cone, &continuation).expect("certified branch finale");
        assert!(replay_certified_branch_finale(&cone, &continuation, &audit).is_empty());
        assert!(audit.continuation_provenance_join_exact);
        assert!(audit.expressivity_gaps.is_empty());
        let chronological_gaps = audit
            .expressivity_gaps
            .iter()
            .filter(|gap| gap.contains(BI_CHRONOLOGICAL_INTERFACE_SLOT_MAP_UNDECLARED_V1))
            .collect::<Vec<_>>();
        assert!(chronological_gaps.is_empty());
        assert!(audit.realization_gap_ids.is_empty());
        assert_eq!(audit.structural_realization_count, 13);
        assert!(audit.every_structural_hole_realized);
        assert!(audit.stage3_wrinkle_reproduced);
        assert!(audit.focus_projection_reproduces_branch_ladder);
        assert_eq!(audit.derivable_instance_count, 89);
        assert!(audit.underdetermined_instance_ids.is_empty());
        assert!(audit.issuer_gap_instance_ids.is_empty());
        let contextual_unary = audit
            .membership
            .iter()
            .find(|row| {
                row.a3_instance_id
                    == "blake3:e1799e2d36ee0629b58e1f2f9fb9e566e3df79e0f7f93556f5d6f1db7ad9667d"
            })
            .expect("registered contextual unary membership");
        assert_eq!(contextual_unary.rule_constructor, "unary_action");
        assert!(contextual_unary.hypothetical_derivation_replayed);
        assert!(contextual_unary.output_kernel_typed);
        assert!(contextual_unary.internal_closure_preimage);
        assert!(matches!(
            contextual_unary.disposition,
            BranchDMembershipDisposition::Derivable
        ));
        assert!(audit.exact_d_partition);
        assert!(audit.f1_executed);
        assert!(!audit.f1_triggered);
        assert!(audit.f1_excluded);
        assert!(audit.semantic_successor_o_empty);
        assert!(audit.e5_class_complete);
    }

    #[test]
    fn replay_rejects_rehashed_gap_membership_and_f1_mutations() {
        let (input, audit) = raw_fixture();

        let mut gap_mutation = audit.clone();
        gap_mutation.expressivity_gaps.push("forged_gap".to_owned());
        rehash_audit(&mut gap_mutation);
        assert!(!replay_branch_finale_input(&input, &gap_mutation).is_empty());

        let mut registration_mutation = audit.clone();
        registration_mutation.registrations[0].gap_id = Some("forged_gap".to_owned());
        registration_mutation.registrations[0]
            .derivation_hash
            .clear();
        registration_mutation.registrations[0].derivation_hash = tagged_hash(
            "future-registration-audit",
            &registration_mutation.registrations[0],
        );
        rehash_audit(&mut registration_mutation);
        assert!(!replay_branch_finale_input(&input, &registration_mutation).is_empty());

        let mut realization_mutation = audit.clone();
        realization_mutation.realizations[0].gap_id = Some("forged_gap".to_owned());
        realization_mutation.realizations[0].derivation_hash.clear();
        realization_mutation.realizations[0].derivation_hash = tagged_hash(
            "structural-realization-audit",
            &realization_mutation.realizations[0],
        );
        rehash_audit(&mut realization_mutation);
        assert!(!replay_branch_finale_input(&input, &realization_mutation).is_empty());

        let mut membership_mutation = audit.clone();
        membership_mutation.membership[0].evidence_hash = "blake3:forged".to_owned();
        membership_mutation.membership[0].derivation_hash.clear();
        membership_mutation.membership[0].derivation_hash =
            tagged_hash("d-membership-audit", &membership_mutation.membership[0]);
        rehash_audit(&mut membership_mutation);
        assert!(!replay_branch_finale_input(&input, &membership_mutation).is_empty());

        let mut f1_mutation = audit;
        f1_mutation.f1_excluded = !f1_mutation.f1_excluded;
        f1_mutation.f1_triggered = !f1_mutation.f1_triggered;
        rehash_audit(&mut f1_mutation);
        assert!(!replay_branch_finale_input(&input, &f1_mutation).is_empty());
    }

    #[test]
    fn successor_beyond_recorded_engine_limit_is_an_artifact_gap() {
        let (mut input, _) = raw_fixture();
        input.lawful_engine_stage_limit = input.halt_step;
        let audit = execute_branch_finale(&input).expect("gap audit");
        assert!(!audit.e5_class_complete);
        assert_eq!(audit.expressivity_gaps.len(), 1);
        assert!(audit.expressivity_gaps[0].contains("exceeds_certified_engine_limit"));
    }
}
