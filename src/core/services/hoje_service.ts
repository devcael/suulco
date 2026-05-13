import { HojeItem, HojeSummary } from "../models";

export interface IHojeService {
  getHojeItems(): Promise<HojeItem[]>;
  getHojeSummary(): Promise<HojeSummary>;
  toggleTaskStatus(id: number, isDone: boolean): Promise<void>;
  deleteTask(id: number): Promise<void>;
  linkTaskToSulco(taskId: number, sulcoId: number | null): Promise<void>;
  updateTaskText(id: number, text: string): Promise<void>;
  deferTaskToInbox(id: number): Promise<void>;
  createTaskLinkedToSulco(text: string, sulcoId: number): Promise<void>;
}
