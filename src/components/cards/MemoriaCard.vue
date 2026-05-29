<script setup lang="ts">
import type { MemoriaItem } from "../../core/models";
import { formatRelativeDate } from "../../lib/formatters";

defineProps<{ item: MemoriaItem }>();

const emit = defineEmits<{
  (e: "keep"): void;
  (e: "archive"): void;
  (e: "categorize"): void;
  (e: "edit"): void;
}>();
</script>

<template>
  <div class="py-5 border-b border-line last:border-b-0">
    <div class="mb-2.5">
      <div
        class="font-body text-lg text-fg-secondary cursor-text text-ellipsis"
        @dblclick="emit('edit')"
      >
        {{ item.title }}
      </div>
    </div>
    <div class="flex items-center justify-between flex-wrap gap-2">
      <span class="text-xs text-fg-subtle">
        {{ formatRelativeDate(item.createdAt) }}
      </span>
      <div class="flex gap-1.5 items-center flex-wrap">
        <button
          class="text-xs px-2 py-0.5 rounded-sm border border-line-strong text-accent bg-transparent cursor-pointer font-body transition-all duration-150 hover:bg-bg-muted"
          @click="emit('categorize')"
        >
          {{ item.category || "categorizar" }}
        </button>
        <button
          class="text-sm px-2.5 py-1 rounded-sm border border-line-strong bg-transparent text-fg-secondary cursor-pointer font-body transition-all duration-150 hover:border-fg-secondary hover:text-fg"
          @click="emit('keep')"
        >
          ainda quero isso
        </button>
        <button
          class="text-sm px-2.5 py-1 rounded-sm border border-line bg-transparent text-fg-subtle cursor-pointer font-body transition-all duration-150 hover:border-fg-subtle hover:text-fg-secondary"
          @click="emit('archive')"
        >
          deixar ir
        </button>
      </div>
    </div>
  </div>
</template>
