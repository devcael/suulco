export type InputDestination = "hoje" | "sulco" | "memoria" | "inbox";

export interface CreateItemOptions {
  sulcoId?: number;
  category?: string;
}

export interface IInputService {
  createGlobalItem(destination: InputDestination, text: string, options?: CreateItemOptions): Promise<void>;
}
