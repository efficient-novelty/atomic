//! Relative exhaustiveness theorem for the operational A3 rule inventory.
//!
//! The theorem is deliberately a successor to, rather than an edit of,
//! `a3-historical-demand-grammar-v1`.  Its domain is independently generated:
//! the three pre-instance shapes from DEMAND-COMPLETE (unary action,
//! chronological binary comparison, and higher open-box reduction), plus the
//! adopted structural future-hole clause indexed by the raw directive-debt
//! predicates.  For every occurrence it reconstructs the raw two-step window
//! and proves exactly one of:
//!
//! * promotion to one operational typed instance, scheme, and quotient orbit;
//! * rejection by the typed eligibility predicate; or
//! * absence because the two-step window is incomplete.
//!
//! This is not an assertion that `A3RuleConstructor::ALL` is exhaustive.  The
//! target enum is used only after every independently generated source
//! occurrence has been joined, and every operational occurrence has been
//! consumed by the reverse join.  Structural seeds are independently rebuilt
//! with `summarize_structural_debt` and `required_packages_for`, then joined to
//! A3 constructor evidence and the adopted future-hole open judgment.

use crate::a3_demand_grammar::{
    A3DemandConstructor, A3DemandOutputType, A3DemandSchemeOrigin, A3HistoricalWindow,
    A3RuleConstructor, A3SeedRejectionReason, A3StructuralSnapshot, A3WindowLayer,
    generate_a3_window_for_prefix, issue_historical_a3_demand_grammar,
};
use crate::debt_guard::{WINDOW_DEPTH, required_packages_for};
use crate::demand_completeness::{
    DemandCompletenessCertificate, IntendedRuleSeedKind, IntendedWindowSpecification,
    build_demand_completeness_certificate,
};
use crate::future_hole_hypothesis::issue_structural_future_holes_for_window;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_schema::grammar_completion::{
    GrammarCompletionCertificate, replay_grammar_completion_certificate,
};
use pen_type::admissibility::StructuralFamily;
use pen_type::elaborate::{KernelTy, SealedSignature};
use pen_type::obligations::{StructuralDebt, summarize_structural_debt};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const A3_RULE_INVENTORY_EXHAUSTIVENESS_SCHEMA: &str = "a3-rule-inventory-exhaustiveness-v2";
pub const A3_RULE_INVENTORY_EXHAUSTIVENESS_DATE: &str = "2026-07-21";

/// The relative theorem does not silently turn the adopted historical grammar
/// into a theorem about every conceivable semantic schema language.
pub const A3_BROADER_SEMANTIC_SCOPE_GAP: &str =
    "A3_ABSOLUTE_RULE_SHAPE_EXHAUSTIVENESS_BEYOND_ADOPTED_DEPTH_TWO_GRAMMAR_NOT_PROVED";
pub const A3_COMPLETED_SCHEMA2_LIVE_REPLAY_DRIFT: &str =
    "A3_COMPLETED_SCHEMA2_ARCHIVE_LIVE_REPLAY_SOURCE_DRIFT";

const THIS_SOURCE_BYTES: &[u8] = include_bytes!("a3_rule_inventory_exhaustiveness.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("a3_demand_grammar.rs");
const DEMAND_COMPLETENESS_SOURCE_BYTES: &[u8] = include_bytes!("demand_completeness.rs");
const FUTURE_HOLE_SOURCE_BYTES: &[u8] = include_bytes!("future_hole_hypothesis.rs");
const FUTURE_HOLE_ADOPTION_BYTES: &[u8] =
    include_bytes!("../../../docs/future_hole_definition_adjudication.md");
const DEBT_GUARD_SOURCE_BYTES: &[u8] = include_bytes!("debt_guard.rs");
const OBLIGATIONS_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/obligations.rs");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const SCHEMA_COMPLETION_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/grammar_completion.rs");
const SCHEMA_COMPLETION_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_grammar_completion_v1.json");
const GLOBAL_E4_V10_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_global_e4_assembly_v10.json");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(A3_RULE_INVENTORY_EXHAUSTIVENESS_SCHEMA, domain, value))
        .expect("A3 exhaustiveness evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3ExhaustivenessSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

fn source_bindings() -> Vec<A3ExhaustivenessSourceBinding> {
    [
        (
            "crates/pen-eval/src/a3_rule_inventory_exhaustiveness.rs",
            "relative_exhaustiveness_issuer_and_replay",
            THIS_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/demand_completeness.rs",
            "independent_unary_binary_higher_seed_grammar",
            DEMAND_COMPLETENESS_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "operational_target_grammar_v1_unchanged",
            A3_SOURCE_BYTES,
        ),
        (
            "docs/future_hole_definition_adjudication.md",
            "adopted_structural_future_hole_shape",
            FUTURE_HOLE_ADOPTION_BYTES,
        ),
        (
            "crates/pen-eval/src/future_hole_hypothesis.rs",
            "motive_typed_future_hole_issuer",
            FUTURE_HOLE_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/debt_guard.rs",
            "independent_raw_required_package_projection",
            DEBT_GUARD_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/obligations.rs",
            "raw_structural_debt_predicates",
            OBLIGATIONS_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "independent_twelve_structural_family_inventory",
            ADMISSIBILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/grammar_completion.rs",
            "live_completed_schema2_typing_normalization_naturality_premise",
            SCHEMA_COMPLETION_SOURCE_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "frozen_completed_schema2_registration_surface",
            SCHEMA_COMPLETION_ARTIFACT_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v10.json",
            "frozen_wrapped_domain_class_exhaustion_scope",
            GLOBAL_E4_V10_ARTIFACT_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| A3ExhaustivenessSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IndependentA3RuleShape {
    UnaryAction,
    ChronologicalBinaryComparison,
    HigherOpenBoxReduction,
    StructuralFutureCompletionHole,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3RuleShapeMapping {
    pub intended_shape: IndependentA3RuleShape,
    pub independent_definition_source: String,
    pub operational_rule_constructor: String,
    pub shape_is_parameterized_by_typed_source_families: bool,
    pub shape_is_parameterized_by_structural_family: bool,
    pub unique_operational_image: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawClauseCoordinate {
    pub step: u32,
    pub candidate_hash: String,
    pub clause_index: u16,
    pub declared_role: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RawIndependentRuleSeed {
    seed_id: String,
    legacy_seed_id: Option<String>,
    kind: IntendedRuleSeedKind,
    sources: Vec<RawClauseCoordinate>,
}

fn bool_is_false(value: &bool) -> bool {
    !*value
}

fn usize_is_zero(value: &usize) -> bool {
    *value == 0
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3BaseSeedDispositionProof {
    pub independent_seed_id: String,
    pub legacy_demand_completeness_seed_id: Option<String>,
    pub independent_kind: String,
    pub mapped_rule_constructor: String,
    pub raw_sources: Vec<RawClauseCoordinate>,
    pub typed_source_anchor_ids: Vec<String>,
    pub every_source_joined_to_exact_typed_public_clause: bool,
    pub eligible: bool,
    pub rejection_reason: Option<String>,
    pub promoted_instance_id: Option<String>,
    pub promoted_scheme_id: Option<String>,
    pub base_rule_evidence_hash: Option<String>,
    pub quotient_orbit_id: Option<String>,
    pub required_output_shape_replayed: bool,
    pub exactly_one_promotion_or_rejection: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3RuleKindAggregateProof {
    pub independent_kind: String,
    pub mapped_rule_constructor: String,
    pub raw_seed_count: usize,
    pub promoted_seed_count: usize,
    pub rejected_seed_count: usize,
    pub operational_generated_instance_count: usize,
    pub operational_disposition_exactly_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3StructuralSeedDispositionProof {
    pub structural_family: String,
    pub mapped_demand_constructor: String,
    pub mapped_rule_constructor: String,
    pub raw_debt_predicate_holds: bool,
    pub required_packages_projection_contains_family: bool,
    pub rejected_by_raw_debt_predicate: bool,
    pub constructor_evidence_hash: Option<String>,
    pub structural_scheme_id: Option<String>,
    pub structural_instance_id: Option<String>,
    pub structural_orbit_id: Option<String>,
    pub future_hole_open_judgment_hash: Option<String>,
    pub future_hole_motive_typed_zero_charge: bool,
    pub exactly_one_promotion_or_rejection: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3WindowRuleInventoryProof {
    pub stage: u32,
    pub exact_prefix_signature_digest: String,
    pub newest_step: Option<u32>,
    pub older_step: Option<u32>,
    pub complete_two_step_window: bool,
    pub raw_window_rebuilt_without_focus_count_score_bar_or_winner: bool,
    pub raw_structural_debt_json: String,
    pub required_packages_from_raw_debt: Vec<String>,
    pub legacy_independent_specification_digest: Option<String>,
    pub legacy_seed_join_complete: bool,
    pub base_seed_dispositions: Vec<A3BaseSeedDispositionProof>,
    pub base_kind_aggregates: Vec<A3RuleKindAggregateProof>,
    pub structural_seed_dispositions: Vec<A3StructuralSeedDispositionProof>,
    pub independent_base_seed_count: usize,
    pub promoted_base_seed_count: usize,
    pub rejected_base_seed_count: usize,
    pub structural_family_candidate_count: usize,
    pub promoted_structural_seed_count: usize,
    pub rejected_structural_seed_count: usize,
    pub every_independent_seed_has_exactly_one_disposition: bool,
    #[serde(default, skip_serializing_if = "usize_is_zero")]
    pub independent_preimage_occurrence_count: usize,
    #[serde(default, skip_serializing_if = "usize_is_zero")]
    pub operational_instance_count: usize,
    #[serde(default, skip_serializing_if = "bool_is_false")]
    pub independent_preimage_ids_injective: bool,
    pub every_operational_instance_has_exactly_one_independent_preimage: bool,
    pub every_operational_scheme_and_orbit_is_consumed: bool,
    pub structural_future_hole_clause_coverage_complete: bool,
    /// The archived historical theorem used the original motive theorem.
    /// Exact-prefix branch proofs deliberately do not: their dependent
    /// totality is discharged later by `future_hole_hypothesis_v2` against
    /// the branch's own prefix.
    #[serde(default, skip_serializing_if = "bool_is_false")]
    pub legacy_future_hole_typing_consumed: bool,
    #[serde(default, skip_serializing_if = "bool_is_false")]
    pub future_hole_typing_deferred_to_branch_v2: bool,
    pub operational_seed_dispositions_replayed: bool,
    pub relative_rule_inventory_exhaustive_for_window: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedSchemaSourceCoverage {
    pub step: u32,
    pub clause_index: u16,
    pub registered_name: String,
    pub matching_a3_source_anchor_ids: Vec<String>,
    pub kernel_role_type_and_normal_form_match: bool,
    pub normalization_naturality_token_live: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3RuleInventoryExhaustivenessCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<A3ExhaustivenessSourceBinding>,
    pub independent_demand_completeness_digest: String,
    pub independent_seed_generator_replayed: bool,
    pub future_hole_adoption_replayed: bool,
    pub completed_schema2_archived_digest: String,
    pub completed_schema2_artifact_digest_verified: bool,
    pub completed_schema2_live_definition_replay_succeeded: bool,
    pub completed_schema2_live_replay_errors: Vec<String>,
    pub completed_schema_source_coverage: Vec<CompletedSchemaSourceCoverage>,
    pub every_completed_schema_source_is_an_a3_typed_parameter: bool,
    pub completed_schema_constructors_are_source_parameters_not_extra_a3_rule_shapes: bool,
    pub completed_schema2_live_replay_is_not_required_for_relative_a3_rule_enumeration: bool,
    pub global_e4_v10_archived_digest: String,
    pub global_e4_wrapped_domain_class_exhaustion_bound: bool,
    pub global_e4_no_unknown_survives_on_frozen_wrapped_surface: bool,
    pub global_e4_archive_scopes_the_relative_domain_but_does_not_generate_rule_shapes: bool,
    pub shape_mappings: Vec<A3RuleShapeMapping>,
    pub independent_shape_domain_derived_from_live_seed_occurrences_and_adopted_future_clause: bool,
    pub every_independent_shape_has_one_operational_image: bool,
    pub every_operational_rule_constructor_has_an_independent_preimage: bool,
    pub historical_windows: Vec<A3WindowRuleInventoryProof>,
    pub every_historical_window_covered: bool,
    pub every_raw_seed_promoted_or_rejected: bool,
    pub every_operational_occurrence_has_an_independent_preimage: bool,
    pub structural_future_hole_clauses_included: bool,
    pub relative_rule_constructor_inventory_exhaustiveness_proved: bool,
    pub broader_absolute_semantic_exhaustiveness_claimed: bool,
    pub named_scope_limits: Vec<String>,
    pub forbidden_count_score_bar_winner_or_halt_input: bool,
    pub outcome: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3RuleInventoryExhaustivenessReplay {
    pub valid: bool,
    pub historical_window_count: usize,
    pub base_seed_count: usize,
    pub structural_candidate_count: usize,
    pub relative_exhaustiveness_proved: bool,
    pub absolute_exhaustiveness_claimed: bool,
    pub named_scope_limits: Vec<String>,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum A3RuleInventoryExhaustivenessError {
    #[error("A3 generation failed: {0}")]
    A3(String),
    #[error("independent seed grammar failed: {0}")]
    Independent(String),
    #[error("completed Schema2 premise failed: {0}")]
    Schema(String),
    #[error("future-hole premise failed: {0}")]
    FutureHole(String),
    #[error("exhaustiveness invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate failed replay: {0}")]
    EmittedReplay(String),
}

fn intended_kind_name(kind: IntendedRuleSeedKind) -> &'static str {
    match kind {
        IntendedRuleSeedKind::UnaryAction => "unary_action",
        IntendedRuleSeedKind::BinaryComparison => "binary_comparison",
        IntendedRuleSeedKind::HigherOpenBoxReduction => "higher_open_box_reduction",
    }
}

fn intended_shape(kind: IntendedRuleSeedKind) -> IndependentA3RuleShape {
    match kind {
        IntendedRuleSeedKind::UnaryAction => IndependentA3RuleShape::UnaryAction,
        IntendedRuleSeedKind::BinaryComparison => {
            IndependentA3RuleShape::ChronologicalBinaryComparison
        }
        IntendedRuleSeedKind::HigherOpenBoxReduction => {
            IndependentA3RuleShape::HigherOpenBoxReduction
        }
    }
}

fn mapped_rule(kind: IntendedRuleSeedKind) -> A3RuleConstructor {
    match kind {
        IntendedRuleSeedKind::UnaryAction => A3RuleConstructor::UnaryAction,
        IntendedRuleSeedKind::BinaryComparison => A3RuleConstructor::ChronologicalComparison,
        IntendedRuleSeedKind::HigherOpenBoxReduction => A3RuleConstructor::HigherOpenBoxReduction,
    }
}

fn rule_name(rule: A3RuleConstructor) -> &'static str {
    match rule {
        A3RuleConstructor::UnaryAction => "unary_action",
        A3RuleConstructor::ChronologicalComparison => "chronological_comparison",
        A3RuleConstructor::HigherOpenBoxReduction => "higher_open_box_reduction",
        A3RuleConstructor::StructuralCompletionHole => "structural_completion_hole",
    }
}

fn structural_constructor(family: StructuralFamily) -> A3DemandConstructor {
    match family {
        StructuralFamily::FormerEliminator => A3DemandConstructor::FormerEliminator,
        StructuralFamily::InitialHit => A3DemandConstructor::InitialHit,
        StructuralFamily::TruncationHit => A3DemandConstructor::TruncationHit,
        StructuralFamily::HigherHit => A3DemandConstructor::HigherHit,
        StructuralFamily::SphereLift => A3DemandConstructor::SphereLift,
        StructuralFamily::AxiomaticBundle => A3DemandConstructor::AxiomaticBundle,
        StructuralFamily::ModalShell => A3DemandConstructor::ModalShell,
        StructuralFamily::ConnectionShell => A3DemandConstructor::ConnectionShell,
        StructuralFamily::CurvatureShell => A3DemandConstructor::CurvatureShell,
        StructuralFamily::OperatorBundle => A3DemandConstructor::OperatorBundle,
        StructuralFamily::HilbertFunctional => A3DemandConstructor::HilbertFunctional,
        StructuralFamily::TemporalShell => A3DemandConstructor::TemporalShell,
    }
}

fn structural_predicate(debt: StructuralDebt, family: StructuralFamily) -> bool {
    match family {
        StructuralFamily::FormerEliminator => debt.requires_former_eliminator_package(),
        StructuralFamily::InitialHit => debt.requires_initial_hit_package(),
        StructuralFamily::TruncationHit => debt.requires_truncation_hit_package(),
        StructuralFamily::HigherHit => debt.requires_higher_hit_package(),
        StructuralFamily::SphereLift => debt.requires_sphere_lift_package(),
        StructuralFamily::AxiomaticBundle => debt.requires_axiomatic_bundle_package(),
        StructuralFamily::ModalShell => debt.requires_modal_shell_package(),
        StructuralFamily::ConnectionShell => debt.requires_connection_shell_package(),
        StructuralFamily::CurvatureShell => debt.requires_curvature_shell_package(),
        StructuralFamily::OperatorBundle => debt.requires_operator_bundle_package(),
        StructuralFamily::HilbertFunctional => debt.requires_hilbert_functional_package(),
        StructuralFamily::TemporalShell => debt.requires_temporal_shell_package(),
    }
}

fn structural_debt_json(debt: StructuralDebt) -> String {
    serde_json::to_string(&A3StructuralSnapshot::from(debt))
        .expect("structural debt snapshot serializes")
}

fn exact_prefix_library(
    signature: &SealedSignature,
    stage: u32,
) -> Result<Library, A3RuleInventoryExhaustivenessError> {
    let mut library = Library::new();
    for step in 1..stage {
        let entry = signature.entry(step).ok_or_else(|| {
            A3RuleInventoryExhaustivenessError::Invariant(format!("exact prefix omits step {step}"))
        })?;
        library.push(LibraryEntry::from_telescope(&entry.telescope, &library));
    }
    Ok(library)
}

fn raw_coordinate(
    step: u32,
    candidate_hash: &str,
    clause_index: usize,
    role: pen_core::clause::ClauseRole,
) -> RawClauseCoordinate {
    RawClauseCoordinate {
        step,
        candidate_hash: candidate_hash.to_owned(),
        clause_index: u16::try_from(clause_index).expect("historical clause index fits u16"),
        declared_role: serde_json::to_string(&role).expect("clause role serializes"),
    }
}

fn raw_independent_seeds(
    signature: &SealedSignature,
    stage: u32,
) -> Result<Vec<RawIndependentRuleSeed>, A3RuleInventoryExhaustivenessError> {
    if stage < 3 {
        return Ok(Vec::new());
    }
    let older_step = stage - 2;
    let newest_step = stage - 1;
    let older_entry = signature.entry(older_step).ok_or_else(|| {
        A3RuleInventoryExhaustivenessError::Invariant(format!(
            "raw seed generator omits older step {older_step}"
        ))
    })?;
    let newest_entry = signature.entry(newest_step).ok_or_else(|| {
        A3RuleInventoryExhaustivenessError::Invariant(format!(
            "raw seed generator omits newest step {newest_step}"
        ))
    })?;
    let older = older_entry
        .telescope
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| {
            raw_coordinate(older_step, &older_entry.candidate_hash, index, clause.role)
        })
        .collect::<Vec<_>>();
    let newest = newest_entry
        .telescope
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| {
            raw_coordinate(
                newest_step,
                &newest_entry.candidate_hash,
                index,
                clause.role,
            )
        })
        .collect::<Vec<_>>();
    let all = older.iter().chain(&newest).cloned().collect::<Vec<_>>();
    let make_seed = |kind, sources: Vec<RawClauseCoordinate>| RawIndependentRuleSeed {
        seed_id: tagged_hash(
            "independent-raw-rule-seed",
            &(
                signature.digest(),
                stage,
                intended_kind_name(kind),
                &sources,
            ),
        ),
        legacy_seed_id: None,
        kind,
        sources,
    };
    let mut seeds = all
        .iter()
        .cloned()
        .map(|source| make_seed(IntendedRuleSeedKind::UnaryAction, vec![source]))
        .collect::<Vec<_>>();
    for left in &older {
        for right in &newest {
            seeds.push(make_seed(
                IntendedRuleSeedKind::BinaryComparison,
                vec![left.clone(), right.clone()],
            ));
        }
    }
    seeds.push(make_seed(IntendedRuleSeedKind::HigherOpenBoxReduction, all));
    Ok(seeds)
}

fn legacy_sources(
    specification: &IntendedWindowSpecification,
    source_ids: &[String],
) -> Result<Vec<RawClauseCoordinate>, A3RuleInventoryExhaustivenessError> {
    source_ids
        .iter()
        .map(|source_id| {
            let matches = specification
                .clause_seeds
                .iter()
                .filter(|source| &source.seed_id == source_id)
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(A3RuleInventoryExhaustivenessError::Invariant(format!(
                    "legacy source seed {source_id} has {} coordinate rows",
                    matches.len()
                )));
            }
            let source = matches[0];
            Ok(RawClauseCoordinate {
                step: source.step,
                candidate_hash: source.candidate_hash.clone(),
                clause_index: source.clause_index,
                declared_role: serde_json::to_string(&source.role).expect("clause role serializes"),
            })
        })
        .collect()
}

fn join_legacy_seed_ids(
    seeds: &mut [RawIndependentRuleSeed],
    specification: &IntendedWindowSpecification,
) -> Result<bool, A3RuleInventoryExhaustivenessError> {
    let mut used = BTreeSet::new();
    for seed in seeds {
        let matches = specification
            .rule_seeds
            .iter()
            .filter(|legacy| legacy.kind == seed.kind)
            .filter_map(|legacy| {
                legacy_sources(specification, &legacy.source_clause_seeds)
                    .ok()
                    .filter(|sources| sources == &seed.sources)
                    .map(|_| legacy)
            })
            .collect::<Vec<_>>();
        if matches.len() != 1 || !used.insert(matches[0].seed_id.clone()) {
            return Ok(false);
        }
        seed.legacy_seed_id = Some(matches[0].seed_id.clone());
    }
    Ok(used.len() == specification.rule_seeds.len())
}

fn chronological_interface(ty: &KernelTy) -> Option<&'static str> {
    match ty {
        KernelTy::Type => Some("direct_type"),
        KernelTy::Fun(_, codomain) if matches!(codomain.as_ref(), KernelTy::Type) => {
            Some("pointwise_type_valued_function")
        }
        _ => None,
    }
}

fn output_shape_replayed(
    rule: A3RuleConstructor,
    sources: &[&crate::a3_demand_grammar::A3TypedClauseSource],
    output: &A3DemandOutputType,
) -> bool {
    match (rule, output, sources) {
        (
            A3RuleConstructor::UnaryAction,
            A3DemandOutputType::ActionAt {
                source_family,
                source_type,
            },
            [source],
        ) => {
            source_family == &source.canonical_family_key && source_type == &source.kernel_type
        }
        (
            A3RuleConstructor::ChronologicalComparison,
            A3DemandOutputType::ChronologicalInteraction {
                older_family,
                older_type,
                newest_family,
                newest_type,
                interface_mode,
                interface_slot_map,
            },
            [older, newest],
        ) => {
            older_family == &older.canonical_family_key
                && older_type == &older.kernel_type
                && newest_family == &newest.canonical_family_key
                && newest_type == &newest.kernel_type
                && crate::a3_demand_grammar::replay_chronological_interface_slot_map(
                    interface_slot_map,
                    u32::try_from(newest.canonical_presentation.parameters.len())
                        .expect("chronological parameter arity fits u32"),
                )
                .is_ok()
                && match (chronological_interface(&older.kernel_type), interface_mode) {
                    (
                        Some("direct_type"),
                        crate::a3_demand_grammar::A3ChronologicalInterfaceMode::DirectType,
                    ) => true,
                    (
                        Some("pointwise_type_valued_function"),
                        crate::a3_demand_grammar::A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { domain },
                    ) => matches!(&older.kernel_type, KernelTy::Fun(found, codomain) if found.as_ref() == domain && matches!(codomain.as_ref(), KernelTy::Type)),
                    _ => false,
                }
        }
        (
            A3RuleConstructor::HigherOpenBoxReduction,
            A3DemandOutputType::ContractibleOpenBox {
                dimension,
                boundary_families,
                boundary_types,
                path_witness_families,
            },
            sources,
        ) => {
            let path_sources = sources
                .iter()
                .filter(|source| {
                    source.exported_public_clause
                        && matches!(source.kernel_type, KernelTy::PathDecl { .. })
                })
                .collect::<Vec<_>>();
            let expected_dimension = path_sources
                .iter()
                .filter_map(|source| match source.kernel_type {
                    KernelTy::PathDecl { dimension } => Some(dimension),
                    _ => None,
                })
                .max();
            expected_dimension == Some(*dimension)
                && boundary_families
                    == &sources
                        .iter()
                        .map(|source| source.canonical_family_key.clone())
                        .collect::<Vec<_>>()
                && boundary_types
                    == &sources
                        .iter()
                        .map(|source| source.kernel_type.clone())
                        .collect::<Vec<_>>()
                && path_witness_families
                    == &path_sources
                        .iter()
                        .map(|source| source.canonical_family_key.clone())
                        .collect::<Vec<_>>()
        }
        _ => false,
    }
}

fn prove_base_seed(
    window: &A3HistoricalWindow,
    seed: &RawIndependentRuleSeed,
) -> Result<A3BaseSeedDispositionProof, A3RuleInventoryExhaustivenessError> {
    let mut typed_sources = Vec::with_capacity(seed.sources.len());
    for coordinate in &seed.sources {
        let matches = window
            .typed_sources
            .iter()
            .filter(|source| {
                source.step == coordinate.step
                    && source.candidate_hash == coordinate.candidate_hash
                    && source.clause_index == coordinate.clause_index
            })
            .collect::<Vec<_>>();
        if matches.len() != 1
            || serde_json::to_string(&matches[0].kernel_role).expect("clause role serializes")
                != coordinate.declared_role
        {
            return Err(A3RuleInventoryExhaustivenessError::Invariant(format!(
                "seed {} source coordinate failed exact typed join",
                seed.seed_id
            )));
        }
        typed_sources.push(matches[0]);
    }
    let rule = mapped_rule(seed.kind);
    let all_public = typed_sources
        .iter()
        .all(|source| source.exported_public_clause && !source.typing_derivation_hash.is_empty());
    let (eligible, rejection_reason) = match seed.kind {
        IntendedRuleSeedKind::UnaryAction => {
            if typed_sources.len() != 1 {
                return Err(A3RuleInventoryExhaustivenessError::Invariant(
                    "unary independent seed arity drifted".to_owned(),
                ));
            }
            (
                all_public,
                (!all_public).then(|| "source_not_typed_public".to_owned()),
            )
        }
        IntendedRuleSeedKind::BinaryComparison => {
            if typed_sources.len() != 2
                || typed_sources[0].layer != A3WindowLayer::Older
                || typed_sources[1].layer != A3WindowLayer::Newest
            {
                return Err(A3RuleInventoryExhaustivenessError::Invariant(
                    "binary independent seed chronology drifted".to_owned(),
                ));
            }
            let eligible =
                all_public && chronological_interface(&typed_sources[0].kernel_type).is_some();
            (
                eligible,
                (!eligible).then(|| "older_interface_not_type_valued_or_not_public".to_owned()),
            )
        }
        IntendedRuleSeedKind::HigherOpenBoxReduction => {
            let all_window_anchors = window
                .typed_sources
                .iter()
                .map(|source| source.anchor_id.as_str())
                .collect::<Vec<_>>();
            let supplied_anchors = typed_sources
                .iter()
                .map(|source| source.anchor_id.as_str())
                .collect::<Vec<_>>();
            if supplied_anchors != all_window_anchors {
                return Err(A3RuleInventoryExhaustivenessError::Invariant(
                    "higher independent seed does not consume the complete typed window".to_owned(),
                ));
            }
            let has_path = typed_sources.iter().any(|source| {
                source.exported_public_clause
                    && matches!(source.kernel_type, KernelTy::PathDecl { .. })
            });
            (
                all_public && has_path,
                (!(all_public && has_path)).then(|| {
                    if !all_public {
                        "source_not_typed_public".to_owned()
                    } else {
                        "no_typed_path_witness".to_owned()
                    }
                }),
            )
        }
    };
    let anchors = typed_sources
        .iter()
        .map(|source| source.anchor_id.clone())
        .collect::<Vec<_>>();
    let matching_instances = window
        .instances
        .iter()
        .filter(|instance| instance.source_anchor_ids == anchors)
        .filter(|instance| {
            window.schemes.iter().any(|scheme| {
                scheme.scheme_id == instance.scheme_id && scheme.rule_constructor == rule
            })
        })
        .collect::<Vec<_>>();
    if (eligible && matching_instances.len() != 1) || (!eligible && !matching_instances.is_empty())
    {
        return Err(A3RuleInventoryExhaustivenessError::Invariant(format!(
            "seed {} eligibility maps to {} operational instances",
            seed.seed_id,
            matching_instances.len()
        )));
    }
    let (instance_id, scheme_id, evidence_hash, orbit_id, output_replayed) =
        if let Some(instance) = matching_instances.first() {
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .expect("matching instance resolved its scheme");
            if !matches!(scheme.origin, A3DemandSchemeOrigin::BaseRule { .. }) {
                return Err(A3RuleInventoryExhaustivenessError::Invariant(
                    "base seed mapped to structural scheme".to_owned(),
                ));
            }
            let evidence = window
                .base_rule_evidence
                .iter()
                .filter(|evidence| {
                    evidence.evidence_hash == instance.origin_evidence_hash
                        && evidence.rule_constructor == rule
                        && evidence.source_anchor_ids == anchors
                })
                .collect::<Vec<_>>();
            if evidence.len() != 1 {
                return Err(A3RuleInventoryExhaustivenessError::Invariant(
                    "promoted base seed lacks unique occurrence evidence".to_owned(),
                ));
            }
            let orbit = window
                .orbits
                .iter()
                .filter(|orbit| orbit.member_instance_ids.contains(&instance.instance_id))
                .collect::<Vec<_>>();
            if orbit.len() != 1 || orbit[0].scheme_id != scheme.scheme_id {
                return Err(A3RuleInventoryExhaustivenessError::Invariant(
                    "promoted base seed lacks unique quotient orbit".to_owned(),
                ));
            }
            (
                Some(instance.instance_id.clone()),
                Some(scheme.scheme_id.clone()),
                Some(evidence[0].evidence_hash.clone()),
                Some(orbit[0].orbit_id.clone()),
                output_shape_replayed(rule, &typed_sources, &scheme.required_output),
            )
        } else {
            (None, None, None, None, true)
        };
    let exactly_one_promotion_or_rejection =
        (eligible && instance_id.is_some() && rejection_reason.is_none())
            || (!eligible && instance_id.is_none() && rejection_reason.is_some());
    let mut proof = A3BaseSeedDispositionProof {
        independent_seed_id: seed.seed_id.clone(),
        legacy_demand_completeness_seed_id: seed.legacy_seed_id.clone(),
        independent_kind: intended_kind_name(seed.kind).to_owned(),
        mapped_rule_constructor: rule_name(rule).to_owned(),
        raw_sources: seed.sources.clone(),
        typed_source_anchor_ids: anchors,
        every_source_joined_to_exact_typed_public_clause: all_public,
        eligible,
        rejection_reason,
        promoted_instance_id: instance_id,
        promoted_scheme_id: scheme_id,
        base_rule_evidence_hash: evidence_hash,
        quotient_orbit_id: orbit_id,
        required_output_shape_replayed: output_replayed,
        exactly_one_promotion_or_rejection,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("base-seed-disposition", &proof);
    Ok(proof)
}

fn expected_rejection_reason(
    rule: A3RuleConstructor,
    rejected: usize,
) -> Option<A3SeedRejectionReason> {
    match rule {
        A3RuleConstructor::ChronologicalComparison if rejected > 0 => {
            Some(A3SeedRejectionReason::OlderInterfaceNotTypeValuedOrNotPublic)
        }
        A3RuleConstructor::HigherOpenBoxReduction if rejected > 0 => {
            Some(A3SeedRejectionReason::NoTypedPathWitness)
        }
        _ => None,
    }
}

fn aggregate_kind(
    window: &A3HistoricalWindow,
    kind: IntendedRuleSeedKind,
    rows: &[A3BaseSeedDispositionProof],
) -> Result<A3RuleKindAggregateProof, A3RuleInventoryExhaustivenessError> {
    let name = intended_kind_name(kind);
    let rule = mapped_rule(kind);
    let selected = rows
        .iter()
        .filter(|row| row.independent_kind == name)
        .collect::<Vec<_>>();
    let raw_seed_count = selected.len();
    let promoted_seed_count = selected.iter().filter(|row| row.eligible).count();
    let rejected_seed_count = raw_seed_count - promoted_seed_count;
    let generated = selected
        .iter()
        .filter(|row| row.promoted_instance_id.is_some())
        .count();
    let operational = window
        .seed_dispositions
        .iter()
        .filter(|row| row.rule_constructor == rule)
        .collect::<Vec<_>>();
    if operational.len() != 1 {
        return Err(A3RuleInventoryExhaustivenessError::Invariant(format!(
            "Stage {} has {} aggregate rows for {rule:?}",
            window.stage,
            operational.len()
        )));
    }
    let operational = operational[0];
    let exact = operational.raw_seed_count == raw_seed_count
        && operational.promoted_seed_count == promoted_seed_count
        && operational.rejected_seed_count == rejected_seed_count
        && operational.generated_typed_instance_count == generated
        && operational.rejection_reason == expected_rejection_reason(rule, rejected_seed_count);
    let mut proof = A3RuleKindAggregateProof {
        independent_kind: name.to_owned(),
        mapped_rule_constructor: rule_name(rule).to_owned(),
        raw_seed_count,
        promoted_seed_count,
        rejected_seed_count,
        operational_generated_instance_count: generated,
        operational_disposition_exactly_replayed: exact,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("rule-kind-aggregate", &proof);
    Ok(proof)
}

fn prove_structural_seeds(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    debt: StructuralDebt,
    required_packages: &[String],
    issue_legacy_future_holes: bool,
) -> Result<Vec<A3StructuralSeedDispositionProof>, A3RuleInventoryExhaustivenessError> {
    let future_holes = if issue_legacy_future_holes {
        issue_structural_future_holes_for_window(signature, window)
            .map_err(|error| A3RuleInventoryExhaustivenessError::FutureHole(error.to_string()))?
    } else {
        Vec::new()
    };
    StructuralFamily::ALL
        .into_iter()
        .map(|family| {
            let constructor = structural_constructor(family);
            let predicate_holds = structural_predicate(debt, family);
            let package_projection = required_packages.iter().any(|name| name == family.slug());
            let evidence = window
                .constructor_evidence
                .iter()
                .filter(|row| row.constructor == constructor)
                .collect::<Vec<_>>();
            let schemes = window
                .schemes
                .iter()
                .filter(|scheme| {
                    matches!(
                        scheme.origin,
                        A3DemandSchemeOrigin::StructuralCompletion {
                            constructor: found,
                            ..
                        } if found == constructor
                    )
                })
                .collect::<Vec<_>>();
            let instances = schemes
                .iter()
                .flat_map(|scheme| {
                    window
                        .instances
                        .iter()
                        .filter(move |instance| instance.scheme_id == scheme.scheme_id)
                })
                .collect::<Vec<_>>();
            let orbits = schemes
                .iter()
                .flat_map(|scheme| {
                    window
                        .orbits
                        .iter()
                        .filter(move |orbit| orbit.scheme_id == scheme.scheme_id)
                })
                .collect::<Vec<_>>();
            let holes = future_holes
                .iter()
                .filter(|hole| hole.structural_constructor.as_deref() == Some(family.slug()))
                .collect::<Vec<_>>();
            let structural_seed_promoted_exactly_once = predicate_holds
                && package_projection
                && evidence.len() == 1
                && schemes.len() == 1
                && instances.len() == 1
                && orbits.len() == 1;
            let legacy_future_hole_join = !issue_legacy_future_holes
                || (holes.len() == 1
                    && holes[0].a3_instance_id == instances[0].instance_id
                    && holes[0].a3_scheme_id == schemes[0].scheme_id);
            let promoted_exactly_once =
                structural_seed_promoted_exactly_once && legacy_future_hole_join;
            let rejected_exactly_once = !predicate_holds
                && !package_projection
                && evidence.is_empty()
                && schemes.is_empty()
                && instances.is_empty()
                && orbits.is_empty()
                && holes.is_empty();
            let future_zero = issue_legacy_future_holes
                && holes.first().is_none_or(|hole| {
                    hole.open_judgment_kernel_typed
                        && hole.every_motive_bn_formable
                        && hole.d_membership_issued
                        && hole.named_gap.is_none()
                        && hole.charge.marginal_kappa == 0
                        && hole.charge.marginal_nu == 0
                        && hole.charge.anchors_minted == 0
                        && hole.charge.demand_orbits_minted == 0
                        && !hole.charge.credit_minted
                        && !hole.charge.filler_minted_credit
                });
            let mut proof = A3StructuralSeedDispositionProof {
                structural_family: family.slug().to_owned(),
                mapped_demand_constructor: constructor.slug().to_owned(),
                mapped_rule_constructor: rule_name(A3RuleConstructor::StructuralCompletionHole)
                    .to_owned(),
                raw_debt_predicate_holds: predicate_holds,
                required_packages_projection_contains_family: package_projection,
                rejected_by_raw_debt_predicate: !predicate_holds,
                constructor_evidence_hash: evidence.first().map(|row| row.evidence_hash.clone()),
                structural_scheme_id: schemes.first().map(|row| row.scheme_id.clone()),
                structural_instance_id: instances.first().map(|row| row.instance_id.clone()),
                structural_orbit_id: orbits.first().map(|row| row.orbit_id.clone()),
                future_hole_open_judgment_hash: holes
                    .first()
                    .map(|row| row.derivation_hash.clone()),
                future_hole_motive_typed_zero_charge: future_zero,
                exactly_one_promotion_or_rejection: (promoted_exactly_once
                    || rejected_exactly_once)
                    && (!issue_legacy_future_holes || future_zero),
                derivation_hash: String::new(),
            };
            proof.derivation_hash = tagged_hash("structural-seed-disposition", &proof);
            Ok(proof)
        })
        .collect()
}

fn build_window_proof(
    signature: &SealedSignature,
    window: A3HistoricalWindow,
    legacy_specification: Option<&IntendedWindowSpecification>,
    issue_legacy_future_holes: bool,
) -> Result<A3WindowRuleInventoryProof, A3RuleInventoryExhaustivenessError> {
    let library = exact_prefix_library(signature, window.stage)?;
    let debt = summarize_structural_debt(&library, WINDOW_DEPTH);
    let required_packages = required_packages_for(debt)
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let mut raw_seeds = raw_independent_seeds(signature, window.stage)?;
    let legacy_seed_join_complete = match legacy_specification {
        Some(specification) => join_legacy_seed_ids(&mut raw_seeds, specification)?,
        // The exact-prefix API has no legacy artifact dependency.  Its raw
        // coordinate generator is the authority, so this join is vacuously
        // complete rather than a missing premise.
        None => true,
    };
    let base_seed_dispositions = raw_seeds
        .iter()
        .map(|seed| prove_base_seed(&window, seed))
        .collect::<Result<Vec<_>, _>>()?;
    let kinds = [
        IntendedRuleSeedKind::UnaryAction,
        IntendedRuleSeedKind::BinaryComparison,
        IntendedRuleSeedKind::HigherOpenBoxReduction,
    ];
    let base_kind_aggregates = kinds
        .into_iter()
        .map(|kind| aggregate_kind(&window, kind, &base_seed_dispositions))
        .collect::<Result<Vec<_>, _>>()?;
    let structural_seed_dispositions = prove_structural_seeds(
        signature,
        &window,
        debt,
        &required_packages,
        issue_legacy_future_holes,
    )?;

    let promoted_base_occurrences = base_seed_dispositions
        .iter()
        .filter_map(|row| row.promoted_instance_id.clone())
        .collect::<Vec<_>>();
    let promoted_structural_occurrences = structural_seed_dispositions
        .iter()
        .filter_map(|row| row.structural_instance_id.clone())
        .collect::<Vec<_>>();
    let all_consumed_occurrences = promoted_base_occurrences
        .iter()
        .chain(&promoted_structural_occurrences)
        .cloned()
        .collect::<Vec<_>>();
    let all_consumed_ids = all_consumed_occurrences
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let operational_instance_ids = window
        .instances
        .iter()
        .map(|instance| instance.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let consumed_scheme_occurrences = base_seed_dispositions
        .iter()
        .filter_map(|row| row.promoted_scheme_id.clone())
        .chain(
            structural_seed_dispositions
                .iter()
                .filter_map(|row| row.structural_scheme_id.clone()),
        )
        .collect::<Vec<_>>();
    let consumed_scheme_ids = consumed_scheme_occurrences
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let consumed_orbit_occurrences = base_seed_dispositions
        .iter()
        .filter_map(|row| row.quotient_orbit_id.clone())
        .chain(
            structural_seed_dispositions
                .iter()
                .filter_map(|row| row.structural_orbit_id.clone()),
        )
        .collect::<Vec<_>>();
    let consumed_orbit_ids = consumed_orbit_occurrences
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let operational_scheme_ids = window
        .schemes
        .iter()
        .map(|scheme| scheme.scheme_id.clone())
        .collect::<BTreeSet<_>>();
    let operational_orbit_ids = window
        .orbits
        .iter()
        .map(|orbit| orbit.orbit_id.clone())
        .collect::<BTreeSet<_>>();
    let every_disposition = base_seed_dispositions
        .iter()
        .all(|row| row.exactly_one_promotion_or_rejection && row.required_output_shape_replayed)
        && structural_seed_dispositions
            .iter()
            .all(|row| row.exactly_one_promotion_or_rejection);
    let independent_preimage_ids_injective = all_consumed_occurrences.len()
        == all_consumed_ids.len()
        && promoted_base_occurrences
            .iter()
            .all(|id| !promoted_structural_occurrences.contains(id));
    let instance_reverse_join = independent_preimage_ids_injective
        && all_consumed_occurrences.len() == operational_instance_ids.len()
        && all_consumed_ids == operational_instance_ids;
    let scheme_orbit_reverse_join = consumed_scheme_ids == operational_scheme_ids
        && consumed_orbit_ids == operational_orbit_ids;
    let operational_seed_dispositions_replayed = base_kind_aggregates
        .iter()
        .all(|row| row.operational_disposition_exactly_replayed)
        && window
            .seed_dispositions
            .iter()
            .find(|row| row.rule_constructor == A3RuleConstructor::StructuralCompletionHole)
            .is_some_and(|row| {
                let promoted = structural_seed_dispositions
                    .iter()
                    .filter(|proof| proof.raw_debt_predicate_holds)
                    .count();
                row.raw_seed_count == promoted
                    && row.promoted_seed_count == promoted
                    && row.rejected_seed_count == 0
                    && row.generated_typed_instance_count == promoted
                    && row.rejection_reason.is_none()
            });
    let structural_future_hole_clause_coverage_complete =
        structural_seed_dispositions.iter().all(|row| {
            row.raw_debt_predicate_holds == row.future_hole_open_judgment_hash.is_some()
                && row.future_hole_motive_typed_zero_charge
        });
    let independent_base_seed_count = base_seed_dispositions.len();
    let promoted_base_seed_count = base_seed_dispositions
        .iter()
        .filter(|row| row.eligible)
        .count();
    let rejected_base_seed_count = independent_base_seed_count - promoted_base_seed_count;
    let promoted_structural_seed_count = structural_seed_dispositions
        .iter()
        .filter(|row| row.raw_debt_predicate_holds)
        .count();
    let rejected_structural_seed_count =
        structural_seed_dispositions.len() - promoted_structural_seed_count;
    let relative = legacy_seed_join_complete
        && every_disposition
        && instance_reverse_join
        && scheme_orbit_reverse_join
        && (!issue_legacy_future_holes || structural_future_hole_clause_coverage_complete)
        && operational_seed_dispositions_replayed;
    let mut proof = A3WindowRuleInventoryProof {
        stage: window.stage,
        exact_prefix_signature_digest: signature.digest().to_owned(),
        newest_step: window.newest_step,
        older_step: window.older_step,
        complete_two_step_window: window.newest_step.is_some() && window.older_step.is_some(),
        raw_window_rebuilt_without_focus_count_score_bar_or_winner: true,
        raw_structural_debt_json: structural_debt_json(debt),
        required_packages_from_raw_debt: required_packages,
        legacy_independent_specification_digest: legacy_specification
            .map(|specification| specification.specification_digest.clone()),
        legacy_seed_join_complete,
        base_seed_dispositions,
        base_kind_aggregates,
        structural_seed_dispositions,
        independent_base_seed_count,
        promoted_base_seed_count,
        rejected_base_seed_count,
        structural_family_candidate_count: StructuralFamily::ALL.len(),
        promoted_structural_seed_count,
        rejected_structural_seed_count,
        every_independent_seed_has_exactly_one_disposition: every_disposition,
        independent_preimage_occurrence_count: all_consumed_occurrences.len(),
        operational_instance_count: operational_instance_ids.len(),
        independent_preimage_ids_injective,
        every_operational_instance_has_exactly_one_independent_preimage: instance_reverse_join,
        every_operational_scheme_and_orbit_is_consumed: scheme_orbit_reverse_join,
        structural_future_hole_clause_coverage_complete,
        legacy_future_hole_typing_consumed: issue_legacy_future_holes,
        future_hole_typing_deferred_to_branch_v2: !issue_legacy_future_holes,
        operational_seed_dispositions_replayed,
        relative_rule_inventory_exhaustive_for_window: relative,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("window-rule-inventory-proof", &proof);
    Ok(proof)
}

/// Branch-safe theorem API.  The signature must be the exact prefix
/// `1..stage`; A3 itself rejects a longer, shorter, or non-contiguous prefix.
/// No historical winner, focus label, score, count, bar, or next candidate is
/// accepted by this interface.
pub fn prove_a3_window_inventory_for_exact_prefix(
    signature: &SealedSignature,
    stage: u32,
) -> Result<A3WindowRuleInventoryProof, A3RuleInventoryExhaustivenessError> {
    let window = generate_a3_window_for_prefix(signature, stage)
        .map_err(|error| A3RuleInventoryExhaustivenessError::A3(error.to_string()))?;
    build_window_proof(signature, window, None, false)
}

/// Branch-safe exact-prefix inventory theorem with no historical Stage-16
/// ceiling.  It reuses the same raw coordinate generator and proof builder as
/// the frozen historical theorem, so extending the search horizon cannot add
/// a new rule kind or weaken any replay check.
pub fn prove_a3_window_inventory_for_exact_prefix_unbounded(
    signature: &SealedSignature,
    stage: u32,
) -> Result<A3WindowRuleInventoryProof, A3RuleInventoryExhaustivenessError> {
    let window =
        crate::a3_demand_grammar::generate_a3_window_for_exact_prefix_unbounded(signature, stage)
            .map_err(|error| A3RuleInventoryExhaustivenessError::A3(error.to_string()))?;
    build_window_proof(signature, window, None, false)
}

fn historical_prefix(stage: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        (1..stage)
            .map(|step| (step, Telescope::reference(step)))
            .collect(),
    )
}

fn shape_mappings(
    demand: &DemandCompletenessCertificate,
) -> Result<Vec<A3RuleShapeMapping>, A3RuleInventoryExhaustivenessError> {
    let observed_kinds = demand
        .intended_window_specifications
        .iter()
        .flat_map(|window| window.rule_seeds.iter().map(|seed| seed.kind))
        .map(intended_kind_name)
        .collect::<BTreeSet<_>>();
    if observed_kinds
        != [
            "binary_comparison",
            "higher_open_box_reduction",
            "unary_action",
        ]
        .into_iter()
        .collect()
    {
        return Err(A3RuleInventoryExhaustivenessError::Invariant(
            "independent live seed occurrences do not expose exactly the three adopted base shapes"
                .to_owned(),
        ));
    }
    let make = |shape, source: &str, rule, source_parameterized, structural_parameterized| {
        let mut row = A3RuleShapeMapping {
            intended_shape: shape,
            independent_definition_source: source.to_owned(),
            operational_rule_constructor: rule_name(rule).to_owned(),
            shape_is_parameterized_by_typed_source_families: source_parameterized,
            shape_is_parameterized_by_structural_family: structural_parameterized,
            unique_operational_image: true,
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("rule-shape-mapping", &row);
        row
    };
    Ok(vec![
        make(
            intended_shape(IntendedRuleSeedKind::UnaryAction),
            "demand_completeness::IntendedRuleSeedKind::UnaryAction",
            mapped_rule(IntendedRuleSeedKind::UnaryAction),
            true,
            false,
        ),
        make(
            intended_shape(IntendedRuleSeedKind::BinaryComparison),
            "demand_completeness::IntendedRuleSeedKind::BinaryComparison",
            mapped_rule(IntendedRuleSeedKind::BinaryComparison),
            true,
            false,
        ),
        make(
            intended_shape(IntendedRuleSeedKind::HigherOpenBoxReduction),
            "demand_completeness::IntendedRuleSeedKind::HigherOpenBoxReduction",
            mapped_rule(IntendedRuleSeedKind::HigherOpenBoxReduction),
            true,
            false,
        ),
        make(
            IndependentA3RuleShape::StructuralFutureCompletionHole,
            "future-hole-hypothesis-definition-v1 over raw StructuralFamily predicates",
            A3RuleConstructor::StructuralCompletionHole,
            false,
            true,
        ),
    ])
}

fn schema_completion_artifact_digest(certificate: &GrammarCompletionCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(
        certificate.schema.as_str(),
        "grammar-completion-certificate",
        &projection,
    ))
    .expect("grammar completion certificate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn completed_schema_source_coverage(
    grammar: &crate::a3_demand_grammar::A3HistoricalDemandGrammar,
) -> Result<
    (
        String,
        bool,
        bool,
        Vec<String>,
        Vec<CompletedSchemaSourceCoverage>,
    ),
    A3RuleInventoryExhaustivenessError,
> {
    let completion: GrammarCompletionCertificate =
        serde_json::from_slice(SCHEMA_COMPLETION_ARTIFACT_BYTES)
            .map_err(|error| A3RuleInventoryExhaustivenessError::Schema(error.to_string()))?;
    let artifact_digest_verified =
        completion.result_digest == schema_completion_artifact_digest(&completion);
    if !artifact_digest_verified
        || !completion.full_adopted_grammar_normalization_naturality_complete
        || !completion
            .g7_induction
            .intended_inventory_exhaustive_under_adopted_batch
    {
        return Err(A3RuleInventoryExhaustivenessError::Schema(
            "frozen completed Schema2 artifact failed its internal digest or completion flags"
                .to_owned(),
        ));
    }
    // This deliberately reports (rather than launders) source drift.  A3's
    // relative rule enumeration is reconstructed from raw typed clauses and
    // does not consume this archival replay as a premise.
    let live_replay = replay_grammar_completion_certificate(&completion);
    let rows = completion
        .registrations
        .iter()
        .map(|registration| {
            let matches = grammar
                .windows
                .iter()
                .flat_map(|window| &window.typed_sources)
                .filter(|source| {
                    source.step == registration.step
                        && source.clause_index == registration.clause_index
                        && source.kernel_role == registration.kernel_role
                        && serde_json::to_string(&source.kernel_type)
                            .expect("kernel type serializes")
                            == registration.kernel_type_json
                        && source.normal_form == registration.normal_form
                })
                .collect::<Vec<_>>();
            let anchors = matches
                .iter()
                .map(|source| source.anchor_id.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let exact = !anchors.is_empty() && registration.normalization_natural;
            let mut row = CompletedSchemaSourceCoverage {
                step: registration.step,
                clause_index: registration.clause_index,
                registered_name: registration.registered_name.clone(),
                matching_a3_source_anchor_ids: anchors,
                kernel_role_type_and_normal_form_match: exact,
                normalization_naturality_token_live: registration.normalization_natural,
                derivation_hash: String::new(),
            };
            row.derivation_hash = tagged_hash("completed-schema-source-coverage", &row);
            row
        })
        .collect::<Vec<_>>();
    Ok((
        completion.result_digest,
        artifact_digest_verified,
        live_replay.valid,
        live_replay.errors,
        rows,
    ))
}

fn future_hole_adoption_replayed() -> bool {
    std::str::from_utf8(FUTURE_HOLE_ADOPTION_BYTES).is_ok_and(|text| {
        text.contains("future-hole-hypothesis-definition-v1")
            && text.contains("**Adopted.**")
            && text.contains("future-directed demand scheme is an")
            && text.contains("open")
            && text.contains("judgment")
            && text.contains("holes and fillings mint nothing")
    })
}

fn frozen_global_e4_scope() -> Result<(String, bool, bool), A3RuleInventoryExhaustivenessError> {
    let artifact: serde_json::Value = serde_json::from_slice(GLOBAL_E4_V10_ARTIFACT_BYTES)
        .map_err(|error| A3RuleInventoryExhaustivenessError::Schema(error.to_string()))?;
    let string = |field: &str| {
        artifact
            .get(field)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                A3RuleInventoryExhaustivenessError::Schema(format!(
                    "global E-4 v10 archive omits string field {field}"
                ))
            })
    };
    let boolean = |field: &str| {
        artifact
            .get(field)
            .and_then(serde_json::Value::as_bool)
            .ok_or_else(|| {
                A3RuleInventoryExhaustivenessError::Schema(format!(
                    "global E-4 v10 archive omits boolean field {field}"
                ))
            })
    };
    if string("schema")? != "schema2-global-e4-assembly-v10"
        || string("outcome")?
            != "global_e4_v10_complete_wrapped_domain_exhausted_no_unknown_survives"
        || !boolean("global_e4_complete")?
    {
        return Err(A3RuleInventoryExhaustivenessError::Schema(
            "global E-4 v10 archive does not bind the completed frozen wrapped surface".to_owned(),
        ));
    }
    let wrapped = artifact.get("wrapped_domain_exhaustion").ok_or_else(|| {
        A3RuleInventoryExhaustivenessError::Schema(
            "global E-4 v10 archive omits wrapped-domain exhaustion".to_owned(),
        )
    })?;
    let wrapped_complete = wrapped
        .get("domain_finite")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
        && wrapped
            .get("contextual_error_inventory_complete")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        && wrapped
            .get("wrapped_classifier_partition_total")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        && wrapped
            .get("unknown_survivor_count")
            .and_then(serde_json::Value::as_u64)
            == Some(0);
    Ok((
        string("result_digest")?,
        boolean("class_exhaustion_proved")? && wrapped_complete,
        boolean("no_unknown_survives")?,
    ))
}

fn certificate_digest(certificate: &A3RuleInventoryExhaustivenessCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("historical-exhaustiveness-certificate", &projection)
}

pub fn issue_historical_a3_rule_inventory_exhaustiveness()
-> Result<A3RuleInventoryExhaustivenessCertificate, A3RuleInventoryExhaustivenessError> {
    let signature = SealedSignature::genesis_del_h15();
    let operational = issue_historical_a3_demand_grammar(&signature)
        .map_err(|error| A3RuleInventoryExhaustivenessError::A3(error.to_string()))?;
    let independent = build_demand_completeness_certificate()
        .map_err(|error| A3RuleInventoryExhaustivenessError::Independent(error.to_string()))?;
    let specification_by_stage = independent
        .intended_window_specifications
        .iter()
        .map(|specification| (specification.stage, specification))
        .collect::<BTreeMap<_, _>>();
    let mut historical_windows = Vec::new();
    for window in &operational.windows {
        let prefix = historical_prefix(window.stage);
        let branch_window = generate_a3_window_for_prefix(&prefix, window.stage)
            .map_err(|error| A3RuleInventoryExhaustivenessError::A3(error.to_string()))?;
        let specification = specification_by_stage.get(&window.stage).copied();
        if (window.stage >= 3) != specification.is_some() {
            return Err(A3RuleInventoryExhaustivenessError::Invariant(format!(
                "Stage {} independent specification coverage drifted",
                window.stage
            )));
        }
        historical_windows.push(build_window_proof(
            &prefix,
            branch_window,
            specification,
            true,
        )?);
    }
    let mappings = shape_mappings(&independent)?;
    let target_names = A3RuleConstructor::ALL
        .into_iter()
        .map(rule_name)
        .collect::<BTreeSet<_>>();
    let mapped_names = mappings
        .iter()
        .map(|mapping| mapping.operational_rule_constructor.as_str())
        .collect::<BTreeSet<_>>();
    let every_operational_rule_constructor_has_an_independent_preimage =
        target_names == mapped_names;
    let (
        completed_schema2_archived_digest,
        completed_schema2_artifact_digest_verified,
        completed_schema2_live_definition_replay_succeeded,
        completed_schema2_live_replay_errors,
        completed_schema_source_coverage,
    ) = completed_schema_source_coverage(&operational)?;
    let every_completed_schema_source_is_an_a3_typed_parameter = !completed_schema_source_coverage
        .is_empty()
        && completed_schema_source_coverage.iter().all(|row| {
            row.kernel_role_type_and_normal_form_match && row.normalization_naturality_token_live
        });
    let (
        global_e4_v10_archived_digest,
        global_e4_wrapped_domain_class_exhaustion_bound,
        global_e4_no_unknown_survives_on_frozen_wrapped_surface,
    ) = frozen_global_e4_scope()?;
    let every_historical_window_covered = historical_windows.len() == 16
        && historical_windows
            .iter()
            .map(|window| window.stage)
            .eq(1..=16);
    let every_raw_seed_promoted_or_rejected = historical_windows.iter().all(|window| {
        window.every_independent_seed_has_exactly_one_disposition
            && window.operational_seed_dispositions_replayed
    });
    let every_operational_occurrence_has_an_independent_preimage =
        historical_windows.iter().all(|window| {
            window.every_operational_instance_has_exactly_one_independent_preimage
                && window.every_operational_scheme_and_orbit_is_consumed
        });
    let structural_future_hole_clauses_included = historical_windows
        .iter()
        .all(|window| window.structural_future_hole_clause_coverage_complete);
    let relative = every_historical_window_covered
        && every_raw_seed_promoted_or_rejected
        && every_operational_occurrence_has_an_independent_preimage
        && structural_future_hole_clauses_included
        && every_operational_rule_constructor_has_an_independent_preimage
        && global_e4_wrapped_domain_class_exhaustion_bound
        && global_e4_no_unknown_survives_on_frozen_wrapped_surface
        && mappings
            .iter()
            .all(|mapping| mapping.unique_operational_image)
        && historical_windows
            .iter()
            .all(|window| window.relative_rule_inventory_exhaustive_for_window);
    if !relative || !future_hole_adoption_replayed() {
        return Err(A3RuleInventoryExhaustivenessError::Invariant(
            "relative historical exhaustiveness conjunction failed".to_owned(),
        ));
    }
    let mut certificate = A3RuleInventoryExhaustivenessCertificate {
        schema: A3_RULE_INVENTORY_EXHAUSTIVENESS_SCHEMA.to_owned(),
        date: A3_RULE_INVENTORY_EXHAUSTIVENESS_DATE.to_owned(),
        source_bindings: source_bindings(),
        independent_demand_completeness_digest: independent.result_digest,
        independent_seed_generator_replayed: true,
        future_hole_adoption_replayed: true,
        completed_schema2_archived_digest,
        completed_schema2_artifact_digest_verified,
        completed_schema2_live_definition_replay_succeeded,
        completed_schema2_live_replay_errors,
        completed_schema_source_coverage,
        every_completed_schema_source_is_an_a3_typed_parameter,
        completed_schema_constructors_are_source_parameters_not_extra_a3_rule_shapes: true,
        completed_schema2_live_replay_is_not_required_for_relative_a3_rule_enumeration: true,
        global_e4_v10_archived_digest,
        global_e4_wrapped_domain_class_exhaustion_bound,
        global_e4_no_unknown_survives_on_frozen_wrapped_surface,
        global_e4_archive_scopes_the_relative_domain_but_does_not_generate_rule_shapes: true,
        shape_mappings: mappings,
        independent_shape_domain_derived_from_live_seed_occurrences_and_adopted_future_clause: true,
        every_independent_shape_has_one_operational_image: true,
        every_operational_rule_constructor_has_an_independent_preimage,
        historical_windows,
        every_historical_window_covered,
        every_raw_seed_promoted_or_rejected,
        every_operational_occurrence_has_an_independent_preimage,
        structural_future_hole_clauses_included,
        relative_rule_constructor_inventory_exhaustiveness_proved: true,
        broader_absolute_semantic_exhaustiveness_claimed: false,
        named_scope_limits: vec![
            A3_BROADER_SEMANTIC_SCOPE_GAP.to_owned(),
            A3_COMPLETED_SCHEMA2_LIVE_REPLAY_DRIFT.to_owned(),
        ],
        forbidden_count_score_bar_winner_or_halt_input: true,
        outcome:
            "relative_a3_rule_inventory_exhaustive_over_adopted_depth_two_grammar_absolute_scope_unclaimed"
                .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> A3RuleInventoryExhaustivenessReplay {
    A3RuleInventoryExhaustivenessReplay {
        valid: false,
        historical_window_count: 0,
        base_seed_count: 0,
        structural_candidate_count: 0,
        relative_exhaustiveness_proved: false,
        absolute_exhaustiveness_claimed: false,
        named_scope_limits: vec![
            A3_BROADER_SEMANTIC_SCOPE_GAP.to_owned(),
            A3_COMPLETED_SCHEMA2_LIVE_REPLAY_DRIFT.to_owned(),
        ],
        outcome: "a3_rule_inventory_exhaustiveness_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_historical_a3_rule_inventory_exhaustiveness(
    certificate: &A3RuleInventoryExhaustivenessCertificate,
) -> A3RuleInventoryExhaustivenessReplay {
    let expected = match issue_historical_a3_rule_inventory_exhaustiveness() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != &expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    A3RuleInventoryExhaustivenessReplay {
        valid: errors.is_empty(),
        historical_window_count: certificate.historical_windows.len(),
        base_seed_count: certificate
            .historical_windows
            .iter()
            .map(|window| window.independent_base_seed_count)
            .sum(),
        structural_candidate_count: certificate
            .historical_windows
            .iter()
            .map(|window| window.structural_family_candidate_count)
            .sum(),
        relative_exhaustiveness_proved: certificate
            .relative_rule_constructor_inventory_exhaustiveness_proved,
        absolute_exhaustiveness_claimed: certificate
            .broader_absolute_semantic_exhaustiveness_claimed,
        named_scope_limits: certificate.named_scope_limits.clone(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn historical_a3_rule_inventory_exhaustiveness_json_pretty()
-> Result<String, A3RuleInventoryExhaustivenessError> {
    serde_json::to_string_pretty(&issue_historical_a3_rule_inventory_exhaustiveness()?)
        .map_err(|error| A3RuleInventoryExhaustivenessError::Json(error.to_string()))
}

pub fn replay_historical_a3_rule_inventory_exhaustiveness_json(
    json: &str,
) -> A3RuleInventoryExhaustivenessReplay {
    match serde_json::from_str::<A3RuleInventoryExhaustivenessCertificate>(json) {
        Ok(certificate) => replay_historical_a3_rule_inventory_exhaustiveness(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_historical_a3_rule_inventory_exhaustiveness_create_new(
    path: &Path,
) -> Result<A3RuleInventoryExhaustivenessReplay, A3RuleInventoryExhaustivenessError> {
    let certificate = issue_historical_a3_rule_inventory_exhaustiveness()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| A3RuleInventoryExhaustivenessError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| A3RuleInventoryExhaustivenessError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| A3RuleInventoryExhaustivenessError::Io(error.to_string()))?;
    let replay = replay_historical_a3_rule_inventory_exhaustiveness(&certificate);
    if !replay.valid {
        return Err(A3RuleInventoryExhaustivenessError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_historical_seed_has_one_disposition_and_reverse_join() {
        let certificate =
            issue_historical_a3_rule_inventory_exhaustiveness().expect("exhaustiveness theorem");
        assert!(certificate.relative_rule_constructor_inventory_exhaustiveness_proved);
        assert!(certificate.every_raw_seed_promoted_or_rejected);
        assert!(certificate.every_operational_occurrence_has_an_independent_preimage);
        assert!(certificate.structural_future_hole_clauses_included);
        assert_eq!(certificate.historical_windows.len(), 16);
        assert!(certificate.historical_windows.iter().all(|window| {
            window.relative_rule_inventory_exhaustive_for_window
                && window.every_independent_seed_has_exactly_one_disposition
                && window.every_operational_instance_has_exactly_one_independent_preimage
        }));
    }

    #[test]
    fn arbitrary_exact_prefix_api_is_branch_safe_and_rejects_longer_signature() {
        let prefix = historical_prefix(5);
        let proof = prove_a3_window_inventory_for_exact_prefix(&prefix, 5)
            .expect("exact Stage-5 prefix theorem");
        assert!(
            proof.relative_rule_inventory_exhaustive_for_window,
            "{proof:#?}"
        );
        assert!(proof.legacy_independent_specification_digest.is_none());
        assert!(proof.legacy_seed_join_complete);
        assert!(!proof.legacy_future_hole_typing_consumed);
        assert!(proof.future_hole_typing_deferred_to_branch_v2);
        assert!(proof.independent_preimage_ids_injective);
        assert!(proof.every_operational_instance_has_exactly_one_independent_preimage);
        let full = SealedSignature::genesis_del_h15();
        assert!(prove_a3_window_inventory_for_exact_prefix(&full, 5).is_err());
    }

    #[test]
    fn unbounded_exact_prefix_proof_reaches_beyond_the_historical_stage16_surface() {
        let prefix = historical_prefix(17);
        let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(&prefix, 17)
            .expect("exact Stage-17 relative inventory theorem");
        assert_eq!(proof.stage, 17);
        assert_eq!(proof.exact_prefix_signature_digest, prefix.digest());
        assert!(proof.relative_rule_inventory_exhaustive_for_window);
        assert!(!proof.legacy_future_hole_typing_consumed);
        assert!(proof.future_hole_typing_deferred_to_branch_v2);
        assert!(proof.independent_preimage_ids_injective);
        assert!(proof.every_operational_instance_has_exactly_one_independent_preimage);
    }

    #[test]
    fn replay_rejects_redigested_promotion_and_absolute_scope_forgery() {
        let certificate =
            issue_historical_a3_rule_inventory_exhaustiveness().expect("exhaustiveness theorem");
        assert!(replay_historical_a3_rule_inventory_exhaustiveness(&certificate).valid);

        let mut promotion = certificate.clone();
        promotion.historical_windows[2].base_seed_dispositions[0].eligible = false;
        promotion.historical_windows[2].base_seed_dispositions[0].derivation_hash = tagged_hash(
            "base-seed-disposition",
            &promotion.historical_windows[2].base_seed_dispositions[0],
        );
        promotion.historical_windows[2].derivation_hash = tagged_hash(
            "window-rule-inventory-proof",
            &promotion.historical_windows[2],
        );
        promotion.result_digest = certificate_digest(&promotion);
        assert!(!replay_historical_a3_rule_inventory_exhaustiveness(&promotion).valid);

        let mut absolute = certificate;
        absolute.broader_absolute_semantic_exhaustiveness_claimed = true;
        absolute.named_scope_limits.clear();
        absolute.result_digest = certificate_digest(&absolute);
        assert!(!replay_historical_a3_rule_inventory_exhaustiveness(&absolute).valid);
    }
}
