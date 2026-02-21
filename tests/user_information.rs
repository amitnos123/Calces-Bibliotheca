    // pub async fn get_users_me(&self) -> Result<reqwest::Response, reqwest::Error> {
    // pub async fn get_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
    // pub async fn patch_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {

use calces_bibliotheca::{self, api::users};

#[tokio::test]
async fn test_get_users_me() {
    let token = "token".to_string();

    let client = calces_bibliotheca::create_client(token).await;
    assert!(client.is_ok());

    let result = client.unwrap().fetch_self().await;

    assert!(result.is_ok());

}

#[tokio::test]
async fn test_get_users_target() {
    let token = "token".to_string();

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().fetch_user("user_id").await;

    println!("{:?}", result);

    assert!(result.is_ok());

}

#[tokio::test]
async fn test_patch_users_target() {
    let token = "token".to_string();

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().edit_user("user_id").await;

    println!("{:?}", result);

    assert!(result.is_ok());

}