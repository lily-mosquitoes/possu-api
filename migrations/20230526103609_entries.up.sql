CREATE TABLE IF NOT EXISTS entries (
  id INTEGER PRIMARY KEY,
  timestamp DATETIME NOT NULL,
  category_id INTEGER NOT NULL,
  description TEXT NOT NULL,
  value INTEGER NOT NULL,
  FOREIGN KEY(category_id) REFERENCES categories(id)
    ON DELETE CASCADE
);
