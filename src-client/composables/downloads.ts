// composables/useDownloads.ts
import { ref, computed } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

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

/**
 * Initialize global event listeners (only once)
 */
async function initializeListeners() {
  if (listenersInitialized) return;
  listenersInitialized = true;

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
    getReactiveDownloadState,
    isDownloading,
    initDownload,
    updateStage,
    setPaused,
    completeDownload,
    removeDownload,
  };
}