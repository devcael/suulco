<script setup lang="ts">
import { useRouter, useRoute } from "vue-router";
import type { InputDestination } from "../core/services/input_service";
import { useHoje } from "../composables/useHoje";
import InputBar from "../components/InputBar.vue";
import HojeTaskCard from "../components/cards/HojeTaskCard.vue";
import LinkModal from "../components/modals/LinkModal.vue";
import EditModal from "../components/modals/EditModal.vue";

const router = useRouter();
const route = useRoute();
const {
  items,
  summaryText,
  linkModalTaskId,
  editTarget,
  load,
  toggle,
  remove,
  defer,
  onEditSave,
} = useHoje();

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
    <div class="flex-1">
      <div class="flex items-baseline gap-3 pt-[22px] pb-1">
        <span class="font-display text-[13px] text-fg-subtle">{{
          summaryText
        }}</span>
      </div>

      <div
        v-if="items.length === 0"
        class="flex flex-col items-center justify-center text-center pt-20 pb-6 gap-2"
      >
        <p class="font-display text-2xl text-fg">o dia ainda não tem forma.</p>
        <p class="font-body text-sm text-fg-subtle">o que você quer que aconteça hoje?</p>
      </div>

      <HojeTaskCard
        v-for="item in items"
        :key="item.id"
        :item="item"
        @toggle="toggle(item.id, item.isDone)"
        @remove="remove(item.id)"
        @defer="defer(item.id)"
        @link-click="linkModalTaskId = item.id"
        @edit="editTarget = { id: item.id, text: item.text }"
      />
    </div>

    <InputBar initial-target="hoje" @submitted="onSubmitted" />
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
