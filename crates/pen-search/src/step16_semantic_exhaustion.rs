//! Phase 4 (third part) of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`:
//! exhaustive semantic partition of the admitted Step-16 cone.
//!
//! The raw Step-16 telescope surface (up to ~1.1e24 at kappa 4) cannot be
//! materialized, but its per-position expression catalogs (<= ~1.6e6)
//! can. The kernel's clause dispositions are CLAUSE-LOCAL and exactly
//! compositional for the partition flags:
//!
//! - a telescope presents a marginal family iff some clause is
//!   clause-locally marginal against the typed predecessor closure
//!   (an in-candidate subsumption never converts a locally-marginal
//!   clause into an internal one, because subsumption into an internal
//!   family implies closure subsumption of the clause itself);
//! - a telescope is kernel-invalid iff some clause is;
//! - every marginal family of a bridge submission anchors at its own
//!   generator clause, so EGP anchoring never fails on distinct slots.
//!
//! Therefore the partition
//!
//! ```text
//! internal-by-weakening/erasure | EGP-certified marginal | invalid-or-unclassified
//! ```
//!
//! is computed EXACTLY over the full raw product by a dynamic program:
//! stream each position's expression catalog once, classify every
//! expression per candidate ambient (dispositions depend on the
//! telescope's minimal ambient, so the DP tracks the running maximum
//! ambient shortfall and each telescope is counted exactly once under
//! its true ambient), and compose counts across positions in u128.
//! The only named invalid class on the raw surface is the checker's own
//! bare-Univ-argument rejection; any OTHER kernel failure lands in
//! `unclassified`, which the certificate requires to be ZERO in every
//! kappa stratum.

use crate::enumerate::{EnumerationContext, LateFamilySurface, enumerate_exprs};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_eval::typed_families::{
    ClauseClosureDisposition, PredecessorClosure, clause_presentation, closure_clause_disposition,
    predecessor_closure,
};
use pen_type::elaborate::{
    ElabError, SealedSignature, elaborate_single_clause, required_clause_ambient,
};
use serde::Serialize;
use std::collections::BTreeMap;

const MAX_AMBIENT: u32 = 2;
const STEP16_BASE_AMBIENT: u32 = 2;
const STEP16_MAX_EXPR_NODES: u8 = 6;

fn step16_position_context(position: u32) -> EnumerationContext {
    EnumerationContext {
        library_size: 15,
        scope_size: STEP16_BASE_AMBIENT + position,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes: STEP16_MAX_EXPR_NODES,
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

/// One expression's classification at one position, for every candidate
/// ambient it can inhabit and every sort assignment of the prior fields
/// it references.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExprClassKey {
    formation: bool,
    shortfall: u32,
    /// Per ambient A (index A - shortfall): the field indices (0-based
    /// prior clause positions) the canonical form uses, and per sort
    /// assignment (bitmask over those fields, bit=1 ⇒ Formation/Type)
    /// whether the clause is marginal.
    per_ambient: Vec<(u32, Vec<u16>, Vec<bool>)>,
    invalid_named: bool,
    unclassified: bool,
}

fn classify_position_exprs(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    position: u32,
) -> (u64, BTreeMap<ExprClassKey, u128>) {
    let context = step16_position_context(position);
    let exprs = enumerate_exprs(context);
    let width = exprs.len() as u64;
    let mut classes: BTreeMap<ExprClassKey, u128> = BTreeMap::new();

    for expr in &exprs {
        let shortfall = required_clause_ambient(expr, position);
        let formation_probe = elaborate_single_clause(
            expr,
            MAX_AMBIENT,
            &vec![ClauseRole::Formation; position as usize],
            15,
        );
        let (formation, invalid_named, unclassified) = match &formation_probe {
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
                let elaborated = match elaborate_single_clause(
                    expr,
                    ambient,
                    &vec![ClauseRole::Formation; position as usize],
                    15,
                ) {
                    Ok(elaborated) => elaborated,
                    Err(_) => continue,
                };
                let free_scope = ambient + position;
                // Determine used field levels from a probe canonicalization
                // (sorts do not change WHICH levels are used).
                let probe = clause_presentation(
                    &elaborated.normal_form,
                    free_scope,
                    &vec![ClauseRole::Formation; position as usize],
                    ambient,
                );
                let used_fields: Vec<u16> = probe
                    .renaming
                    .forward
                    .iter()
                    .filter(|(level, _)| *level > ambient)
                    .map(|(level, _)| (*level - ambient - 1) as u16)
                    .collect();
                let assignments = 1usize << used_fields.len();
                let mut marginal_by_assignment = Vec::with_capacity(assignments);
                for assignment in 0..assignments {
                    let mut roles = vec![ClauseRole::Introduction; position as usize];
                    for (bit, field) in used_fields.iter().enumerate() {
                        if assignment & (1 << bit) != 0 {
                            roles[*field as usize] = ClauseRole::Formation;
                        }
                    }
                    let presentation =
                        clause_presentation(&elaborated.normal_form, free_scope, &roles, ambient);
                    let disposition = closure_clause_disposition(signature, closure, &presentation);
                    marginal_by_assignment.push(disposition == ClauseClosureDisposition::Marginal);
                }
                per_ambient.push((ambient, used_fields, marginal_by_assignment));
            }
        }

        let key = ExprClassKey {
            formation,
            shortfall,
            per_ambient,
            invalid_named,
            unclassified,
        };
        *classes.entry(key).or_insert(0) += 1;
    }
    (width, classes)
}

/// DP state within one final-ambient pass: (formation bitmask of prior
/// clauses, running max ambient shortfall, any marginal clause, any
/// named-invalid clause, any unclassified clause) → telescope count.
/// The pass fixes the FINAL ambient A: every clause's disposition is
/// looked up under A, clauses demanding more than A are excluded, and
/// only end states whose max shortfall EQUALS A are kept — so each
/// telescope is counted exactly once, under its true minimal ambient,
/// with all dispositions consistent.
type DpState = (u8, u32, bool, bool, bool);

/// serde_json has no native u128 encoding; the exact counts (up to
/// ~1.1e24) serialize as decimal strings.
fn u128_string<S: serde::Serializer>(value: &u128, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KappaStratumExhaustion {
    pub kappa: u16,
    pub per_position_widths: Vec<u64>,
    #[serde(serialize_with = "u128_string")]
    pub raw_total: u128,
    #[serde(serialize_with = "u128_string")]
    pub internal: u128,
    #[serde(serialize_with = "u128_string")]
    pub egp_marginal: u128,
    #[serde(serialize_with = "u128_string")]
    pub invalid_named_bare_univ: u128,
    #[serde(serialize_with = "u128_string")]
    pub unclassified: u128,
    pub sums_match: bool,
    pub unclassified_is_zero: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step16SemanticExhaustion {
    pub signature_digest: String,
    pub closure_digest: String,
    pub strata: Vec<KappaStratumExhaustion>,
    pub every_stratum_classified: bool,
    pub digest: String,
}

fn exhaust_stratum(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    kappa: u16,
    position_classes: &[(u64, BTreeMap<ExprClassKey, u128>)],
) -> KappaStratumExhaustion {
    let _ = (signature, closure);
    let mut internal: u128 = 0;
    let mut egp: u128 = 0;
    let mut invalid: u128 = 0;
    let mut unclassified: u128 = 0;

    // One pass per final ambient A; keep only telescopes whose max
    // clause shortfall equals A (their true minimal ambient).
    for final_ambient in 0..=MAX_AMBIENT {
        let mut states: BTreeMap<DpState, u128> = BTreeMap::new();
        states.insert((0, 0, false, false, false), 1);
        for position in 0..kappa as u32 {
            let (_, classes) = &position_classes[position as usize];
            let mut next: BTreeMap<DpState, u128> = BTreeMap::new();
            for ((bits, max_shortfall, any_marginal, any_invalid, any_uncl), count) in &states {
                for (class, class_count) in classes {
                    if class.shortfall > final_ambient {
                        continue;
                    }
                    let new_bits = if class.formation {
                        bits | (1u8 << position)
                    } else {
                        *bits
                    };
                    let new_shortfall = (*max_shortfall).max(class.shortfall);
                    if class.invalid_named || class.unclassified {
                        let key = (
                            new_bits,
                            new_shortfall,
                            *any_marginal,
                            *any_invalid || class.invalid_named,
                            *any_uncl || class.unclassified,
                        );
                        *next.entry(key).or_insert(0) += count * class_count;
                        continue;
                    }
                    let Some((_, used_fields, marginal_by_assignment)) = class
                        .per_ambient
                        .iter()
                        .find(|(ambient, _, _)| *ambient == final_ambient)
                    else {
                        continue;
                    };
                    let mut assignment = 0usize;
                    for (bit, field) in used_fields.iter().enumerate() {
                        if new_bits & (1u8 << *field as u8) != 0 {
                            assignment |= 1 << bit;
                        }
                    }
                    let marginal = marginal_by_assignment[assignment];
                    let key = (
                        new_bits,
                        new_shortfall,
                        *any_marginal || marginal,
                        *any_invalid,
                        *any_uncl,
                    );
                    *next.entry(key).or_insert(0) += count * class_count;
                }
            }
            states = next;
        }
        for ((_, max_shortfall, any_marginal, any_invalid, any_uncl), count) in &states {
            if *max_shortfall != final_ambient {
                continue;
            }
            if *any_uncl {
                unclassified += count;
            } else if *any_invalid {
                invalid += count;
            } else if *any_marginal {
                egp += count;
            } else {
                internal += count;
            }
        }
    }

    let per_position_widths: Vec<u64> = position_classes
        .iter()
        .take(kappa as usize)
        .map(|(width, _)| *width)
        .collect();
    let raw_total: u128 = per_position_widths
        .iter()
        .fold(1u128, |acc, width| acc * (*width as u128));
    let sum = internal + egp + invalid + unclassified;
    KappaStratumExhaustion {
        kappa,
        per_position_widths,
        raw_total,
        internal,
        egp_marginal: egp,
        invalid_named_bare_univ: invalid,
        unclassified,
        sums_match: sum == raw_total,
        unclassified_is_zero: unclassified == 0,
    }
}

/// Run the full exhaustion over kappa strata 2..=4 (release-grade; the
/// per-position streams are large). `max_kappa` allows sub-cap runs.
pub fn run_step16_semantic_exhaustion(max_kappa: u16) -> Step16SemanticExhaustion {
    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature).expect("closure");
    let max_positions = max_kappa as u32;
    let mut position_classes = Vec::new();
    for position in 0..max_positions {
        position_classes.push(classify_position_exprs(&signature, &closure, position));
    }
    let mut strata = Vec::new();
    for kappa in 2..=max_kappa {
        strata.push(exhaust_stratum(
            &signature,
            &closure,
            kappa,
            &position_classes,
        ));
    }
    let every_stratum_classified = strata
        .iter()
        .all(|stratum| stratum.sums_match && stratum.unclassified_is_zero);
    let payload = serde_json::json!({
        "signature_digest": signature.digest(),
        "closure_digest": closure.digest,
        "strata": strata,
    });
    let digest = format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&payload).expect("exhaustion serialization"))
    );
    Step16SemanticExhaustion {
        signature_digest: signature.digest().to_string(),
        closure_digest: closure.digest.clone(),
        strata,
        every_stratum_classified,
        digest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::ClauseRec;
    use pen_core::telescope::Telescope;
    use pen_eval::typed_families::{CandidateExtractionOutcome, extract_candidate_families};

    /// Mirror of pen-core's role assignment for building raw clauses.
    fn primary_role_for_test(expr: &Expr) -> ClauseRole {
        match expr {
            Expr::Susp(_) => ClauseRole::Formation,
            Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
                ClauseRole::Formation
            }
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => ClauseRole::Formation,
            Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lib(_)) => ClauseRole::Introduction,
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lam(_)) => ClauseRole::Elimination,
            Expr::App(_, _) => ClauseRole::Introduction,
            Expr::PathCon(_) => ClauseRole::PathAttach,
            _ => ClauseRole::Formation,
        }
    }

    /// Exhaustive sub-cap bisimulation: materialize every kappa-2
    /// telescope over 2-node expressions, extract families concretely
    /// through the full Phase-2 pipeline, and compare the partition
    /// with the DP's counts exactly.
    #[test]
    fn sub_cap_bisimulation_matches_concrete_extraction() {
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");

        let sub_nodes: u8 = 2;
        let kappa: u16 = 2;
        let mut position_exprs = Vec::new();
        let mut position_classes = Vec::new();
        for position in 0..kappa as u32 {
            let mut context = step16_position_context(position);
            context.max_expr_nodes = sub_nodes;
            let exprs = enumerate_exprs(context);
            // Reuse the classifier on the reduced catalog.
            let mut classes = BTreeMap::new();
            for expr in &exprs {
                let shortfall = required_clause_ambient(expr, position);
                let probe = elaborate_single_clause(
                    expr,
                    MAX_AMBIENT,
                    &vec![ClauseRole::Formation; position as usize],
                    15,
                );
                let (formation, invalid_named, unclassified) = match &probe {
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
                        let elaborated = elaborate_single_clause(
                            expr,
                            ambient,
                            &vec![ClauseRole::Formation; position as usize],
                            15,
                        )
                        .expect("valid");
                        let free_scope = ambient + position;
                        let probe_presentation = clause_presentation(
                            &elaborated.normal_form,
                            free_scope,
                            &vec![ClauseRole::Formation; position as usize],
                            ambient,
                        );
                        let used_fields: Vec<u16> = probe_presentation
                            .renaming
                            .forward
                            .iter()
                            .filter(|(level, _)| *level > ambient)
                            .map(|(level, _)| (*level - ambient - 1) as u16)
                            .collect();
                        let assignments = 1usize << used_fields.len();
                        let mut marginal_by_assignment = Vec::with_capacity(assignments);
                        for assignment in 0..assignments {
                            let mut roles = vec![ClauseRole::Introduction; position as usize];
                            for (bit, field) in used_fields.iter().enumerate() {
                                if assignment & (1 << bit) != 0 {
                                    roles[*field as usize] = ClauseRole::Formation;
                                }
                            }
                            let presentation = clause_presentation(
                                &elaborated.normal_form,
                                free_scope,
                                &roles,
                                ambient,
                            );
                            let disposition =
                                closure_clause_disposition(&signature, &closure, &presentation);
                            marginal_by_assignment
                                .push(disposition == ClauseClosureDisposition::Marginal);
                        }
                        per_ambient.push((ambient, used_fields, marginal_by_assignment));
                    }
                }
                let key = ExprClassKey {
                    formation,
                    shortfall,
                    per_ambient,
                    invalid_named,
                    unclassified,
                };
                *classes.entry(key).or_insert(0u128) += 1;
            }
            position_classes.push((exprs.len() as u64, classes));
            position_exprs.push(exprs);
        }

        let dp = exhaust_stratum(&signature, &closure, kappa, &position_classes);

        // Concrete side: the full product, one extraction per telescope.
        let mut concrete_internal = 0u128;
        let mut concrete_marginal = 0u128;
        let mut concrete_invalid = 0u128;
        let mut concrete_unclassified = 0u128;
        for first in &position_exprs[0] {
            for second in &position_exprs[1] {
                let telescope = Telescope::new(vec![
                    ClauseRec::new(primary_role_for_test(first), first.clone()),
                    ClauseRec::new(primary_role_for_test(second), second.clone()),
                ]);
                match extract_candidate_families(&signature, &closure, &telescope, 15) {
                    CandidateExtractionOutcome::Extracted(extraction) => {
                        if extraction.marginal_family_count > 0 {
                            concrete_marginal += 1;
                        } else {
                            concrete_internal += 1;
                        }
                    }
                    CandidateExtractionOutcome::KernelInvalid { failure } => {
                        if matches!(failure.error, ElabError::BareUnivArgument) {
                            concrete_invalid += 1;
                        } else {
                            concrete_unclassified += 1;
                        }
                    }
                }
            }
        }

        assert_eq!(
            dp.raw_total,
            concrete_internal + concrete_marginal + concrete_invalid + concrete_unclassified
        );
        assert_eq!(dp.internal, concrete_internal, "internal counts diverge");
        assert_eq!(
            dp.egp_marginal, concrete_marginal,
            "marginal counts diverge"
        );
        assert_eq!(
            dp.invalid_named_bare_univ, concrete_invalid,
            "named-invalid counts diverge"
        );
        assert_eq!(
            dp.unclassified, concrete_unclassified,
            "unclassified counts diverge"
        );
        assert!(dp.sums_match);
        assert_eq!(dp.unclassified, 0, "sub-cap unclassified must be zero");
    }
}
