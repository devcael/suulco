import {
  SulcoListItem,
  SulcoDetail,
  ArchivedSulcoItem,
  SulcoLinkOption,
} from "../models";

export interface ISulcoService {
  getActiveSulcoItems(): Promise<SulcoListItem[]>;
  getSulcoDetail(id: number): Promise<SulcoDetail>;
  updateSulcoText(id: number, text: string): Promise<void>;
  updateSulcoDefinition(id: number, definition: string): Promise<void>;
  archiveSulcoItem(id: number): Promise<void>;
  getArchivedSulcoItems(): Promise<ArchivedSulcoItem[]>;
  restoreSulcoItem(id: number): Promise<void>;
  getSulcoLinkOptions(taskId: number): Promise<SulcoLinkOption[]>;
}
