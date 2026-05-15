# Adding custom types through `cornucopia.toml`
This example shows how you can add custom types to be used. You need to create a new crate which implements the `FromSql` and `ToSql` traits from `postgres-types` for your custom types.

The custom type crates are imported into the generated Cornucopia crate through the manifest configuration.

## Adding custom type dependencies

You can specify custom type dependencies in the `[manifest.dependencies]` section:

```toml
[manifest.dependencies]
# Local crate with a relative path
ctypes = { path = "../ctypes" }

# Crate from crates.io with a specific version
custom_types = "1.0.0"

# Crate with additional configuration
types_with_features = { version = "2.0", features = ["date", "time"] }

# Complete example with all options
full_example = {
    version = "1.2.3",
    path = "../local_types",
    features = ["custom_types"],
    default-features = false,
    optional = true
}
```

You can specify multiple crates, and each one can use any of the standard Cargo dependency specifications. This includes:
- Simple version strings for crates from crates.io
- Local crates with path dependencies
- Crates with specific features enabled
- Crates with default features disabled
- Optional dependencies
- Any combination of these options

The configuration follows the same format as dependencies in `Cargo.toml`, and these dependencies will be merged with the PostgreSQL dependencies that Cornucopia automatically generates based on your SQL queries.

## Mapping PostgreSQL types to Rust types

Use `types.mapping` to map PostgreSQL types to your custom Rust types:

```toml
[types.mapping]
# Simple mapping
"public.element" = "db_types::element::Element"

# Detailed mapping with options
[types.mapping."pg_catalog.date"]
rust-type = "db_types::date::Date"
is-copy = false
attributes = ['serde(skip_serializing_if = "Option::is_none")']
```

## Borrowed type mappings

When you have separate owned and borrowed versions of a type (similar to `String` and `&str`), you can specify a `borrowed-type`. This example demonstrates this with `CustomString` and `CustomStringRef<'a>`:

```toml
[types.mapping."pg_catalog.varchar"]
rust-type = "db_types::string::CustomString"
borrowed-type = "db_types::string::CustomStringRef<'a>"
is-copy = false
```

This will use:
- `CustomString` in owned structs (e.g., `Character`)
- `CustomStringRef<'a>` in borrowed structs (e.g., `CharacterBorrowed<'a>`)

Both types must implement `FromSql`. The owned type must also implement `ToSql`, and the borrowed type should implement `Into<OwnedType>` for the generated `From` implementation.
