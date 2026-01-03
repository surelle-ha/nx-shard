<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const accountStore = useAccountStore();
const globalStore = useGlobalStore();
const toast = useToast();

const sidebarOpen = ref(true);
const sidebarWidth = 248;
const isExperimental = computed(() => accountStore.account?.isExperimental);

const handleLogout = async () => {
  try {
    await accountStore.logout();
    navigateTo("/auth/login");
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to logout",
      color: "error",
    });
  }
};

const mainItems = ref<NavigationMenuItem[][]>([
  [
    {
      label: "Home",
      icon: "i-lucide-home",
      defaultOpen: true,
      to: "/",
    },
    {
      label: "Account",
      icon: "i-lucide-user",
      defaultOpen: true,
      to: "/account",
    },
    {
      label: "Explore",
      icon: "i-lucide-search",
      to: "/explore",
      badge: {
        icon: 'i-lucide-flask-conical',
        label: 'Beta',
        variant: 'subtle',
        color: 'warning'
      }
    },
    {
      label: "Library",
      icon: "i-lucide-gamepad-2",
      to: '/library',
      children: [
        {
          label: "Pokemon ZA",
          icon: "i-lucide-gamepad",
        },
        {
          label: "Monster Hunter Rise",
          icon: "i-lucide-gamepad",
        },
        {
          label: "Celeste",
          icon: "i-lucide-gamepad",
        },
        {
          label: "Pokemon Legends: Arceus",
          icon: "i-lucide-gamepad",
        },
        {
          label: "The Witcher 3: Wild Hunt",
          icon: "i-lucide-gamepad",
        },
      ],
    },
    {
      label: "Cross Lan Play",
      icon: "i-lucide-globe",
      // to: "/lan-play",
      badge: {
        icon: 'i-lucide-wrench',
        label: 'Soon',
        variant: 'subtle',
        color: 'warning'
      }
    },
    {
      label: "Settings",
      icon: "i-lucide-cog",
      children: [
        {
          label: "General",
          icon: "i-lucide-dot",
          to: "/settings/general",
        },
        {
          label: "Source",
          icon: "i-lucide-dot",
          to: "/settings/source",
        },
      ],
    },
    {
      label: "Donate",
      icon: "i-lucide-coffee",
      to: "/donate"
    },
  ],
]);
if (accountStore.account && accountStore.account.isAdmin) {
  mainItems.value[0]?.push({
    label: "Administrator Tool",
    icon: "i-lucide-user-star",
    children: [
      {
        label: "Announcements",
        icon: "i-lucide-newspaper",
        to: "/admin/announcements",
      },
      {
        label: "Game Manager",
        icon: "i-lucide-gamepad",
        to: "/admin/games",
      },
      {
        label: "User Manager",
        icon: "i-lucide-users",
        to: "/admin/users",
      },
    ],
  })
}

const bottomItems = computed<NavigationMenuItem[][]>(() => [
  isExperimental.value
    ? [
        {
          label: "Experimental",
          icon: "i-lucide-flask-conical",
          badge: {
            label: "Enabled",
            color: "primary",
            variant: "subtle",
          },
        },
        {
          label: "Version 1.0.0",
          icon: "i-lucide-rocket",
          badge: "Update",
        },
      ]
    : [
        {
          label: `Version ${globalStore.settings.version}`,
          icon: "i-lucide-rocket",
          badge: "Update",
        },
      ],
]);
</script>

<template>
  <aside
    :class="[
      'fixed left-0 top-[30px] h-[calc(100vh-30px)] z-40 transition-transform duration-300 ease-in-out bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800',
      sidebarOpen ? 'translate-x-0' : '-translate-x-full',
    ]"
    :style="{ width: `${sidebarWidth}px` }"
  >
    <div class="h-full flex flex-col p-4">
      <div class="flex-shrink-0">
        <ProfileBanner />
      </div>

      <div class="flex-1 overflow-y-auto min-h-0 shard-scroll">
        <UNavigationMenu
          orientation="vertical"
          :items="mainItems"
          class="data-[orientation=vertical]:w-full"
        />
      </div>

      <TransferProtocolIndicator />

      <div
        class="flex-shrink-0 border-t border-gray-200 dark:border-gray-800 pt-4 mt-4"
      >
        <UNavigationMenu
          orientation="vertical"
          :items="bottomItems"
          class="data-[orientation=vertical]:w-full"
        />

        <UButton
          @click="handleLogout"
          color="neutral"
          variant="ghost"
          icon="i-lucide-log-out"
          class="w-full justify-start mt-2 cursor-pointer"
          size="md"
        >
          Logout
        </UButton>
      </div>
    </div>
  </aside>
</template>
