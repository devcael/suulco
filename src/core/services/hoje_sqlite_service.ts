import { invoke } from "@tauri-apps/api/core";
import type { IHojeService } from "./hoje_service";
import type { HojeItem, HojeSummary } from "../models";

export class HojeSqliteService implements IHojeService {
  async getHojeItems(): Promise<HojeItem[]> {
    return invoke<HojeItem[]>("get_hoje_items");
  }

  async getHojeSummary(): Promise<HojeSummary> {
    return invoke<HojeSummary>("get_hoje_summary");
  }

  async toggleTaskStatus(id: number, isDone: boolean): Promise<void> {
    return invoke<void>("toggle_task_status", { id, isDone });
  }

  async deleteTask(id: number): Promise<void> {
    return invoke<void>("delete_task", { id });
  }

  async linkTaskToSulco(taskId: number, sulcoId: number | null): Promise<void> {
    return invoke<void>("link_task_to_sulco", { taskId, sulcoId });
  }

  async updateTaskText(id: number, text: string): Promise<void> {
    return invoke<void>("update_task_text", { id, text });
  }

  async deferTaskToInbox(id: number): Promise<void> {
    return invoke<void>("defer_task_to_inbox", { id });
  }

  async createTaskLinkedToSulco(text: string, sulcoId: number): Promise<void> {
    return invoke<void>("create_task_linked_to_sulco", { text, sulcoId });
  }
}
