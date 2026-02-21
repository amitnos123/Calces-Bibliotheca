use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Presence {
    Online,
    Idle,
    Focus,
    Busy,
    Invisible
}

#[test]
fn test_presence_serialization() {
    let presence = Presence::Online;

    let json = serde_json::to_string(&presence).unwrap();

    assert_eq!(json, r#""Online""#);
}

#[test]
fn test_presence_deserialization() {
    let json = r#""Online""#;

    let presence = serde_json::from_str::<Presence>(json).unwrap();

    assert_eq!(presence, Presence::Online);
}