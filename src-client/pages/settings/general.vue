<script setup lang="ts">
  const pageMeta = {
    header: {
      name: "⚙️ General",
      description: "Here are the available games you may download.",
    },
    showHeader: true,
  };
  definePageMeta({
    layout: "home",
  });
  
  const accountStore = useAccountStore();
  const toast = useToast();
  const isLoading = ref(true);
  
  onMounted(() => {
    setTimeout(() => {
      isLoading.value = false;
    }, 1000);
  });
  
  const experimentValue = ref(accountStore.account?.isExperimental || false);
  watch(experimentValue, async (newValue) => {
    if (accountStore.account) {
      try {
        await accountStore.updateUserConfig({ isExperimental: newValue });
        toast.add({
          title: "Settings updated",
          description: `Experimental mode ${newValue ? 'enabled' : 'disabled'}`,
          color: "primary",
        });
      } catch (error: any) {
        toast.add({
          title: "Error",
          description: error.message || "Failed to update settings",
          color: "error",
        });
        experimentValue.value = !newValue;
      }
    }
  });

  const darkmodeValue = ref(accountStore.account?.isDarkmode || false);
  watch(darkmodeValue, async (newValue) => {
    if (accountStore.account) {
      try {
        await accountStore.updateUserConfig({ isDarkmode: newValue });
        toast.add({
          title: "Settings updated",
          description: `Dark mode ${newValue ? 'enabled' : 'disabled'}`,
          color: "primary",
        });
      } catch (error: any) {
        toast.add({
          title: "Error",
          description: error.message || "Failed to update settings",
          color: "error",
        });
        darkmodeValue.value = !newValue;
      }
    }
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
        <div class="p-4">
          <USwitch
            size="lg"
            v-model="experimentValue"
            label="Enable Experimental Mode"
            description="By enabling this mode, you'll be able to use untested features and games."
            class="p-2"
          />
          <UColorModeSwitch size="lg" v-model="darkmodeValue" label="Change dark/light mode" description="This is a checkbox." class="p-2"/>
        </div>
      </div>
    </div>
  </template>