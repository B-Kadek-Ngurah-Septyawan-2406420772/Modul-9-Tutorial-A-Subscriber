use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(async_reactor_trait::AsyncIo);

    let connection = Connection::connect("amqp://guest:guest@127.0.0.1:5672", options).await?;
    let channel = connection.create_channel().await?;

    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions {
                durable: false,
                auto_delete: false,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    let consumer = channel
        .basic_consume(
            "user_created",
            "subscriber",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    consumer.set_delegate(|delivery: DeliveryResult| async move {
        let delivery = match delivery {
            Ok(Some(delivery)) => delivery,
            Ok(None) => return,
            Err(error) => {
                eprintln!("Failed to consume message: {error}");
                return;
            }
        };

        let ten_millis = Duration::from_millis(1000);
        sleep(ten_millis).await;

        match UserCreatedEventMessage::try_from_slice(&delivery.data) {
            Ok(message) => {
                println!(
                    "In Awan's Computer [2406420772]. Message received: {:?}",
                    message
                );
            }
            Err(error) => eprintln!("Failed to deserialize message: {error}"),
        }

        if let Err(error) = delivery.ack(BasicAckOptions::default()).await {
            eprintln!("Failed to ack message: {error}");
        }
    });

    println!("Subscriber is listening for user_created events.");
    std::future::pending::<()>().await;
    Ok(())
}
