<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

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

/* =======================
       AUTOSTART HELPERS
    ======================= */

const isAutostartEnabled = async (): Promise<boolean> => {
  return await invoke("plugin:autostart|is_enabled");
};

const enableAutostart = async (): Promise<void> => {
  await invoke("plugin:autostart|enable");
};

const disableAutostart = async (): Promise<void> => {
  await invoke("plugin:autostart|disable");
};

/* =======================
       STATE
    ======================= */

// Initialize with default values, will be updated in onMounted
const experimentValue = ref<boolean>(false);
const darkmodeValue = ref<boolean>(false);
const autostartValue = ref<boolean>(false);
const animatedHomeValue = ref<boolean>(false);
const debugModeValue = ref<boolean>(false);
const ftpInstallValue = ref<boolean>(false);
const mtpInstallValue = ref<boolean>(false);

// prevents watcher from firing on initial load
const isAutostartInitialized = ref(false);
const isInitialized = ref(false);

/* =======================
       LIFECYCLE
    ======================= */

onMounted(async () => {
  // Wait for account store to be initialized if it's still loading
  if (accountStore.isLoading) {
    // Wait for the loading to complete
    await new Promise<void>((resolve) => {
      const unwatch = watch(
        () => accountStore.isLoading,
        (loading) => {
          if (!loading) {
            unwatch();
            resolve();
          }
        }
      );
    });
  }

  // If account is still not loaded, try to initialize it
  if (!accountStore.account) {
    try {
      await accountStore.initializeUser();
    } catch (error) {
      console.error("Failed to initialize user:", error);
    }
  }

  // Initialize values from store after component mounts
  if (accountStore.account) {
    experimentValue.value = accountStore.account.isExperimental || false;
    darkmodeValue.value = accountStore.account.isDarkmode || false;
    animatedHomeValue.value = accountStore.account.isAnimatedHome ?? true;
    debugModeValue.value = accountStore.account.isLogEnable || false;
    ftpInstallValue.value = accountStore.account.isFtpInstall ?? true;
    mtpInstallValue.value = accountStore.account.isMtpInstall ?? true;
  }

  setTimeout(() => {
    isLoading.value = false;
  }, 1000);

  try {
    autostartValue.value = await isAutostartEnabled();
  } finally {
    isAutostartInitialized.value = true;
    // Mark as initialized after all values are set
    isInitialized.value = true;
  }
});

/* =======================
       WATCHERS
    ======================= */

watch(experimentValue, async (newValue) => {
  if (!accountStore.account || !isInitialized.value) return;

  try {
    await accountStore.updateUserConfig({ isExperimental: newValue });
    toast.add({
      title: "Settings updated",
      description: `Experimental mode ${newValue ? "enabled" : "disabled"}`,
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
});

watch(darkmodeValue, async (newValue) => {
  if (!accountStore.account || !isInitialized.value) return;

  try {
    await accountStore.updateUserConfig({ isDarkmode: newValue });
    toast.add({
      title: "Settings updated",
      description: `Dark mode ${newValue ? "enabled" : "disabled"}`,
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
});

watch(autostartValue, async (newValue) => {
  if (!isAutostartInitialized.value) return;

  try {
    if (newValue) {
      await enableAutostart();
    } else {
      await disableAutostart();
    }
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to update autostart",
      color: "error",
    });
    autostartValue.value = !newValue;
  }
});

watch(animatedHomeValue, async (newValue) => {
  if (!accountStore.account || !isInitialized.value) return;

  try {
    await accountStore.updateUserConfig({ isAnimatedHome: newValue });
    toast.add({
      title: "Settings updated",
      description: `Animated home page ${newValue ? "enabled" : "disabled"}`,
      color: "primary",
    });
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to update settings",
      color: "error",
    });
    animatedHomeValue.value = !newValue;
  }
});

watch(debugModeValue, async (newValue) => {
  if (!accountStore.account || !isInitialized.value) return;

  try {
    await accountStore.updateUserConfig({ isLogEnable: newValue });
    toast.add({
      title: "Settings updated",
      description: `Debug mode ${newValue ? "enabled" : "disabled"}`,
      color: "primary",
    });
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to update settings",
      color: "error",
    });
    debugModeValue.value = !newValue;
  }
});

watch(ftpInstallValue, async (newValue) => {
  if (!accountStore.account || !isInitialized.value) return;

  try {
    await accountStore.updateUserConfig({ isFtpInstall: newValue });
    toast.add({
      title: "Settings updated",
      description: `FTP Installation ${newValue ? "enabled" : "disabled"}`,
      color: "primary",
    });
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to update settings",
      color: "error",
    });
    ftpInstallValue.value = !newValue;
  }
});

watch(mtpInstallValue, async (newValue) => {
  if (!accountStore.account || !isInitialized.value) return;

  try {
    await accountStore.updateUserConfig({ isMtpInstall: newValue });
    toast.add({
      title: "Settings updated",
      description: `MTP Installation ${newValue ? "enabled" : "disabled"}`,
      color: "primary",
    });
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to update settings",
      color: "error",
    });
    mtpInstallValue.value = !newValue;
  }
});

// Watch for account changes (in case it loads after component mounts)
watch(
  () => accountStore.account,
  (newAccount) => {
    if (newAccount && !isInitialized.value) {
      experimentValue.value = newAccount.isExperimental || false;
      darkmodeValue.value = newAccount.isDarkmode || false;
      animatedHomeValue.value = newAccount.isAnimatedHome ?? true;
      debugModeValue.value = newAccount.isLogEnable || false;
      ftpInstallValue.value = newAccount.isFtpInstall ?? true;
      mtpInstallValue.value = newAccount.isMtpInstall ?? true;
    }
  },
  { deep: true }
);
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

        <UColorModeSwitch
          size="lg"
          v-model="darkmodeValue"
          label="Change dark/light mode"
          description="This is a checkbox."
          class="p-2"
        />

        <USwitch
          size="lg"
          v-model="autostartValue"
          label="Enable autostart on boot"
          description="Launch the app automatically when your system starts."
          class="p-2"
        />

        <USwitch
          size="lg"
          v-model="animatedHomeValue"
          label="Enable home page animated background."
          description="This will enable animated background on your home page."
          class="p-2"
        />

        <USwitch
          size="lg"
          v-model="debugModeValue"
          label="Enable debug mode"
          description="This will log all files in your device. <Feature Broken>"
          class="p-2"
          disabled
        />

        <USwitch
          size="lg"
          v-model="ftpInstallValue"
          label="Enable FTP Installation"
          description="By enabling this, you'll be able to install games via FTP"
          class="p-2"
        />

        <USwitch
          size="lg"
          v-model="mtpInstallValue"
          label="Enable MTP Installation"
          description="By enabling this, you'll be able to install games via MTP"
          class="p-2"
        />
      </div>
    </div>
  </div>
</template>
