use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use std::io::Write;
use reqwest;

const GAME_PATH: &str = "./game_path";

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct GameMeta {
    id: u32,
    title: String,
    description: String,
    coverUrl: String,
    downloadUrl: String,
    tags: Vec<String>,
    isExperimental: bool,
    isEnabled: bool,
    isBroken: bool,
    createdAt: String,
}

// ------------------ START UP CHECK ------------------
#[tauri::command]
fn check_file_system() -> Result<(), String> {
    let dir = Path::new(GAME_PATH);

    if !dir.exists() {
        println!("Game Path does not exist. Creating directory.");
        fs::create_dir_all(dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    Ok(())
}

// ------------------ DOWNLOAD CONTROL ------------------
#[tauri::command]
fn get_game_meta(invoke_message: GameMeta) {
    println!("[Shard_Torrent_Backend] Obtaining game data:");
    println!("{:#?}", invoke_message);
}

#[tauri::command]
fn create_game_dir(invoke_message: GameMeta) -> Result<(), String> {
    let game_dir = Path::new(GAME_PATH).join(invoke_message.id.to_string());

    if !game_dir.exists() {
        println!("[Shard_Torrent_Backend] Creating directory for game id {}", invoke_message.id);
        fs::create_dir_all(&game_dir).map_err(|e| format!("Failed to create game directory: {}", e))?;
    } else {
        println!("[Shard_Torrent_Backend] Directory already exists for game id {}", invoke_message.id);
    }

    Ok(())
}

#[tauri::command]
async fn obtain_torrent_file(invoke_message: GameMeta) -> Result<(), String> {
    let game_dir = Path::new(GAME_PATH).join(invoke_message.id.to_string());
    if !game_dir.exists() {
        return Err(format!("Game directory does not exist for id {}", invoke_message.id));
    }

    let torrent_path = game_dir.join("game.torrent");
    println!("[Shard_Torrent_Backend] Downloading torrent to {:?}", torrent_path);

    let response = reqwest::get(&invoke_message.downloadUrl)
        .await
        .map_err(|e| format!("Failed to download torrent: {}", e))?;

    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to read torrent bytes: {}", e))?;

    let mut file = fs::File::create(&torrent_path)
        .map_err(|e| format!("Failed to create torrent file: {}", e))?;

    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write torrent file: {}", e))?;

    println!("[Shard_Torrent_Backend] Torrent file saved successfully.");
    Ok(())
}

#[tauri::command]
fn download_game(_invoke_message: GameMeta) {
    // Learn `librqbit` - https://github.com/ikatson/rqbit/blob/main/crates/librqbit/examples/ubuntu.rs
    println!("[Shard_Torrent_Backend] Download game function is empty for now.");
}

#[tauri::command]
fn pause_game(_invoke_message: GameMeta) {
    println!("[Shard_Torrent_Backend] Send pause signal to download daemon.");
}

#[tauri::command]
fn resume_game(_invoke_message: GameMeta) {
    println!("[Shard_Torrent_Backend] Send resume signal to download daemon.");
}

#[tauri::command]
fn extract_and_clean(invoke_message: GameMeta) -> Result<(), String> {
    let torrent_path = Path::new(GAME_PATH)
        .join(invoke_message.id.to_string())
        .join("game.torrent");

    if torrent_path.exists() {
        println!("[Shard_Torrent_Backend] Removing torrent file at {:?}", torrent_path);
        fs::remove_file(&torrent_path)
            .map_err(|e| format!("Failed to remove torrent file: {}", e))?;
    } else {
        println!("[Shard_Torrent_Backend] Torrent file does not exist, nothing to remove.");
    }

    Ok(())
}

// ------------------ SYSTEM INFO ------------------
#[tauri::command]
fn get_version() -> String {
    "1.0.0".into()
}

// ------------------ RUN ------------------
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            check_file_system,
            get_game_meta,
            create_game_dir,
            obtain_torrent_file,
            download_game,
            pause_game,
            resume_game,
            extract_and_clean,
            get_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
