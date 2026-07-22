mod naturality_orbit_transport {
    pub use pen_search::naturality_orbit_transport::*;
}

mod phase5b_history_certification {
    pub use pen_search::phase5b_history_certification::*;
}

#[path = "../src/e5_future_hole_finale_v2.rs"]
mod e5_future_hole_finale_v2;

use e5_future_hole_finale_v2::{
    E5FutureHoleFinaleV2Certificate, emit_e5_future_hole_finale_v2_create_new,
    replay_e5_future_hole_finale_v2_certificate,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "create-new" => {
            let replay = emit_e5_future_hole_finale_v2_create_new(Path::new(path))
                .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        }
        [command, path] if command == "replay" => {
            let certificate: E5FutureHoleFinaleV2Certificate = serde_json::from_str(
                &std::fs::read_to_string(path).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let replay = replay_e5_future_hole_finale_v2_certificate(&certificate);
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
            if !replay.valid {
                return Err("E-5 future-hole finale v2 replay failed".to_owned());
            }
        }
        _ => {
            return Err(
                "usage: schema2_e5_future_hole_finale_v2 <create-new|replay> <path>".to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("schema2-e5-future-hole-finale-v2-driver".to_owned())
        .stack_size(128 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("E-5 future-hole finale v2 driver panicked".into()),
    }
}
