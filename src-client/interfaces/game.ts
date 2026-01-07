export interface GameMeta {
  id: number;
  title: string;
  description: string;
  coverUrl: string;
  downloadUrl: string;
  tags: string[];
  isExperimental: boolean;
  isEnabled: boolean;
  isBroken: boolean;
  createdAt: string;
}

export interface DownloadProgress {
  progress: number;
  downloadedBytes: number;
  totalBytes: number;
  downloadSpeed: number;
  uploadSpeed: number;
  peers: number;
  state: string;
}

export interface DownloadState {
  isRunning: boolean;
  isPaused: boolean;
  currentStage: number;
  progress: DownloadProgress;
}

export interface TorrentStats {
  gameId: number;
  state: string;
  progress: number;
  downloadedBytes: number;
  totalBytes: number;
  downloadSpeed: number;
  uploadSpeed: number;
  peers: number;
}