#![recursion_limit = "1024"]

use tauri::Manager;

use crate::service::StickiesService;
use crate::storage::Storage;
use crate::window_mgr::WindowManager;

mod commands;
mod models;
mod service;
mod storage;
mod tray;
mod window_mgr;

pub struct AppState {
    pub service: tokio::sync::Mutex<StickiesService>,
    pub window_mgr: tokio::sync::Mutex<WindowManager>,
    pub app_handle: tauri::AppHandle,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            let storage = Storage::new(&app_handle)?;
            storage.run_migrations()?;

            let service = StickiesService::new(storage);
            let window_mgr = WindowManager::new(app_handle.clone());

            // 启动恢复：从 DB 读所有便签，重新开窗
            let stickies = service.list_sync()?;
            log::info!("reopening {} sticky windows", stickies.len());
            for sticky in &stickies {
                if let Err(e) = window_mgr.open_sticky_window(sticky) {
                    log::error!("failed to reopen sticky {}: {}", sticky.id, e);
                }
            }

            app.manage(AppState {
                service: tokio::sync::Mutex::new(service),
                window_mgr: tokio::sync::Mutex::new(window_mgr),
                app_handle: app_handle.clone(),
            });

            // 5. 装菜单栏 tray
            if let Err(e) = tray::setup_tray(&app_handle) {
                log::error!("tray setup failed: {}", e);
            }

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
            commands::patch_window_state,
            commands::show_sticky,
            commands::add_sticky_image,
            commands::remove_sticky_image,
            commands::get_app_data_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
