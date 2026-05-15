# CLI
The CLI exposes three main commands: `schema`, `live`, and `fresh`.

```admonish note
This is only an overview of the CLI. You should read the help message for more complete information (`cornucopia --help`)
```

## Generating code
The code generation can be made either against a database that you manage or by letting Cornucopia manage an ephemeral database container for you.

### `schema`: Automatic container management
The `cornucopia schema` command creates a new container, loads your schema(s), generates your queries and cleanups the container. You will need to provide the path to one or more schema files to build your queries against. This requires `docker` or `podman` to be installed.

### `live`: Manual database management
If you want to manage the database yourself, use the `cornucopia live` command to connect to an arbitrary live database. You will need to provide the connection URL.

### `fresh`: Temporary database on existing server
The `cornucopia fresh` command provides a middle-ground approach between `schema` and `live`. It connects to an existing PostgreSQL server, creates a temporary database, loads your schema files, generates your queries, and then drops the temporary database. This is useful when you have an existing PostgreSQL server but want the convenience of automatic schema loading without managing containers.

## Example Usage

Here are some examples of using the different commands:

```bash
# Using schema command with container management
cornucopia schema schema.sql

# Using live command with existing database
cornucopia live postgresql://user:pass@localhost/mydb

# Using fresh command with existing server
cornucopia fresh schema.sql --url postgresql://user:pass@localhost

# Using fresh command with custom database name and search path
cornucopia fresh schema.sql --url postgresql://user:pass@localhost \
  --db-name my_temp_db \
  --search-path public,custom_schema
```

## Useful flags
### `sync`
By default, Cornucopia will generate asynchronous code, but it can also generate synchronous code using the `--sync` flag.

### `podman`
You can use `podman` as a container manager by passing the `-p` or `--podman` flag.
