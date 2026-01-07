import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { User } from "~/interfaces/user";
import { useGameStore } from "./game";

const userToAccountMap = (user: any, config: any): User => {
  return {
    id: user.id,
    displayName: config?.displayName || "Legendary Cat",
    email: user.email!,
    password: "NA",
    power: config?.power || 10,
    imageUrl:
      config?.imageUrl ||
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRADwvSebpVf1hSgjHDZyMEpVGxqiAlvE09dA&s",
    createdAt: user.created_at,
    isDarkmode: config?.isDarkmode || false,
    isExperimental: config?.isExperimental || false,
    isApproved: config?.isApproved || false,
    isNew: config?.isNew || true,
    isAdmin: config?.isAdmin || false
  };
};

export const useAccountStore = defineStore("account", () => {
  const supabase = useSupabase();
  const account = ref<User | null>(null);
  const library = ref<number[] | null>(null);
  const session = ref<any | null>(null);
  const isLoading = ref(false); // Add loading state to prevent double execution

  // Extract user loading logic into separate function
  const loadUserData = async (user: any) => {
    // Prevent double loading
    if (isLoading.value) return;
    isLoading.value = true;

    try {
      const { data: configData } = await supabase
        .from("user_config")
        .select("*")
        .eq("userId", user.id)
        .single();

      const { data: libraryData } = await supabase
        .from("user_library")
        .select("*")
        .eq("userId", user.id)
        .single();

      account.value = userToAccountMap(user, configData);
      library.value = (libraryData as any)?.gameId || [];

      if (!account.value.isApproved) {
        await supabase.auth.signOut();
        account.value = null;
        library.value = null;
        throw new Error("Account is being processed. Please try again later.");
      }
    } finally {
      isLoading.value = false;
    }
  };

  // Add auth state listener
  const setupAuthListener = () => {
    supabase.auth.onAuthStateChange(async (event, currentSession) => {
      console.log("Auth state changed:", event);

      // Only handle TOKEN_REFRESHED here, not SIGNED_IN (login handles that)
      if (event === "TOKEN_REFRESHED") {
        session.value = currentSession;
        if (currentSession?.user && !isLoading.value) {
          await loadUserData(currentSession.user);
        }
      } else if (event === "SIGNED_OUT") {
        account.value = null;
        library.value = null;
        session.value = null;
      }
    });
  };

  const login = async (email: string, password: string) => {
    const { data, error } = await supabase.auth.signInWithPassword({
      email,
      password,
    });

    if (error) throw error;

    if (data.user) {
      session.value = data.session;
      await loadUserData(data.user); // This handles SIGNED_IN
    }

    return data;
  };

  const logout = async () => {
    const { error } = await supabase.auth.signOut();
    if (error) throw error;
    account.value = null;
    library.value = null;
    session.value = null;
  };

  const isAuthenticated = computed(() => account.value !== null);

  const initializeUser = async () => {
    // Get current session (this will refresh if needed)
    const {
      data: { session: currentSession },
    } = await supabase.auth.getSession();

    if (currentSession?.user) {
      session.value = currentSession;
      await loadUserData(currentSession.user);
    }
  };

  const updateUserConfig = async (updates: Partial<User>) => {
    if (!account.value) throw new Error("No user logged in");

    const { error } = await supabase
      .from("user_config")
      .update(updates as never)
      .eq("userId", account.value.id);

    if (error) throw error;
    account.value = { ...account.value, ...updates };
  };

  const getLibraryGames = computed(() => {
    const gameStore = useGameStore();

    if (!library.value || !gameStore.gameList) {
      return [];
    }

    return gameStore.gameList.filter((game) =>
      library.value!.includes(game.id)
    );
  });

  const addToLibrary = async (gameId: number) => {
    if (!account.value) throw new Error("No user logged in");

    try {
      // Check if game is already in library
      if (library.value?.includes(gameId)) {
        throw new Error("Game already in library");
      }

      // Add game to library array
      const updatedLibrary = [...(library.value || []), gameId];

      const { error } = await supabase
        .from("user_library")
        .upsert(
          {
            userId: account.value.id,
            gameId: updatedLibrary
          },
          {
            onConflict: 'userId' // Specify the unique constraint column
          }
        );

      if (error) {
        console.error("Supabase error:", error);
        throw error;
      }

      // Update local state
      library.value = updatedLibrary;

      return true;
    } catch (error) {
      console.error("Failed to add game to library:", error);
      throw error;
    }
  };

  const removeFromLibrary = async (gameId: number) => {
    if (!account.value) throw new Error("No user logged in");

    try {
      // Remove game from library array
      const updatedLibrary = (library.value || []).filter(
        (id) => id !== gameId
      );

      const { error } = await supabase
        .from("user_library")
        .upsert(
          {
            userId: account.value.id,
            gameId: updatedLibrary
          },
          {
            onConflict: 'userId'
          }
        );

      if (error) {
        console.error("Supabase error:", error);
        throw error;
      }

      // Update local state
      library.value = updatedLibrary;

      return true;
    } catch (error) {
      console.error("Failed to remove game from library:", error);
      throw error;
    }
  };

  const isInLibrary = (gameId: number) => {
    return library.value?.includes(gameId) || false;
  };

  // Initialize auth listener when store is created
  setupAuthListener();

  return {
    login,
    logout,
    initializeUser,
    updateUserConfig,
    isAuthenticated,
    account,
    library,
    getLibraryGames,
    addToLibrary,
    removeFromLibrary,
    isInLibrary,
    session,
    isLoading, // Export if you need it in components
  };
});
