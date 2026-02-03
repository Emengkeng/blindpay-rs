use crate::client::BlindPay;
use crate::error::Result;
use crate::types::Network;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainWallet {
    pub id: String,
    pub name: String,
    pub network: Network,
    pub address: Option<String>,
    pub signature_tx_hash: Option<String>,
    pub is_account_abstraction: bool,
    pub receiver_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockchainWalletWithAddressInput {
    pub receiver_id: String,
    pub name: String,
    pub network: Network,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockchainWalletWithHashInput {
    pub receiver_id: String,
    pub name: String,
    pub network: Network,
    pub signature_tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletMessageResponse {
    pub message: String,
}

pub struct BlockchainWalletsResource {
    client: BlindPay,
}

impl BlockchainWalletsResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List blockchain wallets for a receiver
    pub async fn list(&self, receiver_id: &str) -> Result<Vec<BlockchainWallet>> {
        let path = format!(
            "/instances/{}/receivers/{}/blockchain-wallets",
            self.client.instance_id(),
            receiver_id
        );
        self.client.get(&path).await
    }

    /// Create a blockchain wallet with address (account abstraction)
    pub async fn create_with_address(
        &self,
        input: CreateBlockchainWalletWithAddressInput,
    ) -> Result<BlockchainWallet> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/blockchain-wallets",
            self.client.instance_id(),
            receiver_id
        );
        
        let mut body = serde_json::to_value(input)?;
        body["is_account_abstraction"] = serde_json::json!(true);
        
        self.client.post(&path, body).await
    }

    /// Create a blockchain wallet with signature hash
    pub async fn create_with_hash(
        &self,
        input: CreateBlockchainWalletWithHashInput,
    ) -> Result<BlockchainWallet> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/blockchain-wallets",
            self.client.instance_id(),
            receiver_id
        );
        
        let mut body = serde_json::to_value(input)?;
        body["is_account_abstraction"] = serde_json::json!(false);
        
        self.client.post(&path, body).await
    }

    /// Get wallet sign message
    pub async fn get_wallet_message(&self, receiver_id: &str) -> Result<GetWalletMessageResponse> {
        let path = format!(
            "/instances/{}/receivers/{}/blockchain-wallets/sign-message",
            self.client.instance_id(),
            receiver_id
        );
        self.client.get(&path).await
    }

    /// Get a blockchain wallet by ID
    pub async fn get(&self, receiver_id: &str, wallet_id: &str) -> Result<BlockchainWallet> {
        let path = format!(
            "/instances/{}/receivers/{}/blockchain-wallets/{}",
            self.client.instance_id(),
            receiver_id,
            wallet_id
        );
        self.client.get(&path).await
    }

    /// Delete a blockchain wallet
    pub async fn delete(&self, receiver_id: &str, wallet_id: &str) -> Result<()> {
        let path = format!(
            "/instances/{}/receivers/{}/blockchain-wallets/{}",
            self.client.instance_id(),
            receiver_id,
            wallet_id
        );
        self.client.delete(&path).await
    }
}
