use actix_web::web;
use std::time::Duration;
use tokio::time::sleep;

use crate::{AppState, app::amls::executor, nats::config::handle_processing_error};

pub async fn start_aml_processor(state: web::Data<AppState>) {
    tokio::spawn(async move {
        let mut retry_count = 0;
        let max_retries = 5;

        loop {
            match state.streamer.consume_from_stream("AMLS", 10).await {
                Ok(messages) => {
                    retry_count = 0;

                    for message in messages {
                        if message.subject.to_string() == "amls.execution.new" {
                            let payload = String::from_utf8_lossy(&message.payload).to_string();

                            if let Err(e) = executor::process_aml_actions(payload, &state).await {
                                tracing::error!("Failed to process aml action: {:?}", e);
                                handle_processing_error(&message, e).await;
                            } else {
                                if let Err(e) = message.ack().await {
                                    tracing::error!("Failed to ack message: {}", e);
                                }
                            }
                        };
                    }
                }
                Err(e) => {
                    tracing::error!("Error consuming from stream: {}", e);
                    retry_count += 1;

                    let backoff = Duration::from_secs(2u64.pow(retry_count.min(6) as u32));

                    if retry_count >= max_retries {
                        tracing::error!("Max retries reached, backing off longer...");
                        sleep(Duration::from_secs(30)).await;
                        retry_count = 0;
                    } else {
                        sleep(backoff).await;
                    }
                }
            }
        }
    });
}
