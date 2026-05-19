# Configuration
Cornucopia can be configured using a configuration file (`cornucopia.toml` by default) in your project. This file allows you to customise generated code behaviour, specify static files, manage dependencies, and override type mappings.

## How the config file is loaded
When invoking the CLI, Cornucopia looks for `cornucopia.toml` in the current directory. You can point at a different file with the `--config` flag:

```bash
cornucopia --config path/to/my-cornucopia.toml schema schema.sql
```

Note that CLI flags override config file values when set. This lets you keep a stable, checked-in `cornucopia.toml` for reproducible builds while still allowing one-off overrides from the command line.

## Code generation
These options control where Cornucopia reads queries from, where it writes generated code to, and what flavour of code it produces.

```toml
# Directory containing your `.sql` query files (default: "queries/")
queries = "queries/"

# Directory where the generated crate will be written (default: "cornucopia")
destination = "cornucopia"

# Generate asynchronous code (default: true)
async = true

# Generate synchronous code (default: false)
# Both `sync` and `async` may be enabled at the same time.
sync = false

# For queries that declare a named parameter struct, make the generated
# `bind()` function private so callers must construct the named params
# struct (default: false).
params-only = false

# Skip query files whose name starts with `_` (default: false).
ignore-underscore-files = false
```

~~~admonish warning
Cornucopia will delete and re-create the `destination` directory on every run. As a safety check, if the destination already exists, does not end with the name `cornucopia`, and does not contain a `Cargo.toml`, the CLI will prompt before continuing. Point `destination` at a directory dedicated to Cornucopia's output to avoid overwrites.
~~~

## Container management
When using `cornucopia schema`, Cornucopia starts a temporary database container, applies your schema, and tears it down once generation is complete. These settings control how that container is managed:

```toml
# Container image to launch (default: "docker.io/library/postgres:latest")
container-image = "docker.io/library/postgres:latest"

# Milliseconds to wait after the container reports healthy before
# attempting to connect (default: 250). Bump this if you see flaky
# "connection refused" errors on slower machines.
container-wait = 250

# Use `podman` instead of `docker` to manage the container (default: false)
podman = false
```

These options have no effect on the `live` and `fresh` subcommands, which connect to a database you manage yourself.

## Style
The `[style]` section configures cosmetic aspects of the generated code:

```toml
[style]
# Render enum variants in CamelCase regardless of how they are spelled in
# PostgreSQL (default: false). The PostgreSQL value used at the wire level
# is left untouched, only the Rust identifier is reformatted.
enum-variant-camel-case = true
```

For example, with `enum-variant-camel-case = true`, a PostgreSQL enum value `north_west` will be exposed as the Rust variant `NorthWest` while still serialising as `north_west` on the wire.

## Manifest configuration
The `[manifest]` section allows you to configure the entire Cargo.toml for the generated crate:

```toml
[manifest.package]
name = "furinapp-queries"
version = "1.0.0"
description = "Today I wanted to eat a *quaso*."
license = "MIT"
edition = "2021"

[manifest.dependencies]
serde = { version = "1.0", features = ["derive"] }
my_custom_types = { path = "../types" }
```

This gives you complete control over the generated Cargo.toml. Cornucopia will automatically merge your configuration with the required PostgreSQL dependencies based on the types found in your SQL queries.

### Dependency merging
Cornucopia automatically adds dependencies based on your PostgreSQL schema:
- Core dependencies: `postgres-types`, `postgres-protocol`, `postgres`
- Type-specific dependencies: `chrono`, `uuid`, `serde_json`, etc. (based on column types)
- Async dependencies: `tokio-postgres`, `futures`, `deadpool-postgres` (when async enabled)

Your custom dependencies in `[manifest.dependencies]` will be preserved and merged with these auto-generated ones.

## Workspace dependencies
The `use-workspace-deps` option allows you to integrate the generated crate with your workspace's dependency management:

```toml
# Use workspace dependencies from the current directory's Cargo.toml
use-workspace-deps = true

# Use workspace dependencies from a specific Cargo.toml
use-workspace-deps = "../../Cargo.toml"
```

When this option is set, Cornucopia will:
1. Look for dependencies in the specified Cargo.toml file (or `./Cargo.toml` if set to `true`)
2. Set `workspace = true` for any dependencies that exist in the workspace manifest
3. Fall back to regular dependency declarations for packages not found in the workspace

## Custom type mappings
You can configure custom type mappings using the `types` section:

```toml
[manifest.dependencies]
# Dependencies required for custom type mappings
ctypes = { path = "../ctypes" }
postgres_range = { version = "0.11.1", features = ["with-chrono-0_4"] }

[types.mapping]
# Simple mapping: just specify the Rust type
"pg_catalog.date" = "ctypes::date::Date"
"pg_catalog.tstzrange" = "postgres_range::Range<chrono::DateTime<chrono::FixedOffset>>"
```

Dependencies needed for your custom type mappings should be specified in `[manifest.dependencies]`.

The `types.mapping` table allows you to map PostgreSQL types to Rust types. You can use this to either override Cornucopia's default mappings or add support for PostgreSQL types that aren't supported by default, such as types from extensions.

### Detailed mapping syntax

For more control, you can use the detailed mapping syntax:

```toml
[types.mapping."pg_catalog.date"]
rust-type = "ctypes::date::Date"
is-copy = false
attributes = ['serde(skip_serializing_if = "Option::is_none")']
attributes-borrowed = []
```

The available options are:
- **`rust-type`**: The Rust type to use (required)
- **`is-copy`**: Whether the type implements `Copy` (default: `true`)
- **`attributes`**: Rust attributes to apply to fields in owned structs
- **`attributes-borrowed`**: Rust attributes to apply to fields in borrowed structs
- **`attributes-nullable`**: Rust attributes to apply to fields in owned structs when the column is nullable (i.e., the field is `Option<T>`). When unset or empty, falls back to `attributes`.
- **`attributes-borrowed-nullable`**: Rust attributes to apply to fields in borrowed structs when the column is nullable. When unset or empty, falls back to `attributes-borrowed`.

### Nullable field attributes

Some serde adapters need a different `with =` path depending on whether a field is `T` or `Option<T>`. For example, `time::serde::rfc3339` works on `OffsetDateTime` and `time::serde::rfc3339::option` works on `Option<OffsetDateTime>`. Use `attributes-nullable` to override the attributes applied when the column is nullable:

```toml
[types.mapping."pg_catalog.timestamptz"]
rust-type = "time::OffsetDateTime"
attributes = ['serde(with = "time::serde::rfc3339")']
attributes-nullable = ['serde(with = "time::serde::rfc3339::option")']
```

### Borrowed type mappings

When you have separate owned and borrowed versions of a type (similar to `String` and `&str`), you can specify a `borrowed-type`:

```toml
[types.mapping."pg_catalog.varchar"]
rust-type = "my_crate::CustomString"
borrowed-type = "my_crate::CustomStringRef<'a>"
is-copy = false
```

This will use:
- `my_crate::CustomString` in owned structs (e.g., `Character`)
- `my_crate::CustomStringRef<'a>` in borrowed structs (e.g., `CharacterBorrowed<'a>`)

~~~admonish note
Both the owned type and borrowed type must implement [`FromSql`](https://docs.rs/postgres-types/latest/postgres_types/trait.FromSql.html) from the [`postgres-types`](https://crates.io/crates/postgres-types) crate. The owned type must also implement [`ToSql`](https://docs.rs/postgres-types/latest/postgres_types/trait.ToSql.html).

Additionally, the borrowed type should implement `Into<OwnedType>` so the generated `From` implementation can convert borrowed structs to owned structs.

See the [custom_types](https://github.com/cornucopia-rs/cornucopia/blob/main/examples/custom_types) example for a reference implementation.
~~~

## Derive traits
You can specify `#[derive]` traits for generated structs using this field.

```toml
[types]
derive-traits = ["serde::Serialize", "serde::Deserialize", "Hash"]
```

This will add the traits to **all** structs. If you only want them added to specific structs, see this section in ["Type annotations"](./writing_queries/type_annotations.html#derive-traits).

~~~admonish note
Adding any `serde` trait will automatically add `serde` as a dependency in the package manifest.
~~~

### Custom PostgreSQL type configuration
For more granular control over [custom PostgreSQL types](./introduction/types.html#custom-postgresql-types) (enums, composites, domains), use the `types.custom` section:

```toml
[types]
# Applied to all generated structs and postgres types
derive-traits = ["Default"]

# Configuration for specific custom postgres types
[types.custom.fontaine_region]
derive-traits = ["serde::Deserialize"]
attributes = ["repr(u8)"]
```

This configuration will add the `Default` trait to all generated types (and structs), but will only add `serde::Deserialize` and `#[repr(u8)]` to the `fontaine_region` enum:

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
pub enum FontaineRegion {
    // ...
}
```

The `attributes` field is useful for libraries that require specific attributes on types, such as [facet](https://facet.rs) which requires `#[repr]` on enums for reflection.

~~~admonish note
PostgreSQL identifiers (including type names) are case-insensitive unless quoted during creation. This means that a type created as `CREATE TYPE Fontaine_Region` will be stored as `fontaine_region` in the PostgreSQL system catalogs. When referencing custom PostgreSQL types in `types.custom`, you should use the lowercase form unless the type was explicitly created with quotes.
~~~

You can combine global and type-specific derive traits - the traits will be merged for the specified custom PostgreSQL types.

## Query field metadata
This is an opt-in feature that generates lightweight metadata about each result-row struct.

### Enable via configuration
Add the following to your project's `cornucopia.toml`:

```toml
generate-field-metadata = true
```

### What gets generated
When enabled, the generated crate will include:

- __`FieldMetadata`__ struct in `crate::types`:
  - `name: &'static str`
  - `rust_type: &'static str`
  - `pg_type: &'static str` (schema-qualified PostgreSQL type, e.g. `pg_catalog.int4`)
- __Per-row struct method__: each generated row struct implements

```rust
pub fn field_metadata() -> &'static [FieldMetadata]
```

For example, a row struct like `queries::lock_info::LockInfo` exposes:

```rust
let meta: &'static [cornucopia::types::FieldMetadata] =
    cornucopia::queries::lock_info::LockInfo::field_metadata();
```

### Example: derive column names at runtime
You can map metadata to user-facing headers or diagnostics:

```rust
let headers: Vec<&str> = cornucopia::queries::lock_info::LockInfo::field_metadata()
    .iter()
    .map(|m| m.name)
    .collect();
```

This avoids hardcoding column labels and keeps UIs resilient to query changes.

## Static files
The `static` field allows you to copy or link files into your generated crate directory. This is useful for including files like licenses, build configurations, or other assets that should persist across code generation.

### Simple file copying
```toml
# Simple copy of files to the root of the generated directory
static = ["LICENSE.txt", "build.rs"]
```

### Advanced configuration
```toml
static = [
    # Simple copy (copies to root with original filename)
    "README.md",

    # Rename file during copy
    { path = "config.template.toml", destination = "config.toml" },

    # Place file in subdirectory
    { path = "assets/logo.png", destination = "static/images/logo.png" },

    # Hard link instead of copy (saves disk space for large files)
    { path = "large_asset.bin", hard-link = true },

    # Combine renaming with hard linking
    { path = "data.json", destination = "resources/app_data.json", hard-link = true }
]
```

### Configuration options
- **`path`**: Source file path (required)
- **`destination`**: Target path within the generated directory (optional)
  - If not specified, uses the original filename in the root directory
  - Can include subdirectories which will be created automatically
- **`hard-link`**: Create a hard link instead of copying (optional, default: `false`)
  - Useful for large files to save disk space
  - Both source and destination must be on the same filesystem

### Examples
```toml
static = [
    # Copy LICENSE to root as-is
    "LICENSE",

    # Rename during copy
    { path = "template.env", destination = ".env.example" },

    # Organize into subdirectories
    { path = "docs/api.md", destination = "documentation/api.md" },
    { path = "scripts/build.sh", destination = "tools/build.sh" }
]
```

When using the detailed configuration format, Cornucopia will automatically create any necessary parent directories for the destination path.
