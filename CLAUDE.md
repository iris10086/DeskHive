# CLAUDE.md

本文件为 Claude Code（claude.ai/code）在此仓库中编写代码时提供指导。

## 项目概述

DeskHive 是一个桌面待办事项应用，采用 **Vue 3 + TypeScript** 前端和 **Tauri 2.0 + Rust** 后端。包含三个独立的模块，本地数据存储在 JSON 文件中，同步数据通过 Python FastAPI 服务端中转。

---

## 整体架构（三个模块）

```
DeskHive/
├── src/                  # ┐ 桌面端（Tauri）Vue 前端
├── src-tauri/            # ┘ Tauri Rust 后端（数据层 + 窗口管理）
│
├── web/                  # Web 端 Vue 前端
│   └── (build → deskhive_server/static/)
│
├── deskhive_server/      # Python FastAPI 同步服务器
│   ├── api/              # REST API + WebSocket
│   ├── database.py       # SQLite 存储
│   ├── static/           # Web 前端构建产物（由 web/ 构建生成）
│   └── main.py           # FastAPI 入口
│
└── CLAUDE.md
```

### 模块关系

- **桌面端**（`src/` + `src-tauri/`）：独立的 Tauri 应用，自带 Vue 前端和 Rust 后端，所有数据存 `Documents/DeskHive/data/` 下的 JSON 文件
- **Web 端**（`web/`）：独立的 Vue 前端，构建产物输出到 `deskhive_server/static/`，由 FastAPI 服务器静态托管。数据存储在浏览器 `localStorage` 中，格式与桌面端 JSON 一致
- **同步服务器**（`deskhive_server/`）：Python FastAPI 服务，提供 REST API（`/api/todos`、`/api/groups`、`/api/sync`）和 WebSocket（`/ws`），数据存 SQLite。同时作为 Web 前端的静态托管服务器

### 同步流程

```
桌面端 ──POST /api/sync──→  同步服务器  ←──POST /api/sync──  Web 端
       ←─merge 结果返回───  (last-write-wins)  ──merge 结果返回─→
```

无论桌面端还是 Web 端，都通过 `sync.ts` 中的 `pushAndPull()` 调用 `POST /api/sync` 进行双向合并同步。

---

## 模块一：桌面端（`src/` + `src-tauri/`）

### 前端（`src/`，Vue 3 + TypeScript）

应用使用**双入口**构建模式（两个 HTML 页面，各有独立的入口点）：

- **主应用**（`index.html` → `src/main.ts` → `src/App.vue`）：主要的 300×420 置顶窗口
- **设置**（`settings.html` → `src/settings-main.ts` → `src/Settings.vue`）：通过 Tauri 命令启动的独立 800×600 窗口

**状态管理**：无 Pinia/Vuex — 所有状态集中在 `App.vue` 中，通过 props 向下传递到子组件，事件向上冒泡。子组件直接调用 Tauri 命令进行数据持久化。

**核心组件**（均在 `src/components/` 中）：
- `TodoList.vue` / `TodoGroup.vue` / `TodoItem.vue`：任务列表层级
- `TimelineView.vue`：时间线任务可视化
- `ContextMenu.vue` / `GroupContextMenu.vue`：右键菜单
- `AddTask.vue` / `AddTaskMenu.vue`：任务创建（支持 `/分组名` 快速创建）
- `DeadlineDialog.vue` / `EditTaskDialog.vue` / `GroupNameDialog.vue`：模态对话框
- `Tooltip.vue` / `Toast.vue`：UI 工具组件
- `EmptyState.vue` / `AllCompletedState.vue`：空状态展示

**注意**：`src/router.ts` 定义了 Vue Router 路由（`/` 和 `/settings`），但当前两个入口均未使用该路由，都是直接挂载根组件。

### 后端（`src-tauri/`，Tauri + Rust）

- **入口点**：`src-tauri/src/lib.rs` — 注册所有 Tauri 命令，管理窗口生命周期
- **数据层**（`src-tauri/src/data/`）：
  - `todo_data.rs`：任务和分组的 CRUD，以 JSON 形式持久化到 `Documents/DeskHive/data/`
  - `app_settings.rs`：设置持久化（透明度、主题、窗口层级、通知偏好）
  - `window_position.rs`：窗口位置保存/恢复
- **模型**（`src-tauri/src/models/`）：Serde 可序列化结构体
- **窗口管理**（`src-tauri/src/window/`）：
  - `management.rs`：显示/隐藏、切换、设置窗口的打开/关闭
  - `position.rs`：屏幕边界校验、右上角/居中定位
  - `opacity.rs`：通过 Windows API 设置窗口透明度
- **系统集成**（`src-tauri/src/system/`）：
  - `tray.rs`：系统托盘，包含显示/隐藏/设置/退出菜单
  - `date_info.rs`：中国农历日期信息
  - `auto_start.rs`：Windows 注册表自启动
- **通知**（`src-tauri/src/notification/`）：截止时间检查器 — 每 60 秒轮询一次，通过 `tauri-plugin-notification` 发送 Windows 弹窗通知

### 关键架构模式（桌面端）

- **关闭 = 最小化到托盘**：拦截窗口的 `CloseRequested` 事件，隐藏而非关闭窗口
- **Win+D 恢复**：后台线程监控窗口可见性，在 Win+D（显示桌面）后恢复窗口
- **单实例**：使用 Windows `CreateMutexW` 防止多实例运行
- **窗口层级**：通过 Windows HWND API（`SetWindowPos` + `HWND_BOTTOM`）支持 `normal` / `always_on_top` / `always_on_bottom`

---

## 模块二：Web 端（`web/`）

- 独立的 Vue 3 + TypeScript 前端项目
- 源码在 `web/` 目录，构建输出到 `deskhive_server/static/`（由 FastAPI 静态托管）
- 开发模式下运行 `npm run dev`（端口 5174），通过 `vite.config.ts` 的 proxy 转发 `/api`、`/health`、`/ws` 到 `http://127.0.0.1:8080`
- **数据存储**：本地数据存在 `localStorage`（`deskhive_todos`、`deskhive_groups`），格式与桌面端 JSON 文件一致
- **Tauri 命令适配**：`web/src/api.ts` 中的 `invoke()` 函数模拟了 Tauri 的命令接口，将 `save_todo_data_with_groups` 等命令映射到 `localStorage` 操作
- 代码逻辑与桌面端 `src/App.vue` 基本一致，共享相同的组件结构

---

## 模块三：同步服务器（`deskhive_server/`）

- Python FastAPI 应用，端口 8080
- 提供以下 REST API：
  - `GET /health` — 健康检查
  - `GET /api/todos` / `PUT /api/todos` — 任务 CRUD（全量读写）
  - `GET /api/groups` / `PUT /api/groups` — 分组 CRUD（全量读写）
  - `POST /api/sync` — 双向合并同步（last-write-wins by updated_at）
  - `WS /ws` — WebSocket，广播 `data_updated` 通知
- 数据存储在 `~/DeskHive/server/deskhive.db`（SQLite）

---

## 数据存储

### 桌面端（JSON 文件，`Documents/DeskHive/data/`）
- `todos_with_groups.json` — 带有分组关联的任务
- `groups.json` — 分组定义
- `todo_list.json` — 旧版扁平任务存储
- `app_settings.json` — 用户设置
- `window_position.json` — 保存的窗口位置

### Web 端（localStorage）
- `deskhive_todos` — 任务（格式同 `todos_with_groups.json`）
- `deskhive_groups` — 分组（格式同 `groups.json`）
- `deskhive_settings` — 应用设置

### 同步服务器（JSON 文件，`~/DeskHive/server/`）
- `todos_with_groups.json` — 任务（格式与桌面端 JSON 一致）
- `groups.json` — 分组（格式与桌面端 JSON 一致）

---

## 开发命令

### 桌面端
```bash
npm install              # 安装前端依赖
npm run dev              # 仅 Vite 开发服务器（http://localhost:5173）
npm run tauri dev        # 完整 Tauri 开发（前端 + Rust 后端）
npm run build            # TypeScript 检查 + Vite 生产构建
npm run tauri build      # 构建 MSI 安装包
```

### Web 端
```bash
cd web
npm install
npm run dev              # Vite 开发服务器（http://localhost:5174）
npm run build            # 构建到 deskhive_server/static/
```

### 同步服务器
```bash
pip install -r deskhive_server/requirements.txt
python -m deskhive_server  # 启动在 http://127.0.0.1:8080
```

---

## 开发指南

### 添加功能（桌面端）
1. 在 `src/components/` 中添加/复用 Vue 组件，或修改 `src/App.vue` / `src/Settings.vue`
2. 对于后端操作，在 `src-tauri/src/` 下的现有模块中添加 Rust 命令
3. 通过 `generate_handler![]` 在 `src-tauri/src/lib.rs` 中注册命令
4. 使用 `@tauri-apps/api/core` 的 `invoke()` 从前端调用
5. 通过现有的数据模块函数进行数据持久化

### 添加功能（Web 端）
- 在 `web/src/api.ts` 中添加新的 `invoke()` 命令映射
- 代码逻辑与桌面端 `src/App.vue` 保持一致

### 代码风格
- **Vue**：Composition API + `<script setup lang="ts">`，单文件组件
- **Rust**：标准约定，Serde 用于 JSON 序列化，Result 类型用于错误处理
- **桌面端前端调用后端**：通过 `import { invoke } from '@tauri-apps/api/core'`
- **Web 端模拟 invoke**：通过 `import { invoke } from './api'`

### 调试
- **桌面端前端**：运行 `npm run dev` 后按 F12 打开开发者工具
- **Web 端**：运行 `npm run dev`（端口 5174）后打开浏览器 DevTools
- **桌面端后端**：Tauri 开发模式下输出到控制台，大量使用 `println!`
- **同步服务器**：Python 日志输出到控制台

## 环境要求
- **Node.js**：^20.19.0 || >=22.12.0
- **Rust**：1.77.2+（通过 rustup 安装）
- **Windows**：10/11（64 位）
- **构建工具**：Visual Studio Build Tools（Windows 需要）
- **Python**：3.10+（同步服务器需要）

## 发布流程
1. 更新 `package.json` 和 `src-tauri/Cargo.toml` 中的版本号
2. 运行 `npm run tauri build` 生成 MSI 安装包
3. 在纯净的 Windows 虚拟机上测试安装包
4. 创建 GitHub Release 并附上更新日志


# 开发规范

## 修改原则
- 不修改现有实现逻辑，仅做必要且最小化的改动。
- 修改前先理解上下文，评估影响范围，避免副作用。

## 代码风格
- 保持新增代码行数最少，优先复用已有模块。
- 必须添加注释以说明意图和关键逻辑，注释简洁、精准，避免冗余。
- 注释与代码同步更新，不得留下过时注释。

## 质量保障
- 修改完成后必须运行编译检查，确保零错误零警告。
- 如有类型检查或 lint 脚本，需一并运行通过。