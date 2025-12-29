import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { User } from "~/interfaces/user";

export const useAccountStore = defineStore("account", () => {
  const supabase = useSupabase();
  const account = ref<User | null>(null);

  const login = async (email: string, password: string) => {
    const { data, error } = await supabase.auth.signInWithPassword({
      email,
      password,
    });

    if (error) throw error;

    if (data.user) {
      const { data: configData } = await supabase
        .from("user_config")
        .select("*")
        .eq("userId", data.user.id)
        .single();

      account.value = {
        id: data.user.id,
        name: (configData as any)?.name || "Legendary Cat",
        email: data.user.email!,
        password: "NA",
        power: (configData as any)?.power || 10,
        imageUrl:
          (configData as any)?.imageUrl ||
          "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRADwvSebpVf1hSgjHDZyMEpVGxqiAlvE09dA&s",
        createdAt: data.user.created_at,
      };
    }

    return data;
  };

  const logout = async () => {
    const { error } = await supabase.auth.signOut();
    if (error) throw error;
    account.value = null;
  };

  const isAuthenticated = computed(() => account.value !== null);

  const initializeUser = async () => {
    const { data } = await supabase.auth.getUser();
    if (data.user) {
      // Changed 'user_id' to 'userId' to match your camelCase column
      const { data: configData } = await supabase
        .from("user_config")
        .select("*")
        .eq("userId", data.user.id)
        .single();

      account.value = {
        id: data.user.id,
        name: (configData as any)?.name || "Legendary Cat",
        email: data.user.email!,
        password: "NA",
        power: (configData as any)?.power || 10,
        imageUrl:
          (configData as any)?.imageUrl ||
          "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRADwvSebpVf1hSgjHDZyMEpVGxqiAlvE09dA&s",
        createdAt: data.user.created_at,
      };
    }
  };

  return {
    login,
    logout,
    initializeUser,
    isAuthenticated,
    account,
  };
});
