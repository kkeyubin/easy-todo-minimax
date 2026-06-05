use crate::models::Sticky;
use anyhow::{Context, Result};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub struct WindowManager {
    app: AppHandle,
}

impl WindowManager {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    /// 打开（或聚焦）一个便签窗口。W1 阶段所有便签都用同一个默认 URL `/sticky/:id`，
    /// 后续 W2 起会在窗口里按 type 切换不同的编辑器组件。
    pub fn open_sticky_window(&self, sticky: &Sticky) -> Result<()> {
        let label = format!("sticky-{}", sticky.id);

        // 已经开过则聚焦
        if let Some(existing) = self.app.get_webview_window(&label) {
            existing.set_focus().ok();
            return Ok(());
        }

        let url_path = format!("/sticky/{}", sticky.id);
        let mut builder = WebviewWindowBuilder::new(
            &self.app,
            &label,
            WebviewUrl::App(url_path.into()),
        )
        .title(format!("Sticky #{}", sticky.id))
        .inner_size(sticky.width as f64, sticky.height as f64)
        .position(sticky.x as f64, sticky.y as f64)
        .resizable(true)
        .min_inner_size(180.0, 180.0)
        .decorations(false) // W2: 自绘圆角无边框
        .transparent(true)  // macOS private API：让圆角 + 阴影正确显示
        .shadow(true)       // 系统级窗口阴影
        .visible(true)
        .skip_taskbar(false);

        if sticky.pinned != 0 {
            builder = builder.always_on_top(true);
        }

        builder
            .build()
            .with_context(|| format!("build window for sticky {}", sticky.id))?;
        Ok(())
    }

    pub fn close_sticky_window(&self, sticky_id: i64) {
        let label = format!("sticky-{}", sticky_id);
        if let Some(window) = self.app.get_webview_window(&label) {
            let _ = window.close();
        }
    }

    /// 已开窗 → focus；未开窗 → 返回 false（调用方从 DB 重建）
    pub fn focus_sticky_window(&self, sticky_id: i64) -> bool {
        let label = format!("sticky-{}", sticky_id);
        match self.app.get_webview_window(&label) {
            Some(window) => {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
                true
            }
            None => false,
        }
    }
}
