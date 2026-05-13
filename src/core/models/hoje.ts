export interface HojeItem {
  id: number;
  text: string;
  isDone: boolean;
  linkedSulco: {
    id: number;
    text: string;
  } | null;
}

export interface HojeSummary {
  total: number;
  pending: number;
  done: number;
}
