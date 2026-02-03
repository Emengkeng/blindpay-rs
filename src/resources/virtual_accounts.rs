use crate::client::BlindPay;
use crate::error::Result;
use crate::types::{Network, StablecoinToken};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankingPartner {
    Jpmorgan,
    Citi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingDetails {
    pub routing_number: String,
    pub account_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsBankingInfo {
    pub ach: BankingDetails,
    pub wire: BankingDetails,
    pub rtp: BankingDetails,
    pub swift_bic_code: String,
    pub account_type: String,
    pub beneficiary: BeneficiaryInfo,
    pub receiving_bank: BankInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneficiaryInfo {
    pub name: String,
    pub address_line_1: String,
    pub address_line_2: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    pub name: String,
    pub address_line_1: String,
    pub address_line_2: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainWalletInfo {
    pub network: Network,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualAccount {
    pub id: String,
    pub banking_partner: BankingPartner,
    pub kyc_status: String,
    pub us: UsBankingInfo,
    pub token: StablecoinToken,
    pub blockchain_wallet_id: String,
    pub blockchain_wallet: Option<BlockchainWalletInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualAccountInput {
    pub receiver_id: String,
    pub banking_partner: BankingPartner,
    pub token: StablecoinToken,
    pub blockchain_wallet_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualAccountInput {
    pub receiver_id: String,
    pub id: String,
    pub token: StablecoinToken,
    pub blockchain_wallet_id: String,
}

pub struct VirtualAccountsResource {
    client: BlindPay,
}

impl VirtualAccountsResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List virtual accounts for a receiver
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let accounts = client.virtual_accounts().list("re_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, receiver_id: &str) -> Result<Vec<VirtualAccount>> {
        let path = format!(
            "/instances/{}/receivers/{}/virtual-accounts",
            self.client.instance_id(),
            receiver_id
        );
        self.client.get(&path).await
    }

    /// Get a virtual account by ID
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let account = client.virtual_accounts().get("re_123", "va_456").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, receiver_id: &str, id: &str) -> Result<VirtualAccount> {
        let path = format!(
            "/instances/{}/receivers/{}/virtual-accounts/{}",
            self.client.instance_id(),
            receiver_id,
            id
        );
        self.client.get(&path).await
    }

    /// Create a virtual account
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::virtual_accounts::*;
    /// # use blindpay::types::StablecoinToken;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreateVirtualAccountInput {
    ///     receiver_id: "re_123".to_string(),
    ///     banking_partner: BankingPartner::Jpmorgan,
    ///     token: StablecoinToken::USDC,
    ///     blockchain_wallet_id: "bw_456".to_string(),
    /// };
    /// let account = client.virtual_accounts().create(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: CreateVirtualAccountInput) -> Result<VirtualAccount> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/virtual-accounts",
            self.client.instance_id(),
            receiver_id
        );
        self.client.post(&path, input).await
    }

    /// Update a virtual account
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::virtual_accounts::UpdateVirtualAccountInput;
    /// # use blindpay::types::StablecoinToken;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = UpdateVirtualAccountInput {
    ///     receiver_id: "re_123".to_string(),
    ///     id: "va_456".to_string(),
    ///     token: StablecoinToken::USDT,
    ///     blockchain_wallet_id: "bw_789".to_string(),
    /// };
    /// client.virtual_accounts().update(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, input: UpdateVirtualAccountInput) -> Result<()> {
        let receiver_id = input.receiver_id.clone();
        let id = input.id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/virtual-accounts/{}",
            self.client.instance_id(),
            receiver_id,
            id
        );
        self.client.put(&path, input).await
    }
}
