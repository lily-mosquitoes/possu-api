CREATE TABLE IF NOT EXISTS categories (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO categories (
  name
) VALUES
  ("Emilia's Income"),
  ("Lílian's Income"),
  ("Yearly Bills"),
  ("Monthly Bills"),
  ("One-time Purchases"),
  ("Living Expenses"),
  ("Repeat Purchases");
