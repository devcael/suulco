CREATE TABLE IF NOT EXISTS sulco_items (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  text        TEXT    NOT NULL,
  definition  TEXT    DEFAULT '',
  position    INTEGER DEFAULT 0,
  archived    INTEGER DEFAULT 0,
  archived_at TEXT    DEFAULT NULL,
  created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS hoje_items (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  text        TEXT    NOT NULL,
  done        INTEGER DEFAULT 0,
  date        TEXT    NOT NULL DEFAULT (date('now')),
  carried     INTEGER DEFAULT 0,
  sulco_id    INTEGER DEFAULT NULL REFERENCES sulco_items(id) ON DELETE SET NULL,
  created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_hoje_date     ON hoje_items(date);
CREATE INDEX IF NOT EXISTS idx_hoje_sulco_id ON hoje_items(sulco_id);

CREATE TABLE IF NOT EXISTS memoria_items (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  text          TEXT    NOT NULL,
  category      TEXT    DEFAULT NULL,
  archived      INTEGER DEFAULT 0,
  resurfaced_at TEXT    DEFAULT NULL,
  created_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_memoria_category ON memoria_items(category);
CREATE INDEX IF NOT EXISTS idx_memoria_archived ON memoria_items(archived);

CREATE TABLE IF NOT EXISTS memoria_categories (
  id    INTEGER PRIMARY KEY AUTOINCREMENT,
  name  TEXT    NOT NULL UNIQUE
);
