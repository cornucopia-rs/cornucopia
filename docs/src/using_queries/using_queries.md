# Using your generated queries
Once you have written your queries and generated your Rust code with Cornucopia, it's time to use them. Hurray 🎉!

Let's say you have generated your Rust crate into `./cornucopia` and added it to your `Cargo.toml`, then this is as simple as importing the items you need from it, like so:
```rust
use cornucopia::queries::authors;
```

## Building the query object
Building a query object starts with either the query function:
```rust
authors().bind(&client, &Some("Greece"));
```

or the generated parameter struct:
```rust
use cornucopia::{
    client::Params,
    queries::{authors, AuthorsParams}
};

authors().params(
    &client,
    &AuthorsParams {
        country: Some("Greece")
    }
);
```
The query function is useful when you have a few obvious parameters, while the parameter struct is more explicit.

Note that in order to use the `params` method, you need to import the `cornucopia::client::Params` trait.

```admonish note
Queries that don't have a return value (simple insertions, for example) don't generate a query object. Instead, when calling `bind` or `params` they execute and return the number of rows affected.
```

### Query preparation

Cornucopia provides two ways to execute queries, optimised for different use cases:

#### Default behaviour with `bind()`
The standard way to execute a query is to call `bind()` directly:

```rust
authors().bind(&client, Some("Greece")).all().await?;
```

This approach intelligently handles statement preparation:
- If your client supports statement caching (like `deadpool-postgres`), it will automatically cache-prepare the statement
- If not, it executes without preparation to avoid unnecessary latency for one-off queries

This is the recommended approach for most applications, especially when using connection pools such as `deadpool-postgres`.

#### Explicit preparation with `prepare()`
For queries that will be executed multiple times with different parameters, you can explicitly prepare the statement:

```rust
let prepared = authors().prepare(&client).await?;

// Reuse the prepared statement multiple times
for country in ["Greece", "Italy", "Spain"] {
    let results = prepared.bind(&client, Some(country)).all().await?;
    // Process results...
}
```

This provides the best performance when:
- You're executing the same query repeatedly in a loop
- You're not using a connection pool with built-in statement caching
- You want to ensure the statement is prepared once and reused

```admonish tip
When using connection pools like `deadpool-postgres`, the default `bind()` behavior is usually sufficient as the pool handles statement caching automatically. Only use explicit `prepare()` when you need to guarantee statement reuse within a specific scope.
```

## Row mapping (optional)
Query objects have a `map` method that allows them to transform the query's returned rows without requiring intermediate allocation. The following example is pretty contrived but illustrates how you can use this feature.
```rust
enum Country {
    Greece,
    TheRest
}

impl<'a> From<&'a str> for Country {
    fn from(s: &'a str) -> Self {
        if s == "Greece" {
            Self::Greece
        } else {
            Self::TheRest
        }
    }
}

struct CustomAuthor {
    full_name: String,
    country: Country,
    age: usize,
}

authors()
    .bind(&client)
    .map(|author| {
        let full_name = format!(
            "{}, {}",
            author.last_name.to_uppercase(),
            author.first_name
        );
        let country = Country::from(author.country);
        CustomAuthor {
            full_name,
            country,
            age: author.age,
        }
    });
```
The result of a map is another query object.

## Getting rows out of your queries
Once the query object has been built, use one of the following methods to select the expected number of rows:
* `opt`: one or zero rows (error otherwise).
* `one`: exactly one row (error otherwise).
* `iter`: iterator of zero or more rows.
* `all`: like `iter`, but collects the rows in a  `Vec`.

Here are some example uses:
```rust
author_by_id().bind(&client, &0).opt().await?;
author_by_id().bind(&client, &0).one().await?; // Error if this author id doesn't exist
authors().bind(&client).all().await?;
authors().bind(&client).iter().await?.collect::<Vec<_>>(); // Acts the same as the previous line
```
