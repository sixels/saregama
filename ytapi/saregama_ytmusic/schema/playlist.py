from typing import Optional

from pydantic import BaseModel

from .track import Track


class PlaylistInfo(BaseModel):
    id: str
    name: str
    description: Optional[str]
    url: str
    trackCount: int


class Playlist(BaseModel):
    info: PlaylistInfo
    tracks: list[Track]
