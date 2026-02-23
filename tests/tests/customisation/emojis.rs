//TODO: Test
#[tokio::test]
async fn test_create_new_emoji() {
    let token = std::env::var("token").unwrap_or_else(|_| "token".to_string());

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().create_new_emoji("target", "name".to_string(), calces_bibliotheca::api::customisation::emojis::Parent::Detached, true).await;

    println!("{:?}", result);

    assert!(result.is_ok());
}
//TODO: Test
#[tokio::test]
async fn test_fetch_emoji() {
    let token = std::env::var("token").unwrap_or_else(|_| "token".to_string());

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().fetch_emoji("target").await;

    println!("{:?}", result);

    assert!(result.is_ok());
}
//TODO: Test
#[tokio::test]
async fn test_delete_emoji() {
    let token = std::env::var("token").unwrap_or_else(|_| "token".to_string());

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().delete_emoji("target").await;

    println!("{:?}", result);

    assert!(result.is_ok());
}