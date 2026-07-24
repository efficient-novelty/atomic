use pen_search::t_d2_2_operational_surjectivity_v2::{
    emit_t_d2_2_operational_surjectivity_v2_create_new, issue_t_d2_2_operational_surjectivity_v2,
    render_t_d2_2_operational_surjectivity_v2, replay_t_d2_2_operational_surjectivity_v2,
    replay_t_d2_2_operational_surjectivity_v2_json,
};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => {
            let certificate =
                issue_t_d2_2_operational_surjectivity_v2().map_err(|error| error.to_string())?;
            let replay = replay_t_d2_2_operational_surjectivity_v2(&certificate);
            if !replay.valid {
                return Err(replay.errors.join("; "));
            }
            print!(
                "{}",
                render_t_d2_2_operational_surjectivity_v2(&certificate)
            );
            Ok(())
        }
        [command, certificate_path, report_path] if command == "create-new" => {
            let certificate = emit_t_d2_2_operational_surjectivity_v2_create_new(
                Path::new(certificate_path),
                Path::new(report_path),
            )
            .map_err(|error| error.to_string())?;
            print!(
                "{}",
                render_t_d2_2_operational_surjectivity_v2(&certificate)
            );
            Ok(())
        }
        [command, certificate_path] if command == "replay" => {
            let json = read_to_string(certificate_path).map_err(|error| error.to_string())?;
            let replay = replay_t_d2_2_operational_surjectivity_v2_json(&json);
            if !replay.valid {
                return Err(replay.errors.join("; "));
            }
            println!(
                "T-D2-2 v2 replay valid; status={:?}; disposition={:?}; surjectivity_proved={}; refutation_issued={}; m4_authorized={}",
                replay.status,
                replay.theorem_disposition,
                replay.surjectivity_proved,
                replay.refutation_issued,
                replay.m4_authorized,
            );
            Ok(())
        }
        _ => Err(
            "usage: t_d2_2_operational_surjectivity_v2 [create-new <certificate.json> <report.md> | replay <certificate.json>]"
                .to_owned(),
        ),
    }
}

fn main() {
    let worker = std::thread::Builder::new()
        .name("t-d2-2-v2-worker".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)
        .expect("T-D2-2 v2 worker spawns");
    match worker.join() {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("T-D2-2 v2 worker panicked");
            std::process::exit(1);
        }
    }
}
