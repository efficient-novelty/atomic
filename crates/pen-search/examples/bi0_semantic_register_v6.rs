use pen_search::bi0_semantic_register_v6::{
    emit_bi0_semantic_register_v6_create_new, issue_bi0_semantic_register_v6_artifact,
    replay_bi0_semantic_register_v6_directory,
};
use std::path::Path;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => match issue_bi0_semantic_register_v6_artifact() {
            Ok(artifact) => println!(
                "outcome={} T-BI={} operational={} F-AL1-prime={} F-SM1={}/{}/{} BI-0={} cone={} branch={} opening={} digest={}",
                artifact.certificate.outcome,
                artifact
                    .certificate
                    .t_bi_complete_act_local_semantic_certificate,
                artifact.certificate.t_bi_exact_operational_regression,
                artifact.certificate.t_bi_f_al1_prime_passed,
                artifact.certificate.chronological_exact_72_of_72,
                artifact.certificate.chronological_exact_9_of_9,
                artifact.certificate.chronological_exact_18_of_18,
                artifact.certificate.bi0_passed,
                artifact.certificate.cone_enumerated,
                artifact.certificate.non_enacted_branch_executed,
                artifact.stage4_opening_capability.derivation_hash(),
                artifact.result_digest,
            ),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        },
        [command, directory] if command == "create-new" => {
            match emit_bi0_semantic_register_v6_create_new(Path::new(directory)) {
                Ok(artifact) => println!(
                    "outcome={} BI-0={} cone={} branch={} opening={} digest={}",
                    artifact.certificate.outcome,
                    artifact.certificate.bi0_passed,
                    artifact.certificate.cone_enumerated,
                    artifact.certificate.non_enacted_branch_executed,
                    artifact.stage4_opening_capability.derivation_hash(),
                    artifact.result_digest,
                ),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        [command, directory] if command == "replay" => {
            match replay_bi0_semantic_register_v6_directory(Path::new(directory)) {
                Ok(replay) if replay.valid => {
                    println!("{}", serde_json::to_string_pretty(&replay).unwrap())
                }
                Ok(replay) => {
                    eprintln!("BI-0 v6 replay failed: {:?}", replay.errors);
                    std::process::exit(1);
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: bi0_semantic_register_v6 [create-new|replay DIRECTORY]");
            std::process::exit(2);
        }
    }
}
