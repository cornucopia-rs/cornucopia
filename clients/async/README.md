<div align="center"> <img src="https://raw.githubusercontent.com/cornucopia-rs/cornucopia/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">Cornucopia</h1>
<div align="center">
 <strong>
   Generate type-checked Rust from your SQL
 </strong>
</div>

<br />

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/cornucopia_async">
    <img src="https://img.shields.io/crates/v/cornucopia_async.svg?style=flat-square"
    alt="Crates.io version" />
  </a>

  <!-- Book -->
  <a href="https://cornucopia-rs.netlify.app/book/index.html">
  <img src="https://img.shields.io/badge/book-latest-blue?logo=mdbook&style=flat-square" alt="book">
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/cornucopia_async/latest/cornucopia_async/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/cornucopia_async?style=flat-square">
  </a>

  <!-- License -->
  <a href="https://github.com/cornucopia-rs/cornucopia#License">
    <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square" alt="License">
  </a>

  <!-- Chat -->
  <a href="https://discord.gg/nYwUmQDHBZ">
    <img src="https://img.shields.io/discord/987088069280825401?label=chat&logo=discord&style=flat-square" alt="Chat">
  </a>
</div>

---

**Note:** This crate is the *asynchronous* client. You can find the *synchronous* client [here](https://crates.io/crates/cornucopia_sync).

This is a client crate for [Cornucopia](https://crates.io/crates/cornucopia). This dependency provides
1. Internals required by the generated code.
2. Public items that you may find useful when working with Cornucopia (you can find more info about these in the [docs](https://docs.rs/cornucopia_async/latest/cornucopia_async/)).

***You need to depend on this crate for Cornucopia's generated code to work properly.***