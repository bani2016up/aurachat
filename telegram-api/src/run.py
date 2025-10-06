import logging
import asyncio
import sys


from telegram_api.services.telegram_bot import run_bot



if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, stream=sys.stdout)
    asyncio.run(run_bot())
