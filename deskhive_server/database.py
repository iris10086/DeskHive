"""
JSON 文件存储 — 替代 SQLite

数据格式与桌面端 JSON 文件完全一致，方便数据迁移：
  - todos_with_groups.json = { "todos": [...] }
  - groups.json            = { "groups": [...] }

存储位置: ~/DeskHive/server/
"""

import json
import os
import time

DB_DIR = os.path.join(os.path.expanduser("~"), "DeskHive", "server")
TODOS_PATH = os.path.join(DB_DIR, "todos_with_groups.json")
GROUPS_PATH = os.path.join(DB_DIR, "groups.json")


# ---- 内部工具函数 ----

def _ensure_dir():
    os.makedirs(DB_DIR, exist_ok=True)


def _read_json(path, default):
    _ensure_dir()
    if not os.path.exists(path):
        return default
    try:
        with open(path, "r", encoding="utf-8") as f:
            return json.load(f)
    except (json.JSONDecodeError, FileNotFoundError):
        return default


def _write_json(path, data):
    _ensure_dir()
    tmp = path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    os.replace(tmp, path)  # atomic write


def _to_bool(v):
    """Convert int/bool to Python bool."""
    return bool(v) if not isinstance(v, bool) else v


# ---- Todo 操作 ----

def get_all_todos() -> list[dict]:
    """返回所有 todo 列表（已修复 bool 类型）。"""
    data = _read_json(TODOS_PATH, {"todos": []})
    todos = data.get("todos", [])
    for t in todos:
        t["completed"] = _to_bool(t.get("completed", False))
        t["is_deleted"] = _to_bool(t.get("is_deleted", False))
    return todos


def get_todo(todo_id: str) -> dict | None:
    """按 ID 查找单个 todo。"""
    for t in get_all_todos():
        if t["id"] == todo_id:
            return t
    return None


def _save_all_todos(todos: list[dict]) -> None:
    """全量写入 todos 文件。"""
    _write_json(TODOS_PATH, {"todos": todos})


def replace_all_todos(todos: list[dict]) -> None:
    """全量替换 todos（兼容旧调用方）。"""
    _save_all_todos(todos)


def merge_todos(client_todos: list[dict]) -> None:
    """
    合并客户端 todos — last-write-wins by updated_at。

    服务端副本更新则保留服务端，否则用客户端覆盖。仅存在于一方的条目保留。
    """
    todos = get_all_todos()
    existing = {t["id"]: t for t in todos}

    for todo in client_todos:
        tid = todo["id"]
        if tid in existing:
            existing_todo = existing[tid]
            # 服务端已逻辑删除 → 不接受客户端非删除覆盖（防止时间戳膨胀导致误覆盖）
            if existing_todo.get("is_deleted") and not todo.get("is_deleted"):
                continue
            if existing_todo["updated_at"] > todo["updated_at"]:
                continue  # 服务端更新 — 跳过
        existing[tid] = todo

    _save_all_todos(list(existing.values()))


def upsert_todo(todo: dict) -> None:
    """
    新增或覆盖单个 todo（按 id 匹配）。
    如果服务端已有更新版本则保留服务端版本。
    """
    todos = get_all_todos()
    existing = {t["id"]: t for t in todos}
    tid = todo["id"]

    if tid in existing and existing[tid]["updated_at"] > todo["updated_at"]:
        return  # 服务端更新 — 不覆盖

    existing[tid] = todo
    _save_all_todos(list(existing.values()))


def update_single_todo(todo_id: str, fields: dict) -> dict | None:
    """
    更新单个 todo。
    如果客户端提供了 updated_at，只在客户端版本不旧于服务端时更新。
    如果客户端未提供 updated_at，直接更新并 bump 服务端时间戳。
    返回更新后的 todo，未找到返回 None。
    """
    todos = get_all_todos()
    for i, t in enumerate(todos):
        if t["id"] == todo_id:
            # 如果客户端传了 updated_at，做版本检查
            client_updated = fields.get("updated_at")
            if client_updated is not None and t.get("updated_at", 0) > client_updated:
                return t  # 服务端更新 — 跳过，返回现有数据
            t.update(fields)
            if "updated_at" not in fields:
                t["updated_at"] = int(time.time())
            todos[i] = t
            _save_all_todos(todos)
            return t
    return None


# ---- Group 操作 ----

def get_all_groups() -> list[dict]:
    """返回全部分组列表。"""
    data = _read_json(GROUPS_PATH, {"groups": []})
    groups = data.get("groups", [])
    for g in groups:
        g["collapsed"] = _to_bool(g.get("collapsed", False))
    return groups


def get_group(group_id: str) -> dict | None:
    """按 ID 查找单个分组。"""
    for g in get_all_groups():
        if g["id"] == group_id:
            return g
    return None


def _save_all_groups(groups: list[dict]) -> None:
    """全量写入 groups 文件。"""
    _write_json(GROUPS_PATH, {"groups": groups})


def replace_all_groups(groups: list[dict]) -> None:
    """全量替换 groups（兼容旧调用方）。"""
    _save_all_groups(groups)


def merge_groups(client_groups: list[dict]) -> None:
    """
    合并客户端 groups — last-write-wins by updated_at。
    """
    groups = get_all_groups()
    existing = {g["id"]: g for g in groups}

    for group in client_groups:
        gid = group["id"]
        if gid in existing and existing[gid]["updated_at"] > group["updated_at"]:
            continue
        existing[gid] = group

    _save_all_groups(list(existing.values()))


def upsert_group(group: dict) -> None:
    """新增或覆盖单个分组。"""
    groups = get_all_groups()
    existing = {g["id"]: g for g in groups}
    gid = group["id"]

    if gid in existing and existing[gid]["updated_at"] > group["updated_at"]:
        return

    existing[gid] = group
    _save_all_groups(list(existing.values()))


def update_single_group(group_id: str, fields: dict) -> dict | None:
    """更新单个分组。如果客户端提供了 updated_at 且不比服务端旧则更新，否则跳过。"""
    groups = get_all_groups()
    for i, g in enumerate(groups):
        if g["id"] == group_id:
            client_updated = fields.get("updated_at")
            if client_updated is not None and g.get("updated_at", 0) > client_updated:
                return g
            g.update(fields)
            if "updated_at" not in fields:
                g["updated_at"] = int(time.time())
            groups[i] = g
            _save_all_groups(groups)
            return g
    return None


# ---- 兼容接口 ----

def init_db():
    """初始化数据库（SQLite 时代的兼容接口，仅确保目录存在）。"""
    _ensure_dir()


# 导入时自动创建目录
_ensure_dir()
