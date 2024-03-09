from pydantic import BaseModel


class Track(BaseModel):
    id: str
    name: str
    artists: list[str]
    duration: int  # in seconds
    album: str
    url: str
