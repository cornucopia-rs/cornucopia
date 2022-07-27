# cornucopia
```
Command line interface to interact with Cornucopia SQL

USAGE:
    cornucopia [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -d, --destination <DESTINATION>
            Destination folder for generated modules [default: src/cornucopia.rs]

    -h, --help
            Print help information

    -p, --podman
            Use `podman` instead of `docker`

    -q, --queries-path <QUERIES_PATH>
            Folder containing the queries [default: queries/]

        --serialize
            Derive serde's `Serialize` trait for generated types

        --sync
            Generate synchronous rust code. Async otherwise

    -V, --version
            Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    live      Generate your modules against your own db
    schema    Generate your modules against schema files
```

## Using schema files
```
Generate your modules against schema files

USAGE:
    cornucopia schema [SCHEMA_PATH]...

ARGS:
    <SCHEMA_PATH>...    Paths containing the database schema (SQL files or directory of SQL
                        files)

OPTIONS:
    -h, --help    Print help information
```


## Using running database
```
Generate your modules against your own db

USAGE:
    cornucopia live <URL>

ARGS:
    <URL>    Postgres url to the database

OPTIONS:
    -h, --help    Print help information
```
