//! Operational candidate-level quotient for the frozen Step-16 raw product.
//!
//! This module closes the raw-product/operational-verdict join that the first
//! IP-1 burn deliberately left open.  It does not materialize telescopes.
//! Each position catalog is streamed once and quotiented by the observations
//! used by:
//!
//! * the priority [`TelescopeClass`] classifier;
//! * an explicitly named clause-local proxy for the extraction partition;
//! * conditional EGP bounds that apply only if full extraction succeeds; and
//! * the optional H, P5, and synthesis amplification gates.
//!
//! Whole-telescope fuel/type dependence is not proved compositional here, so
//! this is not an exact intended typed-family classification.  The certificate
//! exposes that residual and keeps the semantic theorem false.
//!
//! H and synthesis remain named fail-closed routes: the public kernel tokens
//! do not establish the stronger sidecar capabilities.  P5 is decided as far
//! as the frozen raw grammar permits.  If an application argument can match a
//! transportable exported domain, this counter emits an explicit residual
//! instead of inferring a whole-candidate token result from a class label.

use super::A5Adjudication;
use crate::enumerate::{
    EnumerationContext, LateFamilySurface, assess_raw_surface_membership, enumerate_exprs,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::certified_novelty::ReplayedP5LiftCapability;
use pen_eval::demand_orbits::kernel_stage_inventories;
use pen_eval::typed_families::{
    ClauseClosureDisposition, PredecessorClosure, clause_presentation, closure_clause_disposition,
    predecessor_closure,
};
use pen_type::elaborate::{
    ElabError, SealedSignature, elaborate_single_clause, issue_typed_lift_token,
    minimal_ambient_parameters, required_clause_ambient,
};
use pen_type::normalize::normalize;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const IP1_JOIN_SCHEMA_VERSION: u32 = 3;
pub const IP1_JOIN_DATE: &str = "2026-07-19";
const A5_CLAUSE: &str = "On a debt-free field, acceptance requires a certified clearing; unrankable candidates cannot be accepted.";
const A5_ADOPTION_SOURCE: &str =
    "Explicit user adoption in this plan: Yes I adopt A5 as an axiom in the context of this plan.";
const STEP16_VISIBLE_LIBRARY: u32 = 15;
const STEP16_BASE_AMBIENT: u32 = 2;
const MAX_AMBIENT: u32 = 2;
const FULL_MAX_EXPR_NODES: u8 = 6;
const FULL_MIN_KAPPA: u16 = 2;
const FULL_MAX_KAPPA: u16 = 4;

fn position_context(position: u32, max_expr_nodes: u8) -> EnumerationContext {
    EnumerationContext {
        library_size: STEP16_VISIBLE_LIBRARY,
        scope_size: STEP16_BASE_AMBIENT + position,
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

#[cfg(test)]
fn primary_role(expr: &Expr) -> ClauseRole {
    match expr {
        Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
            ClauseRole::Formation
        }
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ) => ClauseRole::Formation,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)) => {
            ClauseRole::Elimination
        }
        Expr::PathCon(_) => ClauseRole::PathAttach,
        Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
        Expr::App(_, _) => ClauseRole::Introduction,
        Expr::Susp(_)
        | Expr::Trunc(_)
        | Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Bang(_)
        | Expr::WhyNot(_)
        | Expr::Lib(_) => ClauseRole::Formation,
    }
}

fn is_basic_formation_entry(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Var(_))
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn is_former_root(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Lam(_) | Expr::App(_, _)
    )
}

fn top_modal_kind(expr: &Expr) -> u8 {
    match expr {
        Expr::Flat(_) => 1,
        Expr::Sharp(_) => 2,
        Expr::Disc(_) => 4,
        Expr::Shape(_) => 8,
        _ => 0,
    }
}

fn is_polymorphic_temporal_site(expr: &Expr) -> bool {
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

fn is_spatial_temporal_site(expr: &Expr) -> bool {
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
    u8::from(is_polymorphic_temporal_site(expr)) + u8::from(is_spatial_temporal_site(expr))
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

fn vars_are_binder_internal(expr: &Expr, source_scope: u32) -> bool {
    match expr {
        Expr::Var(level) => *level > source_scope,
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            vars_are_binder_internal(a, source_scope) && vars_are_binder_internal(b, source_scope)
        }
        Expr::Id(a, b, c) => {
            vars_are_binder_internal(a, source_scope)
                && vars_are_binder_internal(b, source_scope)
                && vars_are_binder_internal(c, source_scope)
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
        | Expr::WhyNot(inner) => vars_are_binder_internal(inner, source_scope),
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => true,
    }
}

/// Transportable exported P5 domains for the only two imports admitted by
/// the raw Step-16 grammar.  On the frozen signature these are L11/L12/L13
/// for L14 and the empty set for L15.
fn transportable_exported_domains(signature: &SealedSignature, import: u32) -> Vec<Expr> {
    let Some(entry) = signature.entry(import) else {
        return Vec::new();
    };
    let ambient = minimal_ambient_parameters(&entry.telescope);
    entry
        .formation_clauses
        .iter()
        .filter_map(|index| {
            let clause = &entry.telescope.clauses[usize::from(*index)];
            let Expr::Pi(domain, _) = &clause.expr else {
                return None;
            };
            let source_scope = ambient + u32::from(*index);
            vars_are_binder_internal(domain, source_scope).then(|| (**domain).clone())
        })
        .collect()
}

fn normalized_argument_matches(argument: &Expr, scope: u32, exported_domains: &[Expr]) -> bool {
    let Ok(argument_nf) = normalize(argument, scope, 256) else {
        // A normalization failure is an issuer failure, never a possible
        // successful application.
        return false;
    };
    exported_domains.iter().any(|domain| {
        normalize(domain, 0, 256).is_ok_and(|domain_nf| argument_nf.expr == domain_nf.expr)
    })
}

fn has_potential_typed_application(
    expr: &Expr,
    dominant: u32,
    clause_scope: u32,
    exported_domains: &[Expr],
) -> bool {
    fn walk(
        expr: &Expr,
        dominant: u32,
        scope: u32,
        binders: u32,
        exported_domains: &[Expr],
    ) -> bool {
        match expr {
            Expr::App(function, argument) => {
                (matches!(function.as_ref(), Expr::Lib(step) if *step == dominant)
                    && normalized_argument_matches(argument, scope + binders, exported_domains))
                    || walk(function, dominant, scope, binders, exported_domains)
                    || walk(argument, dominant, scope, binders, exported_domains)
            }
            Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
                walk(domain, dominant, scope, binders, exported_domains)
                    || walk(codomain, dominant, scope, binders + 1, exported_domains)
            }
            Expr::Lam(body) => walk(body, dominant, scope, binders + 1, exported_domains),
            Expr::Id(ty, left, right) => {
                walk(ty, dominant, scope, binders, exported_domains)
                    || walk(left, dominant, scope, binders, exported_domains)
                    || walk(right, dominant, scope, binders, exported_domains)
            }
            Expr::Refl(inner)
            | Expr::Susp(inner)
            | Expr::Trunc(inner)
            | Expr::Flat(inner)
            | Expr::Sharp(inner)
            | Expr::Disc(inner)
            | Expr::Shape(inner)
            | Expr::Next(inner)
            | Expr::Eventually(inner)
            | Expr::Bang(inner)
            | Expr::WhyNot(inner) => walk(inner, dominant, scope, binders, exported_domains),
            Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
        }
    }
    walk(expr, dominant, clause_scope, 0, exported_domains)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct AmbientFacts {
    ambient: u32,
    used_fields: Vec<u16>,
    marginal_by_assignment: Vec<bool>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExprJoinKey {
    formation: bool,
    shortfall: u32,
    per_ambient: Vec<AmbientFacts>,
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

fn classify_position_exprs(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    position: u32,
    max_expr_nodes: u8,
    domains14: &[Expr],
    domains15: &[Expr],
) -> (u64, BTreeMap<ExprJoinKey, u128>) {
    let exprs = enumerate_exprs(position_context(position, max_expr_nodes));
    let width = exprs.len() as u64;
    let mut classes = BTreeMap::new();
    for expr in &exprs {
        let shortfall = required_clause_ambient(expr, position);
        let formation_probe = elaborate_single_clause(
            expr,
            MAX_AMBIENT,
            &vec![ClauseRole::Formation; position as usize],
            STEP16_VISIBLE_LIBRARY,
        );
        let (formation, invalid_named, unclassified) = match formation_probe {
            Ok(elaborated) => (
                elaborated.kernel_role == ClauseRole::Formation,
                false,
                false,
            ),
            Err(ElabError::BareUnivArgument) => (false, true, false),
            Err(_) => (false, false, true),
        };
        let mut per_ambient = Vec::new();
        if !invalid_named && !unclassified {
            for ambient in shortfall..=MAX_AMBIENT {
                let Ok(elaborated) = elaborate_single_clause(
                    expr,
                    ambient,
                    &vec![ClauseRole::Formation; position as usize],
                    STEP16_VISIBLE_LIBRARY,
                ) else {
                    continue;
                };
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
                per_ambient.push(AmbientFacts {
                    ambient,
                    used_fields,
                    marginal_by_assignment,
                });
            }
        }
        let support_mask = lib_mask(expr);
        let app14 = contains_direct_application(expr, 14);
        let app15 = contains_direct_application(expr, 15);
        let clause_scope = MAX_AMBIENT + position;
        let key = ExprJoinKey {
            formation,
            shortfall,
            per_ambient,
            invalid_named,
            unclassified,
            basic_formation: is_basic_formation_entry(expr),
            top_path: matches!(expr, Expr::PathCon(_)),
            top_modal: expr.is_modal(),
            temporal_like: expr.is_temporal_like(),
            top_suspension: matches!(expr, Expr::Susp(_)),
            has_lib: support_mask != 0,
            former_root: is_former_root(expr),
            support_mask,
            modal_kind: top_modal_kind(expr),
            path_basis: match expr {
                Expr::PathCon(dimension) => {
                    u8::try_from(1 + dimension.saturating_mul(*dimension)).unwrap_or(u8::MAX)
                }
                _ => 0,
            },
            synthesis_sites: synthesis_site_count(expr),
            app14,
            app15,
            potential_typed_app14: app14
                && has_potential_typed_application(expr, 14, clause_scope, domains14),
            potential_typed_app15: app15
                && has_potential_typed_application(expr, 15, clause_scope, domains15),
        };
        *classes.entry(key).or_insert(0u128) += 1;
    }
    (width, classes)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DpState {
    formation_bits: u8,
    max_shortfall: u32,
    any_marginal: bool,
    any_invalid: bool,
    any_unclassified: bool,
    all_basic: bool,
    any_path: bool,
    any_modal: bool,
    any_temporal_like: bool,
    any_suspension: bool,
    first_two_have_lib: bool,
    any_lib: bool,
    all_former_root: bool,
    support_mask: u8,
    modal_kind_mask: u8,
    path_basis: u8,
    synthesis_sites: u8,
    app14: bool,
    app15: bool,
    potential_typed_app14: bool,
    potential_typed_app15: bool,
}

impl DpState {
    fn initial() -> Self {
        Self {
            formation_bits: 0,
            max_shortfall: 0,
            any_marginal: false,
            any_invalid: false,
            any_unclassified: false,
            all_basic: true,
            any_path: false,
            any_modal: false,
            any_temporal_like: false,
            any_suspension: false,
            first_two_have_lib: true,
            any_lib: false,
            all_former_root: true,
            support_mask: 0,
            modal_kind_mask: 0,
            path_basis: 0,
            synthesis_sites: 0,
            app14: false,
            app15: false,
            potential_typed_app14: false,
            potential_typed_app15: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClauseLocalExtractionProxy {
    Internal,
    EgpMarginal,
    KernelInvalidBareUniv,
    KernelUnclassified,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AmplificationRoute {
    NoAmplificationRequired,
    HCapabilityUnavailable,
    SynthesisNoSites,
    SynthesisCapabilityUnavailable,
    P5NoUniqueDominantImport,
    P5NoDominantApplications,
    P5LiftNotTypedAgainstExportedFormation,
    P5IssuerAuditResidual,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalA5Verdict {
    KernelInvalid,
    UnrankableTypedFamilyResidual,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CandidateJoinRow {
    pub kappa: u16,
    pub class: TelescopeClass,
    /// Result of the finite single-clause probe.  This is not the full
    /// candidate extractor's verdict while whole-telescope elaboration and
    /// fuel compositionality remain unproved.
    pub clause_local_extraction_proxy: ClauseLocalExtractionProxy,
    pub direct_support: u8,
    pub amplification_route: AmplificationRoute,
    /// A full-candidate EGP bridge verdict.  It remains false in this
    /// certificate because the quotient does not prove that the clause-local
    /// probe equals whole-telescope extraction.
    pub implemented_egp_bridge_returns_bound: bool,
    /// False until parameter-sort-preserving instance matching and a full
    /// naturality basis are proved for the intended typed semantics.
    pub intended_typed_egp_proved: bool,
    /// No full-candidate exact score is claimed at this boundary.
    pub implemented_exact_egp_nu: Option<u32>,
    /// No full-candidate implemented bound is claimed at this boundary.
    pub implemented_egp_nu_upper_bound: Option<u32>,
    /// Conditional theorem only: if the full EGP bridge succeeds, extraction
    /// has created at most one credited generator family per clause.
    pub conditional_on_successful_full_egp_bridge_nu_upper_bound: Option<u32>,
    /// Intentionally absent while the typed EGP residual above is open.
    pub intended_semantic_nu_upper_bound: Option<u32>,
    /// No full-candidate implemented route bound is claimed here.
    pub implemented_route_nu_upper_bound: Option<u32>,
    /// Informational clause-local/class bound, conditional on successful
    /// whole-telescope extraction.  It is never used as a certified score.
    pub conditional_clause_local_route_nu_upper_bound: Option<u32>,
    pub operational_a5_verdict: OperationalA5Verdict,
    #[serde(serialize_with = "u128_string", deserialize_with = "u128_from_string")]
    pub count: u128,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JoinStratum {
    pub kappa: u16,
    pub per_position_widths: Vec<u64>,
    #[serde(serialize_with = "u128_string", deserialize_with = "u128_from_string")]
    pub raw_total: u128,
    #[serde(serialize_with = "u128_string", deserialize_with = "u128_from_string")]
    pub joined_total: u128,
    pub rows: Vec<CandidateJoinRow>,
    pub raw_sum_exact: bool,
    pub no_unclassified_clause_local_probe_outcome: bool,
    pub no_p5_issuer_residual: bool,
    pub maximum_conditional_clause_local_route_nu_upper_bound: Option<u32>,
    pub first_clearing_integer_nu: u32,
    pub conditional_maximum_strictly_below_clearing: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinCompleteness {
    pub raw_expression_catalog_exact: bool,
    pub priority_telescope_classifier_joined: bool,
    pub full_candidate_extraction_join_proved: bool,
    pub intended_typed_family_extraction_join_proved: bool,
    pub whole_telescope_fuel_compositionality_proved: bool,
    pub conditional_egp_generator_anchor_injection_generic: bool,
    pub typed_instance_sort_preservation_proved: bool,
    pub complete_naturality_basis_proved: bool,
    pub exact_family_count_distribution_claimed: bool,
    pub h_amplification_fail_closed: bool,
    pub synthesis_amplification_fail_closed: bool,
    pub p5_raw_issuer_decided: bool,
    /// Every raw tuple is assigned a row in the finite operational quotient.
    /// This does not assert that the quotient equals intended typed-family
    /// extraction.
    pub every_raw_candidate_has_verdict: bool,
    /// The raw-sum join and the rowwise adopted-A5 disjunction were checked.
    pub operational_a5_verdict_join_proved: bool,
    /// Every positive-count row is either below clearing under its implemented
    /// bound or is rejected by adopted A5 as kernel-invalid/unrankable.
    pub every_raw_candidate_operationally_nonclearing: bool,
    /// Operational conclusion only; it is not an exact semantic-family halt
    /// theorem while the typed extraction residuals below remain open.
    pub no_certifiably_clearing_step16_under_adopted_a5: bool,
    /// Stronger intended-semantics claim; remains false at this boundary.
    pub intended_semantic_candidate_join_proved: bool,
    pub small_cap_bisimulation_tested: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OptionalCapabilityBoundary {
    pub public_nonvacuous_p5_lift_issuer_exists: bool,
    pub public_p5_lift_replay_adapter_exists: bool,
    pub public_p5_record_internality_capability_exists: bool,
    pub full_p5_amplification_publicly_constructible: bool,
    pub h_sidecar_capability_publicly_constructible: bool,
    pub synthesis_sidecar_capability_publicly_constructible: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicP5LiftRouteWitness {
    pub candidate: Telescope,
    pub class: TelescopeClass,
    pub raw_step16_surface_member: bool,
    pub raw_step16_rejections: Vec<String>,
    pub token_issued: bool,
    pub token_dominant_import: Option<u32>,
    pub token_lift_clauses: Vec<u16>,
    pub token_derivation_hash: Option<String>,
    pub lift_replay_adapter_succeeded: bool,
    pub record_internality_capability_available: bool,
    pub full_p5_amplification_constructible: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationalA5Premise {
    pub adjudication: A5Adjudication,
    pub clause: String,
    pub adjudication_source: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ip1CandidateJoinCertificate {
    pub schema_version: u32,
    pub date: String,
    pub signature_digest: String,
    pub closure_digest: String,
    pub orbit_derivation_hash: String,
    pub max_expr_nodes: u8,
    pub min_kappa: u16,
    pub max_kappa: u16,
    pub a5: OperationalA5Premise,
    pub p5_transportable_domains_l14: Vec<Expr>,
    pub p5_transportable_domains_l15: Vec<Expr>,
    pub optional_capabilities: OptionalCapabilityBoundary,
    pub public_p5_lift_route_witness: PublicP5LiftRouteWitness,
    pub strata: Vec<JoinStratum>,
    pub completeness: CandidateJoinCompleteness,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinReplay {
    pub valid: bool,
    pub every_raw_candidate_has_verdict: bool,
    pub operational_a5_verdict_join_proved: bool,
    pub every_raw_candidate_operationally_nonclearing: bool,
    pub no_certifiably_clearing_step16_under_adopted_a5: bool,
    pub intended_semantic_candidate_join_proved: bool,
    pub errors: Vec<String>,
}

fn u128_string<S: serde::Serializer>(value: &u128, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}

fn u128_from_string<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u128, D::Error> {
    let encoded = String::deserialize(deserializer)?;
    encoded.parse().map_err(serde::de::Error::custom)
}

fn classify_state(kappa: u16, state: &DpState) -> TelescopeClass {
    if state.all_basic && !state.any_lib {
        TelescopeClass::Foundation
    } else if state.any_path {
        TelescopeClass::Hit
    } else if state.any_modal && !state.any_temporal_like {
        TelescopeClass::Modal
    } else if state.any_temporal_like {
        TelescopeClass::Synthesis
    } else if state.any_suspension {
        TelescopeClass::Suspension
    } else if (2..=4).contains(&usize::from(kappa)) && state.first_two_have_lib {
        TelescopeClass::Map
    } else if kappa >= 3 && state.any_lib && !state.any_modal {
        TelescopeClass::Axiomatic
    } else if state.all_former_root && !state.any_lib {
        TelescopeClass::Former
    } else {
        TelescopeClass::Unknown
    }
}

fn extraction_disposition(state: &DpState) -> ClauseLocalExtractionProxy {
    if state.any_unclassified {
        ClauseLocalExtractionProxy::KernelUnclassified
    } else if state.any_invalid {
        ClauseLocalExtractionProxy::KernelInvalidBareUniv
    } else if state.any_marginal {
        ClauseLocalExtractionProxy::EgpMarginal
    } else {
        ClauseLocalExtractionProxy::Internal
    }
}

fn support_count(mask: u8) -> u8 {
    mask.count_ones() as u8
}

fn route_for(class: TelescopeClass, state: &DpState) -> AmplificationRoute {
    match class {
        TelescopeClass::Hit => AmplificationRoute::HCapabilityUnavailable,
        TelescopeClass::Synthesis if state.synthesis_sites == 0 => {
            AmplificationRoute::SynthesisNoSites
        }
        TelescopeClass::Synthesis => AmplificationRoute::SynthesisCapabilityUnavailable,
        TelescopeClass::Axiomatic => match state.support_mask {
            3 => AmplificationRoute::P5NoUniqueDominantImport,
            1 if !state.app14 => AmplificationRoute::P5NoDominantApplications,
            2 if !state.app15 => AmplificationRoute::P5NoDominantApplications,
            1 if state.potential_typed_app14 => AmplificationRoute::P5IssuerAuditResidual,
            2 if state.potential_typed_app15 => AmplificationRoute::P5IssuerAuditResidual,
            1 | 2 => AmplificationRoute::P5LiftNotTypedAgainstExportedFormation,
            _ => AmplificationRoute::P5NoUniqueDominantImport,
        },
        _ => AmplificationRoute::NoAmplificationRequired,
    }
}

fn base_local_bound(class: TelescopeClass, state: &DpState, kappa: u16) -> Option<u32> {
    let k = u32::from(kappa);
    let r = u32::from(support_count(state.support_mask));
    Some(match class {
        TelescopeClass::Foundation => k,
        TelescopeClass::Former | TelescopeClass::Unknown => 2 * k,
        TelescopeClass::Suspension => 5,
        TelescopeClass::Map => 2 * k + r * r,
        TelescopeClass::Modal => {
            let kinds = state.modal_kind_mask.count_ones();
            k + r + kinds.saturating_mul(kinds.saturating_sub(1)) / 2
        }
        TelescopeClass::Synthesis if state.synthesis_sites == 0 => k,
        // These branches require a capability that is deliberately absent.
        TelescopeClass::Hit | TelescopeClass::Axiomatic | TelescopeClass::Synthesis => {
            return None;
        }
    })
}

fn row_key(kappa: u16, state: &DpState) -> CandidateJoinRow {
    let clause_local_extraction_proxy = extraction_disposition(state);
    let clause_probe_succeeded = matches!(
        clause_local_extraction_proxy,
        ClauseLocalExtractionProxy::Internal | ClauseLocalExtractionProxy::EgpMarginal
    );
    let class = classify_state(kappa, state);
    let conditional_egp_nu_upper_bound = clause_probe_succeeded.then_some(u32::from(kappa));
    let conditional_route_nu_upper_bound = if clause_probe_succeeded {
        let egp = u32::from(kappa);
        Some(
            base_local_bound(class, state, kappa)
                .unwrap_or(egp)
                .max(egp),
        )
    } else {
        None
    };
    CandidateJoinRow {
        kappa,
        class,
        clause_local_extraction_proxy,
        direct_support: support_count(state.support_mask),
        amplification_route: route_for(class, state),
        implemented_egp_bridge_returns_bound: false,
        intended_typed_egp_proved: false,
        implemented_exact_egp_nu: None,
        implemented_egp_nu_upper_bound: None,
        conditional_on_successful_full_egp_bridge_nu_upper_bound: conditional_egp_nu_upper_bound,
        intended_semantic_nu_upper_bound: None,
        implemented_route_nu_upper_bound: None,
        conditional_clause_local_route_nu_upper_bound: conditional_route_nu_upper_bound,
        operational_a5_verdict: if clause_probe_succeeded {
            OperationalA5Verdict::UnrankableTypedFamilyResidual
        } else {
            OperationalA5Verdict::KernelInvalid
        },
        count: 0,
    }
}

fn first_clearing_integer(kappa: u16) -> u32 {
    const NUMERATOR: u64 = 354_333;
    const DENOMINATOR: u64 = 39_040;
    let numerator = NUMERATOR * u64::from(kappa);
    ((numerator + DENOMINATOR - 1) / DENOMINATOR) as u32
}

fn optional_route_is_fail_closed(
    route: AmplificationRoute,
    capabilities: &OptionalCapabilityBoundary,
) -> bool {
    match route {
        AmplificationRoute::NoAmplificationRequired | AmplificationRoute::SynthesisNoSites => true,
        AmplificationRoute::HCapabilityUnavailable => {
            !capabilities.h_sidecar_capability_publicly_constructible
        }
        AmplificationRoute::SynthesisCapabilityUnavailable => {
            !capabilities.synthesis_sidecar_capability_publicly_constructible
        }
        AmplificationRoute::P5NoUniqueDominantImport
        | AmplificationRoute::P5NoDominantApplications
        | AmplificationRoute::P5LiftNotTypedAgainstExportedFormation => {
            !capabilities.full_p5_amplification_publicly_constructible
        }
        AmplificationRoute::P5IssuerAuditResidual => false,
    }
}

/// Operational adopted-A5 disjunction for a quotient row.
///
/// This predicate deliberately does not promote the quotient's clause-local
/// extraction proxy to intended typed semantics.  A probe-success row is
/// nonclearing either because a future full-candidate certificate supplies a
/// bound below the clearing integer, or because full typed certification is
/// absent and A5 therefore marks it unrankable.  Kernel-invalid rows are
/// rejected directly.
fn row_is_operationally_nonclearing(
    row: &CandidateJoinRow,
    first_clearing: u32,
    capabilities: &OptionalCapabilityBoundary,
) -> bool {
    if row.count == 0 {
        return true;
    }
    match row.clause_local_extraction_proxy {
        ClauseLocalExtractionProxy::Internal | ClauseLocalExtractionProxy::EgpMarginal => {
            let certified_below_clearing = row.implemented_egp_bridge_returns_bound
                && row
                    .implemented_route_nu_upper_bound
                    .is_some_and(|bound| bound < first_clearing);
            let a5_unrankable = row.operational_a5_verdict
                == OperationalA5Verdict::UnrankableTypedFamilyResidual
                && !row.intended_typed_egp_proved
                && row.intended_semantic_nu_upper_bound.is_none();
            optional_route_is_fail_closed(row.amplification_route, capabilities)
                && (certified_below_clearing || a5_unrankable)
        }
        ClauseLocalExtractionProxy::KernelInvalidBareUniv => {
            row.operational_a5_verdict == OperationalA5Verdict::KernelInvalid
        }
        ClauseLocalExtractionProxy::KernelUnclassified => false,
    }
}

fn exhaust_stratum(
    kappa: u16,
    position_classes: &[(u64, BTreeMap<ExprJoinKey, u128>)],
) -> JoinStratum {
    let mut joined: BTreeMap<CandidateJoinRow, u128> = BTreeMap::new();
    for final_ambient in 0..=MAX_AMBIENT {
        let mut states = BTreeMap::from([(DpState::initial(), 1u128)]);
        for position in 0..usize::from(kappa) {
            let mut next = BTreeMap::new();
            for (state, count) in &states {
                for (class, class_count) in &position_classes[position].1 {
                    if class.shortfall > final_ambient {
                        continue;
                    }
                    let mut state = state.clone();
                    if class.formation {
                        state.formation_bits |= 1u8 << position;
                    }
                    state.max_shortfall = state.max_shortfall.max(class.shortfall);
                    state.any_invalid |= class.invalid_named;
                    state.any_unclassified |= class.unclassified;
                    if !class.invalid_named && !class.unclassified {
                        let Some(facts) = class
                            .per_ambient
                            .iter()
                            .find(|facts| facts.ambient == final_ambient)
                        else {
                            continue;
                        };
                        let mut assignment = 0usize;
                        for (bit, field) in facts.used_fields.iter().enumerate() {
                            if state.formation_bits & (1u8 << *field as u8) != 0 {
                                assignment |= 1 << bit;
                            }
                        }
                        state.any_marginal |= facts.marginal_by_assignment[assignment];
                    }
                    state.all_basic &= class.basic_formation;
                    state.any_path |= class.top_path;
                    state.any_modal |= class.top_modal;
                    state.any_temporal_like |= class.temporal_like;
                    state.any_suspension |= class.top_suspension;
                    if position < 2 {
                        state.first_two_have_lib &= class.has_lib;
                    }
                    state.any_lib |= class.has_lib;
                    state.all_former_root &= class.former_root;
                    state.support_mask |= class.support_mask;
                    state.modal_kind_mask |= class.modal_kind;
                    state.path_basis = state.path_basis.saturating_add(class.path_basis);
                    state.synthesis_sites =
                        state.synthesis_sites.saturating_add(class.synthesis_sites);
                    state.app14 |= class.app14;
                    state.app15 |= class.app15;
                    state.potential_typed_app14 |= class.potential_typed_app14;
                    state.potential_typed_app15 |= class.potential_typed_app15;
                    *next.entry(state).or_insert(0u128) += count * class_count;
                }
            }
            states = next;
        }
        for (state, count) in states {
            if state.max_shortfall != final_ambient {
                continue;
            }
            let key = row_key(kappa, &state);
            *joined.entry(key).or_insert(0) += count;
        }
    }
    let mut rows = joined
        .into_iter()
        .map(|(mut row, count)| {
            row.count = count;
            row
        })
        .collect::<Vec<_>>();
    rows.sort();
    let per_position_widths = position_classes
        .iter()
        .take(usize::from(kappa))
        .map(|(width, _)| *width)
        .collect::<Vec<_>>();
    let raw_total = per_position_widths
        .iter()
        .fold(1u128, |product, width| product * u128::from(*width));
    let joined_total = rows.iter().map(|row| row.count).sum();
    let no_unclassified_clause_local_probe_outcome = rows.iter().all(|row| {
        row.clause_local_extraction_proxy != ClauseLocalExtractionProxy::KernelUnclassified
            || row.count == 0
    });
    let no_p5_issuer_residual = rows.iter().all(|row| {
        row.amplification_route != AmplificationRoute::P5IssuerAuditResidual || row.count == 0
    });
    let maximum = rows
        .iter()
        .filter_map(|row| row.conditional_clause_local_route_nu_upper_bound)
        .max();
    let threshold = first_clearing_integer(kappa);
    JoinStratum {
        kappa,
        per_position_widths,
        raw_total,
        joined_total,
        rows,
        raw_sum_exact: raw_total == joined_total,
        no_unclassified_clause_local_probe_outcome,
        no_p5_issuer_residual,
        maximum_conditional_clause_local_route_nu_upper_bound: maximum,
        first_clearing_integer_nu: threshold,
        conditional_maximum_strictly_below_clearing: maximum.is_some_and(|value| value < threshold),
    }
}

fn certificate_digest(certificate: &Ip1CandidateJoinCertificate) -> String {
    let mut payload = certificate.clone();
    payload.digest.clear();
    format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&payload).expect("join certificate serialization"))
    )
}

fn public_p5_lift_route_witness(signature: &SealedSignature) -> PublicP5LiftRouteWitness {
    // A genuine public, non-vacuous B15 route.  Lib(13) is deliberately
    // outside the generic Step-16 raw leaf window {L14,L15}; F-IP2's literal
    // wording concerns existence in the frozen AST, while the finite join
    // separately records that no raw candidate reaches this route.
    let candidate = Telescope::new(vec![
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Lib(13))),
        ),
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
    ]);
    let raw = assess_raw_surface_membership(position_context(0, FULL_MAX_EXPR_NODES), &candidate);
    let issued = issue_typed_lift_token(signature, &candidate, STEP16_VISIBLE_LIBRARY);
    let (
        token_issued,
        token_dominant_import,
        token_lift_clauses,
        token_derivation_hash,
        lift_replay_adapter_succeeded,
    ) = match issued {
        Ok(token) => {
            let adapted = ReplayedP5LiftCapability::from_kernel_token(
                signature,
                &candidate,
                STEP16_VISIBLE_LIBRARY,
                &token,
            )
            .is_ok();
            (
                true,
                Some(token.dominant_import()),
                token.lift_clauses().to_vec(),
                Some(token.derivation_hash().to_owned()),
                adapted,
            )
        }
        Err(_) => (false, None, Vec::new(), None, false),
    };
    PublicP5LiftRouteWitness {
        class: candidate.classify(&Vec::new()),
        candidate,
        raw_step16_surface_member: raw.is_member,
        raw_step16_rejections: raw.rejection_reasons,
        token_issued,
        token_dominant_import,
        token_lift_clauses,
        token_derivation_hash,
        lift_replay_adapter_succeeded,
        record_internality_capability_available: false,
        full_p5_amplification_constructible: false,
    }
}

pub fn run_ip1_candidate_join_with_caps(
    min_kappa: u16,
    max_kappa: u16,
    max_expr_nodes: u8,
) -> Ip1CandidateJoinCertificate {
    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature).expect("typed predecessor closure");
    let orbits = kernel_stage_inventories(&signature, &closure).expect("demand orbit extraction");
    let stage16 = orbits.stage(16).expect("Stage 16 orbit inventory");
    assert!(stage16.orbits.iter().all(|orbit| !matches!(
        orbit.resolution,
        pen_eval::semantic_provenance::OrbitResolution::Live
    )));
    let domains14 = transportable_exported_domains(&signature, 14);
    let domains15 = transportable_exported_domains(&signature, 15);
    let public_p5_lift_route_witness = public_p5_lift_route_witness(&signature);
    let optional_capabilities = OptionalCapabilityBoundary {
        public_nonvacuous_p5_lift_issuer_exists: true,
        public_p5_lift_replay_adapter_exists: true,
        public_p5_record_internality_capability_exists: false,
        full_p5_amplification_publicly_constructible: false,
        h_sidecar_capability_publicly_constructible: false,
        synthesis_sidecar_capability_publicly_constructible: false,
    };
    let mut position_classes = Vec::new();
    for position in 0..u32::from(max_kappa) {
        position_classes.push(classify_position_exprs(
            &signature,
            &closure,
            position,
            max_expr_nodes,
            &domains14,
            &domains15,
        ));
    }
    let strata = (min_kappa..=max_kappa)
        .map(|kappa| exhaust_stratum(kappa, &position_classes))
        .collect::<Vec<_>>();
    let p5_raw_issuer_decided = strata.iter().all(|stratum| stratum.no_p5_issuer_residual);
    let every_raw_candidate_has_verdict = strata.iter().all(|stratum| {
        stratum.raw_sum_exact
            && stratum.no_unclassified_clause_local_probe_outcome
            && stratum.no_p5_issuer_residual
    });
    let every_raw_candidate_operationally_nonclearing = every_raw_candidate_has_verdict
        && strata.iter().all(|stratum| {
            stratum.rows.iter().all(|row| {
                row_is_operationally_nonclearing(
                    row,
                    stratum.first_clearing_integer_nu,
                    &optional_capabilities,
                )
            })
        });
    let operational_a5_verdict_join_proved = every_raw_candidate_operationally_nonclearing;
    let no_certifiably_clearing_step16_under_adopted_a5 = operational_a5_verdict_join_proved;
    let mut certificate = Ip1CandidateJoinCertificate {
        schema_version: IP1_JOIN_SCHEMA_VERSION,
        date: IP1_JOIN_DATE.to_owned(),
        signature_digest: signature.digest().to_owned(),
        closure_digest: closure.digest,
        orbit_derivation_hash: orbits.derivation_hash,
        max_expr_nodes,
        min_kappa,
        max_kappa,
        a5: OperationalA5Premise {
            adjudication: A5Adjudication::Adopted,
            clause: A5_CLAUSE.to_owned(),
            adjudication_source: A5_ADOPTION_SOURCE.to_owned(),
        },
        p5_transportable_domains_l14: domains14,
        p5_transportable_domains_l15: domains15,
        optional_capabilities,
        public_p5_lift_route_witness,
        strata,
        completeness: CandidateJoinCompleteness {
            raw_expression_catalog_exact: true,
            priority_telescope_classifier_joined: true,
            full_candidate_extraction_join_proved: false,
            intended_typed_family_extraction_join_proved: false,
            whole_telescope_fuel_compositionality_proved: false,
            conditional_egp_generator_anchor_injection_generic: true,
            typed_instance_sort_preservation_proved: false,
            complete_naturality_basis_proved: false,
            exact_family_count_distribution_claimed: false,
            h_amplification_fail_closed: true,
            synthesis_amplification_fail_closed: true,
            p5_raw_issuer_decided,
            every_raw_candidate_has_verdict,
            operational_a5_verdict_join_proved,
            every_raw_candidate_operationally_nonclearing,
            no_certifiably_clearing_step16_under_adopted_a5,
            intended_semantic_candidate_join_proved: false,
            small_cap_bisimulation_tested: true,
        },
        digest: String::new(),
    };
    certificate.digest = certificate_digest(&certificate);
    certificate
}

pub fn run_ip1_candidate_join() -> Ip1CandidateJoinCertificate {
    run_ip1_candidate_join_with_caps(FULL_MIN_KAPPA, FULL_MAX_KAPPA, FULL_MAX_EXPR_NODES)
}

pub fn ip1_candidate_join_json_pretty() -> String {
    serde_json::to_string_pretty(&run_ip1_candidate_join()).expect("join certificate serializes")
}

pub fn replay_ip1_candidate_join(certificate: &Ip1CandidateJoinCertificate) -> CandidateJoinReplay {
    let mut errors = Vec::new();
    if certificate.schema_version != IP1_JOIN_SCHEMA_VERSION {
        errors.push(format!(
            "candidate-join schema mismatch: expected {}, got {}",
            IP1_JOIN_SCHEMA_VERSION, certificate.schema_version
        ));
    }
    if certificate_digest(certificate) != certificate.digest {
        errors.push("candidate-join digest mismatch".to_owned());
    }
    let strata_structurally_joined = certificate.strata.iter().all(|stratum| {
        let joined_total = stratum.rows.iter().map(|row| row.count).sum::<u128>();
        let raw_total = stratum
            .per_position_widths
            .iter()
            .fold(1u128, |product, width| product * u128::from(*width));
        let no_unclassified = stratum.rows.iter().all(|row| {
            row.clause_local_extraction_proxy != ClauseLocalExtractionProxy::KernelUnclassified
                || row.count == 0
        });
        let no_p5_residual = stratum.rows.iter().all(|row| {
            row.amplification_route != AmplificationRoute::P5IssuerAuditResidual || row.count == 0
        });
        let maximum = stratum
            .rows
            .iter()
            .filter_map(|row| row.conditional_clause_local_route_nu_upper_bound)
            .max();
        let threshold = first_clearing_integer(stratum.kappa);
        raw_total == stratum.raw_total
            && joined_total == stratum.raw_total
            && stratum.joined_total == stratum.raw_total
            && stratum.raw_sum_exact
            && stratum.no_unclassified_clause_local_probe_outcome == no_unclassified
            && stratum.no_p5_issuer_residual == no_p5_residual
            && stratum.maximum_conditional_clause_local_route_nu_upper_bound == maximum
            && stratum.first_clearing_integer_nu == threshold
            && stratum.conditional_maximum_strictly_below_clearing
                == maximum.is_some_and(|value| value < threshold)
    });
    if !strata_structurally_joined {
        errors.push("candidate-join raw sum mismatch".to_owned());
    }
    let expected_every_raw_candidate_has_verdict = strata_structurally_joined
        && certificate.strata.iter().all(|stratum| {
            stratum.no_unclassified_clause_local_probe_outcome && stratum.no_p5_issuer_residual
        });
    if certificate.completeness.every_raw_candidate_has_verdict
        != expected_every_raw_candidate_has_verdict
    {
        errors.push("raw-candidate operational row coverage claim mismatch".to_owned());
    }
    let adopted_a5_provenance = certificate.a5.adjudication == A5Adjudication::Adopted
        && certificate.a5.clause == A5_CLAUSE
        && certificate.a5.adjudication_source == A5_ADOPTION_SOURCE;
    let expected_every_raw_candidate_operationally_nonclearing = adopted_a5_provenance
        && expected_every_raw_candidate_has_verdict
        && certificate.strata.iter().all(|stratum| {
            stratum.rows.iter().all(|row| {
                row_is_operationally_nonclearing(
                    row,
                    stratum.first_clearing_integer_nu,
                    &certificate.optional_capabilities,
                )
            })
        });
    if certificate
        .completeness
        .every_raw_candidate_operationally_nonclearing
        != expected_every_raw_candidate_operationally_nonclearing
    {
        errors.push("rowwise adopted-A5 nonclearing disjunction mismatch".to_owned());
    }
    let optional_routes_fail_closed = certificate.completeness.h_amplification_fail_closed
        && certificate.completeness.synthesis_amplification_fail_closed
        && certificate.completeness.p5_raw_issuer_decided
        && !certificate
            .optional_capabilities
            .h_sidecar_capability_publicly_constructible
        && !certificate
            .optional_capabilities
            .synthesis_sidecar_capability_publicly_constructible
        && !certificate
            .optional_capabilities
            .full_p5_amplification_publicly_constructible;
    let expected_operational_a5_join = certificate.completeness.raw_expression_catalog_exact
        && certificate
            .completeness
            .priority_telescope_classifier_joined
        && optional_routes_fail_closed
        && expected_every_raw_candidate_operationally_nonclearing;
    if certificate.completeness.operational_a5_verdict_join_proved != expected_operational_a5_join {
        errors.push("operational adopted-A5 join claim mismatch".to_owned());
    }
    if certificate
        .completeness
        .no_certifiably_clearing_step16_under_adopted_a5
        != expected_operational_a5_join
    {
        errors.push("operational Step-16 nonclearing conclusion mismatch".to_owned());
    }
    if certificate
        .completeness
        .full_candidate_extraction_join_proved
    {
        errors.push(
            "full-candidate extraction join claimed without whole-telescope exactness".to_owned(),
        );
    }
    if certificate
        .completeness
        .exact_family_count_distribution_claimed
    {
        errors.push("candidate join falsely claims an exact family-count distribution".to_owned());
    }
    if certificate
        .completeness
        .intended_semantic_candidate_join_proved
        && (!certificate
            .completeness
            .intended_typed_family_extraction_join_proved
            || !certificate
                .completeness
                .typed_instance_sort_preservation_proved
            || !certificate.completeness.complete_naturality_basis_proved
            || !certificate
                .completeness
                .whole_telescope_fuel_compositionality_proved)
    {
        errors.push(
            "intended semantic join claimed while typed extraction/naturality/fuel residuals remain"
                .to_owned(),
        );
    }
    if certificate.strata.iter().flat_map(|s| &s.rows).any(|row| {
        row.intended_typed_egp_proved
            || row.intended_semantic_nu_upper_bound.is_some()
            || row.implemented_egp_bridge_returns_bound
            || row.implemented_exact_egp_nu.is_some()
            || row.implemented_egp_nu_upper_bound.is_some()
            || row.implemented_route_nu_upper_bound.is_some()
    }) {
        errors.push(
            "row promotes a full-candidate score or EGP bridge across the extraction residual"
                .to_owned(),
        );
    }
    if certificate
        .optional_capabilities
        .full_p5_amplification_publicly_constructible
        && !certificate
            .optional_capabilities
            .public_p5_record_internality_capability_exists
    {
        errors.push("full P5 amplification claimed without record internality".to_owned());
    }
    if *certificate
        != run_ip1_candidate_join_with_caps(
            certificate.min_kappa,
            certificate.max_kappa,
            certificate.max_expr_nodes,
        )
    {
        errors.push("candidate-join payload differs from definition replay".to_owned());
    }
    CandidateJoinReplay {
        valid: errors.is_empty(),
        every_raw_candidate_has_verdict: certificate.completeness.every_raw_candidate_has_verdict,
        operational_a5_verdict_join_proved: certificate
            .completeness
            .operational_a5_verdict_join_proved,
        every_raw_candidate_operationally_nonclearing: certificate
            .completeness
            .every_raw_candidate_operationally_nonclearing,
        no_certifiably_clearing_step16_under_adopted_a5: certificate
            .completeness
            .no_certifiably_clearing_step16_under_adopted_a5,
        intended_semantic_candidate_join_proved: certificate
            .completeness
            .intended_semantic_candidate_join_proved,
        errors,
    }
}

pub fn replay_ip1_candidate_join_json(json: &str) -> CandidateJoinReplay {
    match serde_json::from_str::<Ip1CandidateJoinCertificate>(json) {
        Ok(certificate) => replay_ip1_candidate_join(&certificate),
        Err(error) => CandidateJoinReplay {
            valid: false,
            every_raw_candidate_has_verdict: false,
            operational_a5_verdict_join_proved: false,
            every_raw_candidate_operationally_nonclearing: false,
            no_certifiably_clearing_step16_under_adopted_a5: false,
            intended_semantic_candidate_join_proved: false,
            errors: vec![format!("invalid candidate-join JSON: {error}")],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_eval::egp::classify_candidate;
    use pen_eval::typed_families::{CandidateExtractionOutcome, extract_candidate_families};
    use pen_type::elaborate::{TokenError, issue_typed_lift_token};

    fn enumerate_products(
        catalogs: &[Vec<Expr>],
        position: usize,
        prefix: &mut Vec<Expr>,
        out: &mut Vec<Telescope>,
    ) {
        if position == catalogs.len() {
            out.push(Telescope::new(
                prefix
                    .iter()
                    .cloned()
                    .map(|expr| ClauseRec::new(primary_role(&expr), expr))
                    .collect(),
            ));
            return;
        }
        for expr in &catalogs[position] {
            prefix.push(expr.clone());
            enumerate_products(catalogs, position + 1, prefix, out);
            prefix.pop();
        }
    }

    fn concrete_row(
        signature: &SealedSignature,
        closure: &PredecessorClosure,
        telescope: &Telescope,
    ) -> CandidateJoinRow {
        let class = telescope.classify(&Vec::new());
        let extraction =
            extract_candidate_families(signature, closure, telescope, STEP16_VISIBLE_LIBRARY);
        let clause_local_extraction_proxy = match &extraction {
            CandidateExtractionOutcome::KernelInvalid { failure }
                if matches!(failure.error, ElabError::BareUnivArgument) =>
            {
                ClauseLocalExtractionProxy::KernelInvalidBareUniv
            }
            CandidateExtractionOutcome::KernelInvalid { .. } => {
                ClauseLocalExtractionProxy::KernelUnclassified
            }
            CandidateExtractionOutcome::Extracted(extraction)
                if extraction.marginal_family_count > 0 =>
            {
                ClauseLocalExtractionProxy::EgpMarginal
            }
            CandidateExtractionOutcome::Extracted(_) => ClauseLocalExtractionProxy::Internal,
        };
        let support = telescope.lib_refs();
        let support_mask = u8::from(support.contains(&14)) | (u8::from(support.contains(&15)) << 1);
        let sites = telescope
            .clauses
            .iter()
            .map(|clause| synthesis_site_count(&clause.expr))
            .sum();
        let route = match class {
            TelescopeClass::Hit => AmplificationRoute::HCapabilityUnavailable,
            TelescopeClass::Synthesis if sites == 0 => AmplificationRoute::SynthesisNoSites,
            TelescopeClass::Synthesis => AmplificationRoute::SynthesisCapabilityUnavailable,
            TelescopeClass::Axiomatic => {
                match issue_typed_lift_token(signature, telescope, STEP16_VISIBLE_LIBRARY) {
                    Err(TokenError::NoDominantImport { .. }) => {
                        AmplificationRoute::P5NoUniqueDominantImport
                    }
                    Err(TokenError::NoDominantApplications { .. }) => {
                        AmplificationRoute::P5NoDominantApplications
                    }
                    Err(TokenError::LiftNotTypedAgainstExportedFormation { .. }) => {
                        AmplificationRoute::P5LiftNotTypedAgainstExportedFormation
                    }
                    Ok(_) | Err(_) => AmplificationRoute::P5IssuerAuditResidual,
                }
            }
            _ => AmplificationRoute::NoAmplificationRequired,
        };
        let clause_probe_succeeded = matches!(
            clause_local_extraction_proxy,
            ClauseLocalExtractionProxy::Internal | ClauseLocalExtractionProxy::EgpMarginal
        );
        let kappa = telescope.kappa() as u16;
        let modal_mask = telescope
            .clauses
            .iter()
            .fold(0, |mask, clause| mask | top_modal_kind(&clause.expr));
        let state = DpState {
            formation_bits: 0,
            max_shortfall: minimal_ambient_parameters(telescope),
            any_marginal: clause_local_extraction_proxy == ClauseLocalExtractionProxy::EgpMarginal,
            any_invalid: clause_local_extraction_proxy
                == ClauseLocalExtractionProxy::KernelInvalidBareUniv,
            any_unclassified: clause_local_extraction_proxy
                == ClauseLocalExtractionProxy::KernelUnclassified,
            all_basic: telescope
                .clauses
                .iter()
                .all(|c| is_basic_formation_entry(&c.expr)),
            any_path: telescope
                .clauses
                .iter()
                .any(|c| matches!(c.expr, Expr::PathCon(_))),
            any_modal: telescope.clauses.iter().any(|c| c.expr.is_modal()),
            any_temporal_like: telescope.clauses.iter().any(|c| c.expr.is_temporal_like()),
            any_suspension: telescope
                .clauses
                .iter()
                .any(|c| matches!(c.expr, Expr::Susp(_))),
            first_two_have_lib: telescope
                .clauses
                .iter()
                .take(2)
                .all(|c| c.expr.has_lib_pointer()),
            any_lib: !support.is_empty(),
            all_former_root: telescope.clauses.iter().all(|c| is_former_root(&c.expr)),
            support_mask,
            modal_kind_mask: modal_mask,
            path_basis: 0,
            synthesis_sites: sites,
            app14: telescope
                .clauses
                .iter()
                .any(|c| contains_direct_application(&c.expr, 14)),
            app15: telescope
                .clauses
                .iter()
                .any(|c| contains_direct_application(&c.expr, 15)),
            potential_typed_app14: false,
            potential_typed_app15: false,
        };
        let conditional_route_bound = if clause_probe_succeeded {
            Some(
                base_local_bound(class, &state, kappa)
                    .unwrap_or(u32::from(kappa))
                    .max(u32::from(kappa)),
            )
        } else {
            None
        };
        CandidateJoinRow {
            kappa,
            class,
            clause_local_extraction_proxy,
            direct_support: support.len() as u8,
            amplification_route: route,
            implemented_egp_bridge_returns_bound: false,
            intended_typed_egp_proved: false,
            implemented_exact_egp_nu: None,
            implemented_egp_nu_upper_bound: None,
            conditional_on_successful_full_egp_bridge_nu_upper_bound: clause_probe_succeeded
                .then_some(u32::from(kappa)),
            intended_semantic_nu_upper_bound: None,
            implemented_route_nu_upper_bound: None,
            conditional_clause_local_route_nu_upper_bound: conditional_route_bound,
            operational_a5_verdict: if clause_probe_succeeded {
                OperationalA5Verdict::UnrankableTypedFamilyResidual
            } else {
                OperationalA5Verdict::KernelInvalid
            },
            count: 0,
        }
    }

    #[test]
    fn exhaustive_one_node_subcap_bisimulates_concrete_candidate_join() {
        let certificate = run_ip1_candidate_join_with_caps(2, 4, 1);
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        let orbits = kernel_stage_inventories(&signature, &closure).expect("orbits");
        for stratum in &certificate.strata {
            let catalogs = (0..stratum.kappa)
                .map(|position| enumerate_exprs(position_context(u32::from(position), 1)))
                .collect::<Vec<_>>();
            let mut telescopes = Vec::new();
            enumerate_products(&catalogs, 0, &mut Vec::new(), &mut telescopes);
            let mut concrete = BTreeMap::new();
            for telescope in &telescopes {
                let mut row = concrete_row(&signature, &closure, telescope);
                if let CandidateExtractionOutcome::Extracted(extraction) =
                    extract_candidate_families(&signature, &closure, telescope, 15)
                {
                    let egp = classify_candidate(&extraction, &orbits, 16, stratum.kappa)
                        .expect("every extracted small-cap candidate is EGP-rankable");
                    assert!(egp.bound.marginal_nu <= u32::from(stratum.kappa));
                }
                row.count = 0;
                *concrete.entry(row).or_insert(0u128) += 1;
            }
            let expected = stratum
                .rows
                .iter()
                .map(|row| {
                    let mut key = row.clone();
                    let count = key.count;
                    key.count = 0;
                    (key, count)
                })
                .collect::<BTreeMap<_, _>>();
            assert_eq!(concrete, expected, "kappa {}", stratum.kappa);
        }
    }

    #[test]
    fn exhaustive_kappa2_three_node_subcap_bisimulates_concrete_join() {
        let certificate = run_ip1_candidate_join_with_caps(2, 2, 3);
        let stratum = &certificate.strata[0];
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        let first = enumerate_exprs(position_context(0, 3));
        let second = enumerate_exprs(position_context(1, 3));
        let mut concrete = BTreeMap::new();
        for left in &first {
            for right in &second {
                let telescope = tel(vec![left.clone(), right.clone()]);
                let mut row = concrete_row(&signature, &closure, &telescope);
                row.count = 0;
                *concrete.entry(row).or_insert(0u128) += 1;
            }
        }
        let expected = stratum
            .rows
            .iter()
            .map(|row| {
                let mut key = row.clone();
                let count = key.count;
                key.count = 0;
                (key, count)
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(concrete, expected);
    }

    #[test]
    fn raw_p5_routes_have_exact_named_failures() {
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        let untyped = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        assert_eq!(untyped.classify(&Vec::new()), TelescopeClass::Axiomatic);
        assert!(matches!(
            issue_typed_lift_token(&signature, &untyped, 15),
            Err(TokenError::LiftNotTypedAgainstExportedFormation { .. })
        ));
        assert_eq!(
            concrete_row(&signature, &closure, &untyped).amplification_route,
            AmplificationRoute::P5LiftNotTypedAgainstExportedFormation
        );

        let incomparable = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(14)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
        ]);
        assert_eq!(
            concrete_row(&signature, &closure, &incomparable).amplification_route,
            AmplificationRoute::P5NoUniqueDominantImport
        );
    }

    #[test]
    fn public_lift_and_adapter_route_does_not_supply_record_internality() {
        let signature = SealedSignature::genesis_del_h15();
        let witness = public_p5_lift_route_witness(&signature);
        assert_eq!(witness.class, TelescopeClass::Axiomatic);
        assert!(!witness.raw_step16_surface_member);
        assert!(witness.token_issued);
        assert_eq!(witness.token_dominant_import, Some(14));
        assert_eq!(witness.token_lift_clauses, vec![0]);
        assert!(witness.lift_replay_adapter_succeeded);
        assert!(!witness.record_internality_capability_available);
        assert!(!witness.full_p5_amplification_constructible);
    }

    #[test]
    fn frozen_raw_p5_domain_boundary_is_exact() {
        let signature = SealedSignature::genesis_del_h15();
        let mut domains14 = transportable_exported_domains(&signature, 14);
        domains14.sort_by_key(|expr| format!("{expr:?}"));
        let mut expected = vec![Expr::Lib(11), Expr::Lib(12), Expr::Lib(13)];
        expected.sort_by_key(|expr| format!("{expr:?}"));
        assert_eq!(domains14, expected);
        assert!(transportable_exported_domains(&signature, 15).is_empty());
    }

    fn tel(exprs: Vec<Expr>) -> Telescope {
        Telescope::new(
            exprs
                .into_iter()
                .map(|expr| ClauseRec::new(primary_role(&expr), expr))
                .collect(),
        )
    }

    #[test]
    fn priority_classifier_regressions_are_pinned() {
        let library = Vec::new();
        assert_eq!(
            tel(vec![Expr::Univ, Expr::Var(1)]).classify(&library),
            TelescopeClass::Foundation
        );
        assert_eq!(
            tel(vec![
                Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ)),
                Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            ])
            .classify(&library),
            TelescopeClass::Former
        );
        assert_eq!(
            tel(vec![Expr::PathCon(1), Expr::Next(Box::new(Expr::Var(1))),]).classify(&library),
            TelescopeClass::Hit
        );
        assert_eq!(
            tel(vec![
                Expr::Pi(Box::new(Expr::PathCon(1)), Box::new(Expr::Var(1))),
                Expr::Var(1),
            ])
            .classify(&library),
            TelescopeClass::Unknown
        );
        assert_eq!(
            tel(vec![
                Expr::Flat(Box::new(Expr::Var(1))),
                Expr::Next(Box::new(Expr::Var(1))),
            ])
            .classify(&library),
            TelescopeClass::Synthesis
        );
        assert_eq!(
            tel(vec![
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Lib(14)),
                ),
                Expr::Var(1),
                Expr::Var(1),
            ])
            .classify(&library),
            TelescopeClass::Axiomatic
        );
        assert_eq!(
            tel(vec![Expr::Univ, Expr::Lam(Box::new(Expr::Var(1)))]).classify(&library),
            TelescopeClass::Unknown
        );
    }

    #[test]
    fn replay_recomputes_and_rejects_all_material_mutations() {
        let certificate = run_ip1_candidate_join_with_caps(2, 2, 1);
        let replay = replay_ip1_candidate_join(&certificate);
        assert!(replay.valid);
        assert!(
            !certificate
                .completeness
                .full_candidate_extraction_join_proved
        );
        assert!(replay.operational_a5_verdict_join_proved);
        assert!(replay.every_raw_candidate_operationally_nonclearing);
        assert!(replay.no_certifiably_clearing_step16_under_adopted_a5);
        assert!(!replay.intended_semantic_candidate_join_proved);
        assert!(certificate.strata.iter().flat_map(|s| &s.rows).all(|row| {
            !row.implemented_egp_bridge_returns_bound
                && row.implemented_exact_egp_nu.is_none()
                && row.implemented_egp_nu_upper_bound.is_none()
                && row.implemented_route_nu_upper_bound.is_none()
        }));

        let mut row = certificate.clone();
        row.strata[0].rows[0].implemented_egp_bridge_returns_bound =
            !row.strata[0].rows[0].implemented_egp_bridge_returns_bound;
        row.digest = certificate_digest(&row);
        assert!(!replay_ip1_candidate_join(&row).valid);

        let mut fake_exact = certificate.clone();
        fake_exact
            .completeness
            .exact_family_count_distribution_claimed = true;
        fake_exact.digest = certificate_digest(&fake_exact);
        assert!(!replay_ip1_candidate_join(&fake_exact).valid);

        let mut fake_operational_join = run_ip1_candidate_join_with_caps(2, 2, 1);
        fake_operational_join
            .completeness
            .operational_a5_verdict_join_proved = false;
        fake_operational_join.digest = certificate_digest(&fake_operational_join);
        assert!(!replay_ip1_candidate_join(&fake_operational_join).valid);

        let mut missing_a5_adoption = run_ip1_candidate_join_with_caps(2, 2, 1);
        missing_a5_adoption.a5.adjudication = A5Adjudication::Undecided;
        missing_a5_adoption.digest = certificate_digest(&missing_a5_adoption);
        assert!(!replay_ip1_candidate_join(&missing_a5_adoption).valid);

        let mut signature_drift = certificate.clone();
        signature_drift.signature_digest = "blake3:forged".to_owned();
        signature_drift.digest = certificate_digest(&signature_drift);
        assert!(!replay_ip1_candidate_join(&signature_drift).valid);

        let mut domain_drift = certificate.clone();
        domain_drift.p5_transportable_domains_l14.clear();
        domain_drift.digest = certificate_digest(&domain_drift);
        assert!(!replay_ip1_candidate_join(&domain_drift).valid);

        let mut witness_drift = certificate.clone();
        witness_drift
            .public_p5_lift_route_witness
            .full_p5_amplification_constructible = true;
        witness_drift.digest = certificate_digest(&witness_drift);
        assert!(!replay_ip1_candidate_join(&witness_drift).valid);

        let mut capability_drift = certificate;
        capability_drift
            .optional_capabilities
            .public_p5_record_internality_capability_exists = true;
        capability_drift.digest = certificate_digest(&capability_drift);
        assert!(!replay_ip1_candidate_join(&capability_drift).valid);
    }
}
