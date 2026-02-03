use blindpay::{BlindPay, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client with your API credentials
    let client = BlindPay::new(
        "your-api-key-here",
        "your-instance-id-here"
    )?;

    // Example 1: Get available rails
    println!("=== Available Rails ===");
    let rails = client.available().get_rails().await?;
    for rail in &rails {
        println!("{} - {} ({})", rail.label, rail.value, rail.country);
    }

    // Example 2: List receivers
    println!("\n=== Receivers ===");
    match client.receivers().list().await {
        Ok(receivers) => {
            println!("Found {} receivers", receivers.len());
            for receiver in receivers.iter().take(5) {
                println!("  - {} ({})", receiver.id, receiver.email);
            }
        }
        Err(e) => {
            eprintln!("Error listing receivers: {}", e);
        }
    }

    // Example 3: List payouts
    println!("\n=== Payouts ===");
    match client.payouts().list(None).await {
        Ok(response) => {
            println!("Found {} payouts", response.data.len());
            println!("Has more: {}", response.pagination.has_more);
            
            for payout in response.data.iter().take(3) {
                println!("  - {} ({:?})", payout.id, payout.status);
            }
        }
        Err(e) => {
            eprintln!("Error listing payouts: {}", e);
        }
    }

    // Example 4: List partner fees
    println!("\n=== Partner Fees ===");
    match client.partner_fees().list().await {
        Ok(fees) => {
            println!("Found {} partner fees", fees.len());
            for fee in &fees {
                println!("  - {}: {} ({}% payout fee)", 
                    fee.id, fee.name, fee.payout_percentage_fee);
            }
        }
        Err(e) => {
            eprintln!("Error listing partner fees: {}", e);
        }
    }

    println!("\n✅ All examples completed successfully!");
    Ok(())
}
