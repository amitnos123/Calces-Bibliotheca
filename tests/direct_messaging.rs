#[tokio::test]
async fn test_get_users_target() {
    let token = "Pu2F-gtzLRqWMKiKYYvWkbYCncJt8dQb_lr9hsrE412WlNMDOmmSrHE9HKqdZRRq".to_string();

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().fetch_direct_message_channels().await;

    println!("{:?}", result);

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_open_direct_message() {
    let token = "Pu2F-gtzLRqWMKiKYYvWkbYCncJt8dQb_lr9hsrE412WlNMDOmmSrHE9HKqdZRRq".to_string();

    let client = calces_bibliotheca::create_client(token).await;

    let result = client.unwrap().open_direct_message("01KHNTT0EYYFSC1RWS7YTEKQHB").await;

    println!("{:?}", result);

    assert!(result.is_ok());
}