export interface User {
  id: string;
  displayName: string | null | undefined;
  email: string;
  password: string | null;
  power: number;
  imageUrl: string;
  createdAt: string;
  isDarkmode: boolean;
  isExperimental: boolean;
  isApproved: boolean;
  isNew: boolean;
  isAdmin: boolean;
  isAnimatedHome: boolean;
  isLogEnable: boolean;
  isFtpInstall: boolean;
  isMtpInstall: boolean;
}
