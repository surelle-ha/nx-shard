<script setup lang="ts">
definePageMeta({
  layout: "home",
});

const faqs = [
  {
    label: "What is Nx-Shard?",
    icon: "i-heroicons-question-mark-circle",
    content:
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    defaultOpen: true,
  },
  {
    label: "How do I get started with Nx-Shard?",
    icon: "i-heroicons-rocket-launch",
    content:
      "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
  },
  {
    label: "What are the system requirements?",
    icon: "i-heroicons-computer-desktop",
    content:
      "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
  },
  {
    label: "How do I configure Nx-Shard?",
    icon: "i-heroicons-cog-6-tooth",
    content:
      "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet.",
  },
  {
    label: "Can I contribute to the project?",
    icon: "i-heroicons-users",
    content:
      "Consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam.",
  },
  {
    label: "Where can I find documentation?",
    icon: "i-heroicons-book-open",
    content:
      "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.",
  },
  {
    label: "How do I update Nx-Shard?",
    icon: "i-heroicons-arrow-path",
    content:
      "Similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit.",
  },
  {
    label: "What if I encounter an error?",
    icon: "i-heroicons-exclamation-triangle",
    content:
      "Quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint.",
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
