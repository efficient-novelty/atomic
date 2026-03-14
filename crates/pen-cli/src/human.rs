use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::telescope::{Telescope, TelescopeClass};

pub fn step_label(step_index: u32) -> &'static str {
    match step_index {
        1 => "Universe",
        2 => "Unit",
        3 => "Witness",
        4 => "Pi",
        5 => "S1",
        6 => "Trunc",
        7 => "S2",
        8 => "S3",
        9 => "Hopf",
        10 => "Cohesion",
        11 => "Connections",
        12 => "Curvature",
        13 => "Metric",
        14 => "Hilbert",
        15 => "DCT",
        _ => "Unknown",
    }
}

pub fn translation_guide() -> [&'static str; 4] {
    [
        "Universe, Pi, Sigma, Trunc, Flat, Sharp, Disc, Shape, Next, Eventually, and Path^d are ASCII renderings of the MBTT constructors.",
        "x1, x2, ... are bound variables in de Bruijn order, with x1 the nearest enclosing binder.",
        "StepNN (Label) names an earlier accepted telescope when a candidate imports prior structure.",
        "Each clause line shows the clause role first, then the translated expression.",
    ]
}

pub fn library_refs(telescope: &Telescope) -> Vec<String> {
    telescope
        .lib_refs()
        .into_iter()
        .map(step_ref_name)
        .collect()
}

pub fn clause_lines(telescope: &Telescope) -> Vec<String> {
    telescope
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| format!("c{:02} [{}] {}", index + 1, role_name(clause), render_expr(&clause.expr)))
        .collect()
}

pub fn describe_candidate(telescope: &Telescope, telescope_class: TelescopeClass) -> String {
    let mut details = Vec::new();
    let refs = library_refs(telescope);
    if refs.is_empty() {
        details.push("uses only the current local scope".to_owned());
    } else {
        details.push(format!("reuses {}", refs.join(", ")));
    }

    let path_dims = telescope.path_dimensions();
    if !path_dims.is_empty() {
        let dims = path_dims
            .iter()
            .map(|dimension| format!("Path^{dimension}"))
            .collect::<Vec<_>>()
            .join(", ");
        details.push(format!("contains {dims}"));
    }

    if telescope
        .clauses
        .iter()
        .any(|clause| clause.expr.is_modal())
    {
        details.push("contains modal structure".to_owned());
    }
    if telescope
        .clauses
        .iter()
        .any(|clause| clause.expr.is_temporal())
    {
        details.push("contains temporal structure".to_owned());
    }

    format!(
        "{} candidate with {} clauses; {}.",
        class_name(telescope_class),
        telescope.kappa(),
        details.join("; ")
    )
}

pub fn render_expr(expr: &Expr) -> String {
    render_expr_with_ctx(&[], expr)
}

fn render_expr_with_ctx(ctx: &[String], expr: &Expr) -> String {
    match expr {
        Expr::App(function, argument) => format!(
            "({} {})",
            render_expr_with_ctx(ctx, function),
            render_expr_with_ctx(ctx, argument)
        ),
        Expr::Lam(body) => {
            let binder = fresh(ctx);
            let mut next_ctx = vec![binder.clone()];
            next_ctx.extend_from_slice(ctx);
            format!("(fun {binder} -> {})", render_expr_with_ctx(&next_ctx, body))
        }
        Expr::Pi(domain, codomain) => {
            let binder = fresh(ctx);
            let mut next_ctx = vec![binder.clone()];
            next_ctx.extend_from_slice(ctx);
            format!(
                "(({binder} : {}) -> {})",
                render_expr_with_ctx(ctx, domain),
                render_expr_with_ctx(&next_ctx, codomain)
            )
        }
        Expr::Sigma(domain, codomain) => {
            let binder = fresh(ctx);
            let mut next_ctx = vec![binder.clone()];
            next_ctx.extend_from_slice(ctx);
            format!(
                "(Sigma {} (fun {binder} -> {}))",
                render_expr_with_ctx(ctx, domain),
                render_expr_with_ctx(&next_ctx, codomain)
            )
        }
        Expr::Univ => "Universe".to_owned(),
        Expr::Var(index) => lookup_var(ctx, *index),
        Expr::Lib(index) => step_ref_name(*index),
        Expr::Id(ty, left, right) => format!(
            "(Id {} {} {})",
            render_expr_with_ctx(ctx, ty),
            render_expr_with_ctx(ctx, left),
            render_expr_with_ctx(ctx, right)
        ),
        Expr::Refl(body) => format!("(Refl {})", render_expr_with_ctx(ctx, body)),
        Expr::Susp(body) => format!("(Susp {})", render_expr_with_ctx(ctx, body)),
        Expr::Trunc(body) => format!("(Trunc {})", render_expr_with_ctx(ctx, body)),
        Expr::PathCon(dimension) => format!("Path^{dimension}"),
        Expr::Flat(body) => format!("(Flat {})", render_expr_with_ctx(ctx, body)),
        Expr::Sharp(body) => format!("(Sharp {})", render_expr_with_ctx(ctx, body)),
        Expr::Disc(body) => format!("(Disc {})", render_expr_with_ctx(ctx, body)),
        Expr::Shape(body) => format!("(Shape {})", render_expr_with_ctx(ctx, body)),
        Expr::Next(body) => format!("(Next {})", render_expr_with_ctx(ctx, body)),
        Expr::Eventually(body) => format!("(Eventually {})", render_expr_with_ctx(ctx, body)),
    }
}

fn role_name(clause: &ClauseRec) -> &'static str {
    match clause.role {
        ClauseRole::Formation => "formation",
        ClauseRole::Introduction => "introduction",
        ClauseRole::Elimination => "elimination",
        ClauseRole::PathAttach => "path",
        ClauseRole::Computation => "computation",
    }
}

fn class_name(telescope_class: TelescopeClass) -> &'static str {
    match telescope_class {
        TelescopeClass::Foundation => "Foundation",
        TelescopeClass::Former => "Former",
        TelescopeClass::Hit => "Higher-inductive",
        TelescopeClass::Suspension => "Suspension",
        TelescopeClass::Map => "Map",
        TelescopeClass::Modal => "Modal",
        TelescopeClass::Axiomatic => "Axiomatic",
        TelescopeClass::Synthesis => "Synthesis",
        TelescopeClass::Unknown => "Unknown-shape",
    }
}

fn step_ref_name(step_index: u32) -> String {
    let label = step_label(step_index);
    if label == "Unknown" {
        format!("Step{step_index:02}")
    } else {
        format!("Step{step_index:02} ({label})")
    }
}

fn lookup_var(ctx: &[String], index: u32) -> String {
    let zero_based = usize::try_from(index.saturating_sub(1)).expect("index should fit usize");
    ctx.get(zero_based)
        .cloned()
        .unwrap_or_else(|| format!("x{index}"))
}

fn fresh(ctx: &[String]) -> String {
    (1..)
        .map(|index| format!("x{index}"))
        .find(|candidate| !ctx.contains(candidate))
        .expect("fresh variable name should exist")
}

#[cfg(test)]
mod tests {
    use super::{describe_candidate, render_expr, step_label};
    use pen_core::expr::Expr;
    use pen_core::telescope::{Telescope, TelescopeClass};

    #[test]
    fn step_labels_match_reference_names() {
        assert_eq!(step_label(9), "Hopf");
    }

    #[test]
    fn expression_renderer_exposes_prior_step_labels() {
        let expr = Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(10)));
        let rendered = render_expr(&expr);
        assert!(rendered.contains("Step10 (Cohesion)"));
        assert!(rendered.contains("x1"));
    }

    #[test]
    fn candidate_summary_mentions_structure_kind_and_imports() {
        let summary = describe_candidate(&Telescope::reference(9), TelescopeClass::Axiomatic);
        assert!(summary.contains("Axiomatic candidate"));
        assert!(summary.contains("Step08 (S3)"));
    }
}
