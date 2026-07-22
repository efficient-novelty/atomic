mod act_local_provenance_v3 {
    pub use pen_search::act_local_provenance_v3::*;
}
mod phase5b_history_certification {
    pub use pen_search::phase5b_history_certification::*;
}
mod phase5b_reselection_v2 {
    pub use pen_search::phase5b_reselection_v2::*;
}
mod phase5b_reselection_v3 {
    pub use pen_search::phase5b_reselection_v3::*;
}

#[path = "../src/act_local_semantic_provenance_v4.rs"]
mod act_local_semantic_provenance_v4;
#[path = "../src/t_bi_nu1_regression_v4.rs"]
mod t_bi_nu1_regression_v4;

use std::path::Path;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if let [command, directory] = args.as_slice() {
        match command.as_str() {
            "create-new" => match t_bi_nu1_regression_v4::emit_t_bi_nu1_regression_v4_create_new(
                Path::new(directory),
            ) {
                Ok(certificate) => {
                    println!(
                        "outcome={} resolved={} impossible={} named_residual={} non_authoritative_extraction_floor={:?} operational={} F-AL1-prime={} digest={}",
                        certificate.outcome,
                        certificate.resolved_role_declaration_count,
                        certificate.theorem_impossibility_declaration_count,
                        certificate.named_registry_residual_count,
                        certificate.registers.non_authoritative_extraction_floor,
                        certificate.operational.exact_operational_regression,
                        certificate.f_al1_prime_passed,
                        certificate.result_digest,
                    );
                    return;
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            },
            "replay" => match t_bi_nu1_regression_v4::replay_t_bi_nu1_regression_v4_directory(
                Path::new(directory),
            ) {
                Ok(replay) => {
                    println!("{}", serde_json::to_string_pretty(&replay).unwrap());
                    if replay.valid {
                        return;
                    }
                    std::process::exit(1);
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            },
            _ => {
                eprintln!("usage: t_bi_nu1_regression_v4 [create-new|replay DIRECTORY]");
                std::process::exit(2);
            }
        }
    }
    if !args.is_empty() {
        eprintln!("usage: t_bi_nu1_regression_v4 [create-new|replay DIRECTORY]");
        std::process::exit(2);
    }
    match t_bi_nu1_regression_v4::issue_t_bi_nu1_regression_v4_certificate() {
        Ok(certificate) => {
            println!(
                "outcome={} resolved={} impossible={} named_residual={} non_authoritative_extraction_floor={:?} operational={} F-AL1-prime={} digest={}",
                certificate.outcome,
                certificate.resolved_role_declaration_count,
                certificate.theorem_impossibility_declaration_count,
                certificate.named_registry_residual_count,
                certificate.registers.non_authoritative_extraction_floor,
                certificate.operational.exact_operational_regression,
                certificate.f_al1_prime_passed,
                certificate.result_digest,
            );
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
