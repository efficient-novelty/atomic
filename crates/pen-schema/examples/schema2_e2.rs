use pen_schema::e2_certificate::{emit_schema2_e2_create_new, replay_schema2_e2_json};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: schema2_e2 <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: schema2_e2 <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: schema2_e2 <emit|replay> <path>".into());
    }
    let path = Path::new(&path);
    match command.as_str() {
        "emit" => {
            let replay = emit_schema2_e2_create_new(path)?;
            println!("{}", serde_json::to_string_pretty(&replay)?);
        }
        "replay" => {
            let json = std::fs::read_to_string(path)?;
            let replay = replay_schema2_e2_json(&json);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("SCHEMA2 E-2 successor replay failed".into());
            }
        }
        _ => return Err("usage: schema2_e2 <emit|replay> <path>".into()),
    }
    Ok(())
}
