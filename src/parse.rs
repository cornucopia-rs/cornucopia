use error::Error;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser as Pest;

#[derive(Pest)]
#[grammar = "../grammar.pest"]
struct CornucopiaParser;

#[derive(Debug)]
pub(crate) struct ParsedQuery {
    pub(crate) line: usize,
    pub(crate) meta: ParsedQueryMeta,
    pub(crate) sql: String,
}

#[derive(Debug)]
pub(crate) struct ParsedQueryMeta {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) ret: ReturnType,
    pub(crate) quantifier: Quantifier,
}

#[derive(Debug)]
pub(crate) enum ReturnType {
    Implicit,
    Explicit { params: Vec<ExplicitReturnParam> },
}

#[derive(Debug)]
pub(crate) struct ExplicitReturnParam {
    pub(crate) name: String,
    pub(crate) is_nullable: bool,
}

#[derive(Debug)]
pub(crate) enum Quantifier {
    Vec,
    Option,
    One,
    Stream,
}

pub(crate) fn parse_query_meta(meta: &str) -> Result<ParsedQueryMeta, Error> {
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
    let params = parse_params(param_tokens)?;
    // Parse return
    let return_tokens = parser_inner.next().unwrap();
    let ret = parse_return(return_tokens);
    // Parse quantifier
    let quantifier_tokens = parser_inner.next().unwrap();
    let quantifier = parse_quantifier(quantifier_tokens);

    Ok(ParsedQueryMeta {
        name,
        params,
        ret,
        quantifier,
    })
}

fn parse_params(pair: Pair<Rule>) -> Result<Vec<String>, Error> {
    let mut param_names = Vec::new();
    for pair in pair.into_inner() {
        param_names.push(pair.as_str().to_string());
    }
    Ok(param_names)
}

fn parse_return(pair: Pair<Rule>) -> ReturnType {
    match pair.as_rule() {
        Rule::implicit_return => ReturnType::Implicit,
        Rule::struct_return => {
            let params = pair
                .into_inner()
                .next()
                .unwrap()
                .into_inner()
                .map(|pair| {
                    let is_nullable = match pair.as_rule() {
                        Rule::nullable_return_param => true,
                        Rule::non_nullable_return_param => false,
                        _ => panic!(),
                    };
                    let name = pair.into_inner().next().unwrap().as_str().to_string();
                    ExplicitReturnParam { name, is_nullable }
                })
                .collect::<Vec<ExplicitReturnParam>>();
            ReturnType::Explicit { params }
        }
        _ => panic!(),
    }
}

fn parse_quantifier(pair: Pair<Rule>) -> Quantifier {
    match pair.into_inner().next().unwrap().as_rule() {
        Rule::zero_or_more => Quantifier::Vec,
        Rule::zero_or_one => Quantifier::Option,
        Rule::one => Quantifier::One,
        Rule::stream => Quantifier::Stream,
        _ => panic!(),
    }
}

pub(crate) mod error {

    use crate::{parse::Rule, pg_type::error::UnsupportedPostgresTypeError};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    pub(crate) enum Error {
        #[error("{0}")]
        UnsupportedPostgresType(#[from] UnsupportedPostgresTypeError),
        #[error("\n{0}")]
        Pest(#[from] pest::error::Error<Rule>),
    }
}
