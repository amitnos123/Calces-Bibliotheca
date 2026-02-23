#[tokio::test]
async fn test_get_users_target() {
    let token = std::env::var("token").unwrap_or_else(|_| "token");

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().fetch_direct_message_channels().await;

    println!("{:?}", result);

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_open_direct_message() {
    let token = std::env::var("token").unwrap_or_else(|_| "token");

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().open_direct_message("target").await;

    println!("{:?}", result);

    assert!(result.is_ok());
}