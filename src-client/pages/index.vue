<script setup lang="ts">
import type { ButtonProps } from "#ui/types";

definePageMeta({
  layout: "home",
});

const accountStore = useAccountStore();
const announcementStore = useAnnouncementStore();

const isAnimatedHome = computed(() => accountStore.account?.isAnimatedHome);

const links = ref<ButtonProps[]>([
  {
    label: "Explore Games",
    to: "/explore",
    icon: "i-lucide-gamepad-2",
  },
  {
    label: "Learn more",
    to: "/#news",
    color: "neutral" as const,
    variant: "subtle" as const,
    trailingIcon: "i-lucide-arrow-right",
  },
]);

// Load announcements on mount
onMounted(async () => {
  await announcementStore.loadData();
  announcementStore.subscribeToChanges();
});

// Clean up subscription on unmount
onUnmounted(() => {
  announcementStore.unsubscribe();
});
</script>

<template>
  <div class="h-full p-4 mt-6">
    <div class="mt-4">
      <div class="relative overflow-hidden">
        <!-- GIF Background -->
        <div v-show="isAnimatedHome" class="absolute inset-0 -z-10">
          <img
            src="https://kotaku.com/app/uploads/2021/08/22ecb492fc415bbb28dab0bbfd3ad25d.gif"
            alt=""
            class="w-full h-full object-cover opacity-30"
          />
        </div>

        <UPageHero
          :title="`🎮\nNxShard`"
          description="Your all-in-one NS game downloader and installer"
          :links="links"
          class="whitespace-pre-line pb-10 relative z-10"
        />
      </div>
    </div>
  </div>
</template>
