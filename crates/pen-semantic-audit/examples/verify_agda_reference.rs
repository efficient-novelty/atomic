use pen_semantic_audit::agda_gate::diagnose_pinned_agda_reference_v1;

fn main() {
    match diagnose_pinned_agda_reference_v1() {
        Ok(certificate) => {
            println!(
                "{{\"status\":\"valid\",\"agda_reference_digest\":\"{}\"}}",
                certificate.digest()
            );
        }
        Err(failure) => {
            eprintln!("Agda reference could not be verified: {failure}");
            eprintln!("diagnostic: {failure:?}");
            std::process::exit(1);
        }
    }
}
