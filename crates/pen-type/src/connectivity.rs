use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::Telescope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectivityWitness {
    pub connected: bool,
    pub references_active_window: bool,
    pub self_contained: bool,
    pub max_lib_ref: u32,
}

pub fn analyze_connectivity(library: &Library, telescope: &Telescope) -> ConnectivityWitness {
    ConnectivityWitness {
        connected: is_structurally_connected(telescope),
        references_active_window: telescope.references_window(library.len() as u32),
        self_contained: telescope.lib_refs().is_empty(),
        max_lib_ref: telescope.max_lib_ref(),
    }
}

pub fn passes_connectivity(library: &Library, telescope: &Telescope) -> bool {
    let witness = analyze_connectivity(library, telescope);
    witness.connected && (witness.references_active_window || witness.self_contained)
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

        has_var_edge || has_lib_edge || has_path_attachment || has_higher_path_witness_attachment
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
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(10)));
    }

    #[test]
    fn connectivity_rejects_stale_library_refs() {
        let library = library_until(4);
        assert!(!passes_connectivity(&library, &Telescope::reference(3)));
    }
}
