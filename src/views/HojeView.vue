<script setup lang="ts">
import { ref, inject, onMounted, computed } from "vue";
import type { IHojeService } from "../core/services/hoje_service";
import type { HojeItem, HojeSummary } from "../core/models";

const hojeService = inject<IHojeService>("hojeService")!;

const items = ref<HojeItem[]>([]);

const summary = ref<HojeSummary>({ total: 0, pending: 0, done: 0 });

const emit = defineEmits<{
  (e: "open-link-modal", taskId: number): void;
  (e: "edit-task", id: number, text: string): void;
}>();

const summaryText = computed(() => {
  if (summary.value.total === 0) return "nenhuma intenção para hoje";
  if (summary.value.pending === 0) return "tudo feito";
  return `${summary.value.pending} ${summary.value.pending === 1 ? "pendente" : "pendentes"}`;
});

async function load() {
  items.value = await hojeService.getHojeItems();
  summary.value = await hojeService.getHojeSummary();
}

async function toggle(id: number, currentDone: boolean) {
  await hojeService.toggleTaskStatus(id, !currentDone);
  await load();
}

async function remove(id: number) {
  await hojeService.deleteTask(id);
  await load();
}

async function defer(id: number) {
  await hojeService.deferTaskToInbox(id);
  await load();
}

defineExpose({ load });

onMounted(load);
</script>

<template>
  <div>
    <div class="flex items-baseline gap-3 pt-[22px] pb-1">
      <span class="font-display text-[13px] text-fg-subtle tracking-tight">{{ summaryText }}</span>
    </div>

    <p v-if="items.length === 0" class="font-display italic text-xl text-fg-subtle leading-loose pt-14 pb-6">
      dia sem forma ainda —<br />o que você quer que aconteça?
    </p>

    <div
      v-for="item in items"
      :key="item.id"
      class="flex items-start gap-4 py-[18px] border-b border-line group"
    >
      <div
        class="w-[17px] h-[17px] border rounded-sm shrink-0 mt-1 cursor-pointer flex items-center justify-center transition-all duration-150 rotate-[1deg]"
        :class="item.isDone ? 'bg-positive border-positive -rotate-[0.5deg]' : 'bg-transparent border-fg-subtle'"
        @click="toggle(item.id, item.isDone)"
      >
        <svg v-if="item.isDone" width="9" height="9" viewBox="0 0 9 9" fill="none">
          <path d="M1.5 4.5l2 2 4-4" stroke="white" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>

      <div class="flex-1 flex flex-col gap-1.5">
        <span
          class="font-display text-2xl text-fg leading-[1.45] transition-colors duration-200 cursor-text"
          :class="item.isDone ? 'text-fg-subtle line-through decoration-fg-subtle' : ''"
          @dblclick="emit('edit-task', item.id, item.text)"
        >
          {{ item.text }}
        </span>

        <span
          v-if="item.linkedSulco"
          class="inline-flex items-center gap-1.5 text-xs text-accent tracking-wide cursor-pointer before:content-['◆'] before:text-[6px]"
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

      <div class="flex items-center gap-0.5 shrink-0">
        <button
          class="bg-transparent border-none text-fg-subtle cursor-pointer text-xs px-1.5 py-0.5 leading-none opacity-0 group-hover:opacity-100 transition-opacity duration-150 font-body tracking-wide whitespace-nowrap"
          @click="defer(item.id)"
        >
          → inbox
        </button>
        <button
          class="bg-transparent border-none text-fg-subtle cursor-pointer text-[16px] px-1 py-0.5 leading-none opacity-0 group-hover:opacity-100 transition-opacity duration-150"
          @click="remove(item.id)"
        >
          ×
        </button>
      </div>
    </div>
  </div>
</template>
