<script setup lang="ts">
definePageMeta({
  layout: "home",
});

const route = useRoute();
const router = useRouter();
const supabase = useSupabase();

const announcementId = computed(() => parseInt(route.params.id as string));

const announcement = ref<any>(null);
const isLoading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
  try {
    const { data, error: fetchError } = await supabase
      .from("announcements")
      .select("*")
      .eq("id", announcementId.value)
      .single();

    if (fetchError) throw fetchError;

    announcement.value = data;
  } catch (err) {
    console.error("Failed to load announcement:", err);
    error.value = "Announcement not found";
  } finally {
    isLoading.value = false;
  }
});

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
};

const getRelativeTime = (dateString: string) => {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 60) return `${diffMins} minutes ago`;
  if (diffHours < 24) return `${diffHours} hours ago`;
  if (diffDays < 30) return `${diffDays} days ago`;
  return formatDate(dateString);
};

const getContent = (content: any) => {
  if (!content) return "";
  if (typeof content === "string") return content;
  if (content?.text) return content.text;
  return "";
};

const getCoverImage = (content: any) => {
  if (!content) return null;
  if (content?.image) return content.image;
  if (content?.coverImage) return content.coverImage;
  return null;
};
</script>

<template>
  <div
    class="mt-6 min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800"
  >
    <div v-if="isLoading" class="max-w-4xl mx-auto p-6">
      <USkeleton class="h-12 w-32 mb-8" />
      <USkeleton class="h-16 w-full mb-4" />
      <USkeleton class="h-96 w-full mb-6" />
      <USkeleton class="h-64 w-full" />
    </div>

    <div v-else-if="error" class="max-w-4xl mx-auto p-6">
      <UButton
        icon="i-heroicons-arrow-left"
        variant="ghost"
        size="lg"
        label="Back to Home"
        @click="router.push('/')"
        class="mb-8"
      />

      <UCard class="text-center py-12">
        <UIcon
          name="i-heroicons-exclamation-triangle"
          class="w-16 h-16 text-red-500 mx-auto mb-4"
        />
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
          Announcement Not Found
        </h2>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          The announcement you're looking for doesn't exist or has been removed.
        </p>
        <UButton
          label="Go to Home"
          icon="i-heroicons-home"
          @click="router.push('/')"
        />
      </UCard>
    </div>

    <div v-else-if="announcement" class="max-w-4xl mx-auto p-6">
      <article>
        <header class="mb-8">
          <div class="flex items-center gap-2 mb-4">
            <div class="px-3 py-1 bg-primary-500/10 rounded-full">
              <span
                class="text-sm font-semibold text-primary-600 dark:text-primary-400"
              >
                Announcement
              </span>
            </div>
          </div>

          <h1
            class="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-4 leading-tight"
          >
            {{ announcement.title }}
          </h1>

          <div class="flex items-center gap-4 text-gray-600 dark:text-gray-400">
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-calendar" class="w-5 h-5" />
              <time :datetime="announcement.createdAt">
                {{ formatDate(announcement.createdAt) }}
              </time>
            </div>
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-clock" class="w-5 h-5" />
              <span>{{ getRelativeTime(announcement.createdAt) }}</span>
            </div>
          </div>
        </header>

        <div v-if="getCoverImage(announcement.content)" class="mb-8">
          <div class="relative overflow-hidden rounded-2xl shadow-2xl">
            <img
              :src="getCoverImage(announcement.content)"
              :alt="announcement.title"
              class="w-full h-auto max-h-[500px] object-cover"
            />
            <div
              class="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent"
            ></div>
          </div>
        </div>

        <UCard class="shadow-xl border-0">
          <div
            class="prose prose-lg dark:prose-invert max-w-none prose-headings:font-bold prose-headings:text-gray-900 dark:prose-headings:text-white prose-p:text-gray-700 dark:prose-p:text-gray-300 prose-p:leading-relaxed prose-a:text-primary-600 dark:prose-a:text-primary-400 prose-a:no-underline hover:prose-a:underline prose-strong:text-gray-900 dark:prose-strong:text-white prose-ul:list-disc prose-ol:list-decimal prose-li:text-gray-700 dark:prose-li:text-gray-300"
            v-html="getContent(announcement.content).replace(/\n/g, '<br>')"
          />
        </UCard>

        <div class="mt-8 flex items-center justify-between">
          <UButton
            label="Back to Announcements"
            icon="i-heroicons-arrow-left"
            variant="outline"
            @click="router.push('/#news')"
          />
        </div>
      </article>
    </div>
  </div>
</template>