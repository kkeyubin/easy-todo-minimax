#![recursion_limit = "1024"]

use tauri::Manager;

use crate::service::StickiesService;
use crate::storage::Storage;
use crate::window_mgr::WindowManager;

mod commands;
mod models;
mod service;
mod storage;
mod window_mgr;

pub struct AppState {
    pub service: tokio::sync::Mutex<crate::service::StickiesService>,
    pub window_mgr: tokio::sync::Mutex<crate::window_mgr::WindowManager>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            let storage = Storage::new(&app_handle)?;
            storage.run_migrations()?;

            let service = StickiesService::new(storage);
            let window_mgr = WindowManager::new(app_handle.clone());

            // 首次启动如果 DB 空，自动创建一个空便签作为引导
            let mut stickies = service.list_sync()?;
            if stickies.is_empty() {
                log::info!("empty db, seeding first sticky note");
                let first = service
                    .create_sync("text".to_string(), 200, 200)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
                stickies.push(first);
            }

            log::info!("reopening {} sticky windows", stickies.len());
            for sticky in &stickies {
                if let Err(e) = window_mgr.open_sticky_window(sticky) {
                    log::error!("failed to reopen sticky {}: {}", sticky.id, e);
                }
            }

            app.manage(AppState {
                service: tokio::sync::Mutex::new(service),
                window_mgr: tokio::sync::Mutex::new(window_mgr),
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
            commands::patch_window_state,
            commands::show_sticky,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
