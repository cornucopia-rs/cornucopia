<div align="center"> <img src="https://raw.githubusercontent.com/cornucopia-rs/cornucopia/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">Cornucopia</h1>
<div align="center">
 <strong>
   Generate type checked  Rust from your SQL
 </strong>
</div>

<br />

<div align="center">
  <!-- Github Actions -->
  <img src="https://img.shields.io/github/workflow/status/cornucopia-rs/cornucopia/ci" alt="actions status" />
  <!-- Version -->
  <a href="https://crates.io/crates/cornucopia">
    <img src="https://img.shields.io/crates/v/cornucopia.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/cornucopia">
    <img src="https://img.shields.io/crates/d/cornucopia.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

<div align="center">
  <h4>
    <a href="#install">
      Install
    </a>
    <span> | </span>
    <a href="/examples/basic/README.md">
      Example
    </a>
  </h4>
</div>

---

This crate is a small library exposing Cornucopia's `GenericClient`. You probably need this if you're a Cornucopia user.

The `GenericClient` is an abstraction over four types of connections (`deadpool_postgres::Client`, `deadpool_postgres::Transaction`, `tokio_postgres::Client`, `tokio_postgres::Transaction`). Its meant to allow you to mix-and-match these connection types in Cornucopia Queries.

|                  | non-pooled                    | pooled                           |
| ---------------- | ----------------------------- | -------------------------------- |
| single-statement | `tokio_postgres::Client`      | `deadpool_postgres::Client`      |
| multi-statement  | `tokio_postgres::Transaction` | `deadpool_postgres::Transaction` |