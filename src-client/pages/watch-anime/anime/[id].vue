<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

definePageMeta({
    layout: "home",
});

const route = useRoute();
const animeId = route.params.id as string;

type AnimeDetails = {
    title: string;
    alternativeTitle: string;
    id: string;
    poster: string;
    episodes: {
        sub: number;
        dub: number;
        eps: number;
    };
    rating?: string;
    type: string;
    is18Plus: boolean;
    synopsis: string;
    synonyms?: string;
    aired?: {
        from: string;
        to: string | null;
    };
    premiered?: string;
    duration: string;
    status?: string;
    MAL_score?: string;
    genres?: string[];
    studios?: string[];
    producers?: string[];
    japanese?: string;
};

type RecommendedAnime = {
    title: string;
    alternativeTitle: string;
    id: string;
    poster: string;
    type: string;
    duration?: string;
    episodes: {
        sub: number;
        dub: number;
        eps: number;
    };
    is18Plus: boolean;
};

type Episode = {
    id: string;
    episodeNumber: number;
    title: string;
    alternativeTitle?: string;
    isFiller?: boolean;
};

type ApiResponse = {
    success: boolean;
    data: AnimeDetails;
};

type EpisodesApiResponse = {
    success: boolean;
    data: Episode[];
};

const isLoading = ref(true);
const isLoadingEpisodes = ref(true);
const animeData = ref<AnimeDetails | null>(null);
const episodes = ref<Episode[]>([]);
const recommendations = ref<RecommendedAnime[]>([]);
const mostPopular = ref<RecommendedAnime[]>([]);
const activeTab = ref<'overview' | 'episodes' | 'recommendations'>('overview');

const footerItems: NavigationMenuItem[] = [
    {
        label: "Browse",
        to: "/browse",
    },
    {
        label: "Schedule",
        to: "/schedule",
    },
    {
        label: "Rankings",
        to: "/rankings",
    },
];

onMounted(async () => {
    try {
        // Fetch anime details
        const res = await fetch(`http://localhost:3030/api/v1/anime/${animeId}`);
        const data: ApiResponse = await res.json();

        if (data.success && data.data) {
            animeData.value = data.data;

            // Use recommended or mostPopular for recommendations
            if (Array.isArray((data.data as any).recommended) && (data.data as any).recommended.length > 0) {
                recommendations.value = (data.data as any).recommended;
            } else if (Array.isArray((data.data as any).mostPopular) && (data.data as any).mostPopular.length > 0) {
                recommendations.value = (data.data as any).mostPopular;
            }

        }
    } catch (error) {
        console.error("Failed to load anime details:", error);
    } finally {
        isLoading.value = false;
    }

    // Fetch episodes separately
    try {
        console.log(animeId)
        const episodesRes = await fetch(`http://localhost:3030/api/v1/episodes/${animeId}`);
        console.log(episodesRes)
        const episodesData: EpisodesApiResponse = await episodesRes.json();
        console.log(episodesData)

        if (episodesData.success && Array.isArray(episodesData.data)) {
            episodes.value = episodesData.data;
        }
    } catch (error) {
        console.error("Failed to load episodes:", error);
    } finally {
        isLoadingEpisodes.value = false;
    }
});

const formatEpisodeCount = (episodes: { sub: number; dub: number; eps: number }) => {
    const parts = [];
    if (episodes.sub) parts.push(`${episodes.sub} SUB`);
    if (episodes.dub) parts.push(`${episodes.dub} DUB`);
    return parts.join(" • ");
};

const scrollToSection = (section: string) => {
    const element = document.getElementById(section);
    if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
};

const watchEpisode = (episodeNumber: number = 1) => {
    navigateTo({
        path: '/watch-anime/watch',
        query: {
            anime: animeId,
            ep: episodeNumber
        }
    });
};
</script>

<template>
    <div class="min-h-screen">
        <!-- Hero Section with Background -->
        <div class="hero-section">
            <div v-if="isLoading" class="loading-hero">
                <div class="loader">
                    <div class="loader-ring"></div>
                    <div class="loader-text">LOADING</div>
                </div>
            </div>

            <div v-else-if="animeData" class="hero-container">
                <!-- Background Image -->
                <div class="hero-background">
                    <img :src="animeData.poster" :alt="animeData.title" />
                    <div class="hero-overlay"></div>
                    <div class="hero-gradient"></div>
                </div>

                <!-- Hero Content -->
                <div class="hero-content">
                    <div class="max-w-7xl mx-auto px-10 py-12">
                        <!-- Back Button -->
                        <UButton to="/watch-anime/" variant="ghost" icon="i-heroicons-arrow-left"
                            class="mb-8 backdrop-blur-sm bg-white/10 dark:bg-black/20 hover:bg-white/20 dark:hover:bg-black/30">
                            Back to Home
                        </UButton>

                        <div class="grid grid-cols-1 lg:grid-cols-12 gap-10">
                            <!-- Poster -->
                            <div class="lg:col-span-3">
                                <div class="poster-card group">
                                    <img :src="animeData.poster" :alt="animeData.title"
                                        class="w-full rounded-2xl shadow-2xl transition-transform duration-500 group-hover:scale-105" />
                                    <div class="absolute top-4 left-4">
                                        <UBadge v-if="animeData.status" :label="animeData.status" color="primary"
                                            variant="solid" size="lg" />
                                    </div>
                                    <div class="absolute top-4 right-4">
                                        <UBadge v-if="animeData.type" :label="animeData.type" color="neutral"
                                            variant="solid" size="lg" />
                                    </div>
                                </div>
                            </div>

                            <!-- Info -->
                            <div class="lg:col-span-9 flex flex-col justify-center">
                                <div class="space-y-4">
                                    <!-- Badges -->
                                    <div class="flex items-center gap-2 flex-wrap">
                                        <UBadge v-if="animeData.premiered" :label="animeData.premiered" color="primary"
                                            variant="soft" size="lg" />
                                        <UBadge v-if="animeData.status" :label="animeData.status" color="neutral"
                                            variant="outline" />
                                        <UBadge v-if="animeData.rating" :label="animeData.rating" color="neutral"
                                            variant="outline" />
                                        <UBadge v-if="animeData.is18Plus" label="18+" color="error" variant="soft" />
                                    </div>

                                    <!-- Title -->
                                    <div>
                                        <h1 class="anime-main-title">{{ animeData.title }}</h1>
                                        <p class="anime-sub-title">{{ animeData.alternativeTitle }}</p>
                                    </div>

                                    <!-- Meta Info -->
                                    <div class="meta-info-grid">
                                        <div class="meta-item">
                                            <UIcon name="i-heroicons-film" class="w-5 h-5 text-primary-500" />
                                            <span>{{ animeData.type }}</span>
                                        </div>
                                        <div class="meta-item">
                                            <UIcon name="i-heroicons-clock" class="w-5 h-5 text-primary-500" />
                                            <span>{{ animeData.duration }}</span>
                                        </div>
                                        <div v-if="animeData.aired" class="meta-item">
                                            <UIcon name="i-heroicons-calendar" class="w-5 h-5 text-primary-500" />
                                            <span>{{ animeData.aired.from }}</span>
                                        </div>
                                        <div class="meta-item">
                                            <UIcon name="i-heroicons-play-circle" class="w-5 h-5 text-primary-500" />
                                            <span>{{ formatEpisodeCount(animeData.episodes) }}</span>
                                        </div>
                                    </div>

                                    <!-- Genres -->
                                    <div v-if="animeData.genres && animeData.genres.length"
                                        class="flex flex-wrap gap-2">
                                        <UBadge v-for="genre in animeData.genres" :key="genre" :label="genre"
                                            variant="soft" color="neutral" />
                                    </div>

                                    <!-- Synopsis -->
                                    <p class="hero-synopsis">{{ animeData.synopsis }}</p>

                                    <!-- Action Buttons -->
                                    <div class="flex flex-wrap gap-3 pt-4">
                                        <UButton @click="watchEpisode(1)" color="primary" size="xl"
                                            icon="i-heroicons-play" class="action-btn-primary">
                                            Watch Episode 1
                                        </UButton>
                                        <UButton variant="outline" size="xl" icon="i-heroicons-plus"
                                            class="action-btn-secondary">
                                            Add to List
                                        </UButton>
                                        <UButton variant="ghost" size="xl" icon="i-heroicons-share"
                                            class="action-btn-ghost">
                                            Share
                                        </UButton>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Navigation Tabs -->
        <div
            class="sticky top-0 z-40 bg-white/80 dark:bg-gray-950/80 backdrop-blur-xl border-b border-gray-200 dark:border-gray-800">
            <div class="max-w-7xl mx-auto px-10">
                <nav class="flex gap-8 py-4">
                    <button @click="activeTab = 'overview'; scrollToSection('overview')" :class="[
                        'tab-button',
                        activeTab === 'overview' ? 'tab-active' : 'tab-inactive'
                    ]" class="cursor-pointer">
                        <UIcon name="i-heroicons-information-circle" class="w-5 h-5" />
                        Overview
                    </button>
                    <button @click="activeTab = 'episodes'; scrollToSection('episodes')" :class="[
                        'tab-button',
                        activeTab === 'episodes' ? 'tab-active' : 'tab-inactive'
                    ]" class="cursor-pointer">
                        <UIcon name="i-heroicons-play-circle" class="w-5 h-5" />
                        Episodes
                    </button>
                    <button @click="activeTab = 'recommendations'; scrollToSection('recommendations')" :class="[
                        'tab-button',
                        activeTab === 'recommendations' ? 'tab-active' : 'tab-inactive'
                    ]" class="cursor-pointer">
                        <UIcon name="i-heroicons-sparkles" class="w-5 h-5" />
                        Recommendations
                    </button>
                </nav>
            </div>
        </div>

        <!-- Content Sections -->
        <div class="max-w-7xl mx-auto px-10 py-12">
            <!-- Overview Section -->
            <section id="overview" class="content-section">
                <h2 class="section-title">
                    <UIcon name="i-heroicons-information-circle" class="w-7 h-7" />
                    Overview
                </h2>

                <div v-if="animeData" class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <!-- Main Info -->
                    <div class="lg:col-span-2 space-y-6">
                        <div class="info-card">
                            <h3 class="info-card-title">Synopsis</h3>
                            <p class="text-gray-700 dark:text-gray-300 leading-relaxed">
                                {{ animeData.synopsis }}
                            </p>
                        </div>

                        <!-- Additional Info -->
                        <div class="info-card">
                            <h3 class="info-card-title">Details</h3>
                            <div class="details-grid">
                                <div class="detail-item">
                                    <span class="detail-label">Type</span>
                                    <span class="detail-value">{{ animeData.type }}</span>
                                </div>
                                <div class="detail-item">
                                    <span class="detail-label">Episodes</span>
                                    <span class="detail-value">{{ animeData.episodes.eps }}</span>
                                </div>
                                <div class="detail-item">
                                    <span class="detail-label">Duration</span>
                                    <span class="detail-value">{{ animeData.duration }}</span>
                                </div>
                                <div v-if="animeData.aired" class="detail-item">
                                    <span class="detail-label">Aired</span>
                                    <span class="detail-value">{{ animeData.aired.from }}{{ animeData.aired.to ? ` -
                                        ${animeData.aired.to}` : ' - ?' }}</span>
                                </div>
                                <div v-if="animeData.premiered" class="detail-item">
                                    <span class="detail-label">Premiered</span>
                                    <span class="detail-value">{{ animeData.premiered }}</span>
                                </div>
                                <div v-if="animeData.status" class="detail-item">
                                    <span class="detail-label">Status</span>
                                    <span class="detail-value">{{ animeData.status }}</span>
                                </div>
                                <div v-if="animeData.rating" class="detail-item">
                                    <span class="detail-label">Rating</span>
                                    <span class="detail-value">{{ animeData.rating }}</span>
                                </div>
                                <div v-if="animeData.MAL_score" class="detail-item">
                                    <span class="detail-label">MAL Score</span>
                                    <span class="detail-value">{{ animeData.MAL_score }}</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Sidebar Info -->
                    <div class="space-y-6">
                        <div class="info-card">
                            <h3 class="info-card-title">Statistics</h3>
                            <div class="space-y-4">
                                <div v-if="animeData.MAL_score && animeData.MAL_score !== '?'" class="stat-item">
                                    <span class="stat-label">MAL Score</span>
                                    <div class="flex items-center gap-2">
                                        <UIcon name="i-heroicons-star-solid" class="w-5 h-5 text-yellow-500" />
                                        <span class="text-2xl font-bold text-gray-900 dark:text-white">
                                            {{ animeData.MAL_score }}
                                        </span>
                                    </div>
                                </div>
                                <div class="stat-item">
                                    <span class="stat-label">Status</span>
                                    <UBadge v-if="animeData.status" :label="animeData.status" color="primary"
                                        variant="solid" size="lg" />
                                </div>
                                <div class="stat-item">
                                    <span class="stat-label">Episodes</span>
                                    <span class="text-xl font-bold text-gray-900 dark:text-white">
                                        {{ animeData.episodes.eps }}
                                    </span>
                                </div>
                            </div>
                        </div>

                        <div v-if="animeData.studios && animeData.studios.length" class="info-card">
                            <h3 class="info-card-title">Studios</h3>
                            <div class="flex flex-wrap gap-2">
                                <UBadge v-for="studio in animeData.studios" :key="studio" :label="studio" variant="soft"
                                    color="neutral" />
                            </div>
                        </div>

                        <div v-if="animeData.producers && animeData.producers.length" class="info-card">
                            <h3 class="info-card-title">Producers</h3>
                            <div class="flex flex-wrap gap-2">
                                <UBadge v-for="producer in animeData.producers" :key="producer" :label="producer"
                                    variant="soft" color="neutral" />
                            </div>
                        </div>

                        <div v-if="animeData.japanese" class="info-card">
                            <h3 class="info-card-title">Japanese Title</h3>
                            <p class="text-gray-700 dark:text-gray-300 font-medium">
                                {{ animeData.japanese }}
                            </p>
                        </div>

                        <div v-if="animeData.synonyms" class="info-card">
                            <h3 class="info-card-title">Alternative Titles</h3>
                            <p class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
                                {{ animeData.synonyms }}
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Episodes Section -->
            <section id="episodes" class="content-section">
                <h2 class="section-title">
                    <UIcon name="i-heroicons-play-circle" class="w-7 h-7" />
                    Episodes
                </h2>

                <div v-if="isLoadingEpisodes" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <USkeleton v-for="i in 6" :key="i" class="h-32 rounded-lg" />
                </div>

                <div v-else-if="episodes.length > 0"
                    class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    <div v-for="episode in episodes" :key="episode.id" @click="watchEpisode(episode.episodeNumber)"
                        class="episode-card group cursor-pointer rounded-2xl border border-slate-700 bg-slate-900 p-4 shadow-sm hover:shadow-lg hover:border-primary-500 transition-all duration-200">
                        <div class="flex items-center gap-4">

                            <!-- Episode Number Badge -->
                            <div
                                class="episode-number flex h-10 w-10 items-center justify-center rounded-full bg-primary-500/10 text-primary-500 font-semibold text-sm">
                                {{ episode.episodeNumber }}
                            </div>

                            <!-- Episode Info -->
                            <div class="flex-1 min-w-0">
                                <p class="episode-title text-sm font-semibold text-white truncate">
                                    {{ episode.title || `Episode ${episode.episodeNumber}` }}
                                </p>
                                <p v-if="episode.isFiller"
                                    class="episode-meta mt-1 flex items-center gap-2 text-xs text-slate-300">
                                    <UIcon name="i-heroicons-exclamation-circle" class="w-4 h-4" />
                                    Filler
                                </p>
                            </div>

                            <!-- Play Icon -->
                            <UIcon name="i-heroicons-play-circle"
                                class="w-8 h-8 text-primary-500 opacity-0 group-hover:opacity-100 transition-opacity duration-200" />
                        </div>
                    </div>
                </div>


                <div v-else class="empty-state">
                    <UIcon name="i-heroicons-film" class="w-16 h-16 text-gray-400" />
                    <p class="text-gray-500 dark:text-gray-400 text-lg">No episodes available</p>
                </div>
            </section>

            <!-- Recommendations Section -->
            <section id="recommendations" class="content-section">
                <h2 class="section-title">
                    <UIcon name="i-heroicons-sparkles" class="w-7 h-7" />
                    You Might Also Like
                </h2>

                <div v-if="isLoading" class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
                    <USkeleton v-for="i in 5" :key="i" class="h-[300px] rounded-lg" />
                </div>

                <div v-else-if="recommendations.length > 0"
                    class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
                    <NuxtLink v-for="item in recommendations" :key="item.id" :to="`/watch-anime/anime/${item.id}`"
                        class="recommendation-card group">
                        <div
                            class="relative overflow-hidden rounded-lg shadow-lg transition-transform duration-300 hover:scale-105">
                            <img :src="item.poster" :alt="item.title" class="h-[300px] w-full object-cover"
                                loading="lazy" />

                            <!-- Type badge -->
                            <div class="absolute top-2 left-2">
                                <UBadge :label="item.type" color="primary" variant="solid" />
                            </div>

                            <!-- 18+ badge -->
                            <div v-if="item.is18Plus" class="absolute top-2 right-2">
                                <UBadge label="18+" color="error" variant="solid" />
                            </div>

                            <!-- Overlay on hover -->
                            <div
                                class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/70 to-transparent p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                                <h3 class="text-white font-semibold text-sm line-clamp-2 mb-1">
                                    {{ item.title }}
                                </h3>
                                <p class="text-xs text-gray-300">
                                    {{ item.type }}{{ item.duration ? ` • ${item.duration}` : '' }}
                                </p>
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

                <div v-else class="empty-state">
                    <UIcon name="i-heroicons-sparkles" class="w-16 h-16 text-gray-400" />
                    <p class="text-gray-500 dark:text-gray-400 text-lg">No recommendations available</p>
                </div>
            </section>
        </div>

        <!-- Decorative scan lines -->
        <div class="scanlines"></div>
    </div>
</template>

<style scoped>
/* Scan lines effect */
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

/* Hero Section - SCALED DOWN */
.hero-section {
    position: relative;
    min-height: 50vh;
    max-height: 60vh;
    overflow: hidden;
}

.loading-hero {
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

.hero-container {
    position: relative;
    min-height: 50vh;
    max-height: 60vh;
}

.hero-background {
    position: absolute;
    inset: 0;
    overflow: hidden;
}

.hero-background img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center 20%;
    filter: brightness(0.7);
}

.hero-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
}

.hero-gradient {
    position: absolute;
    inset: 0;
    background: linear-gradient(to bottom,
            transparent 0%,
            rgba(var(--color-gray-50), 0.3) 50%,
            rgb(var(--color-gray-50)) 100%);
}

@media (prefers-color-scheme: dark) {
    .hero-gradient {
        background: linear-gradient(to bottom,
                transparent 0%,
                rgba(var(--color-gray-950), 0.3) 50%,
                rgb(var(--color-gray-950)) 100%);
    }
}

.hero-content {
    position: relative;
    z-index: 10;
}

.poster-card {
    position: relative;
    overflow: hidden;
    border-radius: 1rem;
}

/* Typography */
.anime-main-title {
    font-size: clamp(1.75rem, 4vw, 2.5rem);
    font-weight: 900;
    line-height: 1.1;
    color: white;
    text-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    margin: 0;
}

.anime-sub-title {
    font-size: clamp(0.875rem, 1.5vw, 1.125rem);
    color: rgba(255, 255, 255, 0.8);
    font-weight: 400;
    margin-top: 0.5rem;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
}

.hero-synopsis {
    font-size: 1rem;
    line-height: 1.6;
    color: rgba(255, 255, 255, 0.95);
    max-width: 700px;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
}

/* Meta Info Grid */
.meta-info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 0.75rem;
}

.meta-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 0.875rem;
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(12px);
    border-radius: 0.75rem;
    color: white;
    font-weight: 500;
    border: 1px solid rgba(255, 255, 255, 0.2);
    font-size: 0.875rem;
}

/* Action Buttons */
.action-btn-primary {
    background: rgb(var(--color-primary-500)) !important;
    box-shadow: 0 8px 24px rgba(var(--color-primary-500), 0.3);
    transition: all 0.3s ease;
}

.action-btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 12px 32px rgba(var(--color-primary-500), 0.4);
}

.action-btn-secondary {
    backdrop-filter: blur(12px);
    background: rgba(255, 255, 255, 0.1) !important;
    border-color: rgba(255, 255, 255, 0.3) !important;
    color: white !important;
}

.action-btn-ghost {
    backdrop-filter: blur(12px);
    background: rgba(255, 255, 255, 0.05) !important;
    color: white !important;
}


/* Tabs */
.tab-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0;
    font-weight: 600;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
}

.tab-active {
    color: rgb(var(--color-primary-500));
    border-bottom-color: rgb(var(--color-primary-500));
}

.tab-inactive {
    color: rgb(var(--color-gray-600));
}

.tab-inactive:hover {
    color: rgb(var(--color-gray-900));
}

@media (prefers-color-scheme: dark) {
    .tab-inactive {
        color: rgb(var(--color-gray-400));
    }

    .tab-inactive:hover {
        color: rgb(var(--color-gray-200));
    }
}

/* Content Sections */
.content-section {
    margin-bottom: 4rem;
}

.section-title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.875rem;
    font-weight: 800;
    color: rgb(var(--color-gray-900));
    margin-bottom: 2rem;
}

@media (prefers-color-scheme: dark) {
    .section-title {
        color: rgb(var(--color-gray-50));
    }
}

/* Info Cards */
.info-card {
    background: white;
    border: 1px solid rgb(var(--color-gray-200));
    border-radius: 1rem;
    padding: 1.5rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

@media (prefers-color-scheme: dark) {
    .info-card {
        background: rgb(var(--color-gray-900));
        border-color: rgb(var(--color-gray-800));
    }
}

.info-card-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: rgb(var(--color-gray-900));
    margin-bottom: 1rem;
}

@media (prefers-color-scheme: dark) {
    .info-card-title {
        color: rgb(var(--color-gray-50));
    }
}

/* Details Grid */
.details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
}

.detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.detail-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: rgb(var(--color-gray-500));
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.detail-value {
    font-size: 1rem;
    font-weight: 600;
    color: rgb(var(--color-gray-900));
}

@media (prefers-color-scheme: dark) {
    .detail-value {
        color: rgb(var(--color-gray-50));
    }
}

/* Stat Items */
.stat-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 1rem;
    border-bottom: 1px solid rgb(var(--color-gray-200));
}

.stat-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
}

@media (prefers-color-scheme: dark) {
    .stat-item {
        border-bottom-color: rgb(var(--color-gray-800));
    }
}

.stat-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: rgb(var(--color-gray-500));
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

/* Episode Cards */
.episode-card {
    background: white;
    border: 1px solid rgb(var(--color-gray-200));
    border-radius: 0.75rem;
    padding: 1rem;
    transition: all 0.2s;
}

.episode-card:hover {
    border-color: rgb(var(--color-primary-500));
    box-shadow: 0 4px 12px rgba(var(--color-primary-500), 0.1);
    transform: translateY(-2px);
}

@media (prefers-color-scheme: dark) {
    .episode-card {
        background: rgb(var(--color-gray-900));
        border-color: rgb(var(--color-gray-800));
    }

    .episode-card:hover {
        border-color: rgb(var(--color-primary-500));
    }
}

.episode-number {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    height: 3rem;
    background: rgb(var(--color-primary-500));
    color: white;
    font-weight: 700;
    font-size: 1.125rem;
    border-radius: 0.5rem;
    flex-shrink: 0;
}

.episode-title {
    font-weight: 600;
    color: rgb(var(--color-gray-900));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

@media (prefers-color-scheme: dark) {
    .episode-title {
        color: rgb(var(--color-gray-50));
    }
}

.episode-meta {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.875rem;
    color: rgb(var(--color-gray-500));
}

/* Recommendation Cards */
.recommendation-card {
    display: block;
    transition: transform 0.2s;
}

.recommendation-card:hover {
    transform: translateY(-4px);
}

/* Empty State */
.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 4rem 2rem;
}

/* Responsive */
@media (max-width: 1024px) {
    .hero-content {
        padding-top: 3rem;
    }

    .anime-main-title {
        font-size: 2rem;
    }
}

@media (max-width: 768px) {
    .poster-card {
        max-width: 300px;
        margin: 0 auto;
    }

    .meta-info-grid {
        grid-template-columns: 1fr 1fr;
    }

    .details-grid {
        grid-template-columns: 1fr;
    }

    .hero-section {
        min-height: 45vh;
        max-height: 55vh;
    }
}
</style>