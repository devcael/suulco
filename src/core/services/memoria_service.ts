import {
  ResurfaceCard,
  MemoriaItem,
  CategoryFilter,
  CategoryOption,
} from "../models";

export interface IMemoriaService {
  getMemoriaItems(category?: string): Promise<MemoriaItem[]>;
  getResurfacedItem(): Promise<ResurfaceCard | null>;
  keepMemoriaItemActive(id: number): Promise<void>;
  archiveMemoriaItem(id: number): Promise<void>;
  getCategoryFilters(): Promise<CategoryFilter[]>;
  getCategoryOptions(itemId: number): Promise<CategoryOption[]>;
  setMemoriaItemCategory(id: number, category: string | null): Promise<void>;
  createMemoriaCategory(name: string): Promise<void>;
  updateMemoriaText(id: number, text: string): Promise<void>;
  updateMemoriaTitle(id: number, title: string): Promise<void>;
}
