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

/** Start Background Processes */
// invoke("start_bgp");
</script>

<template>
  <TitleBar />
  <UApp :toaster="{ position: 'top-center' }">
    <NuxtLayout @contextmenu="handleContextMenu">
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
