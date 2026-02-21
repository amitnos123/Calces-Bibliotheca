use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod avatar;
pub mod display_name;
pub mod data_user_profile;

pub type Badges = i32;
// pub type Flags = i32;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStatus {
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

pub struct TooManyRequestsBody {
    // Milliseconds until calls are replenished
    retry_after: u16 // Max value expected is 10000
}