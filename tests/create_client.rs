use calces_bibliotheca;

#[tokio::test]
async fn test_create_client() {
    let token = "token".to_string();

    let result = calces_bibliotheca::create_client(token).await;

    assert!(result.is_ok());

}