pub mod account;
pub mod mfa;
pub mod onboarding;
pub mod session;

pub(crate) const AUTHENTICATION_ENDPOINT: &'static str = "/auth"; 

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            AUTHENTICATION_ENDPOINT
        )
}