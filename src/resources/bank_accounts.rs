use crate::client::BlindPay;
use crate::error::Result;
use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeiProtocol {
    Clabe,
    Debitcard,
    Phonenum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: Rail,
    pub name: String,
    // PIX fields
    pub pix_key: Option<String>,
    // Common fields
    pub beneficiary_name: Option<String>,
    pub routing_number: Option<String>,
    pub account_number: Option<String>,
    pub account_type_detail: Option<BankAccountType>,
    pub account_class: Option<AccountClass>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state_province_region: Option<String>,
    pub country: Option<Country>,
    pub postal_code: Option<String>,
    // SPEI fields
    pub spei_protocol: Option<String>,
    pub spei_institution_code: Option<String>,
    pub spei_clabe: Option<String>,
    // Argentina transfers fields
    pub transfers_type: Option<ArgentinaTransfers>,
    pub transfers_account: Option<String>,
    // ACH Colombia fields
    pub ach_cop_beneficiary_first_name: Option<String>,
    pub ach_cop_beneficiary_last_name: Option<String>,
    pub ach_cop_document_id: Option<String>,
    pub ach_cop_document_type: Option<AchCopDocument>,
    pub ach_cop_email: Option<String>,
    pub ach_cop_bank_code: Option<String>,
    pub ach_cop_bank_account: Option<String>,
    // SWIFT fields
    pub swift_code_bic: Option<String>,
    pub swift_account_holder_name: Option<String>,
    pub swift_account_number_iban: Option<String>,
    pub swift_beneficiary_address_line_1: Option<String>,
    pub swift_beneficiary_address_line_2: Option<String>,
    pub swift_beneficiary_country: Option<Country>,
    pub swift_beneficiary_city: Option<String>,
    pub swift_beneficiary_state_province_region: Option<String>,
    pub swift_beneficiary_postal_code: Option<String>,
    pub swift_bank_name: Option<String>,
    pub swift_bank_address_line_1: Option<String>,
    pub swift_bank_address_line_2: Option<String>,
    pub swift_bank_country: Option<Country>,
    pub swift_bank_city: Option<String>,
    pub swift_bank_state_province_region: Option<String>,
    pub swift_bank_postal_code: Option<String>,
    pub swift_intermediary_bank_swift_code_bic: Option<String>,
    pub swift_intermediary_bank_account_number_iban: Option<String>,
    pub swift_intermediary_bank_name: Option<String>,
    pub swift_intermediary_bank_country: Option<Country>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBankAccountsResponse {
    pub data: Vec<BankAccount>,
}

// PIX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePixInput {
    pub receiver_id: String,
    pub name: String,
    pub pix_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePixResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub name: String,
    pub pix_key: String,
    pub created_at: String,
}

// Argentina Transfers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgentinaTransfersInput {
    pub receiver_id: String,
    pub name: String,
    pub beneficiary_name: String,
    pub transfers_account: String,
    pub transfers_type: ArgentinaTransfers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgentinaTransfersResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub name: String,
    pub beneficiary_name: String,
    pub transfers_type: ArgentinaTransfers,
    pub transfers_account: String,
    pub created_at: String,
}

// SPEI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpeiInput {
    pub receiver_id: String,
    pub beneficiary_name: String,
    pub name: String,
    pub spei_clabe: String,
    pub spei_institution_code: String,
    pub spei_protocol: SpeiProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpeiResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub name: String,
    pub beneficiary_name: String,
    pub spei_protocol: SpeiProtocol,
    pub spei_institution_code: String,
    pub spei_clabe: String,
    pub created_at: String,
}

// ACH Colombia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateColombiaAchInput {
    pub receiver_id: String,
    pub name: String,
    pub account_type: BankAccountType,
    pub ach_cop_beneficiary_first_name: String,
    pub ach_cop_beneficiary_last_name: String,
    pub ach_cop_document_id: String,
    pub ach_cop_document_type: AchCopDocument,
    pub ach_cop_email: String,
    pub ach_cop_bank_code: String,
    pub ach_cop_bank_account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateColombiaAchResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub name: String,
    pub account_type_detail: BankAccountType,
    pub ach_cop_beneficiary_first_name: String,
    pub ach_cop_beneficiary_last_name: String,
    pub ach_cop_document_id: String,
    pub ach_cop_document_type: AchCopDocument,
    pub ach_cop_email: String,
    pub ach_cop_bank_code: String,
    pub ach_cop_bank_account: String,
    pub created_at: String,
}

// ACH
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAchInput {
    pub receiver_id: String,
    pub name: String,
    pub account_class: AccountClass,
    pub account_number: String,
    pub account_type: BankAccountType,
    pub beneficiary_name: String,
    pub routing_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAchResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub rail_type: String,
    pub name: String,
    pub beneficiary_name: String,
    pub routing_number: String,
    pub account_number: String,
    pub account_type: BankAccountType,
    pub account_class: AccountClass,
    pub created_at: String,
}

// Wire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWireInput {
    pub receiver_id: String,
    pub name: String,
    pub account_number: String,
    pub beneficiary_name: String,
    pub routing_number: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state_province_region: String,
    pub country: Country,
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWireResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub rail_type: String,
    pub name: String,
    pub beneficiary_name: String,
    pub routing_number: String,
    pub account_number: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state_province_region: String,
    pub country: Country,
    pub postal_code: String,
    pub created_at: String,
}

// International SWIFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInternationalSwiftInput {
    pub receiver_id: String,
    pub name: String,
    pub swift_account_holder_name: String,
    pub swift_account_number_iban: String,
    pub swift_bank_address_line_1: String,
    pub swift_bank_address_line_2: Option<String>,
    pub swift_bank_city: String,
    pub swift_bank_country: Country,
    pub swift_bank_name: String,
    pub swift_bank_postal_code: String,
    pub swift_bank_state_province_region: String,
    pub swift_beneficiary_address_line_1: String,
    pub swift_beneficiary_address_line_2: Option<String>,
    pub swift_beneficiary_city: String,
    pub swift_beneficiary_country: Country,
    pub swift_beneficiary_postal_code: String,
    pub swift_beneficiary_state_province_region: String,
    pub swift_code_bic: String,
    pub swift_intermediary_bank_account_number_iban: Option<String>,
    pub swift_intermediary_bank_country: Option<Country>,
    pub swift_intermediary_bank_name: Option<String>,
    pub swift_intermediary_bank_swift_code_bic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInternationalSwiftResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub rail_type: String,
    pub name: String,
    pub swift_code_bic: String,
    pub swift_account_holder_name: String,
    pub swift_account_number_iban: String,
    pub created_at: String,
}

// RTP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRtpInput {
    pub receiver_id: String,
    pub name: String,
    pub beneficiary_name: String,
    pub routing_number: String,
    pub account_number: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state_province_region: String,
    pub country: Country,
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRtpResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub rail_type: String,
    pub name: String,
    pub beneficiary_name: String,
    pub routing_number: String,
    pub account_number: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state_province_region: String,
    pub country: Country,
    pub postal_code: String,
    pub created_at: String,
}

pub struct BankAccountsResource {
    client: BlindPay,
}

impl BankAccountsResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List bank accounts for a receiver
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
    pub async fn list(&self, receiver_id: &str) -> Result<ListBankAccountsResponse> {
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        self.client.get(&path).await
    }

    /// Get a bank account by ID
    pub async fn get(&self, receiver_id: &str, id: &str) -> Result<BankAccount> {
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts/{}",
            self.client.instance_id(),
            receiver_id,
            id
        );
        self.client.get(&path).await
    }

    /// Delete a bank account
    pub async fn delete(&self, receiver_id: &str, id: &str) -> Result<()> {
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts/{}",
            self.client.instance_id(),
            receiver_id,
            id
        );
        self.client.delete(&path).await
    }

    /// Create a PIX bank account
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::bank_accounts::CreatePixInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreatePixInput {
    ///     receiver_id: "re_123".to_string(),
    ///     name: "My PIX Account".to_string(),
    ///     pix_key: "14947677768".to_string(),
    /// };
    /// let account = client.receivers().bank_accounts().create_pix(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_pix(&self, input: CreatePixInput) -> Result<CreatePixResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("pix");
        self.client.post(&path, body).await
    }

    /// Create an Argentina Transfers bank account
    pub async fn create_argentina_transfers(
        &self,
        input: CreateArgentinaTransfersInput,
    ) -> Result<CreateArgentinaTransfersResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("transfers_bitso");
        self.client.post(&path, body).await
    }

    /// Create a SPEI bank account
    pub async fn create_spei(&self, input: CreateSpeiInput) -> Result<CreateSpeiResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("spei_bitso");
        self.client.post(&path, body).await
    }

    /// Create a Colombia ACH bank account
    pub async fn create_colombia_ach(
        &self,
        input: CreateColombiaAchInput,
    ) -> Result<CreateColombiaAchResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("ach_cop_bitso");
        self.client.post(&path, body).await
    }

    /// Create an ACH bank account
    pub async fn create_ach(&self, input: CreateAchInput) -> Result<CreateAchResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("ach");
        self.client.post(&path, body).await
    }

    /// Create a Wire bank account
    pub async fn create_wire(&self, input: CreateWireInput) -> Result<CreateWireResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("wire");
        self.client.post(&path, body).await
    }

    /// Create an International SWIFT bank account
    pub async fn create_international_swift(
        &self,
        input: CreateInternationalSwiftInput,
    ) -> Result<CreateInternationalSwiftResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("international_swift");
        self.client.post(&path, body).await
    }

    /// Create an RTP bank account
    pub async fn create_rtp(&self, input: CreateRtpInput) -> Result<CreateRtpResponse> {
        let receiver_id = input.receiver_id.clone();
        let path = format!(
            "/instances/{}/receivers/{}/bank-accounts",
            self.client.instance_id(),
            receiver_id
        );
        let mut body = serde_json::to_value(input)?;
        body["type"] = serde_json::json!("rtp");
        self.client.post(&path, body).await
    }
}
