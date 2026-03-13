use crate::nu_trace::trace_lines;
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::{Telescope, TelescopeClass};

pub type NuHistory = Vec<(u32, u32)>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StructuralNuResult {
    pub nu_g: u32,
    pub nu_h: u32,
    pub nu_c: u32,
    pub total: u32,
    pub distributive_law_bonus: u32,
    pub universe_polymorphism_bonus: u32,
    pub infinitesimal_shift_bonus: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeNuResult {
    pub nu_g: u32,
    pub nu_h: u32,
    pub nu_c: u32,
    pub total: u32,
    pub trace: Vec<String>,
}

pub fn structural_nu(
    telescope: &Telescope,
    library: &Library,
    nu_history: &[(u32, u32)],
) -> StructuralNuResult {
    if telescope.is_trivially_derivable(library) {
        return StructuralNuResult {
            nu_g: 0,
            nu_h: 0,
            nu_c: 0,
            total: 0,
            distributive_law_bonus: 0,
            universe_polymorphism_bonus: 0,
            infinitesimal_shift_bonus: 0,
        };
    }

    let class = telescope.classify(library);
    let nu_g = compute_nu_g(class, telescope);
    let base_nu_h = compute_nu_h(telescope);
    let base_nu_c = compute_nu_c(class, telescope, library, nu_history);

    let distributive_law_bonus = if class == TelescopeClass::Synthesis {
        detect_distributive_laws(telescope, library, nu_history)
    } else {
        0
    };
    let universe_polymorphism_bonus = if class == TelescopeClass::Synthesis {
        detect_universe_polymorphism(telescope, library)
    } else {
        0
    };
    let infinitesimal_shift_bonus = if class == TelescopeClass::Synthesis {
        detect_infinitesimal_shift(telescope, library)
    } else {
        0
    };

    let nu_h = base_nu_h + infinitesimal_shift_bonus;
    let nu_c = base_nu_c + distributive_law_bonus + universe_polymorphism_bonus;
    let total = nu_g + nu_h + nu_c;

    StructuralNuResult {
        nu_g,
        nu_h,
        nu_c,
        total,
        distributive_law_bonus,
        universe_polymorphism_bonus,
        infinitesimal_shift_bonus,
    }
}

pub fn compute_native_nu(
    telescope: &Telescope,
    library: &Library,
    nu_history: &[(u32, u32)],
) -> NativeNuResult {
    let result = structural_nu(telescope, library, nu_history);
    NativeNuResult {
        nu_g: result.nu_g,
        nu_h: result.nu_h,
        nu_c: result.nu_c,
        total: result.total,
        trace: trace_lines(telescope, &result),
    }
}

pub fn compute_nu_g(class: TelescopeClass, telescope: &Telescope) -> u32 {
    match class {
        TelescopeClass::Foundation => foundation_nu_g(telescope),
        TelescopeClass::Former => former_nu_g(telescope),
        TelescopeClass::Hit => hit_nu_g(telescope),
        TelescopeClass::Suspension => 5,
        TelescopeClass::Map => map_nu_g(telescope),
        TelescopeClass::Modal => modal_nu_g(telescope),
        TelescopeClass::Axiomatic => axiomatic_nu_g(telescope),
        TelescopeClass::Synthesis => synthesis_nu_g(telescope),
        TelescopeClass::Unknown => generic_nu_g(telescope),
    }
}

pub fn compute_nu_h(telescope: &Telescope) -> u32 {
    let dims = telescope.path_dimensions();
    let max_dimension = dims.iter().copied().max().unwrap_or(0);
    let path_count = u32_from_len(dims.len());

    if path_count > 0 {
        path_count + max_dimension * max_dimension
    } else {
        0
    }
}

pub fn compute_nu_c(
    class: TelescopeClass,
    telescope: &Telescope,
    library: &Library,
    nu_history: &[(u32, u32)],
) -> u32 {
    match class {
        TelescopeClass::Foundation => foundation_nu_c(telescope),
        TelescopeClass::Former => former_nu_c(telescope),
        TelescopeClass::Hit => hit_nu_c(telescope, library),
        TelescopeClass::Suspension => 0,
        TelescopeClass::Map => map_nu_c(telescope),
        TelescopeClass::Modal => modal_nu_c(telescope, library),
        TelescopeClass::Axiomatic => axiomatic_nu_c(telescope, nu_history),
        TelescopeClass::Synthesis => synthesis_base_nu_c(telescope),
        TelescopeClass::Unknown => generic_nu_c(telescope),
    }
}

pub fn detect_distributive_laws(
    telescope: &Telescope,
    library: &Library,
    nu_history: &[(u32, u32)],
) -> u32 {
    let inherited_nu = telescope
        .lib_refs()
        .into_iter()
        .filter_map(|step| modal_reference_nu(step, library, nu_history))
        .max()
        .unwrap_or(0);

    if inherited_nu == 0 {
        return 0;
    }

    let law_count = telescope
        .clauses
        .iter()
        .filter(|clause| is_distributive_law(&clause.expr))
        .count();

    u32_from_len(law_count) * inherited_nu
}

pub fn detect_universe_polymorphism(telescope: &Telescope, library: &Library) -> u32 {
    let poly_elims = telescope
        .clauses
        .iter()
        .filter(|clause| is_polymorphic_temporal_elim(&clause.expr))
        .count();

    u32_from_len(poly_elims) * u32_from_len(library.len())
}

pub fn detect_infinitesimal_shift(telescope: &Telescope, library: &Library) -> u32 {
    let has_spatial_temporal = telescope
        .clauses
        .iter()
        .any(|clause| is_spatial_temporal_clause(&clause.expr));
    let library_path_dim_sq_sum: u32 = library
        .iter()
        .filter(|entry| entry.has_loop)
        .flat_map(|entry| entry.path_dims.iter().copied())
        .map(|dimension| dimension * dimension)
        .sum();

    if has_spatial_temporal && library_path_dim_sq_sum > 0 {
        library_path_dim_sq_sum
    } else {
        0
    }
}

fn foundation_nu_g(telescope: &Telescope) -> u32 {
    let exprs = exprs(telescope);
    if exprs.iter().any(|expr| is_univ_expr(expr)) {
        0
    } else {
        u32_from_len(exprs.iter().filter(|expr| is_type_formation(expr)).count())
    }
}

fn former_nu_g(telescope: &Telescope) -> u32 {
    u32_from_len(
        exprs(telescope)
            .into_iter()
            .filter(|expr| is_intro_expr(expr))
            .count(),
    )
}

fn hit_nu_g(telescope: &Telescope) -> u32 {
    let exprs = exprs(telescope);
    let has_formation = exprs.iter().any(|expr| is_type_formation(expr));

    if !has_formation {
        return 0;
    }

    let pre_path_count = telescope
        .clauses
        .iter()
        .take_while(|clause| !is_path_con_expr(&clause.expr))
        .count();
    let parametric_bonus = u32::from(exprs.iter().any(|expr| is_parametric_formation(expr)));

    u32_from_len(pre_path_count) + 3 + parametric_bonus
}

fn map_nu_g(telescope: &Telescope) -> u32 {
    if telescope.kappa() == 1 { 1 } else { 0 }
}

fn modal_nu_g(telescope: &Telescope) -> u32 {
    count_modal_ops(&exprs(telescope)) / 2
}

fn axiomatic_nu_g(telescope: &Telescope) -> u32 {
    u32_from_len(
        exprs(telescope)
            .into_iter()
            .filter(|expr| is_axiomatic_intro(expr))
            .count(),
    )
}

fn synthesis_nu_g(telescope: &Telescope) -> u32 {
    count_temporal_formations(&exprs(telescope)).min(2)
}

fn generic_nu_g(telescope: &Telescope) -> u32 {
    u32_from_len(
        exprs(telescope)
            .into_iter()
            .filter(|expr| is_intro_expr(expr))
            .count(),
    )
}

fn foundation_nu_c(telescope: &Telescope) -> u32 {
    let exprs = exprs(telescope);
    if exprs.iter().any(|expr| is_univ_expr(expr)) {
        kappa_u32(telescope).saturating_sub(1)
    } else {
        0
    }
}

fn former_nu_c(telescope: &Telescope) -> u32 {
    let exprs = exprs(telescope);
    let intros = u32_from_len(exprs.iter().filter(|expr| is_intro_expr(expr)).count());
    let eliminations = u32_from_len(exprs.iter().filter(|expr| is_elim_expr(expr)).count());
    intros + eliminations
}

fn hit_nu_c(telescope: &Telescope, library: &Library) -> u32 {
    let exprs = exprs(telescope);
    let has_formation = exprs.iter().any(|expr| is_type_formation(expr));

    if has_formation {
        let post_path_entries: Vec<_> = telescope
            .clauses
            .iter()
            .skip_while(|clause| !is_path_con_expr(&clause.expr))
            .skip(1)
            .collect();
        let post_count = u32_from_len(post_path_entries.len());
        let post_adjoint = (post_count + 1) / 2;
        post_count + post_adjoint
    } else {
        kappa_u32(telescope) + u32_from_len(library.len())
    }
}

fn map_nu_c(telescope: &Telescope) -> u32 {
    if telescope.kappa() == 1 {
        return 1;
    }

    let kappa = kappa_u32(telescope);
    let lib_ref_count = u32_from_len(telescope.lib_refs().len());
    2 * kappa + lib_ref_count * lib_ref_count
}

fn modal_nu_c(telescope: &Telescope, library: &Library) -> u32 {
    let modal_ops = count_modal_ops(&exprs(telescope));
    let nu_g = modal_ops / 2;
    let axiom_entries = kappa_u32(telescope).saturating_sub(nu_g);
    let lib_size = u32_from_len(library.len());
    let pairwise = (modal_ops * modal_ops.saturating_sub(1)) / 2;
    axiom_entries + lib_size + pairwise
}

fn axiomatic_nu_c(telescope: &Telescope, nu_history: &[(u32, u32)]) -> u32 {
    let max_ref_nu = telescope
        .lib_refs()
        .into_iter()
        .filter_map(|step| history_nu(step, nu_history))
        .max()
        .unwrap_or(0);
    let ref_bonus = u32_from_len(telescope.lib_refs().len()).saturating_sub(1);
    max_ref_nu + kappa_u32(telescope) + ref_bonus
}

fn synthesis_base_nu_c(telescope: &Telescope) -> u32 {
    kappa_u32(telescope).saturating_sub(synthesis_nu_g(telescope))
}

fn generic_nu_c(telescope: &Telescope) -> u32 {
    let exprs = exprs(telescope);
    let intros = u32_from_len(exprs.iter().filter(|expr| is_intro_expr(expr)).count());
    let eliminations = u32_from_len(exprs.iter().filter(|expr| is_elim_expr(expr)).count());
    intros + eliminations
}

fn exprs(telescope: &Telescope) -> Vec<&Expr> {
    telescope
        .clauses
        .iter()
        .map(|clause| &clause.expr)
        .collect()
}

fn kappa_u32(telescope: &Telescope) -> u32 {
    u32_from_len(telescope.kappa())
}

fn history_nu(step_index: u32, history: &[(u32, u32)]) -> Option<u32> {
    history
        .iter()
        .find_map(|(step, nu)| (*step == step_index).then_some(*nu))
}

fn modal_reference_nu(step_index: u32, library: &Library, history: &[(u32, u32)]) -> Option<u32> {
    let entry = library_entry(step_index, library)?;
    if entry.capabilities.has_modal_ops {
        history_nu(step_index, history)
    } else {
        None
    }
}

fn library_entry(step_index: u32, library: &Library) -> Option<&LibraryEntry> {
    step_index
        .checked_sub(1)
        .and_then(|index| library.get(index as usize))
}

fn is_distributive_law(expr: &Expr) -> bool {
    match expr {
        Expr::Pi(domain, codomain) => {
            (is_modal_wrapping_temporal(domain) && is_temporal_wrapping_modal(codomain))
                || (is_temporal_wrapping_modal(domain) && is_modal_wrapping_temporal(codomain))
        }
        _ => false,
    }
}

fn is_modal_wrapping_temporal(expr: &Expr) -> bool {
    match expr {
        Expr::Flat(inner) | Expr::Sharp(inner) | Expr::Disc(inner) | Expr::Shape(inner) => {
            matches!(inner.as_ref(), Expr::Next(_) | Expr::Eventually(_))
        }
        _ => false,
    }
}

fn is_temporal_wrapping_modal(expr: &Expr) -> bool {
    match expr {
        Expr::Next(inner) | Expr::Eventually(inner) => matches!(
            inner.as_ref(),
            Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_)
        ),
        _ => false,
    }
}

fn is_polymorphic_temporal_elim(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::App(function, _) if matches!(function.as_ref(), Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(_)))))
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) if matches!(inner.as_ref(), Expr::Next(inner2) if matches!(inner2.as_ref(), Expr::Var(_))))
                    && matches!(codomain.as_ref(), Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                    && matches!(codomain.as_ref(), Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
}

fn is_spatial_temporal_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(_))
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner) | Expr::Eventually(inner)
                                if matches!(inner.as_ref(), Expr::Var(_))
                        )
            )
    )
}

fn is_univ_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ)
}

fn is_type_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ)
        || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ))
        || matches!(expr, Expr::Trunc(_))
}

fn is_path_con_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::PathCon(_))
}

fn is_intro_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) | Expr::Lam(_) | Expr::Var(_) => true,
        Expr::App(function, _) => !matches!(function.as_ref(), Expr::Lam(_)),
        _ => false,
    }
}

fn is_elim_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)))
}

fn is_axiomatic_intro(expr: &Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) | Expr::Lam(_) | Expr::Var(_) => true,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_) | Expr::Lib(_)) => false,
        Expr::App(_, _) => true,
        Expr::Pi(domain, codomain) => pi_is_new_schema(domain, codomain),
        _ => false,
    }
}

fn pi_is_new_schema(domain: &Expr, codomain: &Expr) -> bool {
    !has_direct_lib(domain)
        && !has_direct_lib(codomain)
        && !has_operator_ref(domain)
        && !has_operator_ref(codomain)
        && !pi_is_preservation(domain, codomain)
}

fn pi_is_preservation(domain: &Expr, codomain: &Expr) -> bool {
    matches!((domain, codomain), (Expr::Var(left), Expr::Var(right)) if left == right)
}

fn has_direct_lib(expr: &Expr) -> bool {
    match expr {
        Expr::Lib(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            has_direct_lib(left) || has_direct_lib(right)
        }
        Expr::Lam(expr)
        | Expr::Flat(expr)
        | Expr::Sharp(expr)
        | Expr::Disc(expr)
        | Expr::Shape(expr)
        | Expr::Next(expr)
        | Expr::Eventually(expr)
        | Expr::Refl(expr)
        | Expr::Susp(expr)
        | Expr::Trunc(expr) => has_direct_lib(expr),
        Expr::Id(ty, left, right) => {
            has_direct_lib(ty) || has_direct_lib(left) || has_direct_lib(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::PathCon(_) => false,
    }
}

fn has_operator_ref(expr: &Expr) -> bool {
    match expr {
        Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_) => true,
        Expr::Pi(left, right) | Expr::Sigma(left, right) | Expr::App(left, right) => {
            has_operator_ref(left) || has_operator_ref(right)
        }
        Expr::Lam(expr) | Expr::Refl(expr) | Expr::Susp(expr) | Expr::Trunc(expr) => {
            has_operator_ref(expr)
        }
        Expr::Id(ty, left, right) => {
            has_operator_ref(ty) || has_operator_ref(left) || has_operator_ref(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn is_parametric_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(_)))
}

fn count_temporal_formations(exprs: &[&Expr]) -> u32 {
    u32_from_len(
        exprs
            .iter()
            .filter(|expr| matches!(expr, Expr::Next(_) | Expr::Eventually(_)))
            .count(),
    )
}

fn count_modal_ops(exprs: &[&Expr]) -> u32 {
    let has_flat = exprs.iter().any(|expr| matches!(expr, Expr::Flat(_)));
    let has_sharp = exprs.iter().any(|expr| matches!(expr, Expr::Sharp(_)));
    let has_disc = exprs.iter().any(|expr| matches!(expr, Expr::Disc(_)));
    let has_shape = exprs.iter().any(|expr| matches!(expr, Expr::Shape(_)));
    u32_from_len(
        [has_flat, has_sharp, has_disc, has_shape]
            .into_iter()
            .filter(|flag| *flag)
            .count(),
    )
}

fn u32_from_len(value: usize) -> u32 {
    u32::try_from(value).expect("value exceeded u32 range")
}

#[cfg(test)]
mod tests {
    use super::{compute_native_nu, structural_nu};
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;

    fn replay_reference_library(last_step: u32) -> (Library, Vec<(u32, u32)>) {
        let mut library = Vec::new();
        let mut history = Vec::new();

        for step in 1..=last_step {
            let telescope = Telescope::reference(step);
            let result = structural_nu(&telescope, &library, &history);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
            history.push((step, result.total));
        }

        (library, history)
    }

    #[test]
    fn reference_sequence_matches_donor_structural_totals() {
        let expected = [1, 1, 2, 5, 7, 8, 10, 18, 17, 19, 26, 34, 46, 62, 103];
        let mut library = Vec::new();
        let mut history = Vec::new();

        for (step, expected_total) in (1_u32..=15).zip(expected) {
            let telescope = Telescope::reference(step);
            let result = structural_nu(&telescope, &library, &history);
            assert_eq!(result.total, expected_total, "step {step} total changed");

            library.push(LibraryEntry::from_telescope(&telescope, &library));
            history.push((step, result.total));
        }
    }

    #[test]
    fn dct_meta_theorem_bonuses_fire() {
        let (library, history) = replay_reference_library(14);
        let dct = Telescope::reference(15);
        let result = structural_nu(&dct, &library, &history);

        assert_eq!(result.total, 103);
        assert!(result.distributive_law_bonus > 0);
        assert!(result.universe_polymorphism_bonus > 0);
        assert!(result.infinitesimal_shift_bonus > 0);
    }

    #[test]
    fn native_nu_trace_is_deterministic() {
        let (library, history) = replay_reference_library(14);
        let dct = Telescope::reference(15);
        let first = compute_native_nu(&dct, &library, &history);
        let second = compute_native_nu(&dct, &library, &history);

        assert_eq!(first.total, second.total);
        assert_eq!(first.trace, second.trace);
        assert!(first.trace.iter().any(|line| line == "nu_total=103"));
    }
}
