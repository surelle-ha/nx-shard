<script setup lang="ts">
definePageMeta({
  layout: "home",
});

const toast = useToast();
const router = useRouter();
const supabase = useSupabase();

const title = ref("Untitled");
const content = ref("");
const coverImage = ref("https://base.ac.in/wp-content/uploads/2019/05/announcement-banner.jpg");
const isSaving = ref(false);

const handleSave = async () => {
  if (!title.value.trim()) {
    toast.add({ title: "Please enter a title", color: "error" });
    return;
  }

  if (!content.value.trim()) {
    toast.add({ title: "Please enter some content", color: "error" });
    return;
  }

  isSaving.value = true;

  try {
    const contentData = {
      text: content.value,
      image: coverImage.value || null,
    };

    const { error } = await supabase.from("announcements").insert({
      title: title.value,
      coverUrl: coverImage.value,
      content: contentData,
    });

    if (error) throw error;

    toast.add({
      title: "Announcement created successfully!",
      color: "success",
    });
    router.push("/admin/announcements");
  } catch (error) {
    console.error("Failed to create announcement:", error);
    toast.add({ title: "Failed to create announcement", color: "error" });
  } finally {
    isSaving.value = false;
  }
};

const handleCancel = () => {
  if (title.value || content.value || coverImage.value) {
    if (confirm("You have unsaved changes. Are you sure you want to leave?")) {
      router.push("/admin/announcements");
    }
  } else {
    router.push("/admin/announcements");
  }
};
</script>

<template>
  <div
    class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
  >
    <div class="max-w-5xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <UButton
          icon="i-heroicons-arrow-left"
          variant="ghost"
          size="lg"
          @click="handleCancel"
          class="mb-4"
        />

        <div class="flex items-center gap-3 mb-2">
          <div class="p-3 bg-primary-500/10 rounded-xl">
            <UIcon
              name="i-heroicons-megaphone"
              class="w-8 h-8 text-primary-500"
            />
          </div>
          <h1
            class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text text-transparent"
          >
            Create Announcement
          </h1>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-lg ml-14">
          Share important updates with your community
        </p>
      </div>

      <!-- Main Form Card -->
      <UCard class="shadow-xl border-0">
        <div class="space-y-8">
          <!-- Title Section -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-document-text" class="w-4 h-4" />
              Title
              <span class="text-red-500">*</span>
            </label>
            <UInput
              v-model="title"
              placeholder="Enter a compelling title..."
              size="xl"
              :disabled="isSaving"
              class="text-lg font-medium"
            />
          </div>

          <!-- Cover Image Section -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-photo" class="w-4 h-4" />
              Cover Image
              <span class="text-red-500">*</span>
            </label>
            <UInput
              v-model="coverImage"
              placeholder="https://example.com/image.jpg"
              :disabled="isSaving"
              icon="i-heroicons-link"
            />

            <!-- Image Preview -->
            <div v-if="coverImage" class="mt-6 relative group">
              <div class="absolute top-2 right-2 z-10">
                <UButton
                  icon="i-heroicons-x-mark"
                  color="error"
                  size="sm"
                  @click="coverImage = ''"
                  class="opacity-0 group-hover:opacity-100 transition-opacity"
                />
              </div>
              <div
                class="relative overflow-hidden rounded-xl border-2 border-gray-200 dark:border-gray-700"
              >
                <img
                  :src="coverImage"
                  alt="Cover preview"
                  class="w-full h-64 object-cover"
                  @error="() => (coverImage = '')"
                />
                <div
                  class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex items-end p-4"
                >
                  <p class="text-white text-sm font-medium">
                    Cover Image Preview
                  </p>
                </div>
              </div>
            </div>

            <!-- Empty State for Image -->
            <div
              v-else
              class="mt-6 border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-xl p-8 text-center"
            >
              <UIcon
                name="i-heroicons-photo"
                class="w-12 h-12 text-gray-400 mx-auto mb-3"
              />
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Add a cover image URL to preview it here
              </p>
            </div>
          </div>

          <!-- Content Section -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-document" class="w-4 h-4" />
              Content
              <span class="text-red-500">*</span>
            </label>
            <div class="relative">
              <UTextarea
                v-model="content"
                placeholder="Write your announcement content here...
    
    Tips:
    - Keep it clear and concise
    - Use bullet points for readability
    - Include important dates or deadlines
    - Add relevant links if needed"
                :rows="16"
                autoresize
                :disabled="isSaving"
                class="font-mono text-sm w-100"
              />
              <div
                class="absolute bottom-3 right-3 bg-white dark:bg-gray-900 px-2 py-1 rounded-md border border-gray-200 dark:border-gray-700"
              >
                <span class="text-xs font-medium text-gray-500">
                  {{ content.length }} characters
                </span>
              </div>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Action Buttons - Fixed at Bottom -->
      <div class="sticky bottom-6 mt-8">
        <UCard
          class="shadow-2xl border-0 bg-white/80 dark:bg-gray-900/80 backdrop-blur-lg"
        >
          <div class="flex items-center justify-between">
            <div
              class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400"
            >
              <UIcon name="i-heroicons-information-circle" class="w-4 h-4" />
              <span>All fields marked with * are required</span>
            </div>

            <div class="flex items-center gap-3">
              <UButton
                label="Cancel"
                variant="ghost"
                size="lg"
                @click="handleCancel"
                :disabled="isSaving"
              />
              <UButton
                label="Publish Announcement"
                icon="i-heroicons-check-circle"
                size="lg"
                :loading="isSaving"
                @click="handleSave"
                class="shadow-lg"
              />
            </div>
          </div>
        </UCard>
      </div>
    </div>
  </div>
</template>
