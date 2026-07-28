#![forbid(unsafe_code)]

use pen_engine::registered_bootstrap_milestone_outcome;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let outcome = registered_bootstrap_milestone_outcome()?;
    println!("{}", serde_json::to_string_pretty(&outcome)?);
    Ok(())
}
