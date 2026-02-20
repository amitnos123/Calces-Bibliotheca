use reqwest;
impl crate::Client {
    // Query Node get /
    pub async fn get_core(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}{}",
            crate::BASE_URL,
            "/"
        );
        let rtn = self.reqwest_client.get(url)
            .send()
            .await?;
        return Ok(rtn);
    }
}