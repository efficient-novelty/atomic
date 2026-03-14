use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::{Telescope, TelescopeClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectivityWitness {
    pub connected: bool,
    pub references_active_window: bool,
    pub self_contained: bool,
    pub max_lib_ref: u32,
    pub historical_reanchor: bool,
}

pub fn analyze_connectivity(library: &Library, telescope: &Telescope) -> ConnectivityWitness {
    ConnectivityWitness {
        connected: is_structurally_connected(telescope),
        references_active_window: telescope.references_window(library.len() as u32),
        self_contained: telescope.lib_refs().is_empty(),
        max_lib_ref: telescope.max_lib_ref(),
        historical_reanchor: allows_temporal_modal_reanchor(library, telescope),
    }
}

pub fn passes_connectivity(library: &Library, telescope: &Telescope) -> bool {
    let witness = analyze_connectivity(library, telescope);
    witness.connected
        && (witness.references_active_window
            || witness.self_contained
            || witness.historical_reanchor)
}

fn is_structurally_connected(telescope: &Telescope) -> bool {
    if telescope.clauses.len() <= 1 {
        return true;
    }

    (0..telescope.clauses.len() - 1).all(|i| {
        let has_var_edge = ((i + 1)..telescope.clauses.len()).any(|j| {
            let de_bruijn = (j - i) as u32;
            telescope.clauses[j].expr.var_refs().contains(&de_bruijn)
                || later_clause_depends_on(i, j, &telescope.clauses[j].expr, 0)
        });
        let has_lib_edge = telescope.clauses[i].expr.has_lib_pointer();
        let has_path_attachment = is_local_path_anchor(telescope, i)
            && ((i + 1)..telescope.clauses.len())
                .any(|j| matches!(telescope.clauses[j].expr, Expr::PathCon(_)));
        let has_higher_path_witness_attachment =
            matches!(telescope.clauses[i].expr, Expr::PathCon(dimension) if dimension > 1)
                && ((i + 1)..telescope.clauses.len()).any(|j| {
                    is_higher_path_witness_clause(&telescope.clauses[j].expr)
                });
        let has_higher_order_bridge_attachment =
            is_higher_order_bridge_clause(&telescope.clauses[i].expr)
                && ((i + 1)..telescope.clauses.len())
                    .any(|j| telescope.clauses[j].expr.has_lib_pointer());
        let has_operator_bundle_bridge_attachment =
            is_operator_action_clause(&telescope.clauses[i].expr)
                && (0..i).any(|j| is_operator_bundle_seed_clause(&telescope.clauses[j].expr))
                && ((i + 1)..telescope.clauses.len())
                    .any(|j| is_operator_bundle_closure_clause(&telescope.clauses[j].expr));

        has_var_edge
            || has_lib_edge
            || has_path_attachment
            || has_higher_path_witness_attachment
            || has_higher_order_bridge_attachment
            || has_operator_bundle_bridge_attachment
    })
}

fn later_clause_depends_on(
    earlier_index: usize,
    later_index: usize,
    expr: &Expr,
    binder_depth: usize,
) -> bool {
    match expr {
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            later_clause_depends_on(earlier_index, later_index, function, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, argument, binder_depth)
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
        | Expr::Eventually(body) => {
            later_clause_depends_on(earlier_index, later_index, body, binder_depth + 1)
        }
        Expr::Var(index) => {
            let index = *index as usize;
            index > binder_depth
                && later_index
                    .checked_sub(index - binder_depth)
                    .is_some_and(|dep| dep == earlier_index)
        }
        Expr::Id(ty, left, right) => {
            later_clause_depends_on(earlier_index, later_index, ty, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, left, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, right, binder_depth)
        }
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn is_local_path_anchor(telescope: &Telescope, clause_index: usize) -> bool {
    let expr = &telescope.clauses[clause_index].expr;
    (telescope.has_point_constructor() && !matches!(expr, Expr::PathCon(_)))
        || matches!(expr, Expr::Trunc(_))
        || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Trunc(_)))
}

fn is_higher_path_witness_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1) | Expr::Var(2)))
}

fn is_higher_order_bridge_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain)
                    if domain.var_refs().contains(&1) && codomain.var_refs().contains(&2)
            )
    )
}

fn is_operator_action_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Var(1))
                        && matches!(argument.as_ref(), Expr::Var(2))
            )
    )
}

fn is_operator_bundle_seed_clause(expr: &Expr) -> bool {
    matches!(
        expr,
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
}

fn is_operator_bundle_closure_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && (matches!(codomain.as_ref(), Expr::Lib(_))
                    || matches!(codomain.as_ref(), Expr::Var(1)))
    )
}

fn allows_temporal_modal_reanchor(library: &Library, telescope: &Telescope) -> bool {
    let Some(anchor) = latest_modal_shell_anchor_ref(library) else {
        return false;
    };

    library
        .last()
        .is_some_and(|entry| entry.capabilities.has_hilbert)
        && matches_temporal_shell_pattern(telescope, anchor)
}

fn latest_modal_shell_anchor_ref(library: &Library) -> Option<u32> {
    library.iter().enumerate().rev().find_map(|(index, entry)| {
        (entry.class == TelescopeClass::Modal && entry.capabilities.has_modal_ops)
            .then_some(index as u32 + 1)
    })
}

fn matches_temporal_shell_pattern(telescope: &Telescope, anchor: u32) -> bool {
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

#[cfg(test)]
mod tests {
    use super::{ConnectivityWitness, analyze_connectivity, passes_connectivity};
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
    fn connectivity_accepts_reference_witness_step() {
        let library = library_until(2);
        let witness = analyze_connectivity(&library, &Telescope::reference(3));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 2,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(3)));
    }

    #[test]
    fn connectivity_accepts_self_contained_former_packages() {
        let library = library_until(3);
        let witness = analyze_connectivity(&library, &Telescope::reference(4));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(4)));
    }

    #[test]
    fn connectivity_accepts_local_path_attachment_packages() {
        let library = library_until(4);
        let witness = analyze_connectivity(&library, &Telescope::reference(5));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(5)));
    }

    #[test]
    fn connectivity_accepts_self_contained_truncation_packages() {
        let library = library_until(5);
        let witness = analyze_connectivity(&library, &Telescope::reference(6));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(6)));
    }

    #[test]
    fn connectivity_accepts_self_contained_higher_hit_packages() {
        let library = library_until(6);
        let witness = analyze_connectivity(&library, &Telescope::reference(7));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(7)));
    }

    #[test]
    fn connectivity_accepts_self_contained_sphere_lift_packages() {
        let library = library_until(7);
        let witness = analyze_connectivity(&library, &Telescope::reference(8));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(8)));
    }

    #[test]
    fn connectivity_accepts_active_window_axiomatic_bundles() {
        let library = library_until(8);
        let witness = analyze_connectivity(&library, &Telescope::reference(9));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 8,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(9)));
    }

    #[test]
    fn connectivity_accepts_self_contained_modal_shells() {
        let library = library_until(9);
        let witness = analyze_connectivity(&library, &Telescope::reference(10));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(10)));
    }

    #[test]
    fn connectivity_accepts_higher_order_bridge_shells_over_the_active_window() {
        let library = library_until(11);
        let witness = analyze_connectivity(&library, &Telescope::reference(12));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 11,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(12)));
    }

    #[test]
    fn connectivity_accepts_operator_bundle_bridges_over_curvature() {
        let library = library_until(12);
        let witness = analyze_connectivity(&library, &Telescope::reference(13));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 12,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(13)));
    }

    #[test]
    fn connectivity_rejects_stale_library_refs() {
        let library = library_until(4);
        assert!(!passes_connectivity(&library, &Telescope::reference(3)));
    }

    #[test]
    fn connectivity_accepts_temporal_shells_reanchored_through_the_modal_history() {
        let library = library_until(14);
        let witness = analyze_connectivity(&library, &Telescope::reference(15));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: true,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(15)));
    }
}
