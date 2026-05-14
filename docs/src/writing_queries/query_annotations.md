# Query annotations
Query annotations decorate a SQL statement and describe the name, parameters and returned row columns of the query.

At their most basic, they look like this

```sql
--! authors_from_country
SELECT id, name, age
FROM authors
WHERE authors.nationality = :country;
```

The `--!` token indicates a Cornucopia query annotation, and `authors_from_country` is the name of the query.

Cornucopia will actually prepare your queries against your schema, automatically finding the parameters, row columns and their respective types. That is why in most simple queries, you don't have to specify the parameters or row columns: only the query name is required.

That said, you can also go further than this simple syntax in order to customise your queries, as you will learn in the next sections

```admonish note
Query annotations are declared with this token: `--!`
```

## Nullity
By default, parameters and returned row columns will all be inferred as non-null. If you want to control their nullity, you can use the question mark (`?`) syntax:

```sql
--! authors_from_country (country?) : (age?)
SELECT id, name, age
FROM authors
WHERE authors.nationality = :country;
```

The `(country?)` and `(age?)` annotations mean that the parameter `country` and returned column `age` will be inferred as nullable (`Option` in Rust).

```admonish note
Use a colon (`:`) to separate bind parameters from row columns (both are optional, only the query name is required).
```

You can also granularly modify the nullity of composites and arrays like so:

```sql
--! example_query : (compos?.some_field?, arr?[?])
SELECT compos, arr
FROM example
```

Which means that the `compos` column and its field `some_field` are both nullable and that the `arr` column and its elements are also nullable.

## Query documentation comments
You can add documentation to your queries using `---` comments after the query annotation. These comments will be added as doc strings to the generated Rust code.

```sql
--! authors_from_country
--- Finds all authors from a specific country.
--- Parameters:
---   country: The nationality to filter by
SELECT id, name, age
FROM authors
WHERE authors.nationality = :country;
```

This will generate:

```rust
/// Finds all authors from a specific country.
/// Parameters:
///   country: The nationality to filter by
pub fn authors_from_country() -> AuthorsFromCountryStmt {
    // ...
}
```

## Custom attributes
You can add custom attributes to generated query functions using the `--#` syntax. This allows you to add deprecation warnings, conditional compilation, or any other Rust attributes.

```sql
--! authors_from_country
--# deprecated = "Use authors_from_country_v2 instead"
--# allow(dead_code)
SELECT id, name, age
FROM authors
WHERE authors.nationality = :country;
```

This will generate:

```rust
#[deprecated = "Use authors_from_country_v2 instead"]
#[allow(dead_code)]
pub fn authors_from_country() -> AuthorsFromCountryStmt {
    // ...
}
```

```admonish note
Custom attributes are declared with this token: `--#` and must come after the query annotation.
```
