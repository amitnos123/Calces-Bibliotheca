    // pub async fn get_users_me(&self) -> Result<reqwest::Response, reqwest::Error> {
    // pub async fn get_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
    // pub async fn patch_users_target(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {

use calces_bibliotheca::{self, api::users, data_model::{Badges, avatar::Avatar, data_user_profile::{background::Background, content::Content}, display_name::{self, DisplayName}, user_status::{self, UserStatus}}};

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

    let avatar = Avatar::new("Avatar").unwrap();
    let badge : Badges = 5; 
    let display_name = display_name::DisplayName::new("DisplayName").unwrap();
    let flags : calces_bibliotheca::data_model::Flags = calces_bibliotheca::data_model::Flags::ManageChannel;
    let data_user_profile: calces_bibliotheca::data_model::data_user_profile::DataUserProfile = calces_bibliotheca::data_model::data_user_profile::DataUserProfile {
        background: Some(Background::new("Background").unwrap()),
        content: Some(Content::new("Content").unwrap()),
    };
    let remove: Vec<calces_bibliotheca::data_model::FieldsUser> = vec![calces_bibliotheca::data_model::FieldsUser::Avatar];
    let user_status = UserStatus {
        presence: user_status::presence::Presence::Online,
        text: user_status::user_status_text::UserStatusText::new("UserStatusText").unwrap()
    };

    // let result : Result<reqwest::Response, reqwest::Error> = client.unwrap().edit_user(
    //     "01KHVQJY2KF2D4VJ399JPB47AE",
    //     Some(avatar),
    //     Some(badge),
    //     Some(display_name),
    //     Some(flags),
    //     Some(data_user_profile),
    //     remove,
    //     Some(user_status)
    // ).await;

    let result : Result<reqwest::Response, reqwest::Error> = client.unwrap().edit_user(
        "01KHVQJY2KF2D4VJ399JPB47AE",
        None,
        None,
        None,
        None,
        None,
        remove,
        None
    ).await;

    println!("{:?}", result.unwrap().text().await);

    // assert!(result.is_ok() && result.unwrap().status() == 200);

}

#[tokio::test]
async fn test_fetch_user_flags() {
    let token = "token".to_string();

    let client = calces_bibliotheca::create_client(token).await;


    let result : Result<reqwest::Response, reqwest::Error> = client.unwrap().fetch_user_flags("target").await;

    println!("{:?}", result);

    assert!(result.is_ok() && result.unwrap().status() == 200);
}

#[tokio::test]
async fn test_fetch_default_avatar() {
    let token = "token".to_string();

    let client = calces_bibliotheca::create_client(token).await;


    let result : Result<reqwest::Response, reqwest::Error> = client.unwrap().fetch_default_avatar("target").await;

    println!("{:?}", result);

    assert!(result.is_ok() && result.unwrap().status() == 200);
}

#[tokio::test]
async fn test_fetch_user_profile() {
    let token = "token".to_string();

    let client = calces_bibliotheca::create_client(token).await;


    let result : Result<reqwest::Response, reqwest::Error> = client.unwrap().fetch_user_profile("target").await;

    println!("{:?}", result);

    assert!(result.is_ok() && result.unwrap().status() == 200);
}