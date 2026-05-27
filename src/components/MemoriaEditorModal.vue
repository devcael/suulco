<script setup lang="ts">
import { ref, inject, onMounted, onUnmounted } from "vue";
import type { IInputService } from "../core/services/input_service";
import type { IMemoriaService } from "../core/services/memoria_service";
import type { CategoryFilter } from "../core/models";

const props = defineProps<{
  editItem?: { id: number; text: string };
}>();

const inputService = inject<IInputService>("inputService")!;
const memoriaService = inject<IMemoriaService>("memoriaService")!;

const emit = defineEmits<{
  (e: "close"): void;
  (e: "submitted"): void;
}>();

const text = ref(props.editItem?.text ?? "");
const categories = ref<CategoryFilter[]>([]);
const selectedCategory = ref<string | null>(null);

onMounted(async () => {
  categories.value = await memoriaService.getCategoryFilters();

  if (props.editItem) {
    const opts = await memoriaService.getCategoryOptions(props.editItem.id);
    const current = opts.find((o) => o.isCurrentlySelected);
    if (current) selectedCategory.value = current.name;
  }

  document.addEventListener("keydown", onKeydown);
});

onUnmounted(() => {
  document.removeEventListener("keydown", onKeydown);
});

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") emit("close");
  if (e.key === "s" && (e.ctrlKey || e.metaKey)) {
    e.preventDefault();
    save();
  }
}

function toggleCategory(name: string) {
  selectedCategory.value = selectedCategory.value === name ? null : name;
}

async function save() {
  const t = text.value.trim();
  if (!t) return;

  if (props.editItem) {
    await memoriaService.updateMemoriaText(props.editItem.id, t);
    await memoriaService.setMemoriaItemCategory(props.editItem.id, selectedCategory.value);
  } else {
    await inputService.createGlobalItem("memoria", t, {
      category: selectedCategory.value ?? undefined,
    });
  }

  emit("submitted");
  emit("close");
}
</script>

<template>
  <div class="fixed inset-0 z-50 flex flex-col bg-bg">
    <div class="flex items-center gap-3 px-6 pt-5 pb-4 border-b border-line shrink-0 flex-wrap">
      <button
        class="w-7 h-7 rounded-sm border border-line bg-transparent text-fg-subtle cursor-pointer flex items-center justify-center hover:border-fg-subtle hover:text-fg transition-all duration-150 shrink-0"
        @click="emit('close')"
        title="fechar (Esc)"
      >
        <svg width="11" height="11" viewBox="0 0 11 11" fill="none">
          <path d="M1 1L10 10M10 1L1 10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
        </svg>
      </button>

      <span class="font-body text-sm text-fg-subtle shrink-0">
        {{ editItem ? "editar anotação" : "nova anotação" }}
      </span>

      <div class="flex gap-1.5 flex-wrap">
        <button
          v-for="cat in categories"
          :key="cat.name"
          class="text-xs px-[11px] py-1 rounded-sm border font-body transition-all duration-150 cursor-pointer"
          :class="selectedCategory === cat.name
            ? 'bg-accent border-accent text-interactive-fg'
            : 'bg-transparent text-fg-subtle border-line hover:border-fg-subtle'"
          @click="toggleCategory(cat.name)"
        >
          {{ cat.name }}
        </button>
      </div>

      <div class="ml-auto">
        <button
          class="text-xs px-[11px] py-1 rounded-sm border font-body transition-all duration-150 cursor-pointer bg-interactive text-interactive-fg border-interactive hover:opacity-[0.85] disabled:opacity-40"
          :disabled="!text.trim()"
          @click="save"
          title="salvar (Ctrl+S)"
        >
          salvar
        </button>
      </div>
    </div>


    <input
    />

    <textarea
      class="flex-1 w-full bg-transparent border-none outline-none resize-none font-body text-base text-fg px-6 py-5 placeholder:text-fg-subtle placeholder:italic"
      v-model="text"
      :placeholder="editItem ? '' : 'escreva em markdown...'"
      autofocus
    />
  </div>
</template>
