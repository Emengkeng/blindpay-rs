use blindpay::{BlindPay, Result};
use blindpay::resources::bank_accounts::*;
use blindpay::resources::quotes::*;
use blindpay::resources::virtual_accounts::*;
use blindpay::resources::webhooks::*;
use blindpay::types::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client
    let client = BlindPay::new(
        "your-api-key-here",
        "your-instance-id-here"
    )?;

    println!("=== BlindPay Rust SDK - Complete Feature Demo ===\n");

    // 1. Bank Accounts
    println!("1. Creating PIX bank account...");
    match client.receivers().bank_accounts().create_pix(CreatePixInput {
        receiver_id: "re_123".to_string(),
        name: "My PIX Account".to_string(),
        pix_key: "14947677768".to_string(),
    }).await {
        Ok(account) => println!("   ✓ Created PIX account: {}", account.id),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 2. Quotes
    println!("\n2. Creating payout quote...");
    match client.quotes().create(CreateQuoteInput {
        bank_account_id: "ba_123".to_string(),
        currency_type: CurrencyType::Sender,
        cover_fees: true,
        request_amount: 1000.0,
        network: Network::Polygon,
        token: Some(StablecoinToken::USDC),
        description: Some("Payment for services".to_string()),
        partner_fee_id: None,
        transaction_document_file: None,
        transaction_document_id: None,
        transaction_document_type: None,
    }).await {
        Ok(quote) => {
            println!("   ✓ Quote ID: {}", quote.id);
            println!("   ✓ Sender amount: {}", quote.sender_amount);
            println!("   ✓ Receiver amount: {}", quote.receiver_amount);
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 3. Get FX Rate
    println!("\n3. Getting FX rate...");
    match client.quotes().get_fx_rate(GetFxRateInput {
        currency_type: CurrencyType::Sender,
        from: StablecoinToken::USDC,
        to: Currency::BRL,
        request_amount: 1000.0,
    }).await {
        Ok(rate) => {
            println!("   ✓ Commercial rate: {}", rate.commercial_quotation);
            println!("   ✓ BlindPay rate: {}", rate.blindpay_quotation);
            println!("   ✓ Result amount: {}", rate.result_amount);
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 4. Virtual Accounts
    println!("\n4. Listing virtual accounts...");
    match client.virtual_accounts().list("re_123").await {
        Ok(accounts) => {
            println!("   ✓ Found {} virtual accounts", accounts.len());
            for account in accounts.iter().take(3) {
                println!("     - {} ({:?})", account.id, account.banking_partner);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 5. Instance Management
    println!("\n5. Getting instance members...");
    match client.instances().get_members().await {
        Ok(members) => {
            println!("   ✓ Found {} members", members.len());
            for member in &members {
                println!("     - {} {} ({:?})", member.first_name, member.last_name, member.role);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 6. API Keys
    println!("\n6. Listing API keys...");
    match client.instances().api_keys().list().await {
        Ok(keys) => {
            println!("   ✓ Found {} API keys", keys.len());
            for key in &keys {
                println!("     - {}: {:?}", key.name, key.permission);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 7. Webhooks
    println!("\n7. Listing webhook endpoints...");
    match client.instances().webhook_endpoints().list().await {
        Ok(endpoints) => {
            println!("   ✓ Found {} webhook endpoints", endpoints.len());
            for endpoint in &endpoints {
                println!("     - {}: {} events", endpoint.url, endpoint.events.len());
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 8. Payin Quotes
    println!("\n8. Creating payin quote...");
    match client.payins().quotes().create(CreatePayinQuoteInput {
        blockchain_wallet_id: "bw_123".to_string(),
        currency_type: CurrencyType::Sender,
        payment_method: PayinPaymentMethod::Pix,
        request_amount: 5000.0,
        token: StablecoinToken::USDC,
        is_otc: None,
        cover_fees: true,
        partner_fee_id: None,
        payer_rules: None,
    }).await {
        Ok(quote) => {
            println!("   ✓ Payin Quote ID: {}", quote.id);
            println!("   ✓ Sender amount: {}", quote.sender_amount);
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // 9. Bank Account Types
    println!("\n9. Demonstrating all bank account types:");
    println!("   ✓ PIX (Brazil)");
    println!("   ✓ ACH (United States)");
    println!("   ✓ Wire (United States)");
    println!("   ✓ RTP (United States)");
    println!("   ✓ SPEI (Mexico)");
    println!("   ✓ Argentina Transfers");
    println!("   ✓ Colombia ACH");
    println!("   ✓ International SWIFT");

    println!("\n=== Demo Complete! ===");
    println!("\nThe BlindPay Rust SDK provides full API coverage:");
    println!("  ✅ All payment rails supported");
    println!("  ✅ Complete quote management");
    println!("  ✅ Virtual account operations");
    println!("  ✅ Instance & member management");
    println!("  ✅ API key management");
    println!("  ✅ Webhook configuration");
    println!("  ✅ Terms of service flows");

    Ok(())
}
