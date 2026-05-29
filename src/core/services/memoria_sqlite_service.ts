import { invoke } from "@tauri-apps/api/core";
import type { IMemoriaService } from "./memoria_service";
import type { MemoriaItem, ResurfaceCard, CategoryFilter, CategoryOption } from "../models";

export class MemoriaSqliteService implements IMemoriaService {

  async getMemoriaItems(category?: string): Promise<MemoriaItem[]> {
    const rawItems = await invoke<{ id: number; title: string; text: string; category: string | null; createdAt: string }[]>(
      "get_memoria_items",
      { category: category ?? null }
    );
    return rawItems.map((item) => ({
      ...item,
      createdAt: new Date(item.createdAt),
    }));
  }

  async getResurfacedItem(): Promise<ResurfaceCard | null> {
    const raw = await invoke<{ id: number; title: string; text: string; createdAt: string } | null>(
      "get_resurfaced_item"
    );
    if (!raw) return null;
    return { ...raw, createdAt: new Date(raw.createdAt) };
  }

  async keepMemoriaItemActive(id: number): Promise<void> {
    return invoke<void>("keep_memoria_item_active", { id });
  }

  async archiveMemoriaItem(id: number): Promise<void> {
    return invoke<void>("archive_memoria_item", { id });
  }

  async getCategoryFilters(): Promise<CategoryFilter[]> {
    return invoke<CategoryFilter[]>("get_categoria_filters");
  }

  async getCategoryOptions(itemId: number): Promise<CategoryOption[]> {
    return invoke<CategoryOption[]>("get_categoria_options", { itemId });
  }

  async setMemoriaItemCategory(id: number, category: string | null): Promise<void> {
    return invoke<void>("set_memoria_item_category", { id, category });
  }

  async createMemoriaCategory(name: string): Promise<void> {
    return invoke<void>("create_memoria_category", { name });
  }

  async updateMemoriaText(id: number, text: string): Promise<void> {
    return invoke<void>("update_memoria_text", { id, text });
  }

  async updateMemoriaTitle(id: number, title: string): Promise<void> {
    return invoke<void>("update_memoria_title", { id, title });
  }

}
