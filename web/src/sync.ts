/**
 * DeskHive Sync Engine
 *
 * Syncs todos and groups with a remote FastAPI server.
 * Strategy: full replacement + last-write-wins based on updated_at.
 */

import type { Todo, TodoGroup } from './types';

let syncEnabled = false;
let serverUrl = '';
let syncTimer: number | null = null;
let lastSyncCallback: ((time: number) => void) | null = null;

const SYNC_INTERVAL = 5 * 60 * 1000; // 5 minutes

// ---- Types for API communication ----

interface SyncPayload {
  todos: SyncTodo[];
  groups: SyncGroup[];
  last_sync_at: number;
}

interface SyncResponse {
  todos: SyncTodo[];
  groups: SyncGroup[];
  server_time: number;
}

interface SyncTodo {
  id: string;
  text: string;
  completed: boolean;
  created_at: number;
  completed_at: number | null;
  deadline: number | null;
  order: number;
  group_id: string;
  priority: number;
  updated_at: number;
  is_deleted: boolean;
}

interface SyncGroup {
  id: string;
  name: string;
  order: number;
  collapsed: boolean;
  updated_at: number;
  is_deleted: boolean;
}

// ---- Helpers ----

function todoToSync(todo: Todo): SyncTodo {
  return {
    id: todo.id,
    text: todo.text,
    completed: todo.completed,
    created_at: todo.createdAt,
    completed_at: todo.completedAt ?? null,
    deadline: todo.deadline ?? null,
    order: todo.order,
    group_id: todo.groupId,
    priority: todo.priority,
    updated_at: todo.updatedAt,
    is_deleted: todo.isDeleted ?? false,
  };
}

function syncToTodo(t: SyncTodo): Todo {
  return {
    id: t.id,
    text: t.text,
    completed: t.completed,
    createdAt: t.created_at,
    completedAt: t.completed_at ?? undefined,
    deadline: t.deadline ?? undefined,
    order: t.order,
    groupId: t.group_id,
    priority: t.priority,
    updatedAt: t.updated_at,
    isDeleted: t.is_deleted,
  };
}

function groupToSync(group: TodoGroup): SyncGroup {
  return {
    id: group.id,
    name: group.name,
    order: group.order,
    collapsed: group.collapsed,
    updated_at: group.updatedAt,
    is_deleted: group.isDeleted ?? false,
  };
}

function syncToGroup(g: SyncGroup): TodoGroup {
  return {
    id: g.id,
    name: g.name,
    order: g.order,
    collapsed: g.collapsed,
    updatedAt: g.updated_at,
    isDeleted: g.is_deleted,
  };
}

// ---- Sync API calls ----

function getApiUrl(path: string): string {
  let base = serverUrl.replace(/\/+$/, '');
  // 自动补上 http:// 防止被当相对路径
  if (base && !/^https?:\/\//i.test(base)) {
    base = 'http://' + base;
  }
  return `${base}${path}`;
}

async function syncToServer(
  todos: Todo[],
  groups: TodoGroup[],
  lastSyncAt: number,
): Promise<SyncResponse> {
  const payload: SyncPayload = {
    todos: todos.map(todoToSync),
    groups: groups.map(groupToSync),
    last_sync_at: lastSyncAt,
  };

  const resp = await fetch(getApiUrl('/api/sync'), {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });

  if (!resp.ok) {
    throw new Error(`Sync failed: ${resp.status} ${resp.statusText}`);
  }

  return resp.json();
}

async function checkHealth(): Promise<boolean> {
  try {
    const resp = await fetch(getApiUrl('/health'), { method: 'GET' });
    return resp.ok;
  } catch {
    return false;
  }
}

// ---- Public API ----

/**
 * Initialize the sync engine. Call once on app startup.
 */
export function initSync(
  enabled: boolean,
  url: string,
  onSyncComplete: (serverTime: number) => void,
): void {
  syncEnabled = enabled;
  serverUrl = url;
  lastSyncCallback = onSyncComplete;
}

/**
 * Update sync configuration (called when settings change).
 */
export function updateConfig(enabled: boolean, url: string): void {
  const wasEnabled = syncEnabled;
  syncEnabled = enabled;
  serverUrl = url;

  if (wasEnabled && !enabled) {
    stopTimer();
  } else if (!wasEnabled && enabled) {
    startTimer();
  }
}

/**
 * Push local data to server and pull server data back.
 */
export async function pushAndPull(
  todos: Todo[],
  groups: TodoGroup[],
  lastSyncAt: number,
): Promise<{ todos: Todo[]; groups: TodoGroup[]; serverTime: number } | null> {
  if (!syncEnabled || !serverUrl) {
    return null;
  }

  try {
    const resp = await syncToServer(todos, groups, lastSyncAt);

    const mergedTodos = resp.todos.map(syncToTodo);
    const mergedGroups = resp.groups.map(syncToGroup);

    return {
      todos: mergedTodos,
      groups: mergedGroups,
      serverTime: resp.server_time,
    };
  } catch (err) {
    console.error('Sync failed:', err);
    return null;
  }
}

/**
 * Test connection to the sync server.
 */
export async function testConnection(url: string): Promise<boolean> {
  const originalUrl = serverUrl;
  serverUrl = url;
  try {
    const ok = await checkHealth();
    return ok;
  } finally {
    serverUrl = originalUrl;
  }
}

/**
 * Start the periodic sync timer.
 */
export function startTimer(): void {
  stopTimer();
  syncTimer = window.setInterval(async () => {
    if (syncEnabled && lastSyncCallback) {
      lastSyncCallback(Date.now());
    }
  }, SYNC_INTERVAL);
}

/**
 * Stop the periodic sync timer.
 */
export function stopTimer(): void {
  if (syncTimer !== null) {
    clearInterval(syncTimer);
    syncTimer = null;
  }
}
