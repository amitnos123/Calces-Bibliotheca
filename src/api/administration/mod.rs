pub mod admin;
pub mod user_safety;

pub(crate) const ADMINISTRATION_ENDPOINT: &'static str = "/"; // TODO: Rethink this one, doesn't really has one

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            ADMINISTRATION_ENDPOINT
        )
}