use pen_core::library::Library;

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
    pub has_modal_ops: bool,
    pub has_temporal_ops: bool,
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
        {
            return 64;
        }

        usize::from(self.active_entries.max(2))
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
            acc.has_modal_ops |= entry.capabilities.has_modal_ops;
            acc.has_temporal_ops |= entry.capabilities.has_temporal_ops;
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{StructuralDebt, summarize_structural_debt};
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;

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
}
