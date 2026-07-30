//! Verifier provenance for the finite inductive-completion slice.
//!
//! This is intentionally separate from the semantic manifest.  Changing an
//! implementation source digest does not silently change the frozen calculus.

use crate::operational::operational_grammar_digest;
use pen_demand::gsc::{
    GscOutcome, GscUnknownReason, VerifiedGscSemanticManifest, VerifiedGscVerifierManifest,
    gsc_toolchain_digest,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const GF2_SLICE_VERIFIER_MANIFEST_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Gf2SliceVerifierManifestV1 {
    pub schema_version: u16,
    pub semantic_manifest_digest: Digest,
    pub demand_verifier_manifest_digest: Digest,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub operational_grammar_digest: Digest,
    pub verifier_source_digest: Digest,
    pub toolchain_digest: Digest,
}

impl Gf2SliceVerifierManifestV1 {
    pub fn canonical_digest(&self) -> Digest {
        Digest::of_canonical("pen-gf2/slice-verifier-manifest/v1", self)
    }
}

impl CanonicalEncode for Gf2SliceVerifierManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.demand_verifier_manifest_digest
            .encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.operational_grammar_digest.encode_canonical(encoder);
        self.verifier_source_digest.encode_canonical(encoder);
        self.toolchain_digest.encode_canonical(encoder);
    }
}

pub fn current_gf2_slice_verifier_manifest_v1(
    semantic: &VerifiedGscSemanticManifest,
    kernel: &Kernel,
    demand_verifier: &VerifiedGscVerifierManifest,
) -> GscOutcome<Gf2SliceVerifierManifestV1> {
    if demand_verifier.semantic_manifest_digest() != semantic.digest()
        || demand_verifier.manifest().kernel_protocol_digest != kernel.kernel_protocol_digest()
        || demand_verifier.manifest().normalizer_protocol_digest
            != kernel.normalizer_protocol_digest()
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier);
    }
    GscOutcome::Proven(Gf2SliceVerifierManifestV1 {
        schema_version: GF2_SLICE_VERIFIER_MANIFEST_SCHEMA_VERSION,
        semantic_manifest_digest: semantic.digest().clone(),
        demand_verifier_manifest_digest: demand_verifier.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        operational_grammar_digest: operational_grammar_digest(semantic),
        verifier_source_digest: gf2_inductive_completion_verifier_source_digest(),
        toolchain_digest: gsc_toolchain_digest(),
    })
}

pub fn gf2_inductive_completion_verifier_source_digest() -> Digest {
    canonical_source_digest(
        "pen-gf2/inductive-completion-verifier-source/v1",
        &[
            include_bytes!("lib.rs"),
            include_bytes!("rewrite.rs"),
            include_bytes!("operational.rs"),
            include_bytes!("quotient.rs"),
            include_bytes!("verifier_manifest.rs"),
            include_bytes!("../Cargo.toml"),
        ],
    )
}

fn canonical_source_digest(domain: &str, chunks: &[&[u8]]) -> Digest {
    let canonical = chunks
        .iter()
        .map(|chunk| {
            let text = std::str::from_utf8(chunk).expect("embedded verifier source is UTF-8");
            let normalized = text.replace("\r\n", "\n");
            assert!(
                !normalized.contains('\r'),
                "embedded verifier source contains a bare carriage return"
            );
            normalized.into_bytes()
        })
        .collect::<Vec<_>>();
    let slices = canonical.iter().map(Vec::as_slice).collect::<Vec<_>>();
    Digest::of_domain_chunks(domain, &slices)
}
