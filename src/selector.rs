use serde_json;
use serde_json::Number;
use serde_json::value::Value;

use errors::*;
use parser::parse;
use structs::Filter;

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
        find_by(root, &self.filters)
    }
}

fn find_by<'a, 'b>(root: &'a Value, filters: &'b [Filter]) -> Option<&'a Value> {
    match filters.split_first() {
        Some((filter, rest_filters)) => {
            match filter {
                &Filter::Root => find_by(root, rest_filters),
                &Filter::Child(ref child_name) => {
                    match root {
                        &Value::Object(ref obj) => {
                            match obj.get(child_name) {
                                Some(ref child) => find_by(child, rest_filters),
                                None => None
                            }
                        },
                        _ => None
                    }
                },
                &Filter::Descendant(ref descendant_name) => {
                    match root {
                        &Value::Object(ref obj) => {
                            for (key, val) in obj {
                                if key == descendant_name {
                                    match find_by(val, rest_filters) {
                                        Some(v) => return Some(v),
                                        None => ()
                                    }
                                } else {
                                    match val {
                                        &Value::Object(ref o) => {
                                            match find_by(val, filters) {
                                                Some(v) => return Some(v),
                                                None => ()
                                            }
                                        },
                                        &Value::Array(_) => panic!("Array handling is not implemented yet"),
                                        _ => ()
                                    }
                                }
                            }
                            None
                        },
                        _ => None
                    }
                }
                _ => None
            }
        },
        None => {
            return Some(root);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
    }

    #[test]
    fn test_find() {
        let json = r#"
            {
                "name": "John",
                "user": {
                    "name": "greyblake",
                    "age": 27,
                    "profile": {
                        "location": "Berlin"
                    }
                }
            }
        "#;
        let value: Value = serde_json::from_str(json).unwrap();

        // root
        let s1 = Selector::new("$").unwrap();
        assert_eq!(s1.find(&value), Some(&value));

        // child
        let s2 = Selector::new("$.name").unwrap();
        assert_eq!(s2.find(&value), Some(&Value::String("John".to_owned())));

        let s3 = Selector::new("$.user.name").unwrap();
        assert_eq!(s3.find(&value), Some(&Value::String("greyblake".to_owned())));

        let s4 = Selector::new("$.date").unwrap();
        assert_eq!(s4.find(&value), None);

        // descendant
        let s5 = Selector::new("$..age").unwrap();
        assert_eq!(s5.find(&value), Some(&Value::Number(Number::from(27))));

        let s6 = Selector::new("$..location").unwrap();
        assert_eq!(s6.find(&value), Some(&Value::String("Berlin".to_owned())));

        // combo
        let s7 = Selector::new("$..profile.location").unwrap();
        assert_eq!(s7.find(&value), Some(&Value::String("Berlin".to_owned())));
    }
}
