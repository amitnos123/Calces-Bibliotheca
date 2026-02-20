use reqwest::Error;

pub(crate) const BASE_URL: &'static str = "https://stoat.chat";
pub(crate) const API_ENDPOINT: &'static str = "/api";
pub(crate) const EVENTS_ENDPOINT: &'static str = "/events";
pub(crate) const FILES_ENDPOINT: &'static str = "https://cdn.stoatusercontent.com";
pub(crate) const PROXY_ENDPOINT: &'static str = "https://external.stoatusercontent.com";

pub async fn create_client(token: String) -> Result<self::Client, reqwest::Error> {
    let reqwest_client = reqwest::Client::new();
    let client = Client { reqwest_client, token };
    // First send request to core to check, if can reach server.
    let _ = client.get_core().await?.error_for_status()?;

    // Second send request to users_me to check, if token valid.
    let _ = client.get_users_me().await?.error_for_status()?;

    Ok(client)
}

pub struct Client {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) token: String
}

pub mod api;