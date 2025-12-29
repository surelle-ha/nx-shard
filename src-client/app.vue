<script setup lang="ts">
const globalStore = useGlobalStore();

const isExperimental = computed(() => globalStore.isExperimental);

const handleContextMenu = (e: MouseEvent) => {
  if (!isExperimental.value) {
    e.preventDefault();
  }
};
</script>

<template>
  <TitleBar />
  <UApp :toaster="{ position: 'top-center' }">
    <NuxtLayout @contextmenu="handleContextMenu">
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
