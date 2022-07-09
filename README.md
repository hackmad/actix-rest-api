# Rest API with Rocket + Diesel

```bash
brew install libpq
brew link --force libpq

export LDFLAGS="-L/usr/local/opt/libpq/lib"
export CPPFLAGS="-I/usr/local/opt/libpq/include"
export PKG_CONFIG_PATH="/usr/local/opt/libpq/lib/pkgconfig"

cargo install diesel_cli --no-default-features --features postgres
```

If using `asdf`, the binary might be in a location not in your path. Watch out
for this at the end of the install.

```bash
Installing /Users/ahmad/.cargo/bin/diesel
Installed package `diesel_cli v1.4.1` (executable `diesel`)
```

Start postgres container:

```bash
docker-compose up -d postgres
```

Setup migrations:

```bash
export DATABASE_URL=postgres://postgres:example@localhost:5432/myapp
diesel setup
```

Generate a new migration:

```bash
diesel migration generate users
```

Run migrations:

```bash
diesel migration run
```

Revert migration:

```bash
diesel migration revert
```

Redo migration:

```bash
diesel migration redo
```

Build and run locally:

```bash
cargo build
cargo run
```

Build container:

```bash
docker-compose build
```

Run container:

```bash
docker-compose up -d rest-api
```

Example cURL commands:

```bash
curl -X GET 'http://localhost:8000/api/v1/users'
```

```bash
curl -X POST 'http://localhost:8000/api/v1/users' \
    -d '{"username": "ahmad", "password": "password", "first_name": "Ahmad"}' \
    -H 'Content-Type: application/json'
```

```bash
curl -X GET 'http://localhost:8000/api/v1/users/ahmad'
```

```bash
curl -X POST 'http://localhost:8000/api/v1/login' \
    -d '{"username": "ahmad", "password": "password"}' \
    -H 'Content-Type: application/json'
```

Stop all containers (all database changes will be lost):

```bash
docker-compose down -v
```