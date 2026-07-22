use pen_search::phase5b_history_certification::{
    emit_phase5b_history_create_new, replay_phase5b_history_json,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: phase5b_history_certification <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: phase5b_history_certification <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: phase5b_history_certification <emit|replay> <path>".into());
    }
    let replay = match command.as_str() {
        "emit" => {
            emit_phase5b_history_create_new(Path::new(&path)).map_err(|error| error.to_string())?
        }
        "replay" => replay_phase5b_history_json(
            &std::fs::read_to_string(path).map_err(|error| error.to_string())?,
        ),
        _ => return Err("usage: phase5b_history_certification <emit|replay> <path>".into()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&replay).map_err(|error| error.to_string())?
    );
    if replay.valid {
        Ok(())
    } else {
        Err("Phase-5b history replay failed".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("phase5b-history-driver".to_owned())
        .stack_size(96 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("Phase-5b history driver panicked".into()),
    }
}
