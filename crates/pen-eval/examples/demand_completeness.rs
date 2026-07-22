use pen_eval::demand_completeness::{
    demand_completeness_json_pretty, replay_demand_completeness_json,
};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("usage: demand_completeness <emit|replay> <path>")?;
    let path = args
        .next()
        .ok_or("usage: demand_completeness <emit|replay> <path>")?;
    if args.next().is_some() {
        return Err("usage: demand_completeness <emit|replay> <path>".into());
    }
    let path = Path::new(&path);
    match command.as_str() {
        "emit" => {
            let json = demand_completeness_json_pretty()?;
            let mut output = OpenOptions::new().write(true).create_new(true).open(path)?;
            output.write_all(json.as_bytes())?;
            output.flush()?;
            let replay = replay_demand_completeness_json(&json);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err(format!("new demand certificate failed: {:?}", replay.errors).into());
            }
        }
        "replay" => {
            let json = std::fs::read_to_string(path)?;
            let replay = replay_demand_completeness_json(&json);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("demand-completeness replay failed".into());
            }
        }
        _ => return Err("usage: demand_completeness <emit|replay> <path>".into()),
    }
    Ok(())
}
