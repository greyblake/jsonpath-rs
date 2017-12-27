#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Root,        // $
    Dot,         // .
    DoubleDot,   // ..
    Name(String) // name
}

enum State {
    Empty,
    OneDot,
    NameStarted(String)
}

pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut state = State::Empty;

    for ch in expression.chars() {
        match state {

            State::Empty => {
                match ch {
                    '$' => tokens.push(Token::Root),
                    '.' => {
                        state = State::OneDot;
                    }
                    c => {
                        let mut name = String::new();
                        name.push(c);
                        state = State::NameStarted(name);
                    }
                }
            },

            State::OneDot => {
                match ch {
                    '$' => {
                        tokens.push(Token::Dot);
                        tokens.push(Token::Root);
                        state = State::Empty;
                    },
                    '.' => {
                        tokens.push(Token::DoubleDot);
                        state = State::Empty;
                    }
                    c => {
                        tokens.push(Token::Dot);
                        let mut name = String::new();
                        name.push(c);
                        state = State::NameStarted(name);
                    }
                }
            },

            State::NameStarted(mut name) => {
                match ch {
                    '$' => {
                        tokens.push(Token::Name(name));
                        tokens.push(Token::Root);
                        state = State::Empty;
                    },
                    '.' => {
                        tokens.push(Token::Name(name));
                        state = State::OneDot;
                    }
                    c => {
                        name.push(c);
                        state = State::NameStarted(name);
                    }
                }
            }
        }
    }

    // Flush
    match state {
        State::Empty => (),
        State::OneDot => tokens.push(Token::Dot),
        State::NameStarted(name) => tokens.push(Token::Name(name))
    }

    tokens
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
