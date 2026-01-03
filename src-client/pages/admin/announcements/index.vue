<script setup lang="ts">
    import type { TableColumn } from "@nuxt/ui";
    import type { Announcement } from "~/stores/announcement";
    
    definePageMeta({
      layout: "home",
    });
    
    const UAvatar = resolveComponent("UAvatar");
    const UButton = resolveComponent("UButton");
    const accountStore = useAccountStore();
    const announcementStore = useAnnouncementStore();
    
    const pageMeta = {
      header: {
        name: `📰 Announcement Management`,
        description: "Create, edit, and manage announcements for your users",
      },
      showHeader: true,
    };
    
    const isLoading = ref(true);
    
    onMounted(async () => {
      try {
        await announcementStore.loadData();
        announcementStore.subscribeToChanges();
      } catch (error) {
        console.error("Failed to load announcements:", error);
      } finally {
        isLoading.value = false;
      }
    });
    
    onUnmounted(() => {
      announcementStore.unsubscribe();
    });
    
    const data = computed(() => announcementStore.announcements || []);
    
    // Format date helper
    const formatDate = (dateString: string) => {
      return new Date(dateString).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      });
    };
    
    // Extract preview text from JSONB content
    const getPreview = (content: any) => {
      if (!content) return 'No content';
      if (typeof content === 'string') return content.substring(0, 50) + '...';
      if (content?.text) return content.text.substring(0, 50) + '...';
      if (content?.description) return content.description.substring(0, 50) + '...';
      return 'No content';
    };
    
    // Get thumbnail from JSONB content
    const getThumbnail = (content: any) => {
      if (!content) return 'https://placehold.co/100x100?text=No+Image';
      if (content?.image) return content.image;
      if (content?.coverImage) return content.coverImage;
      if (content?.thumbnail) return content.thumbnail;
      return 'https://placehold.co/100x100?text=No+Image';
    };
    
    const columns: TableColumn<Announcement>[] = [
      {
        accessorKey: "id",
        header: "ID",
      },
      {
        accessorKey: "title",
        header: "Announcement",
        cell: ({ row }) => {
          return h("div", { class: "flex items-center gap-3" }, [
            h(UAvatar, {
              src: getThumbnail(row.original.content),
              size: "lg",
            }),
            h("div", undefined, [
              h("p", { class: "font-medium text-highlighted" }, row.original.title),
              h("p", { class: "text-sm text-gray-500" }, getPreview(row.original.content)),
            ]),
          ]);
        },
      },
      {
        accessorKey: "createdAt",
        header: "Created",
        cell: ({ row }) => {
          return h("p", { class: "text-sm text-gray-600 dark:text-gray-400" }, formatDate(row.original.createdAt));
        },
      },
      {
        accessorKey: "actions",
        header: "",
        cell: ({ row }) => {
          return h("div", { class: "flex items-center gap-3" }, [
            h(UButton, {
              variant: "ghost",
              size: "sm",
              label: "Edit",
              icon: "i-heroicons-pencil-square",
              onClick: () => {
                navigateTo(`/admin/announcements/${row.original.id}`);
              },
            }),
            h(UButton, {
              variant: "ghost",
              size: "sm",
              color: "red",
              icon: "i-heroicons-trash",
              onClick: async () => {
                if (confirm('Are you sure you want to delete this announcement?')) {
                  try {
                    const supabase = useSupabase();
                    const { error } = await supabase
                      .from('announcements')
                      .delete()
                      .eq('id', row.original.id);
                    
                    if (error) throw error;
                  } catch (error) {
                    console.error('Failed to delete announcement:', error);
                    alert('Failed to delete announcement');
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
      <div class="h-full p-4 mt-6">
        <!-- Header -->
        <div class="mt-4">
          <div v-show="pageMeta.showHeader">
            <div class="flex items-center justify-between">
              <div>
                <USkeleton v-if="isLoading" class="h-8 w-50" />
                <h1 v-else class="text-2xl font-bold">
                  {{ pageMeta.header.name }}
                </h1>
    
                <USkeleton v-if="isLoading" class="mt-2 h-6 w-100" />
                <p v-else class="mt-2 text-gray-600 dark:text-gray-400">
                  {{ pageMeta.header.description }}
                </p>
              </div>
    
              <UButton
                v-if="!isLoading"
                label="New Announcement"
                icon="i-heroicons-plus"
                size="lg"
                @click="navigateTo('/admin/announcements/new')"
              />
            </div>
          </div>
        </div>
    
        <!-- Table -->
        <div class="p-6">
          <!-- Loading State -->
          <USkeleton v-if="isLoading" class="h-[340px] w-full rounded-lg" />
    
          <!-- Error State -->
          <UAlert
            v-else-if="announcementStore.error"
            color="error"
            variant="subtle"
            :title="announcementStore.error"
            icon="i-lucide-alert-circle"
          />
    
          <!-- Empty State -->
          <div v-else-if="!data || data.length === 0" class="text-center py-12">
            <div class="flex flex-col items-center gap-4">
              <UIcon name="i-lucide-inbox" class="w-12 h-12 text-gray-400" />
              <p class="text-gray-500 dark:text-gray-400">No announcements yet</p>
              <UButton
                label="Create First Announcement"
                icon="i-heroicons-plus"
                @click="navigateTo('/admin/announcements/new')"
              />
            </div>
          </div>
    
          <!-- Table with Data -->
          <UTable v-else :data="data" :columns="columns" class="flex-1" />
        </div>
      </div>
    </template>