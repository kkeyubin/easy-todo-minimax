# easy-todo-minimax

macOS 桌面便签，主打"随手贴、轻量、本地"。每个便签独立浮在桌面，标题/颜色/大小/置顶/类型都可调，本地存储，零云依赖。

> 当前是仓库初始化 + 文档 + W1 脚手架。**`cargo check` 暂未通过**，原因见底部「踩坑记录」。

## 4 种便签类型

- **文本** — 标题（可选）+ markdown 文本，**实时渲染**所见即所得
- **待办** — 标题 + checkbox 列表（已完成折叠、增删、排序）
- **链接** — 标题 + 链接卡片，点击打开默认浏览器
- **图片** — 标题（可选）+ 图片（拖入 / 选 / 粘贴）

## 6 个 P0 功能

1. 置顶（窗口 always-on-top，可切换）
2. 窗口大小（边角 8 向 resize + 拖动）
3. 便签标题（顶部 title 行，可空）
4. 颜色更换（8 色 pastel）
5. 待办（独立便签类型）
6. 贴图 = 图片便签

## 技术栈

| 层 | 选型 |
|----|------|
| 壳 | Tauri 2 |
| 前端 | Vue 3 + TypeScript + Vite |
| UI | Naive UI |
| 状态 | Pinia + Vue Router 4 |
| 持久化 | SQLite + sqlx |
| 架构 | 多 webview 独立窗口（一个便签一个 webview）|
| markdown | markdown-it + 透明 textarea + 渲染层 + 300ms 防抖 |

详见 `docs/PRD.md` / `docs/ARCHITECTURE.md` / `docs/ROADMAP.md`。

## 项目结构

```
easy-todo-minimax/
├── docs/                # 设计文档
│   ├── PRD.md
│   ├── ARCHITECTURE.md
│   └── ROADMAP.md
├── src/                 # 前端 Vue 3
│   ├── main.ts
│   ├── App.vue
│   ├── pages/StickyWindow.vue
│   ├── components/      # W2+ 填充
│   ├── stores/sticky.ts
│   ├── ipc/index.ts
│   ├── types/sticky.ts
│   └── router/index.ts
├── src-tauri/           # 后端 Rust
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── models.rs
│   │   ├── storage.rs
│   │   ├── service.rs
│   │   ├── window_mgr.rs
│   │   └── commands.rs
│   ├── migrations/0001_initial.sql
│   ├── tauri.conf.json
│   ├── Cargo.toml
│   └── icons/           # 占位 1×1 透明 PNG（W5 替换正式 icon）
├── index.html
├── package.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
└── pnpm-workspace.yaml
```

## 进度

- [x] W1 脚手架 + 前端骨架 + 后端骨架（源文件全部就位）
- [x] PRD / 架构 / 路线图文档
- [ ] `cargo check` 通过（卡 rustc/Tauri/objc2 兼容，见下）
- [ ] W2 起：自绘窗口 / 实时 markdown / 颜色 / 置顶 / 拖动 / 缩放
- [ ] W3 待办 / W4 链接 + 图片 / W5 Menu Bar + 收尾

## 踩坑记录（环境相关，**非代码问题**）

### objc2 0.5 / 0.6 trait 解析爆炸

`cargo check` 失败：

```
error[E0275]: overflow evaluating the requirement `&_: IntoIterator`
required for `&Retained<Retained<Retained<...>>>` to implement `IntoIterator`
```

根因：Tauri 2.x 用的 `objc2` 0.5/0.6 在 macOS 上 trait 解析触发 rustc 编译期无限递归。这是 [madsmtm/objc2#527](https://github.com/madsmtm/objc2/issues/527) 的已知问题，1.85 / 1.88 / 1.95 三个 stable 都撞。

### 已试过的版本组合

| rustc | Tauri | 结果 |
|-------|-------|------|
| 1.95 | 2.11 | objc2 0.6 递归 ❌ |
| 1.88 | 2.11 | objc2 0.6 递归 ❌ |
| 1.88 | 2.0.4 | objc2 0.5 递归 ❌ |
| 1.86 | 2.0.4 | cargo 拉 edition2024 deps 卡 ❌ |
| 1.85 | 2.0.4 | plist 1.9 / time 0.3.47 要 1.88 ❌ |
| 1.80 | 2.0.4 | time-macros 0.2.27 要 edition2024 ❌ |

### 解决路径

1. **等 objc2 修复 issue**（推荐，等几个 minor release）
2. **切到 Tauri 1.x**（API 完全不同，工作量大）
3. **切到 Electron**（包体大、内存多）
4. **继续往前走**：W1 源文件已就位，`cargo check` 一过即可 `pnpm tauri dev` 跑起来

## 开发

```bash
# 前端独立开发（不依赖 Tauri 后端）
pnpm dev

# Tauri 全栈（需 cargo check 通过）
pnpm tauri:dev

# 构建 DMG
pnpm tauri:build
```

## License

TBD
