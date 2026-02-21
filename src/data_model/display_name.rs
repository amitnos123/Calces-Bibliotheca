use serde::{Deserialize, Deserializer, Serialize, Serializer};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(s: &str) -> Result<Self, String> {
        let len = s.len();
        if len < 2 || len > 32 {
            Err(format!("DisplayName String length must be 2..=32, got {}", len))
        } else {
            let re = Regex::new(r"^[^\u200B\n\r]+$").unwrap();
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


#[test]
fn  test_display_name_regex_fail_1(){
    let r = DisplayName::new("test\nlength\nvalidation\nfail");
    assert_eq!(true, r.is_err());
}
#[test]
fn  test_display_name_regex_fail_2(){
    let r = DisplayName::new("test\rlength\rvalidation\rfail");
    assert_eq!(true, r.is_err());
}
#[test]
fn  test_display_name_regex_fail_3(){
    let r = DisplayName::new("test\u{200B}length\u{200B}validation\u{200B}fail");
    assert_eq!(true, r.is_err());
}

#[test]
fn  test_display_name_length_regex_validation_success(){
    let r = DisplayName::new("test length validation success");
    assert_eq!(true, r.is_ok());
}

#[test]
fn  test_display_name_max_length_validation_fail(){
    let r = DisplayName::new(&"test length validation fail".repeat(3));
    assert_eq!(true, r.is_err());
}
#[test]
fn  test_display_name_min_length_validation_fail(){
    let r = DisplayName::new(&"A");
    assert_eq!(true, r.is_err());
}

#[test]
fn test_display_name_serialization() {
    let display_name = DisplayName("serialization".to_string());

    let json = serde_json::to_string(&display_name).unwrap();

    assert_eq!(json, r#""serialization""#);
}

#[test]
fn test_display_name_deserialization() {
    let json = r#""deserialization""#;

    let display_name = serde_json::from_str::<DisplayName>(json).unwrap();

    assert_eq!(display_name, DisplayName("deserialization".to_string()));
    
}