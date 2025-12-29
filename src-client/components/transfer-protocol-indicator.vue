<script setup lang="ts">
const transferProtocolStore = useTransferProtocolStore();

const openMTP = ref(false);
const openFTP = ref(false);

const method = ref([
  {
    label: "Install with MTP",
    icon: "i-lucide-usb",
    active: transferProtocolStore.isMTPActive,
    class: "cursor-pointer",
    id: "mtp",
  },
  {
    label: "Install with FTP",
    icon: "i-lucide-wifi",
    active: transferProtocolStore.isFTPActive,
    class: "cursor-pointer",
    id: "ftp",
  },
]);

const handleClick = (id: string) => {
  if (id === "mtp") {
    openFTP.value = false;
    openMTP.value = !openMTP.value;
  } else if (id === "ftp") {
    openMTP.value = false;
    openFTP.value = !openFTP.value;
  }
};

defineShortcuts({
  m: () => {
    openFTP.value = false;
    openMTP.value = !openMTP.value;
  },
  f: () => {
    openMTP.value = false;
    openFTP.value = !openFTP.value;
  },
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

  <UModal v-model:open="openFTP" title="Installation via FTP">
    <template #body>
      <Placeholder class="h-48" />
    </template>
  </UModal>
</template>
