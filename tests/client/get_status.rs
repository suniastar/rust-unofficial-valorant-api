use unofficial_valorant_api::client::ValorantApiClient;
use unofficial_valorant_api::types::Affinity;

#[tokio::test]
async fn test() {
    env_logger::builder().is_test(true).init();
    let client = ValorantApiClient::new();
    let t = client.get_status(&Affinity::Europe)
        .await;
    println!("{:?}", t);
}
