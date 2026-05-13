CREATE TABLE hoje_items_new (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  text        TEXT    NOT NULL,
  done        INTEGER DEFAULT 0,
  date        TEXT    DEFAULT NULL,
  carried     INTEGER DEFAULT 0,
  sulco_id    INTEGER DEFAULT NULL REFERENCES sulco_items(id) ON DELETE SET NULL,
  created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO hoje_items_new SELECT id, text, done, date, carried, sulco_id, created_at FROM hoje_items;

DROP TABLE hoje_items;

ALTER TABLE hoje_items_new RENAME TO hoje_items;

CREATE INDEX IF NOT EXISTS idx_hoje_date     ON hoje_items(date);
CREATE INDEX IF NOT EXISTS idx_hoje_sulco_id ON hoje_items(sulco_id);
