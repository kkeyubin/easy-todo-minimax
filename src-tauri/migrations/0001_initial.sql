-- 0001_initial.sql
-- 便签主表。type 字段在 SQL 里是保留字（部分场景），保留。
CREATE TABLE IF NOT EXISTS stickies (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  type          TEXT    NOT NULL,
  title         TEXT    NOT NULL DEFAULT '',
  color         TEXT    NOT NULL DEFAULT 'yellow',
  x             INTEGER NOT NULL,
  y             INTEGER NOT NULL,
  width         INTEGER NOT NULL DEFAULT 280,
  height        INTEGER NOT NULL DEFAULT 280,
  font_size     INTEGER NOT NULL DEFAULT 14,
  pinned        INTEGER NOT NULL DEFAULT 0,
  z_order       INTEGER NOT NULL DEFAULT 0,
  content       TEXT    NOT NULL DEFAULT '',
  image_path    TEXT,
  link_url      TEXT,
  created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
  updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_stickies_z_order ON stickies(z_order DESC);
CREATE INDEX IF NOT EXISTS idx_stickies_type    ON stickies(type);

CREATE TABLE IF NOT EXISTS todos (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  sticky_id   INTEGER NOT NULL,
  text        TEXT    NOT NULL,
  done        INTEGER NOT NULL DEFAULT 0,
  sort_order  INTEGER NOT NULL DEFAULT 0,
  created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
  FOREIGN KEY (sticky_id) REFERENCES stickies(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_todos_sticky_id ON todos(sticky_id);
CREATE INDEX IF NOT EXISTS idx_todos_sort      ON todos(sticky_id, sort_order);
