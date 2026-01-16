import type { Room, Message, RoomListItem } from '~/interfaces/chat';

export const useShardChat = () => {
  const supabase = useSupabase();
  const accountStore = useAccountStore();

  // Create a new room
  const createRoom = async (name: string, description?: string, isPrivate = false) => {
    if (!accountStore.account?.id) throw new Error('No user logged in');

    const { data, error } = await supabase
      .from('rooms')
      .insert({
        name,
        description,
        isPrivate,
        createdBy: accountStore.account.id,
      } as never)
      .select()
      .single();

    if (error) throw error;
    return data as Room;
  };

  // Get all rooms
  const getRooms = async () => {
    const { data, error } = await supabase
      .from('room_list')
      .select('*')
      .order('updatedAt', { ascending: false });

    if (error) throw error;
    return data as RoomListItem[];
  };

  // Get messages for a room
  const getMessages = async (roomId: string, limit = 50) => {
    const { data, error } = await supabase
      .from('messages')
      .select('*')
      .eq('roomId', roomId)
      .order('createdAt', { ascending: true })
      .limit(limit);

    if (error) throw error;
    return data as Message[];
  };

  // Send a message
  const sendMessage = async (roomId: string, content: string) => {
    if (!accountStore.account?.id) throw new Error('No user logged in');

    const { data, error } = await supabase
      .from('messages')
      .insert({
        roomId,
        userId: accountStore.account.id,
        displayName: accountStore.account.displayName,
        content,
        messageType: 'text',
      } as never)
      .select()
      .single();

    console.trace(error)

    if (error) throw error;
    return data as Message;
  };

  // Subscribe to messages in a room
  const subscribeToMessages = (roomId: string, callback: (message: Message) => void) => {
    const channel = supabase
      .channel(`room:${roomId}`)
      .on(
        'postgres_changes',
        {
          event: 'INSERT',
          schema: 'public',
          table: 'messages',
          filter: `roomId=eq.${roomId}`,
        },
        (payload) => {
          callback(payload.new as Message);
        }
      )
      .subscribe();

    return () => {
      supabase.removeChannel(channel);
    };
  };

  // Delete a room (only creator can do this via RLS)
  const deleteRoom = async (roomId: string) => {
    const { error } = await supabase
      .from('rooms')
      .delete()
      .eq('id', roomId);

    if (error) throw error;
  };

  // Update a message
  const updateMessage = async (messageId: string, content: string) => {
    const { data, error } = await supabase
      .from('messages')
      .update({ 
        content,
        isEdited: true,
        updatedAt: new Date().toISOString()
      } as never)
      .eq('id', messageId)
      .select()
      .single();

    if (error) throw error;
    return data as Message;
  };

  // Delete a message (soft delete)
  const deleteMessage = async (messageId: string) => {
    const { error } = await supabase
      .from('messages')
      .update({ 
        isDeleted: true,
        content: '[Message deleted]'
      } as never)
      .eq('id', messageId);

    if (error) throw error;
  };

  return {
    createRoom,
    getRooms,
    getMessages,
    sendMessage,
    subscribeToMessages,
    deleteRoom,
    updateMessage,
    deleteMessage,
  };
};