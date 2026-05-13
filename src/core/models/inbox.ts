
export interface InboxItem {
  id: number;
  text: string;
  isDone: boolean;
  createdAt: Date;
  wasCarried: boolean;
  linkedSulco: {
    id: number;
    text: string;
  } | null;
}

export interface InboxSulcoFilter {
  id: number;
  text: string;
  taskCount?: number;
}
