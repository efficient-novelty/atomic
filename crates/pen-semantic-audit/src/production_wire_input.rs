//! Exact host-to-Agda transport for one canonical production-bundle byte list.
//!
//! This module deliberately proves only byte transport. It does not interpret
//! the bundle, run Agda, or turn a generated module into correspondence
//! authority. The generated module has one varying object, `inputBytes`; every
//! other source byte is fixed by this module.

use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::sync::Arc;

pub const PRODUCTION_BUNDLE_INPUT_MODULE_NAME_V1: &str =
    "LawV2.Wire.GeneratedProductionBundleInputV1";
pub const MAX_PRODUCTION_BUNDLE_INPUT_BYTES_V1: usize = 64 * 1024 * 1024;

const INPUT_MODULE_PREFIX_V1: &str = "\
{-# OPTIONS --safe --without-K #-}\n\
\n\
module LawV2.Wire.GeneratedProductionBundleInputV1 where\n\
\n\
open import Agda.Builtin.List using (List; []; _∷_)\n\
open import Agda.Builtin.Nat using (Nat)\n\
\n\
inputBytes : List Nat\n\
inputBytes =\n";
const INPUT_MODULE_SUFFIX_V1: &str = "  []\n";

pub const PRODUCTION_TRANSCRIPT_INPUT_MODULE_NAME_V1: &str =
    "LawV2.Wire.GeneratedProductionTranscriptInputV1";

const TRANSCRIPT_MODULE_PREFIX_V1: &str = "\
{-# OPTIONS --safe --without-K #-}\n\
\n\
module LawV2.Wire.GeneratedProductionTranscriptInputV1 where\n\
\n\
open import Agda.Builtin.List using (List; []; _∷_)\n\
open import Agda.Builtin.Nat using (Nat)\n\
\n\
transcriptBytes : List Nat\n\
transcriptBytes =\n";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionBundleInputFailureV1 {
    ResourceExhausted,
    NonUtf8Source,
    NonCanonicalTemplate,
    NonCanonicalByteLiteral,
    ByteOutOfRange,
    EmbeddedBytesMismatch,
}

impl std::fmt::Display for ProductionBundleInputFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ResourceExhausted => "the production bundle input exceeds its fixed byte bound",
            Self::NonUtf8Source => "the generated Agda input module is not UTF-8",
            Self::NonCanonicalTemplate => {
                "the generated Agda input module differs from the fixed source template"
            }
            Self::NonCanonicalByteLiteral => {
                "the generated Agda input contains a noncanonical byte literal"
            }
            Self::ByteOutOfRange => "the generated Agda input contains a value above 255",
            Self::EmbeddedBytesMismatch => {
                "the generated Agda input bytes differ from the canonical wire artifact"
            }
        })
    }
}

impl std::error::Error for ProductionBundleInputFailureV1 {}

/// Render the only allowed varying Agda source: one exact list of byte values.
pub fn render_production_bundle_input_module_v1(
    canonical_bytes: &[u8],
) -> Result<String, ProductionBundleInputFailureV1> {
    if canonical_bytes.len() > MAX_PRODUCTION_BUNDLE_INPUT_BYTES_V1 {
        return Err(ProductionBundleInputFailureV1::ResourceExhausted);
    }
    let body_capacity = canonical_bytes
        .len()
        .checked_mul(8)
        .ok_or(ProductionBundleInputFailureV1::ResourceExhausted)?;
    let capacity = INPUT_MODULE_PREFIX_V1
        .len()
        .checked_add(body_capacity)
        .and_then(|length| length.checked_add(INPUT_MODULE_SUFFIX_V1.len()))
        .ok_or(ProductionBundleInputFailureV1::ResourceExhausted)?;
    let mut source = String::with_capacity(capacity);
    source.push_str(INPUT_MODULE_PREFIX_V1);
    for byte in canonical_bytes {
        use std::fmt::Write;
        writeln!(&mut source, "  {byte} ∷")
            .map_err(|_| ProductionBundleInputFailureV1::ResourceExhausted)?;
    }
    source.push_str(INPUT_MODULE_SUFFIX_V1);
    Ok(source)
}

/// Extract bytes only from the exact generated source template.
///
/// Re-rendering is mandatory. Merely locating an `inputBytes` declaration
/// would allow changed imports or added expected answers to ride beside the
/// same byte list.
pub fn extract_production_bundle_input_bytes_v1(
    source: &[u8],
) -> Result<Vec<u8>, ProductionBundleInputFailureV1> {
    let text =
        std::str::from_utf8(source).map_err(|_| ProductionBundleInputFailureV1::NonUtf8Source)?;
    let body = text
        .strip_prefix(INPUT_MODULE_PREFIX_V1)
        .and_then(|text| text.strip_suffix(INPUT_MODULE_SUFFIX_V1))
        .ok_or(ProductionBundleInputFailureV1::NonCanonicalTemplate)?;
    let mut bytes = Vec::new();
    if !body.is_empty() {
        if !body.ends_with('\n') {
            return Err(ProductionBundleInputFailureV1::NonCanonicalByteLiteral);
        }
        for line in body.split_terminator('\n') {
            let digits = line
                .strip_prefix("  ")
                .and_then(|line| line.strip_suffix(" ∷"))
                .ok_or(ProductionBundleInputFailureV1::NonCanonicalByteLiteral)?;
            if digits.is_empty()
                || !digits.bytes().all(|byte| byte.is_ascii_digit())
                || (digits.len() > 1 && digits.starts_with('0'))
            {
                return Err(ProductionBundleInputFailureV1::NonCanonicalByteLiteral);
            }
            let value = digits
                .parse::<u16>()
                .map_err(|_| ProductionBundleInputFailureV1::NonCanonicalByteLiteral)?;
            bytes.push(
                u8::try_from(value).map_err(|_| ProductionBundleInputFailureV1::ByteOutOfRange)?,
            );
            if bytes.len() > MAX_PRODUCTION_BUNDLE_INPUT_BYTES_V1 {
                return Err(ProductionBundleInputFailureV1::ResourceExhausted);
            }
        }
    }
    let canonical = render_production_bundle_input_module_v1(&bytes)?;
    if canonical.as_bytes() != source {
        return Err(ProductionBundleInputFailureV1::NonCanonicalTemplate);
    }
    Ok(bytes)
}

/// Render the second allowed varying Agda source: the Rust-rendered
/// canonical transcript bytes, in the same one-value-per-line template
/// discipline as the bundle input module.
pub fn render_production_transcript_input_module_v1(
    transcript_bytes: &[u8],
) -> Result<String, ProductionBundleInputFailureV1> {
    if transcript_bytes.len() > MAX_PRODUCTION_BUNDLE_INPUT_BYTES_V1 {
        return Err(ProductionBundleInputFailureV1::ResourceExhausted);
    }
    let body_capacity = transcript_bytes
        .len()
        .checked_mul(8)
        .ok_or(ProductionBundleInputFailureV1::ResourceExhausted)?;
    let capacity = TRANSCRIPT_MODULE_PREFIX_V1
        .len()
        .checked_add(body_capacity)
        .and_then(|length| length.checked_add(INPUT_MODULE_SUFFIX_V1.len()))
        .ok_or(ProductionBundleInputFailureV1::ResourceExhausted)?;
    let mut source = String::with_capacity(capacity);
    source.push_str(TRANSCRIPT_MODULE_PREFIX_V1);
    for byte in transcript_bytes {
        use std::fmt::Write;
        writeln!(&mut source, "  {byte} ∷")
            .map_err(|_| ProductionBundleInputFailureV1::ResourceExhausted)?;
    }
    source.push_str(INPUT_MODULE_SUFFIX_V1);
    Ok(source)
}

/// Extract transcript bytes only from the exact generated template, with
/// mandatory re-rendering, mirroring the bundle input extractor.
pub fn extract_production_transcript_input_bytes_v1(
    source: &[u8],
) -> Result<Vec<u8>, ProductionBundleInputFailureV1> {
    let text =
        std::str::from_utf8(source).map_err(|_| ProductionBundleInputFailureV1::NonUtf8Source)?;
    let body = text
        .strip_prefix(TRANSCRIPT_MODULE_PREFIX_V1)
        .and_then(|text| text.strip_suffix(INPUT_MODULE_SUFFIX_V1))
        .ok_or(ProductionBundleInputFailureV1::NonCanonicalTemplate)?;
    let mut bytes = Vec::new();
    if !body.is_empty() {
        if !body.ends_with('\n') {
            return Err(ProductionBundleInputFailureV1::NonCanonicalByteLiteral);
        }
        for line in body.split_terminator('\n') {
            let digits = line
                .strip_prefix("  ")
                .and_then(|line| line.strip_suffix(" ∷"))
                .ok_or(ProductionBundleInputFailureV1::NonCanonicalByteLiteral)?;
            if digits.is_empty()
                || !digits.bytes().all(|byte| byte.is_ascii_digit())
                || (digits.len() > 1 && digits.starts_with('0'))
            {
                return Err(ProductionBundleInputFailureV1::NonCanonicalByteLiteral);
            }
            let value = digits
                .parse::<u16>()
                .map_err(|_| ProductionBundleInputFailureV1::NonCanonicalByteLiteral)?;
            bytes.push(
                u8::try_from(value).map_err(|_| ProductionBundleInputFailureV1::ByteOutOfRange)?,
            );
            if bytes.len() > MAX_PRODUCTION_BUNDLE_INPUT_BYTES_V1 {
                return Err(ProductionBundleInputFailureV1::ResourceExhausted);
            }
        }
    }
    let canonical = render_production_transcript_input_module_v1(&bytes)?;
    if canonical.as_bytes() != source {
        return Err(ProductionBundleInputFailureV1::NonCanonicalTemplate);
    }
    Ok(bytes)
}

/// Exact byte equality between one canonical `.wire` artifact and the only
/// admissible generated Agda input source.
///
/// This is intentionally weaker than safe-Agda bundle acceptance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedAgdaProductionInputArtifactV1 {
    canonical_bytes: Arc<[u8]>,
    generated_source: Arc<[u8]>,
    bundle_digest: Digest,
    generated_source_digest: Digest,
    digest: Digest,
}

impl VerifiedAgdaProductionInputArtifactV1 {
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    pub fn generated_source(&self) -> &[u8] {
        &self.generated_source
    }

    pub fn bundle_digest(&self) -> &Digest {
        &self.bundle_digest
    }

    pub fn generated_source_digest(&self) -> &Digest {
        &self.generated_source_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedAgdaProductionInputArtifactV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.bytes(&self.canonical_bytes);
        encoder.bytes(&self.generated_source);
        self.bundle_digest.encode_canonical(encoder);
        self.generated_source_digest.encode_canonical(encoder);
    }
}

pub fn verify_agda_production_input_artifact_v1(
    canonical_bytes: &[u8],
    generated_source: &[u8],
) -> Result<VerifiedAgdaProductionInputArtifactV1, ProductionBundleInputFailureV1> {
    if canonical_bytes.len() > MAX_PRODUCTION_BUNDLE_INPUT_BYTES_V1 {
        return Err(ProductionBundleInputFailureV1::ResourceExhausted);
    }
    let embedded = extract_production_bundle_input_bytes_v1(generated_source)?;
    if embedded.as_slice() != canonical_bytes {
        return Err(ProductionBundleInputFailureV1::EmbeddedBytesMismatch);
    }
    let canonical_bytes = Arc::<[u8]>::from(canonical_bytes);
    let generated_source = Arc::<[u8]>::from(generated_source);
    let bundle_digest = Digest::of_domain_bytes(
        "law-v2-production-refinement-canonical-bundle/v1",
        &canonical_bytes,
    );
    let generated_source_digest = Digest::of_domain_bytes(
        "law-v2-production-refinement-generated-agda-input/v1",
        &generated_source,
    );
    let mut verified = VerifiedAgdaProductionInputArtifactV1 {
        canonical_bytes,
        generated_source,
        bundle_digest,
        generated_source_digest,
        digest: Digest::of_bytes(b"pending generated Agda production input"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-agda-production-input-artifact/v1",
        &verified,
    );
    Ok(verified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_template_round_trips_empty_and_nonempty_inputs() {
        for bytes in [Vec::new(), vec![0, 1, 2, 127, 128, 254, 255]] {
            let source = render_production_bundle_input_module_v1(&bytes).expect("render");
            assert_eq!(
                extract_production_bundle_input_bytes_v1(source.as_bytes()).expect("extract"),
                bytes
            );
            let verified =
                verify_agda_production_input_artifact_v1(&bytes, source.as_bytes()).expect("bind");
            assert_eq!(verified.canonical_bytes(), bytes);
            assert_eq!(verified.generated_source(), source.as_bytes());
        }
    }

    #[test]
    fn changed_fixed_source_or_injected_expected_answer_is_rejected() {
        let source = render_production_bundle_input_module_v1(&[1, 2, 3]).expect("render");
        let changed_import = source.replace("Agda.Builtin.Nat", "Agda.Builtin.Bool");
        assert_eq!(
            extract_production_bundle_input_bytes_v1(changed_import.as_bytes()),
            Err(ProductionBundleInputFailureV1::NonCanonicalTemplate)
        );
        let injected = format!("{source}expectedSuccess = true\n");
        assert_eq!(
            extract_production_bundle_input_bytes_v1(injected.as_bytes()),
            Err(ProductionBundleInputFailureV1::NonCanonicalTemplate)
        );
    }

    #[test]
    fn noncanonical_and_out_of_range_literals_are_rejected() {
        let source = render_production_bundle_input_module_v1(&[7]).expect("render");
        let leading_zero = source.replace("  7 ∷", "  07 ∷");
        assert_eq!(
            extract_production_bundle_input_bytes_v1(leading_zero.as_bytes()),
            Err(ProductionBundleInputFailureV1::NonCanonicalByteLiteral)
        );
        let out_of_range = source.replace("  7 ∷", "  256 ∷");
        assert_eq!(
            extract_production_bundle_input_bytes_v1(out_of_range.as_bytes()),
            Err(ProductionBundleInputFailureV1::ByteOutOfRange)
        );
    }

    #[test]
    fn actual_bytes_are_compared_instead_of_only_digests() {
        let source = render_production_bundle_input_module_v1(&[1, 2, 3]).expect("render");
        assert_eq!(
            verify_agda_production_input_artifact_v1(&[1, 2, 4], source.as_bytes()),
            Err(ProductionBundleInputFailureV1::EmbeddedBytesMismatch)
        );
    }
}
