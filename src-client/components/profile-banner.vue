<script setup lang="ts">
const accountStore = useAccountStore();
const router = useRouter();

const isLoading = ref(true);
const displayName = computed(() => accountStore.account?.displayName);
const isExperimental = computed(() => accountStore.account?.isExperimental);
const isAdmin = computed(() => accountStore.account?.isAdmin);

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false;
  }, 1000);
});
</script>

<template>
  <div class="flex items-center gap-4 mb-4 cursor-pointer p-2 rounded-md hover:bg-neutral-800" @click="router.push(`/account`)">
    <USkeleton v-if="isLoading" class="h-12 w-12 rounded-full" />
    <UAvatar v-else :src="accountStore.account?.imageUrl" :alt="displayName || 'NA'" class="h-12 w-12" />

    <div class="grid gap-2 flex-1">
      <USkeleton v-if="isLoading" class="h-4 w-[150px]" />
      <span v-else class="font-semibold text-sm">
        <UBadge v-show="isExperimental" icon="i-lucide-flask-conical" size="sm" variant="outline"/>
        <UBadge v-show="isAdmin" icon="i-lucide-shield-user" size="sm" variant="outline" color="info"/>
        {{ displayName }}
      </span>

      <USkeleton v-if="isLoading" class="h-4 w-[100px]" />
      <span
        v-else-if="accountStore.account?.isAdmin"
        class="text-xs text-gray-500 dark:text-gray-400 w-[150px] truncate"
        >👾 The Developer</span
      >
      <span
        v-else
        class="text-xs text-gray-500 dark:text-gray-400 w-[150px] truncate"
        >{{ accountStore.account?.email }}</span
      >
    </div>
  </div>
</template>
