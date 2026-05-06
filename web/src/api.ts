/**
 * Web API 适配层
 * 替代桌面端的 invoke() 调用
 *
 * 所有数据直接从后端服务器获取/保存（PATCH 单条 / PUT 批量）：
 *   - 读取 → GET /api/todos, GET /api/groups
 *   - 单条修改 → PATCH /api/todos/{id}, PATCH /api/groups/{id}
 *   - 批量写入 → PUT /api/todos, PUT /api/groups
 *
 * 设置项仍存 localStorage（无服务端 API）。
 */

// ---- HTTP 工具函数 ----

const API_BASE = ''

async function apiGet(path: string): Promise<any> {
  const resp = await fetch(`${API_BASE}${path}`)
  if (!resp.ok) throw new Error(`GET ${path} failed: ${resp.status}`)
  return resp.json()
}

async function apiPut(path: string, body: any): Promise<any> {
  const resp = await fetch(`${API_BASE}${path}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!resp.ok) throw new Error(`PUT ${path} failed: ${resp.status}`)
  return resp.json()
}

async function apiPatch(path: string, body: any): Promise<any> {
  const resp = await fetch(`${API_BASE}${path}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!resp.ok) throw new Error(`PATCH ${path} failed: ${resp.status}`)
  return resp.json()
}

// ---- Settings (localStorage) ----

const SETTINGS_KEY = 'deskhive_settings'

const DEFAULT_SETTINGS = {
  opacity: 1.0,
  disable_drag: false,
  auto_start: false,
  silent_start: false,
  theme: 'light' as string,
  priority_color: '#FF9800',
  window_level: 'normal',
  timeline_deadline_priority: true,
  enable_deadline_notification: false,
  notification_minutes_before: 30,
  sync_enabled: false,
  sync_server_url: '',
}

function loadJSON<T>(key: string, fallback: T): T {
  try {
    const raw = localStorage.getItem(key)
    if (raw) return JSON.parse(raw)
  } catch (e) {
    console.error(`读取 ${key} 失败:`, e)
  }
  return fallback
}

function saveJSON(key: string, data: unknown): void {
  try {
    localStorage.setItem(key, JSON.stringify(data))
  } catch (e) {
    console.error(`保存 ${key} 失败:`, e)
  }
}

// ---- Tauri command adapter ----

export async function invoke(cmd: string, args?: Record<string, any>): Promise<any> {
  switch (cmd) {
    // ---- Todos ----
    case 'save_todo_data_with_groups': {
      const todos = (args?.todos || []).map((t: any) => ({
        id: t.id,
        text: t.text,
        completed: t.completed,
        created_at: t.created_at,
        completed_at: t.completed_at ?? null,
        deadline: t.deadline ?? null,
        order: t.order,
        group_id: t.group_id,
        priority: t.priority ?? 0,
        updated_at: t.updated_at,
        is_deleted: t.is_deleted ?? false,
      }))
      // 批量全量写入（用于拖拽排序、清空已完成等批量操作）
      return apiPut('/api/todos', todos)
    }

    case 'load_todo_data_with_groups': {
      // 从服务器拉取全部 todo，格式包装为 { todos: [...] }
      const todos = await apiGet('/api/todos')
      return { todos }
    }

    case 'update_single_todo': {
      const todo = args?.todo
      if (!todo) return Promise.reject(new Error('缺少 todo 参数'))
      // saveSingleTodo 已转为 snake_case，直接透传给服务端
      return apiPatch(`/api/todos/${todo.id}`, todo)
    }

    // ---- Groups ----
    case 'save_group_data': {
      const groups = (args?.groups || []).map((g: any) => ({
        id: g.id,
        name: g.name,
        order: g.order,
        collapsed: g.collapsed,
        updated_at: g.updated_at,
      }))
      return apiPut('/api/groups', groups)
    }

    case 'load_group_data': {
      // 从服务器拉取全部分组，格式包装为 { groups: [...] }
      const groups = await apiGet('/api/groups')
      return { groups }
    }

    case 'update_single_group': {
      const group = args?.group
      if (!group) return Promise.reject(new Error('缺少 group 参数'))
      // saveSingleGroup 已转为 snake_case，直接透传
      return apiPatch(`/api/groups/${group.id}`, group)
    }

    // ---- Settings (localStorage) ----
    case 'save_app_settings': {
      saveJSON(SETTINGS_KEY, args?.settings)
      const theme = args?.settings?.theme
      if (theme) {
        document.body.className = theme === 'dark' ? 'dark-theme' : ''
      }
      return Promise.resolve()
    }

    case 'load_app_settings': {
      return Promise.resolve(loadJSON(SETTINGS_KEY, { ...DEFAULT_SETTINGS }))
    }

    case 'apply_opacity': {
      return Promise.resolve()
    }

    // ---- Date ----
    case 'get_current_date': {
      const now = new Date()
      const weekdays = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六']
      return Promise.resolve({
        solar_date: `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`,
        lunar_date: '-',
        weekday: weekdays[now.getDay()],
        lunar_year: '',
        lunar_month: '',
        lunar_day: '',
      })
    }

    // ---- Window management (no-ops in browser) ----
    case 'open_settings_window': {
      return Promise.resolve()
    }

    case 'close_settings_window': {
      return Promise.resolve()
    }

    // ---- Events（在浏览器中直接处理） ----
    case 'emit_theme_changed': {
      const theme = args?.theme
      document.body.className = theme === 'dark' ? 'dark-theme' : ''
      return Promise.resolve()
    }

    case 'emit_priority_color_changed': {
      return Promise.resolve()
    }

    case 'emit_sync_config_changed': {
      return Promise.resolve()
    }

    // ---- App info ----
    case 'get_app_version': {
      return Promise.resolve('1.0.1-web')
    }

    default:
      console.warn(`Unknown invoke command: ${cmd}`, args)
      return Promise.reject(new Error(`Unknown command: ${cmd}`))
  }
}
