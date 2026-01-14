<script setup lang="ts">
const pageMeta = {
  header: {
    name: "Account Settings",
    description: "Manage your personal information and preferences.",
  },
  showHeader: true,
};

definePageMeta({
  layout: "home",
});

const isLoading = ref(true);
const isSaving = ref(false);
const toast = useToast();
const accountStore = useAccountStore();

const accountName = ref(accountStore.account?.displayName || "");
const accountImageUrl = ref(accountStore.account?.imageUrl || "");
const accountEmail = ref(accountStore.account?.email || "");

const hasChanges = computed(() => {
  return (
    accountName.value !== accountStore.account?.displayName ||
    accountImageUrl.value !== accountStore.account?.imageUrl
  );
});

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false;
  }, 1000);
});

const handleSave = async () => {
  if (!hasChanges.value) {
    return;
  }

  isSaving.value = true;

  try {
    await accountStore.updateUserConfig({
      displayName: accountName.value,
      imageUrl: accountImageUrl.value,
    });

    toast.add({
      title: "Account settings updated successfully!",
      color: "success",
    });
  } catch (error) {
    console.error("Failed to update account:", error);
    toast.add({ title: "Failed to update account settings", color: "error" });
  } finally {
    isSaving.value = false;
  }
};

const handleReset = () => {
  accountName.value = accountStore.account?.displayName || "";
  accountImageUrl.value = accountStore.account?.imageUrl || "";
};
</script>

<template>
  <div
    class="mt-6 min-h-screen p-6"
  >
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <USkeleton v-if="isLoading" class="h-12 w-64 mb-2" />
        <div v-else class="flex items-center gap-3 mb-2">
          <div class="p-3 bg-primary-500/10 rounded-xl">
            <UIcon
              name="i-heroicons-user-circle"
              class="w-8 h-8 text-primary-500"
            />
          </div>
          <h1
            class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text"
          >
            {{ pageMeta.header.name }}
          </h1>
        </div>

        <USkeleton v-if="isLoading" class="h-6 w-96" />
        <p v-else class="text-gray-600 dark:text-gray-400 text-lg ml-14">
          {{ pageMeta.header.description }}
        </p>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="space-y-6">
        <UCard class="shadow-xl">
          <div class="space-y-6">
            <USkeleton class="h-32 w-32 rounded-full" />
            <USkeleton class="h-12 w-full" />
            <USkeleton class="h-12 w-full" />
            <USkeleton class="h-12 w-full" />
          </div>
        </UCard>
      </div>

      <!-- Account Settings Form -->
      <UCard v-else class="shadow-xl border-0">
        <div class="space-y-8">
          <!-- Profile Preview -->
          <div
            class="flex items-center gap-6 p-6 bg-gray-50 dark:bg-gray-800/50 rounded-xl"
          >
            <UAvatar
              :src="accountImageUrl"
              :alt="accountName"
              size="3xl"
              class="ring-4 ring-primary-500/20"
            />
            <div>
              <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-1">
                {{ accountName || "Your Name" }}
              </h3>
              <p class="text-gray-600 dark:text-gray-400">
                {{ accountEmail }}
              </p>
              <div class="flex items-center gap-2 mt-2">
                <div class="px-2 py-1 bg-green-500/10 rounded-md">
                  <span
                    class="text-xs font-semibold text-green-600 dark:text-green-400"
                  >
                    Active
                  </span>
                </div>
              </div>
            </div>
          </div>

          <!-- Email Field (Read-only) -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-envelope" class="w-4 h-4" />
              Email Address
            </label>
            <UInput
              :model-value="accountEmail"
              disabled
              icon="i-heroicons-envelope"
              size="xl"
              class="opacity-75"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
              Email cannot be changed
            </p>
          </div>

          <!-- Display Name Field -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-user" class="w-4 h-4" />
              Display Name
              <span class="text-red-500">*</span>
            </label>
            <UInput
              v-model="accountName"
              placeholder="Enter your display name"
              icon="i-heroicons-user"
              size="xl"
              :disabled="isSaving"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
              This will appear as your public display name
            </p>
          </div>

          <!-- Profile Picture URL Field -->
          <div>
            <label
              class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2"
            >
              <UIcon name="i-heroicons-photo" class="w-4 h-4" />
              Profile Picture URL
            </label>
            <UInput
              v-model="accountImageUrl"
              placeholder="https://example.com/your-image.jpg"
              icon="i-heroicons-link"
              size="xl"
              :disabled="isSaving"
            />
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
              Image upload not yet supported. Please paste a direct image URL
            </p>

            <!-- Image Preview -->
            <div v-if="accountImageUrl" class="mt-4">
              <p
                class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >
                Preview:
              </p>
              <div
                class="relative w-32 h-32 rounded-xl overflow-hidden border-2 border-gray-200 dark:border-gray-700"
              >
                <img
                  :src="accountImageUrl"
                  alt="Profile preview"
                  class="w-full h-full object-cover"
                  @error="() => (accountImageUrl = '')"
                />
              </div>
            </div>
          </div>

          <!-- Account Stats -->
          <div
            class="grid grid-cols-1 md:grid-cols-3 gap-4 pt-6 border-t border-gray-200 dark:border-gray-700"
          >
            <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <div class="flex items-center gap-2 mb-1">
                <UIcon
                  name="i-heroicons-calendar"
                  class="w-4 h-4 text-blue-600 dark:text-blue-400"
                />
                <span
                  class="text-xs font-semibold text-blue-600 dark:text-blue-400"
                  >Member Since</span
                >
              </div>
              <p class="text-sm font-bold text-gray-900 dark:text-white">
                {{
                  new Date(
                    accountStore.account?.createdAt || ""
                  ).toLocaleDateString("en-US", {
                    year: "numeric",
                    month: "short",
                  })
                }}
              </p>
            </div>

            <div class="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
              <div class="flex items-center gap-2 mb-1">
                <UIcon
                  name="i-heroicons-check-circle"
                  class="w-4 h-4 text-green-600 dark:text-green-400"
                />
                <span
                  class="text-xs font-semibold text-green-600 dark:text-green-400"
                  >Account Status</span
                >
              </div>
              <p class="text-sm font-bold text-gray-900 dark:text-white">
                {{ accountStore.account?.isApproved ? "Approved" : "Pending" }}
              </p>
            </div>

            <div class="p-4 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
              <div class="flex items-center gap-2 mb-1">
                <UIcon
                  name="i-heroicons-bolt"
                  class="w-4 h-4 text-purple-600 dark:text-purple-400"
                />
                <span
                  class="text-xs font-semibold text-purple-600 dark:text-purple-400"
                  >Power Level</span
                >
              </div>
              <p class="text-sm font-bold text-gray-900 dark:text-white">
                {{ accountStore.account?.power || 0 }}
              </p>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Action Buttons - Sticky at Bottom -->
      <div class="sticky bottom-6 mt-8" v-if="!isLoading">
        <UCard
          class="shadow-2xl border-0 backdrop-blur-lg"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <UIcon
                :name="
                  hasChanges
                    ? 'i-heroicons-exclamation-circle'
                    : 'i-heroicons-check-circle'
                "
                :class="hasChanges ? 'text-yellow-500' : 'text-green-500'"
                class="w-5 h-5"
              />
              <span class="text-sm text-gray-600 dark:text-gray-400">
                {{
                  hasChanges ? "You have unsaved changes" : "All changes saved"
                }}
              </span>
            </div>

            <div class="flex items-center gap-3">
              <UButton
                label="Reset"
                variant="ghost"
                size="lg"
                icon="i-heroicons-arrow-path"
                @click="handleReset"
                :disabled="!hasChanges || isSaving"
              />
              <UButton
                label="Save Changes"
                icon="i-heroicons-check-circle"
                size="lg"
                :loading="isSaving"
                :disabled="!hasChanges"
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
