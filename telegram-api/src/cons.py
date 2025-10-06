import pika

RABBITMQ_HOST = "localhost"
RABBITMQ_PORT = 5672
RABBITMQ_USERNAME = "guest"
RABBITMQ_PASSWORD = "guest"
OUTGOING_MESSAGES_QUEUE = "READ_CHAT"

def on_message(ch, method, properties, body):
    print(f"Received message from Telegram bot: {body.decode()}")
    ch.basic_ack(delivery_tag=method.delivery_tag)

def main():
    connection = pika.BlockingConnection(
        pika.ConnectionParameters(
            host=RABBITMQ_HOST,
            port=RABBITMQ_PORT,
            credentials=pika.PlainCredentials(
                username=RABBITMQ_USERNAME,
                password=RABBITMQ_PASSWORD
            ),
        )
    )

    channel = connection.channel()
    channel.queue_declare(queue=OUTGOING_MESSAGES_QUEUE, durable=False)
    channel.basic_consume(
        queue=OUTGOING_MESSAGES_QUEUE,
        on_message_callback=on_message
    )

    print(f"Waiting for messages from Telegram bot on '{OUTGOING_MESSAGES_QUEUE}' queue. Press CTRL+C to exit.")

    try:
        channel.start_consuming()
    except KeyboardInterrupt:
        print("\nStopping consumer...")
        channel.stop_consuming()
    finally:
        connection.close()

if __name__ == "__main__":
    main()
