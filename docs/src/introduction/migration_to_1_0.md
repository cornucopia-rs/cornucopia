# Migration to 1.0

Cornucopia `1.0` is a major release that adopts the work from the [Clorinde](https://github.com/halcyonnouveau/clorinde) fork (which was itself derived from Cornucopia `0.9`). The two codebases have rejoined under the `cornucopia` name. This page describes the breaking changes you'll encounter when upgrading from Cornucopia `0.9.x`.

## Crate-based code generation
Cornucopia generates a *crate* instead of a single file which allows it to automatically generate a `Cargo.toml` file customised to support all the necessary dependencies and features required by your queries, without polluting your manifest. For example, Cornucopia's ["Full dependencies"](https://cornucopia-rs.netlify.app/book/introduction/dependencies#full-dependencies) example:

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

Could be replaced with:

```toml
[dependencies]
cornucopia = { path = "cornucopia" }
```

Cornucopia also re-exports the dependencies: `postgres`, `tokio-postgres`, and `deadpool-postgres`.

A drawback to crate-based code generation is that `cargo` won't publish crates with path dependencies meaning you either can't publish a crate that depends on Cornucopia or you will need to publish the Cornucopia crate separately.

If doing the latter, you can use a `cornucopia.toml` to specify the `[manifest.package]` section of the `Cargo.toml` in the generated crate. For example, a `cornucopia.toml` that includes:

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
Cornucopia uses the `chrono` crate instead of `time`. If you want to keep using `time`, use ["Custom Type Mappings"](../configuration.html#custom-type-mappings) to map the Postgres types to the `time` crate.

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
