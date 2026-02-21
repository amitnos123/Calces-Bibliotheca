pub mod channel_information;
pub mod channel_invites;
pub mod channel_permissions;
pub mod interactions;
pub mod groups;
pub mod messaging;
pub mod voice;
mod webhooks;

pub(crate) const CHANNELS_ENDPOINT: &'static str = "/channels"; 

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            CHANNELS_ENDPOINT
        )
}