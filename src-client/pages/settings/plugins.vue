<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  const pageMeta = {
    header: {
      name: "🔌 Plugins",
      description: "Additional feature you may add to NxShard.",
    },
    showHeader: true,
  };
  
  definePageMeta({
    layout: "home",
  });
  
  const toast = useToast();
  const isLoading = ref(true);
  
  /* =======================
         TYPES
      ======================= */
  
  interface PluginItem {
    id: string;
    name: string;
    link: string;
  }
  
  /* =======================
         STATE
      ======================= */
  
  const availablePlugins = ref<PluginItem[]>([]);
  const installedPlugins = ref<PluginItem[]>([]);
  const isPluginInstalled = ref<boolean>(false);
  const isInstalling = ref<boolean>(false);
  const isUninstalling = ref<boolean>(false);
  
  /* =======================
         COMPUTED
      ======================= */
  
  const animePlugin = computed(() => {
    return availablePlugins.value.find((p) => p.id === "shard-anime");
  });
  
  /* =======================
         METHODS
      ======================= */
  
  async function loadPlugins() {
    try {
      availablePlugins.value = await invoke<PluginItem[]>("get_available_plugins");
      installedPlugins.value = await invoke<PluginItem[]>("get_installed_plugins");
      
      // Check if anime plugin is installed
      isPluginInstalled.value = installedPlugins.value.some(
        (p) => p.id === "shard-anime"
      );
    } catch (error: any) {
      console.error("Failed to load plugins:", error);
      toast.add({
        title: "Error",
        description: "Failed to load plugin information",
        color: "error",
      });
    }
  }
  
  async function installAnimePlugin() {
    if (!animePlugin.value) return;
  
    isInstalling.value = true;
    try {
      await invoke("install_plugin", { pluginId: animePlugin.value.id });
      
      toast.add({
        title: "Plugin Installed",
        description: "Anime streaming plugin has been installed successfully",
        color: "primary",
      });
  
      // Restart the plugin to start it
      await invoke("restart_plugin", { pluginId: animePlugin.value.id });
      
      // Reload plugin list
      await loadPlugins();
    } catch (error: any) {
      console.error("Failed to install plugin:", error);
      toast.add({
        title: "Installation Failed",
        description: error || "Failed to install anime streaming plugin",
        color: "error",
      });
    } finally {
      isInstalling.value = false;
    }
  }
  
  async function uninstallAnimePlugin() {
    if (!animePlugin.value) return;
  
    isUninstalling.value = true;
    try {
      await invoke("remove_plugin", { pluginId: animePlugin.value.id });
      
      toast.add({
        title: "Plugin Removed",
        description: "Anime streaming plugin has been removed",
        color: "primary",
      });
  
      // Reload plugin list
      await loadPlugins();
    } catch (error: any) {
      console.error("Failed to remove plugin:", error);
      toast.add({
        title: "Removal Failed",
        description: error || "Failed to remove anime streaming plugin",
        color: "error",
      });
    } finally {
      isUninstalling.value = false;
    }
  }
  
  async function toggleAnimePlugin(enabled: boolean) {
    if (enabled) {
      await installAnimePlugin();
    } else {
      await uninstallAnimePlugin();
    }
  }
  
  /* =======================
         LIFECYCLE
      ======================= */
  
  onMounted(async () => {
    await loadPlugins();
    
    setTimeout(() => {
      isLoading.value = false;
    }, 500);
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
  
        <div class="p-4 space-y-4">
          <USkeleton v-if="isLoading" class="h-16 w-full" />
          
          <div v-else class="flex items-center justify-between p-4 border rounded-lg dark:border-gray-700">
            <div class="flex-1">
              <h3 class="font-semibold">Anime Streaming Service</h3>
              <p class="text-sm text-gray-600 dark:text-gray-400">
                Watch anime powered by yahyaMomin/hianime-API
              </p>
            </div>
            
            <div class="flex items-center gap-4">
              <UBadge 
                v-if="isPluginInstalled" 
                color="success" 
                variant="subtle"
              >
                Installed
              </UBadge>
              
              <UButton
                v-if="!isPluginInstalled"
                @click="installAnimePlugin"
                :loading="isInstalling"
                :disabled="isInstalling"
                color="primary"
              >
                {{ isInstalling ? "Installing..." : "Install" }}
              </UButton>
              
              <UButton
                v-else
                @click="uninstallAnimePlugin"
                :loading="isUninstalling"
                :disabled="isUninstalling"
                color="error"
                variant="soft"
              >
                {{ isUninstalling ? "Removing..." : "Remove" }}
              </UButton>
            </div>
          </div>
  
          <!-- Alternative: Using Switch Component -->
          <!-- 
          <USwitch
            size="lg"
            v-model="isPluginInstalled"
            @update:model-value="toggleAnimePlugin"
            :disabled="isInstalling || isUninstalling"
            label="Enable Anime Streaming Service"
            description="By enabling this mode, you'll be able to watch anime powered by yahyaMomin/hianime-API"
            class="p-2"
          />
          -->
        </div>
      </div>
    </div>
  </template>