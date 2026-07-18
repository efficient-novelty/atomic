//! Frozen reproduction path for the Phase 4 Step-16 semantic exhaustion
//! (SEMANTIC_NORMALIZATION_PROGRAM §4):
//!
//! ```text
//! cargo run -p pen-search --release --example step16_semantic_exhaustion -- \
//!     --out docs/step16_semantic_exhaustion.json
//! ```

use pen_search::step16_semantic_exhaustion::run_step16_semantic_exhaustion;
use std::env;
use std::fs;

fn main() {
    let mut out = None;
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        if argument == "--out" {
            out = args.next();
        }
    }
    let out = out.expect("usage: --out <path>");

    let exhaustion = run_step16_semantic_exhaustion(4);
    for stratum in &exhaustion.strata {
        println!(
            "kappa {}: raw {} = internal {} + egp_marginal {} + invalid_named {} + unclassified {} \
             (sums_match {}, unclassified_zero {})",
            stratum.kappa,
            stratum.raw_total,
            stratum.internal,
            stratum.egp_marginal,
            stratum.invalid_named_bare_univ,
            stratum.unclassified,
            stratum.sums_match,
            stratum.unclassified_is_zero,
        );
    }
    println!("every_stratum_classified: {}", exhaustion.every_stratum_classified);
    println!("digest: {}", exhaustion.digest);

    let json = serde_json::to_string_pretty(&exhaustion).expect("serialize exhaustion");
    fs::write(&out, format!("{json}\n")).expect("write artifact");
    println!("wrote {out}");
}
