use pen_search::e2b_quotient_closure::{
    emit_e2b_quotient_closure_create_new, replay_e2b_quotient_closure_json,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: schema2_e2b_quotient_closure <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: schema2_e2b_quotient_closure <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: schema2_e2b_quotient_closure <emit|replay> <path>".to_owned());
    }
    match command.as_str() {
        "emit" => println!(
            "{}",
            serde_json::to_string_pretty(
                &emit_e2b_quotient_closure_create_new(Path::new(&path))
                    .map_err(|error| error.to_string())?
            )
            .map_err(|error| error.to_string())?
        ),
        "replay" => {
            let replay = replay_e2b_quotient_closure_json(
                &std::fs::read_to_string(path).map_err(|error| error.to_string())?,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&replay).map_err(|error| error.to_string())?
            );
            if !replay.valid {
                return Err("E-2b quotient-closure replay failed".to_owned());
            }
        }
        _ => {
            return Err("usage: schema2_e2b_quotient_closure <emit|replay> <path>".to_owned());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("e2b-quotient-closure-driver".to_owned())
        .stack_size(48 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("E-2b quotient-closure driver panicked".into()),
    }
}
