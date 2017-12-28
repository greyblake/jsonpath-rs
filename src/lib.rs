extern crate serde;
extern crate serde_json;

use serde_json::Value;

mod tokenizer;
mod parser;
mod selector;

pub use selector::Selector;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let json = r#"
            {
                "name": "John",
                "user": {
                    "name": "greyblake",
                    "age": 27
                }
            }
        "#;

        let v: Value = serde_json::from_str(json).unwrap();

        println!("{:?}", v);
    }
}
