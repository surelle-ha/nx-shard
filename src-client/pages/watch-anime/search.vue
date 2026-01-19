<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useDebounceFn } from '@vueuse/core';

definePageMeta({
    layout: "home",
});

const route = useRoute();
const router = useRouter();

type SearchResult = {
    title: string;
    alternativeTitle: string;
    id: string;
    poster: string;
    type: string;
    episodes: {
        sub: number;
        dub: number;
        eps: number;
    };
    is18Plus: boolean;
};

type ApiResponse = {
    success: boolean;
    data: {
        pageInfo: {
            totalPages: number;
            currentPage: number;
            hasNextPage: boolean;
        };
        response: SearchResult[];
    };
};

// State
const searchQuery = ref((route.query.q as string) || '');
const suggestions = ref<SearchResult[]>([]);
const searchResults = ref<SearchResult[]>([]);
const isSearching = ref(false);
const isLoadingSuggestions = ref(false);
const showSuggestions = ref(false);
const currentPage = ref(1);
const totalPages = ref(1);

// Filters
const selectedType = ref((route.query.type as string) || 'all');
const selectedGenre = ref((route.query.genre as string) || 'all');
const selectedStatus = ref((route.query.status as string) || 'all');
const selectedSort = ref((route.query.sort as string) || 'relevance');
const showFilters = ref(false);

const types = [
    { value: 'all', label: 'All Types' },
    { value: 'tv', label: 'TV Series' },
    { value: 'movie', label: 'Movie' },
    { value: 'ova', label: 'OVA' },
    { value: 'ona', label: 'ONA' },
    { value: 'special', label: 'Special' },
];

const genres = [
    { value: 'all', label: 'All Genres' },
    { value: 'action', label: 'Action' },
    { value: 'adventure', label: 'Adventure' },
    { value: 'comedy', label: 'Comedy' },
    { value: 'drama', label: 'Drama' },
    { value: 'fantasy', label: 'Fantasy' },
    { value: 'horror', label: 'Horror' },
    { value: 'isekai', label: 'Isekai' },
    { value: 'mecha', label: 'Mecha' },
    { value: 'mystery', label: 'Mystery' },
    { value: 'psychological', label: 'Psychological' },
    { value: 'romance', label: 'Romance' },
    { value: 'sci-fi', label: 'Sci-Fi' },
    { value: 'shounen', label: 'Shounen' },
    { value: 'slice of life', label: 'Slice of Life' },
    { value: 'sports', label: 'Sports' },
    { value: 'supernatural', label: 'Supernatural' },
    { value: 'thriller', label: 'Thriller' },
];

const statuses = [
    { value: 'all', label: 'All Status' },
    { value: 'top-airing', label: 'Currently Airing' },
    { value: 'completed', label: 'Completed' },
    { value: 'top-upcoming', label: 'Upcoming' },
];

const sortOptions = [
    { value: 'relevance', label: 'Relevance' },
    { value: 'most-popular', label: 'Most Popular' },
    { value: 'most-favorite', label: 'Most Favorite' },
    { value: 'recently-added', label: 'Recently Added' },
    { value: 'recently-updated', label: 'Recently Updated' },
];

// Fetch suggestions
const fetchSuggestions = useDebounceFn(async (query: string) => {
    if (!query || query.length < 2) {
        suggestions.value = [];
        showSuggestions.value = false;
        return;
    }

    isLoadingSuggestions.value = true;
    try {
        const res = await fetch(`http://localhost:3030/api/v1/search/suggestion?keyword=${encodeURIComponent(query)}`);
        const data = await res.json();

        if (data.success && data.data?.suggestions) {
            suggestions.value = data.data.suggestions.slice(0, 8);
            showSuggestions.value = true;
        }
    } catch (error) {
        console.error('Failed to fetch suggestions:', error);
    } finally {
        isLoadingSuggestions.value = false;
    }
}, 300);

// Search function
const performSearch = async (page = 1) => {
    if (!searchQuery.value.trim()) {
        searchResults.value = [];
        return;
    }

    isSearching.value = true;
    showSuggestions.value = false;

    try {
        let url = `http://localhost:3030/api/v1/search?keyword=${encodeURIComponent(searchQuery.value)}&page=${page}`;

        const res = await fetch(url);
        const data: ApiResponse = await res.json();

        if (data.success && data.data) {
            searchResults.value = data.data.response || [];
            totalPages.value = data.data.pageInfo?.totalPages || 1;
            currentPage.value = data.data.pageInfo?.currentPage || 1;

            // Update URL with search params
            router.push({
                query: {
                    q: searchQuery.value,
                    page: page > 1 ? page : undefined,
                    type: selectedType.value !== 'all' ? selectedType.value : undefined,
                    genre: selectedGenre.value !== 'all' ? selectedGenre.value : undefined,
                    status: selectedStatus.value !== 'all' ? selectedStatus.value : undefined,
                    sort: selectedSort.value !== 'relevance' ? selectedSort.value : undefined,
                }
            });
        }
    } catch (error) {
        console.error('Search failed:', error);
    } finally {
        isSearching.value = false;
    }
};

// Filter results client-side
const filteredResults = computed(() => {
    let results = [...searchResults.value];

    if (selectedType.value !== 'all') {
        results = results.filter(anime => anime.type.toLowerCase() === selectedType.value);
    }

    return results;
});

// Watch for input changes
watch(searchQuery, (newVal) => {
    fetchSuggestions(newVal);
});

// Watch for filter changes
watch([selectedType, selectedGenre, selectedStatus, selectedSort], () => {
    if (searchQuery.value) {
        performSearch(1);
    }
});

// Handle suggestion click
const selectSuggestion = (anime: SearchResult) => {
    searchQuery.value = anime.title;
    showSuggestions.value = false;
    performSearch();
};

// Clear search
const clearSearch = () => {
    searchQuery.value = '';
    searchResults.value = [];
    suggestions.value = [];
    showSuggestions.value = false;
    router.push({ query: {} });
};

// Format episode count
const formatEpisodeCount = (episodes: { sub: number; dub: number; eps: number }) => {
    const parts = [];
    if (episodes.sub) parts.push(`${episodes.sub} SUB`);
    if (episodes.dub) parts.push(`${episodes.dub} DUB`);
    return parts.join(' • ');
};

// Load initial search if query param exists
onMounted(() => {
    if (searchQuery.value) {
        performSearch(Number(route.query.page) || 1);
    }
});

// Close suggestions when clicking outside
const searchContainer = ref<HTMLElement | null>(null);
const handleClickOutside = (event: MouseEvent) => {
    if (searchContainer.value && !searchContainer.value.contains(event.target as Node)) {
        showSuggestions.value = false;
    }
};

onMounted(() => {
    document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside);
});
</script>

<template>
    <div class="min-h-screen p-10">
        <div class="max-w-7xl mx-auto">
            <!-- Header -->
            <div class="mb-8">
                <div class="flex items-center gap-3 mb-2">
                    <div class="p-3 bg-primary-500/10 rounded-xl">
                        <UIcon name="i-heroicons-magnifying-glass" class="w-8 h-8 text-primary-500" />
                    </div>
                    <h1
                        class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text text-transparent">
                        Search Anime
                    </h1>
                </div>
                <p class="text-gray-600 dark:text-gray-400 text-lg ml-14">
                    Find your next favorite anime
                </p>
            </div>

            <!-- Search Bar -->
            <div class="mb-8" ref="searchContainer">
                <div class="relative">
                    <div class="search-bar-container">
                        <UInput v-model="searchQuery" @keyup.enter="performSearch()" type="text" size="lg" variant="subtle" icon="i-lucide-search"
                            placeholder="Search for anime titles..." class="search-input" autocomplete="off" />
                        <button v-if="searchQuery" @click="clearSearch" class="clear-btn">
                            <UIcon name="i-heroicons-x-mark" class="w-5 h-5" />
                        </button>
                        <UButton @click="performSearch()" color="primary" size="lg" class="search-btn"
                            :loading="isSearching">
                            Search
                        </UButton>
                    </div>

                    <!-- Suggestions Dropdown -->
                    <div v-if="showSuggestions && suggestions.length > 0" class="suggestions-dropdown">
                        <div v-for="anime in suggestions" :key="anime.id" @click="selectSuggestion(anime)"
                            class="suggestion-item">
                            <img :src="anime.poster" :alt="anime.title" class="suggestion-poster" />
                            <div class="flex-1 min-w-0">
                                <p class="suggestion-title">{{ anime.title }}</p>
                                <p class="suggestion-meta">
                                    {{ anime.type }}{{ anime.episodes.eps ? ` • ${anime.episodes.eps} episodes` : '' }}
                                </p>
                            </div>
                            <UIcon name="i-heroicons-arrow-right" class="w-5 h-5 text-gray-400" />
                        </div>
                    </div>

                    <!-- Loading Suggestions -->
                    <div v-if="isLoadingSuggestions" class="suggestions-dropdown">
                        <div v-for="i in 3" :key="i" class="suggestion-item">
                            <USkeleton class="w-12 h-16 rounded" />
                            <div class="flex-1 space-y-2">
                                <USkeleton class="h-4 w-3/4" />
                                <USkeleton class="h-3 w-1/2" />
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Filter Toggle -->
                <div class="flex items-center justify-between mt-4">
                    <UButton @click="showFilters = !showFilters" variant="ghost"
                        :icon="showFilters ? 'i-heroicons-chevron-up' : 'i-heroicons-adjustments-horizontal'">
                        {{ showFilters ? 'Hide Filters' : 'Show Filters' }}
                    </UButton>

                    <p v-if="searchResults.length > 0" class="text-sm text-gray-600 dark:text-gray-400">
                        Found {{ searchResults.length }} results
                    </p>
                </div>

                <!-- Filters Panel -->
                <Transition name="filters">
                    <div v-if="showFilters" class="filters-panel">
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                            <!-- Type Filter -->
                            <div>
                                <label class="filter-label">Type</label>
                                <select v-model="selectedType" class="filter-select">
                                    <option v-for="type in types" :key="type.value" :value="type.value">
                                        {{ type.label }}
                                    </option>
                                </select>
                            </div>

                            <!-- Genre Filter -->
                            <div>
                                <label class="filter-label">Genre</label>
                                <select v-model="selectedGenre" class="filter-select">
                                    <option v-for="genre in genres" :key="genre.value" :value="genre.value">
                                        {{ genre.label }}
                                    </option>
                                </select>
                            </div>

                            <!-- Status Filter -->
                            <div>
                                <label class="filter-label">Status</label>
                                <select v-model="selectedStatus" class="filter-select">
                                    <option v-for="status in statuses" :key="status.value" :value="status.value">
                                        {{ status.label }}
                                    </option>
                                </select>
                            </div>

                            <!-- Sort Filter -->
                            <div>
                                <label class="filter-label">Sort By</label>
                                <select v-model="selectedSort" class="filter-select">
                                    <option v-for="sort in sortOptions" :key="sort.value" :value="sort.value">
                                        {{ sort.label }}
                                    </option>
                                </select>
                            </div>
                        </div>

                        <!-- Active Filters -->
                        <div v-if="selectedType !== 'all' || selectedGenre !== 'all' || selectedStatus !== 'all' || selectedSort !== 'relevance'"
                            class="flex items-center gap-2 mt-4 flex-wrap">
                            <span class="text-sm text-gray-600 dark:text-gray-400">Active filters:</span>
                            <UBadge v-if="selectedType !== 'all'" color="primary" variant="soft" class="cursor-pointer"
                                @click="selectedType = 'all'">
                                {{types.find(t => t.value === selectedType)?.label}} ×
                            </UBadge>
                            <UBadge v-if="selectedGenre !== 'all'" color="primary" variant="soft" class="cursor-pointer"
                                @click="selectedGenre = 'all'">
                                {{genres.find(g => g.value === selectedGenre)?.label}} ×
                            </UBadge>
                            <UBadge v-if="selectedStatus !== 'all'" color="primary" variant="soft"
                                class="cursor-pointer" @click="selectedStatus = 'all'">
                                {{statuses.find(s => s.value === selectedStatus)?.label}} ×
                            </UBadge>
                            <UBadge v-if="selectedSort !== 'relevance'" color="primary" variant="soft"
                                class="cursor-pointer" @click="selectedSort = 'relevance'">
                                {{sortOptions.find(s => s.value === selectedSort)?.label}} ×
                            </UBadge>
                        </div>
                    </div>
                </Transition>
            </div>

            <!-- Search Results -->
            <div v-if="isSearching" class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
                <USkeleton v-for="i in 10" :key="i" class="h-[300px] rounded-lg" />
            </div>

            <div v-else-if="filteredResults.length > 0" class="space-y-8">
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
                    <NuxtLink v-for="anime in filteredResults" :key="anime.id" :to="`/watch-anime/anime/${anime.id}`"
                        class="anime-card group">
                        <div
                            class="relative overflow-hidden rounded-lg shadow-lg transition-transform duration-300 hover:scale-105">
                            <img :src="anime.poster" :alt="anime.title" class="h-[300px] w-full object-cover"
                                loading="lazy" />

                            <!-- Type badge -->
                            <div class="absolute top-2 left-2">
                                <UBadge :label="anime.type" color="primary" variant="solid" />
                            </div>

                            <!-- 18+ badge -->
                            <div v-if="anime.is18Plus" class="absolute top-2 right-2">
                                <UBadge label="18+" color="error" variant="solid" />
                            </div>

                            <!-- Overlay on hover -->
                            <div
                                class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/70 to-transparent p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                                <h3 class="text-white font-semibold text-sm line-clamp-2 mb-1">
                                    {{ anime.title }}
                                </h3>
                                <p class="text-xs text-gray-300">
                                    {{ anime.type }}
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
                                {{ anime.title }}
                            </p>
                            <p class="text-xs text-gray-600 dark:text-gray-400">
                                {{ formatEpisodeCount(anime.episodes) }}
                            </p>
                        </div>
                    </NuxtLink>
                </div>

                <!-- Pagination -->
                <div v-if="totalPages > 1" class="flex justify-center gap-2">
                    <UButton @click="performSearch(currentPage - 1)" :disabled="currentPage === 1" variant="outline"
                        icon="i-heroicons-chevron-left">
                        Previous
                    </UButton>

                    <div class="flex items-center gap-2">
                        <span class="text-sm text-gray-600 dark:text-gray-400">
                            Page {{ currentPage }} of {{ totalPages }}
                        </span>
                    </div>

                    <UButton @click="performSearch(currentPage + 1)" :disabled="currentPage === totalPages"
                        variant="outline" trailing-icon="i-heroicons-chevron-right">
                        Next
                    </UButton>
                </div>
            </div>

            <!-- Empty State -->
            <div v-else-if="searchQuery && !isSearching" class="empty-state">
                <UIcon name="i-heroicons-magnifying-glass" class="w-20 h-20 text-gray-400" />
                <h3 class="text-xl font-semibold text-gray-900 dark:text-white mt-4">No results found</h3>
                <p class="text-gray-600 dark:text-gray-400 mt-2">
                    Try different keywords or adjust your filters
                </p>
                <UButton @click="clearSearch" variant="outline" class="mt-4">
                    Clear Search
                </UButton>
            </div>

            <!-- Initial State -->
            <div v-else class="empty-state">
                <UIcon name="i-heroicons-magnifying-glass-circle" class="w-20 h-20 text-gray-400" />
                <h3 class="text-xl font-semibold text-gray-900 dark:text-white mt-4">Start Your Search</h3>
                <p class="text-gray-600 dark:text-gray-400 mt-2">
                    Enter anime title or keywords to begin
                </p>
            </div>
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
    background: linear-gradient(to bottom, transparent 50%, rgba(0, 0, 0, 0.01) 51%);
    background-size: 100% 4px;
    pointer-events: none;
    z-index: 100;
    opacity: 0.3;
}

/* Search Bar */
.search-bar-container {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: white;
    border: 2px solid rgb(var(--color-gray-200));
    border-radius: 1rem;
    padding: 0.75rem 1rem;
    transition: all 0.2s;
}

.search-bar-container:focus-within {
    border-color: rgb(var(--color-primary-500));
    box-shadow: 0 0 0 3px rgba(var(--color-primary-500), 0.1);
}

@media (prefers-color-scheme: dark) {
    .search-bar-container {
        background: rgb(var(--color-gray-900));
        border-color: rgb(var(--color-gray-800));
    }
}

.search-icon {
    width: 1.5rem;
    height: 1.5rem;
    color: rgb(var(--color-gray-400));
    flex-shrink: 0;
}

.search-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    font-size: 1rem;
    color: rgb(var(--color-gray-900));
}

.search-input::placeholder {
    color: rgb(var(--color-gray-400));
}

@media (prefers-color-scheme: dark) {
    .search-input {
        color: rgb(var(--color-gray-50));
    }
}

.clear-btn {
    padding: 0.5rem;
    border-radius: 0.5rem;
    background: transparent;
    color: rgb(var(--color-gray-400));
    transition: all 0.2s;
    border: none;
    cursor: pointer;
}

.clear-btn:hover {
    background: rgb(var(--color-gray-100));
    color: rgb(var(--color-gray-600));
}

@media (prefers-color-scheme: dark) {
    .clear-btn:hover {
        background: rgb(var(--color-gray-800));
        color: rgb(var(--color-gray-300));
    }
}

.search-btn {
    flex-shrink: 0;
}

/* Suggestions Dropdown */
.suggestions-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    left: 0;
    right: 0;
    background: white;
    border: 1px solid rgb(var(--color-gray-200));
    border-radius: 1rem;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
    max-height: 400px;
    overflow-y: auto;
    z-index: 50;
}

@media (prefers-color-scheme: dark) {
    .suggestions-dropdown {
        background: rgb(var(--color-gray-900));
        border-color: rgb(var(--color-gray-800));
    }
}

.suggestion-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    cursor: pointer;
    transition: background 0.2s;
    border-bottom: 1px solid rgb(var(--color-gray-100));
}

.suggestion-item:last-child {
    border-bottom: none;
}

.suggestion-item:hover {
    background: rgb(var(--color-gray-50));
}

@media (prefers-color-scheme: dark) {
    .suggestion-item {
        border-bottom-color: rgb(var(--color-gray-800));
    }

    .suggestion-item:hover {
        background: rgb(var(--color-gray-800));
    }
}

.suggestion-poster {
    width: 48px;
    height: 64px;
    object-fit: cover;
    border-radius: 0.5rem;
    flex-shrink: 0;
}

.suggestion-title {
    font-weight: 600;
    color: rgb(var(--color-gray-900));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

@media (prefers-color-scheme: dark) {
    .suggestion-title {
        color: rgb(var(--color-gray-50));
    }
}

.suggestion-meta {
    font-size: 0.875rem;
    color: rgb(var(--color-gray-600));
}

@media (prefers-color-scheme: dark) {
    .suggestion-meta {
        color: rgb(var(--color-gray-400));
    }
}

/* Filters Panel */
.filters-panel {
    background: white;
    border: 1px solid rgb(var(--color-gray-200));
    border-radius: 1rem;
    padding: 1.5rem;
    margin-top: 1rem;
}

@media (prefers-color-scheme: dark) {
    .filters-panel {
        background: rgb(var(--color-gray-900));
        border-color: rgb(var(--color-gray-800));
    }
}

.filter-label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: rgb(var(--color-gray-700));
    margin-bottom: 0.5rem;
}

@media (prefers-color-scheme: dark) {
    .filter-label {
        color: rgb(var(--color-gray-300));
    }
}

.filter-select {
    width: 100%;
    padding: 0.625rem 0.875rem;
    border: 1px solid rgb(var(--color-gray-300));
    border-radius: 0.5rem;
    background: white;
    color: rgb(var(--color-gray-900));
    font-size: 0.875rem;
    transition: all 0.2s;
    cursor: pointer;
}

.filter-select:focus {
    outline: none;
    border-color: rgb(var(--color-primary-500));
    box-shadow: 0 0 0 3px rgba(var(--color-primary-500), 0.1);
}

@media (prefers-color-scheme: dark) {
    .filter-select {
        background: rgb(var(--color-gray-800));
        border-color: rgb(var(--color-gray-700));
        color: rgb(var(--color-gray-50));
    }
}

/* Filters Transition */
.filters-enter-active,
.filters-leave-active {
    transition: all 0.3s ease;
}

.filters-enter-from,
.filters-leave-to {
    opacity: 0;
    transform: translateY(-10px);
}

/* Empty State */
.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
}

/* Responsive */
@media (max-width: 768px) {
    .search-bar-container {
        flex-wrap: wrap;
    }

    .search-input {
        min-width: 100%;
        order: -1;
    }

    .search-btn {
        width: 100%;
    }
}
</style>