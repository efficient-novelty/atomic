use pen_search::t_bi_nu1_regression_v6::{
    TBiNu1RegressionV6Certificate, TBiNu1RegressionV6Error, emit_t_bi_nu1_regression_v6_create_new,
    issue_t_bi_nu1_regression_v6_certificate, replay_t_bi_nu1_regression_v6_directory,
};
use std::path::Path;

fn print_summary(certificate: &TBiNu1RegressionV6Certificate) {
    println!(
        "outcome={} packages={} receipts={} roles={} proved={} impossible={} residuals(role/quotient/A3/total/silent)={}/{}/{}/{}/{} B3={} semantic={:?} F-AL1-prime={} BI-0-authorized={} digest={}",
        certificate.outcome,
        certificate.package_count,
        certificate.intrinsic_isolation.issuance_receipt_count,
        certificate.role_declaration_count,
        certificate.proved_family_declaration_count,
        certificate.theorem_impossibility_declaration_count,
        certificate.named_registry_residual_count,
        certificate.named_quotient_residual_count,
        certificate.named_a3_residual_count,
        certificate.total_named_residual_count,
        certificate.silent_residue_count,
        certificate.intrinsic_isolation.theorem_id,
        certificate.authoritative_semantic_register,
        certificate.f_al1_prime_passed,
        certificate.bi0_rerun_authorized_by_t_bi_nu1_side,
        certificate.result_digest,
    );
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let result = match args.as_slice() {
        [] => issue_t_bi_nu1_regression_v6_certificate().map(|certificate| {
            print_summary(&certificate);
        }),
        [command, directory] if command == "create-new" => {
            emit_t_bi_nu1_regression_v6_create_new(Path::new(directory)).map(|certificate| {
                print_summary(&certificate);
            })
        }
        [command, directory] if command == "replay" => {
            match replay_t_bi_nu1_regression_v6_directory(Path::new(directory)) {
                Ok(replay) => {
                    println!("{}", serde_json::to_string_pretty(&replay).unwrap());
                    if replay.valid {
                        Ok(())
                    } else {
                        Err(TBiNu1RegressionV6Error::EmittedReplay(
                            replay.errors.join("; "),
                        ))
                    }
                }
                Err(error) => Err(error),
            }
        }
        _ => {
            eprintln!("usage: t_bi_nu1_regression_v6 [create-new|replay DIRECTORY]");
            std::process::exit(2);
        }
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
