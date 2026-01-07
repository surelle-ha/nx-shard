// types/ftp.ts

export interface GameFile {
  gameId: number;
  gameTitle: string;
  filePath: string;
  fileName: string;
  fileSize: number;
}

export type TransferStatus =
  | "queued"
  | "transferring"
  | "completed"
  | "failed"
  | "cancelled";

export interface TransferProgress {
  gameId: number;
  fileName: string;
  bytesTransferred: number;
  totalBytes: number;
  progressPercent: number;
  transferSpeed: number; // bytes per second
  etaSeconds: number;
  status: TransferStatus;
}

export interface QueueUpdatePayload {
  queueLength: number;
}

export interface TransferCompletePayload {
  gameId: number;
  fileName: string;
}

export interface TransferErrorPayload {
  gameId: number;
  fileName: string;
  error: string;
}
