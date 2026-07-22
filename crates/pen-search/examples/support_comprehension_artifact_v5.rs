use pen_search::support_comprehension_artifact_v5::{
    emit_support_comprehension_v5_create_new, replay_support_comprehension_v5_directory,
};
use std::path::Path;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, directory] if command == "create-new" => {
            match emit_support_comprehension_v5_create_new(Path::new(directory)) {
                Ok(certificate) => println!(
                    "outcome={} inherited={}/71 sealed={}/{} former={}/{} t_sm1b={}/{} BI-0-chrono={} digest={}",
                    certificate.outcome,
                    certificate.inherited_row_equality_count,
                    certificate.sealed_discharge_derived_count,
                    certificate.sealed_discharge_count,
                    certificate.former_derived_instance_ids.len(),
                    certificate.former_expected_instance_ids.len(),
                    certificate.t_sm1b_derived_count,
                    certificate.t_sm1b_surface_count,
                    certificate.bi0_chronological_prerequisite_reopened,
                    certificate.result_digest,
                ),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
        }
        [command, directory] if command == "replay" => {
            if let Err(error) = replay_support_comprehension_v5_directory(Path::new(directory)) {
                eprintln!("{error}");
                std::process::exit(1);
            }
            println!("support-comprehension v5 replay valid");
        }
        _ => {
            eprintln!("usage: support_comprehension_artifact_v5 [create-new|replay DIRECTORY]");
            std::process::exit(2);
        }
    }
}
