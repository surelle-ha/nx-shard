use log::{debug, error, info, warn};
use serde::Serialize;
use std::collections::HashSet;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

const DBI_FTP_PORT: u16 = 5000;
const TIMEOUT_MS: u64 = 1000;
const THREAD_POOL_SIZE: usize = 100;
const RETRY_INTERVAL: u16 = 10;

#[derive(Debug, Clone)]
pub struct FoundService {
    pub ip_address: String,
}

#[derive(Debug)]
pub struct FoundServices {
    pub services: Vec<FoundService>,
}

impl FoundServices {
    fn new() -> Self {
        FoundServices {
            services: Vec::new(),
        }
    }

    fn add(&mut self, service: FoundService) {
        self.services.push(service);
    }
}

pub struct FTPDiscovery;

impl FTPDiscovery {
    pub fn new() -> Self {
        info!("FTPDiscovery instance created");
        FTPDiscovery
    }

    pub fn scan(&self, ip_address: &str) -> Option<FoundService> {
        let target_address = format!("{}:{}", ip_address, DBI_FTP_PORT);
        info!("Scanning FTP service at {}", target_address);

        match TcpStream::connect_timeout(
            &target_address.parse().ok()?,
            Duration::from_millis(TIMEOUT_MS),
        ) {
            Ok(_) => {
                info!("✓ Found FTP service at {}", ip_address);
                Some(FoundService {
                    ip_address: ip_address.to_string(),
                })
            }
            Err(e) => {
                debug!("No FTP service at {}: {}", ip_address, e);
                None
            }
        }
    }

    pub fn scan_all(&self) -> FoundServices {
        info!("Scanning all FTP services on the network...");

        // Get local IP to determine network range
        if let Some(local_ip) = self.get_local_ip() {
            info!("Detected local IP: {}", local_ip);
            let base_ip = self.get_base_ip(&local_ip);
            info!("Scanning network range: {}.1 to {}.254", base_ip, base_ip);
            self.scan_range(&format!("{}.1", base_ip), &format!("{}.254", base_ip))
        } else {
            error!("Could not determine local IP address");
            FoundServices::new()
        }
    }

    pub fn scan_range(&self, start_ip: &str, end_ip: &str) -> FoundServices {
        info!("Scanning FTP services from {} to {}...", start_ip, end_ip);

        let found_services = Arc::new(Mutex::new(FoundServices::new()));

        let ips = self.generate_ip_range(start_ip, end_ip);

        info!("Generated {} IP addresses to scan", ips.len());

        let chunks: Vec<Vec<String>> = ips
            .chunks(THREAD_POOL_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect();

        debug!(
            "Split into {} chunks of up to {} IPs each",
            chunks.len(),
            THREAD_POOL_SIZE
        );

        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            debug!("Processing chunk {} of {}", chunk_idx + 1, chunks.len());
            let mut handles = vec![];

            for ip in chunk {
                let found_services = Arc::clone(&found_services);
                let ip_clone = ip.clone();

                let handle = thread::spawn(move || {
                    let target_address = format!("{}:{}", ip_clone, DBI_FTP_PORT);

                    if let Ok(addr) = target_address.parse() {
                        match TcpStream::connect_timeout(&addr, Duration::from_millis(TIMEOUT_MS)) {
                            Ok(_) => {
                                // Service found - add to collection
                                let mut services = found_services.lock().unwrap();
                                services.add(FoundService {
                                    ip_address: ip_clone.clone(),
                                });
                                info!("✓ Found FTP service at {}", ip_clone);
                            }
                            Err(e) => {
                                debug!("No FTP service at {}: {}", ip_clone, e);
                            }
                        }
                    } else {
                        warn!("Invalid address format: {}", target_address);
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                if let Err(e) = handle.join() {
                    error!("Thread panicked: {:?}", e);
                }
            }
        }

        let services = found_services.lock().unwrap();
        info!(
            "Scan complete. Found {} FTP services.",
            services.services.len()
        );

        // Return owned data
        FoundServices {
            services: services.services.clone(),
        }
    }

    fn get_local_ip(&self) -> Option<String> {
        use std::net::UdpSocket;

        debug!("Attempting to determine local IP address");

        // Connect to an external address to determine which local IP is used
        let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
        socket.connect("8.8.8.8:80").ok()?;
        let local_ip = socket.local_addr().ok().map(|addr| addr.ip().to_string());

        if let Some(ref ip) = local_ip {
            debug!("Local IP determined: {}", ip);
        } else {
            warn!("Failed to determine local IP");
        }

        local_ip
    }

    fn get_base_ip(&self, ip: &str) -> String {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() >= 3 {
            let base = format!("{}.{}.{}", parts[0], parts[1], parts[2]);
            debug!("Base IP extracted: {}", base);
            base
        } else {
            warn!("Invalid IP format, using default: 192.168.1");
            "192.168.1".to_string()
        }
    }

    fn generate_ip_range(&self, start_ip: &str, end_ip: &str) -> Vec<String> {
        let start_parts: Vec<u8> = start_ip.split('.').filter_map(|s| s.parse().ok()).collect();
        let end_parts: Vec<u8> = end_ip.split('.').filter_map(|s| s.parse().ok()).collect();

        if start_parts.len() != 4 || end_parts.len() != 4 {
            error!("Invalid IP range format: {} to {}", start_ip, end_ip);
            return vec![];
        }

        let mut ips = Vec::new();
        let base = format!("{}.{}.{}", start_parts[0], start_parts[1], start_parts[2]);

        // Generate IPs by varying the last octet
        for i in start_parts[3]..=end_parts[3] {
            ips.push(format!("{}.{}", base, i));
        }

        debug!("Generated {} IP addresses in range", ips.len());
        ips
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FTPStatusPayload {
    is_active: bool,
    active_count: usize,
    active_ips: Vec<String>,
}

pub struct FTPMonitor {
    is_running: Arc<AtomicBool>,
    app_handle: AppHandle,
}

impl FTPMonitor {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            app_handle,
        }
    }

    pub fn start(&self, scan_interval_seconds: u64) {
        if self.is_running.load(Ordering::SeqCst) {
            warn!("FTP Monitor already running");
            return;
        }

        self.is_running.store(true, Ordering::SeqCst);

        let is_running = Arc::clone(&self.is_running);
        let app_handle = self.app_handle.clone();

        thread::spawn(move || {
            let discovery = FTPDiscovery::new();
            let mut previous_services: HashSet<String> = HashSet::new();
            let mut iteration = 0;

            while is_running.load(Ordering::SeqCst) {
                iteration += 1;

                info!(
                    "[{}] Scan #{} - {}",
                    chrono::Local::now().format("%H:%M:%S"),
                    iteration,
                    chrono::Local::now().format("%Y-%m-%d")
                );

                // Perform scan
                let found_services = discovery.scan_all();
                let current_services: HashSet<String> = found_services
                    .services
                    .iter()
                    .map(|s| s.ip_address.clone())
                    .collect();

                // Detect changes
                let new_services: Vec<_> =
                    current_services.difference(&previous_services).collect();

                let removed_services: Vec<_> =
                    previous_services.difference(&current_services).collect();

                // Report changes
                if !new_services.is_empty() {
                    info!("  🟢 NEW services detected:");
                    for ip in &new_services {
                        info!("     + {}", ip);
                    }
                }

                if !removed_services.is_empty() {
                    error!("  🔴 Services OFFLINE:");
                    for ip in &removed_services {
                        error!("     - {}", ip);
                    }
                }

                if new_services.is_empty() && removed_services.is_empty() {
                    info!("No changes detected");
                }

                info!("Total active services: {}", current_services.len());

                // Emit event to frontend
                let is_active = !current_services.is_empty();
                let active_ips: Vec<String> = current_services.iter().cloned().collect();

                let payload = FTPStatusPayload {
                    is_active,
                    active_count: current_services.len(),
                    active_ips: active_ips.clone(),
                };

                if let Err(e) = app_handle.emit("ftp-status-changed", &payload) {
                    error!("Failed to emit FTP status event: {}", e);
                }

                if !current_services.is_empty() {
                    info!("Active IPs: {}", active_ips.join(", "));
                }

                // Update previous state
                previous_services = current_services;

                // Wait before next scan
                info!("Waiting {} seconds until next scan.", scan_interval_seconds);
                thread::sleep(Duration::from_secs(scan_interval_seconds));
            }

            error!("Monitor stopped. Total scans performed: {}", iteration);
        });
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        info!("FTP Monitor stop requested");
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

#[tauri::command]
pub fn start_ftp_monitor(
    app_handle: AppHandle,
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPMonitor>>>>,
    scan_interval: Option<u64>,
) -> Result<String, String> {
    let mut monitor_guard = state.lock();

    if monitor_guard.is_none() {
        *monitor_guard = Some(FTPMonitor::new(app_handle));
    }

    if let Some(monitor) = monitor_guard.as_ref() {
        let interval = scan_interval.unwrap_or(RETRY_INTERVAL.into());
        monitor.start(interval);
        Ok(format!(
            "FTP Monitor started with {} second interval",
            interval
        ))
    } else {
        Err("Failed to initialize monitor".to_string())
    }
}

#[tauri::command]
pub fn stop_ftp_monitor(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPMonitor>>>>,
) -> Result<String, String> {
    let monitor_guard = state.lock();

    if let Some(monitor) = monitor_guard.as_ref() {
        monitor.stop();
        Ok("FTP Monitor stopped".to_string())
    } else {
        Err("Monitor not initialized".to_string())
    }
}

#[tauri::command]
pub fn is_ftp_monitor_running(
    state: tauri::State<Arc<parking_lot::Mutex<Option<FTPMonitor>>>>,
) -> bool {
    let monitor_guard = state.lock();
    monitor_guard.as_ref().map_or(false, |m| m.is_running())
}
