//! Candidate-and-prefix-only ordinary provenance for T-BI-NU1 v3.
//!
//! This issuer separates three things which earlier versions conflated:
//!
//! 1. a count-blind enumeration of the candidate's semantic role families;
//! 2. the complete pre-candidate A3 scheme/orbit surface; and
//! 3. an injective provenance assignment.
//!
//! A3 orbit identities and required-output shapes are generated before the
//! candidate is inspected.  They are audited in full, but are not promoted
//! to ordinary credit unless the pre-candidate surface already contains an
//! independently exported, constructed, kernel-typed output.  The current A3
//! generator deliberately stops before output-term construction, so an
//! occupied local role produces a signed theorem gap rather than a
//! candidate-derived output-position id.

use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::a3_demand_grammar::{
    A3DemandOutputType, A3HistoricalWindow, generate_a3_window_for_exact_prefix_unbounded,
};
use pen_eval::a3_rule_inventory_exhaustiveness::prove_a3_window_inventory_for_exact_prefix_unbounded;
use pen_eval::semantic_provenance::{CreditMechanism, LocalRole};
use pen_eval::typed_families::{
    CandidateExtractionOutcome, ExtractedFamily, InstanceKind, extract_candidate_families,
    predecessor_closure,
};
use pen_type::elaborate::{SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::equality::{KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const ACT_LOCAL_PROVENANCE_V3_SCHEMA: &str = "act-local-provenance-v3";
pub const ACT_LOCAL_PROVENANCE_V3_DATE: &str = "2026-07-22";
pub const T_BI_NU1_V3_THEOREM_ID: &str = "T-BI-NU1-v3-full-A3-pre-candidate-orbit-provenance";
pub const ACT_LOCAL_ORDINARY_TOKEN_V3: &str = "act-local-ordinary-family-token-v3";
pub const R1_PACKAGE_THEOREM_ID: &str = "formation-completion-package-R1";
pub const R2_GENERATED_ACTION_THEOREM_ID: &str = "derived-action-generator-membership-rule-v1";
pub const ACT_LOCAL_PREFIX_DECLARATION_V3_SCHEMA: &str =
    "act-local-prefix-declaration-v3-source-first-v1";
pub const ACT_LOCAL_PREFIX_DECLARATION_V3_THEOREM_ID: &str =
    "T-BI-NU1-v3-prefix-local-declaration-projection-v1";
pub const ACT_LOCAL_V3_TYPED_R2_RULE_SCHEMA: &str = "act-local-v3-typed-r2-rule-token-v1";

const R2_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_quotient_adjudications.md");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_PROVENANCE_V3_SCHEMA, domain, value))
        .expect("v3 act-local evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn prefix_local_tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_PREFIX_DECLARATION_V3_SCHEMA, domain, value))
        .expect("v3 prefix-local declaration evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3Gap {
    pub id: String,
    pub stage: u32,
    pub family_id: Option<String>,
    pub kind: String,
    pub detail: String,
    pub derivation_hash: String,
}

fn gap(
    stage: u32,
    family_id: Option<&str>,
    kind: &str,
    detail: impl Into<String>,
) -> ActLocalV3Gap {
    let mut value = ActLocalV3Gap {
        id: format!(
            "T-BI-NU1-V3-S{stage}-{kind}-{}",
            family_id.unwrap_or("stage")
        ),
        stage,
        family_id: family_id.map(str::to_owned),
        kind: kind.to_owned(),
        detail: detail.into(),
        derivation_hash: String::new(),
    };
    value.derivation_hash = tagged_hash("gap", &value);
    value
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3RoleOccurrence {
    pub kind: String,
    pub owner_clause: u16,
    pub mechanism: CreditMechanism,
    pub local_role: LocalRole,
    pub coordinate: Value,
    pub natural_family_shape: Value,
    pub uniform_specialization: bool,
    pub removed_by_adopted_r2: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug)]
struct RoleOccurrence {
    kind: &'static str,
    owner_clause: u16,
    mechanism: CreditMechanism,
    local_role: LocalRole,
    coordinate: Value,
    natural_family_shape: Value,
    uniform_specialization: bool,
    removed_by_r2: bool,
}

impl RoleOccurrence {
    fn serializable(&self) -> ActLocalV3RoleOccurrence {
        let mut value = ActLocalV3RoleOccurrence {
            kind: self.kind.to_owned(),
            owner_clause: self.owner_clause,
            mechanism: self.mechanism,
            local_role: self.local_role,
            coordinate: self.coordinate.clone(),
            natural_family_shape: self.natural_family_shape.clone(),
            uniform_specialization: self.uniform_specialization,
            removed_by_adopted_r2: self.removed_by_r2,
            derivation_hash: String::new(),
        };
        value.derivation_hash = tagged_hash("role-occurrence", &value);
        value
    }
}

fn role(
    kind: &'static str,
    owner_clause: u16,
    mechanism: CreditMechanism,
    local_role: LocalRole,
    coordinates: Value,
) -> RoleOccurrence {
    RoleOccurrence {
        kind,
        owner_clause,
        mechanism,
        local_role,
        coordinate: json!({"kind": kind, "clause": owner_clause, "coordinates": coordinates}),
        natural_family_shape: json!({"kind": kind, "clause": owner_clause, "coordinates": coordinates}),
        uniform_specialization: false,
        removed_by_r2: false,
    }
}

fn uniform_role(
    kind: &'static str,
    owner_clause: u16,
    mechanism: CreditMechanism,
    local_role: LocalRole,
    target: Value,
) -> RoleOccurrence {
    RoleOccurrence {
        kind,
        owner_clause,
        mechanism,
        local_role,
        coordinate: json!({"kind": kind, "clause": owner_clause, "target": target}),
        natural_family_shape: json!({
            "kind": kind,
            "clause": owner_clause,
            "uniform_target_erased": true
        }),
        uniform_specialization: true,
        removed_by_r2: false,
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3GeneratorEvidence {
    pub owner_clause: u16,
    pub extractor_family_id: String,
    pub generator_univalent_key: String,
    pub canonical_normal_form: Expr,
    pub parameter_sorts: Vec<String>,
    pub kernel_type_json: String,
    pub extraction_hash: String,
    pub owner_clause_normalization_and_typing_replayed: bool,
    pub naturality_square_hash: String,
    pub naturality_square_equal: bool,
    pub owner_is_generator_or_instance: bool,
    pub role_schema_term_constructed: bool,
    pub role_schema_typing_replayed: bool,
    pub role_schema_naturality_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3MarginalityEvidence {
    pub equality_procedure: String,
    pub predecessor_family_count: usize,
    pub same_shape_comparison_count: usize,
    pub univalent_preimage_count: usize,
    pub no_predecessor_preimage: bool,
    pub full_sweep_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3A3OrbitAudit {
    pub orbit_id: String,
    pub scheme_id: String,
    pub rule_constructor: String,
    pub required_output_kind: String,
    pub required_output_hash: String,
    pub representative_instance_id: String,
    pub member_instance_count: usize,
    pub uniform_specializations_collapsed: bool,
    pub independently_exported_demand_orbit: bool,
    pub existed_before_candidate: bool,
    pub output_term_constructed_on_generator_boundary: bool,
    pub output_term_kernel_typed_on_generator_boundary: bool,
    pub usable_as_ordinary_credit_without_new_theorem: bool,
    pub orbit_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3NaturalFamilyRow {
    pub semantic_family_id: String,
    pub family_shape_key: String,
    pub representative_role: ActLocalV3RoleOccurrence,
    pub occurrence_count: usize,
    pub collapsed_uniform_instance_count: usize,
    pub generator: ActLocalV3GeneratorEvidence,
    pub marginality: ActLocalV3MarginalityEvidence,
    pub marginal: bool,
    pub removed_by_r2: bool,
    pub credited: bool,
    pub gap_id: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "anchor")]
pub enum ActLocalV3Anchor {
    ChargedLocalRole {
        clause: u16,
        role: LocalRole,
        injection_hash: String,
    },
    PreExistingA3RequiredOutput {
        orbit_id: String,
        scheme_id: String,
        required_output_hash: String,
        orbit_derivation_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3OrdinaryFamilyToken {
    pub token_version: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub semantic_family_id: String,
    pub family_shape_key: String,
    pub mechanism: CreditMechanism,
    pub representative_role: ActLocalV3RoleOccurrence,
    pub generator: ActLocalV3GeneratorEvidence,
    pub marginality: ActLocalV3MarginalityEvidence,
    pub anchor: ActLocalV3Anchor,
    pub typed_normalized_natural: bool,
    pub family_or_instance_quotient_decided: bool,
    pub uniform_specializations_multiplied_without_exported_orbit: bool,
    pub candidate_and_sealed_prefix_only: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalProvenanceV3Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub predecessor_act_local_digest: String,
    pub class: TelescopeClass,
    pub kappa: u32,
    pub candidate_elaboration_hash: String,
    pub candidate_family_extraction_hash: String,
    pub a3_window_hash: String,
    pub a3_inventory_exhaustiveness_hash: String,
    pub a3_window_stage: u32,
    pub a3_scheme_count: usize,
    pub a3_instance_count: usize,
    pub a3_orbit_count: usize,
    pub independently_exported_a3_orbit_count: usize,
    pub a3_orbits: Vec<ActLocalV3A3OrbitAudit>,
    pub relative_a3_rule_inventory_exhaustive_for_window: bool,
    pub full_pre_candidate_a3_surface_replayed: bool,
    pub a3_output_terms_constructed_on_generator_boundary: bool,
    pub a3_output_terms_kernel_typed_on_generator_boundary: bool,
    pub role_occurrences_before_quotient: Vec<ActLocalV3RoleOccurrence>,
    pub role_occurrence_count_before_quotient: usize,
    pub r1_formation_completion_package_replayed: bool,
    pub r2_generated_instance_removal_count: usize,
    pub natural_family_rows: Vec<ActLocalV3NaturalFamilyRow>,
    pub natural_family_count_after_quotient: usize,
    pub role_schema_extraction_complete: bool,
    pub collapsed_uniform_or_repeated_instance_count: usize,
    pub marginal_natural_family_count: usize,
    pub locally_anchored_family_count: usize,
    pub a3_output_anchored_family_count: usize,
    pub blind_local_role_capacity_4kappa: u32,
    pub adopted_export_upper_bound_4kappa_plus_exported_orbits: u32,
    pub counterfactual_all_quotient_orbits_upper_bound: u32,
    pub invalid_raw_occurrence_upper_bound: u32,
    pub ordinary_family_tokens: Vec<ActLocalV3OrdinaryFamilyToken>,
    pub exact_certified_nu: Option<u32>,
    pub local_role_injection_holds: bool,
    pub pre_existing_a3_output_injection_holds: bool,
    pub candidate_minted_output_position_count: usize,
    pub uniform_specializations_not_multiplied: bool,
    pub all_package_and_family_quotients_replayed: bool,
    pub forbidden_inputs: ActLocalV3ForbiddenInputAudit,
    pub theorem_gaps: Vec<ActLocalV3Gap>,
    pub authoritative: bool,
    pub derivation_hash: String,
}

impl ActLocalProvenanceV3Certificate {
    pub fn authoritative_token_hashes(&self) -> Vec<String> {
        self.ordinary_family_tokens
            .iter()
            .map(|token| token.derivation_hash.clone())
            .collect()
    }

    pub fn counted_family_ids(&self) -> Vec<String> {
        self.ordinary_family_tokens
            .iter()
            .map(|token| token.semantic_family_id.clone())
            .collect()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3ForbiddenInputAudit {
    pub archive_read: bool,
    pub historical_total_read: bool,
    pub legacy_novelty_scalar_called: bool,
    pub bar_read: bool,
    pub verdict_read: bool,
    pub enacted_future_read: bool,
    pub caller_supplied_demand_timeline_read: bool,
}

impl ActLocalV3ForbiddenInputAudit {
    fn clean() -> Self {
        Self {
            archive_read: false,
            historical_total_read: false,
            legacy_novelty_scalar_called: false,
            bar_read: false,
            verdict_read: false,
            enacted_future_read: false,
            caller_supplied_demand_timeline_read: false,
        }
    }

    fn none(&self) -> bool {
        !self.archive_read
            && !self.historical_total_read
            && !self.legacy_novelty_scalar_called
            && !self.bar_read
            && !self.verdict_read
            && !self.enacted_future_read
            && !self.caller_supplied_demand_timeline_read
    }
}

/// Closed, typed authority for the one R2 quotient used by the prefix-local
/// declaration issuer.  The slot is typed by the semantic mechanism/role
/// enums; the token has no path, markdown, archive, or caller-selected count
/// input.  Reissuance, rather than possession of a matching digest alone, is
/// the authority check.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3TypedR2RuleToken {
    pub schema: String,
    pub theorem_id: String,
    pub generated_child_kind: String,
    pub surviving_parent_kind: String,
    pub slot_mechanism: CreditMechanism,
    pub slot_role: LocalRole,
    pub same_candidate_and_owner_clause_required: bool,
    pub surviving_parent_must_be_distinct: bool,
    pub e4_generator_membership_witness_required: bool,
    pub operational_membership_verdict_issued: bool,
    pub conclusion: String,
    pub markdown_or_file_read: bool,
    pub archived_count_or_label_read: bool,
    pub derivation_hash: String,
}

impl ActLocalV3TypedR2RuleToken {
    pub fn authority_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3PrefixNaturalFamilyRow {
    pub semantic_family_id: String,
    pub family_shape_key: String,
    pub representative_role: ActLocalV3RoleOccurrence,
    pub occurrence_count: usize,
    pub collapsed_uniform_instance_count: usize,
    pub generator_univalent_key: String,
    pub generator_canonical_normal_form: Expr,
    pub generator_parameter_sorts: Vec<String>,
    pub generator_kernel_type_json: String,
    pub generator_typed_normalized_natural: bool,
    pub marginal: bool,
    pub removed_by_r2: bool,
    pub gap_id: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3PrefixCapabilityAudit {
    pub candidate_and_exact_prefix_capability: bool,
    pub predecessor_projection_capability: bool,
    pub typed_r2_rule_token_capability: bool,
    pub markdown_or_file_source_scan_capability: bool,
    pub historical_v3_package_capability: bool,
    pub historical_counter_or_upper_bound_capability: bool,
    pub predecessor_full_package_hash_capability: bool,
    pub count_based_predecessor_selector_capability: bool,
}

impl ActLocalV3PrefixCapabilityAudit {
    fn source_first() -> Self {
        Self {
            candidate_and_exact_prefix_capability: true,
            predecessor_projection_capability: true,
            typed_r2_rule_token_capability: true,
            markdown_or_file_source_scan_capability: false,
            historical_v3_package_capability: false,
            historical_counter_or_upper_bound_capability: false,
            predecessor_full_package_hash_capability: false,
            count_based_predecessor_selector_capability: false,
        }
    }

    pub fn source_first_only(&self) -> bool {
        self.candidate_and_exact_prefix_capability
            && self.predecessor_projection_capability
            && self.typed_r2_rule_token_capability
            && !self.markdown_or_file_source_scan_capability
            && !self.historical_v3_package_capability
            && !self.historical_counter_or_upper_bound_capability
            && !self.predecessor_full_package_hash_capability
            && !self.count_based_predecessor_selector_capability
    }
}

/// The declaration-only projection consumed by the source-first v4/v5 path.
/// In particular, this type has no field for a historical v3 certificate
/// hash, archive counter, scalar upper bound, or predecessor full-package
/// hash.  Its predecessor chain consists exclusively of hashes of this same
/// projection type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV3PrefixDeclaration {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub kappa: u32,
    pub r2_rule_authority_hash: String,
    pub predecessor_projection_hashes: Vec<String>,
    pub predecessor_surface_digest: String,
    pub role_occurrences_before_quotient: Vec<ActLocalV3RoleOccurrence>,
    pub natural_family_rows: Vec<ActLocalV3PrefixNaturalFamilyRow>,
    pub theorem_gaps: Vec<ActLocalV3Gap>,
    pub r2_rule_application_total: bool,
    pub quotient_partition_exact: bool,
    pub uniform_specializations_not_multiplied: bool,
    pub all_structurally_typed_predecessors_enumerated: bool,
    pub capability_audit: ActLocalV3PrefixCapabilityAudit,
    pub surface_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ActLocalProvenanceV3Error {
    #[error("invalid v3 act-local input: {0}")]
    Input(String),
    #[error("v3 typed-family construction failed: {0}")]
    Family(String),
    #[error("v3 semantic-role enumeration failed: {0}")]
    Enumeration(String),
}

fn as_u16(index: usize) -> Result<u16, ActLocalProvenanceV3Error> {
    u16::try_from(index)
        .map_err(|_| ActLocalProvenanceV3Error::Enumeration("clause index exceeds u16".to_owned()))
}

fn is_type_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ)
        || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ))
        || matches!(expr, Expr::Trunc(_))
}

fn is_intro(expr: &Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) | Expr::Lam(_) | Expr::Var(_) => true,
        Expr::App(function, _) => !matches!(function.as_ref(), Expr::Lam(_)),
        _ => false,
    }
}

fn is_elim(expr: &Expr) -> bool {
    matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)))
}

fn has_direct_lib(expr: &Expr) -> bool {
    match expr {
        Expr::Lib(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            has_direct_lib(left) || has_direct_lib(right)
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
        | Expr::WhyNot(inner) => has_direct_lib(inner),
        Expr::Id(ty, left, right) => {
            has_direct_lib(ty) || has_direct_lib(left) || has_direct_lib(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::PathCon(_) => false,
    }
}

fn has_operator_ref(expr: &Expr) -> bool {
    match expr {
        Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Bang(_)
        | Expr::WhyNot(_) => true,
        Expr::Pi(left, right) | Expr::Sigma(left, right) | Expr::App(left, right) => {
            has_operator_ref(left) || has_operator_ref(right)
        }
        Expr::Lam(inner) | Expr::Refl(inner) | Expr::Susp(inner) | Expr::Trunc(inner) => {
            has_operator_ref(inner)
        }
        Expr::Id(ty, left, right) => {
            has_operator_ref(ty) || has_operator_ref(left) || has_operator_ref(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn pi_is_preservation(domain: &Expr, codomain: &Expr) -> bool {
    matches!((domain, codomain), (Expr::Var(left), Expr::Var(right)) if left == right)
}

fn is_axiomatic_intro(expr: &Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) | Expr::Lam(_) | Expr::Var(_) => true,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_) | Expr::Lib(_)) => false,
        Expr::App(_, _) => true,
        Expr::Pi(domain, codomain) => {
            !has_direct_lib(domain)
                && !has_direct_lib(codomain)
                && !has_operator_ref(domain)
                && !has_operator_ref(codomain)
                && !pi_is_preservation(domain, codomain)
        }
        _ => false,
    }
}

fn is_parametric_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(_)))
}

fn is_modal_wrapping_temporal(expr: &Expr) -> bool {
    matches!(expr, Expr::Flat(inner) | Expr::Sharp(inner) | Expr::Disc(inner) | Expr::Shape(inner)
        if matches!(inner.as_ref(), Expr::Next(_) | Expr::Eventually(_) | Expr::Bang(_) | Expr::WhyNot(_)))
}

fn is_temporal_wrapping_modal(expr: &Expr) -> bool {
    matches!(expr, Expr::Next(inner) | Expr::Eventually(inner) | Expr::Bang(inner) | Expr::WhyNot(inner)
        if matches!(inner.as_ref(), Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_)))
}

fn is_distributive_law(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(domain, codomain)
        if (is_modal_wrapping_temporal(domain) && is_temporal_wrapping_modal(codomain))
            || (is_temporal_wrapping_modal(domain) && is_modal_wrapping_temporal(codomain)))
}

fn is_polymorphic_temporal_elim(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::App(function, _) if matches!(function.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))))
        || matches!(expr, Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Next(inner2) | Expr::Bang(inner2) if matches!(inner2.as_ref(), Expr::Var(_))))
                && matches!(codomain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_))))
        || matches!(expr, Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                && matches!(codomain.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_))))
}

fn is_spatial_temporal_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body)
        if matches!(body.as_ref(), Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Lib(_))
                && matches!(argument.as_ref(), Expr::Next(inner) | Expr::Eventually(inner) | Expr::Bang(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))))
}

fn clause_for_ref(telescope: &Telescope, step: u32) -> Result<u16, ActLocalProvenanceV3Error> {
    telescope
        .clauses
        .iter()
        .position(|clause| clause.expr.lib_refs().contains(&step))
        .ok_or_else(|| {
            ActLocalProvenanceV3Error::Enumeration(format!(
                "candidate reference {step} has no owning clause"
            ))
        })
        .and_then(as_u16)
}

fn modal_kinds(telescope: &Telescope) -> Vec<(String, u16)> {
    ["flat", "sharp", "disc", "shape"]
        .into_iter()
        .filter_map(|name| {
            telescope
                .clauses
                .iter()
                .position(|clause| match name {
                    "flat" => matches!(clause.expr, Expr::Flat(_)),
                    "sharp" => matches!(clause.expr, Expr::Sharp(_)),
                    "disc" => matches!(clause.expr, Expr::Disc(_)),
                    "shape" => matches!(clause.expr, Expr::Shape(_)),
                    _ => false,
                })
                .and_then(|index| u16::try_from(index).ok())
                .map(|index| (name.to_owned(), index))
        })
        .collect()
}

fn dominant_reference(
    refs: impl IntoIterator<Item = u32>,
    packages: &[ActLocalProvenanceV3Certificate],
    predicate: impl Fn(u32) -> bool,
) -> Option<u32> {
    refs.into_iter()
        .filter(|step| predicate(*step))
        .filter_map(|step| {
            packages
                .iter()
                .find(|package| package.stage == step)
                .map(|package| (step, package.natural_family_count_after_quotient))
        })
        .max_by_key(|(step, family_count)| (*family_count, Reverse(*step)))
        .map(|(step, _)| step)
}

fn enumerate_role_occurrences(
    candidate: &Telescope,
    library: &Library,
    prefix_packages: &[ActLocalProvenanceV3Certificate],
    prefix_telescopes: &[(u32, Telescope)],
) -> Result<Vec<RoleOccurrence>, ActLocalProvenanceV3Error> {
    use CreditMechanism as M;
    use LocalRole as L;

    let class = candidate.classify(library);
    let mut rows = Vec::new();
    match class {
        TelescopeClass::Foundation => {
            if let Some(universe) = candidate
                .clauses
                .iter()
                .position(|clause| matches!(clause.expr, Expr::Univ))
            {
                for index in 0..candidate.clauses.len() {
                    if index != universe {
                        rows.push(role(
                            "foundation_completion",
                            as_u16(index)?,
                            M::IntrinsicKernel,
                            L::KernelHead,
                            json!({"r1_package": R1_PACKAGE_THEOREM_ID}),
                        ));
                    }
                }
            } else {
                for (index, clause) in candidate.clauses.iter().enumerate() {
                    if is_type_formation(&clause.expr) {
                        rows.push(role(
                            "foundation_formation",
                            as_u16(index)?,
                            M::IntrinsicKernel,
                            L::KernelHead,
                            json!({}),
                        ));
                    }
                }
            }
        }
        TelescopeClass::Former => {
            for (index, clause) in candidate.clauses.iter().enumerate() {
                let owner = as_u16(index)?;
                if is_intro(&clause.expr) {
                    rows.push(role(
                        "former_introduction",
                        owner,
                        M::IntrinsicKernel,
                        L::KernelHead,
                        json!({}),
                    ));
                    rows.push(role(
                        "former_adjoint",
                        owner,
                        M::AdjointCompletion,
                        L::AdjointMate,
                        json!({}),
                    ));
                }
                if is_elim(&clause.expr) {
                    rows.push(role(
                        "former_eliminator",
                        owner,
                        M::AdjointCompletion,
                        L::AdjointMate,
                        json!({}),
                    ));
                }
            }
        }
        TelescopeClass::Hit => {
            let paths = candidate
                .clauses
                .iter()
                .enumerate()
                .filter_map(|(index, clause)| match clause.expr {
                    Expr::PathCon(dimension) => Some((index, dimension)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let (first_path, _) = paths.first().copied().ok_or_else(|| {
                ActLocalProvenanceV3Error::Enumeration("HIT has no path clause".to_owned())
            })?;
            let formation = candidate
                .clauses
                .iter()
                .position(|clause| is_type_formation(&clause.expr));
            if let Some(formation_index) = formation {
                for index in 0..first_path {
                    rows.push(role(
                        "hit_pre_path_declaration",
                        as_u16(index)?,
                        M::IntrinsicKernel,
                        L::KernelHead,
                        json!({}),
                    ));
                }
                let owner = as_u16(formation_index)?;
                for package_role in 0..3_u8 {
                    let (mechanism, local_role) = match package_role {
                        1 => (M::AdjointCompletion, L::AdjointMate),
                        2 => (M::IntrinsicKernel, L::SupportAction),
                        _ => (M::IntrinsicKernel, L::KernelHead),
                    };
                    rows.push(role(
                        "hit_formation_package",
                        owner,
                        mechanism,
                        local_role,
                        json!({"package_role": package_role}),
                    ));
                }
                if candidate
                    .clauses
                    .iter()
                    .any(|clause| is_parametric_formation(&clause.expr))
                {
                    rows.push(role(
                        "hit_parametric_formation_action",
                        owner,
                        M::P6UniformSpecialization,
                        L::SupportAction,
                        json!({}),
                    ));
                }
            }
            for (index, dimension) in &paths {
                let owner = as_u16(*index)?;
                rows.push(role(
                    "hit_path_beta",
                    owner,
                    M::IntrinsicKernel,
                    L::KernelHead,
                    json!({"dimension": dimension}),
                ));
                for left_axis in 0..*dimension {
                    for right_axis in 0..*dimension {
                        rows.push(role(
                            "hit_kan_coherence",
                            owner,
                            M::DimensionSquared,
                            L::Coherence,
                            json!({"left_axis": left_axis, "right_axis": right_axis}),
                        ));
                    }
                }
            }
            if formation.is_some() {
                let post = (first_path + 1..candidate.clauses.len()).collect::<Vec<_>>();
                for index in &post {
                    rows.push(role(
                        "hit_post_path_face",
                        as_u16(*index)?,
                        M::P6UniformSpecialization,
                        L::SupportAction,
                        json!({}),
                    ));
                }
                for operation_slot in 0..post.len().div_ceil(2) {
                    let index = post[2 * operation_slot];
                    let mut generated = role(
                        "hit_canonical_operation_action",
                        as_u16(index)?,
                        M::P6UniformSpecialization,
                        L::SupportAction,
                        json!({"operation_slot": operation_slot}),
                    );
                    generated.removed_by_r2 = true;
                    rows.push(generated);
                }
            } else {
                let path_owner = as_u16(first_path)?;
                for index in 0..candidate.clauses.len() {
                    rows.push(role(
                        "hit_unformed_local_face",
                        as_u16(index)?,
                        M::IntrinsicKernel,
                        L::KernelHead,
                        json!({}),
                    ));
                }
                for target_step in 1..=library.len() as u32 {
                    rows.push(uniform_role(
                        "hit_unformed_prefix_action",
                        path_owner,
                        M::P6UniformSpecialization,
                        L::SupportAction,
                        json!({"step": target_step}),
                    ));
                }
            }
        }
        TelescopeClass::Suspension => {
            for (operation, mechanism, local_role) in [
                ("formation", M::IntrinsicKernel, L::KernelHead),
                ("north_point", M::IntrinsicKernel, L::KernelHead),
                ("south_point", M::IntrinsicKernel, L::SupportAction),
                ("meridian", M::DimensionSquared, L::Coherence),
                ("eliminator", M::AdjointCompletion, L::AdjointMate),
            ] {
                rows.push(role(
                    "suspension_operation",
                    0,
                    mechanism,
                    local_role,
                    json!({"operation": operation}),
                ));
            }
        }
        TelescopeClass::Map => {
            if candidate.kappa() == 1 {
                rows.push(role(
                    "map_single_head",
                    0,
                    M::IntrinsicKernel,
                    L::KernelHead,
                    json!({}),
                ));
                rows.push(role(
                    "map_single_action",
                    0,
                    M::AdjointCompletion,
                    L::AdjointMate,
                    json!({}),
                ));
            } else {
                for index in 0..candidate.clauses.len() {
                    let owner = as_u16(index)?;
                    rows.push(role(
                        "map_precomposition",
                        owner,
                        M::AdjointCompletion,
                        L::AdjointMate,
                        json!({}),
                    ));
                    rows.push(role(
                        "map_postcomposition",
                        owner,
                        M::AdjointCompletion,
                        L::AdjointMate,
                        json!({}),
                    ));
                }
                let refs = candidate.lib_refs().into_iter().collect::<Vec<_>>();
                for left_step in &refs {
                    for right_step in &refs {
                        rows.push(role(
                            "map_reference_coherence",
                            clause_for_ref(candidate, *left_step)?,
                            M::ReferenceSquared,
                            L::Coherence,
                            json!({"left_step": left_step, "right_step": right_step}),
                        ));
                    }
                }
            }
        }
        TelescopeClass::Modal => {
            for index in 0..candidate.clauses.len() {
                rows.push(role(
                    "modal_local_declaration",
                    as_u16(index)?,
                    M::IntrinsicKernel,
                    L::KernelHead,
                    json!({}),
                ));
            }
            let kinds = modal_kinds(candidate);
            let owner = kinds.first().map(|(_, clause)| *clause).ok_or_else(|| {
                ActLocalProvenanceV3Error::Enumeration("modal act has no modal owner".to_owned())
            })?;
            for target_step in 1..=library.len() as u32 {
                rows.push(uniform_role(
                    "modal_uniform_legacy_action",
                    owner,
                    M::P6UniformSpecialization,
                    L::SupportAction,
                    json!({"step": target_step}),
                ));
            }
            for left in 0..kinds.len() {
                for right in left + 1..kinds.len() {
                    rows.push(role(
                        "modal_pairwise_coherence",
                        kinds[left].1,
                        M::ModalPairwiseCoherence,
                        L::Coherence,
                        json!({"left_kind": kinds[left].0, "right_kind": kinds[right].0}),
                    ));
                }
            }
        }
        TelescopeClass::Axiomatic => {
            for (index, clause) in candidate.clauses.iter().enumerate() {
                if is_axiomatic_intro(&clause.expr) {
                    rows.push(role(
                        "axiomatic_introduction_head",
                        as_u16(index)?,
                        M::IntrinsicKernel,
                        L::KernelHead,
                        json!({}),
                    ));
                }
            }
            let refs = candidate.lib_refs().into_iter().collect::<Vec<_>>();
            if let Some(source_step) =
                dominant_reference(refs.iter().copied(), prefix_packages, |_| true)
            {
                let source = prefix_packages
                    .iter()
                    .find(|package| package.stage == source_step)
                    .ok_or_else(|| {
                        ActLocalProvenanceV3Error::Enumeration(format!(
                            "dominant source step {source_step} is absent"
                        ))
                    })?;
                let owner = clause_for_ref(candidate, source_step)?;
                for family in source
                    .natural_family_rows
                    .iter()
                    .filter(|family| family.marginal && !family.removed_by_r2)
                {
                    rows.push(role("axiomatic_inherited_family", owner, M::P5InheritedSurface, L::SupportAction, json!({"source_step": source_step, "source_family_id": family.semantic_family_id, "source_family_evidence": family.derivation_hash})));
                }
            }
            for index in 0..candidate.clauses.len() {
                rows.push(role(
                    "axiomatic_local_and_bridge_face",
                    as_u16(index)?,
                    M::P5LocalAndBridge,
                    L::SupportAction,
                    json!({}),
                ));
            }
            for pair in refs.windows(2) {
                rows.push(role(
                    "axiomatic_support_bridge",
                    clause_for_ref(candidate, pair[1])?,
                    M::P5LocalAndBridge,
                    L::SupportAction,
                    json!({"left_step": pair[0], "right_step": pair[1]}),
                ));
            }
        }
        TelescopeClass::Synthesis => {
            for index in 0..candidate.clauses.len() {
                rows.push(role(
                    "synthesis_local_declaration",
                    as_u16(index)?,
                    M::IntrinsicKernel,
                    L::KernelHead,
                    json!({}),
                ));
            }
            for (index, clause) in candidate.clauses.iter().enumerate() {
                if is_polymorphic_temporal_elim(&clause.expr) {
                    for target_step in 1..=library.len() as u32 {
                        rows.push(uniform_role(
                            "synthesis_uniform_temporal_action",
                            as_u16(index)?,
                            M::P6UniformSpecialization,
                            L::SupportAction,
                            json!({"step": target_step}),
                        ));
                    }
                }
            }
            let modal_source = dominant_reference(candidate.lib_refs(), prefix_packages, |step| {
                library
                    .get(step.saturating_sub(1) as usize)
                    .is_some_and(|entry| entry.capabilities.has_modal_ops)
            });
            if let Some(source_step) = modal_source {
                let source = prefix_packages
                    .iter()
                    .find(|package| package.stage == source_step)
                    .ok_or_else(|| {
                        ActLocalProvenanceV3Error::Enumeration(format!(
                            "modal source step {source_step} is absent"
                        ))
                    })?;
                for (index, clause) in candidate.clauses.iter().enumerate() {
                    if is_distributive_law(&clause.expr) {
                        for family in source
                            .natural_family_rows
                            .iter()
                            .filter(|family| family.marginal && !family.removed_by_r2)
                        {
                            rows.push(role("synthesis_distributive_transport", as_u16(index)?, M::DistributiveInheritance, L::Coherence, json!({"source_step": source_step, "source_family_id": family.semantic_family_id, "source_family_evidence": family.derivation_hash})));
                        }
                    }
                }
            }
            if let Some(owner) = candidate
                .clauses
                .iter()
                .position(|clause| is_spatial_temporal_clause(&clause.expr))
            {
                let owner = as_u16(owner)?;
                for (source_step, source_telescope) in prefix_telescopes {
                    for (path_index, path_clause) in source_telescope.clauses.iter().enumerate() {
                        let Expr::PathCon(dimension) = path_clause.expr else {
                            continue;
                        };
                        for left_axis in 0..dimension {
                            for right_axis in 0..dimension {
                                rows.push(role("synthesis_infinitesimal_shift", owner, M::CombinatorialSynthesis, L::SupportAction, json!({"source_step": source_step, "source_path_clause": path_index, "left_axis": left_axis, "right_axis": right_axis})));
                            }
                        }
                    }
                }
            }
        }
        TelescopeClass::Unknown => {
            for (index, clause) in candidate.clauses.iter().enumerate() {
                let owner = as_u16(index)?;
                if is_intro(&clause.expr) {
                    rows.push(role(
                        "generic_introduction",
                        owner,
                        M::IntrinsicKernel,
                        L::KernelHead,
                        json!({}),
                    ));
                    rows.push(role(
                        "generic_adjoint",
                        owner,
                        M::AdjointCompletion,
                        L::AdjointMate,
                        json!({}),
                    ));
                }
                if is_elim(&clause.expr) {
                    rows.push(role(
                        "generic_eliminator",
                        owner,
                        M::AdjointCompletion,
                        L::AdjointMate,
                        json!({}),
                    ));
                }
            }
        }
    }
    Ok(rows)
}

fn generator_for_clause(families: &[ExtractedFamily], clause: u16) -> Option<&ExtractedFamily> {
    families.iter().find(|family| {
        family
            .instances
            .iter()
            .any(|instance| instance.clause_index == clause)
    })
}

fn generator_evidence(
    family: &ExtractedFamily,
    owner_clause: u16,
    extraction_hash: &str,
) -> ActLocalV3GeneratorEvidence {
    let parameter_sorts = family
        .presentation
        .parameters
        .iter()
        .map(|sort| format!("{sort:?}"))
        .collect::<Vec<_>>();
    let kernel_type_json =
        serde_json::to_string(&family.generator_kernel_ty).expect("kernel type serializes");
    let generator_univalent_key = tagged_hash(
        "generator-univalent-key",
        &(
            &family.presentation.canonical_normal_form,
            &parameter_sorts,
            &kernel_type_json,
        ),
    );
    let naturality_square_hash = tagged_hash("generator-naturality", &family.naturality);
    let owner_is_generator_or_instance = family.instances.iter().any(|record| {
        record.clause_index == owner_clause
            && matches!(
                record.kind,
                InstanceKind::Generator
                    | InstanceKind::RenamingInstance
                    | InstanceKind::Specialization { .. }
            )
    });
    let mut value = ActLocalV3GeneratorEvidence {
        owner_clause,
        extractor_family_id: family.id.as_str().to_owned(),
        generator_univalent_key,
        canonical_normal_form: family.presentation.canonical_normal_form.clone(),
        parameter_sorts,
        kernel_type_json,
        extraction_hash: extraction_hash.to_owned(),
        owner_clause_normalization_and_typing_replayed: true,
        naturality_square_hash,
        naturality_square_equal: family.naturality.square.equal,
        owner_is_generator_or_instance,
        // The extractor proves the owning clause family.  It does not yet
        // construct the distinct P5/P6/d2/r2/synthesis role expression.
        role_schema_term_constructed: false,
        role_schema_typing_replayed: false,
        role_schema_naturality_replayed: false,
        derivation_hash: String::new(),
    };
    value.derivation_hash = tagged_hash("generator-evidence", &value);
    value
}

fn marginality_sweep(
    family_shape_key: &str,
    generator: &ActLocalV3GeneratorEvidence,
    prior: &[ActLocalV3NaturalFamilyRow],
) -> Result<ActLocalV3MarginalityEvidence, ActLocalProvenanceV3Error> {
    let scope = generator.parameter_sorts.len() as u32;
    let mut comparisons = Vec::with_capacity(prior.len());
    let mut same_shape = 0usize;
    let mut preimages = 0usize;
    for family in prior {
        let comparable = family.family_shape_key == family_shape_key
            && family.generator.parameter_sorts == generator.parameter_sorts
            && family.generator.kernel_type_json == generator.kernel_type_json;
        let equality = if comparable {
            same_shape += 1;
            let witness = univalent_equality(
                &family.generator.canonical_normal_form,
                &generator.canonical_normal_form,
                scope,
                256,
            )
            .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
            preimages += usize::from(witness.equal);
            Some((witness.equal, tagged_hash("marginality-equality", &witness)))
        } else {
            None
        };
        comparisons.push((family.semantic_family_id.as_str(), comparable, equality));
    }
    Ok(ActLocalV3MarginalityEvidence {
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        predecessor_family_count: prior.len(),
        same_shape_comparison_count: same_shape,
        univalent_preimage_count: preimages,
        no_predecessor_preimage: preimages == 0,
        full_sweep_hash: tagged_hash("marginality-full-sweep", &comparisons),
    })
}

fn output_kind(output: &A3DemandOutputType) -> &'static str {
    match output {
        A3DemandOutputType::ActionAt { .. } => "action_at",
        A3DemandOutputType::ChronologicalInteraction { .. } => "chronological_interaction",
        A3DemandOutputType::ContractibleOpenBox { .. } => "contractible_open_box",
        A3DemandOutputType::StructuralCompletion { .. } => "structural_completion",
    }
}

fn audit_a3_surface(
    window: &A3HistoricalWindow,
) -> Result<Vec<ActLocalV3A3OrbitAudit>, ActLocalProvenanceV3Error> {
    let schemes = window
        .schemes
        .iter()
        .map(|scheme| (scheme.scheme_id.as_str(), scheme))
        .collect::<BTreeMap<_, _>>();
    let instances = window
        .instances
        .iter()
        .map(|instance| instance.instance_id.as_str())
        .collect::<BTreeSet<_>>();
    window
        .orbits
        .iter()
        .map(|orbit| {
            let scheme = schemes.get(orbit.scheme_id.as_str()).ok_or_else(|| {
                ActLocalProvenanceV3Error::Family(format!(
                    "A3 orbit {} has no scheme",
                    orbit.orbit_id
                ))
            })?;
            if !orbit
                .member_instance_ids
                .iter()
                .all(|instance| instances.contains(instance.as_str()))
            {
                return Err(ActLocalProvenanceV3Error::Family(format!(
                    "A3 orbit {} contains an unknown instance",
                    orbit.orbit_id
                )));
            }
            let required_output_hash = tagged_hash(
                "pre-candidate-A3-required-output",
                &(
                    &orbit.orbit_id,
                    &orbit.scheme_id,
                    &scheme.required_output,
                    &scheme.formation_derivation_hash,
                ),
            );
            let mut audit = ActLocalV3A3OrbitAudit {
                orbit_id: orbit.orbit_id.clone(),
                scheme_id: orbit.scheme_id.clone(),
                rule_constructor: format!("{:?}", scheme.rule_constructor),
                required_output_kind: output_kind(&scheme.required_output).to_owned(),
                required_output_hash,
                representative_instance_id: orbit.representative_instance_id.clone(),
                member_instance_count: orbit.member_instance_ids.len(),
                uniform_specializations_collapsed: orbit.uniform_specializations_collapsed,
                independently_exported_demand_orbit: orbit.independently_exported_demand_orbit,
                existed_before_candidate: true,
                // These are facts of A3GeneratorBoundary, not missing fields
                // defaulted by this issuer.
                output_term_constructed_on_generator_boundary: false,
                output_term_kernel_typed_on_generator_boundary: false,
                usable_as_ordinary_credit_without_new_theorem: false,
                orbit_derivation_hash: orbit.orbit_derivation_hash.clone(),
                derivation_hash: String::new(),
            };
            audit.derivation_hash = tagged_hash("A3-orbit-audit", &audit);
            Ok(audit)
        })
        .collect()
}

fn certificate_digest(certificate: &ActLocalProvenanceV3Certificate) -> String {
    let mut projection = certificate.clone();
    projection.derivation_hash.clear();
    tagged_hash("certificate", &projection)
}

fn validate_prefix(prefix: &SealedSignature, stage: u32) -> Result<(), ActLocalProvenanceV3Error> {
    if stage == 0
        || prefix.entries().len() != stage.saturating_sub(1) as usize
        || !prefix.entries().iter().map(|entry| entry.step).eq(1..stage)
    {
        return Err(ActLocalProvenanceV3Error::Input(format!(
            "Stage {stage} requires the exact contiguous predecessor prefix"
        )));
    }
    Ok(())
}

fn issue_one(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    prefix_packages: &[ActLocalProvenanceV3Certificate],
) -> Result<ActLocalProvenanceV3Certificate, ActLocalProvenanceV3Error> {
    validate_prefix(prefix, stage)?;
    let mut library: Library = Vec::new();
    for entry in prefix.entries() {
        library.push(LibraryEntry::from_telescope(&entry.telescope, &library));
    }
    let elaboration = elaborate_telescope(prefix, candidate, stage.saturating_sub(1))
        .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
    let closure = predecessor_closure(prefix)
        .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
    let CandidateExtractionOutcome::Extracted(extraction) =
        extract_candidate_families(prefix, &closure, candidate, stage.saturating_sub(1))
    else {
        return Err(ActLocalProvenanceV3Error::Family(
            "candidate family extraction was kernel-invalid after elaboration".to_owned(),
        ));
    };
    let window = generate_a3_window_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
    let inventory_proof = prove_a3_window_inventory_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
    let a3_orbits = audit_a3_surface(&window)?;
    let independently_exported_a3_orbit_count = a3_orbits
        .iter()
        .filter(|orbit| orbit.independently_exported_demand_orbit)
        .count();
    let relative_a3_rule_inventory_exhaustive_for_window = inventory_proof
        .relative_rule_inventory_exhaustive_for_window
        && inventory_proof.exact_prefix_signature_digest == prefix.digest()
        && inventory_proof.operational_instance_count == window.instances.len()
        && inventory_proof.every_operational_instance_has_exactly_one_independent_preimage
        && inventory_proof.every_operational_scheme_and_orbit_is_consumed;
    let full_pre_candidate_a3_surface_replayed = window.finite_by_construction
        && window.every_instance_typed
        && relative_a3_rule_inventory_exhaustive_for_window
        && a3_orbits.len() == window.orbits.len()
        && a3_orbits
            .iter()
            .all(|orbit| orbit.existed_before_candidate && orbit.uniform_specializations_collapsed);

    let prefix_telescopes = prefix
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.telescope.clone()))
        .collect::<Vec<_>>();
    let roles =
        enumerate_role_occurrences(candidate, &library, prefix_packages, &prefix_telescopes)?;
    let role_occurrences_before_quotient = roles
        .iter()
        .map(RoleOccurrence::serializable)
        .collect::<Vec<_>>();
    let r2_text = std::str::from_utf8(R2_ADJUDICATION_BYTES)
        .map_err(|error| ActLocalProvenanceV3Error::Input(error.to_string()))?;
    let r2_replayed = r2_text.contains(R2_GENERATED_ACTION_THEOREM_ID)
        && r2_text.contains("**R2 adopted**")
        && r2_text.contains("position, label, or archived cardinality");
    if !r2_replayed {
        return Err(ActLocalProvenanceV3Error::Input(
            "adopted count-blind R2 source did not replay".to_owned(),
        ));
    }

    let mut grouped: BTreeMap<String, Vec<(RoleOccurrence, ActLocalV3GeneratorEvidence)>> =
        BTreeMap::new();
    let mut removed_by_r2 = 0usize;
    for occurrence in roles {
        if occurrence.removed_by_r2 {
            removed_by_r2 += 1;
            continue;
        }
        let family = generator_for_clause(&extraction.families, occurrence.owner_clause)
            .ok_or_else(|| {
                ActLocalProvenanceV3Error::Family(format!(
                    "role {} at clause {} has no extracted owner family",
                    occurrence.kind, occurrence.owner_clause
                ))
            })?;
        let generator =
            generator_evidence(family, occurrence.owner_clause, &extraction.derivation_hash);
        if !generator.owner_clause_normalization_and_typing_replayed
            || !generator.naturality_square_equal
            || !generator.owner_is_generator_or_instance
        {
            return Err(ActLocalProvenanceV3Error::Family(format!(
                "role {} lacks typed normalized natural owner evidence",
                occurrence.kind
            )));
        }
        let shape_key = tagged_hash(
            "natural-family-shape",
            &(
                &occurrence.natural_family_shape,
                &generator.generator_univalent_key,
                &generator.parameter_sorts,
                &generator.kernel_type_json,
            ),
        );
        grouped
            .entry(shape_key)
            .or_default()
            .push((occurrence, generator));
    }

    let prior_families = prefix_packages
        .iter()
        .flat_map(|package| package.natural_family_rows.iter().cloned())
        .filter(|family| !family.removed_by_r2)
        .collect::<Vec<_>>();
    let candidate_digest = candidate_hash(candidate);
    let mut gaps = Vec::new();
    let mut family_rows = Vec::new();
    let mut tokens = Vec::new();
    let mut local_slots = BTreeSet::<(u16, LocalRole)>::new();
    let mut used_a3_outputs = BTreeSet::<(String, String)>::new();

    for (shape_key, members) in grouped {
        let (representative, generator) = &members[0];
        let semantic_family_id = tagged_hash(
            "semantic-natural-family",
            &(
                &shape_key,
                &generator.generator_univalent_key,
                &generator.derivation_hash,
            ),
        );
        let marginality = marginality_sweep(&shape_key, generator, &prior_families)?;
        let marginal = marginality.no_predecessor_preimage;
        let mut credited = false;
        let mut family_gap_id = None;
        if marginal {
            let role_schema_replayed = generator.role_schema_term_constructed
                && generator.role_schema_typing_replayed
                && generator.role_schema_naturality_replayed;
            let local = (representative.owner_clause, representative.local_role);
            let anchor = if !role_schema_replayed {
                let theorem_gap = gap(
                    stage,
                    Some(&semantic_family_id),
                    "ROLE_SCHEMA_EXTRACTION_GAP",
                    format!(
                        "the typed-family extractor proves owner clause {} but does not construct, type, or naturalize semantic role `{}`; owner typing cannot be promoted to role-family typing",
                        representative.owner_clause, representative.kind
                    ),
                );
                family_gap_id = Some(theorem_gap.id.clone());
                gaps.push(theorem_gap);
                None
            } else if local_slots.insert(local) {
                Some(ActLocalV3Anchor::ChargedLocalRole {
                    clause: local.0,
                    role: local.1,
                    injection_hash: tagged_hash(
                        "local-role-injection",
                        &(stage, &candidate_digest, &semantic_family_id, local),
                    ),
                })
            } else {
                // A3 is swept in full.  No output is silently selected: the
                // current generator boundary constructs/types none of them.
                a3_orbits
                    .iter()
                    .find(|orbit| {
                        orbit.usable_as_ordinary_credit_without_new_theorem
                            && used_a3_outputs.insert((
                                orbit.orbit_id.clone(),
                                orbit.required_output_hash.clone(),
                            ))
                    })
                    .map(|orbit| ActLocalV3Anchor::PreExistingA3RequiredOutput {
                        orbit_id: orbit.orbit_id.clone(),
                        scheme_id: orbit.scheme_id.clone(),
                        required_output_hash: orbit.required_output_hash.clone(),
                        orbit_derivation_hash: orbit.orbit_derivation_hash.clone(),
                    })
            };
            if let Some(anchor) = anchor {
                let representative_role = representative.serializable();
                let mut token = ActLocalV3OrdinaryFamilyToken {
                    token_version: ACT_LOCAL_ORDINARY_TOKEN_V3.to_owned(),
                    theorem_id: T_BI_NU1_V3_THEOREM_ID.to_owned(),
                    stage,
                    candidate_hash: candidate_digest.clone(),
                    predecessor_signature_digest: prefix.digest().to_owned(),
                    semantic_family_id: semantic_family_id.clone(),
                    family_shape_key: shape_key.clone(),
                    mechanism: representative.mechanism,
                    representative_role,
                    generator: generator.clone(),
                    marginality: marginality.clone(),
                    anchor,
                    typed_normalized_natural: true,
                    family_or_instance_quotient_decided: true,
                    uniform_specializations_multiplied_without_exported_orbit: false,
                    candidate_and_sealed_prefix_only: true,
                    derivation_hash: String::new(),
                };
                token.derivation_hash = tagged_hash("ordinary-family-token", &token);
                tokens.push(token);
                credited = true;
            } else {
                if role_schema_replayed {
                    let theorem_gap = gap(
                        stage,
                        Some(&semantic_family_id),
                        "A3_REQUIRED_OUTPUT_TERM_NOT_AVAILABLE",
                        format!(
                            "local slot ({},{:?}) is occupied; the full pre-candidate A3 surface has {} orbits ({} independently exported), but A3GeneratorBoundary constructs and kernel-types zero required-output terms. A versioned output-construction/membership theorem is required; the candidate may not mint a position",
                            representative.owner_clause,
                            representative.local_role,
                            a3_orbits.len(),
                            a3_orbits
                                .iter()
                                .filter(|orbit| orbit.independently_exported_demand_orbit)
                                .count()
                        ),
                    );
                    family_gap_id = Some(theorem_gap.id.clone());
                    gaps.push(theorem_gap);
                }
            }
        }
        let collapsed_uniform_instance_count = members
            .iter()
            .skip(1)
            .filter(|(occurrence, _)| occurrence.uniform_specialization)
            .count();
        let mut row = ActLocalV3NaturalFamilyRow {
            semantic_family_id,
            family_shape_key: shape_key,
            representative_role: representative.serializable(),
            occurrence_count: members.len(),
            collapsed_uniform_instance_count,
            generator: generator.clone(),
            marginality,
            marginal,
            removed_by_r2: false,
            credited,
            gap_id: family_gap_id,
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("natural-family-row", &row);
        family_rows.push(row);
    }

    let marginal_natural_family_count = family_rows.iter().filter(|row| row.marginal).count();
    let locally_anchored_family_count = tokens
        .iter()
        .filter(|token| matches!(token.anchor, ActLocalV3Anchor::ChargedLocalRole { .. }))
        .count();
    let a3_output_anchored_family_count = tokens.len() - locally_anchored_family_count;
    let blind_local_role_capacity_4kappa =
        (LocalRole::ALL.len() as u32).saturating_mul(candidate.kappa() as u32);
    let adopted_export_upper_bound_4kappa_plus_exported_orbits = blind_local_role_capacity_4kappa
        .saturating_add(independently_exported_a3_orbit_count as u32);
    let counterfactual_all_quotient_orbits_upper_bound =
        blind_local_role_capacity_4kappa.saturating_add(a3_orbits.len() as u32);
    let invalid_raw_occurrence_upper_bound =
        blind_local_role_capacity_4kappa.saturating_add(window.instances.len() as u32);
    let collapsed_uniform_or_repeated_instance_count = role_occurrences_before_quotient
        .len()
        .saturating_sub(removed_by_r2)
        .saturating_sub(family_rows.len());
    let local_role_injection_holds = local_slots.len() == locally_anchored_family_count;
    let pre_existing_a3_output_injection_holds =
        used_a3_outputs.len() == a3_output_anchored_family_count;
    let uniform_specializations_not_multiplied = family_rows.iter().all(|row| {
        row.occurrence_count == 1
            || row.collapsed_uniform_instance_count <= row.occurrence_count.saturating_sub(1)
    }) && tokens
        .iter()
        .all(|token| !token.uniform_specializations_multiplied_without_exported_orbit);
    let r1_formation_completion_package_replayed = if stage == 1 {
        role_occurrences_before_quotient.len() == 1
            && role_occurrences_before_quotient[0].kind == "foundation_completion"
    } else {
        true
    };
    let all_package_and_family_quotients_replayed = r1_formation_completion_package_replayed
        && r2_replayed
        && uniform_specializations_not_multiplied
        && role_occurrences_before_quotient.len()
            == removed_by_r2
                + family_rows
                    .iter()
                    .map(|row| row.occurrence_count)
                    .sum::<usize>();
    let role_schema_extraction_complete = family_rows.iter().all(|row| {
        row.generator.role_schema_term_constructed
            && row.generator.role_schema_typing_replayed
            && row.generator.role_schema_naturality_replayed
    });
    let forbidden_inputs = ActLocalV3ForbiddenInputAudit::clean();
    let authoritative = gaps.is_empty()
        && full_pre_candidate_a3_surface_replayed
        && all_package_and_family_quotients_replayed
        && role_schema_extraction_complete
        && local_role_injection_holds
        && pre_existing_a3_output_injection_holds
        && tokens.len() == marginal_natural_family_count
        && forbidden_inputs.none();
    let exact_certified_nu = authoritative.then_some(tokens.len() as u32);
    let predecessor_act_local_digest = tagged_hash(
        "predecessor-packages",
        &prefix_packages
            .iter()
            .map(|package| package.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let mut certificate = ActLocalProvenanceV3Certificate {
        schema: ACT_LOCAL_PROVENANCE_V3_SCHEMA.to_owned(),
        date: ACT_LOCAL_PROVENANCE_V3_DATE.to_owned(),
        theorem_id: T_BI_NU1_V3_THEOREM_ID.to_owned(),
        stage,
        candidate_hash: candidate_digest,
        predecessor_signature_digest: prefix.digest().to_owned(),
        predecessor_act_local_digest,
        class: candidate.classify(&library),
        kappa: candidate.kappa() as u32,
        candidate_elaboration_hash: elaboration.derivation_hash,
        candidate_family_extraction_hash: extraction.derivation_hash,
        a3_window_hash: window.window_derivation_hash,
        a3_inventory_exhaustiveness_hash: inventory_proof.derivation_hash,
        a3_window_stage: window.stage,
        a3_scheme_count: window.schemes.len(),
        a3_instance_count: window.instances.len(),
        a3_orbit_count: window.orbits.len(),
        independently_exported_a3_orbit_count,
        a3_orbits,
        relative_a3_rule_inventory_exhaustive_for_window,
        full_pre_candidate_a3_surface_replayed,
        a3_output_terms_constructed_on_generator_boundary: false,
        a3_output_terms_kernel_typed_on_generator_boundary: false,
        role_occurrences_before_quotient,
        role_occurrence_count_before_quotient: 0,
        r1_formation_completion_package_replayed,
        r2_generated_instance_removal_count: removed_by_r2,
        natural_family_rows: family_rows,
        natural_family_count_after_quotient: 0,
        role_schema_extraction_complete,
        collapsed_uniform_or_repeated_instance_count,
        marginal_natural_family_count,
        locally_anchored_family_count,
        a3_output_anchored_family_count,
        blind_local_role_capacity_4kappa,
        adopted_export_upper_bound_4kappa_plus_exported_orbits,
        counterfactual_all_quotient_orbits_upper_bound,
        invalid_raw_occurrence_upper_bound,
        ordinary_family_tokens: tokens,
        exact_certified_nu,
        local_role_injection_holds,
        pre_existing_a3_output_injection_holds,
        candidate_minted_output_position_count: 0,
        uniform_specializations_not_multiplied,
        all_package_and_family_quotients_replayed,
        forbidden_inputs,
        theorem_gaps: gaps,
        authoritative,
        derivation_hash: String::new(),
    };
    certificate.role_occurrence_count_before_quotient =
        certificate.role_occurrences_before_quotient.len();
    certificate.natural_family_count_after_quotient = certificate.natural_family_rows.len();
    certificate.derivation_hash = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn issue_act_local_sequence_v3(
    entries: &[(u32, Telescope)],
) -> Result<Vec<ActLocalProvenanceV3Certificate>, ActLocalProvenanceV3Error> {
    if entries.is_empty()
        || !entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=entries.len() as u32)
    {
        return Err(ActLocalProvenanceV3Error::Input(
            "v3 act-local sequence must be contiguous from Stage 1".to_owned(),
        ));
    }
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut packages = Vec::new();
    for (stage, candidate) in entries {
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let package = issue_one(&prefix, *stage, candidate, &packages)?;
        accepted.push((*stage, candidate.clone()));
        packages.push(package);
    }
    Ok(packages)
}

pub fn issue_act_local_provenance_v3(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
) -> Result<ActLocalProvenanceV3Certificate, ActLocalProvenanceV3Error> {
    validate_prefix(prefix, stage)?;
    let mut entries = prefix
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.telescope.clone()))
        .collect::<Vec<_>>();
    entries.push((stage, candidate.clone()));
    issue_act_local_sequence_v3(&entries)?
        .pop()
        .ok_or_else(|| ActLocalProvenanceV3Error::Input("empty v3 sequence".to_owned()))
}

pub fn replay_act_local_provenance_v3(
    prefix: &SealedSignature,
    candidate: &Telescope,
    claimed: &ActLocalProvenanceV3Certificate,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != certificate_digest(claimed) {
        errors.push("v3 act-local certificate digest mismatch".to_owned());
    }
    match issue_act_local_provenance_v3(prefix, claimed.stage, candidate) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("v3 act-local certificate differs from reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn typed_r2_rule_token_hash(token: &ActLocalV3TypedR2RuleToken) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    prefix_local_tagged_hash("typed-R2-rule-token", &projection)
}

/// Issue the closed R2 rule used by the prefix-local declaration path.
///
/// This issuer intentionally has no parameters: in particular, it cannot
/// receive a filename, markdown text, archived count, role label choice, or
/// desired verdict.  The exact typed rule is therefore replayed from code and
/// then self-digested.
pub fn issue_act_local_v3_typed_r2_rule_token() -> ActLocalV3TypedR2RuleToken {
    let mut token = ActLocalV3TypedR2RuleToken {
        schema: ACT_LOCAL_V3_TYPED_R2_RULE_SCHEMA.to_owned(),
        theorem_id: R2_GENERATED_ACTION_THEOREM_ID.to_owned(),
        generated_child_kind: "hit_canonical_operation_action".to_owned(),
        surviving_parent_kind: "hit_post_path_face".to_owned(),
        slot_mechanism: CreditMechanism::P6UniformSpecialization,
        slot_role: LocalRole::SupportAction,
        same_candidate_and_owner_clause_required: true,
        surviving_parent_must_be_distinct: true,
        e4_generator_membership_witness_required: true,
        operational_membership_verdict_issued: false,
        conclusion:
            "if a separately replayed E-4 generator-membership witness proves that the typed child is generated by the distinct typed parent, the child is quotient membership of that parent rather than an independent natural family"
                .to_owned(),
        markdown_or_file_read: false,
        archived_count_or_label_read: false,
        derivation_hash: String::new(),
    };
    token.derivation_hash = typed_r2_rule_token_hash(&token);
    token
}

pub fn replay_act_local_v3_typed_r2_rule_token(
    claimed: &ActLocalV3TypedR2RuleToken,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != typed_r2_rule_token_hash(claimed) {
        errors.push("v3 typed R2 rule token digest mismatch".to_owned());
    }
    if *claimed != issue_act_local_v3_typed_r2_rule_token() {
        errors.push("v3 typed R2 rule token differs from closed reissuance".to_owned());
    }
    errors
}

fn prefix_natural_family_row_hash(row: &ActLocalV3PrefixNaturalFamilyRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    prefix_local_tagged_hash("natural-family-row", &projection)
}

fn prefix_declaration_surface_hash(surface: &ActLocalV3PrefixDeclaration) -> String {
    let mut projection = surface.clone();
    projection.surface_hash.clear();
    prefix_local_tagged_hash("declaration-surface", &projection)
}

fn predecessor_projection_surface_digest(stage: u32, hashes: &[String]) -> String {
    prefix_local_tagged_hash("exact-predecessor-projection-surface", &(stage, hashes))
}

fn prefix_referenced_role(
    kind: &'static str,
    owner_clause: u16,
    mechanism: CreditMechanism,
    local_role: LocalRole,
    source_stage: u32,
    source: &ActLocalV3PrefixDeclaration,
    source_family: &ActLocalV3PrefixNaturalFamilyRow,
) -> RoleOccurrence {
    // The exact coordinate retains the source binding.  The natural-family
    // shape deliberately forgets which equivalent predecessor supplied it;
    // this is where the quotient, rather than a count-maximizing selector,
    // collapses duplicate transports.
    RoleOccurrence {
        kind,
        owner_clause,
        mechanism,
        local_role,
        coordinate: json!({
            "kind": kind,
            "clause": owner_clause,
            "coordinates": {
                "source_step": source_stage,
                "source_projection_hash": source.surface_hash,
                "source_family_id": source_family.semantic_family_id,
                "source_family_row_hash": source_family.derivation_hash,
            }
        }),
        natural_family_shape: json!({
            "kind": kind,
            "clause": owner_clause,
            "coordinates": {
                "typed_predecessor_family_shape": source_family.family_shape_key,
                "typed_predecessor_generator": source_family.generator_univalent_key,
            }
        }),
        uniform_specialization: false,
        removed_by_r2: false,
    }
}

/// Prefix-local role enumeration.  The historical enumerator remains
/// unchanged.  The two classes that historically used `dominant_reference`
/// are handled explicitly here: every structurally referenced, typed source
/// projection is enumerated, and source identity is erased only in the
/// natural-family quotient key above.
fn enumerate_prefix_local_role_occurrences(
    candidate: &Telescope,
    library: &Library,
    predecessor_projections: &[ActLocalV3PrefixDeclaration],
    prefix_telescopes: &[(u32, Telescope)],
) -> Result<(Vec<RoleOccurrence>, bool), ActLocalProvenanceV3Error> {
    use CreditMechanism as M;
    use LocalRole as L;

    match candidate.classify(library) {
        TelescopeClass::Axiomatic => {
            let mut rows = Vec::new();
            for (index, clause) in candidate.clauses.iter().enumerate() {
                if is_axiomatic_intro(&clause.expr) {
                    rows.push(role(
                        "axiomatic_introduction_head",
                        as_u16(index)?,
                        M::IntrinsicKernel,
                        L::KernelHead,
                        json!({}),
                    ));
                }
            }
            let refs = candidate.lib_refs().into_iter().collect::<Vec<_>>();
            for source_step in refs.iter().copied() {
                let source = predecessor_projections
                    .iter()
                    .find(|projection| projection.stage == source_step)
                    .ok_or_else(|| {
                        ActLocalProvenanceV3Error::Enumeration(format!(
                            "typed axiomatic source step {source_step} is absent from the exact projection prefix"
                        ))
                    })?;
                let owner = clause_for_ref(candidate, source_step)?;
                for family in source.natural_family_rows.iter().filter(|family| {
                    family.marginal
                        && !family.removed_by_r2
                        && family.generator_typed_normalized_natural
                }) {
                    rows.push(prefix_referenced_role(
                        "axiomatic_inherited_family",
                        owner,
                        M::P5InheritedSurface,
                        L::SupportAction,
                        source_step,
                        source,
                        family,
                    ));
                }
            }
            for index in 0..candidate.clauses.len() {
                rows.push(role(
                    "axiomatic_local_and_bridge_face",
                    as_u16(index)?,
                    M::P5LocalAndBridge,
                    L::SupportAction,
                    json!({}),
                ));
            }
            for pair in refs.windows(2) {
                rows.push(role(
                    "axiomatic_support_bridge",
                    clause_for_ref(candidate, pair[1])?,
                    M::P5LocalAndBridge,
                    L::SupportAction,
                    json!({"left_step": pair[0], "right_step": pair[1]}),
                ));
            }
            Ok((rows, true))
        }
        TelescopeClass::Synthesis => {
            let mut rows = Vec::new();
            for index in 0..candidate.clauses.len() {
                rows.push(role(
                    "synthesis_local_declaration",
                    as_u16(index)?,
                    M::IntrinsicKernel,
                    L::KernelHead,
                    json!({}),
                ));
            }
            for (index, clause) in candidate.clauses.iter().enumerate() {
                if is_polymorphic_temporal_elim(&clause.expr) {
                    for target_step in 1..=library.len() as u32 {
                        rows.push(uniform_role(
                            "synthesis_uniform_temporal_action",
                            as_u16(index)?,
                            M::P6UniformSpecialization,
                            L::SupportAction,
                            json!({"step": target_step}),
                        ));
                    }
                }
            }
            let modal_sources = candidate
                .lib_refs()
                .into_iter()
                .filter(|step| {
                    library
                        .get(step.saturating_sub(1) as usize)
                        .is_some_and(|entry| entry.capabilities.has_modal_ops)
                })
                .collect::<Vec<_>>();
            for source_step in modal_sources {
                let source = predecessor_projections
                    .iter()
                    .find(|projection| projection.stage == source_step)
                    .ok_or_else(|| {
                        ActLocalProvenanceV3Error::Enumeration(format!(
                            "typed modal source step {source_step} is absent from the exact projection prefix"
                        ))
                    })?;
                for (index, clause) in candidate.clauses.iter().enumerate() {
                    if !is_distributive_law(&clause.expr) {
                        continue;
                    }
                    for family in source.natural_family_rows.iter().filter(|family| {
                        family.marginal
                            && !family.removed_by_r2
                            && family.generator_typed_normalized_natural
                    }) {
                        rows.push(prefix_referenced_role(
                            "synthesis_distributive_transport",
                            as_u16(index)?,
                            M::DistributiveInheritance,
                            L::Coherence,
                            source_step,
                            source,
                            family,
                        ));
                    }
                }
            }
            if let Some(owner) = candidate
                .clauses
                .iter()
                .position(|clause| is_spatial_temporal_clause(&clause.expr))
            {
                let owner = as_u16(owner)?;
                for (source_step, source_telescope) in prefix_telescopes {
                    for (path_index, path_clause) in source_telescope.clauses.iter().enumerate() {
                        let Expr::PathCon(dimension) = path_clause.expr else {
                            continue;
                        };
                        for left_axis in 0..dimension {
                            for right_axis in 0..dimension {
                                rows.push(role(
                                    "synthesis_infinitesimal_shift",
                                    owner,
                                    M::CombinatorialSynthesis,
                                    L::SupportAction,
                                    json!({
                                        "source_step": source_step,
                                        "source_path_clause": path_index,
                                        "left_axis": left_axis,
                                        "right_axis": right_axis,
                                    }),
                                ));
                            }
                        }
                    }
                }
            }
            Ok((rows, true))
        }
        // These branches have no predecessor-count selection in the
        // historical implementation.  Supplying an empty package slice also
        // makes a historical package/counter read impossible at the type of
        // this call; Axiomatic and Synthesis were intercepted above.
        _ => enumerate_role_occurrences(candidate, library, &[], prefix_telescopes)
            .map(|rows| (rows, true)),
    }
}

fn prefix_family_has_preimage(
    family_shape_key: &str,
    generator: &ActLocalV3GeneratorEvidence,
    predecessors: &[ActLocalV3PrefixDeclaration],
) -> Result<bool, ActLocalProvenanceV3Error> {
    let scope = generator.parameter_sorts.len() as u32;
    for family in predecessors
        .iter()
        .flat_map(|projection| projection.natural_family_rows.iter())
        .filter(|family| !family.removed_by_r2)
    {
        let comparable = family.family_shape_key == family_shape_key
            && family.generator_parameter_sorts == generator.parameter_sorts
            && family.generator_kernel_type_json == generator.kernel_type_json;
        if !comparable {
            continue;
        }
        let witness = univalent_equality(
            &family.generator_canonical_normal_form,
            &generator.canonical_normal_form,
            scope,
            256,
        )
        .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
        if witness.equal {
            return Ok(true);
        }
    }
    Ok(false)
}

fn validate_typed_r2_applications(
    roles: &[RoleOccurrence],
    token: &ActLocalV3TypedR2RuleToken,
) -> bool {
    roles
        .iter()
        .filter(|occurrence| occurrence.removed_by_r2)
        .all(|child| {
            child.kind == token.generated_child_kind
                && child.mechanism == token.slot_mechanism
                && child.local_role == token.slot_role
                && roles
                    .iter()
                    .filter(|parent| {
                        !parent.removed_by_r2
                            && parent.kind == token.surviving_parent_kind
                            && parent.owner_clause == child.owner_clause
                            && parent.mechanism == token.slot_mechanism
                            && parent.local_role == token.slot_role
                    })
                    .count()
                    == 1
        })
}

fn issue_prefix_declaration_one(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    predecessor_projections: &[ActLocalV3PrefixDeclaration],
    r2_rule: &ActLocalV3TypedR2RuleToken,
) -> Result<ActLocalV3PrefixDeclaration, ActLocalProvenanceV3Error> {
    validate_prefix(prefix, stage)?;
    if !replay_act_local_v3_typed_r2_rule_token(r2_rule).is_empty() {
        return Err(ActLocalProvenanceV3Error::Input(
            "prefix-local declaration received a non-authoritative typed R2 rule token".to_owned(),
        ));
    }
    if predecessor_projections.len() != stage.saturating_sub(1) as usize {
        return Err(ActLocalProvenanceV3Error::Input(format!(
            "Stage {stage} requires every exact predecessor projection"
        )));
    }

    let mut library: Library = Vec::new();
    for entry in prefix.entries() {
        library.push(LibraryEntry::from_telescope(&entry.telescope, &library));
    }
    // Elaboration is a gate, not a field in the projection: downstream only
    // needs the exact candidate/prefix binding and the declaration surface.
    elaborate_telescope(prefix, candidate, stage.saturating_sub(1))
        .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
    let closure = predecessor_closure(prefix)
        .map_err(|error| ActLocalProvenanceV3Error::Family(error.to_string()))?;
    let CandidateExtractionOutcome::Extracted(extraction) =
        extract_candidate_families(prefix, &closure, candidate, stage.saturating_sub(1))
    else {
        return Err(ActLocalProvenanceV3Error::Family(
            "prefix-local candidate family extraction was kernel-invalid".to_owned(),
        ));
    };
    let prefix_telescopes = prefix
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.telescope.clone()))
        .collect::<Vec<_>>();
    let (roles, all_structurally_typed_predecessors_enumerated) =
        enumerate_prefix_local_role_occurrences(
            candidate,
            &library,
            predecessor_projections,
            &prefix_telescopes,
        )?;
    let r2_rule_application_total = validate_typed_r2_applications(&roles, r2_rule);
    if !r2_rule_application_total {
        return Err(ActLocalProvenanceV3Error::Enumeration(
            "a generated R2 child lacked its unique distinct typed parent at the same local slot"
                .to_owned(),
        ));
    }
    let role_occurrences_before_quotient = roles
        .iter()
        .map(RoleOccurrence::serializable)
        .collect::<Vec<_>>();

    let mut grouped: BTreeMap<String, Vec<(RoleOccurrence, ActLocalV3GeneratorEvidence)>> =
        BTreeMap::new();
    let mut removed_by_r2 = 0usize;
    for occurrence in roles {
        if occurrence.removed_by_r2 {
            removed_by_r2 += 1;
            continue;
        }
        let family = generator_for_clause(&extraction.families, occurrence.owner_clause)
            .ok_or_else(|| {
                ActLocalProvenanceV3Error::Family(format!(
                    "prefix-local role {} at clause {} has no extracted owner family",
                    occurrence.kind, occurrence.owner_clause
                ))
            })?;
        let generator =
            generator_evidence(family, occurrence.owner_clause, &extraction.derivation_hash);
        let generator_typed_normalized_natural = generator
            .owner_clause_normalization_and_typing_replayed
            && generator.naturality_square_equal
            && generator.owner_is_generator_or_instance;
        if !generator_typed_normalized_natural {
            return Err(ActLocalProvenanceV3Error::Family(format!(
                "prefix-local role {} lacks typed normalized natural owner evidence",
                occurrence.kind
            )));
        }
        let shape_key = tagged_hash(
            "natural-family-shape",
            &(
                &occurrence.natural_family_shape,
                &generator.generator_univalent_key,
                &generator.parameter_sorts,
                &generator.kernel_type_json,
            ),
        );
        grouped
            .entry(shape_key)
            .or_default()
            .push((occurrence, generator));
    }

    let mut theorem_gaps = Vec::new();
    let mut natural_family_rows = Vec::new();
    for (shape_key, members) in grouped {
        let (representative, generator) = &members[0];
        let semantic_family_id = tagged_hash(
            "semantic-natural-family",
            &(
                &shape_key,
                &generator.generator_univalent_key,
                &generator.derivation_hash,
            ),
        );
        let marginal = !prefix_family_has_preimage(&shape_key, generator, predecessor_projections)?;
        let gap_id = if marginal {
            let theorem_gap = gap(
                stage,
                Some(&semantic_family_id),
                "ROLE_SCHEMA_EXTRACTION_GAP",
                format!(
                    "the typed-family extractor proves owner clause {} but does not construct, type, or naturalize semantic role `{}`; owner typing cannot be promoted to role-family typing",
                    representative.owner_clause, representative.kind
                ),
            );
            let id = theorem_gap.id.clone();
            theorem_gaps.push(theorem_gap);
            Some(id)
        } else {
            None
        };
        let collapsed_uniform_instance_count = members
            .iter()
            .skip(1)
            .filter(|(occurrence, _)| occurrence.uniform_specialization)
            .count();
        let mut row = ActLocalV3PrefixNaturalFamilyRow {
            semantic_family_id,
            family_shape_key: shape_key,
            representative_role: representative.serializable(),
            occurrence_count: members.len(),
            collapsed_uniform_instance_count,
            generator_univalent_key: generator.generator_univalent_key.clone(),
            generator_canonical_normal_form: generator.canonical_normal_form.clone(),
            generator_parameter_sorts: generator.parameter_sorts.clone(),
            generator_kernel_type_json: generator.kernel_type_json.clone(),
            generator_typed_normalized_natural: true,
            marginal,
            removed_by_r2: false,
            gap_id,
            derivation_hash: String::new(),
        };
        row.derivation_hash = prefix_natural_family_row_hash(&row);
        natural_family_rows.push(row);
    }

    let quotient_partition_exact = role_occurrences_before_quotient.len()
        == removed_by_r2
            + natural_family_rows
                .iter()
                .map(|row| row.occurrence_count)
                .sum::<usize>();
    let uniform_specializations_not_multiplied = natural_family_rows.iter().all(|row| {
        row.occurrence_count == 1
            || row.collapsed_uniform_instance_count <= row.occurrence_count.saturating_sub(1)
    });
    let capability_audit = ActLocalV3PrefixCapabilityAudit::source_first();
    if !quotient_partition_exact
        || !uniform_specializations_not_multiplied
        || !all_structurally_typed_predecessors_enumerated
        || !capability_audit.source_first_only()
    {
        return Err(ActLocalProvenanceV3Error::Enumeration(
            "prefix-local declaration projection did not close its source-first quotient"
                .to_owned(),
        ));
    }

    let predecessor_projection_hashes = predecessor_projections
        .iter()
        .map(|projection| projection.surface_hash.clone())
        .collect::<Vec<_>>();
    let predecessor_surface_digest =
        predecessor_projection_surface_digest(stage, &predecessor_projection_hashes);
    let mut surface = ActLocalV3PrefixDeclaration {
        schema: ACT_LOCAL_PREFIX_DECLARATION_V3_SCHEMA.to_owned(),
        date: ACT_LOCAL_PROVENANCE_V3_DATE.to_owned(),
        theorem_id: ACT_LOCAL_PREFIX_DECLARATION_V3_THEOREM_ID.to_owned(),
        stage,
        candidate_hash: candidate_hash(candidate),
        predecessor_signature_digest: prefix.digest().to_owned(),
        kappa: candidate.kappa() as u32,
        r2_rule_authority_hash: r2_rule.authority_hash().to_owned(),
        predecessor_projection_hashes,
        predecessor_surface_digest,
        role_occurrences_before_quotient,
        natural_family_rows,
        theorem_gaps,
        r2_rule_application_total,
        quotient_partition_exact,
        uniform_specializations_not_multiplied,
        all_structurally_typed_predecessors_enumerated,
        capability_audit,
        surface_hash: String::new(),
    };
    surface.surface_hash = prefix_declaration_surface_hash(&surface);
    Ok(surface)
}

fn issue_prefix_declaration_sequence_checked_token(
    entries: &[(u32, Telescope)],
    r2_rule: &ActLocalV3TypedR2RuleToken,
) -> Result<Vec<ActLocalV3PrefixDeclaration>, ActLocalProvenanceV3Error> {
    if entries.is_empty()
        || !entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=entries.len() as u32)
    {
        return Err(ActLocalProvenanceV3Error::Input(
            "v3 prefix-local declaration sequence must be contiguous from Stage 1".to_owned(),
        ));
    }
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut projections = Vec::new();
    for (stage, candidate) in entries {
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let projection =
            issue_prefix_declaration_one(&prefix, *stage, candidate, &projections, r2_rule)?;
        accepted.push((*stage, candidate.clone()));
        projections.push(projection);
    }
    Ok(projections)
}

pub fn issue_act_local_prefix_declaration_sequence_v3(
    entries: &[(u32, Telescope)],
    r2_rule: &ActLocalV3TypedR2RuleToken,
) -> Result<Vec<ActLocalV3PrefixDeclaration>, ActLocalProvenanceV3Error> {
    if !replay_act_local_v3_typed_r2_rule_token(r2_rule).is_empty() {
        return Err(ActLocalProvenanceV3Error::Input(
            "v3 prefix-local sequence requires the closed typed R2 rule token".to_owned(),
        ));
    }
    issue_prefix_declaration_sequence_checked_token(entries, r2_rule)
}

pub fn issue_act_local_prefix_declaration_v3(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    predecessor_projections: &[ActLocalV3PrefixDeclaration],
    r2_rule: &ActLocalV3TypedR2RuleToken,
) -> Result<ActLocalV3PrefixDeclaration, ActLocalProvenanceV3Error> {
    validate_prefix(prefix, stage)?;
    let prefix_entries = prefix
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.telescope.clone()))
        .collect::<Vec<_>>();
    let expected_predecessors = if prefix_entries.is_empty() {
        Vec::new()
    } else {
        issue_act_local_prefix_declaration_sequence_v3(&prefix_entries, r2_rule)?
    };
    if expected_predecessors != predecessor_projections {
        return Err(ActLocalProvenanceV3Error::Input(
            "supplied predecessor declaration projections differ from source-first reissuance"
                .to_owned(),
        ));
    }
    issue_prefix_declaration_one(prefix, stage, candidate, predecessor_projections, r2_rule)
}

pub fn replay_act_local_prefix_declaration_v3(
    prefix: &SealedSignature,
    candidate: &Telescope,
    predecessor_projections: &[ActLocalV3PrefixDeclaration],
    r2_rule: &ActLocalV3TypedR2RuleToken,
    claimed: &ActLocalV3PrefixDeclaration,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.surface_hash != prefix_declaration_surface_hash(claimed) {
        errors.push("v3 prefix-local declaration surface digest mismatch".to_owned());
    }
    match issue_act_local_prefix_declaration_v3(
        prefix,
        claimed.stage,
        candidate,
        predecessor_projections,
        r2_rule,
    ) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors
            .push("v3 prefix-local declaration differs from source-first reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

pub fn replay_act_local_prefix_declaration_sequence_v3(
    entries: &[(u32, Telescope)],
    r2_rule: &ActLocalV3TypedR2RuleToken,
    claimed: &[ActLocalV3PrefixDeclaration],
) -> Vec<String> {
    match issue_act_local_prefix_declaration_sequence_v3(entries, r2_rule) {
        Ok(expected) if expected == claimed => Vec::new(),
        Ok(_) => vec![
            "v3 prefix-local declaration sequence differs from source-first reissuance".to_owned(),
        ],
        Err(error) => vec![error.to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v3_sweeps_full_a3_and_never_mints_output_positions() {
        let entries = (1..=15)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let packages = issue_act_local_sequence_v3(&entries).expect("v3 sequence");
        assert_eq!(packages.len(), 15);
        assert!(packages.iter().all(|package| {
            package.full_pre_candidate_a3_surface_replayed
                && package.a3_orbit_count == package.a3_orbits.len()
                && package.candidate_minted_output_position_count == 0
                && package.forbidden_inputs.none()
                && package.uniform_specializations_not_multiplied
        }));
        assert!(packages.iter().all(|package| !package.authoritative));
        assert!(packages.iter().any(|package| {
            package
                .theorem_gaps
                .iter()
                .any(|gap| gap.kind == "ROLE_SCHEMA_EXTRACTION_GAP")
        }));
    }

    #[test]
    fn replay_rejects_a_self_resigned_output_mint() {
        let entries = (1..=5)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let packages = issue_act_local_sequence_v3(&entries).expect("v3 sequence");
        let prefix = SealedSignature::from_telescopes(entries[..4].to_vec());
        let mut forged = packages[4].clone();
        forged.candidate_minted_output_position_count = 1;
        forged.derivation_hash = certificate_digest(&forged);
        assert!(!replay_act_local_provenance_v3(&prefix, &entries[4].1, &forged).is_empty());
    }

    #[test]
    fn prefix_local_r2_authority_is_closed_typed_and_reissued() {
        let token = issue_act_local_v3_typed_r2_rule_token();
        assert!(replay_act_local_v3_typed_r2_rule_token(&token).is_empty());
        assert!(!token.markdown_or_file_read);
        assert!(!token.archived_count_or_label_read);
        assert!(token.e4_generator_membership_witness_required);
        assert!(!token.operational_membership_verdict_issued);
        assert_eq!(
            token.slot_mechanism,
            CreditMechanism::P6UniformSpecialization
        );
        assert_eq!(token.slot_role, LocalRole::SupportAction);
        assert_eq!(token.authority_hash(), token.derivation_hash);

        let mut forged = token.clone();
        forged.surviving_parent_kind = "desired-count-selected-parent".to_owned();
        forged.derivation_hash = typed_r2_rule_token_hash(&forged);
        assert!(!replay_act_local_v3_typed_r2_rule_token(&forged).is_empty());
    }

    #[test]
    fn prefix_local_sequence_is_deterministic_projection_only_and_r2_total() {
        let entries = (1..=8)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let r2 = issue_act_local_v3_typed_r2_rule_token();
        let first = issue_act_local_prefix_declaration_sequence_v3(&entries, &r2)
            .expect("first source-first declaration sequence");
        let second = issue_act_local_prefix_declaration_sequence_v3(&entries, &r2)
            .expect("second source-first declaration sequence");
        assert_eq!(first, second);
        assert!(replay_act_local_prefix_declaration_sequence_v3(&entries, &r2, &first).is_empty());
        assert!(first.iter().all(|surface| {
            surface.capability_audit.source_first_only()
                && surface.r2_rule_application_total
                && surface.quotient_partition_exact
                && surface.uniform_specializations_not_multiplied
                && surface.all_structurally_typed_predecessors_enumerated
                && surface.r2_rule_authority_hash == r2.derivation_hash
                && surface.predecessor_projection_hashes
                    == first[..surface.stage.saturating_sub(1) as usize]
                        .iter()
                        .map(|prior| prior.surface_hash.clone())
                        .collect::<Vec<_>>()
        }));
        let stage8 = &first[7];
        assert!(
            stage8
                .role_occurrences_before_quotient
                .iter()
                .any(|occurrence| {
                    occurrence.kind == r2.generated_child_kind && occurrence.removed_by_adopted_r2
                })
        );

        let json = serde_json::to_string(&first).expect("projection sequence serializes");
        for forbidden_field in [
            "predecessor_act_local_digest",
            "blind_local_role_capacity_4kappa",
            "adopted_export_upper_bound_4kappa_plus_exported_orbits",
            "counterfactual_all_quotient_orbits_upper_bound",
            "invalid_raw_occurrence_upper_bound",
            "natural_family_count_after_quotient",
        ] {
            assert!(!json.contains(forbidden_field), "leaked {forbidden_field}");
        }
    }

    #[test]
    fn prefix_local_enumerates_every_typed_structural_source_without_count_selection() {
        let entries = (1..=14)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let r2 = issue_act_local_v3_typed_r2_rule_token();
        let surfaces = issue_act_local_prefix_declaration_sequence_v3(&entries, &r2)
            .expect("source-first declaration sequence through Stage 14");
        let stage14 = &surfaces[13];
        assert!(
            !stage14
                .capability_audit
                .count_based_predecessor_selector_capability
        );
        assert!(
            !stage14
                .capability_audit
                .markdown_or_file_source_scan_capability
        );

        let refs = entries[13].1.lib_refs();
        let expected = refs
            .iter()
            .flat_map(|source_step| {
                surfaces[*source_step as usize - 1]
                    .natural_family_rows
                    .iter()
                    .filter(|family| {
                        family.marginal
                            && !family.removed_by_r2
                            && family.generator_typed_normalized_natural
                    })
                    .map(move |family| (*source_step, family.derivation_hash.clone()))
            })
            .collect::<BTreeSet<_>>();
        let observed = stage14
            .role_occurrences_before_quotient
            .iter()
            .filter(|occurrence| occurrence.kind == "axiomatic_inherited_family")
            .filter_map(|occurrence| {
                let coordinate = occurrence.coordinate.get("coordinates")?;
                let source_step = u32::try_from(coordinate.get("source_step")?.as_u64()?).ok()?;
                let row_hash = coordinate
                    .get("source_family_row_hash")?
                    .as_str()?
                    .to_owned();
                Some((source_step, row_hash))
            })
            .collect::<BTreeSet<_>>();
        assert!(!expected.is_empty());
        assert_eq!(observed, expected);
    }

    #[test]
    fn prefix_local_replay_rejects_fully_rehashed_surface_and_predecessor_mutations() {
        let entries = (1..=4)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let r2 = issue_act_local_v3_typed_r2_rule_token();
        let surfaces = issue_act_local_prefix_declaration_sequence_v3(&entries, &r2)
            .expect("source-first declaration prefix");
        let prefix = SealedSignature::from_telescopes(entries[..3].to_vec());

        let mut forged_surface = surfaces[3].clone();
        forged_surface.kappa = forged_surface.kappa.saturating_add(1);
        forged_surface.surface_hash = prefix_declaration_surface_hash(&forged_surface);
        assert!(
            !replay_act_local_prefix_declaration_v3(
                &prefix,
                &entries[3].1,
                &surfaces[..3],
                &r2,
                &forged_surface,
            )
            .is_empty()
        );

        let mut forged_predecessors = surfaces[..3].to_vec();
        forged_predecessors[1].kappa = forged_predecessors[1].kappa.saturating_add(1);
        forged_predecessors[1].surface_hash =
            prefix_declaration_surface_hash(&forged_predecessors[1]);
        assert!(
            issue_act_local_prefix_declaration_v3(
                &prefix,
                4,
                &entries[3].1,
                &forged_predecessors,
                &r2,
            )
            .is_err()
        );
    }

    #[test]
    fn additive_prefix_path_does_not_change_historical_v3_reissuance() {
        let entries = (1..=4)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let before = issue_act_local_sequence_v3(&entries).expect("historical v3 before");
        let r2 = issue_act_local_v3_typed_r2_rule_token();
        let _ = issue_act_local_prefix_declaration_sequence_v3(&entries, &r2)
            .expect("additive prefix-local path");
        let after = issue_act_local_sequence_v3(&entries).expect("historical v3 after");
        assert_eq!(before, after);
    }
}
