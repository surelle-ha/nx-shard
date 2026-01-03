<script setup lang="ts">
definePageMeta({
  layout: "home",
});

const router = useRouter();
const supabase = useSupabase();
const toast = useToast();

const title = ref("");
const description = ref("");
const tags = ref("");
const coverUrl = ref(
  "https://eu-images.contentstack.com/v3/assets/blt740a130ae3c5d529/bltbcd6a7c58395293f/650efbdcb9cdf6152f89a2a6/Switch_Header.png?width=1280&auto=webp&quality=80&format=jpg&disable=upscale"
);
const isExperimental = ref(false);
const isEnabled = ref(true);
const isBroken = ref(false);
const torrentFile = ref<File | null>(null);
const isUploading = ref(false);
const isSaving = ref(false);
const uploadProgress = ref(0);

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files[0]) {
    const file = target.files[0];

    if (!file.name.endsWith(".torrent")) {
      toast.add({
        title: "Please select a valid .torrent file",
        color: "warning",
      });
      return;
    }

    torrentFile.value = file;
  }
};

const uploadTorrent = async (): Promise<string | null> => {
  if (!torrentFile.value) return null;

  isUploading.value = true;
  uploadProgress.value = 0;

  try {
    const fileName = `${Date.now()}-${torrentFile.value.name}`;
    const filePath = `${fileName}`;

    const { data, error } = await supabase.storage
      .from("NintendoSwitchTorrents")
      .upload(filePath, torrentFile.value, {
        cacheControl: "3600",
        upsert: false,
      });

    if (error) throw error;

    const {
      data: { publicUrl },
    } = supabase.storage.from("NintendoSwitchTorrents").getPublicUrl(filePath);

    uploadProgress.value = 100;
    return publicUrl;
  } catch (error) {
    console.error("Failed to upload torrent:", error);
    toast.add({ title: "Failed to upload torrent file", color: "error" });
    return null;
  } finally {
    isUploading.value = false;
  }
};

const handleSave = async () => {
  if (!title.value.trim()) {
    toast.add({ title: "Please enter a title", color: "warning" });
    return;
  }

  if (!description.value.trim()) {
    toast.add({ title: "Please enter a description", color: "warning" });
    return;
  }

  isSaving.value = true;

  try {
    let downloadUrl = null;
    if (torrentFile.value) {
      downloadUrl = await uploadTorrent();
      if (!downloadUrl) {
        throw new Error("Failed to upload torrent file");
      }
    }

    const tagArray = tags.value
      .split(/[,\s]+/)
      .filter((tag) => tag.trim())
      .map((tag) => tag.trim());

    const { error } = await supabase.from("games").insert({
      title: title.value,
      description: description.value,
      tags: tagArray,
      coverUrl: coverUrl.value || null,
      downloadUrl: downloadUrl,
      isExperimental: isExperimental.value,
      isEnabled: isEnabled.value,
      isBroken: isBroken.value,
    } as never);

    if (error) throw error;

    toast.add({ title: "Game added successfully!", color: "warning" });
    router.push("/admin/games");
  } catch (error) {
    console.error("Failed to create game:", error);
    toast.add({ title: "Failed to create game", color: "error" });
  } finally {
    isSaving.value = false;
  }
};

const handleCancel = () => {
  if (
    title.value ||
    description.value ||
    tags.value ||
    coverUrl.value ||
    torrentFile.value
  ) {
    if (confirm("You have unsaved changes. Are you sure you want to leave?")) {
      router.push("/admin/games");
    }
  } else {
    router.push("/admin/games");
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
              name="i-heroicons-plus-circle"
              class="w-8 h-8 text-primary-500"
            />
          </div>
          <h1
            class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text text-transparent"
          >
            Add New Game
          </h1>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-lg ml-14">
          Upload a new game to your collection
        </p>
      </div>

      <!-- Main Form Card -->
      <UCard class="shadow-xl border-0">
        <div class="space-y-8">
          <!-- Title -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-document-text" class="w-4 h-4" />
              Game Title
              <span class="text-red-500">*</span>
            </label>
            <UInput
              v-model="title"
              placeholder="Enter game title"
              size="xl"
              :disabled="isSaving || isUploading"
            />
          </div>

          <!-- Description -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-document" class="w-4 h-4" />
              Description
              <span class="text-red-500">*</span>
            </label>
            <UTextarea
              v-model="description"
              placeholder="Describe the game..."
              :rows="6"
              autoresize
              :disabled="isSaving || isUploading"
            />
          </div>

          <!-- Tags -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-tag" class="w-4 h-4" />
              Tags
            </label>
            <UInput
              v-model="tags"
              placeholder="action, adventure, rpg (comma or space separated)"
              :disabled="isSaving || isUploading"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
              Separate tags with commas or spaces
            </p>
          </div>

          <!-- Cover Image URL -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-photo" class="w-4 h-4" />
              Cover Image URL
            </label>
            <UInput
              v-model="coverUrl"
              placeholder="https://example.com/cover.jpg"
              icon="i-heroicons-link"
              :disabled="isSaving || isUploading"
            />

            <!-- Image Preview -->
            <div v-if="coverUrl" class="mt-6 relative group">
              <div class="absolute top-2 right-2 z-10">
                <UButton
                  icon="i-heroicons-x-mark"
                  color="error"
                  size="sm"
                  @click="
                    coverUrl =
                      'https://eu-images.contentstack.com/v3/assets/blt740a130ae3c5d529/bltbcd6a7c58395293f/650efbdcb9cdf6152f89a2a6/Switch_Header.png?width=1280&auto=webp&quality=80&format=jpg&disable=upscale'
                  "
                  class="opacity-0 group-hover:opacity-100 transition-opacity"
                />
              </div>
              <div
                class="relative overflow-hidden rounded-xl border-2 border-gray-200 dark:border-gray-700"
              >
                <img
                  :src="coverUrl"
                  alt="Cover preview"
                  class="w-full h-64 object-cover"
                  @error="
                    () =>
                      (coverUrl =
                        'https://eu-images.contentstack.com/v3/assets/blt740a130ae3c5d529/bltbcd6a7c58395293f/650efbdcb9cdf6152f89a2a6/Switch_Header.png?width=1280&auto=webp&quality=80&format=jpg&disable=upscale')
                  "
                />
              </div>
            </div>
          </div>

          <!-- Game Status Checkboxes -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-cog-6-tooth" class="w-4 h-4" />
              Game Status
            </label>

            <div
              class="space-y-4 p-4 bg-gray-50 dark:bg-gray-800/50 rounded-xl"
            >
              <!-- Is Enabled -->
              <div class="flex items-start gap-3">
                <UCheckbox
                  v-model="isEnabled"
                  :disabled="isSaving || isUploading"
                />
                <div>
                  <p class="font-medium text-gray-900 dark:text-white">
                    Enabled
                  </p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">
                    Game is visible and downloadable to users
                  </p>
                </div>
              </div>

              <!-- Is Experimental -->
              <div class="flex items-start gap-3">
                <UCheckbox
                  v-model="isExperimental"
                  :disabled="isSaving || isUploading"
                />
                <div>
                  <p class="font-medium text-gray-900 dark:text-white">
                    Experimental 🧪
                  </p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">
                    Mark this game as experimental (beta testing, unstable)
                  </p>
                </div>
              </div>

              <!-- Is Broken -->
              <div class="flex items-start gap-3">
                <UCheckbox
                  v-model="isBroken"
                  :disabled="isSaving || isUploading"
                />
                <div>
                  <p class="font-medium text-gray-900 dark:text-white">
                    Broken ⚠️
                  </p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">
                    Game has known issues or doesn't work properly
                  </p>
                </div>
              </div>
            </div>
          </div>

          <!-- Torrent File Upload -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-arrow-down-tray" class="w-4 h-4" />
              Torrent File
            </label>

            <div
              class="border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-xl p-8 text-center"
            >
              <input
                type="file"
                accept=".torrent"
                @change="handleFileChange"
                class="hidden"
                id="torrent-upload"
                :disabled="isSaving || isUploading"
              />

              <div v-if="!torrentFile">
                <UIcon
                  name="i-heroicons-cloud-arrow-up"
                  class="w-12 h-12 text-gray-400 mx-auto mb-3"
                />
                <label
                  for="torrent-upload"
                  class="cursor-pointer text-primary-600 dark:text-primary-400 hover:underline font-medium"
                >
                  Click to upload
                </label>
                <span class="text-gray-500"> or drag and drop</span>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
                  .torrent files only
                </p>
              </div>

              <div v-else class="flex items-center justify-center gap-3">
                <UIcon
                  name="i-heroicons-document"
                  class="w-8 h-8 text-green-500"
                />
                <div class="text-left">
                  <p class="font-medium text-gray-900 dark:text-white">
                    {{ torrentFile.name }}
                  </p>
                  <p class="text-sm text-gray-500">
                    {{ (torrentFile.size / 1024).toFixed(2) }} KB
                  </p>
                </div>
                <UButton
                  icon="i-heroicons-x-mark"
                  color="error"
                  size="sm"
                  variant="ghost"
                  @click="torrentFile = null"
                  :disabled="isUploading"
                />
              </div>

              <!-- Upload Progress -->
              <div v-if="isUploading" class="mt-4">
                <div
                  class="w-full bg-gray-200 rounded-full h-2 dark:bg-gray-700"
                >
                  <div
                    class="bg-primary-600 h-2 rounded-full transition-all duration-300"
                    :style="{ width: uploadProgress + '%' }"
                  ></div>
                </div>
                <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
                  Uploading... {{ uploadProgress }}%
                </p>
              </div>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Action Buttons -->
      <div class="sticky bottom-6 mt-8">
        <UCard
          class="shadow-2xl border-0 bg-white/80 dark:bg-gray-900/80 backdrop-blur-lg"
        >
          <div class="flex items-center justify-between">
            <div
              class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400"
            >
              <UIcon name="i-heroicons-information-circle" class="w-4 h-4" />
              <span>Fields marked with * are required</span>
            </div>

            <div class="flex items-center gap-3">
              <UButton
                label="Cancel"
                variant="ghost"
                size="lg"
                @click="handleCancel"
                :disabled="isSaving || isUploading"
              />
              <UButton
                label="Add Game"
                icon="i-heroicons-check-circle"
                size="lg"
                :loading="isSaving || isUploading"
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
