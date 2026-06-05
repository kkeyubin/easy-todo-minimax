#![recursion_limit = "1024"]

mod commands;
mod models;
mod service;
mod storage;
mod window_mgr;

use tauri::Manager;
use tokio::sync::Mutex;

use crate::service::StickiesService;
use crate::storage::Storage;
use crate::window_mgr::WindowManager;

pub struct AppState {
    pub service: Mutex<StickiesService>,
    pub window_mgr: Mutex<WindowManager>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 启动 logger（设个默认 level）
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .try_init();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            // 1. 初始化 SQLite
            let storage = Storage::new(&app_handle)?;
            storage.run_migrations()?;

            // 2. 初始化 service + window manager
            let service = StickiesService::new(storage);
            let window_mgr = WindowManager::new(app_handle.clone());

            // 3. 启动恢复：从 DB 读所有便签，重新开窗
            let stickies = service.list_sync()?;
            log::info!("reopening {} sticky windows", stickies.len());
            for sticky in &stickies {
                if let Err(e) = window_mgr.open_sticky_window(sticky) {
                    log::error!("failed to reopen sticky {}: {}", sticky.id, e);
                }
            }

            // 4. 注入到 app state
            app.manage(AppState {
                service: Mutex::new(service),
                window_mgr: Mutex::new(window_mgr),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_stickies,
            commands::get_sticky,
            commands::create_sticky,
            commands::update_sticky,
            commands::delete_sticky,
            commands::list_todos,
            commands::add_todo,
            commands::update_todo,
            commands::delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
