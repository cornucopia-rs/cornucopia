# Error reporting
One of Cornucopia's core goals is to provide best-in-class error reporting. For example, let's say you tried to declare a nullable field, but the query doesn't have a field with this name. You'll receive an error message such as this, *before runtime*:

```
× unknown field
   ╭─[queries/test.sql:1:1]
 1 │ --! author: (age?)
   ·              ─┬─
   ·               ╰── no field with this name was found
 2 │ SELECT * FROM author;
   ╰────
  help: use one of those names: id, name
```

This helps you catch any malformed query annotation, and will offer helpful hints to get you there. If your development environment supports links, you should be able to click the path (here `queries/test.sql:1:1`) to bring you directly to the error site in your SQL code.

Cornucopia's error reporting is quite extensive and covers a lot more than the simple case above. You can take a look at our internal `tests/integration` crate to see our whole error reporting suite.

## Error type
Cornucopia's library API provides a fully fleshed-out error type that you can use if you need more complex error-handling behaviour.
