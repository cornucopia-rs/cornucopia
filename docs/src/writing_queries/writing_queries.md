# Writing queries
Your queries consist of PostgreSQL statements using named parameters and decorated by special comment annotations.

Each query file can contain as many queries as you want and will be translated into a submodule inside your generated code file.

## Named parameters
To make it easier to write robust queries, Cornucopia uses named bind parameters for queries. Named bind parameters start with a colon and are followed by an identifier like `:this`. This is only for user convenience though, behind the scenes the query is rewritten using pure PostgreSQL syntax.

It may seem like a gratuitous deviation from PostgreSQL, but the improved expressivity is worth it in our opinion.

```admonish warning
Queries **MUST** use named parameters (like `:name`) instead of indexed ones like `$3`.
```

## Rust keywords
When generating your code, Cornucopia will automatically escape identifiers that collide with non-strict Rust keywords. For example, if your SQL query has a column named `async`, it will be generated as `r#async`. This can be useful sometimes, but you should avoid such collisions if possible because it makes the generated code more cumbersome.

Strict keywords will result in a code generation error.

## Annotations
Each SQL query that is to be used with Cornucopia must be annotated using simple SQL comments. These special comments are parsed by Cornucopia and allow you to customize the generated code.

In addition to query annotations, you can also use type annotations to reuse returned columns and parameters between multiple queries.

The next subsections cover query and type annotations in greater detail.
