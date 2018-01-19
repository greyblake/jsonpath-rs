use pest::Parser;

use errors::*;
use structs::Criterion;
use std::error::Error as StdError;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct ExpressionParser;

pub fn parse(expression: &str) -> Result<Vec<Criterion>> {
    let pairs = ExpressionParser::parse_str(Rule::expression, expression)
        .map_err(|e| Error::from_kind(ErrorKind::Parse(e.description().to_owned())))?;

    for root in pairs.take(1) {
        let mut criteria: Vec<Criterion> = vec![];
        for token in root.into_inner() {

            match token.as_rule() {
                Rule::dollar => criteria.push(Criterion::Root),
                Rule::child => {
                    let ident = token.into_inner().next().unwrap().as_str().to_owned();
                    criteria.push(Criterion::Child(ident))
                },
                Rule::any_child => {
                    criteria.push(Criterion::AnyChild)
                }
                _ => unreachable!()
            }
        }
        return Ok(criteria);
    }
    unreachable!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let exp = "$.book.title";
        let criteria = parse(exp).unwrap();
        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::Child("book".to_owned()),
                Criterion::Child("title".to_owned()),
            ]
        );
    }

    #[test]
    fn test_any_child() {
        let exp = "$.*.title";
        let criteria = parse(exp).unwrap();
        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::AnyChild,
                Criterion::Child("title".to_owned()),
            ]
        );
    }
}
