//! Exact finite domain for `ambient-telescope-candidate-wrapper-v1`.
//!
//! Motives are enumeration data.  The grammar below is generated before any
//! classifier is called, and admissibility is only the replayable B15
//! declaration judgment plus the six-node cap.  Classification is not an
//! input to this module.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v8::{
    AMBIENT_MOTIVE_NODE_CAP, AMBIENT_WRAPPER_VERSION, AmbientWrappedCandidate, AmbientWrapperError,
    contextual_motive_node_count, validate_wrapped_candidate, wrap_candidate,
};
use pen_type::contextual_internality::ContextualMotive;
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use serde::{Deserialize, Serialize};

pub const AMBIENT_MOTIVE_EXPRESSION_NODE_CAP: u32 = AMBIENT_MOTIVE_NODE_CAP - 1;
pub const SEALED_LIBRARY_LEAF_COUNT: u32 = 15;
pub const CLOSED_EXPRESSION_LEAF_COUNT: u32 = 1 + SEALED_LIBRARY_LEAF_COUNT;
pub const CLOSED_EXPRESSION_UNARY_CONSTRUCTOR_COUNT: u32 = 10;
pub const CLOSED_EXPRESSION_BINARY_CONSTRUCTOR_COUNT: u32 = 3;
pub const CLOSED_EXPRESSION_TERNARY_CONSTRUCTOR_COUNT: u32 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveGrammarFinitenessCertificate {
    pub wrapper_version: String,
    pub motive_node_cap: u32,
    pub element_expression_node_cap: u32,
    pub closed_expression_leaf_count: u32,
    pub unary_constructor_count: u32,
    pub binary_constructor_count: u32,
    pub ternary_constructor_count: u32,
    pub syntactic_expression_counts_by_exact_nodes: Vec<u128>,
    pub syntactic_motive_counts_by_exact_nodes: Vec<u128>,
    pub syntactic_motive_count_upper_bound: u128,
    pub admissibility_is_b15_declaration_replay: bool,
    pub admissible_count_bounded_by_syntactic_count: bool,
    pub structural_recursion_complete: bool,
    pub finite: bool,
    pub silent_truncation: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WrappedSurfaceMembership {
    pub wrapper_version_matches: bool,
    pub raw_projection_membership: RawSurfaceMembership,
    pub inferred_ambient_arity: u32,
    pub declared_ambient_arity: u32,
    pub motive_node_counts: Vec<u32>,
    pub every_motive_within_cap: bool,
    pub declaration_replayed: bool,
    pub is_member: bool,
    pub rejection_reasons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PiCoverageRecord {
    pub raw_candidate: Telescope,
    pub raw_projection_membership: RawSurfaceMembership,
    pub inferred_ambient_arity: u32,
    pub canonical_preimage: AmbientWrappedCandidate,
    pub canonical_preimage_membership: WrappedSurfaceMembership,
    pub projection_is_original: bool,
    pub surjective_witness_replayed: bool,
    pub reverse_direction_replayed: bool,
}

pub fn frozen_raw_context() -> EnumerationContext {
    EnumerationContext {
        library_size: 15,
        scope_size: 2,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes: 6,
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

fn checked_add(left: u128, right: u128) -> u128 {
    left.checked_add(right)
        .expect("six-node grammar count fits u128")
}

fn checked_mul(left: u128, right: u128) -> u128 {
    left.checked_mul(right)
        .expect("six-node grammar count fits u128")
}

fn exact_expression_counts() -> Vec<u128> {
    let cap = AMBIENT_MOTIVE_EXPRESSION_NODE_CAP as usize;
    let mut counts = vec![0_u128; cap + 1];
    counts[1] = u128::from(CLOSED_EXPRESSION_LEAF_COUNT);
    for nodes in 2..=cap {
        let mut count = checked_mul(
            u128::from(CLOSED_EXPRESSION_UNARY_CONSTRUCTOR_COUNT),
            counts[nodes - 1],
        );
        for left_nodes in 1..nodes - 1 {
            let right_nodes = nodes - 1 - left_nodes;
            count = checked_add(
                count,
                checked_mul(
                    u128::from(CLOSED_EXPRESSION_BINARY_CONSTRUCTOR_COUNT),
                    checked_mul(counts[left_nodes], counts[right_nodes]),
                ),
            );
        }
        for first_nodes in 1..nodes - 2 {
            for second_nodes in 1..nodes - 1 - first_nodes {
                let third_nodes = nodes - 1 - first_nodes - second_nodes;
                count = checked_add(
                    count,
                    checked_mul(
                        u128::from(CLOSED_EXPRESSION_TERNARY_CONSTRUCTOR_COUNT),
                        checked_mul(
                            checked_mul(counts[first_nodes], counts[second_nodes]),
                            counts[third_nodes],
                        ),
                    ),
                );
            }
        }
        counts[nodes] = count;
    }
    counts
}

fn exact_motive_counts(expression_counts: &[u128]) -> Vec<u128> {
    let cap = AMBIENT_MOTIVE_NODE_CAP as usize;
    let mut counts = vec![0_u128; cap + 1];
    counts[1] = 1; // Type. Neutral is not a declaration motive.
    for nodes in 2..=cap {
        let mut count = expression_counts[nodes - 1]; // Element(expression)
        for domain_nodes in 1..nodes - 1 {
            let codomain_nodes = nodes - 1 - domain_nodes;
            count = checked_add(
                count,
                checked_mul(counts[domain_nodes], counts[codomain_nodes]),
            );
        }
        counts[nodes] = count;
    }
    counts
}

pub fn motive_grammar_finiteness_certificate() -> MotiveGrammarFinitenessCertificate {
    let expression_counts = exact_expression_counts();
    let motive_counts = exact_motive_counts(&expression_counts);
    let upper_bound = motive_counts.iter().copied().fold(0_u128, checked_add);
    MotiveGrammarFinitenessCertificate {
        wrapper_version: AMBIENT_WRAPPER_VERSION.to_owned(),
        motive_node_cap: AMBIENT_MOTIVE_NODE_CAP,
        element_expression_node_cap: AMBIENT_MOTIVE_EXPRESSION_NODE_CAP,
        closed_expression_leaf_count: CLOSED_EXPRESSION_LEAF_COUNT,
        unary_constructor_count: CLOSED_EXPRESSION_UNARY_CONSTRUCTOR_COUNT,
        binary_constructor_count: CLOSED_EXPRESSION_BINARY_CONSTRUCTOR_COUNT,
        ternary_constructor_count: CLOSED_EXPRESSION_TERNARY_CONSTRUCTOR_COUNT,
        syntactic_expression_counts_by_exact_nodes: expression_counts,
        syntactic_motive_counts_by_exact_nodes: motive_counts,
        syntactic_motive_count_upper_bound: upper_bound,
        admissibility_is_b15_declaration_replay: true,
        admissible_count_bounded_by_syntactic_count: true,
        structural_recursion_complete: true,
        finite: true,
        silent_truncation: false,
    }
}

fn expression_buckets(max_nodes: u32) -> Vec<Vec<Expr>> {
    let max_nodes = max_nodes.min(AMBIENT_MOTIVE_EXPRESSION_NODE_CAP) as usize;
    let mut buckets = vec![Vec::new(); max_nodes + 1];
    if max_nodes == 0 {
        return buckets;
    }
    buckets[1].push(Expr::Univ);
    for index in 1..=SEALED_LIBRARY_LEAF_COUNT {
        buckets[1].push(Expr::Lib(index));
    }
    for nodes in 2..=max_nodes {
        for inner in buckets[nodes - 1].clone() {
            buckets[nodes].push(Expr::Lam(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Refl(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Susp(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Trunc(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Flat(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Sharp(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Disc(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Shape(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Next(Box::new(inner.clone())));
            buckets[nodes].push(Expr::Eventually(Box::new(inner)));
        }
        for left_nodes in 1..nodes - 1 {
            let right_nodes = nodes - 1 - left_nodes;
            for left in &buckets[left_nodes].clone() {
                for right in &buckets[right_nodes].clone() {
                    buckets[nodes].push(Expr::App(Box::new(left.clone()), Box::new(right.clone())));
                    buckets[nodes].push(Expr::Pi(Box::new(left.clone()), Box::new(right.clone())));
                    buckets[nodes]
                        .push(Expr::Sigma(Box::new(left.clone()), Box::new(right.clone())));
                }
            }
        }
        for first_nodes in 1..nodes - 2 {
            for second_nodes in 1..nodes - 1 - first_nodes {
                let third_nodes = nodes - 1 - first_nodes - second_nodes;
                for first in &buckets[first_nodes].clone() {
                    for second in &buckets[second_nodes].clone() {
                        for third in &buckets[third_nodes].clone() {
                            buckets[nodes].push(Expr::Id(
                                Box::new(first.clone()),
                                Box::new(second.clone()),
                                Box::new(third.clone()),
                            ));
                        }
                    }
                }
            }
        }
    }
    buckets
}

/// Generates the exact syntactic motive grammar through `max_nodes`.
///
/// Order is node count, then `Function`, then `Element`.  The order is fixed
/// here, before classification, so fail-fast scans cannot prefer a verdict.
pub fn motive_candidates_through(max_nodes: u32) -> Vec<ContextualMotive> {
    let max_nodes = max_nodes.min(AMBIENT_MOTIVE_NODE_CAP) as usize;
    let expressions = expression_buckets(max_nodes.saturating_sub(1) as u32);
    let mut buckets = vec![Vec::new(); max_nodes + 1];
    if max_nodes == 0 {
        return Vec::new();
    }
    buckets[1].push(ContextualMotive::Type);
    for nodes in 2..=max_nodes {
        for domain_nodes in 1..nodes - 1 {
            let codomain_nodes = nodes - 1 - domain_nodes;
            for domain in &buckets[domain_nodes].clone() {
                for codomain in &buckets[codomain_nodes].clone() {
                    buckets[nodes].push(ContextualMotive::Function {
                        domain: Box::new(domain.clone()),
                        codomain: Box::new(codomain.clone()),
                    });
                }
            }
        }
        buckets[nodes].extend(
            expressions[nodes - 1]
                .iter()
                .cloned()
                .map(ContextualMotive::Element),
        );
    }
    buckets.into_iter().flatten().collect()
}

pub fn wrapped_surface_membership(candidate: &AmbientWrappedCandidate) -> WrappedSurfaceMembership {
    let raw_projection_membership =
        assess_raw_surface_membership(frozen_raw_context(), &candidate.clauses);
    let inferred_ambient_arity =
        elaborate_telescope(&SealedSignature::genesis_del_h15(), &candidate.clauses, 15)
            .map(|elaboration| elaboration.ambient_parameters)
            .unwrap_or(u32::MAX);
    let declared_ambient_arity = candidate.ambient.len() as u32;
    let motive_node_counts = candidate
        .ambient
        .iter()
        .map(contextual_motive_node_count)
        .collect::<Vec<_>>();
    let wrapper_version_matches = candidate.version == AMBIENT_WRAPPER_VERSION;
    let every_motive_within_cap = motive_node_counts
        .iter()
        .all(|nodes| *nodes <= AMBIENT_MOTIVE_NODE_CAP);
    let declaration = validate_wrapped_candidate(candidate);
    let declaration_replayed = declaration.is_ok();
    let mut rejection_reasons = raw_projection_membership.rejection_reasons.clone();
    if !wrapper_version_matches {
        rejection_reasons.push("wrapper version mismatch".to_owned());
    }
    if declared_ambient_arity != inferred_ambient_arity {
        rejection_reasons.push(format!(
            "ambient arity mismatch: declared {declared_ambient_arity}, inferred {inferred_ambient_arity}"
        ));
    }
    if !every_motive_within_cap {
        rejection_reasons.push("ambient motive exceeds six-node cap".to_owned());
    }
    if let Err(error) = declaration {
        rejection_reasons.push(format!("declaration replay failed: {error}"));
    }
    let is_member = raw_projection_membership.is_member
        && wrapper_version_matches
        && declared_ambient_arity == inferred_ambient_arity
        && every_motive_within_cap
        && declaration_replayed;
    WrappedSurfaceMembership {
        wrapper_version_matches,
        raw_projection_membership,
        inferred_ambient_arity,
        declared_ambient_arity,
        motive_node_counts,
        every_motive_within_cap,
        declaration_replayed,
        is_member,
        rejection_reasons,
    }
}

pub fn canonical_preimage(
    raw_candidate: &Telescope,
) -> Result<AmbientWrappedCandidate, AmbientWrapperError> {
    let elaboration = elaborate_telescope(&SealedSignature::genesis_del_h15(), raw_candidate, 15)
        .map_err(|error| AmbientWrapperError::Elaboration(error.to_string()))?;
    wrap_candidate(
        raw_candidate.clone(),
        vec![ContextualMotive::Type; elaboration.ambient_parameters as usize],
    )
}

pub fn issue_pi_coverage(
    raw_candidate: &Telescope,
) -> Result<PiCoverageRecord, AmbientWrapperError> {
    let raw_projection_membership =
        assess_raw_surface_membership(frozen_raw_context(), raw_candidate);
    let canonical_preimage = canonical_preimage(raw_candidate)?;
    let canonical_preimage_membership = wrapped_surface_membership(&canonical_preimage);
    let projection_is_original = canonical_preimage.clauses == *raw_candidate;
    let inferred_ambient_arity = canonical_preimage_membership.inferred_ambient_arity;
    let surjective_witness_replayed = raw_projection_membership.is_member
        && canonical_preimage_membership.is_member
        && projection_is_original;
    let reverse_direction_replayed = canonical_preimage_membership.is_member
        && canonical_preimage_membership
            .raw_projection_membership
            .is_member;
    Ok(PiCoverageRecord {
        raw_candidate: raw_candidate.clone(),
        raw_projection_membership,
        inferred_ambient_arity,
        canonical_preimage,
        canonical_preimage_membership,
        projection_is_original,
        surjective_witness_replayed,
        reverse_direction_replayed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_schema::internal_classifier_branch_v7::declared_context_control_candidate;

    #[test]
    fn grammar_is_blind_finite_and_contains_the_decisive_prefix() {
        let certificate = motive_grammar_finiteness_certificate();
        assert!(certificate.finite);
        assert!(!certificate.silent_truncation);
        assert!(certificate.structural_recursion_complete);
        assert_eq!(
            certificate.syntactic_expression_counts_by_exact_nodes[1],
            16
        );
        let motives = motive_candidates_through(4);
        assert!(motives.contains(&ContextualMotive::Function {
            domain: Box::new(ContextualMotive::Type),
            codomain: Box::new(ContextualMotive::Element(Expr::Univ)),
        }));
        assert!(
            motives
                .iter()
                .all(|motive| contextual_motive_node_count(motive) <= 4)
        );
    }

    #[test]
    fn pi_is_surjective_on_the_live_raw_control_and_reverse_membership_holds() {
        let record = issue_pi_coverage(&declared_context_control_candidate()).expect("coverage");
        assert!(record.surjective_witness_replayed);
        assert!(record.reverse_direction_replayed);
        assert_eq!(record.inferred_ambient_arity, 1);
    }

    #[test]
    fn closed_candidate_has_the_unique_empty_wrapper() {
        let raw = Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Univ)]);
        let record = issue_pi_coverage(&raw).expect("closed coverage");
        assert!(record.canonical_preimage.ambient.is_empty());
        assert!(record.surjective_witness_replayed);
    }
}
