<script setup lang="ts">
import Hls from 'hls.js';

definePageMeta({ layout: 'home' });

const route = useRoute();
const animeId = route.query.anime as string;
const episodeNumber = ref(Number(route.query.ep) || 1);

/* =======================
   TYPES
======================= */

type Episode = {
    id: string;
    episodeNumber: number;
    title: string;
    alternativeTitle?: string;
    isFiller?: boolean;
};

type Server = {
    id: number | null;
    name: string;
    type: 'sub' | 'dub';
};

type StreamData = {
    link: { file: string; type: string };
    tracks?: {
        file: string;
        label?: string;
        kind: string;
        default?: boolean;
    }[];
    intro: { start: number; end: number };
    outro: { start: number; end: number };
    server: string;
};

type AnimeDetails = {
    title: string;
    alternativeTitle: string;
    id: string;
    poster: string;
};

/* =======================
   STATE
======================= */

const isLoading = ref(true);
const isLoadingStream = ref(false);

const animeData = ref<AnimeDetails | null>(null);
const episodes = ref<Episode[]>([]);
const servers = ref<{ sub: Server[]; dub: Server[] }>({ sub: [], dub: [] });
const streamData = ref<StreamData | null>(null);

const selectedType = ref<'sub' | 'dub'>('sub');
const selectedServer = ref('');
const currentEpisodeId = ref('');

const videoElement = ref<HTMLVideoElement | null>(null);
const hls = ref<Hls | null>(null);

/* =======================
   SAFE JSON PARSER (CRITICAL FIX)
======================= */

const safeJson = async (res: Response) => {
    const text = await res.text();
    try {
        return JSON.parse(text);
    } catch {
        console.error('❌ API returned non-JSON:', text);
        throw new Error(text);
    }
};

/* =======================
   COMPUTED
======================= */

const selectedEpisode = computed(() =>
    episodes.value.find(e => e.episodeNumber === episodeNumber.value)
);

const availableServers = computed(() =>
    servers.value[selectedType.value] || []
);

/* =======================
   LOADERS
======================= */

const loadAnimeData = async () => {
    const res = await fetch(`http://localhost:3030/api/v1/anime/${animeId}`);
    const data = await safeJson(res);
    if (data.success) animeData.value = data.data;
};

const loadEpisodes = async () => {
    const res = await fetch(`http://localhost:3030/api/v1/episodes/${animeId}`);
    const data = await safeJson(res);

    if (data.success && Array.isArray(data.data)) {
        episodes.value = data.data;

        // ✅ FORCE first episode selection
        const first = episodes.value.find(e => e.episodeNumber === episodeNumber.value)
            ?? episodes.value[0];

        if (first) {
            episodeNumber.value = first.episodeNumber;
            currentEpisodeId.value = first.id;
        }
    }
};

const loadServers = async () => {
    if (!currentEpisodeId.value) return;

    console.log(currentEpisodeId.value)
    const res = await fetch(
        `http://localhost:3030/api/v1/servers/${currentEpisodeId.value}`
    );
    const data = await safeJson(res);
    console.log(data)

    if (data.success) {
        servers.value = {
            sub: data.data.sub || [],
            dub: data.data.dub || []
        };

        const first = servers.value[selectedType.value][0];
        if (first) selectedServer.value = first.name;
    }
};

const loadStream = async () => {
    if (!currentEpisodeId.value || !selectedServer.value) return;

    isLoadingStream.value = true;

    const res = await fetch(
        `http://localhost:3030/api/v1/stream?id=${currentEpisodeId.value}&server=${selectedServer.value}&type=${selectedType.value}`
    );
    const data = await safeJson(res);

    if (data.success) {
        streamData.value = data.data;
        await nextTick();
        setupVideo();
    }

    isLoadingStream.value = false;
};

/* =======================
   VIDEO PLAYER
======================= */

const setupVideo = () => {
    if (!videoElement.value || !streamData.value) return;

    if (hls.value) {
        hls.value.destroy();
        hls.value = null;
    }

    const video = videoElement.value;
    const url = `/api/v1/proxy-stream?url=${encodeURIComponent(
        streamData.value.link.file
    )}`;

    video.pause();
    video.removeAttribute('src');
    video.load();

    if (Hls.isSupported()) {
        hls.value = new Hls();
        hls.value.loadSource(url);
        hls.value.attachMedia(video);
        hls.value.on(Hls.Events.MANIFEST_PARSED, () => video.play());
    } else {
        video.src = url;
        video.play();
    }
};


/* =======================
   ACTIONS
======================= */

const changeEpisode = async (ep: Episode) => {
    episodeNumber.value = ep.episodeNumber;
    currentEpisodeId.value = ep.id;

    await navigateTo({
        path: '/watch-anime/watch',
        query: { anime: animeId, ep: ep.episodeNumber }
    });

    await loadServers();
    await loadStream();
};

const changeServer = async (name: string) => {
    selectedServer.value = name;
    await loadStream();
};

const changeType = async (type: 'sub' | 'dub') => {
    selectedType.value = type;
    const first = servers.value[type][0];
    if (first) {
        selectedServer.value = first.name;
        await loadStream();
    }
};


const goToNextEpisode = () => {
    const i = episodes.value.findIndex(ep => ep.episodeNumber === episodeNumber.value);
    const nextEp = episodes.value[i + 1];
    if (i !== -1 && i < episodes.value.length - 1 && nextEp) {
        changeEpisode(nextEp);
    }
};

const goToPreviousEpisode = () => {
    const i = episodes.value.findIndex(ep => ep.episodeNumber === episodeNumber.value);
    const prevEp = episodes.value[i - 1];
    if (i > 0 && prevEp) {
        changeEpisode(prevEp);
    }
};

/*
   LIFECYCLE
======================= */

onMounted(async () => {
    isLoading.value = true;

    await loadAnimeData();
    await loadEpisodes();
    await loadServers();
    await loadStream();

    isLoading.value = false;
});

onBeforeUnmount(() => {
    if (hls.value) hls.value.destroy();
});
</script>

<template>
    <div class="watch-page"> <!-- Loading State -->
        <div v-if="isLoading" class="loading-container">
            <div class="loader">
                <div class="loader-ring"></div>
                <div class="loader-text">LOADING</div>
            </div>
        </div> <!-- Main Content -->
        <div v-else class="watch-container"> <!-- Back Button -->
            <div class="back-section">
                <UButton :to="`/watch-anime/anime/${animeId}`" variant="ghost" icon="i-heroicons-arrow-left" size="lg"
                    class="back-button"> Back to Details </UButton>
            </div>
            <div class="content-grid"> <!-- Video Player Section -->
                <div class="video-section"> <!-- Video Player -->
                    <div class="video-container">
                        <div v-if="isLoadingStream" class="video-loading">
                            <div class="loader">
                                <div class="loader-ring"></div>
                                <div class="loader-text">LOADING STREAM</div>
                            </div>
                        </div> <video v-else ref="videoElement" class="video-player" controls
                            crossorigin="anonymous"></video>
                    </div> <!-- Episode Info -->
                    <div class="episode-info-card">
                        <div class="episode-header">
                            <div>
                                <h1 class="anime-title">{{ animeData?.title }}</h1>
                                <p class="episode-title"> Episode {{ episodeNumber }} <span
                                        v-if="selectedEpisode?.title">: {{ selectedEpisode.title }}</span> </p>
                            </div>
                            <UBadge v-if="selectedEpisode?.isFiller" label="Filler" color="warning" variant="soft"
                                size="lg" />
                        </div> <!-- Controls -->
                        <div class="controls-section"> <!-- Type Selection -->
                            <div class="control-group"> <span class="control-label">Audio</span>
                                <div class="button-group">
                                    <UButton @click="changeType('sub')"
                                        :variant="selectedType === 'sub' ? 'solid' : 'outline'"
                                        :color="selectedType === 'sub' ? 'primary' : 'neutral'" size="lg"> SUB
                                    </UButton>
                                    <UButton @click="changeType('dub')"
                                        :variant="selectedType === 'dub' ? 'solid' : 'outline'"
                                        :color="selectedType === 'dub' ? 'primary' : 'neutral'" size="lg"> DUB
                                    </UButton>
                                </div>
                            </div> <!-- Server Selection -->
                            <div class="control-group"> <span class="control-label">Server</span>
                                <div class="button-group">
                                    <UButton v-for="server in availableServers" :key="server.name"
                                        @click="changeServer(server.name)"
                                        :variant="selectedServer === server.name ? 'solid' : 'outline'"
                                        :color="selectedServer === server.name ? 'primary' : 'neutral'" size="lg"
                                        :disabled="!server.id"> {{ server.name.toUpperCase() }} </UButton>
                                </div>
                            </div> <!-- Episode Navigation -->
                            <div class="episode-nav">
                                <UButton @click="goToPreviousEpisode"
                                    :disabled="!episodes.find(ep => ep.episodeNumber === episodeNumber - 1)"
                                    icon="i-heroicons-chevron-left" size="lg" variant="outline"> Previous </UButton>
                                <UButton @click="goToNextEpisode"
                                    :disabled="!episodes.find(ep => ep.episodeNumber === episodeNumber + 1)"
                                    icon="i-heroicons-chevron-right" trailing size="lg" variant="outline"> Next
                                </UButton>
                            </div>
                        </div>
                    </div>
                </div> <!-- Episodes Sidebar -->
                <div class="episodes-sidebar">
                    <div class="sidebar-header">
                        <h2 class="sidebar-title">
                            <UIcon name="i-heroicons-list-bullet" class="w-6 h-6" /> Episodes
                        </h2>
                        <UBadge :label="`${episodes.length} Total`" variant="soft" />
                    </div>
                    <div class="episodes-list"> <button v-for="episode in episodes" :key="episode.id"
                            @click="changeEpisode(episode)"
                            :class="['episode-item', episode.episodeNumber === episodeNumber && 'episode-active']">
                            <div class="episode-number-badge">{{ episode.episodeNumber }}</div>
                            <div class="episode-details">
                                <p class="episode-item-title"> {{ episode.title || `Episode ${episode.episodeNumber}`
                                }} </p>
                                <p v-if="episode.isFiller" class="episode-filler">
                                    <UIcon name="i-heroicons-exclamation-circle" class="w-3 h-3" /> Filler
                                </p>
                            </div>
                            <UIcon v-if="episode.episodeNumber === episodeNumber" name="i-heroicons-play-circle-solid"
                                class="w-5 h-5 text-primary-500" />
                        </button> </div>
                </div>
            </div>
        </div> <!-- Decorative Elements -->
        <div class="scanlines"></div>
    </div>
</template>
<style scoped>
/* Loading States */
.loading-container,
.video-loading {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
}

.video-loading {
    position: absolute;
    inset: 0;
    background: rgb(var(--color-gray-950));
    z-index: 10;
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

/* Page Layout */
.watch-page {
    min-height: 100vh;
    background: linear-gradient(135deg, rgb(var(--color-gray-950)) 0%, rgb(var(--color-gray-900)) 100%);
    padding: 2rem;
}

.watch-container {
    max-width: 1920px;
    margin: 0 auto;
}

.back-section {
    margin-bottom: 2rem;
}

.back-button {
    backdrop-filter: blur(12px);
    background: rgba(255, 255, 255, 0.05) !important;
    border: 1px solid rgba(255, 255, 255, 0.1);
    transition: all 0.3s ease;
}

.back-button:hover {
    background: rgba(255, 255, 255, 0.1) !important;
    border-color: rgba(255, 255, 255, 0.2);
    transform: translateX(-4px);
}

.content-grid {
    display: grid;
    grid-template-columns: 1fr 400px;
    gap: 2rem;
}

/* Video Section */
.video-section {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.video-container {
    position: relative;
    aspect-ratio: 16 / 9;
    background: black;
    border-radius: 1.5rem;
    overflow: hidden;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
    border: 2px solid rgba(var(--color-primary-500), 0.2);
}

.video-player {
    width: 100%;
    height: 100%;
    object-fit: contain;
}

/* Episode Info Card */
.episode-info-card {
    background: rgba(255, 255, 255, 0.03);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 1.5rem;
    padding: 2rem;
}

.episode-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 2rem;
    padding-bottom: 2rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.anime-title {
    font-size: 1.5rem;
    font-weight: 800;
    color: white;
    margin-bottom: 0.5rem;
    font-family: 'Space Grotesk', sans-serif;
    letter-spacing: -0.02em;
}

.episode-title {
    font-size: 1.125rem;
    color: rgba(255, 255, 255, 0.7);
    font-weight: 500;
}

/* Controls */
.controls-section {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.control-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.control-label {
    font-size: 0.875rem;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.6);
    text-transform: uppercase;
    letter-spacing: 0.1em;
}

.button-group {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
}

.episode-nav {
    display: flex;
    gap: 1rem;
    padding-top: 1rem;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.episode-nav button {
    flex: 1;
}

/* Episodes Sidebar */
.episodes-sidebar {
    background: rgba(255, 255, 255, 0.03);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 1.5rem;
    padding: 1.5rem;
    height: fit-content;
    max-height: calc(100vh - 8rem);
    display: flex;
    flex-direction: column;
}

.sidebar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.sidebar-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
}

.episodes-list {
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding-right: 0.5rem;
}

.episodes-list::-webkit-scrollbar {
    width: 6px;
}

.episodes-list::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 3px;
}

.episodes-list::-webkit-scrollbar-thumb {
    background: rgba(var(--color-primary-500), 0.5);
    border-radius: 3px;
}

.episodes-list::-webkit-scrollbar-thumb:hover {
    background: rgba(var(--color-primary-500), 0.7);
}

/* Episode Items */
.episode-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
    width: 100%;
}

.episode-item:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(var(--color-primary-500), 0.3);
    transform: translateX(4px);
}

.episode-active {
    background: rgba(var(--color-primary-500), 0.15) !important;
    border-color: rgba(var(--color-primary-500), 0.5) !important;
}

.episode-number-badge {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    background: rgba(var(--color-primary-500), 0.2);
    color: rgb(var(--color-primary-400));
    font-weight: 700;
    font-size: 1rem;
    border-radius: 0.5rem;
    flex-shrink: 0;
}

.episode-active .episode-number-badge {
    background: rgb(var(--color-primary-500));
    color: white;
}

.episode-details {
    flex: 1;
    min-width: 0;
}

.episode-item-title {
    font-weight: 600;
    color: white;
    font-size: 0.875rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 0.25rem;
}

.episode-filler {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.5);
}

/* Scanlines Effect */
.scanlines {
    position: fixed;
    inset: 0;
    background: linear-gradient(to bottom, transparent 50%, rgba(0, 0, 0, 0.02) 51%);
    background-size: 100% 4px;
    pointer-events: none;
    z-index: 100;
    opacity: 0.3;
}

/* Responsive */
@media (max-width: 1280px) {
    .content-grid {
        grid-template-columns: 1fr;
    }

    .episodes-sidebar {
        max-height: 600px;
    }
}

@media (max-width: 768px) {
    .watch-page {
        padding: 1rem;
    }

    .anime-title {
        font-size: 1.25rem;
    }

    .episode-title {
        font-size: 1rem;
    }

    .button-group {
        width: 100%;
    }

    .button-group button {
        flex: 1;
    }
}
</style>
