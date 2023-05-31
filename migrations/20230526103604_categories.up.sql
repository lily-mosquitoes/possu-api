CREATE TABLE IF NOT EXISTS categories (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO categories (
  id, name
) VALUES
  (0, "Emilia's Income"),
  (1, "Lílian's Income"),
  (2, "Yearly Bills"),
  (3, "Monthly Bills"),
  (4, "One-time Purchases"),
  (5, "Living Expenses"),
  (6, "Repeat Purchases");
