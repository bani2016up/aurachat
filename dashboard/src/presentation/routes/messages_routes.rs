
use crate::infrastructure::messaging::RabbitMQClient;
use crate::application::dtos::message_dto::{ChatMessage, StreammerMessage};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::time::{interval, Duration};
use rocket_okapi::openapi;
use rocket::{post, get};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;


static CONNECTION_COUNT: AtomicUsize = AtomicUsize::new(0);
static MESSAGE_COUNT: AtomicUsize = AtomicUsize::new(0);


#[openapi(tag = "Telegram Chat")]
#[post("/chat/messages", data = "<message>")]
pub async fn post_streammer_message(
    message: Json<StreammerMessage>,
    rabbitmq: &rocket::State<Arc<RabbitMQClient>>,
) -> Result<String, String> {
    rabbitmq
        .publish_message("POST_CHAT", &message.into_inner())
        .await
        .map_err(|e| format!("Failed to send message: {}", e))?;

    Ok("Message sent successfully".to_string())
}



// #[openapi(tag = "Telegram Chat")]
#[get("/chat/stream")]
pub async fn chat_stream() -> EventStream![]{
    CONNECTION_COUNT.fetch_add(1, Ordering::SeqCst);

    EventStream! {
        let mut interval = interval(Duration::from_secs(2));
        let mut message_id = 0;

        // Send welcome message
        yield Event::json(&ChatMessage {
            id: message_id,
            user: "System".to_string(),
            content: "Welcome to the chat!".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        });

        message_id += 1;

        loop {
            select! {
                _ = interval.tick() => {
                    let has_new_msg = true;
                    if has_new_msg {
                        let users = vec!["Alice", "Bob", "Charlie", "Diana"];
                        let messages = vec![
                            "Hello everyone!",
                            "How's it going?",
                            "Anyone working on something interesting?",
                            "The weather is nice today!",
                            "Just deployed a new feature!",
                        ];

                        let user = users[1];
                        let message = messages[1];

                        let chat_message = ChatMessage {
                            id: message_id,
                            user: user.to_string(),
                            content: message.to_string(),
                            timestamp: chrono::Utc::now().timestamp(),
                        };

                        yield Event::json(&chat_message);
                        message_id += 1;
                        MESSAGE_COUNT.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        }
    }
}
