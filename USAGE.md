# BlindPay Rust SDK Usage Guide

This guide provides detailed examples and best practices for using the BlindPay Rust SDK.

## Table of Contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Core Concepts](#core-concepts)
- [Working with Receivers](#working-with-receivers)
- [Working with Payouts](#working-with-payouts)
- [Working with Payins](#working-with-payins)
- [Working with Wallets](#working-with-wallets)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
blindpay = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

## Authentication

Get your API credentials from the [BlindPay Dashboard](https://app.blindpay.com):

```rust
use blindpay::BlindPay;

#[tokio::main]
async fn main() -> blindpay::Result<()> {
    let client = BlindPay::new(
        "your-api-key",
        "your-instance-id"
    )?;
    
    Ok(())
}
```

## Core Concepts

### Result Type

All async methods return `Result<T, BlindPayError>`:

```rust
use blindpay::{BlindPay, BlindPayError};

async fn example() -> blindpay::Result<()> {
    let client = BlindPay::new("api-key", "instance-id")?;
    
    match client.receivers().list().await {
        Ok(receivers) => println!("Success: {} receivers", receivers.len()),
        Err(BlindPayError::ApiError(msg)) => eprintln!("API Error: {}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    Ok(())
}
```

### Type Safety

The SDK uses strong typing:

```rust
use blindpay::types::{Network, StablecoinToken, Country};

let network = Network::Polygon;
let token = StablecoinToken::USDC;
let country = Country::US;
```

## Working with Receivers

### Creating a Receiver

```rust
use blindpay::resources::receivers::*;
use blindpay::types::Country;

async fn create_receiver(client: &BlindPay) -> blindpay::Result<()> {
    let input = CreateIndividualWithStandardKycInput {
        email: "john.doe@example.com".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        tax_id: "123456789".to_string(),
        date_of_birth: "1990-01-01".to_string(),
        country: Country::US,
        address_line_1: "123 Main Street".to_string(),
        city: "New York".to_string(),
        state_province_region: "NY".to_string(),
        postal_code: "10001".to_string(),
        id_doc_country: Country::US,
        id_doc_type: IdentificationDocument::Passport,
        id_doc_front_file: "https://example.com/id-front.jpg".to_string(),
        proof_of_address_doc_type: ProofOfAddressDocType::UtilityBill,
        proof_of_address_doc_file: "https://example.com/proof.pdf".to_string(),
        tos_id: "tos_abc123".to_string(),
        // Optional fields
        address_line_2: None,
        phone_number: Some("+1234567890".to_string()),
        id_doc_back_file: None,
        external_id: Some("ext_123".to_string()),
    };

    let receiver = client.receivers()
        .create_individual_with_standard_kyc(input)
        .await?;
    
    println!("Created receiver: {}", receiver.id);
    Ok(())
}
```

### Listing Receivers

```rust
async fn list_receivers(client: &BlindPay) -> blindpay::Result<()> {
    let receivers = client.receivers().list().await?;
    
    for receiver in receivers {
        println!("ID: {}", receiver.id);
        println!("Email: {}", receiver.email);
        println!("KYC Status: {}", receiver.kyc_status);
        println!("---");
    }
    
    Ok(())
}
```

### Getting Receiver Limits

```rust
async fn check_limits(client: &BlindPay, receiver_id: &str) -> blindpay::Result<()> {
    let limits = client.receivers().get_limits(receiver_id).await?;
    
    println!("Payin Limits:");
    println!("  Daily: {}", limits.limits.payin.daily);
    println!("  Monthly: {}", limits.limits.payin.monthly);
    
    println!("Payout Limits:");
    println!("  Daily: {}", limits.limits.payout.daily);
    println!("  Monthly: {}", limits.limits.payout.monthly);
    
    Ok(())
}
```

## Working with Payouts

### Creating a Payout

```rust
use blindpay::resources::payouts::CreateStellarPayoutInput;

async fn create_payout(client: &BlindPay) -> blindpay::Result<()> {
    let input = CreateStellarPayoutInput {
        quote_id: "qu_abc123".to_string(),
        sender_wallet_address: "GABC...XYZ".to_string(),
        signed_transaction: None,
    };

    let payout = client.payouts().create_stellar(input).await?;
    
    println!("Payout ID: {}", payout.id);
    println!("Status: {:?}", payout.status);
    
    Ok(())
}
```

### Tracking a Payout

```rust
async fn track_payout(client: &BlindPay, payout_id: &str) -> blindpay::Result<()> {
    let payout = client.payouts().get_track(payout_id).await?;
    
    println!("Transaction: {:?}", payout.tracking_transaction.step);
    println!("Payment: {:?}", payout.tracking_payment.step);
    println!("Complete: {:?}", payout.tracking_complete.step);
    
    if let Some(eta) = payout.tracking_payment.estimated_time_of_arrival {
        println!("ETA: {:?}", eta);
    }
    
    Ok(())
}
```

### Listing Payouts with Pagination

```rust
use blindpay::types::PaginationParams;

async fn list_payouts_paginated(client: &BlindPay) -> blindpay::Result<()> {
    let params = PaginationParams {
        limit: Some("50".to_string()),
        offset: Some("0".to_string()),
        starting_after: None,
        ending_before: None,
    };

    let response = client.payouts().list(Some(params)).await?;
    
    println!("Payouts: {}", response.data.len());
    println!("Has more: {}", response.pagination.has_more);
    
    for payout in response.data {
        println!("  {} - {:?}", payout.id, payout.status);
    }
    
    Ok(())
}
```

## Working with Payins

### Creating a Payin

```rust
async fn create_payin(client: &BlindPay, quote_id: &str) -> blindpay::Result<()> {
    let payin = client.payins().create_evm(quote_id).await?;
    
    println!("Payin ID: {}", payin.id);
    println!("Status: {:?}", payin.status);
    
    Ok(())
}
```

### Tracking a Payin

```rust
async fn track_payin(client: &BlindPay, payin_id: &str) -> blindpay::Result<()> {
    let payin = client.payins().get_track(payin_id).await?;
    
    println!("Payment Method: {:?}", payin.payment_method);
    println!("Amount: {}", payin.receiver_amount);
    println!("Token: {:?}", payin.token);
    
    Ok(())
}
```

## Working with Wallets

### Creating a Blockchain Wallet

```rust
use blindpay::resources::wallets::blockchain::*;
use blindpay::types::Network;

async fn create_wallet(client: &BlindPay) -> blindpay::Result<()> {
    let input = CreateBlockchainWalletWithAddressInput {
        receiver_id: "re_123".to_string(),
        name: "My Polygon Wallet".to_string(),
        network: Network::Polygon,
        address: "0xDD6a3aD0949396e57C7738ba8FC1A46A5a1C372C".to_string(),
    };

    let wallet = client.wallets()
        .blockchain()
        .create_with_address(input)
        .await?;
    
    println!("Wallet ID: {}", wallet.id);
    println!("Network: {:?}", wallet.network);
    println!("Address: {:?}", wallet.address);
    
    Ok(())
}
```

### Listing Wallets

```rust
async fn list_wallets(client: &BlindPay, receiver_id: &str) -> blindpay::Result<()> {
    let wallets = client.wallets()
        .blockchain()
        .list(receiver_id)
        .await?;
    
    for wallet in wallets {
        println!("Wallet: {} ({})", wallet.name, wallet.id);
        println!("  Network: {:?}", wallet.network);
        if let Some(addr) = wallet.address {
            println!("  Address: {}", addr);
        }
    }
    
    Ok(())
}
```

## Error Handling

### Comprehensive Error Handling

```rust
use blindpay::BlindPayError;

async fn handle_errors(client: &BlindPay) {
    match client.receivers().list().await {
        Ok(receivers) => {
            println!("Success: {} receivers", receivers.len());
        }
        Err(BlindPayError::ApiError(msg)) => {
            eprintln!("API returned error: {}", msg);
            // Handle API-specific errors
        }
        Err(BlindPayError::RequestFailed(e)) => {
            eprintln!("Network request failed: {}", e);
            // Handle network errors
        }
        Err(BlindPayError::SerializationError(e)) => {
            eprintln!("JSON error: {}", e);
            // Handle serialization errors
        }
        Err(e) => {
            eprintln!("Other error: {}", e);
        }
    }
}
```

### Retrying on Failure

```rust
use tokio::time::{sleep, Duration};

async fn retry_request<F, T>(
    mut f: F,
    max_attempts: u32,
) -> blindpay::Result<T>
where
    F: FnMut() -> blindpay::Result<T>,
{
    let mut attempts = 0;
    
    loop {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_attempts - 1 => {
                attempts += 1;
                eprintln!("Attempt {} failed: {}", attempts, e);
                sleep(Duration::from_secs(2u64.pow(attempts))).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Best Practices

### 1. Use Environment Variables

```rust
use std::env;

#[tokio::main]
async fn main() -> blindpay::Result<()> {
    let api_key = env::var("BLINDPAY_API_KEY")
        .expect("BLINDPAY_API_KEY not set");
    let instance_id = env::var("BLINDPAY_INSTANCE_ID")
        .expect("BLINDPAY_INSTANCE_ID not set");
    
    let client = BlindPay::new(api_key, instance_id)?;
    
    Ok(())
}
```

### 2. Reuse the Client

```rust
// Good: Reuse client
let client = BlindPay::new(api_key, instance_id)?;

let receivers = client.receivers().list().await?;
let payouts = client.payouts().list(None).await?;

// Bad: Creating new clients
let client1 = BlindPay::new(api_key, instance_id)?;
let receivers = client1.receivers().list().await?;

let client2 = BlindPay::new(api_key, instance_id)?;
let payouts = client2.payouts().list(None).await?;
```

### 3. Handle Errors Appropriately

```rust
async fn process_payout(client: &BlindPay, payout_id: &str) -> blindpay::Result<()> {
    let payout = client.payouts().get(payout_id).await?;
    
    match payout.status {
        TransactionStatus::Completed => {
            println!("Payout completed successfully");
        }
        TransactionStatus::Failed => {
            eprintln!("Payout failed");
            return Err(BlindPayError::ApiError("Payout failed".to_string()));
        }
        _ => {
            println!("Payout still processing");
        }
    }
    
    Ok(())
}
```

### 4. Use Type Safety

```rust
// Good: Use enums
use blindpay::types::Network;

let network = Network::Polygon;

// Bad: Using strings
let network = "polygon"; // Type unsafe!
```

### 5. Implement Proper Logging

```rust
use log::{info, error};

async fn create_receiver_with_logging(
    client: &BlindPay,
    input: CreateIndividualWithStandardKycInput,
) -> blindpay::Result<CreateReceiverResponse> {
    info!("Creating receiver for email: {}", input.email);
    
    match client.receivers().create_individual_with_standard_kyc(input).await {
        Ok(receiver) => {
            info!("Successfully created receiver: {}", receiver.id);
            Ok(receiver)
        }
        Err(e) => {
            error!("Failed to create receiver: {}", e);
            Err(e)
        }
    }
}
```

## Complete Example Application

Here's a complete example that puts it all together:

```rust
use blindpay::{BlindPay, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from environment
    let api_key = env::var("BLINDPAY_API_KEY")?;
    let instance_id = env::var("BLINDPAY_INSTANCE_ID")?;
    
    // Initialize client
    let client = BlindPay::new(api_key, instance_id)?;
    
    // Check available rails
    println!("=== Available Payment Rails ===");
    let rails = client.available().get_rails().await?;
    for rail in &rails {
        println!("{} ({})", rail.label, rail.country);
    }
    
    // List receivers
    println!("\n=== Receivers ===");
    let receivers = client.receivers().list().await?;
    println!("Total receivers: {}", receivers.len());
    
    // List payouts
    println!("\n=== Recent Payouts ===");
    let payouts = client.payouts().list(None).await?;
    for payout in payouts.data.iter().take(5) {
        println!("{}: {:?}", payout.id, payout.status);
    }
    
    println!("\n✅ Complete!");
    Ok(())
}
```

For more examples, check the `examples/` directory in the repository.
