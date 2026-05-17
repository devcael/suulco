<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";
import type { InputDestination } from "../core/services/input_service";
import { useSulco } from "../composables/useSulco";
import InputBar from "../components/InputBar.vue";
import SulcoCard from "../components/cards/SulcoCard.vue";
import ArchiveModal from "../components/modals/ArchiveModal.vue";
import EditModal from "../components/modals/EditModal.vue";

const router = useRouter();
const route = useRoute();
const {
  items,
  definitions,
  archivedItems,
  openId,
  showArchived,
  taskInputs,
  archiveTarget,
  editTarget,
  load,
  toggleOpen,
  saveDef,
  restore,
  addLinkedTask,
  onEditSave,
} = useSulco();

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
    <div class="flex-1 pt-1.5 overflow-x-hidden">
      <p
        v-if="items.length === 0"
        class="font-display italic text-fg-subtle text-xl py-14 pb-6"
      >
        nenhuma intenção ativa.
      </p>

      <SulcoCard
        v-for="item in items"
        :key="item.id"
        :item="item"
        :definition="definitions[item.id] ?? ''"
        :task-input="taskInputs[item.id] ?? ''"
        :is-open="openId === item.id"
        @toggle-open="toggleOpen(item.id)"
        @update:definition="definitions[item.id] = $event"
        @save-def="saveDef(item.id)"
        @update:task-input="taskInputs[item.id] = $event"
        @add-linked-task="addLinkedTask(item.id)"
        @archive="archiveTarget = { id: item.id, text: item.text }"
        @edit="editTarget = { id: item.id, text: item.text }"
      />

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
          <span class="text-sm text-fg-subtle">
            {{ archivedItems.length }} arquivado{{
              archivedItems.length > 1 ? "s" : ""
            }}
          </span>
        </div>

        <div v-if="showArchived">
          <div
            v-for="item in archivedItems"
            :key="item.id"
            class="py-3.5 border-b border-line last:border-b-0 flex items-center justify-between gap-3"
          >
            <span class="font-display text-xl text-fg-subtle flex-1">{{
              item.text
            }}</span>
            <button
              class="text-xs text-accent bg-transparent border-none cursor-pointer font-body"
              @click="restore(item.id)"
            >
              restaurar
            </button>
          </div>
        </div>
      </template>
    </div>

    <InputBar initial-target="sulco" @submitted="onSubmitted" />
  </div>

  <ArchiveModal
    v-if="archiveTarget !== null"
    :sulco-id="archiveTarget.id"
    :sulco-text="archiveTarget.text"
    @close="archiveTarget = null"
    @archived="
      archiveTarget = null;
      load();
    "
  />

  <EditModal
    v-if="editTarget !== null"
    :current-text="editTarget.text"
    title="editar sulco"
    @close="editTarget = null"
    @save="onEditSave"
  />
</template>
