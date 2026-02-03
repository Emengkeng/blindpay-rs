use crate::client::BlindPay;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateInput {
    pub idempotency_key: String,
    pub receiver_id: Option<String>,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateResponse {
    pub url: String,
}

pub struct TermsOfServiceResource {
    client: BlindPay,
}

impl TermsOfServiceResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// Initiate terms of service acceptance flow
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::terms_of_service::InitiateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = InitiateInput {
    ///     idempotency_key: "unique-key-123".to_string(),
    ///     receiver_id: Some("re_123".to_string()),
    ///     redirect_url: Some("https://example.com/return".to_string()),
    /// };
    /// let response = client.instances().tos().initiate(input).await?;
    /// println!("TOS URL: {}", response.url);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initiate(&self, input: InitiateInput) -> Result<InitiateResponse> {
        let path = format!("/e/instances/{}/tos", self.client.instance_id());
        self.client.post(&path, input).await
    }
}
