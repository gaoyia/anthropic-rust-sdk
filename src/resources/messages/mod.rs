mod batches;
pub mod types;

use crate::client::Anthropic;
use crate::core::error::Error;
use crate::core::streaming::EventStream;
use crate::runtime::message_stream::MessageStream;
use futures::StreamExt;

pub use batches::*;
pub use types::*;

/// Messages API 资源。
pub struct Messages<'a> {
    client: &'a Anthropic,
}

impl<'a> Messages<'a> {
    pub(crate) fn new(client: &'a Anthropic) -> Self {
        Self { client }
    }

    /// 批处理子资源。
    pub fn batches(&self) -> Batches<'a> {
        Batches::new(self.client)
    }

    /// 创建消息（非流式或流式）。
    pub async fn create(&self, params: MessageCreateParams) -> Result<MessageCreateResult, Error> {
        let stream = params.stream.unwrap_or(false);
        if stream {
            let response = self.client.post_streaming("/v1/messages", &params).await?;
            let byte_stream = response.bytes_stream().boxed();
            let event_stream = EventStream::<RawMessageStreamEvent>::new(byte_stream);
            return Ok(MessageCreateResult::Stream(event_stream));
        }

        let mut non_streaming = params;
        non_streaming.stream = Some(false);
        let message: Message = self.client.post("/v1/messages", &non_streaming).await?;
        Ok(MessageCreateResult::Message(Box::new(message)))
    }

    /// 统计 Token 数量。
    pub async fn count_tokens(
        &self,
        params: MessageCountTokensParams,
    ) -> Result<MessageTokensCount, Error> {
        self.client
            .post("/v1/messages/count_tokens", &params)
            .await
    }

    /// 创建高级消息流。
    pub fn stream(&self, params: MessageCreateParams) -> MessageStream<'a> {
        MessageStream::new(self.client, params)
    }

    /// 结构化输出解析（对齐 `messages.parse()`）。
    pub async fn parse(&self, params: MessageCreateParams) -> Result<ParsedMessage, Error> {
        let mut non_streaming = params;
        non_streaming.stream = Some(false);
        let message = match self.create(non_streaming).await? {
            MessageCreateResult::Message(m) => *m,
            MessageCreateResult::Stream(_) => {
                return Err(crate::core::error::AnthropicError(
                    "parse() requires non-streaming request".into(),
                )
                .into());
            }
        };
        crate::runtime::parser::parse_message(&message)
    }
}

/// 创建消息结果。
pub enum MessageCreateResult {
    Message(Box<Message>),
    Stream(EventStream<RawMessageStreamEvent>),
}

/// 带解析输出的消息。
#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub message: Message,
    pub parsed_output: Option<serde_json::Value>,
}
