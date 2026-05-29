<script setup lang="ts">
import { ref, inject, onMounted } from "vue";
import type { IMemoriaService } from "../../core/services/memoria_service";
import type { CategoryOption } from "../../core/models";

const props = defineProps<{
  itemId: number;
}>();

const emit = defineEmits<{
  (e: "close"): void;
  (e: "updated"): void;
}>();

const memoriaService = inject<IMemoriaService>("memoriaService")!;

const options = ref<CategoryOption[]>([]);
const newCatInput = ref("");

async function load() {
  options.value = await memoriaService.getCategoryOptions(props.itemId);
}

async function selectCat(name: string) {
  await memoriaService.setMemoriaItemCategory(props.itemId, name);
  emit("updated");
  emit("close");
}

async function createAndApply() {
  const name = newCatInput.value.trim();
  if (!name) return;
  await memoriaService.createMemoriaCategory(name);
  await memoriaService.setMemoriaItemCategory(props.itemId, name);
  emit("updated");
  emit("close");
}

onMounted(load);
</script>

<template>
  <div
    class="fixed inset-0 bg-overlay z-[100] flex items-end justify-center"
    @click="emit('close')"
  >
    <div
      class="bg-bg w-full max-w-[580px] rounded-t-lg px-8 pt-7 pb-10 max-h-[70vh] overflow-y-auto"
      @click.stop
    >
      <div class="font-display text-[20px] text-fg mb-1.5">
        categorizar
      </div>
      <div class="text-[12px] text-fg-subtle mb-[22px] ">
        escolha ou crie uma categoria
      </div>

      <div class="flex flex-col gap-0.5">
        <div
          v-for="opt in options"
          :key="opt.name"
          class="flex items-center gap-3.5 py-3.5 border-b border-line cursor-pointer transition-opacity duration-150 hover:opacity-70 last:border-b-0"
          @click="selectCat(opt.name)"
        >
          <div
            class="w-[7px] h-[7px] border border-fg-subtle rotate-45 shrink-0"
            :class="opt.isCurrentlySelected ? 'bg-accent border-accent' : ''"
          />
          <span class="font-display text-xl text-fg flex-1">{{ opt.name }}</span>
        </div>
      </div>

      <div class="mt-5">
        <input
          v-model="newCatInput"
          class="w-full bg-bg-subtle border border-line rounded-md px-3.5 py-[11px] font-display text-lg text-fg outline-none mb-3 transition-[border-color] duration-200 focus:border-accent placeholder:text-fg-subtle placeholder:italic placeholder:text-base placeholder:font-body"
          placeholder="nova categoria..."
          @keydown.enter="createAndApply"
        >
        <button
          class="w-full py-3 rounded-md border-none bg-interactive text-interactive-fg font-body text-[12px] cursor-pointer mb-2"
          @click="createAndApply"
        >
          criar e aplicar
        </button>
        <button
          class="w-full py-[11px] rounded-md border border-line bg-transparent text-fg-subtle font-body text-[12px] cursor-pointer"
          @click="emit('close')"
        >
          cancelar
        </button>
      </div>
    </div>
  </div>
</template>
