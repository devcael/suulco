<script setup lang="ts">
import { ref, inject, computed, onMounted, watch } from "vue";
import type { IMemoriaService } from "../core/services/memoria_service";
import type { MemoriaItem, ResurfaceCard, CategoryFilter } from "../core/models";
import { formatRelativeDate, formatResurfaceDate } from "../lib/formatters";

const memoriaService = inject<IMemoriaService>("memoriaService")!;

const items = ref<MemoriaItem[]>([]);
const resurface = ref<ResurfaceCard | null>(null);
const categoryFilters = ref<CategoryFilter[]>([]);
const activeFilter = ref("todos");
const searchQuery = ref("");

const emit = defineEmits<{
  (e: "open-cat-modal", itemId: number): void;
  (e: "open-memoria-edit", id: number, text: string): void;
}>();

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

defineExpose({ load });

onMounted(load);
</script>

<template>
  <div>
    <div class="flex gap-1.5 pt-[18px] pb-2.5 flex-wrap">
      <button
        class="text-xs px-2.5 py-0.5 rounded-sm border font-body tracking-wide transition-all duration-150 cursor-pointer"
        :class="activeFilter === 'todos'
          ? 'bg-interactive text-interactive-fg border-interactive'
          : 'bg-transparent text-fg-subtle border-line'"
        @click="activeFilter = 'todos'"
      >
        todos
      </button>
      <button
        v-for="cat in categoryFilters"
        :key="cat.name"
        class="text-xs px-2.5 py-0.5 rounded-sm border font-body tracking-wide transition-all duration-150 cursor-pointer"
        :class="activeFilter === cat.name
          ? 'bg-accent border-accent text-interactive-fg'
          : 'bg-transparent text-fg-subtle border-line'"
        @click="activeFilter = cat.name"
      >
        {{ cat.name }}
      </button>
    </div>

    <div class="mb-3">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="pesquisar..."
        class="w-full bg-transparent border-b border-line outline-none font-body text-sm text-fg py-1.5 placeholder:text-fg-subtle placeholder:italic transition-[border-color] duration-200 focus:border-accent"
      />
    </div>

    <div
      v-if="resurface && activeFilter === 'todos' && !searchQuery"
      class="mb-4 bg-bg-subtle border border-line-strong rounded-md p-[18px_20px] border-l-2 border-l-accent"
    >
      <div class="text-xs tracking-widest text-accent mb-3">
        {{ formatResurfaceDate(resurface.createdAt) }}
      </div>
      <div class="font-display text-[17px] text-fg leading-[1.65] mb-4">
        {{ resurface.text }}
      </div>
      <div class="flex gap-2">
        <button
          class="text-sm px-2.5 py-1 rounded-sm border border-line-strong bg-transparent text-fg-secondary cursor-pointer font-body tracking-tight transition-all duration-150 hover:border-fg-secondary hover:text-fg"
          @click="keep(resurface!.id)"
        >
          ainda quero isso
        </button>
        <button
          class="text-sm px-2.5 py-1 rounded-sm border border-line bg-transparent text-fg-subtle cursor-pointer font-body tracking-tight transition-all duration-150 hover:border-fg-subtle hover:text-fg-secondary"
          @click="archive(resurface!.id)"
        >
          deixar ir
        </button>
      </div>
    </div>

    <div class="pt-1">
      <div
        v-for="item in filteredItems"
        :key="item.id"
        class="py-5 border-b border-line"
      >
        <div class="mb-2.5">
          <div
            class="font-display text-lg text-fg-secondary leading-[1.7] cursor-text"
            @dblclick="emit('open-memoria-edit', item.id, item.text)"
          >{{ item.text }}</div>
        </div>
        <div class="flex items-center justify-between flex-wrap gap-2">
          <span class="text-xs tracking-wider text-fg-subtle">
            {{ formatRelativeDate(item.createdAt) }}
          </span>
          <div class="flex gap-1.5 items-center flex-wrap">
            <button
              class="text-xs px-2 py-0.5 rounded-sm border border-line-strong text-accent bg-transparent cursor-pointer tracking-wide font-body transition-all duration-150 hover:bg-bg-muted"
              @click="emit('open-cat-modal', item.id)"
            >
              {{ item.category || "categorizar" }}
            </button>
            <button
              class="text-sm px-2.5 py-1 rounded-sm border border-line-strong bg-transparent text-fg-secondary cursor-pointer font-body tracking-tight transition-all duration-150 hover:border-fg-secondary hover:text-fg"
              @click="keep(item.id)"
            >
              ainda quero isso
            </button>
            <button
              class="text-sm px-2.5 py-1 rounded-sm border border-line bg-transparent text-fg-subtle cursor-pointer font-body tracking-tight transition-all duration-150 hover:border-fg-subtle hover:text-fg-secondary"
              @click="archive(item.id)"
            >
              deixar ir
            </button>
          </div>
        </div>
      </div>

      <p v-if="filteredItems.length === 0" class="font-display italic text-xl text-fg-subtle leading-loose py-9">
        {{ searchQuery ? "nenhum resultado." : "nada aqui ainda." }}
      </p>
    </div>
  </div>
</template>
