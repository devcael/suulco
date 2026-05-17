import { ref, inject, onMounted } from "vue";
import type { ISulcoService } from "../core/services/sulco_service";
import type { IHojeService } from "../core/services/hoje_service";
import type { SulcoListItem, ArchivedSulcoItem } from "../core/models";

export function useSulco() {
  const sulcoService = inject<ISulcoService>("sulcoService")!;
  const hojeService = inject<IHojeService>("hojeService")!;

  const items = ref<SulcoListItem[]>([]);
  const definitions = ref<Record<number, string>>({});
  const archivedItems = ref<ArchivedSulcoItem[]>([]);
  const openId = ref<number | null>(null);
  const showArchived = ref(false);
  const taskInputs = ref<Record<number, string>>({});
  const archiveTarget = ref<{ id: number; text: string } | null>(null);
  const editTarget = ref<{ id: number; text: string } | null>(null);

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

  async function onEditSave(newText: string) {
    if (!editTarget.value) return;
    await sulcoService.updateSulcoText(editTarget.value.id, newText);
    editTarget.value = null;
    await load();
  }

  onMounted(load);

  return {
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
  };
}
