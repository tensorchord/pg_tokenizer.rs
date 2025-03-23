# Limitation

## object cache may not follow transaction isolation level

`pg_tokenizer` will cache `text analyzer`, `model` and `tokenizer` object in memory for each connection. The cache will always be updated when calling `create_...`, `drop_...` functions. And it doesn't follow transaction isolation level.

The cache may not be cleared when you rollback a transaction. You may need to call `drop_...` functions manually or reconnect to the database to clear the cache.

Example:

```sql
BEGIN;
SELECT create_text_analyzer('text_analyzer1', $$
pre_tokenizer = "unicode_segmentation"
$$);
-- The text analyzer is created and cached in memory
ROLLBACK;
-- The text analyzer is still cached in memory, but no effect for other connections
SELECT drop_text_analyzer('text_analyzer1');  -- extra call to clear the cache
```
