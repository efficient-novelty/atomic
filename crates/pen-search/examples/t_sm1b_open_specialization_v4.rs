//! Read-only development harness for T-SM1b.
//!
//! This example intentionally emits no certificate.  The positive F-SM1
//! artifact remains gated on the full 72/72 theorem sweep.

#[path = "../src/motive_typed_open_specialization_v4.rs"]
mod motive_typed_open_specialization_v4;

use pen_eval::a3_demand_grammar::{
    A3DemandOutputType, A3RuleConstructor, generate_a3_window_for_exact_prefix_unbounded,
};
use pen_search::contextual_formation_coherence_v3::kernel_context_from_parameter_sorts;
use pen_core::clause::ClauseRec;
use pen_core::telescope::Telescope;
use pen_type::contextual_internality::{
    ContextualMotive, issue_ambient_context_declaration_token,
    issue_explicit_ambient_context_declaration_token,
};
use pen_type::dependent_context::{
    DependentContextMotive, issue_dependent_ambient_context_declaration,
    issue_dependent_total_specialization_theorem,
};
use pen_type::elaborate::SealedSignature;
use pen_type::motive_parametric_coherence_v2::{
    issue_actual_body_closure_derivation_v2, issue_explicit_contextual_closure_derivation_v2,
};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
struct OpenSourceDiagnostic {
    instance_id: String,
    older_clause: u16,
    older_expression: pen_core::expr::Expr,
    older_kernel_type: pen_type::elaborate::KernelTy,
    newest_clause: u16,
    source_context: Vec<pen_type::elaborate::KernelTy>,
    target_context: Vec<pen_type::elaborate::KernelTy>,
    newest_expression: pen_core::expr::Expr,
    newest_kernel_type: pen_type::elaborate::KernelTy,
    inferred_v2_attempt: String,
    explicit_v2_attempt: String,
    dependent_declaration_attempt: String,
    dependent_totality_attempt: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("audit") {
        let audit =
            motive_typed_open_specialization_v4::issue_t_sm1b_corpus_audit_v4()?;
        println!("{}", serde_json::to_string_pretty(&audit)?);
        if audit.positive_artifact_permitted {
            return Err("positive artifact emission is intentionally not implemented here".into());
        }
        return Ok(());
    }
    let full = SealedSignature::genesis_del_h15();
    let window = generate_a3_window_for_exact_prefix_unbounded(&full, 16)?;
    let mut rows = Vec::new();
    for instance in &window.instances {
        let Some(scheme) = window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
        else {
            return Err("orphan A3 instance".into());
        };
        if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
            continue;
        }
        let A3DemandOutputType::ChronologicalInteraction { .. } = &scheme.required_output else {
            continue;
        };
        let older = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[0])
            .ok_or("missing older source")?;
        let newest = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[1])
            .ok_or("missing newest source")?;
        if newest.step != 15 || !matches!(newest.clause_index, 3 | 6) {
            continue;
        }
        let source_context =
            kernel_context_from_parameter_sorts(&newest.canonical_presentation.parameters);
        let target_arity = older
            .canonical_presentation
            .parameters
            .len()
            .max(newest.canonical_presentation.parameters.len());
        let target_parameters = (0..target_arity)
            .map(|index| {
                older
                    .canonical_presentation
                    .parameters
                    .get(index)
                    .or_else(|| newest.canonical_presentation.parameters.get(index))
                    .cloned()
                    .ok_or("target parameter hole")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let body = Telescope::new(vec![ClauseRec::new(
            newest.kernel_role,
            newest.canonical_presentation.canonical_normal_form.clone(),
        )]);
        let motives = source_context
            .iter()
            .map(|ty| match ty {
                pen_type::elaborate::KernelTy::Type => ContextualMotive::Type,
                pen_type::elaborate::KernelTy::El(term) => {
                    ContextualMotive::Element(term.clone())
                }
                _ => ContextualMotive::Neutral,
            })
            .collect::<Vec<_>>();
        let inferred_v2_attempt = issue_ambient_context_declaration_token(
            &full,
            &body,
            15,
            motives.clone(),
        )
        .map_err(|error| error.to_string())
        .and_then(|declaration| {
            issue_actual_body_closure_derivation_v2(
                &full,
                &body,
                15,
                0,
                Some(&declaration),
                &BTreeMap::new(),
            )
            .map(|token| format!("ok:{}", token.derivation_hash()))
            .map_err(|error| error.to_string())
        })
        .unwrap_or_else(|error| format!("error:{error}"));
        let explicit_v2_attempt = issue_explicit_ambient_context_declaration_token(
            &full,
            &body,
            15,
            motives,
        )
        .map_err(|error| error.to_string())
        .and_then(|declaration| {
            issue_explicit_contextual_closure_derivation_v2(&full, &declaration)
                .map(|token| format!("ok:{}", token.derivation_hash()))
                .map_err(|error| error.to_string())
        })
        .unwrap_or_else(|error| format!("error:{error}"));
        let dependent_motives = if newest.clause_index == 6 {
            vec![
                DependentContextMotive::Independent {
                    motive: ContextualMotive::Type,
                },
                DependentContextMotive::ElementOfApplicationHead {
                    head: pen_core::expr::Expr::Eventually(Box::new(
                        pen_core::expr::Expr::Var(1),
                    )),
                },
            ]
        } else {
            vec![DependentContextMotive::Independent {
                motive: ContextualMotive::Type,
            }]
        };
        let dependent = issue_dependent_ambient_context_declaration(
            &full,
            &body,
            15,
            dependent_motives,
        );
        let dependent_declaration_attempt = dependent
            .as_ref()
            .map(|token| format!("ok:{}", token.projection().declaration_hash))
            .unwrap_or_else(|error| format!("error:{error}"));
        let dependent_totality_attempt = dependent
            .and_then(|declaration| {
                issue_dependent_total_specialization_theorem(&full, &declaration)
            })
            .map(|token| format!("ok:{}", token.projection().theorem_hash))
            .unwrap_or_else(|error| format!("error:{error}"));
        rows.push(OpenSourceDiagnostic {
            instance_id: instance.instance_id.clone(),
            older_clause: older.clause_index,
            older_expression: older.canonical_presentation.canonical_normal_form.clone(),
            older_kernel_type: older.kernel_type.clone(),
            newest_clause: newest.clause_index,
            source_context,
            target_context: kernel_context_from_parameter_sorts(&target_parameters),
            newest_expression: newest.canonical_presentation.canonical_normal_form.clone(),
            newest_kernel_type: newest.kernel_type.clone(),
            inferred_v2_attempt,
            explicit_v2_attempt,
            dependent_declaration_attempt,
            dependent_totality_attempt,
        });
    }
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
