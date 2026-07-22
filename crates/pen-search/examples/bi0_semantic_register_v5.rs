use pen_search::bi0_semantic_register_v5::{
    emit_bi0_semantic_register_v5_create_new, issue_bi0_semantic_register_v5_certificate,
    replay_bi0_semantic_register_v5_directory,
};
use std::path::Path;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => match issue_bi0_semantic_register_v5_certificate() {
            Ok(certificate) => println!(
                "outcome={} T-BI={} B3={} F-SM1={} BI-0={} BI-1-invoked={} cone-invoked={} semantic={:?} digest={}",
                certificate.outcome,
                certificate.t_bi_side_passed,
                certificate.t_bi_intrinsic_isolation_theorem_id,
                certificate.f_sm1_fixed_positive_gate_passed,
                certificate.bi0_passed,
                certificate.bi1_invoked,
                certificate.non_enacted_cone_invoked,
                certificate.authoritative_semantic_register,
                certificate.result_digest,
            ),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        },
        [command, directory] if command == "create-new" => {
            match emit_bi0_semantic_register_v5_create_new(Path::new(directory)) {
                Ok(certificate) => println!(
                    "outcome={} T-BI={} B3={} F-SM1={} BI-0={} BI-1-invoked={} cone-invoked={} digest={}",
                    certificate.outcome,
                    certificate.t_bi_side_passed,
                    certificate.t_bi_intrinsic_isolation_theorem_id,
                    certificate.f_sm1_fixed_positive_gate_passed,
                    certificate.bi0_passed,
                    certificate.bi1_invoked,
                    certificate.non_enacted_cone_invoked,
                    certificate.result_digest,
                ),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        [command, directory] if command == "replay" => {
            match replay_bi0_semantic_register_v5_directory(Path::new(directory)) {
                Ok(replay) if replay.valid => {
                    println!("{}", serde_json::to_string_pretty(&replay).unwrap())
                }
                Ok(replay) => {
                    eprintln!("BI-0 v5 replay failed: {:?}", replay.errors);
                    std::process::exit(1);
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: bi0_semantic_register_v5 [create-new|replay DIRECTORY]");
            std::process::exit(2);
        }
    }
}
