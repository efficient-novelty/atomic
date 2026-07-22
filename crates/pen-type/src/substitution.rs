//! Capture-safe simultaneous parameter substitution for the bounded kernel
//! expression IR.
//!
//! This module deliberately separates two statements which the legacy
//! family matcher used to conflate:
//!
//! 1. every structurally well-scoped parallel substitution can be applied;
//! 2. a substitution is known to preserve the coarse `Type`/`Opaque`
//!    parameter sorts.
//!
//! The second statement is issued only for variable images whose target sort
//! is read from the target context.  A non-variable image needs an actual
//! typing derivation; the current bounded kernel does not expose one for an
//! arbitrary canonical-family context, so such an image is a named error,
//! never a silently accepted specialization.

use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

pub const SUBSTITUTION_FRAGMENT_VERSION: &str = "kernel-parallel-substitution-v1";
pub const SORT_PRESERVATION_SCOPE: &str =
    "sort-identical variable images; arbitrary typed instance images remain an explicit gap";

/// Exhaustive constructor inventory for [`Expr`].  The recursive functions
/// below match every constructor explicitly; adding a new constructor to the
/// core enum therefore fails compilation until this module is extended.
pub const EXPR_CONSTRUCTOR_COVERAGE: [&str; 20] = [
    "App",
    "Lam",
    "Pi",
    "Sigma",
    "Univ",
    "Var",
    "Lib",
    "Id",
    "Refl",
    "Susp",
    "Trunc",
    "PathCon",
    "Flat",
    "Sharp",
    "Disc",
    "Shape",
    "Next",
    "Eventually",
    "Bang",
    "WhyNot",
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterSort {
    Type,
    Opaque,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SortedParameterContext {
    parameters: Vec<ParameterSort>,
}

impl SortedParameterContext {
    pub fn new(parameters: Vec<ParameterSort>) -> Self {
        Self { parameters }
    }

    pub fn all_type(arity: u32) -> Self {
        Self {
            parameters: vec![ParameterSort::Type; arity as usize],
        }
    }

    pub fn parameters(&self) -> &[ParameterSort] {
        &self.parameters
    }

    pub fn arity(&self) -> u32 {
        u32::try_from(self.parameters.len()).expect("parameter context fits u32")
    }

    fn sort(&self, one_based_level: u32) -> Option<ParameterSort> {
        one_based_level
            .checked_sub(1)
            .and_then(|index| self.parameters.get(index as usize))
            .copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SubstitutionImage {
    pub source_parameter: u32,
    pub term: Expr,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum SubstitutionError {
    #[error("substitution has {found} images, expected exactly {expected}")]
    ImageArity { expected: u32, found: u32 },
    #[error("substitution images must name each source parameter 1..={arity} exactly once")]
    InvalidSourceInventory { arity: u32 },
    #[error(
        "image for source parameter {source_parameter} is not scoped by the {target_arity}-parameter target context"
    )]
    ImageOutOfScope {
        source_parameter: u32,
        target_arity: u32,
    },
    #[error("substitution body is not scoped by the {source_arity}-parameter source context")]
    BodyOutOfScope { source_arity: u32 },
    #[error(
        "sort-preservation requires a variable image for source parameter {source_parameter}; an arbitrary image needs a typed instance judgement"
    )]
    NonVariableImageNeedsTypedJudgement { source_parameter: u32 },
    #[error(
        "variable image Var({target_parameter}) for source parameter {source_parameter} is outside the target context"
    )]
    TargetParameterOutOfScope {
        source_parameter: u32,
        target_parameter: u32,
    },
    #[error(
        "source parameter {source_parameter} has sort {source_sort:?}, but target parameter {target_parameter} has sort {target_sort:?}"
    )]
    SortMismatch {
        source_parameter: u32,
        source_sort: ParameterSort,
        target_parameter: u32,
        target_sort: ParameterSort,
    },
    #[error("substitution replay mismatch")]
    ReplayMismatch,
    #[error("substitution composition contexts do not match")]
    CompositionContextMismatch,
    #[error("sequential substitution and the composed substitution produced different results")]
    CompositionResultMismatch,
}

/// Opaque evidence that a structurally well-scoped simultaneous
/// substitution was applied in one traversal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StructuralSubstitutionToken {
    source: SortedParameterContext,
    target: SortedParameterContext,
    images: Vec<SubstitutionImage>,
    body: Expr,
    result: Expr,
    constructor_coverage: Vec<String>,
    derivation_hash: String,
}

impl StructuralSubstitutionToken {
    pub fn result(&self) -> &Expr {
        &self.result
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn source(&self) -> &SortedParameterContext {
        &self.source
    }

    pub fn target(&self) -> &SortedParameterContext {
        &self.target
    }

    pub fn images(&self) -> &[SubstitutionImage] {
        &self.images
    }
}

/// Opaque evidence for the restricted sort theorem currently justified by
/// the kernel: every image is a target-context variable of the same sort.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SortPreservingSubstitutionToken {
    structural: StructuralSubstitutionToken,
    scope: String,
    image_sort_equalities: Vec<(u32, ParameterSort, u32, ParameterSort)>,
    derivation_hash: String,
}

impl SortPreservingSubstitutionToken {
    pub fn result(&self) -> &Expr {
        self.structural.result()
    }

    pub fn structural(&self) -> &StructuralSubstitutionToken {
        &self.structural
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn theorem_scope(&self) -> &str {
        &self.scope
    }
}

/// Replayable support theorem for the restricted variable-image fragment.
/// Since every image is a variable, applying such a substitution cannot
/// introduce a library leaf absent from the source expression.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VariableImageSupportToken {
    substitution_derivation_hash: String,
    source_library_support: Vec<u32>,
    result_library_support: Vec<u32>,
    no_image_contains_library_leaf: bool,
    support_preserved_exactly: bool,
    derivation_hash: String,
}

impl VariableImageSupportToken {
    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn support_preserved_exactly(&self) -> bool {
        self.support_preserved_exactly
    }
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(SUBSTITUTION_FRAGMENT_VERSION, domain, payload))
        .expect("substitution proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

/// Decide whether an expression is scoped by `free_parameters` plus the
/// binders encountered on the recursive path.
pub fn is_well_scoped(expr: &Expr, free_parameters: u32) -> bool {
    well_scoped_at_depth(expr, free_parameters, 0)
}

fn well_scoped_at_depth(expr: &Expr, free_parameters: u32, depth: u32) -> bool {
    let recurse = |inner: &Expr| well_scoped_at_depth(inner, free_parameters, depth);
    match expr {
        Expr::Var(level) => *level > 0 && *level <= free_parameters.saturating_add(depth),
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => true,
        Expr::App(left, right) => recurse(left) && recurse(right),
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            recurse(domain) && well_scoped_at_depth(codomain, free_parameters, depth + 1)
        }
        Expr::Lam(body) => well_scoped_at_depth(body, free_parameters, depth + 1),
        Expr::Id(ty, left, right) => recurse(ty) && recurse(left) && recurse(right),
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
        | Expr::WhyNot(inner) => recurse(inner),
    }
}

fn canonical_images(
    source_arity: u32,
    target_arity: u32,
    images: &[SubstitutionImage],
) -> Result<Vec<SubstitutionImage>, SubstitutionError> {
    let found = u32::try_from(images.len()).expect("image inventory fits u32");
    if found != source_arity {
        return Err(SubstitutionError::ImageArity {
            expected: source_arity,
            found,
        });
    }
    let inventory = images
        .iter()
        .map(|image| image.source_parameter)
        .collect::<BTreeSet<_>>();
    if inventory != (1..=source_arity).collect() {
        return Err(SubstitutionError::InvalidSourceInventory {
            arity: source_arity,
        });
    }
    let mut canonical = images.to_vec();
    canonical.sort_by_key(|image| image.source_parameter);
    for image in &canonical {
        if !is_well_scoped(&image.term, target_arity) {
            return Err(SubstitutionError::ImageOutOfScope {
                source_parameter: image.source_parameter,
                target_arity,
            });
        }
    }
    Ok(canonical)
}

/// Apply a simultaneous parameter substitution.  Parameter levels
/// `1..=source_arity` are replaced; rigid binder levels are rebased from the
/// source parameter prefix onto the target parameter prefix.
pub fn apply_parallel_substitution(
    body: &Expr,
    source_arity: u32,
    target_arity: u32,
    images: &[SubstitutionImage],
) -> Result<Expr, SubstitutionError> {
    if !is_well_scoped(body, source_arity) {
        return Err(SubstitutionError::BodyOutOfScope { source_arity });
    }
    let images = canonical_images(source_arity, target_arity, images)?;
    Ok(substitute_at_depth(
        body,
        source_arity,
        target_arity,
        &images,
        0,
    ))
}

fn substitute_at_depth(
    expr: &Expr,
    source_arity: u32,
    target_arity: u32,
    images: &[SubstitutionImage],
    depth: u32,
) -> Expr {
    match expr {
        Expr::Var(level) if *level <= source_arity => {
            let image = &images[(*level - 1) as usize].term;
            lift_image_binders(image, target_arity, depth)
        }
        Expr::Var(level) => Expr::Var(target_arity + (*level - source_arity)),
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(left, right) => Expr::App(
            Box::new(substitute_at_depth(
                left,
                source_arity,
                target_arity,
                images,
                depth,
            )),
            Box::new(substitute_at_depth(
                right,
                source_arity,
                target_arity,
                images,
                depth,
            )),
        ),
        Expr::Pi(domain, codomain) => Expr::Pi(
            Box::new(substitute_at_depth(
                domain,
                source_arity,
                target_arity,
                images,
                depth,
            )),
            Box::new(substitute_at_depth(
                codomain,
                source_arity,
                target_arity,
                images,
                depth + 1,
            )),
        ),
        Expr::Sigma(domain, codomain) => Expr::Sigma(
            Box::new(substitute_at_depth(
                domain,
                source_arity,
                target_arity,
                images,
                depth,
            )),
            Box::new(substitute_at_depth(
                codomain,
                source_arity,
                target_arity,
                images,
                depth + 1,
            )),
        ),
        Expr::Lam(body) => Expr::Lam(Box::new(substitute_at_depth(
            body,
            source_arity,
            target_arity,
            images,
            depth + 1,
        ))),
        Expr::Id(ty, left, right) => Expr::Id(
            Box::new(substitute_at_depth(
                ty,
                source_arity,
                target_arity,
                images,
                depth,
            )),
            Box::new(substitute_at_depth(
                left,
                source_arity,
                target_arity,
                images,
                depth,
            )),
            Box::new(substitute_at_depth(
                right,
                source_arity,
                target_arity,
                images,
                depth,
            )),
        ),
        Expr::Refl(inner) => Expr::Refl(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Susp(inner) => Expr::Susp(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Flat(inner) => Expr::Flat(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Disc(inner) => Expr::Disc(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Shape(inner) => Expr::Shape(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Next(inner) => Expr::Next(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::Bang(inner) => Expr::Bang(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(substitute_at_depth(
            inner,
            source_arity,
            target_arity,
            images,
            depth,
        ))),
    }
}

/// Reposition binders internal to an image under `extra_depth` surrounding
/// binders.  Free target parameters stay fixed.
fn lift_image_binders(expr: &Expr, target_arity: u32, extra_depth: u32) -> Expr {
    if extra_depth == 0 {
        return expr.clone();
    }
    match expr {
        Expr::Var(level) if *level > target_arity => Expr::Var(level + extra_depth),
        Expr::Var(level) => Expr::Var(*level),
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(left, right) => Expr::App(
            Box::new(lift_image_binders(left, target_arity, extra_depth)),
            Box::new(lift_image_binders(right, target_arity, extra_depth)),
        ),
        Expr::Pi(domain, codomain) => Expr::Pi(
            Box::new(lift_image_binders(domain, target_arity, extra_depth)),
            Box::new(lift_image_binders(codomain, target_arity, extra_depth)),
        ),
        Expr::Sigma(domain, codomain) => Expr::Sigma(
            Box::new(lift_image_binders(domain, target_arity, extra_depth)),
            Box::new(lift_image_binders(codomain, target_arity, extra_depth)),
        ),
        Expr::Lam(body) => Expr::Lam(Box::new(lift_image_binders(
            body,
            target_arity,
            extra_depth,
        ))),
        Expr::Id(ty, left, right) => Expr::Id(
            Box::new(lift_image_binders(ty, target_arity, extra_depth)),
            Box::new(lift_image_binders(left, target_arity, extra_depth)),
            Box::new(lift_image_binders(right, target_arity, extra_depth)),
        ),
        Expr::Refl(inner) => Expr::Refl(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Susp(inner) => Expr::Susp(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Flat(inner) => Expr::Flat(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Disc(inner) => Expr::Disc(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Shape(inner) => Expr::Shape(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Next(inner) => Expr::Next(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::Bang(inner) => Expr::Bang(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(lift_image_binders(
            inner,
            target_arity,
            extra_depth,
        ))),
    }
}

pub fn issue_structural_substitution(
    source: SortedParameterContext,
    target: SortedParameterContext,
    images: Vec<SubstitutionImage>,
    body: Expr,
) -> Result<StructuralSubstitutionToken, SubstitutionError> {
    let images = canonical_images(source.arity(), target.arity(), &images)?;
    let result = apply_parallel_substitution(&body, source.arity(), target.arity(), &images)?;
    let constructor_coverage = EXPR_CONSTRUCTOR_COVERAGE
        .iter()
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    let derivation_hash = tagged_hash(
        "structural-parallel-substitution",
        &(
            &source,
            &target,
            &images,
            &body,
            &result,
            &constructor_coverage,
        ),
    );
    Ok(StructuralSubstitutionToken {
        source,
        target,
        images,
        body,
        result,
        constructor_coverage,
        derivation_hash,
    })
}

pub fn replay_structural_substitution(
    token: &StructuralSubstitutionToken,
) -> Result<(), SubstitutionError> {
    let replay = issue_structural_substitution(
        token.source.clone(),
        token.target.clone(),
        token.images.clone(),
        token.body.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(SubstitutionError::ReplayMismatch)
    }
}

pub fn issue_sort_preserving_substitution(
    source: SortedParameterContext,
    target: SortedParameterContext,
    images: Vec<SubstitutionImage>,
    body: Expr,
) -> Result<SortPreservingSubstitutionToken, SubstitutionError> {
    let structural = issue_structural_substitution(source, target, images, body)?;
    let mut equalities = Vec::with_capacity(structural.images.len());
    for image in &structural.images {
        let Expr::Var(target_parameter) = image.term else {
            return Err(SubstitutionError::NonVariableImageNeedsTypedJudgement {
                source_parameter: image.source_parameter,
            });
        };
        let source_sort = structural
            .source
            .sort(image.source_parameter)
            .expect("canonical source inventory is in scope");
        let Some(target_sort) = structural.target.sort(target_parameter) else {
            return Err(SubstitutionError::TargetParameterOutOfScope {
                source_parameter: image.source_parameter,
                target_parameter,
            });
        };
        if source_sort != target_sort {
            return Err(SubstitutionError::SortMismatch {
                source_parameter: image.source_parameter,
                source_sort,
                target_parameter,
                target_sort,
            });
        }
        equalities.push((
            image.source_parameter,
            source_sort,
            target_parameter,
            target_sort,
        ));
    }
    let scope = SORT_PRESERVATION_SCOPE.to_owned();
    let derivation_hash = tagged_hash(
        "sort-preserving-variable-substitution",
        &(&structural, &scope, &equalities),
    );
    Ok(SortPreservingSubstitutionToken {
        structural,
        scope,
        image_sort_equalities: equalities,
        derivation_hash,
    })
}

pub fn replay_sort_preserving_substitution(
    token: &SortPreservingSubstitutionToken,
) -> Result<(), SubstitutionError> {
    let replay = issue_sort_preserving_substitution(
        token.structural.source.clone(),
        token.structural.target.clone(),
        token.structural.images.clone(),
        token.structural.body.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(SubstitutionError::ReplayMismatch)
    }
}

pub fn issue_variable_image_support_preservation(
    token: &SortPreservingSubstitutionToken,
) -> VariableImageSupportToken {
    let source_library_support = token
        .structural
        .body
        .lib_refs()
        .into_iter()
        .collect::<Vec<_>>();
    let result_library_support = token
        .structural
        .result
        .lib_refs()
        .into_iter()
        .collect::<Vec<_>>();
    let no_image_contains_library_leaf = token
        .structural
        .images
        .iter()
        .all(|image| image.term.lib_refs().is_empty());
    let support_preserved_exactly =
        no_image_contains_library_leaf && source_library_support == result_library_support;
    let substitution_derivation_hash = token.derivation_hash.clone();
    let derivation_hash = tagged_hash(
        "variable-image-library-support-preservation",
        &(
            &substitution_derivation_hash,
            &source_library_support,
            &result_library_support,
            no_image_contains_library_leaf,
            support_preserved_exactly,
        ),
    );
    VariableImageSupportToken {
        substitution_derivation_hash,
        source_library_support,
        result_library_support,
        no_image_contains_library_leaf,
        support_preserved_exactly,
        derivation_hash,
    }
}

pub fn replay_variable_image_support_preservation(
    substitution: &SortPreservingSubstitutionToken,
    token: &VariableImageSupportToken,
) -> Result<(), SubstitutionError> {
    let replay = issue_variable_image_support_preservation(substitution);
    if replay == *token {
        Ok(())
    } else {
        Err(SubstitutionError::ReplayMismatch)
    }
}

pub fn identity_sort_preserving_substitution(
    context: SortedParameterContext,
    body: Expr,
) -> Result<SortPreservingSubstitutionToken, SubstitutionError> {
    let images = (1..=context.arity())
        .map(|source_parameter| SubstitutionImage {
            source_parameter,
            term: Expr::Var(source_parameter),
        })
        .collect();
    issue_sort_preserving_substitution(context.clone(), context, images, body)
}

/// Compose two already-issued restricted sort-preserving substitutions.
pub fn compose_sort_preserving_substitutions(
    first: &SortPreservingSubstitutionToken,
    second: &SortPreservingSubstitutionToken,
    body: Expr,
) -> Result<SortPreservingSubstitutionToken, SubstitutionError> {
    if first.structural.target != second.structural.source {
        return Err(SubstitutionError::CompositionContextMismatch);
    }
    let composed_images = first
        .structural
        .images
        .iter()
        .map(|image| {
            apply_parallel_substitution(
                &image.term,
                second.structural.source.arity(),
                second.structural.target.arity(),
                &second.structural.images,
            )
            .map(|term| SubstitutionImage {
                source_parameter: image.source_parameter,
                term,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let sequential_first = apply_parallel_substitution(
        &body,
        first.structural.source.arity(),
        first.structural.target.arity(),
        &first.structural.images,
    )?;
    let sequential_result = apply_parallel_substitution(
        &sequential_first,
        second.structural.source.arity(),
        second.structural.target.arity(),
        &second.structural.images,
    )?;
    let composed = issue_sort_preserving_substitution(
        first.structural.source.clone(),
        second.structural.target.clone(),
        composed_images,
        body,
    )?;
    if composed.result() != &sequential_result {
        return Err(SubstitutionError::CompositionResultMismatch);
    }
    Ok(composed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_constructors() -> Expr {
        let unary_chain = Expr::PathCon(1);
        let unary_chain = Expr::WhyNot(Box::new(unary_chain));
        let unary_chain = Expr::Bang(Box::new(unary_chain));
        let unary_chain = Expr::Eventually(Box::new(unary_chain));
        let unary_chain = Expr::Next(Box::new(unary_chain));
        let unary_chain = Expr::Shape(Box::new(unary_chain));
        let unary_chain = Expr::Disc(Box::new(unary_chain));
        let unary_chain = Expr::Sharp(Box::new(unary_chain));
        let unary_chain = Expr::Flat(Box::new(unary_chain));
        let dependent_body = Expr::Pi(
            Box::new(Expr::Sigma(Box::new(Expr::Univ), Box::new(Expr::Var(3)))),
            Box::new(Expr::Id(
                Box::new(Expr::Lib(1)),
                Box::new(Expr::Refl(Box::new(Expr::Var(1)))),
                Box::new(Expr::Susp(Box::new(Expr::Trunc(Box::new(Expr::Var(2)))))),
            )),
        );
        Expr::App(
            Box::new(Expr::Lam(Box::new(dependent_body))),
            Box::new(unary_chain),
        )
    }

    #[test]
    fn structural_substitution_covers_every_expr_constructor_and_replays() {
        let source = SortedParameterContext::new(vec![ParameterSort::Type, ParameterSort::Opaque]);
        let target = source.clone();
        let images = vec![
            SubstitutionImage {
                source_parameter: 1,
                term: Expr::Var(1),
            },
            SubstitutionImage {
                source_parameter: 2,
                term: Expr::Var(2),
            },
        ];
        let token = issue_structural_substitution(source, target, images, all_constructors())
            .expect("full grammar term is scoped");
        assert_eq!(token.result(), &all_constructors());
        assert_eq!(
            token.constructor_coverage.len(),
            EXPR_CONSTRUCTOR_COVERAGE.len()
        );
        replay_structural_substitution(&token).expect("token replays");
    }

    #[test]
    fn parallel_substitution_is_capture_safe_under_nested_binders() {
        let body = Expr::Lam(Box::new(Expr::Var(1)));
        let image = Expr::Lam(Box::new(Expr::Var(2)));
        let result = apply_parallel_substitution(
            &body,
            1,
            1,
            &[SubstitutionImage {
                source_parameter: 1,
                term: image,
            }],
        )
        .expect("scoped");
        assert_eq!(
            result,
            Expr::Lam(Box::new(Expr::Lam(Box::new(Expr::Var(3)))))
        );
    }

    #[test]
    fn mixed_sort_image_is_rejected() {
        let error = issue_sort_preserving_substitution(
            SortedParameterContext::new(vec![ParameterSort::Type]),
            SortedParameterContext::new(vec![ParameterSort::Opaque]),
            vec![SubstitutionImage {
                source_parameter: 1,
                term: Expr::Var(1),
            }],
            Expr::Var(1),
        )
        .unwrap_err();
        assert!(matches!(error, SubstitutionError::SortMismatch { .. }));
    }

    #[test]
    fn nonvariable_specialization_is_a_named_gap_not_typed_evidence() {
        let error = issue_sort_preserving_substitution(
            SortedParameterContext::all_type(1),
            SortedParameterContext::all_type(1),
            vec![SubstitutionImage {
                source_parameter: 1,
                term: Expr::Next(Box::new(Expr::Var(1))),
            }],
            Expr::Var(1),
        )
        .unwrap_err();
        assert_eq!(
            error,
            SubstitutionError::NonVariableImageNeedsTypedJudgement {
                source_parameter: 1
            }
        );
    }

    #[test]
    fn identity_and_composition_replay() {
        let context = SortedParameterContext::new(vec![
            ParameterSort::Type,
            ParameterSort::Opaque,
            ParameterSort::Type,
        ]);
        let body = Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(3)));
        let identity = identity_sort_preserving_substitution(context.clone(), body.clone())
            .expect("identity is sorted");
        assert_eq!(identity.result(), &body);
        replay_sort_preserving_substitution(&identity).expect("identity replays");

        let swap = issue_sort_preserving_substitution(
            context.clone(),
            context.clone(),
            vec![
                SubstitutionImage {
                    source_parameter: 1,
                    term: Expr::Var(3),
                },
                SubstitutionImage {
                    source_parameter: 2,
                    term: Expr::Var(2),
                },
                SubstitutionImage {
                    source_parameter: 3,
                    term: Expr::Var(1),
                },
            ],
            body.clone(),
        )
        .expect("equal-sort swap");
        let composed = compose_sort_preserving_substitutions(&swap, &swap, body.clone())
            .expect("composition is sorted");
        assert_eq!(composed.result(), &body);
        replay_sort_preserving_substitution(&composed).expect("composition replays");
    }

    #[test]
    fn composition_agrees_with_sequential_application_over_the_full_expr_grammar() {
        let context = SortedParameterContext::all_type(2);
        let swap = vec![
            SubstitutionImage {
                source_parameter: 1,
                term: Expr::Var(2),
            },
            SubstitutionImage {
                source_parameter: 2,
                term: Expr::Var(1),
            },
        ];
        let body = all_constructors();
        let first = issue_sort_preserving_substitution(
            context.clone(),
            context.clone(),
            swap.clone(),
            body.clone(),
        )
        .expect("first full-grammar substitution");
        let second = issue_sort_preserving_substitution(
            context.clone(),
            context,
            swap,
            first.result().clone(),
        )
        .expect("second full-grammar substitution");
        let composed = compose_sort_preserving_substitutions(&first, &second, body.clone())
            .expect("composition theorem checks sequential equality");
        assert_eq!(composed.result(), &body);
    }

    #[test]
    fn mutation_of_private_witness_data_is_detected_by_definition_replay() {
        let mut token = identity_sort_preserving_substitution(
            SortedParameterContext::all_type(1),
            Expr::Var(1),
        )
        .expect("identity");
        token.derivation_hash.push('0');
        assert_eq!(
            replay_sort_preserving_substitution(&token),
            Err(SubstitutionError::ReplayMismatch)
        );
    }

    #[test]
    fn variable_images_preserve_library_support_exactly() {
        let substitution = identity_sort_preserving_substitution(
            SortedParameterContext::all_type(1),
            Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
        )
        .expect("identity");
        let support = issue_variable_image_support_preservation(&substitution);
        assert!(support.support_preserved_exactly());
        replay_variable_image_support_preservation(&substitution, &support)
            .expect("support theorem replays");
    }
}
