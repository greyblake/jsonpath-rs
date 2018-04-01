use pest::iterators::Pair;
use pest::Parser;

use errors::*;
use structs::Criterion;
use std::error::Error as StdError;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct ExpressionParser;

pub fn parse(expression: &str) -> Result<Vec<Criterion>> {
    let pairs = ExpressionParser::parse(Rule::expression, expression)
        .map_err(|e| Error::from_kind(ErrorKind::Parse(e.description().to_owned())))?;

    for root in pairs.take(1) {
        let criteria = parse_tokens(root)?;
        return Ok(criteria);
    }
    unreachable!()
}

fn parse_tokens<'i>(element: Pair<'i, Rule>) -> Result<Vec<Criterion>> {
    let mut criteria: Vec<Criterion> = vec![];
    for token in element.into_inner() {
        match token.as_rule() {
            Rule::dollar => criteria.push(Criterion::Root),
            Rule::arobase => criteria.push(Criterion::Element),
            Rule::condition => match token.into_inner().next().unwrap().as_rule() {
                Rule::equal => {
                    criteria.push(Criterion::Equal);
                }
                Rule::different => {
                    criteria.push(Criterion::Different);
                }
                Rule::greater => {
                    criteria.push(Criterion::Greater);
                }
                Rule::lower => {
                    criteria.push(Criterion::Lower);
                }
                _ => {}
            },
            Rule::literal => {
                let literal = token.into_inner().next().unwrap().as_str().to_owned();
                criteria.push(Criterion::Literal(literal))
            }
            Rule::number => {
                let value = token.as_str().parse::<i64>().unwrap();
                criteria.push(Criterion::Number(value))
            }
            Rule::float => {
                let value = token.as_str().parse::<f64>().unwrap();
                criteria.push(Criterion::Float(value))
            }
            Rule::filter => {
                let filter_criteria = parse_tokens(token)?;
                criteria.push(Criterion::Filter(filter_criteria))
            }
            Rule::child => {
                let ident = token.into_inner().next().unwrap().as_str().to_owned();
                criteria.push(Criterion::NamedChild(ident))
            }
            Rule::any_child => criteria.push(Criterion::AnyChild),
            Rule::indexed_child => {
                let index: usize = token.into_inner().next().unwrap().as_str().parse()?;
                criteria.push(Criterion::IndexedChild(index));
            }
            Rule::slice => {
                let mut iter = token.into_inner();
                let from: usize = iter.next().unwrap().as_str().parse()?;
                let to: usize = iter.next().unwrap().as_str().parse()?;
                criteria.push(Criterion::Slice(from..to));
            }
            Rule::slice_to => {
                let mut iter = token.into_inner();
                let to: usize = iter.next().unwrap().as_str().parse()?;
                criteria.push(Criterion::SliceTo(..to));
            }
            Rule::slice_from => {
                let mut iter = token.into_inner();
                let from: usize = iter.next().unwrap().as_str().parse()?;
                criteria.push(Criterion::SliceFrom(from));
            }
            lol => {
                println!("{:?}", lol);
                unreachable!()
            }
        }
    }

    Ok(criteria)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        let exp = "$";
        let criteria = parse(exp).unwrap();
        assert_eq!(criteria, vec![Criterion::Root]);
    }

    #[test]
    fn test_child() {
        let exp = "$.book.title";
        let criteria = parse(exp).unwrap();
        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::NamedChild("book".to_owned()),
                Criterion::NamedChild("title".to_owned()),
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
                Criterion::NamedChild("title".to_owned()),
            ]
        );
    }

    #[test]
    fn test_indexed_child() {
        let exp = "$.books[34]";
        let criteria = parse(exp).unwrap();
        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::NamedChild("books".to_owned()),
                Criterion::IndexedChild(34),
            ]
        );
    }

    #[test]
    fn test_slice() {
        let exp = "$.books[4:7]";
        let criteria = parse(exp).unwrap();
        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::NamedChild("books".to_owned()),
                Criterion::Slice(4..7),
            ]
        );
    }

    #[test]
    fn test_slice_to() {
        let exp = "$.books[:4]";
        let criteria = parse(exp).unwrap();
        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::NamedChild("books".to_owned()),
                Criterion::SliceTo(..4),
            ]
        );
    }

    #[test]
    fn test_slice_from() {
        let exp = "$[2:]";
        let criteria = parse(exp).unwrap();
        assert_eq!(criteria, vec![Criterion::Root, Criterion::SliceFrom(2)]);
    }

    #[test]
    fn test_filter_absolute() {
        let exp = "$.books[?($.title == 'Sword Of Honor')]";
        let criteria = parse(exp).unwrap();

        let filter = vec![
            Criterion::Root,
            Criterion::NamedChild("title".to_owned()),
            Criterion::Equal,
            Criterion::Literal("Sword Of Honor".to_owned()),
        ];

        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::NamedChild("books".to_owned()),
                Criterion::Filter(filter),
            ]
        );
    }

    #[test]
    fn test_filter_relative() {
        let exp = "$.books[?(@.title != 'Sword Of Honor')]";
        let criteria = parse(exp).unwrap();

        let filter = vec![
            Criterion::Element,
            Criterion::NamedChild("title".to_owned()),
            Criterion::Different,
            Criterion::Literal("Sword Of Honor".to_owned()),
        ];

        assert_eq!(
            criteria,
            vec![
                Criterion::Root,
                Criterion::NamedChild("books".to_owned()),
                Criterion::Filter(filter),
            ]
        );
    }
}
