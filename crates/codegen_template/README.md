Performs variable interpolation against the input and store the result into
a writable output.

# Display

You can interpolate any type implementing the [`Display`](std::fmt::Display) trait using `$var`
or `${var}`. This grabs the `var` variable that is currently in scope and
format it into the output.

# Lazy

You can interpolate formatting closure implementing the [`Fn(&mut W)`] trait
using `$!lazy` or `$!{lazy}`. This grabs the `lazy` variable that is currently
in scope and call it with th output as arg in the right time.

# Repetition

Repetition is done using `$(...)`. This iterates through the elements of any variable
interpolated within the repetition and inserts a copy of the repetition body
for each one. The variables in an interpolation must implement the [`Iterator`] and the
[`Clone`] traits.

- `$($var)` — simple repetition
- `$( struct ${var}; )` — the repetition can contain other tokens
- `$( $k => println!("{}", $!v), )` — even multiple interpolations