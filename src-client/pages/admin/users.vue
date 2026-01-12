<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";

const pageMeta = {
  header: {
    name: "👾 User Manager",
    description: "Manage user accounts and approvals.",
  },
  showHeader: true,
};

definePageMeta({
  layout: "home",
});

// Define proper TypeScript interface
interface UserConfig {
  userId: string;
  name?: string;
  imageUrl?: string;
  power?: number;
  isApproved?: boolean;
  isDarkmode?: boolean;
  isExperimental?: boolean;
  isAnimatedHome?: boolean;
  isLogEnable?: boolean;
  isFtpInstall?: boolean;
  isMtpInstall?: boolean;
}

interface ManagedUser {
  id: string;
  email: string;
  name: string;
  imageUrl: string;
  power: number;
  isApproved: boolean;
  isDarkmode: boolean;
  isExperimental: boolean;
  isAnimatedHome: boolean;
  isLogEnable: boolean;
  isFtpInstall: boolean;
  isMtpInstall: boolean;
  createdAt: string;
}

const supabase = useSupabase();
const toast = useToast();
const isLoading = ref(true);
const users = ref<ManagedUser[]>([]);

const UAvatar = resolveComponent("UAvatar");
const UBadge = resolveComponent("UBadge");
const UButton = resolveComponent("UButton");

// Fetch users from Supabase
const fetchUsers = async () => {
  isLoading.value = true;
  try {
    const { data: usersData, error: usersError } =
      await supabase.auth.admin.listUsers();

    if (usersError) throw usersError;

    // Fetch user configs
    const { data: configsData, error: configsError } = await supabase
      .from("user_config")
      .select("*");

    if (configsError) throw configsError;

    // Merge auth users with their configs
    users.value = usersData.users.map((user: any) => {
      const config = (configsData as UserConfig[])?.find(
        (c) => c.userId === user.id
      );
      return {
        id: user.id,
        email: user.email,
        name: config?.name || "N/A",
        imageUrl:
          config?.imageUrl ||
          "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRADwvSebpVf1hSgjHDZyMEpVGxqiAlvE09dA&s",
        power: config?.power || 0,
        isApproved: config?.isApproved || false,
        isDarkmode: config?.isDarkmode || false,
        isExperimental: config?.isExperimental || false,
        isAnimatedHome: config?.isAnimatedHome || true,
        isLogEnable: config?.isLogEnable || false,
        isFtpInstall: config?.isFtpInstall || true,
        isMtpInstall: config?.isMtpInstall || true,
        createdAt: user.created_at,
      };
    });
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to fetch users",
      color: "error",
    });
  } finally {
    isLoading.value = false;
  }
};

// Toggle user approval
const toggleApproval = async (userId: string, currentStatus: boolean) => {
  try {
    const { error } = await supabase
      .from("user_config")
      .update({ isApproved: !currentStatus } as never)
      .eq("userId", userId);

    if (error) throw error;

    toast.add({
      title: "Success",
      description: `User ${!currentStatus ? "approved" : "unapproved"}`,
      color: "success",
    });

    // Refresh the list
    await fetchUsers();
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to update approval status",
      color: "error",
    });
  }
};

// Delete user
const deleteUser = async (userId: string) => {
  if (!confirm("Are you sure you want to delete this user?")) return;

  try {
    // Delete from auth (this will cascade to user_config due to foreign key)
    const { error } = await supabase.auth.admin.deleteUser(userId);

    if (error) throw error;

    toast.add({
      title: "Success",
      description: "User deleted successfully",
      color: "success",
    });

    // Refresh the list
    await fetchUsers();
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to delete user",
      color: "error",
    });
  }
};

const columns: TableColumn<ManagedUser>[] = [
  {
    accessorKey: "imageUrl",
    header: "Avatar",
    cell: ({ row }) =>
      h(UAvatar, {
        src: row.original.imageUrl,
        alt: row.original.name,
      }),
  },
  {
    accessorKey: "name",
    header: "Name",
  },
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    accessorKey: "power",
    header: "Power",
  },
  {
    accessorKey: "isApproved",
    header: "Status",
    cell: ({ row }) =>
      h(
        UBadge,
        {
          color: row.original.isApproved ? "success" : "warning",
          variant: "subtle",
        },
        () => (row.original.isApproved ? "Approved" : "Pending")
      ),
  },
  {
    accessorKey: "createdAt",
    header: "Joined",
    cell: ({ row }) =>
      new Date(row.original.createdAt).toLocaleDateString("en-US", {
        year: "numeric",
        month: "short",
        day: "numeric",
      }),
  },
  {
    accessorKey: "actions",
    header: "Actions",
    cell: ({ row }) =>
      h("div", { class: "flex gap-2" }, [
        h(
          UButton,
          {
            color: row.original.isApproved ? "warning" : "success",
            variant: "soft",
            size: "xs",
            onClick: () =>
              toggleApproval(row.original.id, row.original.isApproved),
          },
          () => (row.original.isApproved ? "Unapprove" : "Approve")
        ),
        h(
          UButton,
          {
            color: "error",
            variant: "soft",
            size: "xs",
            onClick: () => deleteUser(row.original.id),
          },
          () => "Delete"
        ),
      ]),
  },
];

onMounted(() => {
  fetchUsers();
});
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
    </div>

    <div class="p-6">
      <div class="flex justify-between items-center mb-4">
        <div>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Total Users: {{ users.length }}
          </p>
        </div>
        <UButton
          icon="i-lucide-refresh-cw"
          variant="soft"
          @click="fetchUsers"
          :loading="isLoading"
        >
          Refresh
        </UButton>
      </div>

      <UTable
        :data="users"
        :columns="columns"
        :loading="isLoading"
        :empty-state="{
          icon: 'i-lucide-users',
          label: 'No users found',
        }"
      />
    </div>
  </div>
</template>
