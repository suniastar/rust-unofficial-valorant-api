use rust_unofficial_valorant_api::client::ValorantApiClient;

#[tokio::test]
async fn test() {
    let _ = env_logger::builder().is_test(true).try_init();
    let client = ValorantApiClient::new();
    let result = client.get_v1_account("t00manysecrets", "EUW")
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(200, response.status)
}

#[tokio::test]
async fn test_opts() {
    let _ = env_logger::builder().is_test(true).try_init();
    let client = ValorantApiClient::new();
    let result = client.get_v1_account_opts("t00manysecrets", "EUW", Some(false))
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(200, response.status)
}
