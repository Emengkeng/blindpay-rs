use crate::error::{BlindPayError, Result};
use crate::resources::{
    available::AvailableResource, instances::InstancesResource, partner_fees::PartnerFeesResource,
    payins::PayinsResource, payouts::PayoutsResource, quotes::QuotesResource,
    receivers::ReceiversResource, virtual_accounts::VirtualAccountsResource,
    wallets::WalletsResources,
};
use crate::types::{BlindPayApiResponse, BlindPayErrorResponse, BlindPaySuccessResponse};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

const BASE_URL: &str = "https://api.blindpay.com/v1";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main BlindPay SDK client
#[derive(Clone)]
pub struct BlindPay {
    client: Client,
    api_key: String,
    instance_id: String,
    base_url: String,
}

impl BlindPay {
    /// Create a new BlindPay client
    ///
    /// # Arguments
    /// * `api_key` - Your BlindPay API key
    /// * `instance_id` - Your BlindPay instance ID
    ///
    /// # Example
    /// ```no_run
    /// use blindpay::BlindPay;
    ///
    /// let client = BlindPay::new("your-api-key", "your-instance-id").unwrap();
    /// ```
    pub fn new(api_key: impl Into<String>, instance_id: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        let instance_id = instance_id.into();

        if api_key.is_empty() {
            return Err(BlindPayError::MissingApiKey);
        }

        if instance_id.is_empty() {
            return Err(BlindPayError::MissingInstanceId);
        }

        let client = Client::builder()
            .user_agent(format!("blindpay-rust/{}", VERSION))
            .build()?;

        Ok(Self {
            client,
            api_key,
            instance_id,
            base_url: BASE_URL.to_string(),
        })
    }

    /// Get the available resource
    pub fn available(&self) -> AvailableResource {
        AvailableResource::new(self.clone())
    }

    /// Get the instances resource
    pub fn instances(&self) -> InstancesResource {
        InstancesResource::new(self.clone())
    }

    /// Get the partner fees resource
    pub fn partner_fees(&self) -> PartnerFeesResource {
        PartnerFeesResource::new(self.clone())
    }

    /// Get the payins resource
    pub fn payins(&self) -> PayinsResource {
        PayinsResource::new(self.clone())
    }

    /// Get the payouts resource
    pub fn payouts(&self) -> PayoutsResource {
        PayoutsResource::new(self.clone())
    }

    /// Get the quotes resource
    pub fn quotes(&self) -> QuotesResource {
        QuotesResource::new(self.clone())
    }

    /// Get the receivers resource
    pub fn receivers(&self) -> ReceiversResource {
        ReceiversResource::new(self.clone())
    }

    /// Get the virtual accounts resource
    pub fn virtual_accounts(&self) -> VirtualAccountsResource {
        VirtualAccountsResource::new(self.clone())
    }

    /// Get the wallets resources
    pub fn wallets(&self) -> WalletsResources {
        WalletsResources::new(self.clone())
    }

    // Internal HTTP methods
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(Method::GET, path, None::<()>).await
    }

    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: B,
    ) -> Result<T> {
        self.request(Method::POST, path, Some(body)).await
    }

    pub(crate) async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: B,
    ) -> Result<T> {
        self.request(Method::PUT, path, Some(body)).await
    }

    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: B,
    ) -> Result<T> {
        self.request(Method::PATCH, path, Some(body)).await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(Method::DELETE, path, None::<()>).await
    }

    async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<B>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .client
            .request(method, &url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key));

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body: BlindPayErrorResponse = response.json().await?;
            return Err(BlindPayError::ApiError(error_body.error.message));
        }

        let api_response: BlindPayApiResponse<T> = response.json().await?;

        match api_response {
            BlindPayApiResponse::Success(success) => Ok(success.data),
            BlindPayApiResponse::Error(error) => Err(BlindPayError::ApiError(error.error.message)),
        }
    }

    pub(crate) fn instance_id(&self) -> &str {
        &self.instance_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client() {
        let client = BlindPay::new("test-api-key", "test-instance-id");
        assert!(client.is_ok());
    }

    #[test]
    fn test_missing_api_key() {
        let client = BlindPay::new("", "test-instance-id");
        assert!(matches!(client, Err(BlindPayError::MissingApiKey)));
    }

    #[test]
    fn test_missing_instance_id() {
        let client = BlindPay::new("test-api-key", "");
        assert!(matches!(client, Err(BlindPayError::MissingInstanceId)));
    }
}
