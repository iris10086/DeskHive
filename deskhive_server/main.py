import os
import subprocess
import sys
from pathlib import Path
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.middleware.cors import CORSMiddleware
from contextlib import asynccontextmanager

from deskhive_server.database import init_db
from deskhive_server.api.todos import router as todos_router
from deskhive_server.api.groups import router as groups_router
from deskhive_server.api.sync import router as sync_router
from deskhive_server.api.ws import router as ws_router


@asynccontextmanager
async def lifespan(app: FastAPI):
    init_db()
    yield


app = FastAPI(
    title="DeskHive Sync Server",
    description="Sync server for DeskHive desktop app + Web UI",
    version="1.0.0",
    lifespan=lifespan,
)

# CORS: allow any origin (local sync server, no auth/cookies involved)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(todos_router)
app.include_router(groups_router)
app.include_router(sync_router)
app.include_router(ws_router)   # WebSocket endpoint at /ws


@app.get("/health")
def health():
    return {"status": "ok"}


# Serve static web UI if available
static_dir = Path(__file__).parent / "static"
if static_dir.exists():
    app.mount("/", StaticFiles(directory=str(static_dir), html=True), name="web")


def open_browser():
    """Open browser after a short delay."""
    import time
    time.sleep(1.5)
    try:
        if sys.platform == "win32":
            os.startupinfo = None
            subprocess.Popen(["cmd", "/c", "start", "http://127.0.0.1:8080"],
                             shell=True, startupinfo=None)
        elif sys.platform == "darwin":
            subprocess.Popen(["open", "http://127.0.0.1:8080"])
        else:
            subprocess.Popen(["xdg-open", "http://127.0.0.1:8080"])
    except Exception:
        pass  # Browser opening is optional


def main():
    import uvicorn
    print("=" * 50)
    print("  DeskHive Sync Server")
    print("  http://127.0.0.1:8080")
    print("  API docs: http://127.0.0.1:8080/docs")
    print("=" * 50)
    # open_browser()
    uvicorn.run(app, host="0.0.0.0", port=8080)


if __name__ == "__main__":
    main()
