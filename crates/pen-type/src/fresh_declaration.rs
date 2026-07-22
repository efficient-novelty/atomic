//! Kernel issuance and replay for structural fresh declarations.
//!
//! The evidence in this module is the trusted bridge between the name-free
//! declaration overlay in `pen-core` and downstream semantic counting.  A
//! legacy `Telescope` by itself cannot mint a fresh head: issuance requires an
//! explicit declaration whose predecessor digest is the digest of the actual
//! checked signature.  Tokens have private fields and no deserializer.

use crate::elaborate::{
    ClauseFailure, ElabError, KernelTy, SealedSignature, candidate_hash, elaborate_single_clause,
    elaborate_telescope,
};
use pen_core::declaration::{
    ConstructorCompletionOverlay, ConstructorSlotRef, ConstructorSlotShape, DeclarationType,
    FreshHeadId, InductiveFormationOverlay,
};
use pen_core::hash::blake3_hex;
use serde::Serialize;
use thiserror::Error;

/// Version tag included in every declaration-evidence digest.
pub const FRESH_DECLARATION_RULES: &str = "fresh-declaration-rules-v1";

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum FreshDeclarationError {
    #[error("declaration overlay has an empty legacy presentation")]
    EmptyPresentation,
    #[error("declaration predecessor context does not match the checked signature")]
    PredecessorContextMismatch,
    #[error("the inductive classifier is not in the legacy expression fragment")]
    UnsupportedInductiveClassifier,
    #[error("inductive classifier failed kernel elaboration: {0}")]
    ClassifierInvalid(ElabError),
    #[error("inductive classifier does not synthesize Type")]
    ClassifierNotType,
    #[error("inductive classifier requires a coarse typing assumption")]
    CoarseClassifier,
    #[error("constructor-slot inventory exceeds the structural u16 boundary")]
    TooManyConstructorSlots,
    #[error("legacy presentation failed kernel elaboration: {0}")]
    PresentationInvalid(ClauseFailure),
    #[error("constructor completion is not in the context produced by its formation")]
    CompletionContextMismatch,
    #[error("constructor presentation signature is not the checked predecessor plus its formation")]
    PresentationSignatureNotFormationExtension,
    #[error("constructor completion does not answer a pending slot of this formation")]
    ConstructorSlotNotPending,
    #[error("constructor completion has the wrong classifier for its pending slot")]
    ConstructorClassifierMismatch,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum FreshDeclarationReplayError {
    #[error("declaration evidence no longer issues: {0}")]
    Rejected(FreshDeclarationError),
    #[error("reissued declaration evidence differs from the supplied token")]
    EvidenceMismatch,
}

/// Opaque, replayable evidence for an anonymous inductive formation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InductiveFormationEvidence {
    signature_digest: String,
    predecessor_entries: Vec<(u32, String)>,
    legacy_presentation_hash: String,
    semantic_subject_digest: String,
    declaration_digest: String,
    fresh_head: FreshHeadId,
    pending_slots: Vec<ConstructorSlotRef>,
    pending_slot_shapes: Vec<ConstructorSlotShape>,
    resulting_context_digest: String,
    classifier_kernel_ty: KernelTy,
    classifier_derivation_hash: String,
    presentation_derivation_hash: String,
    derivation_hash: String,
}

impl InductiveFormationEvidence {
    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }

    pub fn legacy_presentation_hash(&self) -> &str {
        &self.legacy_presentation_hash
    }

    pub fn predecessor_entries(&self) -> &[(u32, String)] {
        &self.predecessor_entries
    }

    pub fn semantic_subject_digest(&self) -> &str {
        &self.semantic_subject_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn fresh_head(&self) -> &FreshHeadId {
        &self.fresh_head
    }

    pub fn pending_slots(&self) -> &[ConstructorSlotRef] {
        &self.pending_slots
    }

    pub fn pending_slot_shape(&self, slot: &ConstructorSlotRef) -> Option<ConstructorSlotShape> {
        self.pending_slots
            .iter()
            .position(|pending| pending == slot)
            .and_then(|index| self.pending_slot_shapes.get(index).copied())
    }

    pub fn resulting_context_digest(&self) -> &str {
        &self.resulting_context_digest
    }

    pub fn classifier_kernel_ty(&self) -> &KernelTy {
        &self.classifier_kernel_ty
    }

    pub fn classifier_derivation_hash(&self) -> &str {
        &self.classifier_derivation_hash
    }

    pub fn presentation_derivation_hash(&self) -> &str {
        &self.presentation_derivation_hash
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Opaque evidence that one fresh constructor answers one exact pending slot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstructorCompletionEvidence {
    presentation_signature_digest: String,
    formation_derivation_hash: String,
    legacy_presentation_hash: String,
    semantic_subject_digest: String,
    declaration_digest: String,
    target_head: FreshHeadId,
    fresh_constructor: FreshHeadId,
    answered_slot: ConstructorSlotRef,
    remaining_slots: Vec<ConstructorSlotRef>,
    resulting_context_digest: String,
    presentation_derivation_hash: String,
    derivation_hash: String,
}

impl ConstructorCompletionEvidence {
    pub fn presentation_signature_digest(&self) -> &str {
        &self.presentation_signature_digest
    }

    pub fn formation_derivation_hash(&self) -> &str {
        &self.formation_derivation_hash
    }

    pub fn legacy_presentation_hash(&self) -> &str {
        &self.legacy_presentation_hash
    }

    pub fn semantic_subject_digest(&self) -> &str {
        &self.semantic_subject_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn target_head(&self) -> &FreshHeadId {
        &self.target_head
    }

    pub fn fresh_constructor(&self) -> &FreshHeadId {
        &self.fresh_constructor
    }

    pub fn answered_slot(&self) -> &ConstructorSlotRef {
        &self.answered_slot
    }

    pub fn remaining_slots(&self) -> &[ConstructorSlotRef] {
        &self.remaining_slots
    }

    pub fn is_inductive_signature_closed(&self) -> bool {
        self.remaining_slots.is_empty()
    }

    pub fn resulting_context_digest(&self) -> &str {
        &self.resulting_context_digest
    }

    pub fn presentation_derivation_hash(&self) -> &str {
        &self.presentation_derivation_hash
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Issue formation evidence by checking both the explicit classifier and the
/// unchanged legacy surface against the actual predecessor signature.
pub fn issue_inductive_formation(
    signature: &SealedSignature,
    overlay: &InductiveFormationOverlay,
) -> Result<InductiveFormationEvidence, FreshDeclarationError> {
    if overlay.presentation().clauses.is_empty() {
        return Err(FreshDeclarationError::EmptyPresentation);
    }
    let declaration = overlay.declaration();
    if declaration.site().predecessor_digest() != signature.digest() {
        return Err(FreshDeclarationError::PredecessorContextMismatch);
    }
    if declaration.constructor_shapes().len() > usize::from(u16::MAX) + 1 {
        return Err(FreshDeclarationError::TooManyConstructorSlots);
    }

    let DeclarationType::Existing { expr: classifier } = declaration.classifier() else {
        return Err(FreshDeclarationError::UnsupportedInductiveClassifier);
    };
    let classifier_elaboration = elaborate_single_clause(
        classifier,
        0,
        &[],
        u32::try_from(signature.len()).expect("signature length fits u32"),
    )
    .map_err(FreshDeclarationError::ClassifierInvalid)?;
    if classifier_elaboration.kernel_ty != KernelTy::Type {
        return Err(FreshDeclarationError::ClassifierNotType);
    }
    if classifier_elaboration.coarse_assumptions != 0 {
        return Err(FreshDeclarationError::CoarseClassifier);
    }

    let presentation_elaboration = elaborate_telescope(
        signature,
        overlay.presentation(),
        u32::try_from(signature.len()).expect("signature length fits u32"),
    )
    .map_err(FreshDeclarationError::PresentationInvalid)?;
    let classifier_derivation_hash = evidence_digest(
        "inductive-classifier",
        &(signature.digest(), classifier, &classifier_elaboration),
    );

    let signature_digest = signature.digest().to_owned();
    let predecessor_entries = signature
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.candidate_hash.clone()))
        .collect::<Vec<_>>();
    let legacy_presentation_hash = candidate_hash(overlay.presentation());
    let semantic_subject_digest = overlay.semantic_subject_digest();
    let declaration_digest = declaration.declaration_digest();
    let fresh_head = declaration.fresh_head();
    let pending_slots = declaration.slots();
    let pending_slot_shapes = declaration.constructor_shapes().to_vec();
    let resulting_context_digest = declaration.resulting_context_digest();
    let classifier_kernel_ty = classifier_elaboration.kernel_ty;
    let presentation_derivation_hash = presentation_elaboration.derivation_hash;
    let derivation_hash = evidence_digest(
        "inductive-formation",
        &(
            &signature_digest,
            &predecessor_entries,
            &legacy_presentation_hash,
            &semantic_subject_digest,
            &declaration_digest,
            &fresh_head,
            &pending_slots,
            &pending_slot_shapes,
            &resulting_context_digest,
            &classifier_kernel_ty,
            &classifier_derivation_hash,
            &presentation_derivation_hash,
        ),
    );

    Ok(InductiveFormationEvidence {
        signature_digest,
        predecessor_entries,
        legacy_presentation_hash,
        semantic_subject_digest,
        declaration_digest,
        fresh_head,
        pending_slots,
        pending_slot_shapes,
        resulting_context_digest,
        classifier_kernel_ty,
        classifier_derivation_hash,
        presentation_derivation_hash,
        derivation_hash,
    })
}

/// Replay formation issuance and compare every evidence field.
pub fn replay_inductive_formation(
    signature: &SealedSignature,
    overlay: &InductiveFormationOverlay,
    evidence: &InductiveFormationEvidence,
) -> Result<(), FreshDeclarationReplayError> {
    let reissued = issue_inductive_formation(signature, overlay)
        .map_err(FreshDeclarationReplayError::Rejected)?;
    if &reissued == evidence {
        Ok(())
    } else {
        Err(FreshDeclarationReplayError::EvidenceMismatch)
    }
}

/// Issue evidence that a fresh nullary constructor fills exactly one slot of
/// a previously issued formation.  `presentation_signature` checks the
/// legacy surface at the prefix where it is proposed; the semantic context is
/// independently chained through the formation token.
pub fn issue_constructor_completion(
    presentation_signature: &SealedSignature,
    formation: &InductiveFormationEvidence,
    overlay: &ConstructorCompletionOverlay,
) -> Result<ConstructorCompletionEvidence, FreshDeclarationError> {
    if overlay.presentation().clauses.is_empty() {
        return Err(FreshDeclarationError::EmptyPresentation);
    }
    let declaration = overlay.declaration();
    if declaration.site().predecessor_digest() != formation.resulting_context_digest() {
        return Err(FreshDeclarationError::CompletionContextMismatch);
    }
    let signature_entries = presentation_signature.entries();
    let extends_checked_formation = signature_entries.len()
        == formation.predecessor_entries.len() + 1
        && signature_entries
            .iter()
            .zip(&formation.predecessor_entries)
            .all(|(entry, (step, hash))| {
                entry.step == *step && entry.candidate_hash == hash.as_str()
            })
        && signature_entries.last().is_some_and(|entry| {
            entry.candidate_hash == formation.legacy_presentation_hash.as_str()
        });
    if !extends_checked_formation {
        return Err(FreshDeclarationError::PresentationSignatureNotFormationExtension);
    }
    let Some(slot_index) = formation
        .pending_slots()
        .iter()
        .position(|slot| slot == declaration.answers())
    else {
        return Err(FreshDeclarationError::ConstructorSlotNotPending);
    };

    let expected_classifier = DeclarationType::fresh_head(formation.fresh_head().clone());
    if declaration.answers().owner() != formation.fresh_head()
        || declaration.classifier() != &expected_classifier
    {
        return Err(FreshDeclarationError::ConstructorClassifierMismatch);
    }
    // The v1 declaration alphabet only has a nullary slot.  Keep the check
    // explicit so extending that alphabet fails closed here.
    if formation.pending_slot_shapes.get(slot_index) != Some(&ConstructorSlotShape::Nullary) {
        return Err(FreshDeclarationError::ConstructorClassifierMismatch);
    }

    let presentation_elaboration = elaborate_telescope(
        presentation_signature,
        overlay.presentation(),
        u32::try_from(presentation_signature.len()).expect("signature length fits u32"),
    )
    .map_err(FreshDeclarationError::PresentationInvalid)?;

    let presentation_signature_digest = presentation_signature.digest().to_owned();
    let formation_derivation_hash = formation.derivation_hash().to_owned();
    let legacy_presentation_hash = candidate_hash(overlay.presentation());
    let semantic_subject_digest = overlay.semantic_subject_digest();
    let declaration_digest = declaration.declaration_digest();
    let target_head = formation.fresh_head().clone();
    let fresh_constructor = declaration.fresh_head();
    let answered_slot = declaration.answers().clone();
    let remaining_slots = formation
        .pending_slots()
        .iter()
        .filter(|slot| *slot != &answered_slot)
        .cloned()
        .collect::<Vec<_>>();
    let resulting_context_digest = declaration.resulting_context_digest();
    let presentation_derivation_hash = presentation_elaboration.derivation_hash;
    let derivation_hash = evidence_digest(
        "constructor-completion",
        &(
            &presentation_signature_digest,
            &formation_derivation_hash,
            &legacy_presentation_hash,
            &semantic_subject_digest,
            &declaration_digest,
            &target_head,
            &fresh_constructor,
            &answered_slot,
            &remaining_slots,
            &resulting_context_digest,
            &presentation_derivation_hash,
        ),
    );

    Ok(ConstructorCompletionEvidence {
        presentation_signature_digest,
        formation_derivation_hash,
        legacy_presentation_hash,
        semantic_subject_digest,
        declaration_digest,
        target_head,
        fresh_constructor,
        answered_slot,
        remaining_slots,
        resulting_context_digest,
        presentation_derivation_hash,
        derivation_hash,
    })
}

/// Replay constructor issuance and compare every evidence field.
pub fn replay_constructor_completion(
    presentation_signature: &SealedSignature,
    formation: &InductiveFormationEvidence,
    overlay: &ConstructorCompletionOverlay,
    evidence: &ConstructorCompletionEvidence,
) -> Result<(), FreshDeclarationReplayError> {
    let reissued = issue_constructor_completion(presentation_signature, formation, overlay)
        .map_err(FreshDeclarationReplayError::Rejected)?;
    if &reissued == evidence {
        Ok(())
    } else {
        Err(FreshDeclarationReplayError::EvidenceMismatch)
    }
}

fn evidence_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(FRESH_DECLARATION_RULES, domain, payload))
        .expect("evidence hashing only serializes infallible in-memory data");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::declaration::{
        ConstructorDeclaration, DeclarationType, FreshExportSite, InductiveHeadDeclaration,
    };
    use pen_core::expr::Expr;
    use pen_core::telescope::Telescope;

    fn signature_prefix(last: u32) -> SealedSignature {
        SealedSignature::from_telescopes(
            (1..=last)
                .map(|step| (step, Telescope::reference(step)))
                .collect(),
        )
    }

    fn formation_overlay(signature: &SealedSignature) -> InductiveFormationOverlay {
        let declaration = InductiveHeadDeclaration::one_nullary(
            FreshExportSite::new(signature.digest(), 0),
            DeclarationType::existing(Expr::Univ),
        );
        InductiveFormationOverlay::new(Telescope::reference(2), declaration)
    }

    #[test]
    fn unit_shape_issues_one_fresh_head_and_one_pending_orbit() {
        let signature = signature_prefix(1);
        let overlay = formation_overlay(&signature);
        let evidence = issue_inductive_formation(&signature, &overlay).expect("formation issues");

        assert_eq!(evidence.signature_digest(), signature.digest());
        assert_eq!(evidence.pending_slots().len(), 1);
        assert_eq!(evidence.pending_slots()[0].owner(), evidence.fresh_head());
        assert_eq!(
            evidence.pending_slot_shape(&evidence.pending_slots()[0]),
            Some(ConstructorSlotShape::Nullary)
        );
        assert_eq!(evidence.classifier_kernel_ty(), &KernelTy::Type);
        assert!(evidence.derivation_hash().starts_with("blake3:"));
        replay_inductive_formation(&signature, &overlay, &evidence).expect("formation replays");
    }

    #[test]
    fn legacy_app_surface_cannot_supply_an_unbound_fresh_declaration() {
        let signature = signature_prefix(1);
        let declaration = InductiveHeadDeclaration::one_nullary(
            FreshExportSite::new("blake3:not-the-checked-prefix", 0),
            DeclarationType::existing(Expr::Univ),
        );
        let overlay = InductiveFormationOverlay::new(Telescope::reference(2), declaration);

        assert_eq!(
            issue_inductive_formation(&signature, &overlay),
            Err(FreshDeclarationError::PredecessorContextMismatch)
        );
    }

    #[test]
    fn nullary_completion_answers_the_exact_slot_and_closes_the_signature() {
        let before_formation = signature_prefix(1);
        let formation_overlay = formation_overlay(&before_formation);
        let formation = issue_inductive_formation(&before_formation, &formation_overlay)
            .expect("formation issues");
        let slot = formation_overlay
            .declaration()
            .slot(0)
            .expect("one pending constructor");
        let completion_declaration = ConstructorDeclaration::fill_nullary(
            FreshExportSite::new(formation.resulting_context_digest(), 0),
            slot.clone(),
        );
        let completion_overlay =
            ConstructorCompletionOverlay::new(Telescope::reference(3), completion_declaration);
        let presentation_signature = signature_prefix(2);
        let completion =
            issue_constructor_completion(&presentation_signature, &formation, &completion_overlay)
                .expect("constructor issues");

        assert_eq!(completion.target_head(), formation.fresh_head());
        assert_eq!(completion.answered_slot(), &slot);
        assert!(completion.remaining_slots().is_empty());
        assert!(completion.is_inductive_signature_closed());
        assert_ne!(completion.fresh_constructor(), completion.target_head());
        replay_constructor_completion(
            &presentation_signature,
            &formation,
            &completion_overlay,
            &completion,
        )
        .expect("completion replays");
    }

    #[test]
    fn completion_from_another_anonymous_head_cannot_answer_the_slot() {
        let signature = signature_prefix(1);
        let first_overlay = formation_overlay(&signature);
        let first = issue_inductive_formation(&signature, &first_overlay).expect("first issues");
        let other_declaration = InductiveHeadDeclaration::one_nullary(
            FreshExportSite::new(signature.digest(), 1),
            DeclarationType::existing(Expr::Univ),
        );
        let other_slot = other_declaration.slot(0).expect("other slot");
        let completion = ConstructorDeclaration::fill_nullary(
            FreshExportSite::new(first.resulting_context_digest(), 0),
            other_slot,
        );
        let overlay = ConstructorCompletionOverlay::new(Telescope::reference(3), completion);

        assert_eq!(
            issue_constructor_completion(&signature_prefix(2), &first, &overlay),
            Err(FreshDeclarationError::ConstructorSlotNotPending)
        );
    }

    #[test]
    fn completion_surface_must_be_checked_against_the_exact_legacy_extension() {
        let signature = signature_prefix(1);
        let formation_overlay = formation_overlay(&signature);
        let formation =
            issue_inductive_formation(&signature, &formation_overlay).expect("formation issues");
        let completion = ConstructorDeclaration::fill_nullary(
            FreshExportSite::new(formation.resulting_context_digest(), 0),
            formation_overlay.declaration().slot(0).expect("one slot"),
        );
        let overlay = ConstructorCompletionOverlay::new(Telescope::reference(3), completion);

        assert_eq!(
            issue_constructor_completion(&signature_prefix(3), &formation, &overlay),
            Err(FreshDeclarationError::PresentationSignatureNotFormationExtension)
        );
    }

    #[test]
    fn replay_detects_surface_drift_without_changing_fresh_identity() {
        let signature = signature_prefix(1);
        let overlay = formation_overlay(&signature);
        let evidence = issue_inductive_formation(&signature, &overlay).expect("formation issues");
        let drifted =
            InductiveFormationOverlay::new(Telescope::reference(1), overlay.declaration().clone());

        assert_eq!(
            replay_inductive_formation(&signature, &drifted, &evidence),
            Err(FreshDeclarationReplayError::EvidenceMismatch)
        );
        assert_eq!(
            drifted.declaration().fresh_head(),
            overlay.declaration().fresh_head()
        );
    }
}
