use crate::client::BlindPay;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfframpWallet {
    pub id: String,
    pub external_id: String,
    pub instance_id: String,
    pub receiver_id: String,
    pub bank_account_id: String,
    pub network: String,
    pub address: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOfframpWalletInput {
    pub receiver_id: String,
    pub bank_account_id: String,
    pub external_id: String,
    pub network: String,
}

pub struct OfframpWalletsResource {
    client: BlindPay,
}

impl OfframpWalletsResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List offramp wallets
    pub async fn list(
        &self,
        receiver_id: &str,
        bank_account_id: &str,
    ) -> Result<Vec<OfframpWallet>> {
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts/{}/offramp-wallets",
            self.client.instance_id(),
            receiver_id,
            bank_account_id
        );
        self.client.get(&path).await
    }

    /// Create an offramp wallet
    pub async fn create(&self, input: CreateOfframpWalletInput) -> Result<OfframpWallet> {
        let receiver_id = input.receiver_id.clone();
        let bank_account_id = input.bank_account_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts/{}/offramp-wallets",
            self.client.instance_id(),
            receiver_id,
            bank_account_id
        );
        self.client.post(&path, input).await
    }

    /// Get an offramp wallet
    pub async fn get(
        &self,
        receiver_id: &str,
        bank_account_id: &str,
        wallet_id: &str,
    ) -> Result<OfframpWallet> {
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts/{}/offramp-wallets/{}",
            self.client.instance_id(),
            receiver_id,
            bank_account_id,
            wallet_id
        );
        self.client.get(&path).await
    }
}
