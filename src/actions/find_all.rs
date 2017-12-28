use serde_json::value::Value;

use structs::Filter;

pub fn find_all<'a, 'b, 'c>(root: &'a Value, filters: &'b [Filter], findings: &'c mut Vec<&'a Value>) {
    match filters.split_first() {
        Some((filter, remaining_filters)) => {
            match filter {
                &Filter::Root => find_all(root, remaining_filters, findings),
                &Filter::Child(ref child_name) => find_all_in_child(root, remaining_filters, findings, child_name),
                &Filter::Descendant(ref descendant_name) => find_all_in_descendant(root, filters, remaining_filters, findings, descendant_name)
            }
        },
        None => findings.push(root)
    }
}

fn find_all_in_child<'a, 'b, 'c>(
    root: &'a Value,
    filters: &'b [Filter],
    findings: &'c mut Vec<&'a Value>,
    child_name: &str) {

    match root {
        &Value::Object(ref obj) => {
            match obj.get(child_name) {
                Some(ref child) => find_all(child, filters, findings),
                None => ()
            }
        },
        _ => ()
    }
}

fn find_all_in_descendant<'a, 'b, 'c>(
    root: &'a Value,
    filters: &'b [Filter],
    remaining_filters: &'b [Filter],
    findings: &'c mut Vec<&'a Value>,
    descendant_name: &str) {

    match root {
        &Value::Object(ref obj) => {
            for (key, val) in obj {
                if key == descendant_name {
                    find_all(val, remaining_filters, findings);
                }

                match val {
                    &Value::Object(_) => {
                        find_all(val, filters, findings);
                    },
                    &Value::Array(ref arr) => {
                        find_all_in_array(arr, filters, findings);
                    },
                    _ => ()
                }
            }
        },
        &Value::Array(ref arr) => {
            find_all_in_array(arr, filters, findings)
        },
        _ => ()
    }
}

fn find_all_in_array<'a, 'b, 'c>(
    arr: &'a [Value],
    filters: &'b [Filter],
    findings: &'c mut Vec<&'a Value>) {

    for item in arr {
        find_all(item, filters, findings)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_find_all() {
        let json = r#"
            {
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
        "#;
        let root: Value = serde_json::from_str(json).unwrap();

        let filters = vec![Filter::Root, Filter::Descendant("type".to_owned())];

        let mut findings = vec![];
        find_all(&root, &filters, &mut findings);
        assert_eq!(findings, vec!["dog", "cat"]);
    }
}
