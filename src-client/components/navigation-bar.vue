<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const sidebarOpen = ref(true);
const sidebarWidth = 248;

const mainItems = ref<NavigationMenuItem[][]>([
  [
    {
      label: "Explore",
      description: "Fully styled and customizable components for Nuxt.",
      icon: "i-lucide-search",
      to: "/",
    },
    {
      label: "Library",
      icon: "i-lucide-gamepad-2",
      active: false,
      defaultOpen: false,
      children: [
        {
          label: "Pokemon ZA",
          icon: "i-lucide-gamepad",
          description: "Display a modal within your application."
        },
        {
          label: "Monster Hunter Rise",
          icon: "i-lucide-gamepad",
          description: "Display a modal within your application."
        },
        {
          label: "Celeste",
          icon: "i-lucide-gamepad",
          description: "Display a modal within your application."
        },
        {
          label: "Pokemon Legends: Arceus",
          icon: "i-lucide-gamepad",
          description: "Display a modal within your application."
        },
        {
          label: "The Witcher 3: Wild Hunt",
          icon: "i-lucide-gamepad",
          description: "Display a modal within your application."
        }
      ],
    },
    {
      label: "Settings",
      icon: "i-lucide-cog",
      children: [
      {
          label: "General",
          icon: "i-lucide-dot",
          description: "Use NuxtLink with superpowers.",
          to: "/settings/general",
        },
        {
          label: "Appearance",
          icon: "i-lucide-dot",
          description: "Display a modal within your application.",
          to: "/settings/appearance",
        },
        {
          label: "Source",
          icon: "i-lucide-dot",
          description: "Display a list of links.",
          to: "/settings/source",
        },
      ],
    },
    {
      label: "Administrator Tool",
      icon: "i-lucide-user-star",
      children: [
      {
          label: "Game Manager",
          icon: "i-lucide-plus",
          description: "Use NuxtLink with superpowers.",
          to: "/docs/components/link",
        }
      ],
    },
  ],
]);

const bottomItems = ref<NavigationMenuItem[][]>([
  [
    {
      label: "Development Mode",
      icon: "i-lucide-flask-conical",
      badge: "On",
    },
    {
      label: "Version 1.0.0",
      icon: "i-lucide-rocket",
      badge: "Update",
    },
  ],
]);
const method = ref([
  {
    label: 'Install with MTP',
    icon: 'i-lucide-usb',
    to: '/docs/getting-started'
  },
  {
    label: 'Install with FTP',
    icon: 'i-lucide-wifi',
    to: '/docs/components'
  }
])
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

      <div class="flex-1 overflow-y-auto min-h-0">
        <UNavigationMenu
          orientation="vertical"
          :items="mainItems"
          class="data-[orientation=vertical]:w-full"
        />
      </div>

      <UPageAnchors :links="method" />

      <div class="flex-shrink-0 border-t border-gray-200 dark:border-gray-800 pt-4 mt-4">
        <UNavigationMenu
          orientation="vertical"
          :items="bottomItems"
          class="data-[orientation=vertical]:w-full"
        />
      </div>
    </div>
  </aside>
</template>
