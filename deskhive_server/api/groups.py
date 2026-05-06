from fastapi import APIRouter
from deskhive_server.models import TodoGroup
from deskhive_server.database import (
    get_all_groups,
    get_group,
    merge_groups,
    upsert_group,
    update_single_group,
)
from deskhive_server.api.ws import manager

router = APIRouter(prefix="/api/groups", tags=["groups"])


@router.get("")
def list_groups() -> list[dict]:
    """Get all groups."""
    return get_all_groups()


@router.get("/{group_id}")
def get_single_group(group_id: str) -> dict:
    """Get a single group by ID."""
    group = get_group(group_id)
    if group is None:
        from fastapi import HTTPException
        raise HTTPException(status_code=404, detail="Group not found")
    return group


@router.put("")
async def update_groups(groups: list[TodoGroup]) -> dict:
    """Merge incoming groups — last-write-wins by updated_at."""
    data = [g.model_dump() for g in groups]
    merge_groups(data)
    await manager.broadcast({"type": "data_updated"})
    return {"ok": True, "count": len(data)}


@router.patch("/{group_id}")
async def patch_group(group_id: str, fields: dict) -> dict:
    """Update specific fields of a single group."""
    from fastapi import HTTPException
    result = update_single_group(group_id, fields)
    if result is None:
        raise HTTPException(status_code=404, detail="Group not found")
    await manager.broadcast({"type": "data_updated"})
    return result
