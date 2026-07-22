use pen_search::global_e4_assembly_v11::{
    emit_global_e4_v11_create_new, issue_global_e4_v11_certificate, replay_global_e4_v11_json,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: schema2_global_e4_assembly_v11 <issue|emit|replay> [path]")?;
    match command.as_str() {
        "issue" => {
            if args.next().is_some() {
                return Err("usage: schema2_global_e4_assembly_v11 issue".to_owned());
            }
            let certificate = issue_global_e4_v11_certificate().map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&certificate).map_err(|e| e.to_string())?
            );
        }
        "emit" => {
            let path = args
                .next()
                .ok_or("usage: schema2_global_e4_assembly_v11 emit <path>")?;
            if args.next().is_some() {
                return Err("usage: schema2_global_e4_assembly_v11 emit <path>".to_owned());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &emit_global_e4_v11_create_new(Path::new(&path)).map_err(|e| e.to_string())?
                )
                .map_err(|e| e.to_string())?
            );
        }
        "replay" => {
            let path = args
                .next()
                .ok_or("usage: schema2_global_e4_assembly_v11 replay <path>")?;
            if args.next().is_some() {
                return Err("usage: schema2_global_e4_assembly_v11 replay <path>".to_owned());
            }
            let replay = replay_global_e4_v11_json(
                &std::fs::read_to_string(path).map_err(|e| e.to_string())?,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&replay).map_err(|e| e.to_string())?
            );
            if !replay.valid {
                return Err("global E-4 v11 replay failed".to_owned());
            }
        }
        _ => {
            return Err(
                "usage: schema2_global_e4_assembly_v11 <issue|emit|replay> [path]".to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("global-e4-v11-driver".to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("global E-4 v11 driver panicked".into()),
    }
}
