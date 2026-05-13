import { invoke } from "@tauri-apps/api/core";
import type { IInboxService } from "./inbox_service";
import type { InboxItem, InboxSulcoFilter } from "../models";

export class InboxSqliteService implements IInboxService {
  async getAllTasks(): Promise<InboxItem[]> {
    const rawItems = await invoke<{ id: number; text: string; isDone: boolean; createdAt: string; wasCarried: boolean; linkedSulco: { id: number; text: string } | null }[]>(
      "get_all_tasks"
    );
    return rawItems.map((item) => ({
      ...item,
      createdAt: new Date(item.createdAt),
    }));
  }

  async getTasksWithoutSulco(): Promise<InboxItem[]> {
    const rawItems = await invoke<{ id: number; text: string; isDone: boolean; createdAt: string; wasCarried: boolean; linkedSulco: null }[]>(
      "get_tasks_without_sulco"
    );
    return rawItems.map((item) => ({
      ...item,
      createdAt: new Date(item.createdAt),
    }));
  }

  async getTasksBySulcoId(sulcoId: number): Promise<InboxItem[]> {
    const rawItems = await invoke<{ id: number; text: string; isDone: boolean; createdAt: string; wasCarried: boolean; linkedSulco: { id: number; text: string } | null }[]>(
      "get_tasks_by_sulco_id",
      { sulcoId }
    );
    return rawItems.map((item) => ({
      ...item,
      createdAt: new Date(item.createdAt),
    }));
  }

  async getInboxSulcoFilters(): Promise<InboxSulcoFilter[]> {
    return invoke<InboxSulcoFilter[]>("get_inbox_sulco_filters");
  }

  async moveTaskToHoje(id: number): Promise<void> {
    return invoke<void>("move_task_to_hoje", { id });
  }
}
