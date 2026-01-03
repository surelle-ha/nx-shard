import { defineStore } from "pinia";
import { ref } from "vue";
import type { RealtimeChannel } from "@supabase/supabase-js";

export interface Announcement {
  id: number;
  title: string;
  content: any; // JSONB type
  createdAt: string;
}

export const useAnnouncementStore = defineStore("announcement", () => {
  const announcements = ref<Announcement[] | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  let realtimeChannel: RealtimeChannel | null = null;

  const loadData = async () => {
    loading.value = true;
    error.value = null;
    
    try {
      const supabase = useSupabase();
      const { data, error: fetchError } = await supabase
        .from("announcements")
        .select("*")
        .order("createdAt", { ascending: false });
      
      if (fetchError) throw fetchError;
      
      announcements.value = data;
    } catch (err) {
      error.value = err instanceof Error ? err.message : "Failed to load announcements";
      console.error("Error fetching announcements:", err);
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
      .channel("announcements-changes")
      .on(
        "postgres_changes",
        {
          event: "*", // Listen to all events (INSERT, UPDATE, DELETE)
          schema: "public",
          table: "announcements",
        },
        (payload) => {
          console.log("Announcement realtime update:", payload);
          
          if (!announcements.value) return;

          if (payload.eventType === "INSERT") {
            // Add new announcement at the beginning (newest first)
            announcements.value = [payload.new as Announcement, ...announcements.value];
          } else if (payload.eventType === "UPDATE") {
            // Update existing announcement
            announcements.value = announcements.value.map((announcement) =>
              announcement.id === payload.new.id ? (payload.new as Announcement) : announcement
            );
          } else if (payload.eventType === "DELETE") {
            // Remove deleted announcement
            announcements.value = announcements.value.filter(
              (announcement) => announcement.id !== payload.old.id
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

  // Get single announcement by ID
  const getById = (id: number) => {
    return announcements.value?.find((announcement) => announcement.id === id) || null;
  };

  return {
    announcements,
    loading,
    error,
    loadData,
    subscribeToChanges,
    unsubscribe,
    getById,
  };
});