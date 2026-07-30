//! Rust-side pinning of the Phase G independent replay.
//!
//! The committed canonical byte vector (the exact literal embedded in
//! `LawV2/Wire/BundleDecodeTestV1.agda`) must replay end-to-end through
//! the unchanged kernel and synthesis checker: the slot table
//! round-trips through `Kernel::verify_signature`, every conversion and
//! synthesis certificate verifies through protocol V2, every supplement
//! resolves and replays in its derived binder-local context with
//! exactly-one premise coverage, and the inventory discipline replays
//! with every judgment through the kernel.
//!
//! This is a development regression test, not correspondence authority.

use pen_semantic_audit::{
    ProductionReplayFailureV1, render_production_transcript_v1, replay_production_bundle_v1,
};

const AGDA_VECTOR_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/agda/LawV2/Wire/BundleDecodeTestV1.agda"
);

fn committed_agda_literal(path: &str, header: &str) -> Vec<u8> {
    let source =
        std::fs::read_to_string(path).expect("committed Agda vector module must be readable");
    let mut bytes = Vec::new();
    let mut in_literal = false;
    for line in source.lines() {
        if line.trim_end() == header {
            in_literal = true;
            continue;
        }
        if !in_literal {
            continue;
        }
        if line.trim() == "[]" {
            break;
        }
        for token in line.split('\u{2237}') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            bytes.push(
                token
                    .parse::<u8>()
                    .expect("Agda vector literal must contain only bytes"),
            );
        }
    }
    assert!(in_literal, "Agda byte literal not found: {header}");
    bytes
}

#[test]
fn canonical_vector_replays_through_unchanged_kernel() {
    let bytes = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    let evidence = replay_production_bundle_v1(&bytes).expect("canonical vector must replay");
    assert_eq!(evidence.replayed_bytes, bytes);
    // Four declarations: UnitType (level 0), UnitType, the dependent
    // family-application pi (level 1: its parameter quantifies over a
    // Sort-0-valued family), and the eliminator pi (level 0).
    assert_eq!(
        evidence.computed.declaration_formation_levels,
        vec![0, 0, 1, 0]
    );
    // Eleven conversions, three synthesis certificates, two
    // supplements, seven Q0 categories, one fresh rule, six families.
    assert_eq!(evidence.computed.conversion_normalized_left.len(), 11);
    assert_eq!(evidence.computed.synthesis_types.len(), 3);
    assert_eq!(evidence.computed.supplement_derived_contexts.len(), 2);
    assert_eq!(
        evidence.computed.q0_categories,
        vec![0, 0, 1, 1, 1, 0, 2]
    );
    assert_eq!(evidence.computed.fresh_type_formation_levels, vec![0]);
    assert_eq!(evidence.computed.family_normalized_types.len(), 6);
}

const AGDA_SEMANTIC_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/agda/LawV2/Wire/SemanticReplayTestV1.agda"
);
const AGDA_TYPING_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/agda/LawV2/Wire/TypingReplayTestV1.agda"
);
const AGDA_INVENTORY_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/agda/LawV2/Wire/InventoryReplayTestV1.agda"
);

/// Every committed structurally-valid mutant — semantic, typing, and
/// inventory alike — is rejected by the independent replay: the replay
/// enforces the union of the layered Agda disciplines through the
/// unchanged kernel.
#[test]
fn committed_mutants_are_rejected_by_replay() {
    for (module, header) in [
        (AGDA_SEMANTIC_MODULE, "mutant-delta-wrong-body-v1 ="),
        (AGDA_SEMANTIC_MODULE, "mutant-beta-wrong-result-v1 ="),
        (AGDA_SEMANTIC_MODULE, "mutant-lookup-wrong-type-v1 ="),
        (AGDA_SEMANTIC_MODULE, "mutant-result-not-normalized-v1 ="),
        (AGDA_TYPING_MODULE, "mutant-signature-ill-typed-v1 ="),
        (AGDA_TYPING_MODULE, "mutant-context-ill-typed-v1 ="),
        (AGDA_TYPING_MODULE, "mutant-endpoint-wrong-expected-v1 ="),
        (AGDA_TYPING_MODULE, "mutant-supplement-step-path-v1 ="),
        (AGDA_TYPING_MODULE, "mutant-supplement-formation-level-v1 ="),
        (AGDA_TYPING_MODULE, "mutant-supplement-missing-v1 ="),
        (AGDA_TYPING_MODULE, "mutant-supplement-derived-context-v1 ="),
        (AGDA_INVENTORY_MODULE, "mutant-fresh-ill-typed-v1 ="),
        (AGDA_INVENTORY_MODULE, "mutant-fresh-owner-bodyful-v1 ="),
        (AGDA_INVENTORY_MODULE, "mutant-fresh-duplicate-pair-v1 ="),
        (AGDA_INVENTORY_MODULE, "mutant-seed-subject-mismatch-v1 ="),
        (AGDA_INVENTORY_MODULE, "mutant-seed-type-convertible-v1 ="),
        (
            AGDA_INVENTORY_MODULE,
            "mutant-application-subject-mismatch-v1 =",
        ),
        (
            AGDA_INVENTORY_MODULE,
            "mutant-application-context-mismatch-v1 =",
        ),
        (AGDA_INVENTORY_MODULE, "mutant-action-type-changed-v1 ="),
    ] {
        let bytes = committed_agda_literal(module, header);
        assert!(
            replay_production_bundle_v1(&bytes).is_err(),
            "replay must reject {header}"
        );
    }
}

#[test]
fn truncated_bytes_fail_closed() {
    let mut bytes = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    bytes.pop();
    assert!(matches!(
        replay_production_bundle_v1(&bytes),
        Err(ProductionReplayFailureV1::Decode)
    ));
}

const AGDA_TRANSCRIPT_AGREEMENT_MODULE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/agda/LawV2/Wire/TranscriptAgreementTestV1.agda"
);

/// The Rust-rendered canonical transcript equals the committed literal
/// that `TranscriptAgreementTestV1.agda` proves equal to the
/// independent Agda rendering by refl.
#[test]
fn rust_transcript_matches_committed_agda_literal() {
    let bytes = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    let evidence = replay_production_bundle_v1(&bytes).expect("canonical vector must replay");
    let bundle = pen_production_wire::decode_bundle_v1(&bytes).expect("decodes");
    let transcript = render_production_transcript_v1(&bundle, &evidence.computed);
    assert_eq!(
        transcript,
        committed_agda_literal(
            AGDA_TRANSCRIPT_AGREEMENT_MODULE,
            "canonical-transcript-v1 ="
        ),
        "the committed Agda transcript literal must equal the Rust rendering"
    );
}

/// Prints the canonical transcript as an Agda literal. Run manually
/// after a fixture or transcript-schema change:
/// `cargo test --manifest-path crates/pen-semantic-audit/Cargo.toml \
///    --locked --test production_replay_vectors regenerate -- --ignored --nocapture`
#[test]
#[ignore]
fn regenerate_transcript_literal() {
    let bytes = committed_agda_literal(AGDA_VECTOR_MODULE, "canonical-vector-v1 =");
    let evidence = replay_production_bundle_v1(&bytes).expect("canonical vector must replay");
    let bundle = pen_production_wire::decode_bundle_v1(&bytes).expect("decodes");
    let transcript = render_production_transcript_v1(&bundle, &evidence.computed);
    println!("-- canonical-transcript-v1 ({} bytes)", transcript.len());
    println!("canonical-transcript-v1 =");
    for chunk in transcript.chunks(12) {
        let mut line = String::from("  ");
        for byte in chunk {
            line.push_str(&format!("{byte} \u{2237} "));
        }
        println!("{line}");
    }
    println!("  []");
}
