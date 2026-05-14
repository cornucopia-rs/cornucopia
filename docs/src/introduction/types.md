# Supported types
## Base types
| PostgreSQL type                                   | Rust type                 |
| ------------------------------------------------- | ------------------------- |
| `bool`, `boolean`                                 | `bool`                    |
| `char`                                            | `i8`                      |
| `smallint`, `int2`, `smallserial`, `serial2`      | `i16`                     |
| `int`, `int4`, `serial`, `serial4`                | `i32`                     |
| `bigint`, `int8`, `bigserial`, `serial8`          | `i64`                     |
| `real`, `float4`                                  | `f32`                     |
| `double precision`, `float8`                      | `f64`                     |
| `text`                                            | `String`                  |
| `varchar`                                         | `String`                  |
| `bpchar`                                          | `String`                  |
| `bytea`                                           | `Vec<u8>`                 |
| `timestamp without time zone`, `timestamp`        | `chrono::NaiveDateTime`   |
| `timestamp with time zone`, `timestamptz`         | `chrono::DateTime<chrono::FixedOffset>` |
| `date`                                            | `chrono::NaiveDate`       |
| `time`                                            | `chrono::NaiveTime`       |
| `json`                                            | `serde_json::Value`       |
| `jsonb`                                           | `serde_json::Value`       |
| `uuid`                                            | `uuid::Uuid`              |
| `inet`                                            | `std::net::IpAddr`        |
| `macaddr`                                         | `eui48::MacAddress`       |
| `numeric`                                         | `rust_decimal::Decimal`   |

## Custom PostgreSQL types
Custom types like `enum`, `composite` and `domain` will be generated automatically by inspecting your database. The only requirement for your custom types is that they should be based on other supported types (base or custom).

Cornucopia is aware of your types' namespaces (what PostgreSQL calls schemas), so it will correctly handle custom types like `my_schema.my_custom_type`.

```admonish note
Domains are unwrapped into their inner types in your Rust queries.
```

## Custom Rust types
You can define custom Rust types through a `cornucopia.toml` configuration file. See ["Custom Type Mappings"](../configuration.html#custom-type-mappings) for more information.

## Array types
Cornucopia supports one-dimensional arrays when the element type is also a type supported. That is, Cornucopia supports `example_elem_type[]` if `example_elem_type` is itself a type supported by Cornucopia (base or custom).
