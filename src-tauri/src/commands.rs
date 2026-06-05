use tauri::State;
use crate::models::{Sticky, StickyPatch, Todo, TodoPatch};
use crate::AppState;

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
        svc.create(sticky_type, x, y).await.map_err(|e| e.to_string())?
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
        crate::models::StickyPatch {
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

/// W2.3: 打开/聚焦一个便签窗口。
/// - 窗口已开 → focus
/// - 窗口被关（DB 还在）→ 从 DB 重建
#[tauri::command]
pub async fn show_sticky(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    // 先尝试 focus
    let needs_recreate = {
        let wm = state.window_mgr.lock().await;
        !wm.focus_sticky_window(id)
    };
    if !needs_recreate {
        return Ok(());
    }
    // 重建
    let sticky = {
        let svc = state.service.lock().await;
        svc.get(id).await.map_err(|e| e.to_string())?
    };
    let wm = state.window_mgr.lock().await;
    wm.open_sticky_window(&sticky).map_err(|e| e.to_string())
}
