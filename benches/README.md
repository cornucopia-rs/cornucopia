# Cornucopia Benchmarks
Benchmarking suite for code generation and code execution, heavily based on [the diesel benchmarking suite](https://github.com/diesel-rs/diesel/tree/master/diesel_bench).

Note that the benchmarks use the `diesel` crate which links directly against `libpq`, so you will need to install it.
On debian-based distros, the package is `libpq-dev`, on RHEl-based distros, it is named `libpq-devel`, and for Arch-based distros, it is included with `postgresql-libs`.

## Running
```bash
cargo bench
```

## Results
These results are from benchmarks run locally on `2025-06-20` and may not reflect production environment performance. See the full Criterion report [here](https://beanpuppy.github.io/clorinde-benches/2025-06-20/report).

### System
- CPU: AMD Ryzen 7 9800X3D
- RAM: 64GB DDR5-6400
- OS: EndeavourOS

### Versions
- Rust: 1.86.0
- PostgreSQL: 17.2
- Libraries:
  - postgres: 0.19.9
  - tokio-postgres: 0.7.12
  - diesel: 2.2.10
  - sqlx: 0.8.6
  - cornucopia: 1.0.0

### `bench_trivial_query`
Measures performance of `SELECT * FROM users`.

![Trivial Query Benchmark](https://raw.githubusercontent.com/beanpuppy/clorinde-benches/refs/heads/main/2025-06-20/bench_trivial_query/report/lines.svg)

### `bench_complex_query`
Measures performance of a LEFT JOIN between users and posts.

![Medium Complex Query Benchmark](https://raw.githubusercontent.com/beanpuppy/clorinde-benches/refs/heads/main/2025-06-20/bench_medium_complex_query/report/lines.svg)

### `bench_loading_associations_sequentially`
Measures performance of loading users with their associated posts and comments.

![Loading Associations Benchmark](https://raw.githubusercontent.com/beanpuppy/clorinde-benches/refs/heads/main/2025-06-20/bench_loading_associations_sequentially/report/violin.svg)

### `bench_insert`
Measures performance of inserting multiple rows (1, 100, 1000 rows).

![Batch Insert Benchmark](https://raw.githubusercontent.com/beanpuppy/clorinde-benches/refs/heads/main/2025-06-20/bench_insert/report/lines.svg)

## Disclaimer
These benchmarks are meant to provide a rough comparison of different approaches in a local development environment. Real-world performance can vary significantly based on many factors including network latency, concurrent access patterns, and specific use cases.
