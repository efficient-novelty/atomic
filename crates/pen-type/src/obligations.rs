use pen_core::library::Library;
use pen_core::telescope::TelescopeClass;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionFocus {
    #[default]
    OpenBand,
    Former,
    Hit,
    Axiomatic,
    Modal,
    Temporal,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    RareFocusHead,
    RareBridgeHead,
    StructuralSupport,
    #[default]
    GenericMacro,
}

impl RetentionClass {
    pub const fn priority_rank(self) -> u8 {
        match self {
            Self::RareFocusHead => 0,
            Self::RareBridgeHead => 1,
            Self::StructuralSupport => 2,
            Self::GenericMacro => 3,
        }
    }

    pub const fn is_rare_head(self) -> bool {
        matches!(self, Self::RareFocusHead | Self::RareBridgeHead)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RetentionSignals {
    pub telescope_class: TelescopeClass,
    pub eliminator_score: u16,
    pub former_score: u16,
    pub dependent_motive_density: u16,
    pub library_reference_density: u16,
    pub generic_binder_count: u16,
    pub closure_score: u16,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RetentionPolicy {
    pub focus: RetentionFocus,
    pub focus_quota: usize,
    pub bridge_quota: usize,
    pub support_quota: usize,
    pub macro_quota: usize,
    pub cold_limit: usize,
}

impl RetentionPolicy {
    pub fn quota_for(self, class: RetentionClass) -> usize {
        match class {
            RetentionClass::RareFocusHead => self.focus_quota,
            RetentionClass::RareBridgeHead => self.bridge_quota,
            RetentionClass::StructuralSupport => self.support_quota,
            RetentionClass::GenericMacro => self.macro_quota,
        }
    }

    pub fn classify(self, signals: RetentionSignals) -> RetentionClass {
        if self.matches_focus_head(signals) {
            return RetentionClass::RareFocusHead;
        }
        if self.matches_bridge_head(signals) {
            return RetentionClass::RareBridgeHead;
        }
        if self.looks_structural_support(signals) {
            return RetentionClass::StructuralSupport;
        }
        RetentionClass::GenericMacro
    }

    fn matches_focus_head(self, signals: RetentionSignals) -> bool {
        match self.focus {
            RetentionFocus::OpenBand => false,
            RetentionFocus::Former => {
                matches!(
                    signals.telescope_class,
                    TelescopeClass::Former | TelescopeClass::Foundation
                ) || (signals.former_score > 0 && signals.eliminator_score > 0)
            }
            RetentionFocus::Hit => {
                matches!(
                    signals.telescope_class,
                    TelescopeClass::Hit | TelescopeClass::Suspension
                ) || (signals.eliminator_score > 0 && signals.closure_score > 1)
            }
            RetentionFocus::Axiomatic => matches!(
                signals.telescope_class,
                TelescopeClass::Axiomatic | TelescopeClass::Map
            ),
            RetentionFocus::Modal => signals.telescope_class == TelescopeClass::Modal,
            RetentionFocus::Temporal => signals.telescope_class == TelescopeClass::Synthesis,
        }
    }

    fn matches_bridge_head(self, signals: RetentionSignals) -> bool {
        match self.focus {
            RetentionFocus::OpenBand => false,
            RetentionFocus::Former => {
                signals.telescope_class == TelescopeClass::Hit
                    || (signals.telescope_class == TelescopeClass::Unknown
                        && signals.eliminator_score > 0)
            }
            RetentionFocus::Hit => matches!(
                signals.telescope_class,
                TelescopeClass::Former | TelescopeClass::Foundation
            ),
            RetentionFocus::Axiomatic => matches!(
                signals.telescope_class,
                TelescopeClass::Modal | TelescopeClass::Former | TelescopeClass::Map
            ),
            RetentionFocus::Modal => matches!(
                signals.telescope_class,
                TelescopeClass::Axiomatic | TelescopeClass::Map
            ),
            RetentionFocus::Temporal => matches!(
                signals.telescope_class,
                TelescopeClass::Modal | TelescopeClass::Axiomatic
            ),
        }
    }

    fn looks_structural_support(self, signals: RetentionSignals) -> bool {
        let structural_weight = u32::from(signals.eliminator_score)
            + u32::from(signals.former_score)
            + u32::from(signals.dependent_motive_density)
            + u32::from(signals.library_reference_density)
            + u32::from(signals.closure_score);
        let macro_pressure = u32::from(signals.generic_binder_count);

        structural_weight > macro_pressure
            || signals.library_reference_density > 0
            || matches!(
                signals.telescope_class,
                TelescopeClass::Foundation
                    | TelescopeClass::Former
                    | TelescopeClass::Hit
                    | TelescopeClass::Suspension
                    | TelescopeClass::Map
                    | TelescopeClass::Modal
                    | TelescopeClass::Axiomatic
                    | TelescopeClass::Synthesis
            )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StructuralDebt {
    pub active_entries: u16,
    pub active_exports: u16,
    pub foundation_entries: u16,
    pub constructor_entries: u16,
    pub dependent_entries: u16,
    pub truncated_entries: u16,
    pub max_path_dimension: u16,
    pub modal_entries: u16,
    pub modal_coupled_entries: u16,
    pub differential_entries: u16,
    pub differential_coupled_entries: u16,
    pub curvature_entries: u16,
    pub operator_bundle_entries: u16,
    pub hilbert_shell_entries: u16,
    pub temporal_shell_entries: u16,
    pub has_modal_ops: bool,
    pub has_temporal_ops: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClaimDebtAxes {
    pub kappa_min: u16,
    pub kappa_max: u16,
    pub path_pressure: u8,
    pub trunc_pressure: u8,
    pub coupling_pressure: u8,
    pub support_pressure: u8,
    pub modal_pressure: u8,
    pub temporal_pressure: u8,
    pub reanchor_pressure: u8,
    pub closure_pressure: u8,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ClaimAnchorPolicy {
    #[default]
    None,
    Loop,
    Modal,
}

impl StructuralDebt {
    pub fn requires_former_eliminator_package(self) -> bool {
        self.max_path_dimension == 0
            && !self.has_modal_ops
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.constructor_entries >= 2
            && self.dependent_entries == 0
    }

    pub fn requires_initial_hit_package(self) -> bool {
        self.max_path_dimension == 0
            && !self.has_modal_ops
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.constructor_entries >= 1
            && self.dependent_entries >= 1
    }

    pub fn requires_truncation_hit_package(self) -> bool {
        self.max_path_dimension == 1
            && self.truncated_entries == 0
            && !self.has_modal_ops
            && !self.has_temporal_ops
            && self.active_entries >= 2
    }

    pub fn requires_higher_hit_package(self) -> bool {
        self.max_path_dimension == 1
            && self.truncated_entries > 0
            && !self.has_modal_ops
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.constructor_entries >= 1
    }

    pub fn requires_sphere_lift_package(self) -> bool {
        self.max_path_dimension == 2
            && self.truncated_entries > 0
            && !self.has_modal_ops
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.constructor_entries >= 1
    }

    pub fn requires_axiomatic_bundle_package(self) -> bool {
        self.max_path_dimension == 3
            && self.truncated_entries == 0
            && !self.has_modal_ops
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.active_exports >= 6
            && self.constructor_entries >= 2
    }

    pub fn requires_modal_shell_package(self) -> bool {
        self.max_path_dimension == 3
            && self.truncated_entries == 0
            && !self.has_modal_ops
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.active_exports >= 4
            && self.active_exports < 6
            && self.constructor_entries == 1
    }

    pub fn requires_connection_shell_package(self) -> bool {
        self.max_path_dimension == 0
            && self.truncated_entries == 0
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.active_exports >= 2
            && self.constructor_entries == 0
            && self.modal_entries >= 1
            && self.modal_coupled_entries == 0
    }

    pub fn requires_curvature_shell_package(self) -> bool {
        self.max_path_dimension == 0
            && self.truncated_entries == 0
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.active_exports >= 2
            && self.constructor_entries == 0
            && self.modal_entries >= 1
            && self.modal_coupled_entries >= 1
            && self.differential_entries >= 1
            && self.curvature_entries == 0
    }

    pub fn requires_operator_bundle_package(self) -> bool {
        self.max_path_dimension == 0
            && self.truncated_entries == 0
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.active_exports >= 2
            && self.constructor_entries == 0
            && self.differential_coupled_entries >= 2
            && self.curvature_entries >= 1
            && self.operator_bundle_entries == 0
    }

    pub fn requires_hilbert_functional_package(self) -> bool {
        self.max_path_dimension == 0
            && self.truncated_entries == 0
            && !self.has_temporal_ops
            && self.active_entries >= 2
            && self.active_exports >= 2
            && self.constructor_entries == 0
            && self.differential_coupled_entries >= 2
            && self.curvature_entries >= 1
            && self.operator_bundle_entries >= 1
            && self.hilbert_shell_entries == 0
    }

    pub fn requires_temporal_shell_package(self) -> bool {
        self.max_path_dimension == 0
            && self.truncated_entries == 0
            && self.active_entries >= 2
            && self.active_exports >= 2
            && self.constructor_entries == 0
            && self.operator_bundle_entries >= 1
            && self.hilbert_shell_entries >= 1
            && self.temporal_shell_entries == 0
    }

    pub fn exact_kappa_cap(self) -> u16 {
        if self.requires_former_eliminator_package() {
            return 3;
        }
        if self.requires_initial_hit_package() {
            return 3;
        }
        if self.requires_truncation_hit_package() {
            return 3;
        }
        if self.requires_higher_hit_package() {
            return 3;
        }
        if self.requires_sphere_lift_package() {
            return 5;
        }
        if self.requires_axiomatic_bundle_package() {
            return 4;
        }
        if self.requires_modal_shell_package() {
            return 4;
        }
        if self.requires_connection_shell_package() {
            return 5;
        }
        if self.requires_curvature_shell_package() {
            return 6;
        }
        if self.requires_operator_bundle_package() {
            return 7;
        }
        if self.requires_hilbert_functional_package() {
            return 9;
        }
        if self.requires_temporal_shell_package() {
            return 8;
        }

        let export_pressure = self.active_exports.max(self.foundation_entries.max(1));
        let path_pressure = self.max_path_dimension.saturating_sub(1);
        let modal_pressure = u16::from(self.has_modal_ops) + u16::from(self.has_temporal_ops);
        (export_pressure + path_pressure + modal_pressure).clamp(1, 9)
    }

    pub fn quota_per_bucket(self) -> usize {
        if self.requires_former_eliminator_package()
            || self.requires_initial_hit_package()
            || self.requires_truncation_hit_package()
            || self.requires_higher_hit_package()
            || self.requires_sphere_lift_package()
            || self.requires_axiomatic_bundle_package()
            || self.requires_modal_shell_package()
            || self.requires_connection_shell_package()
            || self.requires_curvature_shell_package()
            || self.requires_operator_bundle_package()
            || self.requires_hilbert_functional_package()
            || self.requires_temporal_shell_package()
        {
            return 64;
        }

        usize::from(self.active_entries.max(2))
    }

    pub fn claim_exact_cap_hint(self) -> u16 {
        self.exact_kappa_cap().max(3)
    }

    pub fn claim_clause_band_hint(self) -> (u16, u16) {
        let exact_cap = self.claim_exact_cap_hint();
        match exact_cap {
            3 => (3, 3),
            4 => (4, 4),
            5 if self.claim_path_pressure() == 0
                && (self.claim_modal_pressure() > 0 || self.claim_coupling_pressure() > 0) =>
            {
                (5, 6)
            }
            5 => (5, 5),
            6 => (5, 6),
            7 => (7, 7),
            8 => (8, 8),
            _ => (9, 9),
        }
    }

    pub fn claim_anchor_policy(self) -> ClaimAnchorPolicy {
        let axes = self.claim_debt_axes();
        if axes.temporal_pressure > 0 {
            ClaimAnchorPolicy::Modal
        } else if axes.reanchor_pressure > 0 && axes.kappa_min <= 4 && axes.kappa_max >= 4 {
            ClaimAnchorPolicy::Loop
        } else {
            ClaimAnchorPolicy::None
        }
    }

    pub fn claim_debt_axes(self) -> ClaimDebtAxes {
        let (kappa_min, kappa_max) = self.claim_clause_band_hint();
        ClaimDebtAxes {
            kappa_min,
            kappa_max,
            path_pressure: self.claim_path_pressure(),
            trunc_pressure: u8::from(self.max_path_dimension > 0 || self.truncated_entries > 0),
            coupling_pressure: self.claim_coupling_pressure(),
            support_pressure: self.claim_support_pressure(),
            modal_pressure: self.claim_modal_pressure(),
            temporal_pressure: self.claim_temporal_pressure(),
            reanchor_pressure: self.claim_reanchor_pressure(),
            closure_pressure: self.claim_closure_pressure(),
        }
    }

    pub fn retention_focus(self) -> RetentionFocus {
        if self.requires_former_eliminator_package() || self.requires_initial_hit_package() {
            return RetentionFocus::Former;
        }
        if self.requires_truncation_hit_package()
            || self.requires_higher_hit_package()
            || self.requires_sphere_lift_package()
        {
            return RetentionFocus::Hit;
        }
        if self.requires_modal_shell_package() {
            return RetentionFocus::Modal;
        }
        if self.requires_temporal_shell_package() {
            return RetentionFocus::Temporal;
        }
        if self.requires_axiomatic_bundle_package()
            || self.requires_connection_shell_package()
            || self.requires_curvature_shell_package()
            || self.requires_operator_bundle_package()
            || self.requires_hilbert_functional_package()
        {
            return RetentionFocus::Axiomatic;
        }
        if self.has_temporal_ops {
            return RetentionFocus::Temporal;
        }
        if self.has_modal_ops {
            return RetentionFocus::Modal;
        }
        if self.max_path_dimension > 0 {
            return RetentionFocus::Hit;
        }
        RetentionFocus::OpenBand
    }

    pub fn retention_policy(self) -> RetentionPolicy {
        let focus = self.retention_focus();
        let quota_cap = self.quota_per_bucket().max(1);
        let focus_quota = match focus {
            RetentionFocus::OpenBand => 1,
            RetentionFocus::Former | RetentionFocus::Hit => 1,
            RetentionFocus::Axiomatic | RetentionFocus::Modal | RetentionFocus::Temporal => 2,
        }
        .min(quota_cap);
        let bridge_quota = match focus {
            RetentionFocus::OpenBand => 0,
            RetentionFocus::Former | RetentionFocus::Hit | RetentionFocus::Modal => 1,
            RetentionFocus::Axiomatic | RetentionFocus::Temporal => 2,
        }
        .min(quota_cap);
        let support_quota = usize::from(self.active_entries >= 2).min(quota_cap);
        let macro_quota = usize::from(matches!(focus, RetentionFocus::OpenBand)).min(quota_cap);
        let cold_limit = match focus {
            RetentionFocus::OpenBand | RetentionFocus::Former | RetentionFocus::Hit => 4,
            RetentionFocus::Axiomatic | RetentionFocus::Modal | RetentionFocus::Temporal => 6,
        };

        RetentionPolicy {
            focus,
            focus_quota,
            bridge_quota,
            support_quota,
            macro_quota,
            cold_limit,
        }
    }

    fn claim_path_pressure(self) -> u8 {
        if self.max_path_dimension == 0 && self.dependent_entries > 0 {
            1
        } else if self.max_path_dimension > 0 && self.truncated_entries > 0 {
            u8::try_from(self.max_path_dimension.saturating_add(1).min(3))
                .expect("claim path pressure exceeded u8")
        } else {
            u8::try_from(self.max_path_dimension.min(3)).expect("claim path pressure exceeded u8")
        }
    }

    fn claim_coupling_pressure(self) -> u8 {
        u8::try_from(
            self.modal_coupled_entries
                .saturating_add(self.differential_coupled_entries)
                .min(3),
        )
        .expect("claim coupling pressure exceeded u8")
    }

    fn claim_support_pressure(self) -> u8 {
        u8::try_from(
            self.active_exports
                .saturating_add(self.foundation_entries)
                .min(3),
        )
        .expect("claim support pressure exceeded u8")
    }

    fn claim_modal_pressure(self) -> u8 {
        if self.has_temporal_ops || self.temporal_shell_entries > 0 {
            2
        } else if self.has_modal_ops
            || self.modal_entries > 0
            || self.modal_coupled_entries > 0
            || self.active_exports >= 4
        {
            1
        } else {
            0
        }
    }

    fn claim_temporal_pressure(self) -> u8 {
        if self.has_temporal_ops || self.temporal_shell_entries > 0 {
            2
        } else if self.operator_bundle_entries > 0 && self.hilbert_shell_entries > 0 {
            1
        } else {
            0
        }
    }

    fn claim_reanchor_pressure(self) -> u8 {
        if self.claim_temporal_pressure() > 0 {
            2
        } else if self.max_path_dimension >= 3 || self.active_exports >= 4 {
            1
        } else {
            0
        }
    }

    fn claim_closure_pressure(self) -> u8 {
        u8::from(self.active_entries >= 2)
            + u8::from(self.active_exports >= 2)
            + u8::from(self.constructor_entries == 0)
    }
}

pub fn summarize_structural_debt(library: &Library, window_depth: u16) -> StructuralDebt {
    let depth = usize::from(window_depth.max(1));
    let start = library.len().saturating_sub(depth);
    let active = &library[start..];

    active.iter().fold(
        StructuralDebt {
            active_entries: active.len() as u16,
            ..StructuralDebt::default()
        },
        |mut acc, entry| {
            acc.active_exports = acc.active_exports.saturating_add(entry.axiomatic_exports);
            acc.foundation_entries = acc
                .foundation_entries
                .saturating_add(u16::from(entry.constructors > 0));
            acc.constructor_entries = acc
                .constructor_entries
                .saturating_add(u16::from(entry.constructors > 0));
            acc.dependent_entries = acc
                .dependent_entries
                .saturating_add(u16::from(entry.capabilities.has_dependent_functions));
            acc.truncated_entries = acc
                .truncated_entries
                .saturating_add(u16::from(entry.is_truncated.is_some()));
            acc.max_path_dimension = acc
                .max_path_dimension
                .max(entry.path_dims.iter().copied().max().unwrap_or(0) as u16);
            acc.modal_entries = acc
                .modal_entries
                .saturating_add(u16::from(entry.capabilities.has_modal_ops));
            acc.modal_coupled_entries = acc.modal_coupled_entries.saturating_add(u16::from(
                entry.capabilities.has_modal_ops && entry.library_refs > 0,
            ));
            acc.differential_entries = acc
                .differential_entries
                .saturating_add(u16::from(entry.capabilities.has_differential_ops));
            acc.differential_coupled_entries = acc.differential_coupled_entries.saturating_add(
                u16::from(entry.capabilities.has_differential_ops && entry.library_refs > 0),
            );
            acc.curvature_entries = acc
                .curvature_entries
                .saturating_add(u16::from(entry.capabilities.has_curvature));
            acc.operator_bundle_entries = acc.operator_bundle_entries.saturating_add(u16::from(
                entry.capabilities.has_metric && entry.library_refs > 0,
            ));
            acc.hilbert_shell_entries = acc
                .hilbert_shell_entries
                .saturating_add(u16::from(entry.capabilities.has_hilbert));
            acc.temporal_shell_entries = acc
                .temporal_shell_entries
                .saturating_add(u16::from(entry.capabilities.has_temporal_shell));
            acc.has_modal_ops |= entry.capabilities.has_modal_ops;
            acc.has_temporal_ops |= entry.capabilities.has_temporal_ops;
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ClaimAnchorPolicy, ClaimDebtAxes, RetentionClass, RetentionFocus, RetentionPolicy,
        RetentionSignals, StructuralDebt, summarize_structural_debt,
    };
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::{Telescope, TelescopeClass};

    #[test]
    fn debt_summary_tracks_active_window_capabilities() {
        let mut library: Library = Vec::new();
        for step in 1..=3 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        assert_eq!(
            summarize_structural_debt(&library, 2),
            StructuralDebt {
                active_entries: 2,
                active_exports: 2,
                foundation_entries: 2,
                constructor_entries: 2,
                dependent_entries: 0,
                truncated_entries: 0,
                max_path_dimension: 0,
                modal_entries: 0,
                modal_coupled_entries: 0,
                differential_entries: 0,
                differential_coupled_entries: 0,
                curvature_entries: 0,
                operator_bundle_entries: 0,
                hilbert_shell_entries: 0,
                temporal_shell_entries: 0,
                has_modal_ops: false,
                has_temporal_ops: false,
            }
        );
        assert!(summarize_structural_debt(&library, 2).requires_former_eliminator_package());
    }

    #[test]
    fn debt_summary_opens_first_truncation_package_after_s1() {
        let mut library: Library = Vec::new();
        for step in 1..=5 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.max_path_dimension, 1);
        assert_eq!(debt.truncated_entries, 0);
        assert!(debt.requires_truncation_hit_package());
    }

    #[test]
    fn debt_summary_opens_first_higher_hit_package_after_truncation() {
        let mut library: Library = Vec::new();
        for step in 1..=6 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.max_path_dimension, 1);
        assert_eq!(debt.truncated_entries, 1);
        assert!(debt.requires_higher_hit_package());
    }

    #[test]
    fn debt_summary_opens_first_sphere_lift_package_after_s2() {
        let mut library: Library = Vec::new();
        for step in 1..=7 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.max_path_dimension, 2);
        assert_eq!(debt.truncated_entries, 1);
        assert!(debt.requires_sphere_lift_package());
    }

    #[test]
    fn debt_summary_opens_first_axiomatic_bundle_package_after_s3() {
        let mut library: Library = Vec::new();
        for step in 1..=8 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.max_path_dimension, 3);
        assert_eq!(debt.truncated_entries, 0);
        assert_eq!(debt.active_exports, 6);
        assert!(debt.requires_axiomatic_bundle_package());
    }

    #[test]
    fn claim_axes_open_structural_bands_without_named_family_progression() {
        let former = StructuralDebt {
            active_entries: 2,
            constructor_entries: 2,
            ..StructuralDebt::default()
        };
        assert_eq!(
            former.claim_debt_axes(),
            ClaimDebtAxes {
                kappa_min: 3,
                kappa_max: 3,
                path_pressure: 0,
                trunc_pressure: 0,
                coupling_pressure: 0,
                support_pressure: 0,
                modal_pressure: 0,
                temporal_pressure: 0,
                reanchor_pressure: 0,
                closure_pressure: 1,
            }
        );
        assert_eq!(former.claim_anchor_policy(), ClaimAnchorPolicy::None);

        let modal_bridge = StructuralDebt {
            active_entries: 2,
            active_exports: 4,
            constructor_entries: 1,
            max_path_dimension: 3,
            ..StructuralDebt::default()
        };
        assert_eq!(
            modal_bridge.claim_debt_axes(),
            ClaimDebtAxes {
                kappa_min: 4,
                kappa_max: 4,
                path_pressure: 3,
                trunc_pressure: 1,
                coupling_pressure: 0,
                support_pressure: 3,
                modal_pressure: 1,
                temporal_pressure: 0,
                reanchor_pressure: 1,
                closure_pressure: 2,
            }
        );
        assert_eq!(modal_bridge.claim_anchor_policy(), ClaimAnchorPolicy::Loop);

        let temporal = StructuralDebt {
            active_entries: 2,
            active_exports: 2,
            operator_bundle_entries: 1,
            hilbert_shell_entries: 1,
            ..StructuralDebt::default()
        };
        assert_eq!(
            temporal.claim_debt_axes(),
            ClaimDebtAxes {
                kappa_min: 8,
                kappa_max: 8,
                path_pressure: 0,
                trunc_pressure: 0,
                coupling_pressure: 0,
                support_pressure: 2,
                modal_pressure: 0,
                temporal_pressure: 1,
                reanchor_pressure: 2,
                closure_pressure: 3,
            }
        );
        assert_eq!(temporal.claim_anchor_policy(), ClaimAnchorPolicy::Modal);
    }

    #[test]
    fn debt_summary_opens_first_modal_shell_package_after_hopf() {
        let mut library: Library = Vec::new();
        for step in 1..=9 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.max_path_dimension, 3);
        assert_eq!(debt.active_exports, 4);
        assert_eq!(debt.constructor_entries, 1);
        assert_eq!(debt.modal_entries, 0);
        assert!(debt.requires_modal_shell_package());
    }

    #[test]
    fn debt_summary_opens_first_connection_shell_after_cohesion() {
        let mut library: Library = Vec::new();
        for step in 1..=10 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.max_path_dimension, 0);
        assert_eq!(debt.active_exports, 2);
        assert_eq!(debt.constructor_entries, 0);
        assert_eq!(debt.modal_entries, 1);
        assert_eq!(debt.modal_coupled_entries, 0);
        assert!(debt.requires_connection_shell_package());
    }

    #[test]
    fn debt_summary_closes_first_connection_shell_once_modal_layer_is_coupled() {
        let mut library: Library = Vec::new();
        for step in 1..=11 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.modal_entries, 2);
        assert_eq!(debt.modal_coupled_entries, 1);
        assert!(!debt.requires_connection_shell_package());
    }

    #[test]
    fn debt_summary_opens_first_curvature_shell_after_connections() {
        let mut library: Library = Vec::new();
        for step in 1..=11 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.modal_entries, 2);
        assert_eq!(debt.modal_coupled_entries, 1);
        assert_eq!(debt.differential_entries, 1);
        assert_eq!(debt.differential_coupled_entries, 1);
        assert_eq!(debt.curvature_entries, 0);
        assert_eq!(debt.operator_bundle_entries, 0);
        assert!(debt.requires_curvature_shell_package());
    }

    #[test]
    fn debt_summary_closes_curvature_shell_once_curvature_entry_exists() {
        let mut library: Library = Vec::new();
        for step in 1..=12 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.differential_entries, 2);
        assert_eq!(debt.differential_coupled_entries, 2);
        assert_eq!(debt.curvature_entries, 1);
        assert!(!debt.requires_curvature_shell_package());
    }

    #[test]
    fn debt_summary_opens_first_operator_bundle_after_curvature() {
        let mut library: Library = Vec::new();
        for step in 1..=12 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.differential_coupled_entries, 2);
        assert_eq!(debt.curvature_entries, 1);
        assert_eq!(debt.operator_bundle_entries, 0);
        assert!(debt.requires_operator_bundle_package());
    }

    #[test]
    fn debt_summary_closes_operator_bundle_once_metric_reading_exists() {
        let mut library: Library = Vec::new();
        for step in 1..=13 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.curvature_entries, 1);
        assert_eq!(debt.operator_bundle_entries, 1);
        assert!(!debt.requires_operator_bundle_package());
    }

    #[test]
    fn debt_summary_opens_first_hilbert_shell_after_metric_bundle() {
        let mut library: Library = Vec::new();
        for step in 1..=13 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.differential_coupled_entries, 2);
        assert_eq!(debt.curvature_entries, 1);
        assert_eq!(debt.operator_bundle_entries, 1);
        assert_eq!(debt.hilbert_shell_entries, 0);
        assert!(debt.requires_hilbert_functional_package());
    }

    #[test]
    fn debt_summary_closes_hilbert_shell_once_genuine_hilbert_entry_exists() {
        let mut library: Library = Vec::new();
        for step in 1..=14 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.operator_bundle_entries, 2);
        assert_eq!(debt.hilbert_shell_entries, 1);
        assert!(!debt.requires_hilbert_functional_package());
    }

    #[test]
    fn debt_summary_opens_first_temporal_shell_after_hilbert() {
        let mut library: Library = Vec::new();
        for step in 1..=14 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.operator_bundle_entries, 2);
        assert_eq!(debt.hilbert_shell_entries, 1);
        assert_eq!(debt.temporal_shell_entries, 0);
        assert!(debt.requires_temporal_shell_package());
    }

    #[test]
    fn debt_summary_closes_temporal_shell_once_genuine_temporal_entry_exists() {
        let mut library: Library = Vec::new();
        for step in 1..=15 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        assert_eq!(debt.hilbert_shell_entries, 1);
        assert_eq!(debt.temporal_shell_entries, 1);
        assert!(!debt.requires_temporal_shell_package());
    }

    #[test]
    fn retention_policy_tracks_the_active_obligation_focus() {
        let mut library: Library = Vec::new();
        for step in 1..=14 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let debt = summarize_structural_debt(&library, 2);
        let policy = debt.retention_policy();

        assert_eq!(policy.focus, RetentionFocus::Temporal);
        assert_eq!(policy.focus_quota, 2);
        assert_eq!(policy.bridge_quota, 2);
        assert_eq!(policy.cold_limit, 6);
    }

    #[test]
    fn retention_policy_demotes_generic_macro_pressure() {
        let policy = RetentionPolicy {
            focus: RetentionFocus::Axiomatic,
            focus_quota: 2,
            bridge_quota: 1,
            support_quota: 1,
            macro_quota: 0,
            cold_limit: 6,
        };
        let macro_like = RetentionSignals {
            telescope_class: TelescopeClass::Unknown,
            generic_binder_count: 4,
            ..RetentionSignals::default()
        };
        let focused = RetentionSignals {
            telescope_class: TelescopeClass::Axiomatic,
            generic_binder_count: 4,
            library_reference_density: 2,
            ..RetentionSignals::default()
        };

        assert_eq!(policy.classify(macro_like), RetentionClass::GenericMacro);
        assert_eq!(policy.classify(focused), RetentionClass::RareFocusHead);
    }
}
