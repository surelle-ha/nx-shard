<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { GameMeta } from "~/interfaces/game";
  
  const accountStore = useAccountStore();
  const toast = useToast();
  const {
    getReactiveDownloadState,
    initDownload,
    updateStage,
    setPaused,
    completeDownload,
    removeDownload,
  } = useDownloads();
  
  const props = defineProps<{
    game: GameMeta;
  }>();
  
  const isDownloaded = ref(false);
  
  const checkIfDownloaded = async () => {
    try {
      isDownloaded.value = await invoke<boolean>("is_game_downloaded", {
        invokeMessage: props.game,
      });
    } catch {
      isDownloaded.value = false;
    }
  };
  
  const removeFromLibrary = async () => {
    try {
      await accountStore.removeFromLibrary(props.game.id);
      toast.add({
        title: "Removed from Library",
        description: `${props.game.title} has been removed`,
        color: "warning",
        icon: "i-heroicons-trash",
      });
    } catch (error: any) {
      toast.add({
        title: "Error",
        description: error.message || "Failed to remove game",
        color: "error",
        icon: "i-heroicons-exclamation-circle",
      });
    }
  };
  
  const removeFromDisk = async () => {
    try {
      // Call your new Tauri command
      await invoke("uninstall_game", { invokeMessage: props.game });
  
      // Update local state
      isDownloaded.value = false;
      removeDownload(props.game.id);
  
      toast.add({
        title: "Game Removed",
        description: `${props.game.title} has been removed from disk`,
        color: "warning",
        icon: "i-heroicons-trash",
      });
    } catch (error: any) {
      toast.add({
        title: "Failed to Remove Game",
        description: error.message || "An error occurred while removing the game",
        color: "error",
        icon: "i-heroicons-exclamation-circle",
      });
    }
  };
  
  onMounted(async () => {
    await checkIfDownloaded();
    // Note: download-restored listener is now in the global composable
  });
  
  // Get reactive download state for this specific game
  const downloadState = getReactiveDownloadState(props.game.id);
  
  function sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
  
  /**
   * Stage labels (UI only)
   */
  const stages = [
    "Getting game metadata...",
    "Creating game directory...",
    "Obtaining torrent file...",
    "Downloading game...",
    "Extracting and cleaning up...",
    "Done!",
  ];
  
  /**
   * Pipeline steps
   */
  const getGameMetadata = async () => {
    console.log("[Shard_Torrent_Runner] Obtaining game data.");
    invoke("get_game_meta", { invokeMessage: props.game });
    await sleep(500);
  };
  
  const createGameDirectory = async () => {
    console.log("[Shard_Torrent_Runner] Creating game directory.");
    await invoke("create_game_dir", { invokeMessage: props.game });
    await sleep(500);
  };
  
  const obtainTorrent = async () => {
    console.log("[Shard_Torrent_Runner] Obtaining torrent file.");
    await invoke("obtain_torrent_file", { invokeMessage: props.game });
    await sleep(500);
  };
  
  const downloadGame = async () => {
    console.log("[Shard_Torrent_Runner] Downloading.");
    // This now starts the download but doesn't block
    await invoke("download_game", { invokeMessage: props.game });
  };
  
  const extractAndCleanup = async () => {
    console.log("[Shard_Torrent_Runner] Run extraction and clean up");
    await invoke("extract_and_clean", { invokeMessage: props.game });
    await sleep(500);
  };
  
  /**
   * Pause/Resume controls
   */
  const pauseDownload = async () => {
    try {
      await invoke("pause_game", { invokeMessage: props.game });
      setPaused(props.game.id, true);
      toast.add({
        title: "Download Paused",
        icon: "i-lucide-pause",
      });
    } catch (err) {
      toast.add({
        title: "Failed to pause",
        description: err instanceof Error ? err.message : "Unknown error",
        color: "error",
      });
    }
  };
  
  const resumeDownload = async () => {
    try {
      await invoke("resume_game", { invokeMessage: props.game });
      setPaused(props.game.id, false);
      toast.add({
        title: "Download Resumed",
        icon: "i-lucide-play",
      });
    } catch (err) {
      toast.add({
        title: "Failed to resume",
        description: err instanceof Error ? err.message : "Unknown error",
        color: "error",
      });
    }
  };
  
  /**
   * Format bytes to human-readable
   */
  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  };
  
  /**
   * Format speed (already in Mbps from backend)
   */
  const formatSpeed = (mbps: number): string => {
    if (mbps < 1) {
      return `${(mbps * 1000).toFixed(0)} Kbps`;
    }
    return `${mbps.toFixed(2)} Mbps`;
  };
  
  /**
   * Main download pipeline
   */
  const startDownload = async () => {
    if (downloadState.value?.isRunning) {
      toast.add({
        title: "Already Downloading",
        description: "This game is already being downloaded",
        color: "warning",
      });
      return;
    }
  
    // Initialize download state in global store
    initDownload(props.game.id);
  
    try {
      // Stage 0: Get metadata
      updateStage(props.game.id, 0);
      await getGameMetadata();
  
      // Stage 1: Create directory
      updateStage(props.game.id, 1);
      await createGameDirectory();
  
      // Stage 2: Obtain torrent
      updateStage(props.game.id, 2);
      await obtainTorrent();
  
      // Stage 3: Download (runs in background)
      updateStage(props.game.id, 3);
      await downloadGame();
  
      // Note: Stages 4 and 5 will be handled by event listeners
      // Stage 4 is triggered by "download-complete" event
      // Stage 5 is set after extraction completes
    } catch (err) {
      removeDownload(props.game.id);
      toast.add({
        title: "Download Failed",
        description: err instanceof Error ? err.message : "Unknown error",
        color: "error",
      });
    }
  };
  
  /**
   * Handle completion (triggered by download-complete event)
   */
  watchEffect(() => {
    if (downloadState.value?.currentStage === 4) {
      // Extraction stage - run cleanup
      extractAndCleanup()
        .then(async () => {
          completeDownload(props.game.id);
  
          await checkIfDownloaded();
  
          toast.add({
            title: "Download Complete",
            description: `${props.game.title} is ready to play!`,
            icon: "i-lucide-check-circle",
          });
        })
        .catch((err) => {
          toast.add({
            title: "Cleanup Failed",
            description: err instanceof Error ? err.message : "Unknown error",
            color: "error",
          });
          removeDownload(props.game.id);
        });
    }
  });
  </script>
  
  <template>
    <div>
      <!-- Downloading State - Show if actively running -->
      <div v-if="downloadState?.isRunning" class="mt-2 space-y-2">
        <!-- Stage Progress -->
        <UProgress
          :model-value="downloadState?.currentStage ?? 0"
          :max="stages.length - 1"
          size="sm"
        />
        <div class="text-xs text-gray-500">
          {{ stages[downloadState?.currentStage ?? 0] }}
        </div>
  
        <!-- Download Progress (Stage 3 only) -->
        <div v-if="downloadState?.currentStage === 3" class="space-y-2">
          <!-- Main Progress Bar -->
          <UProgress
            :model-value="downloadState?.progress.progress ?? 0"
            :max="100"
            size="md"
            color="primary"
          />
  
          <!-- Stats -->
          <div class="flex justify-between text-xs">
            <span>{{ (downloadState?.progress.progress ?? 0).toFixed(1) }}%</span>
            <span>
              {{ formatBytes(downloadState?.progress.downloadedBytes ?? 0) }} /
              {{ formatBytes(downloadState?.progress.totalBytes ?? 0) }}
            </span>
          </div>
  
          <!-- Speed and Peers -->
          <div class="flex justify-between text-xs text-gray-500">
            <span>
              <span class="i-lucide-download inline-block w-3 h-3" />
              {{ formatSpeed(downloadState?.progress.downloadSpeed ?? 0) }}
            </span>
            <span>
              <span class="i-lucide-upload inline-block w-3 h-3" />
              {{ formatSpeed(downloadState?.progress.uploadSpeed ?? 0) }}
            </span>
            <span>
              <span class="i-lucide-users inline-block w-3 h-3" />
              {{ downloadState?.progress.peers ?? 0 }} peers
            </span>
          </div>
  
          <!-- State Info -->
          <div class="text-xs text-gray-400">
            State: {{ downloadState?.progress.state ?? "Unknown" }}
          </div>
  
          <!-- Controls -->
          <div class="flex gap-2">
            <UButton
              v-if="!downloadState?.isPaused"
              icon="i-lucide-pause"
              size="xs"
              color="neutral"
              @click="pauseDownload"
            >
              Pause
            </UButton>
            <UButton
              v-else
              icon="i-lucide-play"
              size="xs"
              color="primary"
              @click="resumeDownload"
            >
              Resume
            </UButton>
            <UButton
              icon="i-lucide-x"
              size="xs"
              variant="subtle"
              color="error"
              @click="removeFromDisk"
            >
              Cancel
            </UButton>
          </div>
        </div>
  
        <!-- Completion Message -->
        <div
          v-if="downloadState?.currentStage === 5"
          class="text-sm text-green-600 dark:text-green-400"
        >
          <span class="i-lucide-check-circle inline-block w-4 h-4 mr-1" />
          Download complete!
        </div>
      </div>
  
      <!-- Install - Only show if downloaded AND not currently downloading -->
      <div v-else-if="isDownloaded" class="mt-2 flex justify-between gap-2">
        <UButton
          label="Install"
          icon="i-lucide-hard-drive"
          variant="ghost"
          color="primary"
          class="cursor-pointer"
        />
        <UButton
          icon="i-lucide-trash"
          variant="ghost"
          color="error"
          class="cursor-pointer"
          @click="removeFromDisk"
        />
      </div>
  
      <!-- Download - Show if not running and not downloaded -->
      <div v-else class="mt-2 flex justify-between gap-2">
        <UButton
          label="Download"
          icon="i-lucide-download"
          color="primary"
          variant="ghost"
          class="mt-2 cursor-pointer"
          @click="startDownload"
        />
        <UButton
          icon="i-lucide-circle-minus"
          variant="ghost"
          color="error"
          class="cursor-pointer"
          @click="removeFromLibrary"
        />
      </div>
    </div>
  </template>