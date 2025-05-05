// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::sync::{RwLock};
use tauri::{Emitter, State, Manager};
use std::path::Path;
use tauri::AppHandle;
use std::sync::OnceLock;

static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

struct AppState {
    root_folder: RwLock<String>,
    // args: Vec<String>,
}

fn init(app: AppHandle) {
    GLOBAL_APP_HANDLE.set(app).unwrap();
}

#[tauri::command]
fn get_root_folder(state: State<AppState>) -> String {
    log("get_root_folder");
    state.root_folder.read().unwrap().clone()
}

fn log(message: &str) {
    GLOBAL_APP_HANDLE.get().unwrap().emit("log", message).unwrap();
}
#[tauri::command]
fn set_root_folder(state: State<AppState>, root_folder: String) {
    if Path::new(&root_folder).exists() {
        *state.root_folder.write().unwrap() = root_folder;
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let exe_folder = env::current_exe().unwrap()
        .parent().unwrap()
        .to_string_lossy().into_owned();
    let root_folder: &String = args.get(1).unwrap_or(&exe_folder);

    tauri::Builder::default()
        .manage(AppState {
                root_folder: RwLock::new(root_folder.to_owned()),
        })
        .setup(|app| {
            init(app.app_handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_root_folder,
            set_root_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/