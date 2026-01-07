<script setup lang="ts">
  
</script>

<template>
  <div
    class="bg flex flex-col items-center justify-center gap-4 p-4 min-h-screen"
  >
    <slot />
  </div>
</template>

<style scoped>
.bg {
  background-image: url("@/assets/image/bg-auth.gif");
  background-repeat: no-repeat;
  background-size: cover;
}
</style>
