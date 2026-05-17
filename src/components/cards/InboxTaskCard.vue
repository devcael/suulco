<script setup lang="ts">
import type { InboxItem } from "../../core/models";
import { formatRelativeDate } from "../../lib/formatters";

defineProps<{ item: InboxItem }>();

const emit = defineEmits<{
  (e: "move-to-hoje"): void;
  (e: "link-click"): void;
  (e: "edit"): void;
}>();
</script>

<template>
  <div class="py-[18px] border-b border-line last:border-b-0 group">
    <div class="flex items-start justify-between gap-3 mb-2">
      <span
        class="font-display text-[19px] text-fg flex-1 cursor-text"
        @dblclick="emit('edit')"
        >{{ item.text }}</span
      >

      <div class="flex items-center gap-2 shrink-0 mt-0.5">
        <button
          class="bg-transparent border-none text-fg-subtle cursor-pointer text-xs px-1.5 py-0.5 opacity-0 group-hover:opacity-100 transition-opacity duration-150 font-body whitespace-nowrap"
          @click="emit('move-to-hoje')"
        >
          → hoje
        </button>
        <span
          class="text-xs px-2 py-0.5 rounded-sm font-body"
          :class="
            item.isDone
              ? 'bg-bg-subtle text-positive border border-[rgba(89,141,108,0.3)]'
              : 'bg-bg-subtle text-fg-subtle border border-line'
          "
        >
          {{ item.isDone ? "feito" : "pendente" }}
        </span>
      </div>
    </div>

    <div class="flex items-center gap-2.5 flex-wrap">
      <span class="text-xs text-fg-subtle">
        {{ formatRelativeDate(item.createdAt) }}
      </span>
      <span
        v-if="item.linkedSulco"
        class="inline-flex items-center gap-1 text-xs text-accent cursor-pointer before:content-['◆'] before:text-[6px]"
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
  </div>
</template>
