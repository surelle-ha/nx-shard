<script setup lang="ts">
  import { storeToRefs } from "pinia";
  
  const pageMeta = {
    header: {
      name: "🕹️ Explore Games",
      description: "Here are the available games you may download.",
    },
    showHeader: true,
  };
  definePageMeta({
    layout: "home",
  });
  
  const gameStore = useGameStore();
  const accountStore = useAccountStore();
  const { gameList, loading, error } = storeToRefs(gameStore);
  const isExperimental = computed(() => accountStore.account?.isExperimental);
  
  const filteredGames = computed(() => {
    if (!gameList.value) return null;
  
    return gameList.value.filter((game) => {
      if (isExperimental.value) return true;
      return !game.isExperimental;
    });
  });
  
  onMounted(async () => {
    await gameStore.loadData();
    gameStore.subscribeToChanges();
  });
  
  onUnmounted(() => {
    gameStore.unsubscribe();
  });
  </script>
  
  <template>
    <div class="h-full p-4 mt-6">
      <div class="mt-4">
        <div v-show="pageMeta.showHeader">
          <USkeleton v-if="loading" class="h-8 w-50" />
          <h1 v-else class="text-2xl font-bold">{{ pageMeta.header.name }}</h1>
  
          <USkeleton v-if="loading" class="mt-2 h-6 w-100" />
          <p v-else class="mt-2 text-gray-600 dark:text-gray-400">
            {{ pageMeta.header.description }}
          </p>
        </div>
  
        <div class="mt-4">
          <USkeleton v-if="loading" class="h-8 w-50" />
          <h2 v-else class="text-2xl font-bold">Featured Games</h2>
        </div>
  
        <div v-if="error" class="p-4 text-red-600 dark:text-red-400">
          Error loading games: {{ error }}
        </div>
  
        <div
          v-else-if="loading"
          class="p-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-4 gap-y-4"
        >
          <USkeleton v-for="i in 6" :key="i" class="h-72 w-full" />
        </div>
  
        <div
          v-else-if="filteredGames && filteredGames.length > 0"
          class="p-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-4 gap-y-4"
        >
          <GameCard
            v-for="game in filteredGames"
            :key="game.title"
            :game="game"
          />
        </div>
  
        <div v-else class="p-4 text-center text-gray-600 dark:text-gray-400">
          No games available.
        </div>
      </div>
    </div>
  </template>
  