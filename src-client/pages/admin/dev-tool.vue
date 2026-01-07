<script setup lang="ts">
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

definePageMeta({
  layout: "home",
});

const toast = useToast();

/**
 * State
 */
const isLoading = ref(true);
const checkingPermission = ref(false);
const permissionGranted = ref<boolean | null>(null);

const notificationTitle = ref("Test Notification");
const notificationBody = ref(
  "This is a test notification from the developer tools"
);

/**
 * Check notification permission
 */
const checkPermission = async () => {
  checkingPermission.value = true;
  try {
    permissionGranted.value = await isPermissionGranted();
  } catch (err: any) {
    toast.add({
      title: "Error",
      description: err.message || "Failed to check permission",
      color: "error",
    });
  } finally {
    checkingPermission.value = false;
  }
};

/**
 * Request permission
 */
const requestNotifPermission = async () => {
  try {
    const permission = await requestPermission();
    permissionGranted.value = permission === "granted";

    toast.add({
      title: "Permission Result",
      description: permissionGranted.value
        ? "Notification permission granted"
        : "Notification permission denied",
      color: permissionGranted.value ? "success" : "warning",
    });
  } catch (err: any) {
    toast.add({
      title: "Error",
      description: err.message || "Failed to request permission",
      color: "error",
    });
  }
};

/**
 * Send test notification
 */
const sendTestNotification = async () => {
  if (!permissionGranted.value) {
    toast.add({
      title: "Permission Required",
      description: "Please grant notification permission first",
      color: "warning",
    });
    return;
  }

  try {
    sendNotification({
      title: notificationTitle.value,
      body: notificationBody.value,
    });

    toast.add({
      title: "Success",
      description: "Notification sent successfully",
      color: "success",
    });
  } catch (err: any) {
    toast.add({
      title: "Error",
      description: err.message || "Failed to send notification",
      color: "error",
    });
  }
};

onMounted(async () => {
  try {
    await checkPermission();
  } catch (error) {
    console.error("Failed to check permissions:", error);
  } finally {
    isLoading.value = false;
  }
});
</script>

<template>
  <div
    class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
  >
    <div class="max-w-7xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <USkeleton v-if="isLoading" class="h-12 w-64 mb-2" />
        <div v-else class="flex items-center gap-3 mb-2">
          <div class="p-3 bg-primary-500/10 rounded-xl">
            <UIcon
              name="i-heroicons-wrench-screwdriver"
              class="w-8 h-8 text-primary-500"
            />
          </div>
          <h1
            class="text-4xl font-bold bg-gradient-to-r from-primary-500 to-primary-600 bg-clip-text text-transparent"
          >
            🛠 Developer Tools
          </h1>
        </div>

        <USkeleton v-if="isLoading" class="h-6 w-96" />
        <p v-else class="text-gray-600 dark:text-gray-400 text-lg ml-14">
          Internal tools for testing native app capabilities.
        </p>
      </div>

      <!-- Loading State -->
      <USkeleton v-if="isLoading" class="h-[400px] w-full rounded-lg" />

      <!-- Tools Grid -->
      <div v-else class="grid gap-6">
        <!-- Notification Tester -->
        <UCard class="shadow-xl border-0">
          <div class="space-y-6">
            <!-- Tool Header -->
            <div
              class="flex items-center justify-between pb-6 border-b border-gray-200 dark:border-gray-800"
            >
              <div class="flex items-center gap-4">
                <div class="p-3 bg-blue-500/10 rounded-xl">
                  <UIcon
                    name="i-heroicons-bell"
                    class="w-6 h-6 text-blue-500"
                  />
                </div>
                <div>
                  <h2 class="text-xl font-bold text-gray-900 dark:text-white">
                    Notification Tester
                  </h2>
                  <p class="text-sm text-gray-600 dark:text-gray-400">
                    Test native system notifications
                  </p>
                </div>
              </div>

              <!-- Permission Badge -->
              <UBadge
                v-if="permissionGranted !== null"
                :color="permissionGranted ? 'success' : 'warning'"
                variant="subtle"
                size="lg"
              >
                <UIcon
                  :name="
                    permissionGranted
                      ? 'i-heroicons-check-circle'
                      : 'i-heroicons-exclamation-circle'
                  "
                  class="w-4 h-4"
                />
                {{ permissionGranted ? "Granted" : "Not Granted" }}
              </UBadge>
              <USkeleton v-else class="h-7 w-28" />
            </div>

            <!-- Permission Controls -->
            <div class="space-y-4">
              <div>
                <h3
                  class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3"
                >
                  Permission Control
                </h3>
                <div class="flex flex-wrap gap-2">
                  <UButton
                    icon="i-heroicons-arrow-path"
                    variant="outline"
                    color="neutral"
                    :loading="checkingPermission"
                    @click="checkPermission"
                  >
                    Check Status
                  </UButton>

                  <UButton
                    icon="i-heroicons-key"
                    variant="outline"
                    color="primary"
                    :disabled="permissionGranted === true"
                    @click="requestNotifPermission"
                  >
                    Request Permission
                  </UButton>
                </div>
              </div>

              <UDivider />

              <!-- Notification Form -->
              <div class="space-y-4">
                <h3
                  class="text-sm font-semibold text-gray-700 dark:text-gray-300"
                >
                  Send Test Notification
                </h3>

                <UFormField  label="Notification Title" size="lg">
                  <UInput
                    v-model="notificationTitle"
                    placeholder="Enter notification title"
                    size="lg"
                    class="w-100"
                  />
                </UFormField>

                <UFormField label="Notification Body" size="lg">
                  <UTextarea
                    v-model="notificationBody"
                    placeholder="Enter notification message"
                    :rows="3"
                    size="lg"
                    class="w-100"
                  />
                </UFormField>

                <div class="flex justify-end pt-2">
                  <UButton
                    icon="i-heroicons-paper-airplane"
                    color="primary"
                    size="lg"
                    :disabled="!permissionGranted"
                    @click="sendTestNotification"
                  >
                    Send Notification
                  </UButton>
                </div>
              </div>
            </div>
          </div>
        </UCard>

        <!-- Placeholder for Future Tools -->
        <UCard class="shadow-xl border-0 border-dashed">
          <div class="text-center py-12">
            <UIcon
              name="i-heroicons-plus-circle"
              class="w-12 h-12 text-gray-400 mx-auto mb-4"
            />
            <h3
              class="text-lg font-semibold text-gray-900 dark:text-white mb-2"
            >
              More Tools Coming Soon
            </h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Additional developer tools will be added here
            </p>
          </div>
        </UCard>
      </div>
    </div>
  </div>
</template>
