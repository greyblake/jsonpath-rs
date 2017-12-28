use std::mem;

use structs::Token;

enum State {
    Empty,
    OneDot,
    NameStarted(String)
}

pub fn tokenize(expression: &str) -> Vec<Token> {
    Tokenizer::new().tokenize(expression).tokens
}

struct Tokenizer {
    state: State,
    tokens: Vec<Token>
}

impl Tokenizer {
    fn new() -> Self {
        Self {
            state: State::Empty,
            tokens: vec![]
        }
    }

    fn flush(&mut self) {
        let state = mem::replace(&mut self.state, State::Empty);

        match state {
            State::Empty => (),
            State::OneDot => self.tokens.push(Token::Dot),
            State::NameStarted(name) => {
                self.tokens.push(Token::Name(name))
            }
        }
    }

    fn tokenize(mut self, expression: &str) -> Self {
        for ch in expression.chars() {
            match ch {
                '$' => {
                    self.flush();
                    self.tokens.push(Token::Root);
                },
                '.' => {
                    match self.state {
                        State::Empty => {
                            self.state = State::OneDot
                        },
                        State::OneDot => {
                            self.tokens.push(Token::DoubleDot);
                            self.state = State::Empty;
                        },
                        State::NameStarted(_) => {
                            self.flush();
                            self.state = State::OneDot;
                        }
                    }
                },
                c => {
                    match self.state {
                        State::Empty => {
                            let mut name = String::with_capacity(16);
                            name.push(c);
                            self.state = State::NameStarted(name);
                        },
                        State::OneDot => {
                            self.flush();
                            let mut name = String::with_capacity(16);
                            name.push(c);
                            self.state = State::NameStarted(name);
                        },
                        State::NameStarted(mut name) => {
                            name.push(c);
                            self.state = State::NameStarted(name);
                        }
                    }
                }
            }
        }
        self.flush();
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use self::Token::*;

    #[test]
    fn test_tokenize() {
        let hey = Name("hey".to_owned());
        let hop = Name("hop".to_owned());

        assert_eq!(tokenize("$"), vec![Root]);
        assert_eq!(tokenize("$."), vec![Root, Dot]);
        assert_eq!(tokenize("$.."), vec![Root, DoubleDot]);
        assert_eq!(tokenize("$.$"), vec![Root, Dot, Root]);
        assert_eq!(tokenize("$..$"), vec![Root, DoubleDot, Root]);
        assert_eq!(tokenize("$.hey.hop"), vec![Root, Dot, hey.clone(), Dot, hop.clone()]);
        assert_eq!(tokenize("hey.hop"), vec![hey.clone(), Dot, hop.clone()]);
        assert_eq!(tokenize("hey..hop"), vec![hey, DoubleDot, hop]);
    }
}
