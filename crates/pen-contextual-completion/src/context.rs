use crate::model::{AuditDecisionV1, AuditFailureV1, SealedPublicJudgmentV1};
use pen_kernel::{
    DependentContext, Digest, Kernel, KernelError, OpenJudgment, ResourceKind, Term,
    VerifiedSignature,
};

#[derive(Clone, Debug)]
pub struct VerifiedContextProjectionV1 {
    base: DependentContext,
    extension: DependentContext,
    projection_images: Vec<Term>,
    digest: Digest,
}

impl VerifiedContextProjectionV1 {
    pub fn base(&self) -> &DependentContext {
        &self.base
    }

    pub fn extension(&self) -> &DependentContext {
        &self.extension
    }

    /// Images are ordered by base de Bruijn index, newest variable first.
    pub fn projection_images(&self) -> &[Term] {
        &self.projection_images
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedProjectionReindexingV1 {
    projection_digest: Digest,
    source: OpenJudgment,
    reindexed: OpenJudgment,
    action_digest: Digest,
}

impl VerifiedProjectionReindexingV1 {
    pub fn projection_digest(&self) -> &Digest {
        &self.projection_digest
    }

    pub fn source(&self) -> &OpenJudgment {
        &self.source
    }

    pub fn reindexed(&self) -> &OpenJudgment {
        &self.reindexed
    }

    pub fn action_digest(&self) -> &Digest {
        &self.action_digest
    }
}

fn kernel_failure<T>(error: KernelError) -> AuditDecisionV1<T> {
    match error {
        KernelError::ResourceExhausted(
            ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
        ) => AuditDecisionV1::ResourceExhausted(AuditFailureV1::KernelResourceExhausted(
            error.to_string(),
        )),
        _ => AuditDecisionV1::Unknown(AuditFailureV1::KernelRejected(error.to_string())),
    }
}

fn shift(term: &Term, amount: u32, cutoff: u32) -> Option<Term> {
    Some(match term {
        Term::Sort { level } => Term::Sort { level: *level },
        Term::Var { index } => Term::Var {
            index: if *index >= cutoff {
                index.checked_add(amount)?
            } else {
                *index
            },
        },
        Term::Global { id } => Term::Global { id: id.clone() },
        Term::Pi { parameter, body } => Term::Pi {
            parameter: Box::new(shift(parameter, amount, cutoff)?),
            body: Box::new(shift(body, amount, cutoff.checked_add(1)?)?),
        },
        Term::Sigma { parameter, body } => Term::Sigma {
            parameter: Box::new(shift(parameter, amount, cutoff)?),
            body: Box::new(shift(body, amount, cutoff.checked_add(1)?)?),
        },
        Term::Lambda {
            parameter_type,
            body,
        } => Term::Lambda {
            parameter_type: Box::new(shift(parameter_type, amount, cutoff)?),
            body: Box::new(shift(body, amount, cutoff.checked_add(1)?)?),
        },
        Term::Apply { function, argument } => Term::Apply {
            function: Box::new(shift(function, amount, cutoff)?),
            argument: Box::new(shift(argument, amount, cutoff)?),
        },
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Term::Pair {
            sigma_type: Box::new(shift(sigma_type, amount, cutoff)?),
            first: Box::new(shift(first, amount, cutoff)?),
            second: Box::new(shift(second, amount, cutoff)?),
        },
        Term::First { pair } => Term::First {
            pair: Box::new(shift(pair, amount, cutoff)?),
        },
        Term::Second { pair } => Term::Second {
            pair: Box::new(shift(pair, amount, cutoff)?),
        },
        Term::UnitType => Term::UnitType,
        Term::Unit => Term::Unit,
    })
}

fn shift_judgment(judgment: &OpenJudgment, context: DependentContext) -> Option<OpenJudgment> {
    Some(match judgment {
        OpenJudgment::TypeFormation { term, .. } => OpenJudgment::TypeFormation {
            context,
            term: shift(term, 1, 0)?,
        },
        OpenJudgment::HasType { term, ty, .. } => OpenJudgment::HasType {
            context,
            term: shift(term, 1, 0)?,
            ty: shift(ty, 1, 0)?,
        },
        OpenJudgment::DefinitionallyEqual {
            left, right, ty, ..
        } => OpenJudgment::DefinitionallyEqual {
            context,
            left: shift(left, 1, 0)?,
            right: shift(right, 1, 0)?,
            ty: shift(ty, 1, 0)?,
        },
    })
}

fn judgment_payload_changed(source: &OpenJudgment, target: &OpenJudgment) -> bool {
    match (source, target) {
        (
            OpenJudgment::TypeFormation { term: source, .. },
            OpenJudgment::TypeFormation { term: target, .. },
        ) => source != target,
        (
            OpenJudgment::HasType {
                term: source_term,
                ty: source_type,
                ..
            },
            OpenJudgment::HasType {
                term: target_term,
                ty: target_type,
                ..
            },
        ) => source_term != target_term || source_type != target_type,
        (
            OpenJudgment::DefinitionallyEqual {
                left: source_left,
                right: source_right,
                ty: source_type,
                ..
            },
            OpenJudgment::DefinitionallyEqual {
                left: target_left,
                right: target_right,
                ty: target_type,
                ..
            },
        ) => {
            source_left != target_left || source_right != target_right || source_type != target_type
        }
        _ => true,
    }
}

fn digest_json<T: serde::Serialize>(domain: &str, value: &T) -> Digest {
    let bytes = serde_json::to_vec(value).expect("verified prototype data serializes");
    Digest::of_domain_bytes(domain, &bytes)
}

/// Verify an exact prefix-preserving one-entry extension and all structural
/// projection images using the production kernel's public judgment checker.
pub fn verify_single_extension_projection(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    base: &DependentContext,
    extension: &DependentContext,
) -> AuditDecisionV1<VerifiedContextProjectionV1> {
    let verified_base = match kernel.verify_context(signature, base) {
        Ok(value) => value,
        Err(error) => return kernel_failure(error),
    };
    let verified_extension = match kernel.verify_context(signature, extension) {
        Ok(value) => value,
        Err(error) => return kernel_failure(error),
    };
    let base = verified_base.normalized_wire();
    let extension = verified_extension.normalized_wire();
    if extension.0.len() != base.0.len().saturating_add(1)
        || extension.0.get(..base.0.len()) != Some(base.0.as_slice())
    {
        return AuditDecisionV1::OutsideFragment(AuditFailureV1::NotSingleExtension);
    }

    let mut projection_images = Vec::with_capacity(base.0.len());
    for index in 0..base.0.len() {
        let Ok(index) = u32::try_from(index) else {
            return AuditDecisionV1::ResourceExhausted(AuditFailureV1::UnsupportedReindexingSyntax);
        };
        let image = Term::Var {
            index: match index.checked_add(1) {
                Some(value) => value,
                None => {
                    return AuditDecisionV1::ResourceExhausted(
                        AuditFailureV1::UnsupportedReindexingSyntax,
                    );
                }
            },
        };
        let position = base.0.len() - 1 - index as usize;
        let Some(expected_type) = shift(&base.0[position], index + 2, 0) else {
            return AuditDecisionV1::ResourceExhausted(AuditFailureV1::UnsupportedReindexingSyntax);
        };
        let image_judgment = OpenJudgment::HasType {
            context: extension.clone(),
            term: image.clone(),
            ty: expected_type,
        };
        if let Err(error) = kernel.verify_open_judgment(signature, &image_judgment) {
            return kernel_failure(error);
        }
        projection_images.push(image);
    }

    let digest = digest_json(
        "generic-context-projection-v1",
        &(
            verified_base.digest(),
            verified_extension.digest(),
            &projection_images,
        ),
    );
    AuditDecisionV1::Proven(VerifiedContextProjectionV1 {
        base,
        extension,
        projection_images,
        digest,
    })
}

/// Check that a sealed public judgment was genuinely and nontrivially
/// reindexed along a verified one-entry projection.
pub fn verify_projection_reindexing(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    projection: &VerifiedContextProjectionV1,
    input: &SealedPublicJudgmentV1,
) -> AuditDecisionV1<VerifiedProjectionReindexingV1> {
    if !input.sealed || !input.public {
        return AuditDecisionV1::Unknown(AuditFailureV1::NotSealedPublic);
    }
    if input.judgment.context() != projection.base() {
        return AuditDecisionV1::OutsideFragment(AuditFailureV1::ContextMismatch);
    }
    let source = match kernel.verify_open_judgment(signature, &input.judgment) {
        Ok(value) => value,
        Err(error) => return kernel_failure(error),
    };
    let Some(target_wire) = shift_judgment(&source, projection.extension().clone()) else {
        return AuditDecisionV1::ResourceExhausted(AuditFailureV1::UnsupportedReindexingSyntax);
    };
    let reindexed = match kernel.verify_open_judgment(signature, &target_wire) {
        Ok(value) => value,
        Err(error) => return kernel_failure(error),
    };
    if !judgment_payload_changed(&source, &reindexed) {
        return AuditDecisionV1::OutsideFragment(AuditFailureV1::TrivialAction);
    }
    let source_digest = Digest::of_canonical("generic-public-source-v1", &source);
    let target_digest = Digest::of_canonical("generic-public-reindexed-v1", &reindexed);
    if source == reindexed || source_digest == target_digest {
        return AuditDecisionV1::OutsideFragment(AuditFailureV1::TrivialAction);
    }
    let action_digest = digest_json(
        "generic-projection-reindexing-v1",
        &(
            projection.digest(),
            source_digest,
            target_digest,
            &input.birth_digest,
            &input.type_support_digest,
        ),
    );
    AuditDecisionV1::Proven(VerifiedProjectionReindexingV1 {
        projection_digest: projection.digest().clone(),
        source,
        reindexed,
        action_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_kernel::{KernelLimits, UncheckedSignature};

    fn kernel_and_signature() -> (Kernel, VerifiedSignature) {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("signature");
        (kernel, signature)
    }

    fn public_judgment(context: DependentContext) -> SealedPublicJudgmentV1 {
        SealedPublicJudgmentV1 {
            judgment: OpenJudgment::HasType {
                context,
                term: Term::Var { index: 0 },
                ty: Term::UnitType,
            },
            birth_digest: Digest::of_domain_bytes("generic-birth", b"birth"),
            type_support_digest: Digest::of_domain_bytes("generic-type-support", b"unit"),
            sealed: true,
            public: true,
        }
    }

    #[test]
    fn verifies_projection_and_genuine_nonidentity_reindexing() {
        let (kernel, signature) = kernel_and_signature();
        let base = DependentContext(vec![Term::UnitType]);
        let extension = DependentContext(vec![Term::UnitType, Term::UnitType]);
        let projection = verify_single_extension_projection(&kernel, &signature, &base, &extension)
            .proven()
            .expect("projection");
        assert_eq!(projection.projection_images(), &[Term::Var { index: 1 }]);

        let action =
            verify_projection_reindexing(&kernel, &signature, &projection, &public_judgment(base))
                .proven()
                .expect("nonidentity action");
        let OpenJudgment::HasType { term, .. } = action.reindexed() else {
            panic!("expected has-type judgment");
        };
        assert_eq!(term, &Term::Var { index: 1 });
        assert_ne!(action.source(), action.reindexed());
    }

    #[test]
    fn identity_and_unsealed_inputs_fail_closed() {
        let (kernel, signature) = kernel_and_signature();
        let base = DependentContext(vec![Term::UnitType]);
        assert!(matches!(
            verify_single_extension_projection(&kernel, &signature, &base, &base),
            AuditDecisionV1::OutsideFragment(AuditFailureV1::NotSingleExtension)
        ));

        let extension = DependentContext(vec![Term::UnitType, Term::UnitType]);
        let projection = verify_single_extension_projection(&kernel, &signature, &base, &extension)
            .proven()
            .expect("projection");
        let mut input = public_judgment(base);
        input.sealed = false;
        assert!(matches!(
            verify_projection_reindexing(&kernel, &signature, &projection, &input),
            AuditDecisionV1::Unknown(AuditFailureV1::NotSealedPublic)
        ));

        let irrelevant = SealedPublicJudgmentV1 {
            judgment: OpenJudgment::HasType {
                context: projection.base().clone(),
                term: Term::Unit,
                ty: Term::UnitType,
            },
            birth_digest: Digest::of_domain_bytes("generic-birth", b"closed"),
            type_support_digest: Digest::of_domain_bytes("generic-type-support", b"unit"),
            sealed: true,
            public: true,
        };
        assert!(matches!(
            verify_projection_reindexing(&kernel, &signature, &projection, &irrelevant),
            AuditDecisionV1::OutsideFragment(AuditFailureV1::TrivialAction)
        ));
    }

    #[test]
    fn dependent_prefix_projection_is_checked() {
        let (kernel, signature) = kernel_and_signature();
        let base = DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]);
        let extension = DependentContext(vec![
            Term::Sort { level: 0 },
            Term::Var { index: 0 },
            Term::UnitType,
        ]);
        let projection = verify_single_extension_projection(&kernel, &signature, &base, &extension)
            .proven()
            .expect("dependent projection");
        assert_eq!(
            projection.projection_images(),
            &[Term::Var { index: 1 }, Term::Var { index: 2 }]
        );
    }
}
