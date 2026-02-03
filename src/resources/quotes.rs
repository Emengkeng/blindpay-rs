use crate::client::BlindPay;
use crate::error::Result;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub abi: Vec<HashMap<String, serde_json::Value>>,
    pub address: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "blindpayContractAddress")]
    pub blindpay_contract_address: String,
    pub amount: String,
    pub network: NetworkInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteInput {
    pub bank_account_id: String,
    pub currency_type: CurrencyType,
    pub cover_fees: bool,
    pub request_amount: f64,
    pub network: Network,
    pub token: Option<StablecoinToken>,
    pub description: Option<String>,
    pub partner_fee_id: Option<String>,
    pub transaction_document_file: Option<String>,
    pub transaction_document_id: Option<String>,
    pub transaction_document_type: Option<TransactionDocumentType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteResponse {
    pub id: String,
    pub expires_at: i64,
    pub commercial_quotation: f64,
    pub blindpay_quotation: f64,
    pub receiver_amount: f64,
    pub sender_amount: f64,
    pub partner_fee_amount: Option<f64>,
    pub flat_fee: Option<f64>,
    pub contract: Option<ContractInfo>,
    pub receiver_local_amount: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFxRateInput {
    pub currency_type: CurrencyType,
    pub from: StablecoinToken,
    pub to: Currency,
    pub request_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFxRateResponse {
    pub commercial_quotation: f64,
    pub blindpay_quotation: f64,
    pub result_amount: f64,
    pub instance_flat_fee: Option<f64>,
    pub instance_percentage_fee: f64,
}

pub struct QuotesResource {
    client: BlindPay,
}

impl QuotesResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// Create a quote for a payout
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::quotes::CreateQuoteInput;
    /// # use blindpay::types::{CurrencyType, Network, StablecoinToken};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreateQuoteInput {
    ///     bank_account_id: "ba_123".to_string(),
    ///     currency_type: CurrencyType::Sender,
    ///     cover_fees: true,
    ///     request_amount: 1000.0,
    ///     network: Network::Polygon,
    ///     token: Some(StablecoinToken::USDC),
    ///     description: None,
    ///     partner_fee_id: None,
    ///     transaction_document_file: None,
    ///     transaction_document_id: None,
    ///     transaction_document_type: None,
    /// };
    /// let quote = client.quotes().create(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: CreateQuoteInput) -> Result<CreateQuoteResponse> {
        let path = format!("/instances/{}/quotes", self.client.instance_id());
        self.client.post(&path, input).await
    }

    /// Get FX rate for currency conversion
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::quotes::GetFxRateInput;
    /// # use blindpay::types::{CurrencyType, StablecoinToken, Currency};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = GetFxRateInput {
    ///     currency_type: CurrencyType::Sender,
    ///     from: StablecoinToken::USDC,
    ///     to: Currency::BRL,
    ///     request_amount: 1000.0,
    /// };
    /// let rate = client.quotes().get_fx_rate(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fx_rate(&self, input: GetFxRateInput) -> Result<GetFxRateResponse> {
        let path = format!("/instances/{}/quotes/fx", self.client.instance_id());
        self.client.post(&path, input).await
    }
}

// Payin Quotes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayinQuoteInput {
    pub blockchain_wallet_id: String,
    pub currency_type: CurrencyType,
    pub payment_method: PayinPaymentMethod,
    pub request_amount: f64,
    pub token: StablecoinToken,
    pub is_otc: Option<bool>,
    pub cover_fees: bool,
    pub partner_fee_id: Option<String>,
    pub payer_rules: Option<PayerRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayinQuoteResponse {
    pub id: String,
    pub expires_at: i64,
    pub commercial_quotation: f64,
    pub blindpay_quotation: f64,
    pub receiver_amount: f64,
    pub sender_amount: f64,
    pub partner_fee_amount: Option<f64>,
    pub flat_fee: f64,
    pub is_otc: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPayinFxRateInput {
    pub currency_type: CurrencyType,
    pub from: Currency,
    pub to: Currency,
    pub request_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPayinFxRateResponse {
    pub commercial_quotation: f64,
    pub blindpay_quotation: f64,
    pub result_amount: f64,
    pub instance_flat_fee: f64,
    pub instance_percentage_fee: f64,
}

pub struct PayinQuotesResource {
    client: BlindPay,
}

impl PayinQuotesResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// Create a payin quote
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::quotes::CreatePayinQuoteInput;
    /// # use blindpay::types::{CurrencyType, PayinPaymentMethod, StablecoinToken};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreatePayinQuoteInput {
    ///     blockchain_wallet_id: "bw_123".to_string(),
    ///     currency_type: CurrencyType::Sender,
    ///     payment_method: PayinPaymentMethod::Pix,
    ///     request_amount: 5000.0,
    ///     token: StablecoinToken::USDC,
    ///     is_otc: None,
    ///     cover_fees: true,
    ///     partner_fee_id: None,
    ///     payer_rules: None,
    /// };
    /// let quote = client.payins().quotes().create(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: CreatePayinQuoteInput) -> Result<CreatePayinQuoteResponse> {
        let path = format!("/instances/{}/payin-quotes", self.client.instance_id());
        self.client.post(&path, input).await
    }

    /// Get FX rate for payin
    pub async fn get_fx_rate(&self, input: GetPayinFxRateInput) -> Result<GetPayinFxRateResponse> {
        let path = format!("/instances/{}/payin-quotes/fx", self.client.instance_id());
        self.client.post(&path, input).await
    }
}
