<script setup lang="ts">
const globalStore = useGlobalStore();
const accountStore = useAccountStore();

const isLoading = ref(true);
const isExperimental = computed(() => globalStore.isExperimental);

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false;
  }, 1000);
});
</script>

<template>
  <div class="flex items-center gap-4 mb-4">
    <USkeleton v-if="isLoading" class="h-12 w-12 rounded-full" />
    <UAvatar v-else :src="accountStore.account?.imageUrl" :alt="accountStore.account?.name || 'NA'" class="h-12 w-12" />

    <div class="grid gap-2 flex-1">
      <USkeleton v-if="isLoading" class="h-4 w-[150px]" />
      <span v-else class="font-semibold text-sm"><UBadge v-show="isExperimental" icon="i-lucide-flask-conical" size="sm" variant="outline"/> {{ accountStore.account?.name }} </span>

      <USkeleton v-if="isLoading" class="h-4 w-[100px]" />
      <span
        v-else
        class="text-xs text-gray-500 dark:text-gray-400 w-[150px] truncate"
        >{{ accountStore.account?.email }}</span
      >
    </div>
  </div>
</template>
