# Automatic rebuilding with `build.rs`
**Note** The build script contains very simple instructions that need to be done before seeing the automatic build feature in action.

Cornucopia can used with your custom `build.rs` to achieve more advanced workflows. This example shows how you can make your queries be regenerated automatically when you build your crate everytime your schema and queries are modified.

Taking a look at the `build.rs` should give you enough ideas get you started with your own custom workflow. If you want to see this example in action, add more queries and observe how they are added to the generated file when you rebuild the crate. Rebuilding the crate without modifying queries or schema should be instant as the build script does not need to be rerun.