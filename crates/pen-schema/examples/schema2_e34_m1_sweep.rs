use pen_schema::e34_m1_sweep::{emit_e34_m1_sweep_create_new, replay_e34_m1_sweep_json};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: schema2_e34_m1_sweep <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: schema2_e34_m1_sweep <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: schema2_e34_m1_sweep <emit|replay> <path>".into());
    }
    match command.as_str() {
        "emit" => {
            let replay = emit_e34_m1_sweep_create_new(Path::new(&path))?;
            println!("{}", serde_json::to_string_pretty(&replay)?);
        }
        "replay" => {
            let replay = replay_e34_m1_sweep_json(&std::fs::read_to_string(path)?);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("SCHEMA2 enlarged-basis M1 sweep replay failed".into());
            }
        }
        _ => return Err("usage: schema2_e34_m1_sweep <emit|replay> <path>".into()),
    }
    Ok(())
}
