//! Read-only replay harness for `support-comprehension-context-v1`.

use pen_search::support_comprehension_v5::{
    issue_support_comprehension_derivation_v5, replay_support_comprehension_derivation_v5,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let derivation = issue_support_comprehension_derivation_v5()?;
    replay_support_comprehension_derivation_v5(&derivation)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "valid": true,
            "instance_id": derivation.canonical_analysis.sealed_instance.instance_id,
            "support_arity": derivation.support_probe_declaration.declared_arity,
            "support_motives": derivation.canonical_analysis.support_motives,
            "realizers": derivation.canonical_analysis.interface_realizers,
            "acyclic": derivation.support_context_acyclic,
            "canonical": derivation.support_context_canonical,
            "zero_accounting": derivation.zero_accounting_proved,
            "derivation_hash": derivation.derivation_hash,
        }))?
    );
    Ok(())
}
