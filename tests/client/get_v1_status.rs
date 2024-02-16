use rust_unofficial_valorant_api::client::ValorantApiClient;
use rust_unofficial_valorant_api::types::Affinity;

#[tokio::test]
async fn test() {
    let _ = env_logger::builder().is_test(true).try_init();
    let client = ValorantApiClient::new();
    let t = client.get_v1_status(&Affinity::Europe)
        .await;
    assert!(t.is_ok());
    println!("{:?}", t);
}
