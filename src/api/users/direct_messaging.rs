impl crate::Client {
    pub async fn fetch_direct_message_channels(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            super::create_url(),
            "dms"
        );

        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

    pub async fn open_direct_message(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}/{}",
            super::create_url(),
            target,
            "dm"
        );
        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }
}