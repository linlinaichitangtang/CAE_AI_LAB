use std::fs;
use std::path::PathBuf;
use tauri::command;

#[command]
pub fn read_file(path: String) -> Result<String, String> {
    tracing::info!("Reading file: {}", path);
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

#[command]
pub fn write_file(path: String, content: String) -> Result<(), String> {
    tracing::info!("Writing file: {}", path);
    let path_buf = PathBuf::from(&path);
    if let Some(parent) = path_buf.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    fs::write(&path, content).map_err(|e| format!("Failed to write file: {}", e))
}