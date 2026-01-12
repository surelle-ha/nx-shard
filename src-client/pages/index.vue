<script setup lang="ts">
  import type { ButtonProps } from '#ui/types';
  
  definePageMeta({
    layout: "home",
  });
  
  const accountStore = useAccountStore();
  const announcementStore = useAnnouncementStore();
  
  const isExperimental = computed(() => accountStore.account?.isExperimental);
  const isAnimatedHome = computed(() => accountStore.account?.isAnimatedHome);

  const links = ref<ButtonProps[]>([
    {
      label: "Explore Games",
      to: "/explore",
      icon: "i-lucide-gamepad-2",
    },
    {
      label: "Learn more",
      to: "/#news",
      color: "neutral" as const,
      variant: "subtle" as const,
      trailingIcon: "i-lucide-arrow-right",
    },
  ]);
  
  // Load announcements on mount
  onMounted(async () => {
    await announcementStore.loadData();
    announcementStore.subscribeToChanges();
  });
  
  // Clean up subscription on unmount
  onUnmounted(() => {
    announcementStore.unsubscribe();
  });
  
  // Format date helper
  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  };
  
  // Extract text content from JSONB
  const getDescription = (content: any) => {
    if (!content) return 'No description available';
    if (typeof content === 'string') return content;
    if (content?.text) return content.text;
    if (content?.description) return content.description;
    if (content?.blocks?.[0]?.text) return content.blocks[0].text;
    return 'No description available';
  };
  
  // Extract image from JSONB content
  const getImage = (content: any) => {
    if (!content) return 'https://placehold.co/600x400?text=Announcement';
    if (content?.image) return content.image;
    if (content?.coverImage) return content.coverImage;
    if (content?.thumbnail) return content.thumbnail;
    return 'https://placehold.co/600x400?text=Announcement';
  };
  </script>
  
  <template>
    <div class="h-full p-4 mt-6">
      <div class="mt-4">
        <div class="relative overflow-hidden">
          <!-- GIF Background -->
          <div v-show="isAnimatedHome" class="absolute inset-0 -z-10">
            <img 
              src="https://kotaku.com/app/uploads/2021/08/22ecb492fc415bbb28dab0bbfd3ad25d.gif" 
              alt="" 
              class="w-full h-full object-cover opacity-30"
            />
          </div>
          
          <UPageHero
            :title="`🌿\nnxShard`"
            description="A Nintendo Switch desktop client to download and install games."
            :links="links"
            class="whitespace-pre-line pb-10 relative z-10"
          />
        </div>
  
        <div class="p-4 mt-4" id="news">
          <h2 class="text-2xl font-bold mb-6">Latest Announcements</h2>
  
          <!-- Loading State -->
          <div v-if="announcementStore.loading" class="space-y-4">
            <USkeleton class="h-[200px] w-full rounded-lg" />
            <USkeleton class="h-[200px] w-full rounded-lg" />
            <USkeleton class="h-[200px] w-full rounded-lg" />
          </div>
  
          <!-- Error State -->
          <UAlert
            v-else-if="announcementStore.error"
            color="error"
            variant="subtle"
            :title="announcementStore.error"
            icon="i-lucide-alert-circle"
            class="mb-4"
          />
  
          <!-- Empty State -->
          <div 
            v-else-if="!announcementStore.announcements || announcementStore.announcements.length === 0"
            class="text-center py-12"
          >
            <div class="flex flex-col items-center gap-2">
              <UIcon name="i-lucide-inbox" class="w-12 h-12 text-gray-400" />
              <p class="text-gray-500 dark:text-gray-400">No announcements yet</p>
            </div>
          </div>
  
          <!-- Announcements List -->
          <div v-else class="space-y-4">
            <UBlogPost
              v-for="announcement in announcementStore.announcements"
              :key="announcement.id"
              :title="announcement.title"
              :description="getDescription(announcement.content)"
              :image="getImage(announcement.content)"
              :date="formatDate(announcement.createdAt)"
              :to="`/announcements/${announcement.id}`"
              orientation="horizontal"
              variant="outline"
            />
          </div>
        </div>
      </div>
    </div>
  </template>