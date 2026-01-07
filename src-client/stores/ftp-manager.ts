// stores/ftpManager.ts
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  GameFile,
  TransferProgress,
  QueueUpdatePayload,
  TransferCompletePayload,
  TransferErrorPayload,
} from "~/interfaces/ftp";

interface FTPManagerState {
  ftpIp: string | null;
  availableGames: GameFile[];
  transferQueue: GameFile[];
  currentTransfer: TransferProgress | null;
  isTransferring: boolean;
  isScanning: boolean;
  error: string | null;
}

export const useFTPManagerStore = defineStore("ftpManager", {
  state: (): FTPManagerState => ({
    ftpIp: null,
    availableGames: [],
    transferQueue: [],
    currentTransfer: null,
    isTransferring: false,
    isScanning: false,
    error: null,
  }),

  getters: {
    hasActiveFTP: (state) => state.ftpIp !== null,
    queueLength: (state) => state.transferQueue.length,
    formattedTransferSpeed: (state) => {
      if (!state.currentTransfer) return "0 MB/s";
      const mbps = state.currentTransfer.transferSpeed / 1_000_000;
      return `${mbps.toFixed(2)} MB/s`;
    },
    formattedETA: (state) => {
      if (!state.currentTransfer) return "--:--";
      const seconds = state.currentTransfer.etaSeconds;
      const mins = Math.floor(seconds / 60);
      const secs = seconds % 60;
      return `${mins}:${secs.toString().padStart(2, "0")}`;
    },
    formattedFileSize: () => (bytes: number) => {
      const mb = bytes / 1_000_000;
      if (mb > 1000) {
        return `${(mb / 1000).toFixed(2)} GB`;
      }
      return `${mb.toFixed(2)} MB`;
    },
  },

  actions: {
    // Initialize event listeners
    async initListeners() {
      try {
        // Listen for transfer progress updates
        await listen<TransferProgress>("ftp-transfer-progress", (event) => {
          this.currentTransfer = event.payload;
          this.isTransferring = event.payload.status === "transferring";
        });

        // Listen for queue updates
        await listen<QueueUpdatePayload>("ftp-queue-updated", (event) => {
          // Refresh queue from backend
          this.refreshQueue();
        });

        // Listen for transfer completion
        await listen<TransferCompletePayload>(
          "ftp-transfer-complete",
          (event) => {
            console.log(`Transfer completed: ${event.payload.fileName}`);
            this.currentTransfer = null;
            this.isTransferring = false;
            this.refreshQueue();
          }
        );

        // Listen for transfer errors
        await listen<TransferErrorPayload>("ftp-transfer-error", (event) => {
          console.error(`Transfer error: ${event.payload.error}`);
          this.error = event.payload.error;
          this.currentTransfer = null;
          this.isTransferring = false;
          this.refreshQueue();
        });

        console.log("FTP Manager listeners initialized");
      } catch (error) {
        console.error("Failed to initialize FTP Manager listeners:", error);
        throw error;
      }
    },

    // Set FTP IP address
    async setFtpIp(ip: string) {
      try {
        this.error = null;
        const result = await invoke<string>("set_ftp_ip", { ip });
        this.ftpIp = ip;
        console.log(result);
        return result;
      } catch (error) {
        this.error = error as string;
        throw error;
      }
    },

    // Get current FTP IP
    async getFtpIp() {
      try {
        const ip = await invoke<string | null>("get_ftp_ip");
        this.ftpIp = ip;
        return ip;
      } catch (error) {
        console.error("Failed to get FTP IP:", error);
        throw error;
      }
    },

    // Scan for available game files
    async scanGameFiles(gamePath: string = "./game_path") {
      try {
        this.isScanning = true;
        this.error = null;
        const games = await invoke<GameFile[]>("scan_game_files", { gamePath });
        this.availableGames = games;
        console.log(`Found ${games.length} game files`);
        return games;
      } catch (error) {
        this.error = error as string;
        console.error("Failed to scan game files:", error);
        throw error;
      } finally {
        this.isScanning = false;
      }
    },

    // Queue a file for transfer
    async queueFile(gameFile: GameFile) {
      try {
        this.error = null;
        const result = await invoke<string>("queue_file", { gameFile });
        console.log(result);
        await this.refreshQueue();
        return result;
      } catch (error) {
        this.error = error as string;
        throw error;
      }
    },

    // Get current transfer queue
    async refreshQueue() {
      try {
        const queue = await invoke<GameFile[]>("get_transfer_queue");
        this.transferQueue = queue;
        return queue;
      } catch (error) {
        console.error("Failed to refresh queue:", error);
        throw error;
      }
    },

    // Clear transfer queue
    async clearQueue() {
      try {
        this.error = null;
        const result = await invoke<string>("clear_transfer_queue");
        this.transferQueue = [];
        console.log(result);
        return result;
      } catch (error) {
        this.error = error as string;
        throw error;
      }
    },

    // Remove specific file from queue
    async removeFromQueue(filePath: string) {
      try {
        this.error = null;
        const result = await invoke<string>("remove_from_transfer_queue", {
          filePath,
        });
        await this.refreshQueue();
        console.log(result);
        return result;
      } catch (error) {
        this.error = error as string;
        throw error;
      }
    },

    // Get current transfer status
    async getCurrentTransfer() {
      try {
        const transfer = await invoke<TransferProgress | null>(
          "get_current_transfer"
        );
        this.currentTransfer = transfer;
        return transfer;
      } catch (error) {
        console.error("Failed to get current transfer:", error);
        throw error;
      }
    },

    // Check if currently transferring
    async checkIsTransferring() {
      try {
        const isTransferring = await invoke<boolean>("is_ftp_transferring");
        this.isTransferring = isTransferring;
        return isTransferring;
      } catch (error) {
        console.error("Failed to check transfer status:", error);
        return false;
      }
    },

    // Cleanup
    cleanup() {
      this.currentTransfer = null;
      this.isTransferring = false;
      this.error = null;
    },

    // Clear error
    clearError() {
      this.error = null;
    },
  },
});
