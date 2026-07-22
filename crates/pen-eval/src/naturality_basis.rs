//! Sort-aware finite renaming/weakening audit for canonical families.
//!
//! This closes the *raw parameter-renaming* basis that the legacy extractor
//! represented by one unsorted transposition.  It intentionally does not
//! claim completeness for arbitrary typed substitutions or for the intended
//! depth-two semantic schema grammar; those are separate named premises in
//! the schema-4 bridge.

use crate::typed_families::{CanonicalPresentation, ParamSort, RenamingMap};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_type::equality::{EqualityWitness, univalent_equality};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const RAW_RENAMING_BASIS_VERSION: &str = "typed-family-sort-aware-renaming-basis-v1";
pub const SEMANTIC_COMPLETENESS_GAP: &str =
    "C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RenamingGeneratorWitness {
    pub left_parameter: u32,
    pub right_parameter: u32,
    pub sort: ParamSort,
    pub square: EqualityWitness,
    pub witness_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WeakeningWitness {
    pub appended_sort: ParamSort,
    pub square: EqualityWitness,
    pub witness_hash: String,
}

/// Opaque replayable certificate for the finite group of sort-preserving
/// parameter renamings and for weakening by one unused parameter of either
/// coarse sort.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RawRenamingBasisToken {
    presentation: CanonicalPresentation,
    generators: Vec<RenamingGeneratorWitness>,
    weakening: Vec<WeakeningWitness>,
    sort_preserving_permutation_count: u32,
    all_sort_preserving_permutations_checked: bool,
    identity_checked: bool,
    composition_closed: bool,
    complete_for_raw_sort_preserving_renamings: bool,
    semantic_completeness_gap: String,
    derivation_hash: String,
}

impl RawRenamingBasisToken {
    pub fn generators(&self) -> &[RenamingGeneratorWitness] {
        &self.generators
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn complete_for_raw_sort_preserving_renamings(&self) -> bool {
        self.complete_for_raw_sort_preserving_renamings
    }

    pub fn semantic_completeness_gap(&self) -> &str {
        &self.semantic_completeness_gap
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum RawRenamingBasisError {
    #[error("malformed canonical presentation: {reason}")]
    MalformedCanonicalPresentation { reason: String },
    #[error("canonical family has too many parameters for the finite audit: {arity}")]
    ArityTooLarge { arity: u32 },
    #[error("a supposedly sort-preserving renaming did not re-canonicalize to the source family")]
    NaturalitySquareFailed,
    #[error("weakening by an unused parameter did not re-canonicalize to the source family")]
    WeakeningSquareFailed,
    #[error("univalent equality normalization failed: {0}")]
    Normalize(pen_type::normalize::NormalizeError),
    #[error("raw renaming basis replay mismatch")]
    ReplayMismatch,
}

impl From<pen_type::normalize::NormalizeError> for RawRenamingBasisError {
    fn from(error: pen_type::normalize::NormalizeError) -> Self {
        Self::Normalize(error)
    }
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(RAW_RENAMING_BASIS_VERSION, domain, payload))
        .expect("renaming proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn collect_free_levels(expr: &Expr, free_scope_len: u32, order: &mut Vec<u32>) {
    match expr {
        Expr::Var(level) if *level <= free_scope_len => {
            if !order.contains(level) {
                order.push(*level);
            }
        }
        Expr::Var(_) | Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => {}
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            collect_free_levels(a, free_scope_len, order);
            collect_free_levels(b, free_scope_len, order);
        }
        Expr::Id(a, b, c) => {
            collect_free_levels(a, free_scope_len, order);
            collect_free_levels(b, free_scope_len, order);
            collect_free_levels(c, free_scope_len, order);
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
        | Expr::WhyNot(inner) => collect_free_levels(inner, free_scope_len, order),
    }
}

fn renumber_levels(
    expr: &Expr,
    free_scope_len: u32,
    map: &BTreeMap<u32, u32>,
    parameters_used: u32,
) -> Expr {
    let recurse = |inner: &Expr| renumber_levels(inner, free_scope_len, map, parameters_used);
    match expr {
        Expr::Var(level) if *level <= free_scope_len => {
            Expr::Var(*map.get(level).expect("free level is in compacting map"))
        }
        Expr::Var(level) => Expr::Var(parameters_used + (*level - free_scope_len)),
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(a, b) => Expr::App(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Pi(a, b) => Expr::Pi(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Sigma(a, b) => Expr::Sigma(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Id(a, b, c) => Expr::Id(
            Box::new(recurse(a)),
            Box::new(recurse(b)),
            Box::new(recurse(c)),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(recurse(inner))),
        Expr::Refl(inner) => Expr::Refl(Box::new(recurse(inner))),
        Expr::Susp(inner) => Expr::Susp(Box::new(recurse(inner))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(recurse(inner))),
        Expr::Flat(inner) => Expr::Flat(Box::new(recurse(inner))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(recurse(inner))),
        Expr::Disc(inner) => Expr::Disc(Box::new(recurse(inner))),
        Expr::Shape(inner) => Expr::Shape(Box::new(recurse(inner))),
        Expr::Next(inner) => Expr::Next(Box::new(recurse(inner))),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(recurse(inner))),
        Expr::Bang(inner) => Expr::Bang(Box::new(recurse(inner))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(recurse(inner))),
    }
}

fn recanonicalize_with_sorts(expr: &Expr, context: &[ParamSort]) -> CanonicalPresentation {
    let free_scope_len = context.len() as u32;
    let mut order = Vec::new();
    collect_free_levels(expr, free_scope_len, &mut order);
    let forward = order
        .iter()
        .enumerate()
        .map(|(index, level)| (*level, index as u32 + 1))
        .collect::<Vec<_>>();
    let map = forward.iter().copied().collect::<BTreeMap<_, _>>();
    let canonical_normal_form = renumber_levels(expr, free_scope_len, &map, forward.len() as u32);
    let parameters = order
        .iter()
        .map(|level| context[(*level - 1) as usize].clone())
        .collect();
    CanonicalPresentation {
        canonical_normal_form,
        parameters,
        renaming: RenamingMap {
            free_scope_len,
            forward,
        },
    }
}

fn rename_parameters(expr: &Expr, parameter_arity: u32, permutation: &[u32]) -> Expr {
    let recurse = |inner: &Expr| rename_parameters(inner, parameter_arity, permutation);
    match expr {
        Expr::Var(level) if *level <= parameter_arity => {
            Expr::Var(permutation[(*level - 1) as usize])
        }
        Expr::Var(level) => Expr::Var(*level),
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(a, b) => Expr::App(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Pi(a, b) => Expr::Pi(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Sigma(a, b) => Expr::Sigma(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Id(a, b, c) => Expr::Id(
            Box::new(recurse(a)),
            Box::new(recurse(b)),
            Box::new(recurse(c)),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(recurse(inner))),
        Expr::Refl(inner) => Expr::Refl(Box::new(recurse(inner))),
        Expr::Susp(inner) => Expr::Susp(Box::new(recurse(inner))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(recurse(inner))),
        Expr::Flat(inner) => Expr::Flat(Box::new(recurse(inner))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(recurse(inner))),
        Expr::Disc(inner) => Expr::Disc(Box::new(recurse(inner))),
        Expr::Shape(inner) => Expr::Shape(Box::new(recurse(inner))),
        Expr::Next(inner) => Expr::Next(Box::new(recurse(inner))),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(recurse(inner))),
        Expr::Bang(inner) => Expr::Bang(Box::new(recurse(inner))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(recurse(inner))),
    }
}

fn rebase_parameter_prefix(expr: &Expr, old_arity: u32, new_arity: u32) -> Expr {
    let recurse = |inner: &Expr| rebase_parameter_prefix(inner, old_arity, new_arity);
    match expr {
        Expr::Var(level) if *level > old_arity => Expr::Var(new_arity + (*level - old_arity)),
        Expr::Var(level) => Expr::Var(*level),
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(a, b) => Expr::App(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Pi(a, b) => Expr::Pi(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Sigma(a, b) => Expr::Sigma(Box::new(recurse(a)), Box::new(recurse(b))),
        Expr::Id(a, b, c) => Expr::Id(
            Box::new(recurse(a)),
            Box::new(recurse(b)),
            Box::new(recurse(c)),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(recurse(inner))),
        Expr::Refl(inner) => Expr::Refl(Box::new(recurse(inner))),
        Expr::Susp(inner) => Expr::Susp(Box::new(recurse(inner))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(recurse(inner))),
        Expr::Flat(inner) => Expr::Flat(Box::new(recurse(inner))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(recurse(inner))),
        Expr::Disc(inner) => Expr::Disc(Box::new(recurse(inner))),
        Expr::Shape(inner) => Expr::Shape(Box::new(recurse(inner))),
        Expr::Next(inner) => Expr::Next(Box::new(recurse(inner))),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(recurse(inner))),
        Expr::Bang(inner) => Expr::Bang(Box::new(recurse(inner))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(recurse(inner))),
    }
}

fn permutations(values: &mut [u32], index: usize, out: &mut Vec<Vec<u32>>) {
    if index == values.len() {
        out.push(values.to_vec());
        return;
    }
    for swap in index..values.len() {
        values.swap(index, swap);
        permutations(values, index + 1, out);
        values.swap(index, swap);
    }
}

fn compose_permutations(first: &[u32], second: &[u32]) -> Vec<u32> {
    first
        .iter()
        .map(|level| second[(*level - 1) as usize])
        .collect()
}

fn validate_canonical_presentation(
    presentation: &CanonicalPresentation,
) -> Result<(), RawRenamingBasisError> {
    let arity = presentation.parameters.len() as u32;
    if !pen_type::substitution::is_well_scoped(&presentation.canonical_normal_form, arity) {
        return Err(RawRenamingBasisError::MalformedCanonicalPresentation {
            reason: "normal form is not scoped by its canonical parameter telescope".to_owned(),
        });
    }
    let mut first_use = Vec::new();
    collect_free_levels(&presentation.canonical_normal_form, arity, &mut first_use);
    if first_use != (1..=arity).collect::<Vec<_>>() {
        return Err(RawRenamingBasisError::MalformedCanonicalPresentation {
            reason: "canonical parameters are not all present in first-use order".to_owned(),
        });
    }
    let forward = &presentation.renaming.forward;
    if forward.len() != presentation.parameters.len()
        || forward
            .iter()
            .map(|(_, target)| *target)
            .collect::<Vec<_>>()
            != (1..=arity).collect::<Vec<_>>()
    {
        return Err(RawRenamingBasisError::MalformedCanonicalPresentation {
            reason: "renaming targets are not the exact canonical parameter inventory".to_owned(),
        });
    }
    let sources = forward
        .iter()
        .map(|(source, _)| *source)
        .collect::<BTreeSet<_>>();
    if sources.len() != forward.len()
        || sources
            .iter()
            .any(|source| *source == 0 || *source > presentation.renaming.free_scope_len)
    {
        return Err(RawRenamingBasisError::MalformedCanonicalPresentation {
            reason: "renaming sources are duplicated or outside the recorded free scope".to_owned(),
        });
    }
    Ok(())
}

pub fn issue_raw_renaming_basis(
    presentation: CanonicalPresentation,
) -> Result<RawRenamingBasisToken, RawRenamingBasisError> {
    validate_canonical_presentation(&presentation)?;
    let arity = presentation.parameters.len() as u32;
    if arity > 8 {
        return Err(RawRenamingBasisError::ArityTooLarge { arity });
    }
    let mut values = (1..=arity).collect::<Vec<_>>();
    let mut all_permutations = Vec::new();
    permutations(&mut values, 0, &mut all_permutations);
    let allowed = all_permutations
        .into_iter()
        .filter(|permutation| {
            permutation.iter().enumerate().all(|(source, target)| {
                presentation.parameters[source] == presentation.parameters[(*target - 1) as usize]
            })
        })
        .collect::<Vec<_>>();
    let allowed_set = allowed.iter().cloned().collect::<BTreeSet<_>>();
    let identity = (1..=arity).collect::<Vec<_>>();
    let identity_checked = allowed_set.contains(&identity);
    let all_sort_preserving_permutations_checked = true;
    for permutation in &allowed {
        let renamed = rename_parameters(&presentation.canonical_normal_form, arity, permutation);
        let recanonical = recanonicalize_with_sorts(&renamed, &presentation.parameters);
        let square = univalent_equality(
            &presentation.canonical_normal_form,
            &recanonical.canonical_normal_form,
            arity,
            256,
        )?;
        if !square.equal || recanonical.parameters != presentation.parameters {
            return Err(RawRenamingBasisError::NaturalitySquareFailed);
        }
    }
    let composition_closed = allowed.iter().all(|first| {
        allowed
            .iter()
            .all(|second| allowed_set.contains(&compose_permutations(first, second)))
    });

    let mut generators = Vec::new();
    for sort in [ParamSort::Type, ParamSort::Opaque] {
        let positions = presentation
            .parameters
            .iter()
            .enumerate()
            .filter_map(|(index, parameter_sort)| {
                (*parameter_sort == sort).then_some(index as u32 + 1)
            })
            .collect::<Vec<_>>();
        for pair in positions.windows(2) {
            let left = pair[0];
            let right = pair[1];
            let mut permutation = identity.clone();
            permutation[(left - 1) as usize] = right;
            permutation[(right - 1) as usize] = left;
            let renamed =
                rename_parameters(&presentation.canonical_normal_form, arity, &permutation);
            let recanonical = recanonicalize_with_sorts(&renamed, &presentation.parameters);
            let square = univalent_equality(
                &presentation.canonical_normal_form,
                &recanonical.canonical_normal_form,
                arity,
                256,
            )?;
            if !square.equal || recanonical.parameters != presentation.parameters {
                return Err(RawRenamingBasisError::NaturalitySquareFailed);
            }
            let witness_hash = tagged_hash("renaming-generator", &(left, right, &sort, &square));
            generators.push(RenamingGeneratorWitness {
                left_parameter: left,
                right_parameter: right,
                sort: sort.clone(),
                square,
                witness_hash,
            });
        }
    }

    let mut weakening = Vec::new();
    for appended_sort in [ParamSort::Type, ParamSort::Opaque] {
        let mut extended = presentation.parameters.clone();
        extended.push(appended_sort.clone());
        let weakened =
            rebase_parameter_prefix(&presentation.canonical_normal_form, arity, arity + 1);
        let recanonical = recanonicalize_with_sorts(&weakened, &extended);
        let square = univalent_equality(
            &presentation.canonical_normal_form,
            &recanonical.canonical_normal_form,
            arity,
            256,
        )?;
        if !square.equal || recanonical.parameters != presentation.parameters {
            return Err(RawRenamingBasisError::WeakeningSquareFailed);
        }
        let witness_hash = tagged_hash("unused-parameter-weakening", &(&appended_sort, &square));
        weakening.push(WeakeningWitness {
            appended_sort,
            square,
            witness_hash,
        });
    }
    let complete_for_raw_sort_preserving_renamings =
        identity_checked && composition_closed && all_sort_preserving_permutations_checked;
    let sort_preserving_permutation_count = allowed.len() as u32;
    let semantic_completeness_gap = SEMANTIC_COMPLETENESS_GAP.to_owned();
    let derivation_hash = tagged_hash(
        "raw-renaming-basis",
        &(
            &presentation,
            &generators,
            &weakening,
            sort_preserving_permutation_count,
            all_sort_preserving_permutations_checked,
            identity_checked,
            composition_closed,
            complete_for_raw_sort_preserving_renamings,
            &semantic_completeness_gap,
        ),
    );
    Ok(RawRenamingBasisToken {
        presentation,
        generators,
        weakening,
        sort_preserving_permutation_count,
        all_sort_preserving_permutations_checked,
        identity_checked,
        composition_closed,
        complete_for_raw_sort_preserving_renamings,
        semantic_completeness_gap,
        derivation_hash,
    })
}

pub fn replay_raw_renaming_basis(
    token: &RawRenamingBasisToken,
) -> Result<(), RawRenamingBasisError> {
    let replay = issue_raw_renaming_basis(token.presentation.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(RawRenamingBasisError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_sort_permutation_group_and_weakening_replay() {
        let presentation = CanonicalPresentation {
            canonical_normal_form: Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(3)))),
            ),
            parameters: vec![ParamSort::Type, ParamSort::Opaque, ParamSort::Type],
            renaming: RenamingMap {
                free_scope_len: 3,
                forward: vec![(1, 1), (2, 2), (3, 3)],
            },
        };
        let token = issue_raw_renaming_basis(presentation).expect("raw basis closes");
        assert!(token.complete_for_raw_sort_preserving_renamings());
        assert_eq!(token.generators().len(), 1);
        assert_eq!(token.semantic_completeness_gap(), SEMANTIC_COMPLETENESS_GAP);
        replay_raw_renaming_basis(&token).expect("token replays");
    }

    #[test]
    fn witness_mutation_is_rejected() {
        let presentation = CanonicalPresentation {
            canonical_normal_form: Expr::Var(1),
            parameters: vec![ParamSort::Type],
            renaming: RenamingMap {
                free_scope_len: 1,
                forward: vec![(1, 1)],
            },
        };
        let mut token = issue_raw_renaming_basis(presentation).expect("raw basis closes");
        token.derivation_hash.push('0');
        assert_eq!(
            replay_raw_renaming_basis(&token),
            Err(RawRenamingBasisError::ReplayMismatch)
        );
    }

    #[test]
    fn malformed_public_presentations_are_rejected_before_token_issuance() {
        let missing_parameter = CanonicalPresentation {
            canonical_normal_form: Expr::Var(1),
            parameters: vec![ParamSort::Type, ParamSort::Type],
            renaming: RenamingMap {
                free_scope_len: 2,
                forward: vec![(1, 1), (2, 2)],
            },
        };
        assert!(matches!(
            issue_raw_renaming_basis(missing_parameter),
            Err(RawRenamingBasisError::MalformedCanonicalPresentation { .. })
        ));

        let malformed_renaming = CanonicalPresentation {
            canonical_normal_form: Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2))),
            parameters: vec![ParamSort::Type, ParamSort::Type],
            renaming: RenamingMap {
                free_scope_len: 2,
                forward: vec![(1, 1), (1, 2)],
            },
        };
        assert!(matches!(
            issue_raw_renaming_basis(malformed_renaming),
            Err(RawRenamingBasisError::MalformedCanonicalPresentation { .. })
        ));
    }
}
