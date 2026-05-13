export interface SulcoListItem {
  id: number;
  text: string;
  isOpen: boolean;
  hasDefinition: boolean;
}

export interface SulcoDetail {
  id: number;
  text: string;
  definition: string;
}

export interface ArchivedSulcoItem {
  id: number;
  text: string;
  archivedAt: Date;
}
