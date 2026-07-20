mod clicker;
mod key_binder;
mod state;

use serde::Serialize;
use state::{AppConfig, AppState, KeyBind};
use std::sync::atomic::Ordering;
use tauri::{Emitter, State};

#[derive(Serialize, Clone)]
pub struct AppStatus {
    clicker_running: bool,
    keys_running: bool,
}

fn emit_status(app: &tauri::AppHandle, state: &AppState) {
    let status = AppStatus {
        clicker_running: state.clicker_running.load(Ordering::SeqCst),
        keys_running: state.keys_running.load(Ordering::SeqCst),
    };
    let _ = app.emit("status-changed", status);
}

#[tauri::command]
fn start_clicker(
    app: tauri::AppHandle,
    state: State<AppState>,
    speed_ms: u64,
    button: String,
) -> Result<(), String> {
    if state.clicker_running.load(Ordering::SeqCst) {
        return Err("clicker already running".into());
    }
    let handle = clicker::spawn(speed_ms, button, state.clicker_running.clone())?;
    *state.clicker_handle.lock().map_err(|e| e.to_string())? = Some(handle);
    emit_status(&app, &state);
    Ok(())
}

#[tauri::command]
fn stop_clicker(app: tauri::AppHandle, state: State<AppState>) -> Result<(), String> {
    state.clicker_running.store(false, Ordering::SeqCst);
    if let Some(handle) = state.clicker_handle.lock().map_err(|e| e.to_string())?.take() {
        let _ = handle.join();
    }
    emit_status(&app, &state);
    Ok(())
}

#[tauri::command]
fn start_key_binder(
    app: tauri::AppHandle,
    state: State<AppState>,
    key_binds: Vec<KeyBind>,
) -> Result<(), String> {
    if state.keys_running.load(Ordering::SeqCst) {
        return Err("key binder already running".into());
    }
    let handles = key_binder::spawn(key_binds, state.keys_running.clone())?;
    *state.keys_handles.lock().map_err(|e| e.to_string())? = handles;
    emit_status(&app, &state);
    Ok(())
}

#[tauri::command]
fn stop_key_binder(app: tauri::AppHandle, state: State<AppState>) -> Result<(), String> {
    state.keys_running.store(false, Ordering::SeqCst);
    let handles = std::mem::take(&mut *state.keys_handles.lock().map_err(|e| e.to_string())?);
    for handle in handles {
        let _ = handle.join();
    }
    emit_status(&app, &state);
    Ok(())
}

#[tauri::command]
fn get_status(state: State<AppState>) -> Result<AppStatus, String> {
    Ok(AppStatus {
        clicker_running: state.clicker_running.load(Ordering::SeqCst),
        keys_running: state.keys_running.load(Ordering::SeqCst),
    })
}

#[tauri::command]
fn load_config() -> Result<AppConfig, String> {
    state::load_config()
}

#[tauri::command]
fn save_config(config: AppConfig) -> Result<(), String> {
    state::save_config(&config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            start_clicker,
            stop_clicker,
            start_key_binder,
            stop_key_binder,
            get_status,
            load_config,
            save_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
