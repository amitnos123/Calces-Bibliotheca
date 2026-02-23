use calces_bibliotheca;

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