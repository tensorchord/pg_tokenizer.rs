# Installation

## Docker

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

## From Debian package

> Installation from the Debian package requires a dependency on `GLIBC >= 2.35`, e.g:
> - `Ubuntu 22.04` or later
> - `Debian Bullseye` or later

Debian packages(.deb) are used in distributions based on Debian, such as Ubuntu and many others. They can be easily installed by `dpkg` or `apt-get`.

1. Download the deb package in [the release page](https://github.com/tensorchord/pg_tokenizer.rs/releases/latest), and type `sudo apt install postgresql-17-pg-tokenizer_*.deb` to install the deb package.

2. Configure your PostgreSQL by modifying the `shared_preload_libraries` and `search_path` to include the extension.

```sh
psql -U postgres -c 'ALTER SYSTEM SET shared_preload_libraries = "pg_tokenizer.so"'
psql -U postgres -c 'ALTER SYSTEM SET search_path TO "$user", public, tokenizer_catalog'
# You need restart the PostgreSQL cluster to take effects.
sudo systemctl restart postgresql.service   # for pg_tokenizer running with systemd
```

3. Connect to the database and enable the extension.

```sql
DROP EXTENSION IF EXISTS pg_tokenizer;
CREATE EXTENSION pg_tokenizer CASCADE;
```

## From ZIP package

> Installation from the ZIP package requires a dependency on `GLIBC >= 2.35`, e.g:
> - `RHEL 9` or later

For systems that are not Debian based and cannot run a Docker container, please follow these steps to install:

1. Before install, make sure that you have the necessary packages installed, including `PostgreSQL`, `pg_config`, `unzip`, `wget`.

```sh
# Example for RHEL 9 dnf
# Please check your package manager
sudo dnf install -y unzip wget libpq-devel
sudo dnf module install -y postgresql:15/server
sudo postgresql-setup --initdb
sudo systemctl start postgresql.service
sudo systemctl enable postgresql.service
```

2. Verify whether `$pkglibdir` and `$shardir` have been set by PostgreSQL. 

```sh
pg_config --pkglibdir
# Print something similar to:
# /usr/lib/postgresql/15/lib or
# /usr/lib64/pgsql

pg_config --sharedir
# Print something similar to:
# /usr/share/postgresql/15 or
# /usr/share/pgsql
```

3. Download the zip package in [the release page](https://github.com/tensorchord/pg_tokenizer.rs/releases/latest) and extract it to a temporary directory.

```sh
wget https://github.com/tensorchord/pg_tokenizer.rs/releases/download/0.1.0/postgresql-17-pg-tokenizer_*_x86_64-linux-gnu.zip -O pg_tokenizer.zip
unzip pg_tokenizer.zip -d pg_tokenizer
```

4. Copy the extension files to the PostgreSQL directory.

```sh
# Copy library to `$pkglibdir`
sudo cp pg_tokenizer/pg_tokenizer.so $(pg_config --pkglibdir)/
# Copy schema to `$shardir`
sudo cp pg_tokenizer/pg_tokenizer--*.sql $(pg_config --sharedir)/extension/
sudo cp pg_tokenizer/pg_tokenizer.control $(pg_config --sharedir)/extension/
```

5. Configure your PostgreSQL by modifying the `shared_preload_libraries` and `search_path` to include the extension.

```sh
psql -U postgres -c 'ALTER SYSTEM SET shared_preload_libraries = "pg_tokenizer.so"'
psql -U postgres -c 'ALTER SYSTEM SET search_path TO "$user", public, tokenizer_catalog'
# You need restart the PostgreSQL cluster to take effects.
sudo systemctl restart postgresql.service   # for pg_tokenizer running with systemd
```

6. Connect to the database and enable the extension.

```sql
DROP EXTENSION IF EXISTS pg_tokenizer;
CREATE EXTENSION pg_tokenizer CASCADE;
```


## From Source

Before building from source, you could refer to the [development](02-development.md) guide to set up the development environment.

1. Build and install the extension.

```sh
cargo pgrx install --sudo --release
```

2. Configure your PostgreSQL by modifying the `shared_preload_libraries` and `search_path` to include the extension.

```sh
psql -U postgres -c 'ALTER SYSTEM SET shared_preload_libraries = "pg_tokenizer.so"'
psql -U postgres -c 'ALTER SYSTEM SET search_path TO "$user", public, tokenizer_catalog'
# You need restart the PostgreSQL cluster to take effects.
sudo systemctl restart postgresql.service   # for pg_tokenizer running with systemd
```

3. Connect to the database and enable the extension.

```sql
CREATE EXTENSION pg_tokenizer;
```
