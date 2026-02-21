mod emojis;

pub(crate) const CUSTOMISATION_ENDPOINT: &'static str = "/custom"; 

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            CUSTOMISATION_ENDPOINT
        )
}