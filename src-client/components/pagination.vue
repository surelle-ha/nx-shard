<script setup lang="ts">
const props = defineProps({
  page: { type: Number, required: true },
  total: { type: Number, required: true },
});

const emit = defineEmits(["change"]);

const next = () => {
  if (props.page < props.total) emit("change", props.page + 1);
};

const prev = () => {
  if (props.page > 1) emit("change", props.page - 1);
};
</script>

<template>
  <div class="flex gap-3 mt-4">
    <button
      @click="prev"
      class="px-4 py-2 bg-gray-800 rounded-lg"
      :disabled="page === 1"
    >
      Prev
    </button>

    <span class="px-4 py-2 bg-gray-800 rounded-lg">
      Page {{ page }} / {{ total }}
    </span>

    <button
      @click="next"
      class="px-4 py-2 bg-gray-800 rounded-lg"
      :disabled="page === total"
    >
      Next
    </button>
  </div>
</template>
