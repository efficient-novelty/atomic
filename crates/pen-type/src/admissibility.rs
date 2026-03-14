use crate::obligations::summarize_structural_debt;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::{Telescope, TelescopeClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StrictAdmissibility {
    pub min_clause_kappa: u16,
    pub max_clause_kappa: u16,
    pub ambient_depth: u32,
    pub max_expr_nodes: u8,
    pub max_path_dimension: u32,
    pub include_trunc: bool,
    pub include_modal: bool,
    pub include_temporal: bool,
    pub quota_per_bucket: usize,
    pub require_former_eliminator_package: bool,
    pub require_initial_hit_package: bool,
    pub require_truncation_hit_package: bool,
    pub require_higher_hit_package: bool,
    pub require_sphere_lift_package: bool,
    pub require_axiomatic_bundle_package: bool,
    pub require_modal_shell_package: bool,
    pub require_connection_shell_package: bool,
    pub require_curvature_shell_package: bool,
    pub require_operator_bundle_package: bool,
    pub require_hilbert_functional_package: bool,
    pub require_temporal_shell_package: bool,
    pub historical_anchor_ref: Option<u32>,
}

impl StrictAdmissibility {
    pub fn supports_exact_clause_kappa(self, clause_kappa: u16) -> bool {
        (self.min_clause_kappa..=self.max_clause_kappa).contains(&clause_kappa)
    }
}

pub fn passes_strict_admissibility(
    step_index: u32,
    library: &Library,
    telescope: &Telescope,
    admissibility: StrictAdmissibility,
) -> bool {
    if !admissibility.supports_exact_clause_kappa(telescope.kappa() as u16) {
        return false;
    }

    match step_index {
        1 => {
            telescope.kappa() == 2
                && telescope
                    .clauses
                    .iter()
                    .any(|clause| matches!(clause.expr, Expr::Univ))
                && telescope
                    .clauses
                    .iter()
                    .any(|clause| is_universe_application(&clause.expr))
        }
        2 => {
            telescope.kappa() == 1
                && telescope
                    .clauses
                    .iter()
                    .all(|clause| is_universe_application(&clause.expr))
        }
        3 => {
            telescope.kappa() == 1
                && telescope.clauses.iter().all(|clause| {
                    is_latest_library_witness_application(&clause.expr, library.len() as u32)
                })
        }
        _ => {
            !telescope.is_trivially_derivable(library)
                && (!admissibility.require_former_eliminator_package
                    || matches_former_eliminator_package(telescope))
                && (!admissibility.require_initial_hit_package
                    || matches_initial_hit_package(telescope))
                && (!admissibility.require_truncation_hit_package
                    || matches_truncation_hit_package(telescope))
                && (!admissibility.require_higher_hit_package
                    || matches_higher_hit_package(telescope))
                && (!admissibility.require_sphere_lift_package
                    || matches_sphere_lift_package(telescope))
                && (!admissibility.require_axiomatic_bundle_package
                    || matches_axiomatic_bundle_package(
                        library,
                        telescope,
                        admissibility.historical_anchor_ref,
                    ))
                && (!admissibility.require_modal_shell_package
                    || matches_modal_shell_package(telescope))
                && (!admissibility.require_connection_shell_package
                    || matches_connection_shell_package(library, telescope))
                && (!admissibility.require_curvature_shell_package
                    || matches_curvature_shell_package(library, telescope))
                && (!admissibility.require_operator_bundle_package
                    || matches_operator_bundle_package(library, telescope))
                && (!admissibility.require_hilbert_functional_package
                    || matches_hilbert_functional_package(library, telescope))
                && (!admissibility.require_temporal_shell_package
                    || matches_temporal_shell_package(
                        library,
                        telescope,
                        admissibility.historical_anchor_ref,
                    ))
        }
    }
}

pub fn strict_admissibility(
    step_index: u32,
    window_depth: u16,
    library: &Library,
) -> StrictAdmissibility {
    match step_index {
        1 => StrictAdmissibility {
            min_clause_kappa: 2,
            max_clause_kappa: 2,
            ambient_depth: 1,
            max_expr_nodes: 3,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            quota_per_bucket: 16,
            require_former_eliminator_package: false,
            require_initial_hit_package: false,
            require_truncation_hit_package: false,
            require_higher_hit_package: false,
            require_sphere_lift_package: false,
            require_axiomatic_bundle_package: false,
            require_modal_shell_package: false,
            require_connection_shell_package: false,
            require_curvature_shell_package: false,
            require_operator_bundle_package: false,
            require_hilbert_functional_package: false,
            require_temporal_shell_package: false,
            historical_anchor_ref: None,
        },
        2 => StrictAdmissibility {
            min_clause_kappa: 1,
            max_clause_kappa: 1,
            ambient_depth: 1,
            max_expr_nodes: 3,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            quota_per_bucket: 16,
            require_former_eliminator_package: false,
            require_initial_hit_package: false,
            require_truncation_hit_package: false,
            require_higher_hit_package: false,
            require_sphere_lift_package: false,
            require_axiomatic_bundle_package: false,
            require_modal_shell_package: false,
            require_connection_shell_package: false,
            require_curvature_shell_package: false,
            require_operator_bundle_package: false,
            require_hilbert_functional_package: false,
            require_temporal_shell_package: false,
            historical_anchor_ref: None,
        },
        3 => StrictAdmissibility {
            min_clause_kappa: 1,
            max_clause_kappa: 1,
            ambient_depth: 1,
            max_expr_nodes: 3,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            quota_per_bucket: 16,
            require_former_eliminator_package: false,
            require_initial_hit_package: false,
            require_truncation_hit_package: false,
            require_higher_hit_package: false,
            require_sphere_lift_package: false,
            require_axiomatic_bundle_package: false,
            require_modal_shell_package: false,
            require_connection_shell_package: false,
            require_curvature_shell_package: false,
            require_operator_bundle_package: false,
            require_hilbert_functional_package: false,
            require_temporal_shell_package: false,
            historical_anchor_ref: None,
        },
        _ => {
            let debt = summarize_structural_debt(library, window_depth);
            let require_former_eliminator_package = debt.requires_former_eliminator_package();
            let require_initial_hit_package =
                !require_former_eliminator_package && debt.requires_initial_hit_package();
            let require_truncation_hit_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && debt.requires_truncation_hit_package();
            let require_higher_hit_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && debt.requires_higher_hit_package();
            let require_sphere_lift_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && debt.requires_sphere_lift_package();
            let derived_historical_anchor_ref = historical_loop_anchor_ref(library, window_depth);
            let derived_historical_modal_anchor_ref =
                historical_modal_shell_anchor_ref(library, window_depth);
            let require_axiomatic_bundle_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && !require_sphere_lift_package
                && debt.requires_axiomatic_bundle_package()
                && derived_historical_anchor_ref.is_some();
            let require_modal_shell_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && !require_sphere_lift_package
                && !require_axiomatic_bundle_package
                && debt.requires_modal_shell_package();
            let require_connection_shell_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && !require_sphere_lift_package
                && !require_axiomatic_bundle_package
                && !require_modal_shell_package
                && debt.requires_connection_shell_package();
            let require_curvature_shell_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && !require_sphere_lift_package
                && !require_axiomatic_bundle_package
                && !require_modal_shell_package
                && !require_connection_shell_package
                && debt.requires_curvature_shell_package();
            let require_operator_bundle_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && !require_sphere_lift_package
                && !require_axiomatic_bundle_package
                && !require_modal_shell_package
                && !require_connection_shell_package
                && !require_curvature_shell_package
                && debt.requires_operator_bundle_package();
            let require_hilbert_functional_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && !require_sphere_lift_package
                && !require_axiomatic_bundle_package
                && !require_modal_shell_package
                && !require_connection_shell_package
                && !require_curvature_shell_package
                && !require_operator_bundle_package
                && debt.requires_hilbert_functional_package();
            let require_temporal_shell_package = !require_former_eliminator_package
                && !require_initial_hit_package
                && !require_truncation_hit_package
                && !require_higher_hit_package
                && !require_sphere_lift_package
                && !require_axiomatic_bundle_package
                && !require_modal_shell_package
                && !require_connection_shell_package
                && !require_curvature_shell_package
                && !require_operator_bundle_package
                && !require_hilbert_functional_package
                && debt.requires_temporal_shell_package()
                && derived_historical_modal_anchor_ref.is_some();
            let historical_anchor_ref = if require_axiomatic_bundle_package {
                derived_historical_anchor_ref
            } else if require_temporal_shell_package {
                derived_historical_modal_anchor_ref
            } else {
                None
            };
            let min_clause_kappa = if require_former_eliminator_package {
                3
            } else if require_initial_hit_package {
                3
            } else if require_truncation_hit_package {
                3
            } else if require_higher_hit_package {
                3
            } else if require_sphere_lift_package {
                5
            } else if require_axiomatic_bundle_package {
                4
            } else if require_modal_shell_package {
                4
            } else if require_connection_shell_package {
                5
            } else if require_curvature_shell_package {
                5
            } else if require_operator_bundle_package {
                7
            } else if require_hilbert_functional_package {
                9
            } else if require_temporal_shell_package {
                8
            } else if debt.max_path_dimension > 0 || debt.has_modal_ops || debt.has_temporal_ops {
                2
            } else {
                1
            };
            let max_clause_kappa = if require_former_eliminator_package {
                3
            } else if require_initial_hit_package {
                3
            } else if require_truncation_hit_package {
                3
            } else if require_higher_hit_package {
                3
            } else if require_sphere_lift_package {
                5
            } else if require_axiomatic_bundle_package {
                4
            } else if require_modal_shell_package {
                4
            } else if require_connection_shell_package {
                5
            } else if require_curvature_shell_package {
                6
            } else if require_operator_bundle_package {
                7
            } else if require_hilbert_functional_package {
                9
            } else if require_temporal_shell_package {
                8
            } else {
                debt.exact_kappa_cap().max(min_clause_kappa).clamp(2, 9)
            };
            StrictAdmissibility {
                min_clause_kappa,
                max_clause_kappa,
                ambient_depth: 2,
                max_expr_nodes: if require_former_eliminator_package {
                    5
                } else if require_initial_hit_package {
                    3
                } else if require_truncation_hit_package {
                    4
                } else if require_higher_hit_package {
                    3
                } else if require_sphere_lift_package {
                    3
                } else if require_axiomatic_bundle_package {
                    4
                } else if require_modal_shell_package {
                    2
                } else if require_connection_shell_package {
                    5
                } else if require_curvature_shell_package {
                    5
                } else if require_operator_bundle_package {
                    7
                } else if require_hilbert_functional_package {
                    7
                } else if require_temporal_shell_package {
                    7
                } else if max_clause_kappa <= 3 {
                    5
                } else {
                    6
                },
                max_path_dimension: if require_former_eliminator_package {
                    0
                } else if require_initial_hit_package || require_truncation_hit_package {
                    1
                } else if require_higher_hit_package {
                    2
                } else if require_sphere_lift_package {
                    3
                } else if require_axiomatic_bundle_package {
                    0
                } else if require_modal_shell_package {
                    0
                } else if require_connection_shell_package {
                    0
                } else if require_curvature_shell_package {
                    0
                } else if require_operator_bundle_package {
                    0
                } else if require_hilbert_functional_package {
                    0
                } else if require_temporal_shell_package {
                    0
                } else {
                    u32::from(debt.max_path_dimension).max(1)
                },
                include_trunc: require_truncation_hit_package
                    || (!require_former_eliminator_package
                        && !require_initial_hit_package
                        && !require_higher_hit_package
                        && !require_sphere_lift_package
                        && !require_axiomatic_bundle_package
                        && !require_modal_shell_package
                        && !require_connection_shell_package
                        && !require_curvature_shell_package
                        && !require_operator_bundle_package
                        && !require_hilbert_functional_package
                        && !require_temporal_shell_package
                        && debt.max_path_dimension > 0),
                include_modal: if require_temporal_shell_package {
                    true
                } else if require_operator_bundle_package || require_hilbert_functional_package {
                    false
                } else {
                    require_modal_shell_package
                        || require_connection_shell_package
                        || require_curvature_shell_package
                        || debt.has_modal_ops
                },
                include_temporal: require_temporal_shell_package || debt.has_temporal_ops,
                quota_per_bucket: debt.quota_per_bucket(),
                require_former_eliminator_package,
                require_initial_hit_package,
                require_truncation_hit_package,
                require_higher_hit_package,
                require_sphere_lift_package,
                require_axiomatic_bundle_package,
                require_modal_shell_package,
                require_connection_shell_package,
                require_curvature_shell_package,
                require_operator_bundle_package,
                require_hilbert_functional_package,
                require_temporal_shell_package,
                historical_anchor_ref,
            }
        }
    }
}

fn is_universe_application(expr: &Expr) -> bool {
    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_)))
}

fn is_latest_library_witness_application(expr: &Expr, latest_library: u32) -> bool {
    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Lib(index) if *index == latest_library) && matches!(right.as_ref(), Expr::Var(_)))
}

fn matches_former_eliminator_package(telescope: &Telescope) -> bool {
    telescope.lib_refs().is_empty()
        && telescope.path_dimensions().is_empty()
        && telescope
            .clauses
            .iter()
            .any(|clause| contains_former_expr(&clause.expr))
        && telescope
            .clauses
            .iter()
            .any(|clause| contains_lambda_expr(&clause.expr))
        && telescope
            .clauses
            .iter()
            .any(|clause| contains_eliminator_expr(&clause.expr))
}

fn matches_initial_hit_package(telescope: &Telescope) -> bool {
    telescope.lib_refs().is_empty()
        && telescope.has_point_constructor()
        && telescope.path_dimensions() == vec![1]
}

fn matches_truncation_hit_package(telescope: &Telescope) -> bool {
    telescope.lib_refs().is_empty()
        && telescope.path_dimensions() == vec![1]
        && telescope
            .clauses
            .iter()
            .any(|clause| matches!(clause.expr, Expr::Trunc(_)))
        && telescope.clauses.iter().any(|clause| {
            matches!(
                &clause.expr,
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                        && matches!(argument.as_ref(), Expr::Var(2))
            )
        })
}

fn matches_higher_hit_package(telescope: &Telescope) -> bool {
    telescope.lib_refs().is_empty()
        && telescope.path_dimensions() == vec![2]
        && telescope.clauses.iter().any(|clause| {
            matches!(
                &clause.expr,
                Expr::App(left, right)
                    if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_))
            )
        })
        && telescope
            .clauses
            .iter()
            .any(|clause| matches!(clause.expr, Expr::Var(_)))
}

fn matches_sphere_lift_package(telescope: &Telescope) -> bool {
    telescope.lib_refs().is_empty()
        && telescope.path_dimensions() == vec![3]
        && telescope.clauses.iter().any(|clause| {
            matches!(
                &clause.expr,
                Expr::App(left, right)
                    if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_))
            )
        })
        && telescope
            .clauses
            .iter()
            .any(|clause| matches!(clause.expr, Expr::Var(_)))
        && telescope.clauses.iter().any(|clause| {
            matches!(&clause.expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1)))
        })
        && telescope.clauses.iter().any(|clause| {
            matches!(&clause.expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(2)))
        })
}

fn matches_axiomatic_bundle_package(
    library: &Library,
    telescope: &Telescope,
    historical_anchor_ref: Option<u32>,
) -> bool {
    let Some(anchor) = historical_anchor_ref else {
        return false;
    };
    let latest = library.len() as u32;
    if latest < 2 {
        return false;
    }
    let previous = latest - 1;

    telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 4
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == previous)
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                    && matches!(argument.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                            && matches!(argument.as_ref(), Expr::Lib(index) if *index == previous)
                )
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == previous)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        )
}

fn matches_modal_shell_package(telescope: &Telescope) -> bool {
    telescope.lib_refs().is_empty()
        && telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 4
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Flat(body) if matches!(body.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Sharp(body) if matches!(body.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Disc(body) if matches!(body.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::Shape(body) if matches!(body.as_ref(), Expr::Var(1))
        )
}

fn matches_connection_shell_package(library: &Library, telescope: &Telescope) -> bool {
    let latest = library.len() as u32;
    latest > 0
        && telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 5
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(
                        codomain.as_ref(),
                        Expr::Pi(inner_domain, inner_codomain)
                            if matches!(inner_domain.as_ref(), Expr::Var(1))
                                && matches!(inner_codomain.as_ref(), Expr::Var(1))
                    )
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(2))
                )
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(argument.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[4].expr,
            Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1))
        )
}

fn matches_curvature_shell_package(library: &Library, telescope: &Telescope) -> bool {
    let latest = library.len() as u32;
    latest > 0
        && telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 6
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(
                        codomain.as_ref(),
                        Expr::Pi(inner_domain, inner_codomain)
                            if matches!(inner_domain.as_ref(), Expr::Var(1))
                                && matches!(inner_codomain.as_ref(), Expr::Var(1))
                    )
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                            && matches!(argument.as_ref(), Expr::Var(1))
                )
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(
                        argument.as_ref(),
                        Expr::App(inner_function, inner_argument)
                            if matches!(inner_function.as_ref(), Expr::Var(1))
                                && matches!(inner_argument.as_ref(), Expr::Var(2))
                    )
        )
        && matches!(
            &telescope.clauses[4].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(2))
                )
        )
        && matches!(
            &telescope.clauses[5].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        )
}

fn matches_operator_bundle_package(library: &Library, telescope: &Telescope) -> bool {
    let latest = library.len() as u32;
    if latest < 2 {
        return false;
    }
    let previous = latest - 1;

    telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 7
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Sigma(left, right)
                if matches!(
                    left.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                ) && matches!(
                    right.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                )
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Sigma(left, right)
                        if matches!(left.as_ref(), Expr::Var(1))
                            && matches!(right.as_ref(), Expr::Var(2))
                ) && matches!(codomain.as_ref(), Expr::Lib(index) if *index == previous)
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Pi(inner_domain, inner_codomain)
                            if matches!(inner_domain.as_ref(), Expr::Var(1))
                                && matches!(inner_codomain.as_ref(), Expr::Var(1))
                    )
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Var(1))
                            && matches!(argument.as_ref(), Expr::Var(2))
                )
        )
        && matches!(
            &telescope.clauses[4].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        )
        && matches!(
            &telescope.clauses[5].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                )
        )
        && matches!(
            &telescope.clauses[6].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
}

fn matches_hilbert_functional_package(library: &Library, telescope: &Telescope) -> bool {
    let latest = library.len() as u32;
    if latest < 3 {
        return false;
    }
    let previous = latest - 1;
    let older = latest - 2;

    telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 9
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Sigma(left, right)
                if matches!(
                    left.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(
                                codomain.as_ref(),
                                Expr::Pi(inner_domain, inner_codomain)
                                    if matches!(inner_domain.as_ref(), Expr::Var(1))
                                        && matches!(inner_codomain.as_ref(), Expr::Univ)
                            )
                ) && matches!(right.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Sigma(left, right)
                            if matches!(left.as_ref(), Expr::Var(1))
                                && matches!(right.as_ref(), Expr::Var(1))
                    )
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1)))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Sigma(left, right)
                            if matches!(left.as_ref(), Expr::Var(1))
                                && matches!(right.as_ref(), Expr::Var(2))
                    )
        )
        && matches!(
            &telescope.clauses[4].expr,
            Expr::Sigma(left, right)
                if matches!(
                    left.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                ) && matches!(
                    right.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                )
        )
        && matches!(
            &telescope.clauses[5].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[6].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == previous)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[7].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == older)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[8].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Univ)
                )
        )
}

fn matches_temporal_shell_package(
    library: &Library,
    telescope: &Telescope,
    historical_anchor_ref: Option<u32>,
) -> bool {
    let Some(anchor) = historical_anchor_ref else {
        return false;
    };
    if !library.iter().any(|entry| entry.capabilities.has_hilbert) {
        return false;
    }

    telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 8
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
                    )
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                            && matches!(
                                argument.as_ref(),
                                Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                            )
                )
        )
        && matches!(
            &telescope.clauses[4].expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
        )
        && matches!(
            &telescope.clauses[5].expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Sharp(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Eventually(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
        )
        && matches!(
            &telescope.clauses[6].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        )
        && matches!(
            &telescope.clauses[7].expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
                )
        )
}

fn historical_loop_anchor_ref(library: &Library, window_depth: u16) -> Option<u32> {
    let depth = usize::from(window_depth.max(1));
    let cutoff = library.len().saturating_sub(depth);
    library
        .iter()
        .take(cutoff)
        .enumerate()
        .rev()
        .find_map(|(index, entry)| {
            (entry.has_loop && entry.constructors > 0 && entry.is_truncated.is_none())
                .then_some(index as u32 + 1)
        })
}

fn historical_modal_shell_anchor_ref(library: &Library, window_depth: u16) -> Option<u32> {
    let depth = usize::from(window_depth.max(1));
    let cutoff = library.len().saturating_sub(depth);
    library
        .iter()
        .take(cutoff)
        .enumerate()
        .rev()
        .find_map(|(index, entry)| {
            (entry.class == TelescopeClass::Modal && entry.capabilities.has_modal_ops)
                .then_some(index as u32 + 1)
        })
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
    use super::{StrictAdmissibility, passes_strict_admissibility, strict_admissibility};
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;

    fn library_until(step: u32) -> Library {
        let mut library = Vec::new();
        for current in 1..=step {
            let telescope = Telescope::reference(current);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }
        library
    }

    #[test]
    fn bootstrap_steps_keep_frozen_exact_clause_bands() {
        assert_eq!(
            strict_admissibility(1, 2, &Vec::new()),
            StrictAdmissibility {
                min_clause_kappa: 2,
                max_clause_kappa: 2,
                ambient_depth: 1,
                max_expr_nodes: 3,
                max_path_dimension: 0,
                include_trunc: false,
                include_modal: false,
                include_temporal: false,
                quota_per_bucket: 16,
                require_former_eliminator_package: false,
                require_initial_hit_package: false,
                require_truncation_hit_package: false,
                require_higher_hit_package: false,
                require_sphere_lift_package: false,
                require_axiomatic_bundle_package: false,
                require_modal_shell_package: false,
                require_connection_shell_package: false,
                require_curvature_shell_package: false,
                require_operator_bundle_package: false,
                require_hilbert_functional_package: false,
                require_temporal_shell_package: false,
                historical_anchor_ref: None,
            }
        );
        assert!(strict_admissibility(3, 2, &library_until(2)).supports_exact_clause_kappa(1));
    }

    #[test]
    fn later_steps_open_structural_space_from_active_window_debt() {
        let admissibility = strict_admissibility(7, 2, &library_until(6));
        assert!(admissibility.max_clause_kappa >= admissibility.min_clause_kappa);
        assert!(admissibility.max_path_dimension >= 2);
        assert!(!admissibility.require_former_eliminator_package);
    }

    #[test]
    fn first_former_package_opens_a_structural_step_four_band() {
        let library = library_until(3);
        let admissibility = strict_admissibility(4, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 3);
        assert_eq!(admissibility.max_clause_kappa, 3);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
    }

    #[test]
    fn first_hit_package_opens_a_structural_step_five_band() {
        let library = library_until(4);
        let admissibility = strict_admissibility(5, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 3);
        assert_eq!(admissibility.max_clause_kappa, 3);
        assert_eq!(admissibility.max_path_dimension, 1);
        assert!(admissibility.require_initial_hit_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_truncation_hit_package);
    }

    #[test]
    fn first_truncation_package_opens_a_structural_step_six_band() {
        let library = library_until(5);
        let admissibility = strict_admissibility(6, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 3);
        assert_eq!(admissibility.max_clause_kappa, 3);
        assert_eq!(admissibility.max_expr_nodes, 4);
        assert_eq!(admissibility.max_path_dimension, 1);
        assert!(admissibility.include_trunc);
        assert!(admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_higher_hit_package);
    }

    #[test]
    fn first_higher_hit_package_opens_a_structural_step_seven_band() {
        let library = library_until(6);
        let admissibility = strict_admissibility(7, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 3);
        assert_eq!(admissibility.max_clause_kappa, 3);
        assert_eq!(admissibility.max_expr_nodes, 3);
        assert_eq!(admissibility.max_path_dimension, 2);
        assert!(!admissibility.include_trunc);
        assert!(admissibility.require_higher_hit_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
    }

    #[test]
    fn first_sphere_lift_package_opens_a_structural_step_eight_band() {
        let library = library_until(7);
        let admissibility = strict_admissibility(8, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 5);
        assert_eq!(admissibility.max_clause_kappa, 5);
        assert_eq!(admissibility.max_expr_nodes, 3);
        assert_eq!(admissibility.max_path_dimension, 3);
        assert!(!admissibility.include_trunc);
        assert!(admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_axiomatic_bundle_package);
        assert!(!admissibility.require_modal_shell_package);
        assert!(!admissibility.require_connection_shell_package);
        assert!(!admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_operator_bundle_package);
        assert!(!admissibility.require_hilbert_functional_package);
        assert_eq!(admissibility.historical_anchor_ref, None);
    }

    #[test]
    fn first_axiomatic_bundle_package_opens_a_structural_step_nine_band() {
        let library = library_until(8);
        let admissibility = strict_admissibility(9, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 4);
        assert_eq!(admissibility.max_clause_kappa, 4);
        assert_eq!(admissibility.max_expr_nodes, 4);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(!admissibility.include_trunc);
        assert!(admissibility.require_axiomatic_bundle_package);
        assert_eq!(admissibility.historical_anchor_ref, Some(5));
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_modal_shell_package);
        assert!(!admissibility.require_connection_shell_package);
        assert!(!admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_operator_bundle_package);
        assert!(!admissibility.require_hilbert_functional_package);
    }

    #[test]
    fn first_modal_shell_package_opens_a_structural_step_ten_band() {
        let library = library_until(9);
        let admissibility = strict_admissibility(10, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 4);
        assert_eq!(admissibility.max_clause_kappa, 4);
        assert_eq!(admissibility.max_expr_nodes, 2);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(admissibility.include_modal);
        assert!(admissibility.require_modal_shell_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_axiomatic_bundle_package);
        assert!(!admissibility.require_connection_shell_package);
        assert!(!admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_operator_bundle_package);
        assert!(!admissibility.require_hilbert_functional_package);
        assert_eq!(admissibility.historical_anchor_ref, None);
    }

    #[test]
    fn first_connection_shell_package_opens_a_structural_step_eleven_band() {
        let library = library_until(10);
        let admissibility = strict_admissibility(11, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 5);
        assert_eq!(admissibility.max_clause_kappa, 5);
        assert_eq!(admissibility.max_expr_nodes, 5);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(admissibility.include_modal);
        assert!(admissibility.require_connection_shell_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_axiomatic_bundle_package);
        assert!(!admissibility.require_modal_shell_package);
        assert!(!admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_operator_bundle_package);
        assert!(!admissibility.require_hilbert_functional_package);
        assert_eq!(admissibility.historical_anchor_ref, None);
    }

    #[test]
    fn first_curvature_shell_package_opens_a_structural_step_twelve_band() {
        let library = library_until(11);
        let admissibility = strict_admissibility(12, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 5);
        assert_eq!(admissibility.max_clause_kappa, 6);
        assert_eq!(admissibility.max_expr_nodes, 5);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(admissibility.include_modal);
        assert!(!admissibility.include_trunc);
        assert!(admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_axiomatic_bundle_package);
        assert!(!admissibility.require_modal_shell_package);
        assert!(!admissibility.require_connection_shell_package);
        assert!(!admissibility.require_operator_bundle_package);
        assert!(!admissibility.require_hilbert_functional_package);
        assert_eq!(admissibility.historical_anchor_ref, None);
    }

    #[test]
    fn first_operator_bundle_package_opens_a_structural_step_thirteen_band() {
        let library = library_until(12);
        let admissibility = strict_admissibility(13, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 7);
        assert_eq!(admissibility.max_clause_kappa, 7);
        assert_eq!(admissibility.max_expr_nodes, 7);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(!admissibility.include_modal);
        assert!(!admissibility.include_trunc);
        assert!(admissibility.require_operator_bundle_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_axiomatic_bundle_package);
        assert!(!admissibility.require_modal_shell_package);
        assert!(!admissibility.require_connection_shell_package);
        assert!(!admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_hilbert_functional_package);
        assert_eq!(admissibility.historical_anchor_ref, None);
    }

    #[test]
    fn first_hilbert_functional_shell_opens_an_exact_step_fourteen_band() {
        let library = library_until(13);
        let admissibility = strict_admissibility(14, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 9);
        assert_eq!(admissibility.max_clause_kappa, 9);
        assert_eq!(admissibility.max_expr_nodes, 7);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(!admissibility.include_modal);
        assert!(!admissibility.include_trunc);
        assert!(admissibility.require_hilbert_functional_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_axiomatic_bundle_package);
        assert!(!admissibility.require_modal_shell_package);
        assert!(!admissibility.require_connection_shell_package);
        assert!(!admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_operator_bundle_package);
        assert_eq!(admissibility.historical_anchor_ref, None);
    }

    #[test]
    fn first_temporal_shell_opens_an_exact_step_fifteen_band() {
        let library = library_until(14);
        let admissibility = strict_admissibility(15, 2, &library);
        assert_eq!(admissibility.min_clause_kappa, 8);
        assert_eq!(admissibility.max_clause_kappa, 8);
        assert_eq!(admissibility.max_expr_nodes, 7);
        assert_eq!(admissibility.max_path_dimension, 0);
        assert!(admissibility.include_modal);
        assert!(admissibility.include_temporal);
        assert!(!admissibility.include_trunc);
        assert!(admissibility.require_temporal_shell_package);
        assert!(!admissibility.require_former_eliminator_package);
        assert!(!admissibility.require_initial_hit_package);
        assert!(!admissibility.require_truncation_hit_package);
        assert!(!admissibility.require_higher_hit_package);
        assert!(!admissibility.require_sphere_lift_package);
        assert!(!admissibility.require_axiomatic_bundle_package);
        assert!(!admissibility.require_modal_shell_package);
        assert!(!admissibility.require_connection_shell_package);
        assert!(!admissibility.require_curvature_shell_package);
        assert!(!admissibility.require_operator_bundle_package);
        assert!(!admissibility.require_hilbert_functional_package);
        assert_eq!(admissibility.historical_anchor_ref, Some(10));
    }

    #[test]
    fn bootstrap_admissibility_filters_match_the_current_targets() {
        assert!(passes_strict_admissibility(
            1,
            &Vec::new(),
            &Telescope::reference(1),
            strict_admissibility(1, 2, &Vec::new())
        ));
        assert!(passes_strict_admissibility(
            2,
            &library_until(1),
            &Telescope::reference(2),
            strict_admissibility(2, 2, &library_until(1))
        ));
        assert!(passes_strict_admissibility(
            3,
            &library_until(2),
            &Telescope::reference(3),
            strict_admissibility(3, 2, &library_until(2))
        ));
        let library = library_until(3);
        let admissibility = strict_admissibility(4, 2, &library);
        assert!(passes_strict_admissibility(
            4,
            &library,
            &Telescope::reference(4),
            admissibility
        ));
        let library = library_until(5);
        let admissibility = strict_admissibility(6, 2, &library);
        assert!(passes_strict_admissibility(
            6,
            &library,
            &Telescope::reference(6),
            admissibility
        ));
        let library = library_until(6);
        let admissibility = strict_admissibility(7, 2, &library);
        assert!(passes_strict_admissibility(
            7,
            &library,
            &Telescope::reference(7),
            admissibility
        ));
        let library = library_until(7);
        let admissibility = strict_admissibility(8, 2, &library);
        assert!(passes_strict_admissibility(
            8,
            &library,
            &Telescope::reference(8),
            admissibility
        ));
        let library = library_until(8);
        let admissibility = strict_admissibility(9, 2, &library);
        assert!(passes_strict_admissibility(
            9,
            &library,
            &Telescope::reference(9),
            admissibility
        ));
        let library = library_until(9);
        let admissibility = strict_admissibility(10, 2, &library);
        assert!(passes_strict_admissibility(
            10,
            &library,
            &Telescope::reference(10),
            admissibility
        ));
        let library = library_until(10);
        let admissibility = strict_admissibility(11, 2, &library);
        assert!(passes_strict_admissibility(
            11,
            &library,
            &Telescope::reference(11),
            admissibility
        ));
        let library = library_until(11);
        let admissibility = strict_admissibility(12, 2, &library);
        assert!(passes_strict_admissibility(
            12,
            &library,
            &Telescope::reference(12),
            admissibility
        ));
        let library = library_until(12);
        let admissibility = strict_admissibility(13, 2, &library);
        assert!(passes_strict_admissibility(
            13,
            &library,
            &Telescope::reference(13),
            admissibility
        ));
        let library = library_until(13);
        let admissibility = strict_admissibility(14, 2, &library);
        assert!(passes_strict_admissibility(
            14,
            &library,
            &Telescope::reference(14),
            admissibility
        ));
        let library = library_until(14);
        let admissibility = strict_admissibility(15, 2, &library);
        assert!(passes_strict_admissibility(
            15,
            &library,
            &Telescope::reference(15),
            admissibility
        ));
    }
}
