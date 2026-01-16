// types/supabase.ts
export interface Room {
    id: string;
    name: string;
    description?: string;
    createdBy: string;
    createdAt: string;
    updatedAt: string;
    isPrivate: boolean;
    metadata: Record<string, any>;
  }
  
  export interface Message {
    id: string;
    roomId: string;
    userId: string;
    displayName: string;
    content: string;
    messageType: 'text' | 'image' | 'file' | 'system';
    metadata: Record<string, any>;
    createdAt: string;
    updatedAt: string;
    isEdited: boolean;
    isDeleted: boolean;
  }
  
  export interface RoomListItem extends Room {
    messageCount: number;
    lastMessage?: {
      id: string;
      content: string;
      displayName: string;
      createdAt: string;
    };
  }
  
  // Component-specific message type
  export interface ChatMessage {
    id: string;
    role: 'user' | 'other';
    displayName: string;
    parts: Array<{
      type: string;
      text: string;
    }>;
  }