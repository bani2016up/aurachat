import datetime
from pydantic import BaseModel



class BaseConfig(BaseModel):
    ...



class BaseChat(BaseModel):
    telegram_id: int


class ExtendedChatInfo(BaseChat):
    title: str

class BaseMessage(BaseModel):
        text: str
        username: str | None
        timestamp: datetime.datetime | None = None
        chat: ExtendedChatInfo



class PostMassage(BaseModel):
    text: str
    username: str
    chat: BaseChat
