//! Verified declaration-order translation from kernel globals to wire slots.
//!
//! This is the only semantic-side entry point that creates the slot-indexed
//! declaration table. Callers cannot supply an ordering: every ordinal and
//! `GlobalId` lookup comes from `VerifiedGlobalSlotTableV1`.

use crate::production_refinement::VerifiedGlobalSlotTableV1;
use pen_kernel::{Digest, GlobalId, Term, VerifiedSignature};
use pen_production_wire::{GlobalSlotEntryWireV1, GlobalSlotTableWireV1, WireIdV1, WireTermV1};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductionWireSlotFailureV1 {
    SignatureBindingMismatch,
    SlotOverflow,
    SlotCoverageMismatch,
    InvalidDigestEncoding,
    UnsupportedTerm,
    MissingVerifiedGlobal,
}

impl std::fmt::Display for ProductionWireSlotFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SignatureBindingMismatch => {
                "the verified global-slot table belongs to another signature"
            }
            Self::SlotOverflow => "a verified global slot does not fit the V1 u32 wire",
            Self::SlotCoverageMismatch => {
                "verified slot coverage differs from the signature declaration order"
            }
            Self::InvalidDigestEncoding => "a verified digest is not canonical lowercase BLAKE3",
            Self::UnsupportedTerm => "the verified declaration uses syntax outside lambda/unit V3",
            Self::MissingVerifiedGlobal => {
                "a declaration term references a global absent from the verified slot table"
            }
        })
    }
}

impl std::error::Error for ProductionWireSlotFailureV1 {}

pub fn derive_global_slot_table_wire_v1(
    signature: &VerifiedSignature,
    slots: &VerifiedGlobalSlotTableV1,
) -> Result<GlobalSlotTableWireV1, ProductionWireSlotFailureV1> {
    if slots.signature_digest() != signature.digest()
        || slots.entries().len() != signature.declarations().len()
    {
        return Err(ProductionWireSlotFailureV1::SignatureBindingMismatch);
    }
    let mut entries = Vec::with_capacity(slots.entries().len());
    for (ordinal, (verified, declaration)) in slots
        .entries()
        .iter()
        .zip(signature.declarations())
        .enumerate()
    {
        if verified.declaration() != declaration
            || verified.slot().ordinal()
                != u64::try_from(ordinal).map_err(|_| ProductionWireSlotFailureV1::SlotOverflow)?
        {
            return Err(ProductionWireSlotFailureV1::SlotCoverageMismatch);
        }
        entries.push(GlobalSlotEntryWireV1 {
            slot: u32::try_from(verified.slot().ordinal())
                .map_err(|_| ProductionWireSlotFailureV1::SlotOverflow)?,
            global_id_bytes: digest_wire_id(&declaration.id.0)?,
            declaration_type: term_to_wire_v1(&declaration.ty, slots)?,
            declaration_body: declaration
                .body
                .as_ref()
                .map(|term| term_to_wire_v1(term, slots))
                .transpose()?,
        });
    }
    Ok(GlobalSlotTableWireV1 { entries })
}

pub fn term_to_wire_v1(
    term: &Term,
    slots: &VerifiedGlobalSlotTableV1,
) -> Result<WireTermV1, ProductionWireSlotFailureV1> {
    Ok(match term {
        Term::Sort { level } => WireTermV1::Sort { level: *level },
        Term::Var { index } => WireTermV1::Variable { index: *index },
        Term::Global { id } => WireTermV1::GlobalSlot {
            slot: global_slot_u32(id, slots)?,
        },
        Term::Pi { parameter, body } => WireTermV1::Pi {
            parameter: Box::new(term_to_wire_v1(parameter, slots)?),
            body: Box::new(term_to_wire_v1(body, slots)?),
        },
        Term::Lambda {
            parameter_type,
            body,
        } => WireTermV1::Lambda {
            parameter: Box::new(term_to_wire_v1(parameter_type, slots)?),
            body: Box::new(term_to_wire_v1(body, slots)?),
        },
        Term::Apply { function, argument } => WireTermV1::Apply {
            function: Box::new(term_to_wire_v1(function, slots)?),
            argument: Box::new(term_to_wire_v1(argument, slots)?),
        },
        Term::UnitType => WireTermV1::UnitType,
        Term::Unit => WireTermV1::Unit,
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            return Err(ProductionWireSlotFailureV1::UnsupportedTerm);
        }
    })
}

fn global_slot_u32(
    id: &GlobalId,
    slots: &VerifiedGlobalSlotTableV1,
) -> Result<u32, ProductionWireSlotFailureV1> {
    let slot = slots
        .slot_for_global(id)
        .ok_or(ProductionWireSlotFailureV1::MissingVerifiedGlobal)?;
    u32::try_from(slot.ordinal()).map_err(|_| ProductionWireSlotFailureV1::SlotOverflow)
}

pub(crate) fn digest_wire_id(digest: &Digest) -> Result<WireIdV1, ProductionWireSlotFailureV1> {
    let hex = digest
        .as_str()
        .strip_prefix(Digest::PREFIX)
        .ok_or(ProductionWireSlotFailureV1::InvalidDigestEncoding)?;
    if hex.len() != 64 {
        return Err(ProductionWireSlotFailureV1::InvalidDigestEncoding);
    }
    let mut bytes = [0; 32];
    for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
        bytes[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
    }
    Ok(WireIdV1(bytes))
}

fn hex_nibble(byte: u8) -> Result<u8, ProductionWireSlotFailureV1> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(ProductionWireSlotFailureV1::InvalidDigestEncoding),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::AuditDecision;
    use crate::production_refinement::verify_global_slot_table_v1;
    use pen_kernel::{Declaration, Kernel, KernelLimits, UncheckedSignature};

    fn global(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    #[test]
    fn declaration_order_and_global_references_come_only_from_verified_slots() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let type_head = global(b"production-wire-slots/type");
        let value = global(b"production-wire-slots/value");
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: type_head.clone(),
                        ty: Term::Sort { level: 0 },
                        body: None,
                    },
                    Declaration {
                        id: value,
                        ty: Term::Global {
                            id: type_head.clone(),
                        },
                        body: None,
                    },
                ],
            })
            .expect("signature");
        let AuditDecision::Proven(slots) = verify_global_slot_table_v1(&kernel, &signature) else {
            panic!("slot table");
        };
        let wire = derive_global_slot_table_wire_v1(&signature, &slots).expect("wire");
        assert_eq!(wire.entries.len(), 2);
        assert_eq!(wire.entries[0].slot, 0);
        assert_eq!(wire.entries[1].slot, 1);
        assert_eq!(
            wire.entries[1].declaration_type,
            WireTermV1::GlobalSlot { slot: 0 }
        );
        assert_eq!(
            wire.entries[0].global_id_bytes,
            digest_wire_id(&type_head.0).expect("digest bytes")
        );
    }

    #[test]
    fn non_lambda_unit_terms_are_not_silently_encoded() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let AuditDecision::Proven(slots) = verify_global_slot_table_v1(&kernel, &signature) else {
            panic!("slot table");
        };
        assert_eq!(
            term_to_wire_v1(
                &Term::Sigma {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::UnitType),
                },
                &slots,
            ),
            Err(ProductionWireSlotFailureV1::UnsupportedTerm)
        );
    }
}
