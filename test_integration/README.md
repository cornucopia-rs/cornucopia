Integration testing entrypoint.

This acts like a testing harness. It manages the database required for testing, coordinates the execution of tests and reports the results of the test suite. Internally, it uses many of our workspace crates, notably `test_codegen`, but also `examples` and `benches`.

Our integration testing not only checks that Cornucopia is able to generate the code, but it also tests that the right error messages are reported in case of user errors. It also runs the generated code to ensure its correctness.

The test cases are auto-described using TOML fixtures. These files are deserialized when the integration tests are run and describe what should be generated, where it shoulld be generated, etc.

# How to use

The crate can be executed directly with `cargo run`, but it will also automatically be invoked when running workspace tests.

When executed directly, the crate accepts CLI arguments to *update* the generated code or error messages. This is useful when you made changes (either to the generated code or to error message) and want to propagate them to the rest of the workspace:
* Update generated code: --apply-codegen
* Update errors: --apply-errors

Note that if you made modifications that affect generated code or errors and you don't update the workspace code, the integration tests will fail.

By default, the tests run using `docker`, but you can pass a `--podman` CLI argument.