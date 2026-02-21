mod user_information;
mod direct_messaging;
mod relationships;

pub(crate) const USERS_ENDPOINT: &'static str = "/users";

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            USERS_ENDPOINT
        )
}