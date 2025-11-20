#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub name: OperatorName,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),
    Reference(Reference),
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumberLiteral {
    Float(String),
    Hex(String),
    Oct(String),
    Bin(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OperatorName(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct Reference(pub Vec<String>);

impl Reference {
}

pub mod parser {
    use super::*;

    use chumsky::{prelude::*, text::whitespace};

    impl Expr {
        pub fn try_parse(input: &str) -> Option<Expr> {
            let result = expr_parser().parse(input).into_result();

            match result {
                Ok(expr) => Some(expr),
                Err(_) => None, 
            }
        }
    }

    // TODO: Handle much more than just ascii alphanumeric
    fn at_least_x_alphanumeric_parser<'src>(amount: usize) -> impl Parser<'src, &'src str, Vec<char>> {
        any()
            .filter(char::is_ascii_alphanumeric)
            .repeated().at_least(amount)
            .collect::<Vec<char>>()
    }

    fn operator_parser<'src>() -> impl Parser<'src, &'src str, OperatorName> {
        let one_alpha = any()
            .filter(char::is_ascii_alphabetic);

        one_alpha
            .then(at_least_x_alphanumeric_parser(0))
            .map(|(start, rest)| {
                let mut val = rest.clone();
                val.insert(0, start);
                OperatorName(val.iter().collect())
            })
    }

    fn hex_integer_parser<'src>() -> impl Parser<'src, &'src str, NumberLiteral> {
        just("0")
            .ignore_then(one_of("xX"))
            .ignore_then(
                any()
                    .filter(char::is_ascii_hexdigit)
                    .repeated().at_least(1)
                    .collect::<String>()
                    .map(|v| String::from(v.to_lowercase()))
            )
            .map(NumberLiteral::Hex)
    }

    fn oct_integer_parser<'src>() -> impl Parser<'src, &'src str, NumberLiteral> {
        just("0")
            .ignore_then(one_of("oO"))
            .ignore_then(
                one_of("01234567")
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
            )
            .map(NumberLiteral::Oct)
    }

    fn bin_integer_parser<'src>() -> impl Parser<'src, &'src str, NumberLiteral> {
        just("0")
            .ignore_then(one_of("bB"))
            .ignore_then(
                one_of("01")
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
            )
            .map(NumberLiteral::Bin)
    }

    fn dec_parser<'src>() -> impl Parser<'src, &'src str, String> {
        one_of("0123456789")
            .repeated().at_least(1)
            .collect::<String>()
    }

    fn whole_part_only_float_parser<'src>() -> impl Parser<'src, &'src str, NumberLiteral> {
        dec_parser()
            .then_ignore(just(".").or_not())
            .map(|mut v| {
                v.push_str(&".0");
                NumberLiteral::Float(v)
            })
    }

    fn no_whole_part_float_parser<'src>() -> impl Parser<'src, &'src str, NumberLiteral> {
        just(".")
            .ignore_then(dec_parser())
            .map(|v| {
                let mut res = String::from("0.");
                res.push_str(&v);
                NumberLiteral::Float(res)
            })
    }

    fn whole_and_fractional_part_float_parser<'src>() -> impl Parser<'src, &'src str, NumberLiteral> {
        dec_parser()
            .then(just("."))
            .then(dec_parser())
            .map(|((whole, _), fractional)| {
                let res = whole + "." + &fractional;
                NumberLiteral::Float(res)
            })
    }

    fn number_literal_parser<'src>() -> impl Parser<'src, &'src str, NumberLiteral> {
        choice((
            bin_integer_parser(),
            oct_integer_parser(),
            hex_integer_parser(),
            whole_and_fractional_part_float_parser(),
            no_whole_part_float_parser(),
            whole_part_only_float_parser()
        ))
    }

    fn string_literal_parser<'src>() -> impl Parser<'src, &'src str, StringLiteral> {
        at_least_x_alphanumeric_parser(0)
            .map(|v| v.iter().collect::<String>())
            .padded_by(just('"'))
            .map(StringLiteral)
    }

    // TODO: Support a."b.c" parsing as ["a", "b.c"]
    // TODO: Support escaped double quotes, escaped dots, and escaped escapes
    // TODO: Test it
    fn reference_parser<'src>() -> impl Parser<'src, &'src str, Reference> {
        just("&")
            .ignore_then(
                at_least_x_alphanumeric_parser(1)
                .map(|v| v.iter().collect::<String>())
                .then(
                    just(".")
                    .ignore_then(at_least_x_alphanumeric_parser(1))
                    .map(|v| v.iter().collect::<String>())
                    .repeated()
                    .collect::<Vec<String>>()
                )
                .map(|(first, mut rest)| {
                    rest.insert(0, first);
                    rest
                })
            )
            .map(Reference)
    }

    fn argument_parser<'src>() -> impl Parser<'src, &'src str, Argument> {
        choice((
            number_literal_parser().map(Argument::NumberLiteral),
            string_literal_parser().map(Argument::StringLiteral),
            reference_parser().map(Argument::Reference)
        ))
    }

    fn expr_parser<'src>() -> impl Parser<'src, &'src str, Expr> {
            let arguments = 
                whitespace().at_least(1)
                .ignore_then(argument_parser()) 
                .repeated()
                .collect::<Vec<Argument>>()
                .then_ignore(whitespace());

            let inner_parser =
                whitespace()
                .ignore_then(operator_parser())
                .then(arguments)
                .then_ignore(whitespace())
                .map(|expr| Expr { name: expr.0, arguments: expr.1 });


            let opening_double_parens = just("((");
            let closing_double_parens: chumsky::primitive::Just<&'static str, _, _> = just("))");

            let double_parens = inner_parser
                .delimited_by(opening_double_parens, closing_double_parens)
                .padded();

            double_parens
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_operator_parser() {
            let parser = operator_parser();

            assert_eq!(parser.parse("toto").unwrap(), OperatorName("toto".to_owned()));
            assert_ne!(parser.parse("tata").unwrap(), OperatorName("toto".to_owned()));

            assert_eq!(parser.parse("a123").unwrap(), OperatorName("a123".to_owned()));
            assert_ne!(parser.parse("a321").unwrap(), OperatorName("a123".to_owned()));

            assert!(!parser.parse("").has_output());

            assert!(parser.parse("123").has_errors());
            assert!(parser.parse("1ab").has_errors());
            assert!(parser.parse("a.b").has_errors());

            assert!(parser.parse("").has_errors());
            assert!(parser.parse(" ").has_errors());
            assert!(parser.parse(" toto").has_errors());
            assert!(parser.parse("toto ").has_errors());
            assert!(parser.parse("toto tata").has_errors());
            assert!(parser.parse(" toto tata ").has_errors());
        }

        #[test]
        fn test_hex_parser() {
            let parser = hex_integer_parser();

            assert_eq!(parser.parse("0x123").unwrap(), NumberLiteral::Hex("123".to_owned()));
            assert_eq!(parser.parse("0XAbCd").unwrap(), NumberLiteral::Hex("abcd".to_owned()));

            assert!(parser.parse("0xdefg").has_errors());
            assert!(parser.parse("x123abc").has_errors());
            assert!(parser.parse("X123abc").has_errors());
            assert!(parser.parse("0123abc").has_errors());
            assert!(parser.parse("123abc").has_errors());

            assert!(parser.parse("0o123").has_errors());
            assert!(parser.parse("0b10").has_errors());

            assert!(parser.parse("").has_errors());
            assert!(parser.parse(" ").has_errors());
            assert!(parser.parse(" 0x123").has_errors());
            assert!(parser.parse("0x123 ").has_errors());
            assert!(parser.parse("0x123 0xabc").has_errors());
            assert!(parser.parse(" 0x123 0xabc ").has_errors());
        }

        #[test]
        fn test_oct_parser() {
            let parser = oct_integer_parser();

            assert_eq!(parser.parse("0o123").unwrap(), NumberLiteral::Oct("123".to_owned()));
            assert_eq!(parser.parse("0O4567").unwrap(), NumberLiteral::Oct("4567".to_owned()));

            assert!(parser.parse("0o78").has_errors());
            assert!(parser.parse("0o7a").has_errors());
            assert!(parser.parse("o123").has_errors());
            assert!(parser.parse("O123").has_errors());
            assert!(parser.parse("0123").has_errors());
            assert!(parser.parse("123").has_errors());

            assert!(parser.parse("0x123").has_errors());
            assert!(parser.parse("0b10").has_errors());

            assert!(parser.parse("").has_errors());
            assert!(parser.parse(" ").has_errors());
            assert!(parser.parse(" 0o123").has_errors());
            assert!(parser.parse("0o123 ").has_errors());
            assert!(parser.parse("0o123 0o456").has_errors());
            assert!(parser.parse(" 0x123 0x456 ").has_errors());
        }

        #[test]
        fn test_bin_parser() {
            let parser = bin_integer_parser();

            assert_eq!(parser.parse("0b1").unwrap(), NumberLiteral::Bin("1".to_owned()));
            assert_eq!(parser.parse("0b101").unwrap(), NumberLiteral::Bin("101".to_owned()));
            assert_eq!(parser.parse("0B0101").unwrap(), NumberLiteral::Bin("0101".to_owned()));

            assert!(parser.parse("0b12").has_errors());
            assert!(parser.parse("0bab").has_errors());
            assert!(parser.parse("o101").has_errors());
            assert!(parser.parse("O101").has_errors());
            assert!(parser.parse("0101").has_errors());
            assert!(parser.parse("101").has_errors());

            assert!(parser.parse("0x101").has_errors());
            assert!(parser.parse("0o101").has_errors());

            assert!(parser.parse("").has_errors());
            assert!(parser.parse(" ").has_errors());
            assert!(parser.parse(" 0b").has_errors());
            assert!(parser.parse("0b101 ").has_errors());
            assert!(parser.parse("0b101 0b010").has_errors());
            assert!(parser.parse(" 0x101 0x010 ").has_errors());
        }

        // TODO: Finish unit testing
    }
}
