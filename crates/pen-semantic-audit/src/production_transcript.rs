//! The versioned canonical production transcript (Phase G).
//!
//! Rust and safe Agda independently render this byte string from the
//! same canonical bundle: Rust from the unchanged-kernel replay
//! artifacts (`ProductionReplayComputedV1`), Agda from its own
//! decode/replay layers. The two renderings are compared as actual
//! bytes; digests are recorded only after that comparison.
//!
//! The transcript has its own schema identity (magic and version) and
//! a fixed flat layout: it is a comparison artifact, never a decoded
//! input, so it carries no section tags. Every list is length-prefixed
//! with a little-endian `u64`, every term uses the wire term byte
//! encoding, and every 32-byte identity is raw. The content follows
//! the adjudication's field list exactly: decoded signature and
//! contexts, variable and global resolutions, inferred types, numeric
//! formation levels, every conversion step and endpoint, binder-local
//! conversion supplements, common normal forms and no-redex censuses,
//! synthesis rule trees with premise and conversion identifiers,
//! dependent application result types, Q0 mappings, fresh-rule
//! dispositions, and family payload mappings. Computed entries
//! (normalized forms, replayed types, formation levels, derived
//! supplement contexts, Q0 categories) come from each side's own
//! replay, so a divergence in any semantically important choice is
//! byte-visible.

use crate::production_wire_replay::ProductionReplayComputedV1;
use pen_production_wire::{
    BaseQ0ReductionStepWireV1, BaseQ0ReductionTraceWireV1, ConversionPathComponentWireV1,
    EndpointJudgmentWireV1, FamilyPayloadWireV1, NoRedexCensusWireV1, NoRedexDispositionWireV1,
    ProductionContextWireV1, ProductionRefinementBundleV1, Q0RuleWireV1, SeedSourceWireV1,
    SynthesisCodeWireV1, WireIdV1, WireTermV1,
};

pub const PRODUCTION_TRANSCRIPT_MAGIC_V1: [u8; 16] = *b"PEN-PROD-TRAN-V1";
pub const PRODUCTION_TRANSCRIPT_SCHEMA_VERSION_V1: u16 = 1;

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_id(out: &mut Vec<u8>, id: &WireIdV1) {
    out.extend_from_slice(&id.0);
}

fn push_term(out: &mut Vec<u8>, term: &WireTermV1) {
    match term {
        WireTermV1::Sort { level } => {
            out.push(0);
            push_u16(out, *level);
        }
        WireTermV1::Variable { index } => {
            out.push(1);
            push_u32(out, *index);
        }
        WireTermV1::GlobalSlot { slot } => {
            out.push(2);
            push_u32(out, *slot);
        }
        WireTermV1::Pi { parameter, body } => {
            out.push(3);
            push_term(out, parameter);
            push_term(out, body);
        }
        WireTermV1::Lambda { parameter, body } => {
            out.push(4);
            push_term(out, parameter);
            push_term(out, body);
        }
        WireTermV1::Apply { function, argument } => {
            out.push(5);
            push_term(out, function);
            push_term(out, argument);
        }
        WireTermV1::UnitType => out.push(6),
        WireTermV1::Unit => out.push(7),
    }
}

fn push_context_entries(out: &mut Vec<u8>, context: &ProductionContextWireV1) {
    push_u64(out, context.entries_oldest_first.len() as u64);
    for entry in &context.entries_oldest_first {
        push_term(out, entry);
    }
}

fn push_step(out: &mut Vec<u8>, step: &BaseQ0ReductionStepWireV1) {
    use BaseQ0ReductionStepWireV1 as Wire;
    match step {
        Wire::Beta { source, target } => {
            out.push(0);
            push_term(out, source);
            push_term(out, target);
        }
        Wire::TransparentDelta {
            source,
            target,
            global_slot,
        } => {
            out.push(1);
            push_term(out, source);
            push_term(out, target);
            push_u32(out, *global_slot);
        }
        Wire::PiParameterCongruence {
            source,
            target,
            premise,
        } => {
            out.push(2);
            push_term(out, source);
            push_term(out, target);
            push_step(out, premise);
        }
        Wire::PiBodyCongruence {
            source,
            target,
            premise,
        } => {
            out.push(3);
            push_term(out, source);
            push_term(out, target);
            push_step(out, premise);
        }
        Wire::LambdaParameterCongruence {
            source,
            target,
            premise,
        } => {
            out.push(4);
            push_term(out, source);
            push_term(out, target);
            push_step(out, premise);
        }
        Wire::LambdaBodyCongruence {
            source,
            target,
            premise,
        } => {
            out.push(5);
            push_term(out, source);
            push_term(out, target);
            push_step(out, premise);
        }
        Wire::ApplyFunctionCongruence {
            source,
            target,
            premise,
        } => {
            out.push(6);
            push_term(out, source);
            push_term(out, target);
            push_step(out, premise);
        }
        Wire::ApplyArgumentCongruence {
            source,
            target,
            premise,
        } => {
            out.push(7);
            push_term(out, source);
            push_term(out, target);
            push_step(out, premise);
        }
    }
}

fn push_trace(out: &mut Vec<u8>, trace: &BaseQ0ReductionTraceWireV1) {
    push_term(out, &trace.start);
    push_u64(out, trace.steps.len() as u64);
    for step in &trace.steps {
        push_step(out, step);
    }
    push_term(out, &trace.end);
}

fn path_component_tag(component: ConversionPathComponentWireV1) -> u8 {
    match component {
        ConversionPathComponentWireV1::PiParameter => 0,
        ConversionPathComponentWireV1::PiBody => 1,
        ConversionPathComponentWireV1::LambdaParameter => 2,
        ConversionPathComponentWireV1::LambdaBody => 3,
        ConversionPathComponentWireV1::ApplyFunction => 4,
        ConversionPathComponentWireV1::ApplyArgument => 5,
    }
}

fn disposition_tag(disposition: NoRedexDispositionWireV1) -> u8 {
    match disposition {
        NoRedexDispositionWireV1::Sort => 0,
        NoRedexDispositionWireV1::Variable => 1,
        NoRedexDispositionWireV1::GlobalNotEnabledByPolicy => 2,
        NoRedexDispositionWireV1::Pi => 3,
        NoRedexDispositionWireV1::Lambda => 4,
        NoRedexDispositionWireV1::NeutralApplication => 5,
        NoRedexDispositionWireV1::UnitType => 6,
        NoRedexDispositionWireV1::Unit => 7,
    }
}

fn push_census(out: &mut Vec<u8>, census: &NoRedexCensusWireV1) {
    push_u64(out, census.entries.len() as u64);
    for entry in &census.entries {
        push_u64(out, entry.path.len() as u64);
        for component in &entry.path {
            out.push(path_component_tag(*component));
        }
        push_term(out, &entry.term);
        out.push(disposition_tag(entry.disposition));
    }
}

fn push_code(out: &mut Vec<u8>, code: &SynthesisCodeWireV1) {
    match code {
        SynthesisCodeWireV1::Sort { level } => {
            out.push(0);
            push_u16(out, *level);
        }
        SynthesisCodeWireV1::UnitType => out.push(1),
        SynthesisCodeWireV1::Unit => out.push(2),
        SynthesisCodeWireV1::VariableLookup {
            index,
            context_ordinal,
            shift_distance,
        } => {
            out.push(3);
            push_u32(out, *index);
            push_u32(out, *context_ordinal);
            push_u32(out, *shift_distance);
        }
        SynthesisCodeWireV1::GlobalLookup { global_slot } => {
            out.push(4);
            push_u32(out, *global_slot);
        }
        SynthesisCodeWireV1::PiFormation { parameter, body } => {
            out.push(5);
            push_code(out, parameter);
            push_code(out, body);
        }
        SynthesisCodeWireV1::LambdaIntroduction {
            parameter_type,
            body,
        } => {
            out.push(6);
            push_code(out, parameter_type);
            push_code(out, body);
        }
        SynthesisCodeWireV1::ApplicationElimination {
            function,
            argument,
            function_conversion_id,
            argument_conversion_id,
            dependent_result_type,
        } => {
            out.push(7);
            push_code(out, function);
            push_code(out, argument);
            push_id(out, function_conversion_id);
            push_id(out, argument_conversion_id);
            push_term(out, dependent_result_type);
        }
    }
}

fn push_endpoint(
    out: &mut Vec<u8>,
    endpoint: &EndpointJudgmentWireV1,
    formation_level: Option<u16>,
) {
    match endpoint {
        EndpointJudgmentWireV1::HasType { expected_type } => {
            out.push(0);
            push_term(out, expected_type);
        }
        EndpointJudgmentWireV1::TypeFormation => {
            out.push(1);
            push_u16(out, formation_level.unwrap_or(u16::MAX));
        }
    }
}

/// Render the canonical transcript from the decoded bundle and the
/// replay's computed artifacts. Pure rendering: no authority, no
/// digests, no diagnostics.
pub fn render_production_transcript_v1(
    bundle: &ProductionRefinementBundleV1,
    computed: &ProductionReplayComputedV1,
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&PRODUCTION_TRANSCRIPT_MAGIC_V1);
    push_u16(&mut out, PRODUCTION_TRANSCRIPT_SCHEMA_VERSION_V1);

    // Section 1: decoded signature with computed formation levels.
    let globals = &bundle.global_slot_table.entries;
    push_u64(&mut out, globals.len() as u64);
    for (entry, level) in globals.iter().zip(&computed.declaration_formation_levels) {
        push_u32(&mut out, entry.slot);
        push_term(&mut out, &entry.declaration_type);
        match &entry.declaration_body {
            None => out.push(0),
            Some(body) => {
                out.push(1);
                push_term(&mut out, body);
            }
        }
        push_u16(&mut out, *level);
    }

    // Section 2: contexts with computed entry formation levels and the
    // variable-resolution table (oldest-first ordinal, shift distance,
    // shifted looked-up type).
    push_u64(&mut out, bundle.contexts.len() as u64);
    for (context, levels) in bundle
        .contexts
        .iter()
        .zip(&computed.context_entry_formation_levels)
    {
        let entries = &context.entries_oldest_first;
        push_u64(&mut out, entries.len() as u64);
        for (entry, level) in entries.iter().zip(levels) {
            push_term(&mut out, entry);
            push_u16(&mut out, *level);
        }
        let count = entries.len() as u32;
        for index in 0..count {
            let oldest = count - 1 - index;
            let shift = index + 1;
            push_u32(&mut out, index);
            push_u32(&mut out, oldest);
            push_u32(&mut out, shift);
            let mut looked_up = entries[oldest as usize].clone();
            for _ in 0..shift {
                looked_up = shift_term(0, &looked_up);
            }
            push_term(&mut out, &looked_up);
        }
    }

    // Section 3: conversions with computed normalized endpoints and
    // TypeFormation levels, every step, and the census.
    push_u64(&mut out, bundle.conversions.len() as u64);
    for (position, certificate) in bundle.conversions.iter().enumerate() {
        push_id(&mut out, &certificate.conversion_id);
        push_context_entries(&mut out, &certificate.context);
        push_term(&mut out, &certificate.left);
        push_term(&mut out, &certificate.right);
        push_endpoint(
            &mut out,
            &certificate.endpoint_judgment,
            computed.conversion_formation_levels[position],
        );
        push_term(&mut out, &certificate.common_normal_form);
        push_term(&mut out, &computed.conversion_normalized_left[position]);
        push_term(&mut out, &computed.conversion_normalized_right[position]);
        push_trace(&mut out, &certificate.left_trace);
        push_trace(&mut out, &certificate.right_trace);
        push_census(&mut out, &certificate.no_redex_census);
    }

    // Section 4: binder-local supplements with the DERIVED contexts and
    // premise endpoints computed by the replay.
    push_u64(&mut out, bundle.conversion_typing_supplements.len() as u64);
    for (position, supplement) in bundle.conversion_typing_supplements.iter().enumerate() {
        push_id(&mut out, &supplement.conversion_id);
        push_u64(&mut out, supplement.step_path.len() as u64);
        for component in &supplement.step_path {
            push_u32(&mut out, *component);
        }
        let derived = &computed.supplement_derived_contexts[position];
        push_u64(&mut out, derived.len() as u64);
        for entry in derived {
            push_term(&mut out, entry);
        }
        push_term(&mut out, &computed.supplement_premise_sources[position]);
        push_term(&mut out, &computed.supplement_premise_targets[position]);
        match &supplement.local_endpoint_judgment {
            EndpointJudgmentWireV1::HasType { expected_type } => {
                out.push(0);
                push_term(&mut out, expected_type);
            }
            EndpointJudgmentWireV1::TypeFormation => out.push(1),
        }
        match supplement.formation_level {
            None => out.push(0),
            Some(level) => {
                out.push(1);
                push_u16(&mut out, level);
            }
        }
        let (source_type, target_type) = &computed.supplement_certificate_types[position];
        push_id(&mut out, &supplement.source_typing_code.synthesis_id);
        push_term(&mut out, source_type);
        push_id(&mut out, &supplement.target_typing_code.synthesis_id);
        push_term(&mut out, target_type);
    }

    // Section 5: synthesis certificates with the replayed types and
    // dependent application results.
    push_u64(&mut out, bundle.synthesis_codes.len() as u64);
    for (position, certificate) in bundle.synthesis_codes.iter().enumerate() {
        push_id(&mut out, &certificate.synthesis_id);
        push_context_entries(&mut out, &certificate.context);
        push_term(&mut out, &certificate.subject);
        push_term(&mut out, &computed.synthesis_types[position]);
        push_code(&mut out, &certificate.code);
        let results = &computed.synthesis_dependent_results[position];
        push_u64(&mut out, results.len() as u64);
        for result in results {
            push_term(&mut out, result);
        }
    }

    // Section 6: Q0 mappings.
    push_u64(&mut out, bundle.q0_inventory.ordered_rules.len() as u64);
    for (rule, category) in bundle
        .q0_inventory
        .ordered_rules
        .iter()
        .zip(&computed.q0_categories)
    {
        out.push(q0_rule_tag(*rule));
        out.push(*category);
    }

    // Section 7: fresh-rule dispositions.
    push_u64(&mut out, bundle.fresh_rule_schemas.len() as u64);
    for (schema, level) in bundle
        .fresh_rule_schemas
        .iter()
        .zip(&computed.fresh_type_formation_levels)
    {
        push_id(&mut out, &schema.equation_id);
        push_u32(&mut out, schema.owner_slot);
        push_u32(&mut out, schema.constructor_slot);
        push_u16(&mut out, schema.arity);
        push_u32(&mut out, schema.scrutinee_ordinal);
        push_context_entries(&mut out, &schema.parameter_context);
        push_term(&mut out, &schema.left);
        push_term(&mut out, &schema.right);
        push_term(&mut out, &schema.ty);
        push_u16(&mut out, *level);
    }

    // Section 8: family payload mappings with the replayed normalized
    // subject types.
    push_u64(&mut out, bundle.family_payloads.len() as u64);
    for (payload, normalized) in bundle
        .family_payloads
        .iter()
        .zip(&computed.family_normalized_types)
    {
        match payload {
            FamilyPayloadWireV1::Seed {
                family_id,
                source,
                judgment,
            } => {
                out.push(0);
                push_id(&mut out, family_id);
                match source {
                    SeedSourceWireV1::PublicHead { owner_slot } => {
                        out.push(0);
                        push_u32(&mut out, *owner_slot);
                    }
                    SeedSourceWireV1::PublicEquation { equation_id } => {
                        out.push(1);
                        push_id(&mut out, equation_id);
                    }
                }
                push_context_entries(&mut out, &judgment.context);
                push_term(&mut out, &judgment.subject);
                push_term(&mut out, &judgment.ty);
            }
            FamilyPayloadWireV1::GenericPublicApplication {
                family_id,
                function_family_id,
                argument_family_id,
                judgment,
            } => {
                out.push(1);
                push_id(&mut out, family_id);
                push_id(&mut out, function_family_id);
                push_id(&mut out, argument_family_id);
                push_context_entries(&mut out, &judgment.context);
                push_term(&mut out, &judgment.subject);
                push_term(&mut out, &judgment.ty);
            }
            FamilyPayloadWireV1::GenericEquationAction {
                family_id,
                equation_id,
                source_family_id,
                judgment,
            } => {
                out.push(2);
                push_id(&mut out, family_id);
                push_id(&mut out, equation_id);
                push_id(&mut out, source_family_id);
                push_context_entries(&mut out, &judgment.context);
                push_term(&mut out, &judgment.subject);
                push_term(&mut out, &judgment.ty);
            }
        }
        push_term(&mut out, normalized);
    }

    out
}

fn q0_rule_tag(rule: Q0RuleWireV1) -> u8 {
    match rule {
        Q0RuleWireV1::DeBruijn => 0,
        Q0RuleWireV1::SequentialSubstitution => 1,
        Q0RuleWireV1::Beta => 2,
        Q0RuleWireV1::ProvenancePreservingDelta => 3,
        Q0RuleWireV1::Unit => 4,
        Q0RuleWireV1::TelescopeFlattening => 5,
        Q0RuleWireV1::FreshNonrecursiveConstructorComputation => 6,
    }
}

fn shift_term(cutoff: u32, term: &WireTermV1) -> WireTermV1 {
    match term {
        WireTermV1::Sort { level } => WireTermV1::Sort { level: *level },
        WireTermV1::Variable { index } => WireTermV1::Variable {
            index: if *index < cutoff { *index } else { index + 1 },
        },
        WireTermV1::GlobalSlot { slot } => WireTermV1::GlobalSlot { slot: *slot },
        WireTermV1::Pi { parameter, body } => WireTermV1::Pi {
            parameter: Box::new(shift_term(cutoff, parameter)),
            body: Box::new(shift_term(cutoff + 1, body)),
        },
        WireTermV1::Lambda { parameter, body } => WireTermV1::Lambda {
            parameter: Box::new(shift_term(cutoff, parameter)),
            body: Box::new(shift_term(cutoff + 1, body)),
        },
        WireTermV1::Apply { function, argument } => WireTermV1::Apply {
            function: Box::new(shift_term(cutoff, function)),
            argument: Box::new(shift_term(cutoff, argument)),
        },
        WireTermV1::UnitType => WireTermV1::UnitType,
        WireTermV1::Unit => WireTermV1::Unit,
    }
}
