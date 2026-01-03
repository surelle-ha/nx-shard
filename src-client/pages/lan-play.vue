<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

definePageMeta({
  layout: "home",
});

const pageMeta = {
  header: {
    name: "👾 Cross Lan Play",
    description: "This requires `ldn_mitm` plugin on your Switch to work.",
  },
  showHeader: true,
};

type SpotlightAnime = {
  id: string;
  title: string;
  japaneseTitle: string;
  banner: string;
  url: string;
  type: string;
  genres: string[];
  releaseDate: string;
  quality: string;
  sub: number;
  dub: number;
  description: string;
};

const isLoading = ref(true);
const animeItems = ref<SpotlightAnime[]>([]);

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

onMounted(async () => {
  try {
    const res = await fetch(
      "https://api-consumet-lime.vercel.app/anime/animekai/spotlight"
    );
    const data = await res.json();
    animeItems.value = data.results ?? [];
  } catch (error) {
    console.error("Failed to load spotlight anime:", error);
  } finally {
    isLoading.value = false;
  }
});
</script>

<template>
    <div class="h-full p-4 mt-6">
      <!-- Header -->
      <div class="mt-4">
        <div v-show="pageMeta.showHeader">
          <USkeleton v-if="isLoading" class="h-8 w-50" />
          <h1 v-else class="text-2xl font-bold">
            {{ pageMeta.header.name }}
          </h1>

          <USkeleton v-if="isLoading" class="mt-2 h-6 w-100" />
          <p v-else class="mt-2 text-gray-600 dark:text-gray-400">
            {{ pageMeta.header.description }}
          </p>
        </div>
      </div>

      <!-- Carousel -->
      <div class="p-6">
        <USkeleton v-if="isLoading" class="h-[340px] w-full rounded-lg" />

        <UCarousel
          v-else
          v-slot="{ item }"
          loop
          arrows
          :autoplay="{ delay: 2500 }"
          wheel-gestures
          :prev="{ variant: 'solid' }"
          :next="{ variant: 'solid' }"
          :items="animeItems"
          :ui="{
            item: 'basis-1/3 ps-0',
            prev: 'sm:start-8',
            next: 'sm:end-8',
            container: 'ms-0',
          }"
        >
          <div class="relative">
            <img
              :src="item.banner"
              :alt="item.title"
              class="h-[320px] w-full object-cover rounded-lg"
              loading="lazy"
            />

            <!-- Overlay -->
            <div
              class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 to-transparent p-3 rounded-b-lg"
            >
              <h3 class="text-white font-semibold text-sm line-clamp-1">
                {{ item.title }}
              </h3>
              <p class="text-xs text-gray-300">
                {{ item.type }} • {{ item.releaseDate }}
              </p>
            </div>
          </div>
        </UCarousel>
      </div>
    </div>
</template>
