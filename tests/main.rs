use calces_bibliotheca;

#[cfg(test)]
mod tests{
    pub(self) mod core;
    pub(self) mod users;
    pub(self) mod bots;
    pub(self) mod channels;
    pub(self) mod servers;
    pub(self) mod invites;
    pub(self) mod customisation;
    pub(self) mod administration;
    pub(self) mod authentication;
    pub(self) mod misc;
}
#[tokio::test]
async fn test_create_client() {
    let token = std::env::var("token").unwrap_or_else(|_| "token".to_string());
    if token == "token" {
        panic!("Please set env var 'token'");
    }

    let result = calces_bibliotheca::create_client(token).await;

    if let Err(x) = &result {
        println!("{}", x)
    }

    assert!(result.is_ok());

}