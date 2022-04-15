use error::Error;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser as Pest;

use crate::pg_type::TypeRegistrar;

#[derive(Pest)]
#[grammar = "../grammar.pest"]
struct CornucopiaParser;

#[derive(Debug)]
pub struct ParsedQuery {
    pub meta: ParsedQueryMeta,
    pub sql: String,
}

#[derive(Debug)]
pub struct ParsedQueryMeta {
    pub name: String,
    pub params: Vec<String>,
    pub override_types: Vec<Type>,
    pub ret: ReturnType,
    pub quantifier: Quantifier,
}

#[derive(Debug)]
struct UntypedParam {
    name: String,
}

impl UntypedParam {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Self {
            name: pairs.next().unwrap().as_str().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Type {
    pub schema: String,
    pub name: String,
}
#[derive(Debug)]
pub struct TypedParam {
    name: String,
    ty: Type,
}

impl TypedParam {
    fn from_pair(type_registrar: &TypeRegistrar, pair: Pair<Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();
        let s1 = pairs.next().unwrap().as_str().to_string();
        let s2 = pairs.next().unwrap().as_str().to_string().to_lowercase();
        let (name, ty_schema, ty_name) =
            if let Some(s3) = pairs.next().map(|s| s.as_str().to_string().to_lowercase()) {
                (s1, s2, s3)
            } else {
                let schema = if type_registrar.base_type(&s2).is_some() {
                    String::from("pg_catalog")
                } else {
                    //TODO
                    String::from("public")
                };
                (s1, schema, s2)
            };

        Ok(Self {
            name,
            ty: Type {
                schema: ty_schema,
                name: ty_name,
            },
        })
    }
}

#[derive(Debug)]
pub enum ReturnType {
    Implicit,
    Explicit { field_names: Vec<String> },
}

#[derive(Debug)]
pub enum Quantifier {
    ZeroOrMore,
    ZeroOrOne,
    One,
}

pub fn parse_query_meta(
    type_registrar: &TypeRegistrar,
    meta: &str,
) -> Result<ParsedQueryMeta, Error> {
    // Get top level tokens
    let mut parser_inner = CornucopiaParser::parse(Rule::parser, meta)?
        .next()
        .unwrap()
        .into_inner();
    // Parse top level tokens
    // Parse name
    let name_tokens = parser_inner.next().unwrap();
    let name = name_tokens.as_str().to_string();
    // Parse params
    let param_tokens = parser_inner.next().unwrap();
    let (params, override_types) = parse_params(type_registrar, param_tokens)?;
    // Parse return
    let return_tokens = parser_inner.next().unwrap();
    let ret = parse_return(return_tokens);
    // Parse quantifier
    let quantifier_tokens = parser_inner.next().unwrap();
    let quantifier = parse_quantifier(quantifier_tokens);

    Ok(ParsedQueryMeta {
        name,
        params,
        override_types,
        ret,
        quantifier,
    })
}

fn parse_params(
    type_registrar: &TypeRegistrar,
    pair: Pair<Rule>,
) -> Result<(Vec<String>, Vec<Type>), Error> {
    let mut override_types = Vec::new();
    let mut param_names = Vec::new();
    for pair in pair.into_inner() {
        let rule = pair.as_rule();
        if let Rule::override_params = rule {
            for pair in pair.into_inner() {
                let TypedParam { name, ty } = TypedParam::from_pair(type_registrar, pair)?;
                param_names.push(name);
                override_types.push(ty)
            }
        } else if let Rule::inferred_params = rule {
            for pair in pair.into_inner() {
                param_names.push(UntypedParam::from_pair(pair).name);
            }
        }
    }
    Ok((param_names, override_types))
}

fn parse_return(pair: Pair<Rule>) -> ReturnType {
    if let Rule::implicit_return = pair.as_rule() {
        ReturnType::Implicit
    } else {
        let field_names = pair
            .into_inner()
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| pair.as_str().to_string())
            .collect::<Vec<String>>();
        ReturnType::Explicit { field_names }
    }
}

fn parse_quantifier(pair: Pair<Rule>) -> Quantifier {
    match pair.into_inner().next().unwrap().as_rule() {
        Rule::zero_or_more => Quantifier::ZeroOrMore,
        Rule::zero_or_one => Quantifier::ZeroOrOne,
        Rule::one => Quantifier::One,
        _ => panic!(),
    }
}

pub mod error {
    use crate::{parse::Rule, pg_type::error::UnsupportedPostgresTypeError};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("error while parsing file")]
    pub enum Error {
        UnsupportedPostgresType(#[from] UnsupportedPostgresTypeError),
        Pest(#[from] pest::error::Error<Rule>),
    }
}
