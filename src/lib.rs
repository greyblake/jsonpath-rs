extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate error_chain;

use serde_json::Value;

mod errors;
mod parser;
mod selector;
mod structs;

pub use selector::Selector;
