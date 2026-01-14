<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";
import type { GameMeta } from "~/interfaces/game";

definePageMeta({
  layout: "home",
});

const UAvatar = resolveComponent("UAvatar");
const UButton = resolveComponent("UButton");
const DownloadDrawer = resolveComponent("DownloadDrawer");
const accountStore = useAccountStore();
const gameStore = useGameStore();
const toast = useToast();

const pageMeta = {
  header: {
    name: `${accountStore.account?.displayName}'s Library`,
    description: "Manage your game collection.",
  },
  showHeader: true,
};

const isLoading = ref(true);

onMounted(async () => {
  try {
    await gameStore.loadData();
  } catch (error) {
    toast.add({ title: "Failed to load games:", color: "error" });
  } finally {
    isLoading.value = false;
  }
});

// Format date helper
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
};

// Table columns
const columns: TableColumn<GameMeta>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "title",
    header: "Game",
    cell: ({ row }) => {
      return h("div", { class: "flex items-center gap-3" }, [
        h(UAvatar, {
          src: row.original.coverUrl,
          size: "lg",
        }),
        h("div", undefined, [
          h(
            "p",
            { class: "font-medium text-gray-900 dark:text-white" },
            row.original.title
          ),
          h(
            "p",
            { class: "text-sm text-gray-500 dark:text-gray-400" },
            row.original.tags?.join(" • ") || "No tags"
          ),
        ]),
      ]);
    },
  },
  {
    accessorKey: "createdAt",
    header: "Added",
    cell: ({ row }) => {
      return h(
        "p",
        { class: "text-sm text-gray-600 dark:text-gray-400" },
        formatDate(row.original.createdAt)
      );
    },
  },
  {
    accessorKey: "actions",
    header: "",
    cell: ({ row }) => {
      return h("div", { class: "flex items-center gap-2" }, [
        h(DownloadDrawer, {
          game: row.original,
        }),
        h(UButton, {
          variant: "ghost",
          size: "sm",
          color: "error",
          class: "cursor-pointer",
          icon: "i-heroicons-trash",
          onClick: async () => {
            try {
              await accountStore.removeFromLibrary(row.original.id);
            } catch (error) {
              toast.add({
                title: "Failed to remove game from library",
                color: "error",
              });
            }
          },
        }),
      ]);
    },
  },
];
</script>

<template>
  <div class="min-h-screen p-10">
    <div class="max-w-7xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <USkeleton v-if="isLoading" class="h-12 w-64 mb-2" />
        <div v-else class="flex items-center justify-between mb-2">
          <div class="flex items-center gap-3">
            <div class="p-3 bg-primary-500/10 rounded-xl">
              <UIcon name="i-heroicons-book-open" class="w-8 h-8 text-primary-500" />
            </div>
            <h1 class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text">
              {{ pageMeta.header.name }}
            </h1>
          </div>
        </div>

        <USkeleton v-if="isLoading" class="h-6 w-96" />
        <p v-else class="text-gray-600 dark:text-gray-400 text-lg ml-14">
          {{ pageMeta.header.description }}
        </p>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="space-y-4">
        <USkeleton class="h-[340px] w-full rounded-lg" />
      </div>

      <!-- Empty State -->
      <div v-else-if="!accountStore.getLibraryGames.length" class="text-center py-20">
        <UCard class="max-w-md mx-auto">
          <div class="flex flex-col items-center gap-4">
            <UIcon name="i-heroicons-inbox" class="w-16 h-16 text-gray-400" />
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Your library is empty
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              Start adding games to your collection
            </p>
            <UButton label="Explore Games" icon="i-heroicons-magnifying-glass" to="/explore" size="lg" />
          </div>
        </UCard>
      </div>

      <!-- Grid View -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div v-for="item in accountStore.getLibraryGames" :key="item.id"
          class="relative group overflow-hidden rounded-lg shadow-lg">
          <img :src="item.coverUrl" :alt="item.title"
            class="h-[400px] w-full object-cover transition-transform duration-300 group-hover:scale-105"
            loading="lazy" />

          <!-- Overlay -->
          <div
            class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/80 to-transparent p-4 transition-transform duration-300 group-hover:scale-105">
            <h3 class="text-white font-semibold text-lg line-clamp-1 mb-1">
              {{ item.title }}
            </h3>
            <p v-if="item.tags?.length" class="text-sm text-gray-300 mb-2">
              {{ item.tags.join(" • ") }}
            </p>
            <DownloadDrawer :game="item" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>