//! Mechanics artifact for the runtime calculus audit trail
//! (`docs/RUNTIME_CALCULUS.md` §8 corollary and §9 freeze-checklist item 3).
//!
//! Emits ν numbers ONLY: single-chain ν_k profiles (Stage-1 R1 dedup) and
//! field-schedule per-step ν lists across the disclosed search budgets (the
//! §9.1 sensitivity check). No engagement fractions, no ledger demand, no
//! crossing index — those belong to the Stage-2 blind run.
//!
//! Usage:
//!   cargo run -p pen-eval --example runtime_mechanics -- \
//!     --out docs/runtime_mechanics_profiles.json \
//!     [--levels 6] [--steps 4] [--budgets 64,256,1024]

use pen_eval::lambda_trigger_v2::frozen_structure_schema_variants_v2;
use pen_eval::runtime_dedup::nu_profile;
use pen_eval::runtime_field::{FieldPolicy, field_nu_profile, genesis_reference_context};
use serde_json::json;
use std::env;
use std::fs;
use std::path::PathBuf;

fn budget_label(budget: u64) -> String {
    if budget == u64::MAX {
        "exhaustive".to_owned()
    } else {
        budget.to_string()
    }
}

fn main() {
    let mut out = None;
    let mut levels: u32 = 6;
    let mut steps: u32 = 4;
    let mut budgets: Vec<u64> = vec![64, 256, 1024];

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => out = args.next().map(PathBuf::from),
            "--levels" => {
                levels = args
                    .next()
                    .expect("--levels needs a value")
                    .parse()
                    .expect("--levels must be a positive integer");
            }
            "--steps" => {
                steps = args
                    .next()
                    .expect("--steps needs a value")
                    .parse()
                    .expect("--steps must be a positive integer");
            }
            "--budgets" => {
                budgets = args
                    .next()
                    .expect("--budgets needs a comma-separated list")
                    .split(',')
                    .map(|budget| {
                        if budget == "exhaustive" {
                            // No cap: the per-round field is finite, so the
                            // stream exhausts and the budget ceases to be
                            // policy (order reduces to a tie-break).
                            u64::MAX
                        } else {
                            budget
                                .parse()
                                .expect("budgets must be integers or 'exhaustive'")
                        }
                    })
                    .collect();
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let out = out.expect("--out is required");

    let (library, history) = genesis_reference_context();

    let mut single_chain = serde_json::Map::new();
    let mut field_schedule = serde_json::Map::new();

    for (name, _, telescope) in frozen_structure_schema_variants_v2() {
        single_chain.insert(
            name.to_owned(),
            json!(nu_profile(&telescope, &library, &history, levels, true)),
        );

        let mut per_budget = serde_json::Map::new();
        for budget in &budgets {
            let policy = FieldPolicy {
                search_budget: *budget,
            };
            per_budget.insert(
                format!("budget_{}", budget_label(*budget)),
                json!(field_nu_profile(
                    &telescope, &library, &history, steps, &policy
                )),
            );
        }
        field_schedule.insert(name.to_owned(), serde_json::Value::Object(per_budget));
    }

    let budget_labels: Vec<String> = budgets.iter().map(|budget| budget_label(*budget)).collect();

    let artifact = json!({
        "artifact": "runtime_mechanics_profiles",
        "date": "2026-07-05",
        "design": "docs/RUNTIME_CALCULUS.md (Stage 1; §8 corollary, §9 checklist item 3)",
        "discipline": "nu numbers only — no engagement fractions, no ledger demand, no crossing index",
        "single_chain_levels": levels,
        "field_cadence_steps": steps,
        "field_search_budgets": budget_labels,
        "single_chain_nu_profiles": serde_json::Value::Object(single_chain),
        "field_schedule_nu_per_step": serde_json::Value::Object(field_schedule),
    });

    let payload = serde_json::to_string_pretty(&artifact).expect("serialize mechanics artifact");
    fs::write(&out, format!("{payload}\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));
    println!("mechanics artifact written to {}", out.display());
}
