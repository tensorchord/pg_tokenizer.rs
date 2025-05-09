# pg_tokenizer

A PostgreSQL extension that provides tokenizers for full-text search.

## Quick Start

The official `tensorchord/vchord-suite` Docker image comes pre-configured with several complementary extensions, you can find more details in the [VectorChord-images](https://github.com/tensorchord/VectorChord-images) repository:
- `pg_tokenizer` - This extension
- [`VectorChord-bm25`](https://github.com/tensorchord/VectorChord-bm25) - Native BM25 Ranking Index
- [`VectorChord`](https://github.com/tensorchord/VectorChord) - Scalable, high-performance, and disk-efficient vector similarity search

Simply run the Docker container as shown below:

```bash
docker run   \           
  --name vchord-suite  \
  -e POSTGRES_PASSWORD=postgres  \
  -p 5432:5432 \
  -d tensorchord/vchord-suite:pg17-latest
  # If you want to use ghcr image, you can change the image to `ghcr.io/tensorchord/vchord-suite:pg17-latest`.
  # if you want to use the specific version, you can use the tag `pg17-20250414`, supported version can be found in the support matrix.
```

Once everything’s set up, you can connect to the database using the `psql` command line tool. The default username is `postgres`, and the default password is `postgres`. Here’s how to connect:

```sh
psql -h localhost -p 5432 -U postgres
```

After connecting, run the following SQL to make sure the extension is enabled:

```sql
CREATE EXTENSION pg_tokenizer;
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
