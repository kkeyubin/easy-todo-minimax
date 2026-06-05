# easy-todo-minimax · 路线图

> 5 周节奏，每个 W 结束有可演示产物。
> v0.1 = MVP 全部 4 种便签类型 + Menu Bar 入口。
> 标注 ⏱ 的是并行小任务（不阻塞主线）。

## W1 · 脚手架 + 窗口基建

> 交付：能跑起来的多窗口 app，新建一个空便签并保存到 DB，关闭再开能恢复。

- [ ] Tauri 2 + Vue 3 + TS + Vite 脚手架（`npm create tauri-app@latest` 起步）
- [ ] 配 `tauri.conf.json`：`productName=Easy Sticky`、`identifier=com.easytodo.sticky`、`macOSPrivateApi=true`
- [ ] 加 `tauri-plugin-shell`（链接打开要用）
- [ ] Cargo 依赖：`sqlx`、`tokio`、`serde`、`chrono`、`uuid`、`anyhow`、`thiserror`
- [ ] npm 依赖：`vue-router`、`pinia`、`naive-ui`、`markdown-it`、`@types/markdown-it`
- [ ] 目录结构按 `ARCHITECTURE.md` 第 2 节搭好
- [ ] SQL 迁移脚本 `0001_initial.sql`（含 stickies + todos 两表 + 索引）
- [ ] Rust: `Storage` 封装（连接池 + 迁移加载 + 事务辅助）
- [ ] Rust: `models.rs` 定义 `Sticky` / `StickyPatch` / `StickyType`
- [ ] Rust: `StickiesService`（CRUD + 锁内事务）
- [ ] Rust: `WindowManager`（创建/销毁 webview、置顶切换、z-order 维护）
- [ ] Rust: `commands.rs` 暴露 `list/get/create/update/delete` 五个基础 command
- [ ] Vue: 路由 `/sticky/:id` → `StickyWindow.vue`
- [ ] Vue: `ipc/index.ts` 封装 invoke
- [ ] Vue: `stores/sticky.ts` Pinia store
- [ ] Vue: `App.vue` 启动时 `ipc.list()` 拉数据，循环 `WindowManager.reopen()` 恢复所有便签
- [ ] 验证：手动建一个便签 → 关闭 app → 重开 → 便签位置/内容都还在
- [ ] ⏱ 写 `scripts/dev.sh` 集成 `pnpm tauri dev` + log 转发

**W1 Demo**：能新建/关闭/恢复便签，但便签窗口里只显示 id + 一个空 div。

## W2 · 文本便签核心

> 交付：能写一个文本便签，改颜色、改字号、置顶、拖动、缩放。

- [ ] Rust: `create_webview_window` 配 `decorations=false` + `transparent=true` + `shadow=true`
- [ ] Vue: `StickyCard.vue` 圆角 + 阴影 + 背景色（按 color id 映射）
- [ ] Vue: `StickyTitle.vue`（顶部一行，可编辑，blur 触发 update）
- [ ] Vue: `useDraggable` composable（用 `data-tauri-drag-region`，Tauri 2 自带 OS 级拖动）
- [ ] Vue: `useResizable` composable（4 边 + 4 角 8px 区域，mousedown → mousemove → mouseup 调 `window.setSize` + IPC patch）
- [ ] Vue: `StickyColorPicker.vue` 8 色面板（popover，hover 显色块）
- [ ] Vue: `TextEditor.vue` 双层叠加（透明 textarea + 渲染层 + 300ms 防抖）
- [ ] Vue: `StickyToolbar.vue` 底部工具栏（颜色 / 字号 A- A+ / 置顶 / 删 / 关闭）
- [ ] Rust: 监听窗口位置/大小变化 → debounce 500ms → `patch_sticky_window_state`
- [ ] Rust: 工具栏"置顶"按钮调 `window.setAlwaysOnTop`，同步 DB `pinned` 字段
- [ ] 字号 5 档：12 / 14 / 16 / 18 / 20，写入 `font_size` 字段
- [ ] 验证：拖 5 个便签、改 5 种颜色、置顶切换、关闭再开位置/颜色/字号全恢复

**W2 Demo**：5 个彩色便签，每个写点 markdown（粗体/列表/代码），随便拖随便缩放。

## W3 · 待办便签

> 交付：能创建一个待办便签，加子项、勾选、删除、已完成折叠。

- [ ] Vue: `TodoEditor.vue` 子项列表（添加 = 输入框回车 / + 按钮）
- [ ] Vue: 子项 checkbox 切换 → IPC `update_todo_done` → DB
- [ ] Vue: 子项删除（点 X / hover 出现）
- [ ] Vue: 子项文本双击进入编辑
- [ ] Vue: 已完成列表折叠（默认折叠） + 显示已完成数
- [ ] Rust: `commands.rs` 加 `list_todos / add_todo / update_todo / delete_todo / reorder_todos`
- [ ] Rust: `delete_sticky` 同步删除子项（外键 CASCADE 已处理）
- [ ] 验证：建 10 个子项、勾一半、删一个、改一个文本、拖动排序、关闭再开全恢复

**W3 Demo**：建一个待办便签，演示完整 CRUD + 折叠 + 重启恢复。

## W4 · 链接 + 图片

> 交付：4 种便签类型都能创建 + 完整使用。

### 链接便签
- [ ] Rust: 链接 `link_url` 字段 + 简单 `LinkPreview` 缓存（v0.1 简化为只存 URL，不抓 og:image）
- [ ] Vue: `LinkEditor.vue` 标题 + 链接卡片（域名 / URL / 点击 icon）
- [ ] Vue: 点击链接用 `@tauri-apps/plugin-shell` 的 `open()` 打开
- [ ] Vue: 粘贴 URL 自动识别（监听 title 框 paste 事件 + blur 后正则提取）

### 图片便签
- [ ] Rust: 图片接收 IPC（`add_sticky_image(sticky_id, src_path, ext)` → 复制到 `~/Library/Application Support/.../images/<uuid>.<ext>` → 更新 `image_path`）
- [ ] Rust: 孤儿图片清理（删除便签时检查引用计数）
- [ ] Vue: `ImageEditor.vue` 图片显示（用 `convertFileSrc` 协议转 `asset://`）
- [ ] Vue: 拖入支持（监听 `dragover` + `drop`，拿到 `dataTransfer.files`）
- [ ] Vue: 文件选择（工具栏 + 按钮 → 调 `@tauri-apps/plugin-dialog` 的 `open`）
- [ ] Vue: 粘贴支持（监听 `paste` 事件 + `clipboard.readImage`）
- [ ] Vue: 图片自适应（contain 模式，保持比例，背景纯色）
- [ ] 验证：拖入 / 选 / 粘贴各测一次图片，删便签后图片文件真的清理

**W4 Demo**：4 种便签各一个（文本/待办/链接/图片），演示完整交互。

## W5 · Menu Bar + 收尾

> 交付：Menu Bar 驻留入口，启动恢复稳定，DMG 可发。

- [ ] Rust: `tray.rs` 用 `TrayIconBuilder` 驻留 Menu Bar
- [ ] Tray 菜单：新建便签 / 显示所有便签（focus 全部窗口）/ 关于 / 退出
- [ ] Tray 左键点击 = 新建便签
- [ ] 启动恢复加 z-order 排序（按 DB `z_order` 倒序 reopen，让置顶的在上）
- [ ] 启动恢复校验窗口位置（不能超出当前显示器范围，否则 fallback 到屏幕中心）
- [ ] 性能 pass：连续开 10 个便签不卡顿、输入流畅、拖动不掉帧
- [ ] Bug pass：边界场景（最小尺寸、最大化、超出屏幕、删最后一个便签、app 升级后字段兼容）
- [ ] CI / 构建：`tauri build --target universal-apple-darwin` 出 DMG
- [ ] 写 `docs/RELEASE.md`（v0.1 已知问题 + 升级指引）
- [ ] ⏱ App 图标（暂时用占位 SF Symbol，正式版再设计）

**W5 Demo**：Menu Bar 驻留 + 新建 / 列表 / 退出 / 重启全便签窗口位置正确 + DMG 包。

## 进度跟踪

```
W1 ░░░░░░░░░░  脚手架 + 窗口基建
W2 ░░░░░░░░░░  文本便签核心
W3 ░░░░░░░░░░  待办便签
W4 ░░░░░░░░░░  链接 + 图片
W5 ░░░░░░░░░░  Menu Bar + 收尾
```

每个 W 结束我会出一个简短 demo summary，里程碑可挪动但不让 v0.1 漏 4 种类型中的任何一种。

## 风险缓冲

- **多 webview 内存**：5 个便签后还流畅没问题；>10 个便签时若卡顿，W5 加便签"虚拟化"（不可见的不创建 webview，缩到 < 100×100 时销毁，移回恢复）
- **图片备份策略**：v0.1 图片无云备份，重装系统丢；v0.2 加导出/导入
- **跨显示器**：v0.1 不做完整 multi-display 支持，启动恢复时窗口若不在任一显示器内 fallback 到主显示器中心
