use pen_core::atom::Atom;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::encode::expr_bit_length;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::passes_connectivity;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnumerationContext {
    pub library_size: u32,
    pub scope_size: u32,
    pub max_path_dimension: u32,
    pub include_trunc: bool,
    pub include_modal: bool,
    pub include_temporal: bool,
    pub max_expr_nodes: u8,
    pub require_former_eliminator_clauses: bool,
    pub require_initial_hit_clauses: bool,
    pub require_truncation_hit_clauses: bool,
    pub require_higher_hit_clauses: bool,
    pub require_sphere_lift_clauses: bool,
    pub require_axiomatic_bundle_clauses: bool,
    pub require_modal_shell_clauses: bool,
    pub require_connection_shell_clauses: bool,
    pub historical_anchor_ref: Option<u32>,
}

pub fn enumerate_next_clauses(context: EnumerationContext) -> Vec<ClauseRec> {
    enumerate_exprs(context)
        .into_iter()
        .filter(|expr| {
            (!context.require_former_eliminator_clauses || supports_former_eliminator_clause(expr))
                && (!context.require_initial_hit_clauses || supports_initial_hit_clause(expr))
                && (!context.require_truncation_hit_clauses || supports_truncation_hit_clause(expr))
                && (!context.require_higher_hit_clauses || supports_higher_hit_clause(expr))
                && (!context.require_sphere_lift_clauses || supports_sphere_lift_clause(expr))
                && (!context.require_axiomatic_bundle_clauses
                    || supports_axiomatic_bundle_clause(expr))
                && (!context.require_modal_shell_clauses || supports_modal_shell_clause(expr))
                && (!context.require_connection_shell_clauses
                    || supports_connection_shell_clause(expr))
        })
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect()
}

pub fn enumerate_telescopes(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
) -> Vec<Telescope> {
    let mut telescopes = Vec::new();
    let mut prefix = Vec::new();
    let clause_options_by_position = (0..usize::from(clause_kappa))
        .map(|position| {
            let clause_context = EnumerationContext {
                scope_size: base_context.scope_size + position as u32,
                ..base_context
            };
            let mut clauses = enumerate_next_clauses(clause_context);
            if base_context.require_former_eliminator_clauses {
                clauses.retain(|clause| {
                    supports_former_package_clause_at_position(position, &clause.expr)
                });
            }
            if base_context.require_initial_hit_clauses {
                clauses.retain(|clause| {
                    supports_initial_hit_clause_at_position(position, &clause.expr)
                });
            }
            if base_context.require_truncation_hit_clauses {
                clauses.retain(|clause| {
                    supports_truncation_hit_clause_at_position(position, &clause.expr)
                });
            }
            if base_context.require_higher_hit_clauses {
                clauses.retain(|clause| {
                    supports_higher_hit_clause_at_position(position, &clause.expr)
                });
            }
            if base_context.require_sphere_lift_clauses {
                clauses.retain(|clause| {
                    supports_sphere_lift_clause_at_position(position, &clause.expr)
                });
            }
            if base_context.require_axiomatic_bundle_clauses {
                clauses.retain(|clause| {
                    supports_axiomatic_bundle_clause_at_position(
                        position,
                        &clause.expr,
                        base_context.library_size,
                        base_context.historical_anchor_ref,
                    )
                });
            }
            if base_context.require_modal_shell_clauses {
                clauses.retain(|clause| {
                    supports_modal_shell_clause_at_position(position, &clause.expr)
                });
            }
            if base_context.require_connection_shell_clauses {
                clauses.retain(|clause| {
                    supports_connection_shell_clause_at_position(
                        position,
                        &clause.expr,
                        base_context.library_size,
                    )
                });
            }
            clauses
        })
        .collect::<Vec<_>>();
    enumerate_telescopes_dfs(
        library,
        clause_kappa,
        &clause_options_by_position,
        &mut prefix,
        &mut telescopes,
    );
    telescopes.sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
    telescopes
}

pub fn enumerate_exprs(context: EnumerationContext) -> Vec<Expr> {
    let mut cache = BTreeMap::new();
    let mut all = Vec::new();
    for nodes in 1..=context.max_expr_nodes {
        all.extend(enumerate_exprs_exact(context, nodes, &mut cache));
    }
    unique_sorted_exprs(all)
}

fn enumerate_telescopes_dfs(
    library: &Library,
    remaining: u16,
    clause_options_by_position: &[Vec<ClauseRec>],
    prefix: &mut Vec<ClauseRec>,
    out: &mut Vec<Telescope>,
) {
    if remaining == 0 {
        let telescope = Telescope::new(prefix.clone());
        if check_telescope(library, &telescope) == CheckResult::Ok
            && passes_connectivity(library, &telescope)
        {
            out.push(telescope);
        }
        return;
    }

    let position = prefix.len();
    for clause in &clause_options_by_position[position] {
        prefix.push(clause.clone());
        let partial = Telescope::new(prefix.clone());
        if check_telescope(library, &partial) == CheckResult::Ok {
            enumerate_telescopes_dfs(
                library,
                remaining - 1,
                clause_options_by_position,
                prefix,
                out,
            );
        }
        prefix.pop();
    }
}

fn enumerate_exprs_exact(
    context: EnumerationContext,
    nodes: u8,
    cache: &mut BTreeMap<u8, Vec<Expr>>,
) -> Vec<Expr> {
    if let Some(cached) = cache.get(&nodes) {
        return cached.clone();
    }

    let mut exprs = Vec::new();

    if nodes == 1 {
        exprs.push(Expr::Univ);
        for index in 1..=context.scope_size {
            exprs.push(Expr::Var(index));
        }
        if context.library_size > 0 {
            let start = context.library_size.saturating_sub(1).max(1);
            let mut refs = BTreeSet::new();
            for index in start..=context.library_size {
                refs.insert(index);
            }
            if let Some(anchor) = context.historical_anchor_ref {
                if (1..=context.library_size).contains(&anchor) {
                    refs.insert(anchor);
                }
            }
            for index in refs {
                exprs.push(Expr::Lib(index));
            }
        }
        for dimension in 1..=context.max_path_dimension {
            exprs.push(Expr::PathCon(dimension));
        }
    }

    if nodes >= 2 {
        let subexprs = enumerate_exprs_exact(context, nodes - 1, cache);
        for body in subexprs {
            exprs.push(Expr::Lam(Box::new(body.clone())));
            if context.include_trunc {
                exprs.push(Expr::Trunc(Box::new(body.clone())));
            }
            if context.include_modal {
                exprs.push(Expr::Flat(Box::new(body.clone())));
                exprs.push(Expr::Sharp(Box::new(body.clone())));
                exprs.push(Expr::Disc(Box::new(body.clone())));
                exprs.push(Expr::Shape(Box::new(body.clone())));
            }
            if context.include_temporal {
                exprs.push(Expr::Next(Box::new(body.clone())));
                exprs.push(Expr::Eventually(Box::new(body)));
            }
        }
    }

    if nodes >= 3 {
        for left_nodes in 1..=nodes - 2 {
            let right_nodes = nodes - 1 - left_nodes;
            let left_exprs = enumerate_exprs_exact(context, left_nodes, cache);
            let right_exprs = enumerate_exprs_exact(context, right_nodes, cache);
            for left in &left_exprs {
                for right in &right_exprs {
                    exprs.push(Expr::App(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Pi(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Sigma(Box::new(left.clone()), Box::new(right.clone())));
                }
            }
        }
    }

    let unique = unique_sorted_exprs(exprs);
    cache.insert(nodes, unique.clone());
    unique
}

fn unique_sorted_exprs(exprs: Vec<Expr>) -> Vec<Expr> {
    let mut keyed = BTreeMap::new();
    for expr in exprs {
        keyed.entry(expr_sort_key(&expr)).or_insert(expr);
    }
    keyed.into_values().collect()
}

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
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lib(_)) => {
            ClauseRole::Introduction
        }
        Expr::App(_, _) => ClauseRole::Introduction,
        Expr::Susp(_)
        | Expr::Trunc(_)
        | Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Lib(_) => ClauseRole::Formation,
    }
}

fn expr_sort_key(expr: &Expr) -> (u8, u32, String) {
    (
        atom_rank(expr.atom()),
        expr_bit_length(expr),
        serde_json::to_string(expr).expect("expr should serialize"),
    )
}

fn atom_rank(atom: Atom) -> u8 {
    atom as u8
}

fn supports_former_eliminator_clause(expr: &Expr) -> bool {
    !expr.has_lib_pointer()
        && !expr.is_modal()
        && !expr.is_temporal()
        && (contains_former_expr(expr)
            || contains_lambda_expr(expr)
            || contains_eliminator_expr(expr))
}

fn supports_initial_hit_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Var(_))
        || matches!(expr, Expr::PathCon(1))
}

fn supports_truncation_hit_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        || matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                    && matches!(argument.as_ref(), Expr::Var(2))
        )
        || matches!(expr, Expr::PathCon(1))
}

fn supports_higher_hit_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Var(_))
        || matches!(expr, Expr::PathCon(2))
}

fn supports_sphere_lift_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Var(_))
        || matches!(expr, Expr::PathCon(3))
        || matches!(
            expr,
            Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1) | Expr::Var(2))
        )
}

fn supports_axiomatic_bundle_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(domain, codomain)
        if matches!(domain.as_ref(), Expr::Lib(_)) && matches!(codomain.as_ref(), Expr::Lib(_)))
        || matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(_))
                    && matches!(argument.as_ref(), Expr::Var(_))
        )
        || matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(_))
                            && matches!(argument.as_ref(), Expr::Lib(_))
                )
        )
}

fn supports_modal_shell_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Flat(body) if matches!(body.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Sharp(body) if matches!(body.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Disc(body) if matches!(body.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Shape(body) if matches!(body.as_ref(), Expr::Var(_)))
}

fn supports_connection_shell_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && matches!(
                    codomain.as_ref(),
                    Expr::Pi(inner_domain, inner_codomain)
                        if matches!(inner_domain.as_ref(), Expr::Var(_))
                            && matches!(inner_codomain.as_ref(), Expr::Var(_))
                )
    ) || matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            ) || matches!(body.as_ref(), Expr::Var(_))
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                && matches!(codomain.as_ref(), Expr::Var(_))
    ) || matches!(
        expr,
        Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Lib(_))
                && matches!(argument.as_ref(), Expr::Var(_))
    )
}

fn supports_initial_hit_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => {
            matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(1)))
        }
        1 => matches!(expr, Expr::Var(1)),
        _ => matches!(expr, Expr::PathCon(1)),
    }
}

fn supports_former_package_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => is_minimal_former_intro_clause(expr),
        1 => is_multi_argument_eliminator_clause(expr),
        _ => is_beta_like_computation_clause(expr),
    }
}

fn supports_truncation_hit_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(1))),
        1 => matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                    && matches!(argument.as_ref(), Expr::Var(2))
        ),
        _ => matches!(expr, Expr::PathCon(1)),
    }
}

fn supports_higher_hit_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => {
            matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(1)))
        }
        1 => matches!(expr, Expr::Var(1)),
        _ => matches!(expr, Expr::PathCon(2)),
    }
}

fn supports_sphere_lift_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => {
            matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(1)))
        }
        1 => matches!(expr, Expr::Var(1)),
        2 => matches!(expr, Expr::PathCon(3)),
        3 => matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1))),
        _ => matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(2))),
    }
}

fn supports_axiomatic_bundle_clause_at_position(
    position: usize,
    expr: &Expr,
    library_size: u32,
    historical_anchor_ref: Option<u32>,
) -> bool {
    let Some(anchor) = historical_anchor_ref else {
        return false;
    };
    if library_size < 2 {
        return false;
    }

    let latest = library_size;
    let previous = latest - 1;

    match position {
        0 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == previous)
        ),
        1 => matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                    && matches!(argument.as_ref(), Expr::Var(1))
        ),
        2 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                            && matches!(argument.as_ref(), Expr::Lib(index) if *index == previous)
                )
        ),
        _ => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == previous)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        ),
    }
}

fn supports_modal_shell_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => matches!(expr, Expr::Flat(body) if matches!(body.as_ref(), Expr::Var(1))),
        1 => matches!(expr, Expr::Sharp(body) if matches!(body.as_ref(), Expr::Var(1))),
        2 => matches!(expr, Expr::Disc(body) if matches!(body.as_ref(), Expr::Var(1))),
        _ => matches!(expr, Expr::Shape(body) if matches!(body.as_ref(), Expr::Var(1))),
    }
}

fn supports_connection_shell_clause_at_position(
    position: usize,
    expr: &Expr,
    library_size: u32,
) -> bool {
    if library_size == 0 {
        return false;
    }

    let latest = library_size;
    match position {
        0 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(
                        codomain.as_ref(),
                        Expr::Pi(inner_domain, inner_codomain)
                            if matches!(inner_domain.as_ref(), Expr::Var(1))
                                && matches!(inner_codomain.as_ref(), Expr::Var(1))
                    )
        ),
        1 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(2))
                )
        ),
        2 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                    && matches!(codomain.as_ref(), Expr::Var(1))
        ),
        3 => matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(argument.as_ref(), Expr::Var(1))
        ),
        _ => matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1))),
    }
}

fn is_minimal_former_intro_clause(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(body) => {
            let has_forward_dependency = match body.as_ref() {
                Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
                    max_var_ref(domain) < max_var_ref(codomain)
                }
                _ => false,
            };
            contains_former_expr(body)
                && !contains_lambda_expr(body)
                && distinct_var_ref_count(expr) >= 2
                && has_forward_dependency
        }
        _ => false,
    }
}

fn is_multi_argument_eliminator_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::App(_, _))
        && !contains_former_expr(expr)
        && application_spine_len(expr) >= 3
        && distinct_var_ref_count(expr) >= 3
        && application_head_var(expr) == Some(1)
}

fn is_beta_like_computation_clause(expr: &Expr) -> bool {
    match expr {
        Expr::App(function, argument) => {
            matches!(argument.as_ref(), Expr::Var(index) if *index <= 2)
                && matches!(
                    function.as_ref(),
                    Expr::Lam(body)
                        if !contains_lambda_expr(body)
                            && !contains_former_expr(body)
                            && body.var_refs().len() == 1
                            && body.var_refs().contains(&1)
                )
                && distinct_var_ref_count(expr) >= 2
        }
        _ => false,
    }
}

fn application_spine_len(expr: &Expr) -> usize {
    match expr {
        Expr::App(function, _) => 1 + application_spine_len(function),
        _ => 1,
    }
}

fn distinct_var_ref_count(expr: &Expr) -> usize {
    expr.var_refs().len()
}

fn application_head_var(expr: &Expr) -> Option<u32> {
    match expr {
        Expr::App(function, _) => application_head_var(function),
        Expr::Var(index) => Some(*index),
        _ => None,
    }
}

fn max_var_ref(expr: &Expr) -> u32 {
    expr.var_refs().iter().next_back().copied().unwrap_or(0)
}

fn contains_former_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Pi(_, _) | Expr::Sigma(_, _) => true,
        Expr::App(left, right) => contains_former_expr(left) || contains_former_expr(right),
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => contains_former_expr(body),
        Expr::Id(ty, left, right) => {
            contains_former_expr(ty) || contains_former_expr(left) || contains_former_expr(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn contains_lambda_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            contains_lambda_expr(left) || contains_lambda_expr(right)
        }
        Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => contains_lambda_expr(body),
        Expr::Id(ty, left, right) => {
            contains_lambda_expr(ty) || contains_lambda_expr(left) || contains_lambda_expr(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn contains_eliminator_expr(expr: &Expr) -> bool {
    match expr {
        Expr::App(function, argument) => {
            matches!(function.as_ref(), Expr::Lam(_) | Expr::App(_, _))
                || contains_eliminator_expr(function)
                || contains_eliminator_expr(argument)
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => contains_eliminator_expr(body),
        Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            contains_eliminator_expr(left) || contains_eliminator_expr(right)
        }
        Expr::Id(ty, left, right) => {
            contains_eliminator_expr(ty)
                || contains_eliminator_expr(left)
                || contains_eliminator_expr(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EnumerationContext, enumerate_exprs, enumerate_next_clauses, enumerate_telescopes,
        supports_axiomatic_bundle_clause_at_position, supports_former_package_clause_at_position,
        supports_connection_shell_clause_at_position,
        supports_higher_hit_clause_at_position, supports_initial_hit_clause_at_position,
        supports_modal_shell_clause_at_position, supports_sphere_lift_clause_at_position,
        supports_truncation_hit_clause_at_position,
    };
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;
    use pen_type::admissibility::strict_admissibility;

    fn library_until(step: u32) -> Library {
        let mut library = Vec::new();
        for current in 1..=step {
            let telescope = Telescope::reference(current);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        library
    }

    #[test]
    fn enumeration_is_deterministic_and_window_limited() {
        let context = EnumerationContext {
            library_size: 3,
            scope_size: 1,
            max_path_dimension: 2,
            include_trunc: true,
            include_modal: true,
            include_temporal: true,
            max_expr_nodes: 3,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            historical_anchor_ref: None,
        };

        let first = enumerate_next_clauses(context);
        let second = enumerate_next_clauses(context);

        assert_eq!(first, second);
        assert!(
            first
                .iter()
                .any(|clause| clause.expr == pen_core::expr::Expr::Lib(3))
        );
        assert!(
            first
                .iter()
                .all(|clause| clause.expr != pen_core::expr::Expr::Lib(1))
        );
        assert!(
            first
                .iter()
                .any(|clause| clause.expr == pen_core::expr::Expr::PathCon(2))
        );
    }

    #[test]
    fn recursive_enumeration_reaches_nested_bootstrap_forms() {
        let exprs = enumerate_exprs(EnumerationContext {
            library_size: 0,
            scope_size: 2,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            max_expr_nodes: 4,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            historical_anchor_ref: None,
        });
        let rendered = exprs
            .iter()
            .map(|expr| serde_json::to_string(expr).expect("serialize"))
            .collect::<Vec<_>>();

        assert!(
            rendered
                .iter()
                .any(|expr| expr.contains("\"Lam\":{\"Pi\":"))
        );
    }

    #[test]
    fn telescope_enumeration_discovers_bootstrap_candidates() {
        let telescopes = enumerate_telescopes(
            &Vec::new(),
            EnumerationContext {
                library_size: 0,
                scope_size: 1,
                max_path_dimension: 0,
                include_trunc: false,
                include_modal: false,
                include_temporal: false,
                max_expr_nodes: 3,
                require_former_eliminator_clauses: false,
                require_initial_hit_clauses: false,
                require_truncation_hit_clauses: false,
                require_higher_hit_clauses: false,
                require_sphere_lift_clauses: false,
                require_axiomatic_bundle_clauses: false,
                require_modal_shell_clauses: false,
                require_connection_shell_clauses: false,
                historical_anchor_ref: None,
            },
            2,
        );

        assert!(
            telescopes
                .iter()
                .any(|telescope| *telescope == pen_core::telescope::Telescope::reference(1))
        );
    }

    #[test]
    fn former_eliminator_filter_preserves_nested_step_four_clauses() {
        let clauses = enumerate_next_clauses(EnumerationContext {
            library_size: 0,
            scope_size: 3,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            max_expr_nodes: 5,
            require_former_eliminator_clauses: true,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            historical_anchor_ref: None,
        });
        assert!(clauses.iter().any(|clause| {
            clause.expr
                == pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(2)),
                )))
        }));
    }

    #[test]
    fn step_four_positional_filters_accept_reference_eliminator_and_computation_clauses() {
        assert!(supports_former_package_clause_at_position(
            1,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::App(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(2)),
                )),
                Box::new(pen_core::expr::Expr::Var(3)),
            )
        ));
        assert!(supports_former_package_clause_at_position(
            2,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lam(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Var(2)),
            )
        ));
    }

    #[test]
    fn initial_hit_filters_accept_the_reference_point_and_path_package() {
        assert!(supports_initial_hit_clause_at_position(
            0,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Univ),
                Box::new(pen_core::expr::Expr::Var(1)),
            )
        ));
        assert!(supports_initial_hit_clause_at_position(
            1,
            &pen_core::expr::Expr::Var(1)
        ));
        assert!(supports_initial_hit_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(1)
        ));
    }

    #[test]
    fn truncation_hit_filters_accept_the_reference_shell_and_path_package() {
        assert!(supports_truncation_hit_clause_at_position(
            0,
            &pen_core::expr::Expr::Trunc(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_truncation_hit_clause_at_position(
            1,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Trunc(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Var(2)),
            )
        ));
        assert!(supports_truncation_hit_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(1)
        ));
    }

    #[test]
    fn higher_hit_filters_accept_the_reference_point_and_higher_path_package() {
        assert!(supports_higher_hit_clause_at_position(
            0,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Univ),
                Box::new(pen_core::expr::Expr::Var(1)),
            )
        ));
        assert!(supports_higher_hit_clause_at_position(
            1,
            &pen_core::expr::Expr::Var(1)
        ));
        assert!(supports_higher_hit_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(2)
        ));
    }

    #[test]
    fn sphere_lift_filters_accept_the_reference_point_path_and_witness_package() {
        assert!(supports_sphere_lift_clause_at_position(
            0,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Univ),
                Box::new(pen_core::expr::Expr::Var(1)),
            )
        ));
        assert!(supports_sphere_lift_clause_at_position(
            1,
            &pen_core::expr::Expr::Var(1)
        ));
        assert!(supports_sphere_lift_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(3)
        ));
        assert!(supports_sphere_lift_clause_at_position(
            3,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_sphere_lift_clause_at_position(
            4,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Var(2)))
        ));
    }

    #[test]
    fn axiomatic_bundle_filters_accept_the_reference_library_bundle() {
        assert!(supports_axiomatic_bundle_clause_at_position(
            0,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(8)),
                Box::new(pen_core::expr::Expr::Lib(7)),
            ),
            8,
            Some(5),
        ));
        assert!(supports_axiomatic_bundle_clause_at_position(
            1,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(5)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            8,
            Some(5),
        ));
        assert!(supports_axiomatic_bundle_clause_at_position(
            2,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(8)),
                Box::new(pen_core::expr::Expr::Lib(7)),
            ))),
            8,
            Some(5),
        ));
        assert!(supports_axiomatic_bundle_clause_at_position(
            3,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(7)),
                Box::new(pen_core::expr::Expr::Lib(8)),
            ),
            8,
            Some(5),
        ));
    }

    #[test]
    fn modal_shell_filters_accept_the_reference_modal_bundle() {
        assert!(supports_modal_shell_clause_at_position(
            0,
            &pen_core::expr::Expr::Flat(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_modal_shell_clause_at_position(
            1,
            &pen_core::expr::Expr::Sharp(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_modal_shell_clause_at_position(
            2,
            &pen_core::expr::Expr::Disc(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_modal_shell_clause_at_position(
            3,
            &pen_core::expr::Expr::Shape(Box::new(pen_core::expr::Expr::Var(1)))
        ));
    }

    #[test]
    fn connection_shell_filters_accept_the_reference_connection_bundle() {
        assert!(supports_connection_shell_clause_at_position(
            0,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(10)),
                Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(1)),
                )),
            ),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            1,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Var(2)),
            ))),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            2,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Flat(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            3,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(10)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            4,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Var(1))),
            10,
        ));
    }

    #[test]
    fn step_four_enumeration_contains_the_reference_former_package() {
        let library = library_until(3);
        let admissibility = strict_admissibility(4, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(4)),
            "enumerated {} step-4 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_five_enumeration_contains_the_reference_hit_package() {
        let library = library_until(4);
        let admissibility = strict_admissibility(5, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(5)),
            "enumerated {} step-5 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_six_enumeration_contains_the_reference_truncation_package() {
        let library = library_until(5);
        let admissibility = strict_admissibility(6, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(6)),
            "enumerated {} step-6 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_seven_enumeration_contains_the_reference_higher_hit_package() {
        let library = library_until(6);
        let admissibility = strict_admissibility(7, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(7)),
            "enumerated {} step-7 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_eight_enumeration_contains_the_reference_sphere_package() {
        let library = library_until(7);
        let admissibility = strict_admissibility(8, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(8)),
            "enumerated {} step-8 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_nine_enumeration_contains_the_reference_axiomatic_bundle() {
        let library = library_until(8);
        let admissibility = strict_admissibility(9, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(9)),
            "enumerated {} step-9 telescopes, but not the reference bundle",
            telescopes.len()
        );
    }

    #[test]
    fn step_ten_enumeration_contains_the_reference_modal_shell() {
        let library = library_until(9);
        let admissibility = strict_admissibility(10, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(10)),
            "enumerated {} step-10 telescopes, but not the reference modal shell",
            telescopes.len()
        );
    }

    #[test]
    fn step_eleven_enumeration_contains_the_reference_connection_shell() {
        let library = library_until(10);
        let admissibility = strict_admissibility(11, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility
                    .require_connection_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(11)),
            "enumerated {} step-11 telescopes, but not the reference connection shell",
            telescopes.len()
        );
    }
}
