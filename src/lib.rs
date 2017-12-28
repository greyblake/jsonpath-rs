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
