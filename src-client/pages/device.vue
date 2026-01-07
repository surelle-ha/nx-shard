<script setup lang="ts">
  definePageMeta({
    layout: "home",
  });
  </script>
  
  <template>
    <UError
      :clear="{
        color: 'neutral',
        size: 'xl',
        icon: 'i-lucide-arrow-left',
        class: 'rounded-full',
      }"
      :error="{
        statusCode: 501,
        statusMessage: 'Page not Implemented',
        message: 'The page you are looking for is not ready.',
      }"
    />
  </template>
  