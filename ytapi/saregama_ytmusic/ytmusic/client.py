from ytmusicapi import YTMusic
from saregama_ytmusic.schema import Playlist, PlaylistInfo, Track


class YTMusicClient:
    client: YTMusic
    _unauth: YTMusic

    def __init__(self, auth):
        self.client = YTMusic(auth)
        self._unauth = YTMusic()

    def user_playlists(self, limit: int = 25):
        print("getting user playlists")

        res = self.client.get_library_playlists(limit=limit)

        return [
            PlaylistInfo(
                id=playlist["playlistId"],
                name=playlist["title"],
                description=playlist["description"],
                url=f"https://music.youtube.com/playlist?list={playlist['playlistId']}",
                trackCount=playlist["count"] if "count" in playlist else 0,
            )
            for playlist in res
        ]

    def get_playlist(self, playlist_id: str):
        print("getting playlist")
        res = self.client.get_playlist(playlist_id, limit=1000)

        return Playlist(
            id=res["id"],
            name=res["title"],
            description=res["description"],
            url=f"https://music.youtube.com/playlist?list={res['id']}",
            trackCount=res["trackCount"],
            tracks=[_track_from_playlist_item(track) for track in res["tracks"]],
        )

    def search_song(self, query: str, limit: int = 10):
        res = self._unauth.search(query, filter="songs", limit=limit)

        return [_track_from_search_result(result) for result in res]


def _track_from_playlist_item(res) -> Track:
    album = res["views"]  # res["album"] is always empty, maybe it's a bug?

    return Track(
        id=res["videoId"],
        name=res["title"],
        artists=[artist["name"] for artist in res["artists"]],
        duration=res["duration_seconds"],
        album=album,
        url=f"https://music.youtube.com/watch?v={res['videoId']}",
    )


def _track_from_search_result(res) -> Track:
    return Track(
        id=res["videoId"],
        name=res["title"],
        artists=[artist["name"] for artist in res["artists"]],
        duration=res["duration_seconds"],
        album=res["album"]["name"],
        url=f"https://music.youtube.com/watch?v={res['videoId']}",
    )
