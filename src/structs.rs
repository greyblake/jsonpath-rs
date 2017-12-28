use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Root,        // $
    Dot,         // .
    DoubleDot,   // ..
    Name(String) // name
}

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    Root,
    Child(String),
    Descendant(String)
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Root => write!(f, "$"),
            Token::Dot => write!(f, "."),
            Token::DoubleDot => write!(f, ".."),
            Token::Name(ref name) => write!(f, "{}", name)
        }
    }
}
