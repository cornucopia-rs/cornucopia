use proc_macro::TokenStream;
use unscanny::Scanner;

enum Pattern<'a> {
    Display(&'a str),
    Iterator(&'a str),
    Unknown(&'a str),
}

const PATTERN: char = '#';

fn ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn parse_raw<'a>(scan: &mut Scanner<'a>) -> &'a str {
    let start = scan.cursor();
    loop {
        scan.eat_until(PATTERN);
        let before_pat = scan.cursor();
        // Check if is pattern
        if scan.eat_if(PATTERN) {
            scan.eat_whitespace();
            match scan.peek() {
                Some('[') => continue,
                Some(_) => {
                    scan.jump(before_pat);
                    return scan.from(start);
                }
                None => return scan.from(start),
            }
        } else {
            return scan.from(start);
        }
    }
}

fn parse_next<'a>(scan: &mut Scanner<'a>) -> (&'a str, Option<Pattern<'a>>) {
    let raw = parse_raw(scan);

    if scan.eat_if(PATTERN) {
        scan.eat_whitespace();
        let pattern = if scan.at(char::is_alphabetic) {
            Some(Pattern::Display(scan.eat_while(ident)))
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
            let mut sep = scan.eat();
            if sep != Some('*') {
                scan.eat_whitespace();
                scan.expect("*");
            } else {
                sep.take();
            }
            Some(Pattern::Iterator(inner))
        } else {
            Some(Pattern::Unknown(scan.eat_while(|_| true)))
        };
        (raw, pattern)
    } else {
        (raw, None)
    }
}

fn ident_in_iterator<'a>(scan: &'a mut Scanner) -> Vec<&'a str> {
    let start = scan.cursor();
    let mut idents = Vec::new();
    loop {
        parse_raw(scan);
        if scan.eat_if(PATTERN) {
            scan.eat_whitespace();
            if scan.at(char::is_alphabetic) {
                idents.push(scan.eat_while(ident))
            } else {
                //panic!("Expected ident");
            }
        } else {
            break;
        }
    }
    scan.jump(start);
    idents
}

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

fn gen_disp(s: &mut String, out: &str, ident: &str) {
    s.push_str(out);
    s.push_str(".write_fmt(format_args!(\"{}\",");
    s.push_str(ident);
    s.push_str(")).unwrap();\n");
}

fn gen_recursive<'a>(scan: &'a mut Scanner, s: &mut String, out: &str) {
    s.push('{');
    loop {
        let (raw, pattern) = parse_next(scan);
        if raw.is_empty() && pattern.is_none() {
            break;
        }
        gen_str(s, out, raw);
        if let Some(pattern) = pattern {
            match pattern {
                Pattern::Display(pat) => gen_disp(s, out, pat),
                Pattern::Iterator(inner) => {
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
                Pattern::Unknown(it) => {
                    panic!("unknown: '{}'", it);
                    //writeln!(s, "// \"unknown: '{}'\";", it.replace('"', "\\\"")).unwrap()
                }
            }
        }
    }
    s.push('}');
}

#[proc_macro]
pub fn quote(pattern: TokenStream) -> TokenStream {
    let pattern = pattern.to_string();
    let mut scan = unscanny::Scanner::new(&pattern);
    scan.eat_whitespace();
    let out = scan.eat_while(ident);
    scan.eat_whitespace();
    scan.expect("=>");
    scan.eat_whitespace();

    let mut s = String::new();
    gen_recursive(&mut scan, &mut s, out);
    s.parse().unwrap()
}
