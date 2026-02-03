use crate::client::BlindPay;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerFee {
    pub id: String,
    pub instance_id: String,
    pub name: String,
    pub payout_percentage_fee: f64,
    pub payout_flat_fee: f64,
    pub payin_percentage_fee: f64,
    pub payin_flat_fee: f64,
    pub evm_wallet_address: String,
    pub stellar_wallet_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartnerFeeInput {
    pub name: String,
    pub payout_percentage_fee: f64,
    pub payout_flat_fee: f64,
    pub payin_percentage_fee: f64,
    pub payin_flat_fee: f64,
    pub evm_wallet_address: String,
    pub stellar_wallet_address: Option<String>,
    pub virtual_account_set: Option<bool>,
}

pub struct PartnerFeesResource {
    client: BlindPay,
}

impl PartnerFeesResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List partner fees
    pub async fn list(&self) -> Result<Vec<PartnerFee>> {
        let path = format!("/instances/{}/partner-fees", self.client.instance_id());
        self.client.get(&path).await
    }

    /// Create a partner fee
    pub async fn create(&self, input: CreatePartnerFeeInput) -> Result<PartnerFee> {
        let path = format!("/instances/{}/partner-fees", self.client.instance_id());
        self.client.post(&path, input).await
    }

    /// Get a partner fee by ID
    pub async fn get(&self, id: &str) -> Result<PartnerFee> {
        let path = format!(
            "/instances/{}/partner-fees/{}",
            self.client.instance_id(),
            id
        );
        self.client.get(&path).await
    }

    /// Delete a partner fee
    pub async fn delete(&self, id: &str) -> Result<()> {
        let path = format!(
            "/instances/{}/partner-fees/{}",
            self.client.instance_id(),
            id
        );
        self.client.delete(&path).await
    }
}
