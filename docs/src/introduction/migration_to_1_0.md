# Migration to 1.0

Cornucopia `1.0` is a major release that adopts the work from the [Clorinde](https://github.com/halcyonnouveau/clorinde) fork (which was itself derived from Cornucopia `0.9`). The two codebases have rejoined under the `cornucopia` name. This page describes the breaking changes you'll encounter when upgrading from Cornucopia `0.9.x`.

## Crate-based code generation
Cornucopia `1.0` generates a *crate* instead of a single file. The crate has its own `Cargo.toml` listing the dependencies and features your queries need, so you don't have to add them to your project's manifest.

For example, Cornucopia `0.9`'s ["Full dependencies"](https://github.com/cornucopia-rs/website/blob/main/book/introduction/dependencies.md#full-dependencies) example:

```toml
[dependencies]
# Required
postgres-types = { version = "*", features = ["derive"] }

# Async
cornucopia_async = { version = "*", features = ["with-serde_json-1"] }
tokio = { version = "*", features = ["full"] }
tokio-postgres = { version = "*", features = [
    "with-serde_json-1",
    "with-time-0_3",
    "with-uuid-1",
    "with-eui48-1",
] }
futures = "*"
# Async connection pooling
deadpool-postgres = { version = "*" }

# Row serialization
serde = { version = "*", features = ["derive"] }

# Extra types
serde_json = "*"
time = "*"
uuid = "*"
eui48 = "*"
rust_decimal = { version = "*", features = ["db-postgres"] }
```

Can be replaced with:

```toml
[dependencies]
cornucopia = { path = "cornucopia" }
```

Cornucopia also re-exports the dependencies: `postgres`, `tokio-postgres`, and `deadpool-postgres`.

A drawback of this approach is that `cargo publish` rejects crates with path-only dependencies. If you want to publish a crate that depends on Cornucopia, you need to publish the generated crate separately first.

To do that, use a [`cornucopia.toml`](../configuration.md) file to set the `[manifest.package]` section of the generated `Cargo.toml`. For example, a `cornucopia.toml` containing:

```toml
[manifest.package]
name = "my-cornucopia-queries"
version = "0.1.0"
license = "MIT"
homepage = "https://github.com/furina/my-repo"
repository = "https://github.com/furina/my-repo"
publish = true
```

Will generate `cornucopia/Cargo.toml` with the specified `[package]` where you can then publish the crate as `my-cornucopia-queries`.

## `chrono` instead of `time`
Cornucopia `1.0` uses the `chrono` crate instead of `time`. If you want to keep using `time`, use ["Custom Type Mappings"](../configuration.md#custom-type-mappings) to map the Postgres types to the `time` crate.

```toml
[types.mapping]
"pg_catalog.timestamp" = "time::PrimitiveDateTime"
"pg_catalog.timestamptz" = "time::OffsetDateTime"
"pg_catalog.time" = "time::Time"
"pg_catalog.date" = "time::Date"

[manifest.dependencies]
time = { version = "0.3", features = ["serde"] }
# enable the time feature of postgres and tokio-postgres
postgres = { version = "0.19", features = [
    "with-time-0_3",
    "with-serde_json-1",
] }
tokio-postgres = { version = "0.7", features = [
    "with-time-0_3",
    "with-serde_json-1",
] }
```
