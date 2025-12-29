<script setup lang="ts">
  definePageMeta({
    layout: "home",
  });
  
  const isLoading = ref(true);
  const pageMeta = {
    header: {
      name: "⚙️ General",
      description: "Here are the available games you may download.",
    },
    showHeader: true,
  };
  onMounted(() => {
    setTimeout(() => {
      isLoading.value = false;
    }, 1000);
  });

  const nativeValue = ref(false)
  const reportValue = ref(true)
  const appnotifValue = ref(false)
  const desktnofifValue = ref(false)
  const experimentValue = ref(false)
  </script>
  
  <template>
    <div class="h-full p-4 mt-6">
      <div class="mt-4">
        <div v-show="pageMeta.showHeader">
          <USkeleton v-if="isLoading" class="h-8 w-50" />
          <h1 v-else class="text-2xl font-bold">{{ pageMeta.header.name }}</h1>
          <USkeleton v-if="isLoading" class="mt-2 h-6 w-100" />
          <p v-else class="mt-2 text-gray-600 dark:text-gray-400">
            {{ pageMeta.header.description }}
          </p>
        </div>
        <div class="p-4">
          <USwitch size="lg" v-model="nativeValue" label="Use Native Window tab" description="This is a checkbox." class="p-2" disabled/>
          <USwitch size="lg" v-model="reportValue" label="Send Error Report/Logs" description="This is a checkbox." class="p-2" disabled/>
          <USwitch size="lg" v-model="appnotifValue" label="In-App Notification" description="This is a checkbox." class="p-2" disabled/>
          <USwitch size="lg" v-model="desktnofifValue" label="Desktop Notification" description="This is a checkbox." class="p-2" disabled/>
          <USwitch size="lg" v-model="experimentValue" label="Opt for Experimental" description="This is a checkbox." class="p-2" disabled/>
        </div>
      </div>
    </div>
  </template>
  