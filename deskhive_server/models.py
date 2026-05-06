from pydantic import BaseModel
from typing import Optional


class Todo(BaseModel):
    id: str
    text: str
    completed: bool
    created_at: int  # Unix timestamp (seconds)
    completed_at: Optional[int] = None
    deadline: Optional[int] = None
    order: int = 0
    group_id: str = "default"
    priority: int = 0
    updated_at: int  # Last update time, for sync conflict detection
    is_deleted: bool = False  # Soft delete for sync


class TodoGroup(BaseModel):
    id: str
    name: str
    order: int = 0
    collapsed: bool = False
    updated_at: int  # Last update time, for sync conflict detection


class SyncRequest(BaseModel):
    todos: list[Todo]
    groups: list[TodoGroup]
    last_sync_at: int  # Client's last sync timestamp


class SyncResponse(BaseModel):
    todos: list[Todo]
    groups: list[TodoGroup]
    server_time: int  # Server's current timestamp
