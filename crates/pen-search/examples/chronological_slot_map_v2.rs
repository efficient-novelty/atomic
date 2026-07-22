use pen_search::chronological_slot_map_v2::{
    ChronologicalSlotMapV2Certificate, emit_chronological_slot_map_v2_create_new,
    replay_chronological_slot_map_v2_certificate,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, directory] if command == "create-new" => {
            let replay = emit_chronological_slot_map_v2_create_new(Path::new(directory))
                .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        }
        [command, json_path] if command == "replay" => {
            let certificate: ChronologicalSlotMapV2Certificate = serde_json::from_str(
                &std::fs::read_to_string(json_path).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let replay = replay_chronological_slot_map_v2_certificate(&certificate);
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
            if !replay.valid {
                return Err("v2 replay failed".to_owned());
            }
        }
        _ => {
            return Err(
                "usage: chronological_slot_map_v2 <create-new DIRECTORY|replay JSON>".to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("chronological-slot-map-v2-driver".to_owned())
        .stack_size(128 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("chronological slot-map v2 driver panicked".into()),
    }
}
