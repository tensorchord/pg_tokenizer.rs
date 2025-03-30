# pg_tokenizer

A PostgreSQL extension that provides tokenizers for full-text search.

## Quick Start
The official `ghcr.io/tensorchord/vchord_bm25-postgres` Docker image comes pre-configured with several complementary extensions:
- `pg_tokenizer` - This extension
- [`VectorChord-bm25`](https://github.com/tensorchord/VectorChord-bm25) - Native BM25 Ranking Index
- [`VectorChord`](https://github.com/tensorchord/VectorChord) - Scalable, high-performance, and disk-efficient vector similarity search
- [`pgvector`](https://github.com/pgvector/pgvector) - Popular vector similarity search

Simply run the Docker container as shown below:

```bash
docker run \
  --name vectorchord-demo \
  -e POSTGRES_PASSWORD=mysecretpassword \
  -p 5432:5432 \
  -d ghcr.io/tensorchord/vchord_bm25-postgres:pg17-v0.2.0
```

Once everything’s set up, you can connect to the database using the `psql` command line tool. The default username is `postgres`, and the default password is `mysecretpassword`. Here’s how to connect:

```sh
psql -h localhost -p 5432 -U postgres
```

After connecting, run the following SQL to make sure the extension is enabled:

```sql
CREATE EXTENSION pg_tokenizer;
```

Then, don’t forget to add `tokenizer_catalog` to your `search_path`:

```sql
ALTER SYSTEM SET search_path TO "$user", public, tokenizer_catalog;
SELECT pg_reload_conf();
```

## Example

```sql
SELECT create_tokenizer('tokenizer1', $$
model = "llmlingua2"
$$);

SELECT tokenize('PostgreSQL is a powerful, open-source object-relational database system. It has over 15 years of active development.', 'tokenizer1');
```

More examples can be found in [docs/03-examples.md](docs/03-examples.md).

## Documentation

- [Installation](docs/01-installation.md)
- [Development](docs/02-development.md)
- [Examples](docs/03-examples.md)
- [Usage](docs/04-usage.md)
- [Text Analyzer](docs/05-text-analyzer.md)
- [Model](docs/06-model.md)
- [Limitation](docs/07-limitation.md)
- [Reference](docs/00-reference.md)
