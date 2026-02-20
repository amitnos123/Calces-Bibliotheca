    // pub async fn get_users_me(&self) -> Result<reqwest::Response, reqwest::Error> {
    // pub async fn get_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
    // pub async fn patch_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {

use calces_bibliotheca::{self, api::users};

#[tokio::test]
async fn test_get_users_me() {
    let token = "mU_2kVrtLPTHkhK6D3pcPusSZj23o0v6Kadhi8Wk7pL7L6w_4FytaAyfQVfNQfS6".to_string();

    let client = calces_bibliotheca::create_client(token).await;
    assert!(client.is_ok());

    let result = client.unwrap().get_users_me().await;

    assert!(result.is_ok());

}

#[tokio::test]
async fn test_get_users_target() {
    let token = "mU_2kVrtLPTHkhK6D3pcPusSZj23o0v6Kadhi8Wk7pL7L6w_4FytaAyfQVfNQfS6".to_string();

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().get_users_target("01KHNTT0EYYFSC1RWS7YTEKQHB").await;

    println!("{:?}", result);

    assert!(result.is_ok());

}

#[tokio::test]
async fn test_patch_users_target() {
    let token = "mU_2kVrtLPTHkhK6D3pcPusSZj23o0v6Kadhi8Wk7pL7L6w_4FytaAyfQVfNQfS6".to_string();

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().patch_users_target("01KHNTT0EYYFSC1RWS7YTEKQHB").await;

    println!("{:?}", result);

    assert!(result.is_ok());

}