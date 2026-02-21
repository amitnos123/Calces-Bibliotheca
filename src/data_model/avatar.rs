use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Avatar(String);

impl Avatar {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if len < 1 || len > 128 {
            Err(format!("Avatar String length must be 1..=128, got {}", len))
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



#[test]
fn  test_avatar_length_validation_success(){
    let r = Avatar::new("test length validation success");
    assert_eq!(true, r.is_ok());
}

#[test]
fn  test_avatar_max_length_validation_fail(){
    let r = Avatar::new(&"test length validation fail".repeat(10));
    assert_eq!(true, r.is_err());
}
#[test]
fn  test_avatar_min_length_validation_fail(){
    let r = Avatar::new(&"");
    assert_eq!(true, r.is_err());
}

#[test]
fn test_avatar_serialization() {
    let avatar = Avatar("serialization".to_string());

    let json = serde_json::to_string(&avatar).unwrap();

    assert_eq!(json, r#""serialization""#);
}

#[test]
fn test_avatar_deserialization() {
    let json = r#""deserialization""#;

    let avatar = serde_json::from_str::<Avatar>(json).unwrap();

    assert_eq!(avatar, Avatar("deserialization".to_string()));
    
}