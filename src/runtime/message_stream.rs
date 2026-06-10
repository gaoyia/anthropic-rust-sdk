//! 高级消息流，对齐上游 `src/lib/MessageStream.ts`。

use crate::client::Anthropic;
use crate::core::error::Error;
use crate::core::streaming::EventStream;
use crate::resources::messages::{Message, MessageCreateParams, RawMessageStreamEvent};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 累积中的消息快照。
#[derive(Debug, Clone, Default)]
pub struct MessageSnapshot {
    pub message: Option<Message>,
    pub text: String,
}

/// 高级消息流封装。
pub struct MessageStream<'a> {
    client: &'a Anthropic,
    params: MessageCreateParams,
    snapshot: Arc<Mutex<MessageSnapshot>>,
}

impl<'a> MessageStream<'a> {
    pub fn new(client: &'a Anthropic, mut params: MessageCreateParams) -> Self {
        params.stream = Some(true);
        Self {
            client,
            params,
            snapshot: Arc::new(Mutex::new(MessageSnapshot::default())),
        }
    }

    /// 返回底层事件流。
    pub async fn events(&self) -> Result<EventStream<RawMessageStreamEvent>, Error> {
        let response = self
            .client
            .post_streaming("/v1/messages", &self.params)
            .await?;
        let byte_stream = response.bytes_stream().boxed();
        Ok(EventStream::new(byte_stream))
    }

    /// 迭代流式事件并累积快照。
    pub async fn run<F>(&self, mut on_event: F) -> Result<Message, Error>
    where
        F: FnMut(RawMessageStreamEvent, MessageSnapshot),
    {
        let mut events = self.events().await?;
        let mut final_message = None;

        while let Some(event) = events.next().await {
            let event = event?;
            self.apply_event(&event).await;
            let snap = self.snapshot.lock().await.clone();
            let event_type = event.event_type.clone();
            on_event(event.clone(), snap.clone());

            if event_type == "message_start" {
                if let Some(msg) = event.fields.get("message") {
                    if let Ok(m) = serde_json::from_value::<Message>(msg.clone()) {
                        final_message = Some(m);
                    }
                }
            }

            if event_type == "message_delta" {
                if let Some(delta) = event.fields.get("delta") {
                    if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                        let mut snap = self.snapshot.lock().await;
                        snap.text.push_str(text);
                    }
                }
            }
        }

        final_message.ok_or_else(|| {
            Error::Anthropic(crate::core::error::AnthropicError(
                "stream ended without a final message".into(),
            ))
        })
    }

    /// 等待流结束并返回最终消息。
    pub async fn final_message(&self) -> Result<Message, Error> {
        self.run(|_, _| {}).await
    }

    async fn apply_event(&self, event: &RawMessageStreamEvent) {
        let mut snap = self.snapshot.lock().await;
        if event.event_type == "content_block_delta" {
            if let Some(delta) = event.fields.get("delta") {
                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                    snap.text.push_str(text);
                }
            }
        }
    }
}
