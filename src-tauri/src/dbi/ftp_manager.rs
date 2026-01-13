use crate::configs::defaults::get_game_path;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

const DBI_FTP_PORT: u16 = 5000;
const FTP_TIMEOUT_MS: u64 = 5000;
const TRANSFER_THROUGHPUT_BUFFER: u64 = 2048;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameFile {
    pub game_id: u32,
    pub game_title: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferProgress {
    pub game_id: u32,
    pub file_name: String,
    pub bytes_transferred: u64,
    pub total_bytes: u64,
    pub progress_percent: f64,
    pub transfer_speed: f64, // bytes per second
    pub eta_seconds: u64,
    pub status: TransferStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransferStatus {
    Queued,
    Transferring,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug)]
struct TransferTask {
    game_file: GameFile,
    status: TransferStatus,
}

pub struct FTPManager {
    ftp_ip: Arc<Mutex<Option<String>>>,
    transfer_queue: Arc<Mutex<VecDeque<TransferTask>>>,
    is_transferring: Arc<AtomicBool>,
    current_transfer: Arc<Mutex<Option<TransferProgress>>>,
    app_handle: AppHandle,
    bytes_transferred: Arc<AtomicU64>,
    total_bytes: Arc<AtomicU64>,
}

impl FTPManager {
    pub fn new(app_handle: AppHandle) -> Self {
        info!("FTPManager instance created");
        Self {
            ftp_ip: Arc::new(Mutex::new(None)),
            transfer_queue: Arc::new(Mutex::new(VecDeque::new())),
            is_transferring: Arc::new(AtomicBool::new(false)),
            current_transfer: Arc::new(Mutex::new(None)),
            app_handle,
            bytes_transferred: Arc::new(AtomicU64::new(0)),
            total_bytes: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn set_ftp_ip(&self, ip: String) -> Result<(), String> {
        info!("Setting FTP IP to: {}", ip);

        // Test connection before setting
        let test_address = format!("{}:{}", ip, DBI_FTP_PORT);
        match TcpStream::connect_timeout(
            &test_address
                .parse()
                .map_err(|e| format!("Invalid IP address: {}", e))?,
            Duration::from_millis(FTP_TIMEOUT_MS),
        ) {
            Ok(_) => {
                let mut ftp_ip_guard = self.ftp_ip.lock().unwrap();
                *ftp_ip_guard = Some(ip.clone());
                info!("FTP IP set successfully: {}", ip);
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to FTP at {}: {}", ip, e);
                Err(format!("Cannot connect to FTP server at {}: {}", ip, e))
            }
        }
    }

    pub fn get_ftp_ip(&self) -> Option<String> {
        self.ftp_ip.lock().unwrap().clone()
    }

    pub fn scan_game_files(&self, game_path: &PathBuf) -> Result<Vec<GameFile>, String> {
        info!("Scanning for game files in: {}", game_path.display());
        let mut game_files = Vec::new();
        let base_path = game_path.as_path(); // Convert PathBuf to Path reference

        if !base_path.exists() {
            return Err(format!("Game path does not exist: {}", game_path.display()));
        }

        // Iterate through game directories
        let entries =
            fs::read_dir(base_path).map_err(|e| format!("Failed to read game path: {}", e))?;

        for entry in entries.flatten() {
            let game_dir = entry.path();
            if !game_dir.is_dir() {
                continue;
            }

            // Try to parse game ID from directory name
            if let Some(dir_name) = game_dir.file_name().and_then(|n| n.to_str()) {
                if let Ok(game_id) = dir_name.parse::<u32>() {
                    // Scan for game files in this directory
                    self.scan_directory(&game_dir, game_id, &mut game_files)?;
                }
            }
        }

        info!("Found {} game files", game_files.len());
        Ok(game_files)
    }

    fn scan_directory(
        &self,
        dir: &Path,
        game_id: u32,
        game_files: &mut Vec<GameFile>,
    ) -> Result<(), String> {
        let valid_extensions = ["nsp", "nsz", "nsc", "xci", "xcz"];

        let entries =
            fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if valid_extensions
                        .iter()
                        .any(|&valid_ext| valid_ext.eq_ignore_ascii_case(ext))
                    {
                        let metadata = fs::metadata(&path)
                            .map_err(|e| format!("Failed to get file metadata: {}", e))?;

                        let file_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        // Extract game title from directory or file name
                        let game_title = path
                            .parent()
                            .and_then(|p| p.file_name())
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown Game")
                            .to_string();

                        game_files.push(GameFile {
                            game_id,
                            game_title,
                            file_path: path.to_string_lossy().to_string(),
                            file_name,
                            file_size: metadata.len(),
                        });
                    }
                }
            } else if path.is_dir() {
                // Recursively scan subdirectories
                self.scan_directory(&path, game_id, game_files)?;
            }
        }

        Ok(())
    }

    pub fn queue_file(&self, game_file: GameFile) -> Result<(), String> {
        info!(
            "Queueing file: {} ({})",
            game_file.file_name, game_file.game_id
        );

        let mut queue = self.transfer_queue.lock().unwrap();

        // Check if file is already queued
        if queue
            .iter()
            .any(|task| task.game_file.file_path == game_file.file_path)
        {
            return Err(format!(
                "File {} is already in the queue",
                game_file.file_name
            ));
        }

        queue.push_back(TransferTask {
            game_file: game_file.clone(),
            status: TransferStatus::Queued,
        });

        info!("File queued successfully. Queue length: {}", queue.len());

        // Emit queue update event
        let _ = self.app_handle.emit(
            "ftp-queue-updated",
            QueueUpdatePayload {
                queue_length: queue.len(),
            },
        );

        // Start transfer if not already running
        if !self.is_transferring.load(Ordering::SeqCst) {
            drop(queue); // Release lock before starting transfer
            self.start_transfer_worker();
        }

        Ok(())
    }

    pub fn get_queue(&self) -> Vec<GameFile> {
        let queue = self.transfer_queue.lock().unwrap();
        queue.iter().map(|task| task.game_file.clone()).collect()
    }

    pub fn clear_queue(&self) -> Result<(), String> {
        if self.is_transferring.load(Ordering::SeqCst) {
            return Err("Cannot clear queue while transfer is in progress".to_string());
        }

        let mut queue = self.transfer_queue.lock().unwrap();
        queue.clear();
        info!("Transfer queue cleared");

        let _ = self
            .app_handle
            .emit("ftp-queue-updated", QueueUpdatePayload { queue_length: 0 });

        Ok(())
    }

    pub fn remove_from_queue(&self, file_path: &str) -> Result<(), String> {
        let mut queue = self.transfer_queue.lock().unwrap();

        // Check if the file being removed is currently transferring
        if let Some(current) = self.current_transfer.lock().unwrap().as_ref() {
            if queue.front().map(|t| &t.game_file.file_path) == Some(&file_path.to_string()) {
                return Err("Cannot remove file that is currently transferring".to_string());
            }
        }

        let original_len = queue.len();
        queue.retain(|task| task.game_file.file_path != file_path);

        if queue.len() < original_len {
            info!("Removed file from queue: {}", file_path);
            let _ = self.app_handle.emit(
                "ftp-queue-updated",
                QueueUpdatePayload {
                    queue_length: queue.len(),
                },
            );
            Ok(())
        } else {
            Err(format!("File not found in queue: {}", file_path))
        }
    }

    fn start_transfer_worker(&self) {
        if self.is_transferring.load(Ordering::SeqCst) {
            warn!("Transfer worker already running");
            return;
        }

        info!("Starting transfer worker");
        self.is_transferring.store(true, Ordering::SeqCst);

        let is_transferring = Arc::clone(&self.is_transferring);
        let transfer_queue = Arc::clone(&self.transfer_queue);
        let current_transfer = Arc::clone(&self.current_transfer);
        let ftp_ip = Arc::clone(&self.ftp_ip);
        let app_handle = self.app_handle.clone();
        let bytes_transferred = Arc::clone(&self.bytes_transferred);
        let total_bytes = Arc::clone(&self.total_bytes);

        thread::spawn(move || {
            info!("Transfer worker thread started");

            while is_transferring.load(Ordering::SeqCst) {
                // Get next task from queue
                let task = {
                    let mut queue = transfer_queue.lock().unwrap();
                    queue.pop_front()
                };

                match task {
                    Some(mut task) => {
                        info!("Processing transfer for: {}", task.game_file.file_name);

                        // Get FTP IP
                        let ftp_ip_str = {
                            let ip_guard = ftp_ip.lock().unwrap();
                            match ip_guard.as_ref() {
                                Some(ip) => ip.clone(),
                                None => {
                                    error!("No FTP IP set");
                                    task.status = TransferStatus::Failed;
                                    let _ = app_handle.emit(
                                        "ftp-transfer-error",
                                        TransferErrorPayload {
                                            game_id: task.game_file.game_id,
                                            file_name: task.game_file.file_name.clone(),
                                            error: "No FTP IP configured".to_string(),
                                        },
                                    );
                                    continue;
                                }
                            }
                        };

                        // Initialize progress
                        bytes_transferred.store(0, Ordering::SeqCst);
                        total_bytes.store(task.game_file.file_size, Ordering::SeqCst);

                        let progress = TransferProgress {
                            game_id: task.game_file.game_id,
                            file_name: task.game_file.file_name.clone(),
                            bytes_transferred: 0,
                            total_bytes: task.game_file.file_size,
                            progress_percent: 0.0,
                            transfer_speed: 0.0,
                            eta_seconds: 0,
                            status: TransferStatus::Transferring,
                        };

                        {
                            let mut current = current_transfer.lock().unwrap();
                            *current = Some(progress.clone());
                        }

                        let _ = app_handle.emit("ftp-transfer-progress", &progress);

                        // Perform transfer
                        let result = Self::transfer_file(
                            &task.game_file,
                            &ftp_ip_str,
                            &app_handle,
                            &bytes_transferred,
                            &total_bytes,
                        );

                        match result {
                            Ok(_) => {
                                info!(
                                    "Transfer completed successfully: {}",
                                    task.game_file.file_name
                                );
                                task.status = TransferStatus::Completed;

                                let final_progress = TransferProgress {
                                    game_id: task.game_file.game_id,
                                    file_name: task.game_file.file_name.clone(),
                                    bytes_transferred: task.game_file.file_size,
                                    total_bytes: task.game_file.file_size,
                                    progress_percent: 100.0,
                                    transfer_speed: 0.0,
                                    eta_seconds: 0,
                                    status: TransferStatus::Completed,
                                };

                                let _ = app_handle.emit("ftp-transfer-progress", &final_progress);
                                let _ = app_handle.emit(
                                    "ftp-transfer-complete",
                                    TransferCompletePayload {
                                        game_id: task.game_file.game_id,
                                        file_name: task.game_file.file_name.clone(),
                                    },
                                );
                            }
                            Err(e) => {
                                error!("Transfer failed: {} - {}", task.game_file.file_name, e);
                                task.status = TransferStatus::Failed;

                                let _ = app_handle.emit(
                                    "ftp-transfer-error",
                                    TransferErrorPayload {
                                        game_id: task.game_file.game_id,
                                        file_name: task.game_file.file_name.clone(),
                                        error: e,
                                    },
                                );
                            }
                        }

                        // Clear current transfer
                        {
                            let mut current = current_transfer.lock().unwrap();
                            *current = None;
                        }

                        // Emit queue update
                        let queue_len = transfer_queue.lock().unwrap().len();
                        let _ = app_handle.emit(
                            "ftp-queue-updated",
                            QueueUpdatePayload {
                                queue_length: queue_len,
                            },
                        );
                    }
                    None => {
                        // Queue is empty, stop worker
                        info!("Transfer queue empty, stopping worker");
                        is_transferring.store(false, Ordering::SeqCst);
                        break;
                    }
                }
            }

            info!("Transfer worker thread stopped");
        });
    }

    fn transfer_file(
        game_file: &GameFile,
        ftp_ip: &str,
        app_handle: &AppHandle,
        bytes_transferred: &Arc<AtomicU64>,
        total_bytes: &Arc<AtomicU64>,
    ) -> Result<(), String> {
        use std::fs::File;
        use std::io::Cursor;
        use std::io::Read;
        use std::time::Instant;
        use suppaftp::native_tls::TlsConnector;
        use suppaftp::types::FileType;
        use suppaftp::FtpStream;

        info!(
            "Starting FTP transfer: {} -> {}",
            game_file.file_name, ftp_ip
        );

        // Connect to FTP server
        let mut ftp = FtpStream::connect(format!("{}:5000", ftp_ip))
            .map_err(|e| format!("Failed to connect to FTP server: {}", e))?;

        // Login anonymously; replace with username/password if needed
        ftp.login("anonymous", "anonymous")
            .map_err(|e| format!("FTP login failed: {}", e))?;

        ftp.transfer_type(FileType::Binary)
            .map_err(|e| format!("Failed to set binary mode: {}", e))?;

        // Open local file
        let mut file = File::open(&game_file.file_path)
            .map_err(|e| format!("Failed to open local file: {}", e))?;

        let mut buffer = vec![0u8; (TRANSFER_THROUGHPUT_BUFFER * 1024).try_into().unwrap()];
        let mut _total_sent = 0u64;
        let start_time = Instant::now();
        let mut last_update = Instant::now();

        // Create a memory cursor for the file upload
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        let mut cursor = Cursor::new(file_contents);

        // Start upload using SuppaFTP's put_file
        ftp.put_file(&game_file.file_name, &mut cursor)
            .map_err(|e| format!("FTP upload failed: {}", e))?;

        // Since SuppaFTP's put_file uploads all at once, we manually emit "completed" progress
        bytes_transferred.store(game_file.file_size, Ordering::SeqCst);

        let progress = TransferProgress {
            game_id: game_file.game_id,
            file_name: game_file.file_name.clone(),
            bytes_transferred: game_file.file_size,
            total_bytes: game_file.file_size,
            progress_percent: 100.0,
            transfer_speed: (game_file.file_size as f64) / start_time.elapsed().as_secs_f64(),
            eta_seconds: 0,
            status: TransferStatus::Completed,
        };

        let _ = app_handle.emit("ftp-transfer-progress", &progress);

        ftp.quit().ok(); // ignore quit errors

        info!(
            "FTP transfer completed: {} ({} bytes in {:.2}s)",
            game_file.file_name,
            game_file.file_size,
            start_time.elapsed().as_secs_f64()
        );

        Ok(())
    }

    pub fn get_current_transfer(&self) -> Option<TransferProgress> {
        self.current_transfer.lock().unwrap().clone()
    }

    pub fn is_transferring(&self) -> bool {
        self.is_transferring.load(Ordering::SeqCst)
    }
}

// Event payloads
#[derive(Clone, Serialize)]
struct QueueUpdatePayload {
    queue_length: usize,
}

#[derive(Clone, Serialize)]
struct TransferCompletePayload {
    game_id: u32,
    file_name: String,
}

#[derive(Clone, Serialize)]
struct TransferErrorPayload {
    game_id: u32,
    file_name: String,
    error: String,
}

// Tauri commands
#[tauri::command]
pub fn set_ftp_ip(
    ip: String,
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<String, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        manager.set_ftp_ip(ip.clone())?;
        Ok(format!("FTP IP set to {}", ip))
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn get_ftp_ip(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<Option<String>, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        Ok(manager.get_ftp_ip())
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn scan_game_files(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<Vec<GameFile>, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        manager.scan_game_files(&get_game_path())
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn queue_file(
    game_file: GameFile,
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<String, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        manager.queue_file(game_file.clone())?;
        Ok(format!("File {} queued for transfer", game_file.file_name))
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn get_transfer_queue(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<Vec<GameFile>, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        Ok(manager.get_queue())
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn clear_transfer_queue(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<String, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        manager.clear_queue()?;
        Ok("Transfer queue cleared".to_string())
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn remove_from_transfer_queue(
    file_path: String,
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<String, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        manager.remove_from_queue(&file_path)?;
        Ok(format!("File removed from queue: {}", file_path))
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn get_current_transfer(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> Result<Option<TransferProgress>, String> {
    let manager_guard = state.lock();

    if let Some(manager) = manager_guard.as_ref() {
        Ok(manager.get_current_transfer())
    } else {
        Err("FTP Manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn is_ftp_transferring(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPManager>>>>,
) -> bool {
    let manager_guard = state.lock();
    manager_guard
        .as_ref()
        .map_or(false, |m| m.is_transferring())
}
