from fastapi import APIRouter
from deskhive_server.models import Todo
from deskhive_server.database import (
    get_all_todos,
    get_todo,
    merge_todos,
    upsert_todo,
    update_single_todo,
)
from deskhive_server.api.ws import manager

router = APIRouter(prefix="/api/todos", tags=["todos"])


@router.get("")
def list_todos() -> list[dict]:
    """Get all todos."""
    return get_all_todos()


@router.get("/{todo_id}")
def get_single_todo(todo_id: str) -> dict:
    """Get a single todo by ID."""
    todo = get_todo(todo_id)
    if todo is None:
        from fastapi import HTTPException
        raise HTTPException(status_code=404, detail="Todo not found")
    return todo


@router.put("")
async def update_todos(todos: list[Todo]) -> dict:
    """Merge incoming todos — last-write-wins by updated_at."""
    data = [t.model_dump() for t in todos]
    merge_todos(data)
    await manager.broadcast({"type": "data_updated"})
    return {"ok": True, "count": len(data)}


@router.patch("/{todo_id}")
async def patch_todo(todo_id: str, fields: dict) -> dict:
    """Update specific fields of a single todo."""
    from fastapi import HTTPException
    result = update_single_todo(todo_id, fields)
    if result is None:
        raise HTTPException(status_code=404, detail="Todo not found")
    await manager.broadcast({"type": "data_updated"})
    return result
