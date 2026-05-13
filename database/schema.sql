-- -------------------------------------------------------------
-- sulco_items
-- Intenções de longo prazo. Cada uma pode ter uma definição
-- livre que explica o que aquela intenção significa de verdade.
-- -------------------------------------------------------------
CREATE TABLE sulco_items (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  text        TEXT    NOT NULL,
  definition  TEXT    DEFAULT '',
  position    INTEGER DEFAULT 0,        -- ordem manual na lista
  archived    INTEGER DEFAULT 0,        -- 0 = ativo, 1 = arquivado
  archived_at TEXT    DEFAULT NULL,     -- ISO 8601
  created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);
 
 
-- -------------------------------------------------------------
-- hoje_items
-- Tasks do dia. Pertencem a uma data específica.
-- Podem ser linkadas a um item do Sulco.
-- -------------------------------------------------------------
CREATE TABLE hoje_items (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  text        TEXT    NOT NULL,
  done        INTEGER DEFAULT 0,        -- 0 = pendente, 1 = feito
  date        TEXT    NOT NULL DEFAULT (date('now')),  -- YYYY-MM-DD
  carried     INTEGER DEFAULT 0,        -- foi carregada do dia anterior?
  sulco_id    INTEGER DEFAULT NULL REFERENCES sulco_items(id) ON DELETE SET NULL,
  created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);
 
CREATE INDEX idx_hoje_date     ON hoje_items(date);
CREATE INDEX idx_hoje_sulco_id ON hoje_items(sulco_id);
 
 
-- -------------------------------------------------------------
-- memoria_items
-- Pensamentos, ideias, referências. Sem pressão, sem prazo.
-- Ressurgem automaticamente com o tempo.
-- -------------------------------------------------------------
CREATE TABLE memoria_items (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  text          TEXT    NOT NULL,
  category      TEXT    DEFAULT NULL,   -- tag livre, criada pelo usuário
  archived      INTEGER DEFAULT 0,
  resurfaced_at TEXT    DEFAULT NULL,   -- última vez que ressurgiu (ISO 8601)
  created_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);
 
CREATE INDEX idx_memoria_category ON memoria_items(category);
CREATE INDEX idx_memoria_archived ON memoria_items(archived);
 
 
-- -------------------------------------------------------------
-- memoria_categories
-- Categorias existentes (derivadas dos itens, mas mantidas aqui
-- pra listar sem depender de GROUP BY toda vez)
-- -------------------------------------------------------------
CREATE TABLE memoria_categories (
  id    INTEGER PRIMARY KEY AUTOINCREMENT,
  name  TEXT    NOT NULL UNIQUE
);
