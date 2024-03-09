from saregama_ytmusic.api import app
from saregama_ytmusic.schema import Playlist, PlaylistInfo

from saregama_ytmusic.ytmusic import default_client


@app.get("/playlists")
async def get_playlists() -> list[PlaylistInfo]:
    """Get all user's playlists information."""
    return default_client.user_playlists()


@app.get("/playlists/:id")
async def get_playlists_by_id(id: str) -> Playlist:
    """Get a playlist by its id."""
    return default_client.get_playlist(id)


@app.post("/playlists")
async def create_playlist() -> Playlist:
    """Create a new playlist."""
    pass
