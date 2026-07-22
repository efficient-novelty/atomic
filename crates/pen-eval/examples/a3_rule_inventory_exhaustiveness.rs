use pen_eval::a3_rule_inventory_exhaustiveness::{
    A3RuleInventoryExhaustivenessCertificate,
    emit_historical_a3_rule_inventory_exhaustiveness_create_new,
    replay_historical_a3_rule_inventory_exhaustiveness,
};
use std::path::Path;

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "create-new" => {
            let replay =
                emit_historical_a3_rule_inventory_exhaustiveness_create_new(Path::new(path))
                    .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        }
        [command, path] if command == "replay" => {
            let certificate: A3RuleInventoryExhaustivenessCertificate = serde_json::from_str(
                &std::fs::read_to_string(path).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let replay = replay_historical_a3_rule_inventory_exhaustiveness(&certificate);
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
            if !replay.valid {
                return Err("A3 rule-inventory exhaustiveness replay failed".to_owned());
            }
        }
        _ => {
            return Err(
                "usage: a3_rule_inventory_exhaustiveness <create-new|replay> <path>".to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("a3-rule-inventory-exhaustiveness-driver".to_owned())
        .stack_size(128 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("A3 exhaustiveness driver panicked".into()),
    }
}
