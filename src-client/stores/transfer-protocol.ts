// stores/transferProtocol.ts
import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

interface FTPStatusPayload {
  isActive: boolean;
  activeCount: number;
  activeIps: string[];
}

interface TransferProtocolState {
  isMTPActive: boolean;
  isFTPActive: boolean;
  isMonitoring: boolean;
  ftpIps: string[];
  selectedFtpIp: string | null;
  listeners: UnlistenFn[];
}

export const useTransferProtocolStore = defineStore('transferProtocol', {
  state: (): TransferProtocolState => ({
    isMTPActive: false,
    isFTPActive: false,
    isMonitoring: false,
    ftpIps: [],
    selectedFtpIp: null,
    listeners: [],
  }),

  getters: {
    hasActiveFTP: (state) => state.isFTPActive && state.ftpIps.length > 0,
    primaryFtpIp: (state) => state.selectedFtpIp || state.ftpIps[0] || null,
  },

  actions: {
    async initFTPListener() {
      try {
        // Listen for FTP status changes
        const unlisten = await listen<FTPStatusPayload>('ftp-status-changed', async (event) => {
          console.log('FTP status changed:', event.payload);
          
          this.isFTPActive = event.payload.isActive;
          this.ftpIps = event.payload.activeIps;

          // Automatically set FTP IP in the manager when detected
          if (event.payload.isActive && event.payload.activeIps.length > 0) {
            const ftpManagerStore = useFTPManagerStore();
            const primaryIp = event.payload.activeIps[0];
            
            // Only set if not already set or if IP changed
            const currentIp = await ftpManagerStore.getFtpIp();
            if (primaryIp && currentIp !== primaryIp) {
              try {
                await ftpManagerStore.setFtpIp(primaryIp);
                this.selectedFtpIp = primaryIp;
                console.log(`✓ FTP IP automatically set to: ${primaryIp}`);
              } catch (error) {
                console.error('Failed to set FTP IP:', error);
                // Even if setting fails, mark as active since we detected it
                this.isFTPActive = true;
              }
            } else if (currentIp) {
              // IP already set and matches, just update selectedFtpIp
              this.selectedFtpIp = currentIp;
              console.log(`✓ FTP connection maintained: ${currentIp}`);
            }
          } else {
            // No active FTP connections
            this.selectedFtpIp = null;
            console.log('✗ No active FTP connections');
          }
        });

        this.listeners.push(unlisten);
        console.log('FTP listener initialized');
      } catch (error) {
        console.error('Failed to initialize FTP listener:', error);
        throw error;
      }
    },

    async startFTPMonitor(scanInterval: number = 10) {
      try {
        const result = await invoke<string>('start_ftp_monitor', {
          scanInterval,
        });
        this.isMonitoring = true;
        console.log(result);
        return result;
      } catch (error) {
        console.error('Failed to start FTP monitor:', error);
        throw error;
      }
    },

    async stopFTPMonitor() {
      try {
        const result = await invoke<string>('stop_ftp_monitor');
        this.isMonitoring = false;
        console.log(result);
        return result;
      } catch (error) {
        console.error('Failed to stop FTP monitor:', error);
        throw error;
      }
    },

    async checkMonitorStatus() {
      try {
        const isRunning = await invoke<boolean>('is_ftp_monitor_running');
        this.isMonitoring = isRunning;
        return isRunning;
      } catch (error) {
        console.error('Failed to check monitor status:', error);
        return false;
      }
    },

    // Manual FTP IP selection (for multiple devices)
    async selectFtpIp(ip: string) {
      const ftpManagerStore = useFTPManagerStore();
      try {
        await ftpManagerStore.setFtpIp(ip);
        this.selectedFtpIp = ip;
        console.log(`Manually selected FTP IP: ${ip}`);
      } catch (error) {
        console.error('Failed to select FTP IP:', error);
        throw error;
      }
    },

    cleanup() {
      // Unlisten from all events
      this.listeners.forEach((unlisten) => unlisten());
      this.listeners = [];
      console.log('Transfer protocol listeners cleaned up');
    },
  },
});