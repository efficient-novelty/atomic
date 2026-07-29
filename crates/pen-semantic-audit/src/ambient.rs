//! Verifier-derived ambient primitive public-export census.
//!
//! Ambient kernel formability is not public availability. This module scans
//! the complete normalized declaration bodies retained by a
//! [`VerifiedPublicAuditInventoryV1`] and mints one export-class capability
//! for each primitive exported by at least one successor-new declaration.
//! A primitive occurring only in a type or proper subterm is not an export.
//!
//! There is deliberately no unchecked wire and no caller `first_export`,
//! `is_alias`, or representative flag. First export versus prior re-export,
//! transparent aliases, and the complete Q2-equivalent member set are all
//! reconstructed from the verified inventory.

use crate::inventory::{VerifiedPublicAuditInventoryV1, VerifiedPublicDeclarationV1};
use crate::manifest::{AuditDecision, AuditUnknownReason};
use crate::model::{AmbientPrimitiveV1, EventIdV1, PublicAvailabilityV1};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, GlobalId, Term};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const AMBIENT_PRIMITIVE_EXPORT_CENSUS_SCHEMA_VERSION: u16 = 1;

/// Which side of the verified exact extension contains an export member.
///
/// This value is descriptive data inside a verified capability; constructing
/// the enum directly does not mint census authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AmbientPrimitiveExportBoundaryV1 {
    PredecessorPublic,
    SuccessorNew,
}

impl CanonicalEncode for AmbientPrimitiveExportBoundaryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::PredecessorPublic => 0,
            Self::SuccessorNew => 1,
        });
    }
}

/// Verifier-derived presentation of one whole-body export.
///
/// An alias path contains the immediate target through the first non-alias
/// declaration. Every hop is checked against verifier-derived inventory
/// availability, and every declaration on the path has the same complete
/// normalized ambient body.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AmbientPrimitiveExportMemberKindV1 {
    Direct,
    TransparentAlias {
        immediate_target: GlobalId,
        direct_root: GlobalId,
        resolution_path: Vec<GlobalId>,
    },
}

impl AmbientPrimitiveExportMemberKindV1 {
    pub fn is_direct(&self) -> bool {
        matches!(self, Self::Direct)
    }

    pub fn immediate_alias_target(&self) -> Option<&GlobalId> {
        match self {
            Self::Direct => None,
            Self::TransparentAlias {
                immediate_target, ..
            } => Some(immediate_target),
        }
    }

    pub fn direct_root(&self) -> Option<&GlobalId> {
        match self {
            Self::Direct => None,
            Self::TransparentAlias { direct_root, .. } => Some(direct_root),
        }
    }

    pub fn resolution_path(&self) -> &[GlobalId] {
        match self {
            Self::Direct => &[],
            Self::TransparentAlias {
                resolution_path, ..
            } => resolution_path,
        }
    }
}

impl CanonicalEncode for AmbientPrimitiveExportMemberKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Direct => encoder.tag(0),
            Self::TransparentAlias {
                immediate_target,
                direct_root,
                resolution_path,
            } => {
                encoder.tag(1);
                immediate_target.encode_canonical(encoder);
                direct_root.encode_canonical(encoder);
                encoder.sequence(resolution_path);
            }
        }
    }
}

/// One verifier-derived member of an ambient primitive export class.
///
/// Fields are private, and this type intentionally has no `Deserialize`
/// implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedAmbientPrimitiveExportMemberV1 {
    declaration: GlobalId,
    origin: EventIdV1,
    boundary: AmbientPrimitiveExportBoundaryV1,
    source_identity: Digest,
    presentation: AmbientPrimitiveExportMemberKindV1,
    digest: Digest,
}

impl VerifiedAmbientPrimitiveExportMemberV1 {
    pub fn declaration(&self) -> &GlobalId {
        &self.declaration
    }

    pub fn origin(&self) -> &EventIdV1 {
        &self.origin
    }

    pub fn boundary(&self) -> AmbientPrimitiveExportBoundaryV1 {
        self.boundary
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn presentation(&self) -> &AmbientPrimitiveExportMemberKindV1 {
        &self.presentation
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedAmbientPrimitiveExportMemberV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.declaration.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.boundary.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.presentation.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Whether the successor-new members constitute the first public export class
/// or re-export a class already present at the predecessor boundary.
///
/// The predecessor digest is the order-independent Q2 identity of the
/// predecessor member class, not a caller-selected representative.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum AmbientExportDispositionV1 {
    FirstPublicExportClass,
    PriorPublicReexport { predecessor_q2_class_digest: Digest },
}

impl AmbientExportDispositionV1 {
    pub fn is_first_public_export_class(&self) -> bool {
        matches!(self, Self::FirstPublicExportClass)
    }

    pub fn predecessor_q2_class_digest(&self) -> Option<&Digest> {
        match self {
            Self::FirstPublicExportClass => None,
            Self::PriorPublicReexport {
                predecessor_q2_class_digest,
            } => Some(predecessor_q2_class_digest),
        }
    }
}

impl CanonicalEncode for AmbientExportDispositionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::FirstPublicExportClass => encoder.tag(0),
            Self::PriorPublicReexport {
                predecessor_q2_class_digest,
            } => {
                encoder.tag(1);
                predecessor_q2_class_digest.encode_canonical(encoder);
            }
        }
    }
}

/// Opaque proof capability for one successor ambient export event.
///
/// Members are sorted by declaration identity. If several successor
/// declarations have the same complete normalized primitive body, all of them
/// occur in this single class; neither source order nor a digest selects a
/// representative. Fields are private, and this type intentionally has no
/// `Deserialize` implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedAmbientPrimitiveExportClassV1 {
    primitive: AmbientPrimitiveV1,
    predecessor_members: Vec<VerifiedAmbientPrimitiveExportMemberV1>,
    successor_new_members: Vec<VerifiedAmbientPrimitiveExportMemberV1>,
    predecessor_scan_digest: Digest,
    successor_scan_digest: Digest,
    q0_equivalence_digest: Digest,
    q2_duplicate_class_digest: Digest,
    disposition: AmbientExportDispositionV1,
    digest: Digest,
}

impl VerifiedAmbientPrimitiveExportClassV1 {
    pub fn primitive(&self) -> &AmbientPrimitiveV1 {
        &self.primitive
    }

    pub fn predecessor_members(&self) -> &[VerifiedAmbientPrimitiveExportMemberV1] {
        &self.predecessor_members
    }

    pub fn successor_new_members(&self) -> &[VerifiedAmbientPrimitiveExportMemberV1] {
        &self.successor_new_members
    }

    pub fn successor_new_member(
        &self,
        declaration: &GlobalId,
    ) -> Option<&VerifiedAmbientPrimitiveExportMemberV1> {
        self.successor_new_members
            .iter()
            .find(|member| member.declaration() == declaration)
    }

    /// Every successor member when this is the first public export class.
    ///
    /// This deliberately returns the complete Q2 class, including verified
    /// aliases, rather than choosing one paid representative.
    pub fn first_export_members(&self) -> &[VerifiedAmbientPrimitiveExportMemberV1] {
        if self.disposition.is_first_public_export_class() {
            &self.successor_new_members
        } else {
            &[]
        }
    }

    pub fn successor_new_direct_members(
        &self,
    ) -> impl Iterator<Item = &VerifiedAmbientPrimitiveExportMemberV1> {
        self.successor_new_members
            .iter()
            .filter(|member| member.presentation.is_direct())
    }

    pub fn successor_new_alias_members(
        &self,
    ) -> impl Iterator<Item = &VerifiedAmbientPrimitiveExportMemberV1> {
        self.successor_new_members
            .iter()
            .filter(|member| !member.presentation.is_direct())
    }

    pub fn predecessor_scan_digest(&self) -> &Digest {
        &self.predecessor_scan_digest
    }

    pub fn successor_scan_digest(&self) -> &Digest {
        &self.successor_scan_digest
    }

    pub fn q0_equivalence_digest(&self) -> &Digest {
        &self.q0_equivalence_digest
    }

    /// Order-independent identity of the complete predecessor/successor
    /// duplicate-presentation class.
    pub fn q2_duplicate_class_digest(&self) -> &Digest {
        &self.q2_duplicate_class_digest
    }

    pub fn disposition(&self) -> &AmbientExportDispositionV1 {
        &self.disposition
    }

    /// Inventory-bound capability identity. Unlike the Q2 class digest, this
    /// digest intentionally changes if the replayed inventory subject changes.
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedAmbientPrimitiveExportClassV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.primitive.encode_canonical(encoder);
        encoder.sequence(&self.predecessor_members);
        encoder.sequence(&self.successor_new_members);
        self.predecessor_scan_digest.encode_canonical(encoder);
        self.successor_scan_digest.encode_canonical(encoder);
        self.q0_equivalence_digest.encode_canonical(encoder);
        self.q2_duplicate_class_digest.encode_canonical(encoder);
        self.disposition.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Complete verifier-derived successor ambient-export census.
///
/// There is one class for each primitive with at least one successor-new
/// whole-body export. A primitive present only at the predecessor boundary
/// does not create a successor export event. Fields are private, and this type
/// intentionally has no `Deserialize` implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedAmbientPrimitiveExportCensusV1 {
    inventory_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    predecessor_scanned_declarations: usize,
    successor_scanned_declarations: usize,
    predecessor_scan_digest: Digest,
    successor_scan_digest: Digest,
    classes: Vec<VerifiedAmbientPrimitiveExportClassV1>,
    digest: Digest,
}

impl VerifiedAmbientPrimitiveExportCensusV1 {
    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn predecessor_scanned_declarations(&self) -> usize {
        self.predecessor_scanned_declarations
    }

    pub fn successor_scanned_declarations(&self) -> usize {
        self.successor_scanned_declarations
    }

    pub fn predecessor_scan_digest(&self) -> &Digest {
        &self.predecessor_scan_digest
    }

    pub fn successor_scan_digest(&self) -> &Digest {
        &self.successor_scan_digest
    }

    pub fn classes(&self) -> &[VerifiedAmbientPrimitiveExportClassV1] {
        &self.classes
    }

    pub fn class_for(
        &self,
        primitive: &AmbientPrimitiveV1,
    ) -> Option<&VerifiedAmbientPrimitiveExportClassV1> {
        self.classes
            .iter()
            .find(|candidate| candidate.primitive() == primitive)
    }

    pub fn class_for_successor_declaration(
        &self,
        declaration: &GlobalId,
    ) -> Option<&VerifiedAmbientPrimitiveExportClassV1> {
        self.classes.iter().find(|class| {
            class
                .successor_new_members()
                .iter()
                .any(|member| member.declaration() == declaration)
        })
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedAmbientPrimitiveExportCensusV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        encoder.u64(self.predecessor_scanned_declarations as u64);
        encoder.u64(self.successor_scanned_declarations as u64);
        self.predecessor_scan_digest.encode_canonical(encoder);
        self.successor_scan_digest.encode_canonical(encoder);
        encoder.sequence(&self.classes);
        self.digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PrimitiveKey {
    Sort(u16),
    UnitType,
    Unit,
}

impl PrimitiveKey {
    fn from_primitive(primitive: &AmbientPrimitiveV1) -> Self {
        match primitive {
            AmbientPrimitiveV1::Sort { level } => Self::Sort(*level),
            AmbientPrimitiveV1::UnitType => Self::UnitType,
            AmbientPrimitiveV1::Unit => Self::Unit,
        }
    }

    fn into_primitive(self) -> AmbientPrimitiveV1 {
        match self {
            Self::Sort(level) => AmbientPrimitiveV1::Sort { level },
            Self::UnitType => AmbientPrimitiveV1::UnitType,
            Self::Unit => AmbientPrimitiveV1::Unit,
        }
    }
}

#[derive(Default)]
struct ClassBuilder {
    predecessor_members: Vec<VerifiedAmbientPrimitiveExportMemberV1>,
    successor_members: Vec<VerifiedAmbientPrimitiveExportMemberV1>,
}

struct CompleteScanMaterial<'a> {
    schema_version: u16,
    inventory_digest: &'a Digest,
    boundary_digest: &'a Digest,
    boundary: AmbientPrimitiveExportBoundaryV1,
    declaration_ids: &'a [GlobalId],
}

impl CanonicalEncode for CompleteScanMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.inventory_digest.encode_canonical(encoder);
        self.boundary_digest.encode_canonical(encoder);
        self.boundary.encode_canonical(encoder);
        encoder.sequence(self.declaration_ids);
    }
}

struct MemberDigestMaterial<'a> {
    declaration: &'a GlobalId,
    origin: &'a EventIdV1,
    boundary: AmbientPrimitiveExportBoundaryV1,
    source_identity: &'a Digest,
    primitive: &'a AmbientPrimitiveV1,
    presentation: &'a AmbientPrimitiveExportMemberKindV1,
}

impl CanonicalEncode for MemberDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.declaration.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.boundary.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.primitive.encode_canonical(encoder);
        self.presentation.encode_canonical(encoder);
    }
}

struct PrimitiveScanMaterial<'a> {
    complete_scan_digest: &'a Digest,
    primitive: &'a AmbientPrimitiveV1,
    member_digests: &'a [Digest],
}

impl CanonicalEncode for PrimitiveScanMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.complete_scan_digest.encode_canonical(encoder);
        self.primitive.encode_canonical(encoder);
        encoder.sequence(self.member_digests);
    }
}

struct Q0ClassMaterial<'a> {
    primitive: &'a AmbientPrimitiveV1,
    predecessor_members: &'a [GlobalId],
    successor_members: &'a [GlobalId],
}

impl CanonicalEncode for Q0ClassMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.primitive.encode_canonical(encoder);
        encoder.sequence(self.predecessor_members);
        encoder.sequence(self.successor_members);
    }
}

struct Q2ClassMaterial<'a> {
    primitive: &'a AmbientPrimitiveV1,
    predecessor_members: &'a [VerifiedAmbientPrimitiveExportMemberV1],
    successor_members: &'a [VerifiedAmbientPrimitiveExportMemberV1],
}

impl CanonicalEncode for Q2ClassMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.primitive.encode_canonical(encoder);
        encoder.sequence(self.predecessor_members);
        encoder.sequence(self.successor_members);
    }
}

struct ExportClassDigestMaterial<'a> {
    inventory_digest: &'a Digest,
    primitive: &'a AmbientPrimitiveV1,
    predecessor_scan_digest: &'a Digest,
    successor_scan_digest: &'a Digest,
    q0_equivalence_digest: &'a Digest,
    q2_duplicate_class_digest: &'a Digest,
    disposition: &'a AmbientExportDispositionV1,
}

impl CanonicalEncode for ExportClassDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory_digest.encode_canonical(encoder);
        self.primitive.encode_canonical(encoder);
        self.predecessor_scan_digest.encode_canonical(encoder);
        self.successor_scan_digest.encode_canonical(encoder);
        self.q0_equivalence_digest.encode_canonical(encoder);
        self.q2_duplicate_class_digest.encode_canonical(encoder);
        self.disposition.encode_canonical(encoder);
    }
}

struct CensusDigestMaterial<'a> {
    inventory_digest: &'a Digest,
    predecessor_boundary_digest: &'a Digest,
    successor_boundary_digest: &'a Digest,
    predecessor_scanned_declarations: u64,
    successor_scanned_declarations: u64,
    predecessor_scan_digest: &'a Digest,
    successor_scan_digest: &'a Digest,
    class_digests: &'a [Digest],
}

impl CanonicalEncode for CensusDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(AMBIENT_PRIMITIVE_EXPORT_CENSUS_SCHEMA_VERSION);
        self.inventory_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        encoder.u64(self.predecessor_scanned_declarations);
        encoder.u64(self.successor_scanned_declarations);
        self.predecessor_scan_digest.encode_canonical(encoder);
        self.successor_scan_digest.encode_canonical(encoder);
        encoder.sequence(self.class_digests);
    }
}

/// Derive the complete ambient primitive export census from a verified public
/// inventory.
///
/// This is the only constructor for the verified census and class
/// capabilities. It accepts no serialized census, primitive code, first-export
/// flag, alias flag, Q2 representative, or caller scan digest.
pub fn verify_ambient_primitive_export_census_v1(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> AuditDecision<VerifiedAmbientPrimitiveExportCensusV1> {
    match verify_ambient_primitive_export_census_inner(inventory) {
        Ok(census) => AuditDecision::Proven(census),
        Err(reason) => AuditDecision::Unknown(reason),
    }
}

fn verify_ambient_primitive_export_census_inner(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> Result<VerifiedAmbientPrimitiveExportCensusV1, AuditUnknownReason> {
    if inventory.coverage().declaration_count() != inventory.declarations().len() {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }

    let mut predecessor_ids = Vec::new();
    let mut successor_ids = Vec::new();
    let mut builders = BTreeMap::<PrimitiveKey, ClassBuilder>::new();
    let mut seen_declarations = BTreeSet::new();

    for declaration in inventory.declarations() {
        if !seen_declarations.insert(declaration.declaration().clone())
            || declaration.normalized().id != *declaration.declaration()
            || declaration.source().id != *declaration.declaration()
        {
            return Err(AuditUnknownReason::ProvenanceCollision);
        }

        let boundary = declaration_boundary(inventory, declaration.declaration())?;
        match boundary {
            AmbientPrimitiveExportBoundaryV1::PredecessorPublic => {
                predecessor_ids.push(declaration.declaration().clone());
            }
            AmbientPrimitiveExportBoundaryV1::SuccessorNew => {
                successor_ids.push(declaration.declaration().clone());
            }
        }

        let Some(primitive) = declaration
            .normalized()
            .body
            .as_ref()
            .and_then(ambient_primitive_from_whole_term)
        else {
            continue;
        };
        let presentation = derive_member_presentation(inventory, declaration, &primitive)?;
        let digest = Digest::of_canonical(
            "pen-semantic-audit/ambient-export-member/v1",
            &MemberDigestMaterial {
                declaration: declaration.declaration(),
                origin: declaration.origin(),
                boundary,
                source_identity: declaration.source_identity(),
                primitive: &primitive,
                presentation: &presentation,
            },
        );
        let member = VerifiedAmbientPrimitiveExportMemberV1 {
            declaration: declaration.declaration().clone(),
            origin: declaration.origin().clone(),
            boundary,
            source_identity: declaration.source_identity().clone(),
            presentation,
            digest,
        };
        let builder = builders
            .entry(PrimitiveKey::from_primitive(&primitive))
            .or_default();
        match boundary {
            AmbientPrimitiveExportBoundaryV1::PredecessorPublic => {
                builder.predecessor_members.push(member);
            }
            AmbientPrimitiveExportBoundaryV1::SuccessorNew => {
                builder.successor_members.push(member);
            }
        }
    }

    predecessor_ids.sort();
    successor_ids.sort();
    if predecessor_ids.len() != inventory.predecessor_boundary().declarations().len()
        || successor_ids.len() != inventory.exact_extension().new_declarations().len()
        || predecessor_ids.len().checked_add(successor_ids.len())
            != Some(inventory.declarations().len())
    {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }

    let expected_successor_ids = inventory
        .exact_extension()
        .new_declarations()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if successor_ids.iter().cloned().collect::<BTreeSet<_>>() != expected_successor_ids {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }

    let predecessor_scan_digest = Digest::of_canonical(
        "pen-semantic-audit/ambient-complete-predecessor-scan/v1",
        &CompleteScanMaterial {
            schema_version: AMBIENT_PRIMITIVE_EXPORT_CENSUS_SCHEMA_VERSION,
            inventory_digest: inventory.digest(),
            boundary_digest: inventory.predecessor_boundary().digest(),
            boundary: AmbientPrimitiveExportBoundaryV1::PredecessorPublic,
            declaration_ids: &predecessor_ids,
        },
    );
    let successor_scan_digest = Digest::of_canonical(
        "pen-semantic-audit/ambient-complete-successor-scan/v1",
        &CompleteScanMaterial {
            schema_version: AMBIENT_PRIMITIVE_EXPORT_CENSUS_SCHEMA_VERSION,
            inventory_digest: inventory.digest(),
            boundary_digest: inventory.successor_boundary().digest(),
            boundary: AmbientPrimitiveExportBoundaryV1::SuccessorNew,
            declaration_ids: &successor_ids,
        },
    );

    let mut classes = Vec::new();
    for (key, mut builder) in builders {
        // A predecessor-only class is scan evidence, but it is not a
        // successor export event and therefore has no cost disposition here.
        if builder.successor_members.is_empty() {
            continue;
        }
        builder
            .predecessor_members
            .sort_by(|left, right| left.declaration.cmp(&right.declaration));
        builder
            .successor_members
            .sort_by(|left, right| left.declaration.cmp(&right.declaration));
        reject_duplicate_members(&builder.predecessor_members)?;
        reject_duplicate_members(&builder.successor_members)?;

        let primitive = key.into_primitive();
        let predecessor_member_digests = builder
            .predecessor_members
            .iter()
            .map(|member| member.digest().clone())
            .collect::<Vec<_>>();
        let successor_member_digests = builder
            .successor_members
            .iter()
            .map(|member| member.digest().clone())
            .collect::<Vec<_>>();
        let class_predecessor_scan_digest = Digest::of_canonical(
            "pen-semantic-audit/ambient-primitive-predecessor-scan/v1",
            &PrimitiveScanMaterial {
                complete_scan_digest: &predecessor_scan_digest,
                primitive: &primitive,
                member_digests: &predecessor_member_digests,
            },
        );
        let class_successor_scan_digest = Digest::of_canonical(
            "pen-semantic-audit/ambient-primitive-successor-scan/v1",
            &PrimitiveScanMaterial {
                complete_scan_digest: &successor_scan_digest,
                primitive: &primitive,
                member_digests: &successor_member_digests,
            },
        );

        let predecessor_member_ids = builder
            .predecessor_members
            .iter()
            .map(|member| member.declaration().clone())
            .collect::<Vec<_>>();
        let successor_member_ids = builder
            .successor_members
            .iter()
            .map(|member| member.declaration().clone())
            .collect::<Vec<_>>();
        let q0_equivalence_digest = Digest::of_canonical(
            "pen-semantic-audit/ambient-whole-body-q0-class/v1",
            &Q0ClassMaterial {
                primitive: &primitive,
                predecessor_members: &predecessor_member_ids,
                successor_members: &successor_member_ids,
            },
        );
        let q2_duplicate_class_digest = Digest::of_canonical(
            "pen-semantic-audit/ambient-export-q2-class/v1",
            &Q2ClassMaterial {
                primitive: &primitive,
                predecessor_members: &builder.predecessor_members,
                successor_members: &builder.successor_members,
            },
        );
        let disposition = if builder.predecessor_members.is_empty() {
            AmbientExportDispositionV1::FirstPublicExportClass
        } else {
            let predecessor_q2_class_digest = Digest::of_canonical(
                "pen-semantic-audit/ambient-predecessor-export-q2-class/v1",
                &Q2ClassMaterial {
                    primitive: &primitive,
                    predecessor_members: &builder.predecessor_members,
                    successor_members: &[],
                },
            );
            AmbientExportDispositionV1::PriorPublicReexport {
                predecessor_q2_class_digest,
            }
        };
        let digest = Digest::of_canonical(
            "pen-semantic-audit/verified-ambient-export-class/v1",
            &ExportClassDigestMaterial {
                inventory_digest: inventory.digest(),
                primitive: &primitive,
                predecessor_scan_digest: &class_predecessor_scan_digest,
                successor_scan_digest: &class_successor_scan_digest,
                q0_equivalence_digest: &q0_equivalence_digest,
                q2_duplicate_class_digest: &q2_duplicate_class_digest,
                disposition: &disposition,
            },
        );
        classes.push(VerifiedAmbientPrimitiveExportClassV1 {
            primitive,
            predecessor_members: builder.predecessor_members,
            successor_new_members: builder.successor_members,
            predecessor_scan_digest: class_predecessor_scan_digest,
            successor_scan_digest: class_successor_scan_digest,
            q0_equivalence_digest,
            q2_duplicate_class_digest,
            disposition,
            digest,
        });
    }

    let class_digests = classes
        .iter()
        .map(|class| class.digest().clone())
        .collect::<Vec<_>>();
    let predecessor_scanned_declarations = predecessor_ids.len();
    let successor_scanned_declarations = successor_ids.len();
    let predecessor_scanned_u64 = u64::try_from(predecessor_scanned_declarations)
        .map_err(|_| AuditUnknownReason::ResourceExhausted)?;
    let successor_scanned_u64 = u64::try_from(successor_scanned_declarations)
        .map_err(|_| AuditUnknownReason::ResourceExhausted)?;
    let digest = Digest::of_canonical(
        "pen-semantic-audit/verified-ambient-export-census/v1",
        &CensusDigestMaterial {
            inventory_digest: inventory.digest(),
            predecessor_boundary_digest: inventory.predecessor_boundary().digest(),
            successor_boundary_digest: inventory.successor_boundary().digest(),
            predecessor_scanned_declarations: predecessor_scanned_u64,
            successor_scanned_declarations: successor_scanned_u64,
            predecessor_scan_digest: &predecessor_scan_digest,
            successor_scan_digest: &successor_scan_digest,
            class_digests: &class_digests,
        },
    );

    Ok(VerifiedAmbientPrimitiveExportCensusV1 {
        inventory_digest: inventory.digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        predecessor_scanned_declarations,
        successor_scanned_declarations,
        predecessor_scan_digest,
        successor_scan_digest,
        classes,
        digest,
    })
}

fn declaration_boundary(
    inventory: &VerifiedPublicAuditInventoryV1,
    declaration: &GlobalId,
) -> Result<AmbientPrimitiveExportBoundaryV1, AuditUnknownReason> {
    let predecessor = inventory.contains_predecessor_declaration(declaration);
    let successor_new = inventory.is_successor_new_declaration(declaration);
    match (predecessor, successor_new) {
        (true, false) => Ok(AmbientPrimitiveExportBoundaryV1::PredecessorPublic),
        (false, true) => Ok(AmbientPrimitiveExportBoundaryV1::SuccessorNew),
        _ => Err(AuditUnknownReason::IncompleteSupport),
    }
}

fn ambient_primitive_from_whole_term(term: &Term) -> Option<AmbientPrimitiveV1> {
    match term {
        Term::Sort { level } => Some(AmbientPrimitiveV1::Sort { level: *level }),
        Term::UnitType => Some(AmbientPrimitiveV1::UnitType),
        Term::Unit => Some(AmbientPrimitiveV1::Unit),
        Term::Var { .. }
        | Term::Global { .. }
        | Term::Pi { .. }
        | Term::Sigma { .. }
        | Term::Lambda { .. }
        | Term::Apply { .. }
        | Term::Pair { .. }
        | Term::First { .. }
        | Term::Second { .. } => None,
    }
}

fn derive_member_presentation(
    inventory: &VerifiedPublicAuditInventoryV1,
    declaration: &VerifiedPublicDeclarationV1,
    primitive: &AmbientPrimitiveV1,
) -> Result<AmbientPrimitiveExportMemberKindV1, AuditUnknownReason> {
    let Some(Term::Global {
        id: immediate_target,
    }) = declaration.source().body.as_ref()
    else {
        return Ok(AmbientPrimitiveExportMemberKindV1::Direct);
    };

    let mut resolution_path = Vec::new();
    let mut seen = BTreeSet::new();
    seen.insert(declaration.declaration().clone());
    let mut dependent = declaration.declaration().clone();
    let mut target = immediate_target.clone();

    loop {
        if !seen.insert(target.clone()) || resolution_path.len() >= inventory.declarations().len() {
            return Err(AuditUnknownReason::ProvenanceCollision);
        }
        verify_exact_alias_availability(inventory, &dependent, &target)?;
        let target_declaration = inventory
            .public_declaration(&target)
            .ok_or(AuditUnknownReason::IncompleteSupport)?;
        let target_primitive = target_declaration
            .normalized()
            .body
            .as_ref()
            .and_then(ambient_primitive_from_whole_term)
            .ok_or(AuditUnknownReason::NormalizationFailure)?;
        if &target_primitive != primitive {
            return Err(AuditUnknownReason::NormalizationFailure);
        }
        resolution_path.push(target.clone());

        match target_declaration.source().body.as_ref() {
            Some(Term::Global { id: next }) => {
                dependent = target;
                target = next.clone();
            }
            _ => {
                return Ok(AmbientPrimitiveExportMemberKindV1::TransparentAlias {
                    immediate_target: immediate_target.clone(),
                    direct_root: target,
                    resolution_path,
                });
            }
        }
    }
}

fn verify_exact_alias_availability(
    inventory: &VerifiedPublicAuditInventoryV1,
    dependent: &GlobalId,
    target: &GlobalId,
) -> Result<(), AuditUnknownReason> {
    let exact = match inventory.declaration_availability(dependent, target) {
        Some(PublicAvailabilityV1::PredecessorPublicExport { target: available })
        | Some(PublicAvailabilityV1::DependencyPriorExport { target: available }) => {
            available == target
        }
        Some(
            PublicAvailabilityV1::DerivedFromPublicInterface { .. }
            | PublicAvailabilityV1::AmbientOnly
            | PublicAvailabilityV1::OutsideFragment
            | PublicAvailabilityV1::Unknown,
        )
        | None => false,
    };
    if exact {
        Ok(())
    } else {
        Err(AuditUnknownReason::IncompleteSupport)
    }
}

fn reject_duplicate_members(
    members: &[VerifiedAmbientPrimitiveExportMemberV1],
) -> Result<(), AuditUnknownReason> {
    if members
        .windows(2)
        .any(|pair| pair[0].declaration() == pair[1].declaration())
    {
        Err(AuditUnknownReason::ProvenanceCollision)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        ORIGIN_CUTOFF_Q3_SCHEMA_VERSION, PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
        PublicDependencyUseV1, PublicSubjectV1, UncheckedOriginCutoffQ3RegistryV1,
        UncheckedPublicAuditInventoryV1, UncheckedPublicAvailabilityClaimV1,
        UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEventCensusV1,
        UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::manifest::{VerifiedSemanticAuditManifestV1, verify_core_manifests_v1};
    use pen_kernel::{Declaration, Kernel, KernelLimits, UncheckedSignature};

    struct Fixture {
        manifest: VerifiedSemanticAuditManifestV1,
        kernel: Kernel,
        wire: UncheckedPublicAuditInventoryV1,
    }

    fn digest(label: &str) -> Digest {
        Digest::of_bytes(label.as_bytes())
    }

    fn global(label: &str) -> GlobalId {
        GlobalId(digest(&format!("ambient/global/{label}")))
    }

    fn event(label: &str) -> EventIdV1 {
        EventIdV1(digest(&format!("ambient/event/{label}")))
    }

    fn declaration(label: &str, ty: Term, body: Option<Term>) -> Declaration {
        Declaration {
            id: global(label),
            ty,
            body,
        }
    }

    fn source_declaration(
        source: &Declaration,
        normalized: &Declaration,
    ) -> UncheckedSourceNormalizedDeclarationV1 {
        UncheckedSourceNormalizedDeclarationV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-declaration/v1",
                source,
            ),
            source: source.clone(),
            claimed_normalized: normalized.clone(),
        }
    }

    fn fixture(predecessor: Vec<Declaration>, successor_new: Vec<Declaration>) -> Fixture {
        assert!(!successor_new.is_empty());
        let AuditDecision::Proven((manifest, _)) = verify_core_manifests_v1() else {
            panic!("proposed manifests verify");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let predecessor_event = event("predecessor");
        let successor_event = event("successor");
        let predecessor_group = global("group/predecessor");
        let successor_group = global("group/successor");

        let predecessor_boundary = UncheckedSignature {
            declarations: predecessor.clone(),
        };
        let mut all_source = predecessor;
        all_source.extend(successor_new);
        let successor_boundary = UncheckedSignature {
            declarations: all_source.clone(),
        };
        let normalized = kernel
            .verify_signature(&successor_boundary)
            .expect("fixture signature verifies")
            .normalized_wire();
        let normalized_by_id = normalized
            .declarations
            .iter()
            .map(|candidate| (candidate.id.clone(), candidate))
            .collect::<BTreeMap<_, _>>();

        let predecessor_ids = predecessor_boundary
            .declarations
            .iter()
            .map(|candidate| candidate.id.clone())
            .collect::<BTreeSet<_>>();
        let successor_ids = all_source[predecessor_boundary.declarations.len()..]
            .iter()
            .map(|candidate| candidate.id.clone())
            .collect::<BTreeSet<_>>();

        let mut declaration_groups = Vec::new();
        if !predecessor_ids.is_empty() {
            declaration_groups.push(UncheckedPublicGroupV1 {
                group: predecessor_group.clone(),
                origin: predecessor_event.clone(),
                declarations: predecessor_ids.iter().cloned().collect(),
            });
        }
        declaration_groups.push(UncheckedPublicGroupV1 {
            group: successor_group.clone(),
            origin: successor_event.clone(),
            declarations: successor_ids.iter().cloned().collect(),
        });

        let declarations = all_source
            .iter()
            .map(|source| {
                let is_predecessor = predecessor_ids.contains(&source.id);
                UncheckedPublicDeclarationV1 {
                    declaration: source.id.clone(),
                    origin: if is_predecessor {
                        predecessor_event.clone()
                    } else {
                        successor_event.clone()
                    },
                    group: if is_predecessor {
                        predecessor_group.clone()
                    } else {
                        successor_group.clone()
                    },
                    source_to_normal: source_declaration(
                        source,
                        normalized_by_id
                            .get(&source.id)
                            .expect("normalized fixture declaration"),
                    ),
                }
            })
            .collect::<Vec<_>>();

        let mut dependencies = BTreeSet::new();
        for source in &all_source {
            let mut globals = BTreeSet::new();
            collect_declaration_globals(source, &mut globals);
            collect_declaration_globals(
                normalized_by_id
                    .get(&source.id)
                    .expect("normalized fixture declaration"),
                &mut globals,
            );
            for prerequisite in globals {
                dependencies.insert(PublicDependencyUseV1 {
                    dependent: PublicSubjectV1::Declaration {
                        declaration: source.id.clone(),
                    },
                    prerequisite,
                });
            }
        }
        let dependency_dag = dependencies.iter().cloned().collect::<Vec<_>>();
        let public_availability = dependencies
            .iter()
            .map(|dependency| UncheckedPublicAvailabilityClaimV1 {
                dependency: dependency.clone(),
                claimed: if predecessor_ids.contains(&dependency.prerequisite) {
                    PublicAvailabilityV1::PredecessorPublicExport {
                        target: dependency.prerequisite.clone(),
                    }
                } else {
                    PublicAvailabilityV1::DependencyPriorExport {
                        target: dependency.prerequisite.clone(),
                    }
                },
            })
            .collect::<Vec<_>>();

        let predecessor_history = if predecessor_ids.is_empty() {
            Vec::new()
        } else {
            vec![UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group],
                    added_declarations: predecessor_boundary
                        .declarations
                        .iter()
                        .map(|candidate| candidate.id.clone())
                        .collect(),
                    added_equations: Vec::new(),
                    added_forced_projections: Vec::new(),
                    added_demand_contracts: Vec::new(),
                },
                successor_boundary: predecessor_boundary.clone(),
            }]
        };
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history,
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: successor_event.clone(),
                added_groups: vec![successor_group],
                added_declarations: successor_ids.iter().cloned().collect(),
                added_equations: Vec::new(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups,
            declarations,
            equations: Vec::new(),
            forced_projections: Vec::new(),
            predecessor_demand_contracts: Vec::new(),
            public_availability,
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: dependency_dag,
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: (!predecessor_ids.is_empty()).then_some(predecessor_event),
                entries: Vec::new(),
            },
        };
        Fixture {
            manifest,
            kernel,
            wire,
        }
    }

    fn verify_inventory(fixture: &Fixture) -> VerifiedPublicAuditInventoryV1 {
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&fixture.manifest, &fixture.kernel, &fixture.wire)
        else {
            panic!("fixture inventory verifies");
        };
        inventory
    }

    fn verify_census(
        inventory: &VerifiedPublicAuditInventoryV1,
    ) -> VerifiedAmbientPrimitiveExportCensusV1 {
        let AuditDecision::Proven(census) = verify_ambient_primitive_export_census_v1(inventory)
        else {
            panic!("ambient census verifies");
        };
        census
    }

    fn collect_declaration_globals(declaration: &Declaration, globals: &mut BTreeSet<GlobalId>) {
        collect_term_globals(&declaration.ty, globals);
        if let Some(body) = &declaration.body {
            collect_term_globals(body, globals);
        }
    }

    fn collect_term_globals(term: &Term, globals: &mut BTreeSet<GlobalId>) {
        match term {
            Term::Global { id } => {
                globals.insert(id.clone());
            }
            Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
                collect_term_globals(parameter, globals);
                collect_term_globals(body, globals);
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                collect_term_globals(parameter_type, globals);
                collect_term_globals(body, globals);
            }
            Term::Apply { function, argument } => {
                collect_term_globals(function, globals);
                collect_term_globals(argument, globals);
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                collect_term_globals(sigma_type, globals);
                collect_term_globals(first, globals);
                collect_term_globals(second, globals);
            }
            Term::First { pair } | Term::Second { pair } => {
                collect_term_globals(pair, globals);
            }
            Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => {}
        }
    }

    #[test]
    fn scan_requires_the_complete_normalized_body_not_types_or_subterms() {
        let type_only = declaration("type-only", Term::UnitType, None);
        let lambda_with_unit = declaration(
            "lambda-with-unit",
            Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            },
            Some(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Unit),
            }),
        );
        let reduces_as_a_whole = declaration(
            "whole-reduct",
            Term::UnitType,
            Some(Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Unit),
                }),
                argument: Box::new(Term::Unit),
            }),
        );
        let fixture = fixture(
            Vec::new(),
            vec![type_only, lambda_with_unit, reduces_as_a_whole.clone()],
        );
        let inventory = verify_inventory(&fixture);
        let census = verify_census(&inventory);

        assert_eq!(census.predecessor_scanned_declarations(), 0);
        assert_eq!(census.successor_scanned_declarations(), 3);
        assert!(census.class_for(&AmbientPrimitiveV1::UnitType).is_none());
        let unit = census
            .class_for(&AmbientPrimitiveV1::Unit)
            .expect("whole normalized reduct exports Unit");
        assert_eq!(unit.successor_new_members().len(), 1);
        assert_eq!(
            unit.successor_new_members()[0].declaration(),
            &reduces_as_a_whole.id
        );
        assert!(unit.successor_new_members()[0].presentation().is_direct());
    }

    #[test]
    fn predecessor_export_makes_a_successor_alias_a_prior_reexport() {
        let predecessor = declaration(
            "predecessor-unit-type",
            Term::Sort { level: 0 },
            Some(Term::UnitType),
        );
        let successor_alias = declaration(
            "successor-alias",
            Term::Sort { level: 0 },
            Some(Term::Global {
                id: predecessor.id.clone(),
            }),
        );
        let successor_alias_chain = declaration(
            "successor-alias-chain",
            Term::Sort { level: 0 },
            Some(Term::Global {
                id: successor_alias.id.clone(),
            }),
        );
        let fixture = fixture(
            vec![predecessor.clone()],
            vec![successor_alias.clone(), successor_alias_chain.clone()],
        );
        let inventory = verify_inventory(&fixture);
        let census = verify_census(&inventory);
        let class = census
            .class_for(&AmbientPrimitiveV1::UnitType)
            .expect("successor alias is a whole-body export");

        assert_eq!(class.predecessor_members().len(), 1);
        assert_eq!(class.successor_new_members().len(), 2);
        assert!(matches!(
            class.disposition(),
            AmbientExportDispositionV1::PriorPublicReexport { .. }
        ));
        assert!(class.first_export_members().is_empty());
        let alias = class
            .successor_new_members()
            .iter()
            .find(|member| member.declaration() == &successor_alias.id)
            .expect("direct predecessor alias");
        assert_eq!(alias.declaration(), &successor_alias.id);
        assert_eq!(
            alias.presentation().immediate_alias_target(),
            Some(&predecessor.id)
        );
        assert_eq!(alias.presentation().direct_root(), Some(&predecessor.id));
        assert_eq!(
            alias.presentation().resolution_path(),
            &[predecessor.id.clone()]
        );
        let alias_chain = class
            .successor_new_members()
            .iter()
            .find(|member| member.declaration() == &successor_alias_chain.id)
            .expect("transitive alias");
        assert_eq!(
            alias_chain.presentation().immediate_alias_target(),
            Some(&successor_alias.id)
        );
        assert_eq!(
            alias_chain.presentation().direct_root(),
            Some(&predecessor.id)
        );
        assert_eq!(
            alias_chain.presentation().resolution_path(),
            &[successor_alias.id, predecessor.id]
        );
    }

    #[test]
    fn simultaneous_equivalent_exports_form_one_first_class_without_a_representative() {
        let first = declaration(
            "simultaneous-a",
            Term::Sort { level: 0 },
            Some(Term::UnitType),
        );
        let second = declaration(
            "simultaneous-b",
            Term::Sort { level: 0 },
            Some(Term::UnitType),
        );
        let alias = declaration(
            "simultaneous-alias",
            Term::Sort { level: 0 },
            Some(Term::Global {
                id: first.id.clone(),
            }),
        );

        let left_inventory = verify_inventory(&fixture(
            Vec::new(),
            vec![first.clone(), second.clone(), alias.clone()],
        ));
        let right_inventory = verify_inventory(&fixture(
            Vec::new(),
            vec![second.clone(), first.clone(), alias.clone()],
        ));
        let left = verify_census(&left_inventory);
        let right = verify_census(&right_inventory);
        let left_class = left
            .class_for(&AmbientPrimitiveV1::UnitType)
            .expect("first export class");
        let right_class = right
            .class_for(&AmbientPrimitiveV1::UnitType)
            .expect("first export class");

        assert_eq!(left.classes().len(), 1);
        assert!(matches!(
            left_class.disposition(),
            AmbientExportDispositionV1::FirstPublicExportClass
        ));
        assert_eq!(left_class.first_export_members().len(), 3);
        assert_eq!(left_class.successor_new_direct_members().count(), 2);
        assert_eq!(left_class.successor_new_alias_members().count(), 1);
        assert_eq!(
            member_ids(left_class.successor_new_members()),
            member_ids(right_class.successor_new_members())
        );
        assert_eq!(
            left_class.q2_duplicate_class_digest(),
            right_class.q2_duplicate_class_digest()
        );
        assert_eq!(left_class.disposition(), right_class.disposition());
        // The capability digests remain bound to their distinct signature
        // replay subjects even though the semantic Q2 class is order-free.
        assert_ne!(left.digest(), right.digest());
    }

    #[test]
    fn reordered_unchecked_census_records_do_not_change_the_verified_census() {
        let first = declaration(
            "wire-order-a",
            Term::Sort { level: 0 },
            Some(Term::UnitType),
        );
        let second = declaration("wire-order-b", Term::UnitType, Some(Term::Unit));
        let fixture = fixture(Vec::new(), vec![first, second]);
        let original_inventory = verify_inventory(&fixture);
        let original = verify_census(&original_inventory);

        let mut reordered = fixture;
        reordered.wire.declarations.reverse();
        reordered.wire.declaration_groups.reverse();
        reordered.wire.successor_event.added_declarations.reverse();
        for group in &mut reordered.wire.declaration_groups {
            group.declarations.reverse();
        }
        reordered.wire.public_availability.reverse();
        reordered.wire.dependency_dag.edges.reverse();
        let reordered_inventory = verify_inventory(&reordered);
        let replayed = verify_census(&reordered_inventory);

        assert_eq!(original_inventory.digest(), reordered_inventory.digest());
        assert_eq!(original.digest(), replayed.digest());
        assert_eq!(original.classes(), replayed.classes());
    }

    #[test]
    fn later_direct_export_is_a_reexport_not_a_new_first_flag() {
        let predecessor = declaration("predecessor-unit", Term::UnitType, Some(Term::Unit));
        let successor = declaration("successor-unit", Term::UnitType, Some(Term::Unit));
        let inventory = verify_inventory(&fixture(vec![predecessor], vec![successor.clone()]));
        let census = verify_census(&inventory);
        let class = census
            .class_for(&AmbientPrimitiveV1::Unit)
            .expect("successor re-export");

        assert!(matches!(
            class.disposition(),
            AmbientExportDispositionV1::PriorPublicReexport { .. }
        ));
        assert_eq!(class.successor_new_direct_members().count(), 1);
        assert_eq!(
            class.successor_new_members()[0].declaration(),
            &successor.id
        );
        assert!(class.first_export_members().is_empty());
    }

    #[test]
    fn ambient_formability_without_a_public_body_mints_no_export_class() {
        let bodyless = declaration("bodyless-type", Term::Sort { level: 0 }, None);
        let inventory = verify_inventory(&fixture(Vec::new(), vec![bodyless]));
        let census = verify_census(&inventory);

        assert!(census.classes().is_empty());
        assert!(
            census
                .class_for(&AmbientPrimitiveV1::Sort { level: 0 })
                .is_none()
        );
        assert!(census.class_for(&AmbientPrimitiveV1::UnitType).is_none());
    }

    fn member_ids(members: &[VerifiedAmbientPrimitiveExportMemberV1]) -> Vec<GlobalId> {
        members
            .iter()
            .map(|member| member.declaration().clone())
            .collect()
    }
}
