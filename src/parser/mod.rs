mod tokenizer;

use self::tokenizer::tokenize;
use errors::*;
use structs::{Token, Filter};

pub fn parse(expression: &str) -> Result<Vec<Filter>> {
    let tokens = tokenize(expression);
    build_filters(tokens)
}

enum State {
    Empty,
    Dot,
    DoubleDot
}

fn build_filters(tokens: Vec<Token>) -> Result<Vec<Filter>> {
    let mut filters = vec![];
    let mut state = State::Empty;

    let mut is_new = true;

    for token in tokens {
        match state {
            State::Empty => {
                match token {
                    Token::Root => {
                        if is_new {
                            filters.push(Filter::Root);
                        } else {
                            return Err(Error::from_kind(ErrorKind::UnexpectedToken(Token::Root)));
                        }
                    },
                    Token::Dot => {
                        state = State::Dot;
                    },
                    Token::DoubleDot => {
                        state = State::DoubleDot;
                    },
                    Token::Name(name) => {
                        filters.push(Filter::Child(name))
                    }
                }
            },
            State::Dot => {
                match token {
                    Token::Root => {
                        return Err(Error::from_kind(ErrorKind::UnexpectedToken(Token::Root)));
                    },
                    Token::Dot => {
                        return Err(Error::from_kind(ErrorKind::UnexpectedToken(Token::Dot)));
                    },
                    Token::DoubleDot => {
                        return Err(Error::from_kind(ErrorKind::UnexpectedToken(Token::DoubleDot)));
                    },
                    Token::Name(name) => {
                        filters.push(Filter::Child(name));
                        state = State::Empty;
                    }
                }
            },
            State::DoubleDot => {
                match token {
                    Token::Root => {
                        return Err(Error::from_kind(ErrorKind::UnexpectedToken(Token::Root)));
                    },
                    Token::Dot => {
                        return Err(Error::from_kind(ErrorKind::UnexpectedToken(Token::Dot)));
                    },
                    Token::DoubleDot => {
                        return Err(Error::from_kind(ErrorKind::UnexpectedToken(Token::DoubleDot)));
                    },
                    Token::Name(name) => {
                        filters.push(Filter::Descendant(name));
                        state = State::Empty;
                    }
                }
            }
        }
        is_new = false;
    }
    Ok(filters)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_filters() {
        let tokens = vec![
            Token::Root,
            Token::Dot,
            Token::Name("age".to_owned())
        ];
        let filters = build_filters(tokens).unwrap();
        assert_eq!(filters, vec![Filter::Root, Filter::Child("age".to_owned())]);

        let tokens = vec![
            Token::Root,
            Token::DoubleDot,
            Token::Name("nickname".to_owned()),
        ];
        let filters = build_filters(tokens).unwrap();
        assert_eq!(filters, vec![Filter::Root, Filter::Descendant("nickname".to_owned())]);
    }
}
