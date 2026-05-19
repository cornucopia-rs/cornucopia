# Ergonomic parameters
To make working with bind parameters, Cornucopia uses umbrella traits that allow you to pass different concrete types to the same query.

For example:
```rust
authors_by_first_name.bind(&client, &"John").all(); // This works
authors_by_first_name.bind(&client, &String::from("John")).all(); // This also works
```

Here's the list of umbrella traits and the concrete types they abstract over.

```admonish
The pseudo trait bounds given here are very informal, but they should be easy enough to understand.

If you need to see exactly what the trait bounds are, these traits are generated from the `core_type_traits` function
of [codegen/client.rs](https://github.com/cornucopia-rs/cornucopia/blob/main/src/codegen/client.rs) in Cornucopia.
```

## `StringSql`
* `String`
* `&str`
* `Cow<'_, str>`
* `Box<str>`

## `BytesSql`
* `Vec<u8>`
* `&[u8]`

## `JsonSql`
* `serde_json::Value`
* `postgres_types::Json`

## `ArraySql`
* `Vec<T>`
* `&[T]`
* `IterSql`

### Notes on `IterSql`
This is a wrapper type that allows you to treat an iterator as an `ArraySql` for the purpose of passing parameters.

```admonish note
Ergonomic parameters are not supported in composite types yet. This means that composite types fields will only accept concrete types. It should be possible to lift this restriction in the future.
```

## Passing `None` for a nullable parameter

When an ergonomic parameter is declared nullable (e.g. `--! insert_book (author?)`), the generated `bind` signature accepts an `&Option<T>` where `T` is constrained by one of the umbrella traits above. Because the trait abstracts over several concrete types, the compiler cannot infer `T` from a bare `None`:

```rust
insert_book().bind(&client, &None, &"Necronomicon"); // Error: type annotations needed
```

To pass `None`, specify a concrete type (any type that implements the umbrella trait works):

```rust
insert_book().bind(&client, &None::<&str>, &"Necronomicon");   // OK
insert_book().bind(&client, &None::<String>, &"Necronomicon"); // Also OK
```

This isn't specific to `StringSql`; the same applies to `BytesSql`, `JsonSql`, and `ArraySql` whenever the parameter is nullable.
