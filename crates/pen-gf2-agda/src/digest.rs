use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use thiserror::Error;

/// A validated lowercase BLAKE3 digest used only for evidence identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct EvidenceDigest(String);

impl EvidenceDigest {
    pub const PREFIX: &'static str = "blake3:";

    pub fn of_bytes(bytes: &[u8]) -> Self {
        Self(format!(
            "{0}{1}",
            Self::PREFIX,
            blake3::hash(bytes).to_hex()
        ))
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, EvidenceDigestError> {
        let value = value.into();
        let Some(hex) = value.strip_prefix(Self::PREFIX) else {
            return Err(EvidenceDigestError::MissingPrefix);
        };
        if hex.len() != 64 {
            return Err(EvidenceDigestError::InvalidLength(hex.len()));
        }
        if !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(EvidenceDigestError::InvalidHex);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EvidenceDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for EvidenceDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum EvidenceDigestError {
    #[error("evidence digest must begin with 'blake3:'")]
    MissingPrefix,
    #[error("evidence digest payload must contain 64 hexadecimal characters, found {0}")]
    InvalidLength(usize),
    #[error("evidence digest payload must be lowercase hexadecimal")]
    InvalidHex,
}

#[cfg(test)]
mod tests {
    use super::{EvidenceDigest, EvidenceDigestError};

    #[test]
    fn digest_wire_format_is_strict() {
        assert_eq!(
            EvidenceDigest::parse("sha256:00").expect_err("wrong algorithm"),
            EvidenceDigestError::MissingPrefix
        );
        assert_eq!(
            EvidenceDigest::parse(format!("blake3:{}", "A".repeat(64))).expect_err("uppercase"),
            EvidenceDigestError::InvalidHex
        );
        assert_eq!(
            EvidenceDigest::parse("blake3:00").expect_err("short"),
            EvidenceDigestError::InvalidLength(2)
        );
    }
}
