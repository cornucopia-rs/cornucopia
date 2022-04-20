# cornucopia
```
Command line interface to interact with Cornucopia SQL
    cornucopia <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    generate      Generate Rust modules from queries
    help          Print this message or the help of the given subcommand(s)
    migrations    Create and run migrations
```

## migrations
```
Create and run migrations

USAGE:
    cornucopia migrations [OPTIONS] <SUBCOMMAND>
    -h, --help
            Print help information

    -m, --migrations-path <MIGRATIONS_PATH>
            Folder containing the migrations [default: migrations/]

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    new     Create a new migration
    run     Run all migrations
```

### migrations new
```
Create a new migration

USAGE:
    cornucopia migrations new <NAME>

ARGS:
    <NAME>    

OPTIONS:
    -h, --help    Print help information
```

### migrations run
```
Run all migrations

USAGE:
    cornucopia migrations run --url <URL>

OPTIONS:
    -h, --help         Print help information
        --url <URL>    Postgres url to the database
```

## generate
```
Generate Rust modules from queries

USAGE:
    cornucopia generate [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -d, --destination <DESTINATION>
            Destination folder for generated modules [default: src/cornucopia.rs]

    -h, --help
            Print help information

    -m, --migrations-path <MIGRATIONS_PATH>

    -p, --podman
            Folder containing the migrations

    -q, --queries-path <QUERIES_PATH>
            Folder containing the queries [default: queries/]

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    live    Generate your modules against your own db
```

### generate live
```
Generate your modules against your own db

USAGE:
    cornucopia generate live --url <URL>

OPTIONS:
    -h, --help         Print help information
    -u, --url <URL>    Postgres url to the database
```