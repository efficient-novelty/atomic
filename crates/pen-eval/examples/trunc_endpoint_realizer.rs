use pen_eval::trunc_endpoint_realizer::{
    replay_trunc_endpoint_realizer_json, trunc_endpoint_realizer_json_pretty,
};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: trunc_endpoint_realizer <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: trunc_endpoint_realizer <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: trunc_endpoint_realizer <emit|replay> <path>".into());
    }
    let path = Path::new(&path);
    match command.as_str() {
        "emit" => {
            let json = trunc_endpoint_realizer_json_pretty()?;
            let mut output = OpenOptions::new().write(true).create_new(true).open(path)?;
            output.write_all(json.as_bytes())?;
            output.flush()?;
            let replay = replay_trunc_endpoint_realizer_json(&json);
            if !replay.valid {
                return Err(format!("new certificate failed replay: {:?}", replay.errors).into());
            }
            println!("{}", serde_json::to_string_pretty(&replay)?);
        }
        "replay" => {
            let json = std::fs::read_to_string(path)?;
            let replay = replay_trunc_endpoint_realizer_json(&json);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("Trunc endpoint realizer replay failed".into());
            }
        }
        _ => return Err("usage: trunc_endpoint_realizer <emit|replay> <path>".into()),
    }
    Ok(())
}
