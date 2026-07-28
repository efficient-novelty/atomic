use schemars::JsonSchema;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Versioned register tag.  Law-level APIs accept only
/// [`SemanticFamilyValue`]; historical structural values have a distinct type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueRegister {
    SemanticFamilyV2,
    LegacyStructuralV1,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Numeric carrier for the semantic-family register.
///
/// Construction does not certify an audit.  A proof-bearing wrapper will be
/// introduced with the deterministic certificate verifier; this PR establishes
/// only the type-level register firewall.
pub struct SemanticFamilyValue(u32);

impl SemanticFamilyValue {
    pub const REGISTER: ValueRegister = ValueRegister::SemanticFamilyV2;

    pub const fn new(families: u32) -> Self {
        Self(families)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Numeric carrier for frozen structural testimony.
pub struct LegacyStructuralValue(u32);

impl LegacyStructuralValue {
    pub const REGISTER: ValueRegister = ValueRegister::LegacyStructuralV1;

    pub const fn new(total: u32) -> Self {
        Self(total)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RegisteredValueWire {
    register: ValueRegister,
    value: u32,
}

impl Serialize for SemanticFamilyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RegisteredValueWire {
            register: Self::REGISTER,
            value: self.get(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SemanticFamilyValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = RegisteredValueWire::deserialize(deserializer)?;
        if wire.register != Self::REGISTER {
            return Err(de::Error::custom(
                "legacy structural value cannot enter the semantic-family register",
            ));
        }
        Ok(Self::new(wire.value))
    }
}

impl Serialize for LegacyStructuralValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RegisteredValueWire {
            register: Self::REGISTER,
            value: self.get(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LegacyStructuralValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = RegisteredValueWire::deserialize(deserializer)?;
        if wire.register != Self::REGISTER {
            return Err(de::Error::custom(
                "semantic-family value cannot enter the legacy structural register",
            ));
        }
        Ok(Self::new(wire.value))
    }
}

#[cfg(test)]
mod tests {
    use super::{LegacyStructuralValue, SemanticFamilyValue, ValueRegister};

    #[test]
    fn registers_are_distinct_types_and_tags() {
        let semantic = SemanticFamilyValue::new(6);
        let legacy = LegacyStructuralValue::new(103);
        assert_eq!(
            SemanticFamilyValue::REGISTER,
            ValueRegister::SemanticFamilyV2
        );
        assert_eq!(
            LegacyStructuralValue::REGISTER,
            ValueRegister::LegacyStructuralV1
        );
        assert_eq!(semantic.get(), 6);
        assert_eq!(legacy.get(), 103);
    }

    #[test]
    fn serialized_values_carry_and_enforce_their_register() {
        let semantic = SemanticFamilyValue::new(6);
        let semantic_json = serde_json::to_string(&semantic).expect("serialize semantic value");
        assert_eq!(
            semantic_json,
            r#"{"register":"semantic_family_v2","value":6}"#
        );
        assert_eq!(
            serde_json::from_str::<SemanticFamilyValue>(&semantic_json)
                .expect("deserialize semantic value"),
            semantic
        );

        let legacy_json =
            serde_json::to_string(&LegacyStructuralValue::new(103)).expect("serialize legacy");
        assert!(
            serde_json::from_str::<SemanticFamilyValue>(&legacy_json).is_err(),
            "cross-register deserialization must fail"
        );
    }
}
