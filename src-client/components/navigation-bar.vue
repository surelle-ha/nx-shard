<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { getVersion } from "@tauri-apps/api/app";

const update = await check();
const accountStore = useAccountStore();
const globalStore = useGlobalStore();
const toast = useToast();

const sidebarOpen = ref(true);
const sidebarWidth = 248;
const isExperimental = computed(() => accountStore.account?.isExperimental);
const appVersion = await getVersion();

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

const handleUpdate = async () => {
  console.log("Update Handled");
  if (update) {
    console.log(
      `found update ${update.version} from ${update.date} with notes ${update.body}`
    );
    let downloaded = 0;
    let contentLength = 0;
    // alternatively we could also call update.download() and update.install() separately
    await update.downloadAndInstall((event: any) => {
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength;
          console.log(`started downloading ${event.data.contentLength} bytes`);
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          console.log(`downloaded ${downloaded} from ${contentLength}`);
          break;
        case "Finished":
          console.log("download finished");
          break;
      }
    });

    console.log("update installed");
    await relaunch();
  } else {
    console.error(`No update found.`);
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
      label: "News",
      icon: "i-lucide-newspaper",
      to: "/news",
    },
    {
      label: "Explore",
      icon: "i-lucide-search",
      to: "/explore",
      badge: {
        icon: "i-lucide-flask-conical",
        label: "Beta",
        variant: "subtle",
        color: "warning",
      },
    },
    {
      label: "Library",
      icon: "i-lucide-gamepad-2",
      to: "/library",
    },
    {
      label: "Watch Anime",
      icon: "i-lucide-tv",
      to: "/watch-anime",
      badge: {
        icon: "i-lucide-flask-conical",
        label: "Beta",
        variant: "subtle",
        color: "warning",
      },
    },
    {
      label: "Cross Lan Play",
      icon: "i-lucide-globe",
      to: "/lan-play",
      badge: {
        icon: "i-lucide-wrench",
        label: "Soon",
        variant: "subtle",
        color: "warning",
      },
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
      ],
    },
    {
      label: "Help & Support",
      icon: "i-lucide-book",
      to: "/help",
    },
    {
      label: "Feature List",
      icon: "i-lucide-book",
      to: "/help",
      enable: false,
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
        label: "Developer Tools",
        icon: "i-lucide-cog",
        to: "/admin/dev-tool",
      },
      {
        label: "User Manager",
        icon: "i-lucide-users",
        to: "/admin/users",
      },
    ],
  });
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
      ]
    : [],
]);
</script>

<template>
  <aside
    :class="[
      'fixed left-0 top-[30px] h-[calc(100vh-30px)] z-40 transition-transform duration-300 ease-in-out border-r border-gray-200 dark:border-gray-800',
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

        <ChatSystem />

        <UButton
          @click="handleUpdate"
          color="neutral"
          variant="ghost"
          icon="i-lucide-rocket"
          class="w-full justify-start cursor-pointer group relative w-full flex items-center gap-1.5 font-medium text-sm before:absolute before:z-[-1] before:rounded-md focus:outline-none focus-visible:outline-none dark:focus-visible:outline-none focus-visible:before:ring-inset focus-visible:before:ring-2 focus-visible:before:ring-primary flex-row px-2.5 py-1.5 before:inset-y-px before:inset-x-0 text-muted hover:text-highlighted hover:before:bg-elevated/50 transition-colors before:transition-colors"
          size="md"
        >
          Version {{ appVersion }}
          <UBadge
            v-if="update"
            :label="`${update?.version} Available`"
            variant="subtle"
            size="sm"
          />
        </UButton>

        <UButton
          @click="handleLogout"
          color="neutral"
          variant="ghost"
          icon="i-lucide-log-out"
          class="w-full justify-start cursor-pointer group relative w-full flex items-center gap-1.5 font-medium text-sm before:absolute before:z-[-1] before:rounded-md focus:outline-none focus-visible:outline-none dark:focus-visible:outline-none focus-visible:before:ring-inset focus-visible:before:ring-2 focus-visible:before:ring-primary flex-row px-2.5 py-1.5 before:inset-y-px before:inset-x-0 text-muted hover:text-highlighted hover:before:bg-elevated/50 transition-colors before:transition-colors"
          size="md"
        >
          Logout
        </UButton>
      </div>
    </div>
  </aside>
</template>
