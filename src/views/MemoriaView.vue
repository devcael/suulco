<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";
import type { InputDestination } from "../core/services/input_service";
import { useMemoria } from "../composables/useMemoria";
import InputBar from "../components/InputBar.vue";
import MemoriaCard from "../components/cards/MemoriaCard.vue";
import { formatResurfaceDate } from "../lib/formatters";
import CategoryModal from "../components/modals/CategoryModal.vue";
import MemoriaEditorModal from "../components/MemoriaEditorModal.vue";

const router = useRouter();
const route = useRoute();
const {
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
} = useMemoria();

function onSubmitted(dest: InputDestination) {
  if (dest === route.path.slice(1)) {
    load();
  } else {
    router.push(`/${dest}`);
  }
}
</script>

<template>
  <div class="flex flex-col min-h-full">
    <div class="flex-1">
      <div class="flex gap-1.5 pt-item-y pb-2.5 flex-wrap">
        <button
          class="text-xs px-2.5 py-0.5 rounded-sm border font-body transition-all duration-150 cursor-pointer"
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
          class="text-xs px-2.5 py-0.5 rounded-sm border font-body transition-all duration-150 cursor-pointer"
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
        <div class="text-xs text-accent mb-3">
          {{ formatResurfaceDate(resurface.createdAt) }}
        </div>
        <div class="font-display text-[17px] text-fg mb-4">
          {{ resurface.title }}
        </div>
        <div class="flex gap-2">
          <button
            class="text-sm px-2.5 py-1 rounded-sm border border-line-strong bg-transparent text-fg-secondary cursor-pointer font-body transition-all duration-150 hover:border-fg-secondary hover:text-fg"
            @click="keep(resurface!.id)"
          >
            ainda quero isso
          </button>
          <button
            class="text-sm px-2.5 py-1 rounded-sm border border-line bg-transparent text-fg-subtle cursor-pointer font-body transition-all duration-150 hover:border-fg-subtle hover:text-fg-secondary"
            @click="archive(resurface!.id)"
          >
            deixar ir
          </button>
        </div>
      </div>

      <div class="pt-1">
        <MemoriaCard
          v-for="item in filteredItems"
          :key="item.id"
          :item="item"
          @keep="keep(item.id)"
          @archive="archive(item.id)"
          @categorize="catModalItemId = item.id"
          @edit="memoriaEditItem = { id: item.id, text: item.text }"
        />

        <div
          v-if="filteredItems.length === 0"
          class="flex flex-col items-center justify-center text-center pt-16 pb-6 gap-2"
        >
          <p class="font-display text-2xl text-fg">{{ searchQuery ? "nenhum resultado." : "memória vazia." }}</p>
          <p class="font-body text-sm text-fg-subtle">{{ searchQuery ? "tente outras palavras." : "o que você quer lembrar?" }}</p>
        </div>
      </div>
    </div>

    <InputBar initial-target="hoje" @submitted="onSubmitted" />
  </div>

  <CategoryModal
    v-if="catModalItemId !== null"
    :item-id="catModalItemId"
    @close="catModalItemId = null"
    @updated="catModalItemId = null; load()"
  />

  <MemoriaEditorModal
    v-if="memoriaEditItem !== null"
    :edit-item="memoriaEditItem"
    @close="memoriaEditItem = null"
    @submitted="memoriaEditItem = null; load()"
  />
</template>
