use pen_core::expr::Expr;
use pen_core::telescope::Telescope;

pub fn step_module_name(step_index: u32) -> String {
    format!("Step{step_index:02}")
}

pub fn step_source_file(step_index: u32) -> String {
    format!("{}.agda", step_module_name(step_index))
}

pub fn step_verify_log_file(step_index: u32) -> String {
    format!("{}.verify.log", step_module_name(step_index))
}

pub fn render_step_module(
    step_index: u32,
    label: &str,
    telescope: &Telescope,
    candidate_hash: &str,
    canonical_hash: &str,
) -> String {
    let module_name = step_module_name(step_index);
    let imports = render_imports(telescope);
    let clauses = telescope
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| {
            let clause_name = format!("clause{:02}", index + 1);
            let translated = expr_to_agda_comment(&clause.expr);
            let mbtt = serde_json::to_string(&clause.expr).expect("expr should serialize");
            format!("  {clause_name} : Set\n    -- translated: {translated}\n    -- mbtt: {mbtt}")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let imports_block = if imports.is_empty() {
        String::new()
    } else {
        format!("{}\n\n", imports.join("\n"))
    };

    format!(
        "module {module_name} where\n\nopen import Agda.Primitive using (Set)\n\n{imports_block}-- step: {step_index}\n-- label: {label}\n-- candidate_hash: {candidate_hash}\n-- canonical_hash: {canonical_hash}\n\npostulate\n  T : Set\n{clauses}\n"
    )
}

fn render_imports(telescope: &Telescope) -> Vec<String> {
    telescope
        .lib_refs()
        .into_iter()
        .map(|step| {
            format!(
                "open import {} as {}",
                step_module_name(step),
                step_module_name(step)
            )
        })
        .collect()
}

pub fn expr_to_agda_comment(expr: &Expr) -> String {
    expr_to_agda_with_ctx(&[], expr)
}

fn expr_to_agda_with_ctx(ctx: &[String], expr: &Expr) -> String {
    match expr {
        Expr::App(function, argument) => format!(
            "({} {})",
            expr_to_agda_with_ctx(ctx, function),
            expr_to_agda_with_ctx(ctx, argument)
        ),
        Expr::Lam(body) => {
            let binder = fresh(ctx);
            let mut next_ctx = vec![binder.clone()];
            next_ctx.extend_from_slice(ctx);
            format!("(λ {binder} -> {})", expr_to_agda_with_ctx(&next_ctx, body))
        }
        Expr::Pi(domain, codomain) => {
            let binder = fresh(ctx);
            let mut next_ctx = vec![binder.clone()];
            next_ctx.extend_from_slice(ctx);
            format!(
                "(({binder} : {}) -> {})",
                expr_to_agda_with_ctx(ctx, domain),
                expr_to_agda_with_ctx(&next_ctx, codomain)
            )
        }
        Expr::Sigma(domain, codomain) => {
            let binder = fresh(ctx);
            let mut next_ctx = vec![binder.clone()];
            next_ctx.extend_from_slice(ctx);
            format!(
                "(Sigma {} (λ {binder} -> {}))",
                expr_to_agda_with_ctx(ctx, domain),
                expr_to_agda_with_ctx(&next_ctx, codomain)
            )
        }
        Expr::Univ => "Set".to_owned(),
        Expr::Var(index) => lookup_var(ctx, *index),
        Expr::Lib(index) => format!("{}.T", step_module_name(*index)),
        Expr::Id(ty, left, right) => format!(
            "(Id {} {} {})",
            expr_to_agda_with_ctx(ctx, ty),
            expr_to_agda_with_ctx(ctx, left),
            expr_to_agda_with_ctx(ctx, right)
        ),
        Expr::Refl(body) => format!("(Refl {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::Susp(body) => format!("(Susp {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::Trunc(body) => format!("(Trunc0 {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::PathCon(dimension) => format!("(PathCon {dimension})"),
        Expr::Flat(body) => format!("(Flat {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::Sharp(body) => format!("(Sharp {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::Disc(body) => format!("(Disc {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::Shape(body) => format!("(Shape {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::Next(body) => format!("(Next {})", expr_to_agda_with_ctx(ctx, body)),
        Expr::Eventually(body) => format!("(Eventually {})", expr_to_agda_with_ctx(ctx, body)),
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
    use super::{expr_to_agda_comment, render_step_module, step_module_name};
    use pen_core::expr::Expr;
    use pen_core::telescope::Telescope;

    #[test]
    fn renderer_uses_stable_step_module_names() {
        assert_eq!(step_module_name(5), "Step05");
    }

    #[test]
    fn renderer_emits_useful_comments_and_imports() {
        let source = render_step_module(9, "Hopf", &Telescope::reference(9), "a", "b");
        assert!(source.contains("module Step09 where"));
        assert!(source.contains("open import Step08 as Step08"));
        assert!(source.contains("-- translated:"));
    }

    #[test]
    fn expression_comments_are_human_readable() {
        let expr = Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(10)));
        assert!(expr_to_agda_comment(&expr).contains("Step10.T"));
    }
}
