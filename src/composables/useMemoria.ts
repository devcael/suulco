import { ref, inject, computed, onMounted, watch } from "vue";
import type { IMemoriaService } from "../core/services/memoria_service";
import type { MemoriaItem, ResurfaceCard, CategoryFilter } from "../core/models";

export function useMemoria() {
  const memoriaService = inject<IMemoriaService>("memoriaService")!;

  const items = ref<MemoriaItem[]>([]);
  const resurface = ref<ResurfaceCard | null>(null);
  const categoryFilters = ref<CategoryFilter[]>([]);
  const activeFilter = ref("todos");
  const searchQuery = ref("");
  const catModalItemId = ref<number | null>(null);
  const memoriaEditItem = ref<{ id: number; text: string } | null>(null);

  async function loadFilters() {
    categoryFilters.value = await memoriaService.getCategoryFilters();
  }

  async function loadItems() {
    const cat = activeFilter.value === "todos" ? undefined : activeFilter.value;
    items.value = await memoriaService.getMemoriaItems(cat);
  }

  async function loadResurface() {
    resurface.value = await memoriaService.getResurfacedItem();
  }

  async function load() {
    await Promise.all([loadFilters(), loadItems(), loadResurface()]);
  }

  watch(activeFilter, loadItems);

  const filteredItems = computed(() => {
    const q = searchQuery.value.trim().toLowerCase();
    if (!q) return items.value;
    return items.value.filter((item) => item.text.toLowerCase().includes(q));
  });

  async function keep(id: number) {
    await memoriaService.keepMemoriaItemActive(id);
    await load();
  }

  async function archive(id: number) {
    await memoriaService.archiveMemoriaItem(id);
    await load();
  }

  onMounted(load);

  return {
    items,
    resurface,
    categoryFilters,
    activeFilter,
    searchQuery,
    filteredItems,
    catModalItemId,
    memoriaEditItem,
    load,
    keep,
    archive,
  };
}
