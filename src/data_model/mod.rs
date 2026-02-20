use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod avatar;
pub mod display_name;
pub mod data_user_profile;

type Badges = i32;
type Flags = i32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
    DisplayName
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct UserStatus {
    presence: Presence,
    text: UserStatusText
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Presence {
    Online,
    Idle,
    Focus,
    Busy,
    Invisible
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserStatusText(String);

impl UserStatusText {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if len > 128 {
            Err(format!("UserStatusText String length under 128, got {}", len))
        } else {
            Ok(Self(s.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Serialization: just serialize the inner string
impl Serialize for UserStatusText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

// Deserialization: validate length
impl<'de> Deserialize<'de> for UserStatusText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        UserStatusText::new(&s).map_err(serde::de::Error::custom)
    }
}
