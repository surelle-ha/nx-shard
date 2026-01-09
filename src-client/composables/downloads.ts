// composables/useDownloads.ts
import { ref, computed } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from "@tauri-apps/api/core";

interface DownloadProgress {
  progress: number;
  downloadedBytes: number;
  totalBytes: number;
  downloadSpeed: number;
  uploadSpeed: number;
  peers: number;
  state: string;
}

interface DownloadState {
  isRunning: boolean;
  isPaused: boolean;
  currentStage: number;
  progress: DownloadProgress;
}

// Global state - shared across all components
const downloads = ref<Map<number, DownloadState>>(new Map());

// Track if event listeners have been initialized
let listenersInitialized = false;
let progressUnlisten: UnlistenFn | null = null;
let completeUnlisten: UnlistenFn | null = null;
let errorUnlisten: UnlistenFn | null = null;
let restoredUnlisten: UnlistenFn | null = null;

/**
 * Initialize global event listeners (only once)
 */
async function initializeListeners() {
  if (listenersInitialized) return;
  listenersInitialized = true;

  // Listen for restored downloads (on app restart)
  restoredUnlisten = await listen<any>('download-restored', (event) => {
    const { gameId, progress, downloadedBytes, totalBytes, state } = event.payload;

    console.log(`[UI] Restored download for game ${gameId}`);

    // Initialize download state at stage 3 (downloading)
    downloads.value.set(gameId, {
      isRunning: true,
      isPaused: false,
      currentStage: 3,
      progress: {
        progress: progress,
        downloadedBytes: downloadedBytes,
        totalBytes: totalBytes,
        downloadSpeed: 0,
        uploadSpeed: 0,
        peers: 0,
        state: state,
      },
    });
  });

  // Listen for progress updates
  progressUnlisten = await listen<any>('download-progress', (event) => {
    const gameId = event.payload.gameId;
    const current = downloads.value.get(gameId);
    if (current) {
      current.progress = {
        progress: event.payload.progress,
        downloadedBytes: event.payload.downloadedBytes,
        totalBytes: event.payload.totalBytes,
        downloadSpeed: event.payload.downloadSpeed,
        uploadSpeed: event.payload.uploadSpeed,
        peers: event.payload.peers,
        state: event.payload.state,
      };
    }
  });

  // Listen for download completion
  completeUnlisten = await listen<any>('download-complete', (event) => {
    const gameId = event.payload.gameId;
    const current = downloads.value.get(gameId);
    if (current) {
      current.currentStage = 4; // Move to extraction stage
    }
  });

  // Listen for download errors
  errorUnlisten = await listen<any>('download-error', (event) => {
    const gameId = event.payload.gameId;
    const current = downloads.value.get(gameId);
    if (current) {
      current.isRunning = false;
      // Optionally remove from map or keep for error display
      // downloads.value.delete(gameId);
    }
  });
}

/**
 * Cleanup global listeners (call when app unmounts if needed)
 */
export function cleanupDownloadListeners() {
  if (progressUnlisten) progressUnlisten();
  if (completeUnlisten) completeUnlisten();
  if (errorUnlisten) errorUnlisten();
  if (restoredUnlisten) restoredUnlisten();
  listenersInitialized = false;
}

/**
 * Main composable for managing downloads
 */
export function useDownloads() {
  // Initialize listeners on first use
  initializeListeners();

  /**
   * Get download state for a specific game
   */
  const getDownloadState = (gameId: number) => {
    return downloads.value.get(gameId);
  };

  /**
   * Get reactive computed download state for a game
   */
  const getReactiveDownloadState = (gameId: number) => {
    return computed(() => downloads.value.get(gameId));
  };

  /**
   * Check if a game is currently downloading
   */
  const isDownloading = (gameId: number) => {
    const state = downloads.value.get(gameId);
    return state?.isRunning ?? false;
  };

  /**
 * Check for any active downloads on app startup
 */
  const checkForActiveDownloads = async () => {
    try {
      const activeDownloads = await invoke<any[]>('get_active_downloads');

      for (const download of activeDownloads) {
        console.log(`[UI] Found active download for game ${download.gameId}`);

        // Initialize download state at stage 3 (downloading)
        downloads.value.set(download.gameId, {
          isRunning: true,
          isPaused: false,
          currentStage: 3,
          progress: {
            progress: download.progress,
            downloadedBytes: download.downloadedBytes,
            totalBytes: download.totalBytes,
            downloadSpeed: 0,
            uploadSpeed: 0,
            peers: 0,
            state: download.state,
          },
        });
      }
    } catch (error) {
      console.error('[UI] Failed to check for active downloads:', error);
    }
  };

  /**
   * Initialize a new download
   */
  const initDownload = (gameId: number) => {
    downloads.value.set(gameId, {
      isRunning: true,
      isPaused: false,
      currentStage: 0,
      progress: {
        progress: 0,
        downloadedBytes: 0,
        totalBytes: 0,
        downloadSpeed: 0,
        uploadSpeed: 0,
        peers: 0,
        state: 'Initializing',
      },
    });
  };

  /**
   * Update the current stage of a download
   */
  const updateStage = (gameId: number, stage: number) => {
    const download = downloads.value.get(gameId);
    if (download) {
      download.currentStage = stage;
    }
  };

  /**
   * Mark download as paused
   */
  const setPaused = (gameId: number, paused: boolean) => {
    const download = downloads.value.get(gameId);
    if (download) {
      download.isPaused = paused;
    }
  };

  /**
   * Mark download as completed/stopped
   */
  const completeDownload = (gameId: number) => {
    const download = downloads.value.get(gameId);
    if (download) {
      download.isRunning = false;
      download.currentStage = 5; // Done stage
    }
  };

  /**
   * Remove download from state
   */
  const removeDownload = (gameId: number) => {
    downloads.value.delete(gameId);
  };

  /**
   * Get all active downloads
   */
  const activeDownloads = computed(() => {
    return Array.from(downloads.value.entries())
      .filter(([_, state]) => state.isRunning)
      .map(([id, state]) => ({ id, ...state }));
  });

  /**
   * Get count of active downloads
   */
  const activeDownloadCount = computed(() => {
    return activeDownloads.value.length;
  });

  return {
    // State
    downloads,
    activeDownloads,
    activeDownloadCount,

    // Methods
    getDownloadState,
    checkForActiveDownloads,
    getReactiveDownloadState,
    isDownloading,
    initDownload,
    updateStage,
    setPaused,
    completeDownload,
    removeDownload,
  };
}