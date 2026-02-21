use reqwest;
impl crate::Client {
    // Query Node get /
    pub async fn query_node(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}{}{}",
            "https://",
            crate::BASE_URL,
            "/"
        );
        let rtn = self.reqwest_client.get(url)
            .send()
            .await?;
        return Ok(rtn);
    }
}