use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[test]
fn  test_display_name_max_length_validation_fail(){
    let r = UserStatusText::new(&"test length validation fail".repeat(30));
    assert_eq!(true, r.is_err());
}

#[test]
fn test_user_status_text_serialization() {
    let user_status_text = UserStatusText("serialization".to_string());

    let json = serde_json::to_string(&user_status_text).unwrap();

    assert_eq!(json, r#""serialization""#);
}

#[test]
fn test_user_status_text_deserialization() {
    let json = r#""deserialization""#;

    let display_name = serde_json::from_str::<UserStatusText>(json).unwrap();

    assert_eq!(display_name, UserStatusText("deserialization".to_string()));
    
}