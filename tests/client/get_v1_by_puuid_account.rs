use uuid::uuid;

use rust_unofficial_valorant_api::client::ValorantApiClient;

#[tokio::test]
async fn test() {
    let _ = env_logger::builder().is_test(true).try_init();
    let client = ValorantApiClient::new();
    let result = client.get_v1_by_puuid_account(&uuid!("f314b09f-7694-5ac0-ba34-5fc27e4d185c"))
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
    let result = client.get_v1_by_puuid_account_opts(&uuid!("f314b09f-7694-5ac0-ba34-5fc27e4d185c"), Some(false))
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(200, response.status)
}
