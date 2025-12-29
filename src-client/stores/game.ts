import { defineStore } from "pinia";
import { ref } from "vue";
import type { GameMeta } from "~/interfaces/game";
import type { RealtimeChannel } from "@supabase/supabase-js";

export const useGameStore = defineStore("game", () => {
  const gameList = ref<GameMeta[] | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  let realtimeChannel: RealtimeChannel | null = null;

  const loadData = async () => {
    loading.value = true;
    error.value = null;
    
    try {
      const supabase = useSupabase();
      const { data, error: fetchError } = await supabase.from("games").select();
      
      if (fetchError) throw fetchError;
      
      gameList.value = data;
    } catch (err) {
      error.value = err instanceof Error ? err.message : "Failed to load games";
      console.error("Error fetching games:", err);
    } finally {
      loading.value = false;
    }
  };

  const subscribeToChanges = () => {
    const supabase = useSupabase();
    
    // Unsubscribe from previous channel if exists
    if (realtimeChannel) {
      supabase.removeChannel(realtimeChannel);
    }

    realtimeChannel = supabase
      .channel("games-changes")
      .on(
        "postgres_changes",
        {
          event: "*", // Listen to all events (INSERT, UPDATE, DELETE)
          schema: "public",
          table: "games",
        },
        (payload) => {
          console.log("Realtime update:", payload);
          
          if (!gameList.value) return;

          if (payload.eventType === "INSERT") {
            // Add new game to the list
            gameList.value = [...gameList.value, payload.new as GameMeta];
          } else if (payload.eventType === "UPDATE") {
            // Update existing game
            gameList.value = gameList.value.map((game) =>
              game.id === payload.new.id ? (payload.new as GameMeta) : game
            );
          } else if (payload.eventType === "DELETE") {
            // Remove deleted game
            gameList.value = gameList.value.filter(
              (game) => game.id !== payload.old.id
            );
          }
        }
      )
      .subscribe();
  };

  const unsubscribe = () => {
    if (realtimeChannel) {
      const supabase = useSupabase();
      supabase.removeChannel(realtimeChannel);
      realtimeChannel = null;
    }
  };

  return {
    gameList,
    loading,
    error,
    loadData,
    subscribeToChanges,
    unsubscribe,
  };
});