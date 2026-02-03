use crate::client::BlindPay;
use crate::error::Result;
use crate::types::Rail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDetail {
    pub label: String,
    pub regex: String,
    pub key: String,
    pub items: Option<Vec<BankDetailItem>>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDetailItem {
    pub label: String,
    pub value: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailInfo {
    pub label: String,
    pub value: Rail,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftCodeBankDetail {
    pub id: String,
    pub bank: String,
    pub city: String,
    pub branch: String,
    #[serde(rename = "swiftCode")]
    pub swift_code: String,
    #[serde(rename = "swiftCodeLink")]
    pub swift_code_link: String,
    pub country: String,
    #[serde(rename = "countrySlug")]
    pub country_slug: String,
}

pub struct AvailableResource {
    client: BlindPay,
}

impl AvailableResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// Get bank details for a specific rail
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::types::Rail;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let details = client.available().get_bank_details(Rail::Pix).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bank_details(&self, rail: Rail) -> Result<Vec<BankDetail>> {
        let rail_str = serde_json::to_string(&rail)?;
        let rail_str = rail_str.trim_matches('"');
        self.client
            .get(&format!("/available/bank-details?rail={}", rail_str))
            .await
    }

    /// Get available rails
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let rails = client.available().get_rails().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_rails(&self) -> Result<Vec<RailInfo>> {
        self.client.get("/available/rails").await
    }

    /// Get bank details for a SWIFT code
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let details = client.available().get_swift_code_bank_details("BOFAUS3NLMA").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_swift_code_bank_details(
        &self,
        swift_code: &str,
    ) -> Result<Vec<SwiftCodeBankDetail>> {
        self.client
            .get(&format!("/available/swift/{}", swift_code))
            .await
    }
}
