use pen_search::semantic_nu_transport_maps_v2::emit_semantic_nu_transport_maps_v2_create_new;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("docs"));
    let replay = emit_semantic_nu_transport_maps_v2_create_new(&directory)?;
    println!("{}", serde_json::to_string_pretty(&replay)?);
    Ok(())
}
