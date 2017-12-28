use serde_json::value::Value;
use structs::Filter;

macro_rules! return_some {
    ( $opt:expr ) => {
        match $opt {
            Some(_) => return $opt,
            None => {
                let none: Option<&'a Value> = None;
                none
            }
        }
    }
}

pub fn find_first<'a, 'b>(root: &'a Value, filters: &'b [Filter]) -> Option<&'a Value> {
    match filters.split_first() {
        Some((filter, rest_filters)) => {
            match filter {
                &Filter::Root => find_first(root, rest_filters),
                &Filter::Child(ref child_name) => find_in_child(root, child_name, rest_filters),
                &Filter::Descendant(ref descendant_name) => find_in_descendant(root, descendant_name, filters, rest_filters)
            }
        },
        None => {
            return Some(root);
        }
    }
}

pub fn find_in_child<'a, 'b>(root: &'a Value, child_name: &str, filters: &'b [Filter]) -> Option<&'a Value> {
    match root {
        &Value::Object(ref obj) => {
            match obj.get(child_name) {
                Some(ref child) => find_first(child, filters),
                None => None
            }
        },
        _ => None
    }
}

pub fn find_in_descendant<'a, 'b>(root: &'a Value, descendant_name: &str, filters: &'b [Filter], rest_filters: &'b [Filter]) -> Option<&'a Value> {
    match root {
        &Value::Object(ref obj) => {
            for (key, val) in obj {
                if key == descendant_name {
                    return_some!(find_first(val, rest_filters));
                } else {
                    match val {
                        &Value::Object(_) => {
                            return_some!(find_first(val, filters));
                        },
                        &Value::Array(ref arr) => {
                            return_some!(find_in_array(arr, filters));
                        },
                        _ => ()
                    }
                }
            }
            None
        },
        &Value::Array(ref arr) => {
            return_some!(find_in_array(arr, filters))
        },
        _ => None
    }
}

fn find_in_array<'a, 'f>(arr: &'a [Value], filters: &'f [Filter]) -> Option<&'a Value> {
    for item in arr {
        return_some!(find_first(item, filters));
    }
    None
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use serde_json::map::Map;

    lazy_static! {
        static ref JSON: Value = {
            let json = r#"
                {
                    "name": "Sergey",
                    "user": {
                        "name": "greyblake",
                        "age": 27,
                        "profile": {
                            "location": "Berlin"
                        },
                        "pets": [
                            {
                                "type": "dog",
                                "name": "Rex"
                            },
                            {
                                "type": "cat",
                                "name": "Tom"
                            }
                        ]
                    }
                }
            "#;
            let value: Value = serde_json::from_str(json).unwrap();
            value
        };
    }

    fn assert_found<T: Into<Value>>(filters: Vec<Filter>, item: T) {
        let result = find_first(&JSON, &filters);
        assert_eq!(result, Some(&item.into()));
    }

    fn assert_not_found(filters: Vec<Filter>) {
        let result = find_first(&JSON, &filters);
        assert_eq!(result, None);
    }

    #[test]
    fn root() {
        let json: Value = JSON.clone();
        let filters = vec![Filter::Root];
        let result = find_first(&json, &filters);
        assert_eq!(result, Some(&json));
    }

    #[test]
    fn child_string() {
        let filters = vec![Filter::Root, Filter::Child("name".to_owned())];
        assert_found(filters, "Sergey");
    }

    #[test]
    fn deep_child_string() {
        let filters = vec![
            Filter::Root,
            Filter::Child("user".to_owned()),
            Filter::Child("name".to_owned())
        ];
        assert_found(filters, "greyblake");
    }

    #[test]
    fn deep_child_object() {
        let filters = vec![
            Filter::Root,
            Filter::Child("user".to_owned()),
            Filter::Child("profile".to_owned())
        ];

        let mut expected = Map::new();
        expected.insert("location".into(), "Berlin".into());
        assert_found(filters, expected);
    }

    #[test]
    fn when_child_not_found() {
        let filters = vec![
            Filter::Root,
            Filter::Child("girlfriend".to_owned()),
        ];
        assert_not_found(filters);
    }

    #[test]
    fn when_deep_child_not_found() {
        let filters = vec![
            Filter::Root,
            Filter::Child("user".to_owned()),
            Filter::Child("location".to_owned())
        ];
        assert_not_found(filters);
    }

    #[test]
    fn when_descendant() {
        let filters = vec![
            Filter::Root,
            Filter::Descendant("age".to_owned()),
        ];
        assert_found(filters, 27);
    }

    #[test]
    fn when_deep_descendant() {
        let filters = vec![
            Filter::Root,
            Filter::Descendant("location".to_owned()),
        ];
        assert_found(filters, "Berlin");
    }

    #[test]
    fn when_descendant_in_array() {
        let filters = vec![
            Filter::Root,
            Filter::Descendant("type".to_owned()),
        ];
        assert_found(filters, "dog");
    }

    #[test]
    fn when_child_and_descendant() {
        let filters = vec![
            Filter::Root,
            Filter::Child("user".to_owned()),
            Filter::Descendant("location".to_owned()),
        ];
        assert_found(filters, "Berlin");
    }

    #[test]
    fn when_descendant_and_child() {
        let filters = vec![
            Filter::Root,
            Filter::Descendant("profile".to_owned()),
            Filter::Child("location".to_owned()),
        ];
        assert_found(filters, "Berlin");
    }

    #[test]
    fn when_child_and_descendant_in_array() {
        let filters = vec![
            Filter::Root,
            Filter::Child("user".to_owned()),
            Filter::Child("pets".to_owned()),
            Filter::Descendant("type".to_owned()),
        ];
        assert_found(filters, "dog");
    }

    #[test]
    fn when_child_and_descendant_not_found() {
        let filters = vec![
            Filter::Root,
            Filter::Child("location".to_owned()),
            Filter::Descendant("user".to_owned()),
        ];
        assert_not_found(filters);
    }

    #[test]
    fn when_descendant_and_child_not_found() {
        let filters = vec![
            Filter::Root,
            Filter::Descendant("profile".to_owned()),
            Filter::Child("user".to_owned()),
        ];
        assert_not_found(filters);
    }
}
