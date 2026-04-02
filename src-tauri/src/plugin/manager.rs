use crate::plugin::interface::PluginInfo;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// 插件管理器 - 管理插件的加载、卸载和生命周期
pub struct PluginManager {
    /// 已加载的插件信息
    plugins: Mutex<HashMap<String, PluginInfo>>,
    /// 插件目录
    plugin_dir: PathBuf,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugins: Mutex::new(HashMap::new()),
            plugin_dir,
        }
    }

    /// 加载插件（从动态库路径）
    ///
    /// 注意：完整的动态库加载需要 `libloading` crate，
    /// 当前实现为框架层，提供注册和元数据管理。
    /// 实际的动态库加载将在后续版本中完善。
    pub fn load_plugin(&self, path: &Path) -> Result<PluginInfo, String> {
        if !path.exists() {
            return Err(format!("插件文件不存在: {}", path.display()));
        }

        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // 检查是否已加载
        {
            let plugins = self.plugins.lock().map_err(|e| e.to_string())?;
            if plugins.contains_key(&file_name) {
                return Err(format!("插件 '{}' 已加载", file_name));
            }
        }

        // TODO: 实际的动态库加载 (需要 libloading crate)
        // 当前为框架层实现，创建一个模拟的插件信息
        let info = PluginInfo {
            name: file_name.clone(),
            version: "0.1.0".to_string(),
            description: format!("Plugin loaded from {}", path.display()),
            author: None,
            loaded: true,
            path: Some(path.to_string_lossy().to_string()),
            registered_solvers: Vec::new(),
            registered_importers: Vec::new(),
            registered_exporters: Vec::new(),
            registered_materials: Vec::new(),
        };

        {
            let mut plugins = self.plugins.lock().map_err(|e| e.to_string())?;
            plugins.insert(file_name.clone(), info.clone());
        }

        tracing::info!("Plugin '{}' loaded from {}", file_name, path.display());
        Ok(info)
    }

    /// 卸载插件
    pub fn unload_plugin(&self, name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.lock().map_err(|e| e.to_string())?;
        if let Some(mut info) = plugins.remove(name) {
            info.loaded = false;
            tracing::info!("Plugin '{}' unloaded", name);
            Ok(())
        } else {
            Err(format!("插件 '{}' 未找到", name))
        }
    }

    /// 列出所有已加载的插件
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.lock().unwrap_or_else(|e| e.into_inner());
        plugins.values().cloned().collect()
    }

    /// 获取指定插件信息
    pub fn get_plugin_info(&self, name: &str) -> Result<PluginInfo, String> {
        let plugins = self.plugins.lock().map_err(|e| e.to_string())?;
        plugins
            .get(name)
            .cloned()
            .ok_or_else(|| format!("插件 '{}' 未找到", name))
    }

    /// 获取插件目录
    pub fn get_plugin_dir(&self) -> &Path {
        &self.plugin_dir
    }

    /// 调用插件钩子
    pub fn call_hook(&self, _event: &str, _data: serde_json::Value) -> Vec<serde_json::Value> {
        // TODO: 实现钩子调用机制
        // 当前为框架层实现
        Vec::new()
    }

    /// 检查插件是否已加载
    pub fn is_plugin_loaded(&self, name: &str) -> bool {
        let plugins = self.plugins.lock().unwrap_or_else(|e| e.into_inner());
        plugins.contains_key(name)
    }
}
