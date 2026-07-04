use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructureSchemaVariant {
    pub name: &'static str,
    pub rationale: &'static str,
    pub telescope: Telescope,
}

pub fn frozen_structure_schema_variants() -> Vec<StructureSchemaVariant> {
    vec![
        StructureSchemaVariant {
            name: "collapse_only",
            rationale: "Minimal formation rule: an overdensity premise opens a bound-interface stratum.",
            telescope: Telescope::new(vec![overdensity_to_bound_interface()]),
        },
        StructureSchemaVariant {
            name: "collapse_virial",
            rationale: "Adds the balance/closure equation expressing virialized self-consistency of the bound interface.",
            telescope: Telescope::new(vec![overdensity_to_bound_interface(), virial_balance()]),
        },
        StructureSchemaVariant {
            name: "collapse_virial_merger",
            rationale: "Adds hierarchical composition: a paired bound interface composes back into a bound interface.",
            telescope: Telescope::new(vec![
                overdensity_to_bound_interface(),
                virial_balance(),
                hierarchical_composition(),
            ]),
        },
    ]
}

fn overdensity_to_bound_interface() -> ClauseRec {
    ClauseRec::new(
        ClauseRole::Formation,
        Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
        ),
    )
}

fn virial_balance() -> ClauseRec {
    ClauseRec::new(
        ClauseRole::Computation,
        Expr::Id(
            Box::new(Expr::Var(1)),
            Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            Box::new(Expr::Var(2)),
        ),
    )
}

fn hierarchical_composition() -> ClauseRec {
    ClauseRec::new(
        ClauseRole::Formation,
        Expr::Pi(
            Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            Box::new(Expr::Var(1)),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::frozen_structure_schema_variants;

    #[test]
    fn structure_schema_variants_are_frozen_in_a_priori_order() {
        let variants = frozen_structure_schema_variants();
        let names: Vec<_> = variants.iter().map(|variant| variant.name).collect();
        assert_eq!(
            names,
            vec!["collapse_only", "collapse_virial", "collapse_virial_merger"]
        );
        assert_eq!(variants[0].telescope.kappa(), 1);
        assert_eq!(variants[1].telescope.kappa(), 2);
        assert_eq!(variants[2].telescope.kappa(), 3);
    }
}
