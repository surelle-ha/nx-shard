<script setup lang="ts">
import type { Message, ChatMessage } from '~/interfaces/chat';

// Props
const props = defineProps<{
  roomId: string;
  roomName?: string;
}>();

const accountStore = useAccountStore();
const router = useRouter();
const { sendMessage: sendMessageToSupabase, getMessages, subscribeToMessages } = useShardChat();

const isLoading = ref(true);
const displayName = computed(() => accountStore.account?.displayName || "Me");
const isExperimental = computed(() => accountStore.account?.isExperimental);
const isAdmin = computed(() => accountStore.account?.isAdmin);
const currentUserId = computed(() => accountStore.account?.id);

const isOpen = ref(false);
const inputMessage = ref("");
const messagesContainerRef = ref<HTMLElement | null>(null);
const messages = ref<ChatMessage[]>([]);
const unreadCount = ref(0);
const lastSeenMessageId = ref<string | null>(null);

let unsubscribe: (() => void) | null = null;

const scrollToBottom = () => {
  nextTick(() => {
    if (messagesContainerRef.value) {
      messagesContainerRef.value.scrollTop =
        messagesContainerRef.value.scrollHeight;
    }
  });
};

// Convert Supabase message format to component format
const convertMessage = (msg: Message): ChatMessage => ({
  id: msg.id,
  role: msg.userId === currentUserId.value ? 'user' as const : 'other' as const,
  displayName: msg.displayName,
  parts: [
    {
      type: 'text',
      text: msg.content,
    },
  ],
});

// Load messages from Supabase
const loadMessages = async () => {
  try {
    const supabaseMessages = await getMessages(props.roomId);
    messages.value = supabaseMessages.map(convertMessage);
    scrollToBottom();

    // Set last seen message when loading
    if (messages.value.length > 0) {
      lastSeenMessageId.value = messages.value[messages.value.length - 1].id;
    }
  } catch (error) {
    console.error('Failed to load messages:', error);
  }
};

const sendMessage = async () => {
  if (!inputMessage.value.trim()) return;

  const messageContent = inputMessage.value;
  inputMessage.value = "";

  try {
    // Send message to Supabase
    const sentMessage = await sendMessageToSupabase(props.roomId, messageContent);

    // Note: The message will be added via real-time subscription
    // But we can add it optimistically for better UX
    messages.value.push(convertMessage(sentMessage));
    scrollToBottom();
  } catch (error) {
    console.error('Failed to send message:', error);
    // Restore the input on error
    inputMessage.value = messageContent;
  }
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault();
    sendMessage();
  }
};

const closeDrawer = () => {
  isOpen.value = false;
};

// Mark messages as seen when drawer opens
const markAsSeen = () => {
  if (messages.value.length > 0) {
    lastSeenMessageId.value = messages.value[messages.value.length - 1].id;
    unreadCount.value = 0;
  }
};

// Setup real-time subscription
const setupSubscription = () => {
  if (unsubscribe) {
    unsubscribe();
  }

  unsubscribe = subscribeToMessages(props.roomId, (newMessage: Message) => {
    // Only add if it's not already in the list (avoid duplicates from optimistic updates)
    const exists = messages.value.some(m => m.id === newMessage.id);
    if (!exists) {
      messages.value.push(convertMessage(newMessage));
      scrollToBottom();

      // Increment unread count if drawer is closed and message is from another user
      if (!isOpen.value && newMessage.userId !== currentUserId.value) {
        unreadCount.value++;
      }
    }
  });
};

onMounted(async () => {
  isLoading.value = true;
  try {
    // Load existing messages
    await loadMessages();

    // Setup real-time subscription
    setupSubscription();
  } catch (error) {
    console.error('Failed to initialize chat:', error);
  } finally {
    isLoading.value = false;
  }
});

onUnmounted(() => {
  // Clean up subscription
  if (unsubscribe) {
    unsubscribe();
  }
});

watch(isOpen, (newValue) => {
  if (newValue) {
    scrollToBottom();
    markAsSeen();
  }
});

// Watch for room changes
watch(() => props.roomId, async (newRoomId) => {
  if (newRoomId) {
    messages.value = [];
    unreadCount.value = 0;
    lastSeenMessageId.value = null;
    await loadMessages();
    setupSubscription();
  }
});
</script>

<template>
  <UDrawer v-model="isOpen" direction="right">
    <div class="relative">
      <UButton @click="isOpen = true" color="neutral" variant="ghost" icon="i-lucide-message-square" :class="[
        'w-full justify-start cursor-pointer group relative w-full flex items-center gap-1.5 font-medium text-sm before:absolute before:z-[-1] before:rounded-md focus:outline-none focus-visible:outline-none dark:focus-visible:outline-none focus-visible:before:ring-inset focus-visible:before:ring-2 focus-visible:before:ring-primary flex-row px-2.5 py-1.5 before:inset-y-px before:inset-x-0 text-muted hover:text-highlighted hover:before:bg-elevated/50 transition-colors before:transition-colors',
        unreadCount > 0 && 'animate-pulse'
      ]" size="md">
        {{ roomName || 'Shard Chat' }}
      </UButton>

      <!-- Notification Badge -->
      <UTooltip v-if="unreadCount > 0" :text="`${unreadCount} new message${unreadCount > 1 ? 's' : ''}`"
        :popper="{ placement: 'right' }">
        <UBadge :label="unreadCount > 99 ? '99+' : unreadCount.toString()" color="primary" variant="solid" size="xs"
          class="absolute -top-1 -right-1 animate-bounce" />
      </UTooltip>
    </div>

    <template #content>
      <div class="flex flex-col h-full max-w-lg w-full overflow-hidden">
        <!-- Header -->
        <div class="flex items-center justify-between p-4 border-b dark:border-gray-800 flex-shrink-0">
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-users" class="w-5 h-5" />
            <h2 class="text-lg font-semibold">
              {{ roomName || 'Shard Chat' }}
              <UBadge label="Beta" variant="subtle" color="warning" icon="i-lucide-flask-conical" size="sm"
                class="ml-4" />
            </h2>
          </div>
          <UButton @click="closeDrawer" color="neutral" variant="ghost" icon="i-lucide-x" size="sm" />
        </div>

        <!-- Messages Container -->
        <div ref="messagesContainerRef"
          class="flex-1 overflow-y-auto overflow-x-hidden p-4 space-y-4 min-h-0 shard-scroll">
          <!-- Loading State -->
          <div v-if="isLoading" class="flex justify-center items-center h-full">
            <UIcon name="i-lucide-loader-2" class="w-6 h-6 animate-spin" />
          </div>

          <!-- Messages -->
          <template v-else>
            <div v-for="message in messages" :key="message.id" :class="[
              'flex w-full',
              message.role === 'user' ? 'justify-end' : 'justify-start',
            ]">
              <div :class="[
                'max-w-[80%] rounded-lg p-3 overflow-hidden',
                message.role === 'user'
                  ? 'bg-primary text-white'
                  : 'bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100',
              ]">
                <div class="flex flex-col gap-1">
                  <div class="flex items-center gap-2">
                    <UIcon name="i-lucide-user" class="w-4 h-4 flex-shrink-0" />
                    <span class="text-xs font-semibold opacity-90">
                      {{ message.displayName }}
                    </span>
                  </div>
                  <p class="text-sm whitespace-pre-wrap break-words overflow-wrap-anywhere min-w-0 pl-6">
                    {{ message.parts[0]?.text }}
                  </p>
                </div>
              </div>
            </div>

            <!-- Empty State -->
            <div v-if="messages.length === 0" class="flex flex-col items-center justify-center h-full text-gray-500">
              <UIcon name="i-lucide-message-circle" class="w-12 h-12 mb-2 opacity-50" />
              <p class="text-sm">No messages yet. Start the conversation!</p>
            </div>
          </template>
        </div>

        <!-- Input Area -->
        <div class="p-4 border-t dark:border-gray-800 flex-shrink-0">
          <div class="flex gap-2">
            <UTextarea v-model="inputMessage" placeholder="Type your message..." :rows="2" autoresize :maxrows="6"
              class="flex-1" @keydown="handleKeydown" />
            <UButton @click="sendMessage" :disabled="!inputMessage.trim()" icon="i-lucide-send" color="primary"
              size="md" class="self-end" />
          </div>
          <p class="text-xs text-gray-500 mt-2">
            Press Enter to send, Shift+Enter for new line
          </p>
        </div>
      </div>
    </template>
  </UDrawer>
</template>