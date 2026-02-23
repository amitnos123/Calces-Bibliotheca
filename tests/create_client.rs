use calces_bibliotheca;

#[tokio::test]
async fn test_create_client() {
    let token = std::env::var("token").unwrap_or_else(|_| "token");

    let result = calces_bibliotheca::create_client(token).await;

    assert!(result.is_ok());

}