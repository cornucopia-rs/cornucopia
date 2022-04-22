<div align="center"> <img src="https://raw.githubusercontent.com/louisgariepy/cornucopia/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">Cornucopia</h1>
<div align="center">
 <strong>
   Generate type checked  Rust from your SQL
 </strong>
</div>

<br />

<div align="center">
  <!-- Github Actions -->
  <img src="https://img.shields.io/github/workflow/status/LouisGariepy/cornucopia/ci" alt="actions status" />
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
</div>

<div align="center">
  <h4>
    <a href="#install">
      Install
    </a>
    <span> | </span>
    <a href="examples/basic/README.md">
      Example
    </a>
  </h4>
</div>

<br />

Cornucopia is a small CLI utility resting on `tokio-postgres` designed to facilitate PostgreSQL workflows in Rust.

Cornucopia aims to get out of your way, **transpiling your PostgreSQL queries to Rust** on demand without requiring you to maintain a live database connection. Each query is prepared against your schema, ensuring that the query statements are valid SQL. These prepared statements are then be used to generate properly type-checked Rust code for this query. 

Keep reading for more info, or take a look at the [basic example](https://github.com/LouisGariepy/cornucopia/tree/main/examples/basic) for a quickstart 🚀.

---

## Install
### Container manager
Cornucopia spawns a `postgres` container when it generates your Rust modules, so, you'll need a working `docker` or `podman` command. 

Note that to use docker on linux, **non-sudo users need to be in the docker group**. For a step-by-step guide, please read the official docker [installation](https://docs.docker.com/get-docker/) and [post-installation](https://docs.docker.com/engine/install/linux-postinstall/) docs. 

No special installation steps are needed for `podman`, but note that you will need to pass a CLI flag (`-p` or `--podman`) because cornucopia defaults to `docker`.

### Dependencies
Cornucopia will generate queries powered by the `tokio` runtime through `tokio-postgres`, so you will need add the latest version of these to your `Cargo.toml`. If you wish to use pooled connections, you'll also need `deadpool-postgres`. Finally, you will need the `cornucopia_client` crate, which has only one item: the `GenericClient` trait. You might need more dependencies depending on which features you intend to use. The code block below shows what your dependencies might look like with every feature that `cornucopia` supports enabled:
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.17.0", features = ["full"] }
deadpool-postgres = { version = "0.10.2" }
postgres-types = { version = "0.2.2", features = ["derive"] }
tokio-postgres = { version = "0.7.5", features = [
    "with-serde_json-1",
    "with-time-0_3",
    "with-uuid-0_8",
    "with-eui48-1",
] }
cornucopia_client = "0.1.0"
serde = { version = "1.0.136", features = ["derive"] }
serde_json = "1.0.79"
time = "0.3.9"
uuid = "0.8.2"
eui48 = "1.1.0"
```
You can omit `tokio-postgres` feature flags for `json`, `time`, `uuid`, `eui48` and their corresponding crates if you don't need them.

### Cornucopia CLI
Aside from the dependencies, you will need the `cornucopia` cli to generate your Rust modules. This can be done via a simple `cargo install cornucopia` which will pull the latest binary and install it in your `cargo` path.

## Concepts
This section explain a bit more about how cornucopia works. If you just want to get started, you should take a look at the [basic example](https://github.com/LouisGariepy/cornucopia/tree/main/examples/basic).

Cornucopia is pretty simple to use. Your migrations and queries should each reside in a dedicated folder, and from there the CLI takes care of the rest for you. In the next sections, we'll explore the basic usage, but feel free to explore the CLI's whole interface using the `--help` option at any point. For convenience, this is also available [in this repository](https://github.com/LouisGariepy/cornucopia/blob/main/cli.md).

### Migrations
The basic `cornucopia generate` command spins a new container, runs your migrations, generates your queries and cleanups the container. If you want to manage the database and migrations yourself, use the `cornucopia generate live` command to connect to an arbitrary live database. Keep in mind that your queries must still be otherwise compatible with cornucopia (e.g. with regards to [supported types](https://github.com/LouisGariepy/cornucopia#supported-types) and [annotation syntax](https://github.com/LouisGariepy/cornucopia#query-annotation-syntax).

New migrations can be added using the command `cornucopia migration new`. Cornucopia will automatically manage migrations when it generates your Rust modules, but you can also use the command `cornucopia migration run` to run migrations on your production database too if you so desire.

### Queries
Each `.sql` file in your queries directory will be converted into a Rust module containing functions corresponding to each query. These functions are fully typed, giving you insight into your SQL, and pretty strong guards against runtime errors.

### Generated modules
Assume you have the following migration:
```sql
CREATE TABLE Authors (
    Id SERIAL NOT NULL,
    Name VARCHAR(70) NOT NULL,
    Country VARCHAR(100) NOT NULL,
    PRIMARY KEY(Id)
);
```
Then, the following query
```sql
--! authors()*
SELECT * FROM Authors;
```
will be turned by `cornucopia` into
```rust
pub async fn authors(client: &Client) -> Result<Vec<(i32, String, String)>, Error> {
    let stmt = client
        .prepare_typed_cached(
            "SELECT * FROM Authors;", &[],
        )
        .await?;

    let res = client.query(&stmt, &[]).await?;

    let return_value = res
        .iter()
        .map(|res| {
            let return_value_0: i32 = res.get(0);
            let return_value_1: String = res.get(1);
            let return_value_2: String = res.get(2);
            (return_value_0, return_value_1, return_value_2)
        })
        .collect::<Vec<(i32, String, String)>>();
    Ok(return_value)
}
```
Not bad! The generated function uses prepared statements, a statement cache, and strong typing (Notice how the returned rows' types have been inferred!). This is only a taste of what you can achieve, but should be fairly representative of what's going on under the hood.

### Query annotation syntax
As you may have noticed from the previous section, this little comment `--! authors()*` is doing a lot of heavy-lifting for us. It tells `cornucopia` to generate a function named `authors` with no parameters. Since there is no specified return, cornucopia will automatically infer what's being returned. Then, there's the asterisk `*` which signals that this query will return zero or more results. That's how we ended up with a `Vec` return in the generated query in the [section above](#generated-modules).

Note that comments that do not start with `--!` (e.g. `-- This`) are simply ignored by `cornucopia`, so feel free to use them as you usually would.

So, what else can we do with those annotations? The grammar can be summed up as:
```<NAME> (<PARAMS>) <RETURN> <QUANTIFIER>```
The full grammar is available in the `grammar.pest` file, but you shouldn't really have to look at it. The syntax is pretty intuitive once you look at it.

The next subsections will explain what each token means. Each section start with examples so you should be able to follow just by skimming through.

#### Name
> `helloWorld2`, `informative_query_name`

The name of the generated function. It has to be a valid PostgresQL and Rust identifier.

#### Params
> `()`, `(a_nice_param, )`, `(a, b)`

The parameters of the prepared statement, separated by commas (with an optional trailing comma.) 

The order in which parameters are given corresponds to the parameter number (e.g. the first parameter is `$1` in the statement). **Every PostgreSQL parameter `$i` must have a corresponding parameter in the annotation parameter list**.

#### Return type
There are two kinds of returns, implicit and explicit. 

##### Implicit return
Implicit returns don't name the returned columns. The column types are inferred using prepared statements. To make a return implicit, simply omit it (you don't have to write anything).

Implicit returns are further categorized into void, scalar, and tuple types depending on the number of columns returned. For example,

* A query returning no column would result in `()`
* A query returning a single `TEXT` column would result in `String`, 
* A query returning a `TEXT` and a `INTEGER` would result in `(String, i32)`

##### Explicit return
> `{}`, `{cool_field, }`, `{a, b?, c}`

Explicit returns give a name to the returned columns. The column types are inferred using prepared statements. To make a return explicit, list the returned column names inside curly brackets, in the same order as they are returned in the statement, separated by commas, with an optional trailing comma. Each identifier can also be followed by an optional nullable marker `?` which indicates that the column is potentially null (`Option`al in Rust). **There must be exactly as many names in the explicit return the as there are returned columns**. Each query that has an explicit return will generate a Rust `struct` to hold the query data. For example, this query
```sql
--! example_query() {name, country} *
SELECT Name, Country FROM Authors;
```
would result in this `struct` and function being generated
```rust
pub struct ExampleQuery {
    pub name: String,
    pub country: String
}

pub async fn authors(client: &Client) -> Result<Vec<ExampleQuery>, Error> {
    /* ....omitted for brevity... */
}
```

#### Quantifier
> ` ` (no quantifier), `?`, `*`

The quantifier indicates the expected number of rows returned by a query. If no quantifier is specified, then it is assumed that only one record will be returned. Using `*` and `?` (corresponding to the "zero or more" and "zero or one" quantifiers) will wrap the resulting Rust type in a `Vec` and `Option` respectively. To sum it up:

* ` ` (no quantifier) results in `T`
* `*` results in `Vec<T>`
* `?` results in `Option<T>`

Note that explicit returns' columns  can be marked as nullable with `?`, while the `?` quantifier acts on the whole row.

### Transactions
Generated queries take a `GenericClient` as parameter, which accepts both `Client`s and `Transaction`s.

### Connection pools
Generated queries take a `GenericClient` as parameter, which accepts both connections from `tokio-postgres` (non-pooled) and `deadpool_postgres` (pooled).

## Automatically generate queries
You can make use of Rust's build script feature to automatically regenerate your cornucopia queries upon building your crate, only when your SQL has changed. The simplest way to achieve this is simply to call cornucopia's CLI inside your `build.rs` file. You can learn more about this feature in this [example](examples/auto_build/README.md).

## Formatting
By default, Cornucopia will run `rustfmt` on your your queries to facilitate manual inspection of the code. If you don't want this, or you don't have access to `rustfmt` you can use the `--no-formatting` CLI flag.

## Supported types
### Base types
| PostgrsQL type                               | Rust type                 |
| -------------------------------------------- | ------------------------- |
| `bool`, `boolean`                            | `bool`                    |
| `"char"`                                     | `i8`                      |
| `smallint`, `int2`, `smallserial`, `serial2` | `i16`                     |
| `int`, `int4`, `serial`, `serial4`           | `i32`                     |
| `bigint`, `int8`, `bigserial`, `serial8`     | `i64`                     |
| `real`, `float4`                             | `f32`                     |
| `double precision`, `float8`                 | `f64`                     |
| `text`                                       | `String`                  |
| `varchar`                                    | `String`                  |
| `bytea`                                      | `Vec<u8>`                 |
| `timestamp without time zone`, `timestamp`   | `time::PrimitiveDateTime` |
| `timestamp with time zone`, `timestamptz`    | `time::OffsetDateTime`    |
| `date`                                       | `time::Date`              |
| `time`                                       | `time::Time`              |
| `json`                                       | `serde_json::Value`       |
| `jsonb`                                      | `serde_json::Value`       |
| `uuid`                                       | `uuid::Uuid`              |
| `inet`                                       | `std::net::IpAddr`        |
| `macaddr`                                    | `eui48::MacAddress`       |

### Custom types
Cornucopia also supports user-defined `enum`s, `composite`s and `domain`s. Just like base types, custom types will be generated automatically
by inspecting your database. The only requirement for your custom types is that they be based on other supported types (base or custom).
Cornucopia is also aware of your types' namespaces (what PostgreSQL calls schemas), so it will correctly handle custom types like `my_schema.my_custom_type`.

## MSRV
This crate uses Rust 2021 edition, which requires at least version 1.56.

## License
Licensed under the [MIT license](http://opensource.org/licenses/MIT).
