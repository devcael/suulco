import { ref, inject, onMounted, computed } from "vue";
import type { IHojeService } from "../core/services/hoje_service";
import type { HojeItem, HojeSummary } from "../core/models";

export function useHoje() {
  const hojeService = inject<IHojeService>("hojeService")!;

  const items = ref<HojeItem[]>([]);
  const summary = ref<HojeSummary>({ total: 0, pending: 0, done: 0 });
  const linkModalTaskId = ref<number | null>(null);
  const editTarget = ref<{ id: number; text: string } | null>(null);

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

  async function onEditSave(newText: string) {
    if (!editTarget.value) return;
    await hojeService.updateTaskText(editTarget.value.id, newText);
    editTarget.value = null;
    await load();
  }

  onMounted(load);

  return {
    items,
    summary,
    summaryText,
    linkModalTaskId,
    editTarget,
    load,
    toggle,
    remove,
    defer,
    onEditSave,
  };
}
