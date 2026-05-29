<script setup lang="ts">
import type { SulcoListItem } from "../../core/models";

defineProps<{
  item: SulcoListItem;
  definition: string;
  taskInput: string;
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: "toggle-open"): void;
  (e: "update:definition", value: string): void;
  (e: "save-def"): void;
  (e: "update:task-input", value: string): void;
  (e: "add-linked-task"): void;
  (e: "archive"): void;
  (e: "edit"): void;
}>();
</script>

<template>
  <div
    class="border-b border-line last:border-b-0 group"
    :class="{ open: isOpen }"
  >
    <div
      class="flex items-center gap-4 py-[22px] cursor-pointer select-none"
      @click="emit('toggle-open')"
    >
      <div
        class="w-2 h-2 border border-fg-subtle rotate-45 shrink-0 transition-all duration-200"
        :class="isOpen ? 'bg-accent border-accent' : 'bg-transparent'"
      />
      <span
        class="font-body text-xl text-fg flex-1 cursor-text"
        @dblclick.stop="emit('edit')"
      >{{ item.text }}</span>

      <button
        class="bg-transparent border-none text-fg-subtle text-sm cursor-pointer font-body opacity-0 group-hover:opacity-100 transition-opacity duration-150 px-1 py-0.5 whitespace-nowrap"
        @click.stop="emit('archive')"
      >
        arquivar
      </button>

      <svg
        class="text-fg-subtle shrink-0 transition-transform duration-200"
        :class="isOpen ? 'rotate-180' : ''"
        width="13"
        height="13"
        viewBox="0 0 13 13"
        fill="none"
      >
        <path
          d="M2 4.5l4.5 4 4.5-4"
          stroke="currentColor"
          stroke-width="1.2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </div>

    <div
      class="overflow-hidden transition-all duration-300 ease-in-out"
      :style="{ maxHeight: isOpen ? '420px' : '0px' }"
    >
      <div class="pt-0.5 pb-[22px] pl-6">
        <textarea
          class="w-full bg-bg-subtle border border-line rounded-md px-3.5 py-3 font-display text-[15px] text-fg resize-none min-h-[76px] outline-none transition-[border-color] duration-200 focus:border-accent placeholder:text-fg-subtle placeholder:italic placeholder:font-body placeholder:text-[13px]"
          :value="definition"
          placeholder="o que isso significa pra você? como chegar lá?"
          rows="3"
          @input="emit('update:definition', ($event.target as HTMLTextAreaElement).value)"
          @blur="emit('save-def')"
        />
        <div class="flex gap-2 items-center mt-3">
          <input
            class="flex-1 bg-bg-subtle border border-line rounded-md px-3 py-2 font-body text-[13px] text-fg outline-none transition-[border-color] duration-200 focus:border-accent placeholder:text-fg-subtle placeholder:italic placeholder:text-xs"
            :value="taskInput"
            placeholder="adicionar task vinculada..."
            @input="emit('update:task-input', ($event.target as HTMLInputElement).value)"
            @keydown.enter="emit('add-linked-task')"
          >
          <button
            class="w-8 h-8 rounded-md bg-interactive border-none cursor-pointer flex items-center justify-center shrink-0 transition-opacity duration-150 hover:opacity-[0.78]"
            @click="emit('add-linked-task')"
          >
            <svg
              width="12"
              height="12"
              viewBox="0 0 15 15"
              fill="none"
            >
              <path
                d="M7.5 12.5V2.5M7.5 2.5L3 7M7.5 2.5L12 7"
                stroke="#F7FAF8"
                stroke-width="1.3"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
