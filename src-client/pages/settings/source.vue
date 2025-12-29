<script setup lang="ts">
  definePageMeta({
    layout: "home",
  });
  
  const isLoading = ref(true);
  
  const pageMeta = {
    header: {
      name: "🕹️ Source Repository",
      description: "Here are the available games you may download.",
    },
    showHeader: true,
  };
  
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
      </div>
    </div>
  </template>
  