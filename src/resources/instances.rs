use crate::client::BlindPay;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstanceMemberRole {
    Owner,
    Admin,
    Finance,
    Checker,
    Operations,
    Developer,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceMember {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub image_url: String,
    pub created_at: String,
    pub role: InstanceMemberRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstanceInput {
    pub name: String,
    pub receiver_invite_redirect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstanceMemberRoleInput {
    pub member_id: String,
    pub role: InstanceMemberRole,
}

pub struct InstancesResource {
    client: BlindPay,
}

impl InstancesResource {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    /// Get instance members
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let members = client.instances().get_members().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_members(&self) -> Result<Vec<InstanceMember>> {
        let path = format!("/instances/{}/members", self.client.instance_id());
        self.client.get(&path).await
    }

    /// Update instance details
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::instances::UpdateInstanceInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = UpdateInstanceInput {
    ///     name: "New Instance Name".to_string(),
    ///     receiver_invite_redirect_url: None,
    /// };
    /// client.instances().update(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, input: UpdateInstanceInput) -> Result<()> {
        let path = format!("/instances/{}", self.client.instance_id());
        self.client.put(&path, input).await
    }

    /// Delete the instance
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// client.instances().delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self) -> Result<()> {
        let path = format!("/instances/{}", self.client.instance_id());
        self.client.delete(&path).await
    }

    /// Delete an instance member
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// client.instances().delete_member("us_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_member(&self, member_id: &str) -> Result<()> {
        let path = format!(
            "/instances/{}/members/{}",
            self.client.instance_id(),
            member_id
        );
        self.client.delete(&path).await
    }

    /// Update an instance member's role
    ///
    /// # Example
    /// ```no_run
    /// # use blindpay::BlindPay;
    /// # use blindpay::resources::instances::{UpdateInstanceMemberRoleInput, InstanceMemberRole};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlindPay::new("api-key", "instance-id")?;
    /// let input = UpdateInstanceMemberRoleInput {
    ///     member_id: "us_123".to_string(),
    ///     role: InstanceMemberRole::Admin,
    /// };
    /// client.instances().update_member_role(input).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_member_role(&self, input: UpdateInstanceMemberRoleInput) -> Result<()> {
        let member_id = input.member_id.clone();
        let path = format!(
            "/instances/{}/members/{}",
            self.client.instance_id(),
            member_id
        );
        let body = serde_json::json!({ "role": input.role });
        self.client.put(&path, body).await
    }
}

// API Keys sub-resource
use crate::resources::api_keys::ApiKeysResource;
pub use crate::resources::api_keys;

// Webhooks sub-resource
use crate::resources::webhooks::WebhookEndpointsResource;
pub use crate::resources::webhooks;

// Terms of Service sub-resource
use crate::resources::terms_of_service::TermsOfServiceResource;
pub use crate::resources::terms_of_service;

impl InstancesResource {
    /// Access API keys sub-resource
    pub fn api_keys(&self) -> ApiKeysResource {
        ApiKeysResource::new(self.client.clone())
    }

    /// Access webhook endpoints sub-resource
    pub fn webhook_endpoints(&self) -> WebhookEndpointsResource {
        WebhookEndpointsResource::new(self.client.clone())
    }

    /// Access terms of service sub-resource
    pub fn tos(&self) -> TermsOfServiceResource {
        TermsOfServiceResource::new(self.client.clone())
    }
}
