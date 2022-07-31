# Example
**Note:** This example uses synchronous Rust. You can find the asynchronous version [here](../basic_async/README.md).

## Before starting
Please follow the [install procedure](../../README.md#install) to ensure
you're ready to get started. 

Before running this example, you should familiarize yourself with
Cornucopia's CLI using the `--help` flag.

## Take a look!
This crate contains a fully working example of a minimal Cornucopia crate. 
There are a few queries defined for you in the `queries/` folder, along with a
schema in the `schema.sql` file. The Rust modules have already been generated in the
`src/cornucopia.rs` file.

In `src/main.rs` you can see the queries in action, as you would use them in your own crate.

## (Optional) Running the example
Looking at the `main.rs` file in your IDE of choice should be instructive enough, 
but this example is also fully runnable.

If you want to be able to run this example, you have to

- Have a reachable PostgreSQL database up and running (container or otherwise).
- Modify the connection pool config (user, password, etc.) in `main.rs` so that
  it can connect to your database.
- Load the schema into your database.
- That's it! You should now be able to run the example.

## Start experimenting
You can add or modify the schema and queries with your favorite SQL tool. 
**When you're done modifying, rebuild the Rust modules for your SQL
with Cornucopia's CLI. This will recreate the `src/cornucopia.rs` file.**
Then, you can import and use your new queries in the `main.rs` file.
