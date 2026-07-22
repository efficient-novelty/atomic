pub mod naturality_orbit_transport {
    pub use pen_search::naturality_orbit_transport::*;
}

pub mod t_bf1_prefix {
    pub use pen_search::t_bf1_prefix::*;
}

pub mod t_bf_tie_protocol_rerun {
    pub use pen_search::t_bf_tie_protocol_rerun::*;
}

#[path = "../src/r_t2_future_hole_confluence.rs"]
mod r_t2_future_hole_confluence;

use r_t2_future_hole_confluence::{
    Rt2FutureHoleConfluenceCertificate, emit_r_t2_future_hole_confluence_create_new,
    replay_r_t2_future_hole_confluence_certificate,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "create-new" => {
            let replay = emit_r_t2_future_hole_confluence_create_new(Path::new(path))
                .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        }
        [command, path] if command == "replay" => {
            let certificate: Rt2FutureHoleConfluenceCertificate = serde_json::from_str(
                &std::fs::read_to_string(path).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let replay = replay_r_t2_future_hole_confluence_certificate(&certificate);
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
            if !replay.valid {
                return Err("R-T2 future-hole confluence replay failed".to_owned());
            }
        }
        _ => {
            return Err(
                "usage: r_t2_future_hole_confluence <create-new|replay> <artifact.json>".to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("r-t2-future-hole-confluence-driver".to_owned())
        .stack_size(256 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("R-T2 future-hole confluence driver panicked".into()),
    }
}
