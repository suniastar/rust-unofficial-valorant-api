use rust_unofficial_valorant_api::client::ValorantApiClient;
use rust_unofficial_valorant_api::types::Affinity;

#[tokio::test]
async fn test() {
    let _ = env_logger::builder().is_test(true).try_init();
    let client = ValorantApiClient::new();
    let result = client.get_v1_version(&Affinity::Europe)
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(200, response.status)
}
