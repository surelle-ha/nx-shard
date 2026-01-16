<script setup lang="ts">
definePageMeta({
  layout: "home",
});

const faqs = [
  {
    label: "What is NxShard?",
    icon: "i-heroicons-question-mark-circle",
    content:
      "NxShard is a game downloader powered by a peer-to-peer (torrent-based) mechanism. It is designed to help users easily download, monitor, and install games in one place.",
    defaultOpen: true,
  },
  {
    label: "How do I get started with Nx-Shard?",
    icon: "i-heroicons-rocket-launch",
    content:
      "Getting started with Nx-Shard is simple: (1) Navigate to the Explore tab and select the game you want to download. (2) View and manage your selected games in the Library tab. (3) Monitor the download progress and install the game once it’s complete.",
  },
  {
    label: "How do I configure Nx-Shard?",
    icon: "i-heroicons-cog-6-tooth",
    content:
      "You can configure Nx-Shard through the Settings page. From there, you can adjust download locations, bandwidth limits, and other preferences to optimize performance based on your system and network.",
  },
  {
    label: "Can I contribute to the project?",
    icon: "i-heroicons-users",
    content:
      "Yes, contributions are welcome. You can help by reporting bugs, suggesting features, improving documentation, or contributing code through the project’s repository.",
  },
  {
    label: "Where can I find documentation?",
    icon: "i-heroicons-book-open",
    content:
      "Official documentation is available in the project repository and website. It includes setup guides, usage instructions, and troubleshooting tips to help you get the most out of Nx-Shard.",
  },
  {
    label: "How do I update Nx-Shard?",
    icon: "i-heroicons-arrow-path",
    content:
      "Nx-Shard notifies you when a new update is available. You can update the app directly through the built-in update feature or by downloading the latest version from the official release page.",
  },
  {
    label: "What if I encounter an error?",
    icon: "i-heroicons-exclamation-triangle",
    content:
      "If you encounter an error, check the error message and logs for details. You can also refer to the documentation or report the issue through the project’s support or issue tracker for assistance.",
  },
];
</script>

<template>
  <div class="h-full p-4 mt-6">
    <UContainer>
      <div class="max-w-4xl mx-auto space-y-6">
        <!-- Header -->
        <UCard>
          <template #header>
            <div
              class="bg-gradient-to-r from-primary-600 to-purple-600 rounded-lg p-8 -m-6 -m-2 text-white"
            >
              <h1 class="text-4xl font-bold mb-3">Help & Support</h1>
              <p class="text-lg opacity-90">
                Find answers to common questions here.
              </p>
            </div>
          </template>
        </UCard>

        <!-- Issue Reporting Card -->
        <UCard>
          <div
            class="flex flex-col md:flex-row items-start justify-between gap-4"
          >
            <div class="flex-1">
              <div class="flex items-center gap-2 mb-2">
                <UIcon
                  name="i-heroicons-bug-ant"
                  class="text-xl text-red-500"
                />
                <h2 class="text-xl font-semibold">Found a Bug or Issue?</h2>
              </div>
              <p class="text-gray-600 dark:text-gray-400">
                Help us improve Nx-Shard by reporting bugs, issues, or
                suggesting new features. Our team reviews all submissions on
                GitHub.
              </p>
            </div>
            <UButton
              icon="i-heroicons-exclamation-circle"
              size="lg"
              color="primary"
              variant="subtle"
              label="Report Issue"
              to="https://github.com/surelle-ha/nx-shard/issues"
              external
              target="_blank"
              class="whitespace-nowrap cursor-pointer"
            />
          </div>
        </UCard>

        <!-- FAQ Section -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-question-mark-circle" class="text-2xl" />
              <h2 class="text-2xl font-bold">Frequently Asked Questions</h2>
            </div>
          </template>

          <UAccordion :items="faqs" />
        </UCard>

        <!-- Additional Help Section -->
        <UCard>
          <div class="flex items-start gap-3 mb-4">
            <UIcon
              name="i-heroicons-light-bulb"
              class="text-2xl text-primary-600 dark:text-primary-400 mt-0.5"
            />
            <div>
              <h3 class="text-lg font-semibold mb-2">Still Need Help?</h3>
              <p class="text-gray-600 dark:text-gray-400">
                If you couldn't find the answer you're looking for, feel free to
                reach out to our community or open a discussion on GitHub. We're
                here to help!
              </p>
            </div>
          </div>

          <div class="flex flex-wrap gap-3">
            <UButton
              icon="i-heroicons-chat-bubble-left-right"
              color="primary"
              variant="solid"
              label="GitHub Discussions"
              disabled
            />
            <UButton
              icon="i-heroicons-code-bracket"
              color="warning"
              variant="subtle"
              label="View Repository"
              to="https://github.com/surelle-ha/nx-shard"
              external
              target="_blank"
            />
          </div>
        </UCard>
      </div>
    </UContainer>
  </div>
</template>
