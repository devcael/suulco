import { invoke } from "@tauri-apps/api/core";
import type { IInputService, InputDestination, CreateItemOptions } from "./input_service";

export class InputSqliteService implements IInputService {
  async createGlobalItem(destination: InputDestination, text: string, options?: CreateItemOptions): Promise<void> {
    return invoke<void>("create_global_item", {
      destination,
      text,
      sulcoId: options?.sulcoId ?? null,
      category: options?.category ?? null,
    });
  }
}
