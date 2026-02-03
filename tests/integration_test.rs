use blindpay::{BlindPay, BlindPayError};

#[test]
fn test_client_creation() {
    let client = BlindPay::new("test-api-key", "test-instance-id");
    assert!(client.is_ok());
}

#[test]
fn test_missing_api_key() {
    let client = BlindPay::new("", "test-instance-id");
    assert!(matches!(client, Err(BlindPayError::MissingApiKey)));
}

#[test]
fn test_missing_instance_id() {
    let client = BlindPay::new("test-api-key", "");
    assert!(matches!(client, Err(BlindPayError::MissingInstanceId)));
}

#[test]
fn test_client_has_resources() {
    let client = BlindPay::new("test-api-key", "test-instance-id").unwrap();
    
    // Test that all resource methods are accessible
    let _available = client.available();
    let _instances = client.instances();
    let _partner_fees = client.partner_fees();
    let _payins = client.payins();
    let _payouts = client.payouts();
    let _receivers = client.receivers();
    let _virtual_accounts = client.virtual_accounts();
    let _wallets = client.wallets();
}

#[tokio::test]
async fn test_error_handling() {
    // This test demonstrates error handling
    // In a real scenario, you would mock the HTTP responses
    let client = BlindPay::new("invalid-key", "invalid-instance").unwrap();
    
    // This should fail with an API error (if network is available)
    // or a network error (if offline)
    let result = client.receivers().list().await;
    assert!(result.is_err());
}
