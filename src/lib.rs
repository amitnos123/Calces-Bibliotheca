const BASE_URL: &'static str = "https://stoat.chat";
const API_ENDPOINT: &'static str = "/api";
const EVENTS_ENDPOINT: &'static str = "/events";
const FILES_ENDPOINT: &'static str = "https://cdn.stoatusercontent.com";
const PROXY_ENDPOINT: &'static str = "https://external.stoatusercontent.com";

mod core;
mod users;
mod bots;
mod channels;
mod servers;
mod invites;
mod customisation;
mod administration;
mod misc;
