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
    <a href="#install">
      Install
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

You can learn more about using Cornucopia by reading our [book](https://cornucopia-rs.netlify.app/book/index.html), or you can get a quickstart by looking at our [examples](https://cornucopia-rs.netlify.app/book/examples.html).
## MSRV
This crate uses Rust 2021 edition, which requires at least version 1.56.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
