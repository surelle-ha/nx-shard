<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

definePageMeta({
  layout: "home",
});

const pageMeta = {
  header: {
    name: "SHARD ANIME",
    description: "Game still downloading? • Chill and Relax here.",
  },
  showHeader: true,
};

type SpotlightAnime = {
  title: string;
  alternativeTitle: string;
  id: string;
  poster: string;
  episodes: {
    sub: number;
    dub: number;
    eps: number;
  };
  rank: number;
  type: string;
  quality: string;
  duration: string;
  aired: string;
  synopsis: string;
};

type ApiResponse = {
  success: boolean;
  data: {
    spotlight: SpotlightAnime[];
  };
};

const isLoading = ref(true);
const spotlightItems = ref<SpotlightAnime[]>([]);
const currentSlide = ref(0);

const footerItems: NavigationMenuItem[] = [
  {
    label: "Search",
    to: "/watch-anime/search",
  }
];

onMounted(async () => {
  try {
    const res = await fetch("http://localhost:3030/api/v1/home");
    const data: ApiResponse = await res.json();

    if (data.success && data.data.spotlight) {
      spotlightItems.value = data.data.spotlight;
    }
  } catch (error) {
    console.error("Failed to load spotlight anime:", error);
  } finally {
    isLoading.value = false;
  }
});

// Auto-advance spotlight
let intervalId: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
  intervalId = setInterval(() => {
    if (spotlightItems.value.length > 0) {
      currentSlide.value = (currentSlide.value + 1) % spotlightItems.value.length;
    }
  }, 5000);
});

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId);
});

const formatEpisodeCount = (episodes: { sub: number; dub: number; eps: number }) => {
  const parts = [];
  if (episodes.sub) parts.push(`${episodes.sub} SUB`);
  if (episodes.dub) parts.push(`${episodes.dub} DUB`);
  return parts.join(" • ");
};
</script>

<template>
  <div class="min-h-screen p-10">
    <div class="max-w-7xl mx-auto">
      <!-- Header - Matching library page style -->
      <div class="mb-8">
        <USkeleton v-if="isLoading" class="h-12 w-64 mb-2" />
        <div v-else class="flex items-center justify-between mb-2">
          <div class="flex items-center gap-3">
            <div class="p-3 bg-primary-500/10 rounded-xl">
              <UIcon name="i-heroicons-play-circle" class="w-8 h-8 text-primary-500" />
            </div>
            <h1 class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text">
              {{ pageMeta.header.name }}
            </h1>
          </div>

          <nav class="flex items-center gap-4">
            <UButton v-for="item in footerItems" :key="item.label" :to="item.to" variant="ghost" size="sm">
              {{ item.label }}
            </UButton>
          </nav>
        </div>

        <USkeleton v-if="isLoading" class="h-6 w-96" />
        <p v-else class="text-gray-600 dark:text-gray-400 text-lg ml-14">
          {{ pageMeta.header.description }}
        </p>
      </div>

      <!-- Main spotlight section -->
      <section class="spotlight-section">
        <div v-if="isLoading" class="loading-state">
          <div class="loader">
            <div class="loader-ring"></div>
            <div class="loader-text">LOADING</div>
          </div>
        </div>

        <div v-else class="spotlight-container">
          <!-- Featured anime display -->
          <div class="featured-anime" v-for="(item, index) in spotlightItems" :key="item.id"
            :class="{ active: index === currentSlide }" :style="{ '--delay': index * 0.1 + 's' }">

            <!-- Background poster with overlay -->
            <div class="poster-background">
              <img :src="item.poster" :alt="item.title" />
              <div class="poster-overlay"></div>
            </div>

            <!-- Content -->
            <div class="anime-content">
              <div class="rank-badge">
                <UBadge :label="`#${item.rank}`" color="error" variant="solid" size="lg" />
                <UBadge label="TRENDING" color="primary" variant="soft" />
              </div>

              <h1 class="anime-title">{{ item.title }}</h1>
              <p class="anime-alt-title">{{ item.alternativeTitle }}</p>

              <div class="meta-info">
                <UBadge :label="item.quality" color="primary" variant="solid" />
                <span class="text-gray-600 dark:text-gray-400">•</span>
                <span class="text-sm text-gray-700 dark:text-gray-300">{{ item.type }}</span>
                <span class="text-gray-600 dark:text-gray-400">•</span>
                <span class="text-sm text-gray-700 dark:text-gray-300">{{ item.duration }}</span>
                <span class="text-gray-600 dark:text-gray-400">•</span>
                <span class="text-sm text-gray-700 dark:text-gray-300">{{ item.aired }}</span>
              </div>

              <p class="text-sm text-gray-600 dark:text-gray-400 font-medium">
                {{ formatEpisodeCount(item.episodes) }} • {{ item.episodes.eps }} EPISODES
              </p>

              <p class="synopsis">{{ item.synopsis }}</p>

              <div class="action-buttons">
                <UButton :to="`watch-anime/anime/${item.id}`" color="primary" size="lg" icon="i-heroicons-play">
                  Watch Now
                </UButton>
              </div>
            </div>
          </div>

          <!-- Carousel indicators -->
          <div class="carousel-indicators">
            <button v-for="(item, index) in spotlightItems" :key="index" @click="currentSlide = index" class="indicator"
              :class="{ active: index === currentSlide }">
              <span class="indicator-line"></span>
            </button>
          </div>
        </div>
      </section>

      <!-- Carousel Section - More Anime -->
      <section class="mt-12">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Spotlight
          </h2>
          <UButton variant="ghost" trailing-icon="i-heroicons-arrow-right">
            View All
          </UButton>
        </div>

        <USkeleton v-if="isLoading" class="h-[300px] w-full rounded-lg" />

        <div v-else class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          <NuxtLink v-for="item in spotlightItems.slice(0, 10)" :key="item.id" :to="`watch-anime/anime/${item.id}`"
            class="group cursor-pointer">
            <div
              class="relative overflow-hidden rounded-lg shadow-lg transition-transform duration-300 hover:scale-105">
              <img :src="item.poster" :alt="item.title" class="h-[300px] w-full object-cover" loading="lazy" />

              <!-- Quality badge -->
              <div class="absolute top-2 left-2">
                <UBadge :label="item.quality" color="primary" variant="solid" />
              </div>

              <!-- Rank badge -->
              <div class="absolute top-2 right-2">
                <UBadge :label="`#${item.rank}`" color="error" variant="solid" />
              </div>

              <!-- Overlay on hover -->
              <div
                class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/70 to-transparent p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                <h3 class="text-white font-semibold text-sm line-clamp-2 mb-1">
                  {{ item.title }}
                </h3>
                <p class="text-xs text-gray-300">
                  {{ item.type }} • {{ item.duration }}
                </p>
                <div class="flex items-center gap-2 mt-2">
                  <UButton size="xs" color="primary">
                    Watch
                  </UButton>
                  <UButton size="xs" variant="ghost" color="neutral" icon="i-heroicons-plus">
                  </UButton>
                </div>
              </div>
            </div>

            <!-- Title below image -->
            <div class="mt-2">
              <p class="text-sm font-medium text-gray-900 dark:text-white line-clamp-1">
                {{ item.title }}
              </p>
              <p class="text-xs text-gray-600 dark:text-gray-400">
                {{ formatEpisodeCount(item.episodes) }}
              </p>
            </div>
          </NuxtLink>
        </div>
      </section>

      <section class="mt-12">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Trending Now
          </h2>
          <UButton variant="ghost" trailing-icon="i-heroicons-arrow-right">
            View All
          </UButton>
        </div>

        <USkeleton v-if="isLoading" class="h-[300px] w-full rounded-lg" />

        <div v-else class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          <NuxtLink v-for="item in spotlightItems.slice(0, 10)" :key="item.id" :to="`watch-anime/anime/${item.id}`"
            class="group cursor-pointer">
            <div
              class="relative overflow-hidden rounded-lg shadow-lg transition-transform duration-300 hover:scale-105">
              <img :src="item.poster" :alt="item.title" class="h-[300px] w-full object-cover" loading="lazy" />

              <!-- Quality badge -->
              <div class="absolute top-2 left-2">
                <UBadge :label="item.quality" color="primary" variant="solid" />
              </div>

              <!-- Rank badge -->
              <div class="absolute top-2 right-2">
                <UBadge :label="`#${item.rank}`" color="error" variant="solid" />
              </div>

              <!-- Overlay on hover -->
              <div
                class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/70 to-transparent p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                <h3 class="text-white font-semibold text-sm line-clamp-2 mb-1">
                  {{ item.title }}
                </h3>
                <p class="text-xs text-gray-300">
                  {{ item.type }} • {{ item.duration }}
                </p>
                <div class="flex items-center gap-2 mt-2">
                  <UButton size="xs" color="primary">
                    Watch
                  </UButton>
                  <UButton size="xs" variant="ghost" color="neutral" icon="i-heroicons-plus">
                  </UButton>
                </div>
              </div>
            </div>

            <!-- Title below image -->
            <div class="mt-2">
              <p class="text-sm font-medium text-gray-900 dark:text-white line-clamp-1">
                {{ item.title }}
              </p>
              <p class="text-xs text-gray-600 dark:text-gray-400">
                {{ formatEpisodeCount(item.episodes) }}
              </p>
            </div>
          </NuxtLink>
        </div>
      </section>

      <section class="mt-12">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Top Airing
          </h2>
          <UButton variant="ghost" trailing-icon="i-heroicons-arrow-right">
            View All
          </UButton>
        </div>

        <USkeleton v-if="isLoading" class="h-[300px] w-full rounded-lg" />

        <div v-else class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          <NuxtLink v-for="item in spotlightItems.slice(0, 10)" :key="item.id" :to="`watch-anime/anime/${item.id}`"
            class="group cursor-pointer">
            <div
              class="relative overflow-hidden rounded-lg shadow-lg transition-transform duration-300 hover:scale-105">
              <img :src="item.poster" :alt="item.title" class="h-[300px] w-full object-cover" loading="lazy" />

              <!-- Quality badge -->
              <div class="absolute top-2 left-2">
                <UBadge :label="item.quality" color="primary" variant="solid" />
              </div>

              <!-- Rank badge -->
              <div class="absolute top-2 right-2">
                <UBadge :label="`#${item.rank}`" color="error" variant="solid" />
              </div>

              <!-- Overlay on hover -->
              <div
                class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/70 to-transparent p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                <h3 class="text-white font-semibold text-sm line-clamp-2 mb-1">
                  {{ item.title }}
                </h3>
                <p class="text-xs text-gray-300">
                  {{ item.type }} • {{ item.duration }}
                </p>
                <div class="flex items-center gap-2 mt-2">
                  <UButton size="xs" color="primary">
                    Watch
                  </UButton>
                  <UButton size="xs" variant="ghost" color="neutral" icon="i-heroicons-plus">
                  </UButton>
                </div>
              </div>
            </div>

            <!-- Title below image -->
            <div class="mt-2">
              <p class="text-sm font-medium text-gray-900 dark:text-white line-clamp-1">
                {{ item.title }}
              </p>
              <p class="text-xs text-gray-600 dark:text-gray-400">
                {{ formatEpisodeCount(item.episodes) }}
              </p>
            </div>
          </NuxtLink>
        </div>
      </section>
    </div>
  </div>

  <!-- Decorative scan lines -->
  <div class="scanlines"></div>
</template>

<style scoped>
/* Minimal scan lines effect - subtle */
.scanlines {
  position: fixed;
  inset: 0;
  background: linear-gradient(to bottom,
      transparent 50%,
      rgba(0, 0, 0, 0.01) 51%);
  background-size: 100% 4px;
  pointer-events: none;
  z-index: 100;
  opacity: 0.3;
}

/* Spotlight section */
.spotlight-section {
  position: relative;
  margin-bottom: 2rem;
}

/* Loading state */
.loading-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 50vh;
}

.loader {
  position: relative;
  width: 60px;
  height: 60px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.loader-ring {
  position: absolute;
  width: 100%;
  height: 100%;
  border: 3px solid transparent;
  border-top-color: rgb(var(--color-primary-500));
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.loader-text {
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.2em;
  color: rgb(var(--color-primary-500));
}

/* Spotlight container */
.spotlight-container {
  position: relative;
  border-radius: 0.5rem;
  overflow: hidden;
}

.featured-anime {
  position: absolute;
  inset: 0;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.8s ease, visibility 0.8s ease;
  display: flex;
  align-items: center;
  min-height: 60vh;
}

.featured-anime.active {
  opacity: 1;
  visibility: visible;
  position: relative;
}

.featured-anime.active .anime-content {
  animation: slideIn 0.6s ease-out forwards;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.poster-background {
  position: absolute;
  top: 0;
  right: 0;
  width: 60%;
  height: 100%;
  overflow: hidden;
  border-radius: 0.5rem;
}

.poster-background img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.poster-overlay {
  position: absolute;
  inset: 0;
  background: linear-gradient(to right,
      rgb(var(--color-gray-50)) 0%,
      rgba(var(--color-gray-50), 0.95) 30%,
      transparent 100%);
}

@media (prefers-color-scheme: dark) {
  .poster-overlay {
    background: linear-gradient(to right,
        rgb(var(--color-gray-950)) 0%,
        rgba(var(--color-gray-950), 0.95) 30%,
        transparent 100%);
  }
}

.anime-content {
  position: relative;
  z-index: 10;
  max-width: 50%;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.rank-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  width: fit-content;
}

.anime-title {
  font-size: 2.5rem;
  font-weight: 900;
  line-height: 1.1;
  color: rgb(var(--color-gray-900));
  margin: 0;
}

@media (prefers-color-scheme: dark) {
  .anime-title {
    color: rgb(var(--color-gray-50));
  }
}

.anime-alt-title {
  font-size: 1rem;
  color: rgb(var(--color-gray-600));
  font-weight: 400;
  margin: -0.5rem 0 0 0;
}

@media (prefers-color-scheme: dark) {
  .anime-alt-title {
    color: rgb(var(--color-gray-400));
  }
}

.meta-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.synopsis {
  font-size: 0.95rem;
  line-height: 1.6;
  color: rgb(var(--color-gray-700));
  max-width: 600px;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@media (prefers-color-scheme: dark) {
  .synopsis {
    color: rgb(var(--color-gray-300));
  }
}

.action-buttons {
  display: flex;
  gap: 0.75rem;
  margin-top: 0.5rem;
}

/* Carousel indicators */
.carousel-indicators {
  position: absolute;
  bottom: 1.5rem;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 0.5rem;
  z-index: 20;
}

.indicator {
  background: transparent;
  border: none;
  padding: 0;
  cursor: pointer;
  position: relative;
}

.indicator-line {
  display: block;
  width: 40px;
  height: 3px;
  background: rgba(var(--color-gray-400), 0.3);
  position: relative;
  overflow: hidden;
  border-radius: 2px;
  transition: background 0.2s;
}

.indicator:hover .indicator-line {
  background: rgba(var(--color-gray-400), 0.5);
}

.indicator-line::before {
  content: '';
  position: absolute;
  inset: 0;
  background: rgb(var(--color-primary-500));
  transform: translateX(-100%);
  transition: transform 0.3s ease;
}

.indicator.active .indicator-line {
  background: rgba(var(--color-primary-500), 0.3);
}

.indicator.active .indicator-line::before {
  transform: translateX(0);
  animation: indicatorProgress 5s linear;
}

@keyframes indicatorProgress {
  from {
    transform: translateX(-100%);
  }

  to {
    transform: translateX(0);
  }
}

/* Responsive design */
@media (max-width: 1024px) {
  .anime-content {
    max-width: 55%;
  }

  .poster-background {
    width: 65%;
  }

  .anime-title {
    font-size: 2rem;
  }
}

@media (max-width: 768px) {
  .featured-anime {
    flex-direction: column;
    min-height: auto;
  }

  .poster-background {
    position: relative;
    width: 100%;
    height: 300px;
    margin-bottom: 1.5rem;
  }

  .poster-overlay {
    background: linear-gradient(to bottom,
        transparent 0%,
        rgba(var(--color-gray-50), 0.95) 70%,
        rgb(var(--color-gray-50)) 100%);
  }

  @media (prefers-color-scheme: dark) {
    .poster-overlay {
      background: linear-gradient(to bottom,
          transparent 0%,
          rgba(var(--color-gray-950), 0.95) 70%,
          rgb(var(--color-gray-950)) 100%);
    }
  }

  .anime-content {
    max-width: 100%;
    padding: 1rem;
  }

  .anime-title {
    font-size: 1.75rem;
  }

  .synopsis {
    font-size: 0.85rem;
    -webkit-line-clamp: 2;
    line-clamp: 2;
  }

  .action-buttons {
    flex-direction: column;
  }
}
</style>