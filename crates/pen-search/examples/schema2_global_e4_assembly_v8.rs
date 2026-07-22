use pen_search::global_e4_assembly_v8::{emit_global_e4_v8_create_new, replay_global_e4_v8_json};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: schema2_global_e4_assembly_v8 <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: schema2_global_e4_assembly_v8 <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: schema2_global_e4_assembly_v8 <emit|replay> <path>".into());
    }
    match command.as_str() {
        "emit" => println!(
            "{}",
            serde_json::to_string_pretty(&emit_global_e4_v8_create_new(Path::new(&path))?)?
        ),
        "replay" => {
            let replay = replay_global_e4_v8_json(&std::fs::read_to_string(path)?);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("global E-4 v8 replay failed".into());
            }
        }
        _ => return Err("usage: schema2_global_e4_assembly_v8 <emit|replay> <path>".into()),
    }
    Ok(())
}
