use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Avatar(String);

impl Avatar {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if len < 1 || len > 128 {
            Err(format!("avatar String length must be 1..=128, got {}", len))
        } else {
            Ok(Self(s.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Serialization: just serialize the inner string
impl Serialize for Avatar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

// Deserialization: validate length
impl<'de> Deserialize<'de> for Avatar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Avatar::new(&s).map_err(serde::de::Error::custom)
    }
}
