use serde::{Deserialize, Serialize};

// Response wrapper types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlindPayApiResponse<T> {
    Success(BlindPaySuccessResponse<T>),
    Error(BlindPayErrorResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindPaySuccessResponse<T> {
    pub data: T,
    pub error: Option<()>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindPayErrorResponse {
    pub data: Option<()>,
    pub error: ErrorResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

// Enums
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyType {
    Sender,
    Receiver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Network {
    Base,
    Sepolia,
    ArbitrumSepolia,
    BaseSepolia,
    Arbitrum,
    Polygon,
    PolygonAmoy,
    Ethereum,
    Stellar,
    StellarTestnet,
    Tron,
    Solana,
    SolanaDevnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StablecoinToken {
    USDC,
    USDT,
    USDB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionDocumentType {
    Invoice,
    PurchaseOrder,
    DeliverySlip,
    Contract,
    CustomsDeclaration,
    BillOfLading,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountType {
    Checking,
    Saving,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    USDC,
    USDT,
    USDB,
    BRL,
    USD,
    MXN,
    COP,
    ARS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rail {
    Wire,
    Ach,
    Pix,
    SpeiBitso,
    TransfersBitso,
    AchCopBitso,
    InternationalSwift,
    Rtp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountClass {
    Individual,
    Business,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Refunded,
    Processing,
    Completed,
    Failed,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Country {
    US,
    BR,
    MX,
    AR,
    CO,
    // Add other countries as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub limit: Option<String>,
    pub offset: Option<String>,
    pub starting_after: Option<String>,
    pub ending_before: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMetadata {
    pub has_more: bool,
    pub next_page: String,
    pub prev_page: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgentinaTransfers {
    CVU,
    CBU,
    ALIAS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchCopDocument {
    CC,
    CE,
    NIT,
    PASS,
    PEP,
}

// Tracking types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackingStatus {
    Processing,
    OnHold,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstimatedTimeOfArrival {
    #[serde(rename = "5_min")]
    FiveMin,
    #[serde(rename = "30_min")]
    ThirtyMin,
    #[serde(rename = "2_hours")]
    TwoHours,
    #[serde(rename = "1_business_day")]
    OneBusinessDay,
    #[serde(rename = "2_business_days")]
    TwoBusinessDays,
    #[serde(rename = "5_business_days")]
    FiveBusinessDays,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayinTrackingTransaction {
    pub step: TrackingStatus,
    pub status: Option<String>,
    pub external_id: Option<String>,
    pub completed_at: Option<String>,
    pub sender_name: Option<String>,
    pub sender_tax_id: Option<String>,
    pub sender_bank_code: Option<String>,
    pub sender_account_number: Option<String>,
    pub trace_number: Option<String>,
    pub transaction_reference: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayinTrackingPayment {
    pub step: TrackingStatus,
    pub provider_name: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayinTrackingComplete {
    pub step: TrackingStatus,
    pub transaction_hash: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayinTrackingPartnerFee {
    pub step: TrackingStatus,
    pub transaction_hash: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTrackingTransaction {
    pub step: TrackingStatus,
    pub status: String,
    pub transaction_hash: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTrackingPayment {
    pub step: TrackingStatus,
    pub provider_name: Option<String>,
    pub provider_transaction_id: Option<String>,
    pub provider_status: Option<String>,
    pub recipient_name: Option<String>,
    pub recipient_tax_id: Option<String>,
    pub recipient_bank_code: Option<String>,
    pub recipient_branch_code: Option<String>,
    pub recipient_account_number: Option<String>,
    pub recipient_account_type: Option<String>,
    pub estimated_time_of_arrival: Option<EstimatedTimeOfArrival>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTrackingLiquidity {
    pub step: TrackingStatus,
    pub provider_transaction_id: Option<String>,
    pub provider_status: Option<String>,
    pub estimated_time_of_arrival: Option<EstimatedTimeOfArrival>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTrackingComplete {
    pub step: TrackingStatus,
    pub status: Option<String>,
    pub transaction_hash: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTrackingPartnerFee {
    pub step: TrackingStatus,
    pub transaction_hash: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayinPaymentMethod {
    Ach,
    Wire,
    Pix,
    Spei,
    Transfers,
    Pse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayerRules {
    pub pix_allowed_tax_ids: Option<Vec<String>>,
    pub transfers_allowed_tax_id: Option<String>,
    pub pse_allowed_tax_ids: Option<Vec<String>>,
    pub pse_full_name: Option<String>,
    pub pse_document_type: Option<AchCopDocument>,
    pub pse_document_number: Option<String>,
    pub pse_email: Option<String>,
    pub pse_phone: Option<String>,
    pub pse_bank_code: Option<String>,
}
