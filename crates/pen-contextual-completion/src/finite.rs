use crate::context::{VerifiedContextProjectionV1, VerifiedProjectionReindexingV1};
use crate::model::{AuditDecisionV1, AuditFailureV1, FiniteProjectionInputV1};
use pen_kernel::Digest;
use std::collections::BTreeSet;

/// Exhaustive powerset checking is bounded to keep certificate size and replay
/// cost finite and transparent.
pub const MAX_FINITE_ATOMS: u8 = 6;

#[derive(Clone, Debug)]
pub struct VerifiedFiniteProjectionModelV1 {
    input: FiniteProjectionInputV1,
    digest: Digest,
}

impl VerifiedFiniteProjectionModelV1 {
    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn context_projection_digest(&self) -> &Digest {
        &self.input.context_projection_digest
    }

    pub fn reindexing_action_digest(&self) -> &Digest {
        &self.input.reindexing_action_digest
    }

    pub fn base_atom_count(&self) -> u8 {
        self.input.base_atoms.len() as u8
    }

    pub fn extension_atom_count(&self) -> u8 {
        self.input.extension_atoms.len() as u8
    }

    pub fn base_subset_count(&self) -> u16 {
        1_u16 << self.base_atom_count()
    }

    pub fn extension_subset_count(&self) -> u16 {
        1_u16 << self.extension_atom_count()
    }

    pub(crate) fn valid_base_mask(&self, mask: u16) -> bool {
        mask < self.base_subset_count()
    }

    pub(crate) fn valid_extension_mask(&self, mask: u16) -> bool {
        mask < self.extension_subset_count()
    }

    /// Inverse image along the finite projection.
    pub fn reindex(&self, base_mask: u16) -> Option<u16> {
        if !self.valid_base_mask(base_mask) {
            return None;
        }
        let mut result = 0_u16;
        for (extension_index, base_index) in self.input.projection.iter().copied().enumerate() {
            if base_mask & (1_u16 << base_index) != 0 {
                result |= 1_u16 << extension_index;
            }
        }
        Some(result)
    }

    /// The mathematically forced object table for a left universal operation
    /// in this finite powerset-preorder interpretation.
    pub(crate) fn expected_left(&self, extension_mask: u16) -> Option<u16> {
        if !self.valid_extension_mask(extension_mask) {
            return None;
        }
        let mut result = 0_u16;
        for (extension_index, base_index) in self.input.projection.iter().copied().enumerate() {
            if extension_mask & (1_u16 << extension_index) != 0 {
                result |= 1_u16 << base_index;
            }
        }
        Some(result)
    }

    /// The mathematically forced object table for a right universal operation
    /// in this finite powerset-preorder interpretation.
    pub(crate) fn expected_right(&self, extension_mask: u16) -> Option<u16> {
        if !self.valid_extension_mask(extension_mask) {
            return None;
        }
        let mut result = 0_u16;
        for base_index in 0..self.base_atom_count() {
            let fiber_is_contained = self.input.projection.iter().copied().enumerate().all(
                |(extension_index, image)| {
                    image != base_index || extension_mask & (1_u16 << extension_index) != 0
                },
            );
            if fiber_is_contained {
                result |= 1_u16 << base_index;
            }
        }
        Some(result)
    }
}

fn digest_json<T: serde::Serialize>(domain: &str, value: &T) -> Digest {
    let bytes = serde_json::to_vec(value).expect("finite input serializes");
    Digest::of_domain_bytes(domain, &bytes)
}

/// Bind an anonymous finite interpretation to one already-verified context
/// projection and one already-verified nonidentity reindexing action.
pub fn verify_finite_projection_model(
    projection: &VerifiedContextProjectionV1,
    action: &VerifiedProjectionReindexingV1,
    input: &FiniteProjectionInputV1,
) -> AuditDecisionV1<VerifiedFiniteProjectionModelV1> {
    if action.projection_digest() != projection.digest()
        || &input.context_projection_digest != projection.digest()
        || &input.reindexing_action_digest != action.action_digest()
    {
        return AuditDecisionV1::Unknown(AuditFailureV1::UnboundFiniteInterpretation);
    }
    if input.base_atoms.is_empty()
        || input.extension_atoms.is_empty()
        || input.base_atoms.len() > usize::from(MAX_FINITE_ATOMS)
        || input.extension_atoms.len() > usize::from(MAX_FINITE_ATOMS)
        || input.projection.len() != input.extension_atoms.len()
    {
        return AuditDecisionV1::OutsideFragment(AuditFailureV1::InvalidFiniteCarrier);
    }
    if input.base_atoms.iter().collect::<BTreeSet<_>>().len() != input.base_atoms.len()
        || input.extension_atoms.iter().collect::<BTreeSet<_>>().len()
            != input.extension_atoms.len()
    {
        return AuditDecisionV1::Unknown(AuditFailureV1::InvalidFiniteCarrier);
    }
    if input
        .projection
        .iter()
        .any(|image| usize::from(*image) >= input.base_atoms.len())
    {
        return AuditDecisionV1::Unknown(AuditFailureV1::InvalidFiniteProjection);
    }
    let digest = digest_json("generic-finite-projection-model-v1", input);
    AuditDecisionV1::Proven(VerifiedFiniteProjectionModelV1 {
        input: input.clone(),
        digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{
        VerifiedProjectionReindexingV1, verify_projection_reindexing,
        verify_single_extension_projection,
    };
    use crate::model::SealedPublicJudgmentV1;
    use pen_kernel::{
        DependentContext, Kernel, KernelLimits, OpenJudgment, Term, UncheckedSignature,
        VerifiedSignature,
    };

    fn context_action() -> (VerifiedContextProjectionV1, VerifiedProjectionReindexingV1) {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature: VerifiedSignature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("signature");
        let base = DependentContext(vec![Term::UnitType]);
        let extension = DependentContext(vec![Term::UnitType, Term::UnitType]);
        let projection = verify_single_extension_projection(&kernel, &signature, &base, &extension)
            .proven()
            .expect("projection");
        let public = SealedPublicJudgmentV1 {
            judgment: OpenJudgment::HasType {
                context: base,
                term: Term::Var { index: 0 },
                ty: Term::UnitType,
            },
            birth_digest: Digest::of_domain_bytes("generic-birth", b"a"),
            type_support_digest: Digest::of_domain_bytes("generic-support", b"a"),
            sealed: true,
            public: true,
        };
        let action = verify_projection_reindexing(&kernel, &signature, &projection, &public)
            .proven()
            .expect("action");
        (projection, action)
    }

    #[test]
    fn finite_projection_has_genuine_inverse_image_action() {
        let (projection, action) = context_action();
        let input = FiniteProjectionInputV1 {
            context_projection_digest: projection.digest().clone(),
            reindexing_action_digest: action.action_digest().clone(),
            base_atoms: vec![Digest::of_domain_bytes("base-atom", b"0")],
            extension_atoms: vec![
                Digest::of_domain_bytes("extension-atom", b"0"),
                Digest::of_domain_bytes("extension-atom", b"1"),
            ],
            projection: vec![0, 0],
        };
        let model = verify_finite_projection_model(&projection, &action, &input)
            .proven()
            .expect("finite projection");
        assert_eq!(model.reindex(0), Some(0));
        assert_eq!(model.reindex(1), Some(3));
        assert_eq!(
            (0..4)
                .map(|mask| model.expected_left(mask).expect("mask"))
                .collect::<Vec<_>>(),
            vec![0, 1, 1, 1]
        );
        assert_eq!(
            (0..4)
                .map(|mask| model.expected_right(mask).expect("mask"))
                .collect::<Vec<_>>(),
            vec![0, 0, 0, 1]
        );
    }

    #[test]
    fn anonymous_identifier_permutation_does_not_change_semantic_tables() {
        let (projection, action) = context_action();
        let base_zero = Digest::of_domain_bytes("base-atom", b"0");
        let extension_zero = Digest::of_domain_bytes("extension-atom", b"0");
        let extension_one = Digest::of_domain_bytes("extension-atom", b"1");
        let first = FiniteProjectionInputV1 {
            context_projection_digest: projection.digest().clone(),
            reindexing_action_digest: action.action_digest().clone(),
            base_atoms: vec![base_zero],
            extension_atoms: vec![extension_zero.clone(), extension_one.clone()],
            projection: vec![0, 0],
        };
        let second = FiniteProjectionInputV1 {
            context_projection_digest: projection.digest().clone(),
            reindexing_action_digest: action.action_digest().clone(),
            base_atoms: vec![Digest::of_domain_bytes("renamed-base-atom", b"x")],
            extension_atoms: vec![extension_one, extension_zero],
            projection: vec![0, 0],
        };
        let first = verify_finite_projection_model(&projection, &action, &first)
            .proven()
            .expect("first presentation");
        let second = verify_finite_projection_model(&projection, &action, &second)
            .proven()
            .expect("second presentation");
        assert_ne!(first.digest(), second.digest());
        assert_eq!(
            (0..first.extension_subset_count())
                .map(|mask| first.expected_left(mask))
                .collect::<Vec<_>>(),
            (0..second.extension_subset_count())
                .map(|mask| second.expected_left(mask))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            (0..first.extension_subset_count())
                .map(|mask| first.expected_right(mask))
                .collect::<Vec<_>>(),
            (0..second.extension_subset_count())
                .map(|mask| second.expected_right(mask))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn malformed_or_unbound_models_fail_closed() {
        let (projection, action) = context_action();
        let mut input = FiniteProjectionInputV1 {
            context_projection_digest: projection.digest().clone(),
            reindexing_action_digest: action.action_digest().clone(),
            base_atoms: vec![Digest::of_domain_bytes("base-atom", b"0")],
            extension_atoms: vec![Digest::of_domain_bytes("extension-atom", b"0")],
            projection: vec![1],
        };
        assert!(matches!(
            verify_finite_projection_model(&projection, &action, &input),
            AuditDecisionV1::Unknown(AuditFailureV1::InvalidFiniteProjection)
        ));
        input.projection = vec![0];
        input.context_projection_digest =
            Digest::of_domain_bytes("wrong-context-projection", b"wrong");
        assert!(matches!(
            verify_finite_projection_model(&projection, &action, &input),
            AuditDecisionV1::Unknown(AuditFailureV1::UnboundFiniteInterpretation)
        ));
    }
}
