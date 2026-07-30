use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TermRoleV1 {
    Public,
    CheckerProduced,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BundleValidationErrorV1 {
    HeaderMismatch,
    ManifestSurfaceMismatch,
    ResourceLimit {
        kind: &'static str,
    },
    TermDepthExceeded,
    SlotOrder {
        expected: u32,
        actual: u32,
    },
    DuplicateGlobalId,
    GlobalSlotOutOfRange {
        slot: u32,
        limit: u32,
    },
    ForwardGlobalReference {
        declaration_slot: u32,
        referenced_slot: u32,
    },
    VariableOutOfRange {
        index: u32,
        local_count: u32,
    },
    UniverseLevelOutOfRange {
        role: TermRoleV1,
        level: u16,
    },
    DeltaPolicyOrder,
    DeltaPolicyEntryMismatch,
    DuplicateConversionId,
    DuplicateSynthesisId,
    UnknownConversionReference,
    SupplementContextMismatch,
    FormationLevelOutOfRange(u16),
    VariableMetadataMismatch,
    SynthesisSubjectMismatch,
    DependentResultTypeMismatch,
    ReductionTraceEndpointMismatch,
    ReductionTraceChainMismatch,
    ReductionStepMismatch,
    NoRedexCensusMismatch,
    NormalFormHasRedex,
    Q0InventoryMismatch,
    DuplicateFreshEquationId,
    FreshSlotOutOfRange,
    FreshScrutineeOutOfRange,
    FreshPatternMismatch,
    FamilyInventoryMismatch,
    DuplicateFamilyId,
    MissingFamilyReference,
    MissingEquationReference,
}

impl fmt::Display for BundleValidationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WireErrorV1 {
    UnexpectedEnd {
        offset: usize,
        needed: usize,
        remaining: usize,
    },
    MagicMismatch,
    SchemaVersionMismatch(u16),
    SectionCountMismatch(u16),
    UnknownSectionTag {
        offset: usize,
        tag: u8,
    },
    DuplicateSectionTag {
        offset: usize,
        tag: u8,
    },
    NonCanonicalSectionOrder {
        offset: usize,
        expected: u8,
        actual: u8,
    },
    LengthOverflow {
        offset: usize,
        value: u64,
    },
    LengthLimitExceeded {
        offset: usize,
        value: u64,
    },
    SectionNotFullyConsumed {
        tag: u8,
        remaining: usize,
    },
    TrailingBytes {
        offset: usize,
        remaining: usize,
    },
    UnknownTag {
        offset: usize,
        kind: &'static str,
        tag: u8,
    },
    InvalidOptionTag {
        offset: usize,
        tag: u8,
    },
    InvalidBooleanTag {
        offset: usize,
        tag: u8,
    },
    NonCanonicalReencoding,
    InvalidBundle(BundleValidationErrorV1),
}

impl fmt::Display for WireErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd {
                offset,
                needed,
                remaining,
            } => write!(
                formatter,
                "unexpected end at {offset}: need {needed} bytes, have {remaining}"
            ),
            Self::MagicMismatch => formatter.write_str("production-wire magic mismatch"),
            Self::SchemaVersionMismatch(version) => {
                write!(formatter, "unsupported production-wire schema {version}")
            }
            Self::SectionCountMismatch(count) => {
                write!(formatter, "section count must be 11, got {count}")
            }
            Self::UnknownSectionTag { offset, tag } => {
                write!(formatter, "unknown section tag {tag} at {offset}")
            }
            Self::DuplicateSectionTag { offset, tag } => {
                write!(formatter, "duplicate section tag {tag} at {offset}")
            }
            Self::NonCanonicalSectionOrder {
                offset,
                expected,
                actual,
            } => write!(
                formatter,
                "noncanonical section order at {offset}: expected {expected}, got {actual}"
            ),
            Self::LengthOverflow { offset, value } => {
                write!(
                    formatter,
                    "length {value} at {offset} does not fit this host"
                )
            }
            Self::LengthLimitExceeded { offset, value } => {
                write!(
                    formatter,
                    "length {value} at {offset} exceeds the protocol limit"
                )
            }
            Self::SectionNotFullyConsumed { tag, remaining } => write!(
                formatter,
                "section {tag} has {remaining} unconsumed payload bytes"
            ),
            Self::TrailingBytes { offset, remaining } => {
                write!(formatter, "{remaining} trailing bytes at {offset}")
            }
            Self::UnknownTag { offset, kind, tag } => {
                write!(formatter, "unknown {kind} tag {tag} at {offset}")
            }
            Self::InvalidOptionTag { offset, tag } => {
                write!(formatter, "invalid option tag {tag} at {offset}")
            }
            Self::InvalidBooleanTag { offset, tag } => {
                write!(formatter, "invalid boolean tag {tag} at {offset}")
            }
            Self::NonCanonicalReencoding => {
                formatter.write_str("decoded bundle does not canonically re-encode byte-for-byte")
            }
            Self::InvalidBundle(error) => write!(formatter, "invalid bundle: {error}"),
        }
    }
}

impl std::error::Error for WireErrorV1 {}

impl From<BundleValidationErrorV1> for WireErrorV1 {
    fn from(error: BundleValidationErrorV1) -> Self {
        Self::InvalidBundle(error)
    }
}
