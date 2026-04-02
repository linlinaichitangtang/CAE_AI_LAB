use crate::plugin::interface::PluginInfo;
use crate::plugin::manager::PluginManager;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

/// 插件系统状态（由 Tauri 管理）
pub struct PluginState {
    pub manager: Mutex<PluginManager>,
}

impl PluginState {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            manager: Mutex::new(PluginManager::new(plugin_dir)),
        }
    }
}

/// 列出所有已加载的插件
#[tauri::command]
pub fn list_plugins(state: tauri::State<'_, PluginState>) -> Vec<PluginInfo> {
    let manager = state.manager.lock().unwrap_or_else(|e| e.into_inner());
    manager.list_plugins()
}

/// 加载插件
#[tauri::command]
pub fn load_plugin(state: tauri::State<'_, PluginState>, path: String) -> Result<PluginInfo, String> {
    let plugin_path = PathBuf::from(&path);
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.load_plugin(&plugin_path)
}

/// 卸载插件
#[tauri::command]
pub fn unload_plugin(state: tauri::State<'_, PluginState>, name: String) -> Result<(), String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.unload_plugin(&name)
}

/// 获取插件信息
#[tauri::command]
pub fn get_plugin_info(state: tauri::State<'_, PluginState>, name: String) -> Result<PluginInfo, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.get_plugin_info(&name)
}
