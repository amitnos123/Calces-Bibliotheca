// get /invites/{target}
// post /invites/{target}
// delete /invites/{target}

pub(crate) const INVITES_ENDPOINT: &'static str = "/invites"; 

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            INVITES_ENDPOINT
        )
}