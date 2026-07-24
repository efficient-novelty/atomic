use pen_search::t_d2_1_domain_build_v1::emit_t_d2_1_domain_build_v1_create_new;
use pen_search::t_d2_2_completeness_attempt_v1::emit_t_d2_2_completeness_attempt_v1_create_new;
use pen_search::t_d2_3_candidate_provenance_key_schema_v1::emit_t_d2_3_candidate_provenance_key_schema_v1_create_new;
use std::path::Path;

fn run() -> Result<(), String> {
    let td21 = emit_t_d2_1_domain_build_v1_create_new(
        Path::new("docs/t_d2_1_domain_build_v1.json"),
        Path::new("docs/T_D2_1_DOMAIN_BUILD_V1_RESULT.md"),
    )
    .map_err(|error| error.to_string())?;

    let td22 = emit_t_d2_2_completeness_attempt_v1_create_new(
        Path::new("docs/t_d2_2_completeness_attempt_v1.json"),
        Path::new("docs/T_D2_2_COMPLETENESS_ATTEMPT_V1_RESULT.md"),
    )
    .map_err(|error| error.to_string())?;

    let td23 = emit_t_d2_3_candidate_provenance_key_schema_v1_create_new(
        Path::new("docs/t_d2_3_candidate_provenance_key_schema_v1.json"),
        Path::new("docs/T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_RESULT.md"),
    )
    .map_err(|error| error.to_string())?;

    println!(
        "T-D2 program create-new complete:\n  T-D2-1: {} ({:?})\n  T-D2-2: {} ({:?})\n  T-D2-3: {} ({:?})\n  M-3 v1 authoritative: {}\n  M-4 authorized: {}",
        td21.result_digest,
        td21.status,
        td22.result_digest,
        td22.status,
        td23.result_digest,
        td23.status,
        td21.m3_v1_remains_authoritative
            && !td22.gate.m3_or_m4_gate_moved
            && td23.gate.m3_v1_remains_authoritative,
        td21.m4_authorized || td22.gate.m3_or_m4_gate_moved || td23.gate.m4_authorized,
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("depth-two-domain-program-execution-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("depth-two domain program worker panicked".into()),
    }
}
