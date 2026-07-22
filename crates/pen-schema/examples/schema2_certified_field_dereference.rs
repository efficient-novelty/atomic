use pen_schema::internal_classifier_branch_v6::{
    emit_certified_field_internality_create_new, replay_certified_field_internality_json,
};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: schema2_certified_field_dereference <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: schema2_certified_field_dereference <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: schema2_certified_field_dereference <emit|replay> <path>".into());
    }
    match command.as_str() {
        "emit" => {
            let replay = emit_certified_field_internality_create_new(Path::new(&path))?;
            println!("{}", serde_json::to_string_pretty(&replay)?);
        }
        "replay" => {
            let replay = replay_certified_field_internality_json(&std::fs::read_to_string(path)?);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("certified-field internality replay failed".into());
            }
        }
        _ => {
            return Err("usage: schema2_certified_field_dereference <emit|replay> <path>".into());
        }
    }
    Ok(())
}
