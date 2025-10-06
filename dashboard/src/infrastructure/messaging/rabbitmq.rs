
use lapin::{
    options::*, BasicProperties, Channel, Connection, ConnectionProperties,
};
use serde::Serialize;

pub struct RabbitMQClient {
    connection: Connection,
}

impl RabbitMQClient {
    pub async fn new(url: &str) -> Result<Self, lapin::Error> {
        let connection = Connection::connect(url, ConnectionProperties::default()).await?;
        Ok(Self { connection })
    }

    pub async fn create_channel(&self) -> Result<Channel, lapin::Error> {
        self.connection.create_channel().await
    }

    pub async fn publish_message<T: Serialize>(
        &self,
        queue_name: &str,
        message: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let channel = self.create_channel().await?;

        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?;

        let message_json = serde_json::to_string(message)?;

        channel
            .basic_publish(
                "",
                queue_name,
                BasicPublishOptions::default(),
                message_json.as_bytes(),
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }
}
