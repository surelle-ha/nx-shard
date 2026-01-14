<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";
import type { GameMeta } from "~/interfaces/game";

definePageMeta({
  layout: "home",
});

const UAvatar = resolveComponent("UAvatar");
const UButton = resolveComponent("UButton");
const accountStore = useAccountStore();
const gameStore = useGameStore();

const pageMeta = {
  header: {
    name: `🎮 Game Management`,
    description: "Manage game and torrent collections.",
  },
  showHeader: true,
};

const isLoading = ref(true);

onMounted(async () => {
  try {
    await gameStore.loadData();
    gameStore.subscribeToChanges();
  } catch (error) {
    console.error("Failed to load games:", error);
  } finally {
    isLoading.value = false;
  }
});

onUnmounted(() => {
  gameStore.unsubscribe();
});

const data = computed(() => gameStore.gameList || []);

// Format date helper
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
};

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
    accessorKey: "downloadUrl",
    header: "Download",
    cell: ({ row }) => {
      return h("div", { class: "flex items-center gap-2" }, [
        h(UButton, {
          variant: "ghost",
          size: "sm",
          label: "Download",
          icon: "i-heroicons-arrow-down-tray",
          disabled: !row.original.downloadUrl,
          onClick: () => {
            if (row.original.downloadUrl) {
              window.open(row.original.downloadUrl, "_blank");
            }
          },
        }),
      ]);
    },
  },
  {
    accessorKey: "actions",
    header: "",
    cell: ({ row }) => {
      return h("div", { class: "flex items-center gap-2" }, [
        h(UButton, {
          variant: "ghost",
          size: "sm",
          class: "cursor-pointer",
          icon: "i-heroicons-pencil-square",
          onClick: () => {
            navigateTo(`/admin/games/${row.original.id}`);
          },
        }),
        h(UButton, {
          variant: "ghost",
          size: "sm",
          class: "cursor-pointer",
          color: "error",
          icon: "i-heroicons-trash",
          onClick: async () => {
            if (confirm(`Delete ${row.original.title}?`)) {
              try {
                const supabase = useSupabase();
                const { error } = await supabase
                  .from("games")
                  .delete()
                  .eq("id", row.original.id);

                if (error) throw error;
              } catch (error) {
                console.error("Failed to delete game:", error);
                alert("Failed to delete game");
              }
            }
          },
        }),
      ]);
    },
  },
];
</script>

<template>
  <div
    class="min-h-screen p-6"
  >
    <div class="max-w-7xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <div class="flex items-center justify-between">
          <div>
            <USkeleton v-if="isLoading" class="h-12 w-64 mb-2" />
            <div v-else class="flex items-center gap-3 mb-2">
              <div class="p-3 bg-primary-500/10 rounded-xl">
                <UIcon
                  name="i-heroicons-cube"
                  class="w-8 h-8 text-primary-500"
                />
              </div>
              <h1
                class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text text-transparent"
              >
                {{ pageMeta.header.name }}
              </h1>
            </div>

            <USkeleton v-if="isLoading" class="h-6 w-96" />
            <p v-else class="text-gray-600 dark:text-gray-400 text-lg ml-14">
              {{ pageMeta.header.description }}
            </p>
          </div>

          <UButton
            v-if="!isLoading"
            label="Add New Game"
            icon="i-heroicons-plus"
            size="lg"
            @click="navigateTo('/admin/games/new')"
          />
        </div>
      </div>

      <!-- Loading State -->
      <USkeleton v-if="isLoading" class="h-[400px] w-full rounded-lg" />

      <!-- Error State -->
      <UAlert
        v-else-if="gameStore.error"
        color="error"
        variant="subtle"
        :title="gameStore.error"
        icon="i-heroicons-exclamation-circle"
      />

      <!-- Empty State -->
      <div v-else-if="!data || data.length === 0" class="text-center py-20">
        <UCard class="max-w-md mx-auto">
          <div class="flex flex-col items-center gap-4">
            <UIcon
              name="i-heroicons-cube-transparent"
              class="w-16 h-16 text-gray-400"
            />
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              No games yet
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              Start by adding your first game
            </p>
            <UButton
              label="Add First Game"
              icon="i-heroicons-plus"
              size="lg"
              @click="navigateTo('/admin/games/new')"
            />
          </div>
        </UCard>
      </div>

      <!-- Table -->
      <UCard v-else class="shadow-xl border-0">
        <div class="flex items-center justify-between mb-6">
          <div>
            <h2 class="text-xl font-bold text-gray-900 dark:text-white">
              All Games
            </h2>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {{ data.length }} game(s) in collection
            </p>
          </div>
        </div>

        <UTable :data="data" :columns="columns" class="w-full" />
      </UCard>
    </div>
  </div>
</template>
