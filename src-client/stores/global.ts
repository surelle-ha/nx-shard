import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface GlobalSettings {
  version: string | null;
}

export const useGlobalStore = defineStore("global", () => {
  const settings = ref<GlobalSettings>({
    version: null,
  });

  async function initialize() {
    try {
      const version = await invoke<string>("get_version");
      settings.value.version = version ?? null;
    } catch (error) {
      console.error("Failed to get app version:", error);
      settings.value.version = null;
    }
  }

  return {
    settings,
    initialize,
  };
});
