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

        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        map.insert("avatar", "avatar"); //string | null; min length: 1 max length: 128
        map.insert("badges", "d"); //integer | null ; int32
        map.insert("display_name", "display_name"); // string | null ; min length: 2 max length: 32 ; ^[^\u200B\n\r]+$
        map.insert("flags", "flags"); //integer | null ; int32
        map.insert("profile", "profile"); // DataUserProfile ; nullable
        map.insert("remove", "remove"); // array FieldsUser[] ; default: []
        map.insert("status", "status"); // UserStatus nullable

        let rtn = self.reqwest_client.patch(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&map)
            .send()
            .await?;
        return Ok(rtn);
    }
}