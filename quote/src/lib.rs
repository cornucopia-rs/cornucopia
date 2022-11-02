use proc_macro::TokenStream;
use unscanny::Scanner;

enum Pattern<'a> {
    Display(&'a str),
    Iterator(&'a str),
    Call(&'a str),
}

const PATTERN: char = '$';

fn ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn ident_body(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn parse_ident<'a>(scan: &mut Scanner<'a>) -> Option<&'a str> {
    if scan.at(ident_start) {
        Some(scan.eat_while(ident_body))
    } else if scan.eat_if('{') {
        scan.eat_whitespace();
        let start = scan.cursor();
        scan.eat_while(ident_body);
        let ident = scan.from(start);
        scan.eat_whitespace();
        scan.expect('}');
        scan.eat_whitespace();
        Some(ident)
    } else {
        None
    }
}

fn parse_next<'a>(scan: &mut Scanner<'a>) -> (&'a str, Option<Pattern<'a>>) {
    let raw = scan.eat_until(PATTERN);

    if scan.eat_if(PATTERN) {
        scan.eat_whitespace();
        let pattern = if let Some(ident) = parse_ident(scan) {
            Some(Pattern::Display(ident))
        } else if scan.eat_if('!') {
            scan.eat_whitespace();
            if let Some(ident) = parse_ident(scan) {
                Some(Pattern::Call(ident))
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
            let mut sep = scan.eat();
            if sep != Some('*') {
                scan.eat_whitespace();
                scan.expect("*");
            } else {
                sep.take();
            }
            Some(Pattern::Iterator(inner))
        } else {
            panic!("Unknown pattern ${}", scan.eat_while(|_| true))
        };
        (raw, pattern)
    } else {
        (raw, None)
    }
}

fn ident_in_iterator<'a>(scan: &'a mut Scanner) -> Vec<&'a str> {
    let start = scan.cursor();
    let mut idents = Vec::new();
    while let (_, Some(pat)) = parse_next(scan) {
        match pat {
            Pattern::Display(ident) | Pattern::Call(ident) => idents.push(ident),
            Pattern::Iterator(_) => unreachable!(),
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
    loop {
        let (raw, pattern) = parse_next(scan);
        if raw.is_empty() && pattern.is_none() {
            break;
        }
        gen_str(s, out, raw);
        if let Some(pattern) = pattern {
            match pattern {
                Pattern::Display(ident) => gen_disp(s, out, ident),
                Pattern::Call(ident) => {
                    s.push_str("let w = &mut*w;");
                    s.push_str(ident);
                    s.push_str("(&mut *");
                    s.push_str(out);
                    s.push_str(");\n");
                }
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
            }
        }
    }
}

#[proc_macro]
pub fn quote(pattern: TokenStream) -> TokenStream {
    let pattern = pattern.to_string();
    let mut scan = unscanny::Scanner::new(&pattern);
    scan.eat_whitespace();
    let out = scan.eat_while(ident_body);
    scan.eat_whitespace();
    scan.expect("=>");
    scan.eat_whitespace();

    let mut s = String::new();
    s.push('{');
    gen_recursive(&mut scan, &mut s, out);
    s.push('}');
    s.parse().unwrap()
}
