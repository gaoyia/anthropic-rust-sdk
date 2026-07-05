//! 分页类型，对齐上游 `src/core/pagination.ts`。
//!
//! 兼容网关（如智谱 Claude 兼容接口）可能省略 `has_more` 等字段，反序列化时使用默认值。

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 游标分页响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageCursor<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenPage<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Sample {
        id: String,
    }

    #[test]
    fn token_page_without_has_more_defaults_false() {
        let raw = r#"{"data":[{"id":"glm-5.2"}]}"#;
        let page: TokenPage<Sample> = serde_json::from_str(raw).unwrap();
        assert_eq!(page.data.len(), 1);
        assert!(!page.has_more);
        assert!(!page.has_next_page());
    }

    #[test]
    fn token_page_with_next_token_implies_more_pages() {
        let raw = r#"{"data":[{"id":"a"}],"next_page_token":"tok"}"#;
        let page: TokenPage<Sample> = serde_json::from_str(raw).unwrap();
        assert!(page.has_next_page());
    }

    #[test]
    fn page_cursor_without_has_more_defaults_false() {
        let raw = r#"{"data":[{"id":"x"}]}"#;
        let page: PageCursor<Sample> = serde_json::from_str(raw).unwrap();
        assert!(!page.has_more);
    }
}
