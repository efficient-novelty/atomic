use pen_search::naturality_orbit_transport::{
    NaturalityOrbitTransportCertificate, emit_naturality_orbit_transport_create_new,
    replay_naturality_orbit_transport_certificate,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "create-new" => {
            let replay = emit_naturality_orbit_transport_create_new(Path::new(path))?;
            println!("{}", serde_json::to_string_pretty(&replay)?);
        }
        [command, path] if command == "replay" => {
            let certificate: NaturalityOrbitTransportCertificate =
                serde_json::from_str(&std::fs::read_to_string(path)?)?;
            let replay = replay_naturality_orbit_transport_certificate(&certificate);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err("naturality/orbit transport replay failed".into());
            }
        }
        _ => {
            return Err(
                "usage: naturality_orbit_transport <create-new|replay> <artifact.json>".into(),
            );
        }
    }
    Ok(())
}
