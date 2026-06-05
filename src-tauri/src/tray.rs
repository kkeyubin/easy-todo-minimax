use crate::models::StickyPatch;
use crate::AppState;
use anyhow::Result;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State,
};

/// W5: 装 macOS 系统菜单栏 tray。
/// 菜单：4 种新建 + 显示所有 + 退出
/// 左键 tray = 新建文本便签
pub fn setup_tray(app: &AppHandle) -> Result<()> {
    let new_text = MenuItemBuilder::with_id("new_text", "📝  新建文本便签")
        .build(app)?;
    let new_todo = MenuItemBuilder::with_id("new_todo", "☑️  新建待办便签")
        .build(app)?;
    let new_link = MenuItemBuilder::with_id("new_link", "🔗  新建链接便签")
        .build(app)?;
    let new_image = MenuItemBuilder::with_id("new_image", "🖼  新建图片便签")
        .build(app)?;
    let show_all = MenuItemBuilder::with_id("show_all", "显示所有便签")
        .build(app)?;
    let hide_all = MenuItemBuilder::with_id("hide_all", "隐藏所有便签")
        .build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出")
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&new_text)
        .item(&new_todo)
        .item(&new_link)
        .item(&new_image)
        .separator()
        .item(&show_all)
        .item(&hide_all)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&quit)
        .build()?;

    TrayIconBuilder::new()
        .icon(
            app.default_window_icon()
                .ok_or_else(|| anyhow::anyhow!("无法获取默认窗口图标"))?
                .clone(),
        )
        .tooltip("Easy Sticky")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "new_text" => new_sticky(app, "text"),
                "new_todo" => new_sticky(app, "todo"),
                "new_link" => new_sticky(app, "link"),
                "new_image" => new_sticky(app, "image"),
                "show_all" => show_all_stickies(app),
                "hide_all" => hide_all_stickies(app),
                "quit" => app.exit(0),
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // 左键单击 = 新建文本便签
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                new_sticky(tray.app_handle(), "text");
            }
        })
        .build(app)?;

    Ok(())
}

/// 通过 tray / 便签工具栏新建便签。位置自动 cascade。
fn new_sticky(app: &AppHandle, sticky_type: &str) {
    let state: State<'_, AppState> = app.state();
    let result: Result<()> = (|| {
        // 计算 cascade 位置
        let stickies = {
            let svc = state.service.blocking_lock();
            tauri::async_runtime::block_on(async { svc.list().await })?
        };
        let (cx, cy) = match stickies.first() {
            Some(s) => {
                let nx = s.x + 30;
                let ny = s.y + 30;
                if nx > 1000 || ny > 800 { (200, 200) } else { (nx, ny) }
            }
            None => (200, 200),
        };

        // 创建 + 持久化
        let sticky = {
            let svc = state.service.blocking_lock();
            tauri::async_runtime::block_on(async {
                svc.create(sticky_type.to_string(), cx, cy).await
            })?
        };

        // 开窗
        let wm = state.window_mgr.blocking_lock();
        wm.open_sticky_window(&sticky)?;
        Ok(())
    })();

    if let Err(e) = result {
        log::error!("[tray] new_sticky({}) failed: {}", sticky_type, e);
    }
}

fn show_all_stickies(app: &AppHandle) {
    let state: State<'_, AppState> = app.state();
    let result: Result<()> = (|| {
        let stickies = {
            let svc = state.service.blocking_lock();
            tauri::async_runtime::block_on(async { svc.list().await })?
        };
        for s in &stickies {
            let _ = state.window_mgr.blocking_lock().focus_sticky_window(s.id);
        }
        Ok(())
    })();
    if let Err(e) = result {
        log::error!("[tray] show_all failed: {}", e);
    }
}

fn hide_all_stickies(app: &AppHandle) {
    // 拿到所有 webview 窗口，hide
    for (_label, window) in app.webview_windows() {
        if window.label().starts_with("sticky-") {
            let _ = window.hide();
        }
    }
}

// 保留一个 stub 让 StickyPatch 在 tray.rs 也能用
#[allow(dead_code)]
fn _use_sticky_patch() -> StickyPatch {
    StickyPatch::default()
}
