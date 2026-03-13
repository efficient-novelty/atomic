use crate::nu::StructuralNuResult;
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;

pub fn trace_lines(telescope: &Telescope, result: &StructuralNuResult) -> Vec<String> {
    let mut lines = vec![
        String::from("source=structural-ast"),
        format!("nu_g={}", result.nu_g),
        format!("nu_h={}", result.nu_h),
        format!("nu_c={}", result.nu_c),
        format!("bonus_distributive={}", result.distributive_law_bonus),
        format!("bonus_universe_poly={}", result.universe_polymorphism_bonus),
        format!(
            "bonus_infinitesimal_shift={}",
            result.infinitesimal_shift_bonus
        ),
        format!("nu_total={}", result.total),
    ];

    let node_trace: Vec<_> = telescope
        .clauses
        .iter()
        .enumerate()
        .flat_map(|(index, clause)| trace_expr(&format!("entry{}", index + 1), &clause.expr))
        .collect();
    lines.push(format!("node_trace_count={}", node_trace.len()));
    lines.extend(node_trace);
    lines
}

pub fn render_trace(telescope: &Telescope, result: &StructuralNuResult) -> String {
    trace_lines(telescope, result).join("\n")
}

fn trace_expr(path: &str, expr: &Expr) -> Vec<String> {
    let mut lines = vec![format!("node={path}|ctor={}", ctor_name(expr))];
    match expr {
        Expr::Var(_) | Expr::Lib(_) | Expr::Univ | Expr::PathCon(_) => {}
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            lines.extend(trace_expr(&format!("{path}/0"), function));
            lines.extend(trace_expr(&format!("{path}/1"), argument));
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => {
            lines.extend(trace_expr(&format!("{path}/0"), body));
        }
        Expr::Id(ty, left, right) => {
            lines.extend(trace_expr(&format!("{path}/0"), ty));
            lines.extend(trace_expr(&format!("{path}/1"), left));
            lines.extend(trace_expr(&format!("{path}/2"), right));
        }
    }
    lines
}

fn ctor_name(expr: &Expr) -> &'static str {
    match expr {
        Expr::App(_, _) => "App",
        Expr::Lam(_) => "Lam",
        Expr::Pi(_, _) => "Pi",
        Expr::Sigma(_, _) => "Sigma",
        Expr::Univ => "Univ",
        Expr::Var(_) => "Var",
        Expr::Lib(_) => "Lib",
        Expr::Id(_, _, _) => "Id",
        Expr::Refl(_) => "Refl",
        Expr::Susp(_) => "Susp",
        Expr::Trunc(_) => "Trunc",
        Expr::PathCon(_) => "PathCon",
        Expr::Flat(_) => "Flat",
        Expr::Sharp(_) => "Sharp",
        Expr::Disc(_) => "Disc",
        Expr::Shape(_) => "Shape",
        Expr::Next(_) => "Next",
        Expr::Eventually(_) => "Eventually",
    }
}

#[cfg(test)]
mod tests {
    use super::render_trace;
    use crate::nu::StructuralNuResult;
    use pen_core::telescope::Telescope;

    #[test]
    fn trace_output_is_stable_for_reference_telescope() {
        let telescope = Telescope::reference(4);
        let result = StructuralNuResult {
            nu_g: 2,
            nu_h: 0,
            nu_c: 3,
            total: 5,
            distributive_law_bonus: 0,
            universe_polymorphism_bonus: 0,
            infinitesimal_shift_bonus: 0,
        };

        let trace = render_trace(&telescope, &result);
        assert!(trace.contains("nu_total=5"));
        assert!(trace.contains("node=entry1|ctor=Lam"));
        assert!(trace.contains("node=entry2/0/0|ctor=Var"));
    }
}
