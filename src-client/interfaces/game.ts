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