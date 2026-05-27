
export interface ResurfaceCard {
  id: number;
  title: string;
  text: string;
  createdAt: Date;
}

export interface MemoriaItem {
  id: number;
  title: string;
  text: string;
  category: string | null;
  createdAt: Date;
}

export interface CategoryFilter {
  name: string;
  count: number;
}
