import { defineStore } from "pinia";
import { ref } from "vue";

export const useTransferProtocolStore = defineStore("transferProtocol", () => {
  const isMTPActive = ref(false);
  const isFTPActive = ref(false);

  return {
    isMTPActive,
    isFTPActive,
  };
});
