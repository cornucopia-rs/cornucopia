# Configuration
Clorinde can be configured using a configuration file (`clorinde.toml` by default) in your project. This file allows you to customise generated code behaviour, specify static files, manage dependencies, and override type mappings.

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

This gives you complete control over the generated Cargo.toml. Clorinde will automatically merge your configuration with the required PostgreSQL dependencies based on the types found in your SQL queries.

### Dependency merging
Clorinde automatically adds dependencies based on your PostgreSQL schema:
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

When this option is set, Clorinde will:
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

The `types.mapping` table allows you to map PostgreSQL types to Rust types. You can use this to either override Clorinde's default mappings or add support for PostgreSQL types that aren't supported by default, such as types from extensions.

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

See the [custom_types](https://github.com/halcyonnouveau/clorinde/blob/main/examples/custom_types) example for a reference implementation.
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
Add the following to your project's `clorinde.toml`:

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
let meta: &'static [clorinde::types::FieldMetadata] =
    clorinde::queries::lock_info::LockInfo::field_metadata();
```

### Example: derive column names at runtime
You can map metadata to user-facing headers or diagnostics:

```rust
let headers: Vec<&str> = clorinde::queries::lock_info::LockInfo::field_metadata()
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

When using the detailed configuration format, Clorinde will automatically create any necessary parent directories for the destination path.
