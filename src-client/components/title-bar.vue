<template>
  <div
    class="titlebar dark:text-white bg-white dark:bg-gray-900 px-2 py-[5px] text-sm select-none flex items-center justify-between fixed top-0 left-0 right-0 z-50"
    data-tauri-drag-region
  >
    <div class="flex items-center gap-2">🌿 {{ title }}</div>

    <div class="flex items-center gap-2">
      <div class="flex items-center gap-2">
        <button
          @click="minimize"
          class="window-btn w-3 h-3 rounded-full bg-[#FEBC2E] hover:bg-[#FEBC2E]/80 cursor-pointer"
        ></button>
        <button
          @click="close"
          class="window-btn w-3 h-3 rounded-full bg-[#FF5F57] hover:bg-[#FF5F57]/80 cursor-pointer"
        ></button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, onUnmounted, ref } from "vue";

const win = getCurrentWindow();
const title = ref("");
const isMaximized = ref(false);

const minimize = async () => {
  await win.minimize();
  console.log("minimized");
};

const close = async () => {
  await win.close();
  console.log("closed");
};

const checkMaximized = async () => {
  isMaximized.value = await win.isMaximized();
};

let unlistenResize: (() => void) | null = null;

onMounted(async () => {
  title.value = await win.title();
  await checkMaximized();
  unlistenResize = await win.onResized(async () => {
    await checkMaximized();
  });
});

onUnmounted(() => {
  if (unlistenResize) {
    unlistenResize();
  }
});
const value = ref({})
</script>

<style scoped>
.titlebar {
  -webkit-app-region: drag;
  app-region: drag;
  user-select: none;
  -webkit-user-select: none;
}

.titlebar button {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.window-btn {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}
</style>
