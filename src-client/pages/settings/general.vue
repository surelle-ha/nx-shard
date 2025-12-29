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

const globalStore = useGlobalStore();
const isLoading = ref(true);

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false;
  }, 1000);
});

const nativeValue = ref(globalStore.isNativeWindowed);
watch(nativeValue, (newValue) => {
  globalStore.isNativeWindowed = newValue;
});

const reportValue = ref(globalStore.isErrorReportable);
watch(reportValue, (newValue) => {
  globalStore.isErrorReportable = newValue;
});

const appnotifValue = ref(globalStore.isInAppNotifEnabled);
watch(appnotifValue, (newValue) => {
  globalStore.isInAppNotifEnabled = newValue;
});

const desktnofifValue = ref(globalStore.isDesktopNotifEnabled);
watch(desktnofifValue, (newValue) => {
  globalStore.isDesktopNotifEnabled = newValue;
});

const experimentValue = ref(globalStore.isExperimental);
watch(experimentValue, (newValue) => {
  globalStore.isExperimental = newValue;
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
          v-model="nativeValue"
          label="Use Native Window tab"
          description="This is a checkbox."
          class="p-2"
          disabled
        />
        <USwitch
          size="lg"
          v-model="reportValue"
          label="Send Error Report/Logs"
          description="This is a checkbox."
          class="p-2"
          disabled
        />
        <USwitch
          size="lg"
          v-model="appnotifValue"
          label="In-App Notification"
          description="This is a checkbox."
          class="p-2"
          disabled
        />
        <USwitch
          size="lg"
          v-model="desktnofifValue"
          label="Desktop Notification"
          description="This will authorize NxShard notify you outside application."
          class="p-2"
          disabled
        />
        <USwitch
          size="lg"
          v-model="experimentValue"
          label="Enable Experimental Mode"
          description="By enabling this mode, you'll be able to use untested features and games."
          class="p-2"
        />
      </div>
    </div>
  </div>
</template>
