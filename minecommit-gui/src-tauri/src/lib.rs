use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Save {
    pub name: String,
    pub path: String,
    pub repo_path: String,
    pub remote_repo_path: String,
}

struct SaveState {
    saves: Mutex<Vec<Save>>,
    data_dir: PathBuf,
}

fn saves_file_path(data_dir: &PathBuf) -> PathBuf {
    data_dir.join("saves.json")
}

fn load_saves(data_dir: &PathBuf) -> Vec<Save> {
    let path = saves_file_path(data_dir);
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        // Ensure the data directory exists
        let _ = fs::create_dir_all(data_dir);
        vec![]
    }
}

fn save_saves(data_dir: &PathBuf, saves: &[Save]) {
    let path = saves_file_path(data_dir);
    let _ = fs::create_dir_all(data_dir);
    if let Ok(content) = serde_json::to_string_pretty(saves) {
        let _ = fs::write(&path, content);
    }
}

#[tauri::command]
fn list_saves(state: tauri::State<SaveState>) -> Vec<Save> {
    state.saves.lock().unwrap().clone()
}

#[tauri::command]
fn add_save(
    state: tauri::State<SaveState>,
    name: String,
    path: String,
    repo_path: String,
    remote_repo_path: String,
) -> Result<Save, String> {
    let mut saves = state.saves.lock().unwrap();

    // Check for duplicate name
    if saves.iter().any(|s| s.name == name) {
        return Err(format!("已存在名为「{}」的存档", name));
    }

    let save = Save {
        name,
        path,
        repo_path,
        remote_repo_path,
    };
    saves.push(save.clone());
    save_saves(&state.data_dir, &saves);
    Ok(save)
}

#[tauri::command]
fn delete_save(state: tauri::State<SaveState>, name: String) -> Result<(), String> {
    let mut saves = state.saves.lock().unwrap();
    let len_before = saves.len();
    saves.retain(|s| s.name != name);
    if saves.len() == len_before {
        return Err(format!("未找到名为「{}」的存档", name));
    }
    save_saves(&state.data_dir, &saves);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");

            let saves = load_saves(&data_dir);

            app.manage(SaveState {
                saves: Mutex::new(saves),
                data_dir,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_saves, add_save, delete_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
