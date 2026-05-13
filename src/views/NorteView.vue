<script setup lang="ts">
import { ref, inject, onMounted } from "vue";
import type { ISulcoService } from "../core/services/sulco_service";
import type { IHojeService } from "../core/services/hoje_service";
import type { SulcoListItem, ArchivedSulcoItem } from "../core/models";

const sulcoService = inject<ISulcoService>("sulcoService")!;
const hojeService = inject<IHojeService>("hojeService")!;

const items = ref<SulcoListItem[]>([]);
const definitions = ref<Record<number, string>>({});
const archivedItems = ref<ArchivedSulcoItem[]>([]);
const openId = ref<number | null>(null);
const showArchived = ref(false);
const taskInputs = ref<Record<number, string>>({});

const emit = defineEmits<{
  (e: "archive-confirm", id: number): void;
  (e: "edit-item", id: number, text: string): void;
}>();

async function load() {
  items.value = await sulcoService.getActiveSulcoItems();
  archivedItems.value = await sulcoService.getArchivedSulcoItems();
}

async function toggleOpen(id: number) {
  if (openId.value === id) {
    openId.value = null;
    return;
  }
  openId.value = id;
  if (definitions.value[id] === undefined) {
    const detail = await sulcoService.getSulcoDetail(id);
    definitions.value[id] = detail.definition;
  }
}

async function saveDef(id: number) {
  const def = definitions.value[id] ?? "";
  await sulcoService.updateSulcoDefinition(id, def);
}

async function restore(id: number) {
  await sulcoService.restoreSulcoItem(id);
  await load();
}

async function addLinkedTask(sulcoId: number) {
  const text = (taskInputs.value[sulcoId] ?? "").trim();
  if (!text) return;
  await hojeService.createTaskLinkedToSulco(text, sulcoId);
  taskInputs.value[sulcoId] = "";
}

defineExpose({ load });

onMounted(load);
</script>

<template>
  <div class="pt-1.5 overflow-x-hidden">
    <p
      v-if="items.length === 0"
      class="font-display italic text-fg-subtle text-xl leading-loose py-14 pb-6"
    >
      nenhuma intenção ativa.
    </p>

    <div
      v-for="item in items"
      :key="item.id"
      class="border-b border-line group"
      :class="{ open: openId === item.id }"
    >
      <div
        class="flex items-center gap-4 py-[22px] cursor-pointer select-none"
        @click="toggleOpen(item.id)"
      >
        <div
          class="w-2 h-2 border border-fg-subtle rotate-45 shrink-0 transition-all duration-200"
          :class="
            openId === item.id ? 'bg-accent border-accent' : 'bg-transparent'
          "
        />
        <span
          class="font-display text-3xl text-fg flex-1 leading-[1.4] cursor-text"
          @dblclick.stop="emit('edit-item', item.id, item.text)"
          >{{ item.text }}</span
        >
        <button
          class="bg-transparent border-none text-fg-subtle text-sm tracking-wide cursor-pointer font-body opacity-0 group-hover:opacity-100 transition-opacity duration-150 px-1 py-0.5 whitespace-nowrap"
          @click.stop="emit('archive-confirm', item.id)"
        >
          arquivar
        </button>
        <svg
          class="text-fg-subtle shrink-0 transition-transform duration-200"
          :class="openId === item.id ? 'rotate-180' : ''"
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
        :style="{ maxHeight: openId === item.id ? '420px' : '0px' }"
      >
        <div class="pt-0.5 pb-[22px] pl-6">
          <textarea
            class="w-full bg-bg-subtle border border-line rounded-md px-3.5 py-3 font-display text-[15px] text-fg leading-loose resize-none min-h-[76px] outline-none transition-[border-color] duration-200 focus:border-accent placeholder:text-fg-subtle placeholder:italic placeholder:font-body placeholder:text-[13px]"
            :value="definitions[item.id] ?? ''"
            @input="
              definitions[item.id] = (
                $event.target as HTMLTextAreaElement
              ).value
            "
            @blur="saveDef(item.id)"
            placeholder="o que isso significa pra você? como chegar lá?"
            rows="3"
          />
          <div class="flex gap-2 items-center mt-3">
            <input
              class="flex-1 bg-bg-subtle border border-line rounded-md px-3 py-2 font-body text-[13px] text-fg outline-none transition-[border-color] duration-200 focus:border-accent placeholder:text-fg-subtle placeholder:italic placeholder:text-xs"
              :value="taskInputs[item.id] ?? ''"
              @input="
                taskInputs[item.id] = ($event.target as HTMLInputElement).value
              "
              @keydown.enter="addLinkedTask(item.id)"
              placeholder="adicionar task vinculada..."
            />
            <button
              class="w-8 h-8 rounded-md bg-interactive border-none cursor-pointer flex items-center justify-center shrink-0 transition-opacity duration-150 hover:opacity-[0.78]"
              @click="addLinkedTask(item.id)"
            >
              <svg width="12" height="12" viewBox="0 0 15 15" fill="none">
                <path
                  d="M7.5 12.5V2.5M7.5 2.5L3 7M7.5 2.5L12 7"
                  stroke="#F0EAE0"
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

    <template v-if="archivedItems.length > 0">
      <div
        class="flex items-center gap-2 pt-[18px] pb-1.5 cursor-pointer"
        @click="showArchived = !showArchived"
      >
        <svg
          width="10"
          height="10"
          viewBox="0 0 10 10"
          fill="none"
          class="transition-transform duration-200"
          :style="{ transform: showArchived ? 'rotate(90deg)' : 'none' }"
        >
          <path
            d="M3 2l4 3-4 3"
            stroke="currentColor"
            stroke-width="1.2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
        <span class="text-sm tracking-wide text-fg-subtle">
          {{ archivedItems.length }} arquivado{{
            archivedItems.length > 1 ? "s" : ""
          }}
        </span>
      </div>

      <div
        v-if="showArchived"
        v-for="item in archivedItems"
        :key="item.id"
        class="py-3.5 border-b border-line flex items-center justify-between gap-3"
      >
        <span class="font-display text-xl text-fg-subtle flex-1">{{
          item.text
        }}</span>
        <button
          class="text-xs text-accent bg-transparent border-none cursor-pointer tracking-wide font-body"
          @click="restore(item.id)"
        >
          restaurar
        </button>
      </div>
    </template>
  </div>
</template>
