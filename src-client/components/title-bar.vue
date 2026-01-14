<template>
  <div
    class="titlebar text-black dark:text-white px-2 py-[5px] text-sm select-none flex items-center justify-between fixed top-0 left-0 right-0 z-50 overflow-hidden"
    :class="
      isOnline
        ? 'transition-colors duration-300 ease-in-out'
        : 'bg-warning-500 dark:bg-warning-600'
    "
    data-tauri-drag-region
  >
    <!-- Backdrop blur layer with gradient mask -->
    <div
      v-if="isOnline"
      class="absolute inset-0 backdrop-blur-md blur-gradient pointer-events-none"
    ></div>

    <!-- Shimmer overlay when offline -->
    <div
      v-if="!isOnline"
      class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent animate-shimmer pointer-events-none"
    ></div>

    <div class="flex items-center gap-2 relative z-10">
      <img src="@/assets/image/favicon.png" class="ml-1 h-4 w-4" alt="🌿" />
      {{ title }}
    </div>

    <div v-if="!isOnline" class="relative z-10">
      <UBadge icon="i-lucide-wifi" size="md" color="warning" variant="soft">
        No Internet Connection
      </UBadge>
    </div>
    <div v-else class="relative z-10"></div>

    <div class="flex items-center gap-2 relative z-10">
      <div class="flex items-center gap-2">
        <UIcon
          v-if="charging"
          name="i-lucide-battery-charging"
          class="h-4 w-4"
        />
        <UIcon
          v-else-if="level >= 0.9"
          name="i-lucide-battery-full"
          class="h-4 w-4"
        />
        <UIcon
          v-else-if="level >= 0.5"
          name="i-lucide-battery-medium"
          class="h-4 w-4"
        />
        <UIcon
          v-else-if="level >= 0.25"
          name="i-lucide-battery-low"
          class="h-4 w-4"
        />
        <UIcon
          v-else-if="level >= 0.1"
          name="i-lucide-battery-warning"
          class="h-4 w-4"
        />
        <UIcon v-else name="i-lucide-plug-zap" class="h-4 w-4" />

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

/* Gradient mask for blur effect */
.blur-gradient {
  mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 100%);
  -webkit-mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 100%);
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.animate-shimmer {
  animation: shimmer 5s ease-in-out infinite;
}
</style>

<script setup lang="ts">
import { useBattery } from "@vueuse/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, onUnmounted, ref } from "vue";

const win = getCurrentWindow();
const title = ref("");
const isMaximized = ref(false);
const isOnline = useOnline();
const { charging, level } = useBattery();

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
</script>