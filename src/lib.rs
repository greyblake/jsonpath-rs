//! JSONPath implementation for Rust.
//!
//! The library is in development stage at the moment, but you can use already
//! limited functionality.
//!
//! Currently the following operators are supported:
//!
//! * `$` - The root element. All path expression should start with it.
//! * `.` - A direct child element.
//! * `..` - Any descendant element (aka deep child).
//!
//!
//! # Example
//!
//! ```
//! extern crate jsonpath;
//! extern crate serde_json;
//!
//! use jsonpath::Selector;
//! use serde_json::Value;
//!
//! fn main() {
//!     let jsondoc = r#"
//!         {
//!             "favorites": {
//!                "books": [
//!                    {
//!                        "title": "Der schwarze Obelist",
//!                        "author": "Erich Maria Remarque"
//!                    },
//!                    {
//!                        "title": "Le mur",
//!                        "author": "Jean-Paul Sartre"
//!                    }
//!                ]
//!             }
//!         }
//!     "#;
//!
//!     // Parse JSON document
//!     let json: Value = serde_json::from_str(jsondoc).unwrap();
//!
//!     // Create a JSONPath selector
//!     let selector = Selector::new("$.favorites..title").unwrap();
//!
//!     // Apply the selector to the JSON and convert Vec<&Value> into Vec<&str>
//!     let titles: Vec<&str> = selector.find_all(&json)
//!         .iter()
//!         .map(|t| t.as_str().unwrap())
//!         .collect();
//!     assert_eq!(titles, vec!["Der schwarze Obelist", "Le mur"]);
//! }
//!
//! ```


extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod actions;
mod errors;
mod parser;
mod selector;
mod structs;

pub use selector::Selector;
