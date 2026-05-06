import time
from fastapi import APIRouter
from deskhive_server.models import SyncRequest, SyncResponse
from deskhive_server.database import (
    get_all_todos,
    get_all_groups,
    merge_todos,
    merge_groups,
)
from deskhive_server.api.ws import manager

router = APIRouter(prefix="/api/sync", tags=["sync"])


@router.post("")
async def sync(data: SyncRequest) -> SyncResponse:
    """
    Bidirectional merge sync — last-write-wins by updated_at.

    The client sends its full data set.  The server merges item-by-item:
    if the server's copy is newer (larger updated_at), it keeps its own;
    otherwise it adopts the client's copy.  Items that exist only on one
    side are preserved.  The full merged state is returned to the client.
    """
    server_time = int(time.time())

    # Merge incoming data（merge 函数内已读写 JSON 文件）
    merge_todos([t.model_dump() for t in data.todos])
    merge_groups([g.model_dump() for g in data.groups])

    # Return full server state
    todos = get_all_todos()
    groups = get_all_groups()

    # Notify all WebSocket clients that data has changed
    await manager.broadcast({"type": "data_updated"})

    return SyncResponse(
        todos=todos,
        groups=groups,
        server_time=server_time,
    )
