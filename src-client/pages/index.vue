<script setup lang="ts">
definePageMeta({
  layout: "home",
});

const isLoading = ref(true);

const pageMeta = {
  header: {
    name: "🕹️ Explore Games",
    description: "Here are the available games you may download.",
  },
  showHeader: true,
};

const games = ref([
  {
    title: "Pokemon ZA",
    image:
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS9dAA665o-0TzCQuYoboZ156LkBPHA61G8aw&s",
    date: "2024-11-25",
  },
  {
    title: "Monster Hunter: Rise",
    image:
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSsM82lTf9dEsCielkhZUQOHC5o_XUR4CFpfQ&s",
    date: "2024-11-25",
  },
  {
    title: "Celeste",
    image:
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRAURXrDNJiE-wMz_su4JCuLpVPMEuBeLUFfw&s",
    date: "2024-11-25",
  },
  {
    title: "Pokemon Legends: Arceus",
    image:
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRkjxs9bmqWpfot0n7ZDHG42otfyFwceSnyKA&s",
    date: "2024-11-25",
  },
  {
    title: "The Witcher 3: Wild Hunt",
    image:
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQUwLu1J6R5lHWXjea4MQBEDBrtFXrdOyj8Ng&s",
    date: "2024-11-25",
  },
  {
    title: "No Man's Sky",
    image:
      "https://static0.srcdn.com/wordpress/wp-content/uploads/2016/09/No-Mans-Sky-Banner.jpg?w=1200&h=900&fit=crop",
    date: "2024-11-25",
  },
  {
    title: "Pokemon ZA",
    image:
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS9dAA665o-0TzCQuYoboZ156LkBPHA61G8aw&s",
    date: "2024-11-25",
  },
]);

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false;
  }, 1000);
});
</script>

<template>
  <div class="h-full p-4 mt-6">
    <div class="mt-4">
      <div v-show="pageMeta.showHeader">
        <USkeleton v-if="isLoading" class="h-8 w-50" />
        <h1 v-else class="text-2xl font-bold">{{ pageMeta.header.name }}</h1>

        <USkeleton v-if="isLoading" class="mt-2 h-6 w-100" />
        <p v-else class="mt-2 text-gray-600 dark:text-gray-400">
          {{ pageMeta.header.description }}
        </p>
      </div>

      <div v-if="isLoading" class="p-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-4 gap-y-4">
        <USkeleton v-for="i in 6" :key="i" class="h-70 w-full" />
      </div>
      <div v-else class="p-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-4 gap-y-4">
        <GameCard v-for="game in games" :key="game.title" :game="game" />
      </div>
    </div>
  </div>
</template>
