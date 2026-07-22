use pen_core::telescope::Telescope;
use pen_search::act_local_provenance::{ActLocalProvenanceAnchor, issue_act_local_sequence};

fn main() {
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    let packages = issue_act_local_sequence(&entries).expect("T-BI-NU1 enacted probe");
    for package in packages {
        let local = package
            .ordinary_family_tokens
            .iter()
            .filter(|token| {
                matches!(
                    token.anchor,
                    ActLocalProvenanceAnchor::ChargedLocalRole { .. }
                )
            })
            .count();
        let demand = package.ordinary_family_tokens.len() - local;
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            package.stage,
            package.structural_formula_total,
            package.exact_certified_nu,
            local,
            demand,
            package.independent_structural_demand_orbit_ids.len(),
        );
    }
}
