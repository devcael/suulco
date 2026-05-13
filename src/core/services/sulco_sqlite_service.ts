import { invoke } from "@tauri-apps/api/core";
import type { ISulcoService } from "./sulco_service";
import type {
  SulcoListItem,
  SulcoDetail,
  ArchivedSulcoItem,
  SulcoLinkOption,
} from "../models";

export class SulcoSqliteService implements ISulcoService {
  async getActiveSulcoItems(): Promise<SulcoListItem[]> {
    return invoke<SulcoListItem[]>("get_active_sulco_items");
  }

  async getSulcoDetail(id: number): Promise<SulcoDetail> {
    return invoke<SulcoDetail>("get_sulco_detail", { id });
  }

  async updateSulcoText(id: number, text: string): Promise<void> {
    return invoke<void>("update_sulco_text", { id, text });
  }

  async updateSulcoDefinition(id: number, definition: string): Promise<void> {
    return invoke<void>("update_sulco_definition", { id, definition });
  }

  async archiveSulcoItem(id: number): Promise<void> {
    return invoke<void>("archive_sulco_item", { id });
  }

  async getArchivedSulcoItems(): Promise<ArchivedSulcoItem[]> {
    const rawItems = await invoke<{ id: number; text: string; archivedAt: string }[]>(
      "get_archived_sulco_items"
    );
    return rawItems.map((item) => ({
      id: item.id,
      text: item.text,
      archivedAt: new Date(item.archivedAt),
    }));
  }

  async restoreSulcoItem(id: number): Promise<void> {
    return invoke<void>("restore_sulco_item", { id });
  }

  async getSulcoLinkOptions(taskId: number): Promise<SulcoLinkOption[]> {
    return invoke<SulcoLinkOption[]>("get_sulco_link_options", { taskId });
  }
}
