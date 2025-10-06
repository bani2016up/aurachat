from dotenv import load_dotenv
from aiogram.filters import CommandStart
from telegram_api.domain.models.message import BaseMessage, PostMassage, ExtendedChatInfo
from aiogram import Bot, Dispatcher
from aiogram.client.default import DefaultBotProperties
from aiogram.enums import ParseMode
from telegram_api.domain.infrastructure.rabbitMQ.consumer import RabbitMQConsumer
import logging
from telegram_api.domain.infrastructure.rabbitMQ.producer import RabbitMQProducer
from aiogram.types import Message
import asyncio
from os import getenv

load_dotenv()
TOKEN: str | None = getenv("BOT_TOKEN")
if TOKEN is None:
    raise ValueError("BOT_TOKEN environment variable is not set")

dp = Dispatcher()
bot_instance: Bot | None = None
rabbit_mq_producer: RabbitMQProducer | None = None
rabbit_mq_consumer: RabbitMQConsumer | None = None

@dp.message(CommandStart())
async def command_start_handler(message: Message) -> None:
    await message.answer(f"Hello!")

@dp.message()
async def forward_to_dashboard(message: Message) -> None:
    username: str | None = "Undefined user"
    if message.from_user is not None:
        username = message.from_user.username
    chat = ExtendedChatInfo(title=message.chat.title or "", telegram_id=message.chat.id)

    message_data = BaseMessage(
        text=message.text or "",
        username=username,
        timestamp=message.date,
        chat=chat,
    ).model_dump_json()

    logging.info(f"Sending message to READ_CHAT queue: {message_data}")
    await rabbit_mq_producer.send_message(message_data)

async def handle_incoming_message(message_json: str) -> None:
    try:
        logging.info(f"Received message from POST_CHAT queue: {message_json}")
        message = PostMassage.model_validate_json(message_json)
        await post_streammer_massage(message)
    except Exception as e:
        logging.error(f"Error processing message: {e}")

async def post_streammer_massage(message: PostMassage) -> None:
    if bot_instance is None:
        raise ValueError("Bot instance is not initialized")
    await bot_instance.send_message(chat_id=message.chat.telegram_id, text=f"{message.username}: {message.text}")

async def run_bot() -> None:
    global bot_instance, rabbit_mq_producer, rabbit_mq_consumer
    bot_instance = Bot(token=TOKEN, default=DefaultBotProperties(parse_mode=ParseMode.HTML))
    rabbit_mq_producer = RabbitMQProducer()
    await rabbit_mq_producer.connect()
    rabbit_mq_consumer = RabbitMQConsumer(handle_incoming_message)

    consumer_task = asyncio.create_task(rabbit_mq_consumer.start_consuming())
    polling_task = asyncio.create_task(dp.start_polling(bot_instance))

    await asyncio.gather(consumer_task, polling_task)

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    asyncio.run(run_bot())
