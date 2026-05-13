import { InboxItem, InboxSulcoFilter } from "../models";

export interface IInboxService {
  getAllTasks(): Promise<InboxItem[]>;
  getTasksWithoutSulco(): Promise<InboxItem[]>;
  getTasksBySulcoId(sulcoId: number): Promise<InboxItem[]>;
  getInboxSulcoFilters(): Promise<InboxSulcoFilter[]>;
  moveTaskToHoje(id: number): Promise<void>;
}
