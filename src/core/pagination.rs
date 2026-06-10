//! 分页类型，对齐上游 `src/core/pagination.ts`。

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 游标分页响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageCursor<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

impl<T> PageCursor<T> {
    pub fn items(&self) -> &[T] {
        &self.data
    }

    pub fn has_next_page(&self) -> bool {
        self.has_more || self.next_page.is_some()
    }
}

/// Token 分页响应（Models API）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPage<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

impl<T> TokenPage<T> {
    pub fn items(&self) -> &[T] {
        &self.data
    }

    pub fn has_next_page(&self) -> bool {
        self.has_more || self.next_page_token.is_some()
    }
}
