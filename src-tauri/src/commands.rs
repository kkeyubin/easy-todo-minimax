use std::path::PathBuf;

use tauri::Manager;
use tauri::State;
use uuid::Uuid;

use crate::models::{Sticky, StickyPatch, Todo, TodoPatch};
use crate::AppState;

fn images_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("images");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

#[tauri::command]
pub async fn list_stickies(state: State<'_, AppState>) -> Result<Vec<Sticky>, String> {
    let svc = state.service.lock().await;
    svc.list().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_sticky(state: State<'_, AppState>, id: i64) -> Result<Sticky, String> {
    let svc = state.service.lock().await;
    svc.get(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_sticky(
    state: State<'_, AppState>,
    sticky_type: String,
    x: i32,
    y: i32,
) -> Result<Sticky, String> {
    let sticky = {
        let svc = state.service.lock().await;
        svc.create(sticky_type, x, y)
            .await
            .map_err(|e| e.to_string())?
    };
    let wm = state.window_mgr.lock().await;
    wm.open_sticky_window(&sticky).map_err(|e| e.to_string())?;
    Ok(sticky)
}

#[tauri::command]
pub async fn update_sticky(
    state: State<'_, AppState>,
    id: i64,
    patch: StickyPatch,
) -> Result<(), String> {
    let svc = state.service.lock().await;
    svc.update(id, patch).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_sticky(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    {
        let wm = state.window_mgr.lock().await;
        wm.close_sticky_window(id);
    }
    let svc = state.service.lock().await;
    svc.delete(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_todos(
    state: State<'_, AppState>,
    sticky_id: i64,
) -> Result<Vec<Todo>, String> {
    let svc = state.service.lock().await;
    svc.list_todos(sticky_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_todo(
    state: State<'_, AppState>,
    sticky_id: i64,
    text: String,
) -> Result<Todo, String> {
    let svc = state.service.lock().await;
    svc.add_todo(sticky_id, text).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_todo(
    state: State<'_, AppState>,
    id: i64,
    patch: TodoPatch,
) -> Result<(), String> {
    let svc = state.service.lock().await;
    svc.update_todo(id, patch).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_todo(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let svc = state.service.lock().await;
    svc.delete_todo(id).await.map_err(|e| e.to_string())
}

/// 窗口位置/大小变化时由前端调用（debounce 后），用于持久化窗口几何
#[tauri::command]
pub async fn patch_window_state(
    state: State<'_, AppState>,
    id: i64,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<(), String> {
    let svc = state.service.lock().await;
    svc.update(
        id,
        StickyPatch {
            x: Some(x),
            y: Some(y),
            width: Some(width),
            height: Some(height),
            ..Default::default()
        },
    )
    .await
    .map_err(|e| e.to_string())
}

/// W2.3: 打开/聚焦一个便签窗口
#[tauri::command]
pub async fn show_sticky(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let needs_recreate = {
        let wm = state.window_mgr.lock().await;
        !wm.focus_sticky_window(id)
    };
    if !needs_recreate {
        return Ok(());
    }
    let sticky = {
        let svc = state.service.lock().await;
        svc.get(id).await.map_err(|e| e.to_string())?
    };
    let wm = state.window_mgr.lock().await;
    wm.open_sticky_window(&sticky)
        .map_err(|e| e.to_string())
}

// ============ W4.2 图片便签 ============

/// 从文件路径复制图片到 app_data_dir/images/，更新 sticky.image_path
#[tauri::command]
pub async fn add_sticky_image(
    state: State<'_, AppState>,
    id: i64,
    src_path: String,
) -> Result<String, String> {
    let src = PathBuf::from(&src_path);
    if !src.exists() {
        return Err(format!("source file not found: {}", src_path));
    }
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_lowercase();
    let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
    let dir = images_dir(&state.app_handle)?;
    let dest = dir.join(&new_filename);
    std::fs::copy(&src, &dest).map_err(|e| format!("copy failed: {}", e))?;
    let rel_path = format!("images/{}", new_filename);
    state
        .service
        .lock()
        .await
        .update(
            id,
            StickyPatch {
                image_path: Some(Some(rel_path.clone())),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(rel_path)
}

/// 移除便签图片（清 DB 字段 + 删文件）
#[tauri::command]
pub async fn remove_sticky_image(
    state: State<'_, AppState>,
    id: i64,
) -> Result<(), String> {
    let image_path = {
        let svc = state.service.lock().await;
        svc.get(id).await.map_err(|e| e.to_string())?.image_path
    };
    state
        .service
        .lock()
        .await
        .update(
            id,
            StickyPatch {
                image_path: Some(None),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.to_string())?;
    if let Some(rel) = image_path {
        let abs = state
            .app_handle
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?
            .join(&rel);
        let _ = std::fs::remove_file(&abs);
    }
    Ok(())
}

/// W4.2: 暴露 app_data_dir 给前端
#[tauri::command]
pub fn get_app_data_dir(state: State<'_, AppState>) -> Result<String, String> {
    state
        .app_handle
        .path()
        .app_data_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}
