Benchmarking suite for code generation and code execution, heavily based on [the diesel benchmarking suite](https://github.com/diesel-rs/diesel/tree/master/diesel_bench).

Note that the benchmarks use the `diesel` crate which links directly against `libpq`, so you will need to install it. On debian-based distros, the package is `libpq-dev`, and on RHEl-based distros, it is named `libpq-devel`.