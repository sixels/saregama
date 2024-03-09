import uvicorn

from saregama_ytmusic.api import app


def dev_server():
    uvicorn.run(app, host="localhost", port=8000)
