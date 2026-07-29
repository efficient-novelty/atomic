use serde::{Deserialize, Serialize};

pub const CONTEXTUAL_PROFILE_ID_V1: &str = "gsc-contextual-internalization-v1";
pub const CONTEXTUAL_CALCULUS_ID_V1: &str = "finite-contextual-completion-core-v1";

/// Machine-readable authority boundary for this research crate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProposedContextualManifestV1 {
    pub profile_id: String,
    pub calculus_id: String,
    pub status: String,
    pub authority: String,
    pub generic_data_only: bool,
    pub live_profile_a_access: bool,
    pub registered_prefix_access: bool,
    pub candidate_generation: bool,
    pub production_payment_authorized: bool,
    pub max_base_atoms: u8,
    pub max_extension_atoms: u8,
}

pub fn proposed_contextual_manifest_v1() -> ProposedContextualManifestV1 {
    ProposedContextualManifestV1 {
        profile_id: CONTEXTUAL_PROFILE_ID_V1.to_owned(),
        calculus_id: CONTEXTUAL_CALCULUS_ID_V1.to_owned(),
        status: "draft_proposed_not_adopted".to_owned(),
        authority: "generic_falsification_only".to_owned(),
        generic_data_only: true,
        live_profile_a_access: false,
        registered_prefix_access: false,
        candidate_generation: false,
        production_payment_authorized: false,
        max_base_atoms: crate::finite::MAX_FINITE_ATOMS,
        max_extension_atoms: crate::finite::MAX_FINITE_ATOMS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_is_fail_closed() {
        let manifest = proposed_contextual_manifest_v1();
        assert_eq!(manifest.status, "draft_proposed_not_adopted");
        assert!(manifest.generic_data_only);
        assert!(!manifest.live_profile_a_access);
        assert!(!manifest.registered_prefix_access);
        assert!(!manifest.candidate_generation);
        assert!(!manifest.production_payment_authorized);
    }
}
