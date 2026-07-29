use crate::manifest::OutsideFragmentReason;
use crate::model::GenericJudgmentV1;
use pen_kernel::{DependentContext, Term};

/// A total classification of syntax that is outside the lambda/unit surface.
///
/// The declaration order is intentional: when a term contains more than one
/// violation, descriptor projections take precedence over otherwise
/// unsupported constructors, which take precedence over unsupported universe
/// levels. This keeps the result independent of traversal and caller order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum LambdaUnitSyntaxViolation {
    UnsupportedUniverseLevel,
    UnsupportedTerm,
    DescriptorProjection,
}

impl LambdaUnitSyntaxViolation {
    pub(crate) fn outside_reason(self) -> OutsideFragmentReason {
        match self {
            Self::UnsupportedUniverseLevel => OutsideFragmentReason::UnsupportedUniverseLevel,
            Self::UnsupportedTerm => OutsideFragmentReason::UnsupportedTerm,
            Self::DescriptorProjection => OutsideFragmentReason::DescriptorProjection,
        }
    }
}

pub(crate) fn lambda_unit_context_syntax_violation(
    context: &DependentContext,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    context
        .0
        .iter()
        .filter_map(|term| lambda_unit_term_syntax_violation(term, universe_levels))
        .max()
}

pub(crate) fn lambda_unit_judgment_syntax_violation(
    judgment: &GenericJudgmentV1,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => [
            lambda_unit_context_syntax_violation(context, universe_levels),
            lambda_unit_term_syntax_violation(term, universe_levels),
            lambda_unit_term_syntax_violation(ty, universe_levels),
        ]
        .into_iter()
        .flatten()
        .max(),
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => [
            lambda_unit_context_syntax_violation(context, universe_levels),
            lambda_unit_term_syntax_violation(left, universe_levels),
            lambda_unit_term_syntax_violation(right, universe_levels),
            lambda_unit_term_syntax_violation(ty, universe_levels),
        ]
        .into_iter()
        .flatten()
        .max(),
    }
}

pub(crate) fn lambda_unit_term_syntax_violation(
    term: &Term,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    let mut violation = None;
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        match term {
            Term::Sort { level } if !universe_levels.contains(level) => {
                violation =
                    violation.max(Some(LambdaUnitSyntaxViolation::UnsupportedUniverseLevel));
            }
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => {}
            Term::Pi { parameter, body }
            | Term::Apply {
                function: parameter,
                argument: body,
            } => {
                pending.push(body);
                pending.push(parameter);
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                pending.push(body);
                pending.push(parameter_type);
            }
            Term::First { pair } | Term::Second { pair } => {
                violation = Some(LambdaUnitSyntaxViolation::DescriptorProjection);
                pending.push(pair);
            }
            Term::Sigma { parameter, body } => {
                violation = violation.max(Some(LambdaUnitSyntaxViolation::UnsupportedTerm));
                pending.push(body);
                pending.push(parameter);
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                violation = violation.max(Some(LambdaUnitSyntaxViolation::UnsupportedTerm));
                pending.push(second);
                pending.push(first);
                pending.push(sigma_type);
            }
        }
    }
    violation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_classification_is_recursive_and_deterministic() {
        let projected = Term::Lambda {
            parameter_type: Box::new(Term::UnitType),
            body: Box::new(Term::Apply {
                function: Box::new(Term::Global {
                    id: pen_kernel::GlobalId(pen_kernel::Digest::of_bytes(b"projection-scan")),
                }),
                argument: Box::new(Term::Second {
                    pair: Box::new(Term::Var { index: 0 }),
                }),
            }),
        };
        assert_eq!(
            lambda_unit_term_syntax_violation(&projected, &[0, 1]),
            Some(LambdaUnitSyntaxViolation::DescriptorProjection)
        );

        let plain_pair = Term::Pair {
            sigma_type: Box::new(Term::UnitType),
            first: Box::new(Term::Unit),
            second: Box::new(Term::Unit),
        };
        assert_eq!(
            lambda_unit_term_syntax_violation(&plain_pair, &[0, 1]),
            Some(LambdaUnitSyntaxViolation::UnsupportedTerm)
        );

        let plain_sigma = Term::Sigma {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        };
        assert_eq!(
            lambda_unit_term_syntax_violation(&plain_sigma, &[0, 1]),
            Some(LambdaUnitSyntaxViolation::UnsupportedTerm)
        );
    }
}
