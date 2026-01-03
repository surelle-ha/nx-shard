<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { GameMeta } from "~/interfaces/game";

const toast = useToast();

const props = defineProps<{
  game: GameMeta;
}>();

const isOpen = ref(false);
const isRunning = ref(false);
const currentStage = ref(0);

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
 * Helper: advance to a stage
 */
const goToStage = (index: number) => {
  currentStage.value = index;
};

const getGameMetadata = async () => {
  console.log("[Shard_Torrent_Runner] Obtaining game data.");
  console.log();
  invoke("get_game_meta", { invokeMessage: props.game });
  await sleep(1000);
};

const createGameDirectory = async () => {
  console.log("[Shard_Torrent_Runner] Creating game directory.");
  await invoke("create_game_dir", { invokeMessage: props.game });
  await sleep(1000);
};

const obtainTorrent = async () => {
  console.log("[Shard_Torrent_Runner] Obtaining torrent file.");
  await invoke("obtain_torrent_file", { invokeMessage: props.game });
  await sleep(1000);
};

const downloadGame = async () => {
  console.log("[Shard_Torrent_Runner] Downloading.");
  await invoke("download_game", { invokeMessage: props.game });
  await sleep(1000);
};

const extractAndCleanup = async () => {
  console.log("[Shard_Torrent_Runner] Run extraction and clean up");
  await invoke("extract_and_clean", { invokeMessage: props.game });
  await sleep(1000);
};

/**
 * Main pipeline
 */
const startDownload = async () => {
  if (isRunning.value) return;

  isRunning.value = true;
  isOpen.value = true;
  currentStage.value = 0;

  try {
    goToStage(0);
    await getGameMetadata();

    goToStage(1);
    await createGameDirectory();

    goToStage(2);
    await obtainTorrent();

    goToStage(3);
    await downloadGame();

    goToStage(4);
    await extractAndCleanup();

    goToStage(5);

    toast.add({
      title: "Download Complete",
      icon: "i-lucide-download",
    });
  } catch (err) {
    toast.add({
      title: "Download Failed",
      description: err instanceof Error ? err.message : "Unknown error",
      color: "error",
    });
  } finally {
    isRunning.value = false;
  }
};
</script>

<template>
  <UButton
    v-if="!isRunning"
    label="Download"
    icon="i-lucide-download"
    color="primary"
    class="mt-2 cursor-pointer"
    variant="ghost"
    :loading="isRunning"
    @click="startDownload"
  />
  <UProgress
    v-else
    :model-value="currentStage"
    :max="stages.length - 1"
    class="cursor-pointer"
  />
</template>
