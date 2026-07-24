use pen_search::uc1_scoring_v1::emit_uc1_scoring_v1_create_new;
use std::path::PathBuf;

fn run() -> Result<(), String> {
    let directory = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("docs"));
    let replay = emit_uc1_scoring_v1_create_new(&directory).map_err(|error| error.to_string())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&replay).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("uc1-scoring-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("UC-1 scoring worker panicked".into()),
    }
}
