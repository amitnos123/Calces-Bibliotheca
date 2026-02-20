use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataUserProfile {
    background: Option<Background>,
    content: Option<Content>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Background(String);

impl Background {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if len < 1 || len > 128 {
            Err(format!("DisplayName String length must be 1..=128, got {}", len))
        } else {
            Ok(Self(s.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Serialization: just serialize the inner string
impl Serialize for Background {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

// Deserialization: validate length
impl<'de> Deserialize<'de> for Background {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Background::new(&s).map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content(String);

impl Content {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if  len > 2000 {
            Err(format!("Content String length shorter then 2000, got {}", len))
        } else {
            Ok(Self(s.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Serialization: just serialize the inner string
impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

// Deserialization: validate length
impl<'de> Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Content::new(&s).map_err(serde::de::Error::custom)
    }
}
