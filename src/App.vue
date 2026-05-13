<script setup lang="ts">
import { ref, inject } from "vue";
import { useRouter, useRoute } from "vue-router";
import type { ISulcoService } from "./core/services/sulco_service";
import type { IHojeService } from "./core/services/hoje_service";
import type { IMemoriaService } from "./core/services/memoria_service";
import type { InputDestination } from "./core/services/input_service";
import InputBar from "./components/InputBar.vue";
import MemoriaEditorModal from "./components/MemoriaEditorModal.vue";
import LinkModal from "./components/modals/LinkModal.vue";
import CategoryModal from "./components/modals/CategoryModal.vue";
import ArchiveModal from "./components/modals/ArchiveModal.vue";
import EditModal from "./components/modals/EditModal.vue";

const sulcoService = inject<ISulcoService>("sulcoService")!;
const hojeService = inject<IHojeService>("hojeService")!;
const memoriaService = inject<IMemoriaService>("memoriaService")!;

const router = useRouter();
const route = useRoute();

const tabs = ["sulco", "hoje", "inbox", "memoria"] as const;

const currentView = ref<any>(null);

const memoriaEditorOpen = ref(false);
const memoriaEditItem = ref<{ id: number; text: string } | null>(null);
const linkModalTaskId = ref<number | null>(null);
const catModalItemId = ref<number | null>(null);
const archiveModalId = ref<number | null>(null);
const archiveModalText = ref("");

type EditTarget = {
  id: number;
  text: string;
  type: "sulco" | "task";
};
const editTarget = ref<EditTarget | null>(null);

async function openArchiveModal(id: number) {
  const detail = await sulcoService.getSulcoDetail(id);
  archiveModalText.value = detail.text;
  archiveModalId.value = id;
}

function reloadCurrentTab() {
  currentView.value?.load?.();
}

function onSubmitted(dest: InputDestination) {
  router.push(`/${dest}`);
  setTimeout(() => reloadCurrentTab(), 50);
}

function onMemoriaSubmitted() {
  router.push("/memoria");
  setTimeout(() => reloadCurrentTab(), 50);
}

function onLinked() {
  reloadCurrentTab();
}

function onArchived() {
  reloadCurrentTab();
}

function onCatUpdated() {
  reloadCurrentTab();
}

async function onEditSave(newText: string) {
  if (!editTarget.value) return;
  const { id, type } = editTarget.value;

  if (type === "sulco") {
    await sulcoService.updateSulcoText(id, newText);
  } else if (type === "task") {
    await hojeService.updateTaskText(id, newText);
  }
  reloadCurrentTab();
}
</script>

<template>
  <div
    class="pointer-events-none fixed inset-0 z-[999] opacity-[0.032]"
    aria-hidden="true"
    style="
      background-image: url(&quot;data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E&quot;);
      background-repeat: repeat;
      background-size: 180px 180px;
    "
  />

  <div
    class="max-w-[580px] pt-10 mx-auto min-h-screen flex flex-col bg-bg font-body text-fg relative overflow-x-hidden"
  >
    <div class="pt-spacing-page-y px-spacing-page-x">
      <span
        class="font-display text-logo font-normal tracking-tight text-fg leading-none block"
      >
        sulco
      </span>
      <span
        class="font-body text-sm font-light text-fg-subtle tracking-widest mt-1.5 block"
      >
        sua fonte da verdade
      </span>
    </div>

    <div
      class="mx-spacing-page-x mt-7 h-px bg-gradient-to-r from-line-strong via-line to-transparent"
    />

    <div
      class="flex gap-0 px-spacing-page-x mt-5 overflow-x-auto scrollbar-none"
    >
      <router-link
        v-for="t in tabs"
        :key="t"
        :to="`/${t}`"
        custom
        v-slot="{ navigate, isActive }"
      >
        <button
          class="font-body text-base font-normal tracking-wider pb-3 mr-8 border-none border-b bg-transparent cursor-pointer transition-all duration-200 whitespace-nowrap shrink-0"
          :class="
            isActive
              ? 'text-fg border-b-accent border-b'
              : 'text-fg-subtle border-b-transparent border-b'
          "
          @click="navigate"
        >
          {{ t }}
        </button>
      </router-link>
    </div>

    <div class="flex-1 px-spacing-page-x overflow-y-auto">
      <router-view v-slot="{ Component }">
        <component
          :is="Component"
          ref="currentView"
          @archive-confirm="openArchiveModal"
          @open-link-modal="(id: number) => (linkModalTaskId = id)"
          @open-cat-modal="(id: number) => (catModalItemId = id)"
          @edit-task="
            (id: number, text: string) =>
              (editTarget = { id, text, type: 'task' })
          "
          @edit-item="
            (id: number, text: string) =>
              (editTarget = { id, text, type: 'sulco' })
          "
          @open-memoria-edit="
            (id: number, text: string) =>
              (memoriaEditItem = { id, text })
          "
        />
      </router-view>
    </div>

    <InputBar @submitted="onSubmitted" @open-memoria-editor="memoriaEditorOpen = true" />
  </div>

  <MemoriaEditorModal
    v-if="memoriaEditorOpen"
    @close="memoriaEditorOpen = false"
    @submitted="onMemoriaSubmitted"
  />

  <MemoriaEditorModal
    v-if="memoriaEditItem !== null"
    :edit-item="memoriaEditItem"
    @close="memoriaEditItem = null"
    @submitted="() => { memoriaEditItem = null; reloadCurrentTab(); }"
  />

  <LinkModal
    v-if="linkModalTaskId !== null"
    :task-id="linkModalTaskId"
    @close="linkModalTaskId = null"
    @linked="onLinked"
  />

  <CategoryModal
    v-if="catModalItemId !== null"
    :item-id="catModalItemId"
    @close="catModalItemId = null"
    @updated="onCatUpdated"
  />

  <ArchiveModal
    v-if="archiveModalId !== null"
    :sulco-id="archiveModalId"
    :sulco-text="archiveModalText"
    @close="archiveModalId = null"
    @archived="onArchived"
  />

  <EditModal
    v-if="editTarget !== null"
    :current-text="editTarget.text"
    :title="editTarget.type === 'sulco' ? 'editar sulco' : 'editar task'"
    @close="editTarget = null"
    @save="onEditSave"
  />
</template>
