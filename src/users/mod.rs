mod user_information;
mod direct_messaging;
mod relationships;

pub(crate) const USERS_ENDPOINT: &'static str = "/users";

fn create_users_url() -> String {
    format!(
            "{}{}{}",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            USERS_ENDPOINT
        )
}