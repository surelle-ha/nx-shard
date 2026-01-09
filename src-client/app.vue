<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const accountStore = useAccountStore();
const globalStore = useGlobalStore();
const isExperimental = computed(() => accountStore.account?.isExperimental);
const handleContextMenu = (e: MouseEvent) => {
  if (!isExperimental.value) {
    e.preventDefault();
  }
};
onMounted(() => {
  globalStore.initialize();
});

/**
 * BELOW ARE THE RUST-LEVEL
 * OPERATIONS.
 */
/** Check File System */
invoke("check_file_system");


const { activeDownloads } = useDownloads();

onMounted(async () => {
  // Check for any downloads that were restored
  try {
    const activeDownloads: any = await invoke('get_active_downloads');
    
    for (const download of activeDownloads) {
      console.log(`[UI] Found active download for game ${download.gameId}`);
      
      // Use your composable to initialize the state
      const { initDownload, updateStage, getDownloadState } = useDownloads();
      initDownload(download.gameId);
      updateStage(download.gameId, 3); // Set to downloading stage
      
      // Update with current progress
      const downloadState = getDownloadState(download.gameId);
      if (downloadState) {
        downloadState.progress = {
          progress: download.progress,
          downloadedBytes: download.downloadedBytes,
          totalBytes: download.totalBytes,
          downloadSpeed: 0,
          uploadSpeed: 0,
          peers: 0,
          state: download.state,
        };
      }
    }
  } catch (error) {
    console.error('[UI] Failed to check for active downloads:', error);
  }
});
</script>

<template>
  <TitleBar />
  <UApp :toaster="{ position: 'top-center' }">
    <NuxtLayout @contextmenu="handleContextMenu">
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
