INSERT OR IGNORE INTO entries (
  id,
  timestamp,
  category_id,
  description,
  value
) SELECT
  345,
  "2011-10-05T14:48:00.000Z",
  id,
  "Test description",
  30000
FROM categories WHERE categories.name = 'Monthly Bills';

DELETE FROM entries WHERE id = 678;
