<div align="center"> <img src="https://raw.githubusercontent.com/cornucopia-rs/cornucopia/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">Cornucopia</h1>
<div align="center">
 <strong>
   Generate type-checked Rust from your SQL
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
  <!-- Docs -->
  <a href="https://docs.rs/cornucopia_async/latest/cornucopia_async/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/cornucopia_async?style=flat-square">
  </a>
</div>

---

**Note:** This crate is the *asynchronous* client. You can find the *synchronous* client [here](TODO).

This is a client crate for [Cornucopia](https://crates.io/crates/cornucopia). This dependency provides
1. internals required by the generated code.
2. public items that you may find useful when working with Cornucopia.

***You need to depend on this crate for Cornucopia's generated code to work properly.***