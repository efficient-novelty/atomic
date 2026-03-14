use crate::context::CheckContext;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckResult {
    Ok,
    Err(CheckError),
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CheckError {
    #[error("library reference {index} is out of bounds for library size {library_size}")]
    LibRefOutOfBounds { index: u32, library_size: usize },
    #[error("variable reference {index} is out of bounds for scope size {scope_size}")]
    VarRefOutOfBounds { index: u32, scope_size: u32 },
    #[error("ambient context depth {required} exceeds checker limit {max}")]
    AmbientContextTooLarge { required: u32, max: u32 },
    #[error("empty telescopes are not allowed")]
    EmptyTelescope,
    #[error("bare Univ cannot be used as an application argument")]
    BareUnivAsArgument,
}

impl CheckError {
    pub const fn kind_label(&self) -> &'static str {
        match self {
            Self::LibRefOutOfBounds { .. } => "lib_ref_out_of_bounds",
            Self::VarRefOutOfBounds { .. } => "var_ref_out_of_bounds",
            Self::AmbientContextTooLarge { .. } => "ambient_context_too_large",
            Self::EmptyTelescope => "empty_telescope",
            Self::BareUnivAsArgument => "bare_univ_as_argument",
        }
    }
}

pub fn check_telescope(library: &Library, telescope: &Telescope) -> CheckResult {
    if telescope.clauses.is_empty() {
        return CheckResult::Err(CheckError::EmptyTelescope);
    }

    let ambient_depth = required_ambient_depth(telescope);
    let max_ambient = 2;
    if ambient_depth > max_ambient {
        return CheckResult::Err(CheckError::AmbientContextTooLarge {
            required: ambient_depth,
            max: max_ambient,
        });
    }

    let mut clause_depth = 0;
    for clause in &telescope.clauses {
        let context = CheckContext {
            library_size: library.len(),
            ambient_depth,
            clause_depth,
        };
        if let CheckResult::Err(error) = check_expr(context, &clause.expr) {
            return CheckResult::Err(error);
        }
        clause_depth += 1;
    }

    CheckResult::Ok
}

pub fn check_and_filter(library: &Library, telescopes: Vec<Telescope>) -> (Vec<Telescope>, usize) {
    let mut valid = Vec::new();
    let mut rejected = 0;

    for telescope in telescopes {
        if check_telescope(library, &telescope) == CheckResult::Ok {
            valid.push(telescope);
        } else {
            rejected += 1;
        }
    }

    (valid, rejected)
}

fn check_expr(context: CheckContext, expr: &Expr) -> CheckResult {
    match expr {
        Expr::Lib(index) => {
            if *index == 0 || (*index as usize) > context.library_size {
                CheckResult::Err(CheckError::LibRefOutOfBounds {
                    index: *index,
                    library_size: context.library_size,
                })
            } else {
                CheckResult::Ok
            }
        }
        Expr::Var(index) => {
            if *index == 0 || *index > context.scope_size() {
                CheckResult::Err(CheckError::VarRefOutOfBounds {
                    index: *index,
                    scope_size: context.scope_size(),
                })
            } else {
                CheckResult::Ok
            }
        }
        Expr::Univ | Expr::PathCon(_) => CheckResult::Ok,
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            let domain_result = check_expr(context, domain);
            if domain_result != CheckResult::Ok {
                return domain_result;
            }

            let body_context = CheckContext {
                clause_depth: context.clause_depth + 1,
                ..context
            };
            check_expr(body_context, codomain)
        }
        Expr::Lam(body) => {
            let body_context = CheckContext {
                clause_depth: context.clause_depth + 1,
                ..context
            };
            check_expr(body_context, body)
        }
        Expr::App(_, argument) if matches!(argument.as_ref(), Expr::Univ) => {
            CheckResult::Err(CheckError::BareUnivAsArgument)
        }
        Expr::App(function, argument) => {
            let function_result = check_expr(context, function);
            if function_result != CheckResult::Ok {
                return function_result;
            }
            check_expr(context, argument)
        }
        Expr::Id(a, x, y) => {
            let type_result = check_expr(context, a);
            if type_result != CheckResult::Ok {
                return type_result;
            }
            let x_result = check_expr(context, x);
            if x_result != CheckResult::Ok {
                return x_result;
            }
            check_expr(context, y)
        }
        Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => {
            let body_context = CheckContext {
                clause_depth: context.clause_depth + 1,
                ..context
            };
            check_expr(body_context, body)
        }
    }
}

fn required_ambient_depth(telescope: &Telescope) -> u32 {
    telescope
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| required_ambient_expr(index as u32, &clause.expr))
        .max()
        .unwrap_or(0)
}

fn required_ambient_expr(clause_depth: u32, expr: &Expr) -> u32 {
    match expr {
        Expr::Var(index) => index.saturating_sub(clause_depth),
        Expr::Lib(_) | Expr::Univ | Expr::PathCon(_) => 0,
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => required_ambient_expr(clause_depth + 1, body),
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            required_ambient_expr(clause_depth, domain)
                .max(required_ambient_expr(clause_depth + 1, codomain))
        }
        Expr::App(function, argument) => required_ambient_expr(clause_depth, function)
            .max(required_ambient_expr(clause_depth, argument)),
        Expr::Id(a, x, y) => required_ambient_expr(clause_depth, a)
            .max(required_ambient_expr(clause_depth, x))
            .max(required_ambient_expr(clause_depth, y)),
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckError, CheckResult, check_telescope};
    use pen_core::library::Library;
    use pen_core::telescope::Telescope;

    #[test]
    fn bootstrap_reference_telescopes_pass_conservative_checks() {
        let library: Library = Vec::new();
        assert_eq!(
            check_telescope(&library, &Telescope::reference(1)),
            CheckResult::Ok
        );
        assert_eq!(
            check_telescope(&library, &Telescope::reference(2)),
            CheckResult::Ok
        );
    }

    #[test]
    fn unary_shell_reference_telescopes_preserve_scope() {
        let library: Library = Vec::new();
        assert_eq!(
            check_telescope(&library, &Telescope::reference(6)),
            CheckResult::Ok
        );
        assert_eq!(
            check_telescope(&library, &Telescope::reference(10)),
            CheckResult::Ok
        );
    }

    #[test]
    fn invalid_library_references_are_rejected() {
        let library: Library = Vec::new();
        let result = check_telescope(&library, &Telescope::reference(3));
        assert_eq!(
            result,
            CheckResult::Err(CheckError::LibRefOutOfBounds {
                index: 2,
                library_size: 0
            })
        );
    }
}
