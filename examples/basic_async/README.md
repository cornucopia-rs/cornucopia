# Example
## Before starting
Please follow the the [install procedure](../../README.md#install) to ensure you're ready to get started. Before running CLI commands, you can take a look at its interface, either via the `--help` flag, or by reading this [document](../../cli.md).

## Take a look!
This crate contains a fully working example of what a Cornucopia project can look like. There are a few queries and migrations defined for you in the `migrations/` and `queries/` folders. Please bear in mind the used SQL is for demonstration purposes only.
The Rust modules have already been generated in the `src/cornucopia.rs` file. 

In `src/main.rs` you can see the queries in action, as you would use them in your own project. Seeing how the queries are used should give you a solid idea of what `cornucopia` is about, enough to get you started in your own project.
## (Optional) Running the example
If you want to be able to run this example, you should
* Have a reachable postgres database up-and-running (container or otherwise).
* Modify the connection pool config (user, password, etc.) in `main.rs` so that it can connect to your database.
* Run the migrations (with `cornucopia migration run` or otherwise).
* That's it! You should now be able to run the example.

## Start experimenting
Feel free to modify the migrations, or add new ones with `cornucopia migration new`. You can also add or modify queries with your favorite SQL tool (no special command needed). **When you're done modifying, rebuild the Rust modules for your SQL with `cornucopia generation`. This will recreate the `src/cornucopia.rs` file.**

## Going deeper
If you want to know more, the [project's readme](../../README.md) explains pretty much everything there is to know about Cornucopia. The CLI's `--help` flag is also handy.