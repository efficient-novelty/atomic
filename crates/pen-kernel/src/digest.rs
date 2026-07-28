use schemars::JsonSchema;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use thiserror::Error;

/// Canonical BLAKE3 digest used to identify, but never to prove, an artifact.
#[derive(Clone, Debug, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct Digest(String);

impl Digest {
    pub const PREFIX: &'static str = "blake3:";

    pub fn of_bytes(bytes: &[u8]) -> Self {
        Self(format!(
            "{0}{1}",
            Self::PREFIX,
            blake3::hash(bytes).to_hex()
        ))
    }

    /// Hash length-delimited bytes under a separate protocol domain.
    pub fn of_domain_bytes(domain: &str, bytes: &[u8]) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"pen-kernel/domain/v1");
        hasher.update(&(domain.len() as u64).to_le_bytes());
        hasher.update(domain.as_bytes());
        hasher.update(&(bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
        Self(format!("{0}{1}", Self::PREFIX, hasher.finalize().to_hex()))
    }

    /// Hash a length-delimited sequence of byte slices under a protocol domain.
    pub fn of_domain_chunks(domain: &str, chunks: &[&[u8]]) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"pen-kernel/domain-chunks/v1");
        hasher.update(&(domain.len() as u64).to_le_bytes());
        hasher.update(domain.as_bytes());
        hasher.update(&(chunks.len() as u64).to_le_bytes());
        for chunk in chunks {
            hasher.update(&(chunk.len() as u64).to_le_bytes());
            hasher.update(chunk);
        }
        Self(format!("{0}{1}", Self::PREFIX, hasher.finalize().to_hex()))
    }

    pub fn of_canonical<T: CanonicalEncode>(domain: &str, value: &T) -> Self {
        let mut encoder = CanonicalEncoder::new();
        value.encode_canonical(&mut encoder);
        Self::of_domain_bytes(domain, encoder.as_bytes())
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, DigestError> {
        let value = value.into();
        let Some(hex) = value.strip_prefix(Self::PREFIX) else {
            return Err(DigestError::MissingPrefix);
        };
        if hex.len() != 64 {
            return Err(DigestError::InvalidLength(hex.len()));
        }
        if !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(DigestError::InvalidHex);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(de::Error::custom)
    }
}

impl CanonicalEncode for Digest {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.text(self.as_str());
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum DigestError {
    #[error("digest must begin with 'blake3:'")]
    MissingPrefix,
    #[error("digest payload must contain 64 hexadecimal characters, found {0}")]
    InvalidLength(usize),
    #[error("digest payload must be lowercase hexadecimal")]
    InvalidHex,
}

/// Minimal length-delimited encoder for stable proof-subject identities.
#[derive(Clone, Debug, Default)]
pub struct CanonicalEncoder {
    bytes: Vec<u8>,
}

impl CanonicalEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn bytes(&mut self, value: &[u8]) {
        self.u64(value.len() as u64);
        self.bytes.extend_from_slice(value);
    }

    pub fn text(&mut self, value: &str) {
        self.bytes(value.as_bytes());
    }

    pub fn sequence<T: CanonicalEncode>(&mut self, values: &[T]) {
        self.u64(values.len() as u64);
        for value in values {
            value.encode_canonical(self);
        }
    }

    pub fn option<T: CanonicalEncode>(&mut self, value: &Option<T>) {
        match value {
            Some(value) => {
                self.tag(1);
                value.encode_canonical(self);
            }
            None => self.tag(0),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub trait CanonicalEncode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder);
}

#[cfg(test)]
mod tests {
    use super::{Digest, DigestError};

    #[test]
    fn validation_rejects_noncanonical_text() {
        assert_eq!(
            Digest::parse("sha:00").expect_err("wrong algorithm"),
            DigestError::MissingPrefix
        );
        assert_eq!(
            Digest::parse(format!("blake3:{}", "A".repeat(64))).expect_err("uppercase"),
            DigestError::InvalidHex
        );
    }

    #[test]
    fn domains_separate_identical_bytes() {
        assert_ne!(
            Digest::of_domain_bytes("one", b"payload"),
            Digest::of_domain_bytes("two", b"payload")
        );
    }

    #[test]
    fn deserialization_reuses_validation() {
        assert!(serde_json::from_str::<Digest>("\"invalid\"").is_err());
        let digest = Digest::of_bytes(b"payload");
        let wire = serde_json::to_string(&digest).expect("serialize");
        assert_eq!(
            serde_json::from_str::<Digest>(&wire).expect("deserialize"),
            digest
        );
    }
}
