use crate::client::BlindPay;
use crate::error::Result;
use crate::types::*;
use serde::{Deserialize, Serialize};

// Re-export bank accounts
use crate::resources::bank_accounts::BankAccountsResource;
pub use crate::resources::bank_accounts;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProofOfAddressDocType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    TaxDocument,
    GovernmentCorrespondence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurposeOfTransactions {
    BusinessTransactions,
    CharitableDonations,
    InvestmentPurposes,
    PaymentsToFriendsOrFamilyAbroad,
    PersonalOrLivingExpenses,
    ProtectWealth,
    PurchaseGoodAndServices,
    ReceivePaymentForFreelancing,
    ReceiveSalary,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceOfFundsDocType {
    BusinessIncome,
    GamblingProceeds,
    Gifts,
    GovernmentBenefits,
    Inheritance,
    InvestmentLoans,
    PensionRetirement,
    Salary,
    SaleOfAssetsRealEstate,
    Savings,
    Esops,
    InvestmentProceeds,
    SomeoneElseFunds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdentificationDocument {
    Passport,
    IdCard,
    Drivers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KycType {
    Light,
    Standard,
    Enhanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerRole {
    BeneficialControlling,
    BeneficialOwner,
    ControllingPerson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub id: Option<String>,
    pub role: OwnerRole,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub tax_id: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state_province_region: String,
    pub country: Country,
    pub postal_code: String,
    pub id_doc_country: Country,
    pub id_doc_type: IdentificationDocument,
    pub id_doc_front_file: String,
    pub id_doc_back_file: Option<String>,
    pub proof_of_address_doc_type: ProofOfAddressDocType,
    pub proof_of_address_doc_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycWarning {
    pub code: Option<String>,
    pub message: Option<String>,
    pub resolution_status: Option<String>,
    pub warning_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiverLimits {
    pub per_transaction: u64,
    pub daily: u64,
    pub monthly: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receiver {
    pub id: String,
    pub is_tos_accepted: bool,
    #[serde(rename = "type")]
    pub account_type: AccountClass,
    pub kyc_type: KycType,
    pub kyc_status: String,
    pub kyc_warnings: Option<Vec<KycWarning>>,
    pub email: String,
    pub tax_id: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state_province_region: String,
    pub country: Country,
    pub postal_code: String,
    pub ip_address: Option<String>,
    pub image_url: Option<String>,
    pub phone_number: Option<String>,
    pub proof_of_address_doc_type: ProofOfAddressDocType,
    pub proof_of_address_doc_file: String,
    // Individual fields
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub id_doc_country: Option<Country>,
    pub id_doc_type: Option<IdentificationDocument>,
    pub id_doc_front_file: Option<String>,
    pub id_doc_back_file: Option<String>,
    // Business fields
    pub legal_name: Option<String>,
    pub alternate_name: Option<String>,
    pub formation_date: Option<String>,
    pub website: Option<String>,
    pub owners: Option<Vec<Owner>>,
    pub incorporation_doc_file: Option<String>,
    pub proof_of_ownership_doc_file: Option<String>,
    // Enhanced KYC fields
    pub source_of_funds_doc_type: Option<String>,
    pub source_of_funds_doc_file: Option<String>,
    pub individual_holding_doc_front_file: Option<String>,
    pub purpose_of_transactions: Option<PurposeOfTransactions>,
    pub purpose_of_transactions_explanation: Option<String>,
    // Common fields
    pub aiprise_validation_key: String,
    pub instance_id: String,
    pub external_id: Option<String>,
    pub tos_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub limit: ReceiverLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndividualWithStandardKycInput {
    pub external_id: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub country: Country,
    pub date_of_birth: String,
    pub email: String,
    pub first_name: String,
    pub phone_number: Option<String>,
    pub id_doc_country: Country,
    pub id_doc_front_file: String,
    pub id_doc_type: IdentificationDocument,
    pub id_doc_back_file: Option<String>,
    pub last_name: String,
    pub postal_code: String,
    pub proof_of_address_doc_file: String,
    pub proof_of_address_doc_type: ProofOfAddressDocType,
    pub state_province_region: String,
    pub tax_id: String,
    pub tos_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReceiverResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReceiverLimitsResponse {
    pub limits: ReceiverLimitsBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiverLimitsBreakdown {
    pub payin: LimitValues,
    pub payout: LimitValues,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitValues {
    pub daily: u64,
    pub monthly: u64,
}

pub struct ReceiversResource {
    client: BlindPay,
}

impl ReceiversResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List all receivers
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let receivers = client.receivers().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<Receiver>> {
        let path = format!("/instances/{}/receivers", self.client.instance_id());
        self.client.get(&path).await
    }

    /// Create an individual receiver with standard KYC
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::receivers::*;
    /// # use blindpay::types::Country;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreateIndividualWithStandardKycInput {
    ///     email: "user@example.com".to_string(),
    ///     first_name: "John".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     // ... other fields
    /// #   external_id: None,
    /// #   address_line_1: "123 Main St".to_string(),
    /// #   address_line_2: None,
    /// #   city: "New York".to_string(),
    /// #   country: Country::US,
    /// #   date_of_birth: "1990-01-01".to_string(),
    /// #   phone_number: None,
    /// #   id_doc_country: Country::US,
    /// #   id_doc_front_file: "file_url".to_string(),
    /// #   id_doc_type: IdentificationDocument::Passport,
    /// #   id_doc_back_file: None,
    /// #   postal_code: "10001".to_string(),
    /// #   proof_of_address_doc_file: "file_url".to_string(),
    /// #   proof_of_address_doc_type: ProofOfAddressDocType::UtilityBill,
    /// #   state_province_region: "NY".to_string(),
    /// #   tax_id: "123456789".to_string(),
    /// #   tos_id: "tos_123".to_string(),
    /// };
    /// let receiver = client.receivers().create_individual_with_standard_kyc(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_individual_with_standard_kyc(
        &self,
        mut input: CreateIndividualWithStandardKycInput,
    ) -> Result<CreateReceiverResponse> {
        let path = format!("/instances/{}/receivers", self.client.instance_id());
        let mut body = serde_json::to_value(input)?;
        body["kyc_type"] = serde_json::json!("standard");
        body["type"] = serde_json::json!("individual");
        self.client.post(&path, body).await
    }

    /// Get a receiver by ID
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let receiver = client.receivers().get("re_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, receiver_id: &str) -> Result<Receiver> {
        let path = format!(
            "/instances/{}/receivers/{}",
            self.client.instance_id(),
            receiver_id
        );
        self.client.get(&path).await
    }

    /// Delete a receiver
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// client.receivers().delete("re_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, receiver_id: &str) -> Result<()> {
        let path = format!(
            "/instances/{}/receivers/{}",
            self.client.instance_id(),
            receiver_id
        );
        self.client.delete(&path).await
    }

    /// Get receiver limits
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let limits = client.receivers().get_limits("re_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_limits(&self, receiver_id: &str) -> Result<GetReceiverLimitsResponse> {
        let path = format!(
            "/instances/{}/limits/receivers/{}",
            self.client.instance_id(),
            receiver_id
        );
        self.client.get(&path).await
    }

    /// Access bank accounts sub-resource
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let accounts = client.receivers().bank_accounts().list("re_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn bank_accounts(&self) -> BankAccountsResource {
        BankAccountsResource::new(self.client.clone())
    }

    //TODO: Implement other receiver creation methods
}