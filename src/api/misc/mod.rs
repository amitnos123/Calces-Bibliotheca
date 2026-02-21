

pub(crate) const MISC_ENDPOINT: &'static str = "/sync"; 

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            MISC_ENDPOINT
        )
}