use crate::client::BlindPay;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    #[serde(rename = "receiver.new")]
    ReceiverNew,
    #[serde(rename = "receiver.update")]
    ReceiverUpdate,
    #[serde(rename = "bankAccount.new")]
    BankAccountNew,
    #[serde(rename = "payout.new")]
    PayoutNew,
    #[serde(rename = "payout.update")]
    PayoutUpdate,
    #[serde(rename = "payout.complete")]
    PayoutComplete,
    #[serde(rename = "payout.partnerFee")]
    PayoutPartnerFee,
    #[serde(rename = "blockchainWallet.new")]
    BlockchainWalletNew,
    #[serde(rename = "payin.new")]
    PayinNew,
    #[serde(rename = "payin.update")]
    PayinUpdate,
    #[serde(rename = "payin.complete")]
    PayinComplete,
    #[serde(rename = "payin.partnerFee")]
    PayinPartnerFee,
    #[serde(rename = "tos.accept")]
    TosAccept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    pub id: String,
    pub url: String,
    pub events: Vec<WebhookEvent>,
    pub last_event_at: String,
    pub instance_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookEndpointInput {
    pub url: String,
    pub events: Vec<WebhookEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookEndpointResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebhookEndpointSecretResponse {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPortalAccessUrlResponse {
    pub url: String,
}

pub struct WebhookEndpointsResource {
    client: BlindPay,
}

impl WebhookEndpointsResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List webhook endpoints
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let endpoints = client.instances().webhook_endpoints().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<WebhookEndpoint>> {
        let path = format!(
            "/instances/{}/webhook-endpoints",
            self.client.instance_id()
        );
        self.client.get(&path).await
    }

    /// Create a webhook endpoint
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::webhooks::{CreateWebhookEndpointInput, WebhookEvent};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreateWebhookEndpointInput {
    ///     url: "https://example.com/webhook".to_string(),
    ///     events: vec![WebhookEvent::PayoutComplete, WebhookEvent::ReceiverNew],
    /// };
    /// let endpoint = client.instances().webhook_endpoints().create(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        input: CreateWebhookEndpointInput,
    ) -> Result<CreateWebhookEndpointResponse> {
        let path = format!(
            "/instances/{}/webhook-endpoints",
            self.client.instance_id()
        );
        self.client.post(&path, input).await
    }

    /// Delete a webhook endpoint
    pub async fn delete(&self, id: &str) -> Result<()> {
        let path = format!(
            "/instances/{}/webhook-endpoints/{}",
            self.client.instance_id(),
            id
        );
        self.client.delete(&path).await
    }

    /// Get webhook endpoint secret
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let secret = client.instances().webhook_endpoints().get_secret("we_123").await?;
    /// println!("Secret: {}", secret.key);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_secret(&self, id: &str) -> Result<GetWebhookEndpointSecretResponse> {
        let path = format!(
            "/instances/{}/webhook-endpoints/{}/secret",
            self.client.instance_id(),
            id
        );
        self.client.get(&path).await
    }

    /// Get portal access URL
    pub async fn get_portal_access_url(&self) -> Result<GetPortalAccessUrlResponse> {
        let path = format!(
            "/instances/{}/webhook-endpoints/portal-access",
            self.client.instance_id()
        );
        self.client.get(&path).await
    }
}
