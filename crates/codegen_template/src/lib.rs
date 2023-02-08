use proc_macro::TokenStream;
use unscanny::Scanner;

/// Interpolation kind
enum Kind<'a> {
    Display(&'a str),
    Iterator(&'a str),
    Call(&'a str),
}

/// Match allowed character at the beginning of a Rust identifier
fn id_start(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_start(c) || c == '_'
}

/// Match allowed character after the beginning of a Rust identifier
fn id_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

/// Try to parse a Rust identifier from `scan`
fn parse_ident<'a>(scan: &mut Scanner<'a>) -> Option<&'a str> {
    if scan.at(id_start) {
        Some(scan.eat_while(id_continue))
    } else if scan.eat_if('{') {
        scan.eat_whitespace();
        let start = scan.cursor();
        scan.eat_while(id_continue);
        let ident = scan.from(start);
        scan.eat_whitespace();
        scan.expect('}');
        scan.eat_whitespace();
        Some(ident)
    } else {
        None
    }
}

/// Parse out identifier 'ident =>'
fn parse_out<'a>(scan: &mut Scanner<'a>) -> Option<&'a str> {
    let start = scan.cursor();
    scan.eat_whitespace();
    if let Some(ident) = parse_ident(scan) {
        scan.eat_whitespace();
        if scan.eat_if("=>") {
            return Some(ident);
        }
    }
    scan.jump(start);
    None
}

/// Parse the next raw string and optional pattern pair from `scan`
/// Return ("", None) when reach EOF
fn parse_next<'a>(scan: &mut Scanner<'a>) -> (&'a str, Option<Kind<'a>>) {
    let raw = scan.eat_until('$');

    if scan.eat_if('$') {
        scan.eat_whitespace();
        let pattern = if let Some(ident) = parse_ident(scan) {
            Some(Kind::Display(ident))
        } else if scan.eat_if('!') {
            scan.eat_whitespace();
            if let Some(ident) = parse_ident(scan) {
                Some(Kind::Call(ident))
            } else {
                panic!("Unknown pattern $!{}", scan.eat_while(|_| true))
            }
        } else if scan.eat_if('(') {
            let start = scan.cursor();
            let mut count_delim = 0;
            while let Some(c) = scan.peek() {
                match c {
                    ')' if count_delim == 0 => break,
                    ')' => count_delim -= 1,
                    '(' => count_delim += 1,
                    _ => {}
                }
                scan.eat();
            }
            let inner = scan.from(start);
            scan.expect(')');
            scan.eat_whitespace();
            Some(Kind::Iterator(inner))
        } else {
            panic!("Unknown pattern ${}", scan.eat_while(|_| true))
        };

        // Remove whitespace until next pattern
        let start = scan.cursor();
        scan.eat_whitespace();
        if !scan.at("$") {
            scan.jump(start);
        }

        (raw, pattern)
    } else {
        (raw, None)
    }
}

/// Find all non iterator interpolated identifier in `scan`
fn ident_in_iterator<'a>(scan: &'a mut Scanner) -> Vec<&'a str> {
    let start = scan.cursor();
    let mut idents = Vec::new();
    while let (_, Some(pat)) = parse_next(scan) {
        match pat {
            Kind::Display(ident) | Kind::Call(ident) => idents.push(ident),
            Kind::Iterator(_) => panic!("nested repetitions are not supported"),
        }
    }
    scan.jump(start);
    idents
}

/// Generate code to write raw `str` into `out`
fn gen_str(s: &mut String, out: &str, str: &str) {
    if !str.is_empty() {
        s.push_str(out);
        s.push_str(".write_str(\"");
        for c in str.chars() {
            if c == '"' {
                s.push_str("\\\"")
            } else {
                s.push(c)
            }
        }
        s.push_str("\").unwrap();\n");
    }
}

/// Generate code to write displayable `ident` into `out`
fn gen_disp(s: &mut String, out: &str, ident: &str) {
    s.push_str(out);
    s.push_str(".write_fmt(format_args!(\"{}\",");
    s.push_str(ident);
    s.push_str(")).unwrap();\n");
}

/// Generate code to write interpolation patterns in `scan` into `out`
fn gen_recursive(scan: &mut Scanner, s: &mut String, out: &str) {
    loop {
        let (raw, pattern) = parse_next(scan);
        if raw.is_empty() && pattern.is_none() {
            break;
        }
        gen_str(s, out, raw);
        if let Some(pattern) = pattern {
            match pattern {
                Kind::Display(ident) => gen_disp(s, out, ident),
                Kind::Call(ident) => {
                    s.push_str("let w = &mut*w;");
                    s.push_str(ident);
                    s.push_str("(&mut *");
                    s.push_str(out);
                    s.push_str(");\n");
                }
                Kind::Iterator(inner) => {
                    let mut scan = Scanner::new(inner);
                    let idents = ident_in_iterator(&mut scan);
                    let mut iter = idents.iter();
                    s.push_str("{\nlet iter = ");
                    s.push_str(iter.next().unwrap());
                    s.push_str(".clone()");
                    for item in iter {
                        s.push_str(".zip(");
                        s.push_str(item);
                        s.push_str(".clone())");
                    }
                    s.push_str(";\n");
                    s.push_str("for ");
                    for _ in 1..idents.len() {
                        s.push('(');
                    }
                    let mut iter = idents.iter();
                    s.push_str(iter.next().unwrap());
                    for item in iter {
                        s.push(',');
                        s.push_str(item);
                        s.push(')');
                    }
                    s.push_str(" in iter {\n");
                    gen_recursive(&mut scan, s, out);
                    s.push_str("}\n}\n");
                }
            }
        }
    }
}

/// Performs variable interpolation against the input and store the result into
/// a writable output.
///
/// # Display
///
/// You can interpolate any type implementing the [`Display`](std::fmt::Display) trait using `$var`
/// or `${var}`. This grabs the `var` variable that is currently in scope and
/// format it into the output.
///
/// # Lazy
///
/// You can interpolate formatting closure implementing the [`Fn(&mut W)`] trait
/// using `$!lazy` or `$!{lazy}`. This grabs the `lazy` variable that is currently
/// in scope and call it with th output as arg in the right time.
///
/// # Repetition
///
/// Repetition is done using `$(...)`. This iterates through the elements of any variable
/// interpolated within the repetition and inserts a copy of the repetition body
/// for each one. The variables in an interpolation must implement the [`Iterator`] and the
/// [`Clone`] traits.
///
/// - `$($var)` — simple repetition
/// - `$( struct ${var}; )` — the repetition can contain other tokens
/// - `$( $k => println!("{}", $!v), )` — even multiple interpolations
#[proc_macro]
pub fn code(pattern: TokenStream) -> TokenStream {
    let pattern = pattern.to_string();
    let mut scan = unscanny::Scanner::new(&pattern);

    let out = parse_out(&mut scan);
    let mut s = String::new();
    s.push('{');
    if let Some(out) = out {
        gen_recursive(&mut scan, &mut s, out);
    } else {
        s.push_str("let mut out = String::new();\n");
        gen_recursive(&mut scan, &mut s, "out");
        s.push_str("out")
    }
    s.push('}');
    s.parse().unwrap()
}
