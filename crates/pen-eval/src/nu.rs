use crate::nu_trace::trace_lines;
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::{Telescope, TelescopeClass};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SingleClauseStructuralNuCaps {
    pub max_expr_nodes: u8,
    pub max_path_dimension: u32,
    pub include_trunc: bool,
    pub include_modal: bool,
    pub include_temporal: bool,
    pub historical_anchor_ref: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct SingleClauseStructuralNuContext {
    prefix_profile: TelescopeNuProfile,
    step_nu_lookup: Box<[u32]>,
    modal_ref_nu_lookup: Box<[u32]>,
    prefix_max_ref_nu: u32,
    prefix_max_modal_ref_nu: u32,
    library_size: u32,
    library_path_dim_sq_sum: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TelescopeNuProfile {
    kappa: u32,
    class: TelescopeClass,
    trivially_derivable: bool,
    type_formation_count: u32,
    intro_count: u32,
    elim_count: u32,
    axiomatic_intro_count: u32,
    modal_kind_mask: u8,
    modal_kind_count: u32,
    temporal_formation_count: u32,
    any_univ_expr: bool,
    has_formation: bool,
    any_parametric_formation: bool,
    has_temporal: bool,
    has_suspension: bool,
    any_lib_pointer: bool,
    all_basic_formation: bool,
    all_pi_sigma: bool,
    all_lib_or_var: bool,
    first_two_lib_pointer_count: u8,
    path_count: u32,
    max_path_dimension: u32,
    pre_path_count: u32,
    post_path_entry_count: u32,
    has_higher_path: bool,
    non_path_expr_count: usize,
    all_non_path_trunc_context: bool,
    distributive_law_count: u32,
    polymorphic_temporal_elim_count: u32,
    has_spatial_temporal_clause: bool,
    lib_refs: BTreeSet<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalClauseNuFacts {
    lib_refs: Box<[u32]>,
    is_type_formation: bool,
    is_intro: bool,
    is_elim: bool,
    is_axiomatic_intro: bool,
    modal_kind_mask: u8,
    is_temporal_formation: bool,
    is_univ: bool,
    is_single_foundation_expr: bool,
    is_single_map_expr: bool,
    has_formation: bool,
    is_parametric_formation: bool,
    is_temporal: bool,
    is_suspension: bool,
    has_lib_pointer: bool,
    is_basic_formation: bool,
    is_pi_sigma: bool,
    is_lib_or_var: bool,
    is_path_con: bool,
    path_dimension: u32,
    is_trunc_context: bool,
    is_distributive_law: bool,
    is_polymorphic_temporal_elim: bool,
    is_spatial_temporal_clause: bool,
}

impl TelescopeNuProfile {
    fn from_telescope(telescope: &Telescope) -> Self {
        let mut profile = Self {
            kappa: u32_from_len(telescope.kappa()),
            all_basic_formation: true,
            all_pi_sigma: true,
            all_lib_or_var: true,
            all_non_path_trunc_context: true,
            ..Self::default()
        };
        let mut saw_path = false;
        let mut has_flat = false;
        let mut has_sharp = false;
        let mut has_disc = false;
        let mut has_shape = false;

        for (index, clause) in telescope.clauses.iter().enumerate() {
            let expr = &clause.expr;
            let clause_lib_refs = expr.lib_refs();
            let clause_has_lib_pointer = !clause_lib_refs.is_empty();

            if clause_has_lib_pointer {
                profile.any_lib_pointer = true;
                if index < 2 {
                    profile.first_two_lib_pointer_count =
                        profile.first_two_lib_pointer_count.saturating_add(1);
                }
            }
            profile.lib_refs.extend(clause_lib_refs);

            if is_type_formation(expr) {
                profile.type_formation_count += 1;
                profile.has_formation = true;
            }
            if is_intro_expr(expr) {
                profile.intro_count += 1;
            }
            if is_elim_expr(expr) {
                profile.elim_count += 1;
            }
            if is_axiomatic_intro(expr) {
                profile.axiomatic_intro_count += 1;
            }
            if is_univ_expr(expr) {
                profile.any_univ_expr = true;
            }
            if is_parametric_formation(expr) {
                profile.any_parametric_formation = true;
            }
            if expr.is_temporal() {
                profile.has_temporal = true;
            }
            if matches!(expr, Expr::Susp(_)) {
                profile.has_suspension = true;
            }
            if matches!(expr, Expr::Flat(_)) {
                has_flat = true;
            }
            if matches!(expr, Expr::Sharp(_)) {
                has_sharp = true;
            }
            if matches!(expr, Expr::Disc(_)) {
                has_disc = true;
            }
            if matches!(expr, Expr::Shape(_)) {
                has_shape = true;
            }
            if matches!(expr, Expr::Next(_) | Expr::Eventually(_)) {
                profile.temporal_formation_count += 1;
            }

            profile.all_basic_formation &= is_basic_formation_expr(expr);
            profile.all_pi_sigma &= is_pi_sigma_expr(expr);
            profile.all_lib_or_var &= matches!(expr, Expr::Lib(_) | Expr::Var(_));

            if let Expr::PathCon(dimension) = expr {
                if !saw_path {
                    profile.pre_path_count = u32_from_len(index);
                    saw_path = true;
                } else {
                    profile.post_path_entry_count += 1;
                }
                profile.path_count += 1;
                profile.max_path_dimension = profile.max_path_dimension.max(*dimension);
                profile.has_higher_path |= *dimension > 1;
            } else {
                if saw_path {
                    profile.post_path_entry_count += 1;
                }
                profile.non_path_expr_count += 1;
                profile.all_non_path_trunc_context &= expr.is_trunc_context();
            }

            if is_distributive_law(expr) {
                profile.distributive_law_count += 1;
            }
            if is_polymorphic_temporal_elim(expr) {
                profile.polymorphic_temporal_elim_count += 1;
            }
            profile.has_spatial_temporal_clause |= is_spatial_temporal_clause(expr);
        }

        profile.modal_kind_mask = modal_kind_mask(has_flat, has_sharp, has_disc, has_shape);
        profile.modal_kind_count = profile.modal_kind_mask.count_ones();
        profile.trivially_derivable = telescope.clauses.is_empty()
            || profile.all_lib_or_var
            || (profile.has_higher_path
                && profile.non_path_expr_count > 0
                && profile.all_non_path_trunc_context);
        profile.class = profile.classify(telescope);
        profile
    }

    fn classify(&self, telescope: &Telescope) -> TelescopeClass {
        let Some(first_expr) = telescope.clauses.first().map(|clause| &clause.expr) else {
            return TelescopeClass::Unknown;
        };

        if telescope.kappa() == 1 && is_formation_only_expr(first_expr) {
            return classify_single_entry_expr(first_expr);
        }
        if telescope.kappa() == 1
            && matches!(first_expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Lib(_)))
        {
            return TelescopeClass::Map;
        }
        if telescope.kappa() >= 2 && self.all_basic_formation && !self.any_lib_pointer {
            return TelescopeClass::Foundation;
        }
        if self.path_count > 0 {
            return TelescopeClass::Hit;
        }
        if self.modal_kind_count > 0 && !self.has_temporal {
            return TelescopeClass::Modal;
        }
        if self.has_temporal {
            return TelescopeClass::Synthesis;
        }
        if self.has_suspension {
            return TelescopeClass::Suspension;
        }
        if (2..=4).contains(&telescope.kappa()) && self.first_two_lib_pointer_count == 2 {
            return TelescopeClass::Map;
        }
        if telescope.kappa() >= 3 && self.any_lib_pointer && self.modal_kind_count == 0 {
            return TelescopeClass::Axiomatic;
        }
        if self.all_pi_sigma && !self.any_lib_pointer {
            return TelescopeClass::Former;
        }

        TelescopeClass::Unknown
    }

    fn base_nu_g(&self) -> u32 {
        match self.class {
            TelescopeClass::Foundation => {
                if self.any_univ_expr {
                    0
                } else {
                    self.type_formation_count
                }
            }
            TelescopeClass::Former => self.intro_count,
            TelescopeClass::Hit => {
                if self.has_formation {
                    self.pre_path_count + 3 + u32::from(self.any_parametric_formation)
                } else {
                    0
                }
            }
            TelescopeClass::Suspension => 5,
            TelescopeClass::Map => u32::from(self.kappa == 1),
            TelescopeClass::Modal => self.modal_kind_count / 2,
            TelescopeClass::Axiomatic => self.axiomatic_intro_count,
            TelescopeClass::Synthesis => self.temporal_formation_count.min(2),
            TelescopeClass::Unknown => self.intro_count,
        }
    }

    fn base_nu_h(&self) -> u32 {
        if self.path_count > 0 {
            self.path_count + self.max_path_dimension * self.max_path_dimension
        } else {
            0
        }
    }

    fn base_nu_c(&self, library: &Library, nu_history: &[(u32, u32)]) -> u32 {
        match self.class {
            TelescopeClass::Foundation => {
                if self.any_univ_expr {
                    self.kappa.saturating_sub(1)
                } else {
                    0
                }
            }
            TelescopeClass::Former => self.intro_count + self.elim_count,
            TelescopeClass::Hit => {
                if self.has_formation {
                    let post_count = self.post_path_entry_count;
                    post_count + (post_count + 1) / 2
                } else {
                    self.kappa + u32_from_len(library.len())
                }
            }
            TelescopeClass::Suspension => 0,
            TelescopeClass::Map => {
                if self.kappa == 1 {
                    1
                } else {
                    let lib_ref_count = u32_from_len(self.lib_refs.len());
                    2 * self.kappa + lib_ref_count * lib_ref_count
                }
            }
            TelescopeClass::Modal => {
                let nu_g = self.base_nu_g();
                let axiom_entries = self.kappa.saturating_sub(nu_g);
                let lib_size = u32_from_len(library.len());
                let pairwise =
                    (self.modal_kind_count * self.modal_kind_count.saturating_sub(1)) / 2;
                axiom_entries + lib_size + pairwise
            }
            TelescopeClass::Axiomatic => {
                let max_ref_nu = self
                    .lib_refs
                    .iter()
                    .filter_map(|step| history_nu(*step, nu_history))
                    .max()
                    .unwrap_or(0);
                let ref_bonus = u32_from_len(self.lib_refs.len()).saturating_sub(1);
                max_ref_nu + self.kappa + ref_bonus
            }
            TelescopeClass::Synthesis => self.kappa.saturating_sub(self.base_nu_g()),
            TelescopeClass::Unknown => self.intro_count + self.elim_count,
        }
    }

    fn distributive_law_bonus(&self, library: &Library, nu_history: &[(u32, u32)]) -> u32 {
        let inherited_nu = self
            .lib_refs
            .iter()
            .filter_map(|step| modal_reference_nu(*step, library, nu_history))
            .max()
            .unwrap_or(0);

        if inherited_nu == 0 {
            0
        } else {
            self.distributive_law_count * inherited_nu
        }
    }

    fn universe_polymorphism_bonus(&self, library: &Library) -> u32 {
        self.polymorphic_temporal_elim_count * u32_from_len(library.len())
    }

    fn infinitesimal_shift_bonus(&self, library: &Library) -> u32 {
        let library_path_dim_sq_sum: u32 = library
            .iter()
            .filter(|entry| entry.has_loop)
            .flat_map(|entry| entry.path_dims.iter().copied())
            .map(|dimension| dimension * dimension)
            .sum();

        if self.has_spatial_temporal_clause && library_path_dim_sq_sum > 0 {
            library_path_dim_sq_sum
        } else {
            0
        }
    }
}

impl TerminalClauseNuFacts {
    pub fn from_clause(clause: &ClauseRec) -> Self {
        let expr = &clause.expr;
        let mut lib_refs = expr.lib_refs().into_iter().collect::<Vec<_>>();
        lib_refs.sort_unstable();
        lib_refs.dedup();
        let has_lib_pointer = !lib_refs.is_empty();
        let is_path_con = is_path_con_expr(expr);
        Self {
            lib_refs: lib_refs.into_boxed_slice(),
            is_type_formation: is_type_formation(expr),
            is_intro: is_intro_expr(expr),
            is_elim: is_elim_expr(expr),
            is_axiomatic_intro: is_axiomatic_intro(expr),
            modal_kind_mask: modal_kind_mask_for_expr(expr),
            is_temporal_formation: matches!(expr, Expr::Next(_) | Expr::Eventually(_)),
            is_univ: is_univ_expr(expr),
            is_single_foundation_expr: matches!(
                classify_single_entry_expr(expr),
                TelescopeClass::Foundation
            ),
            is_single_map_expr: matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Lib(_))),
            has_formation: is_type_formation(expr),
            is_parametric_formation: is_parametric_formation(expr),
            is_temporal: expr.is_temporal(),
            is_suspension: matches!(expr, Expr::Susp(_)),
            has_lib_pointer,
            is_basic_formation: is_basic_formation_expr(expr),
            is_pi_sigma: is_pi_sigma_expr(expr),
            is_lib_or_var: matches!(expr, Expr::Lib(_) | Expr::Var(_)),
            is_path_con,
            path_dimension: match expr {
                Expr::PathCon(dimension) => *dimension,
                _ => 0,
            },
            is_trunc_context: expr.is_trunc_context(),
            is_distributive_law: is_distributive_law(expr),
            is_polymorphic_temporal_elim: is_polymorphic_temporal_elim(expr),
            is_spatial_temporal_clause: is_spatial_temporal_clause(expr),
        }
    }
}

impl SingleClauseStructuralNuContext {
    pub fn from_prefix(
        prefix_telescope: &Telescope,
        library: &Library,
        nu_history: &[(u32, u32)],
    ) -> Self {
        let prefix_profile = TelescopeNuProfile::from_telescope(prefix_telescope);
        let lookup_len = library.len().saturating_add(1);
        let mut step_nu_lookup = vec![0; lookup_len];
        for &(step, nu) in nu_history {
            if let Ok(index) = usize::try_from(step) {
                if index < lookup_len {
                    step_nu_lookup[index] = nu;
                }
            }
        }

        let mut modal_ref_nu_lookup = vec![0; lookup_len];
        for step in 1..lookup_len {
            modal_ref_nu_lookup[step] =
                modal_reference_nu(step as u32, library, nu_history).unwrap_or(0);
        }

        let prefix_max_ref_nu = prefix_profile
            .lib_refs
            .iter()
            .filter_map(|step| step_nu_lookup.get(*step as usize).copied())
            .max()
            .unwrap_or(0);
        let prefix_max_modal_ref_nu = prefix_profile
            .lib_refs
            .iter()
            .filter_map(|step| modal_ref_nu_lookup.get(*step as usize).copied())
            .max()
            .unwrap_or(0);
        let library_path_dim_sq_sum = library
            .iter()
            .filter(|entry| entry.has_loop)
            .flat_map(|entry| entry.path_dims.iter().copied())
            .map(|dimension| dimension * dimension)
            .sum();

        Self {
            prefix_profile,
            step_nu_lookup: step_nu_lookup.into_boxed_slice(),
            modal_ref_nu_lookup: modal_ref_nu_lookup.into_boxed_slice(),
            prefix_max_ref_nu,
            prefix_max_modal_ref_nu,
            library_size: u32_from_len(library.len()),
            library_path_dim_sq_sum,
        }
    }

    pub fn structural_nu_with_clause(&self, clause: &ClauseRec) -> StructuralNuResult {
        let facts = TerminalClauseNuFacts::from_clause(clause);
        self.structural_nu_with_clause_facts(&facts)
    }

    pub fn structural_nu_with_clause_facts(
        &self,
        facts: &TerminalClauseNuFacts,
    ) -> StructuralNuResult {
        let profile = &self.prefix_profile;
        let kappa = profile.kappa.saturating_add(1);
        let type_formation_count =
            profile.type_formation_count + u32::from(facts.is_type_formation);
        let intro_count = profile.intro_count + u32::from(facts.is_intro);
        let elim_count = profile.elim_count + u32::from(facts.is_elim);
        let axiomatic_intro_count =
            profile.axiomatic_intro_count + u32::from(facts.is_axiomatic_intro);
        let modal_kind_mask = profile.modal_kind_mask | facts.modal_kind_mask;
        let modal_kind_count = modal_kind_mask.count_ones();
        let temporal_formation_count =
            profile.temporal_formation_count + u32::from(facts.is_temporal_formation);
        let any_univ_expr = profile.any_univ_expr || facts.is_univ;
        let has_formation = profile.has_formation || facts.has_formation;
        let any_parametric_formation =
            profile.any_parametric_formation || facts.is_parametric_formation;
        let has_temporal = profile.has_temporal || facts.is_temporal;
        let has_suspension = profile.has_suspension || facts.is_suspension;
        let any_lib_pointer = profile.any_lib_pointer || facts.has_lib_pointer;
        let all_basic_formation = profile.all_basic_formation && facts.is_basic_formation;
        let all_pi_sigma = profile.all_pi_sigma && facts.is_pi_sigma;
        let all_lib_or_var = profile.all_lib_or_var && facts.is_lib_or_var;
        let first_two_lib_pointer_count = profile.first_two_lib_pointer_count
            + u8::from(profile.kappa < 2 && facts.has_lib_pointer);
        let path_count = profile.path_count + u32::from(facts.is_path_con);
        let max_path_dimension = profile.max_path_dimension.max(facts.path_dimension);
        let (pre_path_count, post_path_entry_count) = if profile.path_count == 0 {
            if facts.is_path_con {
                (profile.kappa, 0)
            } else {
                (profile.pre_path_count, profile.post_path_entry_count)
            }
        } else {
            (
                profile.pre_path_count,
                profile.post_path_entry_count.saturating_add(1),
            )
        };
        let has_higher_path = profile.has_higher_path || facts.path_dimension > 1;
        let non_path_expr_count = profile.non_path_expr_count + usize::from(!facts.is_path_con);
        let all_non_path_trunc_context =
            profile.all_non_path_trunc_context && (facts.is_path_con || facts.is_trunc_context);
        let distributive_law_count =
            profile.distributive_law_count + u32::from(facts.is_distributive_law);
        let polymorphic_temporal_elim_count =
            profile.polymorphic_temporal_elim_count + u32::from(facts.is_polymorphic_temporal_elim);
        let has_spatial_temporal_clause =
            profile.has_spatial_temporal_clause || facts.is_spatial_temporal_clause;

        let trivially_derivable = all_lib_or_var
            || (has_higher_path && non_path_expr_count > 0 && all_non_path_trunc_context);
        if trivially_derivable {
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

        let class = if kappa == 1 {
            if facts.is_single_foundation_expr {
                TelescopeClass::Foundation
            } else if facts.is_single_map_expr {
                TelescopeClass::Map
            } else if path_count > 0 {
                TelescopeClass::Hit
            } else if modal_kind_count > 0 && !has_temporal {
                TelescopeClass::Modal
            } else if has_temporal {
                TelescopeClass::Synthesis
            } else if has_suspension {
                TelescopeClass::Suspension
            } else if all_pi_sigma && !any_lib_pointer {
                TelescopeClass::Former
            } else {
                TelescopeClass::Unknown
            }
        } else if all_basic_formation && !any_lib_pointer {
            TelescopeClass::Foundation
        } else if path_count > 0 {
            TelescopeClass::Hit
        } else if modal_kind_count > 0 && !has_temporal {
            TelescopeClass::Modal
        } else if has_temporal {
            TelescopeClass::Synthesis
        } else if has_suspension {
            TelescopeClass::Suspension
        } else if (2..=4).contains(&kappa) && first_two_lib_pointer_count == 2 {
            TelescopeClass::Map
        } else if kappa >= 3 && any_lib_pointer && modal_kind_count == 0 {
            TelescopeClass::Axiomatic
        } else if all_pi_sigma && !any_lib_pointer {
            TelescopeClass::Former
        } else {
            TelescopeClass::Unknown
        };

        let mut lib_ref_count = u32_from_len(profile.lib_refs.len());
        let mut max_ref_nu = self.prefix_max_ref_nu;
        let mut max_modal_ref_nu = self.prefix_max_modal_ref_nu;
        for step in facts.lib_refs.iter().copied() {
            if !profile.lib_refs.contains(&step) {
                lib_ref_count = lib_ref_count.saturating_add(1);
            }
            if let Some(nu) = self.step_nu_lookup.get(step as usize).copied() {
                max_ref_nu = max_ref_nu.max(nu);
            }
            if let Some(nu) = self.modal_ref_nu_lookup.get(step as usize).copied() {
                max_modal_ref_nu = max_modal_ref_nu.max(nu);
            }
        }

        let nu_g = match class {
            TelescopeClass::Foundation => {
                if any_univ_expr {
                    0
                } else {
                    type_formation_count
                }
            }
            TelescopeClass::Former => intro_count,
            TelescopeClass::Hit => {
                if has_formation {
                    pre_path_count + 3 + u32::from(any_parametric_formation)
                } else {
                    0
                }
            }
            TelescopeClass::Suspension => 5,
            TelescopeClass::Map => u32::from(kappa == 1),
            TelescopeClass::Modal => modal_kind_count / 2,
            TelescopeClass::Axiomatic => axiomatic_intro_count,
            TelescopeClass::Synthesis => temporal_formation_count.min(2),
            TelescopeClass::Unknown => intro_count,
        };
        let base_nu_h = if path_count > 0 {
            path_count + max_path_dimension * max_path_dimension
        } else {
            0
        };
        let base_nu_c = match class {
            TelescopeClass::Foundation => {
                if any_univ_expr {
                    kappa.saturating_sub(1)
                } else {
                    0
                }
            }
            TelescopeClass::Former => intro_count + elim_count,
            TelescopeClass::Hit => {
                if has_formation {
                    post_path_entry_count + (post_path_entry_count + 1) / 2
                } else {
                    kappa + self.library_size
                }
            }
            TelescopeClass::Suspension => 0,
            TelescopeClass::Map => {
                if kappa == 1 {
                    1
                } else {
                    2 * kappa + lib_ref_count * lib_ref_count
                }
            }
            TelescopeClass::Modal => {
                let axiom_entries = kappa.saturating_sub(nu_g);
                let pairwise = (modal_kind_count * modal_kind_count.saturating_sub(1)) / 2;
                axiom_entries + self.library_size + pairwise
            }
            TelescopeClass::Axiomatic => max_ref_nu + kappa + lib_ref_count.saturating_sub(1),
            TelescopeClass::Synthesis => kappa.saturating_sub(nu_g),
            TelescopeClass::Unknown => intro_count + elim_count,
        };
        let distributive_law_bonus = if class == TelescopeClass::Synthesis && max_modal_ref_nu > 0 {
            distributive_law_count * max_modal_ref_nu
        } else {
            0
        };
        let universe_polymorphism_bonus = if class == TelescopeClass::Synthesis {
            polymorphic_temporal_elim_count * self.library_size
        } else {
            0
        };
        let infinitesimal_shift_bonus =
            if class == TelescopeClass::Synthesis && has_spatial_temporal_clause {
                self.library_path_dim_sq_sum
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
}

pub fn structural_nu(
    telescope: &Telescope,
    library: &Library,
    nu_history: &[(u32, u32)],
) -> StructuralNuResult {
    let profile = TelescopeNuProfile::from_telescope(telescope);
    if profile.trivially_derivable {
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

    let nu_g = profile.base_nu_g();
    let base_nu_h = profile.base_nu_h();
    let base_nu_c = profile.base_nu_c(library, nu_history);

    let distributive_law_bonus = if profile.class == TelescopeClass::Synthesis {
        profile.distributive_law_bonus(library, nu_history)
    } else {
        0
    };
    let universe_polymorphism_bonus = if profile.class == TelescopeClass::Synthesis {
        profile.universe_polymorphism_bonus(library)
    } else {
        0
    };
    let infinitesimal_shift_bonus = if profile.class == TelescopeClass::Synthesis {
        profile.infinitesimal_shift_bonus(library)
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

pub fn structural_nu_single_clause_upper_bound(
    prefix_telescope: &Telescope,
    library: &Library,
    nu_history: &[(u32, u32)],
    caps: SingleClauseStructuralNuCaps,
) -> u32 {
    let profile = TelescopeNuProfile::from_telescope(prefix_telescope);
    let potential = SingleClauseStructuralNuPotential::new(&profile, library, nu_history, caps);

    potential.upper_bound(&profile)
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

fn is_basic_formation_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Var(_))
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn is_type_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ)
        || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ))
        || matches!(expr, Expr::Trunc(_))
}

fn is_path_con_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::PathCon(_))
}

fn modal_kind_mask(has_flat: bool, has_sharp: bool, has_disc: bool, has_shape: bool) -> u8 {
    u8::from(has_flat)
        | (u8::from(has_sharp) << 1)
        | (u8::from(has_disc) << 2)
        | (u8::from(has_shape) << 3)
}

fn modal_kind_mask_for_expr(expr: &Expr) -> u8 {
    modal_kind_mask(
        matches!(expr, Expr::Flat(_)),
        matches!(expr, Expr::Sharp(_)),
        matches!(expr, Expr::Disc(_)),
        matches!(expr, Expr::Shape(_)),
    )
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

fn is_formation_only_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Susp(_))
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn classify_single_entry_expr(expr: &Expr) -> TelescopeClass {
    match expr {
        Expr::Univ => TelescopeClass::Foundation,
        Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => TelescopeClass::Foundation,
        Expr::Susp(_) => TelescopeClass::Suspension,
        _ => TelescopeClass::Unknown,
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

fn is_pi_sigma_expr(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Lam(_) | Expr::App(_, _)
    )
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

struct SingleClauseStructuralNuPotential {
    final_kappa: u32,
    library_size: u32,
    max_path_dimension: u32,
    max_expr_nodes: u8,
    can_add_univ_expr: bool,
    can_add_foundation_type_formation: bool,
    can_add_parametric_formation: bool,
    can_add_intro: bool,
    can_add_lib_pointer: bool,
    can_add_path: bool,
    can_add_suspension: bool,
    max_new_modal_kinds: u32,
    can_add_temporal: bool,
    can_add_distributive_law: bool,
    can_add_polymorphic_temporal_elim: bool,
    can_add_spatial_temporal_clause: bool,
    lib_ref_count_upper: u32,
    max_ref_nu: u32,
    max_modal_ref_nu: u32,
    library_path_dim_sq_sum: u32,
}

impl SingleClauseStructuralNuPotential {
    fn new(
        profile: &TelescopeNuProfile,
        library: &Library,
        nu_history: &[(u32, u32)],
        caps: SingleClauseStructuralNuCaps,
    ) -> Self {
        let extension_refs = extension_reference_candidates(library, caps.historical_anchor_ref);
        let max_new_library_refs = u32_from_len(extension_refs.len());
        let can_add_lib_pointer = max_new_library_refs > 0 && caps.max_expr_nodes >= 1;
        let prefix_ref_count = u32_from_len(profile.lib_refs.len());
        let lib_ref_count_upper = if can_add_lib_pointer {
            prefix_ref_count
                .saturating_add(max_new_library_refs)
                .min(u32_from_len(library.len()))
        } else {
            prefix_ref_count
        };
        let prefix_max_ref_nu = profile
            .lib_refs
            .iter()
            .filter_map(|step| history_nu(*step, nu_history))
            .max()
            .unwrap_or(0);
        let extension_max_ref_nu = extension_refs
            .iter()
            .filter_map(|step| history_nu(*step, nu_history))
            .max()
            .unwrap_or(0);
        let prefix_max_modal_ref_nu = profile
            .lib_refs
            .iter()
            .filter_map(|step| modal_reference_nu(*step, library, nu_history))
            .max()
            .unwrap_or(0);
        let extension_max_modal_ref_nu = extension_refs
            .iter()
            .filter_map(|step| modal_reference_nu(*step, library, nu_history))
            .max()
            .unwrap_or(0);

        Self {
            final_kappa: profile.kappa.saturating_add(1),
            library_size: u32_from_len(library.len()),
            max_path_dimension: caps.max_path_dimension,
            max_expr_nodes: caps.max_expr_nodes,
            can_add_univ_expr: caps.max_expr_nodes >= 1,
            can_add_foundation_type_formation: caps.max_expr_nodes >= 3,
            can_add_parametric_formation: caps.include_trunc && caps.max_expr_nodes >= 2,
            can_add_intro: caps.max_expr_nodes >= 1,
            can_add_lib_pointer,
            can_add_path: caps.max_path_dimension > 0 && caps.max_expr_nodes >= 1,
            can_add_suspension: caps.max_expr_nodes >= 2,
            max_new_modal_kinds: if caps.include_modal && caps.max_expr_nodes >= 2 {
                u32::from(caps.max_expr_nodes.saturating_sub(1)).min(4)
            } else {
                0
            },
            can_add_temporal: caps.include_temporal && caps.max_expr_nodes >= 2,
            can_add_distributive_law: caps.include_modal
                && caps.include_temporal
                && caps.max_expr_nodes >= 7,
            can_add_polymorphic_temporal_elim: caps.include_temporal && caps.max_expr_nodes >= 5,
            can_add_spatial_temporal_clause: caps.include_temporal
                && can_add_lib_pointer
                && caps.max_expr_nodes >= 5,
            lib_ref_count_upper,
            max_ref_nu: prefix_max_ref_nu.max(extension_max_ref_nu),
            max_modal_ref_nu: prefix_max_modal_ref_nu.max(extension_max_modal_ref_nu),
            library_path_dim_sq_sum: library
                .iter()
                .filter(|entry| entry.has_loop)
                .flat_map(|entry| entry.path_dims.iter().copied())
                .map(|dimension| dimension * dimension)
                .sum(),
        }
    }

    fn upper_bound(&self, profile: &TelescopeNuProfile) -> u32 {
        if profile.path_count > 0 {
            return self.hit_upper_from_existing_path(profile);
        }

        let mut best = 0;

        if self.can_add_path {
            best = best.max(self.hit_upper_with_new_path(profile));
        }
        if profile.has_temporal || self.can_add_temporal {
            best = best.max(self.synthesis_upper(profile));
        }
        if !profile.has_temporal && (profile.modal_kind_count > 0 || self.max_new_modal_kinds > 0) {
            best = best.max(self.modal_upper(profile));
        }
        if !profile.has_temporal
            && profile.modal_kind_count == 0
            && (profile.has_suspension || self.can_add_suspension)
        {
            best = best.max(5);
        }
        if !profile.has_temporal && profile.modal_kind_count == 0 && !profile.has_suspension {
            if let Some(bound) = self.foundation_upper(profile) {
                best = best.max(bound);
            }
            if let Some(bound) = self.map_upper(profile) {
                best = best.max(bound);
            }
            if let Some(bound) = self.axiomatic_upper(profile) {
                best = best.max(bound);
            }
            if let Some(bound) = self.generic_upper(profile) {
                best = best.max(bound);
            }
        }

        best
    }

    fn foundation_upper(&self, profile: &TelescopeNuProfile) -> Option<u32> {
        if profile.any_lib_pointer || !profile.all_basic_formation {
            return None;
        }

        let mut best = profile.type_formation_count;
        if !profile.any_univ_expr && self.can_add_foundation_type_formation {
            best = best.max(profile.type_formation_count.saturating_add(1));
        }
        if profile.any_univ_expr || self.can_add_univ_expr {
            best = best.max(self.final_kappa.saturating_sub(1));
        }

        Some(best)
    }

    fn map_upper(&self, profile: &TelescopeNuProfile) -> Option<u32> {
        if self.final_kappa == 1 {
            return (self.can_add_lib_pointer && self.max_expr_nodes >= 3).then_some(2);
        }

        if !(2..=4).contains(&self.final_kappa) {
            return None;
        }

        let map_locked = profile.kappa >= 2 && profile.first_two_lib_pointer_count == 2;
        let map_from_second_clause = profile.kappa == 1
            && profile.first_two_lib_pointer_count == 1
            && self.can_add_lib_pointer;
        if !map_locked && !map_from_second_clause {
            return None;
        }

        Some(
            self.final_kappa.saturating_mul(2).saturating_add(
                self.lib_ref_count_upper
                    .saturating_mul(self.lib_ref_count_upper),
            ),
        )
    }

    fn axiomatic_upper(&self, profile: &TelescopeNuProfile) -> Option<u32> {
        let map_locked = profile.kappa >= 2 && profile.first_two_lib_pointer_count == 2;
        if map_locked
            || self.final_kappa < 3
            || (!profile.any_lib_pointer && !self.can_add_lib_pointer)
        {
            return None;
        }

        let intro_boost = if profile.any_lib_pointer {
            u32::from(self.can_add_intro)
        } else {
            u32::from(self.can_add_lib_pointer && self.max_expr_nodes >= 2)
        };

        Some(
            profile
                .axiomatic_intro_count
                .saturating_add(intro_boost)
                .saturating_add(self.max_ref_nu)
                .saturating_add(self.final_kappa)
                .saturating_add(self.lib_ref_count_upper.saturating_sub(1)),
        )
    }

    fn generic_upper(&self, profile: &TelescopeNuProfile) -> Option<u32> {
        let map_locked = profile.kappa >= 2 && profile.first_two_lib_pointer_count == 2;
        if map_locked || (profile.any_lib_pointer && self.final_kappa >= 3) {
            return None;
        }

        let generic_intro_possible =
            self.max_expr_nodes >= 2 || profile.any_lib_pointer || !profile.all_basic_formation;
        let generic_possible =
            generic_intro_possible || (self.can_add_lib_pointer && self.final_kappa < 3);
        if !generic_possible {
            return None;
        }

        Some(
            profile
                .intro_count
                .saturating_mul(2)
                .saturating_add(profile.elim_count)
                .saturating_add(u32::from(generic_intro_possible) * 2),
        )
    }

    fn modal_upper(&self, profile: &TelescopeNuProfile) -> u32 {
        let modal_kinds = profile
            .modal_kind_count
            .saturating_add(self.max_new_modal_kinds)
            .min(4);
        let pairwise = (modal_kinds * modal_kinds.saturating_sub(1)) / 2;

        self.final_kappa
            .saturating_add(self.library_size)
            .saturating_add(pairwise)
    }

    fn synthesis_upper(&self, profile: &TelescopeNuProfile) -> u32 {
        let distributive_law_count = profile
            .distributive_law_count
            .saturating_add(u32::from(self.can_add_distributive_law));
        let polymorphic_temporal_elim_count = profile
            .polymorphic_temporal_elim_count
            .saturating_add(u32::from(self.can_add_polymorphic_temporal_elim));
        let infinitesimal_shift_bonus =
            if profile.has_spatial_temporal_clause || self.can_add_spatial_temporal_clause {
                self.library_path_dim_sq_sum
            } else {
                0
            };

        self.final_kappa
            .saturating_add(distributive_law_count.saturating_mul(self.max_modal_ref_nu))
            .saturating_add(polymorphic_temporal_elim_count.saturating_mul(self.library_size))
            .saturating_add(infinitesimal_shift_bonus)
    }

    fn hit_upper_with_new_path(&self, profile: &TelescopeNuProfile) -> u32 {
        self.hit_total(
            1,
            self.max_path_dimension,
            profile.has_formation,
            profile.kappa,
            u32::from(profile.any_parametric_formation),
            0,
        )
    }

    fn hit_upper_from_existing_path(&self, profile: &TelescopeNuProfile) -> u32 {
        let post_path_count = profile.post_path_entry_count.saturating_add(1);
        let mut best = self.hit_total(
            profile
                .path_count
                .saturating_add(u32::from(self.can_add_path)),
            profile.max_path_dimension.max(if self.can_add_path {
                self.max_path_dimension
            } else {
                0
            }),
            profile.has_formation,
            profile.pre_path_count,
            u32::from(profile.any_parametric_formation),
            post_path_count,
        );

        if profile.has_formation || self.can_add_univ_expr || self.can_add_parametric_formation {
            best = best.max(self.hit_total(
                profile.path_count,
                profile.max_path_dimension,
                true,
                profile.pre_path_count,
                u32::from(profile.any_parametric_formation || self.can_add_parametric_formation),
                post_path_count,
            ));
        }

        best
    }

    fn hit_total(
        &self,
        path_count: u32,
        max_path_dimension: u32,
        has_formation: bool,
        pre_path_count: u32,
        parametric_bonus: u32,
        post_path_count: u32,
    ) -> u32 {
        let nu_h = path_count.saturating_add(max_path_dimension.saturating_mul(max_path_dimension));
        if has_formation {
            pre_path_count
                .saturating_add(3)
                .saturating_add(parametric_bonus)
                .saturating_add(nu_h)
                .saturating_add(post_path_count)
                .saturating_add((post_path_count + 1) / 2)
        } else {
            nu_h.saturating_add(self.final_kappa)
                .saturating_add(self.library_size)
        }
    }
}

fn extension_reference_candidates(
    library: &Library,
    historical_anchor_ref: Option<u32>,
) -> BTreeSet<u32> {
    let library_size = u32_from_len(library.len());
    let mut refs = BTreeSet::new();

    if library_size == 0 {
        return refs;
    }

    let start = library_size.saturating_sub(1).max(1);
    for index in start..=library_size {
        refs.insert(index);
    }
    if let Some(anchor) = historical_anchor_ref {
        if (1..=library_size).contains(&anchor) {
            refs.insert(anchor);
        }
    }

    refs
}

#[cfg(test)]
mod tests {
    use super::{
        SingleClauseStructuralNuContext, StructuralNuResult, TerminalClauseNuFacts,
        compute_native_nu, compute_nu_c, compute_nu_g, compute_nu_h, detect_distributive_laws,
        detect_infinitesimal_shift, detect_universe_polymorphism, structural_nu,
    };
    use pen_core::clause::ClauseRec;
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;

    fn legacy_structural_nu(
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
        let distributive_law_bonus = if class == pen_core::telescope::TelescopeClass::Synthesis {
            detect_distributive_laws(telescope, library, nu_history)
        } else {
            0
        };
        let universe_polymorphism_bonus = if class == pen_core::telescope::TelescopeClass::Synthesis
        {
            detect_universe_polymorphism(telescope, library)
        } else {
            0
        };
        let infinitesimal_shift_bonus = if class == pen_core::telescope::TelescopeClass::Synthesis {
            detect_infinitesimal_shift(telescope, library)
        } else {
            0
        };
        let nu_h = base_nu_h + infinitesimal_shift_bonus;
        let nu_c = base_nu_c + distributive_law_bonus + universe_polymorphism_bonus;

        StructuralNuResult {
            nu_g,
            nu_h,
            nu_c,
            total: nu_g + nu_h + nu_c,
            distributive_law_bonus,
            universe_polymorphism_bonus,
            infinitesimal_shift_bonus,
        }
    }

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

    #[test]
    fn structural_nu_fast_path_matches_legacy_helper_composition() {
        let (library, history) = replay_reference_library(14);
        let surfaces = vec![
            Telescope::reference(4),
            Telescope::reference(5),
            Telescope::reference(10),
            Telescope::reference(15),
            Telescope::new(vec![
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Formation,
                    Expr::Pi(Box::new(Expr::Lib(3)), Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Introduction,
                    Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2))),
                ),
            ]),
            Telescope::new(vec![
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Formation,
                    Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(pen_core::clause::ClauseRole::Introduction, Expr::Var(1)),
                ClauseRec::new(pen_core::clause::ClauseRole::PathAttach, Expr::PathCon(1)),
            ]),
        ];

        for (index, telescope) in surfaces.iter().enumerate() {
            assert_eq!(
                structural_nu(telescope, &library, &history),
                legacy_structural_nu(telescope, &library, &history),
                "surface {index} diverged"
            );
        }
    }

    #[test]
    fn single_clause_context_matches_full_structural_nu() {
        let (library, history) = replay_reference_library(14);
        let surfaces = vec![
            Telescope::reference(2),
            Telescope::reference(4),
            Telescope::reference(5),
            Telescope::reference(10),
            Telescope::reference(15),
            Telescope::new(vec![ClauseRec::new(
                pen_core::clause::ClauseRole::Formation,
                Expr::Susp(Box::new(Expr::Var(1))),
            )]),
            Telescope::new(vec![
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Formation,
                    Expr::Pi(Box::new(Expr::Lib(3)), Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Introduction,
                    Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2))),
                ),
            ]),
            Telescope::new(vec![
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Formation,
                    Expr::Flat(Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Introduction,
                    Expr::Eventually(Box::new(Expr::Lib(2))),
                ),
                ClauseRec::new(
                    pen_core::clause::ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(4)),
                        Box::new(Expr::Var(1)),
                    ))),
                ),
            ]),
        ];

        for (index, telescope) in surfaces.iter().enumerate() {
            let expected = structural_nu(telescope, &library, &history);
            let prefix_len = telescope.clauses.len().saturating_sub(1);
            let prefix = Telescope::new(telescope.clauses[..prefix_len].to_vec());
            let last_clause = telescope
                .clauses
                .last()
                .expect("test surface should contain a last clause");
            let context = SingleClauseStructuralNuContext::from_prefix(&prefix, &library, &history);
            let last_clause_facts = TerminalClauseNuFacts::from_clause(last_clause);
            assert_eq!(
                context.structural_nu_with_clause(last_clause),
                expected,
                "surface {index} diverged",
            );
            assert_eq!(
                context.structural_nu_with_clause_facts(&last_clause_facts),
                expected,
                "facts surface {index} diverged",
            );
        }
    }
}
