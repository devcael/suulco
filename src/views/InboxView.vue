<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";
import type { InputDestination } from "../core/services/input_service";
import { useInbox } from "../composables/useInbox";
import InputBar from "../components/InputBar.vue";
import InboxTaskCard from "../components/cards/InboxTaskCard.vue";
import LinkModal from "../components/modals/LinkModal.vue";

const router = useRouter();
const route = useRoute();
const {
  items,
  filters,
  activeFilter,
  linkModalTaskId,
  editTarget,
  load,
  moveToHoje,
  onEditSave,
} = useInbox();

function onSubmitted(dest: InputDestination) {
  if (dest === route.path.slice(1)) {
    load();
  } else {
    router.push(`/${dest}`);
  }
}

</script>

<template>
  <div class="flex flex-col min-h-full p-4">
    <div class="flex-1">
      <Tabs />
      <div class="flex gap-1.5 pt-2 pb-2.5 flex-wrap" />
      <div class="flex gap-1.5 pt-[18px] pb-2.5 flex-wrap">
        <button
          class="text-xs px-2.5 py-0.5 rounded-sm border font-body transition-all duration-150 max-w-[160px] overflow-hidden text-ellipsis whitespace-nowrap cursor-pointer"
          :class="
            activeFilter === 'todos'
              ? 'bg-interactive text-interactive-fg border-interactive'
              : 'bg-transparent text-fg-subtle border-line'
          "
          @click="activeFilter = 'todos'"
        >
          todos
        </button>
        <button
          class="text-xs px-2.5 py-0.5 rounded-sm border font-body transition-all duration-150 max-w-[160px] overflow-hidden text-ellipsis whitespace-nowrap cursor-pointer"
          :class="
            activeFilter === 'sem-sulco'
              ? 'bg-interactive text-interactive-fg border-interactive'
              : 'bg-transparent text-fg-subtle border-line'
          "
          @click="activeFilter = 'sem-sulco'"
        >
          sem sulco
        </button>
        <button
          v-for="f in filters"
          :key="f.id"
          class="text-xs px-2.5 py-0.5 rounded-sm border font-body transition-all duration-150 max-w-[160px] overflow-hidden text-ellipsis whitespace-nowrap cursor-pointer"
          :class="
            activeFilter === f.id
              ? 'bg-accent border-accent text-interactive-fg'
              : 'bg-transparent text-fg-subtle border-line'
          "
          @click="activeFilter = f.id"
        >
          {{ f.text.length > 22 ? f.text.slice(0, 22) + "…" : f.text }}
        </button>
      </div>

      <div
        v-if="items.length === 0"
        class="flex flex-col items-center justify-center text-center pt-20 pb-6 gap-2"
      >
        <p class="font-display text-2xl text-fg">
          inbox limpo.
        </p>
        <p class="font-body text-sm text-fg-subtle">
          nenhuma task esperando por aqui.
        </p>
      </div>

      <InboxTaskCard
        v-for="item in items"
        :key="item.id"
        :item="item"
        @move-to-hoje="moveToHoje(item.id)"
        @link-click="linkModalTaskId = item.id"
        @edit="editTarget = { id: item.id, text: item.text }"
      />
    </div>

    <InputBar
      initial-target="inbox"
      @submitted="onSubmitted"
    />
  </div>

  <LinkModal
    v-if="linkModalTaskId !== null"
    :task-id="linkModalTaskId"
    @close="linkModalTaskId = null"
    @linked="
      linkModalTaskId = null;
      load();
    "
  />

  <EditModal
    v-if="editTarget !== null"
    :current-text="editTarget.text"
    title="editar task"
    @close="editTarget = null"
    @save="onEditSave"
  />
</template>
