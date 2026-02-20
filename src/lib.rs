pub(crate) const BASE_URL: &'static str = "https://stoat.chat";
pub(crate) const API_ENDPOINT: &'static str = "/api";
pub(crate) const EVENTS_ENDPOINT: &'static str = "/events";
pub(crate) const FILES_ENDPOINT: &'static str = "https://cdn.stoatusercontent.com";
pub(crate) const PROXY_ENDPOINT: &'static str = "https://external.stoatusercontent.com";

fn create_client(token: String) -> Result<self::Client, reqwest::Error> {
    let client = Client { token };
    // First send request to core to check, if can reach server.
    // let response = client.get_core()?;

    // Second send request to users_me to check, if token valid.
    // let response = client.get_users_me()?;

    Ok(client)
}

pub struct Client {
    pub(crate) token: String
}

pub mod core;
pub mod users;
pub mod bots;
pub mod channels;
pub mod servers;
pub mod invites;
pub mod customisation;
pub mod administration;
pub mod misc;
