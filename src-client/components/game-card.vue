<script setup lang="ts">
import type { GameMeta } from "~/interfaces/game";

const props = defineProps<{
  game: GameMeta;
}>();

const accountStore = useAccountStore();
const toast = useToast();
const isAdding = ref(false);
const imageLoaded = ref(true);

const defaultCover = "https://placehold.co/600x400/1e293b/94a3b8?text=No+Cover";

const coverImage = computed(() => {
  // If no URL provided or image failed to load, use default
  if (!props.game.coverUrl || !imageLoaded.value) {
    return defaultCover;
  }
  return props.game.coverUrl;
});

// Test if the image URL is actually accessible
const checkImageUrl = async (url: string) => {
  if (!url) {
    imageLoaded.value = false;
    return;
  }

  try {
    const img = new Image();
    img.onload = () => {
      imageLoaded.value = true;
    };
    img.onerror = () => {
      imageLoaded.value = false;
    };
    img.src = url;
  } catch {
    imageLoaded.value = false;
  }
};

// Check image on mount
onMounted(() => {
  if (props.game.coverUrl) {
    checkImageUrl(props.game.coverUrl);
  } else {
    imageLoaded.value = false;
  }
});

const isInLibrary = computed(() => accountStore.isInLibrary(props.game.id));

const handleAddToLibrary = async () => {
  if (!accountStore.isAuthenticated) {
    toast.add({
      title: "Login Required",
      description: "Please login to add games to your library",
      color: "error",
      icon: "i-heroicons-exclamation-circle",
    });
    return;
  }

  if (isInLibrary.value) {
    isAdding.value = true;
    try {
      await accountStore.removeFromLibrary(props.game.id);
      toast.add({
        title: "Removed from Library",
        description: `${props.game.title} has been removed`,
        color: "warning",
        icon: "i-heroicons-trash",
      });
    } catch (error: any) {
      toast.add({
        title: "Error",
        description: error.message || "Failed to remove game",
        color: "error",
        icon: "i-heroicons-exclamation-circle",
      });
    } finally {
      isAdding.value = false;
    }
    return;
  }

  isAdding.value = true;
  try {
    await accountStore.addToLibrary(props.game.id);
    toast.add({
      title: "Added to Library",
      description: `${props.game.title} has been added to your library`,
      color: "success",
      icon: "i-heroicons-check-circle",
    });
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to add game to library",
      color: "error",
      icon: "i-heroicons-exclamation-circle",
    });
  } finally {
    isAdding.value = false;
  }
};
</script>

<template>
  <UBlogPost
    :title="`${game.title}${game.isExperimental ? ' 🧪' : ''}`"
    :image="coverImage"
    :date="game.createdAt"
  >
    <template #footer>
      <UButton
        :icon="isInLibrary ? 'i-lucide-check' : 'i-lucide-plus'"
        :label="isInLibrary ? 'In Library' : 'Add to Library'"
        :variant="isInLibrary ? 'solid' : 'ghost'"
        :color="isInLibrary ? 'primary' : undefined"
        :loading="isAdding"
        :disabled="isAdding"
        class="w-full cursor-pointer rounded-t-none"
        @click="handleAddToLibrary"
      />
    </template>
  </UBlogPost>
</template>
