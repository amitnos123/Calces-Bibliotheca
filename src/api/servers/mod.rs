/* Server Information  */
// post /servers/create
// get /servers/{target}
// delete /servers/{target}
// patch /servers/{target}
// put /servers/{target}/ack
// post /servers/{server}/channels

/* Server Members */
// get /servers/{target}/members
// get /servers/{target}/members/{member}
// delete /servers/{target}/members/{member}
// patch /servers/{server}/members/{member}
// get /servers/{target}/members_experimental_query
// put /servers/{server}/bans/{target}
// delete /servers/{server}/bans/{target}
// get /servers/{target}/bans
// get /servers/{target}/invites

/* Server Permissions */
// post /servers/{target}/roles
// get /servers/{target}/roles/{role_id}
// delete /servers/{target}/roles/{role_id}
// patch /servers/{target}/roles/{role_id}
// put /servers/{target}/permissions/{role_id}
// put /servers/{target}/permissions/default
// patch /servers/{target}/roles/ranks

pub(crate) const SERVERS_ENDPOINT: &'static str = "/servers"; 

fn create_url() -> String {
    format!(
            "{}{}{}{}",
            "https://",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            SERVERS_ENDPOINT
        )
}