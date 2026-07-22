use pen_schema::grammar_completion::{
    emit_grammar_completion_create_new, replay_grammar_completion_json,
};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: schema2_grammar_completion <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: schema2_grammar_completion <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: schema2_grammar_completion <emit|replay> <path>".into());
    }
    match command.as_str() {
        "emit" => {
            let replay = emit_grammar_completion_create_new(Path::new(&path))?;
            println!("{}", serde_json::to_string_pretty(&replay)?);
        }
        "replay" => {
            let replay = replay_grammar_completion_json(&std::fs::read_to_string(path)?);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("grammar completion replay failed".into());
            }
        }
        _ => return Err("usage: schema2_grammar_completion <emit|replay> <path>".into()),
    }
    Ok(())
}
