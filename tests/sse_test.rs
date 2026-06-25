//! SSE 解析测试。

use anthropic_rust_sdk::core::streaming::EventStream;
use futures::{stream, StreamExt};

#[tokio::test]
async fn parses_message_stream_events() {
    let raw = "event: message_start\ndata: {\"type\":\"message_start\",\"message\":{\"id\":\"m1\"}}\n\n\
               event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"Hi\"}}\n\n\
               event: message_stop\ndata: {\"type\":\"message_stop\"}\n\n";

    let stream = stream::iter(vec![Ok(bytes::Bytes::from(raw))]);
    let mut events = EventStream::<serde_json::Value>::new(stream.boxed());

    let first = events.next().await.unwrap().unwrap();
    assert_eq!(first["type"], "message_start");

    let second = events.next().await.unwrap().unwrap();
    assert_eq!(second["type"], "content_block_delta");

    let third = events.next().await.unwrap().unwrap();
    assert_eq!(third["type"], "message_stop");
}

#[tokio::test]
async fn passes_through_system_message_event() {
    let raw = "event: system.message\ndata: {\"type\":\"system.message\",\"message\":{\"role\":\"system\",\"content\":\"ctx\"}}\n\n";

    let stream = stream::iter(vec![Ok(bytes::Bytes::from(raw))]);
    let mut events = EventStream::<serde_json::Value>::new(stream.boxed());

    let event = events.next().await.unwrap().unwrap();
    assert_eq!(event["type"], "system.message");
    assert_eq!(event["message"]["role"], "system");
}
