use serde_json::value::Value;

use errors::*;
use iter::Iter;
use parser::parse;
use structs::Criterion;

pub struct Selector {
    criteria: Vec<Criterion>,
}

impl Selector {
    pub fn new(expression: &str) -> Result<Self> {
        let criteria = parse(expression)?;
        let selector = Self { criteria };
        Ok(selector)
    }

    pub fn find<'a, 'b>(&'b self, root: &'a Value) -> Iter<'a, 'b> {
        Iter::new(root, &self.criteria)
    }
}
