extern crate serde;
extern crate serde_json;

use serde_json::Value;

mod parser;
mod selector;

pub use selector::Selector;
