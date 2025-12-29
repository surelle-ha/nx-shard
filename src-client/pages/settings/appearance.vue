<script setup lang="ts">
  definePageMeta({
    layout: "home",
  });
  
  const isLoading = ref(true);
  const pageMeta = {
    header: {
      name: "👁️ Appearance",
      description: "Here are the available games you may download.",
    },
    showHeader: true,
  };
  onMounted(() => {
    setTimeout(() => {
      isLoading.value = false;
    }, 1000);
  });

  const lightModeValue = ref(false)
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
        <div class="p-4">
          <UColorModeSwitch size="lg" v-model="lightModeValue" label="Change dark/light mode" description="This is a checkbox." class="p-2"/>
        </div>
      </div>
    </div>
  </template>
  