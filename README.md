<!-- markdownlint-disable MD041 MD033 -->
<div align="center">
  <img src="https://raw.githubusercontent.com/cornucopia-rs/cornucopia/main/assets/logo.svg" alt="Cornucopia logo" width=150 />

  <h1>Cornucopia</h1>

  <strong>Generate type-checked Rust from your SQL</strong>

  [![Crates.io](https://img.shields.io/crates/v/cornucopia.svg?style=flat-square)](https://crates.io/crates/cornucopia)
  [![Book](https://img.shields.io/badge/book-latest-blue?logo=mdbook&style=flat-square)](https://cornucopia-rs.github.io/cornucopia/)
  [![License](https://img.shields.io/badge/license-APACHE--2.0%2FMIT-blue?style=flat-square)](https://github.com/cornucopia-rs/cornucopia#license)
  [![Dependencies](https://deps.rs/repo/github/cornucopia-rs/cornucopia/status.svg?style=flat-square)](https://deps.rs/repo/github/cornucopia-rs/cornucopia)
</div>
<!-- markdownlint-enable MD041 MD033 -->

> [!NOTE]
> Cornucopia 1.0 merged the [Clorinde](https://github.com/halcyonnouveau/clorinde) fork back into the original project, adopting its rewritten codegen, expanded capabilities, and accumulated fixes. Huge thanks to [@beanpuppy](https://github.com/beanpuppy) and the Clorinde contributors for their work. If you are upgrading from Cornucopia `0.9.x`, see the [migration guide](https://cornucopia-rs.github.io/cornucopia/introduction/migration_to_1_0.html).

Cornucopia generates type-checked Rust interfaces from PostgreSQL queries, with an emphasis on compile-time safety and high performance. It works by preparing your queries against an actual database and then running an extensive validation suite on them. Rust code is then generated into a separate crate, which can be imported and used in your project.

The basic premise is thus to:

1. Write your PostgreSQL queries.
2. Use Cornucopia to generate a crate with type-safe interfaces to those queries.
3. Import and use the generated code in your project.

You can learn more about Cornucopia by reading the [book](https://cornucopia-rs.github.io/cornucopia/), or you can get a quickstart by looking at the [examples](https://cornucopia-rs.github.io/cornucopia/examples.html).

## Key Features

- **Type Safety** - Catch SQL errors at compile time with powerful diagnostics.
- **SQL-First** - Write plain SQL queries, get generated Rust code. No ORMs or query builders, just the SQL you know and love.
- **Fast** - Performance close to hand-written `rust-postgres` code.
- **Flexible** - Works with sync/async code and connection pools.
- **PostgreSQL Native** - Full support for custom types, enums, and arrays. Leverage PostgreSQL's advanced features without compromise.
- **Custom Types** - Map database types to your own Rust structs.

## Installation

Install with:

```bash
cargo install cornucopia
```

## Quick Example
Write your PostgreSQL queries with annotations and named parameters:
```sql
-- queries/authors.sql

--! insert_author
INSERT INTO authors
    (first_name, last_name, country)
VALUES
    (:first_name, :last_name, :country);

--! authors
SELECT first_name, last_name, country FROM authors;
```

Generate the crate with `cornucopia`, then you can import it into your project after adding it to your `Cargo.toml`:
```toml
cornucopia = { path = "./cornucopia" }
```

And use the generated crate in your code:
```rust
use cornucopia::queries::authors::{authors, insert_author};

insert_author()
    .bind(&mut client, &"Agatha", &"Christie", &"England")
    .unwrap();

let all_authors = authors().bind(&mut client).all().unwrap();

for author in all_authors {
    println!(
        "[{}] {}, {}",
        author.country,
        author.last_name.to_uppercase(),
        author.first_name,
    );
}
```

For more examples go to the [examples](https://github.com/cornucopia-rs/cornucopia/tree/main/examples) directory, or head over to the [book](https://cornucopia-rs.github.io/cornucopia/) to learn more.

## Prior Art

- [sqlc](https://github.com/sqlc-dev/sqlc) (Go) - Generate type-safe code from SQL
- [Kanel](https://github.com/kristiandupont/kanel) (TypeScript) - Generate TypeScript types from Postgres
- [jOOQ](https://github.com/jOOQ/jOOQ) (Java) - Generate typesafe SQL from your database schema

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
