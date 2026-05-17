import { ref, inject, onMounted, watch } from "vue";
import type { IInboxService } from "../core/services/inbox_service";
import type { IHojeService } from "../core/services/hoje_service";
import type { InboxItem, InboxSulcoFilter } from "../core/models";

export function useInbox() {
  const inboxService = inject<IInboxService>("inboxService")!;
  const hojeService = inject<IHojeService>("hojeService")!;

  const items = ref<InboxItem[]>([]);
  const filters = ref<InboxSulcoFilter[]>([]);
  const activeFilter = ref<"todos" | "sem-sulco" | number>("todos");
  const linkModalTaskId = ref<number | null>(null);
  const editTarget = ref<{ id: number; text: string } | null>(null);

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

  async function onEditSave(newText: string) {
    if (!editTarget.value) return;
    await hojeService.updateTaskText(editTarget.value.id, newText);
    editTarget.value = null;
    await loadItems();
  }

  watch(activeFilter, loadItems);
  onMounted(load);

  return {
    items,
    filters,
    activeFilter,
    linkModalTaskId,
    editTarget,
    load,
    moveToHoje,
    onEditSave,
  };
}
