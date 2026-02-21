// post /bots/create
// get /bots/{target}/invite
// post /bots/{target}/invite
// get /bots/{bot}
// get /bots/@me
// delete /bots/{target}
// patch /bots/{target}

pub(crate) const BOTS_ENDPOINT: &'static str = "/bots"; 

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            BOTS_ENDPOINT
        )
}