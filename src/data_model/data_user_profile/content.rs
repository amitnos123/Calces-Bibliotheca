use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[test]
fn  test_length_validation_success(){
    let r = Content::new("test length validation success");
    assert_eq!(true, r.is_ok());
}

#[test]
fn  test_length_validation_fail(){
    let r = Content::new(&"test length validation fail".repeat(2001));
    assert_eq!(true, r.is_err());
}

#[test]
fn test_content_serialization() {
    let content: Content = Content("serialization".to_string());

    let json = serde_json::to_string(&content).unwrap();

    assert_eq!(json, r#""serialization""#);
}

#[test]
fn test_content_deserialization() {
    let json = r#""deserialization""#;

    let content = serde_json::from_str::<Content>(json).unwrap();

    assert_eq!(content, Content("deserialization".to_string()));
}