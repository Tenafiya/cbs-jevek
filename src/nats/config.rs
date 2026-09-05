use async_nats::jetstream::stream::Config;
use async_nats::jetstream::{
    self, AckKind, Context as JetStream, Message, consumer, stream::StorageType,
};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
struct StreamConfig {
    pub name: String,
    pub subjects: Vec<String>,
}

#[derive(Clone)]
pub struct StreamHandler {
    pub stream: jetstream::stream::Stream,
    pub consumer: consumer::PullConsumer,
    pub name: String,
}

#[derive(Clone)]
pub struct StreamManager {
    pub js: JetStream,
    pub streams: Arc<HashMap<String, StreamHandler>>,
}

impl StreamHandler {
    async fn new(
        js: &JetStream,
        config: StreamConfig,
        consumer_name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = js
            .create_stream(Config {
                name: config.name.clone(),
                subjects: config.subjects,
                storage: StorageType::File,
                ..Default::default()
            })
            .await?;

        let consumer = stream
            .create_consumer(consumer::pull::Config {
                durable_name: Some(consumer_name),
                ack_policy: consumer::AckPolicy::Explicit,
                ..Default::default()
            })
            .await?;

        Ok(Self {
            stream,
            consumer,
            name: config.name,
        })
    }

    pub async fn fetch_messages(
        &self,
        max_messages: usize,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error + Send + Sync>> {
        let mut messages = self
            .consumer
            .fetch()
            .max_messages(max_messages)
            .messages()
            .await?;

        let mut collected = Vec::new();
        while let Some(message) = messages.next().await {
            let message = message?;

            println!(
                "[{}] Received on {}: {}",
                self.name,
                message.subject,
                String::from_utf8_lossy(&message.payload)
            );
            // message.ack().await?;
            collected.push(message);
        }

        Ok(collected)
    }
}

impl StreamManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let nats_url = std::env::var("NATS_JETSTREAM").expect("Cannot find Nats url URL");

        let client = async_nats::connect(nats_url).await?;
        let js = jetstream::new(client);

        Ok(Self {
            js,
            streams: Arc::new(HashMap::new()),
        })
    }

    pub async fn register_stream(
        &mut self,
        stream_name: &str,
        subjects: Vec<String>,
        consumer_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = StreamConfig {
            name: stream_name.to_string(),
            subjects,
        };

        let consumer_name = format!("{}-consumer", consumer_name.to_lowercase());

        let handler = StreamHandler::new(&self.js, config, consumer_name).await?;

        let streams = Arc::make_mut(&mut self.streams);

        streams.insert(stream_name.to_string(), handler);

        Ok(())
    }

    pub async fn consume_from_stream(
        &self,
        stream_name: &str,
        max_messages: usize,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(handler) = self.streams.get(stream_name) {
            handler.fetch_messages(max_messages).await
        } else {
            Err(format!("Stream '{}' not found", stream_name).into())
        }
    }

    pub async fn publish_to_stream(
        &self,
        subject: impl Into<String>,
        payload: impl Into<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bytes = payload.into().into();
        let subject_str: String = subject.into();
        self.js.publish(subject_str, bytes).await?;
        Ok(())
    }
}

pub async fn setup_nats() -> Result<StreamManager, Box<dyn std::error::Error>> {
    let mut stream_manager = StreamManager::new().await?;

    stream_manager
        .register_stream("AMLS", vec!["amls.>".into()], "aml-processor".to_string())
        .await?;

    Ok(stream_manager)
}

pub async fn handle_processing_error(message: &Message, error: impl std::fmt::Display) {
    let error_str = error.to_string().to_lowercase();

    if error_str.contains("timeout") || error_str.contains("connection") {
        if let Err(e) = message.ack_with(AckKind::Nak(None)).await {
            tracing::error!("Failed to nak message: {}", e);
        }
    } else if error_str.contains("invalid") || error_str.contains("parse") {
        if let Err(e) = message.ack_with(AckKind::Term).await {
            tracing::error!("Failed to term message: {}", e);
        }
        tracing::warn!("Message terminated due to invalid data");
    } else {
        if let Err(e) = message.ack_with(AckKind::Nak(None)).await {
            tracing::error!("Failed to nak message: {}", e);
        }
    }
}
