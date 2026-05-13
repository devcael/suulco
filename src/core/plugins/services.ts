import type { App, Plugin } from "vue";
import { SulcoSqliteService } from "../services/sulco_sqlite_service";
import { HojeSqliteService } from "../services/hoje_sqlite_service";
import { InboxSqliteService } from "../services/inbox_sqlite_service";
import { MemoriaSqliteService } from "../services/memoria_sqlite_service";
import { InputSqliteService } from "../services/input_sqlite_service";

export const ServicesPlugin: Plugin = {
  install(app: App) {
    app.provide("sulcoService", new SulcoSqliteService());
    app.provide("hojeService", new HojeSqliteService());
    app.provide("inboxService", new InboxSqliteService());
    app.provide("memoriaService", new MemoriaSqliteService());
    app.provide("inputService", new InputSqliteService());
  },
};
