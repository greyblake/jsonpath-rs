use pest::Parser;

use errors::*;
use structs::Filter;
use std::error::Error as StdError;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct ExpressionParser;

pub fn parse(expression: &str) -> Result<Vec<Filter>> {
    let pairs = ExpressionParser::parse_str(Rule::expression, expression)
        .map_err(|e| Error::from_kind(ErrorKind::Parse(e.description().to_owned())))?;

    for root in pairs.take(1) {
        let mut filters:Vec<Filter> = vec![];
        for token in root.into_inner() {
            match token.as_rule() {
                Rule::dollar => filters.push(Filter::Root),
                Rule::child => {
                    let ident = token.into_inner().next().unwrap().as_str().to_owned();
                    filters.push(Filter::Child(ident))
                },
                Rule::descendant => {
                    let ident = token.into_inner().next().unwrap().as_str().to_owned();
                    filters.push(Filter::Descendant(ident))
                },
                _ => unreachable!()
            }
        }
        return Ok(filters);
    }
    unreachable!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let exp = "$..book.title";
        let filters = parse(exp).unwrap();
        assert_eq!(
            filters,
            vec![
                Filter::Root,
                Filter::Descendant("book".to_owned()),
                Filter::Child("title".to_owned()),
            ]
        );
    }
}
