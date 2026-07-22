//! Read-only diagnostic for the exact historical A3 instance/orbit surface.
//!
//! This probe does not issue provenance and does not consult any historical
//! score.  It exposes the pre-candidate demand inventory needed to decide
//! whether an act-local injection can possibly reproduce a later comparator.

use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3RuleConstructor, generate_a3_window_for_exact_prefix_unbounded,
};
use pen_type::elaborate::SealedSignature;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
struct ConstructorCount {
    instances: usize,
    orbits: usize,
    independently_exported_orbits: usize,
}

#[derive(Debug, Serialize)]
struct StageCount {
    stage: u32,
    typed_sources: usize,
    schemes: usize,
    instances: usize,
    orbits: usize,
    base_orbits: usize,
    structural_orbits: usize,
    independently_exported_orbits: usize,
    by_constructor: BTreeMap<String, ConstructorCount>,
}

fn constructor_name(constructor: A3RuleConstructor) -> &'static str {
    match constructor {
        A3RuleConstructor::UnaryAction => "unary_action",
        A3RuleConstructor::ChronologicalComparison => "chronological_comparison",
        A3RuleConstructor::HigherOpenBoxReduction => "higher_open_box_reduction",
        A3RuleConstructor::StructuralCompletionHole => "structural_completion_hole",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stages = Vec::new();
    for stage in 1..=16 {
        let prefix = SealedSignature::from_telescopes(
            (1..stage)
                .map(|prior| (prior, Telescope::reference(prior)))
                .collect(),
        );
        let window = generate_a3_window_for_exact_prefix_unbounded(&prefix, stage)?;
        let schemes = window
            .schemes
            .iter()
            .map(|scheme| (scheme.scheme_id.as_str(), scheme))
            .collect::<BTreeMap<_, _>>();
        let mut by_constructor = A3RuleConstructor::ALL
            .into_iter()
            .map(|constructor| {
                (
                    constructor_name(constructor).to_owned(),
                    ConstructorCount {
                        instances: 0,
                        orbits: 0,
                        independently_exported_orbits: 0,
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        for instance in &window.instances {
            let scheme = schemes
                .get(instance.scheme_id.as_str())
                .ok_or("A3 instance has no scheme")?;
            by_constructor
                .get_mut(constructor_name(scheme.rule_constructor))
                .expect("closed constructor inventory")
                .instances += 1;
        }
        let mut base_orbits = 0;
        let mut structural_orbits = 0;
        let mut independently_exported_orbits = 0;
        for orbit in &window.orbits {
            let scheme = schemes
                .get(orbit.scheme_id.as_str())
                .ok_or("A3 orbit has no scheme")?;
            let count = by_constructor
                .get_mut(constructor_name(scheme.rule_constructor))
                .expect("closed constructor inventory");
            count.orbits += 1;
            if orbit.independently_exported_demand_orbit {
                count.independently_exported_orbits += 1;
                independently_exported_orbits += 1;
            }
            match scheme.origin {
                A3DemandSchemeOrigin::BaseRule { .. } => base_orbits += 1,
                A3DemandSchemeOrigin::StructuralCompletion { .. } => structural_orbits += 1,
            }
        }
        stages.push(StageCount {
            stage,
            typed_sources: window.typed_sources.len(),
            schemes: window.schemes.len(),
            instances: window.instances.len(),
            orbits: window.orbits.len(),
            base_orbits,
            structural_orbits,
            independently_exported_orbits,
            by_constructor,
        });
    }
    println!("{}", serde_json::to_string_pretty(&stages)?);
    Ok(())
}
