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
            "{}/{}",
            super::create_users_url(),
            "@me"
        );
        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

    // target = user id
    pub async fn get_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            super::create_users_url(),
            target
        );
        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

    pub async fn patch_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            super::create_users_url(),
            target
        );
        let rtn = self.reqwest_client.patch(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }
}