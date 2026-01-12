<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { storeToRefs } from "pinia";
import type { GameFile } from "~/interfaces/ftp";

const toast = useToast();
const transferProtocolStore = useTransferProtocolStore();
const ftpManagerStore = useFTPManagerStore();
const accountStore = useAccountStore();

const isFtpInstall = computed(() => accountStore.account?.isFtpInstall);
const isMtpInstall = computed(() => accountStore.account?.isMtpInstall);

/* -----------------------------
 * STORES
 * ----------------------------- */
const { isMTPActive, isFTPActive, ftpIps, primaryFtpIp } = storeToRefs(
  transferProtocolStore
);

const {
  availableGames,
  transferQueue,
  currentTransfer,
  isTransferring,
  isScanning,
  error,
  ftpIp,
  formattedTransferSpeed,
  formattedETA,
} = storeToRefs(ftpManagerStore);

// ⚠️ IMPORTANT: function getter MUST be taken directly
const { formattedFileSize } = ftpManagerStore;

/* -----------------------------
 * LOCAL STATE
 * ----------------------------- */
const openMTP = ref(false);
const openFTP = ref(false);
const selectedGame = ref<GameFile | null>(null);
const manualIpInput = ref("");
const showManualIpInput = ref(false);

/* -----------------------------
 * FTP STATUS (FIXED)
 * ----------------------------- */
const isFTPDetected = computed(() => ftpIps.value.length > 0);
const isFTPConnected = computed(() => ftpIp.value !== null);

/* -----------------------------
 * SIDEBAR METHODS
 * ----------------------------- */
const method = computed(() => {
  const methods: any[] = [];

  if (isFtpInstall.value) {
    methods.push({
      label: "Install with FTP",
      icon: "i-lucide-wifi",
      active: isFTPActive.value,
      class: "cursor-pointer",
      id: "ftp",
    });
  }

  if (isMtpInstall.value) {
    methods.push({
      label: "Install with MTP",
      icon: "i-lucide-usb",
      active: isMTPActive.value,
      disable: true,
      class: "cursor-pointer",
      id: "mtp",
    });
  }

  return methods;
});

/* -----------------------------
 * HANDLERS
 * ----------------------------- */
const handleClick = (id: string) => {
  if (id === "mtp") {
    if (!isMTPActive.value) {
      toast.add({
        title: "No Nintendo Switch detected via MTP.",
        color: "warning",
      });
      return;
    }

    openFTP.value = false;
    openMTP.value = !openMTP.value;
    return;
  }

  if (id === "ftp") {
    openMTP.value = false;
    openFTP.value = !openFTP.value;

    if (openFTP.value && isFTPConnected.value) {
      scanGames();
    }

    if (!isFTPDetected.value) {
      toast.add({
        title: "No Nintendo Switch detected via FTP.",
        description: "You can enter an IP address manually.",
        color: "warning",
      });
    }
  }
};

/* -----------------------------
 * FTP ACTIONS
 * ----------------------------- */
const scanGames = async () => {
  try {
    await ftpManagerStore.scanGameFiles();
    toast.add({
      title: `Found ${availableGames.value.length} game files`,
      color: "success",
    });
  } catch (err) {
    toast.add({
      title: "Failed to scan games",
      description: String(err),
      color: "error",
    });
  }
};

const setManualIp = async () => {
  if (!manualIpInput.value.trim()) {
    toast.add({ title: "Invalid IP address", color: "error" });
    return;
  }

  try {
    await transferProtocolStore.selectFtpIp(manualIpInput.value.trim());
    showManualIpInput.value = false;
    manualIpInput.value = "";
    await scanGames();
  } catch (err) {
    toast.add({
      title: "Failed to set FTP IP",
      description: String(err),
      color: "error",
    });
  }
};

const selectDetectedIp = async (ip: string) => {
  try {
    await transferProtocolStore.selectFtpIp(ip);
    await scanGames();
  } catch (err) {
    toast.add({
      title: "Failed to select IP",
      description: String(err),
      color: "error",
    });
  }
};

const queueGame = async (game: GameFile) => {
  await ftpManagerStore.queueFile(game);
};

const removeFromQueue = async (filePath: string) => {
  await ftpManagerStore.removeFromQueue(filePath);
};

const clearQueue = async () => {
  await ftpManagerStore.clearQueue();
};

/* -----------------------------
 * AUTO-CONNECT PRIMARY IP (KEY FIX)
 * ----------------------------- */
watch(
  () => primaryFtpIp.value,
  async (ip) => {
    if (!ip) return;
    if (ftpIp.value) return;

    try {
      await ftpManagerStore.setFtpIp(ip);
      await scanGames();
    } catch (err) {
      console.error("Auto-connect failed:", err);
    }
  },
  { immediate: true }
);

/* -----------------------------
 * SHORTCUTS
 * ----------------------------- */
defineShortcuts({
  m: () => handleClick("mtp"),
  f: () => handleClick("ftp"),
});

/* -----------------------------
 * LIFECYCLE
 * ----------------------------- */
onMounted(async () => {
  await transferProtocolStore.initFTPListener();
  await ftpManagerStore.initListeners();
  await transferProtocolStore.startFTPMonitor(30);
  await ftpManagerStore.getFtpIp();
  await ftpManagerStore.refreshQueue();
  await ftpManagerStore.checkIsTransferring();
});

onUnmounted(() => {
  transferProtocolStore.cleanup();
  ftpManagerStore.cleanup();
});
</script>

<template>
  <div class="flex flex-col gap-2">
    <div
      v-for="(item, index) in method"
      :key="index"
      class="flex items-center gap-2 w-full cursor-pointer"
      @click="handleClick(item.id)"
    >
      <div class="flex-1">
        <UPageAnchors :links="[item]" class="cursor-pointer" />
      </div>
      <UKbd>{{ index === 0 ? "M" : "F" }}</UKbd>
    </div>
  </div>

  <UModal v-model:open="openMTP" title="Installation via MTP">
    <template #body>
      <Placeholder class="h-48" />
    </template>
  </UModal>

  <UModal
    v-model:open="openFTP"
    title="Installation via FTP"
    class="shard-scroll"
  >
    <template #body>
      <div class="flex flex-col gap-4 p-4">
        <!-- FTP Connection Status -->
        <div
          class="border border-gray-200 dark:border-gray-800 rounded-lg p-4 bg-gray-50 dark:bg-gray-900"
        >
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">
              FTP Connection
            </h3>
            <UBadge :color="ftpIp ? 'success' : 'neutral'">
              {{ ftpIp ? "Connected" : "Not Connected" }}
            </UBadge>
          </div>

          <!-- Current IP Display -->
          <div v-if="ftpIp" class="mb-3">
            <p class="text-xs text-gray-600 dark:text-gray-400 mb-1">
              Current IP:
            </p>
            <div class="flex items-center gap-2">
              <code
                class="text-sm bg-white dark:bg-gray-800 px-2 py-1 rounded"
                >{{ ftpIp }}</code
              >
              <UButton
                icon="i-lucide-refresh-cw"
                color="neutral"
                variant="ghost"
                size="xs"
                @click="scanGames"
                title="Refresh connection"
              />
            </div>
          </div>

          <!-- Detected IPs -->
          <div v-if="ftpIps.length > 0" class="mb-3">
            <p class="text-xs text-gray-600 dark:text-gray-400 mb-2">
              Detected devices:
            </p>
            <div class="flex flex-wrap gap-2">
              <UButton
                v-for="ip in ftpIps"
                :key="ip"
                :color="ip === ftpIp ? 'primary' : 'neutral'"
                variant="soft"
                size="xs"
                @click="selectDetectedIp(ip)"
              >
                {{ ip }}
              </UButton>
            </div>
          </div>

          <!-- Manual IP Entry -->
          <div>
            <UButton
              v-if="!showManualIpInput"
              icon="i-lucide-pencil"
              color="neutral"
              variant="ghost"
              size="xs"
              @click="showManualIpInput = true"
            >
              Enter IP Manually
            </UButton>

            <div v-else class="flex gap-2">
              <UInput
                v-model="manualIpInput"
                placeholder="192.168.1.100"
                size="sm"
                class="flex-1"
                @keyup.enter="setManualIp"
              />
              <UButton
                icon="i-lucide-check"
                color="primary"
                size="sm"
                @click="setManualIp"
              />
              <UButton
                icon="i-lucide-x"
                color="neutral"
                variant="ghost"
                size="sm"
                @click="
                  showManualIpInput = false;
                  manualIpInput = '';
                "
              />
            </div>
          </div>

          <!-- Warning if not connected -->
          <UAlert
            v-if="!ftpIp"
            icon="i-lucide-alert-triangle"
            color="warning"
            title="No FTP connection"
            description="Please wait for auto-detection or enter an IP address manually."
            class="mt-3"
          />
        </div>

        <!-- Error Display -->
        <UAlert
          v-if="error"
          icon="i-lucide-alert-circle"
          color="error"
          :title="error"
          :close-button="{ icon: 'i-lucide-x', color: 'gray', variant: 'link' }"
          @close="ftpManagerStore.clearError()"
        />

        <!-- Current Transfer Progress -->
        <div
          v-if="currentTransfer"
          class="border border-gray-200 dark:border-gray-800 rounded-lg p-4"
        >
          <div class="flex items-center justify-between mb-2">
            <h3 class="text-lg font-semibold">Current Transfer</h3>
            <UBadge color="secondary">{{ currentTransfer.status }}</UBadge>
          </div>

          <div class="space-y-2">
            <div class="flex justify-between text-sm">
              <span class="text-gray-600 dark:text-gray-400">{{
                currentTransfer.fileName
              }}</span>
              <span class="font-medium"
                >{{ currentTransfer.progressPercent.toFixed(1) }}%</span
              >
            </div>

            <UProgress :value="currentTransfer.progressPercent" :max="100" />

            <div
              class="flex justify-between text-xs text-gray-600 dark:text-gray-400"
            >
              <span
                >{{ formattedFileSize(currentTransfer.bytesTransferred) }} /
                {{ formattedFileSize(currentTransfer.totalBytes) }}</span
              >
              <span
                >{{ formattedTransferSpeed }} • ETA: {{ formattedETA }}</span
              >
            </div>
          </div>
        </div>

        <!-- Transfer Queue -->
        <div class="border border-gray-200 dark:border-gray-800 rounded-lg p-4">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-semibold">
              Transfer Queue ({{ transferQueue.length }})
            </h3>
            <UButton
              v-if="transferQueue.length > 0"
              color="error"
              variant="ghost"
              size="xs"
              @click="clearQueue"
              :disabled="isTransferring"
            >
              Clear Queue
            </UButton>
          </div>

          <div
            v-if="transferQueue.length === 0"
            class="text-center py-8 text-gray-500"
          >
            No files in queue
          </div>

          <div v-else class="space-y-2">
            <div
              v-for="(game, index) in transferQueue"
              :key="game.filePath"
              class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-900 rounded"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">{{ game.fileName }}</p>
                <p class="text-xs text-gray-600 dark:text-gray-400">
                  {{ formattedFileSize(game.fileSize) }}
                </p>
              </div>
              <UButton
                v-if="index !== 0 || !isTransferring"
                icon="i-lucide-x"
                color="neutral"
                variant="ghost"
                size="xs"
                @click="removeFromQueue(game.filePath)"
              />
            </div>
          </div>
        </div>

        <!-- Available Games -->
        <div class="border border-gray-200 dark:border-gray-800 rounded-lg p-4">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-semibold">Available Games</h3>
            <UButton
              icon="i-lucide-refresh-cw"
              color="neutral"
              variant="ghost"
              size="xs"
              @click="scanGames"
              :loading="isScanning"
              :disabled="!ftpIp"
            >
              Refresh
            </UButton>
          </div>

          <div v-if="!ftpIp" class="text-center py-8 text-gray-500">
            <UIcon name="i-lucide-wifi-off" class="text-3xl mb-2" />
            <p>Connect to FTP first</p>
          </div>

          <div v-else-if="isScanning" class="text-center py-8">
            <UIcon name="i-lucide-loader-2" class="animate-spin text-2xl" />
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
              Scanning for games...
            </p>
          </div>

          <div
            v-else-if="availableGames.length === 0"
            class="text-center py-8 text-gray-500"
          >
            No game files found. Download some games first!
          </div>

          <div v-else class="space-y-2 max-h-80 overflow-y-auto">
            <div
              v-for="game in availableGames"
              :key="game.filePath"
              class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            >
              <div class="flex-1">
                <p class="text-sm font-medium">{{ game.gameTitle }}</p>
                <p class="text-xs text-gray-600 dark:text-gray-400">
                  {{ game.fileName }} • {{ formattedFileSize(game.fileSize) }}
                </p>
              </div>
              <UButton
                icon="i-lucide-plus"
                color="primary"
                size="xs"
                @click="queueGame(game)"
              >
                Queue
              </UButton>
            </div>
          </div>
        </div>
      </div>
    </template>
  </UModal>
</template>
