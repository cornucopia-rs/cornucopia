<div align="center"> <img src="https://raw.githubusercontent.com/cornucopia-rs/cornucopia/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">Cornucopia</h1>
<div align="center">
 <strong>
   Generate type-checked  Rust from your SQL
 </strong>
</div>

<br />

<div align="center">
  <!-- Downloads -->
  <a href="https://crates.io/crates/cornucopia">
    <img src="https://img.shields.io/crates/d/cornucopia.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- Version -->
  <a href="https://crates.io/crates/cornucopia">
    <img src="https://img.shields.io/crates/v/cornucopia.svg?style=flat-square"
    alt="Crates.io version" />
  </a>

  <!-- Book -->
  <a href="https://cornucopia-rs.netlify.app/book/index.html">
  <img src="https://img.shields.io/badge/book-latest-blue?logo=mdbook&style=flat-square" alt="book">
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/cornucopia/latest/cornucopia/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/cornucopia?style=flat-square">
  </a>
  
  <!-- Dependencies -->
  <a href="https://deps.rs/repo/github/cornucopia-rs/cornucopia">
    <img src="https://deps.rs/repo/github/cornucopia-rs/cornucopia/status.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
 
  <br/>

  <!-- License -->
  <a href="https://github.com/cornucopia-rs/cornucopia#License">
    <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square" alt="License">
  </a>

  <!-- Chat -->
  <a href="https://discord.gg/nYwUmQDHBZ">
    <img src="https://img.shields.io/discord/987088069280825401?label=chat&logo=discord&style=flat-square" alt="Chat">
  </a>
</div>

<div align="center">
  <h4>
    <a href="https://cornucopia-rs.netlify.app/">
      Homepage
    </a>
    <span> | </span>
    <a href="examples/basic_async/README.md">
      Example
    </a>
  </h4>
</div>

<br />

Cornucopia is a tool powered by [`rust-postgres`](https://github.com/sfackler/rust-postgres) designed to generate type-checked Rust interfaces from your PostgreSQL queries. It works by preparing your queries against an actual database and then running an extensive validation suite on them. Once the queries are prepared and validated, Rust code is generated into a module, which can be imported and used in your project. 

The basic premise is thus to:
1. Write your PostgreSQL queries.
2. Use Cornucopia to generate Rust code.
3. Use the generated code in your project.

Compared to other Rust database interfaces, Cornucopia's approach has the benefits of being simple to understand while also generating code that is both ergonomic and free of heavy macros or complex generics. Since Cornucopia generates plain Rust structs, you can also easily build upon the generated items.

Here are some defining features:
* SQL-first. Your SQL is the only source of truth. No intricate ORM.
* Powerful query validation. Catch errors before runtime, with powerful (and pretty) diagnostics.
* Supports custom user types (composites, domains, and enums) and one-dimensional arrays.
* Sync and async driver support, with optional pooling.
* Ergonomic non-allocating row mapping.
* Granular type nullity control.
* Available as a library and a CLI.
* As close to native `rust-postgres` performance as we can make it.

You can learn more about using Cornucopia by reading our [book](https://cornucopia-rs.netlify.app/book/index.html), or you can get a quickstart by looking at our [examples](https://cornucopia-rs.netlify.app/book/examples.html).

## A quick taste of Cornucopia
The [book](https://cornucopia-rs.netlify.app/book/index.html) is the place to go to get more in-depth explanations, but here is the simplest of tasters to give you an idea.

Let's say you have the following PostgreSQL queries
**/queries/some_query_file.sql**
```sql
--! authors
SELECT first_name, last_name, country FROM Authors;

--! insert_author
INSERT INTO Authors(first_name, last_name, country) 
VALUES (:first_name, :last_name, :country)
```
Notice the query annotations (`--! authors`, `--! insert_authors`) and the named bind parameters (`:first_name`, etc.).

1. First of all, you can organize that queries into folder /queries/ as mentioned or pass the folder when running the CLI
2. You can generate the Rust Code with CLI, for example using a local database 
```shell
$ cornucopia live postgres://{usr}:{psw}@localhost:5432/{database}"
```

Then, after generating the Rust code with Cornucopia's CLI, you can import it into your project like so:
```rust
mod cornucopia;
use cornucopia::{authors, insert_author};
```

Finally here is an example usage of these queries:
```rust
insert_author.bind(&client, "Agatha", "Christie", "England");

let all_authors = authors().bind(&client).all();

for author in all_authors {
  println!("[{}] {}, {}", 
    author.country, 
    author.last_name.to_uppercase(), 
    author.first_name
  )
}
```
You can customize pretty much every aspect of your queries easily with Cornucopia (custom parameters and row structs, renaming, nullity control, etc.), so please head over to the [book](https://cornucopia-rs.netlify.app/book/index.html) if you're interested to learn more.

## MSRV
This crate uses Rust 2021 edition, which requires at least version 1.62.1.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
