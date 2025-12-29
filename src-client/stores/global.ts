import { defineStore } from "pinia";
import { ref } from "vue";

export const useGlobalStore = defineStore("global", () => {
  const isDarkMode = ref(true);
  const isNativeWindowed = ref(false);
  const isErrorReportable = ref(true);
  const isInAppNotifEnabled = ref(false);
  const isDesktopNotifEnabled = ref(false);
  const isExperimental = ref(false);

  return {
    isDarkMode,
    isNativeWindowed,
    isErrorReportable,
    isInAppNotifEnabled,
    isDesktopNotifEnabled,
    isExperimental,
  };
});
