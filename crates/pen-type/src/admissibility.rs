use crate::obligations::summarize_structural_debt;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::{Telescope, TelescopeClass};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissibilityMode {
    #[default]
    Guarded,
    RelaxedShadow,
    RealisticShadow,
    DemoBreadthShadow,
    DesktopClaimShadow,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PackagePolicy {
    Forbid,
    #[default]
    Allow,
    Prefer,
    Require,
}

impl PackagePolicy {
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Require)
    }

    pub const fn is_focus(self) -> bool {
        matches!(self, Self::Prefer | Self::Require)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuralFamily {
    FormerEliminator,
    InitialHit,
    TruncationHit,
    HigherHit,
    SphereLift,
    AxiomaticBundle,
    ModalShell,
    ConnectionShell,
    CurvatureShell,
    OperatorBundle,
    HilbertFunctional,
    TemporalShell,
}

impl StructuralFamily {
    pub const ALL: [Self; 12] = [
        Self::FormerEliminator,
        Self::InitialHit,
        Self::TruncationHit,
        Self::HigherHit,
        Self::SphereLift,
        Self::AxiomaticBundle,
        Self::ModalShell,
        Self::ConnectionShell,
        Self::CurvatureShell,
        Self::OperatorBundle,
        Self::HilbertFunctional,
        Self::TemporalShell,
    ];

    pub const fn slug(self) -> &'static str {
        match self {
            Self::FormerEliminator => "former_eliminator",
            Self::InitialHit => "initial_hit",
            Self::TruncationHit => "truncation_hit",
            Self::HigherHit => "higher_hit",
            Self::SphereLift => "sphere_lift",
            Self::AxiomaticBundle => "axiomatic_bundle",
            Self::ModalShell => "modal_shell",
            Self::ConnectionShell => "connection_shell",
            Self::CurvatureShell => "curvature_shell",
            Self::OperatorBundle => "operator_bundle",
            Self::HilbertFunctional => "hilbert_functional",
            Self::TemporalShell => "temporal_shell",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StructuralFamilyMatchMask(u16);

impl StructuralFamilyMatchMask {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn insert(&mut self, family: StructuralFamily) {
        self.0 |= structural_family_bit(family);
    }

    pub const fn matches(self, family: StructuralFamily) -> bool {
        self.0 & structural_family_bit(family) != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

const fn structural_family_bit(family: StructuralFamily) -> u16 {
    match family {
        StructuralFamily::FormerEliminator => 1 << 0,
        StructuralFamily::InitialHit => 1 << 1,
        StructuralFamily::TruncationHit => 1 << 2,
        StructuralFamily::HigherHit => 1 << 3,
        StructuralFamily::SphereLift => 1 << 4,
        StructuralFamily::AxiomaticBundle => 1 << 5,
        StructuralFamily::ModalShell => 1 << 6,
        StructuralFamily::ConnectionShell => 1 << 7,
        StructuralFamily::CurvatureShell => 1 << 8,
        StructuralFamily::OperatorBundle => 1 << 9,
        StructuralFamily::HilbertFunctional => 1 << 10,
        StructuralFamily::TemporalShell => 1 << 11,
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PackagePolicies {
    pub former_eliminator: PackagePolicy,
    pub initial_hit: PackagePolicy,
    pub truncation_hit: PackagePolicy,
    pub higher_hit: PackagePolicy,
    pub sphere_lift: PackagePolicy,
    pub axiomatic_bundle: PackagePolicy,
    pub modal_shell: PackagePolicy,
    pub connection_shell: PackagePolicy,
    pub curvature_shell: PackagePolicy,
    pub operator_bundle: PackagePolicy,
    pub hilbert_functional: PackagePolicy,
    pub temporal_shell: PackagePolicy,
}

impl PackagePolicies {
    pub fn policy_for(self, family: StructuralFamily) -> PackagePolicy {
        match family {
            StructuralFamily::FormerEliminator => self.former_eliminator,
            StructuralFamily::InitialHit => self.initial_hit,
            StructuralFamily::TruncationHit => self.truncation_hit,
            StructuralFamily::HigherHit => self.higher_hit,
            StructuralFamily::SphereLift => self.sphere_lift,
            StructuralFamily::AxiomaticBundle => self.axiomatic_bundle,
            StructuralFamily::ModalShell => self.modal_shell,
            StructuralFamily::ConnectionShell => self.connection_shell,
            StructuralFamily::CurvatureShell => self.curvature_shell,
            StructuralFamily::OperatorBundle => self.operator_bundle,
            StructuralFamily::HilbertFunctional => self.hilbert_functional,
            StructuralFamily::TemporalShell => self.temporal_shell,
        }
    }

    fn with_focus(focus_family: Option<StructuralFamily>, focus_policy: PackagePolicy) -> Self {
        let mut policies = Self::default();
        if let Some(focus_family) = focus_family {
            match focus_family {
                StructuralFamily::FormerEliminator => policies.former_eliminator = focus_policy,
                StructuralFamily::InitialHit => policies.initial_hit = focus_policy,
                StructuralFamily::TruncationHit => policies.truncation_hit = focus_policy,
                StructuralFamily::HigherHit => policies.higher_hit = focus_policy,
                StructuralFamily::SphereLift => policies.sphere_lift = focus_policy,
                StructuralFamily::AxiomaticBundle => policies.axiomatic_bundle = focus_policy,
                StructuralFamily::ModalShell => policies.modal_shell = focus_policy,
                StructuralFamily::ConnectionShell => policies.connection_shell = focus_policy,
                StructuralFamily::CurvatureShell => policies.curvature_shell = focus_policy,
                StructuralFamily::OperatorBundle => policies.operator_bundle = focus_policy,
                StructuralFamily::HilbertFunctional => {
                    policies.hilbert_functional = focus_policy;
                }
                StructuralFamily::TemporalShell => policies.temporal_shell = focus_policy,
            }
        }
        policies
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissibilityDecisionClass {
    RejectedByExactLegality,
    RejectedByStructuralDebtCap,
    AdmittedDeprioritized,
    #[default]
    AdmittedFocusAligned,
}

impl AdmissibilityDecisionClass {
    pub const fn is_admitted(self) -> bool {
        matches!(
            self,
            Self::AdmittedDeprioritized | Self::AdmittedFocusAligned
        )
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RejectedByExactLegality => "rejected_by_exact_legality",
            Self::RejectedByStructuralDebtCap => "rejected_by_structural_debt_cap",
            Self::AdmittedDeprioritized => "admitted_but_deprioritized",
            Self::AdmittedFocusAligned => "admitted_and_focus_aligned",
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissibilityDecision {
    #[serde(default)]
    pub class: AdmissibilityDecisionClass,
    #[serde(default)]
    pub reason: String,
}

impl AdmissibilityDecision {
    fn rejected_by_exact_legality(reason: impl Into<String>) -> Self {
        Self {
            class: AdmissibilityDecisionClass::RejectedByExactLegality,
            reason: reason.into(),
        }
    }

    fn rejected_by_structural_debt_cap(reason: impl Into<String>) -> Self {
        Self {
            class: AdmissibilityDecisionClass::RejectedByStructuralDebtCap,
            reason: reason.into(),
        }
    }

    fn admitted_deprioritized(reason: impl Into<String>) -> Self {
        Self {
            class: AdmissibilityDecisionClass::AdmittedDeprioritized,
            reason: reason.into(),
        }
    }

    fn admitted_focus_aligned(reason: impl Into<String>) -> Self {
        Self {
            class: AdmissibilityDecisionClass::AdmittedFocusAligned,
            reason: reason.into(),
        }
    }

    pub const fn is_admitted(&self) -> bool {
        self.class.is_admitted()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissibilityDiagnostics {
    pub exact_legality_rejections: usize,
    pub structural_debt_cap_rejections: usize,
    pub admitted_deprioritized: usize,
    pub admitted_focus_aligned: usize,
    #[serde(default)]
    pub reason_counts: BTreeMap<String, usize>,
}

impl AdmissibilityDiagnostics {
    pub fn record(&mut self, decision: &AdmissibilityDecision) {
        match decision.class {
            AdmissibilityDecisionClass::RejectedByExactLegality => {
                self.exact_legality_rejections += 1;
            }
            AdmissibilityDecisionClass::RejectedByStructuralDebtCap => {
                self.structural_debt_cap_rejections += 1;
            }
            AdmissibilityDecisionClass::AdmittedDeprioritized => {
                self.admitted_deprioritized += 1;
            }
            AdmissibilityDecisionClass::AdmittedFocusAligned => {
                self.admitted_focus_aligned += 1;
            }
        }
        *self
            .reason_counts
            .entry(decision.reason.clone())
            .or_insert(0) += 1;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StrictAdmissibility {
    pub mode: AdmissibilityMode,
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
    pub package_policies: PackagePolicies,
    pub focus_family: Option<StructuralFamily>,
    pub historical_anchor_ref: Option<u32>,
}

impl StrictAdmissibility {
    pub fn supports_exact_clause_kappa(self, clause_kappa: u16) -> bool {
        (self.min_clause_kappa..=self.max_clause_kappa).contains(&clause_kappa)
    }

    pub fn policy_for(self, family: StructuralFamily) -> PackagePolicy {
        self.package_policies.policy_for(family)
    }

    pub fn required_focus_family(self) -> Option<StructuralFamily> {
        self.focus_family
            .filter(|family| self.policy_for(*family).is_required())
    }
}

pub fn passes_strict_admissibility(
    step_index: u32,
    library: &Library,
    telescope: &Telescope,
    admissibility: StrictAdmissibility,
) -> bool {
    assess_strict_admissibility(step_index, library, telescope, admissibility).is_admitted()
}

pub fn assess_strict_admissibility(
    step_index: u32,
    library: &Library,
    telescope: &Telescope,
    admissibility: StrictAdmissibility,
) -> AdmissibilityDecision {
    if step_index <= 3 {
        return assess_strict_admissibility_from_family_matches(
            step_index,
            library,
            telescope,
            admissibility,
            StructuralFamilyMatchMask::empty(),
        );
    }

    let matched_families = package_match_profile(
        library,
        telescope,
        admissibility.historical_anchor_ref,
        admissibility.mode,
    )
    .to_match_mask();
    assess_strict_admissibility_from_family_matches(
        step_index,
        library,
        telescope,
        admissibility,
        matched_families,
    )
}

pub fn assess_strict_admissibility_from_family_matches(
    step_index: u32,
    library: &Library,
    telescope: &Telescope,
    admissibility: StrictAdmissibility,
    matched_families: StructuralFamilyMatchMask,
) -> AdmissibilityDecision {
    if !admissibility.supports_exact_clause_kappa(telescope.kappa() as u16) {
        return AdmissibilityDecision::rejected_by_exact_legality("outside_exact_kappa_band");
    }

    match step_index {
        1 => {
            if telescope.kappa() == 2
                && telescope
                    .clauses
                    .iter()
                    .any(|clause| matches!(clause.expr, Expr::Univ))
                && telescope
                    .clauses
                    .iter()
                    .any(|clause| is_universe_application(&clause.expr))
            {
                AdmissibilityDecision::admitted_focus_aligned("bootstrap_universe_intro")
            } else {
                AdmissibilityDecision::rejected_by_exact_legality("bootstrap_universe_shape")
            }
        }
        2 => {
            if telescope.kappa() == 1
                && telescope
                    .clauses
                    .iter()
                    .all(|clause| is_universe_application(&clause.expr))
            {
                AdmissibilityDecision::admitted_focus_aligned("bootstrap_unit_shape")
            } else {
                AdmissibilityDecision::rejected_by_exact_legality("bootstrap_unit_shape")
            }
        }
        3 => {
            if telescope.kappa() == 1
                && telescope.clauses.iter().all(|clause| {
                    is_latest_library_witness_application(&clause.expr, library.len() as u32)
                })
            {
                AdmissibilityDecision::admitted_focus_aligned("bootstrap_type_shape")
            } else {
                AdmissibilityDecision::rejected_by_exact_legality("bootstrap_type_shape")
            }
        }
        _ => assess_strict_admissibility_from_terminal_summary(
            step_index,
            telescope.kappa() as u16,
            admissibility,
            matched_families,
            telescope.is_trivially_derivable(library),
        ),
    }
}

pub fn assess_strict_admissibility_from_terminal_summary(
    step_index: u32,
    clause_kappa: u16,
    admissibility: StrictAdmissibility,
    matched_families: StructuralFamilyMatchMask,
    trivially_derivable: bool,
) -> AdmissibilityDecision {
    debug_assert!(step_index > 3);

    if !admissibility.supports_exact_clause_kappa(clause_kappa) {
        return AdmissibilityDecision::rejected_by_exact_legality("outside_exact_kappa_band");
    }

    if trivially_derivable {
        return AdmissibilityDecision::rejected_by_exact_legality("trivially_derivable");
    }

    if let Some(family) = admissibility.required_focus_family() {
        if !matched_families.matches(family) {
            return AdmissibilityDecision::rejected_by_structural_debt_cap(format!(
                "missing_required_{}",
                family.slug()
            ));
        }
    }

    for family in StructuralFamily::ALL {
        if matched_families.matches(family)
            && matches!(admissibility.policy_for(family), PackagePolicy::Forbid)
        {
            return AdmissibilityDecision::rejected_by_structural_debt_cap(format!(
                "forbidden_{}",
                family.slug()
            ));
        }
    }

    let matches_allowed_family = StructuralFamily::ALL.into_iter().any(|family| {
        matched_families.matches(family)
            && !matches!(admissibility.policy_for(family), PackagePolicy::Forbid)
    });
    if admissibility.focus_family.is_some() && !matches_allowed_family {
        return AdmissibilityDecision::rejected_by_structural_debt_cap(
            "no_allowed_structural_family",
        );
    }

    match admissibility.focus_family {
        Some(family) if matched_families.matches(family) => {
            AdmissibilityDecision::admitted_focus_aligned(format!("focus_{}", family.slug()))
        }
        Some(family) if admissibility.policy_for(family).is_focus() => {
            AdmissibilityDecision::admitted_deprioritized(format!("off_focus_{}", family.slug()))
        }
        Some(_) | None => AdmissibilityDecision::admitted_focus_aligned("open_band_structural"),
    }
}

pub fn strict_admissibility(
    step_index: u32,
    window_depth: u16,
    library: &Library,
) -> StrictAdmissibility {
    strict_admissibility_for_mode(
        step_index,
        window_depth,
        library,
        AdmissibilityMode::Guarded,
    )
}

pub fn strict_admissibility_for_mode(
    step_index: u32,
    window_depth: u16,
    library: &Library,
    mode: AdmissibilityMode,
) -> StrictAdmissibility {
    match step_index {
        1 => StrictAdmissibility {
            mode,
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
            package_policies: PackagePolicies::default(),
            focus_family: None,
            historical_anchor_ref: None,
        },
        2 => StrictAdmissibility {
            mode,
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
            package_policies: PackagePolicies::default(),
            focus_family: None,
            historical_anchor_ref: None,
        },
        3 => StrictAdmissibility {
            mode,
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
            package_policies: PackagePolicies::default(),
            focus_family: None,
            historical_anchor_ref: None,
        },
        _ => {
            let debt = summarize_structural_debt(library, window_depth);
            let loop_anchor = historical_loop_anchor_ref(library, window_depth);
            let modal_anchor = historical_modal_shell_anchor_ref(library, window_depth);
            let focus_family = focus_family_from_debt(debt, loop_anchor, modal_anchor);
            let focus_policy = focus_policy_for_mode(mode, focus_family);
            let package_policies = PackagePolicies::with_focus(focus_family, focus_policy);
            let (min_clause_kappa, max_clause_kappa) =
                clause_band_for_mode(mode, focus_family, debt);

            StrictAdmissibility {
                mode,
                min_clause_kappa,
                max_clause_kappa,
                ambient_depth: 2,
                max_expr_nodes: max_expr_nodes_for_mode(mode, focus_family, max_clause_kappa),
                max_path_dimension: max_path_dimension_for_focus(focus_family, debt),
                include_trunc: include_trunc_for_focus(focus_family, debt),
                include_modal: include_modal_for_focus(focus_family, debt),
                include_temporal: include_temporal_for_focus(focus_family, debt),
                quota_per_bucket: debt.quota_per_bucket(),
                require_former_eliminator_package: package_policies.former_eliminator.is_required(),
                require_initial_hit_package: package_policies.initial_hit.is_required(),
                require_truncation_hit_package: package_policies.truncation_hit.is_required(),
                require_higher_hit_package: package_policies.higher_hit.is_required(),
                require_sphere_lift_package: package_policies.sphere_lift.is_required(),
                require_axiomatic_bundle_package: package_policies.axiomatic_bundle.is_required(),
                require_modal_shell_package: package_policies.modal_shell.is_required(),
                require_connection_shell_package: package_policies.connection_shell.is_required(),
                require_curvature_shell_package: package_policies.curvature_shell.is_required(),
                require_operator_bundle_package: package_policies.operator_bundle.is_required(),
                require_hilbert_functional_package: package_policies
                    .hilbert_functional
                    .is_required(),
                require_temporal_shell_package: package_policies.temporal_shell.is_required(),
                package_policies,
                focus_family,
                historical_anchor_ref: historical_anchor_ref_for_focus(
                    mode,
                    focus_family,
                    loop_anchor,
                    modal_anchor,
                ),
            }
        }
    }
}

fn focus_family_from_debt(
    debt: crate::obligations::StructuralDebt,
    loop_anchor: Option<u32>,
    modal_anchor: Option<u32>,
) -> Option<StructuralFamily> {
    if debt.requires_former_eliminator_package() {
        Some(StructuralFamily::FormerEliminator)
    } else if debt.requires_initial_hit_package() {
        Some(StructuralFamily::InitialHit)
    } else if debt.requires_truncation_hit_package() {
        Some(StructuralFamily::TruncationHit)
    } else if debt.requires_higher_hit_package() {
        Some(StructuralFamily::HigherHit)
    } else if debt.requires_sphere_lift_package() {
        Some(StructuralFamily::SphereLift)
    } else if debt.requires_axiomatic_bundle_package() && loop_anchor.is_some() {
        Some(StructuralFamily::AxiomaticBundle)
    } else if debt.requires_modal_shell_package() {
        Some(StructuralFamily::ModalShell)
    } else if debt.requires_connection_shell_package() {
        Some(StructuralFamily::ConnectionShell)
    } else if debt.requires_curvature_shell_package() {
        Some(StructuralFamily::CurvatureShell)
    } else if debt.requires_operator_bundle_package() {
        Some(StructuralFamily::OperatorBundle)
    } else if debt.requires_hilbert_functional_package() {
        Some(StructuralFamily::HilbertFunctional)
    } else if debt.requires_temporal_shell_package() && modal_anchor.is_some() {
        Some(StructuralFamily::TemporalShell)
    } else {
        None
    }
}

fn focus_policy_for_mode(
    mode: AdmissibilityMode,
    focus_family: Option<StructuralFamily>,
) -> PackagePolicy {
    match focus_family {
        Some(family) if can_relax_focus_family(mode, family) => PackagePolicy::Prefer,
        Some(_) => PackagePolicy::Require,
        None => PackagePolicy::Allow,
    }
}

fn can_relax_focus_family(mode: AdmissibilityMode, family: StructuralFamily) -> bool {
    match mode {
        AdmissibilityMode::Guarded => false,
        AdmissibilityMode::RelaxedShadow => matches!(
            family,
            StructuralFamily::AxiomaticBundle
                | StructuralFamily::ModalShell
                | StructuralFamily::ConnectionShell
                | StructuralFamily::CurvatureShell
        ),
        AdmissibilityMode::RealisticShadow
        | AdmissibilityMode::DemoBreadthShadow
        | AdmissibilityMode::DesktopClaimShadow => matches!(
            family,
            StructuralFamily::AxiomaticBundle
                | StructuralFamily::ModalShell
                | StructuralFamily::ConnectionShell
                | StructuralFamily::CurvatureShell
                | StructuralFamily::OperatorBundle
                | StructuralFamily::HilbertFunctional
                | StructuralFamily::TemporalShell
        ),
    }
}

fn clause_band_for_mode(
    mode: AdmissibilityMode,
    focus_family: Option<StructuralFamily>,
    debt: crate::obligations::StructuralDebt,
) -> (u16, u16) {
    match focus_family {
        Some(
            StructuralFamily::FormerEliminator
            | StructuralFamily::InitialHit
            | StructuralFamily::TruncationHit
            | StructuralFamily::HigherHit,
        ) => (3, 3),
        Some(StructuralFamily::SphereLift) => (5, 5),
        Some(StructuralFamily::AxiomaticBundle | StructuralFamily::ModalShell) => (4, 4),
        Some(StructuralFamily::ConnectionShell)
            if matches!(
                mode,
                AdmissibilityMode::RelaxedShadow
                    | AdmissibilityMode::RealisticShadow
                    | AdmissibilityMode::DemoBreadthShadow
                    | AdmissibilityMode::DesktopClaimShadow
            ) =>
        {
            (5, 6)
        }
        Some(StructuralFamily::ConnectionShell) => (5, 5),
        Some(StructuralFamily::CurvatureShell) => (5, 6),
        Some(StructuralFamily::OperatorBundle) => (7, 7),
        Some(StructuralFamily::HilbertFunctional) => (9, 9),
        Some(StructuralFamily::TemporalShell) => (8, 8),
        None => {
            let min_clause_kappa =
                if debt.max_path_dimension > 0 || debt.has_modal_ops || debt.has_temporal_ops {
                    2
                } else {
                    1
                };
            (
                min_clause_kappa,
                debt.exact_kappa_cap().max(min_clause_kappa).clamp(2, 9),
            )
        }
    }
}

fn max_expr_nodes_for_mode(
    mode: AdmissibilityMode,
    focus_family: Option<StructuralFamily>,
    max_clause_kappa: u16,
) -> u8 {
    let focused_cap = match focus_family {
        Some(StructuralFamily::FormerEliminator) => 5,
        Some(StructuralFamily::InitialHit) => 3,
        Some(StructuralFamily::TruncationHit) => 4,
        Some(StructuralFamily::HigherHit) => 3,
        Some(StructuralFamily::SphereLift) => 3,
        Some(StructuralFamily::AxiomaticBundle) => 4,
        Some(StructuralFamily::ModalShell) => 2,
        Some(StructuralFamily::ConnectionShell | StructuralFamily::CurvatureShell) => 5,
        Some(
            StructuralFamily::OperatorBundle
            | StructuralFamily::HilbertFunctional
            | StructuralFamily::TemporalShell,
        ) => 7,
        None if max_clause_kappa <= 3 => 5,
        None => 6,
    };

    match (mode, focus_family) {
        (
            AdmissibilityMode::RelaxedShadow
            | AdmissibilityMode::RealisticShadow
            | AdmissibilityMode::DemoBreadthShadow
            | AdmissibilityMode::DesktopClaimShadow,
            Some(StructuralFamily::AxiomaticBundle),
        ) => focused_cap.max(4),
        _ => focused_cap,
    }
}

fn max_path_dimension_for_focus(
    focus_family: Option<StructuralFamily>,
    debt: crate::obligations::StructuralDebt,
) -> u32 {
    match focus_family {
        Some(StructuralFamily::FormerEliminator) => 0,
        Some(StructuralFamily::InitialHit | StructuralFamily::TruncationHit) => 1,
        Some(StructuralFamily::HigherHit) => 2,
        Some(StructuralFamily::SphereLift) => 3,
        Some(
            StructuralFamily::AxiomaticBundle
            | StructuralFamily::ModalShell
            | StructuralFamily::ConnectionShell
            | StructuralFamily::CurvatureShell
            | StructuralFamily::OperatorBundle
            | StructuralFamily::HilbertFunctional
            | StructuralFamily::TemporalShell,
        ) => 0,
        None => u32::from(debt.max_path_dimension).max(1),
    }
}

fn include_trunc_for_focus(
    focus_family: Option<StructuralFamily>,
    debt: crate::obligations::StructuralDebt,
) -> bool {
    matches!(focus_family, Some(StructuralFamily::TruncationHit))
        || (focus_family.is_none() && debt.max_path_dimension > 0)
}

fn include_modal_for_focus(
    focus_family: Option<StructuralFamily>,
    debt: crate::obligations::StructuralDebt,
) -> bool {
    match focus_family {
        Some(StructuralFamily::TemporalShell) => true,
        Some(StructuralFamily::OperatorBundle | StructuralFamily::HilbertFunctional) => false,
        Some(
            StructuralFamily::ModalShell
            | StructuralFamily::ConnectionShell
            | StructuralFamily::CurvatureShell,
        ) => true,
        Some(_) | None => debt.has_modal_ops,
    }
}

fn include_temporal_for_focus(
    focus_family: Option<StructuralFamily>,
    debt: crate::obligations::StructuralDebt,
) -> bool {
    matches!(focus_family, Some(StructuralFamily::TemporalShell)) || debt.has_temporal_ops
}

fn historical_anchor_ref_for_focus(
    mode: AdmissibilityMode,
    focus_family: Option<StructuralFamily>,
    loop_anchor: Option<u32>,
    modal_anchor: Option<u32>,
) -> Option<u32> {
    match (mode, focus_family) {
        (
            AdmissibilityMode::RelaxedShadow
            | AdmissibilityMode::RealisticShadow
            | AdmissibilityMode::DemoBreadthShadow
            | AdmissibilityMode::DesktopClaimShadow,
            Some(StructuralFamily::ModalShell),
        ) => loop_anchor,
        (_, Some(StructuralFamily::AxiomaticBundle)) => loop_anchor,
        (_, Some(StructuralFamily::TemporalShell)) => modal_anchor,
        (_, Some(_)) | (_, None) => None,
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct PackageMatchProfile {
    former_eliminator: bool,
    initial_hit: bool,
    truncation_hit: bool,
    higher_hit: bool,
    sphere_lift: bool,
    axiomatic_bundle: bool,
    modal_shell: bool,
    connection_shell: bool,
    curvature_shell: bool,
    operator_bundle: bool,
    hilbert_functional: bool,
    temporal_shell: bool,
}

impl PackageMatchProfile {
    fn matches(self, family: StructuralFamily) -> bool {
        match family {
            StructuralFamily::FormerEliminator => self.former_eliminator,
            StructuralFamily::InitialHit => self.initial_hit,
            StructuralFamily::TruncationHit => self.truncation_hit,
            StructuralFamily::HigherHit => self.higher_hit,
            StructuralFamily::SphereLift => self.sphere_lift,
            StructuralFamily::AxiomaticBundle => self.axiomatic_bundle,
            StructuralFamily::ModalShell => self.modal_shell,
            StructuralFamily::ConnectionShell => self.connection_shell,
            StructuralFamily::CurvatureShell => self.curvature_shell,
            StructuralFamily::OperatorBundle => self.operator_bundle,
            StructuralFamily::HilbertFunctional => self.hilbert_functional,
            StructuralFamily::TemporalShell => self.temporal_shell,
        }
    }

    fn to_match_mask(self) -> StructuralFamilyMatchMask {
        let mut mask = StructuralFamilyMatchMask::empty();
        for family in StructuralFamily::ALL {
            if self.matches(family) {
                mask.insert(family);
            }
        }
        mask
    }
}

fn package_match_profile(
    library: &Library,
    telescope: &Telescope,
    historical_anchor_ref: Option<u32>,
    mode: AdmissibilityMode,
) -> PackageMatchProfile {
    PackageMatchProfile {
        former_eliminator: matches_former_eliminator_package(telescope),
        initial_hit: matches_initial_hit_package(telescope),
        truncation_hit: matches_truncation_hit_package(telescope),
        higher_hit: matches_higher_hit_package(telescope),
        sphere_lift: matches_sphere_lift_package(telescope),
        axiomatic_bundle: matches_axiomatic_bundle_package(
            library,
            telescope,
            historical_anchor_ref,
        ),
        modal_shell: matches_modal_shell_package(telescope),
        connection_shell: matches_connection_shell_package(library, telescope),
        curvature_shell: matches_curvature_shell_package(library, telescope),
        operator_bundle: matches_operator_bundle_package(library, telescope, mode),
        hilbert_functional: matches_hilbert_functional_package(library, telescope, mode),
        temporal_shell: matches_temporal_shell_package(
            library,
            telescope,
            historical_anchor_ref,
            mode,
        ),
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

fn is_small_var_expr(expr: &Expr, max_index: u32) -> bool {
    matches!(expr, Expr::Var(index) if *index <= max_index)
}

fn matches_operator_bundle_package(
    library: &Library,
    telescope: &Telescope,
    mode: AdmissibilityMode,
) -> bool {
    let latest = library.len() as u32;
    if latest < 2 {
        return false;
    }
    let previous = latest - 1;

    if matches!(
        mode,
        AdmissibilityMode::RealisticShadow | AdmissibilityMode::DesktopClaimShadow
    ) {
        return telescope.path_dimensions().is_empty()
            && telescope.clauses.len() == 7
            && matches!(
                &telescope.clauses[0].expr,
                Expr::Sigma(left, right)
                    if matches!(
                        left.as_ref(),
                        Expr::Pi(domain, codomain)
                            if is_small_var_expr(domain.as_ref(), 2)
                                && is_small_var_expr(codomain.as_ref(), 2)
                    ) && matches!(
                        right.as_ref(),
                        Expr::Pi(domain, codomain)
                            if is_small_var_expr(domain.as_ref(), 2)
                                && is_small_var_expr(codomain.as_ref(), 2)
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
                            if matches!(domain.as_ref(), Expr::Var(1) | Expr::Var(2))
                                && matches!(codomain.as_ref(), Expr::Var(1))
                    )
            )
            && matches!(
                &telescope.clauses[6].expr,
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                        && matches!(codomain.as_ref(), Expr::Var(1))
            );
    }

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

fn matches_hilbert_functional_package(
    library: &Library,
    telescope: &Telescope,
    mode: AdmissibilityMode,
) -> bool {
    let latest = library.len() as u32;
    if latest < 3 {
        return false;
    }
    let previous = latest - 1;
    let older = latest - 2;

    if matches!(
        mode,
        AdmissibilityMode::RealisticShadow | AdmissibilityMode::DesktopClaimShadow
    ) {
        return telescope.path_dimensions().is_empty()
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
                    if matches!(
                        domain.as_ref(),
                        Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1))
                    ) && matches!(
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
                            if matches!(domain.as_ref(), Expr::Var(1) | Expr::Var(2))
                                && matches!(codomain.as_ref(), Expr::Univ)
                    )
            );
    }

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
    mode: AdmissibilityMode,
) -> bool {
    let Some(anchor) = historical_anchor_ref else {
        return false;
    };
    if !library.iter().any(|entry| entry.capabilities.has_hilbert) {
        return false;
    }

    if matches!(
        mode,
        AdmissibilityMode::RealisticShadow | AdmissibilityMode::DesktopClaimShadow
    ) {
        return telescope.path_dimensions().is_empty()
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
                    if matches!(
                        domain.as_ref(),
                        Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
                    ) && matches!(
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
                    ) && (
                        matches!(
                            codomain.as_ref(),
                            Expr::Next(body)
                                if matches!(
                                    body.as_ref(),
                                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                                )
                        )
                        || matches!(
                            codomain.as_ref(),
                            Expr::Next(body)
                                if matches!(
                                    body.as_ref(),
                                    Expr::Flat(inner)
                                        if matches!(
                                            inner.as_ref(),
                                            Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                        )
                                )
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
            );
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
    use super::{
        AdmissibilityMode, PackagePolicies, PackagePolicy, StrictAdmissibility, StructuralFamily,
        passes_strict_admissibility, strict_admissibility, strict_admissibility_for_mode,
    };
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
                mode: AdmissibilityMode::Guarded,
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
                package_policies: PackagePolicies::default(),
                focus_family: None,
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
    fn relaxed_shadow_demotes_mid_late_focus_to_preference_without_widening_the_step_band() {
        let library = library_until(9);
        let admissibility =
            strict_admissibility_for_mode(10, 2, &library, AdmissibilityMode::RelaxedShadow);

        assert_eq!(admissibility.mode, AdmissibilityMode::RelaxedShadow);
        assert_eq!(
            admissibility.focus_family,
            Some(StructuralFamily::ModalShell)
        );
        assert_eq!(
            admissibility.package_policies.modal_shell,
            PackagePolicy::Prefer
        );
        assert!(!admissibility.require_modal_shell_package);
        assert_eq!(admissibility.min_clause_kappa, 4);
        assert_eq!(admissibility.max_clause_kappa, 4);
        assert!(admissibility.include_modal);
    }

    #[test]
    fn relaxed_shadow_widens_connection_step_to_allow_curvature_competition() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RelaxedShadow);

        assert_eq!(admissibility.mode, AdmissibilityMode::RelaxedShadow);
        assert_eq!(
            admissibility.focus_family,
            Some(StructuralFamily::ConnectionShell)
        );
        assert_eq!(
            admissibility.package_policies.connection_shell,
            PackagePolicy::Prefer
        );
        assert!(!admissibility.require_connection_shell_package);
        assert_eq!(admissibility.min_clause_kappa, 5);
        assert_eq!(admissibility.max_clause_kappa, 6);
        assert!(admissibility.include_modal);
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
