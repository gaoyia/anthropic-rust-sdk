//! Models API，对齐上游 `src/resources/models.ts`。

use crate::client::Anthropic;
use crate::core::error::Error;
use crate::core::pagination::TokenPage;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 模型信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub display_name: String,
    pub created_at: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// Models API 资源。
pub struct Models<'a> {
    client: &'a Anthropic,
}

impl<'a> Models<'a> {
    pub(crate) fn new(client: &'a Anthropic) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        query: Option<&[(&str, &str)]>,
    ) -> Result<TokenPage<ModelInfo>, Error> {
        self.client.get_with_query("/v1/models", query).await
    }

    pub async fn retrieve(&self, model_id: &str) -> Result<ModelInfo, Error> {
        self.client.get(&format!("/v1/models/{model_id}")).await
    }
}
