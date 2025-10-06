import pika
import json
from datetime import datetime

def send_test_message():
    message = {
        "text": "Hello from streamer! This is a test message.",
        "username": "test_streamer",
        "chat": {
            "telegram_id": -1003101224194
        }
    }

    connection = pika.BlockingConnection(
        pika.ConnectionParameters(host='localhost')
    )
    channel = connection.channel()

    channel.queue_declare(queue='POST_CHAT')

    channel.basic_publish(
        exchange='',
        routing_key='POST_CHAT',
        body=json.dumps(message)
    )

    print(f" [x] Sent test message: {message}")

    connection.close()

if __name__ == "__main__":
    send_test_message()
