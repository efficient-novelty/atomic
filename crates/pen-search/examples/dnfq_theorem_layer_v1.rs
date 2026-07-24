use pen_search::dnfq_theorem_layer_v1::{
    emit_dnf_artifacts_create_new, issue_dnf1_canonical_contexts_v1,
    issue_dnf2_unified_judgments_v1, issue_dnf3_naturality_closure_v1,
    issue_dnf4_corpus_projection_v1, render_dnf1, render_dnf2, render_dnf3, render_dnf4,
    replay_dnf1_canonical_contexts_v1_json, replay_dnf2_unified_judgments_v1_json,
    replay_dnf3_naturality_closure_v1_json, replay_dnf4_corpus_projection_v1_json,
};
use pen_search::t_d2_1_operational_domain_v3::{
    emit_t_d2_1_operational_domain_v3_create_new, replay_t_d2_1_operational_domain_v3_json,
};
use std::fs;
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let dnf1 = issue_dnf1_canonical_contexts_v1().map_err(|error| error.to_string())?;
            let dnf2 = issue_dnf2_unified_judgments_v1().map_err(|error| error.to_string())?;
            let dnf3 = issue_dnf3_naturality_closure_v1().map_err(|error| error.to_string())?;
            let dnf4 = issue_dnf4_corpus_projection_v1().map_err(|error| error.to_string())?;
            print!("{}", render_dnf1(&dnf1));
            print!("\n{}", render_dnf2(&dnf2));
            print!("\n{}", render_dnf3(&dnf3));
            print!("\n{}", render_dnf4(&dnf4));
        }
        [command, docs] if command == "create-new" => {
            emit_dnf_artifacts_create_new(Path::new(docs)).map_err(|error| error.to_string())?;
            println!("DNF-1..DNF-4 create-new artifacts issued in {docs}");
        }
        [command, docs] if command == "create-new-all" => {
            let docs = Path::new(docs);
            emit_dnf_artifacts_create_new(docs).map_err(|error| error.to_string())?;
            emit_t_d2_1_operational_domain_v3_create_new(
                &docs.join("t_d2_1_operational_domain_v3.json"),
                &docs.join("T_D2_1_OPERATIONAL_DOMAIN_V3_RESULT.md"),
            )
            .map_err(|error| error.to_string())?;
            println!("DNF-1..DNF-4 and T-D2-1 v3 create-new artifacts issued");
        }
        [command, docs] if command == "replay" => {
            let inputs = [
                (
                    "DNF-1",
                    Path::new(docs).join("dnf1_canonical_contexts_v1.json"),
                ),
                (
                    "DNF-2",
                    Path::new(docs).join("dnf2_unified_judgments_v1.json"),
                ),
                (
                    "DNF-3",
                    Path::new(docs).join("dnf3_naturality_closure_v1.json"),
                ),
                (
                    "DNF-4",
                    Path::new(docs).join("dnf4_corpus_projection_v1.json"),
                ),
            ];
            for (index, (label, path)) in inputs.iter().enumerate() {
                let json = fs::read_to_string(path)
                    .map_err(|error| format!("could not read {}: {error}", path.display()))?;
                let replay = match index {
                    0 => replay_dnf1_canonical_contexts_v1_json(&json),
                    1 => replay_dnf2_unified_judgments_v1_json(&json),
                    2 => replay_dnf3_naturality_closure_v1_json(&json),
                    3 => replay_dnf4_corpus_projection_v1_json(&json),
                    _ => unreachable!(),
                };
                if !replay.valid {
                    return Err(format!(
                        "{label} replay failed: {}",
                        replay.errors.join("; ")
                    ));
                }
                println!(
                    "{label} replay valid; status={:?}; T-D2-2_reopened={}; M4_authorized={}",
                    replay.status, replay.t_d2_2_reopened, replay.m4_authorized
                );
            }
            let v3_path = Path::new(docs).join("t_d2_1_operational_domain_v3.json");
            let v3_json = fs::read_to_string(&v3_path)
                .map_err(|error| format!("could not read {}: {error}", v3_path.display()))?;
            let v3_replay = replay_t_d2_1_operational_domain_v3_json(&v3_json);
            if !v3_replay.valid {
                return Err(format!(
                    "T-D2-1 v3 replay failed: {}",
                    v3_replay.errors.join("; ")
                ));
            }
            println!(
                "T-D2-1 v3 replay valid; status={:?}; bounds_unchanged={}; T-D2-2_reopened={}; M4_authorized={}",
                v3_replay.status,
                v3_replay.frozen_bounds_unchanged,
                v3_replay.t_d2_2_prerequisite_satisfied,
                v3_replay.m4_authorized
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example dnfq_theorem_layer_v1 [create-new <docs-dir> | create-new-all <docs-dir> | replay <docs-dir>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("dnfq-theorem-layer-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("DNF theorem-layer worker panicked".into()),
    }
}
