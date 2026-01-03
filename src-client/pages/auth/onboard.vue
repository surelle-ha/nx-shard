<script setup lang="ts">
definePageMeta({
  layout: "authentication",
});
import type { StepperItem } from "@nuxt/ui";

const items: StepperItem[] = [
  {
    title: "NxShard",
    description: "Add your address here",
    icon: "i-lucide-diamond",
  },
  {
    title: "About You",
    description: "Set your preferred shipping method",
    icon: "i-lucide-user",
  },
  {
    title: "Preference",
    description: "Confirm your order",
    icon: "i-lucide-laptop",
  },
];

const stepper = useTemplateRef("stepper");
</script>

<template>
  <UPageCard class="w-full max-w-xl">
    <div class="w-full">
      <UStepper ref="stepper" :items="items">
        <template #content="{ item }">
          <Placeholder class="aspect-video h-50">
            {{ item.title }}
          </Placeholder>
        </template>
      </UStepper>

      <div class="flex gap-2 justify-between mt-4">
        <UButton
          leading-icon="i-lucide-arrow-left"
          :disabled="!stepper?.hasPrev"
          @click="stepper?.prev()"
        >
          Prev
        </UButton>

        <UButton
          trailing-icon="i-lucide-arrow-right"
          :disabled="!stepper?.hasNext"
          @click="stepper?.next()"
        >
          Next
        </UButton>
      </div>
    </div></UPageCard
  >
</template>
