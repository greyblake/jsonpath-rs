use serde_json::value::Value;

use errors::*;
use parser::parse;
use structs::Filter;

use actions::find_first;

pub struct Selector {
    filters: Vec<Filter>
}

impl Selector {
    pub fn new(expression: &str) -> Result<Self> {
        let filters = parse(expression)?;
        let selector = Self { filters };
        Ok(selector)
    }

    pub fn find<'a>(&self, root: &'a Value) -> Option<&'a Value> {
        find_first(root, &self.filters)
    }
}
