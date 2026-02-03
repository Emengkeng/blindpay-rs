use crate::client::BlindPay;
use crate::error::Result;
use crate::types::*;
use serde::{Deserialize, Serialize};

// Re-export payin quotes
use crate::resources::quotes::PayinQuotesResource;
pub use crate::resources::quotes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payin {
    pub id: String,
    pub receiver_id: String,
    pub status: TransactionStatus,
    pub payin_quote_id: String,
    pub instance_id: String,
    pub tracking_transaction: PayinTrackingTransaction,
    pub tracking_payment: PayinTrackingPayment,
    pub tracking_complete: PayinTrackingComplete,
    pub tracking_partner_fee: Option<PayinTrackingPartnerFee>,
    pub created_at: String,
    pub updated_at: String,
    pub payment_method: PayinPaymentMethod,
    pub sender_amount: f64,
    pub receiver_amount: f64,
    pub token: StablecoinToken,
    pub currency: Currency,
    pub network: Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPayinsResponse {
    pub data: Vec<Payin>,
    pub pagination: PaginationMetadata,
}

pub struct PayinsResource {
    client: BlindPay,
}

impl PayinsResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List payins
    pub async fn list(&self, params: Option<PaginationParams>) -> Result<ListPayinsResponse> {
        let mut path = format!("/instances/{}/payins", self.client.instance_id());
        
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

    /// Get a payin by ID
    pub async fn get(&self, payin_id: &str) -> Result<Payin> {
        let path = format!(
            "/instances/{}/payins/{}",
            self.client.instance_id(),
            payin_id
        );
        self.client.get(&path).await
    }

    /// Get payin tracking information
    pub async fn get_track(&self, payin_id: &str) -> Result<Payin> {
        let path = format!("/e/payins/{}", payin_id);
        self.client.get(&path).await
    }

    /// Create an EVM payin
    pub async fn create_evm(&self, payin_quote_id: &str) -> Result<Payin> {
        let path = format!("/instances/{}/payins/evm", self.client.instance_id());
        let body = serde_json::json!({ "payin_quote_id": payin_quote_id });
        self.client.post(&path, body).await
    }

    /// Access payin quotes sub-resource
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// // Create a payin quote
    /// # Ok(())
    /// # }
    /// ```
    pub fn quotes(&self) -> PayinQuotesResource {
        PayinQuotesResource::new(self.client.clone())
    }
}
