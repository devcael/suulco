<script setup lang="ts">
import type { HojeItem } from "../../core/models";

defineProps<{ item: HojeItem }>();

const emit = defineEmits<{
  (e: "toggle"): void;
  (e: "remove"): void;
  (e: "defer"): void;
  (e: "link-click"): void;
  (e: "edit"): void;
}>();
</script>

<template>
  <div
    class="flex items-start gap-4 py-item-y border-b border-line last:border-b-0 group p-4 max-w-1/2"
  >
    <div
      class="w-4.25 h-4.25 border rounded-sm shrink-0 mt-1 cursor-pointer flex items-center justify-center transition-all duration-150 rotate-[1deg]"
      :class="
        item.isDone
          ? 'bg-positive border-positive rotate-[-0.5deg]'
          : 'bg-transparent border-fg-subtle'
      "
      @click="emit('toggle')"
    >
      <svg
        v-if="item.isDone"
        width="9"
        height="9"
        viewBox="0 0 9 9"
        fill="none"
      >
        <path
          d="M1.5 4.5l2 2 4-4"
          stroke="white"
          stroke-width="1.3"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </div>

    <div class="flex-1 flex flex-col gap-1.5">
      <span
        class="font-body text-xl text-fg transition-colors duration-200 cursor-text tracking-tight"
        :class="
          item.isDone ? 'text-fg-subtle line-through decoration-fg-subtle' : ''
        "
        @dblclick="emit('edit')"
      >
        {{ item.text }}
      </span>

      <span
        v-if="item.linkedSulco"
        class="inline-flex items-center gap-1.5 text-xs text-accent cursor-pointer before:content-['◆'] before:text-[6px]"
        @click="emit('link-click')"
      >
        {{ item.linkedSulco.text }}
      </span>
      <span
        v-else
        class="text-xs text-fg-subtle cursor-pointer"
        @click="emit('link-click')"
      >
        vincular ao sulco →
      </span>
    </div>

    <div class="flex items-center gap-0.5 shrink-0">
      <button
        class="bg-transparent border-none text-fg-subtle cursor-pointer text-xs px-1.5 py-0.5 opacity-0 group-hover:opacity-100 transition-opacity duration-150 font-body whitespace-nowrap"
        @click="emit('defer')"
      >
        → inbox
      </button>
      <button
        class="bg-transparent border-none text-fg-subtle cursor-pointer text-[16px] px-1 py-0.5 opacity-0 group-hover:opacity-100 transition-opacity duration-150"
        @click="emit('remove')"
      >
        ×
      </button>
    </div>
  </div>
</template>
