<script setup lang="ts">
import type { GameMeta } from "~/interfaces/game";

const toast = useToast();
const props = defineProps<{
  game: GameMeta;
}>();

const value = ref(0);
const isOpen = ref(false);
const isSimulating = ref(false);

const stages = [
  "Getting game metadata...",
  "Obtaining torrent file...",
  "Torrent file downloaded. Creating game directory...",
  "Game downloading...",
  "Downloaded. Extracting and removing torrent file...",
  "Done!",
];

const simulateProgress = () => {
  if (isSimulating.value) return;

  isSimulating.value = true;
  isOpen.value = true;
  value.value = 0;

  stages.forEach((_, index) => {
    setTimeout(() => {
      value.value = index;
      if (index === stages.length - 1) {
        setTimeout(() => {
          toast.add({
            title: "Download Complete",
            icon: 'i-lucide-download',
          });
          isSimulating.value = false;
        }, 1000);
      }
    }, index * 1500);
  });
};
</script>

<template>
  <UDrawer v-model="isOpen" :overlay="false">
    <UButton
      label="Download"
      icon="i-lucide-download"
      color="primary"
      variant="solid"
      class="w-full cursor-pointer rounded-t-none"
      @click="simulateProgress"
    />

    <template #content>
      <div class="px-6 py-4">
        <div class="flex items-center gap-4">
          <img
            :src="game.image"
            :alt="game.title"
            class="w-16 h-16 rounded-lg object-cover flex-shrink-0"
          />
          <div class="flex-1 min-w-0">
            <h3 class="text-sm font-semibold mb-2 truncate">
              {{ game.title }}
            </h3>
            <UProgress v-model="value" :max="stages" />
          </div>
        </div>
      </div>
    </template>
  </UDrawer>
</template>
