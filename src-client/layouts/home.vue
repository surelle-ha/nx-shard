<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const sidebarOpen = ref(true);
const sidebarWidth = 248;

const footerItems: NavigationMenuItem[] = [
  {
    label: "Figma Kit",
    to: "https://go.nuxt.com/figma-ui",
    target: "_blank",
  },
  {
    label: "Playground",
    to: "https://stackblitz.com/edit/nuxt-ui",
    target: "_blank",
  },
  {
    label: "Releases",
    to: "https://github.com/nuxt/ui/releases",
    target: "_blank",
  },
];
</script>

<template>
  <div class="flex h-screen">
    <NavigationBar />

    <div
      class="flex-1 overflow-auto transition-all duration-300 ease-in-out shard-scroll"
      :style="{ marginLeft: sidebarOpen ? `${sidebarWidth}px` : '0' }"
    >
      <slot />

      <!-- <UFooter>
        <template #left>
          <p class="text-muted text-sm">
            Copyright © {{ new Date().getFullYear() }}
          </p>
        </template>
        <UNavigationMenu :items="footerItems" variant="link" />
        <template #right>
          <UButton
            icon="i-simple-icons-github"
            color="neutral"
            variant="ghost"
            to="https://github.com/surelle-ha"
            target="_blank"
            aria-label="GitHub"
          />
        </template>
      </UFooter> -->
    </div>
  </div>
</template>
