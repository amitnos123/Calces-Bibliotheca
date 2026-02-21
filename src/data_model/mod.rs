use serde::{Deserialize, Serialize};

pub mod avatar;
pub mod display_name;
pub mod data_user_profile;
pub mod user_status_text;

pub type Badges = i32;

#[bitflag::bitflag(i64)]
#[derive(Clone, Copy, Serialize)]
pub enum Flags {
    ManageChannel = 1 << 0,
    ManageServer = 1 << 1,
    ManagePermissions = 1 << 2,
    ManageRole = 1 << 3,
    ManageCustomisation = 1 << 4,

    KickMembers = 1 << 6,
    BanMembers = 1 << 7,
    TimeoutMembers = 1 << 8,
    AssignRoles = 1 << 9,
    ChangeNickname = 1 << 10,
    ManageNicknames = 1 << 11,
    ChangeAvatar = 1 << 12,
    RemoveAvatars = 1 << 13,

    ViewChannel = 1 << 20,
    ReadMessageHistory = 1 << 21,
    SendMessage = 1 << 22,
    ManageMessages = 1 << 23,
    ManageWebhooks = 1 << 24,
    InviteOthers = 1 << 25,
    SendEmbeds = 1 << 26,
    UploadFiles = 1 << 27,
    Masquerade = 1 << 28,
    React = 1 << 29,
    Connect = 1 << 30,
    Speak = 1 << 31,
    Video = 1 << 32,
    MuteMembers = 1 << 33,
    DeafenMembers = 1 << 34,
    MoveMembers = 1 << 35,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
    DisplayName
}

#[test]
fn test_fields_user_serialization() {
    let avatar = FieldsUser::Avatar;

    let json = serde_json::to_string(&avatar).unwrap();

    assert_eq!(json, r#""Avatar""#);
}

#[test]
fn test_fields_user_deserialization() {
    let json = r#""Avatar""#;

    let avatar = serde_json::from_str::<FieldsUser>(json).unwrap();

    assert_eq!(avatar, FieldsUser::Avatar);
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStatus {
    presence: Presence,
    text: user_status_text::UserStatusText
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Presence {
    Online,
    Idle,
    Focus,
    Busy,
    Invisible
}
pub struct TooManyRequestsBody {
    // Milliseconds until calls are replenished
    retry_after: u16 // Max value expected is 10000
}