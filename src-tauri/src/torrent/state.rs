use anyhow::Context;
use librqbit::{AddTorrent, AddTorrentOptions, AddTorrentResponse, ManagedTorrent, Session};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

use crate::configs::defaults::{get_config_path, get_game_path};

#[derive(Serialize, Deserialize)]
struct SavedTorrent {
    game_id: u32,
    torrent_path: String,
}

pub struct TorrentState {
    pub session: Arc<Session>,
    pub handles: Arc<RwLock<HashMap<u32, (usize, Arc<ManagedTorrent>)>>>,
}

impl TorrentState {
    pub async fn new(app_handle: AppHandle) -> Result<Self, anyhow::Error> {
        let base_path = PathBuf::from(&get_config_path());
        let state_file = base_path.join(".torrent_state.json");

        info!("[Shard_Torrent_Backend] Initializing torrent session...");

        let session = Session::new(base_path)
            .await
            .context("error creating shared session")?;

        let handles = Arc::new(RwLock::new(HashMap::new()));

        // Restore saved torrents
        if state_file.exists() {
            info!("[Shard_Torrent_Backend] Found saved torrent state, restoring...");

            if let Ok(data) = tokio::fs::read_to_string(&state_file).await {
                if let Ok(saved_torrents) = serde_json::from_str::<Vec<SavedTorrent>>(&data) {
                    info!(
                        "[Shard_Torrent_Backend] Restoring {} torrent(s)",
                        saved_torrents.len()
                    );

                    let mut handles_guard = handles.write().await;
                    for saved in saved_torrents {
                        let torrent_path = PathBuf::from(&saved.torrent_path);
                        if torrent_path.exists() {
                            if let Ok(add_torrent) =
                                AddTorrent::from_local_filename(&saved.torrent_path)
                            {
                                let game_dir =
                                    PathBuf::from(&get_game_path()).join(saved.game_id.to_string());

                                match session
                                    .add_torrent(
                                        add_torrent,
                                        Some(AddTorrentOptions {
                                            overwrite: true,
                                            output_folder: Some(
                                                game_dir.to_string_lossy().to_string(),
                                            ),
                                            ..Default::default()
                                        }),
                                    )
                                    .await
                                {
                                    Ok(response) => {
                                        match response {
                                            AddTorrentResponse::Added(id, handle)
                                            | AddTorrentResponse::AlreadyManaged(id, handle) => {
                                                info!(
                                                    "[Shard_Torrent_Backend] Restored torrent for game {} with id: {}",
                                                    saved.game_id, id
                                                );

                                                // Get initial stats to send to UI
                                                let stats = handle.stats();
                                                let progress_percent = (stats.progress_bytes
                                                    as f64
                                                    / stats.total_bytes.max(1) as f64)
                                                    * 100.0;

                                                // Emit download-restored event to notify UI
                                                let _ = app_handle.emit(
                                                    "download-restored",
                                                    serde_json::json!({
                                                        "gameId": saved.game_id,
                                                        "state": format!("{:?}", stats.state),
                                                        "progress": progress_percent,
                                                        "downloadedBytes": stats.progress_bytes,
                                                        "totalBytes": stats.total_bytes,
                                                    }),
                                                );

                                                // UNPAUSE the torrent to resume downloading
                                                if let Err(e) = session.unpause(&handle).await {
                                                    warn!(
                                                        "[Shard_Torrent_Backend] Failed to unpause game {}: {}",
                                                        saved.game_id, e
                                                    );
                                                } else {
                                                    info!(
                                                        "[Shard_Torrent_Backend] Resumed download for game {}",
                                                        saved.game_id
                                                    );

                                                    // Start progress monitoring for restored torrent
                                                    Self::spawn_progress_monitor(
                                                        saved.game_id,
                                                        handle.clone(),
                                                        app_handle.clone(),
                                                    );
                                                }
                                                handles_guard.insert(saved.game_id, (id, handle));
                                            }
                                            _ => {}
                                        }
                                    }
                                    Err(e) => {
                                        error!(
                                            "[Shard_Torrent_Backend] Failed to restore torrent for game {}: {}",
                                            saved.game_id, e
                                        );
                                    }
                                }
                            }
                        } else {
                            warn!(
                                "[Shard_Torrent_Backend] Torrent file not found for game {}, skipping",
                                saved.game_id
                            );
                        }
                    }
                } else {
                    warn!("[Shard_Torrent_Backend] Failed to parse saved torrent state");
                }
            } else {
                warn!("[Shard_Torrent_Backend] Failed to read saved torrent state file");
            }
        } else {
            info!("[Shard_Torrent_Backend] No saved torrent state found");
        }

        let state = Self { session, handles };

        // Start auto-save background task
        info!("[Shard_Torrent_Backend] Starting auto-save background task (interval: 5s)");
        state.start_auto_save(5);

        Ok(state)
    }

    fn spawn_progress_monitor(game_id: u32, handle: Arc<ManagedTorrent>, app_handle: AppHandle) {
        tokio::spawn({
            let start_time = std::time::Instant::now();
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
                        info!(
                            "[Shard_Torrent_Backend] Game {} download completed!",
                            game_id
                        );
                        let _ = app_handle.emit(
                            "download-complete",
                            serde_json::json!({
                                "gameId": game_id,
                            }),
                        );
                        break;
                    }
                }
            }
        });
    }

    fn start_auto_save(&self, interval_secs: u16) {
        let handles = Arc::clone(&self.handles);
        let state_file = PathBuf::from(&get_config_path()).join(".torrent_state.json");

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval_secs as u64));
            loop {
                interval.tick().await;

                let handles_guard = handles.read().await;
                let mut saved_torrents = Vec::new();

                for (game_id, (_, _handle)) in handles_guard.iter() {
                    // Save the torrent file path
                    let torrent_path = PathBuf::from(&get_game_path())
                        .join(game_id.to_string())
                        .join("game.torrent");

                    if torrent_path.exists() {
                        saved_torrents.push(SavedTorrent {
                            game_id: *game_id,
                            torrent_path: torrent_path.to_string_lossy().to_string(),
                        });
                    }
                }

                if !saved_torrents.is_empty() {
                    match serde_json::to_string(&saved_torrents) {
                        Ok(json) => match tokio::fs::write(&state_file, json).await {
                            Ok(_) => {
                                info!(
                                    "[Shard_Torrent_Backend] Auto-saved state for {} torrent(s)",
                                    saved_torrents.len()
                                );
                            }
                            Err(e) => {
                                error!(
                                    "[Shard_Torrent_Backend] Failed to write torrent state: {}",
                                    e
                                );
                            }
                        },
                        Err(e) => {
                            error!(
                                "[Shard_Torrent_Backend] Failed to serialize torrent state: {}",
                                e
                            );
                        }
                    }
                }
            }
        });
    }
}
