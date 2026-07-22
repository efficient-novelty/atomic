//! Structural, name-free declarations layered over the legacy telescope AST.
//!
//! `Expr` describes anonymous clause bodies.  It cannot distinguish a fresh
//! exported head from a repeated presentation of an old family.  This module
//! adds that missing binder information without changing `Expr`, `Telescope`,
//! or their serialized forms.  A declaration is anchored at a semantic
//! predecessor-context digest and an ordinal within the extension.  Human
//! labels are deliberately absent from the identity calculation.

use crate::expr::Expr;
use crate::hash::blake3_hex;
use crate::telescope::Telescope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Version of the structural declaration encoding used in content hashes.
pub const FRESH_DECLARATION_SCHEMA: &str = "fresh-declaration-v1";

/// A binder position in a semantic context extension.
///
/// `predecessor_digest` identifies the context before the extension and
/// `export_ordinal` distinguishes simultaneous anonymous exports.  Neither a
/// stage number nor a user-facing name participates in equality.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct FreshExportSite {
    predecessor_digest: String,
    export_ordinal: u16,
}

impl FreshExportSite {
    pub fn new(predecessor_digest: impl Into<String>, export_ordinal: u16) -> Self {
        Self {
            predecessor_digest: predecessor_digest.into(),
            export_ordinal,
        }
    }

    pub fn predecessor_digest(&self) -> &str {
        &self.predecessor_digest
    }

    pub const fn export_ordinal(&self) -> u16 {
        self.export_ordinal
    }
}

/// Content-addressed identity of an anonymous fresh exported head.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct FreshHeadId {
    digest: String,
}

impl FreshHeadId {
    pub fn as_str(&self) -> &str {
        &self.digest
    }

    fn derive(domain: &str, payload: &impl Serialize) -> Self {
        Self {
            digest: tagged_digest(domain, payload),
        }
    }
}

/// A classifier which can mention either the legacy expression language or
/// an explicitly fresh head.  The latter is what the old
/// `App(Univ, Var(_))` encoding could not express.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DeclarationType {
    Existing {
        expr: Expr,
    },
    FreshHeadApplication {
        head: FreshHeadId,
        arguments: Vec<Expr>,
    },
}

impl DeclarationType {
    pub fn existing(expr: Expr) -> Self {
        Self::Existing { expr }
    }

    pub fn fresh_head(head: FreshHeadId) -> Self {
        Self::FreshHeadApplication {
            head,
            arguments: Vec::new(),
        }
    }
}

/// Constructor shapes supported by the bootstrap declaration kernel.
///
/// The first version intentionally exposes only the nullary shape needed for
/// the freely generated one-constructor type.  Adding dependent constructor
/// arguments requires a typed telescope, not an unchecked arity integer.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ConstructorSlotShape {
    Nullary,
}

/// One pending constructor demand owned by an inductive head.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct ConstructorSlotRef {
    owner: FreshHeadId,
    ordinal: u16,
}

impl ConstructorSlotRef {
    pub fn owner(&self) -> &FreshHeadId {
        &self.owner
    }

    pub const fn ordinal(&self) -> u16 {
        self.ordinal
    }
}

/// An anonymous fresh inductive head together with its exact constructor
/// demand boundary.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct InductiveHeadDeclaration {
    site: FreshExportSite,
    classifier: DeclarationType,
    constructor_slots: Vec<ConstructorSlotShape>,
}

impl InductiveHeadDeclaration {
    pub fn new(
        site: FreshExportSite,
        classifier: DeclarationType,
        constructor_slots: Vec<ConstructorSlotShape>,
    ) -> Self {
        Self {
            site,
            classifier,
            constructor_slots,
        }
    }

    /// The structural formation used for a freely generated type with one
    /// nullary constructor.  It is generic: no `Unit` label or stage index is
    /// stored in the declaration.
    pub fn one_nullary(site: FreshExportSite, classifier: DeclarationType) -> Self {
        Self::new(site, classifier, vec![ConstructorSlotShape::Nullary])
    }

    pub fn site(&self) -> &FreshExportSite {
        &self.site
    }

    pub fn classifier(&self) -> &DeclarationType {
        &self.classifier
    }

    pub fn constructor_shapes(&self) -> &[ConstructorSlotShape] {
        &self.constructor_slots
    }

    pub fn pending_slot_count(&self) -> usize {
        self.constructor_slots.len()
    }

    pub fn declaration_digest(&self) -> String {
        tagged_digest("inductive-declaration", self)
    }

    pub fn fresh_head(&self) -> FreshHeadId {
        FreshHeadId::derive("inductive-head", self)
    }

    pub fn slot(&self, ordinal: u16) -> Option<ConstructorSlotRef> {
        self.constructor_slots
            .get(usize::from(ordinal))
            .map(|_| ConstructorSlotRef {
                owner: self.fresh_head(),
                ordinal,
            })
    }

    pub fn slots(&self) -> Vec<ConstructorSlotRef> {
        self.constructor_slots
            .iter()
            .enumerate()
            .map(|(ordinal, _)| ConstructorSlotRef {
                owner: self.fresh_head(),
                ordinal: u16::try_from(ordinal)
                    .expect("constructor-slot count cannot exceed u16 through this API"),
            })
            .collect()
    }

    /// Digest of the semantic context after this fresh head has been added.
    pub fn resulting_context_digest(&self) -> String {
        tagged_digest(
            "context-after-inductive",
            &(
                self.site.predecessor_digest(),
                self.site.export_ordinal(),
                self.declaration_digest(),
                self.fresh_head(),
            ),
        )
    }

    /// Site for a subsequent anonymous export in the context produced by
    /// this declaration.
    pub fn following_site(&self, export_ordinal: u16) -> FreshExportSite {
        FreshExportSite::new(self.resulting_context_digest(), export_ordinal)
    }
}

/// A fresh nullary constructor which fills one particular pending slot.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct ConstructorDeclaration {
    site: FreshExportSite,
    answers: ConstructorSlotRef,
    classifier: DeclarationType,
}

impl ConstructorDeclaration {
    /// Build the only well-formed nullary classifier for `answers`: an
    /// inhabitant of the owning fresh inductive head.
    pub fn fill_nullary(site: FreshExportSite, answers: ConstructorSlotRef) -> Self {
        let classifier = DeclarationType::fresh_head(answers.owner.clone());
        Self {
            site,
            answers,
            classifier,
        }
    }

    pub fn site(&self) -> &FreshExportSite {
        &self.site
    }

    pub fn answers(&self) -> &ConstructorSlotRef {
        &self.answers
    }

    pub fn classifier(&self) -> &DeclarationType {
        &self.classifier
    }

    pub fn declaration_digest(&self) -> String {
        tagged_digest("constructor-declaration", self)
    }

    pub fn fresh_head(&self) -> FreshHeadId {
        FreshHeadId::derive("constructor-head", self)
    }

    pub fn resulting_context_digest(&self) -> String {
        tagged_digest(
            "context-after-constructor",
            &(
                self.site.predecessor_digest(),
                self.site.export_ordinal(),
                self.declaration_digest(),
                self.fresh_head(),
            ),
        )
    }
}

/// Backward-compatible semantic overlay: the legacy presentation stays byte
/// for byte unchanged while `declaration` records the fresh binder semantics.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct DeclarationOverlay<D> {
    presentation: Telescope,
    declaration: D,
}

impl<D> DeclarationOverlay<D> {
    pub fn new(presentation: Telescope, declaration: D) -> Self {
        Self {
            presentation,
            declaration,
        }
    }

    pub fn presentation(&self) -> &Telescope {
        &self.presentation
    }

    pub fn declaration(&self) -> &D {
        &self.declaration
    }

    pub fn into_parts(self) -> (Telescope, D) {
        (self.presentation, self.declaration)
    }
}

pub type InductiveFormationOverlay = DeclarationOverlay<InductiveHeadDeclaration>;
pub type ConstructorCompletionOverlay = DeclarationOverlay<ConstructorDeclaration>;

impl InductiveFormationOverlay {
    pub fn semantic_subject_digest(&self) -> String {
        tagged_digest("inductive-overlay", self)
    }
}

impl ConstructorCompletionOverlay {
    pub fn semantic_subject_digest(&self) -> String {
        tagged_digest("constructor-overlay", self)
    }
}

fn tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(FRESH_DECLARATION_SCHEMA, domain, payload))
        .expect("declaration hashing only serializes infallible in-memory data");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn formation(parent: &str, ordinal: u16) -> InductiveHeadDeclaration {
        InductiveHeadDeclaration::one_nullary(
            FreshExportSite::new(parent, ordinal),
            DeclarationType::existing(Expr::Univ),
        )
    }

    #[test]
    fn one_constructor_formation_has_exactly_one_owned_slot() {
        let declaration = formation("blake3:parent", 0);
        let slots = declaration.slots();
        assert_eq!(declaration.pending_slot_count(), 1);
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].owner(), &declaration.fresh_head());
        assert_eq!(slots[0].ordinal(), 0);
        assert_eq!(
            declaration.constructor_shapes(),
            &[ConstructorSlotShape::Nullary]
        );
    }

    #[test]
    fn fresh_identity_is_structural_and_name_free() {
        let first = formation("blake3:parent", 0);
        let alpha_same = formation("blake3:parent", 0);
        let distinct_site = formation("blake3:parent", 1);
        let distinct_context = formation("blake3:other-parent", 0);

        assert_eq!(first.fresh_head(), alpha_same.fresh_head());
        assert_ne!(first.fresh_head(), distinct_site.fresh_head());
        assert_ne!(first.fresh_head(), distinct_context.fresh_head());
    }

    #[test]
    fn constructor_completion_targets_the_fresh_head_not_a_legacy_app() {
        let formation = formation("blake3:parent", 0);
        let slot = formation.slot(0).expect("one slot");
        let completion = ConstructorDeclaration::fill_nullary(formation.following_site(0), slot);

        assert_eq!(completion.answers().owner(), &formation.fresh_head());
        assert_eq!(
            completion.classifier(),
            &DeclarationType::fresh_head(formation.fresh_head())
        );
        assert_ne!(completion.fresh_head(), formation.fresh_head());
    }

    #[test]
    fn overlay_does_not_mutate_legacy_telescope_serialization() {
        let telescope = Telescope::reference(2);
        let before = serde_json::to_string(&telescope).expect("serialize telescope");
        let overlay =
            InductiveFormationOverlay::new(telescope.clone(), formation("blake3:parent", 0));
        let after = serde_json::to_string(overlay.presentation()).expect("serialize telescope");

        assert_eq!(before, after);
        assert_ne!(
            overlay.semantic_subject_digest(),
            format!("blake3:{}", blake3_hex(before.as_bytes()))
        );
    }

    #[test]
    fn structural_declarations_round_trip() {
        let formation = formation("blake3:parent", 0);
        let encoded = serde_json::to_string(&formation).expect("serialize declaration");
        let decoded: InductiveHeadDeclaration =
            serde_json::from_str(&encoded).expect("deserialize declaration");
        assert_eq!(decoded, formation);
        assert_eq!(decoded.fresh_head(), formation.fresh_head());
    }
}
