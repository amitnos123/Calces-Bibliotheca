use serde::{Deserialize, Deserializer, Serialize, Serializer};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if len < 2 || len > 32 {
            Err(format!("DisplayName String length must be 2..=128, got {}", len))
        } else {
            let re = Regex::new(r"^[^\u{200B}\n\r]+$").unwrap();
            if !re.is_match(s) {
                Err("DisplayName String most follow ^[^\\u{200B}\\n\\r]+$".to_string())
            } else {
                Ok(Self(s.to_string()))
            }
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Serialization: just serialize the inner string
impl Serialize for DisplayName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

// Deserialization: validate length
impl<'de> Deserialize<'de> for DisplayName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DisplayName::new(&s).map_err(serde::de::Error::custom)
    }
}
