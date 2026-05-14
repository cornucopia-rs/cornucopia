# Introduction
Cornucopia is a tool powered by [`rust-postgres`](https://github.com/sfackler/rust-postgres) designed to generate type-checked Rust interfaces from PostgreSQL queries, with an emphasis on compile-time safety and high performance. It works by preparing your queries against an actual database and then running an extensive validation suite on them. Rust code is then generated into a separate crate, which can be imported and used in your project.

The basic premise is thus to:

1. Write your PostgreSQL queries.
2. Use Cornucopia to generate a crate with type-safe interfaces to those queries.
3. Import and use the generated code in your project.

Compared to other Rust database interfaces, Cornucopia's approach has the benefits of being simple to understand while also generating code that is both ergonomic and free of heavy macros or complex generics. Since Cornucopia generates plain Rust structs, you can also easily build upon the generated items.

```admonish info
If you just want to get started without having to read all of this, you can take a look at our [examples](examples.html).

*This book is pretty short and to the point though, so you should probably at least take a glance.*
```
