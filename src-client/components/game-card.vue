<script setup lang="ts">
import type { GameMeta } from "~/interfaces/game";

const props = defineProps<{
  game: GameMeta;
}>();
</script>

<template>
  <UBlogPost
    :title="game.title"
    :image="game.image"
    :date="game.date"
  >
    <template #footer>
      <DownloadDrawer :game="game" />
    </template>
  </UBlogPost>
</template>
