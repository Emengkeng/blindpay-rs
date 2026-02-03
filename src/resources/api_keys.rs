use crate::client::BlindPay;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiKeyPermission {
    FullAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub permission: ApiKeyPermission,
    pub token: String,
    pub ip_whitelist: Option<Vec<String>>,
    pub unkey_id: String,
    pub last_used_at: Option<String>,
    pub instance_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyInput {
    pub name: String,
    pub permission: ApiKeyPermission,
    pub ip_whitelist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub id: String,
    pub token: String,
}

pub struct ApiKeysResource {
    client: BlindPay,
}

impl ApiKeysResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// List API keys
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let keys = client.instances().api_keys().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<ApiKey>> {
        let path = format!("/instances/{}/api-keys", self.client.instance_id());
        self.client.get(&path).await
    }

    /// Create an API key
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::api_keys::{CreateApiKeyInput, ApiKeyPermission};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = CreateApiKeyInput {
    ///     name: "Production API Key".to_string(),
    ///     permission: ApiKeyPermission::FullAccess,
    ///     ip_whitelist: Some(vec!["192.168.1.1".to_string()]),
    /// };
    /// let key = client.instances().api_keys().create(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: CreateApiKeyInput) -> Result<CreateApiKeyResponse> {
        let path = format!("/instances/{}/api-keys", self.client.instance_id());
        self.client.post(&path, input).await
    }

    /// Get an API key by ID
    pub async fn get(&self, id: &str) -> Result<ApiKey> {
        let path = format!("/instances/{}/api-keys/{}", self.client.instance_id(), id);
        self.client.get(&path).await
    }

    /// Delete an API key
    pub async fn delete(&self, id: &str) -> Result<()> {
        let path = format!("/instances/{}/api-keys/{}", self.client.instance_id(), id);
        self.client.delete(&path).await
    }
}
