use pen_search::stage4_semantic_parsimony_v2::{
    STAGE4_SEMANTIC_PARSIMONY_V2_CERTIFICATE_NAME, Stage4SemanticParsimonyV2Certificate,
};
use pen_search::stage4_semantic_parsimony_v3::{
    emit_stage4_semantic_parsimony_v3_create_new, issue_stage4_semantic_parsimony_v3,
    replay_stage4_semantic_parsimony_v3_directory,
};
use std::path::Path;

fn read_v2_claim(path: &Path) -> Result<Stage4SemanticParsimonyV2Certificate, String> {
    let json = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    serde_json::from_str(&json)
        .map_err(|error| format!("failed to deserialize {}: {error}", path.display()))
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => match read_v2_claim(
            Path::new("docs")
                .join(STAGE4_SEMANTIC_PARSIMONY_V2_CERTIFICATE_NAME)
                .as_path(),
        )
        .and_then(|claim| {
            issue_stage4_semantic_parsimony_v3(&claim).map_err(|error| error.to_string())
        }) {
            Ok(certificate) => println!(
                "outcome={} v2-replayed={} cross-bound={} cone-authorized={} branch-authorized={} divergence={} digest={}",
                certificate.outcome,
                certificate.v2_claim_fully_replayed_before_cross_binding,
                certificate
                    .cross_binding
                    .exact_bi0_to_blind_preseal_cross_binding_proved,
                certificate.authorized_stage4_cone_audit,
                certificate.authorized_bi1_branch_execution,
                certificate.divergence_requires_versioned_adjudication,
                certificate.result_digest,
            ),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        },
        [command, directory] if command == "create-new" => {
            let directory = Path::new(directory);
            let v2_path = directory.join(STAGE4_SEMANTIC_PARSIMONY_V2_CERTIFICATE_NAME);
            match read_v2_claim(&v2_path).and_then(|claim| {
                emit_stage4_semantic_parsimony_v3_create_new(&claim, directory)
                    .map_err(|error| error.to_string())
            }) {
                Ok(certificate) => println!(
                    "outcome={} cone-authorized={} branch-authorized={} divergence={} digest={}",
                    certificate.outcome,
                    certificate.authorized_stage4_cone_audit,
                    certificate.authorized_bi1_branch_execution,
                    certificate.divergence_requires_versioned_adjudication,
                    certificate.result_digest,
                ),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        [command, directory] if command == "replay" => {
            match replay_stage4_semantic_parsimony_v3_directory(Path::new(directory)) {
                Ok(replay) if replay.valid => {
                    println!("{}", serde_json::to_string_pretty(&replay).unwrap())
                }
                Ok(replay) => {
                    eprintln!("Stage-4 v3 replay failed: {:?}", replay.errors);
                    std::process::exit(1);
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: stage4_semantic_parsimony_v3 [create-new|replay DIRECTORY]");
            std::process::exit(2);
        }
    }
}
