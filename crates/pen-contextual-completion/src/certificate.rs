use crate::finite::VerifiedFiniteProjectionModelV1;
use crate::manifest::{CONTEXTUAL_CALCULUS_ID_V1, CONTEXTUAL_PROFILE_ID_V1};
use crate::model::{
    AnonymousOperationIdV1, AuditDecisionV1, AuditFailureV1, ContextSideV1, HomMorphismV1,
    PolarityV1, SubsetObjectV1,
};
use pen_kernel::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceExposureV1 {
    /// Public only inside the anonymous generic fixture.
    GenericFixturePublic,
    /// The same generic interface was already certified; do not reseal it.
    AlreadyCertifiedGenericPublic,
    /// Exists only inside verifier implementation code and cannot pay.
    VerifierOnly,
}

/// An explicit equivalence between the two thin hom-sets for one object pair.
/// `None` represents an empty hom-set; a present morphism is the unique
/// inhabitant in this finite preorder.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HomEquivalenceCellV1 {
    pub extension_object: u16,
    pub base_object: u16,
    pub universal_side_hom: Option<HomMorphismV1>,
    pub reindexing_side_hom: Option<HomMorphismV1>,
}

/// Explicit two-leg triangle data. The verifier recomputes both legs and
/// checks that their composite has the identity's exact endpoints.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriangleIdentityV1 {
    pub indexed_by: SubsetObjectV1,
    pub identity_on: SubsetObjectV1,
    pub first_leg: HomMorphismV1,
    pub second_leg: HomMorphismV1,
    pub asserted_equal_to_identity: bool,
}

/// Untrusted wire certificate for one anonymous universal polarity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnonymousUniversalCertificateV1 {
    pub profile_id: String,
    pub calculus_id: String,
    pub generic_data_only: bool,
    pub production_payment_authorized: bool,
    pub model_digest: Digest,
    pub context_projection_digest: Digest,
    pub reindexing_action_digest: Digest,
    pub polarity: PolarityV1,
    pub operation_id: AnonymousOperationIdV1,
    /// Indexed by extension-subset mask; each value is a base-subset mask.
    pub operation_table: Vec<u16>,
    pub unit: Vec<HomMorphismV1>,
    pub counit: Vec<HomMorphismV1>,
    pub hom_equivalence: Vec<HomEquivalenceCellV1>,
    pub triangle_on_left_functor: Vec<TriangleIdentityV1>,
    pub triangle_on_right_functor: Vec<TriangleIdentityV1>,
    pub exposure: InterfaceExposureV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwoSidedCertificateInputV1 {
    pub left: Option<AnonymousUniversalCertificateV1>,
    pub right: Option<AnonymousUniversalCertificateV1>,
}

#[derive(Clone, Debug)]
pub struct VerifiedTwoSidedCompletionV1 {
    model_digest: Digest,
    left_operation: AnonymousOperationIdV1,
    right_operation: AnonymousOperationIdV1,
    certificate_digest: Digest,
    already_public: bool,
    production_payment_authorized: bool,
}

impl VerifiedTwoSidedCompletionV1 {
    pub fn model_digest(&self) -> &Digest {
        &self.model_digest
    }

    pub fn left_operation(&self) -> &AnonymousOperationIdV1 {
        &self.left_operation
    }

    pub fn right_operation(&self) -> &AnonymousOperationIdV1 {
        &self.right_operation
    }

    pub fn certificate_digest(&self) -> &Digest {
        &self.certificate_digest
    }

    pub fn already_public(&self) -> bool {
        self.already_public
    }

    /// Always false in this proposed generic crate.
    pub fn production_payment_authorized(&self) -> bool {
        self.production_payment_authorized
    }
}

fn is_hom(
    side: ContextSideV1,
    source: u16,
    target: u16,
    model: &VerifiedFiniteProjectionModelV1,
) -> bool {
    let masks_valid = match side {
        ContextSideV1::Base => model.valid_base_mask(source) && model.valid_base_mask(target),
        ContextSideV1::Extension => {
            model.valid_extension_mask(source) && model.valid_extension_mask(target)
        }
    };
    masks_valid && source & !target == 0
}

fn hom(
    side: ContextSideV1,
    source: u16,
    target: u16,
    model: &VerifiedFiniteProjectionModelV1,
) -> Option<HomMorphismV1> {
    is_hom(side, source, target, model).then_some(HomMorphismV1 {
        side,
        source,
        target,
    })
}

fn expected_operation(model: &VerifiedFiniteProjectionModelV1, polarity: PolarityV1) -> Vec<u16> {
    (0..model.extension_subset_count())
        .map(|mask| match polarity {
            PolarityV1::LeftOfReindexing => model.expected_left(mask),
            PolarityV1::RightOfReindexing => model.expected_right(mask),
        })
        .map(|value| value.expect("enumerated mask is valid"))
        .collect()
}

fn expected_unit(
    model: &VerifiedFiniteProjectionModelV1,
    polarity: PolarityV1,
    operation: &[u16],
) -> Vec<HomMorphismV1> {
    match polarity {
        PolarityV1::LeftOfReindexing => (0..model.extension_subset_count())
            .map(|extension| {
                let target = model
                    .reindex(operation[usize::from(extension)])
                    .expect("operation output is valid");
                hom(ContextSideV1::Extension, extension, target, model)
                    .expect("left unit follows from finite projection")
            })
            .collect(),
        PolarityV1::RightOfReindexing => (0..model.base_subset_count())
            .map(|base| {
                let extension = model.reindex(base).expect("base mask is valid");
                let target = operation[usize::from(extension)];
                hom(ContextSideV1::Base, base, target, model)
                    .expect("right unit follows from finite projection")
            })
            .collect(),
    }
}

fn expected_counit(
    model: &VerifiedFiniteProjectionModelV1,
    polarity: PolarityV1,
    operation: &[u16],
) -> Vec<HomMorphismV1> {
    match polarity {
        PolarityV1::LeftOfReindexing => (0..model.base_subset_count())
            .map(|base| {
                let extension = model.reindex(base).expect("base mask is valid");
                let source = operation[usize::from(extension)];
                hom(ContextSideV1::Base, source, base, model)
                    .expect("left counit follows from finite projection")
            })
            .collect(),
        PolarityV1::RightOfReindexing => (0..model.extension_subset_count())
            .map(|extension| {
                let base = operation[usize::from(extension)];
                let source = model.reindex(base).expect("operation output is valid");
                hom(ContextSideV1::Extension, source, extension, model)
                    .expect("right counit follows from finite projection")
            })
            .collect(),
    }
}

fn expected_hom_equivalence(
    model: &VerifiedFiniteProjectionModelV1,
    polarity: PolarityV1,
    operation: &[u16],
) -> Vec<HomEquivalenceCellV1> {
    let mut cells = Vec::with_capacity(
        usize::from(model.extension_subset_count()) * usize::from(model.base_subset_count()),
    );
    for extension in 0..model.extension_subset_count() {
        for base in 0..model.base_subset_count() {
            let reindexed = model.reindex(base).expect("base mask is valid");
            let (universal_side_hom, reindexing_side_hom) = match polarity {
                PolarityV1::LeftOfReindexing => (
                    hom(
                        ContextSideV1::Base,
                        operation[usize::from(extension)],
                        base,
                        model,
                    ),
                    hom(ContextSideV1::Extension, extension, reindexed, model),
                ),
                PolarityV1::RightOfReindexing => (
                    hom(ContextSideV1::Extension, reindexed, extension, model),
                    hom(
                        ContextSideV1::Base,
                        base,
                        operation[usize::from(extension)],
                        model,
                    ),
                ),
            };
            debug_assert_eq!(universal_side_hom.is_some(), reindexing_side_hom.is_some());
            cells.push(HomEquivalenceCellV1 {
                extension_object: extension,
                base_object: base,
                universal_side_hom,
                reindexing_side_hom,
            });
        }
    }
    cells
}

fn triangle(
    indexed_by: SubsetObjectV1,
    identity_on: SubsetObjectV1,
    midpoint: u16,
    model: &VerifiedFiniteProjectionModelV1,
) -> TriangleIdentityV1 {
    let first_leg = hom(identity_on.side, identity_on.mask, midpoint, model)
        .expect("first triangle leg follows from adjunction");
    let second_leg = hom(identity_on.side, midpoint, identity_on.mask, model)
        .expect("second triangle leg follows from adjunction");
    TriangleIdentityV1 {
        indexed_by,
        identity_on,
        first_leg,
        second_leg,
        asserted_equal_to_identity: true,
    }
}

fn expected_triangles(
    model: &VerifiedFiniteProjectionModelV1,
    polarity: PolarityV1,
    operation: &[u16],
) -> (Vec<TriangleIdentityV1>, Vec<TriangleIdentityV1>) {
    match polarity {
        PolarityV1::LeftOfReindexing => {
            let on_left = (0..model.extension_subset_count())
                .map(|extension| {
                    let universal = operation[usize::from(extension)];
                    let reindexed_universal =
                        model.reindex(universal).expect("universal mask is valid");
                    let midpoint = operation[usize::from(reindexed_universal)];
                    triangle(
                        SubsetObjectV1 {
                            side: ContextSideV1::Extension,
                            mask: extension,
                        },
                        SubsetObjectV1 {
                            side: ContextSideV1::Base,
                            mask: universal,
                        },
                        midpoint,
                        model,
                    )
                })
                .collect();
            let on_right = (0..model.base_subset_count())
                .map(|base| {
                    let reindexed = model.reindex(base).expect("base mask is valid");
                    let universal_reindexed = operation[usize::from(reindexed)];
                    let midpoint = model
                        .reindex(universal_reindexed)
                        .expect("universal mask is valid");
                    triangle(
                        SubsetObjectV1 {
                            side: ContextSideV1::Base,
                            mask: base,
                        },
                        SubsetObjectV1 {
                            side: ContextSideV1::Extension,
                            mask: reindexed,
                        },
                        midpoint,
                        model,
                    )
                })
                .collect();
            (on_left, on_right)
        }
        PolarityV1::RightOfReindexing => {
            let on_left = (0..model.base_subset_count())
                .map(|base| {
                    let reindexed = model.reindex(base).expect("base mask is valid");
                    let universal_reindexed = operation[usize::from(reindexed)];
                    let midpoint = model
                        .reindex(universal_reindexed)
                        .expect("universal mask is valid");
                    triangle(
                        SubsetObjectV1 {
                            side: ContextSideV1::Base,
                            mask: base,
                        },
                        SubsetObjectV1 {
                            side: ContextSideV1::Extension,
                            mask: reindexed,
                        },
                        midpoint,
                        model,
                    )
                })
                .collect();
            let on_right = (0..model.extension_subset_count())
                .map(|extension| {
                    let universal = operation[usize::from(extension)];
                    let reindexed_universal =
                        model.reindex(universal).expect("universal mask is valid");
                    let midpoint = operation[usize::from(reindexed_universal)];
                    triangle(
                        SubsetObjectV1 {
                            side: ContextSideV1::Extension,
                            mask: extension,
                        },
                        SubsetObjectV1 {
                            side: ContextSideV1::Base,
                            mask: universal,
                        },
                        midpoint,
                        model,
                    )
                })
                .collect();
            (on_left, on_right)
        }
    }
}

fn verify_one(
    model: &VerifiedFiniteProjectionModelV1,
    certificate: &AnonymousUniversalCertificateV1,
    expected_polarity: PolarityV1,
) -> Result<(), AuditFailureV1> {
    if certificate.production_payment_authorized {
        return Err(AuditFailureV1::ProductionAuthorityUnavailable);
    }
    if certificate.profile_id != CONTEXTUAL_PROFILE_ID_V1
        || certificate.calculus_id != CONTEXTUAL_CALCULUS_ID_V1
        || !certificate.generic_data_only
        || &certificate.model_digest != model.digest()
        || &certificate.context_projection_digest != model.context_projection_digest()
        || &certificate.reindexing_action_digest != model.reindexing_action_digest()
        || certificate.polarity != expected_polarity
    {
        return Err(AuditFailureV1::CertificateBindingMismatch);
    }
    if certificate.exposure == InterfaceExposureV1::VerifierOnly {
        return Err(AuditFailureV1::AmbientOnlyInterface);
    }
    let operation = expected_operation(model, expected_polarity);
    if certificate.operation_table != operation {
        return Err(AuditFailureV1::OperationTableMismatch);
    }
    if certificate.unit != expected_unit(model, expected_polarity, &operation) {
        return Err(AuditFailureV1::UnitMismatch);
    }
    if certificate.counit != expected_counit(model, expected_polarity, &operation) {
        return Err(AuditFailureV1::CounitMismatch);
    }
    if certificate.hom_equivalence != expected_hom_equivalence(model, expected_polarity, &operation)
    {
        return Err(AuditFailureV1::HomEquivalenceMismatch);
    }
    let (left_triangle, right_triangle) = expected_triangles(model, expected_polarity, &operation);
    if certificate.triangle_on_left_functor != left_triangle
        || certificate.triangle_on_right_functor != right_triangle
        || certificate
            .triangle_on_left_functor
            .iter()
            .chain(&certificate.triangle_on_right_functor)
            .any(|triangle| {
                !triangle.asserted_equal_to_identity
                    || triangle.first_leg.source != triangle.identity_on.mask
                    || triangle.second_leg.target != triangle.identity_on.mask
                    || triangle.first_leg.target != triangle.second_leg.source
            })
    {
        return Err(AuditFailureV1::TriangleIdentityMismatch);
    }
    Ok(())
}

/// Exhaustively replay both universal directions. One-sided input, ambient-only
/// interfaces, malformed equations, or any omitted cell fail closed.
pub fn verify_two_sided_completion(
    model: &VerifiedFiniteProjectionModelV1,
    input: &TwoSidedCertificateInputV1,
) -> AuditDecisionV1<VerifiedTwoSidedCompletionV1> {
    let (Some(left), Some(right)) = (&input.left, &input.right) else {
        return AuditDecisionV1::Unknown(AuditFailureV1::TwoSidedCompletionRequired);
    };
    if let Err(error) = verify_one(model, left, PolarityV1::LeftOfReindexing) {
        return AuditDecisionV1::Unknown(error);
    }
    if let Err(error) = verify_one(model, right, PolarityV1::RightOfReindexing) {
        return AuditDecisionV1::Unknown(error);
    }
    let already_public = matches!(
        (left.exposure, right.exposure),
        (
            InterfaceExposureV1::AlreadyCertifiedGenericPublic,
            InterfaceExposureV1::AlreadyCertifiedGenericPublic
        )
    );
    let bytes = serde_json::to_vec(input).expect("verified certificates serialize");
    let verified = VerifiedTwoSidedCompletionV1 {
        model_digest: model.digest().clone(),
        left_operation: left.operation_id.clone(),
        right_operation: right.operation_id.clone(),
        certificate_digest: Digest::of_domain_bytes(
            "generic-two-sided-contextual-completion-v1",
            &bytes,
        ),
        already_public,
        production_payment_authorized: false,
    };
    if already_public {
        AuditDecisionV1::Derived(verified)
    } else {
        AuditDecisionV1::Proven(verified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{
        VerifiedContextProjectionV1, VerifiedProjectionReindexingV1, verify_projection_reindexing,
        verify_single_extension_projection,
    };
    use crate::finite::verify_finite_projection_model;
    use crate::model::{FiniteProjectionInputV1, SealedPublicJudgmentV1};
    use pen_kernel::{
        DependentContext, Kernel, KernelLimits, OpenJudgment, Term, UncheckedSignature,
        VerifiedSignature,
    };

    fn bound_model() -> VerifiedFiniteProjectionModelV1 {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature: VerifiedSignature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("signature");
        let base = DependentContext(vec![Term::UnitType]);
        let extension = DependentContext(vec![Term::UnitType, Term::UnitType]);
        let projection: VerifiedContextProjectionV1 =
            verify_single_extension_projection(&kernel, &signature, &base, &extension)
                .proven()
                .expect("projection");
        let public = SealedPublicJudgmentV1 {
            judgment: OpenJudgment::HasType {
                context: base,
                term: Term::Var { index: 0 },
                ty: Term::UnitType,
            },
            birth_digest: Digest::of_domain_bytes("generic-birth", b"subject"),
            type_support_digest: Digest::of_domain_bytes("generic-support", b"unit"),
            sealed: true,
            public: true,
        };
        let action: VerifiedProjectionReindexingV1 =
            verify_projection_reindexing(&kernel, &signature, &projection, &public)
                .proven()
                .expect("action");
        let input = FiniteProjectionInputV1 {
            context_projection_digest: projection.digest().clone(),
            reindexing_action_digest: action.action_digest().clone(),
            base_atoms: vec![
                Digest::of_domain_bytes("base-atom", b"0"),
                Digest::of_domain_bytes("base-atom", b"1"),
            ],
            extension_atoms: vec![
                Digest::of_domain_bytes("extension-atom", b"0"),
                Digest::of_domain_bytes("extension-atom", b"1"),
                Digest::of_domain_bytes("extension-atom", b"2"),
            ],
            projection: vec![0, 0, 1],
        };
        verify_finite_projection_model(&projection, &action, &input)
            .proven()
            .expect("finite model")
    }

    fn complete_certificate(
        model: &VerifiedFiniteProjectionModelV1,
        polarity: PolarityV1,
        exposure: InterfaceExposureV1,
    ) -> AnonymousUniversalCertificateV1 {
        let operation = expected_operation(model, polarity);
        let (triangle_on_left_functor, triangle_on_right_functor) =
            expected_triangles(model, polarity, &operation);
        AnonymousUniversalCertificateV1 {
            profile_id: CONTEXTUAL_PROFILE_ID_V1.to_owned(),
            calculus_id: CONTEXTUAL_CALCULUS_ID_V1.to_owned(),
            generic_data_only: true,
            production_payment_authorized: false,
            model_digest: model.digest().clone(),
            context_projection_digest: model.context_projection_digest().clone(),
            reindexing_action_digest: model.reindexing_action_digest().clone(),
            polarity,
            operation_id: AnonymousOperationIdV1(Digest::of_domain_bytes(
                "anonymous-operation",
                match polarity {
                    PolarityV1::LeftOfReindexing => b"left",
                    PolarityV1::RightOfReindexing => b"right",
                },
            )),
            operation_table: operation.clone(),
            unit: expected_unit(model, polarity, &operation),
            counit: expected_counit(model, polarity, &operation),
            hom_equivalence: expected_hom_equivalence(model, polarity, &operation),
            triangle_on_left_functor,
            triangle_on_right_functor,
            exposure,
        }
    }

    fn complete_pair(model: &VerifiedFiniteProjectionModelV1) -> TwoSidedCertificateInputV1 {
        TwoSidedCertificateInputV1 {
            left: Some(complete_certificate(
                model,
                PolarityV1::LeftOfReindexing,
                InterfaceExposureV1::GenericFixturePublic,
            )),
            right: Some(complete_certificate(
                model,
                PolarityV1::RightOfReindexing,
                InterfaceExposureV1::GenericFixturePublic,
            )),
        }
    }

    #[test]
    fn explicit_two_sided_universal_data_replays() {
        let model = bound_model();
        let pair = complete_pair(&model);
        let verified = verify_two_sided_completion(&model, &pair)
            .proven()
            .expect("complete two-sided certificate");
        assert_eq!(verified.model_digest(), model.digest());
        assert!(!verified.already_public());
        assert!(!verified.production_payment_authorized());
        assert_ne!(verified.left_operation(), verified.right_operation());
    }

    #[test]
    fn one_sided_and_ambient_only_interfaces_fail_closed() {
        let model = bound_model();
        let mut pair = complete_pair(&model);
        pair.right = None;
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::TwoSidedCompletionRequired)
        ));

        let mut pair = complete_pair(&model);
        pair.left.as_mut().expect("left").exposure = InterfaceExposureV1::VerifierOnly;
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::AmbientOnlyInterface)
        ));
    }

    #[test]
    fn mutations_of_every_universal_layer_fail_closed() {
        let model = bound_model();

        let mut pair = complete_pair(&model);
        pair.left.as_mut().expect("left").operation_table[0] ^= 1;
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::OperationTableMismatch)
        ));

        let mut pair = complete_pair(&model);
        pair.left.as_mut().expect("left").unit.pop();
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::UnitMismatch)
        ));

        let mut pair = complete_pair(&model);
        pair.right.as_mut().expect("right").counit.pop();
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::CounitMismatch)
        ));

        let mut pair = complete_pair(&model);
        pair.left.as_mut().expect("left").hom_equivalence.pop();
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::HomEquivalenceMismatch)
        ));

        let mut pair = complete_pair(&model);
        pair.right
            .as_mut()
            .expect("right")
            .triangle_on_right_functor[0]
            .asserted_equal_to_identity = false;
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::TriangleIdentityMismatch)
        ));
    }

    #[test]
    fn already_public_pair_is_derived_without_resealing() {
        let model = bound_model();
        let pair = TwoSidedCertificateInputV1 {
            left: Some(complete_certificate(
                &model,
                PolarityV1::LeftOfReindexing,
                InterfaceExposureV1::AlreadyCertifiedGenericPublic,
            )),
            right: Some(complete_certificate(
                &model,
                PolarityV1::RightOfReindexing,
                InterfaceExposureV1::AlreadyCertifiedGenericPublic,
            )),
        };
        let AuditDecisionV1::Derived(verified) = verify_two_sided_completion(&model, &pair) else {
            panic!("expected derived result");
        };
        assert!(verified.already_public());
    }

    #[test]
    fn production_authority_claim_is_rejected() {
        let model = bound_model();
        let mut pair = complete_pair(&model);
        pair.left
            .as_mut()
            .expect("left")
            .production_payment_authorized = true;
        assert!(matches!(
            verify_two_sided_completion(&model, &pair),
            AuditDecisionV1::Unknown(AuditFailureV1::ProductionAuthorityUnavailable)
        ));
    }
}
