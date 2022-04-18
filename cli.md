# cornucopia
```
Command line interface to interact with Cornucopia SQL

USAGE:
    cornucopia <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    generation    Generate Rust modules from queries
    help          Print this message or the help of the given subcommand(s)
    migration     Create and run migrations
```

## migration
```
OPTIONS:
    -d, --destination <DESTINATION>
            Destination folder for generated modules [default: src/cornucopia.rs]

    -h, --help
            Print help information

    -m, --migrations-path <MIGRATIONS_PATH>
            Folder containing the migrations [default: migrations/]

    -q, --queries-path <QUERIES_PATH>
            Folder containing the queries [default: queries/]
```

## generation
```
Create and run migrations
    cornucopia migration [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help
            Print help information

    -m, --migrations-path <MIGRATIONS_PATH>
            Folder containing the migrations [default: migrations]

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    new     Create a new migration
    run     Run all migrations
```