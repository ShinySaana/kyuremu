//! Operators to enrich, prune, reference, or allow more complex array operations.
//! Heavily inspired by [spruce](https://github.com/geofffranks/spruce).

use nom::{
    branch::alt, bytes::complete::{is_not, take_while1}, character::complete::{alpha0, alpha1, alphanumeric1, char, multispace0}, combinator::{rest, success}, error::ParseError, multi::many0, sequence::{delimited, pair, terminated}, IResult, Parser
  };

pub enum Operator {
    Sort
}

impl Operator {
    pub fn try_parse(input: &str) -> Option<Operator> {
        let mut operator_parser =
            drop_surrounding_whitespace(
                double_parens::<nom::error::Error<&str>>()
            );
        match operator_parser.parse(input) {
            Ok(inner) => {
                println!("{:?}", inner.1);

                match drop_surrounding_whitespace(rest::<&str, nom::error::Error<&str>>).parse(&inner.1) {
                    Ok(even_inner) => println!("{:?}", even_inner.1),
                    
                    Err(_) => {}
                }
            },
            Err(_) => {}
        };

        None
    }
}

fn maybe_operator_name<'a, E: ParseError<&'a str>>(
) -> impl Parser<&'a str, Output = &'a str, Error = E>
{
    alpha1
}

fn reference<'a, E: ParseError<&'a str>>(
) -> impl Parser<&'a str, Output = &'a str, Error = E>
{
    take_while1()
}

fn drop_surrounding_whitespace<'a, E: ParseError<&'a str>, F>(
    parser: F
) -> impl Parser<&'a str, Output = &'a str, Error = E>
where F: Parser<&'a str, Output = &'a str, Error = E>
{
    delimited(
        multispace0,
        parser,
        multispace0
    )
}

fn double_opening_parens<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, Output = (char, char), Error = E>
{
    pair(char('('), char('('))
}

fn double_closing_parens<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, Output = (char, char), Error = E>
{
    pair(char(')'), char(')'))
}

fn double_parens<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, Output = &'a str, Error = E> {
    delimited(
        double_opening_parens(),
        is_not(")"),
        double_closing_parens()
    )
}
