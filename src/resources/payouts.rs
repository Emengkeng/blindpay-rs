use crate::client::BlindPay;
use crate::error::Result;
use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub receiver_id: String,
    pub id: String,
    pub status: TransactionStatus,
    pub sender_wallet_address: String,
    pub signed_transaction: String,
    pub quote_id: String,
    pub instance_id: String,
    pub tracking_transaction: PayoutTrackingTransaction,
    pub tracking_payment: PayoutTrackingPayment,
    pub tracking_liquidity: PayoutTrackingLiquidity,
    pub tracking_complete: PayoutTrackingComplete,
    pub tracking_partner_fee: PayoutTrackingPartnerFee,
    pub created_at: String,
    pub updated_at: String,
    pub image_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub legal_name: Option<String>,
    pub network: Network,
    pub token: StablecoinToken,
    pub description: String,
    pub sender_amount: f64,
    pub receiver_amount: f64,
    pub partner_fee_amount: f64,
    pub commercial_quotation: f64,
    pub blindpay_quotation: f64,
    pub total_fee_amount: f64,
    pub receiver_local_amount: f64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPayoutsResponse {
    pub data: Vec<Payout>,
    pub pagination: PaginationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStellarPayoutInput {
    pub quote_id: String,
    pub sender_wallet_address: String,
    pub signed_transaction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayoutResponse {
    pub id: String,
    pub status: TransactionStatus,
    pub sender_wallet_address: String,
    pub tracking_complete: Option<PayoutTrackingComplete>,
    pub tracking_payment: Option<PayoutTrackingPayment>,
    pub tracking_transaction: Option<PayoutTrackingTransaction>,
    pub tracking_partner_fee: Option<PayoutTrackingPartnerFee>,
    pub tracking_liquidity: Option<PayoutTrackingLiquidity>,
    pub receiver_id: String,
}

pub struct PayoutsResource {
    client: BlindPay,
}

impl PayoutsResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List payouts
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let payouts = client.payouts().list(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: Option<PaginationParams>) -> Result<ListPayoutsResponse> {
        let mut path = format!("/instances/{}/payouts", self.client.instance_id());
        
        if let Some(p) = params {
            let mut query_params = vec![];
            if let Some(limit) = p.limit {
                query_params.push(format!("limit={}", limit));
            }
            if let Some(offset) = p.offset {
                query_params.push(format!("offset={}", offset));
            }
            if !query_params.is_empty() {
                path.push('?');
                path.push_str(&query_params.join("&"));
            }
        }
        
        self.client.get(&path).await
    }

    /// Get a payout by ID
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let payout = client.payouts().get("pa_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, payout_id: &str) -> Result<Payout> {
        let path = format!(
            "/instances/{}/payouts/{}",
            self.client.instance_id(),
            payout_id
        );
        self.client.get(&path).await
    }

    /// Get payout tracking information
    pub async fn get_track(&self, payout_id: &str) -> Result<Payout> {
        let path = format!("/e/payouts/{}", payout_id);
        self.client.get(&path).await
    }

    /// Create a Stellar payout
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::payouts::CreateStellarPayoutInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreateStellarPayoutInput {
    ///     quote_id: "qu_123".to_string(),
    ///     sender_wallet_address: "GABC...".to_string(),
    ///     signed_transaction: None,
    /// };
    /// let payout = client.payouts().create_stellar(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_stellar(
        &self,
        input: CreateStellarPayoutInput,
    ) -> Result<CreatePayoutResponse> {
        let path = format!("/instances/{}/payouts/stellar", self.client.instance_id());
        self.client.post(&path, input).await
    }

    /// Create an EVM payout
    pub async fn create_evm(
        &self,
        quote_id: &str,
        sender_wallet_address: &str,
    ) -> Result<CreatePayoutResponse> {
        let path = format!("/instances/{}/payouts/evm", self.client.instance_id());
        let body = serde_json::json!({
            "quote_id": quote_id,
            "sender_wallet_address": sender_wallet_address,
        });
        self.client.post(&path, body).await
    }

    /// Create a Solana payout
    pub async fn create_solana(
        &self,
        quote_id: &str,
        sender_wallet_address: &str,
        signed_transaction: Option<String>,
    ) -> Result<CreatePayoutResponse> {
        let path = format!("/instances/{}/payouts/solana", self.client.instance_id());
        let body = serde_json::json!({
            "quote_id": quote_id,
            "sender_wallet_address": sender_wallet_address,
            "signed_transaction": signed_transaction,
        });
        self.client.post(&path, body).await
    }
}
