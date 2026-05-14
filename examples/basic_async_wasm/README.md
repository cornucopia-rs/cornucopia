# Example
**Note:** This example uses asynchronous Rust that can be compiled to WebAssembly for a Cloudflare Worker.

## Before starting
Please follow the [install procedure](../../README.md#install) to ensure you're ready to get started.

Before running this example, you should familiarise yourself with Cornucopia's CLI using the `--help` flag.

## Take a look!
This crate contains a fully working example of a minimal Cornucopia crate. There are a few queries defined for you in the `queries/` folder, along with a schema in the `schema.sql` file. The Rust modules have already been generated in the
`src/cornucopia.rs` file.

In `src/lib.rs` you can see the queries in action, as you would use them in your own crate.

## (Optional) Running the example
To generate WebAssembly you can use [wasm-pack](https://github.com/rustwasm/wasm-pack) and run `wasm-pack build`.
