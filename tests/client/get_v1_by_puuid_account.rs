use uuid::uuid;

use rust_unofficial_valorant_api::client::ValorantApiClient;

#[tokio::test]
async fn test() {
    let _ = env_logger::builder().is_test(true).try_init();
    let client = ValorantApiClient::new();
    let t = client.get_v1_by_puuid_account(&uuid!("f314b09f-7694-5ac0-ba34-5fc27e4d185c"))
        .await;
    assert!(t.is_ok());
    println!("{:?}", t);
}

#[tokio::test]
async fn test_opts() {
    let _ = env_logger::builder().is_test(true).try_init();
    let client = ValorantApiClient::new();
    let t = client.get_v1_by_puuid_account_opts(&uuid!("f314b09f-7694-5ac0-ba34-5fc27e4d185c"), Some(false))
        .await;
    assert!(t.is_ok());
    println!("{:?}", t);
}
