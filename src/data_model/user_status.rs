use serde::{Deserialize, Serialize};

pub mod presence;
pub mod user_status_text;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStatus {
    presence: presence::Presence,
    text: user_status_text::UserStatusText
}