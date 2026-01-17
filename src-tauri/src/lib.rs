mod configs;
mod dbi;
mod plugins;
mod torrent;

use anyhow::Context;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use librqbit::{AddTorrent, AddTorrentOptions, AddTorrentResponse, ManagedTorrent, Session};
use log::{debug, error, info, warn};
use parking_lot::Mutex;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, Listener, Manager, State};
use tokio::sync::RwLock;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::dbi::{ftp_discovery, ftp_manager};

use crate::torrent::state::TorrentState;

use crate::configs::constants::{APP_PATH, CONFIG_PATH, GAME_PATH};
use crate::configs::defaults::{get_app_path, get_config_path, get_game_path};

use crate::plugins::plugin_manager;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
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
    let binding = get_game_path();
    let dir = Path::new(&binding);

    if !dir.exists() {
        warn!("Game Path does not exist. Creating directory.");
        fs::create_dir_all(dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    Ok(())
}

// ------------------ DOWNLOAD CONTROL ------------------
#[tauri::command]
fn get_game_meta(invoke_message: GameMeta) {
    info!("[Shard_Torrent_Backend] Obtaining game data:");
    info!("{:#?}", invoke_message);
}

#[tauri::command]
fn create_game_dir(invoke_message: GameMeta) -> Result<(), String> {
    let game_dir = Path::new(&get_game_path()).join(invoke_message.id.to_string());

    if !game_dir.exists() {
        info!(
            "[Shard_Torrent_Backend] Creating directory for game id {}",
            invoke_message.id
        );
        fs::create_dir_all(&game_dir)
            .map_err(|e| format!("Failed to create game directory: {}", e))?;
    } else {
        warn!(
            "[Shard_Torrent_Backend] Directory already exists for game id {}",
            invoke_message.id
        );
    }

    Ok(())
}

#[tauri::command]
async fn obtain_torrent_file(invoke_message: GameMeta) -> Result<(), String> {
    let game_dir = Path::new(&get_game_path()).join(invoke_message.id.to_string());
    if !game_dir.exists() {
        return Err(format!(
            "Game directory does not exist for id {}",
            invoke_message.id
        ));
    }

    let torrent_path = game_dir.join("game.torrent");
    info!(
        "[Shard_Torrent_Backend] Downloading torrent to {:?}",
        torrent_path
    );

    let response = reqwest::get(&invoke_message.downloadUrl)
        .await
        .map_err(|e| format!("Failed to download torrent: {}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read torrent bytes: {}", e))?;

    let mut file = fs::File::create(&torrent_path)
        .map_err(|e| format!("Failed to create torrent file: {}", e))?;

    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write torrent file: {}", e))?;

    info!("[Shard_Torrent_Backend] Torrent file saved successfully.");
    Ok(())
}

#[tauri::command]
async fn download_game(
    invoke_message: GameMeta,
    state: State<'_, TorrentState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Initialize logging
    match std::env::var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => unsafe { std::env::set_var("RUST_LOG", "info") },
    }

    let game_id = invoke_message.id;
    let game_dir = Path::new(&get_game_path()).join(game_id.to_string());
    let torrent_path = game_dir.join("game.torrent");

    // Check if torrent file exists
    if !torrent_path.exists() {
        return Err(format!(
            "Torrent file does not exist for game id {}",
            game_id
        ));
    }

    // Check if already downloading
    {
        let handles = state.handles.read().await;
        if handles.contains_key(&game_id) {
            return Err(format!("Game {} is already being downloaded", game_id));
        }
    }

    info!(
        "[Shard_Torrent_Backend] Starting download for game id {}",
        game_id
    );

    // Use the SHARED session instead of creating a new one
    let session = state.session.clone();

    // Add the torrent to the session with output_folder set to game-specific directory
    let torrent_path_str = torrent_path.to_string_lossy().to_string();
    let (torrent_id, handle) = match session
        .add_torrent(
            AddTorrent::from_local_filename(&torrent_path_str)
                .map_err(|e| format!("Failed to read torrent file: {}", e))?,
            Some(AddTorrentOptions {
                overwrite: true,
                output_folder: Some(game_dir.to_path_buf().to_string_lossy().to_string()),
                ..Default::default()
            }),
        )
        .await
        .context("error adding torrent")
        .map_err(|e| format!("Failed to add torrent: {}", e))?
    {
        AddTorrentResponse::Added(id, handle) => {
            info!(
                "[Shard_Torrent_Backend] Torrent added successfully with id: {}",
                id
            );
            (id, handle)
        }
        AddTorrentResponse::AlreadyManaged(id, handle) => {
            warn!(
                "[Shard_Torrent_Backend] Torrent already managed with id: {}, reusing handle",
                id
            );
            (id, handle)
        }
        AddTorrentResponse::ListOnly(_resp) => {
            return Err("Torrent added in list-only mode".to_string());
        }
    };

    // Store handle with torrent_id
    let handle: Arc<ManagedTorrent> = handle;
    {
        let mut handles = state.handles.write().await;
        handles.insert(game_id, (torrent_id, handle.clone()));
    }

    // Log metadata
    handle
        .with_metadata(|r| {
            info!("Details: {:?}", &r.info);
        })
        .map_err(|e| format!("Failed to read metadata: {}", e))?;

    // Start the torrent (unpause it to begin downloading)
    info!("[Shard_Torrent_Backend] Starting torrent download...");
    session
        .unpause(&handle)
        .await
        .map_err(|e| format!("Failed to start torrent: {}", e))?;

    // Print stats periodically and emit to frontend
    let start_time = std::time::Instant::now();
    tokio::spawn({
        let handle = handle.clone();
        let app_handle = app_handle.clone();
        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let stats = handle.stats();
                let elapsed = start_time.elapsed().as_secs();

                if let Some(live) = &stats.live {
                    let progress_percent =
                        (stats.progress_bytes as f64 / stats.total_bytes.max(1) as f64) * 100.0;

                    info!(
                        "[{}s] Game {} | State: {:?} | Progress: {:.1}% | Downloaded: {:.2} MB / {:.2} MB | Speed: {:.2} MB/s | Peers: {}",
                        elapsed,
                        game_id,
                        stats.state,
                        progress_percent,
                        stats.progress_bytes as f64 / 1_000_000.0,
                        stats.total_bytes as f64 / 1_000_000.0,
                        live.download_speed.mbps,
                        live.snapshot.peer_stats.live,
                    );

                    // Emit to frontend
                    let _ = app_handle.emit(
                        "download-progress",
                        serde_json::json!({
                            "gameId": game_id,
                            "state": format!("{:?}", stats.state),
                            "progress": progress_percent,
                            "downloadedBytes": stats.progress_bytes,
                            "totalBytes": stats.total_bytes,
                            "downloadSpeed": live.download_speed.mbps,
                            "uploadSpeed": live.upload_speed.mbps,
                            "peers": live.snapshot.peer_stats.live,
                        }),
                    );

                    // Warn if stuck at 0 peers
                    if elapsed > 30 && live.snapshot.peer_stats.live == 0 {
                        warn!(
                            "[WARNING] Game {} - No live peers found after 30 seconds.",
                            game_id
                        );
                    }
                } else {
                    let progress_percent =
                        (stats.progress_bytes as f64 / stats.total_bytes.max(1) as f64) * 100.0;

                    info!(
                        "[{}s] Game {} | State: {:?} | Progress: {:.1}%",
                        elapsed, game_id, stats.state, progress_percent,
                    );

                    // Emit to frontend
                    let _ = app_handle.emit(
                        "download-progress",
                        serde_json::json!({
                            "gameId": game_id,
                            "state": format!("{:?}", stats.state),
                            "progress": progress_percent,
                            "downloadedBytes": stats.progress_bytes,
                            "totalBytes": stats.total_bytes,
                            "downloadSpeed": 0.0,
                            "uploadSpeed": 0.0,
                            "peers": 0,
                        }),
                    );
                }

                // Break if completed
                if stats.finished {
                    break;
                }
            }
        }
    });

    // Wait for completion in background
    let handle_clone = handle.clone();
    let state_handles = state.handles.clone();
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        let result = tokio::select! {
            result = handle_clone.wait_until_completed() => result,
            _ = tokio::time::sleep(Duration::from_secs(3600)) => {
                Err(anyhow::anyhow!("Download timeout after 1 hour"))
            }
        };

        match result {
            Ok(_) => {
                info!("torrent downloaded");
                info!(
                    "[Shard_Torrent_Backend] Game {} download completed!",
                    game_id
                );

                let _ = app_handle_clone.emit(
                    "download-complete",
                    serde_json::json!({
                        "gameId": game_id,
                    }),
                );
            }
            Err(e) => {
                error!("[Shard_Torrent_Backend] Download failed: {}", e);

                let _ = app_handle_clone.emit(
                    "download-error",
                    serde_json::json!({
                        "gameId": game_id,
                        "error": e.to_string(),
                    }),
                );
            }
        }

        // Remove handle
        let mut handles = state_handles.write().await;
        handles.remove(&game_id);
    });

    Ok(())
}

#[tauri::command]
async fn pause_game(
    invoke_message: GameMeta,
    state: State<'_, TorrentState>,
) -> Result<(), String> {
    let game_id = invoke_message.id;
    let handles = state.handles.read().await;

    if let Some((_torrent_id, handle)) = handles.get(&game_id) {
        let handle = handle.clone();
        let session = state.session.clone();
        drop(handles); // Release lock before async operation

        session
            .pause(&handle)
            .await
            .map_err(|e| format!("Failed to pause: {}", e))?;
        info!(
            "[Shard_Torrent_Backend] Paused download for game id {}",
            game_id
        );
        Ok(())
    } else {
        Err(format!("No active download found for game id {}", game_id))
    }
}

#[tauri::command]
async fn resume_game(
    invoke_message: GameMeta,
    state: State<'_, TorrentState>,
) -> Result<(), String> {
    let game_id = invoke_message.id;
    let handles = state.handles.read().await;

    if let Some((_torrent_id, handle)) = handles.get(&game_id) {
        let handle = handle.clone();
        let session = state.session.clone();
        drop(handles); // Release lock before async operation

        session
            .unpause(&handle)
            .await
            .map_err(|e| format!("Failed to resume: {}", e))?;
        info!(
            "[Shard_Torrent_Backend] Resumed download for game id {}",
            game_id
        );
        Ok(())
    } else {
        Err(format!("No active download found for game id {}", game_id))
    }
}

#[tauri::command]
async fn uninstall_game(
    invoke_message: GameMeta,
    state: State<'_, TorrentState>,
) -> Result<(), String> {
    let game_id = invoke_message.id;

    // First, try to remove the torrent if it's active
    if let Some((torrent_id, handle)) = {
        let handles = state.handles.read().await;
        handles.get(&game_id).cloned()
    } {
        // Pause torrent (safe)
        state
            .session
            .pause(&handle)
            .await
            .map_err(|e| format!("Failed to pause torrent: {}", e))?;

        // Remove from session
        state
            .session
            .delete(librqbit::api::TorrentIdOrHash::Id(torrent_id), true)
            .await
            .map_err(|e| format!("Failed to remove torrent: {}", e))?;

        // Remove from app state
        let mut handles = state.handles.write().await;
        handles.remove(&game_id);

        info!(
            "[Shard_Torrent_Backend] Torrent {} fully removed from session",
            game_id
        );
    } else {
        info!(
            "[Shard_Torrent_Backend] No active torrent found for game {}",
            game_id
        );
    }

    // Now, delete game files
    let game_dir = Path::new(&get_game_path()).join(game_id.to_string());
    if game_dir.exists() {
        fs::remove_dir_all(&game_dir).map_err(|e| format!("Failed to delete game files: {}", e))?;
        info!(
            "[Shard_Torrent_Backend] Game files deleted for game {}",
            game_id
        );
    } else {
        info!(
            "[Shard_Torrent_Backend] No game files found for game {}",
            game_id
        );
    }

    Ok(())
}

#[tauri::command]
fn extract_and_clean(invoke_message: GameMeta) -> Result<(), String> {
    let torrent_path = Path::new(&get_game_path())
        .join(invoke_message.id.to_string())
        .join("game.torrent");

    if torrent_path.exists() {
        info!(
            "[Shard_Torrent_Backend] Removing torrent file at {:?}",
            torrent_path
        );
        fs::remove_file(&torrent_path)
            .map_err(|e| format!("Failed to remove torrent file: {}", e))?;
    } else {
        warn!("[Shard_Torrent_Backend] Torrent file does not exist, nothing to remove.");
    }

    Ok(())
}

#[tauri::command]
async fn get_active_downloads(
    state: State<'_, TorrentState>,
) -> Result<Vec<serde_json::Value>, String> {
    let handles = state.handles.read().await;
    let mut active_downloads = Vec::new();

    for (game_id, (_torrent_id, handle)) in handles.iter() {
        let stats = handle.stats();
        let progress_percent =
            (stats.progress_bytes as f64 / stats.total_bytes.max(1) as f64) * 100.0;

        active_downloads.push(serde_json::json!({
            "gameId": *game_id,
            "state": format!("{:?}", stats.state),
            "progress": progress_percent,
            "downloadedBytes": stats.progress_bytes,
            "totalBytes": stats.total_bytes,
        }));
    }

    Ok(active_downloads)
}

// ------------------ SYSTEM INFO ------------------
fn contains_game_file(dir: &Path) -> bool {
    let exts = ["nsp", "nsz", "nsc"];

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                if contains_game_file(&path) {
                    return true;
                }
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if exts.iter().any(|allowed| allowed.eq_ignore_ascii_case(ext)) {
                    return true;
                }
            }
        }
    }

    false
}

#[tauri::command]
fn is_game_downloaded(invoke_message: GameMeta) -> Result<bool, String> {
    let game_dir = Path::new(&get_game_path()).join(invoke_message.id.to_string());

    if !game_dir.exists() {
        return Ok(false);
    }

    Ok(contains_game_file(&game_dir))
}

// ------------------ RUN ------------------
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_app_path().join("app.log"))
        .expect("Failed to open log file");

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(EnvFilter::new("info")),
        )
        .with(
            fmt::layer()
                .with_writer(std::sync::Arc::new(log_file))
                .with_filter(EnvFilter::new("warn")),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_stronghold::Builder::new(|_pass| todo!()).build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = tauri::async_runtime::block_on(async {
                TorrentState::new(app.handle().clone())
                    .await
                    .expect("Failed to initialize torrent state")
            });
            app.manage(state);

            // Initialize FTP Monitor state
            app.manage(Arc::new(Mutex::new(None::<ftp_discovery::FTPMonitor>)));

            // Initialize FTP Manager state
            let ftp_manager = ftp_manager::FTPManager::new(app.handle().clone());
            app.manage(Arc::new(Mutex::new(Some(ftp_manager))));

            // Start all installed plugins
            if let Err(e) = plugin_manager::PluginManager::start_plugins() {
                tracing::error!("Failed to start plugins: {}", e);
            } else {
                tracing::info!("Plugins started successfully");
            }

            if let Some(window) = app.get_webview_window("main") {
                // Listen for when the window is created and show it
                let window_clone = window.clone();
                window.once("tauri://created", move |_| {
                    let _ = window_clone.show();
                });

                // Also add a fallback to show after a short delay
                let window_clone2 = window.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let _ = window_clone2.show();
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // FTP Discovery commands
            ftp_discovery::start_ftp_monitor,
            ftp_discovery::stop_ftp_monitor,
            ftp_discovery::is_ftp_monitor_running,
            // FTP Manager commands
            ftp_manager::set_ftp_ip,
            ftp_manager::get_ftp_ip,
            ftp_manager::scan_game_files,
            ftp_manager::queue_file,
            ftp_manager::get_transfer_queue,
            ftp_manager::clear_transfer_queue,
            ftp_manager::remove_from_transfer_queue,
            ftp_manager::get_current_transfer,
            ftp_manager::is_ftp_transferring,
            // Plugin commands
            plugin_manager::get_available_plugins,
            plugin_manager::get_installed_plugins, // <- ADDED THIS
            plugin_manager::install_plugin,
            plugin_manager::remove_plugin,
            plugin_manager::restart_plugin,
            plugin_manager::restart_plugins,
            // Torrent commands
            check_file_system,
            get_game_meta,
            create_game_dir,
            obtain_torrent_file,
            download_game,
            pause_game,
            resume_game,
            uninstall_game,
            extract_and_clean,
            get_active_downloads,
            is_game_downloaded
        ])
        .on_page_load(|webview, _payload| {
            // Optional: Notify frontend that plugins are ready
            let _ = webview.emit("plugins-ready", ());
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            // Graceful shutdown: stop all plugins when app closes
            if let tauri::RunEvent::Exit = event {
                tracing::info!("Application exiting, stopping plugins...");
                if let Err(e) = plugin_manager::stop_all_plugins() {
                    tracing::error!("Failed to stop plugins: {}", e);
                }
            }
        });
}
