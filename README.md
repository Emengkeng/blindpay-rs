# Unofficial BlindPay Rust SDK

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

The unofficial Rust SDK for [BlindPay](https://blindpay.com) - Stablecoin API for global payments.

## Features

- 🦀 **Idiomatic Rust** - Follows Rust best practices and conventions
- 🔒 **Type-safe** - Leverages Rust's type system for compile-time safety
- ⚡ **Async/await** - Built on tokio for high-performance async operations
- 📦 **Easy to use** - Simple and intuitive API
- 🔧 **Comprehensive** - Full coverage of BlindPay API endpoints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
blindpay ={ git = "https://github.com/Emengkeng/blindpay-rs", branch = "main" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use blindpay::{BlindPay, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client
    let client = BlindPay::new("your-api-key", "your-instance-id")?;

    // List available rails
    let rails = client.available().get_rails().await?;
    println!("Available rails: {:?}", rails);

    // List receivers
    let receivers = client.receivers().list().await?;
    println!("Found {} receivers", receivers.len());

    Ok(())
}
```

## Authentication

To get started, you'll need both your API key and instance ID from the [BlindPay dashboard](https://app.blindpay.com/instances/{instanceId}/api-keys).

```rust
use blindpay::BlindPay;

let client = BlindPay::new("your-api-key", "your-instance-id")?;
```

## Core Concepts

### Response Handling

All API methods return a `Result<T, BlindPayError>` type:

```rust
use blindpay::{BlindPay, BlindPayError};

let client = BlindPay::new("api-key", "instance-id")?;

match client.receivers().list().await {
    Ok(receivers) => {
        println!("Success! Found {} receivers", receivers.len());
    }
    Err(BlindPayError::ApiError(msg)) => {
        eprintln!("API error: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

### Error Handling

The SDK provides a comprehensive error type:

```rust
use blindpay::BlindPayError;

pub enum BlindPayError {
    ApiError(String),           // API returned an error
    RequestFailed(reqwest::Error), // HTTP request failed
    SerializationError(serde_json::Error), // JSON error
    MissingApiKey,              // API key not provided
    MissingInstanceId,          // Instance ID not provided
    InvalidConfiguration(String), // Invalid configuration
}
```

## Usage Examples

### Working with Receivers

```rust
use blindpay::BlindPay;
use blindpay::resources::receivers::*;
use blindpay::types::Country;

#[tokio::main]
async fn main() -> blindpay::Result<()> {
    let client = BlindPay::new("api-key", "instance-id")?;

    // Create a receiver
    let input = CreateIndividualWithStandardKycInput {
        email: "user@example.com".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        tax_id: "123456789".to_string(),
        date_of_birth: "1990-01-01".to_string(),
        country: Country::US,
        address_line_1: "123 Main St".to_string(),
        city: "New York".to_string(),
        state_province_region: "NY".to_string(),
        postal_code: "10001".to_string(),
        id_doc_country: Country::US,
        id_doc_type: IdentificationDocument::Passport,
        id_doc_front_file: "https://example.com/doc.jpg".to_string(),
        proof_of_address_doc_type: ProofOfAddressDocType::UtilityBill,
        proof_of_address_doc_file: "https://example.com/proof.pdf".to_string(),
        tos_id: "tos_123".to_string(),
        address_line_2: None,
        phone_number: None,
        id_doc_back_file: None,
        external_id: None,
    };

    let receiver = client.receivers()
        .create_individual_with_standard_kyc(input)
        .await?;
    
    println!("Created receiver: {}", receiver.id);

    // Get receiver details
    let receiver_details = client.receivers()
        .get(&receiver.id)
        .await?;
    
    println!("Receiver email: {}", receiver_details.email);

    // Get receiver limits
    let limits = client.receivers()
        .get_limits(&receiver.id)
        .await?;
    
    println!("Daily limit: {}", limits.limits.payout.daily);

    Ok(())
}
```

### Working with Payouts

```rust
use blindpay::BlindPay;
use blindpay::resources::payouts::CreateStellarPayoutInput;

#[tokio::main]
async fn main() -> blindpay::Result<()> {
    let client = BlindPay::new("api-key", "instance-id")?;

    // Create a Stellar payout
    let input = CreateStellarPayoutInput {
        quote_id: "qu_123".to_string(),
        sender_wallet_address: "GABC...XYZ".to_string(),
        signed_transaction: None,
    };

    let payout = client.payouts().create_stellar(input).await?;
    println!("Payout created: {}", payout.id);

    // Get payout details
    let payout_details = client.payouts().get(&payout.id).await?;
    println!("Status: {:?}", payout_details.status);

    // List all payouts
    let payouts = client.payouts().list(None).await?;
    println!("Total payouts: {}", payouts.data.len());

    Ok(())
}
```

### Working with Blockchain Wallets

```rust
use blindpay::BlindPay;
use blindpay::resources::wallets::blockchain::*;
use blindpay::types::Network;

#[tokio::main]
async fn main() -> blindpay::Result<()> {
    let client = BlindPay::new("api-key", "instance-id")?;

    // Create a wallet with address
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
    
    println!("Wallet created: {}", wallet.id);

    // List wallets for a receiver
    let wallets = client.wallets()
        .blockchain()
        .list("re_123")
        .await?;
    
    println!("Found {} wallets", wallets.len());

    Ok(())
}
```

### Checking Available Rails

```rust
use blindpay::BlindPay;
use blindpay::types::Rail;

#[tokio::main]
async fn main() -> blindpay::Result<()> {
    let client = BlindPay::new("api-key", "instance-id")?;

    // Get all available rails
    let rails = client.available().get_rails().await?;
    
    for rail in rails {
        println!("{} - {} ({})", rail.label, rail.value, rail.country);
    }

    // Get bank details for a specific rail
    let bank_details = client.available()
        .get_bank_details(Rail::Pix)
        .await?;
    
    for detail in bank_details {
        println!("Field: {} (required: {})", detail.label, detail.required);
    }

    Ok(())
}
```

## API Coverage

The SDK provides coverage of the BlindPay API:

- ✅ **Available** - Rails, bank details, SWIFT codes
- ✅ **Receivers** - Create, list, get, delete, limits
- ✅ **Bank Accounts** - PIX, ACH, Wire, SPEI, SWIFT, RTP, Argentina Transfers, Colombia ACH
- ✅ **Payouts** - Create (Stellar/EVM/Solana), list, get, track
- ✅ **Payins** - Create (EVM), list, get, track
- ✅ **Quotes** - Create payout quotes, get FX rates
- ✅ **Payin Quotes** - Create payin quotes, get payin FX rates
- ✅ **Partner Fees** - Create, list, get, delete
- ✅ **Blockchain Wallets** - Create, list, get, delete, sign messages
- ✅ **Offramp Wallets** - Create, list, get
- ✅ **Virtual Accounts** - Create, list, get, update
- ✅ **Instances** - Get members, update, delete, manage member roles
- ✅ **API Keys** - Create, list, get, delete
- ✅ **Webhooks** - Create, list, delete, get secrets, portal access
- ✅ **Terms of Service** - Initiate acceptance flow

## Advanced Usage

### Custom HTTP Client Configuration

The SDK uses `reqwest` under the hood. While the default configuration works for most use cases, you can customize the client by forking the repository.

### Pagination

For endpoints that support pagination:

```rust
use blindpay::types::PaginationParams;

let params = PaginationParams {
    limit: Some("50".to_string()),
    offset: Some("0".to_string()),
    starting_after: None,
    ending_before: None,
};

let payouts = client.payouts().list(Some(params)).await?;
```

### Type Safety

The SDK leverages Rust's type system to provide compile-time guarantees:

```rust
use blindpay::types::{Network, StablecoinToken, TransactionStatus};

// These are all strongly typed enums
let network = Network::Polygon;
let token = StablecoinToken::USDC;
let status = TransactionStatus::Completed;
```

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Examples

Check out the `examples/` directory for more usage examples:

```bash
cargo run --example basic
cargo run --example receivers
cargo run --example payouts
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support

- 📧 Email: [hello@juslen.site](mailto:hello@juslen.site)
- 🐛 Issues: [GitHub Issues](https://github.com/Emengkeng/blindpay-rs/issues)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes.

---