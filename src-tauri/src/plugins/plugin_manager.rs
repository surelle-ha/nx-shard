use crate::configs::defaults::get_plugin_path;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::Mutex;

#[derive(Clone, serde::Serialize)]
pub struct PluginItem {
    pub id: String,
    pub name: String,
    pub link: String,
}

pub struct PluginManager {
    processes: Mutex<HashMap<String, Child>>,
}

// Static plugin list
static PLUGINS: Lazy<Vec<PluginItem>> = Lazy::new(|| {
    vec![PluginItem {
        id: "shard-anime".to_string(),
        name: "shard-anime".to_string(),
        link: "https://github.com/surelle-ha/shard-anime-api/releases/download/v5-c34a044e2504217c353858f8a7b4adcd5dbc78b4/shard-anime.exe".to_string(),
    }]
});

static PLUGIN_MANAGER: Lazy<PluginManager> = Lazy::new(|| PluginManager {
    processes: Mutex::new(HashMap::new()),
});

impl PluginManager {
    // Start all exe inside the get_plugin_path response
    pub fn start_plugins() -> Result<(), String> {
        let plugin_path = get_plugin_path();

        for plugin in PLUGINS.iter() {
            let exe_path = plugin_path.join(format!("{}.exe", plugin.name));

            if exe_path.exists() {
                match start_plugin_internal(&plugin.id) {
                    Ok(_) => println!("Started plugin: {}", plugin.name),
                    Err(e) => eprintln!("Failed to start plugin {}: {}", plugin.name, e),
                }
            }
        }

        Ok(())
    }
}

fn start_plugin_internal(plugin_id: &str) -> Result<(), String> {
    let plugin_path = get_plugin_path();
    let plugin = PLUGINS
        .iter()
        .find(|p| p.id == plugin_id)
        .ok_or_else(|| "Plugin not found".to_string())?;

    let exe_path = plugin_path.join(format!("{}.exe", plugin.name));

    let child = Command::new(&exe_path)
        .spawn()
        .map_err(|e| format!("Failed to start plugin: {}", e))?;

    PLUGIN_MANAGER
        .processes
        .lock()
        .unwrap()
        .insert(plugin_id.to_string(), child);
    Ok(())
}

fn stop_plugin_internal(plugin_id: &str) -> Result<(), String> {
    let mut processes = PLUGIN_MANAGER.processes.lock().unwrap();

    if let Some(mut child) = processes.remove(plugin_id) {
        child
            .kill()
            .map_err(|e| format!("Failed to kill plugin: {}", e))?;
    }

    Ok(())
}

pub fn stop_all_plugins() -> Result<(), String> {
    let plugin_ids: Vec<String> = PLUGIN_MANAGER
        .processes
        .lock()
        .unwrap()
        .keys()
        .cloned()
        .collect();

    for plugin_id in plugin_ids {
        if let Err(e) = stop_plugin_internal(&plugin_id) {
            tracing::warn!("Failed to stop plugin {}: {}", plugin_id, e);
        }
    }

    Ok(())
}

// Return available plugins
#[tauri::command]
pub fn get_available_plugins() -> Vec<PluginItem> {
    PLUGINS.clone()
}

// Check plugins path and check name + .exe, if file exist its classified as installed
#[tauri::command]
pub fn get_installed_plugins() -> Vec<PluginItem> {
    let plugin_path = get_plugin_path();
    let mut installed = Vec::new();

    for plugin in PLUGINS.iter() {
        let exe_path = plugin_path.join(format!("{}.exe", plugin.name));
        if exe_path.exists() {
            installed.push(plugin.clone());
        }
    }

    installed
}

// Download to plugins path
#[tauri::command]
pub async fn install_plugin(plugin_id: String) -> Result<(), String> {
    let plugin = PLUGINS
        .iter()
        .find(|p| p.id == plugin_id)
        .ok_or_else(|| "Plugin not found".to_string())?;

    let plugin_path = get_plugin_path();
    std::fs::create_dir_all(&plugin_path)
        .map_err(|e| format!("Failed to create plugin directory: {}", e))?;

    let exe_path = plugin_path.join(format!("{}.exe", plugin.name));

    // Download the file
    let response = reqwest::get(&plugin.link)
        .await
        .map_err(|e| format!("Failed to download plugin: {}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read plugin data: {}", e))?;

    std::fs::write(&exe_path, bytes).map_err(|e| format!("Failed to write plugin file: {}", e))?;

    Ok(())
}

// Remove the exe files from plugins
#[tauri::command]
pub fn remove_plugin(plugin_id: String) -> Result<(), String> {
    // Stop the plugin if running
    let _ = stop_plugin_internal(&plugin_id);

    let plugin = PLUGINS
        .iter()
        .find(|p| p.id == plugin_id)
        .ok_or_else(|| "Plugin not found".to_string())?;

    let plugin_path = get_plugin_path();
    let exe_path = plugin_path.join(format!("{}.exe", plugin.name));

    if exe_path.exists() {
        std::fs::remove_file(&exe_path).map_err(|e| format!("Failed to remove plugin: {}", e))?;
    }

    Ok(())
}

// Restart a single plugin
#[tauri::command]
pub fn restart_plugin(plugin_id: String) -> Result<(), String> {
    stop_plugin_internal(&plugin_id)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    start_plugin_internal(&plugin_id)?;
    Ok(())
}

// Restart all plugins
#[tauri::command]
pub fn restart_plugins() -> Result<(), String> {
    let plugin_ids: Vec<String> = PLUGIN_MANAGER
        .processes
        .lock()
        .unwrap()
        .keys()
        .cloned()
        .collect();

    for plugin_id in plugin_ids {
        let _ = restart_plugin(plugin_id);
    }

    Ok(())
}
