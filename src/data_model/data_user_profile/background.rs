use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Background(String);

impl Background {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if len < 1 || len > 128 {
            Err(format!("Background String length must be 1..=128, got {}", len))
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

#[test]
fn  test_background_length_validation_success(){
    let r = Background::new("test length validation success");
    assert_eq!(true, r.is_ok());
}

#[test]
fn  test_background_max_length_validation_fail(){
    let r = Background::new(&"test length validation fail".repeat(10));
    assert_eq!(true, r.is_err());
}
#[test]
fn  test_background_min_length_validation_fail(){
    let r = Background::new(&"");
    assert_eq!(true, r.is_err());
}

#[test]
fn test_background_serialization() {
    let background: Background = Background("serialization".to_string());

    let json = serde_json::to_string(&background).unwrap();

    assert_eq!(json, r#""serialization""#);
}

#[test]
fn test_background_deserialization() {
    let json = r#""deserialization""#;

    let background = serde_json::from_str::<Background>(json).unwrap();

    assert_eq!(background, Background("deserialization".to_string()));
}