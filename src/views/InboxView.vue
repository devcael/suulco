<script setup lang="ts">
import { ref, inject, onMounted, watch } from "vue";
import type { IInboxService } from "../core/services/inbox_service";
import type { InboxItem, InboxSulcoFilter } from "../core/models";
import { formatRelativeDate } from "../lib/formatters";

const inboxService = inject<IInboxService>("inboxService")!;

const items = ref<InboxItem[]>([]);
const filters = ref<InboxSulcoFilter[]>([]);
const activeFilter = ref<"todos" | "sem-sulco" | number>("todos");

const emit = defineEmits<{
  (e: "open-link-modal", taskId: number): void;
  (e: "edit-task", id: number, text: string): void;
}>();

async function loadFilters() {
  filters.value = await inboxService.getInboxSulcoFilters();
}

async function loadItems() {
  if (activeFilter.value === "todos") {
    items.value = await inboxService.getAllTasks();
  } else if (activeFilter.value === "sem-sulco") {
    items.value = await inboxService.getTasksWithoutSulco();
  } else {
    items.value = await inboxService.getTasksBySulcoId(activeFilter.value as number);
  }
}

async function load() {
  await loadFilters();
  await loadItems();
}

async function moveToHoje(id: number) {
  await inboxService.moveTaskToHoje(id);
  await loadItems();
}

watch(activeFilter, loadItems);

defineExpose({ load });

onMounted(load);
</script>

<template>
  <div>
    <div class="flex gap-1.5 pt-[18px] pb-2.5 flex-wrap">
      <button
        class="text-xs px-2.5 py-0.5 rounded-sm border font-body tracking-wide transition-all duration-150 max-w-[160px] overflow-hidden text-ellipsis whitespace-nowrap cursor-pointer"
        :class="activeFilter === 'todos'
          ? 'bg-interactive text-interactive-fg border-interactive'
          : 'bg-transparent text-fg-subtle border-line'"
        @click="activeFilter = 'todos'"
      >
        todos
      </button>
      <button
        class="text-xs px-2.5 py-0.5 rounded-sm border font-body tracking-wide transition-all duration-150 max-w-[160px] overflow-hidden text-ellipsis whitespace-nowrap cursor-pointer"
        :class="activeFilter === 'sem-sulco'
          ? 'bg-interactive text-interactive-fg border-interactive'
          : 'bg-transparent text-fg-subtle border-line'"
        @click="activeFilter = 'sem-sulco'"
      >
        sem sulco
      </button>
      <button
        v-for="f in filters"
        :key="f.id"
        class="text-xs px-2.5 py-0.5 rounded-sm border font-body tracking-wide transition-all duration-150 max-w-[160px] overflow-hidden text-ellipsis whitespace-nowrap cursor-pointer"
        :class="activeFilter === f.id
          ? 'bg-accent border-accent text-interactive-fg'
          : 'bg-transparent text-fg-subtle border-line'"
        @click="activeFilter = f.id"
      >
        {{ f.text.length > 22 ? f.text.slice(0, 22) + "…" : f.text }}
      </button>
    </div>

    <p v-if="items.length === 0" class="font-display italic text-xl text-fg-subtle leading-loose pt-13 pb-6">
      nenhuma task aqui.
    </p>

    <div
      v-for="item in [...items].reverse()"
      :key="item.id"
      class="py-[18px] border-b border-line group"
    >
      <div class="flex items-start justify-between gap-3 mb-2">
        <span
          class="font-display text-[19px] text-fg leading-[1.45] flex-1 cursor-text"
          @dblclick="emit('edit-task', item.id, item.text)"
        >{{ item.text }}</span>
        <div class="flex items-center gap-2 shrink-0 mt-0.5">
          <button
            class="bg-transparent border-none text-fg-subtle cursor-pointer text-xs px-1.5 py-0.5 leading-none opacity-0 group-hover:opacity-100 transition-opacity duration-150 font-body tracking-wide whitespace-nowrap"
            @click="moveToHoje(item.id)"
          >
            → hoje
          </button>
          <span
            class="text-xs px-2 py-0.5 rounded-sm tracking-wide font-body"
            :class="item.isDone
              ? 'bg-bg-subtle text-positive border border-[rgba(107,115,87,0.3)]'
              : 'bg-bg-subtle text-fg-subtle border border-line'"
          >
            {{ item.isDone ? "feito" : "pendente" }}
          </span>
        </div>
      </div>
      <div class="flex items-center gap-2.5 flex-wrap">
        <span class="text-xs text-fg-subtle tracking-wider">
          {{ formatRelativeDate(item.createdAt) }}
        </span>
        <span
          v-if="item.linkedSulco"
          class="inline-flex items-center gap-1 text-xs text-accent tracking-wide cursor-pointer before:content-['◆'] before:text-[6px]"
          @click="emit('open-link-modal', item.id)"
        >
          {{ item.linkedSulco.text }}
        </span>
        <span
          v-else
          class="text-xs text-fg-subtle cursor-pointer tracking-wide"
          @click="emit('open-link-modal', item.id)"
        >
          vincular ao sulco →
        </span>
      </div>
    </div>
  </div>
</template>
