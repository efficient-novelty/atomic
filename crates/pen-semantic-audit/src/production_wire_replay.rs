//! Independent Rust replay of the canonical production bundle (Phase G).
//!
//! This module consumes canonical bundle BYTES and replays every judgment
//! through the unchanged kernel and the unchanged synthesis protocol V2
//! checker. It trusts no wire field: the slot table must round-trip
//! through `Kernel::verify_signature` and `derive_global_slot_table_wire_v1`
//! to byte-identical wire entries, conversions and synthesis certificates
//! are rebuilt as V2 codes and verified by `verify_base_q0_conversion_code_v2`
//! and `verify_synthesis_code_v2`, binder-local conversion-typing
//! supplements are replayed against DERIVED binder-local contexts with
//! exactly-one premise coverage (the Rust-side discharge of the
//! `NestedCongruenceReplayFrontierV2` contract, mirroring
//! `LawV2/Wire/SupplementReplayV1.agda`), and the Phase F inventory
//! discipline (Q0 classification, typed/bodyless/pairwise-disjoint fresh
//! rules, exact family payload binding) is recomputed with every judgment
//! going through `Kernel::verify_open_judgment`.
//!
//! The replay returns the computed artifacts (normalized forms, inferred
//! types, numeric formation levels, derived supplement contexts, Q0
//! categories, family subject types) from which the canonical transcript
//! is rendered. Formation levels are recovered by the deterministic
//! sort probe: a term's level is the unique `k` in the formation-witness
//! set for which the unchanged kernel accepts `ctx |- term : Sort k`;
//! only `TypeMismatch` advances the probe, every other kernel error
//! fails the replay closed.
//!
//! Nothing here mints correspondence authority by itself: the
//! `VerifiedRustProductionReplayV1` capability is minted in
//! `production_refinement_wire_authority` from this replay's evidence
//! plus the rendered transcript.

use crate::manifest::{AuditDecision, Q0RuleV1};
use crate::production_inventory_bridge::{ProductionQ0CategoryV1, production_q0_category_v1};
use crate::production_refinement::verify_global_slot_table_v1;
use crate::production_wire_slots::{
    ProductionWireSlotFailureV1, derive_global_slot_table_wire_v1, term_to_wire_v1,
};
use pen_kernel::{
    Declaration, DependentContext, Digest, GlobalId, Kernel, KernelError, KernelLimits,
    OpenJudgment, Term, UncheckedSignature, VerifiedSignature,
};
use pen_kernel_synthesis::{
    BaseQ0ConversionCodeV2, BaseQ0ConversionPolicyV2, BaseQ0EndpointJudgmentV2,
    BaseQ0ReductionStepCodeV2, BaseQ0ReductionTraceCodeV2, ConversionPathComponentV2,
    DeltaPolicyEntryV2, NoRedexCensusCodeV2, NoRedexDispositionV2, NoRedexEntryV2, SynthesisCodeV2,
    SynthesisV2Error, VerifiedBaseQ0ConversionPolicyV2, VerifiedSynthesisCodeV2,
    verify_base_q0_conversion_code_v2, verify_base_q0_conversion_policy_v2,
    verify_synthesis_code_v2,
};
use pen_production_wire::{
    BaseQ0ReductionStepWireV1, BaseQ0ReductionTraceWireV1, ConversionCertificateWireV1,
    ConversionPathComponentWireV1, EndpointJudgmentWireV1, FORMATION_WITNESS_LEVELS_V1,
    FamilyPayloadWireV1, NoRedexCensusWireV1, NoRedexDispositionWireV1, ProductionContextWireV1,
    ProductionRefinementBundleV1, Q0RuleWireV1, SeedSourceWireV1, SynthesisCertificateWireV1,
    SynthesisCodeWireV1, WireIdV1, WireTermV1, decode_bundle_v1,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductionReplayFailureV1 {
    Decode,
    Kernel(KernelError),
    Synthesis(SynthesisV2Error),
    SlotTranslation(ProductionWireSlotFailureV1),
    GlobalIdNotCanonical,
    SlotOutOfRange,
    SignatureNotSelfNormal,
    SlotTableRoundTripMismatch,
    SlotTableUnavailable,
    DeltaPolicyIncomplete,
    FormationLevelUndetermined,
    ConversionNotFound,
    SubjectMismatch,
    InferredTypeMismatch,
    SupplementPathUnresolvable,
    SupplementContextMismatch,
    SupplementSubjectMismatch,
    SupplementEndpointMismatch,
    SupplementCoverageMismatch,
    Q0ClassificationUnavailable,
    FreshOwnerNotBodyless,
    FreshConstructorNotBodyless,
    FreshOwnerConstructorEqual,
    FreshArityTooSmall,
    FreshPairNotDisjoint,
    SeedBindingMismatch,
    FamilyContextMismatch,
    FamilySubjectMismatch,
    FamilyTypeMismatch,
    FamilyReferenceMissing,
}

impl std::fmt::Display for ProductionReplayFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode => formatter.write_str("the bundle bytes do not decode canonically"),
            Self::Kernel(error) => write!(formatter, "kernel replay rejected the bundle: {error}"),
            Self::Synthesis(error) => {
                write!(formatter, "synthesis replay rejected the bundle: {error}")
            }
            Self::SlotTranslation(error) => {
                write!(formatter, "slot translation failed during replay: {error}")
            }
            Self::GlobalIdNotCanonical => {
                formatter.write_str("a wire global identity is not a canonical digest")
            }
            Self::SlotOutOfRange => formatter.write_str("a wire slot exceeds the slot table"),
            Self::SignatureNotSelfNormal => formatter.write_str(
                "the wire slot table is not the kernel-normalized verified signature",
            ),
            Self::SlotTableRoundTripMismatch => formatter.write_str(
                "the kernel-derived wire slot table differs from the decoded section",
            ),
            Self::SlotTableUnavailable => {
                formatter.write_str("the verified slot table could not be minted during replay")
            }
            Self::DeltaPolicyIncomplete => formatter.write_str(
                "the delta policy does not enumerate exactly the bodyful declarations",
            ),
            Self::FormationLevelUndetermined => formatter.write_str(
                "no formation-witness level was accepted by the kernel sort probe",
            ),
            Self::ConversionNotFound => {
                formatter.write_str("a synthesis or supplement conversion identifier is unknown")
            }
            Self::SubjectMismatch => {
                formatter.write_str("a synthesis subject differs from the replayed term")
            }
            Self::InferredTypeMismatch => {
                formatter.write_str("a synthesis inferred type differs from the replayed type")
            }
            Self::SupplementPathUnresolvable => {
                formatter.write_str("a supplement step path does not address a congruence premise")
            }
            Self::SupplementContextMismatch => formatter.write_str(
                "a supplement local context differs from the derived binder-local context",
            ),
            Self::SupplementSubjectMismatch => formatter.write_str(
                "a supplement certificate subject differs from the premise endpoint",
            ),
            Self::SupplementEndpointMismatch => formatter.write_str(
                "a supplement endpoint or formation level breaks the recovery discipline",
            ),
            Self::SupplementCoverageMismatch => formatter.write_str(
                "the congruence premises are not covered by exactly one supplement each",
            ),
            Self::Q0ClassificationUnavailable => {
                formatter.write_str("a Q0 rule has no V3 production category")
            }
            Self::FreshOwnerNotBodyless => {
                formatter.write_str("a fresh-rule owner declaration carries a body")
            }
            Self::FreshConstructorNotBodyless => {
                formatter.write_str("a fresh-rule constructor declaration carries a body")
            }
            Self::FreshOwnerConstructorEqual => {
                formatter.write_str("a fresh rule uses one slot as both owner and constructor")
            }
            Self::FreshArityTooSmall => {
                formatter.write_str("a fresh rule has fewer than two telescope entries")
            }
            Self::FreshPairNotDisjoint => {
                formatter.write_str("two fresh rules share an owner/constructor pair")
            }
            Self::SeedBindingMismatch => formatter.write_str(
                "a seed judgment does not bind its public head at the exact declared type",
            ),
            Self::FamilyContextMismatch => formatter.write_str(
                "a family payload is judged in a different context than its components",
            ),
            Self::FamilySubjectMismatch => formatter.write_str(
                "an application family subject is not the application of its components",
            ),
            Self::FamilyTypeMismatch => {
                formatter.write_str("an equation action does not preserve its source family type")
            }
            Self::FamilyReferenceMissing => {
                formatter.write_str("a family payload reference does not resolve")
            }
        }
    }
}

impl std::error::Error for ProductionReplayFailureV1 {}

impl From<KernelError> for ProductionReplayFailureV1 {
    fn from(error: KernelError) -> Self {
        Self::Kernel(error)
    }
}

impl From<SynthesisV2Error> for ProductionReplayFailureV1 {
    fn from(error: SynthesisV2Error) -> Self {
        Self::Synthesis(error)
    }
}

impl From<ProductionWireSlotFailureV1> for ProductionReplayFailureV1 {
    fn from(error: ProductionWireSlotFailureV1) -> Self {
        Self::SlotTranslation(error)
    }
}

/// Computed artifacts of one successful replay, in bundle order. Every
/// term is rendered back onto the wire term surface through the verified
/// slot table, so the transcript renderer never re-derives a mapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionReplayComputedV1 {
    pub declaration_formation_levels: Vec<u16>,
    pub context_entry_formation_levels: Vec<Vec<u16>>,
    pub conversion_normalized_left: Vec<WireTermV1>,
    pub conversion_normalized_right: Vec<WireTermV1>,
    pub conversion_formation_levels: Vec<Option<u16>>,
    pub synthesis_types: Vec<WireTermV1>,
    pub synthesis_dependent_results: Vec<Vec<WireTermV1>>,
    pub supplement_derived_contexts: Vec<Vec<WireTermV1>>,
    pub supplement_premise_sources: Vec<WireTermV1>,
    pub supplement_premise_targets: Vec<WireTermV1>,
    pub supplement_certificate_types: Vec<(WireTermV1, WireTermV1)>,
    pub q0_categories: Vec<u8>,
    pub fresh_type_formation_levels: Vec<u16>,
    pub family_normalized_types: Vec<WireTermV1>,
}

/// Evidence of one successful independent replay: the exact bytes, the
/// protocol identities actually used, and the computed artifacts.
#[derive(Clone, Debug)]
pub struct ProductionReplayEvidenceV1 {
    pub replayed_bytes: Vec<u8>,
    pub kernel_protocol_digest: Digest,
    pub synthesis_protocol_digest: Digest,
    pub computed: ProductionReplayComputedV1,
}

fn wire_id_to_digest(id: &WireIdV1) -> Result<Digest, ProductionReplayFailureV1> {
    let mut hex = String::with_capacity(Digest::PREFIX.len() + 64);
    hex.push_str(Digest::PREFIX);
    for byte in id.0 {
        hex.push(char::from_digit(u32::from(byte >> 4), 16).expect("nibble"));
        hex.push(char::from_digit(u32::from(byte & 0xf), 16).expect("nibble"));
    }
    Digest::parse(hex).map_err(|_| ProductionReplayFailureV1::GlobalIdNotCanonical)
}

fn wire_term_to_kernel(
    term: &WireTermV1,
    globals: &[GlobalId],
) -> Result<Term, ProductionReplayFailureV1> {
    Ok(match term {
        WireTermV1::Sort { level } => Term::Sort { level: *level },
        WireTermV1::Variable { index } => Term::Var { index: *index },
        WireTermV1::GlobalSlot { slot } => Term::Global {
            id: globals
                .get(*slot as usize)
                .ok_or(ProductionReplayFailureV1::SlotOutOfRange)?
                .clone(),
        },
        WireTermV1::Pi { parameter, body } => Term::Pi {
            parameter: Box::new(wire_term_to_kernel(parameter, globals)?),
            body: Box::new(wire_term_to_kernel(body, globals)?),
        },
        WireTermV1::Lambda { parameter, body } => Term::Lambda {
            parameter_type: Box::new(wire_term_to_kernel(parameter, globals)?),
            body: Box::new(wire_term_to_kernel(body, globals)?),
        },
        WireTermV1::Apply { function, argument } => Term::Apply {
            function: Box::new(wire_term_to_kernel(function, globals)?),
            argument: Box::new(wire_term_to_kernel(argument, globals)?),
        },
        WireTermV1::UnitType => Term::UnitType,
        WireTermV1::Unit => Term::Unit,
    })
}

fn wire_context_to_kernel(
    context: &ProductionContextWireV1,
    globals: &[GlobalId],
) -> Result<DependentContext, ProductionReplayFailureV1> {
    Ok(DependentContext(
        context
            .entries_oldest_first
            .iter()
            .map(|entry| wire_term_to_kernel(entry, globals))
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

fn wire_path_component_to_v2(component: ConversionPathComponentWireV1) -> ConversionPathComponentV2 {
    match component {
        ConversionPathComponentWireV1::PiParameter => ConversionPathComponentV2::PiParameter,
        ConversionPathComponentWireV1::PiBody => ConversionPathComponentV2::PiBody,
        ConversionPathComponentWireV1::LambdaParameter => {
            ConversionPathComponentV2::LambdaParameter
        }
        ConversionPathComponentWireV1::LambdaBody => ConversionPathComponentV2::LambdaBody,
        ConversionPathComponentWireV1::ApplyFunction => ConversionPathComponentV2::ApplyFunction,
        ConversionPathComponentWireV1::ApplyArgument => ConversionPathComponentV2::ApplyArgument,
    }
}

fn wire_disposition_to_v2(disposition: NoRedexDispositionWireV1) -> NoRedexDispositionV2 {
    match disposition {
        NoRedexDispositionWireV1::Sort => NoRedexDispositionV2::Sort,
        NoRedexDispositionWireV1::Variable => NoRedexDispositionV2::Variable,
        NoRedexDispositionWireV1::GlobalNotEnabledByPolicy => {
            NoRedexDispositionV2::GlobalNotEnabledByPolicy
        }
        NoRedexDispositionWireV1::Pi => NoRedexDispositionV2::Pi,
        NoRedexDispositionWireV1::Lambda => NoRedexDispositionV2::Lambda,
        NoRedexDispositionWireV1::NeutralApplication => NoRedexDispositionV2::NeutralApplication,
        NoRedexDispositionWireV1::UnitType => NoRedexDispositionV2::UnitType,
        NoRedexDispositionWireV1::Unit => NoRedexDispositionV2::Unit,
    }
}

fn wire_census_to_v2(
    census: &NoRedexCensusWireV1,
    globals: &[GlobalId],
) -> Result<NoRedexCensusCodeV2, ProductionReplayFailureV1> {
    Ok(NoRedexCensusCodeV2 {
        entries: census
            .entries
            .iter()
            .map(|entry| {
                Ok(NoRedexEntryV2 {
                    path: entry
                        .path
                        .iter()
                        .copied()
                        .map(wire_path_component_to_v2)
                        .collect(),
                    term: wire_term_to_kernel(&entry.term, globals)?,
                    disposition: wire_disposition_to_v2(entry.disposition),
                })
            })
            .collect::<Result<Vec<_>, ProductionReplayFailureV1>>()?,
    })
}

fn wire_step_to_v2(
    step: &BaseQ0ReductionStepWireV1,
    globals: &[GlobalId],
) -> Result<BaseQ0ReductionStepCodeV2, ProductionReplayFailureV1> {
    use BaseQ0ReductionStepCodeV2 as V2;
    use BaseQ0ReductionStepWireV1 as Wire;
    Ok(match step {
        Wire::Beta { source, target } => V2::Beta {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
        },
        Wire::TransparentDelta {
            source,
            target,
            global_slot,
        } => V2::TransparentDelta {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
            global_slot: *global_slot,
        },
        Wire::PiParameterCongruence {
            source,
            target,
            premise,
        } => V2::PiParameterCongruence {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
            premise: Box::new(wire_step_to_v2(premise, globals)?),
        },
        Wire::PiBodyCongruence {
            source,
            target,
            premise,
        } => V2::PiBodyCongruence {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
            premise: Box::new(wire_step_to_v2(premise, globals)?),
        },
        Wire::LambdaParameterCongruence {
            source,
            target,
            premise,
        } => V2::LambdaParameterCongruence {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
            premise: Box::new(wire_step_to_v2(premise, globals)?),
        },
        Wire::LambdaBodyCongruence {
            source,
            target,
            premise,
        } => V2::LambdaBodyCongruence {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
            premise: Box::new(wire_step_to_v2(premise, globals)?),
        },
        Wire::ApplyFunctionCongruence {
            source,
            target,
            premise,
        } => V2::ApplyFunctionCongruence {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
            premise: Box::new(wire_step_to_v2(premise, globals)?),
        },
        Wire::ApplyArgumentCongruence {
            source,
            target,
            premise,
        } => V2::ApplyArgumentCongruence {
            source: wire_term_to_kernel(source, globals)?,
            target: wire_term_to_kernel(target, globals)?,
            premise: Box::new(wire_step_to_v2(premise, globals)?),
        },
    })
}

fn wire_trace_to_v2(
    trace: &BaseQ0ReductionTraceWireV1,
    globals: &[GlobalId],
) -> Result<BaseQ0ReductionTraceCodeV2, ProductionReplayFailureV1> {
    Ok(BaseQ0ReductionTraceCodeV2 {
        start: wire_term_to_kernel(&trace.start, globals)?,
        steps: trace
            .steps
            .iter()
            .map(|step| wire_step_to_v2(step, globals))
            .collect::<Result<Vec<_>, _>>()?,
        end: wire_term_to_kernel(&trace.end, globals)?,
    })
}

fn wire_endpoint_to_v2(
    endpoint: &EndpointJudgmentWireV1,
    globals: &[GlobalId],
) -> Result<BaseQ0EndpointJudgmentV2, ProductionReplayFailureV1> {
    Ok(match endpoint {
        EndpointJudgmentWireV1::HasType { expected_type } => BaseQ0EndpointJudgmentV2::HasType {
            expected_type: wire_term_to_kernel(expected_type, globals)?,
        },
        EndpointJudgmentWireV1::TypeFormation => BaseQ0EndpointJudgmentV2::TypeFormation,
    })
}

fn wire_conversion_to_v2(
    certificate: &ConversionCertificateWireV1,
    globals: &[GlobalId],
) -> Result<BaseQ0ConversionCodeV2, ProductionReplayFailureV1> {
    Ok(BaseQ0ConversionCodeV2 {
        context: wire_context_to_kernel(&certificate.context, globals)?,
        left: wire_term_to_kernel(&certificate.left, globals)?,
        right: wire_term_to_kernel(&certificate.right, globals)?,
        endpoint_judgment: wire_endpoint_to_v2(&certificate.endpoint_judgment, globals)?,
        common_normal_form: wire_term_to_kernel(&certificate.common_normal_form, globals)?,
        left_trace: wire_trace_to_v2(&certificate.left_trace, globals)?,
        right_trace: wire_trace_to_v2(&certificate.right_trace, globals)?,
        no_redex_census: wire_census_to_v2(&certificate.no_redex_census, globals)?,
    })
}

fn find_wire_conversion<'bundle>(
    conversions: &'bundle [ConversionCertificateWireV1],
    id: &WireIdV1,
) -> Result<&'bundle ConversionCertificateWireV1, ProductionReplayFailureV1> {
    conversions
        .iter()
        .find(|certificate| certificate.conversion_id == *id)
        .ok_or(ProductionReplayFailureV1::ConversionNotFound)
}

fn wire_synthesis_code_to_v2(
    code: &SynthesisCodeWireV1,
    conversions: &[ConversionCertificateWireV1],
    globals: &[GlobalId],
) -> Result<SynthesisCodeV2, ProductionReplayFailureV1> {
    Ok(match code {
        SynthesisCodeWireV1::Sort { level } => SynthesisCodeV2::Sort { level: *level },
        SynthesisCodeWireV1::UnitType => SynthesisCodeV2::UnitType,
        SynthesisCodeWireV1::Unit => SynthesisCodeV2::Unit,
        SynthesisCodeWireV1::VariableLookup {
            index,
            context_ordinal,
            shift_distance,
        } => SynthesisCodeV2::VariableLookup {
            index: *index,
            context_ordinal: *context_ordinal,
            shift_distance: *shift_distance,
        },
        SynthesisCodeWireV1::GlobalLookup { global_slot } => SynthesisCodeV2::GlobalLookup {
            id: globals
                .get(*global_slot as usize)
                .ok_or(ProductionReplayFailureV1::SlotOutOfRange)?
                .clone(),
            global_slot: *global_slot,
        },
        SynthesisCodeWireV1::PiFormation { parameter, body } => SynthesisCodeV2::PiFormation {
            parameter: Box::new(wire_synthesis_code_to_v2(parameter, conversions, globals)?),
            body: Box::new(wire_synthesis_code_to_v2(body, conversions, globals)?),
        },
        SynthesisCodeWireV1::LambdaIntroduction {
            parameter_type,
            body,
        } => SynthesisCodeV2::LambdaIntroduction {
            parameter_type: Box::new(wire_synthesis_code_to_v2(
                parameter_type,
                conversions,
                globals,
            )?),
            body: Box::new(wire_synthesis_code_to_v2(body, conversions, globals)?),
        },
        SynthesisCodeWireV1::ApplicationElimination {
            function,
            argument,
            function_conversion_id,
            argument_conversion_id,
            dependent_result_type: _,
        } => SynthesisCodeV2::ApplicationElimination {
            function: Box::new(wire_synthesis_code_to_v2(function, conversions, globals)?),
            argument: Box::new(wire_synthesis_code_to_v2(argument, conversions, globals)?),
            function_conversion: Box::new(wire_conversion_to_v2(
                find_wire_conversion(conversions, function_conversion_id)?,
                globals,
            )?),
            argument_conversion: Box::new(wire_conversion_to_v2(
                find_wire_conversion(conversions, argument_conversion_id)?,
                globals,
            )?),
        },
    })
}

/// The deterministic kernel sort probe: the unique formation-witness
/// level accepted for `context |- term : Sort level`.
fn formation_level(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    term: &Term,
) -> Result<u16, ProductionReplayFailureV1> {
    for level in FORMATION_WITNESS_LEVELS_V1 {
        let judgment = OpenJudgment::HasType {
            context: context.clone(),
            term: term.clone(),
            ty: Term::Sort { level },
        };
        match kernel.verify_open_judgment(signature, &judgment) {
            Ok(_) => return Ok(level),
            Err(KernelError::TypeMismatch) => continue,
            Err(error) => return Err(error.into()),
        }
    }
    Err(ProductionReplayFailureV1::FormationLevelUndetermined)
}

/// Kernel-normalized subject of one endpoint judgment: for `HasType`
/// the normalized term checked at the expected type, for
/// `TypeFormation` the normalized type.
fn normalized_endpoint_term(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    endpoint: &EndpointJudgmentWireV1,
    term: &Term,
    globals: &[GlobalId],
) -> Result<Term, ProductionReplayFailureV1> {
    let judgment = match endpoint {
        EndpointJudgmentWireV1::HasType { expected_type } => OpenJudgment::HasType {
            context: context.clone(),
            term: term.clone(),
            ty: wire_term_to_kernel(expected_type, globals)?,
        },
        EndpointJudgmentWireV1::TypeFormation => OpenJudgment::TypeFormation {
            context: context.clone(),
            term: term.clone(),
        },
    };
    match kernel.verify_open_judgment(signature, &judgment)? {
        OpenJudgment::HasType { term, .. } | OpenJudgment::TypeFormation { term, .. } => Ok(term),
        OpenJudgment::DefinitionallyEqual { .. } => Err(ProductionReplayFailureV1::Kernel(
            KernelError::WrongJudgmentForm,
        )),
    }
}

/// One addressed congruence premise: the premise step plus the derived
/// binder-local context accumulated along the descent.
struct ResolvedPremise<'certificate> {
    premise: &'certificate BaseQ0ReductionStepWireV1,
    derived_context: Vec<WireTermV1>,
}

fn congruence_premise(
    step: &BaseQ0ReductionStepWireV1,
) -> Option<(&BaseQ0ReductionStepWireV1, &WireTermV1, bool)> {
    use BaseQ0ReductionStepWireV1 as Wire;
    match step {
        Wire::PiBodyCongruence {
            source, premise, ..
        }
        | Wire::LambdaBodyCongruence {
            source, premise, ..
        } => Some((premise, source, true)),
        Wire::PiParameterCongruence {
            source, premise, ..
        }
        | Wire::LambdaParameterCongruence {
            source, premise, ..
        }
        | Wire::ApplyFunctionCongruence {
            source, premise, ..
        }
        | Wire::ApplyArgumentCongruence {
            source, premise, ..
        } => Some((premise, source, false)),
        Wire::Beta { .. } | Wire::TransparentDelta { .. } => None,
    }
}

fn binder_parameter(source: &WireTermV1) -> Option<&WireTermV1> {
    match source {
        WireTermV1::Pi { parameter, .. } | WireTermV1::Lambda { parameter, .. } => Some(parameter),
        _ => None,
    }
}

/// Resolve one supplement step path against its conversion, deriving the
/// binder-local context exactly as `SupplementReplayV1.agda`: the trace
/// selector, the step ordinal, then one zero per premise descent, with
/// pi-body and lambda-body frames extending the context by the source
/// binder's parameter.
fn resolve_step_path<'certificate>(
    certificate: &'certificate ConversionCertificateWireV1,
    step_path: &[u32],
) -> Result<ResolvedPremise<'certificate>, ProductionReplayFailureV1> {
    let (selector, rest) = step_path
        .split_first()
        .ok_or(ProductionReplayFailureV1::SupplementPathUnresolvable)?;
    let trace = match selector {
        0 => &certificate.left_trace,
        1 => &certificate.right_trace,
        _ => return Err(ProductionReplayFailureV1::SupplementPathUnresolvable),
    };
    let (ordinal, descents) = rest
        .split_first()
        .ok_or(ProductionReplayFailureV1::SupplementPathUnresolvable)?;
    if descents.is_empty() || descents.iter().any(|descent| *descent != 0) {
        return Err(ProductionReplayFailureV1::SupplementPathUnresolvable);
    }
    let mut current = trace
        .steps
        .get(*ordinal as usize)
        .ok_or(ProductionReplayFailureV1::SupplementPathUnresolvable)?;
    let mut derived_context = certificate.context.entries_oldest_first.clone();
    for _ in descents {
        let (premise, source, extends) = congruence_premise(current)
            .ok_or(ProductionReplayFailureV1::SupplementPathUnresolvable)?;
        if extends {
            let parameter = binder_parameter(source)
                .ok_or(ProductionReplayFailureV1::SupplementPathUnresolvable)?;
            derived_context.push(parameter.clone());
        }
        current = premise;
    }
    Ok(ResolvedPremise {
        premise: current,
        derived_context,
    })
}

/// Every congruence premise position of one conversion, as
/// `(trace selector, step ordinal, descent depth)` triples.
fn congruence_positions(certificate: &ConversionCertificateWireV1) -> Vec<(u32, u32, u32)> {
    let mut positions = Vec::new();
    for (selector, trace) in [(0u32, &certificate.left_trace), (1u32, &certificate.right_trace)] {
        for (ordinal, step) in trace.steps.iter().enumerate() {
            let mut depth = 0u32;
            let mut current = step;
            while let Some((premise, _, _)) = congruence_premise(current) {
                depth += 1;
                positions.push((selector, ordinal as u32, depth));
                current = premise;
            }
        }
    }
    positions
}

fn step_endpoints(step: &BaseQ0ReductionStepWireV1) -> (&WireTermV1, &WireTermV1) {
    use BaseQ0ReductionStepWireV1 as Wire;
    match step {
        Wire::Beta { source, target }
        | Wire::TransparentDelta { source, target, .. }
        | Wire::PiParameterCongruence { source, target, .. }
        | Wire::PiBodyCongruence { source, target, .. }
        | Wire::LambdaParameterCongruence { source, target, .. }
        | Wire::LambdaBodyCongruence { source, target, .. }
        | Wire::ApplyFunctionCongruence { source, target, .. }
        | Wire::ApplyArgumentCongruence { source, target, .. } => (source, target),
    }
}

struct ReplayEnvironment<'bundle> {
    kernel: Kernel,
    signature: VerifiedSignature,
    policy: VerifiedBaseQ0ConversionPolicyV2,
    globals: Vec<GlobalId>,
    bundle: &'bundle ProductionRefinementBundleV1,
}

impl ReplayEnvironment<'_> {
    fn verify_synthesis(
        &self,
        context: &ProductionContextWireV1,
        code: &SynthesisCodeWireV1,
    ) -> Result<VerifiedSynthesisCodeV2, ProductionReplayFailureV1> {
        let kernel_context = wire_context_to_kernel(context, &self.globals)?;
        let v2_code = wire_synthesis_code_to_v2(code, &self.bundle.conversions, &self.globals)?;
        Ok(verify_synthesis_code_v2(
            &self.kernel,
            &self.signature,
            &self.policy,
            &kernel_context,
            &v2_code,
        )?)
    }

    /// Replay one wire synthesis certificate: the reconstructed subject
    /// and the replayed top-node type must equal the recorded fields
    /// exactly.
    fn replay_certificate(
        &self,
        certificate: &SynthesisCertificateWireV1,
    ) -> Result<VerifiedSynthesisCodeV2, ProductionReplayFailureV1> {
        let verified = self.verify_synthesis(&certificate.context, &certificate.code)?;
        let subject = wire_term_to_kernel(&certificate.subject, &self.globals)?;
        if verified.term() != &subject {
            return Err(ProductionReplayFailureV1::SubjectMismatch);
        }
        let inferred = wire_term_to_kernel(&certificate.inferred_type, &self.globals)?;
        if verified.derivation().inferred_type() != &inferred {
            return Err(ProductionReplayFailureV1::InferredTypeMismatch);
        }
        Ok(verified)
    }
}

/// Collect the dependent application result types of one verified
/// synthesis derivation, outermost first.
fn dependent_results(
    node: &pen_kernel_synthesis::VerifiedSynthesisNodeV2,
    results: &mut Vec<Term>,
) {
    if matches!(node.code(), SynthesisCodeV2::ApplicationElimination { .. }) {
        results.push(node.inferred_type().clone());
    }
    for premise in node.premises() {
        dependent_results(premise, results);
    }
}

pub fn replay_production_bundle_v1(
    bytes: &[u8],
) -> Result<ProductionReplayEvidenceV1, ProductionReplayFailureV1> {
    let bundle = decode_bundle_v1(bytes).map_err(|_| ProductionReplayFailureV1::Decode)?;
    let kernel = Kernel::new(KernelLimits::default())?;

    // --- Signature: the slot table must be exactly the normalized
    // verified signature the kernel stores, and the kernel-derived wire
    // table must round-trip byte-identically.
    let globals = bundle
        .global_slot_table
        .entries
        .iter()
        .map(|entry| Ok(GlobalId(wire_id_to_digest(&entry.global_id_bytes)?)))
        .collect::<Result<Vec<GlobalId>, ProductionReplayFailureV1>>()?;
    let declarations = bundle
        .global_slot_table
        .entries
        .iter()
        .zip(&globals)
        .map(|(entry, id)| {
            Ok(Declaration {
                id: id.clone(),
                ty: wire_term_to_kernel(&entry.declaration_type, &globals)?,
                body: entry
                    .declaration_body
                    .as_ref()
                    .map(|body| wire_term_to_kernel(body, &globals))
                    .transpose()?,
            })
        })
        .collect::<Result<Vec<_>, ProductionReplayFailureV1>>()?;
    let signature = kernel.verify_signature(&UncheckedSignature {
        declarations: declarations.clone(),
    })?;
    if signature.declarations() != declarations.as_slice() {
        return Err(ProductionReplayFailureV1::SignatureNotSelfNormal);
    }
    let AuditDecision::Proven(slots) = verify_global_slot_table_v1(&kernel, &signature) else {
        return Err(ProductionReplayFailureV1::SlotTableUnavailable);
    };
    if derive_global_slot_table_wire_v1(&signature, &slots)? != bundle.global_slot_table {
        return Err(ProductionReplayFailureV1::SlotTableRoundTripMismatch);
    }

    // --- Delta policy: verified against the replayed signature and
    // complete over exactly the bodyful declarations.
    let policy_wire = BaseQ0ConversionPolicyV2 {
        allowed_transparent_deltas: bundle
            .signature
            .allowed_transparent_deltas
            .iter()
            .map(|entry| {
                Ok(DeltaPolicyEntryV2 {
                    global_slot: entry.global_slot,
                    id: GlobalId(wire_id_to_digest(&entry.global_id_bytes)?),
                })
            })
            .collect::<Result<Vec<_>, ProductionReplayFailureV1>>()?,
    };
    let policy = verify_base_q0_conversion_policy_v2(&signature, &policy_wire)?;
    let bodyful = declarations
        .iter()
        .enumerate()
        .filter(|(_, declaration)| declaration.body.is_some())
        .map(|(slot, _)| slot as u32)
        .collect::<Vec<_>>();
    if policy_wire
        .allowed_transparent_deltas
        .iter()
        .map(|entry| entry.global_slot)
        .collect::<Vec<_>>()
        != bodyful
    {
        return Err(ProductionReplayFailureV1::DeltaPolicyIncomplete);
    }

    let environment = ReplayEnvironment {
        kernel,
        signature,
        policy,
        globals,
        bundle: &bundle,
    };
    let ReplayEnvironment {
        kernel,
        signature,
        globals,
        ..
    } = &environment;

    // --- Declared-type formation levels.
    let empty = DependentContext(Vec::new());
    let declaration_formation_levels = declarations
        .iter()
        .map(|declaration| formation_level(kernel, signature, &empty, &declaration.ty))
        .collect::<Result<Vec<_>, _>>()?;

    // --- Standalone contexts: kernel verification plus per-entry
    // formation levels under the strict prefix.
    let mut context_entry_formation_levels = Vec::new();
    for context in &bundle.contexts {
        let kernel_context = wire_context_to_kernel(context, globals)?;
        environment
            .kernel
            .verify_context(signature, &kernel_context)?;
        let mut levels = Vec::new();
        for (position, entry) in kernel_context.0.iter().enumerate() {
            let prefix = DependentContext(kernel_context.0[..position].to_vec());
            levels.push(formation_level(kernel, signature, &prefix, entry)?);
        }
        context_entry_formation_levels.push(levels);
    }

    // --- Conversions through the unchanged V2 checker, plus computed
    // kernel-normalized endpoints and TypeFormation levels.
    let mut conversion_normalized_left = Vec::new();
    let mut conversion_normalized_right = Vec::new();
    let mut conversion_formation_levels = Vec::new();
    for certificate in &bundle.conversions {
        let code = wire_conversion_to_v2(certificate, globals)?;
        verify_base_q0_conversion_code_v2(kernel, signature, &environment.policy, &code)?;
        let kernel_context = wire_context_to_kernel(&certificate.context, globals)?;
        let left = wire_term_to_kernel(&certificate.left, globals)?;
        let right = wire_term_to_kernel(&certificate.right, globals)?;
        let normalized_left = normalized_endpoint_term(
            kernel,
            signature,
            &kernel_context,
            &certificate.endpoint_judgment,
            &left,
            globals,
        )?;
        let normalized_right = normalized_endpoint_term(
            kernel,
            signature,
            &kernel_context,
            &certificate.endpoint_judgment,
            &right,
            globals,
        )?;
        conversion_normalized_left.push(term_to_wire_v1(&normalized_left, &slots)?);
        conversion_normalized_right.push(term_to_wire_v1(&normalized_right, &slots)?);
        conversion_formation_levels.push(match &certificate.endpoint_judgment {
            EndpointJudgmentWireV1::TypeFormation => {
                let common = wire_term_to_kernel(&certificate.common_normal_form, globals)?;
                Some(formation_level(kernel, signature, &kernel_context, &common)?)
            }
            EndpointJudgmentWireV1::HasType { .. } => None,
        });
    }

    // --- Synthesis certificates through the unchanged V2 checker.
    let mut synthesis_types = Vec::new();
    let mut synthesis_dependent_results = Vec::new();
    for certificate in &bundle.synthesis_codes {
        let verified = environment.replay_certificate(certificate)?;
        synthesis_types.push(term_to_wire_v1(verified.derivation().inferred_type(), &slots)?);
        let mut results = Vec::new();
        dependent_results(verified.derivation(), &mut results);
        synthesis_dependent_results.push(
            results
                .iter()
                .map(|result| term_to_wire_v1(result, &slots))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    // --- Binder-local conversion-typing supplements: the Rust-side
    // discharge of the nested-congruence replay contract.
    let mut supplement_derived_contexts = Vec::new();
    let mut supplement_premise_sources = Vec::new();
    let mut supplement_premise_targets = Vec::new();
    let mut supplement_certificate_types = Vec::new();
    let mut covered_positions = Vec::new();
    for supplement in &bundle.conversion_typing_supplements {
        let certificate =
            find_wire_conversion(&bundle.conversions, &supplement.conversion_id)?;
        let resolved = resolve_step_path(certificate, &supplement.step_path)?;
        if resolved.derived_context != supplement.local_context.entries_oldest_first {
            return Err(ProductionReplayFailureV1::SupplementContextMismatch);
        }
        let (source, target) = step_endpoints(resolved.premise);
        if supplement.source_typing_code.subject != *source
            || supplement.target_typing_code.subject != *target
        {
            return Err(ProductionReplayFailureV1::SupplementSubjectMismatch);
        }
        match (&supplement.local_endpoint_judgment, supplement.formation_level) {
            (EndpointJudgmentWireV1::HasType { expected_type }, None) => {
                if supplement.source_typing_code.inferred_type != *expected_type
                    || supplement.target_typing_code.inferred_type != *expected_type
                {
                    return Err(ProductionReplayFailureV1::SupplementEndpointMismatch);
                }
            }
            (EndpointJudgmentWireV1::TypeFormation, Some(level)) => {
                let sort = WireTermV1::Sort { level };
                if supplement.source_typing_code.inferred_type != sort
                    || supplement.target_typing_code.inferred_type != sort
                {
                    return Err(ProductionReplayFailureV1::SupplementEndpointMismatch);
                }
            }
            _ => return Err(ProductionReplayFailureV1::SupplementEndpointMismatch),
        }
        let source_verified = environment.replay_certificate(&supplement.source_typing_code)?;
        let target_verified = environment.replay_certificate(&supplement.target_typing_code)?;
        supplement_certificate_types.push((
            term_to_wire_v1(source_verified.derivation().inferred_type(), &slots)?,
            term_to_wire_v1(target_verified.derivation().inferred_type(), &slots)?,
        ));
        supplement_derived_contexts.push(resolved.derived_context);
        supplement_premise_sources.push(source.clone());
        supplement_premise_targets.push(target.clone());
        let descent_depth = (supplement.step_path.len() - 2) as u32;
        covered_positions.push((
            supplement.conversion_id,
            supplement.step_path[0],
            supplement.step_path[1],
            descent_depth,
        ));
    }
    let mut required_positions = Vec::new();
    for certificate in &bundle.conversions {
        for (selector, ordinal, depth) in congruence_positions(certificate) {
            required_positions.push((certificate.conversion_id, selector, ordinal, depth));
        }
    }
    let mut sorted_required = required_positions.clone();
    sorted_required.sort();
    let mut sorted_covered = covered_positions.clone();
    sorted_covered.sort();
    if sorted_required != sorted_covered {
        return Err(ProductionReplayFailureV1::SupplementCoverageMismatch);
    }

    // --- Q0 classification through the inventory bridge.
    let q0_categories = bundle
        .q0_inventory
        .ordered_rules
        .iter()
        .map(|rule| {
            let semantic = match rule {
                Q0RuleWireV1::DeBruijn => Q0RuleV1::DeBruijn,
                Q0RuleWireV1::SequentialSubstitution => Q0RuleV1::SequentialSubstitution,
                Q0RuleWireV1::Beta => Q0RuleV1::Beta,
                Q0RuleWireV1::ProvenancePreservingDelta => Q0RuleV1::ProvenancePreservingDelta,
                Q0RuleWireV1::Unit => Q0RuleV1::Unit,
                Q0RuleWireV1::TelescopeFlattening => Q0RuleV1::TelescopeFlattening,
                Q0RuleWireV1::FreshNonrecursiveConstructorComputation => {
                    Q0RuleV1::FreshNonrecursiveConstructorComputation
                }
            };
            match production_q0_category_v1(semantic) {
                Some(ProductionQ0CategoryV1::Representation) => Ok(0u8),
                Some(ProductionQ0CategoryV1::BaseSemantic) => Ok(1u8),
                Some(ProductionQ0CategoryV1::RuntimePublic) => Ok(2u8),
                None => Err(ProductionReplayFailureV1::Q0ClassificationUnavailable),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    // --- Fresh rules: the Phase F discipline with every judgment through
    // the kernel.
    let mut fresh_type_formation_levels = Vec::new();
    let mut seen_pairs = Vec::new();
    for schema in &bundle.fresh_rule_schemas {
        if schema.owner_slot == schema.constructor_slot {
            return Err(ProductionReplayFailureV1::FreshOwnerConstructorEqual);
        }
        if schema.arity < 2 {
            return Err(ProductionReplayFailureV1::FreshArityTooSmall);
        }
        let owner = declarations
            .get(schema.owner_slot as usize)
            .ok_or(ProductionReplayFailureV1::SlotOutOfRange)?;
        if owner.body.is_some() {
            return Err(ProductionReplayFailureV1::FreshOwnerNotBodyless);
        }
        let constructor = declarations
            .get(schema.constructor_slot as usize)
            .ok_or(ProductionReplayFailureV1::SlotOutOfRange)?;
        if constructor.body.is_some() {
            return Err(ProductionReplayFailureV1::FreshConstructorNotBodyless);
        }
        if seen_pairs.contains(&(schema.owner_slot, schema.constructor_slot)) {
            return Err(ProductionReplayFailureV1::FreshPairNotDisjoint);
        }
        seen_pairs.push((schema.owner_slot, schema.constructor_slot));
        let parameter_context = wire_context_to_kernel(&schema.parameter_context, globals)?;
        environment
            .kernel
            .verify_context(signature, &parameter_context)?;
        let ty = wire_term_to_kernel(&schema.ty, globals)?;
        fresh_type_formation_levels.push(formation_level(
            kernel,
            signature,
            &parameter_context,
            &ty,
        )?);
        for side in [&schema.left, &schema.right] {
            let term = wire_term_to_kernel(side, globals)?;
            kernel.verify_open_judgment(
                signature,
                &OpenJudgment::HasType {
                    context: parameter_context.clone(),
                    term,
                    ty: ty.clone(),
                },
            )?;
        }
    }

    // --- Family payloads: exact binding plus kernel judgment replay.
    let mut family_normalized_types = Vec::new();
    let mut family_judgments: Vec<(WireIdV1, &FamilyPayloadWireV1)> = Vec::new();
    for payload in &bundle.family_payloads {
        let judgment = match payload {
            FamilyPayloadWireV1::Seed {
                source, judgment, ..
            } => {
                let owner_slot = match source {
                    SeedSourceWireV1::PublicHead { owner_slot } => *owner_slot,
                    SeedSourceWireV1::PublicEquation { equation_id } => {
                        bundle
                            .fresh_rule_schemas
                            .iter()
                            .find(|schema| schema.equation_id == *equation_id)
                            .ok_or(ProductionReplayFailureV1::FamilyReferenceMissing)?
                            .owner_slot
                    }
                };
                let entry = bundle
                    .global_slot_table
                    .entries
                    .get(owner_slot as usize)
                    .ok_or(ProductionReplayFailureV1::SlotOutOfRange)?;
                if judgment.subject != (WireTermV1::GlobalSlot { slot: owner_slot })
                    || judgment.ty != entry.declaration_type
                {
                    return Err(ProductionReplayFailureV1::SeedBindingMismatch);
                }
                judgment
            }
            FamilyPayloadWireV1::GenericPublicApplication {
                function_family_id,
                argument_family_id,
                judgment,
                ..
            } => {
                let function = family_judgments
                    .iter()
                    .find(|(id, _)| id == function_family_id)
                    .ok_or(ProductionReplayFailureV1::FamilyReferenceMissing)?
                    .1;
                let argument = family_judgments
                    .iter()
                    .find(|(id, _)| id == argument_family_id)
                    .ok_or(ProductionReplayFailureV1::FamilyReferenceMissing)?
                    .1;
                let function_judgment = family_payload_judgment(function);
                let argument_judgment = family_payload_judgment(argument);
                if judgment.context != function_judgment.context
                    || judgment.context != argument_judgment.context
                {
                    return Err(ProductionReplayFailureV1::FamilyContextMismatch);
                }
                let expected = WireTermV1::Apply {
                    function: Box::new(function_judgment.subject.clone()),
                    argument: Box::new(argument_judgment.subject.clone()),
                };
                if judgment.subject != expected {
                    return Err(ProductionReplayFailureV1::FamilySubjectMismatch);
                }
                judgment
            }
            FamilyPayloadWireV1::GenericEquationAction {
                equation_id,
                source_family_id,
                judgment,
                ..
            } => {
                if !bundle
                    .fresh_rule_schemas
                    .iter()
                    .any(|schema| schema.equation_id == *equation_id)
                {
                    return Err(ProductionReplayFailureV1::FamilyReferenceMissing);
                }
                let source = family_judgments
                    .iter()
                    .find(|(id, _)| id == source_family_id)
                    .ok_or(ProductionReplayFailureV1::FamilyReferenceMissing)?
                    .1;
                let source_judgment = family_payload_judgment(source);
                if judgment.context != source_judgment.context {
                    return Err(ProductionReplayFailureV1::FamilyContextMismatch);
                }
                if judgment.ty != source_judgment.ty {
                    return Err(ProductionReplayFailureV1::FamilyTypeMismatch);
                }
                judgment
            }
        };
        let kernel_context = wire_context_to_kernel(&judgment.context, globals)?;
        let subject = wire_term_to_kernel(&judgment.subject, globals)?;
        let ty = wire_term_to_kernel(&judgment.ty, globals)?;
        let output = kernel.verify_open_judgment(
            signature,
            &OpenJudgment::HasType {
                context: kernel_context,
                term: subject,
                ty,
            },
        )?;
        let OpenJudgment::HasType { ty: normalized, .. } = output else {
            return Err(ProductionReplayFailureV1::Kernel(
                KernelError::WrongJudgmentForm,
            ));
        };
        family_normalized_types.push(term_to_wire_v1(&normalized, &slots)?);
        family_judgments.push((family_payload_id(payload), payload));
    }

    Ok(ProductionReplayEvidenceV1 {
        replayed_bytes: bytes.to_vec(),
        kernel_protocol_digest: environment.kernel.kernel_protocol_digest(),
        synthesis_protocol_digest: pen_kernel_synthesis::synthesis_protocol_digest_v2(),
        computed: ProductionReplayComputedV1 {
            declaration_formation_levels,
            context_entry_formation_levels,
            conversion_normalized_left,
            conversion_normalized_right,
            conversion_formation_levels,
            synthesis_types,
            synthesis_dependent_results,
            supplement_derived_contexts,
            supplement_premise_sources,
            supplement_premise_targets,
            supplement_certificate_types,
            q0_categories,
            fresh_type_formation_levels,
            family_normalized_types,
        },
    })
}

fn family_payload_id(payload: &FamilyPayloadWireV1) -> WireIdV1 {
    payload.family_id()
}

fn family_payload_judgment(
    payload: &FamilyPayloadWireV1,
) -> &pen_production_wire::FamilyJudgmentWireV1 {
    match payload {
        FamilyPayloadWireV1::Seed { judgment, .. }
        | FamilyPayloadWireV1::GenericPublicApplication { judgment, .. }
        | FamilyPayloadWireV1::GenericEquationAction { judgment, .. } => judgment,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_id_digest_round_trip() {
        let id = WireIdV1([0xab; 32]);
        let digest = wire_id_to_digest(&id).expect("canonical digest");
        assert_eq!(
            crate::production_wire_slots::digest_wire_id(&digest).expect("wire id"),
            id
        );
    }

    #[test]
    fn formation_level_probe_is_deterministic() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("signature");
        let empty = DependentContext(Vec::new());
        assert_eq!(
            formation_level(&kernel, &signature, &empty, &Term::UnitType).expect("level"),
            0
        );
        assert_eq!(
            formation_level(&kernel, &signature, &empty, &Term::Sort { level: 0 }).expect("level"),
            1
        );
        assert_eq!(
            formation_level(&kernel, &signature, &empty, &Term::Unit),
            Err(ProductionReplayFailureV1::FormationLevelUndetermined)
        );
    }
}
