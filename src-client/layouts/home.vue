<script setup lang="ts">
const sidebarOpen = ref(true);
const sidebarWidth = 248;
</script>

<template>
  <div class="flex h-screen overflow-hidden">
    <NavigationBar />
    <div
      class="flex-1 transition-all duration-300 ease-in-out"
      :style="{ marginLeft: sidebarOpen ? `${sidebarWidth}px` : '0' }"
    >
      <slot />
    </div>
  </div>
</template>
