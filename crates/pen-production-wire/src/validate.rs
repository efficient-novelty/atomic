use crate::error::{BundleValidationErrorV1, TermRoleV1};
use crate::model::*;
use std::collections::{BTreeMap, BTreeSet};

const MAX_CONTEXT_ENTRIES_V1: u16 = 32;
const MAX_VALIDATION_ITEMS_V1: usize = 1_000_000;
const MAX_TERM_DEPTH_V1: u16 = 256;

fn limit_len(len: usize, kind: &'static str) -> Result<(), BundleValidationErrorV1> {
    if len > MAX_VALIDATION_ITEMS_V1 {
        Err(BundleValidationErrorV1::ResourceLimit { kind })
    } else {
        Ok(())
    }
}

fn check_context(
    context: &ProductionContextWireV1,
    global_count: u32,
    role: TermRoleV1,
) -> Result<(), BundleValidationErrorV1> {
    if context.entries_oldest_first.len() > usize::from(MAX_CONTEXT_ENTRIES_V1) {
        return Err(BundleValidationErrorV1::ResourceLimit { kind: "context" });
    }
    for (ordinal, term) in context.entries_oldest_first.iter().enumerate() {
        check_term(term, ordinal as u32, global_count, role, 0)?;
    }
    Ok(())
}

fn check_term(
    term: &WireTermV1,
    local_count: u32,
    global_count: u32,
    role: TermRoleV1,
    depth: u16,
) -> Result<(), BundleValidationErrorV1> {
    if depth > MAX_TERM_DEPTH_V1 {
        return Err(BundleValidationErrorV1::TermDepthExceeded);
    }
    match term {
        WireTermV1::Sort { level } => {
            let allowed = match role {
                TermRoleV1::Public => PUBLIC_UNIVERSE_LEVELS_V1.contains(level),
                TermRoleV1::CheckerProduced => CHECKER_UNIVERSE_LEVELS_V1.contains(level),
            };
            if !allowed {
                return Err(BundleValidationErrorV1::UniverseLevelOutOfRange {
                    role,
                    level: *level,
                });
            }
        }
        WireTermV1::Variable { index } => {
            if *index >= local_count {
                return Err(BundleValidationErrorV1::VariableOutOfRange {
                    index: *index,
                    local_count,
                });
            }
        }
        WireTermV1::GlobalSlot { slot } => {
            if *slot >= global_count {
                return Err(BundleValidationErrorV1::GlobalSlotOutOfRange {
                    slot: *slot,
                    limit: global_count,
                });
            }
        }
        WireTermV1::Pi { parameter, body } | WireTermV1::Lambda { parameter, body } => {
            check_term(parameter, local_count, global_count, role, depth + 1)?;
            check_term(body, local_count + 1, global_count, role, depth + 1)?;
        }
        WireTermV1::Apply { function, argument } => {
            check_term(function, local_count, global_count, role, depth + 1)?;
            check_term(argument, local_count, global_count, role, depth + 1)?;
        }
        WireTermV1::UnitType | WireTermV1::Unit => {}
    }
    Ok(())
}

fn collect_global_slots(term: &WireTermV1, slots: &mut Vec<u32>) {
    match term {
        WireTermV1::GlobalSlot { slot } => slots.push(*slot),
        WireTermV1::Pi { parameter, body } | WireTermV1::Lambda { parameter, body } => {
            collect_global_slots(parameter, slots);
            collect_global_slots(body, slots);
        }
        WireTermV1::Apply { function, argument } => {
            collect_global_slots(function, slots);
            collect_global_slots(argument, slots);
        }
        WireTermV1::Sort { .. }
        | WireTermV1::Variable { .. }
        | WireTermV1::UnitType
        | WireTermV1::Unit => {}
    }
}

fn check_trace(
    trace: &BaseQ0ReductionTraceWireV1,
    local_count: u32,
    global_count: u32,
) -> Result<(), BundleValidationErrorV1> {
    check_term(
        &trace.start,
        local_count,
        global_count,
        TermRoleV1::CheckerProduced,
        0,
    )?;
    check_term(
        &trace.end,
        local_count,
        global_count,
        TermRoleV1::CheckerProduced,
        0,
    )?;
    let mut expected = &trace.start;
    for step in &trace.steps {
        if step.source() != expected {
            return Err(BundleValidationErrorV1::ReductionTraceChainMismatch);
        }
        check_reduction_step(step, local_count, global_count, 0)?;
        expected = step.target();
    }
    if expected != &trace.end {
        return Err(BundleValidationErrorV1::ReductionTraceEndpointMismatch);
    }
    Ok(())
}

fn check_reduction_step(
    step: &BaseQ0ReductionStepWireV1,
    local_count: u32,
    global_count: u32,
    depth: u16,
) -> Result<(), BundleValidationErrorV1> {
    if depth > MAX_TERM_DEPTH_V1 {
        return Err(BundleValidationErrorV1::TermDepthExceeded);
    }
    check_term(
        step.source(),
        local_count,
        global_count,
        TermRoleV1::CheckerProduced,
        0,
    )?;
    check_term(
        step.target(),
        local_count,
        global_count,
        TermRoleV1::CheckerProduced,
        0,
    )?;
    match step {
        BaseQ0ReductionStepWireV1::TransparentDelta { global_slot, .. } => {
            if *global_slot >= global_count {
                return Err(BundleValidationErrorV1::GlobalSlotOutOfRange {
                    slot: *global_slot,
                    limit: global_count,
                });
            }
        }
        BaseQ0ReductionStepWireV1::PiBodyCongruence { premise, .. }
        | BaseQ0ReductionStepWireV1::LambdaBodyCongruence { premise, .. } => {
            check_reduction_step(premise, local_count + 1, global_count, depth + 1)?;
        }
        BaseQ0ReductionStepWireV1::PiParameterCongruence { premise, .. }
        | BaseQ0ReductionStepWireV1::LambdaParameterCongruence { premise, .. }
        | BaseQ0ReductionStepWireV1::ApplyFunctionCongruence { premise, .. }
        | BaseQ0ReductionStepWireV1::ApplyArgumentCongruence { premise, .. } => {
            check_reduction_step(premise, local_count, global_count, depth + 1)?;
        }
        BaseQ0ReductionStepWireV1::Beta { .. } => {}
    }
    Ok(())
}

fn append_census(
    term: &WireTermV1,
    path: &mut Vec<ConversionPathComponentWireV1>,
    enabled_deltas: &BTreeSet<u32>,
    entries: &mut Vec<NoRedexEntryWireV1>,
    depth: u16,
) -> Result<(), BundleValidationErrorV1> {
    if depth > MAX_TERM_DEPTH_V1 {
        return Err(BundleValidationErrorV1::TermDepthExceeded);
    }
    let disposition = match term {
        WireTermV1::Sort { .. } => NoRedexDispositionWireV1::Sort,
        WireTermV1::Variable { .. } => NoRedexDispositionWireV1::Variable,
        WireTermV1::GlobalSlot { slot } => {
            if enabled_deltas.contains(slot) {
                return Err(BundleValidationErrorV1::NormalFormHasRedex);
            }
            NoRedexDispositionWireV1::GlobalNotEnabledByPolicy
        }
        WireTermV1::Pi { .. } => NoRedexDispositionWireV1::Pi,
        WireTermV1::Lambda { .. } => NoRedexDispositionWireV1::Lambda,
        WireTermV1::Apply { function, .. } => {
            if matches!(function.as_ref(), WireTermV1::Lambda { .. }) {
                return Err(BundleValidationErrorV1::NormalFormHasRedex);
            }
            NoRedexDispositionWireV1::NeutralApplication
        }
        WireTermV1::UnitType => NoRedexDispositionWireV1::UnitType,
        WireTermV1::Unit => NoRedexDispositionWireV1::Unit,
    };
    entries.push(NoRedexEntryWireV1 {
        path: path.clone(),
        term: term.clone(),
        disposition,
    });
    match term {
        WireTermV1::Pi { parameter, body } => {
            path.push(ConversionPathComponentWireV1::PiParameter);
            append_census(parameter, path, enabled_deltas, entries, depth + 1)?;
            path.pop();
            path.push(ConversionPathComponentWireV1::PiBody);
            append_census(body, path, enabled_deltas, entries, depth + 1)?;
            path.pop();
        }
        WireTermV1::Lambda { parameter, body } => {
            path.push(ConversionPathComponentWireV1::LambdaParameter);
            append_census(parameter, path, enabled_deltas, entries, depth + 1)?;
            path.pop();
            path.push(ConversionPathComponentWireV1::LambdaBody);
            append_census(body, path, enabled_deltas, entries, depth + 1)?;
            path.pop();
        }
        WireTermV1::Apply { function, argument } => {
            path.push(ConversionPathComponentWireV1::ApplyFunction);
            append_census(function, path, enabled_deltas, entries, depth + 1)?;
            path.pop();
            path.push(ConversionPathComponentWireV1::ApplyArgument);
            append_census(argument, path, enabled_deltas, entries, depth + 1)?;
            path.pop();
        }
        WireTermV1::Sort { .. }
        | WireTermV1::Variable { .. }
        | WireTermV1::GlobalSlot { .. }
        | WireTermV1::UnitType
        | WireTermV1::Unit => {}
    }
    Ok(())
}

fn expected_no_redex_census(
    term: &WireTermV1,
    enabled_deltas: &BTreeSet<u32>,
) -> Result<NoRedexCensusWireV1, BundleValidationErrorV1> {
    let mut entries = Vec::new();
    append_census(term, &mut Vec::new(), enabled_deltas, &mut entries, 0)?;
    Ok(NoRedexCensusWireV1 { entries })
}

fn split_application_spine(term: &WireTermV1) -> (&WireTermV1, Vec<&WireTermV1>) {
    let mut head = term;
    let mut reversed = Vec::new();
    while let WireTermV1::Apply { function, argument } = head {
        reversed.push(argument.as_ref());
        head = function.as_ref();
    }
    reversed.reverse();
    (head, reversed)
}

fn contains_global_slot(term: &WireTermV1, target: u32) -> bool {
    match term {
        WireTermV1::GlobalSlot { slot } => *slot == target,
        WireTermV1::Pi { parameter, body } | WireTermV1::Lambda { parameter, body } => {
            contains_global_slot(parameter, target) || contains_global_slot(body, target)
        }
        WireTermV1::Apply { function, argument } => {
            contains_global_slot(function, target) || contains_global_slot(argument, target)
        }
        WireTermV1::Sort { .. }
        | WireTermV1::Variable { .. }
        | WireTermV1::UnitType
        | WireTermV1::Unit => false,
    }
}

fn is_exact_fresh_pattern(schema: &FreshRuleSchemaWireV1) -> bool {
    let scrutinee = schema.scrutinee_ordinal as usize;
    let (head, arguments) = split_application_spine(&schema.left);
    head == &WireTermV1::GlobalSlot {
        slot: schema.owner_slot,
    } && arguments.len() == scrutinee + 1
        && arguments.get(scrutinee)
            == Some(&&WireTermV1::GlobalSlot {
                slot: schema.constructor_slot,
            })
        && arguments[..scrutinee]
            .iter()
            .enumerate()
            .all(|(ordinal, argument)| {
                **argument
                    == WireTermV1::Variable {
                        index: (scrutinee - ordinal - 1) as u32,
                    }
            })
        && !contains_global_slot(&schema.right, schema.owner_slot)
}
fn check_synthesis_code(
    code: &SynthesisCodeWireV1,
    context_len: u32,
    global_count: u32,
    conversion_ids: &BTreeSet<WireIdV1>,
    depth: u16,
) -> Result<(), BundleValidationErrorV1> {
    if depth > MAX_TERM_DEPTH_V1 {
        return Err(BundleValidationErrorV1::TermDepthExceeded);
    }
    match code {
        SynthesisCodeWireV1::Sort { level } => {
            if !PUBLIC_UNIVERSE_LEVELS_V1.contains(level) {
                return Err(BundleValidationErrorV1::UniverseLevelOutOfRange {
                    role: TermRoleV1::Public,
                    level: *level,
                });
            }
        }
        SynthesisCodeWireV1::VariableLookup {
            index,
            context_ordinal,
            shift_distance,
        } => {
            if *index >= context_len
                || *context_ordinal != context_len - 1 - *index
                || *shift_distance != index.saturating_add(1)
            {
                return Err(BundleValidationErrorV1::VariableMetadataMismatch);
            }
        }
        SynthesisCodeWireV1::GlobalLookup { global_slot } => {
            if *global_slot >= global_count {
                return Err(BundleValidationErrorV1::GlobalSlotOutOfRange {
                    slot: *global_slot,
                    limit: global_count,
                });
            }
        }
        SynthesisCodeWireV1::PiFormation { parameter, body }
        | SynthesisCodeWireV1::LambdaIntroduction {
            parameter_type: parameter,
            body,
        } => {
            check_synthesis_code(
                parameter,
                context_len,
                global_count,
                conversion_ids,
                depth + 1,
            )?;
            check_synthesis_code(
                body,
                context_len + 1,
                global_count,
                conversion_ids,
                depth + 1,
            )?;
        }
        SynthesisCodeWireV1::ApplicationElimination {
            function,
            argument,
            function_conversion_id,
            argument_conversion_id,
            dependent_result_type,
        } => {
            if !conversion_ids.contains(function_conversion_id)
                || !conversion_ids.contains(argument_conversion_id)
            {
                return Err(BundleValidationErrorV1::UnknownConversionReference);
            }
            check_synthesis_code(
                function,
                context_len,
                global_count,
                conversion_ids,
                depth + 1,
            )?;
            check_synthesis_code(
                argument,
                context_len,
                global_count,
                conversion_ids,
                depth + 1,
            )?;
            check_term(
                dependent_result_type,
                context_len,
                global_count,
                TermRoleV1::CheckerProduced,
                0,
            )?;
        }
        SynthesisCodeWireV1::UnitType | SynthesisCodeWireV1::Unit => {}
    }
    Ok(())
}

fn check_synthesis_subject_shape(
    code: &SynthesisCodeWireV1,
    subject: &WireTermV1,
    inferred_type: &WireTermV1,
) -> Result<(), BundleValidationErrorV1> {
    let matches = match (code, subject) {
        (SynthesisCodeWireV1::Sort { level }, WireTermV1::Sort { level: actual }) => {
            level == actual && inferred_type == &WireTermV1::Sort { level: level + 1 }
        }
        (SynthesisCodeWireV1::UnitType, WireTermV1::UnitType) => {
            inferred_type == &WireTermV1::Sort { level: 0 }
        }
        (SynthesisCodeWireV1::Unit, WireTermV1::Unit) => inferred_type == &WireTermV1::UnitType,
        (
            SynthesisCodeWireV1::VariableLookup { index, .. },
            WireTermV1::Variable { index: actual },
        ) => index == actual,
        (SynthesisCodeWireV1::GlobalLookup { global_slot }, WireTermV1::GlobalSlot { slot }) => {
            global_slot == slot
        }
        (
            SynthesisCodeWireV1::PiFormation { parameter, body },
            WireTermV1::Pi {
                parameter: subject_parameter,
                body: subject_body,
            },
        ) => {
            synthesis_code_subject_tag_matches(parameter, subject_parameter)
                && synthesis_code_subject_tag_matches(body, subject_body)
        }
        (
            SynthesisCodeWireV1::LambdaIntroduction {
                parameter_type,
                body,
            },
            WireTermV1::Lambda {
                parameter: subject_parameter,
                body: subject_body,
            },
        ) => {
            synthesis_code_subject_tag_matches(parameter_type, subject_parameter)
                && synthesis_code_subject_tag_matches(body, subject_body)
        }
        (
            SynthesisCodeWireV1::ApplicationElimination {
                function,
                argument,
                dependent_result_type,
                ..
            },
            WireTermV1::Apply {
                function: subject_function,
                argument: subject_argument,
            },
        ) => {
            if dependent_result_type != inferred_type {
                return Err(BundleValidationErrorV1::DependentResultTypeMismatch);
            }
            synthesis_code_subject_tag_matches(function, subject_function)
                && synthesis_code_subject_tag_matches(argument, subject_argument)
        }
        _ => false,
    };
    if matches {
        Ok(())
    } else {
        Err(BundleValidationErrorV1::SynthesisSubjectMismatch)
    }
}

fn synthesis_code_subject_tag_matches(code: &SynthesisCodeWireV1, term: &WireTermV1) -> bool {
    matches!(
        (code, term),
        (SynthesisCodeWireV1::Sort { .. }, WireTermV1::Sort { .. })
            | (SynthesisCodeWireV1::UnitType, WireTermV1::UnitType)
            | (SynthesisCodeWireV1::Unit, WireTermV1::Unit)
            | (
                SynthesisCodeWireV1::VariableLookup { .. },
                WireTermV1::Variable { .. }
            )
            | (
                SynthesisCodeWireV1::GlobalLookup { .. },
                WireTermV1::GlobalSlot { .. }
            )
            | (
                SynthesisCodeWireV1::PiFormation { .. },
                WireTermV1::Pi { .. }
            )
            | (
                SynthesisCodeWireV1::LambdaIntroduction { .. },
                WireTermV1::Lambda { .. }
            )
            | (
                SynthesisCodeWireV1::ApplicationElimination { .. },
                WireTermV1::Apply { .. }
            )
    )
}
fn check_synthesis_certificate(
    certificate: &SynthesisCertificateWireV1,
    global_count: u32,
    conversion_ids: &BTreeSet<WireIdV1>,
) -> Result<(), BundleValidationErrorV1> {
    check_context(&certificate.context, global_count, TermRoleV1::Public)?;
    let local_count = certificate.context.entries_oldest_first.len() as u32;
    check_term(
        &certificate.subject,
        local_count,
        global_count,
        TermRoleV1::Public,
        0,
    )?;
    check_term(
        &certificate.inferred_type,
        local_count,
        global_count,
        TermRoleV1::CheckerProduced,
        0,
    )?;
    check_synthesis_subject_shape(
        &certificate.code,
        &certificate.subject,
        &certificate.inferred_type,
    )?;
    check_synthesis_code(
        &certificate.code,
        local_count,
        global_count,
        conversion_ids,
        0,
    )
}

fn check_family_judgment(
    judgment: &FamilyJudgmentWireV1,
    global_count: u32,
) -> Result<(), BundleValidationErrorV1> {
    check_context(&judgment.context, global_count, TermRoleV1::Public)?;
    let local_count = judgment.context.entries_oldest_first.len() as u32;
    check_term(
        &judgment.subject,
        local_count,
        global_count,
        TermRoleV1::Public,
        0,
    )?;
    check_term(
        &judgment.ty,
        local_count,
        global_count,
        TermRoleV1::Public,
        0,
    )
}

pub fn validate_bundle_v1(
    bundle: &ProductionRefinementBundleV1,
) -> Result<(), BundleValidationErrorV1> {
    if bundle.header != WireHeaderV1::canonical() {
        return Err(BundleValidationErrorV1::HeaderMismatch);
    }
    let manifest = &bundle.manifest_surface;
    if manifest.semantic_schema_version != V3_SEMANTIC_SCHEMA_VERSION
        || manifest.profile_id != V3_SEMANTIC_PROFILE_ID
        || manifest.authority != ManifestAuthorityWireV1::GenericPrototypeOnly
        || manifest.frozen
        || manifest.live_profile_a_access
        || manifest.public_universe_levels != PUBLIC_UNIVERSE_LEVELS_V1
        || manifest.checker_universe_levels != CHECKER_UNIVERSE_LEVELS_V1
        || manifest.formation_witness_levels != FORMATION_WITNESS_LEVELS_V1
        || manifest.maximum_context_entries != MAX_CONTEXT_ENTRIES_V1
        || manifest.synthesis_rule_inventory != EXACT_SYNTHESIS_INVENTORY_V1
        || manifest.synthesis_protocol_id != SYNTHESIS_PROTOCOL_ID_V2
        || manifest.synthesis_schema_version != SYNTHESIS_SCHEMA_VERSION_V2
    {
        return Err(BundleValidationErrorV1::ManifestSurfaceMismatch);
    }

    limit_len(bundle.global_slot_table.entries.len(), "global slots")?;
    limit_len(bundle.contexts.len(), "contexts")?;
    limit_len(bundle.conversions.len(), "conversions")?;
    limit_len(
        bundle.conversion_typing_supplements.len(),
        "conversion typing supplements",
    )?;
    limit_len(bundle.synthesis_codes.len(), "synthesis codes")?;
    limit_len(bundle.fresh_rule_schemas.len(), "fresh rule schemas")?;
    limit_len(bundle.family_payloads.len(), "family payloads")?;

    let global_count = u32::try_from(bundle.global_slot_table.entries.len()).map_err(|_| {
        BundleValidationErrorV1::ResourceLimit {
            kind: "global slots",
        }
    })?;
    let mut global_ids = BTreeSet::new();
    for (expected, entry) in bundle.global_slot_table.entries.iter().enumerate() {
        let expected = expected as u32;
        if entry.slot != expected {
            return Err(BundleValidationErrorV1::SlotOrder {
                expected,
                actual: entry.slot,
            });
        }
        if !global_ids.insert(entry.global_id_bytes) {
            return Err(BundleValidationErrorV1::DuplicateGlobalId);
        }
        check_term(
            &entry.declaration_type,
            0,
            global_count,
            TermRoleV1::Public,
            0,
        )?;
        if let Some(body) = &entry.declaration_body {
            check_term(body, 0, global_count, TermRoleV1::Public, 0)?;
        }
        for term in std::iter::once(&entry.declaration_type).chain(entry.declaration_body.iter()) {
            let mut referenced = Vec::new();
            collect_global_slots(term, &mut referenced);
            if let Some(referenced_slot) = referenced.into_iter().find(|slot| *slot >= expected) {
                return Err(BundleValidationErrorV1::ForwardGlobalReference {
                    declaration_slot: expected,
                    referenced_slot,
                });
            }
        }
    }

    let mut previous_delta = None;
    for delta in &bundle.signature.allowed_transparent_deltas {
        if previous_delta.is_some_and(|slot| delta.global_slot <= slot) {
            return Err(BundleValidationErrorV1::DeltaPolicyOrder);
        }
        let Some(entry) = bundle
            .global_slot_table
            .entries
            .get(delta.global_slot as usize)
        else {
            return Err(BundleValidationErrorV1::DeltaPolicyEntryMismatch);
        };
        if entry.global_id_bytes != delta.global_id_bytes || entry.declaration_body.is_none() {
            return Err(BundleValidationErrorV1::DeltaPolicyEntryMismatch);
        }
        previous_delta = Some(delta.global_slot);
    }

    for context in &bundle.contexts {
        check_context(context, global_count, TermRoleV1::Public)?;
    }

    let mut conversion_ids = BTreeSet::new();
    let enabled_deltas = bundle
        .signature
        .allowed_transparent_deltas
        .iter()
        .map(|entry| entry.global_slot)
        .collect::<BTreeSet<_>>();
    for conversion in &bundle.conversions {
        if !conversion_ids.insert(conversion.conversion_id) {
            return Err(BundleValidationErrorV1::DuplicateConversionId);
        }
        check_context(&conversion.context, global_count, TermRoleV1::Public)?;
        let local_count = conversion.context.entries_oldest_first.len() as u32;
        for term in [
            &conversion.left,
            &conversion.right,
            &conversion.common_normal_form,
        ] {
            check_term(
                term,
                local_count,
                global_count,
                TermRoleV1::CheckerProduced,
                0,
            )?;
        }
        if let EndpointJudgmentWireV1::HasType { expected_type } = &conversion.endpoint_judgment {
            check_term(
                expected_type,
                local_count,
                global_count,
                TermRoleV1::CheckerProduced,
                0,
            )?;
        }
        check_trace(&conversion.left_trace, local_count, global_count)?;
        check_trace(&conversion.right_trace, local_count, global_count)?;
        if conversion.left_trace.start != conversion.left
            || conversion.right_trace.start != conversion.right
            || conversion.left_trace.end != conversion.common_normal_form
            || conversion.right_trace.end != conversion.common_normal_form
        {
            return Err(BundleValidationErrorV1::ReductionTraceEndpointMismatch);
        }
        if conversion.no_redex_census
            != expected_no_redex_census(&conversion.common_normal_form, &enabled_deltas)?
        {
            return Err(BundleValidationErrorV1::NoRedexCensusMismatch);
        }
    }

    let mut synthesis_ids = BTreeSet::new();
    for synthesis in &bundle.synthesis_codes {
        if !synthesis_ids.insert(synthesis.synthesis_id) {
            return Err(BundleValidationErrorV1::DuplicateSynthesisId);
        }
        check_synthesis_certificate(synthesis, global_count, &conversion_ids)?;
    }

    for supplement in &bundle.conversion_typing_supplements {
        if !conversion_ids.contains(&supplement.conversion_id) {
            return Err(BundleValidationErrorV1::UnknownConversionReference);
        }
        check_context(&supplement.local_context, global_count, TermRoleV1::Public)?;
        if let Some(level) = supplement.formation_level {
            if !FORMATION_WITNESS_LEVELS_V1.contains(&level) {
                return Err(BundleValidationErrorV1::FormationLevelOutOfRange(level));
            }
        }
        check_synthesis_certificate(
            &supplement.source_typing_code,
            global_count,
            &conversion_ids,
        )?;
        check_synthesis_certificate(
            &supplement.target_typing_code,
            global_count,
            &conversion_ids,
        )?;
        if supplement.source_typing_code.context != supplement.local_context
            || supplement.target_typing_code.context != supplement.local_context
        {
            return Err(BundleValidationErrorV1::SupplementContextMismatch);
        }
    }

    if bundle.q0_inventory.ordered_rules != EXACT_Q0_INVENTORY_V1 {
        return Err(BundleValidationErrorV1::Q0InventoryMismatch);
    }

    let mut equation_ids = BTreeSet::new();
    for schema in &bundle.fresh_rule_schemas {
        if !equation_ids.insert(schema.equation_id) {
            return Err(BundleValidationErrorV1::DuplicateFreshEquationId);
        }
        if schema.owner_slot >= global_count || schema.constructor_slot >= global_count {
            return Err(BundleValidationErrorV1::FreshSlotOutOfRange);
        }
        check_context(&schema.parameter_context, global_count, TermRoleV1::Public)?;
        let local_count = schema.parameter_context.entries_oldest_first.len() as u32;
        if schema.scrutinee_ordinal >= local_count || u32::from(schema.arity) != local_count {
            return Err(BundleValidationErrorV1::FreshScrutineeOutOfRange);
        }
        for term in [&schema.left, &schema.right, &schema.ty] {
            check_term(term, local_count, global_count, TermRoleV1::Public, 0)?;
        }
        if schema.scrutinee_ordinal + 1 != u32::from(schema.arity)
            || !is_exact_fresh_pattern(schema)
        {
            return Err(BundleValidationErrorV1::FreshPatternMismatch);
        }
    }

    if bundle.family_inventory.ordered_codes != EXACT_FAMILY_INVENTORY_V1 {
        return Err(BundleValidationErrorV1::FamilyInventoryMismatch);
    }
    let mut family_ids = BTreeSet::new();
    let mut family_positions = BTreeMap::new();
    for (position, family) in bundle.family_payloads.iter().enumerate() {
        if !family_ids.insert(family.family_id()) {
            return Err(BundleValidationErrorV1::DuplicateFamilyId);
        }
        family_positions.insert(family.family_id(), position);
        match family {
            FamilyPayloadWireV1::Seed {
                source, judgment, ..
            } => {
                match source {
                    SeedSourceWireV1::PublicHead { owner_slot } => {
                        if *owner_slot >= global_count {
                            return Err(BundleValidationErrorV1::FreshSlotOutOfRange);
                        }
                    }
                    SeedSourceWireV1::PublicEquation { equation_id } => {
                        if !equation_ids.contains(equation_id) {
                            return Err(BundleValidationErrorV1::MissingEquationReference);
                        }
                    }
                }
                check_family_judgment(judgment, global_count)?;
            }
            FamilyPayloadWireV1::GenericPublicApplication {
                function_family_id,
                argument_family_id,
                judgment,
                ..
            } => {
                if family_positions
                    .get(function_family_id)
                    .is_none_or(|p| *p >= position)
                    || family_positions
                        .get(argument_family_id)
                        .is_none_or(|p| *p >= position)
                {
                    return Err(BundleValidationErrorV1::MissingFamilyReference);
                }
                check_family_judgment(judgment, global_count)?;
            }
            FamilyPayloadWireV1::GenericEquationAction {
                equation_id,
                source_family_id,
                judgment,
                ..
            } => {
                if !equation_ids.contains(equation_id) {
                    return Err(BundleValidationErrorV1::MissingEquationReference);
                }
                if family_positions
                    .get(source_family_id)
                    .is_none_or(|p| *p >= position)
                {
                    return Err(BundleValidationErrorV1::MissingFamilyReference);
                }
                check_family_judgment(judgment, global_count)?;
            }
        }
    }
    Ok(())
}
