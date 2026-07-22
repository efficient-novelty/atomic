use pen_search::phase5b_reselection_v3::{
    Phase5bReselectionV3Burn, Phase5bReselectionV3Program,
    emit_phase5b_reselection_v3_burn_create_new, emit_phase5b_reselection_v3_program_create_new,
    replay_phase5b_reselection_v3_burn, replay_phase5b_reselection_v3_program,
};
use std::path::Path;

fn read_program(path: &str) -> Result<Phase5bReselectionV3Program, String> {
    serde_json::from_str(&std::fs::read_to_string(path).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "preregister" => {
            emit_phase5b_reselection_v3_program_create_new(Path::new(path))
                .map_err(|error| error.to_string())?;
            println!("preregistered");
        }
        [command, path] if command == "replay-program" => {
            let program = read_program(path)?;
            let errors = replay_phase5b_reselection_v3_program(&program);
            println!("{}", serde_json::to_string_pretty(&errors).unwrap());
            if !errors.is_empty() {
                return Err("program replay failed".to_owned());
            }
        }
        [command, program_path, output_path] if command == "burn" => {
            let program = read_program(program_path)?;
            let replay =
                emit_phase5b_reselection_v3_burn_create_new(&program, Path::new(output_path))
                    .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        }
        [command, program_path, burn_path] if command == "replay-burn" => {
            let program = read_program(program_path)?;
            let burn: Phase5bReselectionV3Burn = serde_json::from_str(
                &std::fs::read_to_string(burn_path).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let replay = replay_phase5b_reselection_v3_burn(&program, &burn);
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
            if !replay.valid {
                return Err("burn replay failed".to_owned());
            }
        }
        _ => {
            return Err("usage: phase5b_reselection_v3 <preregister|replay-program> <path> | <burn|replay-burn> <program> <burn>".to_owned());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("phase5b-reselection-v3-driver".to_owned())
        .stack_size(128 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("Phase-5b reselection v3 driver panicked".into()),
    }
}
