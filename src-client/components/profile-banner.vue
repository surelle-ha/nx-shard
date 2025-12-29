<script setup lang="ts">
const isLoading = ref(true);

const user = {
  id: "c120b694-cec0-41e9-9dd6-feb43f15a5fe",
  name: "Guest",
  role: "🎮 A Legend",
  avatar: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRADwvSebpVf1hSgjHDZyMEpVGxqiAlvE09dA&s",
};

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false;
  }, 1000);
});
</script>

<template>
  <div class="flex items-center gap-4 mb-4">
    <USkeleton v-if="isLoading" class="h-12 w-12 rounded-full" />
    <UAvatar v-else :src="user.avatar" :alt="user.name" class="h-12 w-12" />

    <div class="grid gap-2 flex-1">
      <USkeleton v-if="isLoading" class="h-4 w-[150px]" />
      <span v-else class="font-semibold text-sm">{{ user.name }} </span>

      <USkeleton v-if="isLoading" class="h-4 w-[100px]" />
      <span
        v-else
        class="text-xs text-gray-500 dark:text-gray-400 w-[150px] truncate"
        >{{ user.role }}</span
      >
    </div>
  </div>
</template>
