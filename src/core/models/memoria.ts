
export interface ResurfaceCard {
  id: number;
  text: string;
  createdAt: Date;
}

export interface MemoriaItem {
  id: number;
  text: string;
  category: string | null;
  createdAt: Date;
}

export interface CategoryFilter {
  name: string;
  count: number;
}
