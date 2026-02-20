/* User Information */
// get /users/@me
// get /users/{target}
// patch /users/{target}
// get /users/{target}/flags
// patch /users/@me/username
// get /users/{target}/default_avatar
// get /users/{target}/profile



use reqwest;
impl crate::Client {
    pub async fn get_users_me(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}{}{}{}",
            crate::BASE_URL,
            crate::API_ENDPOINT,
            super::USERS_ENDPOINT,
            "/@me"
        );
        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .send()
            .await?;
        return Ok(rtn);
    }
}