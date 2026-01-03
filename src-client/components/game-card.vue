<script setup lang="ts">
  import type { GameMeta } from "~/interfaces/game";
  
  const props = defineProps<{
    game: GameMeta;
  }>();
  
  const accountStore = useAccountStore();
  const toast = useToast();
  const isAdding = ref(false);
  
  const isInLibrary = computed(() => accountStore.isInLibrary(props.game.id));
  
  const handleAddToLibrary = async () => {
    if (!accountStore.isAuthenticated) {
      toast.add({
        title: 'Login Required',
        description: 'Please login to add games to your library',
        color: 'error',
        icon: 'i-heroicons-exclamation-circle'
      });
      return;
    }
  
    if (isInLibrary.value) {
      isAdding.value = true;
      try {
        await accountStore.removeFromLibrary(props.game.id);
        toast.add({
          title: 'Removed from Library',
          description: `${props.game.title} has been removed`,
          color: 'warning',
          icon: 'i-heroicons-trash'
        });
      } catch (error: any) {
        toast.add({
          title: 'Error',
          description: error.message || 'Failed to remove game',
          color: 'error',
          icon: 'i-heroicons-exclamation-circle'
        });
      } finally {
        isAdding.value = false;
      }
      return;
    }
  
    // Add to library
    isAdding.value = true;
    try {
      await accountStore.addToLibrary(props.game.id);
      toast.add({
        title: 'Added to Library',
        description: `${props.game.title} has been added to your library`,
        color: 'success',
        icon: 'i-heroicons-check-circle'
      });
    } catch (error: any) {
      toast.add({
        title: 'Error',
        description: error.message || 'Failed to add game to library',
        color: 'error',
        icon: 'i-heroicons-exclamation-circle'
      });
    } finally {
      isAdding.value = false;
    }
  };
  </script>
  
  <template>
    <UBlogPost
      :title="`${game.title}${game.isExperimental ? ' 🧪' : ''}`"
      :image="game.coverUrl"
      :date="game.createdAt"
    >
      <template #footer>
        <UButton
          :icon="isInLibrary ? 'i-lucide-check' : 'i-lucide-plus'"
          :label="isInLibrary ? 'In Library' : 'Add to Library'"
          :variant="isInLibrary ? 'solid' : 'ghost'"
          :color="isInLibrary ? 'success' : undefined"
          :loading="isAdding"
          :disabled="isAdding"
          class="w-full cursor-pointer rounded-t-none"
          @click="handleAddToLibrary"
        />
      </template>
    </UBlogPost>
  </template>