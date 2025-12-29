<script setup lang="ts">
import type { GameMeta } from "~/interfaces/game";

const props = defineProps<{
  game: GameMeta;
}>();
</script>

<template>
  <UBlogPost
    :title="game.title"
    :image="game.coverUrl"
    :date="game.createdAt"
  >
    <template #footer>
      <DownloadDrawer :game="game" />
    </template>
  </UBlogPost>
</template>
