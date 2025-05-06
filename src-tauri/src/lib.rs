// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri::menu::{MenuBuilder};
use tauri::{Emitter, State};

struct AppState(Mutex<AppStateInner>);

struct AppStateInner {
    root_folder: String,
}

impl AppState {
    pub fn new(root_folder: &str) -> Self {
        AppState(Mutex::new(AppStateInner {
            root_folder: root_folder.to_owned(),
        }))
    }
    pub fn get_root_folder(&self) -> String {
        self.0.lock().unwrap().root_folder.clone()
    }
    pub fn get_root_folder_name(&self) -> String {
        let root_folder = self.get_root_folder();
        let folder = Path::new(&root_folder);
        let mut folder_name = String::new();
        if folder.exists() {
            folder_name = folder
                .file_name()
                .unwrap_or_else(|| folder.as_os_str())
                .to_string_lossy()
                .to_string();
        }
        folder_name
    }
    pub fn set_root_folder(&self, root_folder: String) {
        self.0.lock().unwrap().root_folder = root_folder;
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
    app.log("get_root_path");
    state.get_root_folder()
}

#[tauri::command]
fn get_root_path_name(state: State<AppState>, app: AppHandle) -> String {
    app.log("get_root_path_name");
    state.get_root_folder_name()
}

#[tauri::command]
fn set_root_path(state: State<AppState>, app: AppHandle, root_folder: String) {
    app.log("set_root_path");
    let folder = Path::new(&root_folder);
    if folder.exists() {
        // let folder_name = folder.file_name().unwrap_or_else(|| folder.as_os_str()).to_string_lossy().to_string();
        // state.set_root_folder(folder_name);
        state.set_root_folder(root_folder);
    }
}

#[derive(Serialize)]
struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
}

#[tauri::command]
fn list_directory(path: String) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    if let Ok(dir) = fs::read_dir(path) {
        for entry in dir.flatten() {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            let file_path = entry.path().to_string_lossy().to_string();
            let is_dir = entry.path().is_dir();
            if is_dir || file_name.to_lowercase().ends_with(".html") {
                entries.push(FileEntry {
                    name: file_name,
                    path: file_path,
                    is_dir,
                });
            }
        }
    }
    entries
}

#[tauri::command]
fn read_html_file(path: String) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "<p>Not exist file</p>".to_string())
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let arg_path = args.get(1).map(|path| PathBuf::from(path));
    let default_path = env::current_exe().unwrap().parent().unwrap().to_path_buf();

    let root_path = match arg_path {
        Some(path) => {
            if path.exists() {
                path
            } else {
                default_path
            }
        }
        None => default_path,
    };
    let root_folder = root_path.to_string_lossy().into_owned();

    tauri::Builder::default()
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                .text("home", "Home")
                .build()?;
            app.set_menu(menu)?;
            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                match event.id().0.as_str() {
                    "home" => {
                        let _ = app_handle.get_webview_window("main").unwrap().eval("window.document.location.href = \"http://tauri.localhost/\"");
                    }
                    _ => {

                    }
                }
            });

            Ok(())
        })
        .manage(AppState::new(&root_folder))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_root_path,
            get_root_path_name,
            set_root_path,
            list_directory,
            read_html_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
