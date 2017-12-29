use serde_json::value::Value;

use errors::*;
use parser::parse;
use structs::Filter;

use actions::find_all;

pub struct Selector {
    filters: Vec<Filter>
}

impl Selector {
    pub fn new(expression: &str) -> Result<Self> {
        let filters = parse(expression)?;
        let selector = Self { filters };
        Ok(selector)
    }

    pub fn find_all<'a>(&self, root: &'a Value) -> Vec<&'a Value> {
        let mut findings = vec![];
        find_all(root, &self.filters, &mut findings);
        findings
    }
}
