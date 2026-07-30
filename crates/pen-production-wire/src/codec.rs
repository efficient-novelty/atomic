use crate::error::{BundleValidationErrorV1, WireErrorV1};
use crate::model::*;
use crate::validate::validate_bundle_v1;

pub const MAX_CANONICAL_BUNDLE_BYTES_V1: usize = 64 * 1024 * 1024;
pub const MAX_SECTION_BYTES_V1: usize = 64 * 1024 * 1024;
pub const MAX_SEQUENCE_ITEMS_V1: usize = 1_000_000;
pub const MAX_RECURSIVE_DEPTH_V1: u16 = 256;

#[derive(Default)]
struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn byte(&mut self, value: u8) {
        self.bytes.push(value);
    }
    fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }
    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }
    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }
    fn raw(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
    }
    fn sized_bytes(&mut self, value: &[u8]) {
        self.u64(value.len() as u64);
        self.raw(value);
    }
    fn boolean(&mut self, value: bool) {
        self.byte(u8::from(value));
    }
    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
    depth: u16,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            offset: 0,
            depth: 0,
        }
    }
    fn remaining(&self) -> usize {
        self.bytes.len() - self.offset
    }
    fn take(&mut self, count: usize) -> Result<&'a [u8], WireErrorV1> {
        if count > self.remaining() {
            return Err(WireErrorV1::UnexpectedEnd {
                offset: self.offset,
                needed: count,
                remaining: self.remaining(),
            });
        }
        let start = self.offset;
        self.offset += count;
        Ok(&self.bytes[start..self.offset])
    }
    fn byte(&mut self) -> Result<u8, WireErrorV1> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, WireErrorV1> {
        Ok(u16::from_le_bytes(
            self.take(2)?.try_into().expect("length checked"),
        ))
    }
    fn u32(&mut self) -> Result<u32, WireErrorV1> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().expect("length checked"),
        ))
    }
    fn u64(&mut self) -> Result<u64, WireErrorV1> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().expect("length checked"),
        ))
    }
    fn bounded_len(&mut self, limit: usize) -> Result<usize, WireErrorV1> {
        let offset = self.offset;
        let value = self.u64()?;
        let length =
            usize::try_from(value).map_err(|_| WireErrorV1::LengthOverflow { offset, value })?;
        if length > limit {
            return Err(WireErrorV1::LengthLimitExceeded { offset, value });
        }
        Ok(length)
    }
    fn sized_bytes(&mut self) -> Result<Vec<u8>, WireErrorV1> {
        let length = self.bounded_len(MAX_SECTION_BYTES_V1)?;
        Ok(self.take(length)?.to_vec())
    }
    fn boolean(&mut self) -> Result<bool, WireErrorV1> {
        let offset = self.offset;
        match self.byte()? {
            0 => Ok(false),
            1 => Ok(true),
            tag => Err(WireErrorV1::InvalidBooleanTag { offset, tag }),
        }
    }
    fn recursive<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, WireErrorV1>,
    ) -> Result<T, WireErrorV1> {
        if self.depth >= MAX_RECURSIVE_DEPTH_V1 {
            return Err(BundleValidationErrorV1::TermDepthExceeded.into());
        }
        self.depth += 1;
        let result = f(self);
        self.depth -= 1;
        result
    }
}

trait WireCodec: Sized {
    fn encode(&self, writer: &mut Writer);
    fn decode(reader: &mut Reader<'_>) -> Result<Self, WireErrorV1>;
}

fn encode_vec<T: WireCodec>(writer: &mut Writer, values: &[T]) {
    writer.u64(values.len() as u64);
    for value in values {
        value.encode(writer);
    }
}

fn decode_vec<T: WireCodec>(reader: &mut Reader<'_>) -> Result<Vec<T>, WireErrorV1> {
    let count = reader.bounded_len(MAX_SEQUENCE_ITEMS_V1)?;
    let mut values = Vec::with_capacity(count.min(4096));
    for _ in 0..count {
        values.push(T::decode(reader)?);
    }
    Ok(values)
}

fn encode_option<T: WireCodec>(writer: &mut Writer, value: &Option<T>) {
    match value {
        None => writer.byte(0),
        Some(value) => {
            writer.byte(1);
            value.encode(writer);
        }
    }
}

fn decode_option<T: WireCodec>(reader: &mut Reader<'_>) -> Result<Option<T>, WireErrorV1> {
    let offset = reader.offset;
    match reader.byte()? {
        0 => Ok(None),
        1 => Ok(Some(T::decode(reader)?)),
        tag => Err(WireErrorV1::InvalidOptionTag { offset, tag }),
    }
}

macro_rules! tag_enum_codec {
    ($ty:ty, $kind:literal, {$($variant:path => $tag:expr),+ $(,)?}) => {
        impl WireCodec for $ty {
            fn encode(&self, writer: &mut Writer) {
                writer.byte(match self { $($variant => $tag),+ });
            }
            fn decode(reader: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
                let offset = reader.offset;
                match reader.byte()? {
                    $($tag => Ok($variant),)+
                    tag => Err(WireErrorV1::UnknownTag { offset, kind: $kind, tag }),
                }
            }
        }
    };
}

tag_enum_codec!(ManifestAuthorityWireV1, "manifest authority", {
    ManifestAuthorityWireV1::GenericPrototypeOnly => 0
});
tag_enum_codec!(SynthesisRuleWireV1, "synthesis inventory", {
    SynthesisRuleWireV1::Sort => 0,
    SynthesisRuleWireV1::UnitType => 1,
    SynthesisRuleWireV1::Unit => 2,
    SynthesisRuleWireV1::VariableLookup => 3,
    SynthesisRuleWireV1::GlobalLookup => 4,
    SynthesisRuleWireV1::PiFormation => 5,
    SynthesisRuleWireV1::LambdaIntroduction => 6,
    SynthesisRuleWireV1::ApplicationElimination => 7
});
tag_enum_codec!(ConversionPathComponentWireV1, "conversion path", {
    ConversionPathComponentWireV1::PiParameter => 0,
    ConversionPathComponentWireV1::PiBody => 1,
    ConversionPathComponentWireV1::LambdaParameter => 2,
    ConversionPathComponentWireV1::LambdaBody => 3,
    ConversionPathComponentWireV1::ApplyFunction => 4,
    ConversionPathComponentWireV1::ApplyArgument => 5
});
tag_enum_codec!(NoRedexDispositionWireV1, "no-redex disposition", {
    NoRedexDispositionWireV1::Sort => 0,
    NoRedexDispositionWireV1::Variable => 1,
    NoRedexDispositionWireV1::GlobalNotEnabledByPolicy => 2,
    NoRedexDispositionWireV1::Pi => 3,
    NoRedexDispositionWireV1::Lambda => 4,
    NoRedexDispositionWireV1::NeutralApplication => 5,
    NoRedexDispositionWireV1::UnitType => 6,
    NoRedexDispositionWireV1::Unit => 7
});
tag_enum_codec!(Q0RuleWireV1, "Q0 rule", {
    Q0RuleWireV1::DeBruijn => 0,
    Q0RuleWireV1::SequentialSubstitution => 1,
    Q0RuleWireV1::Beta => 2,
    Q0RuleWireV1::ProvenancePreservingDelta => 3,
    Q0RuleWireV1::Unit => 4,
    Q0RuleWireV1::TelescopeFlattening => 5,
    Q0RuleWireV1::FreshNonrecursiveConstructorComputation => 6
});
tag_enum_codec!(FamilyCodeWireV1, "family code", {
    FamilyCodeWireV1::Seed => 0,
    FamilyCodeWireV1::GenericPublicApplication => 1,
    FamilyCodeWireV1::GenericEquationAction => 2
});

impl WireCodec for WireIdV1 {
    fn encode(&self, writer: &mut Writer) {
        writer.raw(&self.0);
    }
    fn decode(reader: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self(reader.take(32)?.try_into().expect("length checked")))
    }
}

impl WireCodec for DeltaPolicyEntryWireV1 {
    fn encode(&self, w: &mut Writer) {
        w.u32(self.global_slot);
        self.global_id_bytes.encode(w);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            global_slot: r.u32()?,
            global_id_bytes: WireIdV1::decode(r)?,
        })
    }
}

impl WireCodec for V3CorrespondenceManifestWireV1 {
    fn encode(&self, w: &mut Writer) {
        w.u16(self.semantic_schema_version);
        w.sized_bytes(&self.profile_id);
        self.semantic_manifest_digest.encode(w);
        self.authority.encode(w);
        w.boolean(self.frozen);
        w.boolean(self.live_profile_a_access);
        self.production_inventory_bridge_digest.encode(w);
        self.predecessor_delta_policy_binding_digest.encode(w);
        w.sized_bytes(&self.synthesis_protocol_id);
        w.u16(self.synthesis_schema_version);
        encode_u16_vec(w, &self.public_universe_levels);
        encode_u16_vec(w, &self.checker_universe_levels);
        encode_u16_vec(w, &self.formation_witness_levels);
        w.u16(self.maximum_context_entries);
        encode_vec(w, &self.synthesis_rule_inventory);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            semantic_schema_version: r.u16()?,
            profile_id: r.sized_bytes()?,
            semantic_manifest_digest: WireIdV1::decode(r)?,
            authority: ManifestAuthorityWireV1::decode(r)?,
            frozen: r.boolean()?,
            live_profile_a_access: r.boolean()?,
            production_inventory_bridge_digest: WireIdV1::decode(r)?,
            predecessor_delta_policy_binding_digest: WireIdV1::decode(r)?,
            synthesis_protocol_id: r.sized_bytes()?,
            synthesis_schema_version: r.u16()?,
            public_universe_levels: decode_u16_vec(r)?,
            checker_universe_levels: decode_u16_vec(r)?,
            formation_witness_levels: decode_u16_vec(r)?,
            maximum_context_entries: r.u16()?,
            synthesis_rule_inventory: decode_vec(r)?,
        })
    }
}

fn encode_u16_vec(w: &mut Writer, values: &[u16]) {
    w.u64(values.len() as u64);
    for value in values {
        w.u16(*value);
    }
}
fn decode_u16_vec(r: &mut Reader<'_>) -> Result<Vec<u16>, WireErrorV1> {
    let count = r.bounded_len(MAX_SEQUENCE_ITEMS_V1)?;
    let mut values = Vec::with_capacity(count.min(4096));
    for _ in 0..count {
        values.push(r.u16()?);
    }
    Ok(values)
}

impl WireCodec for ProductionSignatureWireV1 {
    fn encode(&self, w: &mut Writer) {
        self.signature_digest.encode(w);
        self.kernel_protocol_digest.encode(w);
        self.global_slot_table_digest.encode(w);
        encode_vec(w, &self.allowed_transparent_deltas);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            signature_digest: WireIdV1::decode(r)?,
            kernel_protocol_digest: WireIdV1::decode(r)?,
            global_slot_table_digest: WireIdV1::decode(r)?,
            allowed_transparent_deltas: decode_vec(r)?,
        })
    }
}

impl WireCodec for WireTermV1 {
    fn encode(&self, w: &mut Writer) {
        match self {
            Self::Sort { level } => {
                w.byte(0);
                w.u16(*level);
            }
            Self::Variable { index } => {
                w.byte(1);
                w.u32(*index);
            }
            Self::GlobalSlot { slot } => {
                w.byte(2);
                w.u32(*slot);
            }
            Self::Pi { parameter, body } => {
                w.byte(3);
                parameter.encode(w);
                body.encode(w);
            }
            Self::Lambda { parameter, body } => {
                w.byte(4);
                parameter.encode(w);
                body.encode(w);
            }
            Self::Apply { function, argument } => {
                w.byte(5);
                function.encode(w);
                argument.encode(w);
            }
            Self::UnitType => w.byte(6),
            Self::Unit => w.byte(7),
        }
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        r.recursive(|r| {
            let offset = r.offset;
            match r.byte()? {
                0 => Ok(Self::Sort { level: r.u16()? }),
                1 => Ok(Self::Variable { index: r.u32()? }),
                2 => Ok(Self::GlobalSlot { slot: r.u32()? }),
                3 => Ok(Self::Pi {
                    parameter: Box::new(Self::decode(r)?),
                    body: Box::new(Self::decode(r)?),
                }),
                4 => Ok(Self::Lambda {
                    parameter: Box::new(Self::decode(r)?),
                    body: Box::new(Self::decode(r)?),
                }),
                5 => Ok(Self::Apply {
                    function: Box::new(Self::decode(r)?),
                    argument: Box::new(Self::decode(r)?),
                }),
                6 => Ok(Self::UnitType),
                7 => Ok(Self::Unit),
                tag => Err(WireErrorV1::UnknownTag {
                    offset,
                    kind: "term",
                    tag,
                }),
            }
        })
    }
}

impl WireCodec for GlobalSlotEntryWireV1 {
    fn encode(&self, w: &mut Writer) {
        w.u32(self.slot);
        self.global_id_bytes.encode(w);
        self.declaration_type.encode(w);
        encode_option(w, &self.declaration_body);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            slot: r.u32()?,
            global_id_bytes: WireIdV1::decode(r)?,
            declaration_type: WireTermV1::decode(r)?,
            declaration_body: decode_option(r)?,
        })
    }
}
impl WireCodec for GlobalSlotTableWireV1 {
    fn encode(&self, w: &mut Writer) {
        encode_vec(w, &self.entries);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            entries: decode_vec(r)?,
        })
    }
}
impl WireCodec for ProductionContextWireV1 {
    fn encode(&self, w: &mut Writer) {
        encode_vec(w, &self.entries_oldest_first);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            entries_oldest_first: decode_vec(r)?,
        })
    }
}
impl WireCodec for NoRedexEntryWireV1 {
    fn encode(&self, w: &mut Writer) {
        encode_vec(w, &self.path);
        self.term.encode(w);
        self.disposition.encode(w);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            path: decode_vec(r)?,
            term: WireTermV1::decode(r)?,
            disposition: NoRedexDispositionWireV1::decode(r)?,
        })
    }
}
impl WireCodec for NoRedexCensusWireV1 {
    fn encode(&self, w: &mut Writer) {
        encode_vec(w, &self.entries);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            entries: decode_vec(r)?,
        })
    }
}

impl WireCodec for BaseQ0ReductionStepWireV1 {
    fn encode(&self, w: &mut Writer) {
        let encode_congruence =
            |w: &mut Writer, tag, source: &WireTermV1, target: &WireTermV1, premise: &Self| {
                w.byte(tag);
                source.encode(w);
                target.encode(w);
                premise.encode(w);
            };
        match self {
            Self::Beta { source, target } => {
                w.byte(0);
                source.encode(w);
                target.encode(w);
            }
            Self::TransparentDelta {
                source,
                target,
                global_slot,
            } => {
                w.byte(1);
                source.encode(w);
                target.encode(w);
                w.u32(*global_slot);
            }
            Self::PiParameterCongruence {
                source,
                target,
                premise,
            } => encode_congruence(w, 2, source, target, premise),
            Self::PiBodyCongruence {
                source,
                target,
                premise,
            } => encode_congruence(w, 3, source, target, premise),
            Self::LambdaParameterCongruence {
                source,
                target,
                premise,
            } => encode_congruence(w, 4, source, target, premise),
            Self::LambdaBodyCongruence {
                source,
                target,
                premise,
            } => encode_congruence(w, 5, source, target, premise),
            Self::ApplyFunctionCongruence {
                source,
                target,
                premise,
            } => encode_congruence(w, 6, source, target, premise),
            Self::ApplyArgumentCongruence {
                source,
                target,
                premise,
            } => encode_congruence(w, 7, source, target, premise),
        }
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        r.recursive(|r| {
            let offset = r.offset;
            let tag = r.byte()?;
            let source = WireTermV1::decode(r)?;
            let target = WireTermV1::decode(r)?;
            Ok(match tag {
                0 => Self::Beta { source, target },
                1 => Self::TransparentDelta {
                    source,
                    target,
                    global_slot: r.u32()?,
                },
                2 => Self::PiParameterCongruence {
                    source,
                    target,
                    premise: Box::new(Self::decode(r)?),
                },
                3 => Self::PiBodyCongruence {
                    source,
                    target,
                    premise: Box::new(Self::decode(r)?),
                },
                4 => Self::LambdaParameterCongruence {
                    source,
                    target,
                    premise: Box::new(Self::decode(r)?),
                },
                5 => Self::LambdaBodyCongruence {
                    source,
                    target,
                    premise: Box::new(Self::decode(r)?),
                },
                6 => Self::ApplyFunctionCongruence {
                    source,
                    target,
                    premise: Box::new(Self::decode(r)?),
                },
                7 => Self::ApplyArgumentCongruence {
                    source,
                    target,
                    premise: Box::new(Self::decode(r)?),
                },
                tag => {
                    return Err(WireErrorV1::UnknownTag {
                        offset,
                        kind: "conversion step",
                        tag,
                    });
                }
            })
        })
    }
}

impl WireCodec for BaseQ0ReductionTraceWireV1 {
    fn encode(&self, w: &mut Writer) {
        self.start.encode(w);
        encode_vec(w, &self.steps);
        self.end.encode(w);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            start: WireTermV1::decode(r)?,
            steps: decode_vec(r)?,
            end: WireTermV1::decode(r)?,
        })
    }
}
impl WireCodec for EndpointJudgmentWireV1 {
    fn encode(&self, w: &mut Writer) {
        match self {
            Self::HasType { expected_type } => {
                w.byte(0);
                expected_type.encode(w);
            }
            Self::TypeFormation => w.byte(1),
        }
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        let offset = r.offset;
        match r.byte()? {
            0 => Ok(Self::HasType {
                expected_type: WireTermV1::decode(r)?,
            }),
            1 => Ok(Self::TypeFormation),
            tag => Err(WireErrorV1::UnknownTag {
                offset,
                kind: "endpoint judgment",
                tag,
            }),
        }
    }
}
impl WireCodec for ConversionCertificateWireV1 {
    fn encode(&self, w: &mut Writer) {
        self.conversion_id.encode(w);
        self.context.encode(w);
        self.left.encode(w);
        self.right.encode(w);
        self.endpoint_judgment.encode(w);
        self.common_normal_form.encode(w);
        self.left_trace.encode(w);
        self.right_trace.encode(w);
        self.no_redex_census.encode(w);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            conversion_id: WireIdV1::decode(r)?,
            context: ProductionContextWireV1::decode(r)?,
            left: WireTermV1::decode(r)?,
            right: WireTermV1::decode(r)?,
            endpoint_judgment: EndpointJudgmentWireV1::decode(r)?,
            common_normal_form: WireTermV1::decode(r)?,
            left_trace: BaseQ0ReductionTraceWireV1::decode(r)?,
            right_trace: BaseQ0ReductionTraceWireV1::decode(r)?,
            no_redex_census: NoRedexCensusWireV1::decode(r)?,
        })
    }
}

impl WireCodec for SynthesisCodeWireV1 {
    fn encode(&self, w: &mut Writer) {
        match self {
            Self::Sort { level } => {
                w.byte(0);
                w.u16(*level);
            }
            Self::UnitType => w.byte(1),
            Self::Unit => w.byte(2),
            Self::VariableLookup {
                index,
                context_ordinal,
                shift_distance,
            } => {
                w.byte(3);
                w.u32(*index);
                w.u32(*context_ordinal);
                w.u32(*shift_distance);
            }
            Self::GlobalLookup { global_slot } => {
                w.byte(4);
                w.u32(*global_slot);
            }
            Self::PiFormation { parameter, body } => {
                w.byte(5);
                parameter.encode(w);
                body.encode(w);
            }
            Self::LambdaIntroduction {
                parameter_type,
                body,
            } => {
                w.byte(6);
                parameter_type.encode(w);
                body.encode(w);
            }
            Self::ApplicationElimination {
                function,
                argument,
                function_conversion_id,
                argument_conversion_id,
                dependent_result_type,
            } => {
                w.byte(7);
                function.encode(w);
                argument.encode(w);
                function_conversion_id.encode(w);
                argument_conversion_id.encode(w);
                dependent_result_type.encode(w);
            }
        }
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        r.recursive(|r| {
            let offset = r.offset;
            match r.byte()? {
                0 => Ok(Self::Sort { level: r.u16()? }),
                1 => Ok(Self::UnitType),
                2 => Ok(Self::Unit),
                3 => Ok(Self::VariableLookup {
                    index: r.u32()?,
                    context_ordinal: r.u32()?,
                    shift_distance: r.u32()?,
                }),
                4 => Ok(Self::GlobalLookup {
                    global_slot: r.u32()?,
                }),
                5 => Ok(Self::PiFormation {
                    parameter: Box::new(Self::decode(r)?),
                    body: Box::new(Self::decode(r)?),
                }),
                6 => Ok(Self::LambdaIntroduction {
                    parameter_type: Box::new(Self::decode(r)?),
                    body: Box::new(Self::decode(r)?),
                }),
                7 => Ok(Self::ApplicationElimination {
                    function: Box::new(Self::decode(r)?),
                    argument: Box::new(Self::decode(r)?),
                    function_conversion_id: WireIdV1::decode(r)?,
                    argument_conversion_id: WireIdV1::decode(r)?,
                    dependent_result_type: WireTermV1::decode(r)?,
                }),
                tag => Err(WireErrorV1::UnknownTag {
                    offset,
                    kind: "synthesis code",
                    tag,
                }),
            }
        })
    }
}
impl WireCodec for SynthesisCertificateWireV1 {
    fn encode(&self, w: &mut Writer) {
        self.synthesis_id.encode(w);
        self.context.encode(w);
        self.subject.encode(w);
        self.inferred_type.encode(w);
        self.code.encode(w);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            synthesis_id: WireIdV1::decode(r)?,
            context: ProductionContextWireV1::decode(r)?,
            subject: WireTermV1::decode(r)?,
            inferred_type: WireTermV1::decode(r)?,
            code: SynthesisCodeWireV1::decode(r)?,
        })
    }
}
impl WireCodec for ConversionTypingSupplementWireV1 {
    fn encode(&self, w: &mut Writer) {
        self.conversion_id.encode(w);
        encode_u32_vec(w, &self.step_path);
        self.local_context.encode(w);
        self.local_endpoint_judgment.encode(w);
        self.source_typing_code.encode(w);
        self.target_typing_code.encode(w);
        encode_u16_option(w, self.formation_level);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            conversion_id: WireIdV1::decode(r)?,
            step_path: decode_u32_vec(r)?,
            local_context: ProductionContextWireV1::decode(r)?,
            local_endpoint_judgment: EndpointJudgmentWireV1::decode(r)?,
            source_typing_code: SynthesisCertificateWireV1::decode(r)?,
            target_typing_code: SynthesisCertificateWireV1::decode(r)?,
            formation_level: decode_u16_option(r)?,
        })
    }
}
fn encode_u32_vec(w: &mut Writer, values: &[u32]) {
    w.u64(values.len() as u64);
    for value in values {
        w.u32(*value);
    }
}
fn decode_u32_vec(r: &mut Reader<'_>) -> Result<Vec<u32>, WireErrorV1> {
    let count = r.bounded_len(MAX_SEQUENCE_ITEMS_V1)?;
    let mut values = Vec::with_capacity(count.min(4096));
    for _ in 0..count {
        values.push(r.u32()?);
    }
    Ok(values)
}
fn encode_u16_option(w: &mut Writer, value: Option<u16>) {
    match value {
        None => w.byte(0),
        Some(value) => {
            w.byte(1);
            w.u16(value);
        }
    }
}
fn decode_u16_option(r: &mut Reader<'_>) -> Result<Option<u16>, WireErrorV1> {
    let offset = r.offset;
    match r.byte()? {
        0 => Ok(None),
        1 => Ok(Some(r.u16()?)),
        tag => Err(WireErrorV1::InvalidOptionTag { offset, tag }),
    }
}

impl WireCodec for Q0InventoryWireV1 {
    fn encode(&self, w: &mut Writer) {
        encode_vec(w, &self.ordered_rules);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            ordered_rules: decode_vec(r)?,
        })
    }
}
impl WireCodec for FreshRuleSchemaWireV1 {
    fn encode(&self, w: &mut Writer) {
        self.equation_id.encode(w);
        w.u32(self.owner_slot);
        w.u32(self.constructor_slot);
        self.parameter_context.encode(w);
        self.left.encode(w);
        self.right.encode(w);
        self.ty.encode(w);
        w.u32(self.scrutinee_ordinal);
        w.u16(self.arity);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            equation_id: WireIdV1::decode(r)?,
            owner_slot: r.u32()?,
            constructor_slot: r.u32()?,
            parameter_context: ProductionContextWireV1::decode(r)?,
            left: WireTermV1::decode(r)?,
            right: WireTermV1::decode(r)?,
            ty: WireTermV1::decode(r)?,
            scrutinee_ordinal: r.u32()?,
            arity: r.u16()?,
        })
    }
}
impl WireCodec for FamilyInventoryWireV1 {
    fn encode(&self, w: &mut Writer) {
        encode_vec(w, &self.ordered_codes);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            ordered_codes: decode_vec(r)?,
        })
    }
}
impl WireCodec for FamilyJudgmentWireV1 {
    fn encode(&self, w: &mut Writer) {
        self.context.encode(w);
        self.subject.encode(w);
        self.ty.encode(w);
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        Ok(Self {
            context: ProductionContextWireV1::decode(r)?,
            subject: WireTermV1::decode(r)?,
            ty: WireTermV1::decode(r)?,
        })
    }
}
impl WireCodec for SeedSourceWireV1 {
    fn encode(&self, w: &mut Writer) {
        match self {
            Self::PublicHead { owner_slot } => {
                w.byte(0);
                w.u32(*owner_slot);
            }
            Self::PublicEquation { equation_id } => {
                w.byte(1);
                equation_id.encode(w);
            }
        }
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        let offset = r.offset;
        match r.byte()? {
            0 => Ok(Self::PublicHead {
                owner_slot: r.u32()?,
            }),
            1 => Ok(Self::PublicEquation {
                equation_id: WireIdV1::decode(r)?,
            }),
            tag => Err(WireErrorV1::UnknownTag {
                offset,
                kind: "seed source",
                tag,
            }),
        }
    }
}
impl WireCodec for FamilyPayloadWireV1 {
    fn encode(&self, w: &mut Writer) {
        match self {
            Self::Seed {
                family_id,
                source,
                judgment,
            } => {
                w.byte(0);
                family_id.encode(w);
                source.encode(w);
                judgment.encode(w);
            }
            Self::GenericPublicApplication {
                family_id,
                function_family_id,
                argument_family_id,
                judgment,
            } => {
                w.byte(1);
                family_id.encode(w);
                function_family_id.encode(w);
                argument_family_id.encode(w);
                judgment.encode(w);
            }
            Self::GenericEquationAction {
                family_id,
                equation_id,
                source_family_id,
                judgment,
            } => {
                w.byte(2);
                family_id.encode(w);
                equation_id.encode(w);
                source_family_id.encode(w);
                judgment.encode(w);
            }
        }
    }
    fn decode(r: &mut Reader<'_>) -> Result<Self, WireErrorV1> {
        let offset = r.offset;
        match r.byte()? {
            0 => Ok(Self::Seed {
                family_id: WireIdV1::decode(r)?,
                source: SeedSourceWireV1::decode(r)?,
                judgment: FamilyJudgmentWireV1::decode(r)?,
            }),
            1 => Ok(Self::GenericPublicApplication {
                family_id: WireIdV1::decode(r)?,
                function_family_id: WireIdV1::decode(r)?,
                argument_family_id: WireIdV1::decode(r)?,
                judgment: FamilyJudgmentWireV1::decode(r)?,
            }),
            2 => Ok(Self::GenericEquationAction {
                family_id: WireIdV1::decode(r)?,
                equation_id: WireIdV1::decode(r)?,
                source_family_id: WireIdV1::decode(r)?,
                judgment: FamilyJudgmentWireV1::decode(r)?,
            }),
            tag => Err(WireErrorV1::UnknownTag {
                offset,
                kind: "family payload",
                tag,
            }),
        }
    }
}

fn encode_section(writer: &mut Writer, tag: u8, encode: impl FnOnce(&mut Writer)) {
    let mut payload = Writer::default();
    encode(&mut payload);
    let payload = payload.finish();
    writer.byte(tag);
    writer.u64(payload.len() as u64);
    writer.raw(&payload);
}

fn encode_bundle_unchecked(bundle: &ProductionRefinementBundleV1) -> Vec<u8> {
    let mut w = Writer::default();
    w.raw(&PRODUCTION_REFINEMENT_MAGIC_V1);
    w.u16(PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1);
    w.u16(PRODUCTION_REFINEMENT_SECTION_COUNT_V1);
    encode_section(&mut w, 1, |w| bundle.manifest_surface.encode(w));
    encode_section(&mut w, 2, |w| bundle.signature.encode(w));
    encode_section(&mut w, 3, |w| bundle.global_slot_table.encode(w));
    encode_section(&mut w, 4, |w| encode_vec(w, &bundle.contexts));
    encode_section(&mut w, 5, |w| encode_vec(w, &bundle.conversions));
    encode_section(&mut w, 6, |w| {
        encode_vec(w, &bundle.conversion_typing_supplements)
    });
    encode_section(&mut w, 7, |w| encode_vec(w, &bundle.synthesis_codes));
    encode_section(&mut w, 8, |w| bundle.q0_inventory.encode(w));
    encode_section(&mut w, 9, |w| encode_vec(w, &bundle.fresh_rule_schemas));
    encode_section(&mut w, 10, |w| bundle.family_inventory.encode(w));
    encode_section(&mut w, 11, |w| encode_vec(w, &bundle.family_payloads));
    w.finish()
}

pub fn encode_bundle_v1(bundle: &ProductionRefinementBundleV1) -> Result<Vec<u8>, WireErrorV1> {
    validate_bundle_v1(bundle)?;
    let bytes = encode_bundle_unchecked(bundle);
    if bytes.len() > MAX_CANONICAL_BUNDLE_BYTES_V1 {
        return Err(WireErrorV1::LengthLimitExceeded {
            offset: 0,
            value: bytes.len() as u64,
        });
    }
    Ok(bytes)
}

pub fn canonical_reencode_bundle_v1(
    bundle: &ProductionRefinementBundleV1,
) -> Result<Vec<u8>, WireErrorV1> {
    encode_bundle_v1(bundle)
}

fn decode_section<T>(
    reader: &mut Reader<'_>,
    expected: u8,
    seen: &mut [bool; 12],
    decode: impl FnOnce(&mut Reader<'_>) -> Result<T, WireErrorV1>,
) -> Result<T, WireErrorV1> {
    let offset = reader.offset;
    let tag = reader.byte()?;
    if !(1..=11).contains(&tag) {
        return Err(WireErrorV1::UnknownSectionTag { offset, tag });
    }
    if seen[tag as usize] {
        return Err(WireErrorV1::DuplicateSectionTag { offset, tag });
    }
    seen[tag as usize] = true;
    if tag != expected {
        return Err(WireErrorV1::NonCanonicalSectionOrder {
            offset,
            expected,
            actual: tag,
        });
    }
    let length = reader.bounded_len(MAX_SECTION_BYTES_V1)?;
    let payload = reader.take(length)?;
    let mut section = Reader::new(payload);
    let value = decode(&mut section)?;
    if section.remaining() != 0 {
        return Err(WireErrorV1::SectionNotFullyConsumed {
            tag,
            remaining: section.remaining(),
        });
    }
    Ok(value)
}

pub fn decode_bundle_v1(bytes: &[u8]) -> Result<ProductionRefinementBundleV1, WireErrorV1> {
    if bytes.len() > MAX_CANONICAL_BUNDLE_BYTES_V1 {
        return Err(WireErrorV1::LengthLimitExceeded {
            offset: 0,
            value: bytes.len() as u64,
        });
    }
    let mut r = Reader::new(bytes);
    if r.take(16)? != PRODUCTION_REFINEMENT_MAGIC_V1 {
        return Err(WireErrorV1::MagicMismatch);
    }
    let version = r.u16()?;
    if version != PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1 {
        return Err(WireErrorV1::SchemaVersionMismatch(version));
    }
    let count = r.u16()?;
    if count != PRODUCTION_REFINEMENT_SECTION_COUNT_V1 {
        return Err(WireErrorV1::SectionCountMismatch(count));
    }
    let mut seen = [false; 12];
    let manifest_surface =
        decode_section(&mut r, 1, &mut seen, V3CorrespondenceManifestWireV1::decode)?;
    let signature = decode_section(&mut r, 2, &mut seen, ProductionSignatureWireV1::decode)?;
    let global_slot_table = decode_section(&mut r, 3, &mut seen, GlobalSlotTableWireV1::decode)?;
    let contexts = decode_section(&mut r, 4, &mut seen, decode_vec)?;
    let conversions = decode_section(&mut r, 5, &mut seen, decode_vec)?;
    let conversion_typing_supplements = decode_section(&mut r, 6, &mut seen, decode_vec)?;
    let synthesis_codes = decode_section(&mut r, 7, &mut seen, decode_vec)?;
    let q0_inventory = decode_section(&mut r, 8, &mut seen, Q0InventoryWireV1::decode)?;
    let fresh_rule_schemas = decode_section(&mut r, 9, &mut seen, decode_vec)?;
    let family_inventory = decode_section(&mut r, 10, &mut seen, FamilyInventoryWireV1::decode)?;
    let family_payloads = decode_section(&mut r, 11, &mut seen, decode_vec)?;
    if r.remaining() != 0 {
        return Err(WireErrorV1::TrailingBytes {
            offset: r.offset,
            remaining: r.remaining(),
        });
    }
    let bundle = ProductionRefinementBundleV1 {
        header: WireHeaderV1::canonical(),
        manifest_surface,
        signature,
        global_slot_table,
        contexts,
        conversions,
        conversion_typing_supplements,
        synthesis_codes,
        q0_inventory,
        fresh_rule_schemas,
        family_inventory,
        family_payloads,
    };
    validate_bundle_v1(&bundle)?;
    if encode_bundle_unchecked(&bundle).as_slice() != bytes {
        return Err(WireErrorV1::NonCanonicalReencoding);
    }
    Ok(bundle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TermRoleV1;

    fn id(byte: u8) -> WireIdV1 {
        WireIdV1([byte; 32])
    }

    fn unit_synthesis(synthesis_id: WireIdV1) -> SynthesisCertificateWireV1 {
        SynthesisCertificateWireV1 {
            synthesis_id,
            context: ProductionContextWireV1::default(),
            subject: WireTermV1::Unit,
            inferred_type: WireTermV1::UnitType,
            code: SynthesisCodeWireV1::Unit,
        }
    }

    fn canonical_bundle() -> ProductionRefinementBundleV1 {
        let empty = ProductionContextWireV1::default();
        let one_local = ProductionContextWireV1 {
            entries_oldest_first: vec![WireTermV1::UnitType],
        };
        let common = WireTermV1::Unit;
        ProductionRefinementBundleV1 {
            header: WireHeaderV1::canonical(),
            manifest_surface: V3CorrespondenceManifestWireV1 {
                semantic_schema_version: V3_SEMANTIC_SCHEMA_VERSION,
                profile_id: V3_SEMANTIC_PROFILE_ID.to_vec(),
                semantic_manifest_digest: id(20),
                authority: ManifestAuthorityWireV1::GenericPrototypeOnly,
                frozen: false,
                live_profile_a_access: false,
                production_inventory_bridge_digest: id(21),
                public_universe_levels: PUBLIC_UNIVERSE_LEVELS_V1.to_vec(),
                checker_universe_levels: CHECKER_UNIVERSE_LEVELS_V1.to_vec(),
                formation_witness_levels: FORMATION_WITNESS_LEVELS_V1.to_vec(),
                maximum_context_entries: 32,
                synthesis_rule_inventory: EXACT_SYNTHESIS_INVENTORY_V1.to_vec(),
                predecessor_delta_policy_binding_digest: id(22),
                synthesis_protocol_id: SYNTHESIS_PROTOCOL_ID_V2.to_vec(),
                synthesis_schema_version: SYNTHESIS_SCHEMA_VERSION_V2,
            },
            signature: ProductionSignatureWireV1 {
                signature_digest: id(23),
                kernel_protocol_digest: id(24),
                global_slot_table_digest: id(25),
                allowed_transparent_deltas: vec![DeltaPolicyEntryWireV1 {
                    global_slot: 0,
                    global_id_bytes: id(1),
                }],
            },
            global_slot_table: GlobalSlotTableWireV1 {
                entries: vec![
                    GlobalSlotEntryWireV1 {
                        slot: 0,
                        global_id_bytes: id(1),
                        declaration_type: WireTermV1::UnitType,
                        declaration_body: Some(WireTermV1::Unit),
                    },
                    GlobalSlotEntryWireV1 {
                        slot: 1,
                        global_id_bytes: id(2),
                        declaration_type: WireTermV1::GlobalSlot { slot: 0 },
                        declaration_body: None,
                    },
                ],
            },
            contexts: vec![empty.clone(), one_local.clone()],
            conversions: vec![ConversionCertificateWireV1 {
                conversion_id: id(5),
                context: empty.clone(),
                left: WireTermV1::GlobalSlot { slot: 0 },
                right: common.clone(),
                endpoint_judgment: EndpointJudgmentWireV1::HasType {
                    expected_type: WireTermV1::UnitType,
                },
                common_normal_form: common.clone(),
                left_trace: BaseQ0ReductionTraceWireV1 {
                    start: WireTermV1::GlobalSlot { slot: 0 },
                    steps: vec![BaseQ0ReductionStepWireV1::TransparentDelta {
                        source: WireTermV1::GlobalSlot { slot: 0 },
                        target: common.clone(),
                        global_slot: 0,
                    }],
                    end: common.clone(),
                },
                right_trace: BaseQ0ReductionTraceWireV1 {
                    start: common.clone(),
                    steps: Vec::new(),
                    end: common.clone(),
                },
                no_redex_census: NoRedexCensusWireV1 {
                    entries: vec![NoRedexEntryWireV1 {
                        path: Vec::new(),
                        term: common,
                        disposition: NoRedexDispositionWireV1::Unit,
                    }],
                },
            }],
            conversion_typing_supplements: Vec::new(),
            synthesis_codes: vec![SynthesisCertificateWireV1 {
                synthesis_id: id(6),
                context: one_local.clone(),
                subject: WireTermV1::Variable { index: 0 },
                inferred_type: WireTermV1::UnitType,
                code: SynthesisCodeWireV1::VariableLookup {
                    index: 0,
                    context_ordinal: 0,
                    shift_distance: 1,
                },
            }],
            q0_inventory: Q0InventoryWireV1 {
                ordered_rules: EXACT_Q0_INVENTORY_V1.to_vec(),
            },
            fresh_rule_schemas: vec![FreshRuleSchemaWireV1 {
                equation_id: id(7),
                owner_slot: 0,
                constructor_slot: 1,
                parameter_context: one_local,
                left: WireTermV1::Apply {
                    function: Box::new(WireTermV1::GlobalSlot { slot: 0 }),
                    argument: Box::new(WireTermV1::GlobalSlot { slot: 1 }),
                },
                right: WireTermV1::Variable { index: 0 },
                ty: WireTermV1::UnitType,
                scrutinee_ordinal: 0,
                arity: 1,
            }],
            family_inventory: FamilyInventoryWireV1 {
                ordered_codes: EXACT_FAMILY_INVENTORY_V1.to_vec(),
            },
            family_payloads: vec![
                FamilyPayloadWireV1::Seed {
                    family_id: id(8),
                    source: SeedSourceWireV1::PublicHead { owner_slot: 0 },
                    judgment: FamilyJudgmentWireV1 {
                        context: empty.clone(),
                        subject: WireTermV1::Unit,
                        ty: WireTermV1::UnitType,
                    },
                },
                FamilyPayloadWireV1::Seed {
                    family_id: id(9),
                    source: SeedSourceWireV1::PublicEquation { equation_id: id(7) },
                    judgment: FamilyJudgmentWireV1 {
                        context: empty.clone(),
                        subject: WireTermV1::Unit,
                        ty: WireTermV1::UnitType,
                    },
                },
                FamilyPayloadWireV1::GenericPublicApplication {
                    family_id: id(10),
                    function_family_id: id(8),
                    argument_family_id: id(9),
                    judgment: FamilyJudgmentWireV1 {
                        context: empty.clone(),
                        subject: WireTermV1::Unit,
                        ty: WireTermV1::UnitType,
                    },
                },
                FamilyPayloadWireV1::GenericEquationAction {
                    family_id: id(11),
                    equation_id: id(7),
                    source_family_id: id(8),
                    judgment: FamilyJudgmentWireV1 {
                        context: empty,
                        subject: WireTermV1::Unit,
                        ty: WireTermV1::UnitType,
                    },
                },
            ],
        }
    }

    fn section_offsets(bytes: &[u8]) -> Vec<(usize, usize, usize)> {
        let mut cursor = 20;
        let mut result = Vec::new();
        for _ in 0..11 {
            let tag_offset = cursor;
            let length =
                u64::from_le_bytes(bytes[cursor + 1..cursor + 9].try_into().unwrap()) as usize;
            let payload_offset = cursor + 9;
            result.push((tag_offset, payload_offset, length));
            cursor = payload_offset + length;
        }
        assert_eq!(cursor, bytes.len());
        result
    }

    fn assert_invalid(bundle: ProductionRefinementBundleV1, expected: BundleValidationErrorV1) {
        assert_eq!(
            encode_bundle_v1(&bundle),
            Err(WireErrorV1::InvalidBundle(expected))
        );
    }

    #[test]
    fn exact_round_trip_is_byte_stable() {
        let bundle = canonical_bundle();
        let bytes = encode_bundle_v1(&bundle).unwrap();
        let decoded = decode_bundle_v1(&bytes).unwrap();
        assert_eq!(decoded, bundle);
        assert_eq!(canonical_reencode_bundle_v1(&decoded).unwrap(), bytes);
    }

    #[test]
    fn every_truncation_and_trailing_byte_is_rejected() {
        let bytes = encode_bundle_v1(&canonical_bundle()).unwrap();
        for cutoff in 0..bytes.len() {
            assert!(
                decode_bundle_v1(&bytes[..cutoff]).is_err(),
                "accepted cutoff {cutoff}"
            );
        }
        let mut trailing = bytes;
        trailing.push(0);
        assert!(matches!(
            decode_bundle_v1(&trailing),
            Err(WireErrorV1::TrailingBytes { remaining: 1, .. })
        ));
    }

    #[test]
    fn header_section_and_length_mutations_are_rejected() {
        let bytes = encode_bundle_v1(&canonical_bundle()).unwrap();
        let sections = section_offsets(&bytes);

        let mut bad = bytes.clone();
        bad[0] ^= 1;
        assert_eq!(decode_bundle_v1(&bad), Err(WireErrorV1::MagicMismatch));

        let mut bad = bytes.clone();
        bad[16..18].copy_from_slice(&2u16.to_le_bytes());
        assert_eq!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::SchemaVersionMismatch(2))
        );

        let mut bad = bytes.clone();
        bad[18..20].copy_from_slice(&10u16.to_le_bytes());
        assert_eq!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::SectionCountMismatch(10))
        );

        let mut bad = bytes.clone();
        bad[sections[0].0] = 12;
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::UnknownSectionTag { tag: 12, .. })
        ));

        let mut bad = bytes.clone();
        bad[sections[0].0] = 2;
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::NonCanonicalSectionOrder { .. })
        ));

        let mut bad = bytes.clone();
        bad[sections[1].0] = 1;
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::DuplicateSectionTag { tag: 1, .. })
        ));

        let mut bad = bytes.clone();
        bad[sections[0].0 + 1..sections[0].0 + 9].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::LengthLimitExceeded { .. }) | Err(WireErrorV1::LengthOverflow { .. })
        ));

        let mut bad = bytes;
        let (tag, payload, length) = sections[0];
        bad[tag + 1..tag + 9].copy_from_slice(&((length + 1) as u64).to_le_bytes());
        bad.insert(payload + length, 0);
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::SectionNotFullyConsumed {
                tag: 1,
                remaining: 1
            })
        ));
    }

    #[test]
    fn primitive_and_term_tag_mutations_are_rejected() {
        let bytes = encode_bundle_v1(&canonical_bundle()).unwrap();
        let sections = section_offsets(&bytes);
        let manifest_payload = sections[0].1;
        let authority = manifest_payload + 2 + 8 + V3_SEMANTIC_PROFILE_ID.len() + 32;

        let mut bad = bytes.clone();
        bad[authority] = 255;
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::UnknownTag {
                kind: "manifest authority",
                tag: 255,
                ..
            })
        ));

        let mut bad = bytes.clone();
        bad[authority + 1] = 2;
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::InvalidBooleanTag { tag: 2, .. })
        ));

        let slot_payload = sections[2].1;
        let first_term_tag = slot_payload + 8 + 4 + 32;
        let mut bad = bytes.clone();
        bad[first_term_tag] = 255;
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::UnknownTag {
                kind: "term",
                tag: 255,
                ..
            })
        ));

        let mut bad = bytes.clone();
        bad[first_term_tag + 1] = 2;
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::InvalidOptionTag { tag: 2, .. })
        ));

        let mut bad = bytes;
        let contexts_payload = sections[3].1;
        bad[contexts_payload..contexts_payload + 8].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(matches!(
            decode_bundle_v1(&bad),
            Err(WireErrorV1::LengthLimitExceeded { .. }) | Err(WireErrorV1::LengthOverflow { .. })
        ));
    }

    #[test]
    fn slot_context_universe_and_delta_invariants_fail_closed() {
        let mut bundle = canonical_bundle();
        bundle.global_slot_table.entries[0].slot = 1;
        assert_invalid(
            bundle,
            BundleValidationErrorV1::SlotOrder {
                expected: 0,
                actual: 1,
            },
        );

        let mut bundle = canonical_bundle();
        bundle.global_slot_table.entries[1].global_id_bytes = id(1);
        assert_invalid(bundle, BundleValidationErrorV1::DuplicateGlobalId);

        let mut bundle = canonical_bundle();
        bundle.global_slot_table.entries[0].declaration_type = WireTermV1::GlobalSlot { slot: 0 };
        assert_invalid(
            bundle,
            BundleValidationErrorV1::ForwardGlobalReference {
                declaration_slot: 0,
                referenced_slot: 0,
            },
        );

        let mut bundle = canonical_bundle();
        bundle.contexts[1].entries_oldest_first[0] = WireTermV1::Variable { index: 0 };
        assert_invalid(
            bundle,
            BundleValidationErrorV1::VariableOutOfRange {
                index: 0,
                local_count: 0,
            },
        );

        let mut bundle = canonical_bundle();
        bundle.global_slot_table.entries[0].declaration_type = WireTermV1::Sort { level: 2 };
        assert_invalid(
            bundle,
            BundleValidationErrorV1::UniverseLevelOutOfRange {
                role: TermRoleV1::Public,
                level: 2,
            },
        );

        let mut bundle = canonical_bundle();
        bundle.synthesis_codes[0].inferred_type = WireTermV1::Sort { level: 3 };
        assert_invalid(
            bundle,
            BundleValidationErrorV1::UniverseLevelOutOfRange {
                role: TermRoleV1::CheckerProduced,
                level: 3,
            },
        );

        let mut bundle = canonical_bundle();
        bundle.signature.allowed_transparent_deltas[0] = DeltaPolicyEntryWireV1 {
            global_slot: 1,
            global_id_bytes: id(2),
        };
        assert_invalid(bundle, BundleValidationErrorV1::DeltaPolicyEntryMismatch);
    }

    #[test]
    fn conversion_synthesis_and_formation_mutations_are_rejected() {
        let mut bundle = canonical_bundle();
        bundle.conversions[0].left_trace.end = WireTermV1::UnitType;
        assert_invalid(
            bundle,
            BundleValidationErrorV1::ReductionTraceEndpointMismatch,
        );

        let mut bundle = canonical_bundle();
        bundle.conversions[0].no_redex_census.entries.clear();
        assert_invalid(bundle, BundleValidationErrorV1::NoRedexCensusMismatch);

        let mut bundle = canonical_bundle();
        if let SynthesisCodeWireV1::VariableLookup {
            context_ordinal, ..
        } = &mut bundle.synthesis_codes[0].code
        {
            *context_ordinal = 1;
        }
        assert_invalid(bundle, BundleValidationErrorV1::VariableMetadataMismatch);

        let mut bundle = canonical_bundle();
        bundle.synthesis_codes[0].code = SynthesisCodeWireV1::Unit;
        assert_invalid(bundle, BundleValidationErrorV1::SynthesisSubjectMismatch);

        let mut bundle = canonical_bundle();
        bundle
            .conversion_typing_supplements
            .push(ConversionTypingSupplementWireV1 {
                conversion_id: id(5),
                step_path: Vec::new(),
                local_context: ProductionContextWireV1::default(),
                local_endpoint_judgment: EndpointJudgmentWireV1::TypeFormation,
                source_typing_code: unit_synthesis(id(30)),
                target_typing_code: unit_synthesis(id(31)),
                formation_level: Some(4),
            });
        assert_invalid(bundle, BundleValidationErrorV1::FormationLevelOutOfRange(4));
    }

    #[test]
    fn inventory_fresh_and_family_mutations_are_rejected() {
        let mut bundle = canonical_bundle();
        bundle.q0_inventory.ordered_rules.swap(0, 1);
        assert_invalid(bundle, BundleValidationErrorV1::Q0InventoryMismatch);

        let mut bundle = canonical_bundle();
        bundle
            .family_inventory
            .ordered_codes
            .push(FamilyCodeWireV1::Seed);
        assert_invalid(bundle, BundleValidationErrorV1::FamilyInventoryMismatch);

        let mut bundle = canonical_bundle();
        bundle.fresh_rule_schemas[0].owner_slot = 1;
        assert_invalid(bundle, BundleValidationErrorV1::FreshPatternMismatch);

        let mut bundle = canonical_bundle();
        bundle.fresh_rule_schemas[0].right = WireTermV1::GlobalSlot { slot: 0 };
        assert_invalid(bundle, BundleValidationErrorV1::FreshPatternMismatch);

        let mut bundle = canonical_bundle();
        if let FamilyPayloadWireV1::GenericPublicApplication {
            function_family_id, ..
        } = &mut bundle.family_payloads[2]
        {
            *function_family_id = id(99);
        }
        assert_invalid(bundle, BundleValidationErrorV1::MissingFamilyReference);
    }
}
