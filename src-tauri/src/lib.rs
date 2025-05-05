// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::sync::{RwLock};
use std::path::Path;
use tauri::{Emitter, State};
use tauri::AppHandle;

struct AppState(RwLock<AppStateInner>);

struct AppStateInner {
    root_folder: String,
}

impl AppState {
    pub fn new(root_folder: &str) -> Self {
        AppState(RwLock::new(AppStateInner {
            root_folder: root_folder.to_owned(),
        }))
    }
    pub fn get_root_folder(&self) -> String {
        self.0.read().unwrap().root_folder.clone()
    }
    pub fn set_root_folder(&self, root_folder: String) {
        self.0.write().unwrap().root_folder = root_folder;
    }

}

pub trait LogExt {
    fn log(&self, message: &str);
}

impl LogExt for AppHandle {
    fn log(&self, message: &str) {
        let _ = self.emit("log", message);
    }
}


#[tauri::command]
fn get_root_path(state: State<AppState>, app: AppHandle) -> String {
    app.log("get_root_folder");
    state.get_root_folder()
}

#[tauri::command]
fn set_root_path(state: State<AppState>, app: AppHandle, root_folder: String) {
    app.log("set_root_folder");
    if Path::new(&root_folder).exists() {
        state.set_root_folder(root_folder);
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
        .manage(AppState::new(root_folder))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_root_path,
            set_root_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

