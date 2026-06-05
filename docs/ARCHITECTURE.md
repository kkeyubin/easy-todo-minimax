# easy-todo-minimax · 架构

## 1. 总体架构

```
┌──────────────────────────────────────────────────────────────┐
│  Main Process (Rust · Tauri 2)                               │
│                                                              │
│  ┌────────────────┐  ┌─────────────────┐  ┌──────────────┐  │
│  │ WindowManager  │  │ StickiesService │  │ Storage      │  │
│  │  · 创建/销毁 webview  │  · 业务逻辑（CRUD）  │  · SQLite    │  │
│  │  · 位置/大小/置顶  │  · 锁定变更          │  · 迁移      │  │
│  │  · z-order        │  · 事务             │  · 索引      │  │
│  └────────────────┘  └─────────────────┘  └──────────────┘  │
│         │                       │                   │         │
│         └──────────── IPC commands ─────────────────┘        │
└─────────┬──────────────┬──────────────┬───────────────────────┘
          │              │              │
          ▼              ▼              ▼
   ┌──────────┐   ┌──────────┐   ┌──────────┐
   │ webview  │   │ webview  │   │ webview  │   ← 每便签一个 webview
   │ (便签 1)  │   │ (便签 2)  │   │ (便签 N)  │     URL: /sticky/:id
   └──────────┘   └──────────┘   └──────────┘
        ▲             ▲              ▲
        └───── 共享 Vue 组件库 ──────┘
```

### 1.1 为什么选多窗口

| 方案 | 优势 | 劣势 |
|------|------|------|
| ✅ 多窗口（独立 webview）| 各自置顶/缩放/z-order/Mission Control 友好 | 内存开销 ~30MB/webview |
| ❌ 单窗口多卡片 | 内存低、IPC 简单 | 任意卡片无法独立置顶/缩放，参考图形态无法实现 |

5–10 个便签实际只占 150–300MB，Mac 端可接受。

## 2. 目录结构

```
easy-todo-minimax/
├── docs/                           # 设计文档
│   ├── PRD.md
│   ├── ARCHITECTURE.md
│   └── ROADMAP.md
├── src-tauri/                      # Rust 后端
│   ├── src/
│   │   ├── main.rs                 # 入口
│   │   ├── lib.rs                  # tauri::Builder
│   │   ├── window_mgr.rs           # 多窗口管理
│   │   ├── service.rs              # StickiesService（业务）
│   │   ├── storage.rs              # SQLite + 迁移
│   │   ├── models.rs               # Sticky / Todo / Color
│   │   ├── commands.rs             # IPC commands
│   │   └── tray.rs                 # Menu Bar
│   ├── migrations/
│   │   └── 0001_initial.sql
│   ├── tauri.conf.json
│   └── Cargo.toml
├── src/                            # Vue 3 前端
│   ├── main.ts                     # 启动
│   ├── App.vue                     # 根组件
│   ├── pages/
│   │   └── StickyWindow.vue        # 便签窗口主页面（按 type 路由）
│   ├── components/
│   │   ├── StickyCard.vue          # 卡片容器
│   │   ├── StickyTitle.vue         # 标题栏
│   │   ├── StickyToolbar.vue       # 底部工具栏
│   │   ├── StickyColorPicker.vue   # 颜色面板
│   │   ├── TextEditor.vue          # 文本便签（markdown 切换）
│   │   ├── TodoEditor.vue          # 待办便签
│   │   ├── LinkEditor.vue          # 链接便签
│   │   └── ImageEditor.vue         # 图片便签
│   ├── composables/
│   │   ├── useSticky.ts            # 拉/存/防抖
│   │   ├── useDraggable.ts         # 标题栏拖动
│   │   └── useResizable.ts         # 8 向 resize
│   ├── stores/
│   │   └── sticky.ts               # Pinia
│   ├── ipc/
│   │   └── index.ts                # invoke 封装
│   └── types/
│       └── sticky.ts               # Sticky / StickyType / Color
├── index.html
├── vite.config.ts
├── tsconfig.json
├── package.json
└── README.md
```

## 3. 数据模型

### 3.1 SQLite Schema

```sql
-- 0001_initial.sql

CREATE TABLE stickies (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  type          TEXT NOT NULL,            -- 'text' | 'todo' | 'link' | 'image'
  title         TEXT NOT NULL DEFAULT '',
  color         TEXT NOT NULL DEFAULT 'yellow',
  x             INTEGER NOT NULL,
  y             INTEGER NOT NULL,
  width         INTEGER NOT NULL DEFAULT 280,
  height        INTEGER NOT NULL DEFAULT 280,
  font_size     INTEGER NOT NULL DEFAULT 14,    -- 12 / 14 / 16 / 18 / 20
  pinned        INTEGER NOT NULL DEFAULT 0,     -- 0/1
  z_order       INTEGER NOT NULL DEFAULT 0,
  content       TEXT NOT NULL DEFAULT '',       -- text 全文 / markdown 源码
  image_path    TEXT,                           -- image 类型
  link_url      TEXT,                           -- link 类型
  created_at    TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_stickies_z_order ON stickies(z_order DESC);
CREATE INDEX idx_stickies_type ON stickies(type);

CREATE TABLE todos (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  sticky_id   INTEGER NOT NULL,
  text        TEXT NOT NULL,
  done        INTEGER NOT NULL DEFAULT 0,
  sort_order  INTEGER NOT NULL DEFAULT 0,
  created_at  TEXT NOT NULL DEFAULT (datetime('now')),
  FOREIGN KEY (sticky_id) REFERENCES stickies(id) ON DELETE CASCADE
);

CREATE INDEX idx_todos_sticky_id ON todos(sticky_id);
CREATE INDEX idx_todos_sort ON todos(sticky_id, sort_order);
```

### 3.2 颜色枚举

```ts
// src/types/sticky.ts
export const STICKY_COLORS = [
  { id: 'yellow', bg: '#fff3a3', border: '#e6d770' },
  { id: 'pink',   bg: '#ffd1dc', border: '#e8a8b8' },
  { id: 'red',    bg: '#ffb3a7', border: '#e89080' },
  { id: 'blue',   bg: '#b3d9ff', border: '#88b8e8' },
  { id: 'green',  bg: '#b8e6b8', border: '#90c890' },
  { id: 'purple', bg: '#d4b3ff', border: '#b08ce8' },
  { id: 'orange', bg: '#ffcc99', border: '#e8a868' },
  { id: 'gray',   bg: '#e0e0e0', border: '#b8b8b8' },
] as const;

export type StickyColorId = typeof STICKY_COLORS[number]['id'];
```

## 4. IPC 接口

主进程通过 `tauri::generate_handler!` 暴露 commands：

```rust
// commands.rs

#[tauri::command]
async fn list_stickies(app: AppHandle) -> Result<Vec<Sticky>, String>;

#[tauri::command]
async fn get_sticky(id: i64) -> Result<Sticky, String>;

#[tauri::command]
async fn create_sticky(sticky_type: String, x: i32, y: i32) -> Result<Sticky, String>;

#[tauri::command]
async fn update_sticky(id: i64, patch: StickyPatch) -> Result<(), String>;

#[tauri::command]
async fn delete_sticky(id: i64) -> Result<(), String>;

#[tauri::command]
async fn close_sticky_window(window: WebviewWindow) -> Result<(), String>;

// 标题/颜色/位置/大小变化时高频调用，Rust 端写 WAL + 300ms 防抖
#[tauri::command]
async fn patch_sticky_window_state(
    id: i64,
    x: i32, y: i32, width: i32, height: i32
) -> Result<(), String>;
```

前端封装：

```ts
// src/ipc/index.ts
export const ipc = {
  list: () => invoke<Sticky[]>('list_stickies'),
  get: (id: number) => invoke<Sticky>('get_sticky', { id }),
  create: (type: StickyType, x: number, y: number) =>
    invoke<Sticky>('create_sticky', { stickyType: type, x, y }),
  update: (id: number, patch: StickyPatch) =>
    invoke<void>('update_sticky', { id, patch }),
  delete: (id: number) => invoke<void>('delete_sticky', { id }),
};
```

## 5. 多窗口生命周期

### 5.1 创建流程

```
用户操作（Menu Bar 新建 / 点击 + 按钮）
    │
    ▼
Rust: create_sticky(type, x, y)
    │
    ├── 1. INSERT INTO stickies (创建 DB 行)
    │
    └── 2. WebviewWindowBuilder::new(handle, label=`sticky-${id}`)
            .url(format!("/sticky/{}", id))   // 路由带 id
            .decorations(false)
            .always_on_top(false)
            .skip_taskbar(false)              // dock 显示
            .build()
    │
    ▼
Vue 路由 /sticky/:id → StickyWindow
    │
    └── onMounted: ipc.get(id) 拉数据
```

### 5.2 拖动 / 缩放

- **拖动**：title bar 区域加 `data-tauri-drag-region`，Tauri 2 自带 OS 级拖动
- **8 向 resize**：4 边 + 4 角各 8px 区域，监听 mousedown 自己算偏移 → 调 `window.setSize()` + `patch_sticky_window_state()`
- 可选：`tauri-plugin-window-state` 接管，但会和我们自己存的窗口状态冲突，**先不引入**

### 5.3 关闭 vs 删除

- **关闭（X 按钮 / Cmd+W）**：仅销毁 webview，DB 行保留；下次启动时 WindowManager 启动阶段会从 DB 读所有 stickies 并重新开窗
- **删除（工具栏垃圾桶）**：弹确认 → 删 DB 行 → 删孤儿图片 → 销毁 webview
- Menu Bar 列表右键有"彻底删除"项

### 5.4 启动恢复

```
App 启动
    │
    ├── 1. 初始化 SQLite（运行迁移）
    │
    ├── 2. 读所有 stickies 行
    │
    └── 3. 对每行 reopen webview
            · 位置/大小从 DB 恢复
            · 置顶状态从 DB 恢复
            · z_order 从 DB 恢复（值越大越上面）
```

## 6. 关键技术点

### 6.1 无边框窗口 + 阴影

`tauri.conf.json`:
```json
{
  "windows": [],
  "app": {
    "macOSPrivateApi": true
  }
}
```

建窗时：
```rust
WebviewWindowBuilder::new(&handle, label, WebviewUrl::App(path.into()))
    .decorations(false)
    .transparent(true)                  // macOS 上配合 vibrancy
    .shadow(true)                       // 系统级阴影
    .build()?
```

CSS：
```css
.sticky-card {
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0,0,0,0.15);
  background: var(--sticky-bg);
  border: 1px solid var(--sticky-border);
  overflow: hidden;
}
```

### 6.2 实时渲染 markdown（文本便签）

**目标**：所见即所得，输入 markdown 源码停顿 300ms 后实时看到渲染结果，且光标不跳。

**实现**：双层叠加

```
┌─────────────────────────────┐
│ 渲染层 (pointer-events:none) │  ← markdown-it 输出 v-html
│  文字正常显示：粗体、列表、... │
├─────────────────────────────┤
│ 透明 textarea (覆盖在上面)    │  ← color: transparent
│  只有光标可见 (caret-color)   │     background: transparent
└─────────────────────────────┘
```

**约束**：
- 渲染层和 textarea 字号/字体/行高/padding/letter-spacing 严格一致，否则光标与文字错位
- 渲染层 `position: absolute; inset: 0; pointer-events: none;` 不抢焦点
- textarea `color: transparent; background: transparent; caret-color: <按颜色面板动态>`
- textarea 滚动时同步渲染层 `scrollTop`
- 输入监听 debounce 300ms 后调用 `markdown-it.render()` → 渲染层 innerHTML

**库**：`markdown-it`（轻量、成熟、中文友好、可扩展 callout / checkbox 等）

**避坑**：
- 不要用 contenteditable（光标、IME、删除、全选 race 一堆）
- 不要在 keystroke 同步触发渲染（防抖 300ms 必须）
- 不要在渲染层挂事件（`pointer-events: none` 已经杜绝）
- 字体回退：`-apple-system, "PingFang SC", "Helvetica Neue", sans-serif`

### 6.3 图片存储

- 目录：`~/Library/Application Support/com.easytodo.sticky/images/`
- 文件名：`<uuid>.<ext>`（避免冲突 + 不暴露原始名）
- DB 存相对路径，前端用 `convertFileSrc` 协议转 `asset://` URL
- 删除便签时检查引用计数，孤儿文件清理

### 6.4 链接便签

- 自动识别 URL：监听 title 输入框 blur 后做正则提取；或者粘贴时检测
- 显示用一个小卡片：网站 favicon（可选，v0.1 简化为文字链）+ URL
- 点击用 `tauri-plugin-shell` 的 `shell.open()` 打开默认浏览器

## 7. 第三方依赖（计划）

### Rust
- `tauri = "2"`
- `tauri-plugin-shell = "2"`（打开链接）
- `serde` / `serde_json`
- `sqlx` （SQLite，runtime-tokio-rustls）
- `tokio`
- `uuid`
- `chrono`
- `anyhow` / `thiserror`

### Node
- `vue@3`
- `vue-router@4`
- `pinia@2`
- `naive-ui`
- `markdown-it`
- `@tauri-apps/api@2`
- `@tauri-apps/cli@2`
- `vite`
- `typescript`
- `vue-tsc`

## 8. macOS 集成细节

- **Menu Bar**：`tauri::tray::TrayIconBuilder` 驻留
- **全局快捷键**（P1）：`tauri-plugin-global-shortcut`
- **App 图标**：复用 PM Todo 那套 SF Symbol 风格的设计语言（v0.1 暂用占位 icon）
- **Bundle ID**：`com.easytodo.sticky`
- **最低系统**：macOS 14 (Sonoma)，对齐 Naive UI + Tauri 2 要求
